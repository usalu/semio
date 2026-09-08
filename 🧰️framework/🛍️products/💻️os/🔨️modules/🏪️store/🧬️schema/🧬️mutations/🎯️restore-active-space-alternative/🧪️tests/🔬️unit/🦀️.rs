use super::*;
use crate::os_dsl::{FromValue, ToValue};
use crate::os_spr::{MutationLeaf, OpBinary, OpText};

#[test]
fn metadata_and_explicit_nullable_wire_are_canonical() {
    assert_eq!(<RestoreActiveSpaceAlternative as MutationLeaf>::DESCRIPTOR.semantic_kind, "restore-active-space-alternative");
    assert!(<RestoreActiveSpaceAlternative as MutationLeaf>::PROVENANCE.owner.ends_with("/🎯️restore-active-space-alternative"));
    let mutation = SpaceHistoryMutation::RestoreActiveSpaceAlternative(RestoreActiveSpaceAlternative { alternative_id: None });
    let json = serde_json::to_string(&crate::os_store::test_support::SerdeValue(&mutation.to_value())).expect("serialize");
    assert_eq!(json, r#"{"operation":"restoreActiveSpaceAlternative","payload":{"alternativeId":null}}"#);
    assert!(SpaceHistoryMutation::from_value(serde_json::from_str::<serde_json::Value>(r#"{"operation":"restoreActiveSpaceAlternative","payload":{}}"#).unwrap().into()).is_err());
    assert_eq!(SpaceHistoryMutation::parse_op(&json).expect("text"), mutation);
    assert_eq!(SpaceHistoryMutation::decode_op(&mutation.encode_op().expect("binary")).expect("binary decode"), mutation);
}
