//#region 📝️Std1StrictSetValue
//! 📝️ Replaces the Std1Strict fixture's value with an authored i32.
use super::{Std1StrictSnapshot, Std1StrictDiff, Std1StrictMutation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
pub(crate) struct SetValue { pub value: i32 }

impl protocol::MutationKind<Std1StrictSnapshot, Std1StrictMutation> for SetValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "value", kind: "set-value", record: "SetValue" };
    fn diff(&self, _base: &Std1StrictSnapshot) -> protocol::MutationOutcome<Std1StrictDiff> {
        protocol::MutationOutcome::new(Std1StrictDiff { value: Some(self.value) })
    }
    fn inverse(&self, base: &Std1StrictSnapshot) -> Vec<Std1StrictMutation> {
        vec![Std1StrictMutation::SetValue(Self { value: base.value })]
    }
    fn label(&self) -> String { format!("Set value to {}", self.value) }
    fn target(&self) -> Vec<String> { vec!["value".into()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::tests::{assert_codecs, assert_laws, assert_metadata};

    fn operation(value: i32) -> Std1StrictMutation { Std1StrictMutation::SetValue(SetValue { value }) }

    #[test]
    fn actual_leaf_descriptor_and_provenance() {
        assert_metadata::<Std1StrictSnapshot, Std1StrictMutation, SetValue>(include_str!("🔣️.json"), operation);
    }

    #[test]
    fn assignment_inverse_and_structural_diff() {
        assert_laws::<Std1StrictSnapshot, Std1StrictMutation>(|value| Std1StrictSnapshot { value }, operation);
    }

    #[test]
    fn source_json_codecs_and_i32_boundaries() {
        assert_codecs::<Std1StrictSnapshot, Std1StrictMutation, SetValue>(operation);
    }
}
//#endregion 📝️Std1StrictSetValue
