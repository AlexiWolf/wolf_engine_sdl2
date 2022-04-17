pub use sdl2;

pub struct SdlPlugin;

impl SdlPlugin {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod sdl2_plugin_tests {
    use super::*;
    use wolf_engine::*;

    #[test]
    fn should_have_sdl_plugin() {
        let engine_builder = EngineBuilder::new();
        let sdl_plugin = SdlPlugin::new();
    }
}
