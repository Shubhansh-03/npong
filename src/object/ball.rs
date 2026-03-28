// use crate::HEIGHT;
use crate::WIDTH;

#[derive(Default)]
pub struct Ball {
    pub radius: u8,
    pub x: i32,
    pub y: i32,
    pub vx: f32,
    pub vy: f32,
}

impl Ball {
    pub fn draw(&self, frame: &mut [u8]) {
        let r = self.radius as i32;
        let r2 = r * r;

        for dy in -r..=r {
            for dx in -r..=r {
                if dy * dy + dx * dx > r2 {
                    continue;
                }
                let px = self.x + dx;
                let py = self.y + dy;

                let idx = ((py as u32 * WIDTH + px as u32) * 4) as usize;
                frame[idx..idx + 4].copy_from_slice(&[255, 0, 255, 255]);
            }
        }
    }
}
