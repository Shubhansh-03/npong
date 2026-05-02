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
        // let vx: f32 = rand::random_range(-0.5..=0.5);
        // let p = rand::random_bool(0.5);
        // let vy: f32 = (0.25 - vx * vx).sqrt() * { if p { 1.0 } else { -1.0 } };
        let vx = 0.0;
        let vy = 0.0;
        Ball {
            radius: 15,
            x: (WIDTH / 2) as i32,
            y: (HEIGHT / 2) as i32,
            vx,
            vy,
            acceleration: 0.01,
        }
    }
    pub fn draw(&self, player: u8, frame: &mut [u8]) {
        let x = {
            if player == 1 {
                self.x
            } else {
                WIDTH as i32 - self.x
            }
        };
        let y = {
            if player == 1 {
                self.y
            } else {
                HEIGHT as i32 - self.y
            }
        };
        let r = self.radius as i32;
        let r2 = r * r;

        for dy in -r..=r {
            for dx in -r..=r {
                if dy * dy + dx * dx > r2 {
                    continue;
                }
                let px = x + dx;
                let py = y + dy;
                if px < 0 || py < 0 || px >= WIDTH as i32 || py >= HEIGHT as i32 {
                    continue;
                }

                let idx = ((py as u32 * WIDTH + px as u32) * 4) as usize;
                // frame[idx..idx + 4].copy_from_slice(&[255, 0, 255, 255]);
                frame[idx..idx + 4].copy_from_slice(&[
                    (((dx + dy) * 2) % 255 + 10) as u8,
                    ((dx * 2) % 255 + 20) as u8,
                    ((dy * 3) % 255 + 150) as u8,
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
