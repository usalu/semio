//! 🥣️ Companion tests of the quiz product: sheet assembly, validation, scoring, badges, the learner lifecycle, the read models and the challenge rules.
//!
//! 🧭️ Each module's unit file is mounted below a wrapper module of the same name, so the laws that share fixtures through `crate::sheet::tests` and `crate::lifecycle::tests` keep their paths.
#![cfg(test)]

pub use quiz::*;

#[path = "⛰️challenge/🦀️.rs"]
pub mod challenge;

#[path = "🃏️sheet/🦀️.rs"]
pub mod sheet;

#[path = "✅️validation/🦀️.rs"]
pub mod validation;

#[path = "📏️scoring/🦀️.rs"]
pub mod scoring;

#[path = "🏅️badges/🦀️.rs"]
pub mod badges;

#[path = "🧾️lifecycle/🦀️.rs"]
pub mod lifecycle;

#[path = "👁️views/🦀️.rs"]
pub mod views;
