use std::f32::consts::FRAC_1_PI;

use crate::sampler::{cosine_weighted_hemisphere, Sampler};
use crate::types::Real;
use crate::vec3::Vec3;

pub struct BxDFSample {
    pub bxdf: Vec3,
    pub direction: Vec3,
    pub pdf: Real,
}

pub trait BxDF {
    fn sample_bxdf(&self, sampler: &mut Sampler) -> BxDFSample;
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
    fn sample_bxdf(&self, sampler: &mut Sampler) -> BxDFSample {
        let uv = sampler.next_2d();
        let (direction, pdf) = cosine_weighted_hemisphere(uv);
        BxDFSample {
            bxdf: FRAC_1_PI * self.rho,
            direction,
            pdf,
        }
    }
}
