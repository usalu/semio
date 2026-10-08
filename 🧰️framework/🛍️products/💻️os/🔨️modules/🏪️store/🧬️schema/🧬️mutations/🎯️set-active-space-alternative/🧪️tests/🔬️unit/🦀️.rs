use super::*;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use crate::os_spr::{MutationLeaf, OpBinary, OpText};

#[test]
fn metadata_and_explicit_nullable_wire_are_canonical() {
    assert_eq!(<SetActiveSpaceAlternative as MutationLeaf>::DESCRIPTOR.semantic_kind, "set-active-space-alternative");
    assert!(<SetActiveSpaceAlternative as MutationLeaf>::PROVENANCE.owner.ends_with("/🎯️set-active-space-alternative"));
    let mutation = SpaceHistoryMutation::SetActiveSpaceAlternative(SetActiveSpaceAlternative { alternative_id: None });
    let json = serde_json::to_string(&crate::os_store::test_support::SerdeValue(&mutation.to_value())).expect("serialize");
    assert_eq!(json, r#"{"operation":"setActiveSpaceAlternative","payload":{"alternativeId":null}}"#);
    assert!(SpaceHistoryMutation::from_value(serde_json::from_str::<serde_json::Value>(r#"{"operation":"setActiveSpaceAlternative","payload":{}}"#).unwrap().into()).is_err());
    assert_eq!(SpaceHistoryMutation::parse_op(&json).expect("text"), mutation);
    assert_eq!(SpaceHistoryMutation::decode_op(&mutation.encode_op().expect("binary")).expect("binary decode"), mutation);
}
