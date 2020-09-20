use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    shrev::EventChannel,
};

use crate::{
    entities::Block,
    events::BlockLandEvent,
    tetris::{BLOCK_SIZE, MOVE_TIME},
};

#[derive(SystemDesc)]
pub struct MoveBlocksSystem {
    pub left: bool,
    pub right: bool,
}

impl<'s> System<'s> for MoveBlocksSystem {
    type SystemData = (
        WriteStorage<'s, Block>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        Read<'s, Time>,
        Write<'s, EventChannel<BlockLandEvent>>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (mut blocks, mut transforms, entities, time, mut land_channel, input): Self::SystemData,
    ) {
        for (entity, block, transform) in (&*entities, &mut blocks, &mut transforms).join() {
            let passed_time = time.delta_seconds();
            block.time_since_move += passed_time;
            if input.action_is_down("down").unwrap_or(false) {
                block.time_since_move += passed_time * 3.0;
            }

            if input.action_is_down("right").unwrap_or(false) && !self.right {
                transform.prepend_translation_x(BLOCK_SIZE);
            }
            if input.action_is_down("left").unwrap_or(false) && !self.left {
                transform.prepend_translation_x(-BLOCK_SIZE);
            }
            self.right = input.action_is_down("right").unwrap_or(false);
            self.left = input.action_is_down("left").unwrap_or(false);

            if block.time_since_move >= MOVE_TIME {
                transform.prepend_translation_y(-BLOCK_SIZE);
                block.time_since_move -= MOVE_TIME;
            }

            if transform.translation().y < 2.0 * BLOCK_SIZE {
                land_channel.single_write(BlockLandEvent);
                entities.delete(entity).unwrap();
            }
        }
    }
}
