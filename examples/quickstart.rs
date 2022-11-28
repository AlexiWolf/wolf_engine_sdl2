use wolf_engine::prelude::*;

pub fn main() {
    let sdl2 = wolf_engine_sdl2::init()
        .expect("Failed to initialize SDL2");
    let mut engine = Engine::from(sdl2); 

    while let Some(event) = engine.next_event() {
        engine.context_mut().handle_events();
        match event {
            Event::Quit => println!("Goodbye!"),
            Event::Update => (), 
            Event::Render => (),
            Event::EventsCleared => {
                engine.update();
                engine.render();
            },
            _ => (),
        }
    }
}
