use super::object::Object;
use super::{HEIGHT, WIDTH};

#[derive(Default)]
pub struct GameState {
    pub players: u8,
    pub objects: Vec<Object>,
}

impl GameState {
    pub fn draw(&self, frame: &mut [u8]) {
        for object in self.objects.iter() {
            match object {
                Object::Paddle(paddle) => {
                    let (x, y, h, w) = (paddle.x, paddle.y, paddle.height, paddle.width);
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
                _ => {}
            }
        }
    }
}
