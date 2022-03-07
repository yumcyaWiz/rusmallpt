use rusmallpt::camera::Camera;
use rusmallpt::image::Image;
use rusmallpt::vec2::Vec2;
use rusmallpt::vec3::Vec3;

fn main() {
    let mut image = Image::new(512, 512);
    let camera = Camera::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

    for i in 0..image.get_height() {
        for j in 0..image.get_width() {
            let uv = Vec2::new((i as f32) / 512.0, (j as f32) / 512.0);
            let ray = camera.sample_ray(uv);

            image.set_pixel(i, j, 0.5 * (ray.direction + Vec3::new(1.0, 1.0, 1.0)));
        }
    }

    image.write_ppm();
}
