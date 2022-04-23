use crate::core::{IntersectableGlobal, Ray};
use crate::sampler::Sampler;
use crate::scene::Scene;
use crate::types::Real;
use crate::vec3::Vec3;

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
    #[allow(unused_variables)]
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
        let mut radiance = Vec3::new(0.0, 0.0, 0.0);

        for _k in 0..self.n_samples {
            let mut ray = ray_in.clone();
            let mut throughput = Vec3::new(1.0, 1.0, 1.0);

            for _depth in 0..self.max_depth {
                if let Some(info) = scene.intersect(&ray) {
                    // russian roulette
                    let russian_roulette_prob = throughput.max().min(1.0);
                    if sampler.next_1d() >= russian_roulette_prob {
                        break;
                    }
                    throughput /= russian_roulette_prob;

                    // terminate if ray hits light
                    if scene.has_emission(info.prim_idx) {
                        radiance += throughput * scene.get_emission(info.prim_idx);
                        break;
                    }

                    // sample direction by BxDF
                    let shading_info = scene.get_shading_info(-ray.direction, &info);

                    // sample direction
                    let bxdf = scene.get_bxdf(info.prim_idx);
                    let bxdf_sample = bxdf.sample_direction(&shading_info, sampler);

                    // update throughput
                    throughput *= bxdf_sample.f * bxdf_sample.wi.y().abs() / bxdf_sample.pdf;

                    // update ray
                    ray.origin = info.pos;
                    ray.direction = bxdf_sample.wi.local_to_world(
                        shading_info.t,
                        shading_info.n,
                        shading_info.b,
                    );
                } else {
                    radiance += throughput * Vec3::new(1.0, 1.0, 1.0);
                    break;
                }
            }
        }
        radiance
    }
}
