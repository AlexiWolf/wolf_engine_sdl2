pub use sdl2;

#[cfg(test)]
mod sdl2_plugin_tests {
    use wolf_engine::EngineBuilder;

    #[test]
    fn should_have_sdl_plugin() {
        let engine_builder = EngineBuilder::new();
        let sdl_plugin = SdlPlugin::new();
    }
}
