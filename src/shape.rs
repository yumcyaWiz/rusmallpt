use crate::core::{Intersectable, Ray, SurfaceInfo};
use crate::types::Real;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: Real,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Real) -> Self {
        Sphere { center, radius }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInfo> {
        let b = (ray.origin - self.center).dot(ray.direction);
        let c = (ray.origin - self.center).length2() - self.radius * self.radius;
        let d = b * b - c;
        if d < 0.0 {
            return None;
        }

        let mut t = -b - d.sqrt();
        if t < ray.tmin || t > ray.tmax {
            t = -b + d.sqrt();
            if t < ray.tmin || t > ray.tmax {
                return None;
            }
        }

        let pos = ray.position(t);
        Some(SurfaceInfo {
            t,
            pos,
            normal: (pos - self.center).normalize(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::shape::*;

    #[test]
    fn sphere_intersect() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(
            sphere.intersect(&ray),
            Some(SurfaceInfo {
                t: 1.0,
                pos: Vec3::new(0.0, 0.0, -1.0),
                normal: Vec3::new(0.0, 0.0, -1.0)
            })
        );
    }
}
