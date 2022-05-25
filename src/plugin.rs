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
///     // Custom settings go here.
///     .build();
/// ```
pub struct SdlPlugin {
    window_settings: SdlWindowSettings,
}

impl SdlPlugin {
    fn new(window_settings: SdlWindowSettings) -> Self {
        Self { window_settings }
    }

    pub fn builder() -> SdlPluginBuilder {
        SdlPluginBuilder::new()
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
        let sdl_plugin = SdlPlugin::default();

        engine_builder = engine_builder.with_plugin(Box::from(sdl_plugin));
        let engine = engine_builder.build();

        assert!(engine.context.borrow::<SdlContext>().is_some());
        assert!(engine.context.borrow::<SdlVideoContext>().is_some());
    }
}

pub struct SdlPluginBuilder {
    window_settings: SdlWindowSettings,
    
    #[cfg(feature = "mixer")]
    mixer_settings: MixerSettings,
}

impl SdlPluginBuilder {
    pub fn new() -> Self {
        Self {
            window_settings: Default::default(),
            #[cfg(feature = "mixer")]
            mixer_settings: Default::default(),
        }
    }

    pub fn build(self) -> SdlPlugin {
        SdlPlugin::new(self.window_settings)
    }

    pub fn with_window_settings(mut self, window_settings: SdlWindowSettings) -> Self {
        self.window_settings = window_settings;
        self
    }
}

#[cfg(feature = "mixer")]
impl SdlPluginBuilder {
    pub fn with_mixer_settings(mut self, mixer_settings: MixerSettings) -> Self {
        self.mixer_settings = mixer_settings;
        self
    }
}

#[cfg(test)]
mod sdl2_plugin_builder_tests {
    use super::*;

    #[test]
    fn should_set_window_settings() {
        let window_settings = SdlWindowSettings::new("Test Window", 100, 100);
        let builder = SdlPlugin::builder().with_window_settings(window_settings);

        assert_eq!(
            window_settings, builder.window_settings,
            "The window settings do not match"
        );

        let plugin = builder.build();

        assert_eq!(
            window_settings, plugin.window_settings,
            "The plugin was not built with the correct window settings."
        );
    }

    #[test]
    #[cfg(feature = "mixer")]
    fn should_set_mixer_settings() {
        use sdl2::mixer;

        let mixer_settings = MixerSettings {
            frequency: 48_000,
            format: mixer::AUDIO_U8,
            channels: 2,
            chunk_size: 200,
            allocate_channels: 4,
            init_flag: mixer::InitFlag::all(),
        };
        let builder = SdlPlugin::builder().with_mixer_settings(mixer_settings);

        assert_eq!(mixer_settings, builder.mixer_settings, "The mixer settings were not set correctly.");
    }
}
