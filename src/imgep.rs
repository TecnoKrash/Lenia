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
    pub fn _scal(mut self, l: f64){
        self.co = self.co.iter().map(|x| l*x).collect();
    }
}

#[derive(Debug)]
pub struct Param {
    pub nb_kernels: usize,      // Number of used kernels
    pub mu: Vec<f64>,           // Gaussian parameter for the growth functions
    pub sigma: Vec<f64>,        // *
    pub nb_bump: Vec<usize>,    // Number of rings for each kernels
    pub gr: usize,              // Global radius of the final kernel
    pub r: Vec<f64>,            // Proportion of the big radius used by each kernels (r[i]*gr = radius of the i th kernel)
    pub a: Vec<Vec<f64>>,       // proportion of the specific radius for each bump of every kernels
    pub w: Vec<Vec<f64>>,       // Width of each bump for every kernels
    pub b: Vec<Vec<f64>>,       // Hight of each bump for every kernels
    pub h: Vec<f64>,            // Hight of the growth function for every kernels
    pub c: Vec<(usize,usize)>,  // input and output channels use by each kernels
}

// Init of the parameters depending on the settings
impl Param {
    pub fn param_init(set: &Settings) -> Param{
        match set.mode {
            Mode::Lenia =>  {
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
                            h: vec![1.0],
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
                            h: vec![1.0],
                            c: vec![(0,0)]
                        }
                    },
                    Motif::Agent(Agent::Fish) => {
                        Param {
                            nb_kernels: 3,
                            mu: vec![0.156,0.193,0.342],
                            sigma: vec![0.0118,0.049,0.0891],
                            nb_bump: vec![3, 2, 1],
                            gr: 10,
                            r: vec![1.0, 1.0, 1.0],
                            a: vec![vec![1.0/6.0,3.0/6.0,5.0/6.0], vec![1.0/4.0, 3.0/4.0], vec![0.5]],
                            w: vec![vec![0.05,0.05,0.05], vec![0.10,0.10], vec![0.15]],
                            b: vec![vec![1.0, 5.0/12.0, 2.0/3.0], vec![1.0/12.0, 1.0], vec![1.0]],
                            h: vec![1.0, 1.0, 1.0],
                            c: vec![(0,0), (0,0), (0,0)]
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
                            h: vec![1.0],
                            c: vec![(0,0)],
                        }
                    }
                    Motif::Agent(Agent::Aquarium) => {
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
                            h: vec![1.0],
                            c: vec![(0,0)],
                        }
                    }


                }
            },
            Mode::Chan3 => {
                Param {
                    nb_kernels: 15,
                    mu: vec![0.272, 0.349, 0.2, 0.114, 0.447, 0.247, 0.21, 0.462, 0.446, 0.327, 0.476, 0.379, 0.262, 0.412, 0.201],
                    sigma: vec![0.0595, 0.1585, 0.0332, 0.0528, 0.0777, 0.0342, 0.0617, 0.1192, 0.1793, 0.1408, 0.0995, 0.0697, 0.0877, 0.1101, 0.0786],
                    nb_bump: vec![1, 1, 2, 2, 1, 2, 1, 1, 1, 2, 2, 2, 1, 2, 1],
                    gr: 12,
                    r: vec![0.91, 0.62, 0.5, 0.97, 0.72, 0.8, 0.96, 0.56, 0.78, 0.79, 0.5, 0.72, 0.68, 0.82, 0.82],
                    a: vec![vec![1.0], vec![1.0], vec![1.0/4.0, 3.0/4.0], vec![1.0/4.0, 3.0/4.0], vec![1.0], vec![1.0/4.0, 3.0/4.0], vec![1.0], vec![1.0], vec![1.0], vec![1.0/4.0, 3.0/4.0], vec![1.0/4.0, 3.0/4.0], vec![1.0/4.0, 3.0/4.0], vec![1.0], vec![1.0/4.0, 3.0/4.0], vec![1.0]],
                    w: vec![vec![0.15], vec![0.15], vec![0.10, 0.10], vec![0.10, 0.10], vec![0.15], vec![0.10, 0.10], vec![0.15], vec![0.15], vec![0.15], vec![0.10, 0.10], vec![0.10, 0.10], vec![0.10, 0.10], vec![0.15], vec![0.10, 0.10], vec![0.15]],
                    b: vec![vec![1.0], vec![1.0], vec![1.0, 1.0/4.0], vec![0.0, 1.0], vec![1.0], vec![5.0/6.0, 1.0], vec![1.0], vec![1.0], vec![1.0], vec![11.0/12.0, 1.0], vec![3.0/4.0, 1.0], vec![11.0/12.0, 1.0], vec![1.0], vec![1.0/6.0, 1.0], vec![1.0]],
                    h: vec![0.138, 0.48, 0.284, 0.256, 0.5, 0.622, 0.35, 0.218, 0.556, 0.344, 0.456, 0.67, 0.42, 0.43, 0.278],
                    c: vec![(0,0), (0,0), (0,0), (1,1), (1,1), (1,1), (2,2), (2,2), (2,2), (0,1), (0,2), (1,0), (1,2), (2,0), (2,1)],
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
                    h: vec![1.0],
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
                    h: vec![1.0],
                    c: vec![(0,0)],
                }
            },
            Mode::Learning => {
                random_param()
            }
        }
    }
}

// Attempt to find the center of mass of the agent in the field (not working on edges)
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
            }
        }
    }

    (sumx as usize, sumy as usize)
}

// Compute the sum of the values in the indicated column
fn column_sum(f: &Field, y: usize) -> f64{
    let mut res = 0.;
    for i in 0..f.h{
        res += f.m[0][i][y];
    }
    res
}

// Try to find the center position of the agent based on a point that is part of an agent (returns
// new position and the relative distance to the sides of the agent)
pub fn position(f: &Field, (x,y): (usize,usize)) -> ((usize,usize), (usize, usize)){
    let mut dist = [0; 4];

    //up
    while (column_sum(&f,(y+f.l-dist[0])%f.l) != 0.) && (dist[0] < f.l) { dist[0] += 1}
    //down
    while (column_sum(&f,(y+f.l+dist[1])%f.l) != 0.) && (dist[1] < f.l) { dist[1] += 1}
    //left
    while (f.m[0][(x+f.h-dist[2])%f.h].iter().sum::<f64>() != 0.) && (dist[2] < f.h) { dist[2] += 1}
    //right
    while (f.m[0][(x+f.h+dist[3])%f.h].iter().sum::<f64>() != 0.) && (dist[3] < f.h) { dist[3] += 1}

    (((x+f.h-dist[2])%f.h+1, (y+f.l-dist[0])%f.l+1), (dist[2]+dist[3]+1-1,dist[0]+dist[1]+1-1))
}

// Computes random parameters for the automaton
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
                    h: vec![],
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
