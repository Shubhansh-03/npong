use super::Input;
use super::gamestate::*;
use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

pub struct GameLoop {
    pub ticks: Duration,
    pub last_update: Instant,
}

impl GameLoop {
    pub fn game_loop(&mut self, state: Arc<RwLock<GameState>>, inputs: Arc<RwLock<Input>>) {
        // let timer = Instant::now();
        let mut updates = 0;
        'gameloop: loop {
            let start = Instant::now();
            {
                let mut gs = state.write().unwrap();
                let delta = Instant::now().duration_since(self.last_update);
                let delta = delta.as_millis();
                println!("{}", delta);
                {
                    let input_read_lock = inputs.read().unwrap();
                    gs.handle_input(&input_read_lock, delta);
                }
                if let Status::Running = gs.status {
                    gs.update(delta);
                    self.last_update = Instant::now();
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
