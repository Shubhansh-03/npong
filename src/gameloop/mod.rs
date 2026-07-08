use pixels::wgpu::DepthBiasState;

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
        let mut updates = 0;
        let mut reset = false;
        'gameloop: loop {
            let start = Instant::now();
            'game_state_lock: {
                let mut gs = state.write().unwrap();
                let delta;
                match gs.status {
                    Status::Running => {
                        {
                            let input_read_lock = inputs.read().unwrap();
                            delta = self.get_delta();
                            gs.handle_input(&input_read_lock, delta);
                        }
                        gs.update(delta);
                        self.last_update = Instant::now();
                    }
                    Status::Paused => {
                        {
                            let input_read_lock = inputs.read().unwrap();
                            gs.check_paused(&input_read_lock);
                        }
                        self.last_update = Instant::now();
                    }
                    Status::Reset => {
                        reset = true;
                        gs.status = Status::Running;
                        break 'game_state_lock;
                    }
                    Status::Exit => {
                        if let Status::Exit = gs.status {
                            break 'gameloop;
                        }
                    }
                };
            }
            if reset {
                thread::sleep(Duration::from_millis(500));
                reset = false;
                self.last_update = Instant::now();
            }

            updates += 1;
            let elapsed = start.elapsed();
            if elapsed < self.ticks {
                thread::sleep(self.ticks - elapsed);
            }
        }
        println!("Updates: {}", updates);
    }
    pub fn get_delta(&self) -> u128 {
        Instant::now().duration_since(self.last_update).as_millis()
    }
}
