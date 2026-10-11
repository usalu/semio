use super::super::{workflow_parameter_entity_id, WorkflowDiff, WorkflowMutation, WorkflowParameter, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-parameter")]
pub struct ChangeParameter {
    #[dsl(key = "target")]
    pub parameter_id: String,
    #[dsl(statements)]
    pub parameter: Box<WorkflowParameter>,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for ChangeParameter {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "workflow", kind: "change-parameter", record: "ChangedWorkflowParameter" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> {
        protocol::MutationOutcome::new(WorkflowDiff::PatchParameter { parameter_id: self.parameter_id.clone(), parameter: (*self.parameter).clone() })
    }
    fn inverse(&self, base: &WorkflowSnapshot) -> Result<Vec<WorkflowMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        base.parameters.iter().find(|entry| workflow_parameter_entity_id(entry) == self.parameter_id).map_or_else(
            Vec::new,
            |current| vec![WorkflowMutation::ChangeParameter(ChangeParameter { parameter_id: workflow_parameter_entity_id(&self.parameter).to_string(), parameter: Box::new(current.clone()) })],
        )
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change workflow parameter {}", self.parameter_id), &format!("Arbeitsablaufparameter {} ändern", self.parameter_id))
    }
    fn target(&self) -> Vec<String> {
        vec!["parameters".into(), self.parameter_id.clone()]
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
