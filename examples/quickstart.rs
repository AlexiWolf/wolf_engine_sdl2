use colors_transform::{Color, Hsl};
use log::*;
use wolf_engine::*;
use wolf_engine_sdl2::*;

pub fn main() {
    logging::initialize_logging(LevelFilter::Debug);

    EngineBuilder::new()
        .with_plugin(Box::from(SdlPlugin::new(SdlWindowSettings::new(
            "Hello, SDL and Wolf Engine!",
            800,
            600,
        ))))
        .build()
        .run(Box::from(ExampleGame::new()));
}

pub struct ExampleGame {
    hew: f32,
}

impl ExampleGame {
    pub fn new() -> Self {
        Self { hew: 0.0 }
    }
}

impl State for ExampleGame {
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
        self.hew = (self.hew + 1.0) % 360.0;
        None
    }

    fn render(&mut self, context: &mut Context) -> RenderResult {
        if let Some(Ok(mut sdl)) = context.try_borrow_mut::<SdlVideoContext>() {
            let color = Hsl::from(self.hew, 100.0, 50.0);
            sdl.canvas.set_draw_color(SdlColor::RGB(
                color.get_red() as u8,
                color.get_green() as u8,
                color.get_blue() as u8,
            ));
            sdl.canvas.clear();
            sdl.canvas.present();
        }
    }
}
