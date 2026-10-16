//! 🎚️ The config store: config envelope, hydration and apply.
//!
//! 🧭️ Layering and decomposition: `PLAN-OS-KERNEL-CRATES.md` sections 6 and 7 (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`).

#[path = "📥️retained/🦀️.rs"]
pub mod retained_config_hydration;
#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;
