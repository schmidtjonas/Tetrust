extern crate rand;
use rand::Rng;

use crate::entities::{Position, Square};
use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Block {
    pub squares: Vec<Square>,
    pub time_since_move: f32,
    pub color_index: usize,
}

impl Block {
    pub fn from_vec(squares: Vec<Square>, color_index: usize) -> Self {
        Self {
            squares,
            time_since_move: 0.0,
            color_index,
        }
    }

    pub fn from_color_index(color_index: usize) -> Block {
        // { 'I' => 0, 'J' => 1, 'L' => 2, 'O' => 3, 'S' => 4, 'T' => 5, 'Z' => 6 }
        let positions = match color_index {
            3 => vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(1, 0),
                Position::new(1, 1),
            ],
            0 => vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
            ],
            _ => vec![],
        };

        Self::from_vec(
            positions
                .iter()
                .map(|pos| Square { offset: *pos })
                .collect(),
            color_index,
        )
    }

    pub fn rand() -> Self {
        Self::from_color_index(rand::thread_rng().gen_range(0,7))
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}
