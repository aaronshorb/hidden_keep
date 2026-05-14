//! Player data, movement, sprite animation, fading, and player collision checks.

use crate::assets::CharacterAsset;
use crate::config::{
    BOARD_MAX_CENTER, BOARD_MIN_CENTER, PLAYER_DRAW_OFFSET_X, PLAYER_DRAW_OFFSET_Y,
    PLAYER_MOVE_SPEED, PLAYER_RADIUS, SPRITE_SCALE, SPRITE_SIZE,
};
use lerp::Lerp;
use macroquad::experimental::animation::*;
use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Idle,
    Left,
    Right,
    Up,
    Down,
}

pub struct Player {
    pub player_id: usize,
    pub color: Color,
    pub num_moves: i32,
    pub player_pos: Vec2,
    pub tokens_collected: i32,
    pub gamepiece: Circle,
    pub character: CharacterAsset,
    sprite: AnimatedSprite,
}

impl Player {
    pub fn new(
        player_id: usize,
        color: Color,
        num_moves: i32,
        player_pos: Vec2,
        character: CharacterAsset,
    ) -> Self {
        let tokens_collected = 0;
        let gamepiece = Circle::new(player_pos.x, player_pos.y, PLAYER_RADIUS);

        //animated sprite for in game avatars
        let sprite = AnimatedSprite::new(
            SPRITE_SIZE,
            SPRITE_SIZE,
            &[
                Animation {
                    name: "idle".to_string(),
                    row: 2,
                    frames: character.animation_frames.idle,
                    fps: 5,
                },
                Animation {
                    name: "left".to_string(),
                    row: 1,
                    frames: character.animation_frames.left,
                    fps: 15,
                },
                Animation {
                    name: "right".to_string(),
                    row: 0,
                    frames: character.animation_frames.right,
                    fps: 15,
                },
            ],
            true,
        );
        Player {
            player_id,
            color,
            num_moves,
            player_pos,
            tokens_collected,
            gamepiece,
            sprite,
            character,
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.character.texture,
            self.player_pos.x - PLAYER_DRAW_OFFSET_X,
            self.player_pos.y - PLAYER_DRAW_OFFSET_Y,
            self.color,
            DrawTextureParams {
                source: Some(self.sprite.frame().source_rect),
                dest_size: Some(self.sprite.frame().dest_size * SPRITE_SCALE),
                ..Default::default()
            },
        );
    }

    pub fn update_pos(&mut self, new_pos: Vec2) {
        self.player_pos = new_pos;
        self.gamepiece = Circle::new(new_pos.x, new_pos.y, PLAYER_RADIUS);
    }

    //move smoothly between spaces
    pub fn smooth_move(&mut self, target_pos: Vec2, delta: f32, direction: Direction) {
        let distance = self.player_pos.distance(target_pos);
        if distance > 0.1 {
            let move_factor = (PLAYER_MOVE_SPEED * delta).clamp(0.0, distance) / distance;
            self.animate(direction);
            self.player_pos = Lerp::lerp(self.player_pos, target_pos, move_factor);
        } else {
            self.animate(Direction::Idle);
            self.player_pos = target_pos;
        }
        self.gamepiece = Circle::new(self.player_pos.x, self.player_pos.y, PLAYER_RADIUS);
        self.player_pos.x = self.player_pos.x.clamp(BOARD_MIN_CENTER, BOARD_MAX_CENTER);
        self.player_pos.y = self.player_pos.y.clamp(BOARD_MIN_CENTER, BOARD_MAX_CENTER);
    }

    pub fn fade_in_gamepiece(&mut self, delta: f32) {
        self.color.a = Lerp::lerp(self.color.a, 1.0, delta);
        if self.color.a >= 0.9 {
            self.color.a = 1.0;
        }
    }

    pub fn fade_out_gamepiece(&mut self, delta: f32) {
        self.color.a = Lerp::lerp(self.color.a, 0.0, delta);
        if self.color.a <= 0.1 {
            self.color.a = 0.0;
        }
    }

    pub fn update_sprite(&mut self) {
        self.sprite.update();
    }

    //choose correct animation depending on direction moved
    pub fn animate(&mut self, direction: Direction) {
        let movement = if self.player_id == 0 || self.player_id == 3 {
            2
        } else {
            1
        };
        match direction {
            Direction::Left => self.sprite.set_animation(1),
            Direction::Right => self.sprite.set_animation(2),
            Direction::Up => self.sprite.set_animation(movement),
            Direction::Down => self.sprite.set_animation(movement),
            Direction::Idle => self.sprite.set_animation(0),
        }
    }
}

//function for preventing two gamepieces from occupying the same space on board.
//returns true if another gamepiece is already on the target space
pub fn player_collision(players: &[Player], target_pos: Vec2, current_player: usize) -> bool {
    for (index, player) in players.iter().enumerate() {
        if index == current_player {
            continue;
        }
        if target_pos == player.player_pos {
            return true;
        }
    }
    false
}
