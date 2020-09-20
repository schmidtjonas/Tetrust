use amethyst::ecs::{Component, DenseVecStorage};

pub const BOARD_HEIGHT: u8 = 16;
pub const BOARD_WIDTH: u8 = 10;

pub struct Board {
    pub rows: u8,
    pub cols: u8,
}

impl Component for Board {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Board {
    fn default() -> Self {
        Board {
            rows: BOARD_HEIGHT,
            cols: BOARD_WIDTH
        }
    }
}
