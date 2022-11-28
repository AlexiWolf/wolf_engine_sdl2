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
}

impl SdlContext {
    pub fn new() -> Self {
        Self { wolf_event_queue: WolfEventQueue::new() }
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
