use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::ecs::kinematic_comp::KinematicComponent;

pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        // Components
        WriteStorage<'s, KinematicComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );
    fn run(&mut self, (mut kine_comps, mut transforms, time): Self::SystemData) {
        let dt = time.delta_seconds();
        // println!("dt: {}",dt);
        // if dt < 0.016 {
        //     for k in (&mut kine_comps).join() {
        //         k.vdot[0] = 0.0;
        //         k.vdot[1] = 0.0;
        //     }
        //     return
        // }
        // let dt = 0.016;

        for kine in (&mut kine_comps).join() {
            let vel = kine.velocity.clone();
            let acc = kine.acceleration.clone();
            *kine.position.translation_mut() += vel.translation()*dt;
            *kine.velocity.translation_mut() += acc.translation()*dt;
            kine.acceleration.set_xyz(0.0, 0.0, 0.0);

            // println!("{:?}", kine.position.translation());
            // println!("{:?}", kine.velocity.translation());
            // println!("{:?}", kine.acceleration.translation());
        }

        for (k, t) in (&kine_comps, &mut transforms).join() {
            *t = k.position.clone();
        }
        // println!("\n\n\n\n");

        // if (&mut transform).join().next().unwrap().translation()[0] < 9.0 {
        //     panic!();
        // }
    }
}
