use amethyst::ecs::{Component, DenseVecStorage};
use std::ops::{Add, Sub};

use crate::tetris::{ARENA_HEIGHT, ARENA_WIDTH, BLOCK_SIZE};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}

impl Position {
    pub fn new(row: i8, col: i8) -> Self {
        Self { row, col }
    }

    pub fn coordinates(&self, is_block: bool) -> (f32, f32) {
        (
            self.col as f32 * BLOCK_SIZE + BLOCK_SIZE * if is_block { 2.0 } else { 0.5 },
            ARENA_HEIGHT
                - self.row as f32 * BLOCK_SIZE
                - BLOCK_SIZE * if is_block { 1.0 } else { 0.5 },
        )
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl<'a, 'b> Add<&'b Position> for &'a Position {
    type Output = Position;

    fn add(self, other: &'b Position) -> Position {
        Position {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl<'a, 'b> Sub<&'b Position> for &'a Position {
    type Output = Position;

    fn sub(self, other: &'b Position) -> Position {
        Position {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}
