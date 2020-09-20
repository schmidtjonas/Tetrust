use crate::entities::Position;
use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Square {
    pub position: Position,
    pub color_index: usize,
}

impl Component for Square {
    type Storage = DenseVecStorage<Self>;
}
