//! 🏠️ SpaceHost, member store creation and opening, and the SQLite snapshot registration glue.
//!
//! 🧭️ Layering and decomposition: `PLAN-OS-KERNEL-CRATES.md` sections 6 and 7 (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`).

#[path = "📣️registration/🦀️.rs"]
pub mod space_history_registration;
#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;
