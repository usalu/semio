//#region 📝️Std1StrictSetValue
//! 📝️ Replaces the Std1Strict fixture's value with an authored i32.
use super::{Std1StrictDiff, Std1StrictMutation, Std1StrictSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub(crate) struct SetValue {
    pub value: i32,
}

impl protocol::MutationKind<Std1StrictSnapshot, Std1StrictMutation> for SetValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "value", kind: "set-value", record: "SetValue" };
    fn diff(&self, _base: &Std1StrictSnapshot) -> protocol::MutationOutcome<Std1StrictDiff> {
        protocol::MutationOutcome::new(Std1StrictDiff { value: Some(self.value) })
    }
    fn inverse(&self, base: &Std1StrictSnapshot) -> Vec<Std1StrictMutation> {
        vec![Std1StrictMutation::SetValue(Self { value: base.value })]
    }
    fn label(&self) -> String {
        format!("Set value to {}", self.value)
    }
    fn target(&self) -> Vec<String> {
        vec!["value".into()]
    }
}

#[cfg(test)]
#[path = "../../../../../../🧪️tests/🔒️standard-one-strict-set-value-unit/🦀️.rs"]
mod tests;
//#endregion 📝️Std1StrictSetValue
