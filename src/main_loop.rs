use crate::*;
use sdl2::event::Event;
use wolf_engine::utils::*;
use wolf_engine::*;

pub struct SdlMainLoop {
    has_window_quit: bool,
}

impl MainLoop for SdlMainLoop {
    fn run(&mut self, mut engine: Engine) -> Engine {
        log_sdl_version();
        while engine.state_stack.is_not_empty() {
            profile_new_frame();
            profile_scope!("frame");
            self.process_sdl_events(&engine.context);
            self.run_frame(&mut engine);
            self.handle_window_quit(&mut engine);
        }
        engine
    }
}

impl SdlMainLoop {
    pub fn new() -> Self {
        Self {
            has_window_quit: false,
        }
    }

    fn process_sdl_events(&mut self, context: &Context) {
        profile_scope!("process_sdl_events");
        let sdl_context = context.borrow::<SdlContext>().expect("No SdlContext");
        let mut event_pump = sdl_context.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                self.has_window_quit = true;
            }
        }
    }

    fn run_frame(&self, engine: &mut Engine) {
        engine.update();
        engine.render();
    }

    fn handle_window_quit(&mut self, engine: &mut Engine) {
        if self.has_window_quit {
            log::debug!("The SDL window has quit.  Shutting down the engine.");
            engine.quit();
        }
    }
}

impl Default for SdlMainLoop {
    fn default() -> Self {
        Self::new()
    }
}
