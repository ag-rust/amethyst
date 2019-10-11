use amethyst::{
    assets::{HotReloadBundle, Processor},
    audio::Source,
    core::{
        transform::{TransformBundle},
    },
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::RenderToWindow,
        // rendy::mesh::{Normal, Position, TexCoord},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    utils::fps_counter::FpsCounterBundle,
};

mod events;
mod menu;
mod credits;
mod util;
mod welcome;
mod game;
mod pause;


use events::UiEventHandlerSystemDesc;
use welcome::WelcomeScreen;


pub fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("examples/states_ui/config/display.ron");
    let assets_dir = app_root.join("examples/assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(HotReloadBundle::default())?
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with_system_desc(UiEventHandlerSystemDesc::default(), "ui_event_handler", &[])
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.005, 0.005, 0.005, 1.0]),
                )
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::new(assets_dir, WelcomeScreen::default(), game_data)?;
    game.run();

    Ok(())
}
