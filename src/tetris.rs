extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    shrev::EventChannel,
};

use crate::{entities::Block, events::BlockLandEvent};

pub const ARENA_HEIGHT: f32 = 976.0;
pub const ARENA_WIDTH: f32 = 610.0;
pub const BLOCK_SIZE: f32 = 61.0;
pub const MOVE_TIME: f32 = 0.5;

#[derive(Default)]
pub struct Tetris;

impl SimpleState for Tetris {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialize_camera(world);
        initialize_event_channel(world);

        let sprite_sheet_handle = load_sprite_sheet(world);

        // insert first block, why do I have to do this?
        let mut transform = Transform::default();
        transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT, 0.0);
        world
            .create_entity()
            .with(Block::rand())
            .with(SpriteRender::new(sprite_sheet_handle.clone(), 0))
            .with(transform)
            .build();

        world.insert(sprite_sheet_handle);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_event_channel(world: &mut World) {
    let mut land_channel = EventChannel::<BlockLandEvent>::new();
    land_channel.single_write(BlockLandEvent);
    world.insert(land_channel);
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/blocks.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/blocks.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
