//! 🧪️ `semio-repo-test-host` — the Rust native host of the repository test platform.
//!
//! A generated, cache-local integration crate links a case's committed `🦀️.rs` adapter and
//! calls [`run_main`]. This crate is DOMAIN-NEUTRAL and dependency-free: it knows about plans,
//! results, fixtures and adapters, and about no file format, plugin or product whatsoever.
//!
//! Reference implementations live with the owner of the format they reference and are contributed
//! through that owner's `🔣️oracle.json` manifest, which the platform discovers by
//! convention. Adding an artifact family therefore never edits this crate.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json

//#region 🔖️Modules
#[path = "../../🧬️protocol/🦀️.rs"]
pub mod protocol;

#[path = "../../🏃️runner/🦀️.rs"]
pub mod runner;
//#endregion 🔖️Modules

//#region 🔖️Surface
pub use protocol::{digest, parse_json, sha256_hex, Fixture, Json, Outcome, Plan, ProductionDispatch, ResultArtifact, Scenario, SubsetTarget};
pub use runner::{run_main, Adapter, Context};
//#endregion 🔖️Surface
