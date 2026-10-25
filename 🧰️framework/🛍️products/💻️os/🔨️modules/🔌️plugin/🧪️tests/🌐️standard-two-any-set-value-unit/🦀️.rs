use crate::declaration_fixture_mutations::std2_any::Std2AnySnapshot;
use crate::declaration_fixture_mutations::tests::{assert_codecs, assert_laws, assert_metadata};
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::declaration_fixture_mutations::std2_any::set_value::SetValue;
use crate::declaration_fixture_mutations::std2_any::Std2AnyMutation;

#[doc(hidden)]
pub fn operation(value: i32) -> Std2AnyMutation {
    Std2AnyMutation::SetValue(SetValue { value })
}

#[test]
fn actual_leaf_descriptor_and_provenance() {
    assert_metadata::<Std2AnySnapshot, Std2AnyMutation, SetValue>(fixture_text!("../../../../🧪️testing/🛰️declaration-channels/2standard/🌐️any/🧬️mutations/📝️set-value/🔣️.json"), operation);
}

#[test]
fn assignment_inverse_and_structural_diff() {
    assert_laws::<Std2AnySnapshot, Std2AnyMutation>(|value| Std2AnySnapshot { value }, operation);
}

#[test]
fn source_json_codecs_and_i32_boundaries() {
    assert_codecs::<Std2AnySnapshot, Std2AnyMutation, SetValue>(operation);
}
