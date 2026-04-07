use crate::{HEIGHT, WIDTH};

#[derive(Default)]
pub struct Ball {
    pub radius: u8,
    pub x: i32,
    pub y: i32,
    pub vx: f32,
    pub vy: f32,
    pub acceleration: f32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            radius: 15,
            x: (WIDTH / 2) as i32,
            y: (HEIGHT / 2) as i32,
            vx: 0.4,
            vy: 0.3,
            acceleration: 0.01,
        }
    }
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
                if px < 0 || py < 0 || px >= WIDTH as i32 || py >= HEIGHT as i32 {
                    continue;
                }

                let idx = ((py as u32 * WIDTH + px as u32) * 4) as usize;
                // frame[idx..idx + 4].copy_from_slice(&[255, 0, 255, 255]);
                frame[idx..idx + 4].copy_from_slice(&[
                    0,
                    ((idx) % 100 + 100) as u8,
                    ((idx) % 100 + 100) as u8,
                    255,
                ]);
            }
        }
    }

    pub fn update(&mut self, delta: u128) {
        let (mut tx, mut ty) = (self.x, self.y);
        tx += (self.vx * delta as f32).round() as i32;
        ty += (self.vy * delta as f32).round() as i32;

        self.x = tx;
        self.y = ty;
    }
}
//Abel was here
//omnomnom. ()o()thanks for last night
