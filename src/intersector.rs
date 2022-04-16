use crate::core::{IntersectInfo, Intersectable, Ray};

use std::rc::Rc;

pub struct Intersector {
    intersectables: Rc<Vec<Box<dyn Intersectable>>>,
}

impl Intersector {
    pub fn new(intersectables: Rc<Vec<Box<dyn Intersectable>>>) -> Self {
        Intersector { intersectables }
    }
}

impl Intersectable for Intersector {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfo> {
        let mut t = ray.tmax;
        let mut info: Option<IntersectInfo> = None;
        for intersectable in self.intersectables.iter() {
            if let Some(info_each) = intersectable.intersect(ray) {
                if info_each.t < t {
                    t = info_each.t;
                    info = Some(info_each);
                }
            }
        }
        info
    }
}
