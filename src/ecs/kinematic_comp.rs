use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::core::transform::Transform;

#[derive(Clone, Debug)]
pub struct KinematicComponent {
    pub position:Transform,
    pub velocity:Transform,
    pub acceleration:Transform,

    pub m: f32,
}

impl Component for KinematicComponent {
    type Storage = DenseVecStorage<Self>;
}
