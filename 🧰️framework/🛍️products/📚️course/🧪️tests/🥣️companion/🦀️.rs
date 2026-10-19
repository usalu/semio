//! 🥣️ Companion tests of the quiz product: sheet assembly, validation, scoring, badges and the challenge rules.
//!
//! 🧭️ Each module's unit file is mounted below a wrapper module of the same name; the fixtures shared with the run laws live in `semio-framework-course-tests-support`.
#![cfg(test)]

pub use course::*;

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
