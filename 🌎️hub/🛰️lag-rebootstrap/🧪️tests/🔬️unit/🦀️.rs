
use super::*;
use directory::{DslValue, FromValue};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

struct TestControl {
    now: AtomicU64,
    cancelled: AtomicBool,
    progress: Mutex<Vec<RebootstrapProgress>>,
}

impl RebootstrapTransferControl for TestControl {
    fn now_ms(&self) -> u64 {
        self.now.load(Ordering::SeqCst)
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    fn report(&self, progress: RebootstrapProgress) {
        self.progress.lock().expect("progress lock").push(progress);
    }
}

fn test_control() -> TestControl {
    TestControl { now: AtomicU64::new(1), cancelled: AtomicBool::new(false), progress: Mutex::new(Vec::new()) }
}

fn canonical_pair_fixture() -> VerifiedActiveCheckpointPair {
    let pack: Vec<u8> = (0..5_001).map(|index| ((index * 17 + 3) % 256) as u8).collect();
    let spr: Vec<u8> = (0..4_097).map(|index| ((index * 29 + 7) % 256) as u8).collect();
    let mut aggregate = Sha256::new();
    aggregate.update(&pack);
    aggregate.update(&spr);
    VerifiedActiveCheckpointPair {
        selection: CanonicalCheckpointPairSelection {
            scope: DocumentScope::new("space:alpha", "doc:tokyo"),
            descriptor_digest_v1: ArtifactHash([0x11; 32]),
            active_checkpoint_id: ArtifactHash([0x22; 32]),
            baseline_frontier: ArtifactFrontier { document_id: "doc:tokyo".into(), head_edit_ordinal: 7, head_edit_id: "edit:7".into(), last_commit_seq: 6, chain_hash: ArtifactHash([0x33; 32]) },
            pack: PublishedArtifactBlob { sha256: ArtifactHash(Sha256::digest(&pack)), byte_length: pack.len() as u64 },
            spr: PublishedArtifactBlob { sha256: ArtifactHash(Sha256::digest(&spr)), byte_length: spr.len() as u64 },
            aggregate_sha256: ArtifactHash(aggregate.finalize()),
        },
        pair: ArtifactPair { pack, spr },
    }
}

fn frontier_from_fixture(value: &serde_json::Value) -> ArtifactFrontier {
    let encoded = value["chainHash"].as_str().expect("chain hash").as_bytes();
    let mut chain_hash = [0u8; 32];
    for (index, slot) in chain_hash.iter_mut().enumerate() {
        let nibble = |byte: u8| if byte <= b'9' { byte - b'0' } else { byte - b'a' + 10 };
        *slot = nibble(encoded[index * 2]) << 4 | nibble(encoded[index * 2 + 1]);
    }
    ArtifactFrontier {
        document_id: value["documentId"].as_str().expect("document id").to_owned(),
        head_edit_ordinal: value["headEditOrdinal"].as_u64().expect("head ordinal"),
        head_edit_id: value["headEditId"].as_str().expect("head id").to_owned(),
        last_commit_seq: value["lastCommitSeq"].as_u64().expect("commit sequence"),
        chain_hash: ArtifactHash(chain_hash),
    }
}

fn encode_unchecked_frontier(pair: &VerifiedActiveCheckpointPair, context: &RebootstrapContext<'_>) -> Vec<u8> {
    let mut output = Vec::new();
    append_frame(&mut output, &canonical_pair_header_payload_unchecked(&pair.selection).expect("unchecked header")).expect("header frame");
    for ordinal in 0..pair.data_record_count() {
        let record = pair.data_record(ordinal, context).expect("data record").expect("record present");
        append_canonical_pair_data(&mut output, &record).expect("data frame");
    }
    append_canonical_pair_terminal(&mut output, CanonicalPairTerminal::Complete).expect("terminal frame");
    output
}

#[test]
fn canonical_pair_preflight_is_before_allocation_and_record_bounded() {
    assert_eq!(preflight_pair(4_096, AUTHORITY_MAX_PAIR_BYTES - 4_096), Ok(AUTHORITY_MAX_PAIR_BYTES));
    assert_eq!(preflight_pair(1, AUTHORITY_MAX_PAIR_BYTES - 1), Err(RebootstrapError::ResourceLimit));
    assert_eq!(preflight_pair(0, 1), Err(RebootstrapError::Integrity));
    assert_eq!(preflight_pair(u64::MAX, 1), Err(RebootstrapError::ResourceLimit));
}

#[test]
fn canonical_pair_neutral_framing_is_pack_then_spr_terminal_and_fail_closed() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪢️canonical-pair/🔣️.json")).expect("canonical pair fixture");
    let pair = canonical_pair_fixture();
    let control = test_control();
    let context = RebootstrapContext::new(100, &control);
    let encoded = encode_verified_canonical_pair(&pair, &context).expect("encode canonical pair");
    let decoded = decode_canonical_checkpoint_pair(&encoded).expect("decode canonical pair");
    assert_eq!(decoded.selection, pair.selection);
    assert_eq!(decoded.pair(), pair.pair());
    assert_eq!(decoded.data_record_count(), fixture["expected"]["dataRecords"].as_u64().expect("data records") as u32);
    assert_eq!(encoded.len(), fixture["expected"]["wireBytes"].as_u64().expect("wire bytes") as usize);
    assert_eq!(hex_lower(&decoded.selection.pack.sha256.0), fixture["pack"]["sha256"]);
    assert_eq!(hex_lower(&decoded.selection.spr.sha256.0), fixture["spr"]["sha256"]);
    assert_eq!(hex_lower(&decoded.selection.aggregate_sha256.0), fixture["expected"]["aggregateSha256"]);
    assert_eq!(canonical_pair_etag(&decoded.selection).expect("etag"), fixture["expected"]["etag"]);

    let mut no_terminal = encoded.clone();
    no_terminal.truncate(no_terminal.len() - 6);
    assert!(decode_canonical_checkpoint_pair(&no_terminal).is_err());
    let mut wrong_ordinal = encoded.clone();
    let header_length = u32::from_be_bytes(wrong_ordinal[..4].try_into().expect("header length")) as usize;
    wrong_ordinal[4 + header_length + 4 + 2 + 3] ^= 1;
    assert!(matches!(decode_canonical_checkpoint_pair(&wrong_ordinal), Err(RebootstrapError::Integrity)));
    let mut trailing = encoded;
    trailing.push(0);
    assert!(matches!(decode_canonical_checkpoint_pair(&trailing), Err(RebootstrapError::Integrity)));
}

#[test]
fn canonical_pair_baseline_admits_exact_genesis_or_edited_before_pair_allocation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪢️canonical-pair/🔣️.json")).expect("canonical pair fixture");
    let control = test_control();
    let context = RebootstrapContext::new(100, &control);
    for row in fixture["frontierCases"].as_array().expect("frontier cases") {
        let mut pair = canonical_pair_fixture();
        pair.selection.baseline_frontier = frontier_from_fixture(row);
        let accepted = row["accepted"].as_bool().expect("accepted");
        assert_eq!(canonical_pair_frontier_is_exact(&pair.selection.scope, &pair.selection.baseline_frontier), accepted, "{}", row["id"]);
        if accepted {
            let wire = encode_verified_canonical_pair(&pair, &context).expect("accepted pair");
            assert_eq!(decode_canonical_checkpoint_pair(&wire).expect("accepted decode").selection.baseline_frontier, pair.selection.baseline_frontier, "{}", row["id"]);
        } else {
            assert_eq!(encode_verified_canonical_pair(&pair, &context), Err(RebootstrapError::Integrity), "{}", row["id"]);
            assert!(matches!(decode_canonical_checkpoint_pair(&encode_unchecked_frontier(&pair, &context)), Err(RebootstrapError::Integrity)), "{}", row["id"]);
        }
    }
}

#[test]
fn canonical_pair_encoding_cancels_without_terminal_acceptance() {
    let pair = canonical_pair_fixture();
    let control = test_control();
    control.cancelled.store(true, Ordering::SeqCst);
    let context = RebootstrapContext::new(100, &control);
    assert!(matches!(encode_verified_canonical_pair(&pair, &context), Err(RebootstrapError::Cancelled)));
    control.cancelled.store(false, Ordering::SeqCst);
    control.now.store(100, Ordering::SeqCst);
    assert!(matches!(encode_verified_canonical_pair(&pair, &context), Err(RebootstrapError::DeadlineExceeded)));
}

#[tokio::test]
async fn lag_rebootstrap_neutral_wire_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛟️lag-rebootstrap/🔣️.json")).expect("fixture JSON");
    let message = directory::os_directory::DirectoryStreamMessage::from_value(DslValue::from(fixture["control"].clone())).expect("directory control");
    let directory::os_directory::DirectoryStreamMessage::RebootstrapRequired { control } = message else { panic!("typed rebootstrap") };
    let public = directory::os_pack::json::to_json_string(&directory::os_directory::DirectoryStreamMessage::RebootstrapRequired { control: control.clone() });
    assert!(!public.contains("storageKey"));
    let frame = protocol::ServerFrame::RebootstrapRequired {
        control: protocol::RebootstrapRequired {
            space_id: control.scope.space_id,
            document_id: control.scope.document_id,
            checkpoint_id: control.checkpoint_id.0,
            descriptor_hash: control.descriptor_digest_v1.0,
            baseline_frontier: wire_frontier(&control.baseline_frontier),
        },
    };
    let encoded = protocol::encode_server_frame(&frame, protocol::Lane::Command).await;
    assert_eq!(encoded.get(1), Some(&12));
    assert_eq!(protocol::decode_server_frame(&encoded).await.expect("decode"), (protocol::Lane::Command, frame));
    assert_eq!(fixture["closeCode"].as_u64(), Some(1013));
    assert_eq!(fixture["closeReason"].as_str(), Some("rebootstrap-required"));
    assert_eq!(fixture["scopeMaximumBytes"].as_u64(), Some(REBOOTSTRAP_SCOPE_MAX_BYTES as u64));
    assert_eq!(fixture["inlineMaximumBytes"].as_u64(), Some(ARTIFACT_BOOTSTRAP_CHUNK_BYTES as u64));
    assert_eq!(fixture["chunkMaximumBytes"].as_u64(), Some(ARTIFACT_BOOTSTRAP_CHUNK_BYTES as u64));
    assert_eq!(fixture["totalMaximumBytes"].as_u64(), Some(ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES));
    assert_eq!(fixture["chunkMaximumCount"].as_u64(), Some(u64::from(ARTIFACT_BOOTSTRAP_MAX_CHUNKS)));
}

#[test]
fn chunk_boundary_progress_cancel_and_deadline_are_bounded() {
    let control = test_control();
    let context = RebootstrapContext::new(100, &control);
    let scope = DocumentScope::new("space", "document");
    let frontier = ArtifactFrontier { document_id: "document".into(), head_edit_ordinal: 1, head_edit_id: "edit-1".into(), last_commit_seq: 1, chain_hash: ArtifactHash([3; 32]) };
    let public_control = RebootstrapRequired { scope, checkpoint_id: ArtifactHash([1; 32]), descriptor_digest_v1: ArtifactHash([2; 32]), baseline_frontier: frontier.clone() };
    let pair = ArtifactPair { pack: vec![4; ARTIFACT_BOOTSTRAP_CHUNK_BYTES], spr: vec![5] };
    let bootstrap = ArtifactBootstrap {
        format_version: ARTIFACT_BOOTSTRAP_FORMAT_VERSION,
        descriptor_hash: [2; 32],
        artifact_schema: "fixture@1".into(),
        artifact_kind: "fixture".into(),
        pack_schema_hash: [6; 32],
        baseline_frontier: wire_frontier(&frontier),
        pack_hash: [7; 32],
        spr_hash: [8; 32],
        pack_length: ARTIFACT_BOOTSTRAP_CHUNK_BYTES as u64,
        spr_length: 1,
        chunk_count: 2,
        aggregate_hash: [9; 32],
        required_tail_frontier: wire_frontier(&frontier),
        inline: None,
    };
    let transfer = VerifiedArtifactBootstrap { control: public_control, bootstrap, pair: Some(pair) };
    assert_eq!(transfer.chunk(0, &context).expect("first").expect("first chunk").len(), ARTIFACT_BOOTSTRAP_CHUNK_BYTES);
    assert_eq!(transfer.chunk(1, &context).expect("second").expect("second chunk").len(), 1);
    assert!(transfer.chunk(2, &context).expect("end").is_none());
    control.cancelled.store(true, Ordering::SeqCst);
    assert_eq!(transfer.chunk(0, &context), Err(RebootstrapError::Cancelled));
    control.cancelled.store(false, Ordering::SeqCst);
    control.now.store(100, Ordering::SeqCst);
    assert_eq!(transfer.chunk(0, &context), Err(RebootstrapError::DeadlineExceeded));
    assert_eq!(control.progress.lock().expect("progress lock").len(), 2);
}
