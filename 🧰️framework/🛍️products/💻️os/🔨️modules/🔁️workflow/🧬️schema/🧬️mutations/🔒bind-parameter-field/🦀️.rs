use super::super::{UnbindParameterField, WorkflowDiff, WorkflowMutation, WorkflowParameterBinding, WorkflowSnapshot};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "bind-parameter-field")]
pub struct BindParameterField { pub binding: WorkflowParameterBinding }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for BindParameterField {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "workflow", kind: "bind-parameter-field", record: "BoundWorkflowParameterField" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::BindParameterField { binding: self.binding.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::UnbindParameterField(UnbindParameterField { node_id: self.binding.node_id.clone(), field_path: self.binding.field_path.clone() })] }
    fn label(&self) -> String { format!("Bind workflow parameter {}", self.binding.parameter_id) }
    fn target(&self) -> Vec<String> { vec!["parameter-bindings".into(), self.binding.node_id.clone(), self.binding.field_path.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<BindParameterField as MutationLeaf>::DESCRIPTOR.semantic_kind, "bind-parameter-field"); }
}
