//! ⚙️ The ArtifactStore composer: use-case methods, delegates and the store-close host implementation.
//!
//! 🧭️ Layering and decomposition: `PLAN-OS-KERNEL-CRATES.md` sections 6 and 7 (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`).

#[path = "../🧩️composition/🌱️initial/🪪️identity/🦀️.rs"]
pub mod initial_child_identity;
#[path = "../🧩️composition/📬️publication/🤝️group/🦀️.rs"]
pub mod member_group;
#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;
