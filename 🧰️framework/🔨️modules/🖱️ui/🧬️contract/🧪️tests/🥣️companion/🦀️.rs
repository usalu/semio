//! 🥣️ Companion laws of ui-contract: document compare, resident permits, assembly, typed retirement and worker retirement.
//!
//! 🧭️ The laws read the contract through its public vocabulary and the feature-gated `arena_testkit` observers; the arena gate is the companion's own, because the retained arena is process-wide.
#![cfg(test)]

pub use semio_ui_contract::*;

/// 🚧️ Serializes the tests that open, assemble or compare through the process-wide retained arena and resident ledger.
pub(crate) fn arena_test_gate() -> std::sync::MutexGuard<'static, ()> {
    static GATE: std::sync::Mutex<()> = std::sync::Mutex::new(());
    GATE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[path = "../../⚖️compare/📃️document/🧪️tests/📃️document/🦀️.rs"]
mod document_component_compare_tests;
#[path = "../../🎟️resident/🧪️tests/🎟️resident/🦀️.rs"]
mod resident_tests;
#[path = "../../📃️document/🎟️assembly/🧪️tests/🎟️assembly/🦀️.rs"]
mod document_assembly_tests;
#[path = "../../♻️retirement/🌳️typed/🧪️tests/🔬️document/🦀️.rs"]
mod typed_retirement_tests;
#[path = "../../♻️retirement/👷️worker/🧪️tests/🦀️.rs"]
mod worker_retirement_tests;
