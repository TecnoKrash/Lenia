use std::time::SystemTime;
use rustfft::{FftPlanner, num_complex::Complex};
use convolutions_rs::convolutions::*;
use ndarray::*;
use convolutions_rs::Padding;

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

    // println!("conv:");
    // println!("fp1 : {:?}\n fp2 {:?}\n", fp1, fp2);

    let n = fp1.len() as u32;
    let np = f64::from(n);

    // println!("n : {}\n", n);

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


fn complex_vec(p: &mut Vec<f64>) -> Vec<Complex<f64>>{
    let mut result: Vec<Complex<f64>> = Vec::with_capacity(p.len());
    for i in 0..p.len(){
        result.push(Complex{ re: p[i], im : 0.0f64});
    }
    result
}


pub fn fast_convolution_2d(p1: &mut Vec<f64>, p2: &mut Vec<f64>) -> Vec<f64>{


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

    // println!("p1.len() : {}\n", p1.len());

    let mut cp1 = complex_vec(p1);
    let mut cp2 = complex_vec(p2);

    
    // println!("cp1.len() : {}\n", cp1.len());

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(cp1.len());

    fft.process(&mut cp1);
    fft.process(&mut cp2);

    // println!("Fast_conv:");
    // println!("cp1 : {:?}\n cp2 {:?}\n", cp1, cp2);

    let mut result_c: Vec<Complex<f64>> = cp1.iter().zip(cp2.iter())
        .map(|(a, b)| a * b)
        .collect();

    let ifft = planner.plan_fft_inverse(result_c.len());

    ifft.process(&mut result_c);

    let n = result_c.len();
    // println!("n : {}\n", n);

    let mut result: Vec<f64> = Vec::with_capacity(n);

    for i in 0..(lp1+lp2 -1){
        result.push((result_c[i]).re/(n as f64));
    }

    result

    
}



pub fn tore_format(f: &Vec<Vec<f64>>, kernel: &Vec<Vec<f64>>) -> Vec<Vec<f64>>{

    let hf = f.len();
    let lf = f[0].len();
    let mk = kernel[0].len()/2;

    // println!("f.len() : {}, mk : {}\n", f.len(), mk);

    let mut t = Vec::with_capacity(hf + 2*mk);

    for _i in 0..mk{
        t.push(vec![0.]); 
    }

    for i in 0..hf{
        let mut ti = Vec::with_capacity(lf + 2*mk);

        for j in (lf - mk)..(2*lf + mk){
            ti.push(f[i][j%lf]);
        } 

        t.push(ti);
    }

    for i in 0..mk{
        t[i] = t[hf+i].clone();
        t.push(t[i+mk].clone()); 
    }

    t
}

pub fn linearisation(m: & Vec<Vec<f64>>, size: usize) -> Vec<f64>{

    let mut result = Vec::with_capacity(size*size);

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


pub fn convolution_3d(f: &mut Vec<Vec<f64>>, kernel: &Vec<Vec<f64>>){

    let hf = f.len();
    let lf = f[0].len();
    let lk = kernel.len();
    let mk = lk/2;

    let t1 = SystemTime::now();

    let mut t = linearisation(f, f[0].len());
    let t2 = SystemTime::now();
    let mut k = linearisation(& kernel, f[0].len());
    let t3 = SystemTime::now();

    let d1 = t2.duration_since(t1).unwrap();
    let d2 = t3.duration_since(t2).unwrap();

    // println!("linearisation : t {:?}, k {:?}\n", d1, d2);

    // println!("{:?}\n", t);
    //println!("{}",k.len());


    let t4 = SystemTime::now();
    let conv = fast_convolution_2d(&mut t,&mut k);
    let t5 = SystemTime::now();

    let  d3 = t5.duration_since(t4).unwrap();

    // println!("conv2D : {:?}\n", d3);

    // println!("conv : {:?}", conv);

    let si = SystemTime::now();

    let start = 2*mk*(lf + 1) ;
    let end  = start + lf*(hf - 2*mk -1) -1 + lf-2*mk;

    // println!("conv[mk*(lf+1)] : {}\n", conv[mk*(lf+1)]);
    // println!("conv[mk*(lf+1)-1] : {}\n", conv[mk*(lf+1)-1]);
    // println!("mk*(lf+1) : {}\n", mk*(lf+1));

    // println!("start : {}, conv[start] : {}\n", start, conv[start]);
    // println!("conv[46] : {}\n", conv[46]);
    // println!("conv[47] : {}\n", conv[47]);
    // println!("conv[48] : {}\n", conv[48]);
    // println!("end : {}, conv[end] : {}\n", end, conv[end]);
     
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

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+mk][j+mk]);

        f[i+mk][j+mk] = conv[k];
        k += 1;
    }

    /*

    
    // Botom right corner
    k = start;
    loop{
        let i = (k-start)/lf;
        let j = (k-start)%lf;

        if i == mk {break}

        if j == mk{
            k += lf-mk;
            continue
        }

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+lf-mk][j+mk]);

        f[i+hf-mk][j+lf-mk] = conv[k];
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

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+lf-mk][j+mk]);

        f[i+hf-mk][j+mk] = conv[k];
        k += 1;
    }


    // Botom left corner 
    k = start + lf - 3*mk;
    loop{
        let i = (k + 3*mk - start - lf)/lf;
        let j = (k + 3*mk - start - lf)%lf;

        if i == mk {break}

        if j == mk{
            k += lf-mk;
            continue
        }

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+lf-mk][j]);

        f[i+hf-mk][j] = conv[k];
        k += 1;
    }


    // left center
    k = start + lf - 3*mk;
    loop{
        let i = (k + 3*mk - start - lf)/lf;
        let j = (k + 3*mk - start - lf)%lf;

        if i == lf-2*mk {break}

        if j == mk{
            k += lf-mk;
            continue
        }

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+mk][j]);

        f[i+mk][j] = conv[k];
        k += 1;
    }

    // up left corner
    k = end - (mk-1)*lf - mk;
    loop{
        let i = (k + (mk-1)*lf + mk - end)/lf;
        let j = (k + (mk-1)*lf + mk - end)%lf;

        if i == mk {break}

        if j == mk{
            k += lf-mk;
            continue
        }

        //println!("k : {}", k);
        //println!("i : {}", i);
        //println!("j : {}", j);
        //println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i][j]);

        f[i][j] = conv[k];
        k += 1;
    }

    // up middle
    k = end - mk*lf +2*mk;
    loop{
        let i = (k + mk*lf - 2*mk - end)/lf;
        let j = (k + mk*lf - 2*mk - end)%lf;

        if i == mk {break}

        if j == lf-2*mk{
            k += 2*mk;
            continue
        }

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i][j+mk]);

        f[i][j+mk] = conv[k];
        k += 1;
    }

    // up rigth corner
    k = end - mk*lf +2*mk;
    loop{
        let i = (k + mk*lf - 2*mk - end)/lf;
        let j = (k + mk*lf - 2*mk - end)%lf;

        if i == mk {break}

        if j == mk{
            k += lf - mk;
            continue
        }

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i][j+lf-mk]);

        f[i][j+lf-mk] = conv[k];
        k += 1;
    }

    // rigth middel
    k = start;
    loop{
        let i = (k - start)/lf;
        let j = (k - start)%lf;

        if i == lf-2*mk {break}

        if j == mk{
            k += lf - mk;
            continue
        }

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+mk][j+lf-mk]);

        f[i+mk][j+lf-mk] = conv[k];
        k += 1;

    }

    */
    
    let sf = SystemTime::now();

    let duration = sf.duration_since(t1).unwrap();

    // println!("durée conv3D {:?}\n", duration);
}

pub fn rust_conv(t: &mut Vec<f64>, k: &mut Vec<f64>, t_height: usize, t_width: usize, k_size: usize ) -> Vec<f64>{

    // println!("t_height : {}, t_width {}\n", t_height, t_width);

    // println!("tb.len() : {}\n tb : {:?}\n", tb.len(), tb);

    // Input has shape (channels, height, width)
    let t1 = SystemTime::now();
    let input = Array::from_shape_vec((1,t_height, t_width),t.to_vec()).unwrap();

    // Kernel has shape (channels out, channels in, height, width)
    let t2 = SystemTime::now();
    let kernel: Array4<f64> = Array::from_shape_vec((1,1,k_size,k_size),k.to_vec()).unwrap();

    let t3 = SystemTime::now();
    let conv_layer = ConvolutionLayer::new(kernel.clone(), None, 1, Padding::Valid);
    let t4 = SystemTime::now();
    let output_layer: Array3<f64> = conv_layer.convolve(&input);
    //let output_free = conv2d(&kernel, None, &input, Padding::Valid, 1);

    // println!("Layer: {:?}", output_layer);
    // println!("Free: {:?}", output_free);
    // println!("test: {:?}", output_layer.into_raw_vec());
    let t5 = SystemTime::now();

    let d1 = t2.duration_since(t1).unwrap();
    let d2 = t3.duration_since(t2).unwrap();
    let d3 = t4.duration_since(t3).unwrap();
    let d4 = t5.duration_since(t4).unwrap();

    println!("rust_conv : input {:?}, kernel {:?}, conv_layer {:?}, output_layer {:?}\n", d1, d2, d3, d4);

    output_layer.into_raw_vec()
}


pub fn convolution_3d_v2(f: &mut Vec<Vec<f64>>, kernel: &Vec<Vec<f64>>){

    let hf = f.len();
    let lf = f[0].len();
    let lk = kernel.len();
    let mk = lk/2;

    let t1 = SystemTime::now();

    let mut t = linearisation(f, f.len());
    let t2 = SystemTime::now();
    let mut k = linearisation(& kernel, f.len());
    let t3 = SystemTime::now();

    let d1 = t2.duration_since(t1).unwrap();
    let d2 = t3.duration_since(t2).unwrap();

    println!("linearisation : t {:?}, k {:?}\n", d1, d2);

    // println!("{:?}\n", k);
    // println!("{}",k.len());
    
    let mut kb = linearisation(& kernel, lk);

    let t4 = SystemTime::now();
    let conv = rust_conv(&mut t, &mut kb, hf, lf, lk);
    let t5 = SystemTime::now();

    let  d3 = t5.duration_since(t4).unwrap();

    // println!("conv : {:?}\n", conv);

    println!("conv2D : {:?}\n", d3);

    //println!("conv2D : {:?}\n", d3);
    let si = SystemTime::now();

    let lt = lf - 2*mk;
    let end = conv.len();

    // println!("conv[mk*(lf+1)] : {}\n", conv[mk*(lf+1)]);
    // println!("conv[mk*(lf+1)-1] : {}\n", conv[mk*(lf+1)-1]);
    // println!("mk*(lf+1) : {}\n", mk*(lf+1));

    // println!("start : {}, conv[start] : {}\n", start, conv[start]);
    // println!("conv[46] : {}\n", conv[46]);
    // println!("conv[47] : {}\n", conv[47]);
    // println!("conv[48] : {}\n", conv[48]);
    // println!("end : {}, conv[end] : {}\n", end, conv[end]);
     
    // Interior of the tore
    let mut k:usize = 0;
    loop{
        if k >= end{break}

        let i = k/lt;
        let j = k%lt;

        // if j == (lf - 2*mk){
        //     k += 2*mk;
        //      continue
        // }

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+mk][j+mk]);

        f[i+mk][j+mk] = conv[k];
        k += 1;
    }

    

    
    // Botom right corner
    k = 0;
    loop{
        let i = k/lt;
        let j = k%lt;

        if i == mk {break}

        if j == mk{
            k += lt-mk;
            continue
        }

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+lf-mk][j+mk]);

        f[i+lf-mk][j+lf-mk] = conv[k];
        k += 1;

    }

    

    // Botom center 
    k = 0;
    loop{
        if k >= mk*lt {break}

        let i = k/lt;
        let j = k%lt;

        // if j == (lf - 2*mk){
        //     k += 2*mk;
        //     continue
        // }

        //  println!("k : {}", k);
        //  println!("i : {}", i);
        //  println!("j : {}", j);
        //  println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+lf-mk][j+mk]);

        f[i+lf-mk][j+mk] = conv[k];
        k += 1;
    }

    

    // Botom left corner 
    k = lt-mk;
    loop{
        let i = (k + mk - lt)/lt;
        let j = (k + mk - lt)%lt;

        if i == mk {break}

        if j == mk{
            k += lt-mk;
            continue
        }

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+lf-mk][j]);

        f[i+lf-mk][j] = conv[k];
        k += 1;
    }

    

    // left center
    k = lt-mk;
    loop{
        let i = (k + mk - lt)/lt;
        let j = (k + mk - lt)%lt;

        if i == lt {break}

        if j == mk{
            k += lt-mk;
            continue
        }

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+mk][j]);

        f[i+mk][j] = conv[k];
        k += 1;
    }

    

    // up left corner
    k = end - (mk-1)*lt - mk;
    loop{
        let i = (k + (mk-1)*lt + mk - end)/lt;
        let j = (k + (mk-1)*lt + mk - end)%lt;

        if i == mk {break}

        if j == mk{
            k += lt-mk;
            continue
        }

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i][j]);

        f[i][j] = conv[k];
        k += 1;
    }

    

    // up middle
    k = end - mk*lt;
    loop{
        let i = (k + mk*lt - end)/lt;
        let j = (k + mk*lt - end)%lt;

        if i == mk {break}

        // if j == lf-2*mk{
        //     k += 2*mk;
        //     continue
        // }

        //  println!("k : {}", k);
        //  println!("i : {}", i);
        //  println!("j : {}", j);
        //  println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i][j+mk]);

        f[i][j+mk] = conv[k];
        k += 1;
    }

    

    // up rigth corner
    k = end - mk*lt;
    loop{
        let i = (k + mk*lt - end)/lt;
        let j = (k + mk*lt - end)%lt;

        if i == mk {break}

        if j == mk{
            k += lt - mk;
            continue
        }

        //println!("k : {}", k);
        //println!("i : {}", i);
        //println!("j : {}", j);
        //println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i][j+lf-mk]);

        f[i][j+lf-mk] = conv[k];
        k += 1;
    }

    

    // rigth middel
    k = 0;
    loop{
        let i = k/lt;
        let j = k%lt;

        if i == lt {break}

        if j == mk{
            k += lt - mk;
            continue
        }

        // println!("k : {}", k);
        // println!("i : {}", i);
        // println!("j : {}", j);
        // println!("conv[k] : {}, f[i][j] : {}\n", conv[k], f[i+mk][j+lf-mk]);

        f[i+mk][j+lf-mk] = conv[k];
        k += 1;

    }


    let sf = SystemTime::now();

    let duration = sf.duration_since(si).unwrap();

    println!("conv3Dv2 rewrite {:?}\n", duration);
}


