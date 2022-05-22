pub use wolf_engine::*;

pub struct Sdl2MixerContext {}

impl MixerContext {
    pub fn new() -> Self {
        Self {}
    }
}

impl Subcontext for MixerContext {}
