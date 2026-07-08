use crate::{HEIGHT, WIDTH, coordinates::Coordinate};

#[derive(Default)]
pub struct Paddle {
    pub position: Coordinate,
    pub height: u32,
    pub width: u32,
    pub acceleration: f32,
}

impl Paddle {
    pub fn new(players: u8) -> Vec<Paddle> {
        if players == 2 {
            vec![
                Paddle {
                    position: Coordinate::from_cartesian(
                        ((WIDTH - WIDTH / 15) / 2) as f32,
                        (HEIGHT - HEIGHT / 30) as f32,
                    ),
                    height: HEIGHT / 60,
                    width: WIDTH / 15,
                    acceleration: 0.0,
                },
                Paddle {
                    position: Coordinate::from_cartesian(
                        ((WIDTH - WIDTH / 15) / 2) as f32,
                        (HEIGHT / 60) as f32,
                    ),
                    height: HEIGHT / 60,
                    width: WIDTH / 15,
                    acceleration: 0.0,
                },
            ]
        } else {
            todo!();
        }
    }
    pub fn left_shift(&mut self, delta: u128) {
        if self.acceleration > 0.0 {
            self.acceleration = 0.0;
        }
        let (x, y) = self.position.get_cartesian();
        let shift = (0.3 - self.acceleration) * delta as f32;
        let new_x = (x - shift).max(0.0);
        self.position = Coordinate::from_cartesian(new_x, y);
        self.acceleration -= 0.01;
    }
    pub fn right_shift(&mut self, delta: u128) {
        if self.acceleration < 0.0 {
            self.acceleration = 0.0;
        }
        let (x, y) = self.position.get_cartesian();
        let shift = (0.3 + self.acceleration) * delta as f32;
        let max_x = (WIDTH as i32 - self.width as i32).max(0) as f32;
        let new_x = (x + shift).max(0.0).min(max_x);
        self.position = Coordinate::from_cartesian(new_x, y);
        self.acceleration += 0.01;
    }
    pub fn draw(&self, frame: &mut [u8], viewer_id: u8, color: [u8; 4]) {
        let (x, y) = self.position.get_cartesian();
        let (h, w) = (self.height, self.width);
        for dy in 0..h {
            for dx in 0..w {
                let px = x as i32 + dx as i32;
                let py = y as i32 + dy as i32;

                if px < 0 || py < 0 || px >= WIDTH as i32 || py >= HEIGHT as i32 {
                    continue;
                }

                let (rx, ry) = if viewer_id == 2 {
                    (WIDTH as i32 - 1 - px, HEIGHT as i32 - 1 - py)
                } else {
                    (px, py)
                };

                let idx = ((ry as u32 * WIDTH + rx as u32) * 4) as usize;

                frame[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }
}
