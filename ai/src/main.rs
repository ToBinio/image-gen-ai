use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::Write;

use crate::input::Input;
use crate::neural_network::NeuralNetwork;
use crate::output::Output;
use crate::simple_image::SimpleImage;
use image::error::UnsupportedErrorKind::Color;
use image::{DynamicImage, Rgba};

mod input;
mod neural_network;
mod output;
mod simple_image;

fn main() {
    let mut image = SimpleImage::new();

    image.set_rect(0..64, 0..64, Rgba([255, 255, 255, 255]));
    image.set_rect(0..20, 30..40, Rgba([0, 255, 0, 255]));

    let input = Input::new(image.get_image(), image.get_image());

    println!("{}", input.get_size());

    let mut output = Output {};

    let mut network =
        NeuralNetwork::new(&input, vec![10000, 20000, 10000, 5000, 2000, 100], &output);

    println!("created");

    network.set_input(&input);

    println!("setInput");

    network.compute();

    println!("computed");

    network.get_output(&mut output);

    println!("getOutput");

    let value = serde_json::to_string(&network).expect("could not serialize");


    println!("stringy");

    let mut file = File::create("test.json").expect("could not create file :(");
    file.write(value.as_bytes())
        .expect("could not write into file");
}
