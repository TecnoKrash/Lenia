extern crate sdl2;

// SDL imports
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

// Other imports
use crate::init::*;

pub fn found_color(val: f64, chan: usize) -> Color{
    let chan =  1;
    let dgd: [((u8,u8,u8),(u8,u8,u8)); chan] = [((0 as u8,0 as u8,0 as u8), (255 as u8,0 as u8,0 as u8))];
    for i in 0..chan{
        Color::RGB(0,0,0)
    }

}

pub fn display_field(f: Field, canvas: &mut Canvas<Window>, x_start: i32, y_start: i32, pixel_size: u32){

    for x in 0..f.h{
        for y in 0..f.l{
            for i in 0..f.nb_channels{
                let val = &f.get_xy(x, y, i);
                canvas.set_draw_color(found_color(*val, f.nb_channels));

                let r = Rect::new(x_start+(x as i32)*(pixel_size as i32), y_start + (y as i32)*(pixel_size as i32), pixel_size, pixel_size);
                let _ = canvas.fill_rect(r);

            }
        }
    }
}

pub fn sdl_main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        // .fullscreen_desktop()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut monte = true;

    // Event loop
    'running: loop {
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
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::F5), .. } => {
                    i = 255-i;
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        //let _res = window.set_fullscreen(Desktop);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}