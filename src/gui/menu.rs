//! Start, wall setup, and endgame menu widgets.

use crate::game_data::PostGameChoice;
use macroquad::{
    math::{vec2, Vec2},
    ui::{hash, root_ui},
};

pub fn start_menu(num_players: &mut usize, tokens_to_win: &mut usize, set_state: &mut bool) {
    //text and buttons for pre-game menu
    root_ui().window(hash!(), vec2(385.0, 260.0), vec2(630.0, 630.0), |ui| {
        ui.label(
            vec2(50.0, 75.0),
            &format!("Number of players: {}", num_players),
        );
        ui.label(
            vec2(100.0, 235.0),
            &format!("Tokens to win: {}", tokens_to_win),
        );

        if ui.button(vec2(110.0, 405.0), "Start Game") {
            *set_state = true;
        };
        //buttons for modifying num_players
        if ui.button(vec2(90.0, 130.0), "2") {
            *num_players = 2;
        }
        if ui.button(vec2(222.5, 130.0), "3") {
            *num_players = 3;
        }
        if ui.button(vec2(355.0, 130.0), "4") {
            *num_players = 4;
        }
        //buttons for modifying tokens_to_win
        if ui.button(vec2(45.0, 295.0), "1") {
            *tokens_to_win = 1;
        }
        if ui.button(vec2(133.75, 295.0), "2") {
            *tokens_to_win = 2;
        }
        if ui.button(vec2(222.5, 295.0), "3") {
            *tokens_to_win = 3;
        }
        if ui.button(vec2(311.25, 295.0), "4") {
            *tokens_to_win = 4;
        }
        if ui.button(vec2(400.0, 295.0), "5") {
            *tokens_to_win = 5;
        }
    });
}

pub async fn set_walls_menu(element_color_change: &bool, set_state: &mut bool) {
    //button for setting the walls on gameboard
    if root_ui().button(Vec2::new(1060.0, 725.0), "Set Walls") && !element_color_change {
        *set_state = true;
    }
}

pub async fn end_menu(p_name: &str, post_game_choice: &mut PostGameChoice) {
    let x_pos = 185.0 - p_name.len() as f32 * 10.0;
    root_ui().window(hash!(), vec2(382.5, 315.0), vec2(635.0, 370.0), |ui| {
        ui.label(vec2(x_pos, 70.0), &format!("{} wins!", p_name));

        if ui.button(vec2(40.0, 145.0), "Play Again") {
            *post_game_choice = PostGameChoice::PlayAgain;
        };
        if ui.button(vec2(345.0, 145.0), "Quit") {
            *post_game_choice = PostGameChoice::Quit;
        };
    });
}
