use crate::core::Ray;
use crate::vec2::*;
use crate::vec3::*;

pub struct Camera {
  position: Vec3,
  forward: Vec3,
  right: Vec3,
  up: Vec3,
}

impl Camera {
  pub fn new(position: &Vec3, forward: &Vec3) -> Self {
    let right = forward.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize();
    let up = right.cross(&forward).normalize();
    Camera {
      position: position.clone(),
      forward: forward.clone(),
      right: right,
      up: up,
    }
  }

  pub fn sample_ray(&self, uv: &Vec2) {
    let sensor_pos = &(&self.position + uv.x() * &self.right + uv.y() * &self.up);
    let pinhole_pos = &(&self.position + &self.forward);
    Ray::new(sensor_pos, &(pinhole_pos - sensor_pos).normalize());
  }
}

#[cfg(test)]
mod tests {
  use crate::camera::*;

  #[test]
  fn init_camera() {
    let camera = Camera::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, -1.0));
    assert_eq!(camera.position, Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(camera.forward, Vec3::new(0.0, 0.0, -1.0));
    assert_eq!(camera.right, Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(camera.up, Vec3::new(0.0, 1.0, 0.0));
  }
}
