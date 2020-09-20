use amethyst::{
    assets::Handle,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Entities, ReadExpect, System, SystemData, Write, WriteStorage},
    renderer::{SpriteRender, SpriteSheet},
    shrev::{EventChannel, ReaderId},
};

use crate::{
    entities::{Block, Position, Square, BLOCK_COUNT},
    events::BlockLandEvent,
    tetris::{ARENA_HEIGHT, ARENA_WIDTH, BLOCK_SIZE},
};

#[derive(SystemDesc)]
pub struct AddSquaresSystem {
    reader_id: Option<ReaderId<BlockLandEvent>>,
}

impl AddSquaresSystem {
    pub fn new() -> Self {
        Self { reader_id: None }
    }
}

impl<'s> System<'s> for AddSquaresSystem {
    type SystemData = (
        WriteStorage<'s, Position>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Square>,
        Entities<'s>,
        Write<'s, EventChannel<BlockLandEvent>>,
        ReadExpect<'s, Handle<SpriteSheet>>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (
            mut positions,
            mut transforms,
            mut squares,
            entities,
            mut land_channel,
            sprite_sheet_handle,
            mut sprite_renders,
        ): Self::SystemData,
    ) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| land_channel.register_reader());

        for event in land_channel.read(reader_id) {
            let BlockLandEvent { block, position } = event;
            for square_position in &block.square_positions(position) {
                let color_index = block.color_index + BLOCK_COUNT;
                let square = Square {
                    position: *square_position,
                    color_index: color_index,
                };
                let mut transform = Transform::default();
                let (x, y) = square_position.coordinates(false);
                transform.set_translation_xyz(x, y, 0.0);

                entities
                    .build_entity()
                    .with(square, &mut squares)
                    .with(
                        SpriteRender::new(sprite_sheet_handle.clone(), color_index),
                        &mut sprite_renders,
                    )
                    .with(*square_position, &mut positions)
                    .with(transform, &mut transforms)
                    .build();
                println!("Square in {:?}", square_position);
            }
        }
    }
}
