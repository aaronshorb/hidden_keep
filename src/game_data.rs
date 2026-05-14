//! Shared game data used by state updates, rendering, menus, and gameplay setup.

use crate::config::{BOARD_MIN_CENTER, CELL_STRIDE, GAMEBOARD_SIZE, MAX_WALLS};
use ::rand::prelude::*;
use macroquad::prelude::*;

use crate::assets::CharacterAsset;
use crate::player::{Direction, Player};
use crate::token::Token;
use crate::wall_placement::WallPlacementState;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PostGameChoice {
    Undecided,
    Quit,
    PlayAgain,
}

pub struct GameData {
    pub num_players: usize,
    pub player_turn: usize,
    pub players_on_board: Vec<Player>,
    pub target_pos: Vec2,
    pub tokens_to_win: usize,
    pub token_on_board: Option<Token>,
    pub token_taken: bool,
    pub wall_placement: WallPlacementState,
    pub wall_collision: bool,
    pub color_to_transparent: bool,
    pub set_state: bool,
    pub element_color_change: bool,
    pub gamepieces_visible: bool,
    pub post_game_choice: PostGameChoice,
    pub token_positions: Vec<(Vec2, Texture2D)>,
    pub token_list: Vec<(Vec2, Texture2D)>,
    pub static_token_list: Vec<(Vec2, Texture2D)>,
    pub characters: Vec<CharacterAsset>,
    pub direction: Direction,
}

impl GameData {
    pub fn new(initial_pos: [Vec2; 4], max_walls: usize) -> Self {
        GameData {
            num_players: 2,
            player_turn: 0,
            players_on_board: vec![],
            target_pos: initial_pos[0],
            tokens_to_win: 5,
            token_on_board: None,
            token_taken: false,
            wall_placement: WallPlacementState::new(max_walls),
            wall_collision: false,
            color_to_transparent: false,
            set_state: false,
            element_color_change: false,
            gamepieces_visible: false,
            post_game_choice: PostGameChoice::Undecided,
            token_positions: Vec::new(),
            token_list: Vec::new(),
            static_token_list: Vec::new(),
            characters: Vec::new(),
            direction: Direction::Idle,
        }
    }

    pub fn reset(&mut self, initial_pos: [Vec2; 4]) {
        self.wall_placement.reset(MAX_WALLS);
        self.target_pos = initial_pos[self.player_turn];
        self.players_on_board.clear();
        self.post_game_choice = PostGameChoice::Undecided;
    }

    pub fn init_tokens(
        &mut self,
        item_list: &[Texture2D],
        skip_pos: &[i32; 12],
        rng: &mut ThreadRng,
    ) {
        //item textures mapped to token_positions
        let mut item_index = 0;
        self.token_positions.clear();

        for i in 0..GAMEBOARD_SIZE {
            for j in 0..GAMEBOARD_SIZE {
                let pos = i * GAMEBOARD_SIZE + j;
                if !skip_pos.contains(&pos) {
                    self.token_positions.push((
                        vec2(
                            BOARD_MIN_CENTER + (j as f32 * CELL_STRIDE),
                            BOARD_MIN_CENTER + (i as f32 * CELL_STRIDE),
                        ),
                        item_list[item_index].clone(),
                    ));
                    item_index += 1;
                }
            }
        }

        self.token_list = self.token_positions.clone();
        self.token_list.shuffle(rng);
        self.static_token_list = self.token_list.clone();
    }

    pub fn set_characters(&mut self, chars: &[CharacterAsset]) {
        self.characters = chars.to_vec();
    }
}
