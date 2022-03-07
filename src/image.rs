use std::fs;

use crate::vec3::Vec3;

pub struct Image {
  width: usize,
  height: usize,
  pixels: Vec<f32>,
}

impl Image {
  pub fn new(width: usize, height: usize) -> Self {
    Image {
      width,
      height,
      pixels: vec![0.0; 3 * width * height],
    }
  }

  pub fn get_pixel(&self, i: usize, j: usize) -> Vec3 {
    let base_index = 3 * self.width * i + 3 * j;
    Vec3::new(
      self.pixels[base_index + 0],
      self.pixels[base_index + 1],
      self.pixels[base_index + 2],
    )
  }

  pub fn set_pixel(&mut self, i: usize, j: usize, rgb: &Vec3) {
    let base_index = 3 * self.width * i + 3 * j;
    self.pixels[base_index + 0] = rgb.x();
    self.pixels[base_index + 1] = rgb.y();
    self.pixels[base_index + 2] = rgb.z();
  }

  pub fn gamma_correction(&mut self) {
    for i in 0..self.height {
      for j in 0..self.width {
        let base_index = 3 * self.width * i + 3 * j;
        self.pixels[base_index + 0] = self.pixels[base_index + 0].powf(1.0 / 2.2);
        self.pixels[base_index + 1] = self.pixels[base_index + 1].powf(1.0 / 2.2);
        self.pixels[base_index + 2] = self.pixels[base_index + 2].powf(1.0 / 2.2);
      }
    }
  }

  pub fn write_ppm(&self) {
    let mut contents = String::new();
    contents.push_str("P3\n");
    contents.push_str(format!("{}\n", 255).as_str());
    contents.push_str(format!("{} {}\n", self.width, self.height).as_str());

    for i in 0..self.height {
      for j in 0..self.width {
        let base_index = 3 * self.width * i + 3 * j;
        let r = (255.0 * self.pixels[base_index + 0]).clamp(0.0, 255.0) as i32;
        let g = (255.0 * self.pixels[base_index + 1]).clamp(0.0, 255.0) as i32;
        let b = (255.0 * self.pixels[base_index + 2]).clamp(0.0, 255.0) as i32;
        contents.push_str(format!("{} {} {}\n", r, g, b).as_str())
      }
    }

    match fs::write("output.ppm", contents) {
      Err(why) => panic!("couldn't write output.ppm: {}", why),
      Ok(_) => println!("wrote output.ppm"),
    }
  }
}
