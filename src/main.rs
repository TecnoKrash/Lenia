mod init;
mod convolution;
mod sdl;
mod growth;
mod file;

use rand::prelude::*;
use std::time::SystemTime;
use crate::convolution::*;
use crate::sdl::*;
use crate::init::*;
use crate::growth::*;
use crate::file::*;

pub fn print_matrice(f: &Vec<Vec<f64>>, name: &str){
    println!("{} :", name);
    for i in 0..f.len(){
        println!("{:?}\n", f[i]);
    }
}

pub fn duration_test(){
    let mut rng = rand::thread_rng();

    let mut nums1: Vec<i32> = (0..100000).collect();
    nums1.shuffle(&mut rng);

    let mut nums2: Vec<i32> = (0..100000).collect();
    nums2.shuffle(&mut rng);

    let mut p1 = Vec::with_capacity(100000);
    let mut p2 = Vec::with_capacity(100000);

    for i in 0..100000{
        p1.push(f64::from(nums1[i]));
        p2.push(f64::from(nums2[i]));
    }

    let start = SystemTime::now(); 
    let _p = fast_convolution_2d(& mut p1, & mut p2);
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

    let kernel_5 = vec![vec![1.,1.,1.,1.,1.],vec![1.,1.,1.,1.,1.],vec![1.,1.,1.,1.,1.],vec![1.,1.,1.,1.,1.],vec![1.,1.,1.,1.,1.]];
    let kernel_3 = vec![vec![0.,0.,0.],vec![0.,1.,0.],vec![0.,0.,0.]];

    print_matrice(&f, &"f");
    if n == 5{ 
        print_matrice(&kernel_5, &"kernel_5");
        let mut t = tore_format(&f,&kernel_5);
        print_matrice(&t, "t");

        convolution_3d_v2(&mut t, &kernel_5);
        print_matrice(&t, "t");
    }
    else {
        print_matrice(&kernel_3, &"kernel_3");
        let mut t = tore_format(&f,&kernel_3);
        print_matrice(&t, "t");

        convolution_3d(&mut t, &kernel_3);
        print_matrice(&t, "t");
    }



    print_matrice(&f, &"f");

}

pub fn kernel_test(k_type: Kernel, h: usize){
    let kernel = kernel_init(k_type, h);

    print_matrice(&kernel, &"Kernel");
}

pub fn gaussian_test(n : usize, mu: f64, sigma: f64){
    for i in 0..n{
        let k = (i as f64)/(n as f64);
        let x = -1.0 + 2.0*gaussian(mu, sigma, (i as f64)/(n as f64));
        println!("k {}, x {}", k, x);
    }
}

pub fn convolution_correction_test(){
    let mut p1 = vec![1.0,2.0,3.0,4.0,5.0];
    let mut p2 = vec![5.0,6.0,7.0,8.0,9.0];

    let mut p1b = vec![1.0,2.0,3.0,4.0,5.0];
    let mut p2b = vec![5.0,6.0,7.0,8.0,9.0];

    let c1 = convolution_2d(&mut p1,&mut p2);
    let c2 = fast_convolution_2d(&mut p1b, &mut p2b);

    println!("conv_2d : {:?}\nfast_conv_2d : {:?}", c1, c2);
}

pub fn color_test(){
    for i in 0..11{
        println!("{} : {:?}\n", i, found_color(i as f64/10.0, 0));
    }
}

pub fn file_test(){
    let mut f = Field::new_field(10,10,1);
    f.fill(0,0.4645343423765453);
    // write_in_file();
    // read_in_file();

    write_field("storage/test.txt", f.m[0].clone());
    let nf = read_field("storage/test.txt");

    print_matrice(&nf, "nf");
}


fn main() {
    // kernel_test(Kernel::Ring, 13);
    sdl_main();
    // duration_test();
    // convolution_test(5);
    // gaussian_test(1000, 0.15, 0.015)
    // convolution_correction_test() 
    // color_test();
    // println!("{}\n", 9.99 as u8);
    // file_test();
}
