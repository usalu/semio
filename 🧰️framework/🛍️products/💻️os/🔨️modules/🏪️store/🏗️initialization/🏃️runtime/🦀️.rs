//! 🏃️ The Store initialization runtime: digest, edit index, owner catalog and clock helpers.
//!
//! 🧭️ Layering and decomposition: `PLAN-OS-KERNEL-CRATES.md` sections 6 and 7 (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`).

#[path = "../📚️catalog/🦀️.rs"]
pub mod initialization_catalog_close;
#[path = "../📣️publication/🧬️schema/🦀️.rs"]
pub mod native_publication_producer;
#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
#[path = "📥️lifted/🔁️replay/🦀️.rs"]
pub mod lifted_initialization_replay;
#[path = "📥️lifted/🧾️document/🦀️.rs"]
pub mod lifted_document_materialize;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;
