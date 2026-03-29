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
                frame[idx..idx + 4].copy_from_slice(&[
                    ((dx / 10) % 255) as u8,
                    ((dy / 10) % 255) as u8,
                    (((dx + dy) / 10) % 255) as u8,
                    255,
                ]);
                // frame[idx..idx + 4].copy_from_slice(&[(5) as u8, (25) as u8, (255) as u8, 255]);
            }
        }
    }

    pub fn update(&mut self) {
        self.ball.update();

        self.check_collision();
    }

    // Temporary collision code. Have to implement a better collision checker for the future
    pub fn check_collision(&mut self) {
        let mut collision = false;
        let ballx = self.ball.x;
        let bally = self.ball.y;
        let ballradius = self.ball.radius as i32;
        let balls = bally + ballradius;
        let balln = bally - ballradius;
        let balle = ballx + ballradius;
        let ballw = ballx - ballradius;

        let paddle1 = self.paddles.get(0).unwrap().y;
        let paddle1left = self.paddles.get(0).unwrap().x;
        let paddle1right =
            self.paddles.get(0).unwrap().x + self.paddles.get(0).unwrap().width as i32;
        let paddle2 = self.paddles.get(1).unwrap();
        let paddle2 = paddle2.y + paddle2.height as i32;
        let paddle2left = self.paddles.get(1).unwrap().x;
        let paddle2right =
            self.paddles.get(1).unwrap().x + self.paddles.get(1).unwrap().width as i32;

        // Boundary collisions
        if balls >= HEIGHT as i32 {
            self.ball.vy = -self.ball.vy;
            collision = true;
        }
        if (balln) <= 0 {
            self.ball.vy = -self.ball.vy;
            collision = true;
        }
        if balle >= WIDTH as i32 {
            self.ball.vx = -self.ball.vx;
            collision = true;
        }
        if (ballw) <= 0 {
            self.ball.vx = -self.ball.vx;
            collision = true;
        }

        if balls >= paddle1 && (ballx <= paddle1right && ballx >= paddle1left) {
            self.ball.vy = -self.ball.vy;
            collision = true;
        }

        if balln <= paddle2 && (ballx <= paddle2right && ballx >= paddle2left) {
            self.ball.vy = -self.ball.vy;
            collision = true;
        }

        if collision {
            self.ball.update();
        }
    }
}
