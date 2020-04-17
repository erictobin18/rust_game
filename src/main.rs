extern crate amethyst;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;

use amethyst::renderer::{
    plugins::{RenderFlat2D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle};
use amethyst::utils::application_root_dir;

use amethyst::input::{InputBundle, StringBindings};

mod states;
use crate::states::Load;

mod ecs;
use ecs::PhysicsSystem;

mod diffeq;

////////////////////////////////////////////////////////////////////////////////

fn exp(y: Vec<f32>) -> Vec<f32> {
    y
}

fn main() -> amethyst::Result<()> {
    // let ans = diffeq::rk45(&exp, vec![1.,2.,3.], 1.0);
    // println!("{:?}",ans);
    
    amethyst::start_logger(Default::default());    

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config")
	.join("display_config.ron");
    let bindings_config = app_root.join("config")
	.join("bindings.ron");
    let assets_dir = app_root.join("assets");  
    
    let game_data =
        GameDataBuilder::default()
	.with_bundle(InputBundle::<StringBindings>::new()
		     .with_bindings_from_file(bindings_config)?)?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
		     .with_plugin(
			 RenderToWindow::from_config_path(display_config_path)?
			     .with_clear([0.0,0.0,0.0,1.0]),
		     )
		     .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with(PhysicsSystem, "physics_system", &[]);
    let mut game = Application::new(assets_dir, Load::new(), game_data)?;

    game.run();

    Ok(())
}
