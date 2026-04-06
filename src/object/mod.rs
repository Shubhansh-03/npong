// TODO: I do not really know why this code is here but its vibing for now
pub mod ball;
pub mod paddle;
pub mod wall;

#[derive(Default)]
pub struct Objects {
    pub ball: ball::Ball,
    pub paddles: Vec<paddle::Paddle>,
    pub walls: [wall::Wall; 4],
}
