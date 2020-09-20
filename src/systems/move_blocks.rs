use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, System, SystemData, Write, WriteExpect, WriteStorage},
    input::{InputHandler, StringBindings},
    shrev::EventChannel,
};

use crate::{
    entities::{Block, Board, Position},
    events::BlockLandEvent,
    tetris::{BLOCK_SIZE, MOVE_TIME},
};

#[derive(SystemDesc)]
pub struct MoveBlocksSystem {
    pub left: bool,
    pub right: bool,
}

impl MoveBlocksSystem {
    pub fn new() -> Self {
        Self {
            left: false,
            right: false,
        }
    }
}

impl<'s> System<'s> for MoveBlocksSystem {
    type SystemData = (
        WriteStorage<'s, Block>,
        WriteStorage<'s, Position>,
        WriteStorage<'s, Transform>,
        WriteExpect<'s, Board>,
        Entities<'s>,
        Read<'s, Time>,
        Write<'s, EventChannel<BlockLandEvent>>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (
            mut blocks,
            mut positions,
            mut transforms,
            mut board,
            entities,
            time,
            mut land_channel,
            input,
        ): Self::SystemData,
    ) {
        for (entity, block, position, transform) in
            (&*entities, &mut blocks, &mut positions, &mut transforms).join()
        {
            let passed_time = time.delta_seconds();
            block.time_since_move += passed_time;
            if input.action_is_down("down").unwrap_or(false) {
                block.time_since_move += passed_time * 3.0;
            }

            if input.action_is_down("right").unwrap_or(false) && !self.right {
                if board.block_can_move_to(block, &(position.clone() + Position::new(0, 1))) {
                    println!("move right from {:?}", position);
                    transform.prepend_translation_x(BLOCK_SIZE);
                    position.col += 1;
                }
            }
            if input.action_is_down("left").unwrap_or(false) && !self.left {
                if board.block_can_move_to(block, &(position.clone() + Position::new(0, -1))) {
                    println!("move left from {:?}", position);
                    transform.prepend_translation_x(-BLOCK_SIZE);
                    position.col -= 1;
                }
            }
            self.right = input.action_is_down("right").unwrap_or(false);
            self.left = input.action_is_down("left").unwrap_or(false);

            if block.time_since_move >= MOVE_TIME {
                if board.block_can_move_to(block, &(position.clone() + Position::new(1, 0))) {
                    println!("move down from {:?}", position);
                    transform.prepend_translation_y(-BLOCK_SIZE);
                    block.time_since_move -= MOVE_TIME;
                    position.row += 1;
                } else {
                    land_channel.single_write(BlockLandEvent {
                        block: block.clone(),
                        position: *position,
                    });
                    entities.delete(entity).unwrap();
                    board.place_block_at(block, position);
                }
            }
        }
    }
}
