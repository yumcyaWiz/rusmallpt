use rusmallpt::camera::{Camera, PinholeCamera};
use rusmallpt::core::IntersectableLocal;
use rusmallpt::image::Image;
use rusmallpt::integrator::{Integrator, NormalIntegrator, PathTracingIntegrator};
use rusmallpt::sampler::Sampler;
use rusmallpt::scene::{Material, Scene};
use rusmallpt::shape::{Plane, Sphere};
use rusmallpt::types::Real;
use rusmallpt::vec2::Vec2;
use rusmallpt::vec3::Vec3;

fn simple_scene() -> (
    PinholeCamera,
    Vec<Box<dyn IntersectableLocal>>,
    Vec<Material>,
) {
    let camera = PinholeCamera::new(Vec3::new(0.0, 0.0, 6.0), Vec3::new(0.0, 0.0, -1.0));

    let sphere1 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0));
    let sphere2 = Box::new(Sphere::new(Vec3::new(-1.5, 0.0, -1.5), 1.0));
    let sphere3 = Box::new(Sphere::new(Vec3::new(1.5, 0.0, 1.5), 1.0));
    let floor = Box::new(Plane::new(
        Vec3::new(-3.0, -1.0, 3.0),
        Vec3::new(6.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -6.0),
    ));
    let primitives: Vec<Box<dyn IntersectableLocal>> = vec![sphere1, sphere2, sphere3, floor];

    let materials: Vec<Material> = vec![
        Material::new(
            Vec3::new(0.8, 0.2, 0.2),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        ),
        Material::new(
            Vec3::new(0.2, 0.8, 0.2),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        ),
        Material::new(
            Vec3::new(0.2, 0.2, 0.8),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        ),
        Material::new(
            Vec3::new(0.8, 0.8, 0.8),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        ),
    ];

    (camera, primitives, materials)
}

fn main() {
    let n_samples = 1;
    let max_depth = 100;

    let mut image = Image::new(512, 512);

    let (camera, primitives, materials) = simple_scene();
    let scene = Scene::new(primitives, materials);

    let integrator = PathTracingIntegrator::new(max_depth);

    for i in 0..image.get_height() {
        for j in 0..image.get_width() {
            // init sampler
            let seed = j + image.get_width() * i;
            let mut sampler = Sampler::new(seed as u64);
            // warmup
            for _k in 0..n_samples {
                sampler.next_1d();
            }

            let width = image.get_width() as Real;
            let height = image.get_height() as Real;
            let mut radiance = Vec3::new(0.0, 0.0, 0.0);
            for _k in 0..n_samples {
                // generate initial ray from camera
                let uv = Vec2::new(
                    (2.0 * (j as Real + sampler.next_1d()) - width) / height,
                    (2.0 * (i as Real + sampler.next_1d()) - height) / height,
                );
                let ray = camera.sample_ray(uv, &mut sampler);

                // compute radiance by integrator
                radiance += integrator.integrate(&scene, &mut sampler, &ray);
            }
            radiance /= n_samples as Real;

            image.set_pixel(i, j, radiance);
        }
    }

    image.gamma_correction();
    image.write_ppm();
}
