use rusmallpt::camera::Camera;
use rusmallpt::core::Intersectable;
use rusmallpt::image::Image;
use rusmallpt::intersector::Intersector;
use rusmallpt::shape::Sphere;
use rusmallpt::vec2::Vec2;
use rusmallpt::vec3::Vec3;

fn main() {
    let mut image = Image::new(512, 512);
    let camera = Camera::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

    let sphere1 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0));
    let sphere2 = Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 1.0));
    let sphere3 = Box::new(Sphere::new(Vec3::new(1.0, 0.0, 1.0), 1.0));
    let intersector = Intersector::new(vec![sphere1, sphere2, sphere3]);

    for i in 0..image.get_height() {
        for j in 0..image.get_width() {
            let uv = Vec2::new(
                (2.0 * (j as f32) - 512.0) / 512.0,
                (2.0 * (i as f32) - 512.0) / 512.0,
            );
            let ray = camera.sample_ray(uv);

            match intersector.intersect(&ray) {
                Some(info) => {
                    image.set_pixel(i, j, 0.5 * (info.hit_normal + Vec3::new(1.0, 1.0, 1.0)));
                }
                None => {}
            }
        }
    }

    image.write_ppm();
}
