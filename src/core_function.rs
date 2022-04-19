use crate::*;
use wolf_engine::*;

pub fn run_with_sdl(mut engine: Engine) {
    while engine.state_stack.is_not_empty() {
        engine 
            .scheduler
            .update(&mut engine.context, &mut engine.state_stack);
        engine
            .scheduler
            .render(&mut engine.context, &mut engine.state_stack);
    }
}
