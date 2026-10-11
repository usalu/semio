use super::super::{RemoveRunLog, RunArtifact, RunDiff, RunLogLine, RunMutation, RunStep};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "start-run-node")]
pub struct StartRunNode {
    pub node_id: String,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<RunArtifact, RunMutation> for StartRunNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "start", entity: "run-node", kind: "start-run-node", record: "StartedRunNode" };
    fn diff(&self, _base: &RunArtifact) -> protocol::MutationOutcome<RunDiff> {
        protocol::MutationOutcome::new(RunDiff::step(RunStep::LogAppend(RunLogLine { node_id: self.node_id.clone(), level: "info".into(), message: "node started".into(), at: store::now_iso() })))
    }
    fn inverse(&self, _base: &RunArtifact) -> Result<Vec<RunMutation>, semio_framework_value::ValueError> {
        Ok(vec![RunMutation::RemoveRunLog(RemoveRunLog { count: 1 })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Start run node {}", self.node_id), &format!("Laufknoten {} starten", self.node_id))
    }
    fn target(&self) -> Vec<String> {
        vec!["nodes".into(), self.node_id.clone()]
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
