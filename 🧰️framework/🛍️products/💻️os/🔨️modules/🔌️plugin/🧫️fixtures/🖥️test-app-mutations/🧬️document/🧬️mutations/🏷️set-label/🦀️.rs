//#region 🏷️SetLabel
use super::super::{TestDiff, TestMutation, TestSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SetLabel {
    pub value: String,
}

impl MutationKind<TestSnapshot, TestMutation> for SetLabel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "label", kind: "set-label", record: "SetLabel" };
    fn diff(&self, _: &TestSnapshot) -> MutationOutcome<TestDiff> {
        MutationOutcome::new(TestDiff { count: None, label: Some(self.value.clone()) })
    }
    fn inverse(&self, base: &TestSnapshot) -> Vec<TestMutation> {
        vec![Self { value: base.label.clone() }.into()]
    }
    fn label(&self) -> String {
        format!("Set label to {}", self.value)
    }
}

#[cfg(test)]
#[path = "../../../../../🧪️tests/🏷️test-app-document-set-label-unit/🦀️.rs"]
mod tests;
//#endregion 🏷️SetLabel
