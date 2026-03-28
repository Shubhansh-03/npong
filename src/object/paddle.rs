#[derive(Default)]
pub struct Paddle {
    pub x: i32,
    pub y: i32,
    pub height: u32,
    pub width: u32,
}

impl Paddle {
    pub fn left_shift(&mut self) {
        self.x -= 1;
    }
    pub fn right_shift(&mut self) {
        self.x += 1;
    }
}
