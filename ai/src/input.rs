use image::{DynamicImage, GenericImageView};
use rand::{thread_rng, Rng};

pub struct Input {
    data: Vec<f32>,
}

impl Input {
    pub fn new(goal_img: &DynamicImage, current_img: &DynamicImage) -> Input {
        let mut data = vec![];

        Input::push_img(&mut data, goal_img);
        Input::push_img(&mut data, current_img);

        Input { data }
    }

    fn push_img(data: &mut Vec<f32>, img: &DynamicImage) {
        for x in 0..img.width() {
            for y in 0..img.height() {
                let rgba = img.get_pixel(x, y);

                let rgba_u32: u32 = ((rgba.0[0] as u32) << 24)
                    | ((rgba.0[1] as u32) << 16)
                    | ((rgba.0[2] as u32) << 8)
                    | rgba.0[3] as u32;

                data.push(rgba_u32 as f32);
            }
        }
    }
    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self) -> Vec<f32> {
        let mut values = vec![];

        for _ in 0..self.get_size() {
            values.push(thread_rng().gen_range(-1.0..1.0));
        }

        values
    }
}
