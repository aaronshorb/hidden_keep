//! Drawing functions for the board, screens, HUD, tokens, players, and walls.

use crate::config::{
    BACKGROUND_HEIGHT, BACKGROUND_WIDTH, BOARD_LINE_THICKNESS, BOARD_SIZE, CELL_STRIDE,
    GAMEBOARD_SIZE, PADDING, PILLAR_CENTER_OFFSET, PILLAR_DRAW_OFFSET, PILLAR_SIZE, SQUARE_SIZE,
    TOKEN_DRAW_OFFSET, TOKEN_SIZE,
};
use crate::game_state::Game;
use crate::Assets;
use macroquad::prelude::*;

pub fn draw_board_background(game: &Game, assets: &Assets) {
    draw_texture_ex(
        &assets.background,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(BACKGROUND_WIDTH, BACKGROUND_HEIGHT)),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    );

    draw_texture_ex(
        &assets.board_back,
        0.0,
        0.0,
        game.colors.brick_color,
        DrawTextureParams {
            dest_size: Some(vec2(BOARD_SIZE, BOARD_SIZE)),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    );

    draw_board_grid(game);
    draw_walls(game);
}

pub fn draw_board_foreground(game: &Game, assets: &Assets) {
    draw_rectangle_lines(
        0.0,
        0.0,
        BOARD_SIZE,
        BOARD_SIZE,
        BOARD_LINE_THICKNESS,
        game.colors.gameboard_color,
    );
    draw_pillars(game, assets);
}

pub fn draw_start_menu(game: &Game, assets: &Assets) {
    draw_text_ex(
        //game title text
        "HIDDEN KEEP",
        245.0,
        180.0,
        TextParams {
            font: Some(&assets.text_font),
            font_size: 130,
            color: game.colors.title_color,
            ..Default::default()
        },
    );
}

pub fn draw_set_walls(game: &Game, assets: &Assets) {
    draw_text_ex(
        //text instructions for inserting walls
        &format!("Walls Left: {}", game.data.wall_placement.walls_left),
        1060.0,
        265.0,
        TextParams {
            font: Some(&assets.text_font),
            font_size: 40,
            color: game.colors.text_color,
            ..Default::default()
        },
    );

    let wall_messages: [(&str, f32); 7] = [
        ("Use mouse to add walls.", 355.0),
        ("Left click to insert a wall.", 405.0),
        ("Right click to delete a wall.", 455.0),
        ("Click 'Set Walls' to set all", 505.0),
        ("walls on board and start game.", 555.0),
        ("Invalid wall placements that", 605.0),
        ("block access are ignored.", 655.0),
    ];

    for (message, y) in wall_messages.iter() {
        draw_text_ex(
            message,
            1012.0,
            *y,
            TextParams {
                font: Some(&assets.text_font),
                font_size: 22,
                color: game.colors.text_color,
                ..Default::default()
            },
        );
    }

    if game.data.set_state {
        for wall in &game.data.wall_placement.walls_on_board {
            wall.draw(game.colors.wall_color);
        }
    }
}

pub fn draw_in_game(game: &Game, assets: &Assets) {
    draw_static_tokens(game);
    draw_active_token(game);
    draw_players(game);
    draw_turn_hud(game, assets);
}

pub fn draw_end_game(game: &Game) {
    draw_players(game);
}

//draw gameboard
fn draw_board_grid(game: &Game) {
    for row in 0..GAMEBOARD_SIZE {
        for col in 0..GAMEBOARD_SIZE {
            let x = col as f32 * CELL_STRIDE;
            let y = row as f32 * CELL_STRIDE;
            draw_rectangle(x, y, SQUARE_SIZE, PADDING, BLANK);
            draw_rectangle_lines(
                x,
                y,
                SQUARE_SIZE,
                SQUARE_SIZE,
                BOARD_LINE_THICKNESS,
                game.colors.gameboard_color,
            );
        }
    }
}

//draw all walls in hashset
fn draw_walls(game: &Game) {
    for wall in &game.data.wall_placement.walls_on_board {
        wall.draw(game.colors.wall_color);
    }
}

//draw pillars
fn draw_pillars(game: &Game, assets: &Assets) {
    for row in 0..GAMEBOARD_SIZE - 1 {
        for col in 0..GAMEBOARD_SIZE - 1 {
            let x = col as f32 * CELL_STRIDE + PILLAR_CENTER_OFFSET;
            let y = row as f32 * CELL_STRIDE + PILLAR_CENTER_OFFSET;
            draw_texture_ex(
                &assets.pillar_icon,
                x - PILLAR_DRAW_OFFSET,
                y - PILLAR_DRAW_OFFSET,
                game.colors.pillar_color,
                DrawTextureParams {
                    dest_size: Some(vec2(PILLAR_SIZE, PILLAR_SIZE)),
                    source: None,
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            );
        }
    }
}

fn draw_static_tokens(game: &Game) {
    for pos in &game.data.static_token_list {
        draw_texture_ex(
            &pos.1,
            pos.0.x - TOKEN_DRAW_OFFSET,
            pos.0.y - TOKEN_DRAW_OFFSET,
            game.colors.static_item_color,
            DrawTextureParams {
                dest_size: Some(vec2(TOKEN_SIZE, TOKEN_SIZE)),
                ..Default::default()
            },
        )
    }
}

//fade in and draw token once all gamepieces are visible
fn draw_active_token(game: &Game) {
    if game.data.gamepieces_visible {
        if let Some(token) = &game.data.token_on_board {
            token.draw();
        }
    }
}

//draw players on the gameboard
fn draw_players(game: &Game) {
    for player in &game.data.players_on_board {
        player.draw();
    }
}

//draw player turn icon and "moves left" text
fn draw_turn_hud(game: &Game, assets: &Assets) {
    let y_pos = 254.0 + (game.data.players_on_board[game.data.player_turn].player_id * 190) as f32;
    draw_texture_ex(
        &assets.gem_icon,
        1022.0,
        y_pos,
        game.colors.gem_color,
        DrawTextureParams {
            dest_size: Some(vec2(33.0, 33.0)),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    );
    draw_text_ex(
        &format!(
            "Moves: {}",
            game.data.players_on_board[game.data.player_turn].num_moves
        ),
        1060.0,
        85.0,
        TextParams {
            font: Some(&assets.text_font),
            font_size: 60,
            color: game.colors.title_color,
            ..Default::default()
        },
    );

    //in-game player and tokens collected text
    let mut player_messages: Vec<(String, f32, usize)> = Vec::new();

    for (index, player) in game.data.players_on_board.iter().enumerate() {
        player_messages.push((
            player.character.name.clone(),
            285.0 + (index * 190) as f32,
            player.player_id,
        ));
        player_messages.push((
            format!("Tokens: {}", player.tokens_collected),
            330.0 + (index * 190) as f32,
            player.player_id,
        ));
    }

    for (index, (message, y, player_id)) in player_messages.iter().enumerate() {
        if index % 2 == 0 {
            draw_text_ex(
                message,
                1075.0,
                *y,
                TextParams {
                    font: Some(&assets.text_font),
                    font_size: 40,
                    color: game.colors.player_text_colors[*player_id],
                    ..Default::default()
                },
            );
        } else {
            draw_text_ex(
                message,
                1130.0,
                *y,
                TextParams {
                    font: Some(&assets.text_font),
                    font_size: 25,
                    color: game.colors.text_color,
                    ..Default::default()
                },
            );
        }
    }
}
