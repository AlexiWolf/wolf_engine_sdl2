use crate::*;
use wolf_engine::*;

/// Provides [sdl2] integrations for [wolf_engine].
/// 
/// # Examples
///
/// If you want the default settings, you can use [SdlPlugin::default()]
///
/// ```
/// # use wolf_engine_sdl2::*;
/// #
/// let sdl = SdlPlugin::default();
/// ```
///
/// To use custom settings, use the [SdlPluginBuilder].
///
/// ```
/// # use wolf_engine_sdl2::*;
/// #
/// let sdl = SdlPlugin::builder()
///     .build()
///     .expect("Failed to initialize SDL2");
/// ```
pub struct SdlPlugin {
    window_settings: SdlWindowSettings,
}

impl SdlPlugin {
    pub fn new(window_settings: SdlWindowSettings) -> Self {
        Self { window_settings }
    }
}

impl Default for SdlPlugin {
    fn default() -> Self {
        Self::new(SdlWindowSettings::default())
    }
}

impl Plugin for SdlPlugin {
    fn setup(&mut self, mut engine_builder: EngineBuilder) -> PluginResult {
        let sdl_context = SdlContext::new();
        let sdl_video_context = SdlVideoContext::new(&sdl_context.sdl, self.window_settings);
        let sdl_audio_context = SdlAudioContext::new(&sdl_context.sdl);
        engine_builder = engine_builder
            .with_subcontext(sdl_context)
            .with_subcontext(sdl_video_context)
            .with_subcontext(sdl_audio_context)
            .with_engine_core(Box::from(run_with_sdl));
        #[cfg(feature = "mixer")]
        {
            engine_builder =
                engine_builder.with_subcontext(SdlMixerContext::new(MixerSettings::default()));
        }
        Ok(engine_builder)
    }
}

#[cfg(test)]
mod sdl2_plugin_tests {
    use super::*;

    #[test]
    fn should_load_expected_contexts() {
        let mut engine_builder = EngineBuilder::new();
        let window_settings = SdlWindowSettings::default();
        let sdl_plugin = SdlPlugin::new(window_settings);

        engine_builder = engine_builder.with_plugin(Box::from(sdl_plugin));
        let engine = engine_builder.build();

        assert!(engine.context.borrow::<SdlContext>().is_some());
        assert!(engine.context.borrow::<SdlVideoContext>().is_some());
    }
}
