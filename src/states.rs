use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
// use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

use crate::ecs::kinematic_comp::KinematicComponent;
use crate::ecs::gravity_comp::GravityComponent;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;


pub struct Simulate;

impl SimpleState for Simulate {
    
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Hello World!");

        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        let sprite_sheet_handle = load_sprite_sheet(world);
        println!("{:?}",sprite_sheet_handle);
        world.register::<KinematicComponent>();
        initialize_asteroids(world, sprite_sheet_handle);
        
        initialize_camera(world);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -ARENA_WIDTH/2.,
            ARENA_WIDTH/2.,
            -ARENA_HEIGHT/2.,
            ARENA_HEIGHT/2.,
        )))
        .with(transform)
        .build();
}

fn initialize_asteroids(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let ast_1_transform = Transform::default();
    let ast_2_transform = Transform::default();

    // ast_1_transform.set_xyz(10.0, 0.0, 0.0);
    // ast_2_transform.set_xyz(-10.0, 0.0, 0.0);

    let mut ast_1_kinematic = KinematicComponent {
        position: Transform::default(),
        velocity: Transform::default(),
        acceleration: Transform::default(),

        m: 1.0,
    };
    ast_1_kinematic.position.set_xyz(20.0, 0.0, 0.0);
    ast_1_kinematic.velocity.set_xyz(0.0, -8.0, 0.0);
    
    let mut ast_2_kinematic = KinematicComponent {
        position: Transform::default(),
        velocity: Transform::default(),
        acceleration: Transform::default(),

        m: 1E9,
    };
    ast_2_kinematic.position.set_xyz(0.0, 0.0, 0.0);
    ast_2_kinematic.velocity.set_xyz(0.0, 0.0, 0.0);
    
    let ast_1_gravity = GravityComponent{};
    let ast_2_gravity = GravityComponent{};

    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(ast_1_transform)
        .with(ast_1_kinematic)
        .with(ast_1_gravity)
        .build();

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(ast_2_transform)
        .with(ast_2_kinematic)
        .with(ast_2_gravity)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "assets/dot.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "assets/dot.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}    
