use std::f64::consts::PI;


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

pub fn fft(p: Vec<f64>, invert: bool) -> Vec<f64>{

    let n: i32 = p.len() as i32;

    if n == 1 {
        return p;
    }

    let np = f64::from(n);

    let w: f64;

    if invert {
        w = (1.0/np)*(- (2.0 * PI)/np).exp();
    } 

    else{
        w = ((2.0 * PI)/np).exp();
    }

    let mut pe: Vec<f64> = vec![];
    let mut po: Vec<f64> = vec![];

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
    let mut y = vec![0.0; p.len()];

    let n2 = (n/2) as usize;

    for j in 0..(n2){
        y[j] = ye[j] + expo(w,j as u32)*yo[j];
        y[j + n2] = ye[j] - expo(w,j as u32)*yo[j];
    } 
    y
}