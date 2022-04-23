use crate::core::{IntersectableGlobal, Ray};
use crate::sampler::Sampler;
use crate::scene::Scene;
use crate::vec3::{build_orthonormal_basis, Vec3};

pub trait Integrator {
    // compute radiance coming from given ray
    fn integrate(&self, scene: &Scene, sampler: &mut Sampler, ray: &Ray) -> Vec3;
}

pub struct NormalIntegrator {}

impl NormalIntegrator {
    pub fn new() -> Self {
        NormalIntegrator {}
    }
}

impl Integrator for NormalIntegrator {
    fn integrate(&self, scene: &Scene, sampler: &mut Sampler, ray: &Ray) -> Vec3 {
        if let Some(info) = scene.intersect(ray) {
            0.5 * (info.normal + Vec3::new(1.0, 1.0, 1.0))
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

pub struct PathTracingIntegrator {
    n_samples: u32,
    max_depth: u32,
}

impl PathTracingIntegrator {
    pub fn new(n_samples: u32, max_depth: u32) -> Self {
        PathTracingIntegrator {
            n_samples,
            max_depth,
        }
    }
}

impl Integrator for PathTracingIntegrator {
    fn integrate(&self, scene: &Scene, sampler: &mut Sampler, ray_in: &Ray) -> Vec3 {
        let mut ray = ray_in.clone();
        let mut throughput = Vec3::new(1.0, 1.0, 1.0);
        let mut radiance = Vec3::new(0.0, 0.0, 0.0);

        for i in 0..self.max_depth {
            if let Some(info) = scene.intersect(&ray) {
                // russian roulette

                // terminate if ray hits light
                if scene.has_emission(info.prim_idx) {
                    radiance += throughput * scene.get_emission(info.prim_idx);
                    break;
                }

                // sample direction by BxDF
                // compute local basis
                let (t, n, b) = build_orthonormal_basis(info.normal);

                // transform from world to local
                let wo = -ray.direction;
                let wo_local = wo.world_to_local(t, n, b);

                // sample direction
                let bxdf = scene.get_bxdf(info.prim_idx);
                let (f, wi_local, pdf) = bxdf.sample_bxdf(sampler);

                // update throughput
                // throughput *= 1.0;

                // update ray
            } else {
                radiance += throughput * Vec3::new(1.0, 1.0, 1.0);
                break;
            }
        }

        radiance
    }
}
