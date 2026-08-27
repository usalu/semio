//! 🧪️ Language-neutral change-header-fields behavior and codec laws.
use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

#[test]
fn authored_vector_and_inverse() {
    let vector: serde_json::Value = serde_json::from_str(include_str!("🔣️component.json")).unwrap();
    let before: BmpSnapshot = serde_json::from_value(vector["before"].clone()).unwrap();
    let expected: BmpSnapshot = serde_json::from_value(vector["after"].clone()).unwrap();
    let mutation: BmpMutation = serde_json::from_value(vector["mutation"].clone()).unwrap();
    let outcome = mutation.diff(&before);
    let after = outcome.diff().apply(&before).unwrap();
    assert_eq!(after, expected);
    let mut restored = after.clone();
    for inverse in mutation.inverse(&before) {
        restored = inverse.diff(&restored).diff().apply(&restored).unwrap();
    }
    assert_eq!(restored, before);
    assert_eq!(BmpMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    let bytes = mutation.encode_op().unwrap();
    assert_eq!(bytes[1], super::binary::BINARY_TAG);
    assert_eq!(BmpMutation::decode_op(&bytes).unwrap(), mutation);
    assert!(BmpMutation::decode_op(&bytes[..1]).is_err());
}
