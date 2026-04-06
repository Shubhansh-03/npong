use crate::{HEIGHT, WIDTH};

#[derive(Default)]
pub struct Paddle {
    pub x: i32,
    pub y: i32,
    pub height: u32,
    pub width: u32,
    pub acceleration: f32,
}

impl Paddle {
    pub fn new(players: u8) -> Vec<Paddle> {
        if players == 2 {
            vec![
                Paddle {
                    x: ((WIDTH - WIDTH / 15) / 2) as i32,
                    y: (HEIGHT - HEIGHT / 30) as i32,
                    height: HEIGHT / 60,
                    width: WIDTH / 15,
                    acceleration: 0.0,
                },
                Paddle {
                    x: ((WIDTH - WIDTH / 15) / 2) as i32,
                    y: (HEIGHT / 60) as i32,
                    height: HEIGHT / 60,
                    width: WIDTH / 15,
                    acceleration: 0.0,
                },
            ]
        } else {
            todo!();
        }
    }
    pub fn left_shift(&mut self) {
        self.x -= 3 - self.acceleration as i32;
        self.acceleration -= 0.1;
    }
    pub fn right_shift(&mut self) {
        self.x += 3 + self.acceleration as i32;
        self.acceleration += 0.1;
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
