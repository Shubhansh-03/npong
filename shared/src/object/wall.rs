use crate::{HEIGHT, WIDTH, coordinates::Coordinate};

#[derive(Default)]
pub struct Wall {
    pub position: Coordinate,
    pub height: u32,
    pub width: u32,
    pub critical: bool,
}

impl Wall {
    pub fn draw(&self, frame: &mut [u8], viewer_id: u8) {
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

                frame[idx..idx + 4].copy_from_slice(&[200, 235, 255, 255]);
            }
        }
    }
    pub fn new(players: u8) -> [Wall; 4] {
        if players == 2 {
            [
                Wall {
                    position: Coordinate::from_cartesian(0.0, (HEIGHT - 10) as f32),
                    height: 10,
                    width: WIDTH,
                    critical: false,
                },
                Wall {
                    position: Coordinate::from_cartesian((WIDTH - 10) as f32, 0.0),
                    height: HEIGHT,
                    width: 10,
                    critical: false,
                },
                Wall {
                    position: Coordinate::from_cartesian(0.0, 0.0),
                    height: 10,
                    width: WIDTH,
                    critical: false,
                },
                Wall {
                    position: Coordinate::from_cartesian(0.0, 0.0),
                    height: HEIGHT,
                    width: 10,
                    critical: false,
                },
            ]
        } else {
            todo!()
        }
    }
}
