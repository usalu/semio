//! 🧪️ Language-neutral insert-other-segment behavior and codec laws.
use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

#[test]
fn authored_vector_and_inverse() {
    let vector: serde_json::Value = serde_json::from_str(include_str!("🔣️component.json")).unwrap();
    let before: JpgSnapshot = serde_json::from_value(vector["before"].clone()).unwrap();
    let expected: JpgSnapshot = serde_json::from_value(vector["after"].clone()).unwrap();
    let mutation: JpgMutation = serde_json::from_value(vector["mutation"].clone()).unwrap();
    let outcome = mutation.diff(&before);
    let after = outcome.diff().apply(&before).unwrap();
    assert_eq!(after, expected);
    let mut restored = after.clone();
    for inverse in mutation.inverse(&before) {
        restored = inverse.diff(&restored).diff().apply(&restored).unwrap();
    }
    assert_eq!(restored, before);
    assert_eq!(JpgMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    let bytes = mutation.encode_op().unwrap();
    assert_eq!(bytes[1], super::binary::BINARY_TAG);
    assert_eq!(JpgMutation::decode_op(&bytes).unwrap(), mutation);
    assert!(JpgMutation::decode_op(&bytes[..1]).is_err());
}
