use super::super::{HEIGHT, WIDTH};
use crate::gamestate::GameState;

pub struct CollisionSystem;

impl CollisionSystem {
    // TODO: Make this human, I know I can. (Abhi bhi bug h thodu sa)
    pub fn collision(state: &mut GameState) {
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
        }
        if ball_n <= 10 {
            state.objects.ball.vy = -state.objects.ball.vy;
            state.objects.ball.y = 10 + ball_radius;
            collision = true;
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

        let paddle_data: Vec<(i32, i32, i32, i32, bool)> = state
            .objects
            .paddles
            .iter()
            .map(|p| {
                let top = p.y;
                let bot = p.y + p.height as i32;
                let left = p.x;
                let right = p.x + p.width as i32;
                let is_bottom = p.y > (HEIGHT / 2) as i32;
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
            state.objects.ball.update();
        }
    }
}
