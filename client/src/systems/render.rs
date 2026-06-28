use pixels::Pixels;

use super::super::{HEIGHT, WIDTH};
use crate::state::gamestate::*;
use std::sync::Arc;
use std::sync::RwLock;

pub struct Render {}

impl Render {
    pub fn clear_screen(frame: &mut [u8], viewer_id: u8) {
        let (x, y, h, w) = (0, 0, crate::HEIGHT, crate::WIDTH);
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx as i32;
                let py = y + dy as i32;

                if px < 0 || py < 0 || px >= WIDTH as i32 || py >= HEIGHT as i32 {
                    continue;
                }
                
                let (rx, ry) = if viewer_id == 2 {
                    (WIDTH as i32 - 1 - px, HEIGHT as i32 - 1 - py)
                } else {
                    (px, py)
                };
                
                let idx = ((ry as u32 * WIDTH + rx as u32) * 4) as usize;

                frame[idx..idx + 4].copy_from_slice(&[
                    ((dx / 15) % 255) as u8,
                    ((dy / 15) % 255) as u8,
                    (((dx + dy) / 15) % 255) as u8,
                    255,
                ]);
            }
        }
    }

    pub fn draw_objects(state: &GameState, frame: &mut [u8]) {
        for (i, paddle) in state.objects.paddles.iter().enumerate() {
            let color = if i == 0 {
                [0, 255, 0, 255] // Green
            } else {
                [255, 0, 0, 255] // Red
            };
            paddle.draw(frame, state.player_id, color);
        }
        state.objects.ball.draw(frame, state.player_id);
        for wall in state.objects.walls.iter() {
            wall.draw(frame, state.player_id);
        }
    }

    pub fn draw(state: Arc<RwLock<GameState>>, pixels: &mut Pixels<'static>) {
        {
            let gs = state.read().unwrap();
            Self::clear_screen(pixels.frame_mut(), gs.player_id);
            Self::draw_objects(&gs, pixels.frame_mut());
        }
        if let Err(err) = pixels.render() {
            dbg!(err);
        }
    }
}
