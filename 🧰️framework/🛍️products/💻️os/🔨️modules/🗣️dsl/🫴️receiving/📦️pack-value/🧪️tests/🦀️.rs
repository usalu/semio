//! 🧫️ Authored tagged pack wire values share the normative operation codec.

use super::*;
use protocol::OpBinary;

const ORIGINAL_PROTOCOL: &str = include_str!("../../🧪️tests/🧫️fixtures/🏷️protocol.semio");

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
enum TaggedPackValueMutation {
    Remove { text: String },
    Rename { text: String },
}

impl OpBinary for TaggedPackValueMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        tagged_value_binary::encode_op(ORIGINAL_PROTOCOL, tagged_value_binary::VariantTag::Key, self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        tagged_value_binary::decode_op(ORIGINAL_PROTOCOL, tagged_value_binary::VariantTag::Key, bytes)
    }
}

crate::original_pack_value_operation_receiving!(TaggedPackValueMutation, tagged_key = ORIGINAL_PROTOCOL);

#[test]
fn original_tagged_pack_value_receives_the_protocol_variant_and_exact_payload_range() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🏷️tagged.json")).unwrap();
    let expected = TaggedPackValueMutation::Rename { text: fixture["text"].as_str().unwrap().into() };
    let encoded: Vec<u8> = serde_json::from_value(fixture["packWire"].clone()).unwrap();
    assert_eq!(expected.encode_op().unwrap(), encoded);
    let mut source = vec![0xff, 0xff];
    let start = source.len();
    source.extend_from_slice(&encoded);
    let end = source.len();
    source.push(0xff);
    let f = &fixture["turn"];
    let grant = RetainedCloneGrant {
        maximum_items: f["items"].as_u64().unwrap() as usize,
        maximum_copy_bytes: f["copyBytes"].as_u64().unwrap() as usize,
        maximum_capacity_bytes: f["capacityBytes"].as_u64().unwrap() as usize,
        maximum_release_bytes: f["releaseBytes"].as_u64().unwrap() as usize,
        maximum_depth: f["depth"].as_u64().unwrap() as usize,
    };
    let stock = ReceivingStock {
        stock: RetainedCloneGrant {
            maximum_items: 10000,
            maximum_copy_bytes: 1 << 20,
            maximum_capacity_bytes: 1 << 20,
            maximum_release_bytes: 1 << 20,
            maximum_depth: grant.maximum_depth,
        },
        quantum: grant,
        fuel: 10000,
    };
    let denied = RetainedCloneGrant { maximum_items: 0, ..grant };
    let refused = TaggedPackValueMutation::admit_receiving(&source, (start, end), denied);
    assert_eq!(refused.work, 0);
    assert!(refused.result.unwrap().is_none());
    let output = receive_to_completion::<TaggedPackValueMutation>(&source, (start, end), stock).unwrap();
    assert_eq!(output, expected);
    assert_eq!(output, TaggedPackValueMutation::decode_op(&encoded).unwrap());
    assert!(receive_to_completion::<TaggedPackValueMutation>(&source, (start, start + 2), stock).is_err());
    assert!(receive_to_completion::<TaggedPackValueMutation>(&source, (start, end + 1), stock).is_err());
}
