//! 📥️ Retained load laws.
pub use semio_framework_plugin_window_config::retained::*;
pub use super::*;
use semio_framework_plugin_window_config::{WindowConfigPartition};
use std::mem::ManuallyDrop;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};
#[path = "📦️typed-pack-load/🦀️.rs"]
pub mod typed_pack_load;
#[path = "🎞️command/🦀️.rs"]
mod command;
