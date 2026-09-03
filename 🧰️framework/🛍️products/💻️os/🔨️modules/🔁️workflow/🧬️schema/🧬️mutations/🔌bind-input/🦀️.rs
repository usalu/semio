use super::super::{UnbindInput, WorkflowDiff, WorkflowInputBinding, WorkflowMutation, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "bind-input")]
pub struct BindInput { pub binding: WorkflowInputBinding }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for BindInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "workflow", kind: "bind-input", record: "BoundWorkflowInput" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::BindInput { binding: self.binding.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { match base.input_bindings.iter().find(|entry| entry.input_id == self.binding.input_id) { Some(existing) => vec![WorkflowMutation::BindInput(BindInput { binding: existing.clone() })], None => vec![WorkflowMutation::UnbindInput(UnbindInput { input_id: self.binding.input_id.clone() })] } }
    fn label(&self) -> String { format!("Bind workflow input {}", self.binding.input_id) }
    fn target(&self) -> Vec<String> { vec!["input-bindings".into(), self.binding.input_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<BindInput as MutationLeaf>::DESCRIPTOR.semantic_kind, "bind-input"); }
}
