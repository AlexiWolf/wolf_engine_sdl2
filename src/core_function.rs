use crate::*;
use sdl2::event::Event;
use wolf_engine::*;

pub(crate) fn run_with_sdl(mut engine: Engine) {
    log_sdl_version();
    let mut has_window_quit = false;
    while engine.state_stack.is_not_empty() {
        if let Some(Ok(sdl_context)) = engine.context.try_borrow::<SdlContext>() {
            let mut event_pump = sdl_context.sdl.event_pump().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        has_window_quit = true;
                    }
                    _ => (),
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
}
