use crate::{HEIGHT, WIDTH};

#[derive(Default)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub height: u32,
    pub width: u32,
}

impl Wall {
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

                frame[idx..idx + 4].copy_from_slice(&[200, 235, 255, 255]);
            }
        }
    }
}
