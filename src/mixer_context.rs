use wolf_engine::*;
use sdl2::mixer;

pub struct SdlMixerContext {
    pub subsystem: mixer::Sdl2MixerContext,
}

impl SdlMixerContext {
    pub fn new() -> Self {
        Self {
            subsystem: mixer::init(mixer::InitFlag::all())
                .expect("Failed to initialize mixer"),
        }
    }
}

impl Subcontext for SdlMixerContext {}
