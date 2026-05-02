use crate::{HEIGHT, WIDTH};

#[derive(Default)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub height: u32,
    pub width: u32,
    pub critical: bool,
}

impl Wall {
    pub fn draw(&self, frame: &mut [u8], _player: u8) {
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
    pub fn new(players: u8) -> [Wall; 4] {
        if players == 2 {
            [
                Wall {
                    x: 0,
                    y: (HEIGHT - 10) as i32,
                    height: 10,
                    width: WIDTH,
                    critical: true,
                },
                Wall {
                    x: (WIDTH - 10) as i32,
                    y: 0,
                    height: HEIGHT,
                    width: 10,
                    critical: false,
                },
                Wall {
                    x: 0,
                    y: 0,
                    height: 10,
                    width: WIDTH,
                    critical: true,
                },
                Wall {
                    x: 0,
                    y: 0,
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
