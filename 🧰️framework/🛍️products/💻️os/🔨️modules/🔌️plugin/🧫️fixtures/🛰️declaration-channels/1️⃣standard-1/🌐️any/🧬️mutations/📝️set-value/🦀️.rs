//#region 📝️Std1AnySetValue
//! 📝️ Replaces the Std1Any fixture's value with an authored i32.
use super::{Std1AnyDiff, Std1AnyMutation, Std1AnySnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub(crate) struct SetValue {
    pub value: i32,
}

impl protocol::MutationKind<Std1AnySnapshot, Std1AnyMutation> for SetValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "value", kind: "set-value", record: "SetValue" };
    fn diff(&self, _base: &Std1AnySnapshot) -> protocol::MutationOutcome<Std1AnyDiff> {
        protocol::MutationOutcome::new(Std1AnyDiff { value: Some(self.value) })
    }
    fn inverse(&self, base: &Std1AnySnapshot) -> Vec<Std1AnyMutation> {
        vec![Std1AnyMutation::SetValue(Self { value: base.value })]
    }
    fn label(&self) -> String {
        format!("Set value to {}", self.value)
    }
    fn target(&self) -> Vec<String> {
        vec!["value".into()]
    }
}

#[cfg(test)]
#[path = "../../../../../../🧪️tests/🌐️standard-one-any-set-value-unit/🦀️.rs"]
mod tests;
//#endregion 📝️Std1AnySetValue
