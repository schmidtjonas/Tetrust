extern crate rand;
use rand::Rng;

use crate::entities::Position;
use amethyst::ecs::{Component, DenseVecStorage};

pub const BLOCK_COUNT: usize = 7;
pub const SQUARES_IN_BLOCK: usize = 3;

#[derive(Debug, Clone)]
pub struct Block {
    pub square_offsets: Vec<Position>,
    pub time_since_move: f32,
    pub color_index: usize,
}

impl Block {
    pub fn from_vec(square_offsets: Vec<Position>, color_index: usize) -> Self {
        Self {
            square_offsets,
            time_since_move: 0.0,
            color_index,
        }
    }

    pub fn from_color_index(color_index: usize) -> Block {
        // { 'I' => 0, 'J' => 1, 'L' => 2, 'O' => 3, 'S' => 4, 'T' => 5, 'Z' => 6 }
        let square_offsets = match color_index {
            0 => vec![
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(1, 2),
                Position::new(1, 3),
            ],
            1 => vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(1, 2),
            ],
            2 => vec![
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(1, 2),
                Position::new(0, 2),
            ],
            3 => vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(1, 0),
                Position::new(1, 1),
            ],
            4 => vec![
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(0, 1),
                Position::new(0, 2),
            ],
            5 => vec![
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(0, 1),
                Position::new(1, 2),
            ],
            6 => vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(1, 2),
            ],
            _ => vec![],
        };

        Self::from_vec(square_offsets, color_index)
    }

    pub fn rand() -> Self {
        Self::from_color_index(rand::thread_rng().gen_range(0, 7))
    }

    pub fn rotate(&mut self, rotation: i32) {
        let rotation = (rotation % 4 + 4) % 4;
        for mut square_offset in &mut self.square_offsets {
            for _ in 0..rotation {
                let Position { row, col } = square_offset.clone();
                square_offset.row = col;
                square_offset.col = SQUARES_IN_BLOCK as i8 - 1 - row;
            }
        }
    }

    pub fn square_positions(&self, position: &Position) -> Vec<Position> {
        self.square_offsets
            .iter()
            .map(|offset| position + offset)
            .collect()
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}
