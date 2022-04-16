use crate::core::{IntersectInfo, Intersectable, Ray};
use crate::intersector::Intersector;
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
}

impl Intersectable for Scene {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfo> {
        self.intersector.intersect(ray)
    }
}
