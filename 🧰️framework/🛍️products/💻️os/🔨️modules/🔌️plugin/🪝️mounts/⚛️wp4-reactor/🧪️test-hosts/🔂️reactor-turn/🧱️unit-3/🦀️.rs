//! 🧪️ Unit 3 of the facade test host of `reactor-turn`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework::kernel::Event;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::RequestOutcome;
use semio_framework_plugin_actor_host::requests;
use semio_framework_plugin_reactor_turn as turn;

#[doc(hidden)]
#[cfg(test)]
#[path="../../../../../⚛️reactor/🔄️turn/📥️receiving/📦️completed/🧪️tests/🦀️.rs"]
pub mod tests;
#[cfg(test)]
#[path="../../../../../⚛️reactor/🔄️turn/📥️receiving/staged-tests.rs"]
mod staged_request_tests;
#[cfg(test)]
#[path="../../../../../⚛️reactor/🔄️turn/📥️receiving/auxiliary-tests.rs"]
mod staged_auxiliary_tests;
#[cfg(test)]
#[path="../../../../../⚛️reactor/🏃️runtime/🧰️ops/🖥️surface-render/staged-tests.rs"]
mod staged_context_tests;
#[cfg(test)]
#[path="../../../../../⚛️reactor/🪟️surfaces/🧪️tests/context-cursor.rs"]
mod context_cursor_tests;
#[cfg(test)]
#[path="../../../../../📜️declaration/👁️view/staged-source-tests.rs"]
mod staged_source_tests;
