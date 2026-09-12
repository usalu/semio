use super::{TypedOperationResultPage, TypedOperationResultToken};

#[test]
fn renderer_result_lane_vectors_decode_and_reject_unknown_tags() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🔌️plugin/🧫️fixtures/🔬️app-typed-command-full-operation/🔣️renderer-result-lanes.json")).expect("neutral result lanes");
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

/// 🗞️ The demux law: one turn's `Shell{instance}` messages carry TWO wire languages, and the host
/// must tell them apart before either decoder sees the bytes. Drives the production reader
/// (`TypedOperationResultPage::decode_guest_message`) over the neutral stream fixture, and asserts
/// the page magic's first byte is exactly the `AppFrame` tag a missing discrimination reports.
#[test]
fn one_shell_message_stream_splits_into_app_frames_and_typed_operation_pages() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🔌️plugin/🧫️fixtures/🔬️app-typed-command-full-operation/🔣️renderer-result-lanes.json")).expect("neutral result lanes");
    let page_magic = fixture["page"]["pageMagic"].as_str().expect("page magic");
    let ack_magic = fixture["page"]["ackMagic"].as_str().expect("ack magic");
    assert_eq!(page_magic.as_bytes(), TypedOperationResultPage::PAGE_MAGIC, "the fixture declares the production page magic");
    assert_eq!(ack_magic.as_bytes(), TypedOperationResultPage::ACK_MAGIC, "the fixture declares the production ack magic");
    assert_eq!(fixture["page"]["pageMagicFirstByte"].as_u64().unwrap() as u8, TypedOperationResultPage::PAGE_MAGIC[0], "the reported `unknown tag` is the page magic's own first byte");
    assert_eq!(fixture["page"]["headerBytes"].as_u64().unwrap() as usize, 30);
    assert_eq!(fixture["page"]["tokenBytes"].as_u64().unwrap() as usize, 25);
    assert_eq!(fixture["page"]["maxPayloadBytes"].as_u64().unwrap() as usize, super::TYPED_OPERATION_RESULT_PAGE_BYTES);

    let stream = &fixture["shellMessageStream"];
    let receiver = stream["token"]["receiver"].as_u64().unwrap() as u32;
    let token = TypedOperationResultToken {
        receiver,
        operation: stream["token"]["operation"].as_u64().unwrap(),
        generation: stream["token"]["generation"].as_u64().unwrap(),
        sequence: stream["token"]["sequence"].as_u64().unwrap() as u32,
        attempt: stream["token"]["attempt"].as_u64().unwrap() as u8,
    };
    let instance = stream["instanceId"].as_u64().unwrap() as u32;
    let (mut app_frames, mut pages, mut terminal) = (0usize, 0usize, false);
    let terminal_tag = fixture["terminalTag"].as_u64().unwrap() as u8;
    for message in stream["messages"].as_array().expect("stream messages") {
        let addressed = message["instanceId"].as_u64().map(|value| value as u32).unwrap_or(instance);
        let bytes = match message["kind"].as_str().expect("message kind") {
            "page" => {
                let lane = message["lane"].as_u64().unwrap() as u8;
                let payload = message["payload"].as_str().unwrap_or("").as_bytes();
                let mut wire = Vec::from(TypedOperationResultPage::PAGE_MAGIC);
                wire.extend_from_slice(&token.receiver.to_le_bytes());
                wire.extend_from_slice(&token.operation.to_le_bytes());
                wire.extend_from_slice(&token.generation.to_le_bytes());
                wire.extend_from_slice(&token.sequence.to_le_bytes());
                wire.push(token.attempt);
                wire.push(lane);
                wire.extend_from_slice(&(payload.len() as u32).to_le_bytes());
                wire.extend_from_slice(payload);
                wire
            }
            "appFrame" => vec![message["appFrameTag"].as_u64().unwrap() as u8, 0],
            other => panic!("unknown shell message kind {other}"),
        };
        match TypedOperationResultPage::decode_guest_message(&bytes) {
            Some(page) if addressed == instance && page.token.receiver == instance => {
                pages += 1;
                terminal |= page.lane == terminal_tag;
                assert_eq!(page.bytes(), message["payload"].as_str().unwrap_or("").as_bytes(), "a page keeps its exact payload");
            }
            Some(_) => panic!("a page addressed elsewhere must not be claimed by this instance"),
            None => {
                if addressed == instance {
                    app_frames += 1;
                }
            }
        }
    }
    let expect = &stream["expect"];
    assert_eq!(app_frames, expect["appFrames"].as_u64().unwrap() as usize, "app frames this instance owns");
    assert_eq!(pages, expect["pages"].as_u64().unwrap() as usize, "result pages this instance owns");
    assert_eq!(pages, expect["acknowledgements"].as_u64().unwrap() as usize, "every page owes exactly one acknowledgement");
    assert_eq!(terminal, expect["terminal"].as_bool().unwrap(), "the stream's terminal page");
    assert!(stream["messages"].as_array().unwrap().iter().any(|message| message["lane"].as_u64() == Some(13)), "the stream must exercise the window-transient lane this demux used to refuse");
}
