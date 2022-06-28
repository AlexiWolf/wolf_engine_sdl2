use crate::*;
use sdl2::event::Event;
use wolf_engine::utils::*;
use wolf_engine::*;

pub(crate) fn run_with_sdl(mut engine: Engine) {
    log_sdl_version();
    let mut has_window_quit = false;
    while engine.state_stack.is_not_empty() {
        profile_new_frame();
        profile_scope!("frame");
        if let Some(Ok(sdl_context)) = engine.context.try_borrow::<SdlContext>() {
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
            .profile_update(&mut engine.context, &mut engine.state_stack);
        engine
            .scheduler
            .profile_render(&mut engine.context, &mut engine.state_stack);
        if has_window_quit {
            log::debug!("The SDL window has quit.  Shutting down the engine.");
            engine.state_stack.clear(&mut engine.context);
        }
    }
}
