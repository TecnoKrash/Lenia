use rand::prelude::*;
use std::f64::consts::PI;

use crate::init::*;
use crate::imgep::*;

pub struct Seed {
    pub freqs: (Vec<f64>, Vec<f64>, Vec<f64>),
    pub phases: (Vec<f64>, Vec<f64>, Vec<f64>),
}


pub fn gaussian(mu: f64, sigma: f64, x: f64) -> f64{
    (-0.5*(x - mu)*(x - mu)/(sigma*sigma)).exp()
}

                // println!("rate : {}", rate);
pub fn growth_lenia_old(f: &mut Field, new_f: Vec<Vec<f64>>, dt: f64, mu: f64, sigma: f64){
    let mk = f.k_size/2;
    for k in 0..f.nb_channels{
        for i in 0..f.h{
            for j in 0..f.l{
                // println!("new f : {}", new_f[i][j]);
                let rate = -1.0 + 2.0*gaussian(mu,sigma, new_f[mk+i][mk+j]);
                
                // if rate > 0.0 {println!("rate : {}\n", rate);}

                f.m[k][i][j] += dt*rate;

                

                if f.m[k][i][j] < 0.0 {f.m[k][i][j] = 0.0}
                
                if f.m[k][i][j] > 1.0 {f.m[k][i][j] = 1.0}
            }
        }
    }
}

pub fn growth_lenia(f: &mut Field, new_f: Vec<Vec<f64>>, dt: f64, mu: f64, sigma: f64, h: f64, chan: usize, chan_ratio: f64, seed: &Seed, bruit: f64){
    let mk = f.k_size/2;

    for i in 0..f.h{
        for j in 0..f.l{
            // println!("new f : {}", new_f[i][j]);
            let rate = h*(-1.0 + 2.0*gaussian(mu,sigma, new_f[mk+i][mk+j]));

            // if rate > 0.0 { println!("rate : {}\n", rate); }

            f.m[chan][i][j] += dt*rate*chan_ratio*bruit_rand(seed, i,j,f.t, bruit);

            if f.m[chan][i][j] > 0.0 {println!("rate : {}, new_val : {}\n", rate, f.m[chan][i][j]);}


            if f.m[chan][i][j] < 0.0 {f.m[chan][i][j] = 0.0}

            if f.m[chan][i][j] > 1.0 {f.m[chan][i][j] = 1.0}
        }
    }

    f.t += dt;
}

pub fn growth_test(p: &Param) {

    let n = 128;
    let m = ((16 * n) as f64 / 9.0).ceil() as usize;
    
    // Création de la matrice X (équivalent à np.zeros)
    let x_matrix = vec![vec![0.0; m]; n];
    
    // Calcul des half sizes
    let fhs_y = n / 2;
    let fhs_x = m / 2;
    
    // Création des grilles y et x (équivalent à np.ogrid)
    let y: Vec<i32> = (-(fhs_y as i32)..(fhs_y as i32)).collect();
    let x: Vec<i32> = (-(fhs_x as i32)..(fhs_x as i32)).collect();
    
    // Conversion en f64 pour les calculs
    let y_f64: Vec<f64> = y.iter().map(|&val| val as f64).collect();
    let x_f64: Vec<f64> = x.iter().map(|&val| val as f64).collect();
    

    // Fonction principale (à placer dans votre fonction)
    let mut ks: Vec<Vec<f64>> = Vec::new();

    for (b, r) in p.b.iter().zip(p.r.iter()) {
        // Calcul de la distance (assumant que x et y sont des Vec<f64>)
        let distance: Vec<f64> = x_f64.iter().zip(y_f64.iter())
            .map(|(xi, yi)| (xi.powi(2) + yi.powi(2)).sqrt() / r * (b.len() as f64))
            .collect();

        // Initialisation de K avec des zéros
        let mut k = vec![0.0; distance.len()];

        let mu = 0.5;
        let sigma = 0.15;

        for i in 0..b.len() {
            for (j, &dist) in distance.iter().enumerate() {
                if dist as usize == i {
                    k[j] += b[i] * gaussian(mu, sigma, dist % 1.0);
                }
            }
        }

        // Normalisation de K
        let sum_k: f64 = k.iter().sum();
        if sum_k != 0.0 {
            k.iter_mut().for_each(|val| *val /= sum_k);
        }

        ks.push(k);
    }
}


pub fn growth_gol(f: &mut Field, new_f: Vec<Vec<f64>>, _dt: f64, mu: f64, sigma: f64){
    let mk = f.k_size/2;
    // println!("{}", mk);
    for k in 0..f.nb_channels{
        for i in 0..f.h{
            for j in 0..f.l{
                // println!("new f : {}", new_f[i][j]);
                
                if (new_f[mk+i][mk+j] - mu).abs() < sigma { f.m[k][i][j] = 1.0; }
                else { f.m[k][i][j] = 0.0;}
                    
                // println!("rate : {}", rate);
                
                // if rate > 0.0 {println!("rate : {}\n", rate);}

                
            }
        }
    }
}

pub fn generate_seed(fmax: (f64, f64, f64), nb_sin: usize) -> Seed{

    let mut rng = rand::thread_rng();

    let mut freqs = (Vec::with_capacity(nb_sin), Vec::with_capacity(nb_sin), Vec::with_capacity(nb_sin));
    
    let mut phases = (Vec::with_capacity(nb_sin), Vec::with_capacity(nb_sin), Vec::with_capacity(nb_sin));

    for _i in 0..nb_sin {
        freqs.0.push(rng.gen::<f64>()*fmax.0);
        freqs.1.push(rng.gen::<f64>()*fmax.0);
        freqs.2.push(rng.gen::<f64>()*fmax.0);
        phases.0.push(rng.gen::<f64>()*2.0*PI);
        phases.1.push(rng.gen::<f64>()*2.0*PI);
        phases.2.push(rng.gen::<f64>()*2.0*PI);
    }


    Seed { freqs, phases}
}


pub fn bruit_rand(seed: &Seed, x: usize, y: usize, t: f64, ampli: f64) -> f64{

    let mut res = 0.0;

    let x_f64 =  x as f64;
    let y_f64 = y as f64;
    
    for i in 0..seed.freqs.0.len() {
        res += (2.0*PI*seed.freqs.0[i]*x_f64 + seed.phases.0[i]).sin() * (2.0*PI*seed.freqs.1[i]*y_f64 + seed.phases.1[i]).sin() * (2.0*PI*seed.freqs.2[i]*t + seed.phases.2[i]).sin()*ampli;
    }

    1.0 + res
}
