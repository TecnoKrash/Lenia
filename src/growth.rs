use rand::prelude::*;
use std::f64::consts::PI;

use crate::init::*;

// Seed for the random noise
#[derive(Debug)]
pub struct Seed {
    pub freqs: (Vec<f64>, Vec<f64>, Vec<f64>),
    pub phases: (Vec<f64>, Vec<f64>, Vec<f64>),
}

// The gaussian function 
pub fn gaussian(mu: f64, sigma: f64, x: f64) -> f64{
    (-0.5*(x - mu)*(x - mu)/(sigma*sigma)).exp()
}

// Computing and applying the growth rate of each cell in the field
pub fn growth_lenia(f: &mut Field, new_f: Vec<Vec<f64>>, dt: f64, mu: f64, sigma: f64, h: f64, chan: usize, chan_ratio: f64, seed: &Seed, bruit: f64){
    let mk = f.k_size/2;

    for i in 0..f.h{
        for j in 0..f.l{
            // println!("new f : {}", new_f[i][j]);
            let rate = h*(-1.0 + 2.0*gaussian(mu,sigma, new_f[mk+i][mk+j]));

            // if rate > 0.0 { println!("rate : {}\n", rate); }
            let noise = 1.0 + random_noise(seed, i,j,f.t, bruit);

            f.m[chan][i][j] += dt*rate*noise*chan_ratio;

            // if f.m[chan][i][j] > 0.0 {println!("rate : {}, new_val : {}, noise: {}\n", rate, f.m[chan][i][j], noise);}


            if f.m[chan][i][j] < 0.0 {f.m[chan][i][j] = 0.0}

            if f.m[chan][i][j] > 1.0 {f.m[chan][i][j] = 1.0}
        }
    }

    f.t += dt;
}

// Growth function for the game of life mode
pub fn growth_gol(f: &mut Field, new_f: Vec<Vec<f64>>, _dt: f64, mu: f64, sigma: f64){
    let mk = f.k_size/2;
    for k in 0..f.nb_channels{
        for i in 0..f.h{
            for j in 0..f.l{
                
                if (new_f[mk+i][mk+j] - mu).abs() < sigma { f.m[k][i][j] = 1.0; }
                else { f.m[k][i][j] = 0.0;}
                    
            }
        }
    }
}

// Generate the seed
pub fn generate_seed(fmax: (f64, f64, f64), nb_sin: usize) -> Seed{

    let mut rng = rand::thread_rng();

    let mut freqs = (Vec::with_capacity(nb_sin), Vec::with_capacity(nb_sin), Vec::with_capacity(nb_sin));
    
    let mut phases = (Vec::with_capacity(nb_sin), Vec::with_capacity(nb_sin), Vec::with_capacity(nb_sin));

    for _i in 0..nb_sin {
        freqs.0.push(rng.gen::<f64>()*fmax.0);
        freqs.1.push(rng.gen::<f64>()*fmax.1);
        freqs.2.push(rng.gen::<f64>()*fmax.2);
        // freqs.2.push(fmax.2/2.0 + rng.gen::<f64>()*(fmax.2/2.0));
        // println!("freqs.2[i]: {}", freqs.2[i]);
        phases.0.push(rng.gen::<f64>()*2.0*PI);
        phases.1.push(rng.gen::<f64>()*2.0*PI);
        phases.2.push(rng.gen::<f64>()*2.0*PI);
    }


    Seed { freqs, phases}
}

// Compute the random noise at a certain point in space and time
pub fn random_noise(seed: &Seed, x: usize, y: usize, t: f64, ampli: f64) -> f64{

    let mut res = 0.0;

    let x_f64 =  x as f64;
    let y_f64 = y as f64;
    
    for i in 0..seed.freqs.0.len() {
        res += (2.0*PI*seed.freqs.0[i]*x_f64 + seed.phases.0[i]).sin()*(2.0*PI*seed.freqs.1[i]*y_f64 + seed.phases.1[i]).sin()*(2.0*PI*seed.freqs.2[i]*t + seed.phases.2[i]).sin();
    }

    (res/seed.freqs.0.len() as f64)*ampli
}
