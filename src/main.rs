extern crate amethyst;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;

use amethyst::renderer::{
    plugins::{RenderFlat3D, RenderFlat2D, RenderToWindow, RenderDebugLines,
	      RenderSkybox},
    types::DefaultBackend,
    palette::Srgb,
    RenderingBundle};
use amethyst::utils::{application_root_dir, auto_fov::AutoFovSystem};
use amethyst::input::{InputBundle, StringBindings};
use amethyst::controls::FlyControlBundle;

mod states;
use crate::states::Load;

mod ecs;
use ecs::PhysicsSystem;

mod diffeq;

//////////////////////////////////////////////////////////////////////////////

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
    
    let game_data = GameDataBuilder::default()
	.with(PhysicsSystem, "physics_system", &[])
	.with(AutoFovSystem::default(), "auto_fov", &[])
	.with_bundle(InputBundle::<StringBindings>::new()
		     .with_bindings_from_file(bindings_config)?)?
	.with_bundle(
            FlyControlBundle::<StringBindings>::new(
                Some("strafe_horizontal".into()),
                Some("strafe_vertical".into()),
                Some("strafe_normal".into()),
            )
            .with_sensitivity(0.1, 0.1)
            .with_speed(5.),
        )?
        .with_bundle(TransformBundle::new().with_dep(&[
	    "physics_system",
	    "fly_movement",
	]))?
	.with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?)
                .with_plugin(RenderFlat3D::default())
		.with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderSkybox::with_colors(
                    Srgb::new(0.82, 0.51, 0.50),
                    Srgb::new(0.18, 0.11, 0.85),
                )),
        )?;

    let mut game = Application::new(assets_dir, Load::new(), game_data)?;

    game.run();

    Ok(())
}
