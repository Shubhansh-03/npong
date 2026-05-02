use awc::Client;
use futures_util::{SinkExt, StreamExt};

use gameloop::*;
use pixels::{Pixels, SurfaceTexture};
use shared::{gamestate::*, systems::input::*};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};
use systems::render::*;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use crate::clientstate::ClientState;

pub mod clientstate;
pub mod gameloop;
pub mod systems;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 900;

// Using Arc for Window because references to Window is going to used for everything (was facing an
// issue with lifetimes so had to look into Smart Pointers)
#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    state: Arc<RwLock<ClientState>>,
    inputs: Arc<RwLock<Input>>,
}

// This is the ApplicationHandler trait that is used by winit to update window and everything
impl ApplicationHandler for App {
    // Acc to my understanding (not very good yet) this function runs only once when window is
    // being created for the first time for my use case. (Can run multiple times)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let size = LogicalSize::new(400.0, 300.0);
            let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
            Arc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_title("NPONG")
                            .with_inner_size(scaled_size)
                            .with_min_inner_size(size),
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
            let mut p = Pixels::new(WIDTH, HEIGHT, surface_texture).expect("Pixel creation failed");
            p.clear_color(pixels::wgpu::Color {
                r: 0.018,
                g: 0.0,
                b: 0.02,
                a: 1.0,
            });
            p
        };
        self.pixels = Some(pixels);

        self.state.write().unwrap().game.status = Status::Running;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                {
                    let mut gs = self.state.write().unwrap();
                    gs.game.status = Status::Exit;
                }
                println!("The close button was pressed; stopping.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
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
    }
}

pub async fn init_net() -> Result<u8, Box<dyn std::error::Error>> {
    let cl = Client::default();
    let (_res, mut ws) = cl
        .ws("ws://127.0.0.1:8080/ws")
        .connect()
        .await
        .map_err(|e| format!("Failed to connect: {}", e))?;

    if let Some(Ok(awc::ws::Frame::Text(txt))) = ws.next().await {
        let id: u8 = std::str::from_utf8(&txt)?.parse()?;

        actix_rt::spawn(async move {
            while let Some(Ok(m)) = ws.next().await {
                println!("rx: {:?}", m);
            }
        });

        return Ok(id);
    }

    Err("Server connection failed".into())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    match init_net().await {
        Ok(assigned_id) => {
            println!("Joined room as Player {}", assigned_id);
            // 2. Start winit only after connection is successful
            run(assigned_id);
        }
        Err(e) => {
            eprintln!("Could not start game: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn run(player_id: u8) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    // Initialize state with the ID we got from the server
    let initial_state = ClientState {
        player_id,
        ..Default::default()
    };

    let state = Arc::new(RwLock::new(initial_state));
    let inputs = Arc::new(RwLock::new(Input::default()));

    let mut app = App {
        state: Arc::clone(&state),
        inputs: Arc::clone(&inputs),
        ..Default::default()
    };

    // Start GameLoop thread
    let loop_state = Arc::clone(&state);
    let loop_inputs = Arc::clone(&inputs);
    thread::spawn(move || {
        let mut gameloop = GameLoop {
            ticks: Duration::from_millis(16),
            last_update: Instant::now(),
        };
        gameloop.game_loop(loop_state, loop_inputs)
    });

    event_loop.run_app(&mut app).unwrap();
}

// fn run() {
//     let event_loop = EventLoop::new().unwrap();
//
//     // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
//     // dispatched any events. This is ideal for games and similar applications.
//     event_loop.set_control_flow(ControlFlow::Poll);
//
//     let state = Arc::new(RwLock::new(ClientState::default()));
//     let inputs = Arc::new(RwLock::new(Input::default()));
//     let mut app = App {
//         state,
//         inputs,
//         ..Default::default()
//     };
//
//     let state = Arc::clone(&app.state);
//     let inputs = Arc::clone(&app.inputs);
//     let mut gameloop = GameLoop {
//         ticks: Duration::from_millis(16),
//         last_update: Instant::now(),
//     };
//     let _game_loop = thread::spawn(move || {
//         thread::sleep(Duration::from_millis(500));
//         gameloop.last_update = Instant::now();
//         gameloop.game_loop(state, inputs)
//     });
//
//     let time = Instant::now();
//     let _x = event_loop.run_app(&mut app);
//     println!("Time elapsed: {}", time.elapsed().as_secs_f32());
// }
