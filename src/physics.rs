use glm::*;
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::f64::consts::PI;

use super::AU;
use super::BLUE;
use super::DAY;
use super::GRAVITY;
use super::SCALE;

#[derive(Debug, Clone)]
pub struct Body {
  pub name: String,
  pub mass: f64,
  pub position: TVec3<f64>,
  pub velocity: TVec3<f64>,
  pub acceleration: TVec3<f64>,
  pub radius: f64,
}

impl Body {
  pub fn update(&mut self, force: TVec3<f64>) {
    if force.x.is_nan() {
      panic!("osef");
    }
    self.acceleration += force * 10000.0;
    self.velocity += self.acceleration * 10000.0;
    self.position += self.velocity * 10000.0;
  }
  pub fn calculate_force(&self, pos: TVec3<f64>, mass: f64) -> TVec3<f64> {
    let dist = glm::distance(&self.position, &pos);
    if dist == 0.0 {
      return glm::vec3(0.0, 0.0, 0.0);
    }
    let force = -GRAVITY * ((self.mass * mass) / (dist * dist));
    println!("{:?}", force);
    let angle = pos - self.position;
    return force * angle;
  }
  pub fn draw_planet(&self, c: Context, g: &mut GlGraphics, args: &RenderArgs) {
    let model_view = glm::look_at(
      &glm::vec3(0.0, -1.0, 0.0),
      &glm::vec3(0.0, 0.0, 0.0),
      &glm::vec3(0.0, 0.0, 1.0),
    );
    /*
    let perspective_matrix = glm::ortho::<f64>(
      -AU * 1000.0,
      AU * 1000.0,
      -AU * 1000.0,
      AU * 1000.0,
      0.0,
      AU * 1000.0,
    );
    */
    let perspective_matrix = glm::perspective::<f64>(PI / 4.0, 4.0 / 3.0, 0.1, 1.0);
    let coords = glm::project(
      &self.position,
      &model_view,
      &perspective_matrix,
      glm::vec4(0.0, 0.0, args.window_size[0], args.window_size[1]),
    );
    //println!("----------------------------------");
    println!("Real pos {:?}", self.position);
    println!("{:?} : Coords {:?}", self.name, coords);
    Ellipse::new(BLUE) // red color
      .draw(
        [
          coords.x as f64,
          coords.y as f64,
          self.radius as f64,
          self.radius as f64,
        ],
        &c.draw_state,
        c.transform,
        g,
      );
  }
}
