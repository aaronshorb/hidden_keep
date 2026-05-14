//! Wall placement, removal, hover preview, board accessibility checks, and hidden board rotation.

use crate::config::{
    CELL_STRIDE_I32, GAMEBOARD_SIZE, LAST_WALL_OFFSET_I32, MAX_WALLS, PADDING, SQUARE_SIZE,
    SQUARE_SIZE_I32, WALL_ROW_COUNT, WALL_SLOT_COUNT,
};
use crate::wall::{Wall, WallOrientation};
use ::rand::prelude::*;
use macroquad::prelude::*;
use std::collections::{HashSet, VecDeque};

type BoardCell = (i32, i32);

//state used while players are placing walls before the hidden board rotation
pub struct WallPlacementState {
    pub walls_left: usize,
    pub walls_on_board: HashSet<Wall>,
    current_wall: Option<Wall>,
    horizontal_wall_values: [(i32, i32); WALL_SLOT_COUNT],
    vertical_wall_values: [(i32, i32); WALL_SLOT_COUNT],
}

impl WallPlacementState {
    pub fn new(max_walls: usize) -> Self {
        let mut horizontal_walls = [(0, 0); WALL_SLOT_COUNT];
        let mut vertical_walls = [(0, 0); WALL_SLOT_COUNT];

        for i in 0..WALL_ROW_COUNT {
            for j in 0..GAMEBOARD_SIZE {
                horizontal_walls[(i * GAMEBOARD_SIZE + j) as usize] =
                    (j * CELL_STRIDE_I32, SQUARE_SIZE_I32 + i * CELL_STRIDE_I32);
                vertical_walls[(i * GAMEBOARD_SIZE + j) as usize] = (
                    LAST_WALL_OFFSET_I32 - (i * CELL_STRIDE_I32),
                    j * CELL_STRIDE_I32,
                );
            }
        }

        Self {
            walls_left: max_walls,
            walls_on_board: HashSet::new(),
            current_wall: None,
            horizontal_wall_values: horizontal_walls,
            vertical_wall_values: vertical_walls,
        }
    }

    pub fn reset(&mut self, max_walls: usize) {
        self.walls_on_board.clear();
        self.walls_left = max_walls;
        self.current_wall = None;
    }

    pub fn update_placement(&mut self) {
        self.wall_set();
    }

    //function for randomizing the gameboard rotation
    pub fn randomize_board(&mut self, rng: &mut ThreadRng) {
        let rotation = rng.gen_range(0..4);
        let rotated_walls: HashSet<Wall> = self
            .walls_on_board
            .iter()
            .copied()
            .filter_map(|wall| self.rotated_wall(wall, rotation))
            .collect();

        self.walls_on_board = rotated_walls;
    }

    fn rotated_wall(&self, wall: Wall, rotation: usize) -> Option<Wall> {
        let index = self.wall_slot_index(wall)?;
        let reverse_index = WALL_SLOT_COUNT - index - 1;

        let rotated_wall = match (rotation, wall.orientation) {
            (0, _) => wall,
            (1, WallOrientation::Horizontal) => self.wall_at(WallOrientation::Vertical, index),
            (1, WallOrientation::Vertical) => {
                self.wall_at(WallOrientation::Horizontal, reverse_index)
            }
            (2, _) => self.wall_at(wall.orientation, reverse_index),
            (3, WallOrientation::Horizontal) => {
                self.wall_at(WallOrientation::Vertical, reverse_index)
            }
            (3, WallOrientation::Vertical) => self.wall_at(WallOrientation::Horizontal, index),
            _ => return None,
        };

        Some(rotated_wall)
    }

    fn wall_slot_index(&self, wall: Wall) -> Option<usize> {
        self.wall_values(wall.orientation)
            .iter()
            .position(|&wall_pos| wall_pos == (wall.x, wall.y))
    }

    fn wall_at(&self, orientation: WallOrientation, index: usize) -> Wall {
        let (x, y) = self.wall_values(orientation)[index];
        Wall::new(x, y, orientation)
    }

    fn wall_values(&self, orientation: WallOrientation) -> &[(i32, i32); WALL_SLOT_COUNT] {
        match orientation {
            WallOrientation::Horizontal => &self.horizontal_wall_values,
            WallOrientation::Vertical => &self.vertical_wall_values,
        }
    }

    fn try_insert_wall(&mut self, wall: Wall) {
        if self.walls_on_board.len() >= MAX_WALLS {
            return;
        }

        //temporarily add the wall so the BFS can validate the board with it in place
        if self.walls_on_board.insert(wall) {
            if self.board_is_fully_accessible() {
                self.walls_left -= 1;
            } else {
                self.walls_on_board.remove(&wall);
            }
        }
    }

    fn board_is_fully_accessible(&self) -> bool {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let start = (0, 0);

        //BFS starts in the top-left cell and expands through unblocked neighbors
        visited.insert(start);
        queue.push_back(start);

        while let Some(cell) = queue.pop_front() {
            for neighbor in self.open_neighbors(cell) {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        //if every cell was visited, the walls have not cut off any board area
        visited.len() == (GAMEBOARD_SIZE * GAMEBOARD_SIZE) as usize
    }

    fn open_neighbors(&self, cell: BoardCell) -> Vec<BoardCell> {
        let (row, col) = cell;
        //candidate moves are the four neighboring cells on the grid
        let candidates = [
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ];

        candidates
            .into_iter()
            .filter(|&neighbor| {
                Self::cell_in_bounds(neighbor) && !self.wall_blocks_between(cell, neighbor)
            })
            .collect()
    }

    fn cell_in_bounds((row, col): BoardCell) -> bool {
        (0..GAMEBOARD_SIZE).contains(&row) && (0..GAMEBOARD_SIZE).contains(&col)
    }

    fn wall_blocks_between(&self, from: BoardCell, to: BoardCell) -> bool {
        self.wall_between(from, to)
            .is_some_and(|wall| self.walls_on_board.contains(&wall))
    }

    fn wall_between(&self, from: BoardCell, to: BoardCell) -> Option<Wall> {
        let row_delta = from.0 - to.0;
        let col_delta = from.1 - to.1;

        if row_delta.abs() + col_delta.abs() != 1 {
            return None;
        }

        //convert movement between two adjacent cells into the wall slot that blocks it
        if row_delta != 0 {
            let upper_row = from.0.min(to.0);
            Some(Wall::new(
                from.1 * CELL_STRIDE_I32,
                SQUARE_SIZE_I32 + upper_row * CELL_STRIDE_I32,
                WallOrientation::Horizontal,
            ))
        } else {
            let left_col = from.1.min(to.1);
            Some(Wall::new(
                SQUARE_SIZE_I32 + left_col * CELL_STRIDE_I32,
                from.0 * CELL_STRIDE_I32,
                WallOrientation::Vertical,
            ))
        }
    }

    fn wall_set(&mut self) {
        //get the mouse position on screen
        let (mouse_x, mouse_y) = mouse_position();

        self.current_wall = None;

        //if cursor hovering over any possible wall, set the current wall and draw on screen
        for (wall_x, wall_y) in self.horizontal_wall_values {
            if mouse_x > wall_x as f32
                && mouse_x < wall_x as f32 + SQUARE_SIZE
                && mouse_y > wall_y as f32
                && mouse_y < wall_y as f32 + PADDING
            {
                self.current_wall = Some(Wall::new(wall_x, wall_y, WallOrientation::Horizontal));
                break;
            }
        }
        for (wall_x, wall_y) in self.vertical_wall_values {
            if mouse_x > wall_x as f32
                && mouse_x < wall_x as f32 + PADDING
                && mouse_y > wall_y as f32
                && mouse_y < wall_y as f32 + SQUARE_SIZE
            {
                self.current_wall = Some(Wall::new(wall_x, wall_y, WallOrientation::Vertical));
                break;
            }
        }

        //if current_wall is not None, if left mouse key pressed, the hovered wall will be inserted.
        //if right mouse button pressed, and a wall exists at the hovered space, the wall will be deleted.
        if let Some(wall) = &self.current_wall {
            wall.draw(ORANGE);

            if is_mouse_button_pressed(MouseButton::Left) {
                self.try_insert_wall(*wall);
            } else if is_mouse_button_pressed(MouseButton::Right)
                && self.walls_on_board.remove(wall)
            {
                self.walls_left += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_board_is_fully_accessible() {
        let state = WallPlacementState::new(MAX_WALLS);

        assert!(state.board_is_fully_accessible());
    }

    #[test]
    fn rejects_wall_that_would_trap_corner_cell() {
        let mut state = WallPlacementState::new(MAX_WALLS);
        let right_of_corner = Wall::new(SQUARE_SIZE_I32, 0, WallOrientation::Vertical);
        let below_corner = Wall::new(0, SQUARE_SIZE_I32, WallOrientation::Horizontal);

        state.try_insert_wall(right_of_corner);
        state.try_insert_wall(below_corner);

        assert!(state.walls_on_board.contains(&right_of_corner));
        assert!(!state.walls_on_board.contains(&below_corner));
        assert_eq!(state.walls_left, MAX_WALLS - 1);
    }
}
