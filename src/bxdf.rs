use std::f32::consts::FRAC_1_PI;

use crate::core::ShadingInfo;
use crate::sampler::{cosine_weighted_hemisphere, Sampler};
use crate::types::Real;
use crate::vec3::Vec3;

// NOTE: assuming vectors in tangent space

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    -v + 2.0 * v.dot(n) * n
}

fn abs_cos_theta(v: Vec3) -> Real {
    v.y().abs()
}

pub struct BxDFSample {
    pub f: Vec3,   // BxDF value
    pub wi: Vec3,  // sampled direction
    pub pdf: Real, // pdf
}

pub trait BxDF {
    fn sample_direction(&self, info: &ShadingInfo, sampler: &mut Sampler) -> BxDFSample;
}

pub struct Lambert {
    rho: Vec3, // albedo
}

impl Lambert {
    pub fn new(rho: Vec3) -> Self {
        Lambert { rho }
    }
}

impl BxDF for Lambert {
    fn sample_direction(&self, _info: &ShadingInfo, sampler: &mut Sampler) -> BxDFSample {
        let uv = sampler.next_2d();
        let (wi, pdf) = cosine_weighted_hemisphere(uv);
        BxDFSample {
            f: FRAC_1_PI * self.rho,
            wi,
            pdf,
        }
    }
}

pub struct IdealReflection {}

impl BxDF for IdealReflection {
    fn sample_direction(&self, info: &ShadingInfo, _sampler: &mut Sampler) -> BxDFSample {
        let wi = reflect(info.wo, info.n);
        BxDFSample {
            f: Vec3::new(1.0, 1.0, 1.0) / abs_cos_theta(wi),
            wi,
            pdf: 1.0,
        }
    }
}
