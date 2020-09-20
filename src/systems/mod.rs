mod add_squares;
mod move_blocks;
mod remove_lines;
mod spawn_blocks;

pub use self::{
    add_squares::AddSquaresSystem, move_blocks::MoveBlocksSystem, remove_lines::RemoveLinesSystem,
    spawn_blocks::SpawnBlocksSystem,
};
