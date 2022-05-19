//! Provides [sdl2] integrations for [wolf_engine].
//!
//! # Installation
//!
//! Add this package, and [sdl2] to your project's dependencies:
//!
//! ```ignore
//! wolf_engine_sdl2 = "*"
//! sdl2 = "*"
//! ```
//!
//! # Example
//!
//! The main job of this package is to provide thin [Subcontext](wolf_engine::Subcontext)
//! wrappers around the normal [sdl2] types, and a [Plugin](wolf_engine::Plugin) to set
//! everything up, allowing [wolf_engine] to manage sdl2 for you.  The [sdl2] objects are
//! accessible through public fields on the [Subcontexts](wolf_engine::Subcontext), and
//! their normal usage is not changed.
//!
//! Load the [SdlPlugin] with the [EngineBuilder](wolf_engine::EngineBuilder) at startup.
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
//! if let Some(Ok(mut sdl_video)) = context.try_borrow_mut::<SdlVideoContext>() {
//!     sdl_video.canvas.clear();
//!     sdl_video.canvas.present();
//! };
//! ```
//!
//! For a more complete usage example, see the
//! [Quickstart Example](https://github.com/AlexiWolf/wolf_engine_sdl2/tree/main/examples/quickstart).

mod context;
mod core_function;
mod plugin;
mod audio_context;
mod video_context;

pub use context::*;
pub(crate) use core_function::*;
pub use plugin::*;
pub use audio_context::*;
pub use video_context::*;

pub(crate) fn log_sdl_version() {
    let sdl_version = sdl2::version::version();
    log::info!("Using SDL v{}", sdl_version);
}
