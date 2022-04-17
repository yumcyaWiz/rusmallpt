use crate::core::{IntersectableGlobal, Ray};
use crate::scene::Scene;
use crate::vec3::Vec3;

pub trait Integrator {
    // compute radiance coming from given ray
    fn integrate(&self, scene: &Scene, ray: &Ray) -> Vec3;
}

pub struct NormalIntegrator {}

impl NormalIntegrator {
    pub fn new() -> Self {
        NormalIntegrator {}
    }
}

impl Integrator for NormalIntegrator {
    fn integrate(&self, scene: &Scene, ray: &Ray) -> Vec3 {
        if let Some(info) = scene.intersect(ray) {
            0.5 * (info.normal + Vec3::new(1.0, 1.0, 1.0))
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    }
}
