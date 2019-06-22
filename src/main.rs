extern crate amethyst;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline,
                         RenderBundle, Stage};
use amethyst::utils::application_root_dir;

mod states;
use crate::states::Simulate;

mod ecs;
use ecs::PhysicsSystem;
use ecs::GravitySystem;

mod diffeq;

////////////////////////////////////////////////////////////////////////////////

fn exp(y: Vec<f32>) -> Vec<f32> {
    y
}

fn main() -> amethyst::Result<()> {
    // let ans = diffeq::rk45(&exp, vec![1.,2.,3.], 1.0);
    // println!("{:?}",ans);
    
    amethyst::start_logger(Default::default());    

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );
    let config = DisplayConfig::load(&path);

    

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data =
        GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config))
                     .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with(GravitySystem, "gravity_system", &[])
        .with(PhysicsSystem, "physics_system", &["gravity_system"]);
    let mut game = Application::new("./", Simulate, game_data)?;

    game.run();

    Ok(())
}
