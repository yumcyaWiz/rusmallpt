use crate::core::{IntersectInfo, Intersectable, Ray};

struct Intersector {
    intersectables: Vec<Box<dyn Intersectable>>,
}

impl Intersectable for Intersector {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfo> {
        let mut t = ray.tmin;
        let mut info: Option<IntersectInfo> = None;
        for intersectable in &self.intersectables {
            match intersectable.intersect(ray) {
                Some(info_each) => {
                    if t < info_each.t {
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
