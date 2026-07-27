use crate::input::GameInput;
use crate::object::{Objects, ball::*, paddle::*, wall::*};

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    #[default]
    Paused,
    Running,
    Reset,
    Exit,
}

#[derive(Default)]
pub struct GameState {
    pub status: Status,
    pub player_id: u8,
    pub objects: Objects,
}

impl GameState {
    pub fn new(id: u8) -> Self {
        GameState {
            status: Status::Paused,
            player_id: id,
            objects: Objects {
                paddles: Paddle::new(2),
                ball: Ball::new(),
                walls: Wall::new(2),
            },
        }
    }

    pub fn update(&mut self, delta: u128) {
        if self.status == Status::Running {
            self.objects.ball.update(delta);
            self.collision(delta);
        }
    }

    pub fn handle_input(&mut self, input: &GameInput, delta: u128) {
        if let Status::Running = self.status {
            let mut paddle1_movement = false;
            let mut paddle2_movement = false;

            // Offline (singleplayer) mode
            if self.player_id == 0 {
                if input.p1_left_pressed {
                    self.objects.paddles[0].left_shift(delta);
                    paddle1_movement = true;
                }
                if input.p1_right_pressed {
                    self.objects.paddles[0].right_shift(delta);
                    paddle1_movement = true;
                }
                if input.p2_left_pressed {
                    self.objects.paddles[1].left_shift(delta);
                    paddle2_movement = true;
                }
                if input.p2_right_pressed {
                    self.objects.paddles[1].right_shift(delta);
                    paddle2_movement = true;
                }
            } else {
                // Multiplayer mode
                let local_idx = (self.player_id - 1) as usize;
                let mut local_movement = false;

                // Invert movement according to which paddle it is
                if local_idx == 0 {
                    if input.p1_left_pressed {
                        self.objects.paddles[local_idx].left_shift(delta);
                        local_movement = true;
                    }
                    if input.p1_right_pressed {
                        self.objects.paddles[local_idx].right_shift(delta);
                        local_movement = true;
                    }
                } else {
                    if input.p1_left_pressed {
                        self.objects.paddles[local_idx].right_shift(delta);
                        local_movement = true;
                    }
                    if input.p1_right_pressed {
                        self.objects.paddles[local_idx].left_shift(delta);
                        local_movement = true;
                    }
                }

                if local_idx == 0 {
                    paddle1_movement = local_movement;
                } else {
                    paddle2_movement = local_movement;
                }
            }

            if !paddle1_movement {
                self.objects.paddles[0].acceleration = 0.0;
            }
            if !paddle2_movement {
                self.objects.paddles[1].acceleration = 0.0;
            }

            if input.pause_toggled {
                self.status = Status::Paused;
            } else {
                self.status = Status::Running;
            }

            if input.quit_game {
                self.status = Status::Exit;
            }
        }
    }

    // Function to handle inputs even when game is paused
    pub fn check_paused(&mut self, input: &GameInput) {
        if input.pause_toggled {
            self.status = Status::Paused;
        } else {
            // FIXME: Instead of Running it should be what the state was before setting it to Paused (I don't thing I'm ever fixing this but in case I do in the future I deserve a sweet treat for that)
            self.status = Status::Running;
        }
    }

    pub fn reset(&mut self) {
        self.objects.paddles = Paddle::new(2);
        self.objects.ball = Ball::new();
        self.objects.walls = Wall::new(2);
        self.status = Status::Reset;
    }

    pub fn collision(&mut self, delta: u128) {
        let mut collision = false;
        let (ball_x_f, ball_y_f) = self.objects.ball.position.get_cartesian();
        let ball_x = ball_x_f as i32;
        let ball_y = ball_y_f as i32;
        let ball_radius = self.objects.ball.radius as i32;
        let ball_s = ball_y + ball_radius;
        let ball_n = ball_y - ball_radius;
        let ball_e = ball_x + ball_radius;
        let ball_w = ball_x - ball_radius;

        if ball_s >= (crate::HEIGHT - 10) as i32 {
            self.objects.ball.vy = -self.objects.ball.vy.abs();
            self.objects.ball.position = crate::coordinates::Coordinate::from_cartesian(
                ball_x_f,
                ((crate::HEIGHT - 10) as i32 - ball_radius) as f32,
            );
            collision = true;
            // self.reset();
            // return;
        }
        if ball_n <= 10 {
            self.objects.ball.vy = -self.objects.ball.vy;
            self.objects.ball.position =
                crate::coordinates::Coordinate::from_cartesian(ball_x_f, (10 + ball_radius) as f32);
            collision = true;
            // self.reset();
            // return;
        }
        if ball_e >= (crate::WIDTH - 10) as i32 {
            self.objects.ball.vx = -self.objects.ball.vx;
            self.objects.ball.position = crate::coordinates::Coordinate::from_cartesian(
                ((crate::WIDTH - 10) as i32 - ball_radius) as f32,
                ball_y_f,
            );
            collision = true;
        }
        if ball_w <= 10 {
            self.objects.ball.vx = -self.objects.ball.vx;
            self.objects.ball.position =
                crate::coordinates::Coordinate::from_cartesian((10 + ball_radius) as f32, ball_y_f);
            collision = true;
        }

        for paddle in self.objects.paddles.iter_mut() {
            let (px, py) = paddle.position.get_cartesian();
            let mut left = px as i32;
            let right = left + paddle.width as i32;

            if left <= 10 {
                left = 10;
            }
            if right >= (crate::WIDTH - 10) as i32 {
                left = (crate::WIDTH - 10 - paddle.width) as i32;
            }
            paddle.position = crate::coordinates::Coordinate::from_cartesian(left as f32, py);
        }

        let paddle_data: Vec<(i32, i32, i32, i32, bool)> = self
            .objects
            .paddles
            .iter()
            .map(|p| {
                let (px, py) = p.position.get_cartesian();
                let top = py as i32;
                let bot = top + p.height as i32;
                let left = px as i32;
                let right = left + p.width as i32;
                let is_bottom = top > (crate::HEIGHT / 2) as i32;
                (top, bot, left, right, is_bottom)
            })
            .collect();

        for (paddle_top, paddle_bot, paddle_left, paddle_right, is_bottom) in paddle_data.iter() {
            let (paddle_top, paddle_bot, paddle_left, paddle_right) =
                (*paddle_top, *paddle_bot, *paddle_left, *paddle_right);

            let in_x_range = ball_e >= paddle_left && ball_w <= paddle_right;
            let in_y_range = ball_s >= paddle_top && ball_n <= paddle_bot;

            if in_x_range && in_y_range {
                let overlap_face = if *is_bottom {
                    ball_s - paddle_top
                } else {
                    paddle_bot - ball_n
                };
                let overlap_left = ball_e - paddle_left;
                let overlap_right = paddle_right - ball_w;
                let min_overlap = overlap_face.min(overlap_left).min(overlap_right);

                let (mut curr_x, mut curr_y) = self.objects.ball.position.get_cartesian();

                if min_overlap == overlap_face {
                    self.objects.ball.vy = -self.objects.ball.vy;
                    if *is_bottom {
                        curr_y = (paddle_top - ball_radius) as f32;
                    } else {
                        curr_y = (paddle_bot + ball_radius) as f32;
                    }
                } else if min_overlap == overlap_left {
                    self.objects.ball.vx = -self.objects.ball.vx;
                    curr_x = (paddle_left - ball_radius) as f32;
                } else {
                    self.objects.ball.vx = -self.objects.ball.vx;
                    curr_x = (paddle_right + ball_radius) as f32;
                }
                self.objects.ball.position =
                    crate::coordinates::Coordinate::from_cartesian(curr_x, curr_y);
                collision = true;
            } else {
                let corners = if *is_bottom {
                    [(paddle_left, paddle_top), (paddle_right, paddle_top)]
                } else {
                    [(paddle_left, paddle_bot), (paddle_right, paddle_bot)]
                };
                for (cx, cy) in corners {
                    let dist_sq = (ball_x - cx).pow(2) + (ball_y - cy).pow(2);
                    if dist_sq <= ball_radius.pow(2) {
                        let old_vx = self.objects.ball.vx;
                        let old_vy = self.objects.ball.vy;
                        self.objects.ball.vx = -old_vy;
                        self.objects.ball.vy = -old_vx;
                        collision = true;
                        break;
                    }
                }
            }
        }

        if collision {
            self.objects.ball.update(delta);
            let accn = self.objects.ball.acceleration;
            if self.objects.ball.vx != 0.0 {
                self.objects.ball.vx = self.objects.ball.vx
                    + accn * (self.objects.ball.vx / (self.objects.ball.vx.abs()));
            }
            if self.objects.ball.vy != 0.0 {
                self.objects.ball.vy = self.objects.ball.vy
                    + accn * (self.objects.ball.vy / (self.objects.ball.vy.abs()));
            }
        }
    }
}
