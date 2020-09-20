use crate::entities::Position;
use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Square {
    pub offset: Position,
}

impl Component for Square {
    type Storage = DenseVecStorage<Self>;
}
