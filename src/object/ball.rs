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
        let vx: f32 = rand::random_range(-0.5..=0.5);
        let p = rand::random_bool(0.5);
        let vy: f32 = (0.25 - vx * vx).sqrt() * { if p { 1.0 } else { -1.0 } };
        Ball {
            radius: 10,
            x: (WIDTH / 2) as i32,
            y: (HEIGHT / 2) as i32,
            vx,
            vy,
            acceleration: 0.01,
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        let r = self.radius as i32;
        let r2 = r * r;

        for dy in -r..=r {
            for dx in -r..=r {
                let d2 = dy * dy + dx * dx;
                if d2 > r2 {
                    continue;
                }

                let px = self.x + dx;
                let py = self.y + dy;

                if px < 0 || py < 0 || px >= WIDTH as i32 || py >= HEIGHT as i32 {
                    continue;
                }

                // Calculate normalized distance from center (0.0 to 1.0)
                let dist = (d2 as f32).sqrt() / r as f32;

                let idx = ((py as u32 * WIDTH + px as u32) * 4) as usize;

                // Pattern: Bright center that fades to a neon edge with a "zebra" ring
                let c1 = (255.0 * (1.0 - dist)) as u8; // White core
                let c2 = (dist * 200.0) as u8; // Outer glow
                let ring = (((dist * 10.0).sin() * 127.0) + 128.0) as u8;

                frame[idx..idx + 4].copy_from_slice(&[
                    c1.saturating_add(ring / 2), // R
                    255,                         // B (Solid blue base)
                    c2.saturating_add(ring / 4), // G
                    255,                         // A
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
