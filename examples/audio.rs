use std::{borrow::Cow, path::PathBuf};

use log::*;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecWAV, AudioCVT, AudioSpecDesired};
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
        if let Some(Ok(audio)) = context.try_borrow::<SdlAudioContext>() {
            let desired_spec = AudioSpecDesired {
                freq: Some(48_000),
                channels: Some(2),
                samples: None,
            };
            self.audio_device = Some(
                audio.subsystem.open_playback(None, &desired_spec, |spec| {
                    let wav_spec = AudioSpecWAV::load_wav(Cow::from(PathBuf::from("examples/assets/rain.wav")))
                        .expect("Failed to load rain.wav");
                    let cvt = AudioCVT::new(
                        wav_spec.format,
                        wav_spec.channels,
                        wav_spec.freq,
                        spec.format,
                        spec.channels,
                        spec.freq,
                    ).expect("Could not convert wav file");
                    let data = cvt.convert(wav_spec.buffer().to_vec());
                    Sound {
                        data,
                        volume: 0.25,
                        pos: 0,
                    }
                }).expect("Failed to create audio device")
            );
            self.audio_device.as_ref().unwrap().resume();
        }
    }
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
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
