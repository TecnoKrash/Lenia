use std::io::Write;
// use std::io::Read;
use std::fs;
// use bytes::{BytesMut, BufMut};
use std::convert::TryInto;

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

pub fn reduction(f: Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    let mut xd = f.len();
    let mut xf = 0;
    let mut yd = f[0].len();
    let mut yf = 0;

    for i in 0..f.len(){
        for j in 0..f[0].len(){
            if f[i][j] != 0.0{
                if yd > j{
                    yd = j;
                }

                if yf < j{
                    yf = j+1;
                }

                if xd == f.len() {
                    xd = i
                }

                xf = i+1;
            }
        }
    }

    let mut result = Vec::with_capacity(xf-xd);

    for i in xd..xf{
        let mut line = Vec::with_capacity(yf-yd);
        for j in yd..yf{
            line.push(f[i][j]);
        }
        result.push(line);
    }

    return result;
}


