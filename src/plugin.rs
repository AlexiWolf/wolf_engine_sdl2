use crate::*;
use wolf_engine::*;

pub struct SdlPlugin {
    window_settings: WindowSettings,
}

impl SdlPlugin {
    pub fn new(window_settings: WindowSettings) -> Self {
        Self {
            window_settings
        }
    }
}

impl Plugin for SdlPlugin {
    fn setup(&mut self, engine_builder: EngineBuilder) -> PluginResult {
        Ok(engine_builder
            .with_subcontext(SdlContext::new())
            .with_subcontext(SdlVideoContext::new()))
    }
}

#[cfg(test)]
mod sdl2_plugin_tests {
    use super::*;

    #[test]
    fn should_have_sdl_plugin() {
        let mut engine_builder = EngineBuilder::new();
        let window_settings = WindowSettings::new("Test", 800, 600);
        let sdl_plugin = SdlPlugin::new(window_settings);

        engine_builder = engine_builder.with_plugin(Box::from(sdl_plugin));
        let engine = engine_builder.build();

        assert!(engine.context.borrow::<SdlContext>().is_some());
        assert!(engine.context.borrow::<SdlVideoContext>().is_some());
    }
}
