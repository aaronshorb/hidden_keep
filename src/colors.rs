//! Runtime color state and fade helpers for game transitions.

use lerp::Lerp;
use macroquad::color::Color;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorField {
    Brick,
    Pillar,
    Gameboard,
    Wall,
    Title,
    Text,
    Gem,
    PlayerText(usize),
}

pub struct GameColors {
    pub wall_color: Color,
    pub title_color: Color,
    pub text_color: Color,
    pub gameboard_color: Color,
    pub pillar_color: Color,
    pub player_text_colors: [Color; 4],
    pub gem_color: Color,
    pub static_item_color: Color,
    pub brick_color: Color,
}

impl GameColors {
    pub fn new() -> Self {
        let wall_color: Color = Color {
            r: 0.6,
            g: 0.3,
            b: 0.0,
            a: 1.0,
        }; //BROWN
        let title_color: Color = Color {
            r: 0.875,
            g: 0.329,
            b: 0.275,
            a: 1.0,
        }; //ORANGE
        let text_color: Color = Color {
            r: 0.5,
            g: 0.0,
            b: 0.5,
            a: 1.0,
        }; //PURPLE
        let gameboard_color: Color = Color {
            r: 0.0,
            g: 0.46,
            b: 0.17,
            a: 0.0,
        }; //DARKGREEN
        let pillar_color: Color = Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
        }; //WHITE
        let player_text_colors: [Color; 4] = [
            Color {
                r: 0.702,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }, //RED
            Color {
                r: 0.0,
                g: 0.392,
                b: 0.0,
                a: 0.0,
            }, //DARKGREEN
            Color {
                r: 1.0,
                g: 0.843,
                b: 0.0,
                a: 0.0,
            }, //YELLOW
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.702,
                a: 0.0,
            }, //BLUE
        ];
        let gem_color: Color = Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
        };
        let static_item_color: Color = Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.18,
        };
        let brick_color: Color = Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
        };
        GameColors {
            wall_color,
            title_color,
            text_color,
            gameboard_color,
            pillar_color,
            player_text_colors,
            gem_color,
            static_item_color,
            brick_color,
        }
    }

    fn color_mut(&mut self, field: ColorField) -> Option<&mut Color> {
        match field {
            ColorField::Brick => Some(&mut self.brick_color),
            ColorField::Pillar => Some(&mut self.pillar_color),
            ColorField::Gameboard => Some(&mut self.gameboard_color),
            ColorField::Wall => Some(&mut self.wall_color),
            ColorField::Title => Some(&mut self.title_color),
            ColorField::Text => Some(&mut self.text_color),
            ColorField::Gem => Some(&mut self.gem_color),
            ColorField::PlayerText(index) => self.player_text_colors.get_mut(index),
        }
    }

    fn fade_color(
        &mut self,
        field: ColorField,
        target_alpha: f32,
        snap_alpha: f32,
        should_snap: fn(f32) -> bool,
        color_change: &mut bool,
        delta: f32,
    ) {
        let Some(color) = self.color_mut(field) else {
            return;
        };

        color.a = Lerp::lerp(color.a, target_alpha, delta);
        if should_snap(color.a) {
            color.a = snap_alpha;
            *color_change = false;
        } else {
            *color_change = true;
        }
    }

    //fade out function
    pub fn lerp_color_out(&mut self, field: ColorField, color_change: &mut bool, delta: f32) {
        self.fade_color(field, 0.0, 0.0, |alpha| alpha <= 0.1, color_change, delta);
    }

    //fade in function
    pub fn lerp_color_in(&mut self, field: ColorField, color_change: &mut bool, delta: f32) {
        self.fade_color(field, 1.0, 1.0, |alpha| alpha >= 0.9, color_change, delta);
    }
}
