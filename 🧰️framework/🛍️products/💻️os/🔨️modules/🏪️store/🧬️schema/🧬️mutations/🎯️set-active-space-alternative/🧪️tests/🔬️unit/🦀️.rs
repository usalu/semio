use semio_framework_os_store_space_schema::space_history_document::SpaceHistorySnapshot;
use semio_framework_os_store_space_schema::space_history_mutations::set_active_space_alternative::SetActiveSpaceAlternative;
use semio_framework_os_store_space_schema::space_history_mutations::SpaceHistoryMutation;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_os_spr::{MutationLeaf, OpBinary, OpText};

#[test]
fn metadata_and_explicit_nullable_wire_are_canonical() {
    assert_eq!(<SetActiveSpaceAlternative as MutationLeaf>::DESCRIPTOR.semantic_kind, "set-active-space-alternative");
    assert!(<SetActiveSpaceAlternative as MutationLeaf>::PROVENANCE.owner.ends_with("/🎯️set-active-space-alternative"));
    let mutation = SpaceHistoryMutation::SetActiveSpaceAlternative(SetActiveSpaceAlternative { alternative_id: None });
    let json = serde_json::to_string(&semio_framework_os_store_test_support::SerdeValue(&mutation.to_value())).expect("serialize");
    assert_eq!(json, r#"{"operation":"setActiveSpaceAlternative","payload":{"alternativeId":null}}"#);
    assert!(SpaceHistoryMutation::from_value(serde_json::from_str::<serde_json::Value>(r#"{"operation":"setActiveSpaceAlternative","payload":{}}"#).unwrap().into()).is_err());
    assert_eq!(SpaceHistoryMutation::parse_op(&json).expect("text"), mutation);
    assert_eq!(SpaceHistoryMutation::decode_op(&mutation.encode_op().expect("binary")).expect("binary decode"), mutation);
}

fn alternative(id: &str) -> semio_framework_os_store_space_schema::space_history_document::SpaceAlternative {
    semio_framework_os_store_space_schema::space_history_document::SpaceAlternative { id: id.into(), name: id.into(), checkpoint_ids: Vec::new() }
}

/// ➕️ L3: activating another alternative, clearing the active one and re-activating the active one sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    for active in [None, Some("a".to_string())] {
        let base = SpaceHistorySnapshot { alternatives: vec![alternative("a"), alternative("b")], active_alternative_id: active, ..Default::default() };
        for target in [Some("a".to_string()), Some("b".to_string()), None] {
            semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SpaceHistoryMutation::SetActiveSpaceAlternative(SetActiveSpaceAlternative { alternative_id: target }), &base).await;
        }
    }
}
