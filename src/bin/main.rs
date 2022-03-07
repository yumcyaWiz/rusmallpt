use rusmallpt::image::Image;
use rusmallpt::vec3::Vec3;

fn main() {
    let mut image = Image::new(512, 512);

    for i in 0..image.get_height() {
        for j in 0..image.get_width() {
            image.set_pixel(
                i,
                j,
                &Vec3::new((i as f32) / 512.0, (j as f32) / 512.0, 1.0),
            );
        }
    }

    image.write_ppm();
}
