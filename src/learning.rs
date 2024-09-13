// use crate::init::*;

pub fn mass_center(tore: Vec<Vec<f64>>, k_size: usize) -> (usize,usize){
    let mut x = 0;
    let mut y = 0;

    let h = tore.len() - k_size;
    let l = tore[0].len() - k_size;

    for i in k_size..h{
        for j in k_size..l{
            if tore[i][j] > tore[x][y]{
                x = i;
                y = j;
            }
        }
    }

    (x,y)
}

