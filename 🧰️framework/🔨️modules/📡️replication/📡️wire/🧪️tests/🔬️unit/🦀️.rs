
use super::*;

//#region 🧸️Fixtures
async fn sample_envelope(id: &str) -> crate::causal::MutationEnvelope {
    crate::causal::MutationEnvelope {
        mutation_id: crate::ids::MutationId(id.to_string()),
        document_id: crate::ids::ArtifactId("document-1".to_string()),
        actor: crate::ids::ActorId("actor-1".to_string()),
        dependencies: Vec::new(),
        diff: crate::causal::ArtifactDiff { schema: crate::ids::SchemaId("diff.v1".to_string()), payload: format!("value:{id}").into_bytes() },
        inverse: crate::causal::InverseMutation { schema: crate::ids::SchemaId("diff.v1".to_string()), payload: Vec::new() },
        timestamp: crate::ids::HybridLogicalTimestamp::new(1, 0),
    }
}

async fn sample_frontier() -> crate::causal::FrontierSummary {
    crate::causal::FrontierSummary { document_id: crate::ids::ArtifactId("document-1".to_string()), head_edit_ordinal: 5, head_edit_id: "edit-5".to_string(), last_commit_seq: 2, chain_hash: [7u8; 32] }
}

fn artifact_bootstrap_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🚀️artifact-bootstrap/🔣️.json")).expect("artifact bootstrap fixture")
}

fn bytes_from_hex(hex: &str) -> Vec<u8> {
    assert_eq!(hex.len() % 2, 0);
    (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("hex byte")).collect()
}

fn hash_from_hex(hex: &str) -> [u8; 32] {
    bytes_from_hex(hex).try_into().expect("32-byte hash")
}

fn fixture_frontier(value: &serde_json::Value) -> crate::causal::FrontierSummary {
    crate::causal::FrontierSummary {
        document_id: crate::ids::ArtifactId(value["documentId"].as_str().unwrap().to_string()),
        head_edit_ordinal: value["headEditOrdinal"].as_u64().unwrap(),
        head_edit_id: value["headEditId"].as_str().unwrap().to_string(),
        last_commit_seq: value["lastCommitSeq"].as_u64().unwrap(),
        chain_hash: hash_from_hex(value["chainHash"].as_str().unwrap()),
    }
}

fn fixture_artifact_bootstrap(inline: bool) -> ArtifactBootstrap {
    let fixture = artifact_bootstrap_fixture();
    let payload = &fixture["payload"];
    ArtifactBootstrap {
        format_version: fixture["formatVersion"].as_u64().unwrap() as u32,
        descriptor_hash: hash_from_hex(fixture["artifact"]["descriptorHash"].as_str().unwrap()),
        artifact_schema: fixture["artifact"]["schema"].as_str().unwrap().to_string(),
        artifact_kind: fixture["artifact"]["kind"].as_str().unwrap().to_string(),
        pack_schema_hash: hash_from_hex(fixture["artifact"]["packSchemaHash"].as_str().unwrap()),
        baseline_frontier: fixture_frontier(&fixture["artifact"]["baselineFrontier"]),
        pack_hash: hash_from_hex(payload["packHash"].as_str().unwrap()),
        spr_hash: hash_from_hex(payload["sprHash"].as_str().unwrap()),
        pack_length: payload["packLength"].as_u64().unwrap(),
        spr_length: payload["sprLength"].as_u64().unwrap(),
        chunk_count: if inline { 0 } else { fixture["chunkByteLengths"].as_array().unwrap().len() as u32 },
        aggregate_hash: hash_from_hex(payload["aggregateHash"].as_str().unwrap()),
        required_tail_frontier: fixture_frontier(&fixture["artifact"]["requiredTailFrontier"]),
        inline: inline.then(|| ArtifactBootstrapPair { pack: bytes_from_hex(payload["packHex"].as_str().unwrap()), spr: bytes_from_hex(payload["sprHex"].as_str().unwrap()) }),
    }
}

fn fixture_artifact_chunks() -> Vec<Vec<u8>> {
    let fixture = artifact_bootstrap_fixture();
    let mut content = bytes_from_hex(fixture["payload"]["packHex"].as_str().unwrap());
    content.extend_from_slice(&bytes_from_hex(fixture["payload"]["sprHex"].as_str().unwrap()));
    let mut offset = 0usize;
    fixture["chunkByteLengths"]
        .as_array()
        .unwrap()
        .iter()
        .map(|length| {
            let end = offset + length.as_u64().unwrap() as usize;
            let chunk = content[offset..end].to_vec();
            offset = end;
            chunk
        })
        .collect()
}

#[derive(Default)]
struct TestBootstrapControl {
    cancelled_after_progress: Option<usize>,
    cancel_on_check: Option<usize>,
    checks: usize,
    now_ms: u64,
    progress: Vec<ArtifactBootstrapProgress>,
}

impl ArtifactBootstrapControl for TestBootstrapControl {
    fn is_cancelled(&mut self) -> bool {
        self.checks += 1;
        self.cancel_on_check.is_some_and(|check| self.checks >= check) || self.cancelled_after_progress.is_some_and(|count| self.progress.len() >= count)
    }

    fn now_ms(&mut self) -> u64 {
        self.now_ms
    }

    fn on_progress(&mut self, progress: ArtifactBootstrapProgress) {
        self.progress.push(progress);
    }
}
//#endregion 🧸️Fixtures

//#region 🔖️Lane
#[semio_framework_async_macros::async_test]
async fn lane_byte_round_trips() {
    assert_eq!(Lane::from_byte(Lane::Command.to_byte().await).await, Some(Lane::Command));
    assert_eq!(Lane::from_byte(Lane::Preview.to_byte().await).await, Some(Lane::Preview));
    assert_eq!(Lane::from_byte(2).await, None);
}
//#endregion 🔖️Lane

//#region 🔖️ClientFrame
async fn assert_client_round_trips(frame: &ClientFrame, lane: Lane) {
    let bytes = encode_client_frame(frame, lane).await;
    let (decoded_lane, decoded_frame) = decode_client_frame(&bytes).await.expect("decode must succeed");
    assert_eq!(decoded_lane, lane);
    assert_eq!(&decoded_frame, frame);
}

#[semio_framework_async_macros::async_test]
async fn client_frame_tag_zero_is_terminally_rejected() {
    assert!(decode_client_frame(&[0, 0]).await.is_err());
    assert!(decode_client_frame(include_bytes!("../../../🧫️fixtures/📡️wire/🚫️legacy-client-hello-rejected/💾️.bin")).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn client_frame_socket_hello_v1_round_trips_without_credentials() {
    assert_client_round_trips(&ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: "schema.v1".to_string(), pack_schema_hash: [2u8; 32], resume_token: None, frontier: Some(sample_frontier().await) }, Lane::Command).await;
}

#[semio_framework_async_macros::async_test]
async fn client_frame_commands_round_trips() {
    assert_client_round_trips(&ClientFrame::Commands { batch_id: 42, envelopes: vec![sample_envelope("op-1").await, sample_envelope("op-2").await] }, Lane::Command).await;
}

#[semio_framework_async_macros::async_test]
async fn client_frame_frontier_advertise_round_trips() {
    assert_client_round_trips(&ClientFrame::FrontierAdvertise { frontier: sample_frontier().await }, Lane::Command).await;
}

#[semio_framework_async_macros::async_test]
async fn client_frame_preview_publish_round_trips() {
    assert_client_round_trips(&ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] }, Lane::Preview).await;
}

#[semio_framework_async_macros::async_test]
async fn client_frame_presence_round_trips() {
    assert_client_round_trips(&ClientFrame::Presence { peer: b"{\"cursor\":[1,2]}".to_vec() }, Lane::Preview).await;
}

#[semio_framework_async_macros::async_test]
async fn client_frame_credit_grant_round_trips() {
    assert_client_round_trips(&ClientFrame::CreditGrant { n: 16 }, Lane::Command).await;
}

#[semio_framework_async_macros::async_test]
async fn client_frame_bye_round_trips() {
    assert_client_round_trips(&ClientFrame::Bye, Lane::Command).await;
}
//#endregion 🔖️ClientFrame

//#region 🔖️ServerFrame
async fn assert_server_round_trips(frame: &ServerFrame, lane: Lane) {
    let bytes = encode_server_frame(frame, lane).await;
    let (decoded_lane, decoded_frame) = decode_server_frame(&bytes).await.expect("decode must succeed");
    assert_eq!(decoded_lane, lane);
    assert_eq!(&decoded_frame, frame);
}

#[semio_framework_async_macros::async_test]
async fn server_frame_welcome_round_trips_for_every_bootstrap_variant() {
    for bootstrap in [Bootstrap::None, Bootstrap::Snapshot { pack_hash: [3u8; 32], inline: Some(vec![9, 9]) }, Bootstrap::Snapshot { pack_hash: [3u8; 32], inline: None }, Bootstrap::Tail] {
        assert_server_round_trips(&ServerFrame::Welcome { session_id: "session-1".to_string(), resume_token: "resume-1".to_string(), server_frontier: sample_frontier().await, bootstrap }, Lane::Command).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn server_frame_snapshot_chunk_round_trips() {
    assert_server_round_trips(&ServerFrame::SnapshotChunk { seq: 0, bytes: SnapshotChunkBytes::try_from_slice(&[1, 2, 3, 4]).unwrap() }, Lane::Command).await;
}

#[semio_framework_async_macros::async_test]
async fn server_frame_snapshot_done_round_trips() {
    assert_server_round_trips(&ServerFrame::SnapshotDone { seq_count: 4 }, Lane::Command).await;
}

#[semio_framework_async_macros::async_test]
async fn artifact_bootstrap_hashes_match_neutral_fixture() {
    let fixture = artifact_bootstrap_fixture();
    let pack = bytes_from_hex(fixture["payload"]["packHex"].as_str().unwrap());
    let spr = bytes_from_hex(fixture["payload"]["sprHex"].as_str().unwrap());
    assert_eq!(semio_framework_hash::Sha256::digest(&pack), hash_from_hex(fixture["payload"]["packHash"].as_str().unwrap()));
    assert_eq!(semio_framework_hash::Sha256::digest(&spr), hash_from_hex(fixture["payload"]["sprHash"].as_str().unwrap()));
    assert_eq!(artifact_bootstrap_aggregate_hash(&pack, &spr), hash_from_hex(fixture["payload"]["aggregateHash"].as_str().unwrap()));
}

#[semio_framework_async_macros::async_test]
async fn artifact_bootstrap_frames_match_neutral_vectors() {
    let fixture = artifact_bootstrap_fixture();
    let inline = fixture_artifact_bootstrap(true);
    let chunked = fixture_artifact_bootstrap(false);
    let welcome = |bootstrap: ArtifactBootstrap| ServerFrame::Welcome {
        session_id: "session-bootstrap-1".to_string(),
        resume_token: "resume-bootstrap-1".to_string(),
        server_frontier: bootstrap.required_tail_frontier.clone(),
        bootstrap: Bootstrap::ArtifactBootstrap(Box::new(bootstrap)),
    };
    let inline_bytes = encode_server_frame(&welcome(inline), Lane::Command).await;
    let chunked_bytes = encode_server_frame(&welcome(chunked.clone()), Lane::Command).await;
    assert_eq!(inline_bytes, bytes_from_hex(fixture["wire"]["inlineWelcomeHex"].as_str().unwrap()));
    assert_eq!(chunked_bytes, bytes_from_hex(fixture["wire"]["chunkedWelcomeHex"].as_str().unwrap()));
    for (index, chunk) in fixture_artifact_chunks().iter().enumerate() {
        let frame = ServerFrame::ArtifactBootstrapChunk { descriptor_hash: chunked.descriptor_hash, index: index as u32, bytes: ArtifactBootstrapChunkBytes::try_from_slice(chunk).unwrap() };
        let bytes = encode_server_frame(&frame, Lane::Command).await;
        assert_eq!(bytes, bytes_from_hex(fixture["wire"]["chunkHex"][index].as_str().unwrap()));
        assert_eq!(decode_server_frame(&bytes).await.unwrap(), (Lane::Command, frame));
    }
    let done = ServerFrame::ArtifactBootstrapDone { descriptor_hash: chunked.descriptor_hash, chunk_count: chunked.chunk_count };
    let done_bytes = encode_server_frame(&done, Lane::Command).await;
    assert_eq!(done_bytes, bytes_from_hex(fixture["wire"]["doneHex"].as_str().unwrap()));
    assert_eq!(decode_server_frame(&done_bytes).await.unwrap(), (Lane::Command, done));
    assert_eq!(decode_server_frame(&inline_bytes).await.unwrap().1, welcome(fixture_artifact_bootstrap(true)));
    assert_eq!(decode_server_frame(&chunked_bytes).await.unwrap().1, welcome(chunked));
}

#[semio_framework_async_macros::async_test]
async fn artifact_bootstrap_rejects_malformed_transfers_atomically() {
    let valid = fixture_artifact_bootstrap(false);
    let chunks = fixture_artifact_chunks();
    let limits = ArtifactBootstrapLimits { max_total_bytes: 64, max_chunks: 4, max_chunk_bytes: 12 };
    let mut control = TestBootstrapControl::default();
    let mut unknown_version = valid.clone();
    unknown_version.format_version = 2;
    assert!(ArtifactBootstrapAssembler::new(unknown_version.clone(), valid.descriptor_hash, limits, Some(100), &mut control).unwrap_err().to_string().contains("version"));
    assert!(ArtifactBootstrapAssembler::new(valid.clone(), [9; 32], limits, Some(100), &mut control).unwrap_err().to_string().contains("descriptor"));
    let mut oversize = valid.clone();
    oversize.pack_length = 65;
    assert!(ArtifactBootstrapAssembler::new(oversize, valid.descriptor_hash, limits, Some(100), &mut control).unwrap_err().to_string().contains("bytes"));
    let mut backwards = valid.clone();
    backwards.required_tail_frontier.head_edit_ordinal = 6;
    assert!(ArtifactBootstrapAssembler::new(backwards, valid.descriptor_hash, limits, Some(100), &mut control).unwrap_err().to_string().contains("frontier"));
    let mut expired = TestBootstrapControl { now_ms: 100, ..Default::default() };
    assert!(ArtifactBootstrapAssembler::new(valid.clone(), valid.descriptor_hash, limits, Some(100), &mut expired).unwrap_err().to_string().contains("deadline"));
    for indices in [vec![1usize], vec![0, 0]] {
        let mut control = TestBootstrapControl::default();
        let mut assembler = ArtifactBootstrapAssembler::new(valid.clone(), valid.descriptor_hash, limits, Some(100), &mut control).unwrap();
        let error = indices.into_iter().find_map(|index| assembler.push(valid.descriptor_hash, index as u32, &chunks[index], &mut control).err()).unwrap();
        assert!(error.to_string().contains("index"));
        assert_eq!(assembler.retained_bytes(), 0);
    }
    let mut control = TestBootstrapControl::default();
    let mut missing = ArtifactBootstrapAssembler::new(valid.clone(), valid.descriptor_hash, limits, Some(100), &mut control).unwrap();
    missing.push(valid.descriptor_hash, 0, &chunks[0], &mut control).unwrap();
    assert!(missing.finish(Some((valid.descriptor_hash, valid.chunk_count)), &mut control).unwrap_err().to_string().contains("incomplete"));
    assert_eq!(missing.retained_bytes(), 0);
    let mut control = TestBootstrapControl::default();
    let mut oversized_chunk = ArtifactBootstrapAssembler::new(valid.clone(), valid.descriptor_hash, limits, Some(100), &mut control).unwrap();
    assert!(oversized_chunk.push(valid.descriptor_hash, 0, &[1; 13], &mut control).unwrap_err().to_string().contains("chunk"));
    assert_eq!(oversized_chunk.retained_bytes(), 0);
    let mut control = TestBootstrapControl::default();
    let mut wrong_descriptor = ArtifactBootstrapAssembler::new(valid.clone(), valid.descriptor_hash, limits, Some(100), &mut control).unwrap();
    assert!(wrong_descriptor.push([9; 32], 0, &[1], &mut control).unwrap_err().to_string().contains("descriptor"));
    assert_eq!(wrong_descriptor.retained_bytes(), 0);
    for field in 0..3 {
        let mut changed = valid.clone();
        match field {
            0 => changed.pack_hash = [0xaa; 32],
            1 => changed.spr_hash = [0xaa; 32],
            _ => changed.aggregate_hash = [0xaa; 32],
        }
        let mut control = TestBootstrapControl::default();
        let mut assembler = ArtifactBootstrapAssembler::new(changed, valid.descriptor_hash, limits, Some(100), &mut control).unwrap();
        for (index, chunk) in chunks.iter().enumerate() {
            assembler.push(valid.descriptor_hash, index as u32, chunk, &mut control).unwrap();
        }
        assert!(assembler.finish(Some((valid.descriptor_hash, valid.chunk_count)), &mut control).unwrap_err().to_string().contains("hash"));
        assert_eq!(assembler.retained_bytes(), 0);
    }
    let bytes = encode_server_frame(
        &ServerFrame::Welcome { session_id: "bad".to_string(), resume_token: "bad".to_string(), server_frontier: valid.required_tail_frontier.clone(), bootstrap: Bootstrap::ArtifactBootstrap(Box::new(unknown_version)) },
        Lane::Command,
    )
    .await;
    assert!(decode_server_frame(&bytes).await.unwrap_err().to_string().contains("version"));
}

#[semio_framework_async_macros::async_test]
async fn artifact_bootstrap_cancellation_is_atomic_and_restartable() {
    let fixture = artifact_bootstrap_fixture();
    let bootstrap = fixture_artifact_bootstrap(false);
    let chunks = fixture_artifact_chunks();
    let limits = ArtifactBootstrapLimits { max_total_bytes: 64, max_chunks: 4, max_chunk_bytes: 12 };
    let mut cancelled = TestBootstrapControl { cancelled_after_progress: Some(2), ..Default::default() };
    let mut first = ArtifactBootstrapAssembler::new(bootstrap.clone(), bootstrap.descriptor_hash, limits, Some(100), &mut cancelled).unwrap();
    first.push(bootstrap.descriptor_hash, 0, &chunks[0], &mut cancelled).unwrap();
    assert!(first.push(bootstrap.descriptor_hash, 1, &chunks[1], &mut cancelled).unwrap_err().to_string().contains("cancel"));
    assert_eq!(first.retained_bytes(), 0);
    assert_eq!(first.progress().received_bytes, 12);
    assert_eq!(cancelled.progress.iter().map(|progress| progress.received_bytes).collect::<Vec<_>>(), vec![0, 12]);
    let mut late_control = TestBootstrapControl { cancel_on_check: Some(6), ..Default::default() };
    let mut late = ArtifactBootstrapAssembler::new(bootstrap.clone(), bootstrap.descriptor_hash, limits, Some(100), &mut late_control).unwrap();
    for (index, chunk) in chunks.iter().enumerate() {
        late.push(bootstrap.descriptor_hash, index as u32, chunk, &mut late_control).unwrap();
    }
    assert!(late.finish(Some((bootstrap.descriptor_hash, bootstrap.chunk_count)), &mut late_control).unwrap_err().to_string().contains("cancel"));
    assert_eq!(late.retained_bytes(), 0);
    assert_eq!(late.progress().received_bytes, 33);
    let mut resumed_control = TestBootstrapControl::default();
    let mut resumed = ArtifactBootstrapAssembler::new(bootstrap.clone(), bootstrap.descriptor_hash, limits, Some(100), &mut resumed_control).unwrap();
    for (index, chunk) in chunks.iter().enumerate() {
        resumed.push(bootstrap.descriptor_hash, index as u32, chunk, &mut resumed_control).unwrap();
    }
    let pair = resumed.finish(Some((bootstrap.descriptor_hash, bootstrap.chunk_count)), &mut resumed_control).unwrap();
    assert_eq!(pair.pack, bytes_from_hex(fixture["payload"]["packHex"].as_str().unwrap()));
    assert_eq!(pair.spr, bytes_from_hex(fixture["payload"]["sprHex"].as_str().unwrap()));
    assert_eq!(resumed.retained_bytes(), 0);
    assert_eq!(resumed.progress().received_bytes, 33);
    assert!(resumed_control.progress.windows(2).all(|pair| pair[0].received_bytes <= pair[1].received_bytes));
}

#[semio_framework_async_macros::async_test]
async fn server_frame_commands_round_trips() {
    assert_server_round_trips(&ServerFrame::Commands { envelopes: vec![sample_envelope("op-1").await], origin: crate::ids::ActorId("actor-1".to_string()), frontier: sample_frontier().await }, Lane::Command).await;
}

#[semio_framework_async_macros::async_test]
async fn server_frame_ack_round_trips_for_every_stage_and_apply_outcome_variant() {
    for outcome in [ApplyOutcome::Accepted, ApplyOutcome::Transformed { envelope: Box::new(sample_envelope("op-1").await) }, ApplyOutcome::Rejected { reason: "conflict".to_string(), messages: vec![1, 2] }] {
        assert_server_round_trips(&ServerFrame::Ack { batch_id: 7, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(outcome) }], frontier: sample_frontier().await }, Lane::Command).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn server_frame_preview_round_trips() {
    assert_server_round_trips(&ServerFrame::Preview { actor: crate::ids::ActorId("actor-1".to_string()), key: "cursor".to_string(), seq: 3, payload: vec![5, 6] }, Lane::Preview).await;
}

#[semio_framework_async_macros::async_test]
async fn server_frame_presence_round_trips() {
    assert_server_round_trips(&ServerFrame::Presence { peers: vec![b"{\"id\":\"a\"}".to_vec(), b"{\"id\":\"b\"}".to_vec()] }, Lane::Preview).await;
}

#[semio_framework_async_macros::async_test]
async fn server_frame_credit_grant_round_trips() {
    assert_server_round_trips(&ServerFrame::CreditGrant { n: 32 }, Lane::Command).await;
}

#[semio_framework_async_macros::async_test]
async fn server_frame_error_round_trips() {
    assert_server_round_trips(&ServerFrame::Error { code: "rejected".to_string(), message: "bad batch".to_string() }, Lane::Command).await;
}

#[semio_framework_async_macros::async_test]
async fn server_frame_session_round_trips() {
    assert_server_round_trips(&ServerFrame::Session { actor: "actor-1".to_string(), color: 7 }, Lane::Command).await;
}
//#endregion 🔖️ServerFrame

//#region 🔖️Codec
#[semio_framework_async_macros::async_test]
async fn decode_client_frame_rejects_empty_bytes() {
    let err = decode_client_frame(&[]).await.unwrap_err();
    assert!(matches!(err, crate::ProtocolError::Malformed { what: "wire frame", .. }));
}

#[semio_framework_async_macros::async_test]
async fn decode_client_frame_rejects_unknown_lane_byte() {
    let err = decode_client_frame(&[2u8, 0]).await.unwrap_err();
    assert!(matches!(err, crate::ProtocolError::Malformed { what: "wire frame lane byte", .. }));
}

#[semio_framework_async_macros::async_test]
async fn decode_client_frame_rejects_unknown_tag() {
    let bytes = vec![Lane::Command.to_byte().await, 0xFF];
    let err = decode_client_frame(&bytes).await.unwrap_err();
    assert!(matches!(err, crate::ProtocolError::Malformed { what: "wire client-frame tag", .. }));
}

#[semio_framework_async_macros::async_test]
async fn decode_server_frame_rejects_unknown_tag() {
    let bytes = vec![Lane::Command.to_byte().await, 0xFF];
    let err = decode_server_frame(&bytes).await.unwrap_err();
    assert!(matches!(err, crate::ProtocolError::Malformed { what: "wire server-frame tag", .. }));
}

#[semio_framework_async_macros::async_test]
async fn decode_client_frame_rejects_truncated_field() {
    let bytes = encode_client_frame(&ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] }, Lane::Preview).await;
    let truncated = &bytes[..bytes.len() - 2];
    assert!(decode_client_frame(truncated).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn decode_server_frame_rejects_truncated_field() {
    let bytes = encode_server_frame(&ServerFrame::Error { code: "rejected".to_string(), message: "bad batch".to_string() }, Lane::Command).await;
    let truncated = &bytes[..bytes.len() - 3];
    assert!(decode_server_frame(truncated).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn decode_client_frame_rejects_empty_body_after_lane() {
    let err = decode_client_frame(&[Lane::Command.to_byte().await]).await.unwrap_err();
    assert!(matches!(err, crate::ProtocolError::Malformed { what: "wire client-frame tag", .. }));
}

#[semio_framework_async_macros::async_test]
async fn different_lanes_produce_different_leading_bytes_but_same_body() {
    let command_bytes = encode_client_frame(&ClientFrame::Bye, Lane::Command).await;
    let preview_bytes = encode_client_frame(&ClientFrame::Bye, Lane::Preview).await;
    assert_eq!(command_bytes[0], 0);
    assert_eq!(preview_bytes[0], 1);
    assert_eq!(command_bytes[1..], preview_bytes[1..]);
}
//#endregion 🔖️Codec
