use wolf_engine::events::Event as WolfEvent;
use wolf_engine::events::EventLoop as WolfEventLoop;
use wolf_engine::events::EventQueue as WolfEventQueue;
use sdl2::event::Event as SdlEvent;

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
    pub sdl: sdl2::Sdl,
    pub audio: sdl2::AudioSubsystem,
    pub event: sdl2::EventSubsystem,
    pub joystick: sdl2::JoystickSubsystem,
    pub haptic: sdl2::HapticSubsystem,
    pub game_controller: sdl2::GameControllerSubsystem,
    pub timer: sdl2::TimerSubsystem,
    pub video: sdl2::VideoSubsystem,
}

impl SdlContext {
    pub fn new(sdl: sdl2::Sdl) -> Self {
        let wolf_event_queue = WolfEventQueue::new();
        let sdl_event_pump = sdl.event_pump()
            .expect("Failed to initialize SDL2's event pump");
        let audio = sdl.audio()
            .expect("Failed to initialize SDL2's audio subsystem");
        let event = sdl.event()
            .expect("Failed to initialize SDL2's event subsystem");
        let joystick = sdl.joystick()
            .expect("Failed to initialize SDL2's joystick subsystem");
        let haptic = sdl.haptic()
            .expect("Failed to initialize SDL2's haptic subsystem");
        let game_controller = sdl.game_controller()
            .expect("Failed to initialize SDL2's game controller subsystem");
        let timer = sdl.timer()
            .expect("Failed to initialize SDL2's timer subsystem");
        let video = sdl.video()
            .expect("Failed to initialize SDL2's video subsystem");
        Self { 
            wolf_event_queue, 
            sdl_event_pump, 
            sdl,
            audio,
            event,
            joystick,
            haptic,
            game_controller,
            timer,
            video,
        }
    }

    pub fn handle_events(&mut self) {
        for event in self.sdl_event_pump.poll_iter() {
            match event {
                SdlEvent::Quit { .. } => 
                    self.wolf_event_queue.send_event(WolfEvent::Quit),
                _ => (),
            }
        }
    }
}

impl WolfEventLoop<WolfEvent> for SdlContext {
    fn next_event(&mut self) -> Option<WolfEvent> {
        self.wolf_event_queue.next_event()
    }

    fn send_event(&self, event: WolfEvent) {
        self.wolf_event_queue.send_event(event)
    }
}
