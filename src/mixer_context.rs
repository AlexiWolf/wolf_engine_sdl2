use wolf_engine::*;
use sdl2::mixer;

pub struct SdlMixerContext {
    pub subsystem: mixer::Sdl2MixerContext,
}

impl SdlMixerContext {
    pub fn new() -> Self {
        let subsystem = mixer::init(mixer::InitFlag::all())
            .expect("Failed to initialize mixer");

        Self {
            subsystem
        }
    }
}

impl Subcontext for SdlMixerContext {}
