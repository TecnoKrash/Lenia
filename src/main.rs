mod init;
mod convolution;

use crate::convolution::*;


fn main() {
    let p = vec![2.0,1.0];
    let p2 = fft(p, false);
    for i in &p2{
        println!("{}", i);
    }
}
