use crate::convolution::*;

pub struct Field {
    pub t: f64,
    pub l: usize,
    pub h: usize,
    pub k_size: usize,
    pub nb_channels: usize,
    pub m: Vec<Vec<Vec<f64>>>,
}

impl Field {
    // A function to create an empty field
    pub fn new_field(h: usize, l: usize, nb_chan: usize) -> Field {
        Field {
            t: 0.0,
            l: l,
            h: h,
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
        self.m[chanel][self.k_size + x][self.k_size + y]
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
}
