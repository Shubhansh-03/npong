pub mod ball;
pub mod paddle;
pub mod wall;

pub enum Object {
    Ball(ball::Ball),
    Paddle(paddle::Paddle),
    Wall(wall::Wall),
}
