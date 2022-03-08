use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

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
}
