mod init;
mod convolution;

use crate::convolution::*;


fn main() {

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

    /*let ip2 = fft(p2, false);
    for i in &ip2{
        println!("{}", i);
    }

    let ip3: Vec<f64> = vec![ip1[0]*ip2[0], ip1[1]*ip2[1], ip1[2]*ip2[2], ip1[3]*ip2[3], ip1[4]*ip2[4]];

    let p3 = fft(ip3, true);

    for i in &p3{
        println!("{:?}", i);
    }
    */

}
