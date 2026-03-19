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

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

#[derive(Default)]
struct App {
    window: Option<Window>,
}

// This is the ApplicationHandler trait that is used by winit to update window and everything
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // self.window = Some(
        //     event_loop
        //         .create_window(Window::default_attributes().with_title("Love you macha"))
        //         .unwrap(),
        // );
        self.window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
            Some(
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
    // Pixels for filling the frame unsure what's happening rn, copied from Conway's game of life
    // in Pixels' Github
        // let mut pixels = {
        //     let window_size = self.window.as_ref().unwrap().inner_size();
        //     let surface_texture = SurfaceTexture::new(
        //         window_size.width,
        //         window_size.height,
        //         self.window.as_ref().unwrap(),
        //     );
        //     Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
        // };
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // let size = self.window.as_ref().unwrap().inner_size();
                // let surface_texture =
                //     SurfaceTexture::new(size.width, size.height, self.window.as_ref().unwrap());
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

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
