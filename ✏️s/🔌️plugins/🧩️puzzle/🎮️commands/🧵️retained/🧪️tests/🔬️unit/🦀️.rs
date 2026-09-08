
use super::*;
use serde_json::Value;

const VECTOR_IDS: &[&str] =
    &["zero", "max", "maxPlusOne", "malformed", "staleGeneration", "wrongOperation", "abaGeneration", "cancelWirePage", "cancelWireByte", "cancelPreflight", "cancelWork", "cancelPublish", "faultWork", "retry", "close", "replay"];

const CHECKPOINT_VECTOR_IDS: &[&str] = &["checkpointEmpty", "checkpointSingle", "checkpointMax", "checkpointMaxPlusOne", "checkpointCorrupt", "checkpointInterruptedClose"];

fn checkpoint_state(phase: PuzzleCommandPhase) -> PuzzleCommandCheckpointState {
    PuzzleCommandCheckpointState {
        phase,
        operation: Operation::new(semio_framework_job::OperationId(7), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 13),
        tool_hash: puzzle_checkpoint_hash([b"forceLayout".as_slice()]),
        input_hash: puzzle_checkpoint_hash([b"wire".as_slice()]),
        raw_len: 4,
        raw_page_cursor: 1,
        raw_scan_cursor: 4,
        work_extent: 9,
        preflight_cursor: 9,
        work_cursor: 2,
    }
}

#[test]
fn checkpoint_codec_is_exact_fixed_capacity_and_rejects_empty_single_max_plus_one_and_corruption() {
    let state = checkpoint_state(PuzzleCommandPhase::WorkProgress);
    let bytes = state.encode();
    assert_eq!(bytes.len(), PUZZLE_COMMAND_CHECKPOINT_BYTES);
    assert_eq!(PuzzleCommandCheckpointState::decode(&bytes), Some(state));
    assert!(PuzzleCommandCheckpointState::decode(&[]).is_none());
    assert!(PuzzleCommandCheckpointState::decode(&bytes[..1]).is_none());
    let mut max_plus_one = bytes.to_vec();
    max_plus_one.push(0);
    assert!(PuzzleCommandCheckpointState::decode(&max_plus_one).is_none());
    let mut corrupt = bytes;
    corrupt[0] ^= 0xff;
    assert!(PuzzleCommandCheckpointState::decode(&corrupt).is_none());
}

#[test]
fn checkpoint_codec_preserves_custom_work_cursor_and_never_encodes_terminal_close_state() {
    let state = checkpoint_state(PuzzleCommandPhase::WorkProgress);
    let restored = PuzzleCommandCheckpointState::decode(&state.encode()).expect("checkpoint state");
    assert_eq!((restored.work_extent, restored.preflight_cursor, restored.work_cursor), (9, 9, 2));
    assert!(!matches!(restored.phase, PuzzleCommandPhase::Publish | PuzzleCommandPhase::Complete | PuzzleCommandPhase::Fault));
}

#[test]
fn interrupted_checkpoint_close_recursively_retires_both_fixed_page_owners() {
    let source = include_str!("../../🦀️.rs");
    assert!(source.contains("input.begin_close();"));
    assert!(source.contains("checkpoint.begin_close();"));
    assert!(source.contains("input.close_step(maximum_items.min(1), maximum_bytes)"));
    assert!(source.contains("checkpoint.close_step(maximum_items.min(1), maximum_bytes)"));
    assert!(source.contains("self.raw_input.is_none()"));
    assert!(source.contains("self.checkpoint_input.is_none()"));
    assert!(source.contains("self.restore_target = None;"));
}

#[test]
fn completion_rejection_is_retained_and_child_closed_before_typed_puzzle_owners() {
    let source = include_str!("../../🦀️.rs");
    let retained = source.find("self.pending_completion_rejection = Some(rejected)").expect("returned completion rejection owner");
    let close = source.find("emit.close_child_one(maximum_items, maximum_bytes)").expect("bounded child-first rejection closer");
    let work = source[close..].find("if let Some(work) = self.work.as_mut()").map(|offset| close + offset).expect("ordinary work owner close");
    assert!(retained < close && close < work);
    assert!(source.contains("&& self.pending_completion_rejection.is_none()"));
    assert!(!source.contains("completion.complete(Ok(emit), EphemeralEmit::default()).is_err()"));
}

#[test]
fn retained_replay_fails_closed_when_a_custom_cursor_cannot_be_reconstructed() {
    let source = include_str!("../../🦀️.rs");
    assert!(source.contains("fn restore_has_passed_target"));
    assert!(source.contains("puzzle command checkpoint replay diverged"));
    assert!(source.contains("self.work_cursor > target.work_cursor"));
}

#[test]
fn checkpoint_page_backpressure_cannot_advance_any_retained_cursor() {
    let source = include_str!("../../🦀️.rs");
    assert!(source.contains("checkpoint_pending: bool"));
    assert!(source.contains("if self.checkpoint_pending {\n            return self.publish_checkpoint(cx);"));
    assert!(source.contains("Err(rejected) => {\n                drop(rejected.into_source());\n                return StepOutcome::Yield;"));
    assert!(source.contains("self.checkpoint_pending = false;\n        StepOutcome::CheckpointReady"));
}

#[test]
fn every_puzzle_factory_validates_and_adopts_the_exact_checkpoint_owner() {
    for source in [include_str!("../../../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"), include_str!("../../../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs")]
    {
        assert!(source.contains("RetainedPuzzleCommandJob::validate_wire_checkpoint(operation, &payload, &input, &checkpoint)"));
        assert!(source.contains("RetainedPuzzleCommandJob::from_validated_wire_checkpoint(operation, payload, input, checkpoint)"));
        assert!(!source.contains("if checkpoint.is_some() || input.declared_bytes()"));
    }
    let puzzle2d = include_str!("../../../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
    assert!(!puzzle2d.contains("impl semio_framework_plugin::ArtifactOwnedToolJobFactory for BoundedFirstStepCommandJobFactory"));
    assert!(!puzzle2d.contains("registry.register(BoundedFirstStepCommandJobFactory"));
}

#[derive(Debug, PartialEq, Eq)]
struct PuzzleRetainedOracleOutput {
    owner: String,
    document_schema: String,
    payload_schema: String,
    tool_ids: Vec<String>,
    evidence_tool_ids: Vec<String>,
    capacities: [u64; 6],
    locales: Vec<String>,
    vector_ids: Vec<String>,
    fingerprints: Vec<String>,
}

trait PuzzleRetainedFixtureOracle {
    fn evaluate(&self, fixture: &str) -> Result<PuzzleRetainedOracleOutput, String>;
}

struct SerdeJsonFixtureOracle;

impl PuzzleRetainedFixtureOracle for SerdeJsonFixtureOracle {
    fn evaluate(&self, fixture: &str) -> Result<PuzzleRetainedOracleOutput, String> {
        let root: Value = serde_json::from_str(fixture).map_err(|error| error.to_string())?;
        let text = |key: &str| root.get(key).and_then(Value::as_str).map(str::to_string).ok_or_else(|| format!("fixture lacks {key}"));
        let capacities = root.get("capacities").ok_or_else(|| "fixture lacks capacities".to_string())?;
        let cap = |key: &str| capacities.get(key).and_then(Value::as_u64).ok_or_else(|| format!("fixture lacks capacity {key}"));
        let strings = |key: &str| {
            root.get(key).and_then(Value::as_array).ok_or_else(|| format!("fixture lacks {key}"))?.iter().map(|value| value.as_str().map(str::to_string).ok_or_else(|| format!("fixture {key} contains a non-string"))).collect::<Result<Vec<_>, _>>()
        };
        let vectors = root.get("vectors").and_then(Value::as_array).ok_or_else(|| "fixture lacks vectors".to_string())?;
        let vector_ids = vectors.iter().map(|vector| vector.get("id").and_then(Value::as_str).map(str::to_string).ok_or_else(|| "fixture vector lacks id".to_string())).collect::<Result<Vec<_>, _>>()?;
        let fingerprints = vectors.iter().take(VECTOR_IDS.len()).map(|vector| vector.get("fingerprint").and_then(Value::as_str).map(str::to_string).ok_or_else(|| "fixture base vector lacks fingerprint".to_string())).collect::<Result<Vec<_>, _>>()?;
        let locales = root.get("locales").and_then(Value::as_object).ok_or_else(|| "fixture lacks locales".to_string())?.keys().cloned().collect();
        Ok(PuzzleRetainedOracleOutput {
            owner: text("owner")?,
            document_schema: text("documentSchema")?,
            payload_schema: text("payloadSchema")?,
            tool_ids: strings("toolIds")?,
            evidence_tool_ids: strings("evidenceToolIds")?,
            capacities: [cap("rawBytes")?, cap("decodedItems")?, cap("workItems")?, cap("outputBytes")?, cap("stepMicros")?, cap("semanticUnitsPerGrant")?],
            locales,
            vector_ids,
            fingerprints,
        })
    }
}

fn expected(owner: &str, document_schema: &str, tools: &[&str]) -> PuzzleRetainedOracleOutput {
    PuzzleRetainedOracleOutput {
        owner: owner.into(),
        document_schema: document_schema.into(),
        payload_schema: format!("{document_schema}.tool-command.v1"),
        tool_ids: tools.iter().map(|tool| (*tool).to_string()).collect(),
        evidence_tool_ids: Vec::new(),
        capacities: [PUZZLE_COMMAND_RAW_BYTES as u64, PUZZLE_COMMAND_DECODED_ITEMS as u64, PUZZLE_COMMAND_WORK_ITEMS as u64, PUZZLE_COMMAND_OUTPUT_BYTES as u64, PUZZLE_COMMAND_STEP_MICROS as u64, 1],
        locales: vec!["de".into(), "en".into()],
        vector_ids: VECTOR_IDS.iter().map(|vector| (*vector).to_string()).collect(),
        fingerprints: VECTOR_IDS
            .iter()
            .map(|vector| {
                if *vector == "maxPlusOne" {
                    "8193:0:0:0:0:0"
                } else if *vector == "malformed" {
                    "1:0:0:0:0:0"
                } else if matches!(*vector, "staleGeneration" | "wrongOperation" | "abaGeneration") {
                    "1:1:1:0:0:0"
                } else {
                    "0:0:0:0:0:0"
                }
                .to_string()
            })
            .collect(),
    }
}

fn assert_fixture(fixture: &str, expected: PuzzleRetainedOracleOutput) {
    let oracle = SerdeJsonFixtureOracle;
    let actual = oracle.evaluate(fixture).expect("third-party fixture oracle parses");
    assert_eq!(actual.owner, expected.owner);
    assert_eq!(actual.document_schema, expected.document_schema);
    assert_eq!(actual.payload_schema, expected.payload_schema);
    assert_eq!(actual.tool_ids, expected.tool_ids);
    assert_eq!(actual.capacities, expected.capacities);
    assert_eq!(actual.locales, expected.locales);
    assert!(actual.vector_ids.starts_with(&expected.vector_ids));
    assert!(CHECKPOINT_VECTOR_IDS.iter().all(|id| actual.vector_ids.iter().any(|actual| actual == id)));
    assert_eq!(actual.fingerprints, expected.fingerprints);
    assert_eq!(actual.vector_ids.iter().collect::<std::collections::BTreeSet<_>>().len(), actual.vector_ids.len());
    let root: Value = serde_json::from_str(fixture).expect("fixture parses");
    let checkpoint_vector = |id: &str| root.get("vectors").and_then(Value::as_array).and_then(|vectors| vectors.iter().find(|vector| vector.get("id").and_then(Value::as_str) == Some(id))).expect("checkpoint vector");
    for (id, bytes, expected) in [
        ("checkpointEmpty", 0, "rejectedExactHandback"),
        ("checkpointSingle", 1, "rejectedExactHandback"),
        ("checkpointMax", PUZZLE_COMMAND_CHECKPOINT_BYTES as u64, "sameSemanticDigestTerminalEmpty"),
        ("checkpointMaxPlusOne", PUZZLE_COMMAND_CHECKPOINT_BYTES as u64 + 1, "rejectedExactHandback"),
        ("checkpointCorrupt", PUZZLE_COMMAND_CHECKPOINT_BYTES as u64, "rejectedExactHandback"),
        ("checkpointInterruptedClose", PUZZLE_COMMAND_CHECKPOINT_BYTES as u64, "terminalEmptyExactHandback"),
    ] {
        let vector = checkpoint_vector(id);
        assert_eq!(vector.get("checkpointBytes").and_then(Value::as_u64), Some(bytes));
        assert_eq!(vector.get("expected").and_then(Value::as_str), Some(expected));
    }
    let interrupted = checkpoint_vector("checkpointInterruptedClose");
    assert_eq!(interrupted.pointer("/closeGrant/items").and_then(Value::as_u64), Some(1));
    assert_eq!(interrupted.pointer("/closeGrant/bytes").and_then(Value::as_u64), Some(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES as u64));
    assert_eq!(interrupted.get("boundary").and_then(Value::as_str), Some("workProgress"));
    assert_eq!(checkpoint_vector("checkpointCorrupt").get("mutation").and_then(Value::as_str), Some("magic"));
    let tool_ids = actual.evidence_tool_ids.iter().map(String::as_str).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(root.pointer("/capacities/checkpointBytes").and_then(Value::as_u64), Some(PUZZLE_COMMAND_CHECKPOINT_BYTES as u64));
    assert!(root.pointer("/locales/en/cancel").and_then(Value::as_str).is_some());
    assert!(root.pointer("/locales/de/cancel").and_then(Value::as_str).is_some());
    assert!(root.pointer("/locales/default").is_none());
    assert!(root.get("vectors").and_then(Value::as_array).is_some_and(|vectors| vectors.iter().all(|vector| {
        vector.get("id").and_then(Value::as_str).is_some()
            && vector.get("expected").and_then(Value::as_str).is_some()
            && (vector.get("control").is_some()
                || vector.get("authority").is_some()
                || vector.get("closeGrant").is_some()
                || vector.get("toolId").and_then(Value::as_str).is_some_and(|tool| tool_ids.contains(tool))
                || vector.get("toolIds").and_then(Value::as_array).is_some_and(|tools| tools.iter().all(|tool| tool.as_str().is_some_and(|tool| tool_ids.contains(tool)))))
    })));
}

#[test]
fn language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle() {
    assert_fixture(include_str!("../../../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json"), expected("puzzle2d", "puzzle.2d.fixture", crate::editor::puzzle2d::PUZZLE2D_RETAINED_TOOL_IDS));
    assert_fixture(include_str!("../../../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json"), expected("puzzle3d", "puzzle.3d.fixture", crate::editor::puzzle3d::PUZZLE3D_RETAINED_TOOL_IDS));
    assert_fixture(include_str!("../../../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json"), expected("puzzle5d", "puzzle.5d", crate::editor::puzzle5d::PUZZLE5D_RETAINED_TOOL_IDS));
}

#[test]
fn hostile_fixture_mutations_change_the_oracle_result_or_fail_closed() {
    let fixture = include_str!("../../../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json");
    let oracle = SerdeJsonFixtureOracle;
    let baseline = oracle.evaluate(fixture).expect("baseline");
    for mutated in [fixture.replacen("8192", "8193", 1), fixture.replacen("addNode", "missingNode", 1), fixture.replacen("maxPlusOne", "maxPlusTwo", 1), fixture.replacen("\"de\":", "\"fr\":", 1), fixture.replacen("0:0:0:0:0:0", "9:9:9:9:9:9", 1)] {
        assert_ne!(oracle.evaluate(&mutated).expect("mutation remains parseable"), baseline);
    }
    assert!(oracle.evaluate(&fixture.replacen("\"fingerprint\": \"0:0:0:0:0:0\"", "\"missingFingerprint\": true", 1)).is_err());
}
