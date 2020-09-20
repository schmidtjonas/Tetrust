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

    pub fn place_block_at(&mut self, block: &Block, position: &Position) {
        for square_position in &block.square_positions(position) {
            self.add_square(&square_position);
        }
    }

    pub fn is_full(&self) -> bool {
        !self.is_free(&self.start_position())
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
