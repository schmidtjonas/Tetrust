use amethyst::{
    assets::Handle,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Entities, ReadExpect, ReaderId, System, SystemData, Write, WriteStorage},
    renderer::{SpriteRender, SpriteSheet},
    shrev::EventChannel,
};

use crate::{
    entities::Block,
    events::BlockLandEvent,
    tetris::{ARENA_HEIGHT, ARENA_WIDTH},
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
        Entities<'s>,
        ReadExpect<'s, Handle<SpriteSheet>>,
        WriteStorage<'s, SpriteRender>,
        Write<'s, EventChannel<BlockLandEvent>>,
    );

    fn run(
        &mut self,
        (
            mut blocks,
            mut transforms,
            entities,
            sprite_sheet_handle,
            mut sprite_renders,
            mut land_channel,
        ): Self::SystemData,
    ) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| land_channel.register_reader());

        for _ in land_channel.read(reader_id) {
            let mut transform = Transform::default();
            transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT, 0.0);

            let block = Block::rand();
            let color_index = block.color_index;

            entities
                .build_entity()
                .with(block, &mut blocks)
                .with(
                    SpriteRender::new(sprite_sheet_handle.clone(), color_index),
                    &mut sprite_renders,
                )
                .with(transform, &mut transforms)
                .build();
        }
    }
}
