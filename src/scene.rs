use crate::core::{IntersectInfo, Intersectable, Ray};
use crate::material::BxDF;

pub struct Scene {
    primitives: Vec<Box<dyn Intersectable>>,
    bxdfs: Vec<Box<dyn BxDF>>,
}

impl Scene {
    fn intersect(ray: &Ray) -> Option<IntersectInfo> {
        todo!();
    }
}
