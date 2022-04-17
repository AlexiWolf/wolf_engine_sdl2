use wolf_engine::*;

pub struct SdlVideoContext;

impl SdlVideoContext {
    pub fn new() -> Self {
        Self
    }
}

impl Subcontext for SdlVideoContext {}

pub struct WindowSettings;

impl WindowSettings {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self 
    }
}
