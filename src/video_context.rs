use wolf_engine::*;

pub struct SdlVideoContext;

impl SdlVideoContext {
    pub fn new() -> Self {
        Self
    }
}

impl Subcontext for SdlVideoContext {}
