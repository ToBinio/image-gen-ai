use std::fs;

use image::imageops::FilterType;
use image::DynamicImage;

fn main() {
    fs::read_dir("resources/raw")
        .unwrap()
        .filter(|file| file.is_ok())
        .map(|file| {
            (
                image::open(file.as_ref().unwrap().path()),
                file.unwrap().file_name(),
            )
        })
        .map(|img| (img.0.unwrap(), img.1))
        .map(|img| (resize(img.0), img.1))
        .for_each(|img| {
            img.0
                .save("resources/processed/".to_string() + img.1.to_str().unwrap())
                .unwrap();
            println!("processed2 {:?}", img.1);
        })
}

fn resize(image: DynamicImage) -> DynamicImage {
    image.resize_exact(64, 64, FilterType::Nearest)
}
