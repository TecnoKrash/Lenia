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
    pub fn rc(x: C) -> f64{
        x.re
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

// return the same vector but adds zeros to the end to have p1.len() = 2^k
fn add_zeros(p: &mut Vec<f64>){
    let l = p.len() as u32;
    let k = u32::BITS - l.leading_zeros();

    if l != (1 << (k-1)) {
        let k2: u32 = 1 << k;
        for _i in l..k2{
            p.push(0.0);
        }
    }
}

fn c_vec(p: &mut Vec<f64>) -> Vec<C>{
    let mut result: Vec<C> = Vec::with_capacity(p.len());
    for i in 0..p.len(){
        result.push(C::c(p[i]));
    }
    result
}

pub fn convolution_2d(p1: &mut Vec<f64>, p2: &mut Vec<f64>) -> Vec<f64>{

    let lp1 = p1.len();
    let lp2 = p2.len();

    for _i in 1..lp1{
        p2.push(0.0)
    }

    for _j in 1..lp2{
        p1.push(0.0)
    }

    add_zeros(p1);
    add_zeros(p2);

    let cp1 = c_vec(p1);
    let cp2 = c_vec(p2);

    let fp1 = fft(cp1, false);
    let fp2 = fft(cp2, false);

    let n = fp1.len() as u32;
    let np = f64::from(n);

    let mut point = Vec::with_capacity(fp1.len());

    for i in 0..fp1.len(){
        point.push(fp1[i]*fp2[i]);
    }

    let result_c = fft(point, true);
    let mut result: Vec<f64> = Vec::with_capacity(fp1.len());

    for k in 0..(lp1+lp2 - 1){
        result.push((1.0/np)*result_c[k].re);
    }
    result
}


pub fn tore_format(f: &Vec<Vec<f64>>, kernel: &Vec<Vec<f64>>) -> Vec<Vec<f64>>{

    let lf = f[0].len();
    let mk = kernel[0].len()/2;

    println!("f.len() : {}, mk : {}\n", f.len(), mk);

    let mut t = Vec::with_capacity(f.len() + 2*mk);

    for _i in 0..mk{
        t.push(vec![0.]); 
    }

    for i in 0..lf{
        let mut ti = Vec::with_capacity(lf + 2*mk);

        for j in (lf - mk)..(2*lf + mk){
            ti.push(f[i][j%lf]);
        } 

        t.push(ti);
    }

    for i in 0..mk{
        t[i] = t[f.len()+i].clone();
        t.push(t[i+mk].clone()); 
    }

    t
}

pub fn linearisation(m: & Vec<Vec<f64>>, size: usize) -> Vec<f64>{

    let mut result = Vec::with_capacity(size);

    for i in 0..m.len(){
       for j in 0..size{
            if j < m[i].len(){
                result.push(m[i][j]);
                continue
            }
            result.push(0.0); 
       } 
    }

    result
}

pub fn convolution_3d(f: &mut Vec<Vec<f64>>, kernel: Vec<Vec<f64>>){

    let lf = f[0].len();
    let mk = kernel[0].len()/2;

    let mut t = linearisation(f, f.len());
    let mut k = linearisation(& kernel, f.len());

    //println!("{:?}\n", t);
    //println!("{}",t.len());

    let conv = convolution_2d(&mut t,&mut k);
    
    println!("{:?}\n", conv);
    //println!("{}\n", conv.len());
    let start = 2*mk*(lf+1);
    let end  = start + (f.len()-1-2*mk)*lf + lf - 2*mk;

    println!("conv[mk*(lf+1)] : {}\n", conv[mk*(lf+1)]);
    println!("conv[mk*(lf+1)-1] : {}\n", conv[mk*(lf+1)-1]);
    println!("mk*(lf+1) : {}\n", mk*(lf+1));

    println!("start : {}, conv[start] : {}\n", start, conv[start]);
    println!("conv[46] : {}\n", conv[46]);
    println!("conv[47] : {}\n", conv[47]);
    println!("conv[48] : {}\n", conv[48]);
    println!("end : {}, conv[end] : {}\n", end, conv[end]);
     
    // Interior of the tore
    let mut k:usize = start;
    loop{
        if k >= end{break}

        let i = (k-start)/lf;
        let j = (k-start)%lf;

        if j == (lf - 2*mk){
            k += 2*mk;
             continue
        }

        //println!("k : {}", k);
        //println!("i : {}", i);
        //println!("j : {}", j);
        //println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+mk][j+mk]);

        f[i+mk][j+mk] = conv[k];
        k += 1;
    }

    
    // Botom right corner
    k = start;
    loop{
        let i = (k-start)/lf;
        let j = (k-start)%(mk+1);

        if i == mk {break}

        if j == mk{
            k += lf-mk;
            continue
        }

        //println!("k : {}", k);
        //println!("i : {}", i);
        //println!("j : {}", j);
        //println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+mk][j+mk]);

        f[i+lf-mk][j+lf-mk] = conv[k];
        k += 1;

    }

    
    // Botom center 
    k = start;
    loop{
        if k-start >= mk*lf {break}

        let i = (k-start)/lf;
        let j = (k-start)%lf;

        if j == (lf - 2*mk){
            k += 2*mk;
            continue
        }

        //println!("k : {}", k);
        //println!("i : {}", i);
        //println!("j : {}", j);
        //println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+mk][j+mk]);

        f[i+lf-mk][j+mk] = conv[k];
        k += 1;
    }

    // Botom left corner 
    k = start + lf - 3*mk;
    loop{
        let i = (k + 3*mk - start - lf)/mk;
        let j = (k + 3*mk - start - lf)%mk;

        if i == mk {break}

        if j == mk{
            k += lf;
            continue
        }

        println!("k : {}", k);
        println!("i : {}", i);
        println!("j : {}", j);
        println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+mk][j+mk]);

        f[i+lf-mk][j] = 0.;
        k += 1;
    }

    /*

    // left center
    k = start + lf - 3*mk;
    loop{
        let i = (k + 3*mk - start - lf)/mk;
        let j = (k + 3*mk - start - lf)%mk;

        //println!("k : {}\n", k);
        //println!("i : {}\n", i);
        //println!("j : {}\n", j);

        if i == lf-2*mk {break}

        if j == mk{
            k += lf-mk;
            continue
        }

        f[i+mk][j] = conv[k];
        k += 1;
    }
    */ 
}