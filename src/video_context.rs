use wolf_engine::*;
use sdl2::{Sdl, VideoSubsystem, render::WindowCanvas};

/// Provides access to [sdl2]'s [WindowCanvas] and [VideoSubsystem]. 
///
/// # Examples
///
/// ```
/// # use wolf_engine_sdl2::*;
/// # let sdl = sdl2::init().expect("Failed to initialize SDL");
/// # let sdl_video_context = SdlVideoContext::new(&sdl, SdlWindowSettings::default());
/// sdl_video_context.video;
/// sdl_video_context.canvas;
/// ```
///
/// Since the stored type are just the [sdl2] types, you use them as you normally would.
pub struct SdlVideoContext {
    pub video: VideoSubsystem,
    pub canvas: WindowCanvas,
}

impl SdlVideoContext {
    pub fn new(sdl: &Sdl, window_settings: SdlWindowSettings) -> Self {
        Self {
            video: todo!(),
            canvas: todo!(),
        }
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
            height: 600 
        }
    }
}
