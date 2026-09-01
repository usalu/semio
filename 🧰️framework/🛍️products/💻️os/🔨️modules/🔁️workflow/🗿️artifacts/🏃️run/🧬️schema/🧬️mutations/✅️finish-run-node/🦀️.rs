use super::super::{RunArtifact, RunDiff, RunMutation, RunNodeRecord};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "finish-run-node")]
pub struct FinishRunNode { pub node_record: RunNodeRecord }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<RunArtifact, RunMutation> for FinishRunNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "finish", entity: "run-node", kind: "finish-run-node", record: "FinishedRunNode" };
    fn diff(&self, _base: &RunArtifact) -> protocol::MutationOutcome<RunDiff> { protocol::MutationOutcome::new(RunDiff::NodeFinished { node_record: self.node_record.clone() }) }
    fn inverse(&self, base: &RunArtifact) -> Vec<RunMutation> { base.node_records.iter().find(|entry| entry.node_id == self.node_record.node_id).map(|node_record| vec![RunMutation::FinishRunNode(Self { node_record: node_record.clone() })]).unwrap_or_default() }
    fn label(&self) -> String { format!("Finish run node {}", self.node_record.node_id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.node_record.node_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_finish_identity() { assert_eq!(<FinishRunNode as MutationLeaf>::DESCRIPTOR.semantic_kind, "finish-run-node"); }
}
