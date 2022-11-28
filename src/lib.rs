use wolf_engine::events::Event as WolfEvent;
use wolf_engine::events::EventLoop as WolfEventLoop;

pub fn init() -> Result<SdlContext, String> {
    Err("Not implemented".to_string())
}

pub struct SdlContext {

}

impl WolfEventLoop<WolfEvent> for SdlContext {
    fn next_event(&mut self) -> Option<WolfEvent> {
        None
    }

    fn send_event(&self, event: WolfEvent) {}
}
