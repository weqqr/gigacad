pub mod render;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use crate::render::Renderer;

struct App {
    renderer: Renderer,
}

impl App {
    fn new(window: &Window) -> Self {
        let renderer = Renderer::new(window);

        Self {
            renderer,
        }
    }

    fn handle_event(&mut self, event: WindowEvent) -> Option<ControlFlow> {
        match event {
            WindowEvent::CloseRequested => return Some(ControlFlow::Exit),
            WindowEvent::Resized(size) => self.renderer.resize(size),
            _ => (),
        }

        None
    }

    fn redraw(&mut self) {
        self.renderer.render();
    }
}

struct AppContainer {
    event_loop: EventLoop<()>,
    window: Window,
    app: App,
}

impl AppContainer {
    fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("GigaCAD")
            .build(&event_loop)
            .expect("unable to create a window");


        let app = App::new(&window);

        Self {
            event_loop,
            window,
            app,
        }
    }

    fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent { event, .. } => {
                    if let Some(cf) = self.app.handle_event(event) {
                        *control_flow = cf;
                    }
                }
                Event::RedrawRequested(_) => {
                    self.app.redraw();
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => {}
            }
        });
    }
}

fn main() {
    let container = AppContainer::new();
    container.run();
}
