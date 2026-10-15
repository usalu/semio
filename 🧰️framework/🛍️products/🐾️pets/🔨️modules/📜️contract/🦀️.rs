//! 📜️ The pets contract: the hand-written Rust twin of the normative schema `🧬️schema/🔣️.json` (menageries of rigged species, the stage, its events and its frames) and the validation of documents against the rules serde cannot express. Every other pets crate builds on these types; this crate depends on no other pets crate.
//!
//! @see ../../README.md — the domain model
//! @see ../../🧬️schema/🔣️.json — the normative contract

#[path = "../../🧬️schema/🦀️.rs"]
pub mod schema;

#[path = "../✅️validation/🦀️.rs"]
pub mod validation;

#[cfg(any(test, feature = "testkit"))]
#[path = "../../🧬️schema/🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;
