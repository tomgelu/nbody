use glm::*;

fn cartesian_to_spherical(cartesian: &glm::Vec3) -> glm::Vec3 {
  let r: f32 = glm::length(&cartesian);
  if cartesian.x == 0.0 && cartesian.y == 0.0 {
    return glm::vec3(r, 0.0, 0.0);
  }
  let mut theta: f32 = (cartesian.y / cartesian.x).atan();
  let phi: f32 = (glm::length(&glm::vec2(cartesian.x, cartesian.y)) / cartesian.z).atan();
  if cartesian.x < 0.0 && cartesian.y >= 0.0 && theta == 0.0 {
    theta = std::f32::consts::PI;
  } else if cartesian.x < 0.0 && cartesian.y < 0.0 && theta.signum() > 0.0 {
    theta -= std::f32::consts::PI;
  } else if cartesian.x < 0.0 && cartesian.y > 0.0 && theta.signum() < 0.0 {
    theta += std::f32::consts::PI;
  }
  glm::vec3(r, theta, phi)
}

fn spherical_to_cartesian(spherical: &glm::Vec3) -> glm::Vec3 {
  let (r, theta, phi) = (spherical.x, spherical.y, spherical.z);
  let x = r * phi.sin() * theta.cos();
  let y = r * phi.sin() * theta.sin();
  let z = r * phi.cos();
  glm::vec3(x, y, z)
}
