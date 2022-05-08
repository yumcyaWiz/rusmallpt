use crate::core::{IntersectInfoGlobal, IntersectableGlobal, Ray};
use crate::shape::Shape;

use std::sync::Arc;

pub struct Intersector {
    shapes: Arc<Vec<Shape>>,
}

impl Intersector {
    pub fn new(shapes: Arc<Vec<Shape>>) -> Self {
        Intersector { shapes }
    }
}

impl IntersectableGlobal for Intersector {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfoGlobal> {
        let mut t = ray.tmax;
        let mut info: Option<IntersectInfoGlobal> = None;
        for (idx, shape) in self.shapes.iter().enumerate() {
            if let Some(surf_info) = shape.intersect(ray) {
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
