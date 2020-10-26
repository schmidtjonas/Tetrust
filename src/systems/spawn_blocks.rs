use std::process;

use amethyst::{
    assets::Handle,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Entities, ReadExpect, ReaderId, System, SystemData, Write, WriteStorage},
    renderer::{SpriteRender, SpriteSheet},
    shrev::EventChannel,
};

use crate::{
    entities::{Block, Board, Position},
    events::BlockLandEvent,
};

#[derive(SystemDesc)]
pub struct SpawnBlocksSystem {
    reader_id: Option<ReaderId<BlockLandEvent>>,
}

impl SpawnBlocksSystem {
    pub fn new() -> Self {
        Self { reader_id: None }
    }
}

impl<'s> System<'s> for SpawnBlocksSystem {
    type SystemData = (
        WriteStorage<'s, Block>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Position>,
        Entities<'s>,
        ReadExpect<'s, Handle<SpriteSheet>>,
        WriteStorage<'s, SpriteRender>,
        Write<'s, EventChannel<BlockLandEvent>>,
        ReadExpect<'s, Board>,
    );

    fn run(
        &mut self,
        (
            mut blocks,
            mut transforms,
            mut positions,
            entities,
            sprite_sheet_handle,
            mut sprite_renders,
            mut land_channel,
            board,
        ): Self::SystemData,
    ) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| land_channel.register_reader());

        for _ in land_channel.read(reader_id) {
            if board.is_game_over() {
                println!("Game Over!");
                process::exit(0);
            }

            let block = Block::rand();
            let color_index = block.color_index;
            let position = board.start_position();
            let (x, y) = position.coordinates(block.width as f32);

            let mut transform = Transform::default();
            transform.set_translation_xyz(x, y, 0.0);

            entities
                .build_entity()
                .with(block, &mut blocks)
                .with(transform, &mut transforms)
                .with(
                    SpriteRender::new(sprite_sheet_handle.clone(), color_index),
                    &mut sprite_renders,
                )
                .with(board.start_position(), &mut positions)
                .build();
        }
    }
}
