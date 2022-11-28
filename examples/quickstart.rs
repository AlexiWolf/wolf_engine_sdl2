use sdl2::pixels::Color;
use wolf_engine::prelude::*;

pub fn main() {
    let sdl2 = wolf_engine_sdl2::init()
        .expect("Failed to initialize SDL2");
    let mut engine = Engine::from(sdl2); 
    let window = engine.context()
            .window("Wolf Engine / SDL2 Quickstart Demo", 800, 600)
            .position_centered()
            .build()
            .expect("Failed to create the window");
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    while let Some(event) = engine.next_event() {
        engine.context_mut().handle_events();
        match event {
            Event::Quit => println!("Goodbye!"),
            Event::Update => (), 
            Event::Render => {
                canvas.set_draw_color(Color::BLACK);
                canvas.clear();
                canvas.present();
            },
            Event::EventsCleared => {
                engine.update();
                engine.render();
            },
            _ => (),
        }
    }
}
