//! Shared constants for window setup, board geometry, rendering, movement, and gameplay values.

use macroquad::prelude::*;

pub fn conf() -> Conf {
    Conf {
        window_title: "Hidden Keep".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

//Window configuration used by macroquad when creating the app window.
pub const WINDOW_WIDTH: i32 = 1400;
pub const WINDOW_HEIGHT: i32 = 1000;

//Board grid counts. These describe the 6x6 playable grid and its wall slots.
pub const GAMEBOARD_SIZE: i32 = 6;
pub const WALL_SLOT_COUNT: usize = 30;
pub const WALL_ROW_COUNT: i32 = GAMEBOARD_SIZE - 1;

//board geometry used where wall positions are stored as grid coordinates.
pub const SQUARE_SIZE_I32: i32 = 150;
pub const PADDING_I32: i32 = 20;
pub const CELL_STRIDE_I32: i32 = SQUARE_SIZE_I32 + PADDING_I32;
pub const BOARD_SIZE_I32: i32 = GAMEBOARD_SIZE * SQUARE_SIZE_I32 + WALL_ROW_COUNT * PADDING_I32;
pub const LAST_WALL_OFFSET_I32: i32 = BOARD_SIZE_I32 - CELL_STRIDE_I32;

//Board geometry used by rendering, movement, and collision.
pub const SQUARE_SIZE: f32 = SQUARE_SIZE_I32 as f32;
pub const PADDING: f32 = PADDING_I32 as f32;
pub const CELL_STRIDE: f32 = CELL_STRIDE_I32 as f32;
pub const BOARD_SIZE: f32 = BOARD_SIZE_I32 as f32;
pub const CELL_CENTER_OFFSET: f32 = SQUARE_SIZE / 2.0;
pub const BOARD_MIN_CENTER: f32 = CELL_CENTER_OFFSET;
pub const BOARD_MAX_CENTER: f32 = BOARD_SIZE - CELL_CENTER_OFFSET;

//Board rendering sizes for the background image, grid outline, and pillars.
pub const BACKGROUND_WIDTH: f32 = 1400.0;
pub const BACKGROUND_HEIGHT: f32 = BOARD_SIZE;
pub const BOARD_LINE_THICKNESS: f32 = 10.0;

pub const PILLAR_CENTER_OFFSET: f32 = SQUARE_SIZE + PADDING / 2.0;
pub const PILLAR_SIZE: f32 = 54.0;
pub const PILLAR_DRAW_OFFSET: f32 = PILLAR_SIZE / 2.0;

//Token rendering and collision geometry.
pub const TOKEN_SIZE: f32 = 80.0;
pub const TOKEN_DRAW_OFFSET: f32 = TOKEN_SIZE / 2.0;
pub const TOKEN_RADIUS: f32 = 30.0;

//Player rendering, collision, movement, and sprite geometry.
pub const PLAYER_RADIUS: f32 = 50.0;
pub const PLAYER_DRAW_OFFSET_X: f32 = 100.0;
pub const PLAYER_DRAW_OFFSET_Y: f32 = 150.0;
pub const PLAYER_MOVE_SPEED: f32 = 140.0;

pub const SPRITE_SIZE: u32 = 128;
pub const SPRITE_SCALE: f32 = 1.6;

//Gameplay setup values.
pub const DICE_SIDES: [i32; 6] = [1, 2, 2, 3, 3, 4];
pub const SKIP_POS: [i32; 12] = [0, 1, 4, 5, 6, 11, 24, 29, 30, 31, 34, 35];
pub const INITIAL_POS: [Vec2; 4] = [
    vec2(BOARD_MIN_CENTER, BOARD_MIN_CENTER),
    vec2(BOARD_MAX_CENTER, BOARD_MAX_CENTER),
    vec2(BOARD_MAX_CENTER, BOARD_MIN_CENTER),
    vec2(BOARD_MIN_CENTER, BOARD_MAX_CENTER),
];
pub const MAX_WALLS: usize = 24;
pub const ITEM_COLOR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 0.0,
};
