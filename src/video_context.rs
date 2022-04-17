use wolf_engine::*;

pub struct SdlVideoContext;

impl SdlVideoContext {
    pub fn new() -> Self {
        Self
    }
}

impl Subcontext for SdlVideoContext {}

/// Settings for creating the SDL window.
pub struct SdlWindowSettings;

impl SdlWindowSettings {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self 
    }
}
