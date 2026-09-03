use super::super::{workflow_parameter_entity_id, AddParameter, BindParameterField, WorkflowDiff, WorkflowMutation, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "remove-parameter")]
pub struct RemoveParameter { #[dsl(key = "id")] pub parameter_id: String }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for RemoveParameter {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "workflow", kind: "remove-parameter", record: "RemovedWorkflowParameter" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::RemoveParameter { parameter_id: self.parameter_id.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { { let Some(parameter) = base.parameters.iter().find(|parameter| workflow_parameter_entity_id(parameter) == self.parameter_id) else { return Vec::new() }; let mut ops: Vec<WorkflowMutation> = base.parameter_bindings.iter().filter(|binding| binding.parameter_id == self.parameter_id).map(|binding| WorkflowMutation::BindParameterField(BindParameterField { binding: binding.clone() })).collect(); ops.push(WorkflowMutation::AddParameter(AddParameter { parameter: Box::new(parameter.clone()) })); ops } }
    fn label(&self) -> String { format!("Remove workflow parameter {}", self.parameter_id) }
    fn target(&self) -> Vec<String> { vec!["parameters".into(), self.parameter_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<RemoveParameter as MutationLeaf>::DESCRIPTOR.semantic_kind, "remove-parameter"); }
}
