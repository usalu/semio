//#region 🏷️SetLabel
use super::super::{TestDiff, TestMutation, TestSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SetLabel {
    pub value: String,
}

impl MutationKind<TestSnapshot, TestMutation> for SetLabel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "label", kind: "set-label", record: "SetLabel" };
    fn diff(&self, base: &TestSnapshot) -> MutationOutcome<TestDiff> {
        let outcome = MutationOutcome::new(TestDiff { count: None, label: Some(self.value.clone()), slot: None });
        match base.label == self.value {
            true => outcome.warning("mutation.no-op", format!("the label already reads {}", self.value)),
            false => outcome,
        }
    }
    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<TestMutation>, semio_framework_value::ValueError> {
        Ok((|| vec![Self { value: base.label.clone() }.into()])())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set label to {}", self.value), &format!("Beschriftung auf {} setzen", self.value))
    }
}

#[cfg(test)]
#[path = "../../../../../🧪️tests/🏷️test-app-document-set-label-unit/🦀️.rs"]
mod tests;
//#endregion 🏷️SetLabel
