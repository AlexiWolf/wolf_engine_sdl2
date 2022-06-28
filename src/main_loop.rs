use crate::*;
use sdl2::event::Event;
use wolf_engine::utils::*;
use wolf_engine::*;

pub struct SdlMainLoop;

impl SdlMainLoop {
    pub fn new() -> Self {
        Self
    }
}

impl MainLoop for SdlMainLoop {
    fn run(&mut self, engine: Engine) -> Engine {
        log_sdl_version();
        let mut has_window_quit = false;

        while engine.state_stack.is_not_empty() {
            profile_new_frame();
            profile_scope!("frame");
            {
                let sdl_context = engine.context.borrow::<SdlContext>().expect("No SdlContext");
                profile_scope!("process_sdl_events");
                let mut event_pump = sdl_context.sdl.event_pump().unwrap();
                for event in event_pump.poll_iter() {
                    if let Event::Quit { .. } = event {
                        has_window_quit = true;
                    }
                }
            }
            engine
                .scheduler
                .update(&mut engine.context, &mut engine.state_stack);
            engine
                .scheduler
                .render(&mut engine.context, &mut engine.state_stack);
            if has_window_quit {
                log::debug!("The SDL window has quit.  Shutting down the engine.");
                engine.state_stack.clear(&mut engine.context);
            }
        }
        engine
    }
}

