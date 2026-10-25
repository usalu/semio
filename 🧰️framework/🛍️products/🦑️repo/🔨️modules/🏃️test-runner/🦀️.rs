//! 🏃️ Repo test runner: which tests a scope selects, which processes that becomes, and what the
//! runners answered.
//!
//! The domain is split into three layers that never leak into each other:
//! 1. **Planning is pure.** A [`FilesystemSnapshot`] plus a [`TestScope`] deterministically yields an
//!    ordered [`InvocationPlan`]. No process, no clock, no ambient filesystem — so a plan is a
//!    golden value a fixture can pin.
//! 2. **Execution is a port.** [`ProcessRunner`] is the only way a plan reaches an operating system,
//!    which is what lets [`RecordedProcessRunner`] replay a committed transcript byte for byte.
//! 3. **Parsing is per runner.** Each runner's native report is folded into one [`TestOutcome`], so a
//!    caller never learns which of the six report dialects produced a result.
//!
//! Behavioural reference: the Go `🕸️Test Command` and `🖲️Missing Test Functions` regions of
//! `💻️client/⌨️cli/🧩️component.go` as frozen in this ticket's snapshot. Every planning decision here
//! reproduces that source; the execution and parsing layers are net new (Go streams runner output
//! straight to the terminal and never parses it).

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Children

#[path = "🧮️paths-and-identity/🦀️.rs"]
mod paths_and_identity;
pub use paths_and_identity::*;

#[path = "🌲️snapshot-and-scope/🦀️.rs"]
mod snapshot_and_scope;
pub use snapshot_and_scope::*;

#[path = "🗺️planning/🦀️.rs"]
mod planning;
pub use planning::*;

#[path = "⚙️execution-and-parsing/🦀️.rs"]
mod execution_and_parsing;
pub use execution_and_parsing::*;

#[path = "🔎️resolution/🦀️.rs"]
mod resolution;
pub use resolution::*;

#[path = "💽️real-world/🦀️.rs"]
mod real_world;
pub use real_world::*;

//#endregion 🔖️Children

/// 🗺️ Renders a plan as JSON text — concrete on purpose, so no serialization type crosses the API.
pub fn plan_to_json_text(plan: &InvocationPlan) -> String {
    serde_json::to_string(plan).unwrap_or_else(|error| format!("{{\"error\":{:?}}}", error.to_string()))
}

/// 🔭️ Renders a scope as JSON text.
pub fn scope_to_json_text(scope: &TestScope) -> String {
    serde_json::to_string(scope).unwrap_or_else(|error| format!("{{\"error\":{:?}}}", error.to_string()))
}

/// 🔣️ Renders one outcome as JSON text.
pub fn outcome_to_json_text(outcome: &TestOutcome) -> String {
    serde_json::to_string(outcome).unwrap_or_else(|error| format!("{{\"error\":{:?}}}", error.to_string()))
}

/// 📊️ Renders a whole execution report as JSON text.
pub fn report_to_json_text(report: &ExecutionReport) -> String {
    serde_json::to_string(report).unwrap_or_else(|error| format!("{{\"error\":{:?}}}", error.to_string()))
}

/// 📣️ Renders a progress event stream as JSON text.
pub fn progress_to_json_text(events: &[ProgressEvent]) -> String {
    serde_json::to_string(events).unwrap_or_else(|error| format!("{{\"error\":{:?}}}", error.to_string()))
}

//#region 🧪️Tests

#[cfg(test)]
#[path = "./🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
