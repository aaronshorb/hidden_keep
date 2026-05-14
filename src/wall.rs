//! Wall model, orientation, drawing, and wall/player collision checks.

use crate::config::{PADDING_I32, SQUARE_SIZE_I32};
use macroquad::prelude::*;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum WallOrientation {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub orientation: WallOrientation,
}

impl Wall {
    pub fn new(x: i32, y: i32, orientation: WallOrientation) -> Self {
        let (w, h) = match orientation {
            WallOrientation::Horizontal => (SQUARE_SIZE_I32, PADDING_I32),
            WallOrientation::Vertical => (PADDING_I32, SQUARE_SIZE_I32),
        };
        Wall {
            x,
            y,
            w,
            h,
            orientation,
        }
    }

    pub fn draw(&self, color: Color) {
        draw_rectangle(
            self.x as f32,
            self.y as f32,
            self.w as f32,
            self.h as f32,
            color,
        );
    }

    //from macroquad::math::Rect struct
    fn center(&self) -> Vec2 {
        vec2(
            self.x as f32 + self.w as f32 * 0.5f32,
            self.y as f32 + self.h as f32 * 0.5f32,
        )
    }

    //adapted from macroquad::math::Circle struct
    pub fn overlaps_circle(&self, circle: &Circle) -> bool {
        let dist_x = (circle.x - self.center().x).abs();
        let dist_y = (circle.y - self.center().y).abs();
        if dist_x > self.w as f32 / 2.0 + circle.r || dist_y > self.h as f32 / 2.0 + circle.r {
            return false;
        }
        if dist_x <= self.w as f32 / 2.0 || dist_y <= self.h as f32 / 2.0 {
            return true;
        }
        let lhs = dist_x - self.w as f32 / 2.0;
        let rhs = dist_y - self.h as f32 / 2.0;
        let dist_sq = (lhs * lhs) + (rhs * rhs);
        dist_sq <= circle.r * circle.r
    }
}
