//! 🧪️ Unit 2 of the facade test host of `reactor-turn`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework::kernel::Event;
use std::mem::ManuallyDrop;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::RequestOutcome;
use semio_framework_plugin_reactor_lifecycle::instance_lifetime;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework_plugin_reactor_wit_bridge::wit_bridge::poll;
use semio_framework_plugin_reactor_turn as turn;

#[cfg(test)]
#[path="../../../../../⚛️reactor/🔄️turn/📥️receiving/🧪️tests/🦀️.rs"]
mod tests;
