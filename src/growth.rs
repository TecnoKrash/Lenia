use crate::init::Field;
use rand::prelude::*;

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

pub fn growth_lenia(f: &mut Field, new_f: Vec<Vec<f64>>, dt: f64, mu: f64, sigma: f64, chan: usize, chan_ratio: f64, bruit: f64){
    let mk = f.k_size/2;

    let mut rng = thread_rng();

    for i in 0..f.h{
        for j in 0..f.l{
            // println!("new f : {}", new_f[i][j]);
            let rate = -1.0 + 2.0*gaussian(mu,sigma, new_f[mk+i][mk+j]);

            // if rate > 0.0 {println!("rate : {}\n", rate);}

            f.m[chan][i][j] += dt*rate*chan_ratio*(1.0 + rng.gen::<f64>()*bruit);



            if f.m[chan][i][j] < 0.0 {f.m[chan][i][j] = 0.0}

            if f.m[chan][i][j] > 1.0 {f.m[chan][i][j] = 1.0}
        }
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
