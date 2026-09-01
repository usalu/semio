use super::super::{RunArtifact, RunDiff, RunMutation};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "start-run-node")]
pub struct StartRunNode { pub node_id: String }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<RunArtifact, RunMutation> for StartRunNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "start", entity: "run-node", kind: "start-run-node", record: "StartedRunNode" };
    fn diff(&self, _base: &RunArtifact) -> protocol::MutationOutcome<RunDiff> { protocol::MutationOutcome::new(RunDiff::NodeStarted { node_id: self.node_id.clone() }) }
    fn inverse(&self, _base: &RunArtifact) -> Vec<RunMutation> { Vec::new() }
    fn label(&self) -> String { format!("Start run node {}", self.node_id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.node_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_start_node_identity() { assert_eq!(<StartRunNode as MutationLeaf>::DESCRIPTOR.semantic_kind, "start-run-node"); }
}
