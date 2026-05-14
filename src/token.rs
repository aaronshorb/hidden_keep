//! Token data, drawing, fading, and next-token selection.

use crate::config::{TOKEN_DRAW_OFFSET, TOKEN_RADIUS, TOKEN_SIZE};
use crate::player::Player;
use lerp::Lerp;
use macroquad::prelude::*;

#[derive(PartialEq)]
pub struct Token {
    pub tokenpiece: Circle,
    item_color: Color,
    token_pos: Vec2,
    texture: Texture2D,
}

impl Token {
    pub fn new(item_color: Color, token_pos: Vec2, texture: Texture2D) -> Self {
        let tokenpiece = Circle::new(token_pos.x, token_pos.y, TOKEN_RADIUS);
        Token {
            item_color,
            token_pos,
            tokenpiece,
            texture,
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.token_pos.x - TOKEN_DRAW_OFFSET,
            self.token_pos.y - TOKEN_DRAW_OFFSET,
            self.item_color,
            DrawTextureParams {
                dest_size: Some(vec2(TOKEN_SIZE, TOKEN_SIZE)),
                ..Default::default()
            },
        );
    }

    pub fn fade_in_token(&mut self, delta: f32) {
        self.item_color.a = Lerp::lerp(self.item_color.a, 1.0, delta * 1.5);
        if self.item_color.a >= 0.9 {
            self.item_color.a = 1.0;
        }
    }

    pub fn fade_out_token(&mut self, delta: f32) -> bool {
        self.item_color.a = Lerp::lerp(self.item_color.a, 0.0, delta * 2.3);
        if self.item_color.a <= 0.4 {
            self.item_color.a = 0.0;
        }
        self.item_color.a <= 0.0
    }
}

//function for adding tokens to gameboard
//if no token on board, check if first token position is the same as any gamepiece on board.
//If same position, send token position to back of vector, shift all token positions left, and check the new first token position.
//If not same as any gamepiece, add token to board.
pub fn next_token(
    players: &[Player],
    token_list: &mut Vec<(Vec2, Texture2D)>,
    item_color: Color,
) -> Option<Token> {
    loop {
        let token_from_front = token_list.remove(0);
        let new_token = Token::new(item_color, token_from_front.0, token_from_front.1.clone());
        let mut overlapping = false;

        for player in players {
            if new_token.tokenpiece.overlaps(&player.gamepiece) {
                overlapping = true;
                token_list.push(token_from_front);
                break;
            }
        }
        if !overlapping {
            return Some(new_token);
        }
    }
}
