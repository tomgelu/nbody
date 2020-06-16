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
  pub color: [f32; 4],
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
    self.acceleration = force / self.mass;
    self.velocity += self.acceleration * 100000.0;
    self.position += self.velocity * 100000.0;
  }
  pub fn calculate_force(&self, pos: TVec3<f64>, mass: f64) -> TVec3<f64> {
    let dist = glm::distance(&self.position, &pos);
    if dist == 0.0 {
      return glm::vec3(0.0, 0.0, 0.0);
    }
    // println!("{:?}", dist / AU);
    let force = -GRAVITY * ((self.mass * mass) / (dist * dist));
    let direction = normalize(&(self.position - pos));
    return force * direction;
  }
  pub fn draw_planet(&self, c: Context, g: &mut GlGraphics, args: &RenderArgs) {
    let very_little = 0.000000001;
    let model_view = glm::look_at(
      &glm::vec3(0.0, 0.0, 1.0),
      &glm::vec3(very_little, very_little, very_little),
      &glm::vec3(0.0, 1.0, 0.0),
    );

    let perspective_matrix =
      glm::ortho::<f64>(-AU * 10.0, AU * 10.0, -AU * 10.0, AU * 10.0, 0.001, 1.0);
    /*
    let perspective_matrix = glm::perspective::<f64>(
      PI / 4.0,
      args.window_size[0] / args.window_size[1],
      0.1,
      100.0,
    );
    */
    let coords = glm::project(
      &self.position,
      &model_view,
      &perspective_matrix,
      glm::vec4(0.0, 0.0, args.window_size[0], args.window_size[1]),
    );
    Ellipse::new(self.color) // red color
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
