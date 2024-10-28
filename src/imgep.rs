use rand::prelude::*;
use std::ops::Add;

use crate::init::*;

pub struct Vector {
    pub dim: usize,
    pub co: Vec<f64>,
}


impl Add for Vector{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        Self {
            dim :self.dim,
            co: self.co.iter().zip(other.co.iter()).map(|(a,b)| a+b).collect(),
        }
    }
}

impl Vector {
    pub fn scal(mut self, l: f64){
        self.co = self.co.iter().map(|x| l*x).collect();
    }
}

pub struct Param {
    pub mu: f64,
    pub sigma: f64,
    pub nb_bump: usize,
    pub gr: usize,
    pub r: f64,
    pub a: Vec<f64>,
    pub w: Vec<f64>,
    pub b: Vec<f64>,
}


pub fn mass_center(f: &Field) -> (usize,usize){
    let sum: f64 = f.m.iter().flat_map(|c| c.iter().flat_map(|x| x.iter())).sum();

    let mut sumx = 0.;
    let mut sumy = 0.;
    for c in 0..f.nb_channels{
        for x in 0..f.h{
            for y in 0..f.l{
                let xy: f64 = f.m[c][x][y]/sum;
                sumx += xy*(x as f64);
                sumy += xy*(y as f64);
                // if f.m[c][x][y] > 0.0{
                //     println!("x: {}, y: {}, xy: {}, sumi: {}\n", x, y, xy, sumi);
                // }
            }
        }
    }

    // let x: f64 = (0..f.nb_channels).zip((0..f.h).zip(0..f.l)).map(|(c,(x,y))| (f.m[c][x][y]/sum)*(x as f64)).sum();
    // let y: f64 = (0..f.nb_channels).zip((0..f.h).zip(0..f.l)).map(|(c,(x,y))| (f.m[c][x][y]/sum)*(y as f64)).sum();

    // println!("sum: {}, x: {}, y: {}\n", sum, sumx, sumy);

    (sumx as usize, sumy as usize)
}

pub fn random_param(p: & mut Param){
    let mut rng = rand::thread_rng();
    
    p.mu = rng.gen::<f64>();
    p.sigma = rng.gen::<f64>();
    p.r = rng.gen::<f64>();

    p.a = Vec::with_capacity(p.nb_bump);
    p.w = Vec::with_capacity(p.nb_bump);
    p.b = Vec::with_capacity(p.nb_bump);

    for _i in 0..p.nb_bump{
        p.a.push(rng.gen::<f64>());
        p.b.push(rng.gen::<f64>());
        p.w.push(rng.gen::<f64>()*(0.5 - 0.01) + 0.01);
    }
}
