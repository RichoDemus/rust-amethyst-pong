use amethyst::utils::fps_counter::FpsCounter;
use amethyst::{
    assets::Loader,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::Entity,
    ecs::prelude::{Read, ReadExpect, System, SystemData, World, WorldExt, WriteStorage},
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

#[derive(SystemDesc)]
pub struct FpsCounterSystem;

pub struct FpsText {
    pub fps: Entity,
}

impl<'s> System<'s> for FpsCounterSystem {
    type SystemData = (
        Read<'s, FpsCounter>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, FpsText>,
    );

    fn run(&mut self, (data, mut ui_text, fps_text): Self::SystemData) {
        let fps = data.sampled_fps();

        if let Some(text) = ui_text.get_mut(fps_text.fps) {
            text.text = format!("{:.0}", fps);
        }
    }
}

pub fn initialize_fps_counter(world: &mut World) {
    let counter = FpsCounter::new(30);
    world.insert(counter);

    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "FPS".to_string(),
        Anchor::BottomRight,
        Anchor::BottomRight,
        0.,
        0.,
        1.,
        200.,
        50.,
    );

    let fps = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(font, "0".to_string(), [1., 1., 1., 1.], 50.))
        .build();

    world.insert(FpsText { fps });
}
