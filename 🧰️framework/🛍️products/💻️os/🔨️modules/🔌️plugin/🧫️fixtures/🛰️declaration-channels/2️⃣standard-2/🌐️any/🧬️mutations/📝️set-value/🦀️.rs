//#region 📝️Std2AnySetValue
//! 📝️ Replaces the Std2Any fixture's value with an authored i32.
use super::{Std2AnyDiff, Std2AnyMutation, Std2AnySnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub(crate) struct SetValue {
    pub value: i32,
}

impl protocol::MutationKind<Std2AnySnapshot, Std2AnyMutation> for SetValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "value", kind: "set-value", record: "SetValue" };
    fn diff(&self, _base: &Std2AnySnapshot) -> protocol::MutationOutcome<Std2AnyDiff> {
        protocol::MutationOutcome::new(Std2AnyDiff { value: Some(self.value) })
    }
    fn inverse(&self, base: &Std2AnySnapshot) -> Vec<Std2AnyMutation> {
        vec![Std2AnyMutation::SetValue(Self { value: base.value })]
    }
    fn label(&self) -> String {
        format!("Set value to {}", self.value)
    }
    fn target(&self) -> Vec<String> {
        vec!["value".into()]
    }
}

#[cfg(test)]
#[path = "../../../../../../🧪️tests/🌐️standard-two-any-set-value-unit/🦀️.rs"]
mod tests;
//#endregion 📝️Std2AnySetValue
