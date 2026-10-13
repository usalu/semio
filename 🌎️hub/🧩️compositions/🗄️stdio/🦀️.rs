//! 🔌️ Hub Stdio deployment composition over independently compiled artifact packages.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_value_derive as value_derive;

#[path = "🔌️plugin/🦀️.rs"]
pub mod plugin;
pub use plugin::plugin;

