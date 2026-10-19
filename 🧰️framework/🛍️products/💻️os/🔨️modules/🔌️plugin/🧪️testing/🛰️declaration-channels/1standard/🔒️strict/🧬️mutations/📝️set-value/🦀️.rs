//#region 📝️Std1StrictSetValue
//! 📝️ Replaces the Std1Strict fixture's value with an authored i32.
use super::{Std1StrictDiff, Std1StrictSnapshot};
use crate::declaration_fixture_mutations::std1_strict::Std1StrictMutation;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub(crate) struct SetValue {
    pub value: i32,
}

impl protocol::MutationKind<Std1StrictSnapshot, Std1StrictMutation> for SetValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "value", kind: "set-value", record: "SetValue" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set value to {}", self.value), &format!("Wert auf {} setzen", self.value))
    }
    fn target(&self) -> Vec<String> {
        vec!["value".into()]
    }
}

impl protocol::ColdMutationKind<Std1StrictSnapshot, Std1StrictMutation> for SetValue {
    fn diff(&self, _base: &Std1StrictSnapshot) -> protocol::MutationOutcome<Std1StrictDiff> {
        protocol::MutationOutcome::new(Std1StrictDiff { value: Some(self.value) })
    }
    fn inverse(&self, base: &Std1StrictSnapshot) -> Result<Vec<Std1StrictMutation>, semio_framework_value::ValueError> {
        Ok((|| vec![Std1StrictMutation::SetValue(Self { value: base.value })])())
    }
}

#[doc(hidden)]
#[cfg(test)]
#[path = "../../../../../../🧪️tests/🔒️standard-one-strict-set-value-unit/🦀️.rs"]
pub mod tests;
//#endregion 📝️Std1StrictSetValue

#[cfg(test)]
#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
mod sum_law_tests;
