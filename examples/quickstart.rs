use colors_transform::{Color, Hsl};
use log::*;
use sdl2::{
    gfx::primitives::DrawRenderer, pixels::Color as SdlColor, pixels::PixelFormatEnum, rect::{Rect, Point},
    render::{Canvas, Texture}, surface::Surface,
};
use wolf_engine::*;
use wolf_engine_sdl2::*;

pub fn main() {
    logging::initialize_logging(LevelFilter::Debug);
    let window_settings = SdlWindowSettings::new("Hello, SDL and Wolf Engine!", 800, 600);
    let sdl_plugin = SdlPlugin::builder()
        .with_window_settings(window_settings)
        .build();

    EngineBuilder::new()
        .with_plugin(Box::from(sdl_plugin))
        .build()
        .unwrap()
        .run(Box::from(ExampleGame::new()));
}

pub struct ExampleGame {
    hew: f32,
    rotation: f64,
}

impl ExampleGame {
    pub fn new() -> Self {
        Self { 
            hew: 0.0, 
            rotation: 0.0, 
        }
    }
}

impl State for ExampleGame {
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
        self.hew = (self.hew + 1.0) % 360.0;
        self.rotation = (self.rotation + 1.0) % 360.0;
        None
    }

    fn render(&mut self, context: &mut Context) -> RenderResult {
        let mut video = context.borrow_mut::<SdlVideoContext>().unwrap();
        let color = Hsl::from(self.hew, 100.0, 50.0);
        video.canvas.set_draw_color(SdlColor::RGB(
            color.get_red() as u8,
            color.get_green() as u8,
            color.get_blue() as u8,
        ));
        video.canvas.clear();

        let texture_creator = video.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_target(PixelFormatEnum::RGBA8888, 100, 10)
            .unwrap();
        video.canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas
                .string(3, 1, "Hello, SDL2!", SdlColor::WHITE)
                .unwrap();
        }).unwrap();

        let (width, height) = video.canvas.window().drawable_size();


        video.canvas.copy_ex(
            &texture,
            Rect::new(0, 0, 100, 10), 
            Rect::new((width as i32 / 2) - 50, (height as i32 / 2) - 5, 100, 10), 
            self.rotation, 
            None,
            false, 
            false,
        ).unwrap();
        video.canvas.present();
    }
}
