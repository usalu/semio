use super::{TypedOperationResultPage, TypedOperationResultToken};

#[test]
fn renderer_result_lane_vectors_decode_and_reject_unknown_tags() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🔣️renderer-result-lanes.json")).expect("neutral result lanes");
    let token = TypedOperationResultToken { receiver: 1, operation: 2, generation: 3, sequence: 4, attempt: 5 };
    let mut wire = Vec::from(TypedOperationResultPage::PAGE_MAGIC);
    wire.extend_from_slice(&token.receiver.to_le_bytes());
    wire.extend_from_slice(&token.operation.to_le_bytes());
    wire.extend_from_slice(&token.generation.to_le_bytes());
    wire.extend_from_slice(&token.sequence.to_le_bytes());
    wire.push(token.attempt);
    let lane_index = wire.len();
    wire.push(0);
    wire.extend_from_slice(&1_u32.to_le_bytes());
    wire.push(0x5a);
    for row in fixture["lanes"].as_array().expect("lane vectors") {
        wire[lane_index] = row["tag"].as_u64().unwrap() as u8;
        let page = TypedOperationResultPage::decode_guest_message(&wire).expect("declared lane");
        assert_eq!(page.lane, wire[lane_index]);
        assert_eq!(page.token.receiver, token.receiver);
        assert_eq!(page.token.operation, token.operation);
        assert_eq!(page.token.generation, token.generation);
        assert_eq!(page.token.sequence, token.sequence);
        assert_eq!(page.token.attempt, token.attempt);
        assert_eq!(page.bytes(), &[0x5a]);
    }
    for tag in fixture["invalidTags"].as_array().expect("invalid lane vectors") {
        wire[lane_index] = tag.as_u64().unwrap() as u8;
        assert!(TypedOperationResultPage::decode_guest_message(&wire).is_none());
    }
}
