//! 🧪️ Unit 1 of the facade test host of `reactor-turn`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use std::cell::Cell;
use semio_framework::kernel::Event;
use semio_framework::kernel::MessageEndpoint;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::RequestOutcome;
use semio_framework_plugin_reactor_lifecycle::instance_lifetime;
use semio_framework_plugin_reactor_patches as patches;
use semio_framework_plugin_reactor_lifecycle as pending;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework_plugin_reactor_wit_bridge::wit_bridge::poll;
use semio_framework_plugin_reactor_turn as turn;
use semio_framework_ui_contract as ui_contract;

#[cfg(test)]
pub(super) fn charge_turn_execution_us(spent_us: u64) {
    TURN_EXECUTION.with(|execution| {
        let mut current = execution.get();
        current.accumulated_us = current.accumulated_us.saturating_add(spent_us);
        execution.set(current);
    });
}
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🔄️turn/🧪️tests/⏱️execution/🦀️.rs"]
mod turn_execution_tests;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🔄️turn/🧪️tests/📥️inbound-request/🦀️.rs"]
mod inbound_request_tests;
#[cfg(test)]
#[path="../../../../../⚛️reactor/🔄️turn/🎟️receipt/🧪️tests/🦀️.rs"]
mod publication_receipt_tests;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🔄️turn/🧪️tests/📡️wire-effect-round-trip/🦀️.rs"]
mod wire_effect_laws;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🔄️turn/🧪️tests/🕹️deferred-render-drain/🦀️.rs"]
mod deferred_render_drain_laws;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🔄️turn/🧪️tests/⚖️invocation-failure/🦀️.rs"]
mod invocation_failure_laws;
