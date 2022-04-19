use crate::*;
use wolf_engine::*;

pub struct SdlPlugin {
    window_settings: SdlWindowSettings,
}

impl SdlPlugin {
    pub fn new(window_settings: SdlWindowSettings) -> Self {
        Self {
            window_settings
        }
    }
}

impl Plugin for SdlPlugin {
    fn setup(&mut self, engine_builder: EngineBuilder) -> PluginResult {
        let sdl_context = SdlContext::new(); 
        let sdl_video_context = SdlVideoContext::new(&sdl_context.sdl, self.window_settings); 
        Ok(engine_builder
            .with_subcontext(sdl_context)
            .with_subcontext(sdl_video_context))
    }
}

#[cfg(test)]
mod sdl2_plugin_tests {
    use super::*;

    #[test]
    fn should_load_expected_contexts() {
        let mut engine_builder = EngineBuilder::new();
        let window_settings = SdlWindowSettings::new("Test", 800, 600);
        let sdl_plugin = SdlPlugin::new(window_settings);

        engine_builder = engine_builder.with_plugin(Box::from(sdl_plugin));
        let engine = engine_builder.build();

        assert!(engine.context.borrow::<SdlContext>().is_some());
        assert!(engine.context.borrow::<SdlVideoContext>().is_some());
    }
}
