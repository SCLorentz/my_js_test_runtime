use deno_core::{
    error::AnyError,
    op2,
};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App
{
    window: Option<Window>,
    title: String,
}

impl App
{
    fn new(title: String) -> Self
    {
        Self {
            window: None,
            title,
        }
    }
}

impl ApplicationHandler for App
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        let mut attributes = Window::default_attributes();
        attributes.title = String::from(self.title.clone());
        
        self.window = Some(event_loop.create_window(attributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent)
    {
        match event
        {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => self.window.as_ref().unwrap().request_redraw(),
            _ => (),
        }
    }
}

#[op2(fast)]
pub fn new_window(#[string] title: String) -> Result<(), AnyError>
{
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new(title);
    let _ = event_loop.run_app(&mut app);
    
    Ok(())
}