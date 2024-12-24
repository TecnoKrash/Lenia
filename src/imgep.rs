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

fn column_sum(f: &Field, y: usize) -> f64{
    let mut res = 0.;
    for i in 0..f.h{
        res += f.m[0][i][y];
    }
    res
}


pub fn position(f: &Field, (x,y): (usize,usize)) -> ((usize,usize), (usize, usize)){
    let mut dist = [0; 4];

    // println!("x: {}, y : {}\n", x, y);

    //up
    // while f.m[0][x][(y-dist[0])%f.h] != 0.{ dist[0] += 1}
    while column_sum(&f,(y+f.l-dist[0])%f.l) != 0.{ dist[0] += 1}
    //down
    // while f.m[0][x][(y+dist[1])%f.h] != 0.{ dist[1] += 1}
    while column_sum(&f,(y+f.l+dist[1])%f.l) != 0. { dist[1] += 1}
    //left
    // while f.m[0][(x-dist[2])%f.l][y] != 0.{ dist[2] += 1}
    while f.m[0][(x+f.h-dist[2])%f.h].iter().sum::<f64>() != 0.{ dist[2] += 1}
    //right
    // while f.m[0][(x+dist[3])%f.l][y] != 0.{ dist[3] += 1}
    while f.m[0][(x+f.h+dist[3])%f.h].iter().sum::<f64>() != 0.{ dist[3] += 1}

    (((x+f.h-dist[2])%f.h+1, (y+f.l-dist[0])%f.l+1), (dist[2]+dist[3]-1,dist[0]+dist[1]-1))
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

pub fn single_ring(p: & mut Param){
    
    p.gr = 25/2;
    p.r = 1.0;

    p.nb_bump = 1;

    p.a.push(0.5);
    p.w.push(0.15);
    p.b.push(1.0);

}

pub fn triple_kernel(p: & mut Param){

    p.r = 1.0;

    p.nb_bump = 3;

    p.a = vec![1.0/3.0,2.0/3.0,1.0];
    p.w = vec![0.15,0.15,0.15];
    p.b = vec![0.5, 1.0, 0.677];

}

pub fn goal_sample(goal_lib: &Vec<Vec<f64>>, dist: &Vec<f64>){
    let mut rng = rand::thread_rng();
    let close = 0;
    let very_close = 0;

    let mut target_goal: (f64, f64) = (0.1,0.2);

    while close < 1 && very_close > 2{
        let choix = rng.gen::<f64>();

        if choix < 0.2 {
            let id_best: usize = 42;
            target_goal.0 = dist[id_best] + (rng.gen::<f64>()*0.45 - 0.22)/4.0;
            target_goal.1 = dist[id_best] + (rng.gen::<f64>()*0.45 - 0.22)/4.0;
        }
        else{
            if choix < 0.7{
                target_goal.0 = (rng.gen::<f64>()*0.45 - 0.22)/4.0;
                target_goal.1 = (rng.gen::<f64>()*0.45 - 0.22)/4.0;
                
            }
        }
    }
}




