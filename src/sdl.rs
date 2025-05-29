extern crate sdl2;

// SDL imports
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
// use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::mouse::MouseButton;
// use sdl2::mouse::MouseWheelDirection;

// Other imports
use std::time::SystemTime;

use crate::init::*;
use crate::convolution::*;
use crate::growth::*;
use crate::file::*;
use crate::imgep::*;
use crate::plot::*;


#[derive(Clone)]
#[derive(PartialEq)]
pub enum Mode{
    Learning,
    Lenia,
    Smooth,
    Gol,
    Chan3,
}

#[derive(Clone)]
pub struct Settings {
    pub mode: Mode,
    pub motif: Motif,
}

pub fn diff(a: u8, b: u8) -> u8{
    if a > b{
        return a - b;
    }
    b - a
}

pub fn found_color(val: f64, chan: usize, mode: Mode) -> (u8,u8,u8){
    //let dgd: [(u8,u8,u8); 11] = [(255,255,255), (255,204,204), (255, 204, 153), (255, 255, 102), (153, 255, 51), (0,255, 0), (0, 204, 102), (0, 153, 153), (0, 51, 102), (0, 0, 51), (0, 0, 0)];
    //let dgd: [(u8,u8,u8); 11] = [(255,255,255), (255,255,204), (204, 255, 153), (178, 255, 102), (51, 255, 51), (0,255, 0), (0, 204, 102), (0, 153, 76), (0, 102, 102), (0, 51, 51), (0, 0, 0)];
    let mut res = (0,0,0);
    match mode {
        Mode::Lenia | Mode::Smooth | Mode::Learning => {
            let dgd: [(u8,u8,u8); 11] = [(255,255,255), (229,255,204), (153, 255, 153), (102, 255, 102), (51, 255, 153), (0,255, 128), (0, 204, 204), (0, 153, 153), (0, 51, 102), (0, 25, 51), (0, 0, 0)]; // One Channel
            if val == 1.0 {return (0,0,0);}
            let i = (val*10.0) as usize;
            let a = dgd[i];
            let b = dgd[i+1];
            let v2 = val*10.0 - i as f64;
            if a.0 > b.0{ 
                res.0 = a.0 - ((diff(a.0,b.0) as f64)*v2) as u8;
            }
            else {
                res.0 = a.0 + ((diff(a.0,b.0) as f64)*v2) as u8;
            }
            if a.1 > b.1{ 
                res.1 = a.1 - ((diff(a.1,b.1) as f64)*v2) as u8;
            }
            else {
                res.1 = a.1 + ((diff(a.1,b.1) as f64)*v2) as u8;
            }
            if a.2 > b.2{ 
                res.2 = a.2 - ((diff(a.2,b.2) as f64)*v2) as u8;
            }
            else {
                res.2 = a.2 + ((diff(a.2,b.2) as f64)*v2) as u8;
            }
            res
        },
        Mode::Chan3 => {
            if chan == 0 { res.0 = ((255 as f64)*val) as u8; }
            if chan == 1 { res.1 = ((255 as f64)*val) as u8; }
            if chan == 2 { res.2 = ((255 as f64)*val) as u8; }
            res
        },
        Mode::Gol => {
            if 1.0 - val < val { res = (0,0,0); }
            else { res = (255,255,255); }
            res
        },
    }


}



pub fn display_field(f: &Field, canvas: &mut Canvas<Window>, mode: &Mode, x_start: i32, y_start: i32, pixel_size: i32){
    // println!("f.h : {}, f.m[0].len(): {}", f.h, f.m[0].len());
    // println!("f.l : {}, f.m[0][0].len(): {}", f.l, f.m[0][0].len());
    
    for x in 0..f.h{
        for y in 0..f.l{
            let mut col_t = (0,0,0);
            for i in 0..f.nb_channels{
                let val = &f.get_xy(x, y, i);
                let f = found_color(*val, i, mode.clone());
                // if *val > 0.0 && i == 2 { println!("val: {}", val); }
                col_t.0 += f.0;
                col_t.1 += f.1;
                col_t.2 += f.2;
            }
            canvas.set_draw_color(Color::RGB(col_t.0,col_t.1,col_t.2));
            let r = Rect::new(x_start+(x as i32)*pixel_size, y_start + (y as i32)*pixel_size, pixel_size.try_into().unwrap(), pixel_size.try_into().unwrap());
            let _ = canvas.fill_rect(r);
        }
    }
}


pub fn display_kernel(k: &Vec<Vec<Vec<f64>>>, k_sum: &Vec<f64>, canvas: &mut Canvas<Window>, x_start: i32, y_start: i32, pixel_size: i32){
    let h = k[0].len();
    for x in 0..h{
        for y in 0..h{
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for i in 0..k.len(){
                //println!("k[{}][{}]\n", x, y);
                if i%3 == 0 {red += (k[i][x][y]*255.0*k_sum[i]/*(1.0/k.len() as f64)*/) as u8;}
                if i%3 == 1 {green += (k[i][x][y]*255.0*k_sum[i]/*(1.0/k.len() as f64)*/) as u8;}
                if i%3 == 2 {blue += (k[i][x][y]*255.0*k_sum[i]/*(1.0/k.len() as f64)*/) as u8;}
                //println!("{}\n", red);
            }
            canvas.set_draw_color(Color::RGB(red,green,blue));
            let r = Rect::new(x_start+(x as i32)*pixel_size, y_start + (y as i32)*pixel_size, pixel_size.try_into().unwrap(), pixel_size.try_into().unwrap());
            let _ = canvas.fill_rect(r);
        }
    }
}

pub fn display_tore(f: &Vec<Vec<f64>>, canvas: &mut Canvas<Window>, mode: Mode, x_start: i32, y_start: i32, pixel_size: i32){
    for x in 0..f.len(){
        for y in 0..f[0].len(){
            let mut col_t = (0,0,0);
            let val = &f[x][y];
            let f =  found_color(*val, 0, mode.clone());
            col_t.0 += f.0;
            col_t.1 += f.1;
            col_t.2 += f.2;
            
            canvas.set_draw_color(Color::RGB(col_t.0,col_t.1,col_t.2));
            let r = Rect::new(x_start+(x as i32)*pixel_size, y_start + (y as i32)*pixel_size, pixel_size.try_into().unwrap(), pixel_size.try_into().unwrap());
            let _ = canvas.fill_rect(r);
        }
    }
}


pub fn display_scale(canvas: &mut Canvas<Window>, mode: &Mode, h: usize, l: usize, x: i32, y: i32){
    for i in 0..h{
        let val = (i as f64)/(h as f64);
        let col_t = found_color(val, i, mode.clone());
        canvas.set_draw_color(Color::RGB(col_t.0,col_t.1,col_t.2));
        let r = Rect::new(x, ((y as usize) + h - i).try_into().unwrap(), l.try_into().unwrap(), 0);
        let _ = canvas.fill_rect(r);
    }
}
    


pub fn zoom(normal: bool, x_start: i32, y_start: i32, x_mouse: i32, y_mouse: i32, pixel_size: i32) -> (i32,i32,i32){
    println!("huh");
    let x_decalage = (x_mouse- x_start) / pixel_size;
    let y_decalage = (y_mouse- y_start) / pixel_size;

    let x_mod = (x_mouse- x_start) % pixel_size;
    let y_mod = (y_mouse- y_start) % pixel_size;

    let new_pixel_size;

    if normal {
        new_pixel_size = pixel_size + ((pixel_size as f32)/10.0) as i32;
    }
    else {
        new_pixel_size = pixel_size - ((pixel_size as f32)/10.0) as i32;
    }
    
    let new_x = x_mouse - x_mod - x_decalage*new_pixel_size;
    let new_y = y_mouse - y_mod - y_decalage*new_pixel_size;

    return (new_x,new_y,new_pixel_size)
}

pub fn upscale(f: &mut Field, k: &mut Vec<Vec<Vec<f64>>>, k_sum: &mut Vec<f64>, p: &mut Param, pixel_size: &mut i32){
    assert!(((*pixel_size/2)*2) == *pixel_size);

    *pixel_size = *pixel_size/2;

    for i in 0..f.nb_channels{
        let f_old = f.m[i].clone();

        let h_old = f.h;
        let l_old = f.l;

        *f = Field::new_field(f.m[0].len()*2, f.m[i][0].len()*2, f.nb_channels);

        // println!("f.h : {}, f.m[0].len(): {}", f.h, f.m[0].len());
        // println!("f.l : {}, f.m[0][0].len(): {}", f.l, f.m[0][0].len());

        for x in 0..h_old{
            for y in 0..l_old{
                f.m[i][2*x][2*y] = f_old[x][y];
                f.m[i][2*x+1][2*y] = f_old[x][y];
                f.m[i][2*x][2*y+1] = f_old[x][y];
                f.m[i][2*x+1][2*y+1] = f_old[x][y];
            }
        }
    }
    //    println!("test1");

    p.gr = p.gr*2;

    f.k_size = 2*p.gr +1;

    (*k,*k_sum) = kernel_init(Kernel::Bumpy(&p));
}

pub fn downscale(f: &mut Field, k: &mut Vec<Vec<Vec<f64>>>, k_sum: &mut Vec<f64>, p: &mut Param, pixel_size: &mut i32){
    assert!((f.h/2)*2 == f.h);
    assert!((f.l/2)*2 == f.l);

    *pixel_size = *pixel_size*2;

    for i in 0..f.nb_channels{
        let f_old = f.m[i].clone();

        *f = Field::new_field(f.m[0].len()/2, f.m[i][0].len()/2, f.nb_channels);

        // println!("f.h : {}, f.m[0].len(): {}", f.h, f.m[0].len());
        // println!("f.l : {}, f.m[0][0].len(): {}", f.l, f.m[0][0].len());

        for x in 0..f.h{
            for y in 0..f.l{
                f.m[i][x][y] = (f_old[2*x][2*y] + f_old[2*x+1][2*y] + f_old[2*x][2*y+1] + f_old[2*x+1][2*y+1])/4.0;
            }
        }
    }
    //    println!("test1");

    p.gr = p.gr/2;

    f.k_size = 2*p.gr +1;

    (*k,*k_sum) = kernel_init(Kernel::Bumpy(&p));
}

pub fn evolve_1chan(set: &Settings, f: &mut Field, k: &Vec<Vec<f64>>, dt: f64, neigh_sum: &mut Vec<Vec<f64>>, mu: f64, sigma: f64, bruit: f64){

    let s1 = SystemTime::now();
    let mut tore = tore_format(&(f.m[0]),&k);
    let s2 = SystemTime::now();

    // println!("tore avant : {:?}", tore);

    convolution_2d(&mut tore, k);
    let s3 = SystemTime::now();

    // println!("tore après : {:?}", tore);

    *neigh_sum = tore.clone();

    if set.mode == Mode::Gol { growth_gol(f, tore, dt, mu, sigma);}
    // else { growth_lenia(f, tore, dt, mu, sigma);}
    else { growth_lenia_old(f, tore, dt, mu, sigma);}
    let s4 = SystemTime::now();

    let d1 = s2.duration_since(s1).unwrap();
    let d2 = s3.duration_since(s2).unwrap();
    let d3 = s4.duration_since(s3).unwrap();

    // println!("Duration : tore {:?}, conv3D {:?}, growth {:?}\n", d1, d2, d3);


}

pub fn evolve(set: &Settings, f: &mut Field, k: &Vec<Vec<Vec<f64>>>, dt: f64, neigh_sum: &mut Vec<Vec<f64>>, p: &Param, chan_ratios: &Vec<usize>, seed: &Seed, bruit: f64){

    for i in 0..p.nb_kernels{

        let s1 = SystemTime::now();
        let mut tore = tore_format(&(f.m[p.c[i].0]),&k[i]);
        let s2 = SystemTime::now();

        // println!("tore avant : {:?}", tore);

        convolution_2d(&mut tore, &k[i]);
        let s3 = SystemTime::now();

        // println!("tore après : {:?}", tore);

        *neigh_sum = tore.clone();

        if set.mode == Mode::Gol { growth_gol(f, tore, dt, p.mu[i], p.sigma[i]);}
        // else { growth_lenia(f, tore, dt, mu, sigma);}
        else { growth_lenia(f, tore, dt, p.mu[i], p.sigma[i], p.h[i], p.c[i].1, 1.0/(chan_ratios[p.c[i].1] as f64), &seed, bruit);}
        let s4 = SystemTime::now();

        let d1 = s2.duration_since(s1).unwrap();
        let d2 = s3.duration_since(s2).unwrap();
        let d3 = s4.duration_since(s3).unwrap();

        // println!("Duration : tore {:?}, conv3D {:?}, growth {:?}\n", d1, d2, d3);
    }


}

pub fn sdl_main(set: Settings) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .fullscreen_desktop()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let _i = 0;
    //let mut monte = true;
    
    let l = 128;
    let h = 128;

    let nb_chan;
    if set.mode == Mode::Chan3 { nb_chan = 3; }
    else { nb_chan = 1; }
    
    let mut f = Field::new_field(l,h,nb_chan);
    // f.fill_deg(0,0.0,1.0); 
    // f.fill(0,0.15);
    // f.fill_rng(0);
    // f.add(Motif::Rand, 35, 35);

    let mut k;
    let mut k_sum;
    let mut p = Param::param_init(&set);
    let mut chan_ratios = vec![0; f.nb_channels];

    for i in 0..p.nb_kernels{
        chan_ratios[p.c[i].1] += 1;
    }

    f.k_size = 2*p.gr +1;
        
    /*
    match set.mode {
        Mode::Lenia => {
            /*
            let k_h = 25;
            k= kernel_init(Kernel::Ring1(k_h));
            f.k_size = k_h;
            (p.mu, p.sigma) = (0.15, 0.017);
            */
            f.k_size = 2*p.gr+1;

            // triple_kernel(& mut p);
            // println!("k1 == k : {}, f.k_size : {}", (k1 == k), f.k_size);
        },
        Mode::Learning => {
            random_param(& mut p);
            // k = kernel_init(Kernel::Bumpy(&p));
        }
        Mode::Chan3 => {
            /*
            let k_h1 = 25;
            let k_h2 = 25;
            k = kernel_init(Kernel::Ring2(k_h1, k_h2));
            f.k_size = k_h2;
            (p.mu, p.sigma) = (0.15, 0.017);
            */

            triple_ring(& mut p);
            // k = kernel_init(Kernel::Bumpy(&p));
            f.k_size = 2*p.gr +1;
        }
    }
    */

    match set.mode {
        Mode::Lenia | Mode::Learning => {
            (k,k_sum) = kernel_init(Kernel::Bumpy(&p));
            println!("\n{:?}", k);
            println!("k.len() : {}", k.len());
        },
        Mode::Chan3 => {
            // (k,k_sum) = claude_kernel(&p);
            (k,k_sum) = kernel_init(Kernel::Bumpy(&p));
        }
        Mode::Smooth => {
            (k,k_sum) = kernel_init(Kernel::Radical(&p));
        },
        Mode::Gol => {
            k = vec![vec![vec![0.0, 0.0, 0.0, 0.0, 0.0],
                          vec![0.0, 1.0/8.5, 1.0/8.5, 1.0/8.5, 0.0],
                          vec![0.0, 1.0/8.5, 0.5/8.5, 1.0/8.5, 0.0],
                          vec![0.0, 1.0/8.5, 1.0/8.5, 1.0/8.5, 0.0],
                          vec![0.0, 0.0, 0.0, 0.0, 0.0],]];
            k_sum = vec![8.5];
        }
    }




    let mut drag = false;
    let mut add = false;
    let mut add_wait = false;

    let mut zoom_in = false;
    let mut zoom_out = false;


    let mut x_curent = 100;
    let mut y_curent = 20;

    let mut x_mouse = 0;
    let mut y_mouse = 0;

    let mut pixel_size = 8;

    let mut frames = 15;
    if set.mode == Mode::Gol { frames = 10 };

    
    let mut save_compt = 1;

    let mut ev = false;

    let f_max = (1.0/2.0, 1.0/2.0, 1.0/(frames as f64)*2.0);

    let mut bruit = 0.3;
    let seed = generate_seed(f_max, 10);

    let mut show_kernel = false;
    let mut show_tore = false;
    let mut show_scale = false;

    let mut one_frame = false;
    let mut neigh_sum = vec![]; 

    let mut bary = false;
    let mut mc = mass_center(&f);
    println!("mc: {:?}\n", mc);

    // println!("mu: {}, sigma: {}\n", p.mu, p.sigma);

    let _start = SystemTime::now();

    display_field(&f,&mut canvas, &set.mode, x_curent,y_curent,pixel_size);
    // display_scale(&mut canvas, &mode,(pixel_size as usize)*h, 50, x_curent + pixel_size*(l as i32) + 20,y_curent + 100);

    write_field("storage/save/init.txt", f.m[0].clone());

    // Event l
    'running: loop {
        let start = SystemTime::now();

        let background_color;
        if set.mode == Mode::Chan3 { background_color = Color::RGB(255, 255, 255); }
        else {background_color = Color::RGB(0, 0, 0); }

        canvas.set_draw_color(background_color);
        canvas.clear();
        /*
        if i == 254 || (i == 0)&&(!monte){
            monte = !monte; 
        }
        if monte {
            i = (i + 1) % 255;
        }
        else {
            i = (i - 1) % 255;
        }
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.set_draw_color(Color::RGB(255,0,0));
        let r = Rect::new(400,300, 100, 
        100);
        let _ = canvas.fill_rect(r);
        canvas.set_draw_color(Color::RGB(0,0,255));
        let _ = canvas.draw_rect(r);
        // canvas.clear();
        */
        // println!("frame n°{}\n", compt);

        if ev {
            evolve(&set, &mut f, &k, 1.0/frames as f64, &mut neigh_sum, &p, &chan_ratios, &seed, bruit);
        }

        if one_frame {
            ev = false;
            one_frame = false;
        }
        
        display_field(&f, &mut canvas, &set.mode, x_curent, y_curent, pixel_size);

        if show_tore {
            display_tore(&neigh_sum, &mut canvas, set.mode.clone(), x_curent + (l as i32)*pixel_size + 20, y_curent ,pixel_size);
        }

        if show_scale {
            display_scale(&mut canvas, &set.mode,(pixel_size as usize)*h, 50, x_curent + pixel_size*(l as i32) + 20,y_curent + 100);
        }
        

        let pos = position(&f, mc);
        let ((xc,yc),(hc,lc)) = pos;
        
        let mut  c = Field::new_field(hc, lc, 1);
        c.m[0] = better_reduction(&f, pos);

        mc = mass_center(&c);
        mc.0 += xc;
        mc.1 += yc;

    

        if bary {
            canvas.set_draw_color(Color::RGB(218,63,2));
            let r1 = Rect::new(x_curent+ ((mc.0%f.h) as i32)*pixel_size, y_curent + ((mc.1%f.l) as i32)*pixel_size, pixel_size.try_into().unwrap(), pixel_size.try_into().unwrap());
            let _ = canvas.fill_rect(r1);

            canvas.set_draw_color(Color::RGB(218,63,2));
            let r2 = Rect::new(x_curent+ ((xc%f.h) as i32)*pixel_size, y_curent + ((yc%f.l) as i32)*pixel_size, pixel_size.try_into().unwrap(), pixel_size.try_into().unwrap());
            let _ = canvas.fill_rect(r2);

            canvas.set_draw_color(Color::RGB(218,63,2));
            let r3 = Rect::new(x_curent + (((xc + lc)%f.h) as i32)*pixel_size, y_curent + (((yc + hc)%f.l) as i32)*pixel_size, pixel_size.try_into().unwrap(), pixel_size.try_into().unwrap());
            let _ = canvas.fill_rect(r3);

        }
        

        //println!("the display took {:?}\n", duration);
        
        if show_kernel {
            display_kernel(&k, &k_sum, &mut canvas,x_curent, y_curent, pixel_size);
        }
        
        if drag {
            let x_new = event_pump.mouse_state().x();
            let y_new = event_pump.mouse_state().y();
            // println!("x : {}, y : {}\n", x_mouse, y_mouse);
            if x_new != x_mouse {
                x_curent += x_new - x_mouse;
                x_mouse = x_new
            }
            if y_new != y_mouse {
                y_curent += y_new - y_mouse;
                y_mouse = y_new
            }
             

        }

        if add {
            // println!("current : ({},{})", x_curent, y_curent);
            // println!("with pixel size : ({},{})", (x_mouse - x_curent)/pixel_size, (y_mouse - y_curent)/pixel_size);
            let x = ((x_mouse - x_curent)/pixel_size).try_into().unwrap();
            let y = ((y_mouse - y_curent)/pixel_size).try_into().unwrap();

            // println!("xy : ({},{})", x, y);

            f.add(&set, x, y);
            // println!("{:?}", f.m[2]);
            // f.add(&set, 7, 1);
            add = false;
        }

        if zoom_in {
            match zoom(true, x_curent, y_curent, x_mouse,y_mouse, pixel_size) {
                (x,y,p) => {
                    x_curent = x;
                    y_curent = y;
                    pixel_size = p
                }
            }
            zoom_in = false;
        }

        if zoom_out {
            match zoom(false, x_curent, y_curent, x_mouse,y_mouse, pixel_size) {
                (x,y,p) => {
                    x_curent = x;
                    y_curent = y;
                    pixel_size = p
                }
            }
            zoom_out = false;
        }

        

        let mut update_x = false;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    //println!("Field : {:?}\n", f.m);
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    let name = format!("storage/save/s_{}.txt",save_compt);
                    write_field(&name, f.m[0].clone());
                    save_compt += 1;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    let pos = ((0,0),(0,0));
                    let red = better_reduction(&f, pos);
                    let name = format!("storage/save/r_{}.txt",save_compt);
                    write_field(&name, red);
                    save_compt += 1;
                },
                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    show_kernel = !show_kernel;
                },
                Event::KeyDown { keycode: Some(Keycode::T), .. } => {
                    show_tore = !show_tore;
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    show_scale = !show_scale;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    ev = !ev;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if ev == false {
                        ev = true;
                        one_frame = true;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::M), .. } => {
                    bary = !bary;
                },
                Event::KeyDown { keycode: Some(Keycode::G), .. } => {
                    bruit += 0.1;
                    println!("bruit : {}", bruit);
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    bruit -= 0.1;
                    println!("bruit : {}", bruit);
                },
                Event::KeyDown { keycode: Some(Keycode::U), .. } => {
                    if set.mode != Mode::Gol { upscale(&mut f, &mut k, &mut k_sum, &mut p, &mut pixel_size);}
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    if set.mode != Mode::Gol { downscale(&mut f, &mut k, &mut k_sum, &mut p, &mut pixel_size);}
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    plot_kernels(&p, &k_sum, "hydro_kernel", "tadam !", &vec![(255,0,0); 15]).unwrap(); 
                },
                Event::KeyDown { keycode: Some(Keycode::H), .. } => {
                    plot_growth(&p, "3chan_growth", "tadam !", &vec![(255,0,0); 15]).unwrap(); 
                },
                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                    f.fill(0, 0.0);
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left,.. } => {
                    if !drag {
                        update_x = true;
                        drag = true
                    }
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Right,.. } => {
                    if !add_wait {
                        add = true;
                        add_wait = true;
                        update_x = true;
                    }
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => { 
                    drag = false;
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, .. } => { 
                    add_wait = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Comma), ..  } => {
                    println!("{}",true);
                    zoom_out = true;
                    update_x = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Semicolon), ..  } => {
                    zoom_in = true;
                    update_x = true;
                }  
                Event::KeyDown { keycode: Some(Keycode::Colon), ..  } => {
                    frames += 1;
                    println!("frames : {}", frames);
                }  
                Event::KeyDown { keycode: Some(Keycode::Exclaim), ..  } => {
                    frames -= 1;
                    println!("frames : {}", frames);
                }  
                _ => {}
            }
        }
        
        if update_x {
            x_mouse = event_pump.mouse_state().x();
            y_mouse = event_pump.mouse_state().y();
            // println!("mouse : ({},{})", x_mouse, y_mouse);
        }
        

        // The rest of the game loop goes here...
        //let _res = window.set_fullscreen(Desktop);

        canvas.present();

        let end = SystemTime::now();

        let duration = end.duration_since(start).unwrap();

        let f_time = Duration::new(0, 1_000_000_000u32 / frames);
        let fps = 1_000_000_000f64 /(Duration::as_nanos(&duration) as f64);

        if duration < f_time{
            // println!("fps : {}", fps);
            ::std::thread::sleep(f_time- duration);
        }
    }
}
