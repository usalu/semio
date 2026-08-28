use super::super::{BindInput, WorkflowDiff, WorkflowMutation, WorkflowSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "unbind-input")]
pub struct UnbindInput { #[dsl(key = "id")] pub input_id: String }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for UnbindInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unbind", entity: "workflow", kind: "unbind-input", record: "UnboundWorkflowInput" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::UnbindInput { input_id: self.input_id.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { base.input_bindings.iter().find(|binding| binding.input_id == self.input_id).map(|binding| vec![WorkflowMutation::BindInput(BindInput { binding: binding.clone() })]).unwrap_or_default() }
    fn label(&self) -> String { format!("Unbind workflow input {}", self.input_id) }
    fn target(&self) -> Vec<String> { vec!["input-bindings".into(), self.input_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<UnbindInput as MutationLeaf>::DESCRIPTOR.semantic_kind, "unbind-input"); }
}
