use amethyst::assets::{AssetStorage, Loader, Handle};
use amethyst::core::{transform::Transform, ArcThreadPool};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, ImageFormat, camera::Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, sprite::SpriteSheetHandle, Texture,
};

use amethyst::ecs::{Dispatcher, DispatcherBuilder};

use crate::ecs::kinematic_comp::KinematicComponent;
use crate::ecs::gravity_comp::GravityComponent;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;


use crate::ecs::PhysicsSystem;

#[derive(PartialEq, Debug)]
pub enum PhysicsStatus {
    Running,
    Paused,
}

impl Default for PhysicsStatus {
    fn default() -> Self {
	PhysicsStatus::Paused
    }
}

// States:
// Load
// First Person
// Overlay
// Pause

// Loading Screen
pub struct Load<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> Load<'a, 'b> {
    pub fn new() -> Self {
	Self {
	    dispatcher: Some(DispatcherBuilder::new().build()),
	}
    }
}

impl<'a, 'b> SimpleState for Load<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
	println!("Load Mode");
	// load prefabs, create all entities
	println!("Hello World!");
        let world = data.world;

	// Create Dispatcher
	let mut dispatcher_builder = DispatcherBuilder::new();

	let mut dispatcher = dispatcher_builder
	    .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
	    .build();
	dispatcher.setup(world);

	self.dispatcher = Some(dispatcher);

	// Create a resource to pause physics
	let physics_running = PhysicsStatus::default();
	world.insert(physics_running);

        // Load the spritesheet necessary to render the graphics.
        let sprite_sheet_handle = load_sprite_sheet(world);
        println!("{:?}",sprite_sheet_handle);
        world.register::<KinematicComponent>();
        initialize_asteroids(world, sprite_sheet_handle);        
        initialize_camera(world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
	if let Some(dispatcher) = self.dispatcher.as_mut() {
	    dispatcher.dispatch(&data.world);
	}
	let world = &data.world;

	let fetched = world.try_fetch::<PhysicsStatus>();
	if let Some(fetched_resource) = fetched {
	    println!("{:?}", *fetched_resource);
	}

	Trans::Replace(Box::new(Overlay::new()))
    }
}

// Draw an informational overlay
pub struct Overlay<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> Overlay<'a, 'b> {
    pub fn new() -> Self {
	Self {
	    dispatcher: Some(DispatcherBuilder::new().build()),
	}
    }
}

impl<'a, 'b> SimpleState for Overlay<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
	println!("Overlay Mode");
        let world = data.world;
 
	// Create Dispatcher
	let mut dispatcher_builder = DispatcherBuilder::new();
	dispatcher_builder
	    .add(PhysicsSystem.pausable(PhysicsStatus::Running),
		 "physics_system", &[]);

	let mut dispatcher = dispatcher_builder
	    .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
	    .build();
	dispatcher.setup(world);

	self.dispatcher = Some(dispatcher);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
	if let Some(dispatcher) = self.dispatcher.as_mut() {
	    dispatcher.dispatch(&data.world);
	}

	Trans::None
    }
}

// Control an entity from first-person
pub struct FirstPerson<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> FirstPerson<'a, 'b> {
    pub fn new() -> Self {
	Self {
	    dispatcher: Some(DispatcherBuilder::new().build()),
	}
    }
}

impl<'a, 'b> SimpleState for FirstPerson<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
	println!("First Person Mode");
        let world = data.world;
 
	// Create Dispatcher
	let mut dispatcher_builder = DispatcherBuilder::new();
	dispatcher_builder
	    .add(PhysicsSystem, "physics_system", &[]);

	let mut dispatcher = dispatcher_builder
	    .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
	    .build();
	dispatcher.setup(world);

	self.dispatcher = Some(dispatcher);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
	if let Some(dispatcher) = self.dispatcher.as_mut() {
	    dispatcher.dispatch(&data.world);
	}

	Trans::None
    }
}

// Pause Game
pub struct Pause;

impl SimpleState for Pause {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
	println!("Pause Mode");
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_asteroids(world: &mut World,
			sprite_sheet_handle: SpriteSheetHandle) {
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
    ast_1_kinematic.position.set_translation_xyz(20.0, 0.0, 0.0);
    ast_1_kinematic.velocity.set_translation_xyz(0.0, -8.0, 0.0);
    
    let mut ast_2_kinematic = KinematicComponent {
        position: Transform::default(),
        velocity: Transform::default(),
        acceleration: Transform::default(),

        m: 1E9,
    };
    ast_2_kinematic.position.set_translation_xyz(0.0, 0.0, 0.0);
    ast_2_kinematic.velocity.set_translation_xyz(0.0, 0.0, 0.0);
    
    let ast_1_gravity = GravityComponent{};
    let ast_2_gravity = GravityComponent{};

    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
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

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/dot.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/dot.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}    
