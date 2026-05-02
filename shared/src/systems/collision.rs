use super::super::{HEIGHT, WIDTH};
use crate::gamestate::GameState;
use crate::object::paddle::*;

pub struct CollisionSystem;

impl CollisionSystem {
    // TODO: Make this human, I know I can. (Abhi bhi bug h thodu sa)
    pub fn collision(state: &mut GameState, delta: u128) {
        let mut collision = false;
        let ball_x = state.objects.ball.x;
        let ball_y = state.objects.ball.y;
        let ball_radius = state.objects.ball.radius as i32;
        let ball_s = ball_y + ball_radius;
        let ball_n = ball_y - ball_radius;
        let ball_e = ball_x + ball_radius;
        let ball_w = ball_x - ball_radius;

        if ball_s >= (HEIGHT - 10) as i32 {
            state.objects.ball.vy = -state.objects.ball.vy.abs();
            state.objects.ball.y = (HEIGHT - 10) as i32 - ball_radius;
            collision = true;
            state.reset();
        }
        if ball_n <= 10 {
            state.objects.ball.vy = -state.objects.ball.vy;
            state.objects.ball.y = 10 + ball_radius;
            collision = true;
            state.reset();
        }
        if ball_e >= (WIDTH - 10) as i32 {
            state.objects.ball.vx = -state.objects.ball.vx;
            state.objects.ball.x = (WIDTH - 10) as i32 - ball_radius;
            collision = true;
        }
        if ball_w <= 10 {
            state.objects.ball.vx = -state.objects.ball.vx;
            state.objects.ball.x = 10 + ball_radius;
            collision = true;
        }

        for paddle in state.objects.paddles.iter_mut().enumerate() {
            CollisionSystem::paddle_wall_collion(paddle.1, paddle.0 as u8);
        }

        let paddle_data: Vec<(i32, i32, i32, i32, bool)> = state
            .objects
            .paddles
            .iter()
            .enumerate()
            .map(|(idx, p)| {
                let (x, y) = p.global_coordinates(idx as u8 + 1);
                let top = y + (p.height / 2) as i32;
                let bot = y - (p.height / 2) as i32;
                let left = x - (p.width / 2) as i32;
                let right = x + (p.width / 2) as i32;
                let is_bottom = y > (HEIGHT / 2) as i32;
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

                if min_overlap == overlap_face {
                    state.objects.ball.vy = -state.objects.ball.vy;
                    if *is_bottom {
                        state.objects.ball.y = paddle_top - ball_radius;
                    } else {
                        state.objects.ball.y = paddle_bot + ball_radius;
                    }
                } else if min_overlap == overlap_left {
                    state.objects.ball.vx = -state.objects.ball.vx;
                    state.objects.ball.x = paddle_left - ball_radius;
                } else {
                    state.objects.ball.vx = -state.objects.ball.vx;
                    state.objects.ball.x = paddle_right + ball_radius;
                }
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
                        let old_vx = state.objects.ball.vx;
                        let old_vy = state.objects.ball.vy;
                        state.objects.ball.vx = -old_vy;
                        state.objects.ball.vy = -old_vx;
                        collision = true;
                        break;
                    }
                }
            }
        }

        if collision {
            state.objects.ball.update(delta);
            let accn = state.objects.ball.acceleration;
            state.objects.ball.vx = state.objects.ball.vx
                + accn * (state.objects.ball.vx / (state.objects.ball.vx.abs()));
            state.objects.ball.vy = state.objects.ball.vy
                + accn * (state.objects.ball.vy / (state.objects.ball.vy.abs()));
        }
    }

    pub fn paddle_wall_collion(paddle: &mut Paddle, id: u8) {
        let (x, _y) = paddle.global_coordinates(id);
        let left = x - (paddle.width / 2) as i32;
        let right = x + (paddle.width / 2) as i32;

        if left <= 10 {
            paddle.shift = -((WIDTH / 2) as i32) + 10 + (paddle.width / 2) as i32;
        }
        if right >= (WIDTH - 10) as i32 {
            paddle.shift = ((WIDTH / 2) as i32) - 10 - (paddle.width / 2) as i32;
        }
    }
}
