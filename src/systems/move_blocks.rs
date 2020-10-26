use std::f32::consts::PI;
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

pub const INPUT_TIME_ROTATE: f32 = 0.25;
pub const INPUT_TIME_MOVE: f32 = 0.15;

#[derive(SystemDesc)]
pub struct MoveBlocksSystem {
    pub left: f32,
    pub right: f32,
    pub rotate: f32,
}

impl MoveBlocksSystem {
    pub fn new() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            rotate: 0.0,
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
            self.left -= passed_time;
            self.right -= passed_time;
            self.rotate -= passed_time;
            if input.action_is_down("down").unwrap_or(false) {
                block.time_since_move += passed_time * 3.0;
            }

            if input.action_is_down("right").unwrap_or(false) && self.right <= 0.0 {
                self.right += INPUT_TIME_MOVE;
                if board.block_can_move_to(block, &(position.clone() + Position::new(0, 1))) {
                    transform.prepend_translation_x(BLOCK_SIZE);
                    position.col += 1;
                }
            }
            if input.action_is_down("left").unwrap_or(false) && self.left <= 0.0 {
                self.left += INPUT_TIME_MOVE;
                if board.block_can_move_to(block, &(position.clone() + Position::new(0, -1))) {
                    transform.prepend_translation_x(-BLOCK_SIZE);
                    position.col -= 1;
                }
            }
            if input.action_is_down("rotate").unwrap_or(false) && self.rotate <= 0.0 {
                self.rotate += INPUT_TIME_ROTATE;
                if board.block_can_rotate_right(block, &position) {
                    transform.prepend_rotation_z_axis(PI / 2.0);
                    block.rotate(1);
                }
            }
            if !input.action_is_down("right").unwrap_or(false) {
                self.right = 0.0;
            }
            if !input.action_is_down("left").unwrap_or(false) {
                self.left = 0.0;
            }
            if !input.action_is_down("rotate").unwrap_or(false) {
                self.rotate = 0.0;
            }

            if block.time_since_move >= MOVE_TIME {
                if board.block_can_move_to(block, &(position.clone() + Position::new(1, 0))) {
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
