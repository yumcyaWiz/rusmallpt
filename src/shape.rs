use crate::core::{IntersectInfoLocal, IntersectableLocal, Ray};
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

impl IntersectableLocal for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfoLocal> {
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
        Some(IntersectInfoLocal {
            t,
            pos,
            normal: (pos - self.center).normalize(),
        })
    }
}

pub struct Plane {
    left_corner_point: Vec3,
    center: Vec3,
    normal: Vec3,
    right_dir: Vec3,
    up_dir: Vec3,
    right_dir_length: Real,
    up_dir_length: Real,
}

impl Plane {
    pub fn new(left_corner_point: Vec3, right: Vec3, up: Vec3) -> Self {
        Plane {
            left_corner_point,
            center: left_corner_point + 0.5 * right + 0.5 * up,
            normal: right.cross(up).normalize(),
            right_dir: right.normalize(),
            up_dir: up.normalize(),
            right_dir_length: right.length(),
            up_dir_length: up.length(),
        }
    }
}

impl IntersectableLocal for Plane {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfoLocal> {
        let t = -(ray.origin - self.center).dot(self.normal) / ray.direction.dot(self.normal);
        if t < ray.tmin || t > ray.tmax {
            return None;
        }

        let pos = ray.position(t);
        let dx = (pos - self.left_corner_point).dot(self.right_dir);
        let dy = (pos - self.left_corner_point).dot(self.up_dir);
        if dx < 0.0 || dx > self.right_dir_length || dy < 0.0 || dy > self.up_dir_length {
            return None;
        }

        Some(IntersectInfoLocal {
            t,
            pos,
            normal: self.normal,
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
            Some(IntersectInfoLocal {
                t: 1.0,
                pos: Vec3::new(0.0, 0.0, -1.0),
                normal: Vec3::new(0.0, 0.0, -1.0)
            })
        );
    }

    #[test]
    fn plane_intersect() {
        let plane = Plane::new(
            Vec3::new(-1.0, 0.0, 1.0),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -2.0),
        );
        let ray = Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        assert_eq!(
            plane.intersect(&ray),
            Some(IntersectInfoLocal {
                t: 1.0,
                pos: Vec3::new(0.0, 0.0, 0.0),
                normal: Vec3::new(0.0, 1.0, 0.0)
            })
        )
    }
}
