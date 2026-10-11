use super::super::{UnbindParameterField, WorkflowDiff, WorkflowMutation, WorkflowParameterBinding, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "bind-parameter-field")]
pub struct BindParameterField {
    pub binding: WorkflowParameterBinding,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for BindParameterField {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "workflow", kind: "bind-parameter-field", record: "BoundWorkflowParameterField" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> {
        protocol::MutationOutcome::new(WorkflowDiff::BindParameterField { binding: self.binding.clone() })
    }
    fn inverse(&self, base: &WorkflowSnapshot) -> Result<Vec<WorkflowMutation>, semio_framework_value::ValueError> {
        Ok(vec![match base.parameter_bindings.iter().find(|entry| entry.node_id == self.binding.node_id && entry.field_path == self.binding.field_path) {
            Some(prior) => WorkflowMutation::BindParameterField(Self { binding: prior.clone() }),
            None => WorkflowMutation::UnbindParameterField(UnbindParameterField { node_id: self.binding.node_id.clone(), field_path: self.binding.field_path.clone() }),
        }])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Bind workflow parameter {}", self.binding.parameter_id), &format!("Arbeitsablaufparameter {} binden", self.binding.parameter_id))
    }
    fn target(&self) -> Vec<String> {
        vec!["parameter-bindings".into(), self.binding.node_id.clone(), self.binding.field_path.clone()]
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
