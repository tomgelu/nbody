extern crate glutin_window;
extern crate graphics;
extern crate nalgebra_glm as glm;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
mod conv;
mod physics;

use glm::*;
use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use physics::Body;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;
use std::f32::consts::PI;

const GRAVITY: f64 = 6.67428e-11;
const DAY: f64 = 24.0 * 3600.0;
const AU: f64 = 149.6e9; // Astronomical Unit in meters, roughly distance earth -> sun
const SCALE: f64 = 50.0 / AU;
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs, all: &mut [Body], opts: &mut (f64, f64)) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            for body in all.iter() {
                body.draw_planet(c, gl, args);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs, all: &mut Vec<Body>, opts: &mut (f64, f64)) {
        let mut sums: Vec<TVec3<f64>> = Vec::new();
        for body in all.iter() {
            let mut forces: Vec<TVec3<f64>> = Vec::new();
            for i in 0..all.len() {
                let force = body.calculate_force(all[i].position, all[i].mass);
                forces.push(force);
            }
            let sum = forces
                .iter()
                .fold(glm::vec3::<f64>(0.0, 0.0, 0.0), |acc, curr| {
                    acc + curr.clone()
                });
            sums.push(sum);
            // add reduced forces to sum vec
        }
        for body in all.iter_mut() {
            let force = sums.pop();
            match force {
                Some(value) => body.update(value),
                None => (),
            }
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

    let sun = Body {
        name: String::from("Sun"),
        mass: 333000.0,
        radius: 10.0,
        position: glm::vec3(0.0, 0.0, 0.0),
        velocity: glm::vec3(0.0, 0.0, 0.0),
        acceleration: glm::vec3(0.0, 0.0, 0.0),
    };

    let earth = Body {
        name: String::from("Earth"),
        mass: 1.0,
        radius: 5.0,
        position: glm::vec3(AU, 0.0, 0.0),
        velocity: glm::vec3(50000.0, 0.0, 0.0),
        acceleration: glm::vec3(0.0, 0.0, 0.0),
    };

    let mut all: Vec<Body> = Vec::new();
    all.push(sun);
    all.push(earth);
    /*
    let solar_mass: f64 = 1.989 * 10.0_f64.powi(30);
    let sun = Body {
        mass: solar_mass,
        radius: 10.0,
        position: glm::vec3(0.0, 0.0, 0.0),
        velocity: glm::vec3(0.0, 0.0, 0.0),
        acceleration: glm::vec3(0.0, 0.0, 0.0),
    };

    let earth = Body {
        mass: 5.972 * 10.0_f64.powi(24),
        radius: 5.0,
        position: glm::vec3(1.0 * AU, 1.0 * AU, 1.0 * AU),
        velocity: glm::vec3(0.0, 29.78 * 1000.0, 0.0),
        acceleration: glm::vec3(0.0, 0.0, 0.0),
    };

    let mut all: Vec<Body> = Vec::new();
    all.push(sun);
    // all.push(earth);
    let area = 1;
    let mut rng = rand::thread_rng();
    for _ in 0..0 {
        let spherical_position = glm::vec3::<f64>(
            (rng.gen_range(0., 1.).sqrt() - 0.5) * area as f64,
            rng.gen_range(0.0, 2.0 * PI) as f64,
            rng.gen_range(0.0, PI * 2.0) as f64,
        );
        let spherical_velocity = glm::vec3(1.0 as f64, 29.78 * 1000.0 as f64, 0.0 as f64);
        let b = Body {
            mass: 0.1 * solar_mass,
            radius: 5.0,
            position: conv::spherical_to_cartesian(&spherical_position),
            velocity: conv::spherical_to_cartesian(&spherical_velocity),
            acceleration: glm::vec3(0.0, 0.0, 0.0),
        };
        all.push(b);
    }
    */
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        let mut opts = (f64::MAX, f64::MIN);
        if let Some(args) = e.render_args() {
            app.render(&args, &mut all, &mut opts);
        }

        if let Some(args) = e.update_args() {
            app.update(&args, &mut all, &mut opts);
        }
    }
}
