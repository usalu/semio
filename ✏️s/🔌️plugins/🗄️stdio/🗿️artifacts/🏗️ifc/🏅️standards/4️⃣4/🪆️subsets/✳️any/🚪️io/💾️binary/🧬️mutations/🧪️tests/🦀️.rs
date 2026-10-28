//! 📨️ Language-neutral mutation frames enforce version, kind, complete payload and EOF.

use crate::standards::v4::subsets::any::schema::mutations::IfcMutation;
use protocol::OpBinary;

const FIXTURE: &str = include_str!("🧫️fixtures/📨️framing/🔣️.json");

fn bytes(case: &serde_json::Value) -> Vec<u8> {
    case["bytes"].as_array().unwrap().iter().map(|value| u8::try_from(value.as_u64().unwrap()).unwrap()).collect()
}

#[test]
fn committed_frames_decode_and_reencode_exactly() {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
    for case in fixture["valid"].as_array().unwrap() {
        let expected: IfcMutation = semio_framework_pack_json::from_json_str(&case["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let wire = bytes(case);
        assert_eq!(IfcMutation::decode_op(&wire).unwrap(), expected);
        assert_eq!(expected.encode_op().unwrap(), wire);
    }
}

#[test]
fn malformed_frames_are_rejected_at_their_declared_boundary() {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
    for case in fixture["invalid"].as_array().unwrap() {
        let error = IfcMutation::decode_op(&bytes(case)).expect_err(case["id"].as_str().unwrap());
        match error {
            protocol::ProtocolError::Malformed { what, .. } => assert_eq!(what, case["what"].as_str().unwrap(), "{}", case["id"]),
            other => panic!("unexpected error for {}: {other:?}", case["id"]),
        }
    }
}

#[test]
#[cfg(target_pointer_width = "32")]
fn an_index_larger_than_the_native_coordinate_domain_is_rejected() {
    assert!(IfcMutation::decode_op(&[1, 10, 1, 128, 128, 128, 128, 16]).is_err());
}
