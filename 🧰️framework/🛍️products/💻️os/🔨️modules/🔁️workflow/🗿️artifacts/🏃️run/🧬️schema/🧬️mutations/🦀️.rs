use super::{RunArtifact, RunDiff};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Leaves
#[path = "🪵️append-run-log/🦀️.rs"]
mod append_run_log;
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

impl protocol::OpText for RunMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6) — `DslOps` emits `DslVariants` only.
impl protocol::OpBinary for RunMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
