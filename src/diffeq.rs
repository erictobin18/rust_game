use core::f32::INFINITY as INF;

const TOL:f32 = 1E-9;

pub fn rk45(f:&Fn(Vec<f32>) ->  Vec<f32>, y0:Vec<f32>, tf:f32) -> Vec<f32> {
    let verbose = false;
    if verbose {
        println!("------------------------------");
        println!("Entered Runge Kutta ODE Solver");
        println!("------------------------------");
        println!("Initial Vector: {:?}", y0);
    }
    let (a21,
         a31, a32,
         a41, a42, a43,
         a51, a52, a53, a54,
         a61, a62, a63, a64, a65,
         a71, a72, a73, a74, a75, a76) =
        (1./5.,
         3./40.,       9./40.,
         44./45.,     -56./15.,     32./9.,
         19372./6561.,-25360./2187.,64448./6561.,-212./729.,
         9017./3168., -355./33.,    46732./5247.,49./176., -5103./18656.,
         35./384.,    0.,          500./1113.,  125./192.,-2187./6784.,11./84.);
//    let (c2, c3, c4, c5, c6, c7) = (1./5., 3./10., 4./5., 8./9., 1., 1.);
    let (b1, b2, b3, b4, b5, b6, b7) =
        (35./384., 0., 500./1113., 125./192., -2187./6784., 11./84., 0.);
    let (bs1, bs2, bs3, bs4, bs5, bs6, bs7) =
        (5179./57600., 0., 7571./16695., 393./640.,
         -92097./339200., 187./2100., 1./40.);

    let mut t = 0.0;
    let mut step = 0.01*tf;
    let mut yn = y0.clone();

    let mut step_num = 0;

    while t < tf {
        let tr = tf - t;
        if step > tr {
            step = tr;
        }
        if verbose {
            println!("Time: {}", t);
            println!("Timestep: {}", step);
            println!("Time Remaining: {}", tr);
            println!("yn: {:?}", yn);
        }
        // calculate k_i
        let mut vect = yn.clone();
        let k1 = f(vect);

        let scalars = vec![a21];
        let vectors = vec![&k1];
        vect = superposition(&scalars, &vectors);
        let k2 = f(add_vect(&yn, &scalar_mult(&step, &vect)));

        let scalars = vec![a31, a32];
        let vectors = vec![&k1, &k2];
        vect = superposition(&scalars, &vectors);
        let k3 = f(add_vect(&yn, &scalar_mult(&step, &vect)));

        let scalars = vec![a41, a42, a43];
        let vectors = vec![&k1, &k2, &k3];
        vect = superposition(&scalars, &vectors);
        let k4 = f(add_vect(&yn, &scalar_mult(&step, &vect)));

        let scalars = vec![a51, a52, a53, a54];
        let vectors = vec![&k1, &k2, &k3, &k4];
        vect = superposition(&scalars, &vectors);
        let k5 = f(add_vect(&yn, &scalar_mult(&step, &vect)));

        let scalars = vec![a61, a62, a63, a64, a65];
        let vectors = vec![&k1, &k2, &k3, &k4, &k5];
        vect = superposition(&scalars, &vectors);
        let k6 = f(add_vect(&yn, &scalar_mult(&step, &vect)));

        let scalars = vec![a71, a72, a73, a74, a75, a76];
        let vectors = vec![&k1, &k2, &k3, &k4, &k5, &k6];
        vect = superposition(&scalars, &vectors);
        let k7 = f(add_vect(&yn, &scalar_mult(&step, &vect)));

        // check error bound
        let scalars = vec![b1-bs1,b2-bs2,b3-bs3,b4-bs4,b5-bs5,b6-bs6,b7-bs7];
        let vectors = vec![&k1, &k2, &k3, &k4, &k5, &k6, &k7];
        let sum = superposition(&scalars, &vectors);

        if verbose {
            println!("Sum: {:?}", sum);
        }
        
        let mut max_step = INF;

        for s in sum {
            let step_i = TOL/s.abs();
            if step_i < max_step { max_step = step_i }
        }

        if max_step < tr/1E6 {
            panic!("Differential Equation is too stiff!");
        }

        if verbose {
            println!("Max Step: {}", max_step);
        }
        if step > max_step {
            step = step*0.5;
            continue;
        }
        if step < max_step*0.65 && step < tr*0.9 {
            step = step*1.5;
            continue;
        }

        // calculate the new y-value and advance time
        let scalars = vec![b1, b2, b3, b4, b5, b6, b7];
        let vectors = vec![&k1, &k2, &k3, &k4, &k5, &k6, &k7];

        yn = add_vect(&yn,
		      &scalar_mult(&step, &superposition(&scalars, &vectors)));
        t += step;
        // println!("step number: {}", step_num);
        step_num += 1;


        // println!("{}\t{}\t{}\t\t{}\t{}", t, yn[0], yn[3], yn[7], yn[10]);
        
        if verbose {
            println!("scalars: {:?}", scalars);
            println!("vectors: {:?}", vectors);
            println!("Successful timestep length: {}", step);
            println!("Time: {}", t);
            println!("result: {:?}", yn);
        }
        // yn = step*(b1*k1 + b2*k2 + b3*k3 + b4*k4 + b5*k5 + b6*k6 + b7*k7);
        // t += step;
    }
    if verbose {
	println!("Finished solver with {} steps", step_num + 1);
    }
    // println!("Final Vector: {:?}", yn);
    yn
}

pub fn scalar_mult(s: &f32, v: &Vec<f32>) -> Vec<f32> {
    // println!("scalarmult");
    let mut out = v.clone();
    for i in 0..out.len() {
        out[i] = out[i]*s;
    }
    out
}

pub fn add_vect(v1:&Vec<f32>, v2:&Vec<f32>) -> Vec<f32> {
    // println!("addvect");
    let n = v1.len();
    assert_eq!(n, v2.len());
    let mut sum = v1.clone();
    for i in 0..n {
        sum[i] += v2[i];
    }
    sum
}

pub fn superposition(scalars: &Vec<f32>, vectors: &Vec<&Vec<f32>>) -> Vec<f32> {
    // println!("superposition");
    let n = scalars.len();
    assert_eq!(n, vectors.len());
    let mut acc = vec![0.0; vectors[0].len()];

    for i in 0..n {
        let v_i = scalar_mult(&scalars[i], vectors[i]);
        acc = add_vect(&acc, &v_i);
    }
    acc
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smooth_diff_eq() {
        let ans = rk45(&exp, vec![1.,2.,4.], 1.0);
        let expected = vec![2.,4.,8.];
        let error = vector_error(&ans, &expected);
        assert!(error < 1E-6);
    }

    fn exp(y: Vec<f32>) -> Vec<f32> {
        let mut ydot = y.clone();
        let log2 = 2.0f32.ln(); // natural log of 2
        for i in 0..ydot.len() {
            ydot[i] = ydot[i]*log2; // ydot = log(2)*y, thus y(t) = C*2^t
        }
        ydot
    }
    fn vector_error(ans: &Vec<f32>, expected: &Vec<f32>) -> f32 {
        assert_eq!(ans.len(), expected.len());
        let mut difference = 0.0;
        let mut norm = 0.0;
        for i in 0..ans.len() {
            let d = ans[i] - expected[i];
            difference += d*d;
            norm += expected[i]*expected[i];
        }
        (difference/norm).sqrt()
    }
}
