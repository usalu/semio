//! 🫐️ Companion test crate tests-20 of the plugin SDK partition: hosts the retained-command unit tests, which use only the crate's public surface.
//!
//! 🧭️ The test source stays where it is and is mounted here with `#[path]`; `crate::` in it names the retained-command public items re-exported at this root.
#![cfg(test)]

pub use semio_plugin_retained_command::*;

#[path = "../../🧵️retained-command/🧪️tests/🔬️unit/🦀️.rs"]
mod unit;
