use wolf_engine::*;

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
pub struct SdlWindowSettings;

impl SdlWindowSettings {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self 
    }
}
