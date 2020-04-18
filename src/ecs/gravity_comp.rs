use amethyst::ecs::{Component, DenseVecStorage, WriteStorage, Entity};
use amethyst::Error;
use amethyst::derive::PrefabData;
use amethyst::assets::PrefabData;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Component, Debug, Default)]
#[derive(Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct GravityComponent {
    
}


// impl Component for GravityComponent {
//     type Storage = DenseVecStorage<Self>;
// }
