use super::super::{WorkflowDiff, WorkflowMutation, WorkflowSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "update-node-ports")]
pub struct UpdateNodePorts;
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for UpdateNodePorts {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "workflow", kind: "update-node-ports", record: "UpdatedWorkflowNodePorts" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::SyncNodePorts) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { Vec::new() }
    fn label(&self) -> String { "Update workflow node ports".into() }
    fn target(&self) -> Vec<String> { vec!["nodes".into()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<UpdateNodePorts as MutationLeaf>::DESCRIPTOR.semantic_kind, "update-node-ports"); }
}
