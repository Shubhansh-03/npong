use super::object::ball::Ball;
use super::object::paddle::Paddle;
use super::object::wall::Wall;
use super::{HEIGHT, WIDTH};
use std::collections::HashSet;
use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct GameState {
    pub players: u8,
    pub ball: Ball,
    pub paddles: [Paddle; 2],
    pub walls: [Wall; 4],
    pub input: HashSet<KeyCode>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: 2,
            paddles: [
                Paddle {
                    x: ((WIDTH - WIDTH / 15) / 2) as i32,
                    y: (HEIGHT - HEIGHT / 30) as i32,
                    height: HEIGHT / 60,
                    width: WIDTH / 15,
                    acceleration_left: 0.0,
                    acceleration_right: 0.0,
                },
                Paddle {
                    x: ((WIDTH - WIDTH / 15) / 2) as i32,
                    y: (HEIGHT / 60) as i32,
                    height: HEIGHT / 60,
                    width: WIDTH / 15,
                    acceleration_left: 0.0,
                    acceleration_right: 0.0,
                },
            ],
            ball: Ball {
                radius: 15,
                x: (WIDTH / 2) as i32,
                y: (HEIGHT / 2) as i32,
                vx: 4.0,
                vy: 3.0,
            },
            walls: [
                Wall {
                    x: 0,
                    y: 0,
                    height: HEIGHT,
                    width: 10,
                },
                Wall {
                    x: (WIDTH - 10) as i32,
                    y: 0,
                    height: HEIGHT,
                    width: 10,
                },
                Wall {
                    x: 0,
                    y: 0,
                    height: 10,
                    width: WIDTH,
                },
                Wall {
                    x: 0,
                    y: (HEIGHT - 10) as i32,
                    height: 10,
                    width: WIDTH,
                },
            ],
            input: HashSet::new(),
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for paddle in self.paddles.iter() {
            paddle.draw(frame);
        }
        self.ball.draw(frame);

        self.walls[0].draw(frame);
        self.walls[1].draw(frame);
        self.walls[2].draw(frame);
        self.walls[3].draw(frame);
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

    pub fn update(&mut self) {
        self.handle_input();
        self.ball.update();

        self.collision();
    }

    fn handle_input(&mut self) {
        let mut paddle1_movement = false;
        let mut paddle2_movement = false;
        if self.input.contains(&KeyCode::KeyA) {
            self.paddles[0].left_shift();
            paddle1_movement = true;
        }
        if self.input.contains(&KeyCode::KeyD) {
            self.paddles[0].right_shift();
            paddle1_movement = true;
        }
        if self.input.contains(&KeyCode::ArrowLeft) {
            self.paddles[1].left_shift();
            paddle2_movement = true;
        }
        if self.input.contains(&KeyCode::ArrowRight) {
            self.paddles[1].right_shift();
            paddle2_movement = true;
        }

        if !paddle1_movement {
            self.paddles[0].acceleration_left = 0.0;
        }
        if !paddle2_movement {
            self.paddles[1].acceleration_left = 0.0;
        }
    }

    // Temporary collision code. Have to implement a better collision checker for the future
    pub fn collision(&mut self) {
        let mut collision = false;
        let ball_x = self.ball.x;
        let ball_y = self.ball.y;
        let ball_radius = self.ball.radius as i32;
        let ball_s = ball_y + ball_radius;
        let ball_n = ball_y - ball_radius;
        let ball_e = ball_x + ball_radius;
        let ball_w = ball_x - ball_radius;

        let paddle1_top = self.paddles[0].y;
        let paddle1_bot = self.paddles[0].y + self.paddles[0].height as i32;
        let paddle1_left = self.paddles[0].x;
        let paddle1_right = self.paddles[0].x + self.paddles[0].width as i32;

        let paddle2_top = self.paddles[1].y;
        let paddle2_bot = self.paddles[1].y + self.paddles[1].height as i32;
        let paddle2_left = self.paddles[1].x;
        let paddle2_right = self.paddles[1].x + self.paddles[1].width as i32;

        if ball_s >= (HEIGHT - 10) as i32 {
            self.ball.vy = -self.ball.vy.abs();
            self.ball.y = (HEIGHT - 10) as i32 - ball_radius;
            collision = true;
        }
        if ball_n <= 10 {
            self.ball.vy = -self.ball.vy;
            self.ball.y = 10 + ball_radius;
            collision = true;
        }
        if ball_e >= (WIDTH - 10) as i32 {
            self.ball.vx = -self.ball.vx;
            self.ball.x = (WIDTH - 10) as i32 - ball_radius;
            collision = true;
        }
        if ball_w <= 10 {
            self.ball.vx = -self.ball.vx;
            self.ball.x = 10 + ball_radius;
            collision = true;
        }

        let in_x_range_1 = ball_e >= paddle1_left && ball_w <= paddle1_right;
        let in_y_range_1 = ball_s >= paddle1_top && ball_n <= paddle1_bot;

        if in_x_range_1 && in_y_range_1 {
            let overlap_top = ball_s - paddle1_top;
            let overlap_left = ball_e - paddle1_left;
            let overlap_right = paddle1_right - ball_w;

            let min_overlap = overlap_top.min(overlap_left).min(overlap_right);

            if min_overlap == overlap_top {
                self.ball.vy = -self.ball.vy;
                self.ball.y = paddle1_top - ball_radius;
            } else if min_overlap == overlap_left {
                self.ball.vx = -self.ball.vx;
                self.ball.x = paddle1_left - ball_radius;
            } else {
                self.ball.vx = -self.ball.vx;
                self.ball.x = paddle1_right + ball_radius;
            }
            collision = true;
        } else {
            let corners = [(paddle1_left, paddle1_top), (paddle1_right, paddle1_top)];
            for (cx, cy) in corners {
                let dist_sq = (ball_x - cx).pow(2) + (ball_y - cy).pow(2);
                if dist_sq <= ball_radius.pow(2) {
                    let old_vx = self.ball.vx;
                    let old_vy = self.ball.vy;
                    self.ball.vx = -old_vy;
                    self.ball.vy = -old_vx;
                    collision = true;
                    break;
                }
            }
        }

        let in_x_range_2 = ball_e >= paddle2_left && ball_w <= paddle2_right;
        let in_y_range_2 = ball_s >= paddle2_top && ball_n <= paddle2_bot;

        if in_x_range_2 && in_y_range_2 {
            let overlap_bot = paddle2_bot - ball_n;
            let overlap_left = ball_e - paddle2_left;
            let overlap_right = paddle2_right - ball_w;

            let min_overlap = overlap_bot.min(overlap_left).min(overlap_right);

            if min_overlap == overlap_bot {
                self.ball.vy = -self.ball.vy;
                self.ball.y = paddle2_bot + ball_radius;
            } else if min_overlap == overlap_left {
                self.ball.vx = -self.ball.vx;
                self.ball.x = paddle2_left - ball_radius;
            } else {
                self.ball.vx = -self.ball.vx;
                self.ball.x = paddle2_right + ball_radius;
            }
            collision = true;
        } else {
            let corners = [(paddle2_left, paddle2_bot), (paddle2_right, paddle2_bot)];
            for (cx, cy) in corners {
                let dist_sq = (ball_x - cx).pow(2) + (ball_y - cy).pow(2);
                if dist_sq <= ball_radius.pow(2) {
                    let old_vx = self.ball.vx;
                    let old_vy = self.ball.vy;
                    self.ball.vx = -old_vy;
                    self.ball.vy = -old_vx;
                    collision = true;
                    break;
                }
            }
        }

        if collision {
            self.ball.update();
        }
    }
}
