use log::*;
use sdl2::mixer;
use wolf_engine::*;

/// Settings to use when configuring [mixer].
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct MixerSettings {
    /// The desired audio frequency.
    pub frequency: i32,

    /// The desired [mixer::AudioFormat].
    pub format: mixer::AudioFormat,

    /// The number of audio channels.
    pub channels: i32,

    /// The audio chunk size.  See [mixer::open_audio()] for more details.
    pub chunk_size: i32,

    /// The desired number of channels to allocate.  See [mixer::allocate_channels()] for
    /// more details.
    pub allocate_channels: i32,

    /// The desired [mixer::InitFlag].
    pub init_flag: mixer::InitFlag,
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

/// Provides setup and access to [mixer].
///
/// See [sdl2::mixer] for more information.
///
/// # Examples
///
/// Using the default settings:
/// ```
/// let mixer_context = SdlMixerContext::new(MixerSettings::default());
/// ```
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

pub(crate) fn log_mixer_version() {
    log::info!("Using Mixer v{}", mixer::get_linked_version());
}

pub(crate) fn log_music_decoders() {
    let decoders = mixer::get_music_decoders_number();
    debug!("available music decoders: {}", decoders);
    for i in 0..decoders {
        debug!("\tdecoder {} => {}", i, mixer::get_music_decoder(i));
    }
}

pub(crate) fn log_chunk_decoders() {
    let decoders = mixer::get_chunk_decoders_number();
    debug!("available chunk(sample) decoders: {}", decoders);
    for i in 0..decoders {
        debug!("\tdecoder {} => {}", i, mixer::get_chunk_decoder(i));
    }
}
