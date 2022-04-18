use wolf_engine::*;

/// Provides access to SDL's [Window Canvas](Canvas) and [VideoSubsystem]. 
///
/// # Examples
///
/// ```
/// # use wolf_engine_sdl2::SdlVideoContext;
/// # let sdl_video_context = SdlVideoContext::new(SdlWindowSettings::default());
/// ```
pub struct SdlVideoContext;

impl SdlVideoContext {
    pub fn new() -> Self {
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
