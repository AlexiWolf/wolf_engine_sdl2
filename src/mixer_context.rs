pub use wolf_engine::*;

pub struct Sdl2MixerContext {}

impl Sdl2MixerContext {
    pub fn new() -> Self {
        Self {}
    }
}

impl Subcontext for Sdl2MixerContext {}
