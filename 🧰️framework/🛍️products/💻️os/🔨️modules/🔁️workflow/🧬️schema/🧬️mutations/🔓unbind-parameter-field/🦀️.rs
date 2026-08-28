use super::super::{BindParameterField, WorkflowDiff, WorkflowMutation, WorkflowSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "unbind-parameter-field")]
pub struct UnbindParameterField { pub node_id: String, pub field_path: String }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for UnbindParameterField {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unbind", entity: "workflow", kind: "unbind-parameter-field", record: "UnboundWorkflowParameterField" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::UnbindParameterField { node_id: self.node_id.clone(), field_path: self.field_path.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { base.parameter_bindings.iter().find(|binding| binding.node_id == self.node_id && binding.field_path == self.field_path).map(|binding| vec![WorkflowMutation::BindParameterField(BindParameterField { binding: binding.clone() })]).unwrap_or_default() }
    fn label(&self) -> String { format!("Unbind workflow parameter field {}", self.field_path) }
    fn target(&self) -> Vec<String> { vec!["parameter-bindings".into(), self.node_id.clone(), self.field_path.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<UnbindParameterField as MutationLeaf>::DESCRIPTOR.semantic_kind, "unbind-parameter-field"); }
}
