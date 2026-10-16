//! 🏛️ Shard laws: the loop, lifecycle, identity, transport and executor laws over the real shard crates.

#[cfg(test)]
#[path = "../🔬️unit/🦀️.rs"]
mod unit;

#[cfg(test)]
#[path = "../../🔁️lifecycle/🧪️tests/🔁️lifecycle/🦀️.rs"]
mod lifecycle_tests;

#[cfg(test)]
#[path = "../../🪪️identity/🧪️tests/🦀️.rs"]
mod identity_tests;

#[cfg(test)]
#[path = "../../🚚️process-transport/🧪️tests/🔬️unit/🦀️.rs"]
mod process_transport_tests;
