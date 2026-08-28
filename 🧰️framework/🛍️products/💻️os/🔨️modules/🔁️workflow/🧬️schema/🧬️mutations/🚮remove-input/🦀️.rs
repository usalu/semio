use super::super::{AddInput, BindInput, WorkflowDiff, WorkflowMutation, WorkflowSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "remove-input")]
pub struct RemoveInput { #[dsl(key = "id")] pub input_id: String }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for RemoveInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "workflow", kind: "remove-input", record: "RemovedWorkflowInput" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::RemoveInput { input_id: self.input_id.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { { let Some(input) = base.inputs.iter().find(|input| input.id == self.input_id) else { return Vec::new() }; let mut ops: Vec<WorkflowMutation> = base.input_bindings.iter().filter(|binding| binding.input_id == self.input_id).map(|binding| WorkflowMutation::BindInput(BindInput { binding: binding.clone() })).collect(); ops.push(WorkflowMutation::AddInput(AddInput { input: input.clone() })); ops } }
    fn label(&self) -> String { format!("Remove workflow input {}", self.input_id) }
    fn target(&self) -> Vec<String> { vec!["inputs".into(), self.input_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<RemoveInput as MutationLeaf>::DESCRIPTOR.semantic_kind, "remove-input"); }
}
