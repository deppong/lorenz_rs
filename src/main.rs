extern crate sdl2;
extern crate rand;

use std::{time::*, thread};
use rand::Rng;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use structs::*;
pub mod structs;

use rot_matrix::*;
pub mod rot_matrix;

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
    let mut framedata: Vec<u8>;

    canvas.clear();


    let mut event_pump = sdl_context.event_pump().unwrap();

    // Lorenz Constants
    const rho: f32 = 28.0;
    const sigma: f32 = 10.0;
    const beta: f32 = 8.0 / 3.0;
    const particle_count: usize = 10000;

    let dt = 0.001;
    let scale: f32 = 10.0;

    let mut x_angle: f32 = 0.0;
    let mut y_angle: f32 = 0.0;
    let mut z_angle: f32 = 0.0;

    let mut particles = [Particle {x: 0.0, y: 0.0, z: 0.0, color: Color::WHITE}; particle_count];

    for i in 1..particle_count {
        let mut rng = rand::thread_rng();
        particles[i] = Particle::new(i as f32, Color::RGB(rng.gen(), rng.gen(), rng.gen()));
    }

    let mut last_time = Instant::now();

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::W), ..} => { x_angle += 0.05; break; },
                Event::KeyDown { keycode: Some(Keycode::A), ..} => { y_angle -= 0.05; break; },
                Event::KeyDown { keycode: Some(Keycode::S), ..} => { x_angle -= 0.05; break; },
                Event::KeyDown { keycode: Some(Keycode::D), ..} => { y_angle += 0.05; break; },
                Event::KeyDown { keycode: Some(Keycode::Q), ..} => { z_angle -= 0.05; break; },
                Event::KeyDown { keycode: Some(Keycode::E), ..} => { z_angle += 0.05; break; },

                Event::KeyDown { keycode: Some(Keycode::R), ..} => { 
                    x_angle = 0.0;
                    y_angle = 0.0;
                    z_angle = 0.0;
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
            // could use some fixing up
            let (matx, maty, matz) = rot_x(i.x, i.y, i.z, x_angle);
            let (matx, maty, matz) = rot_y(matx, maty, matz, y_angle);
            let (matx, maty, _) = rot_z(matx, maty, matz, z_angle);

            let x = (matx * scale) + WIDTH as f32/2.0;
            let y = (maty * scale) + HEIGHT as f32/2.0;

            if x >= WIDTH as f32 || y >= HEIGHT as f32 || x <= 0.0 || y <= 0.0 { continue; }
            put_pixel(x as u32, y as u32, i.color, &mut framedata);
        }


        if Instant::now() - last_time < Duration::from_secs_f32(1.0 * dt) {
            thread::sleep(Duration::from_secs_f32(1.0 * dt) - (Instant::now() - last_time));
            last_time = Instant::now();
        }

        canvas.clear();
        framebuffer.update(None, &framedata, (WIDTH*4) as usize).expect("Texture update");
        canvas.copy(&framebuffer, None, None).expect("oops");
        canvas.present();
    }


}
