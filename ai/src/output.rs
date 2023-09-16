pub struct Output {}

impl Output {
    pub fn get_size(&self) -> usize {
        8
    }

    pub fn set(&self, values: Vec<f32>) {
        for value in values {
            println!("{}", value);
        }
    }
}
