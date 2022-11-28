use wolf_engine::prelude::*;

pub fn main() {
    let sdl2 = wolf_engine_sdl2::init()
        .expect("Failed to initialize SDL2");
    let engine = Engine::from(sdl2); 
}
