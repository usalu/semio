//! 🎞️ Companion tests of the quiz product: the learner lifecycle decide/evolve laws and the proctor read-model laws.
//!
//! 🧭️ Each module's unit file is mounted below a wrapper module of the same name; the fixtures both share live in `semio-framework-course-tests-support`.
#![cfg(test)]

pub use course::*;

#[path = "🧾️lifecycle/🦀️.rs"]
pub mod lifecycle;

#[path = "👁️views/🦀️.rs"]
pub mod views;
