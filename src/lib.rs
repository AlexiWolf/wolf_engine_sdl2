use wolf_engine::events::Event as WolfEvent;
use wolf_engine::events::EventLoop as WolfEventLoop;
use wolf_engine::events::EventQueue as WolfEventQueue;

pub fn init() -> Result<SdlContext, String> {
    let sdl_result = sdl2::init();
    if let Ok(sdl) = sdl_result {
        let sdl_context = SdlContext::new(sdl);
        Ok(sdl_context) 
    } else {
        Err(sdl_result.err().unwrap())
    }
}

pub struct SdlContext {
    wolf_event_queue: WolfEventQueue<WolfEvent>, 
    sdl_event_pump: sdl2::EventPump,
    sdl: sdl2::Sdl,
}

impl SdlContext {
    pub fn new(sdl: sdl2::Sdl) -> Self {
        let wolf_event_queue = WolfEventQueue::new();
        let sdl_event_pump = sdl.event_pump()
            .expect("Failed to initialize SDL2's event pump");
        Self { wolf_event_queue, sdl_event_pump, sdl }
    }

    pub fn handle_events(&mut self) {}
}

impl WolfEventLoop<WolfEvent> for SdlContext {
    fn next_event(&mut self) -> Option<WolfEvent> {
        self.wolf_event_queue.next_event()
    }

    fn send_event(&self, event: WolfEvent) {
        self.wolf_event_queue.send_event(event)
    }
}
