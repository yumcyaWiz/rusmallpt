use crate::types::Real;
use crate::vec3::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct Ray {
    pub origin: Vec3,    // ray origin
    pub direction: Vec3, // ray direction
    pub tmin: Real,      // minimum hittable distance
    pub tmax: Real,      // maximum hittable distance
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
pub struct IntersectInfoLocal {
    pub t: Real,      // distance to hit point
    pub pos: Vec3,    // hit position
    pub normal: Vec3, // hit normal
}

#[derive(Debug, PartialEq)]
pub struct IntersectInfoGlobal {
    pub t: Real,       // distance to hit point
    pub pos: Vec3,     // hit position
    pub normal: Vec3,  // hit normal
    pub prim_idx: u32, // index of hit primitive
}

#[derive(Debug, PartialEq)]
pub struct ShadingInfo {
    pub x: Vec3,  // position
    pub n: Vec3,  // shading normal
    pub wo: Vec3, // outgoing direction in tangent space
}

// NOTE: local means it doesn't contain hit primitive index
pub trait IntersectableLocal {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfoLocal>;
}

// NOTE: global means it contains hit primitive index
pub trait IntersectableGlobal {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfoGlobal>;
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
    use std::f32::consts::FRAC_PI_4;

    #[test]
    fn ray_position() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(ray.position(1.0), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_spherical_to_cartesian() {
        assert_eq!(spherical_to_cartesian(0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(
            spherical_to_cartesian(FRAC_PI_4, FRAC_PI_4),
            Vec3::new(
                FRAC_PI_4.cos() * FRAC_PI_4.sin(),
                FRAC_PI_4.cos(),
                FRAC_PI_4.sin() * FRAC_PI_4.sin()
            )
        );
    }
}
