# nPong

My first big project in Rust. Trying to maximize learning through this. (Features minimal use of AI and maximal dying inside)

## Features (Currently)

- Game initialization
- Paddle movement
- Ball rendering and collisions
- Separating game updates from rendering (separating game time)
- Both paddle work at the same time
- Acceleration for the paddle
- Game updates use Delta time
- Paddle wall collisions

## Up next

- Ball bouncing randomizing
- Ball wall collisions reset game
- Sound effects
- Frame updates optimization (caching mechanism)
- Custom png rendering pipeline

## Reading Materials

- Rust auto generated documentation
- [Pixels GitHub Page](https://github.com/parasyte/pixels)

### Bugs (potentially)

- Paddle movement should also cause ball collision triggers
