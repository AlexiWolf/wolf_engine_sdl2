use wolf_engine::events::Event as WolfEvent;
use wolf_engine::events::EventLoop as WolfEventLoop;

pub fn init() -> Result<SdlContext, String> {
    let sdl_context = SdlContext::new();
}

pub struct SdlContext {
    
}

impl SdlContext {

}

impl WolfEventLoop<WolfEvent> for SdlContext {
    fn next_event(&mut self) -> Option<WolfEvent> {
        None
    }

    fn send_event(&self, event: WolfEvent) {}
}
