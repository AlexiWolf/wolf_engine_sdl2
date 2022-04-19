use wolf_engine::*;

/// Provides access to SDL's [Window Canvas](Canvas) and [VideoSubsystem]. 
///
/// # Examples
/// 
/// The Video Context allows you to access SDL's [VideoSubsystem] and [Canvas].
/// ```
/// # use wolf_engine_sdl2::*;
/// # let sdl_video_context = SdlVideoContext::new(SdlWindowSettings::default());
/// // Get immutable access to the video subsystem and canvas.
/// let video_subsystem = sdl_video_context.video();
/// let window_canvas = sdl_video_context.canvas();
/// # drop(video_subsystem);
/// # drop(window_canvas);
/// // Get mutable access to the video subsystem and canvas.
/// let video_subsystem_mut = sdl_video_context.video_mut();
/// let window_canvas = sdl_video_context.canvas_mut();
/// ```
///
/// From there, you can use SDL as you would normally.
pub struct SdlVideoContext;

impl SdlVideoContext {
    pub fn new(window_settings: SdlWindowSettings) -> Self {
        Self
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
