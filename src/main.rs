use gamestate::GameState;
use pixels::{Error, Pixels, SurfaceTexture};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::window::WindowId;
use winit::{
    dpi::LogicalSize,
    event::{Event, MouseButton, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::Window,
};

pub mod gamestate;
pub mod object;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 900;

// Using Arc for Window because references to Window is going to used for everything (was facing an
// issue with lifetimes so had to look into Smart Pointers)
#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    state: Arc<Mutex<GameState>>,
    count: u32,
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
                r: 0.008,
                g: 0.0,
                b: 0.01,
                a: 1.0,
            });
            p
        };
        self.pixels = Some(pixels);

        // self.state.draw(self.pixels.as_mut().unwrap().frame_mut());
        self.state
            .lock()
            .unwrap()
            .draw(self.pixels.as_mut().unwrap().frame_mut());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!(
                    "The close button was pressed; stopping. Frames: {}",
                    self.count
                );
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // self.count += 1;
                {
                    let gs = self.state.lock().unwrap();
                    gs.clear_screen(self.pixels.as_mut().unwrap().frame_mut());
                    gs.draw(self.pixels.as_mut().unwrap().frame_mut());
                }
                if let Err(err) = self.pixels.as_ref().unwrap().render() {
                    dbg!(err);
                    return;
                }
                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            // TODO: Learn to use if let you dumass. The below code would be so much more simpler
            // then. Don't forget to change this later
            WindowEvent::KeyboardInput { event, .. } => {
                match &event.physical_key {
                    winit::keyboard::PhysicalKey::Code(key) => match key {
                        KeyCode::KeyA => {
                            self.state
                                .lock()
                                .unwrap()
                                .paddles
                                .get_mut(0)
                                .unwrap()
                                .left_shift();
                        }
                        KeyCode::KeyD => {
                            self.state
                                .lock()
                                .unwrap()
                                .paddles
                                .get_mut(0)
                                .unwrap()
                                .right_shift();
                        }
                        KeyCode::ArrowLeft => {
                            self.state
                                .lock()
                                .unwrap()
                                .paddles
                                .get_mut(1)
                                .unwrap()
                                .left_shift();
                        }
                        KeyCode::ArrowRight => {
                            self.state
                                .lock()
                                .unwrap()
                                .paddles
                                .get_mut(1)
                                .unwrap()
                                .right_shift();
                        }
                        _ => {}
                    },
                    _ => {
                        todo!();
                    }
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let state = Arc::new(Mutex::new(GameState::new()));
    let mut app = App {
        state,
        ..Default::default()
    };

    let state = Arc::clone(&app.state);
    let tick = Duration::from_millis(16);
    thread::spawn(move || {
        loop {
            let start = Instant::now();
            {
                let mut gs = state.lock().unwrap();
                gs.update();
            }

            let elapsed = start.elapsed();
            if elapsed < tick {
                thread::sleep(tick - elapsed);
            }
        }
    });

    let _x = event_loop.run_app(&mut app);
}
