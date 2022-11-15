extern crate sdl2;
extern crate rand;

use std::f32::consts::PI;
use std::{time::*, thread};
use rand::Rng;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use structs::*;
pub mod structs;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn put_pixel(x: u32, y: u32, color: Color, framedata: &mut Vec<u8>) {
    framedata[((x + y * WIDTH)*4 + 0) as usize] = color.b;
    framedata[((x + y * WIDTH)*4 + 1) as usize] = color.g;
    framedata[((x + y * WIDTH)*4 + 2) as usize] = color.r;
    framedata[((x + y * WIDTH)*4 + 3) as usize] = color.a;
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Lorenz Attractor", WIDTH, HEIGHT).resizable().position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let mut framebuffer = texture_creator.create_texture_streaming(Some(PixelFormatEnum::ARGB8888), WIDTH, HEIGHT).unwrap();
    let mut framedata: Vec<u8> = vec![0; ((WIDTH*HEIGHT)*4) as usize];

    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;


    let rho = 28.0;
    let sigma = 10.0;
    let beta = 8.0 / 3.0;
    let dt = 0.001;

    let scale: f32 = 10.0;

    let particle_count = 10000;

    let mut angle: f32 = 0.0;

    let mut particles: Vec<particle> = vec![];

    for i in 0..particle_count {
        let mut rng = rand::thread_rng();
        particles.push(particle::new(i as f32, Color::RGB(rng.gen(), rng.gen(), rng.gen()) ));
    }

    let mut last_time = Instant::now();

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                    break;
                },
                _ => {}
            }
        }

        framedata = vec![0; ((WIDTH*HEIGHT)*4) as usize];
        
        // edit physics
        for mut i in &mut particles {
            let dx = (sigma * (i.y - i.x)) * dt;
            let dy = (i.x * (rho - i.z) - i.y) * dt;
            let dz = (i.x * i.y - beta * i.z) * dt;

            i.x += dx;
            i.y += dy;
            i.z += dz;

        }


        for i in &mut particles {
            /*
                x axis rotation matrix
                [ 1       0           0     ]
                [ 0 angle.cos() -angle.sin()] 
                [ 0 angle.sin()  angle.cos()]
            */
            let matx = i.x * 1.0 + i.y * 0.0 + i.z * 0.0;
            let maty = i.x * 0.0 + i.y * angle.cos() + i.z * -angle.sin();

            let x = (matx * scale) + WIDTH as f32/2.0;
            let y = (maty * scale) + HEIGHT as f32/2.0;

            if x >= WIDTH as f32 || y >= HEIGHT as f32 || x <= 0.0 || y <= 0.0 { continue; }
            put_pixel(x as u32, y as u32, i.color, &mut framedata);
        }


        if Instant::now() - last_time < Duration::from_secs_f32(1.0 * dt) {
            thread::sleep(Duration::from_secs_f32(1.0 * dt) - (Instant::now() - last_time));
            last_time = Instant::now();
        }
        angle += 0.001;

        canvas.clear();
        framebuffer.update(None, &framedata, (WIDTH*4) as usize).expect("Texture update");
        canvas.copy(&framebuffer, None, None).expect("oops");
        canvas.present();
    }


}
