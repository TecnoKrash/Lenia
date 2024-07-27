use crate::init::Field;

pub fn gaussian(mu: f64, sigma: f64, x: f64) -> f64{
    (-0.5*(x - mu)*(x - mu)/(sigma*sigma)).exp()
}

pub fn growth(f: &mut Field, new_f: Vec<Vec<f64>>, dt: f64){
    let mk = f.k_size/2;
    for k in 0..f.nb_channels{
        for i in 0..f.h{
            for j in 0..f.l{
                // println!("new f : {}", new_f[i][j]);
                let rate = -1.0 + 2.0*gaussian(0.15,0.017, new_f[mk+i][mk+ j]);
                // println!("rate : {}", rate);
                
                // if rate > 0.0 {println!("rate : {}\n", rate);}

                f.m[k][i][j] += dt*rate;

                

                if f.m[k][i][j] < 0.0 {f.m[k][i][j] = 0.0}
                
                if f.m[k][i][j] > 1.0 {f.m[k][i][j] = 1.0}
            }
        }
    }
}
