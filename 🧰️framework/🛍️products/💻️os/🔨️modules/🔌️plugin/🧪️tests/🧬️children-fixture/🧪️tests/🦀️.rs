//#region 🧪️EmptyChildrenMutationTests
use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

fn cases() -> serde_json::Value { serde_json::from_str(include_str!("../🔣️.json")).expect("children mutation cases") }

#[test]
fn empty_roster_has_no_fabricated_leaf() {
    assert_eq!(<ChildrenTestMutation as Mutation<ChildrenTestSnapshot>>::DESCRIPTORS.len(), 0);
    let absent: Option<ChildrenTestMutation> = None;
    assert_eq!(serde_json::to_value(absent).expect("absence"), serde_json::Value::Null);
    let schema: serde_json::Value = serde_json::from_str(include_str!("../🧬️mutations/🔣️.json")).expect("owned empty schema");
    assert_eq!(schema["not"], serde_json::json!({}));
}

#[test]
fn every_neutral_json_value_is_uninhabited() {
    for value in cases()["json"].as_array().expect("JSON cases") {
        assert!(serde_json::from_value::<ChildrenTestMutation>(value.clone()).is_err(), "{value}");
    }
}

#[test]
fn empty_and_nonempty_codec_inputs_are_rejected() {
    for value in cases()["text"].as_array().expect("text cases") {
        assert!(ChildrenTestMutation::parse_op(value.as_str().expect("text")).is_err(), "{value}");
    }
    for value in cases()["binary"].as_array().expect("binary cases") {
        let bytes: Vec<u8> = value.as_array().expect("bytes").iter().map(|byte| u8::try_from(byte.as_u64().expect("unsigned byte")).expect("u8")).collect();
        let error = ChildrenTestMutation::decode_op(&bytes).expect_err("uninhabited binary");
        assert!(matches!(error, protocol::ProtocolError::Malformed { what: "children-test-mutation", offset: 0, .. }));
    }
}

#[test]
fn existing_children_diff_stays_identity() {
    let base = ChildrenTestSnapshot;
    let mut diff = ChildrenTestDiff;
    diff.absorb(ChildrenTestDiff);
    assert_eq!(diff.apply(&base).expect("identity"), base);
}
//#endregion 🧪️EmptyChildrenMutationTests

