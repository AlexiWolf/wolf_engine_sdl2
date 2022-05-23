use sdl2::{pixels::Color, render::WindowCanvas, Sdl, VideoSubsystem};
use wolf_engine::*;

/// Provides access to [sdl2]'s [WindowCanvas] and [VideoSubsystem].
///
/// # Examples
///
/// ```
/// # use wolf_engine_sdl2::*;
/// # let sdl = sdl2::init().expect("Failed to initialize SDL");
/// # let sdl_video_context = SdlVideoContext::new(&sdl, SdlWindowSettings::default());
/// sdl_video_context.subsystem;
/// sdl_video_context.canvas;
/// ```
///
/// Since the stored type are just the [sdl2] types, you use them as you normally would.
pub struct SdlVideoContext {
    pub subsystem: VideoSubsystem,
    pub canvas: WindowCanvas,
}

impl SdlVideoContext {
    pub fn new(sdl: &Sdl, window_settings: SdlWindowSettings) -> Self {
        let subsystem = sdl.video().expect("Failed to crate the video subsystem");
        let window = subsystem
            .window(
                window_settings.title,
                window_settings.width,
                window_settings.height,
            )
            .position_centered()
            .build()
            .expect("Failed to create the window");
        let mut canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .expect("Failed to create the window canvas");
        canvas.set_draw_color(Color::BLACK);
        canvas.present();
        Self { subsystem, canvas }
    }
}

impl Subcontext for SdlVideoContext {}

/// Settings for creating the SDL window.
///
/// # Examples
///
/// ```
/// use wolf_engine_sdl2::*;
///
/// let window_settings = SdlWindowSettings::new("Hello, world!", 800, 600);
///
/// assert_eq!(window_settings.title, "Hello, world!");
/// assert_eq!(window_settings.width, 800);
/// assert_eq!(window_settings.height, 600);
/// ```
#[derive(Copy, Clone)]
pub struct SdlWindowSettings {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
}

impl SdlWindowSettings {
    pub fn new(title: &'static str, width: u32, height: u32) -> Self {
        Self {
            title,
            width,
            height,
        }
    }
}

impl Default for SdlWindowSettings {
    fn default() -> Self {
        Self {
            title: "Wolf Engine - Game",
            width: 800,
            height: 600,
        }
    }
}
