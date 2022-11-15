use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{event_loop::*, input::*, window::WindowSettings};
use rand::Rng;
use std::{collections::LinkedList, iter::FromIterator};

fn main(){
    // Change this to OpenGL::V2_1 if this fails.
    let opengl = OpenGL::V3_2;

    const COLS: u32 = 30;
    const ROWS: u32 = 20;
    const SQUARE_WIDTH: u32 = 20;

    const WIDTH: u32 = COLS * SQUARE_WIDTH;
    const HEIGHT: u32 = ROWS * SQUARE_WIDTH;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

        let mut events = Events::new(EventSettings::new()).ups(10);
        while let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                
            }
        }

    println!("Hello!");
}