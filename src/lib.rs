pub use sdl2;
use wolf_engine::*;

pub struct SdlPlugin;

impl SdlPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SdlPlugin {
    fn setup(&mut self, engine_builder: EngineBuilder) -> PluginResult {
        Ok(engine_builder.with_subcontext(SdlContext))
    }
}

pub struct SdlContext;

impl Subcontext for SdlContext {}

pub struct SdlVideoContext;

impl Subcontext for SdlVideoContext {}

#[cfg(test)]
mod sdl2_plugin_tests {
    use super::*;

    #[test]
    fn should_have_sdl_plugin() {
        let mut engine_builder = EngineBuilder::new();
        let sdl_plugin = SdlPlugin::new();

        engine_builder = engine_builder.with_plugin(Box::from(sdl_plugin)); 
        let engine = engine_builder.build();

        assert!(engine.context.borrow::<SdlContext>().is_some());
        assert!(engine.context.borrow::<SdlVideoContext>().is_some());
    }
}
