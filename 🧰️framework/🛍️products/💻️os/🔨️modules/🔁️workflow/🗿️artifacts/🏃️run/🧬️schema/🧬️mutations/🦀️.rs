use super::{RunArtifact, RunDiff};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Leaves
#[path = "🪵️append-run-log/🦀️.rs"]
mod append_run_log;
#[path = "🫥️remove-run-node/🦀️.rs"]
mod remove_run_node;
#[path = "🧽️remove-run-log/🦀️.rs"]
mod remove_run_log;
#[path = "🧷️set-run-header/🦀️.rs"]
mod set_run_header;
#[path = "🔓️set-run-seal/🦀️.rs"]
mod set_run_seal;
#[path = "✅️finish-run-node/🦀️.rs"]
mod finish_run_node;
#[path = "🔏️seal-run/🦀️.rs"]
mod seal_run;
#[path = "🚀️start-run/🦀️.rs"]
mod start_run;
#[path = "▶️start-run-node/🦀️.rs"]
mod start_run_node;

pub use append_run_log::AppendRunLog;
pub use finish_run_node::FinishRunNode;
pub use remove_run_log::RemoveRunLog;
pub use remove_run_node::RemoveRunNode;
pub use set_run_header::SetRunHeader;
pub use set_run_seal::SetRunSeal;
pub use seal_run::SealRun;
pub use start_run::StartRun;
pub use start_run_node::StartRunNode;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = RunArtifact, diff = RunDiff, schema = "os.run")]
pub enum RunMutation {
    StartRun(StartRun),
    StartRunNode(StartRunNode),
    FinishRunNode(FinishRunNode),
    AppendRunLog(AppendRunLog),
    SealRun(SealRun),
    SetRunHeader(SetRunHeader),
    SetRunSeal(SetRunSeal),
    RemoveRunLog(RemoveRunLog),
    RemoveRunNode(RemoveRunNode),
}
//#endregion 🔖️Aggregate

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests




