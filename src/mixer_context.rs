pub use wolf_engine::*;

pub struct SdlMixerContext {}

impl SdlMixerContext {
    pub fn new() -> Self {
        Self {}
    }
}

impl Subcontext for SdlMixerContext {}
