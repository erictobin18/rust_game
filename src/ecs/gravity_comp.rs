use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
#[derive(Clone)]
pub struct GravityComponent {
    

}


impl Component for GravityComponent {
    type Storage = DenseVecStorage<Self>;
}
