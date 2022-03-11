use std::f32::consts::{FRAC_1_PI, PI};

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

use crate::core::{spherical_to_cartesian, SampledDirection};
use crate::vec2::Vec2;

pub struct Sampler {
    rng: Pcg32,
}

impl Sampler {
    pub fn new(seed: u64) -> Self {
        Sampler {
            rng: Pcg32::seed_from_u64(seed),
        }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.rng = Pcg32::seed_from_u64(seed);
    }

    pub fn next(&mut self) -> f32 {
        self.rng.gen()
    }

    pub fn next_2d(&mut self) -> Vec2 {
        Vec2::new(self.next(), self.next())
    }
}

pub fn cosine_weighted_hemisphere(uv: Vec2) -> SampledDirection {
    let theta = 0.5 * (1.0 - 2.0 * uv.x()).clamp(-1.0, 1.0).acos();
    let phi = 2.0 * PI * uv.y();

    let cos_theta = theta.cos();
    let pdf = FRAC_1_PI * cos_theta;

    SampledDirection {
        direction: spherical_to_cartesian(theta, phi),
        pdf,
    }
}
