pub(crate) fn log_sdl_version() {
    let sdl_version = sdl2::version::version();
    log::info!("Using SDL v{}", sdl_version);
}
