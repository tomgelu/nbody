use glm::*;
use graphics::*;
use opengl_graphics::GlGraphics;

use super::BLUE;
use super::DAY;
use super::GRAVITY;
use super::SCALE;

#[derive(Debug, Clone)]
pub struct Body {
  pub mass: f32,
  pub position: Vec3,
  pub scaled_position: Vec3,
  pub velocity: Vec3,
  pub acceleration: Vec3,
  pub radius: f32,
}

impl Body {
  pub fn draw_planet(&self, c: Context, g: &mut GlGraphics, transform: [[f64; 3]; 2]) {
    println!(
      "{:?}",
      [
        self.scaled_position.x as f64,
        self.scaled_position.y as f64,
        self.radius as f64,
        self.radius as f64,
      ]
    );
    Ellipse::new(BLUE) // red color
      .draw(
        [
          self.scaled_position.x as f64,
          self.scaled_position.y as f64,
          self.radius as f64,
          self.radius as f64,
        ],
        &c.draw_state,
        transform,
        g,
      );
  }
}
