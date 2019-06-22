//use num::Signed;

use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::ecs::kinematic_comp::KinematicComponent;
use crate::ecs::gravity_comp::GravityComponent;


const GRAVITATION_CONSTANT:f32 = 1E-6;
const TIMESTEP_MIN:f32 = 1E-9;

use crate::diffeq::rk45;

pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        ReadStorage<'s, GravityComponent>,
        WriteStorage<'s, KinematicComponent>,
        Read<'s, Time>,
    );
    fn run(&mut self, (grav_comps, mut kine_comps, time): Self::SystemData) {
        println!("RUNNING GRAVITY SYSTEM");
        let dt = time.delta_seconds();
        println!("dt: {}",dt);
        if dt < TIMESTEP_MIN { return }
        
        let mut comps:Vec<_> = (&mut kine_comps)
            .join()
            .collect();
        let n = comps.len();
        // println!("n: {}",n);
        // pub fn rk45(f:&Fn(Vec<f32>) ->  Vec<f32>, y0:Vec<f32>, tf:f32) -> Vec<f32> {

        // build y0 vector
        // entries are x,y,z,vx,vy,vz,m for each component
        let mut y0 = vec![0.0; n*7];

        // for i in 0..n {
        //     println!("i: {}",i);
        //     println!("{:?}", comps[i].position.translation());
        //     println!("{:?}", comps[i].velocity.translation());
        //     println!("{:?}", comps[i].acceleration.translation());
        // }
        
        for i in 0..n {
            y0[7*i]     = comps[i].position.translation()[0];
            y0[7*i + 1] = comps[i].position.translation()[1];
            y0[7*i + 2] = comps[i].position.translation()[2];
            y0[7*i + 3] = comps[i].velocity.translation()[0];
            y0[7*i + 4] = comps[i].velocity.translation()[1];
            y0[7*i + 5] = comps[i].velocity.translation()[2];
            y0[7*i + 6] = comps[i].m;
        }
        
        // println!("Vector sent to solver: {:?}", y0);
        let yf = rk45(&compute_gravity, y0, dt);
        println!("Solver returned: {:?}", yf);
        
        // change the kine_comps so that the physics system will compute the
        // correct final state at the end of the frame

        for i in 0..n {
            let dx = yf[7*i]     - comps[i].position.translation()[0];
            let dy = yf[7*i + 1] - comps[i].position.translation()[1];
            let dz = yf[7*i + 2] - comps[i].position.translation()[2];
            let vx0 = dx/dt;
            let vy0 = dy/dt;
            let vz0 = dz/dt;

            comps[i].velocity.set_xyz(vx0, vy0, vz0);

            let dvx = yf[7*i + 3] - vx0;
            let dvy = yf[7*i + 4] - vy0;
            let dvz = yf[7*i + 5] - vz0;
            let ax0 = dvx/dt;
            let ay0 = dvy/dt;
            let az0 = dvz/dt;
            
            comps[i].acceleration.set_xyz(ax0, ay0, az0);
           
        }
        // for i in 0..n {
        //     println!("i: {}",i);
        //     println!("{:?}", comps[i].position.translation());
        //     println!("{:?}", comps[i].velocity.translation());
        //     println!("{:?}", comps[i].acceleration.translation());
        // }
    }
}

fn compute_gravity(y0:Vec<f32>) -> Vec<f32> {
    // println!("RUNNING COMPUTE FUNCTION");
    // println!("INPUT: {:?}", y0);
    let mut dy = vec![0.0; y0.len()];
    // n is number of objects, each object has x,y,z; vx,vy,vz; and m in y0
    let n = y0.len()/7;
    // println!("n: {}",n);
    for i in 0..n {
        // println!("i: {}",i);
        let x_i = y0[7*i];
        let y_i = y0[7*i + 1];
        let z_i = y0[7*i + 2];
        let vx_i = y0[7*i + 3];
        let vy_i = y0[7*i + 4];
        let vz_i = y0[7*i + 5];
        let mi = y0[7*i + 6];
        if mi < 1E-9 { panic!("NONPOSITIVE MASS!") }

        // the derivative of position entries is the velocity entries
        // just copy them over
        dy[7*i]     = vx_i;
        dy[7*i + 1] = vy_i;
        dy[7*i + 2] = vz_i;

        // the derivative of mass entries is zero

        dy[7*i + 6] = 0.0;

        for j in (i+1)..n {
            // println!("j: {}",j);
            let x_j = y0[7*j];
            let y_j = y0[7*j + 1];
            let z_j = y0[7*j + 2];
            let vx_j = y0[7*j + 3];
            let vy_j = y0[7*j + 4];
            let vz_j = y0[7*j + 5];
            let mj = y0[7*j + 6];
            if mj < 1E-9 { panic!("NONPOSITIVE MASS!") }

            let delx = x_i - x_j;
            let dely = y_i - y_j;
            let delz = z_i - z_j;

            // println!("dx: {}, dy: {}, dz: {}", delx, dely, delz);

            let norm = (delx*delx + dely*dely + delz*delz).sqrt();
            let denom = norm*norm*norm;
            // println!("norm: {}", norm);
            if denom < 1E-30 {
                println!("i: {}, j: {}",i,j);
                println!("y: {:?}", y0);
                panic!("GRAVITATIONAL POLE") }

            let coef = mi*mj*GRAVITATION_CONSTANT/(denom + 1E-9);
            let fx = -delx*coef;
            let fy = -dely*coef;
            let fz = -delz*coef;

            // add the calculated acceleration due to gravity to the
            // acceleration entries
            
            dy[7*i + 3] += fx/mi;
            dy[7*i + 4] += fy/mi;
            dy[7*i + 5] += fz/mi;
            
            dy[7*j + 3] += -fx/mj;
            dy[7*j + 4] += -fy/mj;
            dy[7*j + 5] += -fz/mj;
        }
    }
    // println!("RESULT: {:?}", dy);
    dy
}
    
//         for i in 0..comps.len()
//         {

//             for j in i+1..comps.len()
//             {
//                 let disp = comps[i].1.position.translation() -
//                     comps[j].1.position.translation();
// //                println!("disp = {}",disp);
//                 let norm = disp.norm();
//                 // println!("norm = {}",norm);
//                 let denom = norm*norm*norm;
//                 let mi = comps[i].1.m;
//                 let mj = comps[j].1.m;
//                 let numer = disp*mi*mj*GRAVITATION_CONSTANT;
//                 // println!("numer = {}", numer);
                
//                 let force = -numer/(denom + 1.0E-6);

//                 // println!("force = {}", force);

//                 *comps[i].1.acceleration.translation_mut() += force/mi;
//                 *comps[j].1.acceleration.translation_mut() += -force/mj;
// //                println!("{},{}",i,j);
// //                println!("{:?}",disp);
// //                println!("{}",force);
//             }
            
// //             for (j, (grav_j, kine_j, trans_j)) in comps[i +
// //            println!("{:?}",kine_i);
// //            println!("");
// //            kine_i.vdot[0] += 1.0;
//         }
//         // for i in (0..comps.len())
//         // {
//         //     println!("translation = {}", comps[i].2.translation());
//         //     println!("v = {:?}", comps[i].1.v);
//         //     println!("vdot = {:?}", comps[i].1.vdot);
//         // }
//     }
// }


// // trait GravityModel {}
