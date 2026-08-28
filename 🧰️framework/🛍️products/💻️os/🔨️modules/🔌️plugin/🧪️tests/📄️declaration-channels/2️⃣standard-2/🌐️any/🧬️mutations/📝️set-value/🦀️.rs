//#region 📝️Std2AnySetValue
//! 📝️ Replaces the Std2Any fixture's value with an authored i32.
use super::{Std2AnySnapshot, Std2AnyDiff, Std2AnyMutation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
pub(crate) struct SetValue { pub value: i32 }

impl protocol::MutationKind<Std2AnySnapshot, Std2AnyMutation> for SetValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "value", kind: "set-value", record: "SetValue" };
    fn diff(&self, _base: &Std2AnySnapshot) -> protocol::MutationOutcome<Std2AnyDiff> {
        protocol::MutationOutcome::new(Std2AnyDiff { value: Some(self.value) })
    }
    fn inverse(&self, base: &Std2AnySnapshot) -> Vec<Std2AnyMutation> {
        vec![Std2AnyMutation::SetValue(Self { value: base.value })]
    }
    fn label(&self) -> String { format!("Set value to {}", self.value) }
    fn target(&self) -> Vec<String> { vec!["value".into()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::tests::{assert_codecs, assert_laws, assert_metadata};

    fn operation(value: i32) -> Std2AnyMutation { Std2AnyMutation::SetValue(SetValue { value }) }

    #[test]
    fn actual_leaf_descriptor_and_provenance() {
        assert_metadata::<Std2AnySnapshot, Std2AnyMutation, SetValue>(include_str!("🔣️.json"), operation);
    }

    #[test]
    fn assignment_inverse_and_structural_diff() {
        assert_laws::<Std2AnySnapshot, Std2AnyMutation>(|value| Std2AnySnapshot { value }, operation);
    }

    #[test]
    fn source_json_codecs_and_i32_boundaries() {
        assert_codecs::<Std2AnySnapshot, Std2AnyMutation, SetValue>(operation);
    }
}
//#endregion 📝️Std2AnySetValue
