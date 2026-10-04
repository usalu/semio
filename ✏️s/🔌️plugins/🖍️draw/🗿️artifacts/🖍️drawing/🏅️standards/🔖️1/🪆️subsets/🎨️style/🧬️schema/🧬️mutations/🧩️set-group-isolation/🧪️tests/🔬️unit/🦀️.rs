//! 🧩️ Canonical group-isolation changes preserve geometry, history, and operation codecs.
use crate::{DrawingSnapshot,DrawingMutation,DrawingDiff};
use protocol::{Mutation,MutationDiff};
const BEFORE:&str=include_str!("../../../../../🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/📸️snapshot/⬅️before/🔣️.json");
const AFTER:&str=include_str!("../../../../../🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/📸️snapshot/➡️after/🔣️.json");
const DIFF:&str=include_str!("../../../../../🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/🔺️diff/🔣️.json");
const MUTATION:&str=include_str!("../../../../../🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/🦠️mutation/🔣️.json");
#[test]
fn group_isolation_fixture_applies_and_inverts_through_all_owned_codecs() {
    let before:DrawingSnapshot=serde_json::from_str(BEFORE).unwrap();
    let after:DrawingSnapshot=serde_json::from_str(AFTER).unwrap();
    store::os_store::test_support::assert_dsl_pack_equivalence(&before);
    store::os_store::test_support::assert_dsl_pack_equivalence(&after);
    let mutation:DrawingMutation=serde_json::from_str(MUTATION).unwrap();
    let result=mutation.diff(&before);
    assert!(result.messages().is_empty());
    assert_eq!(serde_json::to_value(result.diff()).unwrap(),serde_json::from_str::<serde_json::Value>(DIFF).unwrap());
    assert_eq!(result.diff().apply(&before).unwrap(),after);
    let committed:DrawingDiff=serde_json::from_str(DIFF).unwrap();
    assert_eq!(committed.apply(&before).unwrap(),after);
    let mut restored=after;
    let inverse=mutation.inverse(&before).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(),1);
    for step in inverse {crate::mutations::apply_drawing_mutation(&mut restored,&step).unwrap();}
    assert_eq!(restored,before);
    for (snapshot,text) in [(&before,BEFORE),(&restored,BEFORE)] {assert_eq!(serde_json::to_value(snapshot).unwrap(),serde_json::from_str::<serde_json::Value>(text).unwrap());}
    store::os_store::test_support::assert_op_line_round_trip(&mutation);
    store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
}
#[test]
fn unchanged_and_missing_group_isolation_targets_do_not_mutate() {
    let before:DrawingSnapshot=serde_json::from_str(BEFORE).unwrap();
    let same=crate::mutations::set_group_isolation("group-a".into(),false);
    let result=same.diff(&before);
    assert!(!result.messages().is_empty());
    assert_eq!(result.diff().apply(&before).unwrap(),before);
    let missing=crate::mutations::set_group_isolation("missing".into(),true);
    assert!(!missing.diff(&before).messages().is_empty());
    assert!(missing.inverse(&before).expect("valid retained mutation inverse fixture").is_empty());
    let wrong_kind=crate::mutations::set_group_isolation("shape-a".into(),true);
    assert!(!wrong_kind.diff(&before).messages().is_empty());assert!(wrong_kind.inverse(&before).expect("valid retained mutation inverse fixture").is_empty());
}
