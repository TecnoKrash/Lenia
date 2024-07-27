use rand::prelude::*;

use crate::convolution::*;
use crate::growth::*;

pub struct Field {
    pub t: f64,
    pub l: usize,
    pub h: usize,
    pub k_size: usize,
    pub nb_channels: usize,
    pub m: Vec<Vec<Vec<f64>>>,
}

pub enum Kernel {
    Ring
}

pub enum Motif {
    Orbium,
    Rand,
}

impl Field {
    // A function to create an empty field
    pub fn new_field(h: usize, l: usize, nb_chan: usize) -> Field {
        Field {
            t: 0.0,
            l,
            h,
            k_size: 0,
            nb_channels: nb_chan,
            m: vec![vec![vec![0.; l]; h]; nb_chan],
        }
    }

    pub fn to_tore(mut self: Field, kernel: Vec<Vec<f64>>){
        for i in 0..self.nb_channels{
            self.m[i] = tore_format(&self.m[i], &kernel);
        }
        self.k_size = kernel.len();
    }

    pub fn get_xy(self: &Field, x: usize, y: usize, chanel: usize) -> f64{
        self.m[chanel][x][y]
    }

    pub fn fill(self: &mut Field,chan: usize, val: f64){
        for i in 0..self.h{
            for j in 0..self.l{
                self.m[chan][i][j] = val;
            }
        }
    }

    pub fn fill_deg(self: &mut Field, chan: usize, start: f64, end: f64){
        for i in 0..self.h{
            for j in 0..self.l{
                let fi = i as f64;
                let fj = j as f64;
                let fl = self.l as f64;
                let fh = self.h as f64;
                self.m[chan][i][j] = start + (fi+fj)*(end)/(fh+fl);
            }
        }
    }

    pub fn fill_rng(self:&mut Field, chan: usize){
        let mut rng = rand::thread_rng();
        for i in 0..self.h{
            for j in 0..self.l{
                self.m[chan][i][j] = rng.gen::<f64>();
            }
        }
    }

    pub fn add(self:&mut Field, motif: Motif, x: usize, y: usize){
        let m;
        match motif{
            Motif::Orbium => {
                m = orbium();
            },
            Motif::Rand => {
                m = random_square(30);
            }
        }
        
        if (y + m[0].len() < self.l) && (x + m.len() < self.h){
            for i in 0..m.len(){
                for j in 0..m[0].len(){
                    self.m[0][x+i][y+j] = m[i][j];
                }
            }
        }
    }

    

}


fn orbium() -> Vec<Vec<f64>>{
    vec![[0.0,0.0,0.0,0.0,0.0,0.0,0.1,0.14,0.1,0.0,0.0,0.03,0.03,0.0,0.0,0.3,0.0,0.0,0.0,0.0].to_vec(), 
     [0.0,0.0,0.0,0.0,0.0,0.08,0.24,0.3,0.3,0.18,0.14,0.15,0.16,0.15,0.09,0.2,0.0,0.0,0.0,0.0].to_vec(), 
     [0.0,0.0,0.0,0.0,0.0,0.15,0.34,0.44,0.46,0.38,0.18,0.14,0.11,0.13,0.19,0.18,0.45,0.0,0.0,0.0].to_vec(), 
     [0.0,0.0,0.0,0.0,0.06,0.13,0.39,0.5,0.5,0.37,0.06,0.0,0.0,0.0,0.02,0.16,0.68,0.0,0.0,0.0].to_vec(), 
     [0.0,0.0,0.0,0.11,0.17,0.17,0.33,0.4,0.38,0.28,0.14,0.0,0.0,0.0,0.0,0.0,0.18,0.42,0.0,0.0].to_vec(), 
     [0.0,0.0,0.09,0.18,0.13,0.06,0.08,0.26,0.32,0.32,0.27,0.0,0.0,0.0,0.0,0.0,0.0,0.82,0.0,0.0].to_vec(), 
     [0.27,0.0,0.16,0.12,0.0,0.0,0.0,0.25,0.38,0.44,0.45,0.34,0.0,0.0,0.0,0.0,0.0,0.22,0.17,0.0].to_vec(), 
     [0.0,0.07,0.2,0.02,0.0,0.0,0.0,0.31,0.48,0.57,0.6,0.57,0.0,0.0,0.0,0.0,0.0,0.0,0.49,0.0].to_vec(), 
     [0.0,0.59,0.19,0.0,0.0,0.0,0.0,0.2,0.57,0.69,0.76,0.76,0.49,0.0,0.0,0.0,0.0,0.0,0.36,0.0].to_vec(), 
     [0.0,0.58,0.19,0.0,0.0,0.0,0.0,0.0,0.67,0.83,0.9,0.92,0.87,0.12,0.0,0.0,0.0,0.0,0.22,0.07].to_vec(), 
     [0.0,0.0,0.46,0.0,0.0,0.0,0.0,0.0,0.7,0.93,1.0,1.0,1.0,0.61,0.0,0.0,0.0,0.0,0.18,0.11].to_vec(), 
     [0.0,0.0,0.82,0.0,0.0,0.0,0.0,0.0,0.47,1.0,1.0,0.98,1.0,0.96,0.27,0.0,0.0,0.0,0.19,0.1].to_vec(), 
     [0.0,0.0,0.46,0.0,0.0,0.0,0.0,0.0,0.25,1.0,1.0,0.84,0.92,0.97,0.54,0.14,0.04,0.1,0.21,0.05].to_vec(), 
     [0.0,0.0,0.0,0.4,0.0,0.0,0.0,0.0,0.09,0.8,1.0,0.82,0.8,0.85,0.63,0.31,0.18,0.19,0.2,0.01].to_vec(), 
     [0.0,0.0,0.0,0.36,0.1,0.0,0.0,0.0,0.05,0.54,0.86,0.79,0.74,0.72,0.6,0.39,0.28,0.24,0.13,0.0].to_vec(), 
     [0.0,0.0,0.0,0.01,0.3,0.07,0.0,0.0,0.08,0.36,0.64,0.7,0.64,0.6,0.51,0.39,0.29,0.19,0.04,0.0].to_vec(), 
     [0.0,0.0,0.0,0.0,0.1,0.24,0.14,0.1,0.15,0.29,0.45,0.53,0.52,0.46,0.4,0.31,0.21,0.08,0.0,0.0].to_vec(), 
     [0.0,0.0,0.0,0.0,0.0,0.08,0.21,0.21,0.22,0.29,0.36,0.39,0.37,0.33,0.26,0.18,0.09,0.0,0.0,0.0].to_vec(), 
     [0.0,0.0,0.0,0.0,0.0,0.0,0.03,0.13,0.19,0.22,0.24,0.24,0.23,0.18,0.13,0.05,0.0,0.0,0.0,0.0].to_vec(), 
     [0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.02,0.06,0.08,0.09,0.07,0.05,0.01,0.0,0.0,0.0,0.0,0.0].to_vec()]
}

fn random_square(n: usize) -> Vec<Vec<f64>>{
    let mut rng = rand::thread_rng();

    let mut result: Vec<Vec<f64>> = Vec::with_capacity(n);
    for _i in 0..n{
        let mut ligne: Vec<f64> = Vec::with_capacity(n);
        for _j in 0..n{
           ligne.push(rng.gen::<f64>());
        }
        result.push(ligne);
    }
    result
}

pub fn kernel_init(k_type: Kernel, h: usize) -> Vec<Vec<f64>>{
    match k_type{
        Kernel::Ring => {
            return ring_kernel(h)
        }
    }
}


fn ring_kernel(h: usize) -> Vec<Vec<f64>>{
    let mut result = vec![vec![0.0 ; h]; h];

    let rayon = h/2;
    let mut sum = 0.0;
    
    for x in 0..h{
        for y in 0..h {
            let dx;
            let dy;
            if x > rayon { dx =  x-rayon}
            else { dx = rayon-x}
            if y > rayon { dy =  y-rayon}
            else { dy = rayon-y}

            let distance = ((dx*dx + dy*dy) as f64).sqrt()/(rayon as f64);
            if distance <= 1.0 {
                let d_gauss = gaussian(0.5,0.15,distance);
                sum += d_gauss;
                result[x][y] = d_gauss;
            }
        }
    }
    

    for i in 0..h{
        for j in 0..h{
            result[i][j] /= sum;
        }
    }

    /*
    
    sum = 0.0;

    for i in 0..h{
        for j in 0..h{
            sum += result[i][j];
        }
    }
    
    println!("{}\n", sum);
     */

    result
}
