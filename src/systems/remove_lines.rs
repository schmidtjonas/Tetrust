use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, System, SystemData, Write, WriteExpect, WriteStorage},
    shrev::{EventChannel, ReaderId},
};

use crate::{
    entities::{Board, Position, Square},
    events::BlockLandEvent,
    tetris::BLOCK_SIZE,
};

pub fn get_score(full_rows: i32) -> i32 {
    match full_rows {
        1 => 40,
        2 => 100,
        3 => 300,
        4 => 1200,
        _ => 0,
    }
}

#[derive(SystemDesc)]
pub struct RemoveLinesSystem {
    reader_id: Option<ReaderId<BlockLandEvent>>,
}

impl RemoveLinesSystem {
    pub fn new() -> Self {
        Self { reader_id: None }
    }
}

impl<'s> System<'s> for RemoveLinesSystem {
    type SystemData = (
        WriteStorage<'s, Square>,
        WriteStorage<'s, Position>,
        WriteStorage<'s, Transform>,
        WriteExpect<'s, Board>,
        Entities<'s>,
        Write<'s, EventChannel<BlockLandEvent>>,
    );

    fn run(
        &mut self,
        (
            mut squares,
            mut positions,
            mut transforms,
            mut board,
            entities,
            mut land_channel,
        ): Self::SystemData,
    ) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| land_channel.register_reader());

        for _ in land_channel.read(reader_id) {
            let full_rows_below = board.full_rows_below();
            let is_full = board.full_rows();
            let full_rows_count = is_full.iter().fold(0, |sum, i| sum + if *i { 1 } else { 0 });
            if full_rows_count == 0 {
                continue;
            }

            for (entity, _, position, transform) in
                (&*entities, &mut squares, &mut positions, &mut transforms).join()
            {
                if is_full[position.row as usize] {
                    entities.delete(entity).unwrap();
                } else {
                    transform.prepend_translation_y(
                        -BLOCK_SIZE * full_rows_below[position.row as usize] as f32,
                    );
                    position.row += full_rows_below[position.row as usize] as i32;
                }
            }

            board.remove_full_rows();
            board.score += get_score(full_rows_count);
        }
    }
}
