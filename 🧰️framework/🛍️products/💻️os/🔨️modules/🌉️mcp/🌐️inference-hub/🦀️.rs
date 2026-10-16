//! 🌐️ Inference job wire, hub inference client and bindings.

#[path = "../💡️inference/💼️jobs/📡️hub-wire/🦀️.rs"]
pub mod hub_wire;

#[cfg(test)]
#[path = "../🧬️schema/🧪️tests/🔬️quick/🦀️.rs"]
pub mod schema_quick;

#[path = "../💡️inference/🔌️service/🦀️.rs"]
pub mod service;

#[cfg(test)]
#[path = "../💡️inference/🔌️service/🧪️tests/🦀️.rs"]
pub mod service_tests;
