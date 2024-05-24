mod init;
mod convolution;
mod sdl;

use rand::prelude::*;
use std::time::SystemTime;
use crate::convolution::*;
use crate::sdl::sdl_main;

pub fn print_matrice(f: &Vec<Vec<f64>>, name: &str){
    println!("{} :", name);
    for i in 0..f.len(){
        println!("{:?}\n", f[i]);
    }
}

pub fn duration_test(){
    let mut rng = rand::thread_rng();

    let mut nums1: Vec<i32> = (0..10000).collect();
    nums1.shuffle(&mut rng);

    let mut nums2: Vec<i32> = (0..10000).collect();
    nums2.shuffle(&mut rng);

    let mut p1 = Vec::with_capacity(10000);
    let mut p2 = Vec::with_capacity(10000);

    for i in 0..10000{
        p1.push(f64::from(nums1[i]));
        p2.push(f64::from(nums2[i]));
    }

    let start = SystemTime::now(); 
    let _p = convolution_2d(& mut p1, & mut p2);
    let end = SystemTime::now();
    
    let duration = end.duration_since(start).unwrap();
    println!("it took {:?}", duration);
}

pub fn convolution_test(n : i32){
    let l = 5;

    let mut f = Vec::with_capacity(l);

    let mut rng = rand::thread_rng();

    for _i in 0..l{
        let mut ligne: Vec<f64> = Vec::with_capacity(l);
        for _j in 0..l{
            //ligne.push(rng.gen());
            let r:f64 = rng.gen::<f64>() * 10.0 ;

            ligne.push(r);
        }
        f.push(ligne);
    }

    let kernel_5 = vec![vec![0.,0.,0.,0.,0.],vec![0.,0.,0.,0.,0.],vec![0.,0.,1.,0.,0.],vec![0.,0.,0.,0.,0.],vec![0.,0.,0.,0.,0.]];
    let kernel_3 = vec![vec![0.,0.,0.],vec![0.,1.,0.],vec![0.,0.,0.]];

    print_matrice(&f, &"f");
    if n == 5{ 
        print_matrice(&kernel_5, &"kernel_5");
        let mut t = tore_format(&f,&kernel_5);
        print_matrice(&t, "t");

        convolution_3d(&mut t, kernel_5);
        print_matrice(&t, "t");
    }
    else {
        print_matrice(&kernel_3, &"kernel_3");
        let mut t = tore_format(&f,&kernel_3);
        print_matrice(&t, "t");

        convolution_3d(&mut t, kernel_3);
        print_matrice(&t, "t");
    }



    print_matrice(&f, &"f");

}

fn main() {
    sdl_main();
    // duration_test();
    // convolution_test(5);
}
