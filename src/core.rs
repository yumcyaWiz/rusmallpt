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

#[derive(Debug, PartialEq)]
pub struct IntersectInfo {
    pub t: Real,
    pub hit_pos: Vec3,
    pub hit_normal: Vec3,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfo>;
}

pub struct SampledPosition {
    pub position: Vec3,
    pub pdf: Real,
}

pub struct SampledDirection {
    pub direction: Vec3,
    pub pdf: Real,
}

pub fn spherical_to_cartesian(theta: Real, phi: Real) -> Vec3 {
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    Vec3::new(cos_phi * sin_theta, cos_theta, sin_phi * sin_theta)
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
