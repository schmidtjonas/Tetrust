extern crate amethyst;
use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod entities;
mod events;
mod systems;
mod tetris;

use crate::{
    systems::{AddSquaresSystem, MoveBlocksSystem, RemoveLinesSystem, SpawnBlocksSystem},
    tetris::Tetris,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(MoveBlocksSystem::new(), "move_blocks_system", &[])
        .with(SpawnBlocksSystem::new(), "spawn_blocks_system", &[])
        .with(AddSquaresSystem::new(), "add_squares_system", &[])
        .with(RemoveLinesSystem::new(), "remove_lines_system", &["add_squares_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderUi::default())
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Tetris::default(), game_data)?;
    game.run();

    Ok(())
}
