use amethyst::ecs::{Component, DenseVecStorage};

use crate::entities::{Block, Position};

pub const BOARD_HEIGHT: usize = 16;
pub const BOARD_WIDTH: usize = 10;

pub struct Board {
    pub rows: usize,
    pub cols: usize,
    filled_squares: Vec<Vec<bool>>,
}

impl Component for Board {
    type Storage = DenseVecStorage<Self>;
}

impl Board {
    pub fn square_at(&self, position: &Position) -> bool {
        assert!(self.is_inside(position));
        self.filled_squares[position.row as usize][position.col as usize]
    }

    pub fn is_inside(&self, position: &Position) -> bool {
        position.row >= 0
            && position.row < self.rows as i8
            && position.col >= 0
            && position.col < self.cols as i8
    }

    pub fn is_free(&self, position: &Position) -> bool {
        self.is_inside(position) && !self.square_at(position)
    }

    pub fn add_square(&mut self, position: &Position) {
        assert!(self.is_inside(position));
        self.filled_squares[position.row as usize][position.col as usize] = true;
    }

    pub fn start_position(&self) -> Position {
        Position::new(0, (self.cols / 2) as i8)
    }

    pub fn block_can_move_to(&self, block: &Block, position: &Position) -> bool {
        for square_position in &block.square_positions(position) {
            if !self.is_free(square_position) {
                return false;
            }
        }
        true
    }
    pub fn block_can_rotate_right(&self, block: &Block, position: &Position) -> bool {
        let mut block = block.clone();
        block.rotate(1);
        for square_position in &block.square_positions(position) {
            if !self.is_free(square_position) {
                return false;
            }
        }
        true
    }

    pub fn place_block_at(&mut self, block: &Block, position: &Position) {
        for square_position in &block.square_positions(position) {
            self.add_square(&square_position);
        }
    }

    pub fn is_game_over(&self) -> bool {
        !self.is_free(&self.start_position())
    }

    pub fn row_is_full(&self, row: usize) -> bool {
        for square in &self.filled_squares[row] {
            if !square {
                return false;
            }
        }
        true
    }

    pub fn full_rows_below(&self) -> Vec<usize> {
        let mut result = vec![0; BOARD_HEIGHT];
        for row in (0..self.rows - 1).rev() {
            result[row] = result[row + 1];
            if self.row_is_full(row + 1) {
                result[row] += 1;
            }
        }
        result
    }

    pub fn full_rows(&self) -> Vec<bool> {
        let mut result = vec![false; BOARD_HEIGHT];
        for row in 0..self.rows {
            result[row] |= self.row_is_full(row);
        }
        result
    }

    pub fn remove_full_rows(&mut self) {
        let mut cur_row = self.rows - 1;
        for row in (0..self.rows).rev() {
            if !self.row_is_full(row) {
                self.filled_squares[cur_row] = self.filled_squares[row].clone();
                cur_row -= 1;
            }
        }
        for row in 0..cur_row + 1 {
            self.filled_squares[row] = vec![false; BOARD_WIDTH];
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            rows: BOARD_HEIGHT,
            cols: BOARD_WIDTH,
            filled_squares: vec![vec![false; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }
}
