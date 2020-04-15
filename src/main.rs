extern crate amethyst;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
// use amethyst::window::DisplayConfig;
// use amethyst::renderer::{pass::DrawFlat2D, pipeline::PipelinesBuilder,
// 			 RenderingBundle, plugins::RenderFlat2D,
// 			 plugins::RenderToWindow, types::DefaultBackend};

use amethyst::renderer::{
    plugins::{RenderFlat2D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle};
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

    // let path = format!(
    //     "{}/resources/display_config.ron",
    //     application_root_dir()
    // );

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config")
	.join("display_config.ron");
    let assets_dir = app_root.join("assets");

    // let pipe = PipelinesBuilder::build().with_stage(
    //     Stage::with_backbuffer()
    //         .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
    //         .with_pass(DrawFlat2D::new()),
    // );

    let game_data =
        GameDataBuilder::default()
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
		     .with_plugin(
			 RenderToWindow::from_config_path(display_config_path)?
			     .with_clear([0.0,0.0,0.0,1.0]),
		     )
		     .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with(GravitySystem, "gravity_system", &[])
        .with(PhysicsSystem, "physics_system", &["gravity_system"]);
    let mut game = Application::new(assets_dir, Simulate, game_data)?;

    game.run();

    Ok(())
}
