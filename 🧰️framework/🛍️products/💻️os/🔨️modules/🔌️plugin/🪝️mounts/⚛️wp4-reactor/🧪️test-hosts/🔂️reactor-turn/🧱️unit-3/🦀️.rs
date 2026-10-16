//! 🧪️ Unit 3 of the facade test host of `reactor-turn`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework::kernel::Event;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::RequestOutcome;
use semio_framework_plugin_actor_host::requests;
use semio_framework_plugin_reactor_turn as turn;

#[cfg(test)]
#[path="../../../../../⚛️reactor/🔄️turn/📥️receiving/📦️completed/🧪️tests/🦀️.rs"]
mod tests;
