use wolf_engine::*;

pub struct SdlContext;

impl SdlContext {
    pub fn new() -> Self {
        Self
    }
}

impl Subcontext for SdlContext {}
