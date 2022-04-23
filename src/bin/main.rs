use rusmallpt::camera::Camera;
use rusmallpt::core::IntersectableLocal;
use rusmallpt::image::Image;
use rusmallpt::integrator::{Integrator, NormalIntegrator, PathTracingIntegrator};
use rusmallpt::sampler::Sampler;
use rusmallpt::scene::{Material, Scene};
use rusmallpt::shape::Sphere;
use rusmallpt::vec2::Vec2;
use rusmallpt::vec3::Vec3;

fn main() {
    let mut image = Image::new(512, 512);
    let camera = Camera::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

    let sphere1 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0));
    let sphere2 = Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 1.0));
    let sphere3 = Box::new(Sphere::new(Vec3::new(1.0, 0.0, 1.0), 1.0));
    let primitives: Vec<Box<dyn IntersectableLocal>> = vec![sphere1, sphere2, sphere3];
    let materials: Vec<Material> = vec![
        Material::new(
            Vec3::new(1.0, 0.8, 0.8),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        ),
        Material::new(
            Vec3::new(0.8, 1.0, 0.8),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        ),
        Material::new(
            Vec3::new(0.8, 0.8, 1.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        ),
    ];
    let scene = Scene::new(primitives, materials);

    let integrator = PathTracingIntegrator::new(1, 100);

    for i in 0..image.get_height() {
        for j in 0..image.get_width() {
            let uv = Vec2::new(
                (2.0 * (j as f32) - 512.0) / 512.0,
                (2.0 * (i as f32) - 512.0) / 512.0,
            );
            let ray = camera.sample_ray(uv);

            let mut sampler = Sampler::new(0);
            let radiance = integrator.integrate(&scene, &mut sampler, &ray);

            image.set_pixel(i, j, radiance);
        }
    }

    image.write_ppm();
}
