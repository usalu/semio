use super::super::super::tests::{assert_codecs, assert_laws, assert_metadata};
use super::*;

fn operation(value: i32) -> Std2AnyMutation {
    Std2AnyMutation::SetValue(SetValue { value })
}

#[test]
fn actual_leaf_descriptor_and_provenance() {
    assert_metadata::<Std2AnySnapshot, Std2AnyMutation, SetValue>(include_str!("../../🧫️fixtures/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🔣️.json"), operation);
}

#[test]
fn assignment_inverse_and_structural_diff() {
    assert_laws::<Std2AnySnapshot, Std2AnyMutation>(|value| Std2AnySnapshot { value }, operation);
}

#[test]
fn source_json_codecs_and_i32_boundaries() {
    assert_codecs::<Std2AnySnapshot, Std2AnyMutation, SetValue>(operation);
}
