use pixels::Pixels;

use super::super::gamestate::*;
use super::super::{HEIGHT, WIDTH};
use std::sync::Arc;
use std::sync::RwLock;

pub struct Render {}

impl Render {
    pub fn clear_screen(frame: &mut [u8]) {
        let (x, y, h, w) = (0, 0, crate::HEIGHT, crate::WIDTH);
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx as i32;
                let py = y + dy as i32;

                if px < 0 || py < 0 || px >= WIDTH as i32 || py >= HEIGHT as i32 {
                    continue;
                }
                let idx = ((py as u32 * WIDTH + px as u32) * 4) as usize;

                frame[idx..idx + 4].copy_from_slice(&[
                    ((dx / 15) % 255) as u8,
                    ((dy / 15) % 255) as u8,
                    (((dx + dy) / 15) % 255) as u8,
                    255,
                ]);
                // frame[idx..idx + 4].copy_from_slice(&[(5) as u8, (25) as u8, (255) as u8, 255]);
            }
        }
    }

    pub fn draw_objects(state: &GameState, frame: &mut [u8]) {
        for paddle in state.objects.paddles.iter() {
            paddle.draw(frame);
        }
        state.objects.ball.draw(frame);
        for wall in state.objects.walls.iter() {
            wall.draw(frame);
        }
    }

    pub fn draw(state: Arc<RwLock<GameState>>, pixels: &mut Pixels<'static>) {
        {
            let gs = state.read().unwrap();
            Self::clear_screen(pixels.frame_mut());
            Self::draw_objects(&gs, pixels.frame_mut());
        }
        if let Err(err) = pixels.render() {
            dbg!(err);
        }
    }
}

