use anyhow::anyhow;
use vulkanalia::Entry;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use crate::vulkan_core;


pub fn run_app() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vulkan Thingy with Rust")
        .with_inner_size(LogicalSize::new(800, 600))
        .build(&event_loop)?;

    let mut app = unsafe { RenderApp::create(&window)? };
    let mut destroyed = false;
    event_loop.run(move | event, _, control_flow | {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared if !destroyed =>
                unsafe { app.render(&window) }.unwrap(),

            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                destroyed = true;
                *control_flow = ControlFlow::Exit;
                unsafe { app.destroy(); }
            }
            _ => {}
        }
    });
}


#[derive(Clone, Debug)]
struct RenderApp {}

impl RenderApp {
    unsafe fn create(window: &Window) -> anyhow::Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let _instance = vulkan_core::create_instance(window, &entry)?;
        Ok(Self {})
    }

    unsafe fn render(&mut self, _window: &Window) -> anyhow::Result<()> {
        Ok(())
    }

    unsafe fn destroy(&mut self) {

    }
}