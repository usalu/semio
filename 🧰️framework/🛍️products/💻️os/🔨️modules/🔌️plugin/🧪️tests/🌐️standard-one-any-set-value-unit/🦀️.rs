use super::super::super::tests::{assert_codecs, assert_laws, assert_metadata};
use super::*;

fn operation(value: i32) -> Std1AnyMutation {
    Std1AnyMutation::SetValue(SetValue { value })
}

#[test]
fn actual_leaf_descriptor_and_provenance() {
    assert_metadata::<Std1AnySnapshot, Std1AnyMutation, SetValue>(include_str!("../../🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🔣️.json"), operation);
}

#[test]
fn assignment_inverse_and_structural_diff() {
    assert_laws::<Std1AnySnapshot, Std1AnyMutation>(|value| Std1AnySnapshot { value }, operation);
}

#[test]
fn source_json_codecs_and_i32_boundaries() {
    assert_codecs::<Std1AnySnapshot, Std1AnyMutation, SetValue>(operation);
}
