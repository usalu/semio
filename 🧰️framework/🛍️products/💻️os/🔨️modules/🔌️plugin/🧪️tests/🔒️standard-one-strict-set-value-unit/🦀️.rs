use super::super::super::tests::{assert_codecs, assert_laws, assert_metadata};
use super::*;

fn operation(value: i32) -> Std1StrictMutation {
    Std1StrictMutation::SetValue(SetValue { value })
}

#[test]
fn actual_leaf_descriptor_and_provenance() {
    assert_metadata::<Std1StrictSnapshot, Std1StrictMutation, SetValue>(include_str!("../../🧪️testkit/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🔣️.json"), operation);
}

#[test]
fn assignment_inverse_and_structural_diff() {
    assert_laws::<Std1StrictSnapshot, Std1StrictMutation>(|value| Std1StrictSnapshot { value }, operation);
}

#[test]
fn source_json_codecs_and_i32_boundaries() {
    assert_codecs::<Std1StrictSnapshot, Std1StrictMutation, SetValue>(operation);
}
