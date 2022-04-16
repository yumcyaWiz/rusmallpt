use crate::bxdf::{BxDF, Lambert};
use crate::core::{IntersectInfo, IntersectableGlobal, IntersectableLocal, Ray};
use crate::intersector::Intersector;
use crate::vec3::Vec3;

use std::rc::Rc;

pub struct Material {
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub emission: Vec3,
}

pub struct Scene {
    primitives: Rc<Vec<Box<dyn IntersectableLocal>>>,
    materials: Vec<Material>,
    intersector: Intersector,
}

impl Scene {
    pub fn new(primitives: Vec<Box<dyn IntersectableLocal>>, materials: Vec<Material>) -> Self {
        let primitives = Rc::new(primitives);
        Scene {
            primitives: primitives.clone(),
            materials,
            intersector: Intersector::new(primitives),
        }
    }

    fn get_bxdf(&self, prim_idx: u32) -> Box<dyn BxDF> {
        let material = &self.materials[prim_idx as usize];
        Box::new(Lambert::new(material.diffuse))
    }
}

impl IntersectableGlobal for Scene {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfo> {
        self.intersector.intersect(ray)
    }
}
