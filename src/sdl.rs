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
use crate::init::*;

pub fn found_color(val: f64, chan: usize) -> (u8,u8,u8){
    let dgd: [((u8,u8,u8),(u8,u8,u8)); 1] = [((0 as u8,0 as u8,0 as u8), (255 as u8,0 as u8,0 as u8))];
    let mut res = (0,0,0);
    res.0 = dgd[chan].0.0 + (val*f64::from(dgd[chan].1.0 + dgd[chan].0.0)) as u8;
    res.1 = dgd[chan].0.1 + (val*f64::from(dgd[chan].1.1 + dgd[chan].0.1)) as u8;
    res.2 = dgd[chan].0.2 + (val*f64::from(dgd[chan].1.2 + dgd[chan].0.2)) as u8;
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

pub fn sdl_main() {
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
    let mut i = 0;
    //let mut monte = true;
    
    let mut f = Field::new_field(100,50,1);
    f.fill_deg(0,0.0,1.0);

    let mut drag = false;

    let mut zoom_in = false;
    let mut zoom_out = false;


    let mut x_curent = 20;
    let mut y_curent = 20;

    let mut x_mouse = 0;
    let mut y_mouse = 0;

    let mut pixel_size = 20;

    // Event l
    'running: loop {
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
        display_field(&f,&mut canvas,x_curent,y_curent,pixel_size);
        
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
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::F5), .. } => {
                    i = 255-i
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
                Event::MouseWheel { direction: MouseWheelDirection::Normal, ..  } => {
                    zoom_in = true;
                    update_x = true;
                }  
                Event::MouseWheel { direction: MouseWheelDirection::Flipped, ..  } => {
                    zoom_out = true;
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
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
