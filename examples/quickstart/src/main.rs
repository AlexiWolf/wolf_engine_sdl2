use log::*;
use wolf_engine::*;
use wolf_engine_sdl2::*;

pub fn main() {
    logging::initialize_logging(LevelFilter::Debug);

    EngineBuilder::new()
        .with_plugin(Box::from(SdlPlugin::new(SdlWindowSettings::default())))
        .build()
        .run(Box::from(ExampleGame::new()));
}

pub struct ExampleGame;

impl ExampleGame {
    pub fn new() -> Self {
        Self
    }
}

impl State for ExampleGame {
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
        None
    }

    fn render(&mut self, context: &mut Context) -> RenderResult {
        
    }
}
