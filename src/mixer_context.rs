use sdl2::mixer;
use wolf_engine::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct MixerSettings {
    frequency: i32,
    format: u16,
    channels: i32,
    chunk_size: i32,
    allocate_channels: i32,
    init_flag: mixer::InitFlag,
}

impl Default for MixerSettings {
    fn default() -> Self {
        Self {
            frequency: mixer::DEFAULT_FREQUENCY,
            format: mixer::DEFAULT_FORMAT,
            channels: mixer::DEFAULT_CHANNELS,
            chunk_size: 1024,
            allocate_channels: 4,
            init_flag: mixer::InitFlag::all(),
        }
    }
}

pub struct SdlMixerContext {
    pub subsystem: mixer::Sdl2MixerContext,
}

impl SdlMixerContext {
    pub fn new(settings: MixerSettings) -> Self {
        mixer::open_audio(
            settings.frequency,
            settings.format,
            settings.channels,
            settings.chunk_size,
        )
        .expect("Failed to open mixer");
        let subsystem = mixer::init(settings.init_flag).expect("Failed to initialize mixer");

        Self { subsystem }
    }
}

impl Subcontext for SdlMixerContext {}
