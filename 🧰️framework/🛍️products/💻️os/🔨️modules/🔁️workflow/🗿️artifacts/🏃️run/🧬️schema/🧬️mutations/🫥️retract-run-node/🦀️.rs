use super::super::{FinishRunNode, RunArtifact, RunDiff, RunMutation, RunStep};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
/// 🫥️ Retracts one run node record and is the inverse of `finish-run-node` for a node that had no record.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "retract-run-node")]
pub struct RetractRunNode {
    pub node_id: String,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<RunArtifact, RunMutation> for RetractRunNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "retract", entity: "run-node", kind: "retract-run-node", record: "RetractedRunNode" };
    fn diff(&self, base: &RunArtifact) -> protocol::MutationOutcome<RunDiff> {
        if !base.node_records.iter().any(|entry| entry.node_id == self.node_id) {
            return protocol::MutationOutcome::error("mutation.target-missing", "run node record does not exist", ["nodes", self.node_id.as_str()]);
        }
        protocol::MutationOutcome::new(RunDiff::step(RunStep::Node { node_id: self.node_id.clone(), record: None }))
    }
    fn inverse(&self, base: &RunArtifact) -> Result<Vec<RunMutation>, semio_framework_value::ValueError> {
        Ok(base.node_records.iter().find(|entry| entry.node_id == self.node_id).map(|node_record| vec![RunMutation::FinishRunNode(FinishRunNode { node_record: node_record.clone() })]).unwrap_or_default())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Retract run node {}", self.node_id), &format!("Laufknoten {} zurücknehmen", self.node_id))
    }
    fn target(&self) -> Vec<String> {
        vec!["nodes".into(), self.node_id.clone()]
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
