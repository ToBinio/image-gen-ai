use image::{DynamicImage, GenericImage, Rgba};
use std::ops::Range;
use std::path::{Path, PathBuf};

pub struct SimpleImage {
    img: DynamicImage,
}

impl SimpleImage {
    pub fn new() -> SimpleImage {
        SimpleImage {
            img: DynamicImage::new_rgb8(64, 64),
        }
    }

    pub fn set_rect(&mut self, x: Range<u32>, y: Range<u32>, color: Rgba<u8>) {
        for x in x {
            for y in y.clone() {
                self.img.put_pixel(x, y, color);
            }
        }
    }

    pub fn save(&self, path: &str) {
        self.img.save(path).unwrap();
    }

    pub fn compare(&self, other: &DynamicImage) -> f64 {
        image_compare::rgb_hybrid_compare(self.img.as_rgb8().unwrap(), other.as_rgb8().unwrap())
            .unwrap()
            .score
    }

    pub fn get_image(&self) -> &DynamicImage {
        &self.img
    }
}
