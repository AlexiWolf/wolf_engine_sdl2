use crate::*;
use sdl2::event::Event;
use wolf_engine::utils::*;
use wolf_engine::*;

pub struct SdlMainLoop {
    has_window_quit: bool,
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
}

impl MainLoop for SdlMainLoop {
    fn run(&mut self, mut engine: Engine) -> Engine {
        log_sdl_version();

        while engine.state_stack.is_not_empty() {
            profile_new_frame();
            profile_scope!("frame");
            self.process_sdl_events(&engine.context);

            engine
                .scheduler
                .update(&mut engine.context, &mut engine.state_stack);
            engine
                .scheduler
                .render(&mut engine.context, &mut engine.state_stack);

            if self.has_window_quit {
                log::debug!("The SDL window has quit.  Shutting down the engine.");
                engine.quit();
            }
        }
        engine
    }
}

