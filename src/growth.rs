use crate::init::Field;

pub fn gaussian(mu: f64, sigma: f64, x: f64) -> f64{
    (-0.5*(x - mu)*(x - mu)/(sigma*sigma)).exp()
}

pub fn growth(f: &mut Field, new_f: Vec<Vec<f64>>, dt: f64){
    for k in 0..f.nb_channels{
        for i in 0..f.h{
            for j in 0..f.l{
                let rate = gaussian(0.15,0.015, new_f[i][j]);
                f.m[k][i][j] += dt*rate;
            }
        }
    }
}
