// Copyright 2024 Natalie Baker // AGPLv3 //

use pollster::FutureExt;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::WindowId};

pub trait ActiveApplication<C> {
    #[allow(async_fn_in_trait)]
    async fn new(event_loop: &ActiveEventLoop, config: &C) -> Self;

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent);
}

pub struct ApplicationShim<T: ActiveApplication<C>, C> {
    running: Option<T>,
    config: C,
}

impl<T: ActiveApplication<C>, C> ApplicationShim<T, C> {
    pub const fn new(config: C) -> Self {
        Self { running: None, config }
    }
}

impl<T: ActiveApplication<C>, C> ApplicationHandler for ApplicationShim<T, C> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.running = Some(T::new(event_loop, &self.config).block_on());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(running) = self.running.as_mut() {
            running.window_event(event_loop, window_id, event);
        } else if event == WindowEvent::CloseRequested {
            event_loop.exit();
        }
    }
}