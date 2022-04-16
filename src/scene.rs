use crate::core::{IntersectInfo, Intersectable, Ray};
use crate::intersector::Intersector;

use std::rc::Rc;

pub struct Scene {
    intersectables: Rc<Vec<Box<dyn Intersectable>>>,
    intersector: Intersector,
}

impl Scene {
    pub fn new(intersectables: Vec<Box<dyn Intersectable>>) -> Self {
        let intersectables = Rc::new(intersectables);
        Scene {
            intersectables: intersectables.clone(),
            intersector: Intersector::new(intersectables),
        }
    }
}

impl Intersectable for Scene {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfo> {
        self.intersector.intersect(ray)
    }
}
