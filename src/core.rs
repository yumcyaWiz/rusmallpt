use crate::types::Real;
use crate::vec3::Vec3;

struct Ray {
  origin: Vec3,
  direction: Vec3,
}

impl Ray {
  fn position(&self, t: Real) -> Vec3 {
    &self.origin + t * &self.direction
  }
}

struct IntersectInfo {
  t: Real,
  hit_pos: Vec3,
  hit_normal: Vec3,
}
