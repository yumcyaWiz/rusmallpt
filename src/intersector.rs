use crate::core::{IntersectInfo, Intersectable, Ray};

pub struct Intersector {
    intersectables: Vec<Box<dyn Intersectable>>,
}

impl Intersector {
    pub fn new(intersectables: Vec<Box<dyn Intersectable>>) -> Self {
        Intersector { intersectables }
    }
}

impl Intersectable for Intersector {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfo> {
        let mut t = ray.tmax;
        let mut info: Option<IntersectInfo> = None;
        for intersectable in &self.intersectables {
            match intersectable.intersect(ray) {
                Some(info_each) => {
                    if info_each.t < t {
                        t = info_each.t;
                        info = Some(info_each);
                    }
                }
                None => {}
            }
        }
        info
    }
}
