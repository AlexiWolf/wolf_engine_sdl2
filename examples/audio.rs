use log::*;
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


struct MainState {}

impl MainState {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for MainState {
    fn update(&mut self, context: &mut Context) -> OptionalTransition {
        if let Some(Ok(mut audio)) = context.try_borrow_mut::<SdlAudioContext>() {

        }
        None
    }

    fn render(&mut self, context: &mut Context) -> RenderResult {
        if let Some(Ok(mut video)) = context.try_borrow_mut::<SdlVideoContext>() {
            video.canvas.clear();
        }
    }
}
