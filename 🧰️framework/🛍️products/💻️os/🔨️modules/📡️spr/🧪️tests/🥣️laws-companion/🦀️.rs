//! 🥣️ Companion tests of os-spr: protocol laws and mutation laws.
//!
//! 🧭️ The tests read the law helpers of `semio_framework_os_spr::protocol_laws` through its public API and the facade macros through `semio_framework_os_kernel`.
#![cfg(test)]

pub use semio_framework_os_spr::protocol_laws::*;

#[path = "../🧬️mutation-laws/🦀️.rs"]
mod mutation_law_fixture;

#[path = "../⚖️protocol-laws-unit/🦀️.rs"]
mod tests;
