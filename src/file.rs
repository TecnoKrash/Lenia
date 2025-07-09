use std::io::Write;
use std::fs;

use crate::init::*;

// Save f by writing its values in the file found at `path`
pub fn write_field(path: &str, f: Vec<Vec<f64>>) {
    let mut file = std::fs::File::create(path).expect("create failed");

    let h = f.len();
    let l = f[0].len();
    
    file.write_all(h.to_string().as_bytes()).expect("write failed");
    file.write_all(" ".as_bytes()).expect("write failed");
    file.write_all(l.to_string().as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("write failed");

    for i in 0..h {
        for j in 0..l {
            file.write_all(f[i][j].to_string().as_bytes()).expect("write failed");
            file.write_all(" ".as_bytes()).expect("write failed");
        }
        file.write_all("\n".as_bytes()).expect("write failed");
    }
}

// Read the field stored in the file found at `path`
pub fn read_field(path: &str) -> Vec<Vec<f64>> {
    let content = fs::read_to_string(path).expect("read failed");

    let mut res:Vec<Vec<f64>> = vec![];
    let mut ligne:Vec<f64> = vec![];

    let mut h = 0;
    let mut l = 0;

    let mut buff:String = "".to_string();

    for char in content.chars(){
        if h == 0 {
            if char == ' '{
                h = buff.parse().unwrap();
                buff = "".to_string();
            }
            else{
                buff = format!("{}{}", buff, char);
            }
        }

        else{
            if l == 0 {
                if char == '\n'{
                    l = buff.parse().unwrap();
                    buff = "".to_string();
                    res = Vec::with_capacity(h);
                    ligne = Vec::with_capacity(l);
                    
                }
                else{
                    buff = format!("{}{}", buff, char);
                }
            }

            else{
                if char == ' '{
                    ligne.push(buff.parse().unwrap());
                    buff = "".to_string();
                }


                else{
                    if char == '\n'{
                        res.push(ligne);
                        ligne = Vec::with_capacity(l);
                    }

                    else {
                        buff = format!("{}{}", buff, char);
                    }
                }
            }
        }
    }

    return res;
}

// Attempt to localize an agent in the field (not working)
pub fn reduction(f: &Field, pos: ((usize, usize), (usize, usize))) -> Vec<Vec<f64>>{

    let ((x,y),(h,l)) = pos;

    let mut res = Vec::with_capacity(h);
    for i in x..x+h{
        let mut ligne = Vec::with_capacity(l);
        for j in y..y+l{
            ligne.push(f.m[0][i%f.h][j%f.l]);
        }
        res.push(ligne);
    }

    res
}
