//! Pong Tutorial 1

use amethyst::audio::AudioBundle;
use amethyst::audio::DjSystemDesc;
use amethyst::core::transform::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::utils::fps_counter::FpsCounterBundle;
use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::audio::Music;
use crate::pong::Pong;

mod audio;
mod pong;
mod systems;

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
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        // We have now added our own systems, defined in the systems module
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(
            systems::BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        )
        .with(systems::WinnerSystem, "winner_system", &["ball_system"])
        .with(systems::FpsCounterSystem, "fps_counter_system_display", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and
                // drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(FpsCounterBundle::default())?;

    let assets_dir = "./assets";
    //    let mut world = World::new();
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();

    Ok(())
}
