use log::*;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecWAV};
use wolf_engine::*;
use wolf_engine_sdl2::*;

pub fn main() {
    logging::initialize_logging(LevelFilter::Info);

    let window_settings = SdlWindowSettings::new("Audio Demo", 800, 600);

    EngineBuilder::new()
        .with_plugin(Box::from(SdlPlugin::new(window_settings)))
        .build()
        .run(Box::from(MainState::new()))
}

struct MainState {
    audio_device: Option<AudioDevice<Sound>>,
}

impl MainState {
    pub fn new() -> Self {
        Self {
            audio_device: None,
        }
    }
}

impl State for MainState {
    fn setup(&mut self, context: &mut Context) {
        let wav_spec = AudioSpecWAV::load_wav("examples/assets/rain.wav")
            .expect("Failed to load rain.wav");
        if let Some(Ok(mut audio)) = context.try_borrow_mut::<SdlAudioContext>() {
            self.audio_device = audio.subsystem.open_playback(None, &wav_spec, |spec| {

            }).expect("Failed to create audio device");
        }
    }
    fn update(&mut self, context: &mut Context) -> OptionalTransition {
        None
    }

    fn render(&mut self, context: &mut Context) -> RenderResult {
        if let Some(Ok(mut video)) = context.try_borrow_mut::<SdlVideoContext>() {
            video.canvas.clear();
        }
    }
}

struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for dst in out.iter_mut() {
            let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled;
            self.pos += 1;
        }
    }
}
