use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::gameloop::Gameloop;
use crate::net::NetHandle;
use crate::systems::render::Render;
use shared::state::gamestate::{GameState, Status};

mod gameloop;
mod net;
mod systems;
use systems::input;

#[derive(Default)]
struct App {
    // Stores the window and its required functions
    window: Option<Arc<Window>>,
    // Stores the pixel buffer and the frame
    pixels: Option<Pixels<'static>>,
    // The current state of the game
    state: Arc<RwLock<GameState>>,
    // Stores the inputs of the game
    inputs: Arc<RwLock<input::Input>>,
    frames: u32,
}

impl ApplicationHandler for App {
    // Acc to my understanding (not very good yet) this function runs only once when window is
    // being created for the first time for my use case. (Can run multiple times)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let size = LogicalSize::new(400.0, 300.0);
            let scaled_size =
                LogicalSize::new(shared::WIDTH as f64 * 3.0, shared::HEIGHT as f64 * 3.0);
            Arc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_title("NPONG")
                            .with_inner_size(scaled_size)
                            .with_min_inner_size(size)
                            .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None))),
                    )
                    .unwrap(),
            )
        };

        self.window = Some(window);

        let pixels = {
            let window_size = self.window.as_ref().unwrap().inner_size();
            let surface_texture = SurfaceTexture::new(
                window_size.width,
                window_size.height,
                Arc::clone(self.window.as_ref().unwrap()),
            );
            let mut p = Pixels::new(shared::WIDTH, shared::HEIGHT, surface_texture)
                .expect("Pixel creation failed");
            p.clear_color(pixels::wgpu::Color {
                r: 0.018,
                g: 0.0,
                b: 0.02,
                a: 1.0,
            });
            p
        };
        self.pixels = Some(pixels);

        self.state.write().unwrap().status = Status::Running;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                {
                    let mut gs = self.state.write().unwrap();
                    gs.status = Status::Exit;
                }
                println!("The close button was pressed; stopping.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.frames += 1;
                Render::draw(Arc::clone(&self.state), self.pixels.as_mut().unwrap());
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let winit::keyboard::PhysicalKey::Code(key) = event.physical_key {
                    let mut input_lock = self.inputs.write().unwrap();
                    input_lock.get_inputs(&key, event);
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
        {
            // let gs = self.state.read().unwrap();
            // match gs.status {}
        }
    }

    // Implementation to look for closing window after key press does continuous checks
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let gs = self.state.read().unwrap();
        if let Status::Exit = gs.status {
            println!("Game loop requested exit. Closing OS window wrapper.");
            event_loop.exit();
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let sys = actix_rt::System::new();

        sys.block_on(async move {
            match net::connect().await {
                Ok((handle, player_id)) => {
                    tx.send(Ok((handle, player_id))).unwrap();
                }
                Err(err) => {
                    tx.send(Err(err)).unwrap();
                }
            }
        });

        sys.run().unwrap();
    });

    match rx.recv().unwrap() {
        Ok((handle, player_id)) => {
            run(handle, player_id);
        }
        Err(err) => println!("{}", err),
    }
}

fn run(handle: NetHandle, player_id: u8) {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let state = Arc::new(RwLock::new(GameState::new(player_id)));
    let mut app = App {
        state: Arc::clone(&state),
        ..Default::default()
    };

    let gl_state = Arc::clone(&app.state);
    let gl_inputs = Arc::clone(&app.inputs);
    let mut gameloop = Gameloop {
        ticks: Duration::from_millis(16),
        last_update: Instant::now(),
    };

    let _game_loop = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(500));
        gameloop.last_update = Instant::now();
        gameloop.game_loop(gl_state, gl_inputs, handle);
    });

    let time = Instant::now();
    let _x = event_loop.run_app(&mut app);
    println!("Frames: {}", app.frames);
    println!("Time elapsed: {}", time.elapsed().as_secs_f32());
}
