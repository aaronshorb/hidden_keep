//! Game state machine and per-state update logic.

use crate::assets::Assets;
use crate::config::{
    BOARD_MAX_CENTER, BOARD_MIN_CENTER, CELL_STRIDE, DICE_SIDES, INITIAL_POS, ITEM_COLOR, MAX_WALLS,
};
use crate::player::{self, Direction, Player};
use crate::render;
use crate::{
    colors::{ColorField, GameColors},
    game_data::{GameData, PostGameChoice},
    token,
};
use ::rand::prelude::*;
use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameState {
    StartMenu,
    SetWalls,
    InGame,
    EndGame,
}

pub struct Game {
    pub data: GameData,
    pub colors: GameColors,
    pub state: GameState,
}

impl Game {
    pub async fn new(initial_pos: [Vec2; 4], max_walls: usize) -> Self {
        let data = GameData::new(initial_pos, max_walls);
        let colors = GameColors::new();
        Self {
            data,
            colors,
            state: GameState::StartMenu,
        }
    }

    pub fn update(&mut self, delta: f32, rng: &mut ThreadRng, assets: &Assets) {
        match self.state {
            GameState::StartMenu => self.update_start_menu(rng, assets),
            GameState::SetWalls => self.update_set_walls(delta, rng, assets),
            GameState::InGame => self.update_in_game(delta, rng, assets),
            GameState::EndGame => self.update_end_game(delta, rng),
        }
    }
    pub fn update_start_menu(&mut self, rng: &mut ThreadRng, assets: &Assets) {
        self.update_start_menu_state(rng);
        render::draw_start_menu(self, assets);
    }

    fn update_start_menu_state(&mut self, rng: &mut ThreadRng) {
        //move to SetWalls state
        if self.data.set_state {
            self.data.player_turn = rng.gen_range(0..self.data.num_players);
            self.data.target_pos = INITIAL_POS[self.data.player_turn];
            self.data.set_state = false;
            self.colors.title_color.a = 0.0;
            self.state = GameState::SetWalls;
        }
    }

    pub fn update_set_walls(&mut self, delta: f32, rng: &mut ThreadRng, assets: &Assets) {
        self.update_set_walls_state(delta, rng);
        render::draw_set_walls(self, assets);
    }

    //if walls haven't been set
    fn update_set_walls_state(&mut self, delta: f32, rng: &mut ThreadRng) {
        if !self.data.set_state && !self.data.element_color_change {
            self.update_wall_placement();
        }

        self.fade_in_board_colors(delta);

        // if all walls have finished being set by user
        if self.data.set_state {
            //fade out walls
            self.update_wall_fade_out(delta);
            //move to InGame state
            self.transition_to_in_game_if_ready(rng);
        }
    }

    fn update_wall_placement(&mut self) {
        self.data.wall_placement.update_placement();
    }

    fn fade_in_board_colors(&mut self, delta: f32) {
        if self.colors.gameboard_color.a < 1.0 {
            self.colors.lerp_color_in(
                ColorField::Brick,
                &mut self.data.element_color_change,
                delta * 1.8,
            );
            self.colors.lerp_color_in(
                ColorField::Pillar,
                &mut self.data.element_color_change,
                delta * 1.8,
            );
            self.colors.lerp_color_in(
                ColorField::Gameboard,
                &mut self.data.element_color_change,
                delta * 1.8,
            );
        }
    }

    fn update_wall_fade_out(&mut self, delta: f32) {
        if self.data.wall_placement.walls_on_board.is_empty() {
            self.colors.wall_color.a = 0.0;
        } else {
            for _ in &self.data.wall_placement.walls_on_board {
                self.colors.lerp_color_out(
                    ColorField::Wall,
                    &mut self.data.element_color_change,
                    delta,
                );
            }
        }
    }

    fn transition_to_in_game_if_ready(&mut self, rng: &mut ThreadRng) {
        if self.colors.wall_color.a == 0.0 {
            self.data.wall_placement.randomize_board(rng);
            self.data.set_state = false;
            self.colors.text_color.a = 0.0;
            self.data.element_color_change = true;
            self.state = GameState::InGame;
            self.data.wall_placement.walls_left = MAX_WALLS;
        }
    }

    pub fn update_in_game(&mut self, delta: f32, rng: &mut ThreadRng, assets: &Assets) {
        self.update_in_game_state(delta, rng);
        render::draw_in_game(self, assets);
    }

    fn update_in_game_state(&mut self, delta: f32, rng: &mut ThreadRng) {
        self.fade_in_in_game_colors(delta);
        self.ensure_players_created(rng);
        self.update_active_token_fade(delta);
        self.update_player_visibility(delta);
        self.handle_token_collection(delta);
        self.handle_wall_collision();
        self.animate_wall_collision(delta);
        self.spawn_next_token();
        self.handle_player_input();
        self.move_current_player(delta);
        self.check_win_condition();
        self.advance_turn_if_needed(rng);
        self.fade_out_in_game_colors(delta);
        self.transition_to_end_game_if_ready();
    }

    fn fade_in_in_game_colors(&mut self, delta: f32) {
        if self.data.element_color_change && !self.data.set_state {
            self.colors.lerp_color_in(
                ColorField::Text,
                &mut self.data.element_color_change,
                delta * 1.8,
            );
            self.colors.lerp_color_in(
                ColorField::Title,
                &mut self.data.element_color_change,
                delta * 1.8,
            );
            self.colors.lerp_color_in(
                ColorField::Gem,
                &mut self.data.element_color_change,
                delta * 1.8,
            );

            for index in 0..self.data.num_players {
                self.colors.lerp_color_in(
                    ColorField::PlayerText(index),
                    &mut self.data.element_color_change,
                    delta * 1.8,
                );
            }
        }
    }

    fn update_active_token_fade(&mut self, delta: f32) {
        if self.data.gamepieces_visible {
            if let Some(token) = &mut self.data.token_on_board {
                if !self.data.token_taken && !self.data.set_state {
                    token.fade_in_token(delta);
                }
            }
        }
    }

    //create and initialize new players at the beginning of the game
    fn ensure_players_created(&mut self, rng: &mut ThreadRng) {
        if self.data.players_on_board.len() < self.data.num_players {
            for (p, initial_pos) in INITIAL_POS.iter().enumerate().take(self.data.num_players) {
                let num_moves = *DICE_SIDES.choose(rng).unwrap();
                let player = Player::new(
                    p,
                    Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 0.0,
                    },
                    num_moves,
                    *initial_pos,
                    self.data.characters[p].clone(),
                );
                self.data.players_on_board.push(player);
            }
        }
    }

    //fade in gamepiece if not visible
    fn update_player_visibility(&mut self, delta: f32) {
        self.data.gamepieces_visible = true;

        for player in &mut self.data.players_on_board {
            if !self.data.wall_collision && player.color.a < 1.0 {
                player.fade_in_gamepiece(delta * 1.5);
                self.data.gamepieces_visible = false;
            }
        }
    }

    //if player moves to token space, the remove the token
    fn handle_token_collection(&mut self, delta: f32) {
        let circle = self.data.players_on_board[self.data.player_turn].gamepiece;
        if let Some(token) = &mut self.data.token_on_board {
            let tokenpiece = token.tokenpiece;
            if circle.overlaps(&tokenpiece) {
                self.data.token_taken = true;
                if token.fade_out_token(delta) {
                    self.data.token_on_board = None;
                    self.data.players_on_board[self.data.player_turn].tokens_collected += 1;
                    self.data.players_on_board[self.data.player_turn].num_moves = 0;
                }
            }
        }
    }

    //logic for finding collisions between gamepieces and walls
    fn handle_wall_collision(&mut self) {
        let circle = self.data.players_on_board[self.data.player_turn].gamepiece;
        let hit_wall = self
            .data
            .wall_placement
            .walls_on_board
            .iter()
            .any(|wall| wall.overlaps_circle(&circle));

        if hit_wall {
            self.data.color_to_transparent = true;
            self.data.wall_collision = true;
            self.data.target_pos = INITIAL_POS[self.data.player_turn];
            self.data.players_on_board[self.data.player_turn].num_moves = 0;
        }
    }

    // fade out/fade in gamepiece after wall collision
    fn animate_wall_collision(&mut self, delta: f32) {
        if self.data.wall_collision {
            if self.data.color_to_transparent {
                self.data.players_on_board[self.data.player_turn].animate(Direction::Idle);
                self.data.players_on_board[self.data.player_turn].fade_out_gamepiece(delta * 3.0);
                if self.data.players_on_board[self.data.player_turn].color.a == 0.0 {
                    self.data.players_on_board[self.data.player_turn]
                        .update_pos(INITIAL_POS[self.data.player_turn]);
                    self.data.color_to_transparent = false;
                }
            } else {
                self.data.players_on_board[self.data.player_turn].fade_in_gamepiece(delta * 3.0);
                if self.data.players_on_board[self.data.player_turn].color.a == 1.0 {
                    self.data.wall_collision = false;
                }
            }
        }
    }

    //check if token already on board before adding a new token
    fn spawn_next_token(&mut self) {
        if !self.data.set_state
            && self.data.token_on_board.is_none()
            && !self.data.token_list.is_empty()
        {
            self.data.token_taken = false;
            self.data.token_on_board = token::next_token(
                &self.data.players_on_board,
                &mut self.data.token_list,
                ITEM_COLOR,
            );
        }
    }

    // logic for piece movement, disallows pieces from moving off gameboard
    // set and manipulate move_flag to allow decrementing the number of moves made by the current player
    fn handle_player_input(&mut self) {
        let player_pos = self.data.players_on_board[self.data.player_turn].player_pos;
        let mut temp_target_pos = self.data.target_pos;
        if self.data.players_on_board[self.data.player_turn].num_moves > 0
            && player_pos == self.data.target_pos
            && get_last_key_pressed().is_some()
        {
            let move_flag = match () {
                _ if is_key_pressed(KeyCode::Left) && player_pos.x > BOARD_MIN_CENTER => {
                    temp_target_pos.x -= CELL_STRIDE;
                    self.data.direction = Direction::Left;
                    true
                }
                _ if is_key_pressed(KeyCode::Right) && player_pos.x < BOARD_MAX_CENTER => {
                    temp_target_pos.x += CELL_STRIDE;
                    self.data.direction = Direction::Right;
                    true
                }
                _ if is_key_pressed(KeyCode::Up) && player_pos.y > BOARD_MIN_CENTER => {
                    temp_target_pos.y -= CELL_STRIDE;
                    self.data.direction = Direction::Up;
                    true
                }
                _ if is_key_pressed(KeyCode::Down) && player_pos.y < BOARD_MAX_CENTER => {
                    temp_target_pos.y += CELL_STRIDE;
                    self.data.direction = Direction::Down;
                    true
                }
                _ => false,
            };
            //move player's gamepiece and decrement remaining moves if move_flag is true and no other piece is occupying target space
            if move_flag
                && !player::player_collision(
                    &self.data.players_on_board,
                    temp_target_pos,
                    self.data.player_turn,
                )
            {
                self.data.target_pos = temp_target_pos;
                self.data.players_on_board[self.data.player_turn].num_moves -= 1;
            }
        }
    }

    //if no wall collision, move player smoothly on gameboard
    fn move_current_player(&mut self, delta: f32) {
        if !self.data.wall_collision {
            self.data.players_on_board[self.data.player_turn].smooth_move(
                self.data.target_pos,
                delta,
                self.data.direction,
            );
        }
    }

    //if player has collected enough tokens to win, set state to true
    fn check_win_condition(&mut self) {
        if self.data.players_on_board[self.data.player_turn].tokens_collected
            == self.data.tokens_to_win.try_into().unwrap()
        {
            self.data.set_state = true;
        }
    }

    //randomly set the allowed number of moves at the start of each player's turn
    fn advance_turn_if_needed(&mut self, rng: &mut ThreadRng) {
        if self.data.players_on_board[self.data.player_turn].num_moves == 0
            && self.data.players_on_board[self.data.player_turn].player_pos == self.data.target_pos
            && !self.data.wall_collision
            && !self.data.set_state
        {
            //set player's character to idle
            self.data.direction = Direction::Idle;
            self.data.players_on_board[self.data.player_turn].animate(self.data.direction);
            if self.data.player_turn < self.data.num_players - 1 {
                self.data.player_turn += 1;
            } else {
                self.data.player_turn = 0;
            }
            self.data.players_on_board[self.data.player_turn].num_moves =
                *DICE_SIDES.choose(rng).unwrap();
            self.data.target_pos = self.data.players_on_board[self.data.player_turn].player_pos;
        }
    }

    //if set state, fade out players and text
    fn fade_out_in_game_colors(&mut self, delta: f32) {
        if self.data.set_state {
            for index in 0..self.data.num_players {
                self.colors.lerp_color_out(
                    ColorField::PlayerText(index),
                    &mut self.data.element_color_change,
                    delta * 1.8,
                );
            }
            self.colors.lerp_color_out(
                ColorField::Text,
                &mut self.data.element_color_change,
                delta * 1.8,
            );
            self.colors.lerp_color_out(
                ColorField::Title,
                &mut self.data.element_color_change,
                delta * 1.8,
            );
            self.colors.lerp_color_out(
                ColorField::Gem,
                &mut self.data.element_color_change,
                delta * 1.8,
            );
        }
    }

    //move to EndGame state
    fn transition_to_end_game_if_ready(&mut self) {
        if self.colors.text_color.a == 0.0 && self.data.set_state {
            self.data.set_state = false;
            self.colors.text_color.a = 1.0;
            self.data.gamepieces_visible = false;
            self.colors.title_color.a = 1.0;
            self.data.direction = Direction::Idle;
            self.state = GameState::EndGame;
        }
    }

    pub fn update_end_game(&mut self, delta: f32, rng: &mut ThreadRng) {
        self.update_end_game_state(delta);
        render::draw_end_game(self);
        self.restart_game_if_requested(rng);
    }

    fn update_end_game_state(&mut self, delta: f32) {
        self.update_end_game_player_fade(delta);
        self.fade_out_board_for_end_game(delta);
    }

    //fade out players
    fn update_end_game_player_fade(&mut self, delta: f32) {
        for player in self.data.players_on_board.iter_mut() {
            if player.color.a > 0.0 {
                self.data.gamepieces_visible = true;
                player.fade_out_gamepiece(delta * 3.0);
            }
        }
    }

    //if all players faded out, ask user if they want to play again
    fn fade_out_board_for_end_game(&mut self, delta: f32) {
        if self.data.post_game_choice == PostGameChoice::Undecided
            && self.colors.pillar_color.a > 0.0
        {
            self.colors.lerp_color_out(
                ColorField::Brick,
                &mut self.data.element_color_change,
                delta * 2.0,
            );
            self.colors.lerp_color_out(
                ColorField::Gameboard,
                &mut self.data.element_color_change,
                delta * 2.0,
            );
            self.colors.lerp_color_out(
                ColorField::Pillar,
                &mut self.data.element_color_change,
                delta * 2.0,
            );
        }
    }

    //if restart requested, reset game, else break
    fn restart_game_if_requested(&mut self, rng: &mut ThreadRng) {
        if self.data.post_game_choice == PostGameChoice::PlayAgain {
            self.data.player_turn = rng.gen_range(0..self.data.num_players);

            self.data.reset(INITIAL_POS);
            self.data.token_list = self.data.token_positions.clone();
            self.state = GameState::StartMenu;

            self.colors = GameColors::new();
            self.data.token_list.shuffle(rng);
            self.data.post_game_choice = PostGameChoice::Undecided;
        }
    }
}
