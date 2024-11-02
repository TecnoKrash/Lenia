use std::io::Write;
// use std::io::Read;
use std::fs;
// use bytes::{BytesMut, BufMut};
use std::convert::TryInto;

use crate::init::*;
use crate::imgep::*;

fn demo<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn write_in_file(name: &str, texte: &str) {
   let mut file = std::fs::File::create(name).expect("create failed");
   file.write_all(texte.as_bytes()).expect("write failed");
   // println!("data written to file" );
}

pub fn read_in_file(name: &str) -> String {
    let texte = fs::read_to_string(name).expect("read failed");
    return texte.to_string();
}

pub fn write_field(name: &str, f: Vec<Vec<f64>>) {
    let mut file = std::fs::File::create(name).expect("create failed");

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

pub fn read_field(name: &str) -> Vec<Vec<f64>> {
    let content = fs::read_to_string(name).expect("read failed");

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

pub fn co_calculation(b: (bool,bool), f: Vec<Vec<f64>>) -> (usize,usize,usize,usize){

    let mut co = (f.len()*(1 - b.0 as usize), f.len()*(b.1 as usize), f[0].len(), 0);

    let mut max_white_y = 0;

    let mut co_white_y = (0,0,0,0);

    for i in 0..f.len(){
        for j in 0..f[0].len(){
            if f[i][j] != 0.0{

                if (co.2 > j)&&!b.1{
                    co.2 = j;
                }

                if (co.3 <= j)&&!b.1{
                    co.3  = j+1;
                }

                if (co.0 == f.len())&&!b.0 {
                    co.0 = i
                }

                if !b.0 {co.1 = i+1;}
            }

            // if f[i][j] == 0.0{

        }
    }
    co
}

pub fn reduction(f: Vec<Vec<f64>>) -> Vec<Vec<f64>>{

    let mut b = (false, false);

    let mut redo = false;
    
    let mut co = co_calculation(b,f.clone());

    if (co.0 == 0)&&(co.1 == f.len()){
        b.0 = true;
        redo = true;
    }
    if (co.2 == 0)&&(co.3 == f[0].len()){
        b.1 = true;
        redo = true;
    }

    println!("co0 : {}, co1 : {}", co.0, co.1);
    println!("co2 : {}, co3 : {}", co.2, co.3);
    println!("b0 : {}, b2 : {}", b.0, b.1);

    if redo {co = co_calculation(b,f.clone());}

    println!("co0 : {}, co1 : {}", co.0, co.1);
    println!("co2 : {}, co3 : {}", co.2, co.3);

    let mut result = Vec::with_capacity(co.1-co.0);
    
    for i in co.0..co.1{
        let mut line = Vec::with_capacity(co.3-co.2);
        for j in co.2..co.3{
            line.push(f[i%f.len()][j%f[0].len()]);
        }
        result.push(line);
    }


    return result;
}

pub fn better_reduction(f: &Field) -> Vec<Vec<f64>>{

    let ((x,y),(h,l)) = position(&f);
    // println!("x: {}, y: {}, h: {}, l: {}", x, y, h, l);

    let mut res = Vec::with_capacity(h);
    for i in x..x+h{
        let mut ligne = Vec::with_capacity(l);
        for j in y..y+l{
            ligne.push(f.m[0][i][j]);
        }
        res.push(ligne);
    }

    // println!("res: {:?}", res);

    res
}

pub fn save_data(){
    todo!();
}
