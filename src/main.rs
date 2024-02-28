mod init;
mod convolution;

use rand::prelude::*;
use std::time::SystemTime;
use crate::convolution::*;

fn main() {
    /*
    let p1 = vec![C::c(2.0), C::c(1.0), C::c(-1.0), C::c(5.0), C::c(0.0), C::c(3.0), C::c(0.0), C::c(-4.0)];
    //let p2 = vec![0.0,0.0,4.0,5.0,6.0];
    let ip1 = fft(p1, false);
    for i in &ip1{
        println!("{:?}", i);
    }

    let pr = fft(ip1, true);
    for i in &pr{
        println!("{:?}", &(C::c(1.0/8.0)*(*i)));
    }
    */
    let mut rng = rand::thread_rng();

    let mut nums1: Vec<i32> = (0..10000).collect();
    nums1.shuffle(&mut rng);

    let mut nums2: Vec<i32> = (0..10000).collect();
    nums2.shuffle(&mut rng);

    let mut p1 = vec![];
    let mut p2 = vec![];

    for i in 0..10000{
        p1.push(f64::from(nums1[i]));
        p2.push(f64::from(nums2[i]));
    }

    let start = SystemTime::now(); 
    let p = convolution_2d(p1, p2);
    let end = SystemTime::now();
    
    let duration = end.duration_since(start).unwrap();
    println!("it took {:?}", duration);
    /*
    for i in &p{
        println!("{}", i);
    }
    */
}
