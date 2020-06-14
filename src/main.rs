extern crate glutin_window;
extern crate graphics;
extern crate nalgebra_glm as glm;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
mod physics;

use glm::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use physics::Body;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;

const GRAVITY: f64 = 6.67428e-11;
const DAY: f64 = 24.0 * 3600.0;
const AU: f64 = 149.6e9; // Astronomical Unit in meters, roughly distance earth -> sun
const SCALE: f64 = 50.0 / AU;
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs, all: &mut [Body]) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            let transform = c.transform.trans(
                args.window_size[0] / 2 as f64,
                args.window_size[1] / 2 as f64,
            );
            for body in all.iter() {
                body.draw_planet(c, gl, transform);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs, all: &mut Vec<Body>) {
        let mut sums: Vec<Vec3> = Vec::new();
        for body in all.iter() {
            let mut forces: Vec<Vec3> = Vec::new();
            for i in 0..all.len() {
                // creates forces
            }
            // add reduced forces to sum vec
        }
        for body in all.iter_mut() {
            let force = sums.pop();
            // apply acceleration
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("nBody simulation", [1024, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    /* My code */
    let sun = Body {
        mass: 1.98892 * 10.0_f64.powf(30.0) as f32,
        radius: 10.0,
        position: glm::vec3(0.0, 0.0, 0.0),
        scaled_position: glm::vec3(0.0, 0.0, 0.0),
        velocity: glm::vec3(0.0, 0.0, 0.0),
        acceleration: glm::vec3(0.0, 0.0, 0.0),
    };

    let earth = Body {
        mass: 5.972 * 10.0_f64.powf(24.0) as f32,
        radius: 5.0,
        position: glm::vec3(50.0, 0.0, 0.0),
        scaled_position: glm::vec3(50.0, 0.0, 0.0),
        velocity: glm::vec3(0.0, 29.78 * 1000.0, 0.0),
        acceleration: glm::vec3(0.0, 0.0, 0.0),
    };

    let mut all: Vec<Body> = Vec::new();
    all.push(sun);
    all.push(earth);
    /*
    let mut rng = rand::thread_rng();
    for _ in 0..10000 {
        let b = Body {
            mass: 5.972 * 10.0_f64.powf(24.0) as f32,
            radius: 0.5,
            position: glm::vec3(0.0, 0.0, 0.0),
            scaled_position: glm::vec3(0.0, 0.0, 0.0),
            velocity: glm::vec3(0.0, 29.78 * 1000.0, 0.0),
            acceleration: glm::vec3(0.0, 0.0, 0.0),
        };
    }
    */

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &mut all);
        }

        if let Some(args) = e.update_args() {
            app.update(&args, &mut all);
        }
    }
}
