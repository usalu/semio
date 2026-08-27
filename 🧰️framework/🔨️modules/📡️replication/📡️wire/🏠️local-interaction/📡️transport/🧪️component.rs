//! 🧪️ Exact shared query wire vectors and strict malformed/authority preservation laws.
use super::*;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧪️fixtures.json")).unwrap() }
fn unhex(value: &str) -> Vec<u8> { value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect() }
fn authority() -> LocalInteractionQueryToken {
    LocalInteractionQueryToken { request_id: 13, query_generation: 41, identity: LocalInteractionIdentity { app_instance_id: 7, generation: 9_007_199_254_740_993, revision: [0x11; 32], document_revision: [0x22; 32], topology_revision: [0x33; 32] }, ordinal: 2 }
}
fn expected_token() -> Vec<u8> {
    let mut bytes = vec![13, 41, 7];
    bytes.extend_from_slice(&unhex("8180808080808010"));
    bytes.extend_from_slice(&[0x11; 32]); bytes.extend_from_slice(&[0x22; 32]); bytes.extend_from_slice(&[0x33; 32]); bytes.push(2); bytes
}

#[test]
fn local_interaction_transport_unsigned_matches_language_neutral_leb128() {
    let fixture = fixture();
    for row in fixture["unsigned"].as_array().unwrap() {
        let value = row["decimal"].as_str().unwrap().parse::<u64>().unwrap();
        let expected = unhex(row["hex"].as_str().unwrap());
        let mut encoded = Vec::new(); unsigned(&mut encoded, value); assert_eq!(encoded, expected);
        let mut reader = Reader::new(&encoded).unwrap(); assert_eq!(reader.unsigned().unwrap(), value); reader.finish().unwrap();
    }
    for row in fixture["malformedUnsigned"].as_array().unwrap() { assert!(Reader::new(&unhex(row.as_str().unwrap())).unwrap().unsigned().is_err()); }
}

#[test]
fn local_interaction_transport_three_commands_preserve_full_authority() {
    let commands = [LocalInteractionQueryCommand::Read { request_id: 13 }, LocalInteractionQueryCommand::Acknowledge { token: authority() }, LocalInteractionQueryCommand::Cancel { token: authority() }];
    for (index, command) in commands.into_iter().enumerate() {
        let mut expected = vec![index as u8];
        if index == 0 { expected.push(13); } else { expected.extend_from_slice(&expected_token()); }
        let mut encoded = encode_local_interaction_query_command(&command);
        assert_eq!(encoded, expected); assert_eq!(decode_local_interaction_query_command(&encoded).unwrap(), command);
        encoded.push(0); assert!(decode_local_interaction_query_command(&encoded).is_err());
    }
    assert_eq!(serde_json::to_value(LocalInteractionQueryCommand::Read { request_id: u64::MAX }).unwrap()["requestId"], "18446744073709551615");
}

#[test]
fn local_interaction_transport_output_admission_is_atomic_and_exact() {
    let reply = LocalInteractionQueryReply::Started { token: authority() };
    let expected = encode_local_interaction_query_reply(&reply).unwrap();
    let mut output = Vec::new();
    assert!(encode_local_interaction_query_reply_into(&reply, &mut output).is_err());
    assert!(output.is_empty());
    output.try_reserve_exact(expected.len()).unwrap();
    let capacity = output.capacity();
    encode_local_interaction_query_reply_into(&reply, &mut output).unwrap();
    assert_eq!(output, expected);
    assert_eq!(output.capacity(), capacity);
}

#[test]
fn local_interaction_transport_four_replies_preserve_full_authority_and_page_bytes() {
    let token = authority();
    let page = LocalInteractionPage { request_id: token.request_id, query_generation: token.query_generation, identity: token.identity.clone(), ordinal: token.ordinal, terminal: true, bytes: vec![123, 125] };
    let replies = [LocalInteractionQueryReply::Started { token: token.clone() }, LocalInteractionQueryReply::Page { page }, LocalInteractionQueryReply::Closed { token, cancelled: false }, LocalInteractionQueryReply::Rejected { request_id: 13, code: LocalInteractionQueryRejection::Busy }];
    for (index, reply) in replies.into_iter().enumerate() {
        let mut expected = vec![index as u8];
        if index != 3 { expected.extend_from_slice(&expected_token()); }
        match index { 1 => expected.extend_from_slice(&[1, 2, 123, 125]), 2 => expected.push(0), 3 => expected.extend_from_slice(&[13, 0]), _ => {} }
        let mut encoded = encode_local_interaction_query_reply(&reply).unwrap();
        assert_eq!(encoded, expected); assert_eq!(decode_local_interaction_query_reply(&encoded).unwrap(), reply);
        encoded.push(0); assert!(decode_local_interaction_query_reply(&encoded).is_err());
    }
}

#[test]
fn local_interaction_transport_rejects_oversized_page_before_payload_copy() {
    let mut bytes = vec![1]; bytes.extend_from_slice(&expected_token()); bytes.push(0); unsigned(&mut bytes, 4097);
    assert_eq!(decode_local_interaction_query_reply(&bytes), Err("local-interaction.page-length"));
    assert!(decode_local_interaction_query_reply(&vec![0; MAXIMUM_QUERY_WIRE_BYTES + 1]).is_err());
}
