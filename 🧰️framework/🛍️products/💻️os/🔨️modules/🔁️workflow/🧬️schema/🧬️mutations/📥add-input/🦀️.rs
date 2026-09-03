use super::super::{RemoveInput, WorkflowDiff, WorkflowInput, WorkflowMutation, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-input")]
pub struct AddInput { pub input: WorkflowInput }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for AddInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "workflow", kind: "add-input", record: "AddedWorkflowInput" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::DeclareInput { input: self.input.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::RemoveInput(RemoveInput { input_id: self.input.id.clone() })] }
    fn label(&self) -> String { format!("Add workflow input {}", self.input.id) }
    fn target(&self) -> Vec<String> { vec!["inputs".into(), self.input.id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<AddInput as MutationLeaf>::DESCRIPTOR.semantic_kind, "add-input"); }
}
