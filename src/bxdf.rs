use std::f32::consts::FRAC_1_PI;

use crate::sampler::{cosine_weighted_hemisphere, Sampler};
use crate::types::Real;
use crate::vec3::Vec3;

pub trait BxDF {
    fn sample_bxdf(&self, sampler: &mut Sampler) -> (Vec3, Vec3, Real);
}

pub struct Lambert {
    rho: Vec3,
}

impl Lambert {
    pub fn new(rho: Vec3) -> Self {
        Lambert { rho }
    }
}

impl BxDF for Lambert {
    fn sample_bxdf(&self, sampler: &mut Sampler) -> (Vec3, Vec3, Real) {
        let uv = sampler.next_2d();
        let (direction, pdf) = cosine_weighted_hemisphere(uv);
        (FRAC_1_PI * self.rho, direction, pdf)
    }
}
