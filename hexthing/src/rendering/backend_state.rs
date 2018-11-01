use hal::{window, Instance};

use super::{AdapterState, SurfaceImpl};

pub struct BackendState {
    pub adapter: AdapterState,
    pub surface: SurfaceImpl,
    pub events_loop: winit::EventsLoop,
    _window: winit::Window,
}

impl BackendState {
    pub fn new(window_dimensions: window::Extent2D) -> Self {
        let instance = back::Instance::create("gfx-rs quad", 1);
        let events_loop = winit::EventsLoop::new();

        let window = winit::WindowBuilder::new()
            .with_dimensions(winit::dpi::LogicalSize::new(
                f64::from(window_dimensions.width),
                f64::from(window_dimensions.height),
            ))
            .with_title("quad".to_owned())
            .build(&events_loop)
            .unwrap();

        let surface = instance.create_surface(&window);
        let mut adapters = instance.enumerate_adapters();
        BackendState {
            adapter: AdapterState::new(&mut adapters),
            surface,
            events_loop,
            _window: window,
        }
    }
}
