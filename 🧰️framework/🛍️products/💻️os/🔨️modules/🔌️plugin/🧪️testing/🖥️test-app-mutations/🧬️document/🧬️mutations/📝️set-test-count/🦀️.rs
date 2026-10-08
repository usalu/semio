//#region 🔢️SetCount
use super::super::{TestDiff, TestMutation, TestSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, semio_framework_value_derive::RetireOwned, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SetCount {
    pub value: i32,
}

impl MutationKind<TestSnapshot, TestMutation> for SetCount {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "count", kind: "set-count", record: "SetCount" };
    fn diff(&self, _: &TestSnapshot) -> MutationOutcome<TestDiff> {
        MutationOutcome::new(TestDiff { count: Some(self.value), label: None, slot: None })
    }
    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<TestMutation>, semio_framework_value::ValueError> {
        Ok((|| vec![Self { value: base.count }.into()])())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set count to {}", self.value), &format!("Anzahl auf {} setzen", self.value))
    }
}

#[cfg(test)]
#[path = "../../../../../🧪️tests/📝️test-app-document-set-count-unit/🦀️.rs"]
mod tests;
//#endregion 🔢️SetCount
