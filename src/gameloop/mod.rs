use super::Input;
use super::gamestate::*;
use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

pub struct GameLoop {
    pub ticks: Duration,
}

impl GameLoop {
    pub fn game_loop(self, state: Arc<RwLock<GameState>>, inputs: Arc<RwLock<Input>>) {
        let mut updates = 0;
        'gameloop: loop {
            let start = Instant::now();
            {
                let mut gs = state.write().unwrap();
                {
                    let input_read_lock = inputs.read().unwrap();
                    gs.handle_input(&input_read_lock);
                }
                if let Status::Running = gs.status {
                    gs.update();
                }
                if let Status::Exit = gs.status {
                    break 'gameloop;
                }
            }

            updates += 1;
            let elapsed = start.elapsed();
            if elapsed < self.ticks {
                thread::sleep(self.ticks - elapsed);
            }
        }
        println!("Updates: {}", updates);
    }
}
