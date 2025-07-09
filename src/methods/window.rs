use std::sync::mpsc::{Receiver, Sender};

use deno_core::{
    error::AnyError,
    op2,
    OpState
};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};


pub enum Command
{
    OpenWindow(String),
}

#[derive(Default)]
pub struct App
{
    window: Option<Window>,
    rx: Option<Receiver<Command>>,
}

impl App
{
    pub fn new(rx: Receiver<Command>) -> Self
    {
        Self {
            window: None,
            rx: Some(rx)
        }
    }
}

impl ApplicationHandler for App
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        let Some(rx) = &self.rx else { panic!("missing `rx` at ApplicationHandler") };

        while let Ok(command) = rx.try_recv()
        {
            match command
            {
                Command::OpenWindow(title) => {
                    let mut attributes = Window::default_attributes();
                    attributes.title = title;
                    self.window = Some(event_loop.create_window(attributes).unwrap());
                }
            }
        }
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
pub fn new_window(state: &mut OpState, #[string] title: String) -> Result<(), AnyError>
{
    /*let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.set_control_flow(ControlFlow::Wait);*/

    let sender = state.borrow::<Sender<Command>>().clone();
    sender.send(Command::OpenWindow(title))?;
    
    Ok(())
}