use wolf_engine::prelude::*;

#[test]
pub fn should_send_and_recieve_wolf_engine_events() {
    let mut sdl = wolf_engine_sdl2::init()
        .expect("Failed to initialize SDL2");

    sdl.send_event(Event::Update);
    assert_eq!(sdl.next_event().unwrap(), Event::Update);
    assert!(sdl.next_event().is_none());
}
