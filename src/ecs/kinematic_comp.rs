use amethyst::core::transform::Transform;
use amethyst::ecs::{Component, DenseVecStorage, WriteStorage, Entity};
use amethyst::Error;
use amethyst::derive::PrefabData;
use amethyst::assets::PrefabData;
use serde::{Serialize, Deserialize};

#[derive(Clone, Component, Debug, Default)]
#[derive(Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct KinematicComponent {
    pub position:Transform,
    pub velocity:Transform,
    pub acceleration:Transform,

    pub m: f32,
}

// impl Component for KinematicComponent {
//     type Storage = DenseVecStorage<Self>;
// }
