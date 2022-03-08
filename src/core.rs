use crate::types::Real;
use crate::vec3::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub tmin: Real,
    pub tmax: Real,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction,
            tmin: 1E-3,
            tmax: 1E9,
        }
    }

    pub fn position(&self, t: Real) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub struct IntersectInfo {
    pub t: Real,
    pub hit_pos: Vec3,
    pub hit_normal: Vec3,
}

#[cfg(test)]
mod test {
    use crate::core::*;

    #[test]
    fn ray_position() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(ray.position(1.0), Vec3::new(0.0, 0.0, 1.0));
    }
}
