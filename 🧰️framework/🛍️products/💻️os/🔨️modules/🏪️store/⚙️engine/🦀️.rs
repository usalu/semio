//! ⚙️ The ArtifactStore composer: use-case methods, delegates and the store-close host implementation.
//!
//! 🧭️ Layering and decomposition: `PLAN-OS-KERNEL-CRATES.md` sections 6 and 7 (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`).

#[path = "../🧩️composition/🌱️initial/🪪️identity/🦀️.rs"]
pub mod initial_child_identity;
#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
#[path="🌿️branch/🦀️.rs"]
pub mod original_history_branch;
pub use original_history_branch::transitions as original_history_transitions;
#[path="../🕰️history/🏁️replay/📚️metadata/🦀️.rs"]
pub mod original_replay_metadata;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;

/// 🧪️ Library unit tests observe every physical birth and release through the shared heap witness.
#[cfg(test)]
#[global_allocator]
static ENGINE_LIBRARY_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[path="../🕰️history/🏁️replay/🦀️.rs"]
pub mod original_replay_publication;
