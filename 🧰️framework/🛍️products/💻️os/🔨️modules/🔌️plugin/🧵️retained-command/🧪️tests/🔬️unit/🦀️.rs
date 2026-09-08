use super::*;

const CHECKPOINT_FIXTURE_JSON: &str = include_str!("../../🧫️fixtures/📸️artifact-command-checkpoint.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheckpointFixture {
    format: String,
    version: u8,
    maximum_bytes: usize,
    header_bytes: usize,
    cases: Vec<CheckpointCase>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheckpointCase {
    name: String,
    work_phase: bool,
    raw_page_cursor: u64,
    raw_bytes: u64,
    work_progress: u64,
    context_digest: u64,
    workspace_identity: u64,
    work_state: CheckpointWorkState,
    outcome: String,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum CheckpointWorkState {
    Bytes { bytes: Vec<u8> },
    Fill { fill: u8, length: usize },
}

impl CheckpointWorkState {
    fn materialize(&self) -> Vec<u8> {
        match self {
            Self::Bytes { bytes } => bytes.clone(),
            Self::Fill { fill, length } => vec![*fill; *length],
        }
    }
}

fn write_owned_little_endian_u64(target: &mut Vec<u8>, value: u64) {
    for shift in (0..64).step_by(8) {
        target.push((value >> shift) as u8);
    }
}

fn owned_checkpoint_oracle(case: &CheckpointCase, work: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES + work.len());
    encoded.extend_from_slice(b"ARC1");
    encoded.push(3);
    encoded.push(u8::from(case.work_phase));
    encoded.extend_from_slice(&[0, 0]);
    write_owned_little_endian_u64(&mut encoded, case.raw_page_cursor);
    write_owned_little_endian_u64(&mut encoded, case.raw_bytes);
    write_owned_little_endian_u64(&mut encoded, case.work_progress);
    write_owned_little_endian_u64(&mut encoded, case.context_digest);
    write_owned_little_endian_u64(&mut encoded, case.workspace_identity);
    encoded.extend_from_slice(work);
    encoded
}

#[test]
fn checkpoint_binary_matches_schema_fixture_and_owned_oracle() {
    let fixture: CheckpointFixture = serde_json::from_str(CHECKPOINT_FIXTURE_JSON).expect("checkpoint fixture shape");
    assert_eq!(fixture.format, "ARC1");
    assert_eq!(fixture.version, 3);
    assert_eq!(fixture.maximum_bytes, ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES);
    assert_eq!(fixture.header_bytes, ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES);
    for case in fixture.cases {
        let work = case.work_state.materialize();
        let mut bytes = [0_u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES];
        let encoded = encode_artifact_command_checkpoint(
            ArtifactCommandCheckpoint {
                work_phase: case.work_phase,
                raw_page_cursor: case.raw_page_cursor,
                raw_bytes: case.raw_bytes,
                work_progress: case.work_progress,
                context_digest: case.context_digest,
                workspace_identity: case.workspace_identity,
                work: &work,
            },
            &mut bytes,
        );
        if case.outcome == "capacityError" {
            assert!(encoded.is_err(), "{} must reject maximum plus one", case.name);
            continue;
        }
        assert_eq!(case.outcome, "ok", "unknown fixture outcome for {}", case.name);
        let length = encoded.unwrap_or_else(|error| panic!("{} unexpectedly rejected: {error:?}", case.name));
        let oracle = owned_checkpoint_oracle(&case, &work);
        assert_eq!(&bytes[..length], oracle, "{} differs from owned oracle", case.name);
        let decoded = decode_artifact_command_checkpoint(&bytes[..length], case.raw_bytes as usize, case.raw_page_cursor as usize, case.raw_bytes as usize, case.context_digest, case.workspace_identity)
            .unwrap_or_else(|error| panic!("{} decode rejected: {error:?}", case.name));
        assert_eq!(decoded.work_phase, case.work_phase);
        assert_eq!(decoded.raw_page_cursor, case.raw_page_cursor);
        assert_eq!(decoded.raw_bytes, case.raw_bytes);
        assert_eq!(decoded.work_progress, case.work_progress);
        assert_eq!(decoded.context_digest, case.context_digest);
        assert_eq!(decoded.workspace_identity, case.workspace_identity);
        assert_eq!(decoded.work, work);
    }
}

#[test]
fn owned_little_endian_oracle_preserves_every_hostile_byte_lane() {
    let mut bytes = Vec::new();
    write_owned_little_endian_u64(&mut bytes, 0x8877_6655_4433_2211);
    write_owned_little_endian_u64(&mut bytes, u64::MAX);
    assert_eq!(bytes, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
}

#[test]
fn checkpoint_decode_rejects_context_workspace_and_reserved_byte_drift() {
    let mut bytes = [0_u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES];
    let length =
        encode_artifact_command_checkpoint(ArtifactCommandCheckpoint { work_phase: true, raw_page_cursor: 2, raw_bytes: 8_192, work_progress: 7, context_digest: 9, workspace_identity: 11, work: &[1, 0xfe] }, &mut bytes).expect("exact checkpoint");
    assert!(decode_artifact_command_checkpoint(&bytes[..length], 8_192, 2, 8_192, 10, 11).is_err());
    assert!(decode_artifact_command_checkpoint(&bytes[..length], 8_192, 2, 8_192, 9, 12).is_err());
    bytes[6] = 1;
    assert!(decode_artifact_command_checkpoint(&bytes[..length], 8_192, 2, 8_192, 9, 11).is_err());
}
