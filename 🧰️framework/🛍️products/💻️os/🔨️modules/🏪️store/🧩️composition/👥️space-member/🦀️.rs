//! 👥️ SpaceMember, MemberFactory, member directory, link resolver and the unsupported open stubs.
//!
//! 🧭️ Layering and decomposition: `PLAN-OS-KERNEL-CRATES.md` sections 6 and 7 (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`).

#[path = "../🚪️open/📜️history/🏭️factory/🦀️.rs"]
pub mod factory;
#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
#[path = "📥️lifted/🏭️operation/🦀️.rs"]
pub mod lifted_operation;
#[path = "📥️lifted/🧾️document/🦀️.rs"]
pub mod lifted_document_materialize;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;
