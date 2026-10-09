use super::super::{AppendRunLog, RunArtifact, RunDiff, RunMutation, RunStep};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
/// 🧽️ Removes the last `count` run log lines and is the inverse of `append-run-log` and `start-run-node`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "remove-run-log")]
pub struct RemoveRunLog {
    pub count: u64,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<RunArtifact, RunMutation> for RemoveRunLog {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "run-log", kind: "remove-run-log", record: "RemovedRunLog" };
    fn diff(&self, base: &RunArtifact) -> protocol::MutationOutcome<RunDiff> {
        if self.count == 0 {
            return protocol::MutationOutcome::empty();
        }
        if usize::try_from(self.count).map_or(true, |count| count > base.logs.len()) {
            return protocol::MutationOutcome::error("mutation.target-missing", "run log has fewer lines than the retraction names", ["logs"]);
        }
        protocol::MutationOutcome::new(RunDiff::step(RunStep::LogRetract { count: self.count }))
    }
    fn inverse(&self, base: &RunArtifact) -> Result<Vec<RunMutation>, semio_framework_value::ValueError> {
        let kept = base.logs.len().saturating_sub(usize::try_from(self.count).unwrap_or(usize::MAX));
        Ok(base.logs[kept..].iter().rev().map(|line| RunMutation::AppendRunLog(AppendRunLog { node_id: line.node_id.clone(), level: line.level.clone(), message: line.message.clone(), at: line.at.clone() })).collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove {} run log lines", self.count), &format!("{} Laufprotokollzeilen zurücknehmen", self.count))
    }
    fn target(&self) -> Vec<String> {
        vec!["logs".into()]
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
