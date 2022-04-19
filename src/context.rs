use sdl2::Sdl;
use wolf_engine::*;

pub struct SdlContext {
    pub sdl: Sdl,
}

impl SdlContext {
    pub fn new() -> Self {
        let sdl = sdl2::init().expect("Failed to initialize SDL");
        Self { sdl }
    }
}

impl Subcontext for SdlContext {}
