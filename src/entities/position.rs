use amethyst::ecs::{Component, DenseVecStorage};
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}

impl Position {
    pub fn new(row: i8, col: i8) -> Self {
        Self { row, col }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + Self::new(-other.row, -other.col)
    }
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}
