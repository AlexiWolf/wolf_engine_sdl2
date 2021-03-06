use sdl2::{AudioSubsystem, Sdl};
use wolf_engine::*;

/// Provides access to [sdl2]'s [AudioSubsystem].
pub struct SdlAudioContext {
    pub subsystem: AudioSubsystem,
}

impl SdlAudioContext {
    pub fn new(sdl: &Sdl) -> Self {
        Self {
            subsystem: sdl
                .audio()
                .expect("Failed to initialize the audio subsystem"),
        }
    }
}

impl Subcontext for SdlAudioContext {}
