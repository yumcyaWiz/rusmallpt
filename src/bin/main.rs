use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};
use std::sync::{Arc, Mutex};

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

fn simple_scene() -> (PinholeCamera, Scene) {
    let camera = PinholeCamera::new(
        Vec3::new(0.0, 0.0, 6.0),
        Vec3::new(0.0, 0.0, -1.0),
        FRAC_PI_2,
    );

    let sphere1 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0));
    let sphere2 = Box::new(Sphere::new(Vec3::new(-1.5, 0.0, -1.5), 1.0));
    let sphere3 = Box::new(Sphere::new(Vec3::new(1.5, 0.0, 1.5), 1.0));
    let floor = Box::new(Plane::new(
        Vec3::new(-3.0, -1.0, 3.0),
        Vec3::new(6.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -6.0),
    ));
    let primitives: Vec<Box<dyn IntersectableLocal + Send + Sync>> =
        vec![sphere1, sphere2, sphere3, floor];

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

    (camera, Scene::new(primitives, materials))
}

fn cornellbox_scene() -> (PinholeCamera, Scene) {
    let camera = PinholeCamera::new(
        Vec3::new(278.0, 273.0, -900.0),
        Vec3::new(0.0, 0.0, 1.0),
        FRAC_PI_4,
    );

    let floor = Box::new(Plane::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 559.2),
        Vec3::new(556.0, 0.0, 0.0),
    ));
    let right_wall = Box::new(Plane::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 548.8, 0.0),
        Vec3::new(0.0, 0.0, 559.2),
    ));
    let left_wall = Box::new(Plane::new(
        Vec3::new(556.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 559.2),
        Vec3::new(0.0, 548.8, 0.0),
    ));
    let ceil = Box::new(Plane::new(
        Vec3::new(0.0, 548.8, 0.0),
        Vec3::new(556.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 559.2),
    ));
    let back_wall = Box::new(Plane::new(
        Vec3::new(0.0, 0.0, 559.2),
        Vec3::new(0.0, 548.8, 0.0),
        Vec3::new(556.0, 0.0, 0.0),
    ));

    let short_box1 = Box::new(Plane::new(
        Vec3::new(130.0, 165.0, 65.0),
        Vec3::new(-48.0, 0.0, 160.0),
        Vec3::new(160.0, 0.0, 49.0),
    ));
    let short_box2 = Box::new(Plane::new(
        Vec3::new(290.0, 0.0, 114.0),
        Vec3::new(0.0, 165.0, 0.0),
        Vec3::new(-50.0, 0.0, 158.0),
    ));
    let short_box3 = Box::new(Plane::new(
        Vec3::new(130.0, 0.0, 65.0),
        Vec3::new(0.0, 165.0, 0.0),
        Vec3::new(160.0, 0.0, 49.0),
    ));
    let short_box4 = Box::new(Plane::new(
        Vec3::new(82.0, 0.0, 225.0),
        Vec3::new(0.0, 165.0, 0.0),
        Vec3::new(48.0, 0.0, -160.0),
    ));
    let short_box5 = Box::new(Plane::new(
        Vec3::new(240.0, 0.0, 272.0),
        Vec3::new(0.0, 165.0, 0.0),
        Vec3::new(-158.0, 0.0, -47.0),
    ));

    let tall_box1 = Box::new(Plane::new(
        Vec3::new(423.0, 330.0, 247.0),
        Vec3::new(-158.0, 0.0, 49.0),
        Vec3::new(49.0, 0.0, 159.0),
    ));
    let tall_box2 = Box::new(Plane::new(
        Vec3::new(423.0, 0.0, 247.0),
        Vec3::new(0.0, 330.0, 0.0),
        Vec3::new(49.0, 0.0, 159.0),
    ));
    let tall_box3 = Box::new(Plane::new(
        Vec3::new(472.0, 0.0, 406.0),
        Vec3::new(0.0, 330.0, 0.0),
        Vec3::new(-158.0, 0.0, 50.0),
    ));
    let tall_box4 = Box::new(Plane::new(
        Vec3::new(314.0, 0.0, 456.0),
        Vec3::new(0.0, 330.0, 0.0),
        Vec3::new(-49.0, 0.0, -160.0),
    ));
    let tall_box5 = Box::new(Plane::new(
        Vec3::new(265.0, 0.0, 296.0),
        Vec3::new(0.0, 330.0, 0.0),
        Vec3::new(158.0, 0.0, -49.0),
    ));

    let light = Box::new(Plane::new(
        Vec3::new(343.0, 548.6, 227.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 105.0),
    ));

    let primitives: Vec<Box<dyn IntersectableLocal + Send + Sync>> = vec![
        floor, right_wall, left_wall, ceil, back_wall, short_box1, short_box2, short_box3,
        short_box4, short_box5, tall_box1, tall_box2, tall_box3, tall_box4, tall_box5, light,
    ];

    let white = Material::new(
        Vec3::new(0.8, 0.8, 0.8),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );
    let red = Material::new(
        Vec3::new(0.8, 0.05, 0.05),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );
    let green = Material::new(
        Vec3::new(0.05, 0.8, 0.05),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );
    let light_material = Material::new(
        Vec3::new(0.8, 0.8, 0.8),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(34.0, 19.0, 10.0),
    );

    let materials = vec![
        white.clone(),
        red,
        green,
        white.clone(),
        white.clone(),
        white.clone(),
        white.clone(),
        white.clone(),
        white.clone(),
        white.clone(),
        white.clone(),
        white.clone(),
        white.clone(),
        white.clone(),
        white,
        light_material,
    ];

    (camera, Scene::new(primitives, materials))
}

fn main() {
    let width = 512;
    let height = 512;
    let n_samples = 100;
    let max_depth = 100;

    let image = Arc::new(Mutex::new(Image::new(width, height)));

    let (camera, scene) = cornellbox_scene();
    let camera = Arc::new(camera);
    let scene = Arc::new(scene);

    let integrator = Arc::new(PathTracingIntegrator::new(max_depth));

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build()
        .unwrap();

    pool.scope(|s| {
        for i in 0..height {
            for j in 0..width {
                let (image, camera, scene, integrator) = (
                    image.clone(),
                    camera.clone(),
                    scene.clone(),
                    integrator.clone(),
                );

                s.spawn(move |_| {
                    // init sampler
                    let seed = j + width * i;
                    let mut sampler = Sampler::new(seed as u64);
                    // warmup
                    for _k in 0..n_samples {
                        sampler.next_1d();
                    }

                    let width = width as Real;
                    let height = height as Real;
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

                    image.lock().unwrap().set_pixel(i, j, radiance);
                });
            }
        }
    });

    image.lock().unwrap().gamma_correction();
    image.lock().unwrap().write_ppm();
}
