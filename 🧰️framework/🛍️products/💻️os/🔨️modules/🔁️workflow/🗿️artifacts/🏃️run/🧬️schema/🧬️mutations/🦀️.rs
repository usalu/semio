use super::{RunArtifact, RunDiff};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Leaves
#[path = "🚀️start-run/🦀️.rs"]
mod start_run;
#[path = "▶️start-run-node/🦀️.rs"]
mod start_run_node;
#[path = "✅️finish-run-node/🦀️.rs"]
mod finish_run_node;
#[path = "🪵️append-run-log/🦀️.rs"]
mod append_run_log;
#[path = "🔏️seal-run/🦀️.rs"]
mod seal_run;

pub use append_run_log::AppendRunLog;
pub use finish_run_node::FinishRunNode;
pub use seal_run::SealRun;
pub use start_run::StartRun;
pub use start_run_node::StartRunNode;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, dsl::DslOps)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = RunArtifact, diff = RunDiff, schema = "os.run")]
pub enum RunMutation {
    StartRun(StartRun),
    StartRunNode(StartRunNode),
    FinishRunNode(FinishRunNode),
    AppendRunLog(AppendRunLog),
    SealRun(SealRun),
}
//#endregion 🔖️Aggregate

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, SemanticMutation};

    #[test]
    fn descriptors_follow_the_canonical_run_roster() {
        assert_eq!(RunMutation::kinds().iter().map(|value| value.kind).collect::<Vec<_>>(), vec!["start-run", "start-run-node", "finish-run-node", "append-run-log", "seal-run"]);
        assert_eq!(<RunMutation as Mutation<RunArtifact>>::DESCRIPTORS.iter().map(|value| value.binary_tag).collect::<Vec<_>>(), vec![Some(0), Some(1), Some(2), Some(3), Some(4)]);
    }
}
//#endregion 🧪️Tests
