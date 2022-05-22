use log::*;
use sdl2::{pixels::Color, mixer};
use wolf_engine::*;
use wolf_engine_sdl2::*;

pub fn main() {
    logging::initialize_logging(LevelFilter::Info);

    let window_settings = SdlWindowSettings::new("Mixer Demo", 800, 600);

    EngineBuilder::new()
        .with_plugin(Box::from(SdlPlugin::new(window_settings)))
        .build()
        .run(Box::from(MainState::new()));
}

pub struct MainState {}

impl MainState {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for MainState {
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
        None
    }

    fn render(&mut self, context: &mut Context) -> RenderResult {
        if let Some(Ok(mut video)) = context.try_borrow_mut::<SdlVideoContext>() {
            video.canvas.set_draw_color(Color::BLACK);
            video.canvas.present();
        }
    }
}
