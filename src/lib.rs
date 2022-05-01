//! Provides [sdl2] integrations for [wolf_engine].
//!
//! # Example
//!
//! Load the [SdlPlugin] with the [EngineBuilder](wolf_engine::EngineBuilder).
//!
//! ```
//! use wolf_engine::*;
//! use wolf_engine_sdl2::*;
//!
//! # let my_game = EmptyState;
//! EngineBuilder::new()
//!     .with_plugin(Box::from(SdlPlugin::new(SdlWindowSettings::default())))
//!     .build()
//!     .run(Box::from(my_game));
//! ```
//!
//! SDL Subcontexts can be accessed through the [Context](wolf_engine::Context) object.
//!
//! ```
//! # use wolf_engine::*;
//! # use wolf_engine_sdl2::*;
//! #
//! # let context = Context::new();
//! #
//! if let Some(Ok(sdl)) = context.try_borrow::<SdlContext>() {
//!     // Do something cool.
//! };
//!
//! if let Some(Ok(sdl_video)) = context.try_borrow_mut::<SdlVideoContext>() {
//!     // Do something cool.
//! };
//! ```

mod context;
mod core_function;
mod plugin;
mod video_context;
pub use context::*;
pub(crate) use core_function::*;
pub use plugin::*;
pub use video_context::*;

pub(crate) fn log_sdl_version() {
    let sdl_version = sdl2::version::version();
    log::info!("Using SDL v{}", sdl_version);
}
