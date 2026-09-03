use super::super::{workflow_parameter_entity_id, RemoveParameter, WorkflowDiff, WorkflowMutation, WorkflowParameter, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-parameter")]
pub struct AddParameter { #[dsl(statements)] pub parameter: Box<WorkflowParameter> }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for AddParameter {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "workflow", kind: "add-parameter", record: "AddedWorkflowParameter" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::AddParameter { parameter: (*self.parameter).clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::RemoveParameter(RemoveParameter { parameter_id: workflow_parameter_entity_id(&self.parameter).into() })] }
    fn label(&self) -> String { format!("Add workflow parameter {}", workflow_parameter_entity_id(&self.parameter)) }
    fn target(&self) -> Vec<String> { vec!["parameters".into(), workflow_parameter_entity_id(&self.parameter).into()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<AddParameter as MutationLeaf>::DESCRIPTOR.semantic_kind, "add-parameter"); }
}
