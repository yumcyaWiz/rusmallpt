use crate::core::{IntersectInfo, Intersectable, Ray};

use std::rc::Rc;

pub struct Intersector {
    intersectables: Rc<Vec<Box<dyn Intersectable>>>,
}

impl Intersector {
    pub fn new(intersectables: Rc<Vec<Box<dyn Intersectable>>>) -> Self {
        Intersector { intersectables }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<IntersectInfo> {
        let mut t = ray.tmax;
        let mut info: Option<IntersectInfo> = None;
        for (idx, intersectable) in self.intersectables.iter().enumerate() {
            if let Some(surf_info) = intersectable.intersect(ray) {
                if surf_info.t < t {
                    t = surf_info.t;
                    info = Some(IntersectInfo {
                        t: surf_info.t,
                        pos: surf_info.pos,
                        normal: surf_info.normal,
                        prim_idx: idx as u32,
                    });
                }
            }
        }
        info
    }
}
