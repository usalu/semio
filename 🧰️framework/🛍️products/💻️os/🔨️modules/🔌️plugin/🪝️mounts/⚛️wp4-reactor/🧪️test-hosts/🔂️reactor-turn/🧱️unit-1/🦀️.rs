//! 🧪️ Unit 1 of the facade test host of `reactor-turn`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

#[cfg(test)]
#[path = "../../../../../⚛️reactor/🔄️turn/🧪️tests/⏱️execution/🦀️.rs"]
mod turn_execution_tests;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🔄️turn/🧪️tests/📥️inbound-request/🦀️.rs"]
mod inbound_request_tests;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🔄️turn/🧪️tests/📡️wire-effect-round-trip/🦀️.rs"]
mod wire_effect_laws;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🔄️turn/🧪️tests/🕹️deferred-render-drain/🦀️.rs"]
mod deferred_render_drain_laws;

#[cfg(test)]
#[path = "../../../../../⚛️reactor/🔄️turn/🧪️tests/⚖️invocation-failure/🦀️.rs"]
mod invocation_failure_laws;
