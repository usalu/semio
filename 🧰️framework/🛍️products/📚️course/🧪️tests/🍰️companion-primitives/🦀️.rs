//! 🍰️ Companion tests of the quiz primitives: the Mersenne Twister with its shuffle and the presence tags.
//!
//! 🧭️ Each module's unit file is mounted below a wrapper module of the same name.
#![cfg(test)]

pub use course::*;

#[path = "🎲️randomness/🦀️.rs"]
pub mod randomness;

#[path = "👥️presence/🦀️.rs"]
pub mod presence;
