use crate::{HEIGHT, WIDTH};

#[derive(Default)]
pub struct Paddle {
    pub x: i32,
    pub y: i32,
    pub height: u32,
    pub width: u32,
    pub acceleration_left: f32,
    pub acceleration_right: f32,
}

impl Paddle {
    pub fn left_shift(&mut self) {
        self.acceleration_right = 0.0;
        self.x -= 3 + self.acceleration_left as i32;
        self.acceleration_left += 0.1;
    }
    pub fn right_shift(&mut self) {
        self.acceleration_left = 0.0;
        self.x += 3 + self.acceleration_right as i32;
        self.acceleration_right += 0.1;
    }
    pub fn draw(&self, frame: &mut [u8]) {
        let (x, y, h, w) = (self.x, self.y, self.height, self.width);
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx as i32;
                let py = y + dy as i32;

                if px < 0 || py < 0 || px >= WIDTH as i32 || py >= HEIGHT as i32 {
                    continue;
                }
                let idx = ((py as u32 * WIDTH + px as u32) * 4) as usize;

                frame[idx..idx + 4].copy_from_slice(&[255, 255, 255, 255]);
            }
        }
    }
}
