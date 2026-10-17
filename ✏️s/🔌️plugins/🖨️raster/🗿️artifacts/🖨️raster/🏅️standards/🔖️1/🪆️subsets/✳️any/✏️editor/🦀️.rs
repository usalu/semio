//! 🖥️ Raster editor — the `ArtifactEditor` app of the raster artifact, composed from its session, mode, panel and command crates.
//!
//! 🔗 This root is wiring only: the app, its retained command routes, the host action bridge, the media export, the store preparation, the reserved import and the app assembly each live in their own taxonomy node, and the lower layers arrive from their own crates under their original module names.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

pub use semio_s_artifact_raster_raster_editor_commands::{commands, RasterCommand};
pub use semio_s_artifact_raster_raster_editor_modes::modes;
pub use semio_s_artifact_raster_raster_editor_panels::panels;
pub use semio_s_artifact_raster_raster_editor_session::*;

//#region 🪢️TaxonomyMounts
#[path = "🖥️app/🦀️.rs"]
mod app;
#[path = "🔌️app-assembly/🦀️.rs"]
mod app_assembly;
#[path = "🔁️action-bridge/🦀️.rs"]
mod args_bridge;
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[path = "📤️export/🦀️.rs"]
pub mod media_export;
#[path = "🎞️reserved-import/🦀️.rs"]
mod reserved_import;
#[path = "🧵️retained-commands/🦀️.rs"]
mod retained_commands;
#[path = "📬️store-preparation/🦀️.rs"]
mod store_preparation;
#[path = "🌉️wasm/🦀️.rs"]
pub mod wasm;
//#endregion 🪢️TaxonomyMounts

pub use app::*;
pub use app_assembly::*;
pub use retained_commands::*;
use reserved_import::*;
use store_preparation::*;
