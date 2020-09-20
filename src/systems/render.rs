use amethyst::{
    assets::Handle,
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
    renderer::{SpriteRender, SpriteSheet},
};

use crate::{
    entities::{Block, Position, Square},
    tetris::{ARENA_HEIGHT, ARENA_WIDTH, BLOCK_SIZE},
};

#[derive(SystemDesc)]
pub struct RenderSystem;

impl<'s> System<'s> for RenderSystem {
    type SystemData = (
        ReadStorage<'s, Block>,
        WriteStorage<'s, Square>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Position>,
        Entities<'s>,
        ReadExpect<'s, Handle<SpriteSheet>>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (
            blocks,
            mut squares,
            mut transforms,
            positions,
            entities,
            sprite_sheet_handle,
            mut sprite_renders,
        ): Self::SystemData,
    ) {
        for (_, entity) in (&squares, &*entities).join() {
            entities.delete(entity);
        }

        for (block, position) in (&blocks, &positions).join() {
            for square in block.squares {
                let mut transform = Transform::default();
                let Position { row, col } = *position - square.offset;
                transform.set_translation_xyz((row * 10).into(), (col * 10).into(), 0.0);

                entities
                    .build_entity()
                    .with(Square::new(), &mut blocks)
                    .with(
                        SpriteRender::new(sprite_sheet_handle.clone(), 0),
                        &mut sprite_renders,
                    )
                    .with(transform, &mut transforms)
                    .build();
            }
        }
    }
}
