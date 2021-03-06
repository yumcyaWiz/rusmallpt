use crate::core::Ray;
use crate::sampler::Sampler;
use crate::types::Real;
use crate::vec2::*;
use crate::vec3::*;

pub trait Camera {
    fn sample_ray(&self, uv: Vec2, sampler: &mut Sampler) -> Ray;
}

pub struct PinholeCamera {
    position: Vec3, // camera position
    forward: Vec3,  // camera forward direction
    right: Vec3,    // camera right direction
    up: Vec3,       // camera up direction
    f: Real,        // focal length
}

impl PinholeCamera {
    pub fn new(position: Vec3, forward: Vec3, fov: Real) -> Self {
        let right = forward.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        let up = right.cross(forward).normalize();
        let f = 1.0 / (0.5 * fov).tan();
        PinholeCamera {
            position,
            forward,
            right,
            up,
            f,
        }
    }
}

impl Camera for PinholeCamera {
    fn sample_ray(&self, uv: Vec2, _sampler: &mut Sampler) -> Ray {
        let sensor_pos = self.position + uv.x() * self.right + uv.y() * self.up;
        let pinhole_pos = self.position + self.f * self.forward;
        Ray::new(sensor_pos, (pinhole_pos - sensor_pos).normalize())
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use crate::camera::*;

    #[test]
    fn init_camera() {
        let camera = PinholeCamera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            FRAC_PI_2,
        );
        assert_eq!(camera.position, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(camera.forward, Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(camera.right, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(camera.up, Vec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn sample_ray() {
        let camera = PinholeCamera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            FRAC_PI_2,
        );
        let mut sampler = Sampler::new(0);
        let sensor_pos = Vec3::new(1.0, 1.0, 0.0);
        let pinhole_pos = Vec3::new(0.0, 0.0, -1.0);

        assert_eq!(
            camera.sample_ray(Vec2::new(1.0, 1.0), &mut sampler),
            Ray::new(sensor_pos, (pinhole_pos - sensor_pos).normalize())
        );
    }
}
