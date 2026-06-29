use crate::{HEIGHT, WIDTH, coordinates::Coordinate};

#[derive(Default)]
pub struct Ball {
    pub radius: u8,
    pub position: Coordinate,
    pub vx: f32,
    pub vy: f32,
    pub acceleration: f32,
}

impl Ball {
    pub fn new() -> Self {
        let vx: f32 = rand::random_range(-0.5..=0.5);
        let p = rand::random_bool(0.5);
        let vy: f32 = (0.25 - vx * vx).sqrt() * { if p { 1.0 } else { -1.0 } };

        let position = Coordinate::from_cartesian((WIDTH / 2) as f32, (HEIGHT / 2) as f32);
        Ball {
            radius: 10,
            position,
            vx,
            vy,
            acceleration: 0.01,
        }
    }

    pub fn draw(&self, frame: &mut [u8], viewer_id: u8) {
        let r = self.radius as i32;
        let r2 = r * r;

        for dy in -r..=r {
            for dx in -r..=r {
                let d2 = dy * dy + dx * dx;
                if d2 > r2 {
                    continue;
                }

                let (x, y) = self.position.get_cartesian();

                let px = x as i32 + dx;
                let py = y as i32 + dy;

                if px < 0 || py < 0 || px >= WIDTH as i32 || py >= HEIGHT as i32 {
                    continue;
                }

                let (rx, ry) = if viewer_id == 2 {
                    (WIDTH as i32 - 1 - px, HEIGHT as i32 - 1 - py)
                } else {
                    (px, py)
                };

                // Calculate normalized distance from center (0.0 to 1.0)
                let dist = (d2 as f32).sqrt() / r as f32;

                let idx = ((ry as u32 * WIDTH + rx as u32) * 4) as usize;

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
        let (mut tx, mut ty) = self.position.get_cartesian();
        tx += self.vx * delta as f32;
        ty += self.vy * delta as f32;

        self.position = Coordinate::from_cartesian(tx, ty);
    }
}
