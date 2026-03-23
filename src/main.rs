use gamestate::GameState;
use object::Object;
use pixels::{Error, Pixels, SurfaceTexture};
use std::sync::Arc;
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

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

// Using Arc for Window because references to Window is going to used for everything (was facing an
// issue with lifetimes so had to look into Smart Pointers)
#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    state: GameState,
}

// This is the ApplicationHandler trait that is used by winit to update window and everything
impl ApplicationHandler for App {
    // Acc to my understanding (not very good yet) this function runs only once when window is
    // being created for the first time for my use case. (Can run multiple times)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            let scaled_size = LogicalSize::new(WIDTH as f64 * 4.0, HEIGHT as f64 * 4.0);
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
                r: 0.004,
                g: 0.0,
                b: 0.008,
                a: 1.0,
            });
            p
        };
        self.pixels = Some(pixels);

        let gamestate = GameState {
            players: 2,
            objects: vec![
                Object::Paddle(object::paddle::Paddle {
                    x: 185,
                    y: 290,
                    height: 5,
                    width: 30,
                }),
                Object::Paddle(object::paddle::Paddle {
                    x: 185,
                    y: 5,
                    height: 5,
                    width: 30,
                }),
                Object::Ball(object::ball::Ball {
                    x: 185,
                    y: 5,
                    radius: 4,
                    vx: 0.0,
                    vy: 0.0,
                }),
            ],
        };
        self.state = gamestate;
        self.state.draw(self.pixels.as_mut().unwrap().frame_mut());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // let frame = self.pixels.as_mut().unwrap().frame_mut();
                // draw_rect(frame, 0, 0, 10, 10);
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
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    // let window = Window::new(&event_loop).unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    // event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    let _x = event_loop.run_app(&mut app);
}
