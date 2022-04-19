use crate::*;
use wolf_engine::*;
use sdl2::event::Event;

pub fn run_with_sdl(mut engine: Engine) {
    log_sdl_version();
    let mut should_quit = false;
    while engine.state_stack.is_not_empty() {
        if let Some(Ok(sdl_context)) = engine.context.try_borrow::<SdlContext>() {
            let mut event_pump = sdl_context.sdl.event_pump().unwrap();
            for even in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        should_quit = true;
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
        if should_quit {
            engine.state_stack.clear(&mut context); 
        }
    }
}
