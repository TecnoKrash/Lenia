use std::f64::consts::PI;
use std::ops::{Add, Sub, Mul};
#[derive(Debug)]

pub struct C{
    pub re: f64,
    pub im: f64,
}

impl Add for C{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}
impl Sub for C{
    type Output = Self;

    fn sub(self, other: Self) -> Self{
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}
impl Mul for C{
    type Output = Self;

    fn mul(self, other: Self) -> Self{
        Self {
            re: self.re*other.re - self.im*other.im,
            im: self.im*other.re + other.im*self.re,
        }
    }
}

impl Copy for C{ }

impl Clone for C{
    fn clone(&self) -> Self{
        *self
    }
}

impl C{
    pub fn c(x: f64) -> C{
        C {
            re: x,
            im: 0.0,
        }
    }
}

fn expo(x: f64, p: u32) -> f64{
    if p == 0{
        return 1.0;
    }
    if p == 1{
        return x;
    }
    let e2 = expo(x, p/2);
    if p % 2 == 0{
        e2*e2
    }
    else{
        e2*e2*x
    }
}

// Apply the fast fourier transform to the list in entry
pub fn fft(p: Vec<C>, invert: bool) -> Vec<C>{

    let n: i32 = p.len() as i32;

    if n == 1 {
        return p;
    }

    let np = f64::from(n);
    let n2 = (n/2) as usize;

    let mut w: Vec<C> = vec![];

    let mut theta = (2.0 * PI)/np;

    if invert {
        theta = -theta; 
    } 

    for j in 0..n2{
        w.push(C {
            re: (theta*(f64::from(j as i32))).cos(),
            im: (theta*(f64::from(j as i32))).sin(),
        })
    }

    let mut pe: Vec<C> = vec![];
    let mut po: Vec<C> = vec![];

    for i in 0..(p.len()){
        if i % 2 == 0 {
            pe.push(p[i]);
        }
        else {
            po.push(p[i]);
        }   
    }

    let ye = fft(pe, invert);
    let yo = fft(po, invert);
    let mut y = vec![C {re: 0.0, im: 0.0}; p.len()];

    for j in 0..(n2){
        y[j] = ye[j] +w[j]*yo[j];
        y[j + n2] = ye[j] - w[j]*yo[j];
    } 
    y
}


//pub fn 2D-convolution(p1: Vec<f64>, p2: Vec<f64>) -> Vec<f64>{}