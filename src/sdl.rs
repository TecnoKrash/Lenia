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
use sdl2::mouse::MouseWheelDirection;


// Other imports
use std::time::SystemTime;

use crate::init::*;
use crate::convolution::*;
use crate::growth::*;
use crate::file::*;
use crate::imgep::*;

pub enum Mode{
    Learning,
    Classic,
}

pub fn diff(a: u8, b: u8) -> u8{
    if a > b{
        return a - b;
    }
    b - a
}

pub fn found_color(val: f64, _chan: usize) -> (u8,u8,u8){
    //let dgd: [(u8,u8,u8); 11] = [(255,255,255), (255,204,204), (255, 204, 153), (255, 255, 102), (153, 255, 51), (0,255, 0), (0, 204, 102), (0, 153, 153), (0, 51, 102), (0, 0, 51), (0, 0, 0)];
    //let dgd: [(u8,u8,u8); 11] = [(255,255,255), (255,255,204), (204, 255, 153), (178, 255, 102), (51, 255, 51), (0,255, 0), (0, 204, 102), (0, 153, 76), (0, 102, 102), (0, 51, 51), (0, 0, 0)];
    let dgd: [(u8,u8,u8); 11] = [(255,255,255), (229,255,204), (153, 255, 153), (102, 255, 102), (51, 255, 153), (0,255, 128), (0, 204, 204), (0, 153, 153), (0, 51, 102), (0, 25, 51), (0, 0, 0)];
    let mut res = (0,0,0);
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
}



pub fn display_field(f: &Field, canvas: &mut Canvas<Window>, x_start: i32, y_start: i32, pixel_size: i32){
    for x in 0..f.h{
        for y in 0..f.l{
            let mut col_t = (0,0,0);
            for i in 0..f.nb_channels{
                let val = &f.get_xy(x, y, i);
                let f =  found_color(*val, i);
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


pub fn display_kernel(k: &Vec<Vec<f64>>, canvas: &mut Canvas<Window>, x_start: i32, y_start: i32, pixel_size: i32){
    let h = k.len();
    for x in 0..h{
        for y in 0..h{
            //println!("k[{}][{}]\n", x, y);
            let red = (k[x][y]*255.0) as u8;
            //println!("{}\n", red);
            canvas.set_draw_color(Color::RGB(red,0,0));
            let r = Rect::new(x_start+(x as i32)*pixel_size, y_start + (y as i32)*pixel_size, pixel_size.try_into().unwrap(), pixel_size.try_into().unwrap());
            let _ = canvas.fill_rect(r);
        }
    }
}

pub fn display_tore(f: &Vec<Vec<f64>>, canvas: &mut Canvas<Window>, x_start: i32, y_start: i32, pixel_size: i32){
    for x in 0..f.len(){
        for y in 0..f[0].len(){
            let mut col_t = (0,0,0);
            let val = &f[x][y];
            let f =  found_color(*val, 0);
            col_t.0 += f.0;
            col_t.1 += f.1;
            col_t.2 += f.2;
            
            canvas.set_draw_color(Color::RGB(col_t.0,col_t.1,col_t.2));
            let r = Rect::new(x_start+(x as i32)*pixel_size, y_start + (y as i32)*pixel_size, pixel_size.try_into().unwrap(), pixel_size.try_into().unwrap());
            let _ = canvas.fill_rect(r);
        }
    }
}


pub fn display_scale(canvas: &mut Canvas<Window>, h: usize, l: usize, x: i32, y: i32){
    for i in 0..h{
        let val = (i as f64)/(h as f64);
        let col_t = found_color(val, i);
        canvas.set_draw_color(Color::RGB(col_t.0,col_t.1,col_t.2));
        let r = Rect::new(x, ((y as usize) + h - i).try_into().unwrap(), l.try_into().unwrap(), 0);
        let _ = canvas.fill_rect(r);
    }
}
    


pub fn zoom(normal: bool, x_start: i32, y_start: i32, x_mouse: i32, y_mouse: i32, pixel_size: i32) -> (i32,i32,i32){
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

pub fn evolve_1chan(f: &mut Field, k: &Vec<Vec<f64>>, dt: f64, neigh_sum: &mut Vec<Vec<f64>>, mu: f64, sigma: f64){

    let s1 = SystemTime::now();
    let mut tore = tore_format(&(f.m[0]),&k);
    let s2 = SystemTime::now();

    // println!("tore avant : {:?}", tore);

    convolution_3d(&mut tore, k);
    let s3 = SystemTime::now();

    // println!("tore après : {:?}", tore);

    *neigh_sum = tore.clone();

    growth(f, tore, dt, mu, sigma);
    let s4 = SystemTime::now();

    let d1 = s2.duration_since(s1).unwrap();
    let d2 = s3.duration_since(s2).unwrap();
    let d3 = s4.duration_since(s3).unwrap();

    // println!("Duration : tore {:?}, conv3D {:?}, growth {:?}\n", d1, d2, d3);


}


pub fn sdl_main(mode: Mode) {
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
    
    let l = 100;
    let h = 100;
    
    let mut f = Field::new_field(l,h,1);
    // f.fill_deg(0,0.0,1.0); 
    // f.fill(0,0.15);
    // f.fill_rng(0);
    // f.add(Motif::Rand, 35, 35);
    f.add(Motif::Orbium, 10, 10);

    let k;
    let mut p = Param {
        mu: 0.15,
        sigma: 0.017,
        nb_bump: 3,
        gr: 25,
        r: 0.,
        a: vec![],
        w: vec![],
        b: vec![],
    };
        

    match mode {
        Mode::Classic => {
            let k_h = 25;
            k = kernel_init(Kernel::Ring(k_h));
            f.k_size = k_h;
            (p.mu, p.sigma) = (0.15, 0.017);
        },
        Mode::Learning => {
            random_param(& mut p);
            k = kernel_init(Kernel::Bumpy(&p));
            f.k_size = 2*p.gr;
        }
    }


    let mut drag = false;

    let mut zoom_in = false;
    let mut zoom_out = false;


    let mut x_curent = 100;
    let mut y_curent = 20;

    let mut x_mouse = 0;
    let mut y_mouse = 0;

    let mut pixel_size = 10;

    let frames = 12;
    
    let mut save_compt = 1;

    let mut ev = true;
    let mut neigh_sum = vec![]; 

    let mut bary = false;

    // println!("mu: {}, sigma: {}\n", p.mu, p.sigma);

    let start = SystemTime::now();

    display_field(&f,&mut canvas,x_curent,y_curent,pixel_size);
    // display_scale(&mut canvas,(pixel_size as usize)*h, 50, x_curent + pixel_size*(l as i32) + 20,y_curent + 100);

    write_field("storage/save/init.txt", f.m[0].clone());

    // Event l
    'running: loop {
        let start = SystemTime::now();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
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
        let r = Rect::new(400,300, 100, 100);
        let _ = canvas.fill_rect(r);
        canvas.set_draw_color(Color::RGB(0,0,255));
        let _ = canvas.draw_rect(r);
        // canvas.clear();
        */
        // println!("frame n°{}\n", compt);

        if ev {
            evolve_1chan(&mut f, &k, 1.0/frames as f64, &mut neigh_sum, p.mu, p.sigma);
        }
        
        display_field(&f, &mut canvas, x_curent, y_curent, pixel_size);
        // display_tore(&neigh_sum, &mut canvas, x_curent + (l as i32)*pixel_size + 20, y_curent ,pixel_size);
        // display_scale(&mut canvas,(pixel_size as usize)*h, 50, x_curent + pixel_size*(l as i32) + 20,y_curent + 100);
        
        if bary {
            let mc = mass_center(&f);

            canvas.set_draw_color(Color::RGB(218,63,2));
            let r = Rect::new(x_curent+ (mc.0 as i32)*pixel_size, y_curent + (mc.1 as i32)*pixel_size, pixel_size.try_into().unwrap(), pixel_size.try_into().unwrap());
            let _ = canvas.fill_rect(r);
        }

        //println!("the display took {:?}\n", duration);
        
        // display_kernel(&k, &mut canvas,x_curent, y_curent, pixel_size);
        
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
                    let red = reduction(f.m[0].clone());
                    let name = format!("storage/save/r_{}.txt",save_compt);
                    write_field(&name, red);
                    save_compt += 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    ev = !ev;
                },
                Event::KeyDown { keycode: Some(Keycode::M), .. } => {
                    bary = !bary;
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left,.. } => {
                    if !drag {
                        update_x = true;
                        drag = true
                    }
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => { 
                    drag = false
                },
                Event::MouseWheel { direction: MouseWheelDirection::Flipped, ..  } => {
                    zoom_out = true;
                    update_x = true;
                } 
                Event::MouseWheel { direction: MouseWheelDirection::Normal, ..  } => {
                    
                    zoom_in = true;
                    update_x = true;
                }  
                _ => {}
            }
        }
        
        if update_x {
            x_mouse = event_pump.mouse_state().x();
            y_mouse = event_pump.mouse_state().y();
        }
        

        // The rest of the game loop goes here...
        //let _res = window.set_fullscreen(Desktop);

        canvas.present();

        let end = SystemTime::now();

        let duration = end.duration_since(start).unwrap();

        let f_time = Duration::new(0, 1_000_000_000u32 / frames);

        if duration < f_time{
            ::std::thread::sleep(f_time- duration);
        }
    }
}
