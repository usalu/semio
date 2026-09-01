use super::super::{workflow_parameter_entity_id, WorkflowDiff, WorkflowMutation, WorkflowParameter, WorkflowSnapshot};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-parameter")]
pub struct ChangeParameter { #[dsl(key = "target")] pub parameter_id: String, #[dsl(statements)] pub parameter: Box<WorkflowParameter> }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for ChangeParameter {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "workflow", kind: "change-parameter", record: "ChangedWorkflowParameter" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::PatchParameter { parameter_id: self.parameter_id.clone(), parameter: (*self.parameter).clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { base.parameters.iter().find(|entry| workflow_parameter_entity_id(entry) == self.parameter_id).map_or_else(|| vec![WorkflowMutation::ChangeParameter(ChangeParameter { parameter_id: self.parameter_id.clone(), parameter: self.parameter.clone() })], |current| vec![WorkflowMutation::ChangeParameter(ChangeParameter { parameter_id: self.parameter_id.clone(), parameter: Box::new(current.clone()) })]) }
    fn label(&self) -> String { format!("Change workflow parameter {}", self.parameter_id) }
    fn target(&self) -> Vec<String> { vec!["parameters".into(), self.parameter_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<ChangeParameter as MutationLeaf>::DESCRIPTOR.semantic_kind, "change-parameter"); }
}
