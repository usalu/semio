use super::super::{RemoveRunNode, RunArtifact, RunDiff, RunMutation, RunNodeRecord, RunStep};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "finish-run-node")]
pub struct FinishRunNode {
    pub node_record: RunNodeRecord,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<RunArtifact, RunMutation> for FinishRunNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "finish", entity: "run-node", kind: "finish-run-node", record: "FinishedRunNode" };
    fn diff(&self, _base: &RunArtifact) -> protocol::MutationOutcome<RunDiff> {
        protocol::MutationOutcome::new(RunDiff::step(RunStep::Node { node_id: self.node_record.node_id.clone(), record: Some(self.node_record.clone()) }))
    }
    fn inverse(&self, base: &RunArtifact) -> Result<Vec<RunMutation>, semio_framework_value::ValueError> {
        Ok(vec![match base.node_records.iter().find(|entry| entry.node_id == self.node_record.node_id) {
            Some(node_record) => RunMutation::FinishRunNode(Self { node_record: node_record.clone() }),
            None => RunMutation::RemoveRunNode(RemoveRunNode { node_id: self.node_record.node_id.clone() }),
        }])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Finish run node {}", self.node_record.node_id), &format!("Laufknoten {} abschließen", self.node_record.node_id))
    }
    fn target(&self) -> Vec<String> {
        vec!["nodes".into(), self.node_record.node_id.clone()]
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
