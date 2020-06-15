use glm::*;

pub fn cartesian_to_spherical(cartesian: &glm::TVec3<f64>) -> glm::TVec3<f64> {
  let r: f64 = glm::length(&cartesian);
  if cartesian.x == 0.0 && cartesian.y == 0.0 {
    return glm::vec3(r, 0.0, 0.0);
  }
  let mut theta: f64 = (cartesian.y / cartesian.x).atan();
  let phi: f64 = (glm::length(&glm::vec2(cartesian.x, cartesian.y)) / cartesian.z).atan();
  if cartesian.x < 0.0 && cartesian.y >= 0.0 && theta == 0.0 {
    theta = std::f64::consts::PI;
  } else if cartesian.x < 0.0 && cartesian.y < 0.0 && theta.signum() > 0.0 {
    theta -= std::f64::consts::PI;
  } else if cartesian.x < 0.0 && cartesian.y > 0.0 && theta.signum() < 0.0 {
    theta += std::f64::consts::PI;
  }
  glm::vec3(r, theta, phi)
}

pub fn spherical_to_cartesian(spherical: &glm::TVec3<f64>) -> glm::TVec3<f64> {
  let (r, theta, phi) = (spherical.x, spherical.y, spherical.z);
  let x = r * phi.sin() * theta.cos();
  let y = r * phi.sin() * theta.sin();
  let z = r * phi.cos();
  glm::vec3(x, y, z)
}
