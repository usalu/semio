//! 🧪️ `semio-repo-test-host` — the Rust native host of the repository test platform.
//!
//! A generated, cache-local integration crate links a case's committed `🦀️component.rs` adapter and
//! calls [`run_main`]. Nothing here is generated and nothing here is production code: the `oracles`
//! feature, and only that feature, links the registered third-party reference implementations.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️component.json

//#region 🔖️Modules
#[path = "../../🧬️protocol/🦀️component.rs"]
pub mod protocol;

#[path = "../../🏃️runner/🦀️component.rs"]
pub mod runner;

#[path = "../../🔮️oracle/🦀️component.rs"]
pub mod oracle;

#[path = "../../🔮️oracle/🖼️raster/🦀️component.rs"]
pub mod oracle_raster;

#[path = "../../🔮️oracle/🎒️archive/🦀️component.rs"]
pub mod oracle_archive;

#[path = "../../🔮️oracle/🔊️audio/🦀️component.rs"]
pub mod oracle_audio;

#[path = "../../🔮️oracle/📊️tabular/🦀️component.rs"]
pub mod oracle_tabular;
//#endregion 🔖️Modules

//#region 🔖️Surface
pub use protocol::{digest, parse_json, Fixture, Json, Outcome, Plan, Scenario};
pub use runner::{run_main, Adapter, Context};
//#endregion 🔖️Surface
