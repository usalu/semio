use super::super::{RemoveInput, WorkflowDiff, WorkflowInput, WorkflowMutation, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-input")]
pub struct AddInput {
    pub input: WorkflowInput,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for AddInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "workflow", kind: "add-input", record: "AddedWorkflowInput" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> {
        protocol::MutationOutcome::new(WorkflowDiff::DeclareInput { input: self.input.clone() })
    }
    fn inverse(&self, _base: &WorkflowSnapshot) -> Result<Vec<WorkflowMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![WorkflowMutation::RemoveInput(RemoveInput { input_id: self.input.id.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add workflow input {}", self.input.id), &format!("Arbeitsablaufeingabe {} hinzufügen", self.input.id))
    }
    fn target(&self) -> Vec<String> {
        vec!["inputs".into(), self.input.id.clone()]
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
