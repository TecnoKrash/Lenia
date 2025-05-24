use rand::prelude::*;
use std::ops::Add;

use crate::sdl::*;
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

#[derive(Debug)]
pub struct Param {
    pub nb_kernels: usize,
    pub mu: Vec<f64>,
    pub sigma: Vec<f64>,
    pub nb_bump: Vec<usize>,
    pub gr: usize,
    pub r: Vec<f64>,
    pub a: Vec<Vec<f64>>,
    pub w: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<(usize,usize)>,
}

impl Param {
    pub fn param_init(set: &Settings) -> Param{
        match set.mode {
            Mode::Lenia | Mode::Chan3 => {
                match set.motif {
                    Motif::Agent(Agent::Orbium) => {
                        Param {
                            nb_kernels: 1,
                            mu: vec![0.15],
                            sigma: vec![0.017],
                            nb_bump: vec![1],
                            gr: 25/2,
                            r: vec![1.0],
                            a: vec![vec![0.5]],
                            w: vec![vec![0.15]],
                            b: vec![vec![1.0]],
                            c: vec![(0,0)],
                        }
                    },
                    Motif::Agent(Agent::Hydrogeminium) => {
                        Param {
                            nb_kernels: 1,
                            mu: vec![0.26],
                            sigma: vec![0.036],
                            nb_bump: vec![3],
                            gr: 18,
                            r: vec![1.0],
                            a: vec![vec![1.0/6.0,3.0/6.0,5.0/6.0]],
                            w: vec![vec![0.05,0.05,0.05]],
                            b: vec![vec![0.5, 1.0, 0.667]],
                            c: vec![(0,0)]
                        }
                    },
                    Motif::Rand(_h, _l) => {
                        Param {
                            nb_kernels: 1,
                            mu: vec![0.15],
                            sigma: vec![0.017],
                            nb_bump: vec![1],
                            gr: 25/2,
                            r: vec![1.0],
                            a: vec![vec![0.5]],
                            w: vec![vec![0.15]],
                            b: vec![vec![1.0]],
                            c: vec![(0,0)],
                        }
                    }

                }
            },
            Mode::Smooth => {
                Param {
                    nb_kernels: 1,
                    mu: vec![0.31],
                    sigma: vec![0.049],
                    nb_bump: vec![1],
                    gr: 25/2,
                    r: vec![1.0],
                    a: vec![vec![0.5]],
                    w: vec![vec![0.20]],
                    b: vec![vec![1.0]],
                    c: vec![(0,0)],
                }
            },
            Mode::Gol => {
                Param {
                    nb_kernels: 1,
                    mu: vec![0.35],
                    sigma: vec![0.07],
                    nb_bump: vec![1],
                    gr: 2,
                    r: vec![1.0],
                    a: vec![],
                    w: vec![],
                    b: vec![],
                    c: vec![(0,0)],
                }
            },
            Mode::Learning => {
                random_param()
            }
        }
    }
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
    while (column_sum(&f,(y+f.l-dist[0])%f.l) != 0.) && (dist[0] < f.l) { dist[0] += 1}
    //down
    // while f.m[0][x][(y+dist[1])%f.h] != 0.{ dist[1] += 1}
    while (column_sum(&f,(y+f.l+dist[1])%f.l) != 0.) && (dist[1] < f.l) { dist[1] += 1}
    //left
    // while f.m[0][(x-dist[2])%f.l][y] != 0.{ dist[2] += 1}
    while (f.m[0][(x+f.h-dist[2])%f.h].iter().sum::<f64>() != 0.) && (dist[2] < f.h) { dist[2] += 1}
    //right
    // while f.m[0][(x+dist[3])%f.l][y] != 0.{ dist[3] += 1}
    while (f.m[0][(x+f.h+dist[3])%f.h].iter().sum::<f64>() != 0.) && (dist[3] < f.h) { dist[3] += 1}

    (((x+f.h-dist[2])%f.h+1, (y+f.l-dist[0])%f.l+1), (dist[2]+dist[3]+1-1,dist[0]+dist[1]+1-1))
}


pub fn random_param() -> Param{
    let mut p = Param {
                    nb_kernels: 1,
                    mu: vec![],
                    sigma: vec![],
                    nb_bump: vec![3],
                    gr: 25/2,
                    r: vec![1.0],
                    a: vec![],
                    w: vec![],
                    b: vec![],
                    c: vec![],
                };

    let mut rng = rand::thread_rng();
    
    for _i in 0..p.nb_kernels{
        p.mu.push(rng.gen::<f64>());
        p.sigma.push(rng.gen::<f64>());
        p.r.push(rng.gen::<f64>());
    }

    p.a = Vec::with_capacity(p.nb_kernels);
    p.w = Vec::with_capacity(p.nb_kernels);
    p.b = Vec::with_capacity(p.nb_kernels);

    for i in 0..p.nb_kernels{
        p.a.push(Vec::with_capacity(p.nb_kernels));
        p.w.push(Vec::with_capacity(p.nb_kernels));
        p.b.push(Vec::with_capacity(p.nb_kernels));

        for _j in 0..p.nb_bump[i]{
            p.a[i].push(rng.gen::<f64>());
            p.b[i].push(rng.gen::<f64>());
            p.w[i].push(rng.gen::<f64>()*(0.5 - 0.01) + 0.01);
        }
    }

    p
}

/*
pub fn single_ring(p: & mut Param){
    
    p.gr = 25/2;
    p.r = 1.0;

    p.nb_bump = 1;

    p.a.push(0.5);
    p.w.push(0.15);
    p.b.push(1.0);

    p.mu = 0.15;
    p.sigma = 0.017;
}

pub fn triple_ring(p: & mut Param){

    p.r = 1.0;

    p.nb_bump = 3;

    p.a = vec![1.0/6.0,3.0/6.0,5.0/6.0];
    p.w = vec![0.05,0.05,0.05];
    p.b = vec![0.5, 1.0, 0.677];

    p.mu = 0.26;
    p.sigma = 0.036;

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

*/



