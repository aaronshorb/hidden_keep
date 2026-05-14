//! Application entry point, asset setup, main loop, and top-level UI routing.

use ::rand::prelude::*;
use gui::skin::GuiStyle;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
mod assets;
mod colors;
mod config;
mod game_data;
mod game_state;
mod gui;
mod player;
mod render;
mod token;
mod wall;
mod wall_placement;
use crate::assets::Assets;
use crate::config::{conf, INITIAL_POS, MAX_WALLS, SKIP_POS};
use crate::game_data::PostGameChoice;
use game_state::Game;
use game_state::GameState;

#[macroquad::main(conf)]
async fn main() {
    let mut rng = thread_rng();

    let assets = Assets::load().await;

    let mut game = Game::new(INITIAL_POS, MAX_WALLS).await;
    game.data
        .init_tokens(&assets.item_list, &SKIP_POS, &mut rng);
    game.data.set_characters(&assets.characters);

    let skin = GuiStyle::new();
    root_ui().push_skin(&skin.menu_skin);

    loop {
        clear_background(SKYBLUE);
        let delta = get_frame_time();

        render::draw_board_background(&game, &assets);
        game.update(delta, &mut rng, &assets);

        match game.state {
            GameState::StartMenu => {
                gui::menu::start_menu(
                    &mut game.data.num_players,
                    &mut game.data.tokens_to_win,
                    &mut game.data.set_state,
                );
            }

            GameState::SetWalls => {
                gui::menu::set_walls_menu(
                    &game.data.element_color_change,
                    &mut game.data.set_state,
                )
                .await;
            }

            GameState::InGame => {}

            GameState::EndGame => {
                let p_name = &game.data.players_on_board[game.data.player_turn]
                    .character
                    .name;

                if game.colors.gameboard_color.a == 0.0 {
                    gui::menu::end_menu(p_name, &mut game.data.post_game_choice).await;
                }

                if game.data.post_game_choice == PostGameChoice::Quit {
                    break;
                }
            }
        }

        render::draw_board_foreground(&game, &assets);

        for player in game.data.players_on_board.iter_mut() {
            player.update_sprite();
        }

        next_frame().await;
    }
}
