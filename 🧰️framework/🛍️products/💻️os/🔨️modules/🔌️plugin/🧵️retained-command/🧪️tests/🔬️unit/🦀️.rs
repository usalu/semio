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

//#region 🧮️WorkCapacity
const WORK_CAPACITY_FIXTURE_JSON: &str = include_str!("../../🧫️fixtures/🧮️work-capacity.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkCapacityFixture {
    format: String,
    version: u8,
    invertible_rows_per_item: usize,
    cases: Vec<WorkCapacityCase>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkCapacityCase {
    name: String,
    invertible_items: usize,
    items: usize,
    rows: usize,
    work_items: usize,
    admitted: bool,
}

/// 🧮️ The declaration law, language-agnostic: for every declared route capacity the rows a gesture
/// folds, the ceiling its payload carries as `maximum_work_items`, and the admission preflight
/// applies are ONE quantity read three ways. Every row of the fixture is checked against the store's
/// own invertible row cost, so a change to either side breaks here first
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn one_declared_capacity_answers_rows_ceiling_and_admission() {
    let fixture: WorkCapacityFixture = serde_json::from_str(WORK_CAPACITY_FIXTURE_JSON).expect("work-capacity fixture parses");
    assert_eq!(fixture.format, "semio.retained-command.work-capacity");
    assert_eq!(fixture.version, 1);
    assert_eq!(fixture.invertible_rows_per_item, store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS, "the fixture states the store's own invertible row cost");
    for case in &fixture.cases {
        let capacity = ArtifactRetainedWorkCapacity::for_invertible_items(case.invertible_items);
        assert_eq!(capacity.invertible_items(), case.invertible_items, "{}", case.name);
        assert_eq!(capacity.rows(case.items), case.rows, "{}", case.name);
        assert_eq!(capacity.work_items(), case.work_items, "{}", case.name);
        assert_eq!(capacity.admits(case.rows), case.admitted, "{}", case.name);
        assert_eq!(capacity.rows_for_items(case.items), case.admitted.then_some(case.rows), "{}", case.name);
        eprintln!("[DEBUG] work capacity {}: items={} rows={} maximum_work_items={} admitted={}", case.name, case.items, case.rows, case.work_items, case.admitted);
    }
}

/// 🧾️ The law the runtime broke: a route whose one-item store FOOTPRINT is N rows must admit an
/// EXTENT of N. Those were two literals in two units before this lane — the footprint counted rows
/// (two, since `📓️fold-contract-2026-09-10.md`) and the extent counted items (one) — so nothing
/// compared them and preflight measured a quantity the store never used.
#[test]
fn a_route_footprint_of_n_rows_admits_an_extent_of_n() {
    for invertible_items in [1_usize, 2, 32, 4_096] {
        let capacity = ArtifactRetainedWorkCapacity::for_invertible_items(invertible_items);
        let footprint = capacity.one_item_footprint(0);
        assert!(footprint.is_admissible(), "the store admits its own one-item footprint");
        assert_eq!(footprint.work_items, capacity.rows(1), "the footprint's rows ARE the capacity's rows for one item");
        assert!(capacity.admits(footprint.work_items), "a route whose footprint is {} rows must admit an extent of {}", footprint.work_items, footprint.work_items);
        assert_eq!(capacity.rows_for_items(1), Some(footprint.work_items), "one durable item's extent IS its footprint");
    }
}

/// 🚫️ The hostile control, pinning the exact defect shape: an item-counted extent against a
/// row-counted footprint under-declares by exactly the inverse rows, and a capacity of zero items
/// admits nothing at all rather than admitting everything.
#[test]
fn an_item_counted_extent_cannot_stand_in_for_a_row_counted_footprint() {
    let capacity = ArtifactRetainedWorkCapacity::for_invertible_items(32);
    assert_ne!(capacity.rows(1), 1, "one point-invertible item never costs one row");
    assert_eq!(capacity.rows(1), capacity.one_item_footprint(0).work_items);
    let empty = ArtifactRetainedWorkCapacity::for_invertible_items(0);
    assert_eq!(empty.work_items(), 0);
    assert!(!empty.admits(1), "a route that declares no items admits no extent");
    assert_eq!(empty.rows_for_items(1), None);
}
//#endregion 🧮️WorkCapacity
