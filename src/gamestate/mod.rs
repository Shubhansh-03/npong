use super::object::ball::Ball;
use super::object::paddle::Paddle;
use super::object::wall::Wall;
use super::{HEIGHT, WIDTH};

#[derive(Default)]
pub struct GameState {
    pub players: u8,
    pub ball: Ball,
    pub paddles: [Paddle; 2],
    pub walls: [Wall; 4],
}

impl GameState {
    pub fn draw(&self, frame: &mut [u8]) {
        for paddle in self.paddles.iter() {
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
        self.ball.draw(frame);
    }

    // Wrote code to clear frame by myself. (Could not find the library method for it TT )
    pub fn clear_screen(&self, frame: &mut [u8]) {
        let (x, y, h, w) = (0, 0, crate::HEIGHT, crate::WIDTH);
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx as i32;
                let py = y + dy as i32;

                if px < 0 || py < 0 || px >= WIDTH as i32 || py >= HEIGHT as i32 {
                    continue;
                }
                let idx = ((py as u32 * WIDTH + px as u32) * 4) as usize;

                // Funky color scheme in the frame
                // frame[idx..idx + 4].copy_from_slice(&[
                //     (dx % 255) as u8,
                //     (dy % 255) as u8,
                //     ((dx + dy) % 255) as u8,
                //     255,
                // ]);
                frame[idx..idx + 4].copy_from_slice(&[(5) as u8, (25) as u8, (255) as u8, 255]);
            }
        }
    }
}
