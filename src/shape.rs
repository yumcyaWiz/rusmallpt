use crate::core::{IntersectInfo, Ray};
use crate::types::Real;
use crate::vec3::Vec3;

trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfo>;
}

struct Sphere {
    center: Vec3,
    radius: Real,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfo> {
        let b = (ray.origin - self.center).dot(ray.direction);
        let c = (ray.origin - self.center).length2() - self.radius * self.radius;
        let d = b * b - c;
        if d < 0.0 {
            return None;
        }

        let mut t = -b - b.sqrt();
        if t < ray.tmin || t > ray.tmax {
            t = -b + b.sqrt();
            if t < ray.tmin || t > ray.tmax {
                return None;
            }
        }

        let hit_pos = ray.position(t);
        Some(IntersectInfo {
            t,
            hit_pos,
            hit_normal: (hit_pos - self.center).normalize(),
        })
    }
}
