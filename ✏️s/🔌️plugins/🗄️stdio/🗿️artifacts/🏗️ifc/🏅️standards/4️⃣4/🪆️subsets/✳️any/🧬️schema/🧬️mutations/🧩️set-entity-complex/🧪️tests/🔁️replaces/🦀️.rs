//! 🧪️ Language-neutral fixtures for complex constituent replacement, replay and undo.

use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use protocol::command::DiffAlgebra;

const BEFORE: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-entity-complex/🔁️replaces/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-entity-complex/🔁️replaces/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-entity-complex/🔁️replaces/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-entity-complex/🔁️replaces/🎯️outcome/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-entity-complex/🔁️replaces/🔺️diff/🔣️.json");

fn decode<T: semio_framework_value::FromValue>(text: &str) -> T {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture decodes")
}

#[test]
fn committed_complex_replacement_replays_and_inverts() {
    let before: IfcSnapshot = decode(BEFORE);
    let after: IfcSnapshot = decode(AFTER);
    let mutation: IfcMutation = decode(MUTATION);
    let expected: IfcDiff = decode(DIFF);
    let outcome = mutation.diff(&before);
    assert!(outcome.messages().is_empty());
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).unwrap();
    assert_eq!(declared["status"], "applied");
    assert_eq!(declared["messages"], serde_json::json!([]));
    assert_eq!(outcome.diff(), &expected);
    assert_eq!(protocol::apply_diff(&expected, &before).unwrap(), after);
    assert_eq!(protocol::apply_diff(&expected.inverse(&before), &after).unwrap(), before);
    let mut restored = after;
    for inverse in mutation.inverse(&before).unwrap() {
        restored = protocol::apply_diff(inverse.diff(&restored).diff(), &restored).unwrap();
    }
    assert_eq!(restored, before);
}

#[test]
fn nested_complex_values_round_trip_in_both_operation_codecs() {
    let mutation: IfcMutation = decode(MUTATION);
    assert_eq!(IfcMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    assert_eq!(IfcMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
}

#[test]
fn an_empty_complex_list_restores_a_simple_instance_and_inverts() {
    let before: IfcSnapshot = decode(BEFORE);
    let mutation = IfcMutation::SetEntityComplex(SetEntityComplex { id: 1, complex: Vec::new() });
    let outcome = mutation.diff(&before);
    let after = protocol::apply_diff(outcome.diff(), &before).unwrap();
    assert!(after.entities[0].complex.is_empty());
    assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&before), &after).unwrap(), before);
}

#[test]
fn a_missing_complex_target_is_rejected_atomically() {
    let before: IfcSnapshot = decode(BEFORE);
    let mutation = IfcMutation::SetEntityComplex(SetEntityComplex { id: 999, complex: Vec::new() });
    let mut snapshot = before.clone();
    let outcome = apply_ifc_mutation(&mut snapshot, &mutation);
    assert!(!outcome.messages().is_empty());
    assert_eq!(snapshot, before);
}

#[test]
fn a_later_complex_edit_absorbs_into_an_inserted_entity() {
    let before = IfcSnapshot::default();
    let after: IfcSnapshot = decode(AFTER);
    let mut combined = diff::diff_insert_entity(0, after.entities[0].clone());
    combined.absorb(diff::diff_set_entity_complex(1, Vec::new()));
    let actual = protocol::apply_diff(&combined, &before).unwrap();
    assert!(actual.entities[0].complex.is_empty());
    assert_eq!(actual.entities[0].args, after.entities[0].args);
}
