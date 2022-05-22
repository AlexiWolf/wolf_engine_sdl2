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

pub struct MainState<'a> {
    music: mixer::Music<'a>,
}

impl<'a> MainState<'a> {
    pub fn new() -> Self {
        Self {
            music: mixer::Music::from_file("examples/assets/rain.ogg")
                .expect("Failed to load audio"),
        }
    }
}

impl<'a> State for MainState<'a> {
    fn setup(&mut self, _context: &mut Context) {
        mixer::allocate_channels(4);
        self.music.play(-1).unwrap();
    }

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

