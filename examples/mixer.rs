use log::*;
use sdl2::{mixer, pixels::Color};
use wolf_engine::*;
use wolf_engine_sdl2::*;

pub fn main() {
    logging::initialize_logging(LevelFilter::Debug);

    if cfg!(feature = "mixer") {
        let window_settings = SdlWindowSettings::new("Mixer Demo", 800, 600);
        let sdl_plugin = SdlPlugin::builder()
            .with_window_settings(window_settings)
            .build();
        EngineBuilder::new()
            .with_plugin(Box::from(sdl_plugin))
            .build()
            .unwrap()
            .run(Box::from(MainState::new()));
    } else {
        error!(
            "The mixer feature is required for this demo: Try running with \"--features mixer\""
        );
    }
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
        if let Some(mut video) = context.borrow_mut::<SdlVideoContext>() {
            video.canvas.set_draw_color(Color::BLACK);
            video.canvas.present();
        }
    }
}
