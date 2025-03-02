use amethyst::ecs::{Component, DenseVecStorage};
use std::ops::{Add, Sub};

use crate::tetris::{ARENA_HEIGHT, BLOCK_SIZE};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub fn coordinates(&self, block_width: f32) -> (f32, f32) {
        (
            self.col as f32 * BLOCK_SIZE + BLOCK_SIZE * block_width / 2.0,
            ARENA_HEIGHT - self.row as f32 * BLOCK_SIZE - BLOCK_SIZE * block_width / 2.0,
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
