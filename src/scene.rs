use crate::core::{IntersectInfo, Intersectable, Ray};
use crate::intersector::Intersector;
use crate::material::{BxDF, Lambert};
use crate::vec3::Vec3;

use std::rc::Rc;

pub struct Material {
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub emission: Vec3,
}

pub struct Scene {
    intersectables: Rc<Vec<Box<dyn Intersectable>>>,
    materials: Vec<Material>,
    intersector: Intersector,
}

impl Scene {
    pub fn new(intersectables: Vec<Box<dyn Intersectable>>, materials: Vec<Material>) -> Self {
        let intersectables = Rc::new(intersectables);
        Scene {
            intersectables: intersectables.clone(),
            materials,
            intersector: Intersector::new(intersectables),
        }
    }

    fn get_bxdf(&self, prim_idx: u32) -> Box<dyn BxDF> {
        Box::new(Lambert::new(Vec3::new(1.0, 1.0, 1.0)))
    }
}

impl Intersectable for Scene {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfo> {
        self.intersector.intersect(ray)
    }
}
