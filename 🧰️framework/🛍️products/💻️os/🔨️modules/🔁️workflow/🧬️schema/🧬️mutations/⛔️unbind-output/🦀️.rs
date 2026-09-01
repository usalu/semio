use super::super::{BindOutput, WorkflowDiff, WorkflowMutation, WorkflowSnapshot};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "unbind-output")]
pub struct UnbindOutput { pub node_id: String, pub port_id: String }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for UnbindOutput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unbind", entity: "workflow", kind: "unbind-output", record: "UnboundWorkflowOutput" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::UnbindOutput { node_id: self.node_id.clone(), port_id: self.port_id.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { base.output_bindings.iter().find(|binding| binding.node_id == self.node_id && binding.port_id == self.port_id).map(|binding| vec![WorkflowMutation::BindOutput(BindOutput { binding: binding.clone() })]).unwrap_or_default() }
    fn label(&self) -> String { format!("Unbind workflow output {}", self.node_id) }
    fn target(&self) -> Vec<String> { vec!["output-bindings".into(), self.node_id.clone(), self.port_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<UnbindOutput as MutationLeaf>::DESCRIPTOR.semantic_kind, "unbind-output"); }
}
