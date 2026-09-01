//#region 📝️Std1AnySetValue
//! 📝️ Replaces the Std1Any fixture's value with an authored i32.
use super::{Std1AnySnapshot, Std1AnyDiff, Std1AnyMutation};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub(crate) struct SetValue { pub value: i32 }

impl protocol::MutationKind<Std1AnySnapshot, Std1AnyMutation> for SetValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "value", kind: "set-value", record: "SetValue" };
    fn diff(&self, _base: &Std1AnySnapshot) -> protocol::MutationOutcome<Std1AnyDiff> {
        protocol::MutationOutcome::new(Std1AnyDiff { value: Some(self.value) })
    }
    fn inverse(&self, base: &Std1AnySnapshot) -> Vec<Std1AnyMutation> {
        vec![Std1AnyMutation::SetValue(Self { value: base.value })]
    }
    fn label(&self) -> String { format!("Set value to {}", self.value) }
    fn target(&self) -> Vec<String> { vec!["value".into()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::tests::{assert_codecs, assert_laws, assert_metadata};

    fn operation(value: i32) -> Std1AnyMutation { Std1AnyMutation::SetValue(SetValue { value }) }

    #[test]
    fn actual_leaf_descriptor_and_provenance() {
        assert_metadata::<Std1AnySnapshot, Std1AnyMutation, SetValue>(include_str!("🔣️.json"), operation);
    }

    #[test]
    fn assignment_inverse_and_structural_diff() {
        assert_laws::<Std1AnySnapshot, Std1AnyMutation>(|value| Std1AnySnapshot { value }, operation);
    }

    #[test]
    fn source_json_codecs_and_i32_boundaries() {
        assert_codecs::<Std1AnySnapshot, Std1AnyMutation, SetValue>(operation);
    }
}
//#endregion 📝️Std1AnySetValue
