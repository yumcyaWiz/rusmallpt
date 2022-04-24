use crate::core::{IntersectInfoGlobal, IntersectableGlobal, IntersectableLocal, Ray};

use std::sync::Arc;

pub struct Intersector {
    intersectables: Arc<Vec<Box<dyn IntersectableLocal + Send + Sync>>>,
}

impl Intersector {
    pub fn new(intersectables: Arc<Vec<Box<dyn IntersectableLocal + Send + Sync>>>) -> Self {
        Intersector { intersectables }
    }
}

impl IntersectableGlobal for Intersector {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfoGlobal> {
        let mut t = ray.tmax;
        let mut info: Option<IntersectInfoGlobal> = None;
        for (idx, intersectable) in self.intersectables.iter().enumerate() {
            if let Some(surf_info) = intersectable.intersect(ray) {
                if surf_info.t < t {
                    t = surf_info.t;
                    info = Some(IntersectInfoGlobal {
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
