use super::*;

/// ♻️ Takes ownership of a faulting step outcome and returns its retained page to the ledger.
/// `RetainedJobPayload::drop` asserts one-page close (`🧵️job/🦀️.rs`), so a fault detail dropped on the
/// floor raises a second panic during unwinding and aborts the whole binary — every production caller
/// closes it, and so must every assertion that consumes one.
fn faulted(outcome: StepOutcome) -> bool {
    let StepOutcome::Fault(mut fault) = outcome else { return false };
    while !fault.detail.terminal_is_empty() {
        fault.detail.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    true
}

use crate::editor::puzzle3d::precompute::geometry::{collision_body_from_buffers, OwnerReservationLimit, DOCUMENT_CELL_SLOTS, FIXED_OWNER_PAGE_BYTES, FIXED_OWNER_SLOTS};
use semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
use crate::editor::puzzle3d::precompute::{FILL_ENVELOPE_MAX_BYTES, FILL_ENVELOPE_MAX_ITEMS};
use crate::standards::v1::subsets::any::schema::{BrushKindWeights, KindCatalogBundle, ObjectKind, ObjectKindRepresentation, ObjectKindVortexTemplate, VortexProps};
use semio_framework_job::{root_cancel_token, Generation, OperationId, RevisionId, StepBudget};
use std::time::{Duration, Instant};

/// 🎯️ What a fresh editor asks for — the product default, never a planner ceiling.
const TEST_REQUESTED_COUNT: usize = 100;

fn empty_builder() -> FillBuilder {
    let scene = Arc::new(SceneConfig {
        fixture: Fixture::default(),
        kind_catalogs: Some(KindCatalogBundle::default()),
        kind_compatibility: Vec::new(),
        overlap_budget: 0.0,
        seed: 17,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    });
    FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(HashMap::new())), Operation::new(OperationId(1), RevisionId(1), Generation(1), 17), TEST_REQUESTED_COUNT)
}

fn test_context<'a>(builder: &FillBuilder, cancel: semio_framework_job::CancelToken, sequence: &'a mut u64) -> StepContext<'a> {
    fn now() -> Option<u64> {
        Some(0)
    }
    StepContext::new(builder.operation.operation, builder.operation.generation, StepBudget::new(100, 10), cancel, now, sequence)
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct OracleGhost<'a> {
    target_vortex_full_id: &'a str,
    object_kind_id: &'a str,
    source_vortex_index: usize,
    mesh_url: &'a str,
    origin: [f64; 3],
    orientation: [f64; 4],
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct OracleTried<'a> {
    sequence: u64,
    verdict: &'a str,
    reason: Option<&'a str>,
    ghost: OracleGhost<'a>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct OracleDiagnostic<'a> {
    operation: u64,
    base_revision: u64,
    registry_generation: u64,
    sequence: u64,
    generation: u64,
    stage: &'a str,
    status_label: &'a str,
    target_vortex_full_id: Option<&'a str>,
    candidate_object_kind_id: Option<&'a str>,
    candidate_ghost: Option<OracleGhost<'a>>,
    current_pair_object_id: Option<&'a str>,
    collision_count: usize,
    sample_cursor: usize,
    inside_both: usize,
    last_sample: Option<[f32; 3]>,
    candidate_page: &'a [Option<String>; 8],
    truncated: bool,
    rejection_reason: Option<&'a str>,
    target_cursor: usize,
    candidate_cursor: usize,
    accepted_count: usize,
    requested_count: usize,
    search_count: u64,
    rejected_count: u64,
    verdict: &'a str,
    tested_count: u64,
    stall_reason: Option<&'a str>,
    tried: Vec<Option<OracleTried<'a>>>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct OracleRoot<'a> {
    target_vortex_full_id: &'a str,
    object_kind_id: &'a str,
    source_vortex_index: usize,
    mesh_url: &'a str,
    origin: [f64; 3],
    orientation: [f64; 4],
    color: &'a str,
    opacity: f64,
    verdict: &'a str,
    fill_build_preview: OracleDiagnostic<'a>,
}

fn oracle_ghost(ghost: &BrushPreviewState) -> OracleGhost<'_> {
    OracleGhost { target_vortex_full_id: &ghost.target_vortex_full_id, object_kind_id: &ghost.object_kind_id, source_vortex_index: ghost.source_vortex_index, mesh_url: &ghost.mesh_url, origin: ghost.origin, orientation: ghost.orientation }
}

fn oracle_tried(entry: &FillTriedCandidate) -> OracleTried<'_> {
    OracleTried { sequence: entry.sequence, verdict: entry.verdict.wire(), reason: entry.reason.as_deref(), ghost: oracle_ghost(&entry.ghost) }
}

fn oracle_json_scalar_admits(preview: &FillBuildPreview, color: &str, status_label: &str) -> bool {
    let safe_u64 = |value: u64, minimum: u64| value >= minimum && value <= 9_007_199_254_740_991;
    let safe_usize = |value: usize| value as u128 <= 9_007_199_254_740_991;
    let source_authority = preview.candidate_ghost.as_ref().map_or(true, |ghost| {
        let root_source_vortex_index = ghost.source_vortex_index as u128;
        let candidate_ghost_source_vortex_index = ghost.source_vortex_index as u128;
        root_source_vortex_index <= 9_007_199_254_740_991 && candidate_ghost_source_vortex_index <= 9_007_199_254_740_991
    });
    let tried_authority = preview.tried.iter().flatten().all(|entry| entry.ghost.source_vortex_index as u128 <= 9_007_199_254_740_991 && safe_u64(entry.sequence, 0));
    tried_authority
        && safe_u64(preview.tested_count, 0)
        && safe_usize(preview.requested_count)
        && color.len() <= 128
        && !status_label.is_empty()
        && status_label.len() <= 256
        && source_authority
        && safe_u64(preview.operation, 1)
        && safe_u64(preview.base_revision, 1)
        && safe_u64(preview.registry_generation, 1)
        && safe_u64(preview.sequence, 0)
        && safe_u64(preview.generation, 1)
        && safe_usize(preview.collision_count)
        && safe_usize(preview.sample_cursor)
        && safe_usize(preview.inside_both)
        && safe_usize(preview.target_cursor)
        && safe_usize(preview.candidate_cursor)
        && safe_usize(preview.accepted_count)
        && safe_u64(preview.search_count, 0)
        && safe_u64(preview.rejected_count, 0)
}

fn oracle_json_unfenced(preview: &FillBuildPreview, color: &str, status_label: &str) -> String {
    let ghost = preview.candidate_ghost.as_ref().expect("oracle case ghost");
    serde_json::to_string(&OracleRoot {
        target_vortex_full_id: &ghost.target_vortex_full_id,
        object_kind_id: &ghost.object_kind_id,
        source_vortex_index: ghost.source_vortex_index,
        mesh_url: &ghost.mesh_url,
        origin: ghost.origin,
        orientation: ghost.orientation,
        color,
        opacity: 0.35,
        verdict: preview.verdict.wire(),
        fill_build_preview: OracleDiagnostic {
            operation: preview.operation,
            base_revision: preview.base_revision,
            registry_generation: preview.registry_generation,
            sequence: preview.sequence,
            generation: preview.generation,
            stage: &preview.stage,
            status_label,
            target_vortex_full_id: preview.target_vortex_full_id.as_deref(),
            candidate_object_kind_id: preview.candidate_object_kind_id.as_deref(),
            candidate_ghost: preview.candidate_ghost.as_ref().map(oracle_ghost),
            current_pair_object_id: preview.current_pair_object_id.as_deref(),
            collision_count: preview.collision_count,
            sample_cursor: preview.sample_cursor,
            inside_both: preview.inside_both,
            last_sample: preview.last_sample,
            candidate_page: &preview.candidate_page,
            truncated: preview.truncated,
            rejection_reason: preview.rejection_reason.as_deref(),
            target_cursor: preview.target_cursor,
            candidate_cursor: preview.candidate_cursor,
            accepted_count: preview.accepted_count,
            requested_count: preview.requested_count,
            search_count: preview.search_count,
            rejected_count: preview.rejected_count,
            verdict: preview.verdict.wire(),
            tested_count: preview.tested_count,
            stall_reason: preview.stall_reason.as_deref(),
            tried: preview.tried.iter().map(|entry| entry.as_ref().map(oracle_tried)).collect(),
        },
    })
    .expect("test-only serde oracle")
}

fn oracle_json_admits(preview: &FillBuildPreview, color: &str, status_label: &str) -> bool {
    oracle_json_scalar_admits(preview, color, status_label) && oracle_json_unfenced(preview, color, status_label).len() <= FILL_PREVIEW_JSON_MAX_BYTES
}

fn oracle_json(preview: &FillBuildPreview, color: &str, status_label: &str) -> String {
    assert!(oracle_json_scalar_admits(preview, color, status_label), "owned scalar schema semantics guard the test-only serde oracle");
    let text = oracle_json_unfenced(preview, color, status_label);
    assert!(text.len() <= FILL_PREVIEW_JSON_MAX_BYTES, "owned full-wire byte semantics guard the test-only serde oracle");
    text
}

/// 📏️ Two one-byte-per-grant passes (census then encode) over a page that may hold the whole wire
/// cap, plus the phase transitions between them — the bound is a terminal-observation guard, not a
/// budget.
const PREVIEW_JSON_GRANT_CEILING: usize = 4 * FILL_PREVIEW_JSON_MAX_BYTES;

fn drive_preview_json(builder: &mut FillBuilder, color: &str, status_label: &str) -> FillPreviewJsonStep {
    for _ in 0..PREVIEW_JSON_GRANT_CEILING {
        let mut fuel = 1;
        let step = builder.preview_json_step(color, status_label, &mut fuel, false, false);
        if !matches!(step, FillPreviewJsonStep::Pending { .. }) {
            return step;
        }
    }
    panic!("preview cursor did not reach a bounded terminal observation")
}

fn fixture_preview() -> (FillBuildPreview, String, String, String, String, String) {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("language-neutral law fixture");
    let preview = serde_json::from_value(law["preview"].clone()).expect("schema-first preview");
    let color = law["color"].as_str().expect("color").to_string();
    let english = law["locales"]["en"]["statusLabel"].as_str().expect("English status label").to_string();
    let german = law["locales"]["de"]["statusLabel"].as_str().expect("German status label").to_string();
    let expected_english = law["locales"]["en"]["expected"].as_str().expect("English expected bytes").to_string();
    let expected_german = law["locales"]["de"]["expected"].as_str().expect("German expected bytes").to_string();
    (preview, color, english, german, expected_english, expected_german)
}

fn assert_preview_ready_matches_oracle(preview: FillBuildPreview, color: &str, status_label: &str) {
    assert!(oracle_json_admits(&preview, color, status_label));
    let expected = oracle_json(&preview, color, status_label);
    let mut builder = empty_builder();
    builder.preview = preview;
    assert_eq!(drive_preview_json(&mut builder, color, status_label), FillPreviewJsonStep::Ready);
    assert_eq!(builder.preview_json_ready(), Some(expected.as_str()));
}

fn assert_preflight_rejection_preserves_ready(builder: &mut FillBuilder, color: &str, status_label: &str) {
    let ready_pointer = builder.preview_json.ready.as_ref().expect("retained page").as_ptr();
    let ready_page = builder.preview_json_ready().expect("retained page").to_string();
    let ready_identity = builder.preview_json_ready_identity();
    let color_pointer = builder.preview_json.color.as_ptr();
    let retained_color = builder.preview_json.color.clone();
    let status_label_pointer = builder.preview_json.status_label.as_ptr();
    let retained_status_label = builder.preview_json.status_label.clone();
    let checkpoint = builder.preview_json.checkpoint();
    let phase = builder.preview_json.phase;
    let mut fuel = 1;
    assert_eq!(builder.preview_json_step(color, status_label, &mut fuel, false, false), FillPreviewJsonStep::Rejected);
    assert_eq!(fuel, 1, "preflight rejects before consuming a semantic grant");
    assert_eq!(builder.preview_json.checkpoint(), checkpoint);
    assert_eq!(builder.preview_json.phase, phase);
    assert_eq!(builder.preview_json.ready.as_ref().expect("same owner").as_ptr(), ready_pointer);
    assert_eq!(builder.preview_json_ready(), Some(ready_page.as_str()));
    assert_eq!(builder.preview_json_ready_identity(), ready_identity);
    assert_eq!(builder.preview_json.color.as_ptr(), color_pointer);
    assert_eq!(builder.preview_json.color, retained_color);
    assert_eq!(builder.preview_json.status_label.as_ptr(), status_label_pointer);
    assert_eq!(builder.preview_json.status_label, retained_status_label);
    assert!(
        builder.preview_json.output.is_none() && builder.preview_json.retiring_bytes.is_none() && builder.preview_json.retiring_ready.is_none() && builder.preview_json.retiring_color.is_none() && builder.preview_json.retiring_status_label.is_none()
    );
}

#[test]
fn retained_preview_json_matches_language_neutral_fixture_and_test_only_serde_oracle() {
    let module: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧬️schema/🔣️.json")).expect("s.puzzle3d owner schema module");
    let schema = &module["$defs"]["Puzzle3dFillPreviewJson"];
    let diagnostic = &module["$defs"]["Puzzle3dFillPreviewDiagnostic"];
    let ghost = &module["$defs"]["Puzzle3dFillPreviewGhost"];
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("language-neutral law fixture");
    assert!(diagnostic["required"].as_array().is_some_and(|fields| fields.iter().any(|field| field.as_str() == Some("statusLabel"))));
    assert_eq!(schema["properties"]["sourceVortexIndex"]["maximum"].as_u64(), Some(FILL_PREVIEW_JSON_MAX_SOURCE_VORTEX_INDEX));
    assert_eq!(ghost["properties"]["sourceVortexIndex"]["maximum"].as_u64(), Some(FILL_PREVIEW_JSON_MAX_SOURCE_VORTEX_INDEX));
    assert_eq!(schema["properties"]["color"]["maxLength"].as_u64(), Some(FILL_PREVIEW_JSON_MAX_COLOR_BYTES as u64));
    assert_eq!(schema["properties"]["color"]["x-semio-maxUtf8Bytes"].as_u64(), Some(FILL_PREVIEW_JSON_MAX_COLOR_BYTES as u64));
    assert_eq!(schema["x-semio-maxEncodedUtf8Bytes"].as_u64(), Some(FILL_PREVIEW_JSON_MAX_BYTES as u64));
    assert_eq!(diagnostic["properties"]["statusLabel"]["x-semio-maxUtf8Bytes"].as_u64(), Some(FILL_PREVIEW_JSON_MAX_STATUS_LABEL_BYTES as u64));
    assert_eq!(law["limits"]["maximumSourceVortexIndex"].as_u64(), Some(FILL_PREVIEW_JSON_MAX_SOURCE_VORTEX_INDEX));
    assert_eq!(law["limits"]["maximumDiagnosticInteger"].as_u64(), Some(FILL_PREVIEW_JSON_MAX_DIAGNOSTIC_INTEGER));
    assert_eq!(law["limits"]["maximumColorBytes"].as_u64(), Some(FILL_PREVIEW_JSON_MAX_COLOR_BYTES as u64));
    assert_eq!(law["limits"]["maximumStatusLabelBytes"].as_u64(), Some(FILL_PREVIEW_JSON_MAX_STATUS_LABEL_BYTES as u64));
    assert_eq!(law["limits"]["maximumBytes"].as_u64(), Some(FILL_PREVIEW_JSON_MAX_BYTES as u64));
    let tried = &module["$defs"]["Puzzle3dFillPreviewTried"];
    let verdict = &module["$defs"]["Puzzle3dFillPreviewVerdict"];
    assert_eq!(law["limits"]["triedItems"].as_u64(), Some(FILL_TRIED_RING as u64));
    assert_eq!(diagnostic["properties"]["tried"]["minItems"].as_u64(), Some(FILL_TRIED_RING as u64));
    assert_eq!(diagnostic["properties"]["tried"]["maxItems"].as_u64(), Some(FILL_TRIED_RING as u64));
    assert_eq!(tried["properties"]["ghost"]["$ref"].as_str(), Some("#/$defs/Puzzle3dFillPreviewGhost"));
    let declared_verdicts = verdict["enum"].as_array().expect("verdict enum").iter().map(|value| value.as_str().expect("verdict")).collect::<Vec<_>>();
    let owned_verdicts = [FillCandidateVerdict::Testing, FillCandidateVerdict::Free, FillCandidateVerdict::Collision, FillCandidateVerdict::Rejected, FillCandidateVerdict::Accepted].map(FillCandidateVerdict::wire);
    assert_eq!(declared_verdicts, owned_verdicts, "the schema enum and the owned wire spellings are one law");
    assert_eq!(law["boundaryLaws"]["verdicts"].as_array().expect("verdict laws").iter().map(|value| value.as_str().expect("verdict")).collect::<Vec<_>>(), owned_verdicts);
    let numeric_fields = law["diagnosticNumericFields"].as_array().expect("diagnostic numeric laws");
    assert_eq!(numeric_fields.len(), 15);
    for field in numeric_fields {
        let name = field["field"].as_str().expect("numeric field");
        assert_eq!(diagnostic["properties"][name]["minimum"], field["minimum"]);
        assert_eq!(diagnostic["properties"][name]["maximum"].as_u64(), Some(FILL_PREVIEW_JSON_MAX_DIAGNOSTIC_INTEGER));
    }
    let (preview, color, english, german, expected_english, expected_german) = fixture_preview();
    assert_eq!(oracle_json(&preview, &color, &english), expected_english, "English fixture and third-party oracle are byte-identical");
    assert_eq!(oracle_json(&preview, &color, &german), expected_german, "German fixture and third-party oracle are byte-identical");
    let mut builder = empty_builder();
    builder.preview = preview;
    assert_eq!(drive_preview_json(&mut builder, &color, &english), FillPreviewJsonStep::Ready);
    assert_eq!(builder.preview_json_ready(), Some(expected_english.as_str()));
    assert_eq!(builder.preview_json_ready().map(str::len), Some(expected_english.len()));
    assert_eq!(drive_preview_json(&mut builder, &color, &german), FillPreviewJsonStep::Ready);
    assert_eq!(builder.preview_json_ready(), Some(expected_german.as_str()));
}

#[test]
fn retained_preview_json_safe_index_boundary_is_schema_first_portable_and_preflighted() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("language-neutral law fixture");
    let boundaries = law["boundaryLaws"]["sourceVortexIndex"].as_array().expect("source index laws");
    let safe_maximum = boundaries[0]["value"].as_u64().expect("safe maximum");
    let maximum_plus_one = boundaries[1]["value"].as_u64().expect("safe maximum plus one");
    assert_eq!((boundaries[0]["admitted"].as_bool(), boundaries[1]["admitted"].as_bool()), (Some(true), Some(false)));
    assert_eq!(safe_maximum, FILL_PREVIEW_JSON_MAX_SOURCE_VORTEX_INDEX);
    assert_eq!(maximum_plus_one, FILL_PREVIEW_JSON_MAX_SOURCE_VORTEX_INDEX + 1);

    if let Ok(source_vortex_index) = usize::try_from(safe_maximum) {
        let (mut preview, color, english, _, _, _) = fixture_preview();
        preview.candidate_ghost.as_mut().expect("fixture ghost").source_vortex_index = source_vortex_index;
        assert!(oracle_json_admits(&preview, &color, &english));
        let expected = oracle_json(&preview, &color, &english);
        let mut builder = empty_builder();
        builder.preview = preview;
        assert_eq!(drive_preview_json(&mut builder, &color, &english), FillPreviewJsonStep::Ready);
        assert_eq!(builder.preview_json_ready(), Some(expected.as_str()));
    } else {
        assert!((usize::MAX as u128) <= safe_maximum as u128, "narrow usize platforms admit every representable source index");
    }

    if let Ok(source_vortex_index) = usize::try_from(maximum_plus_one) {
        let (preview, color, english, _, _, _) = fixture_preview();
        let mut builder = empty_builder();
        builder.preview = preview;
        assert_eq!(drive_preview_json(&mut builder, &color, &english), FillPreviewJsonStep::Ready);
        let ready_pointer = builder.preview_json.ready.as_ref().expect("retained page").as_ptr();
        let ready_page = builder.preview_json_ready().expect("retained page").to_string();
        let ready_identity = builder.preview_json_ready_identity();
        let color_pointer = builder.preview_json.color.as_ptr();
        let status_label_pointer = builder.preview_json.status_label.as_ptr();
        let checkpoint = builder.preview_json.checkpoint();
        let phase = builder.preview_json.phase;
        builder.preview.candidate_ghost.as_mut().expect("fixture ghost").source_vortex_index = source_vortex_index;
        assert!(!oracle_json_admits(&builder.preview, &color, &english));
        let mut fuel = 1;
        assert_eq!(builder.preview_json_step("#000", "Füllfortschritt", &mut fuel, false, false), FillPreviewJsonStep::Rejected);
        assert_eq!(fuel, 1, "preflight rejects before consuming a semantic grant");
        assert_eq!(builder.preview_json.checkpoint(), checkpoint);
        assert_eq!(builder.preview_json.phase, phase);
        assert_eq!(builder.preview_json.ready.as_ref().expect("same owner").as_ptr(), ready_pointer);
        assert_eq!(builder.preview_json_ready(), Some(ready_page.as_str()));
        assert_eq!(builder.preview_json_ready_identity(), ready_identity);
        assert_eq!(builder.preview_json.color.as_ptr(), color_pointer);
        assert_eq!(builder.preview_json.status_label.as_ptr(), status_label_pointer);
        assert!(builder.preview_json.output.is_none() && builder.preview_json.retiring_bytes.is_none() && builder.preview_json.retiring_ready.is_none());
    } else {
        assert!((usize::MAX as u128) <= safe_maximum as u128, "maximum plus one is unrepresentable only where every usize is wire-safe");
    }
}

#[test]
fn retained_preview_json_all_diagnostic_numeric_boundaries_are_preflighted() {
    type U64Setter = fn(&mut FillBuildPreview, u64);
    type UsizeSetter = fn(&mut FillBuildPreview, usize);
    let u64_fields: [(&str, U64Setter); 8] = [
        ("operation", |preview, value| preview.operation = value),
        ("baseRevision", |preview, value| preview.base_revision = value),
        ("registryGeneration", |preview, value| preview.registry_generation = value),
        ("sequence", |preview, value| preview.sequence = value),
        ("generation", |preview, value| preview.generation = value),
        ("searchCount", |preview, value| preview.search_count = value),
        ("rejectedCount", |preview, value| preview.rejected_count = value),
        ("testedCount", |preview, value| preview.tested_count = value),
    ];
    let usize_fields: [(&str, UsizeSetter); 7] = [
        ("collisionCount", |preview, value| preview.collision_count = value),
        ("sampleCursor", |preview, value| preview.sample_cursor = value),
        ("insideBoth", |preview, value| preview.inside_both = value),
        ("targetCursor", |preview, value| preview.target_cursor = value),
        ("candidateCursor", |preview, value| preview.candidate_cursor = value),
        ("acceptedCount", |preview, value| preview.accepted_count = value),
        ("requestedCount", |preview, value| preview.requested_count = value),
    ];
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("language-neutral law fixture");
    let mut declared = law["diagnosticNumericFields"].as_array().expect("diagnostic numeric laws").iter().map(|field| field["field"].as_str().expect("field")).collect::<Vec<_>>();
    let mut tested = u64_fields.iter().map(|(field, _)| *field).chain(usize_fields.iter().map(|(field, _)| *field)).collect::<Vec<_>>();
    declared.sort_unstable();
    tested.sort_unstable();
    assert_eq!(tested, declared);
    let maximum = law["limits"]["maximumDiagnosticInteger"].as_u64().expect("diagnostic maximum");
    let maximum_plus_one = maximum + 1;

    for (_, set) in u64_fields {
        let (mut preview, color, english, _, _, _) = fixture_preview();
        set(&mut preview, maximum);
        assert_preview_ready_matches_oracle(preview, &color, &english);

        let (preview, color, english, _, _, _) = fixture_preview();
        let mut builder = empty_builder();
        builder.preview = preview;
        assert_eq!(drive_preview_json(&mut builder, &color, &english), FillPreviewJsonStep::Ready);
        set(&mut builder.preview, maximum_plus_one);
        assert!(!oracle_json_admits(&builder.preview, &color, &english));
        assert_preflight_rejection_preserves_ready(&mut builder, "#000", "Füllfortschritt");
    }

    if let Ok(maximum) = usize::try_from(maximum) {
        for (_, set) in usize_fields {
            let (mut preview, color, english, _, _, _) = fixture_preview();
            set(&mut preview, maximum);
            assert_preview_ready_matches_oracle(preview, &color, &english);
        }
    } else {
        assert!((usize::MAX as u128) <= FILL_PREVIEW_JSON_MAX_DIAGNOSTIC_INTEGER as u128);
        for (_, set) in usize_fields {
            let (mut preview, color, english, _, _, _) = fixture_preview();
            set(&mut preview, usize::MAX);
            assert_preview_ready_matches_oracle(preview, &color, &english);
        }
    }

    if let Ok(maximum_plus_one) = usize::try_from(maximum_plus_one) {
        for (_, set) in usize_fields {
            let (preview, color, english, _, _, _) = fixture_preview();
            let mut builder = empty_builder();
            builder.preview = preview;
            assert_eq!(drive_preview_json(&mut builder, &color, &english), FillPreviewJsonStep::Ready);
            set(&mut builder.preview, maximum_plus_one);
            assert!(!oracle_json_admits(&builder.preview, &color, &english));
            assert_preflight_rejection_preserves_ready(&mut builder, "#000", "Füllfortschritt");
        }
    } else {
        assert!((usize::MAX as u128) <= FILL_PREVIEW_JSON_MAX_DIAGNOSTIC_INTEGER as u128, "unrepresentable plus one means every usize is wire-safe");
    }
}

#[test]
fn retained_preview_json_status_label_byte_boundary_matches_owned_serde_oracle() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("language-neutral law fixture");
    for boundary in law["boundaryLaws"]["statusLabel"].as_array().expect("status label laws") {
        let unit = boundary["unit"].as_str().expect("status label unit");
        let repeat = boundary["repeat"].as_u64().and_then(|value| usize::try_from(value).ok()).expect("status label repeat");
        let expected_bytes = boundary["utf8Bytes"].as_u64().and_then(|value| usize::try_from(value).ok()).expect("status label bytes");
        let admitted = boundary["admitted"].as_bool().expect("status label admission");
        let status_label = unit.repeat(repeat);
        assert_eq!(status_label.len(), expected_bytes, "fixture declares UTF-8 bytes");
        let (preview, color, english, _, _, _) = fixture_preview();
        assert_eq!(oracle_json_admits(&preview, &color, &status_label), admitted);
        if admitted {
            assert_preview_ready_matches_oracle(preview, &color, &status_label);
        } else {
            let mut builder = empty_builder();
            builder.preview = preview;
            assert_eq!(drive_preview_json(&mut builder, &color, &english), FillPreviewJsonStep::Ready);
            assert_preflight_rejection_preserves_ready(&mut builder, &color, &status_label);
        }
    }
}

#[test]
fn retained_preview_json_color_byte_boundary_matches_owned_serde_oracle() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("language-neutral law fixture");
    for boundary in law["boundaryLaws"]["color"].as_array().expect("color laws") {
        let unit = boundary["unit"].as_str().expect("color unit");
        let repeat = boundary["repeat"].as_u64().and_then(|value| usize::try_from(value).ok()).expect("color repeat");
        let expected_bytes = boundary["utf8Bytes"].as_u64().and_then(|value| usize::try_from(value).ok()).expect("color bytes");
        let admitted = boundary["admitted"].as_bool().expect("color admission");
        let color = unit.repeat(repeat);
        assert_eq!(color.len(), expected_bytes, "fixture declares UTF-8 bytes");
        let (preview, _, english, _, _, _) = fixture_preview();
        assert_eq!(oracle_json_admits(&preview, &color, &english), admitted);
        let mut builder = empty_builder();
        builder.preview = preview;
        let step = drive_preview_json(&mut builder, &color, &english);
        if admitted {
            assert_eq!(step, FillPreviewJsonStep::Ready);
            let expected = oracle_json(&builder.preview, &color, &english);
            assert_eq!(builder.preview_json_ready(), Some(expected.as_str()));
        } else {
            assert_eq!(step, FillPreviewJsonStep::Rejected);
            assert!(builder.preview_json.output.is_none() && builder.preview_json.ready().is_none(), "oversized color never reserves or publishes");
        }
    }
}

#[test]
fn retained_preview_json_exact_cap_and_plus_one_fail_closed_before_reserve() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("language-neutral law fixture");
    let boundaries = law["boundaryLaws"]["fullWire"].as_array().expect("full-wire laws");
    let maximum_bytes = boundaries[0]["utf8Bytes"].as_u64().and_then(|value| usize::try_from(value).ok()).expect("maximum bytes");
    let maximum_plus_one_bytes = boundaries[1]["utf8Bytes"].as_u64().and_then(|value| usize::try_from(value).ok()).expect("maximum plus one bytes");
    assert_eq!((boundaries[0]["admitted"].as_bool(), boundaries[1]["admitted"].as_bool()), (Some(true), Some(false)));
    assert_eq!((boundaries[0]["sourceField"].as_str(), boundaries[1]["sourceField"].as_str()), (Some("stage"), Some("stage")));
    assert_eq!(boundaries[0]["oracle"].as_str(), Some("serde_json"));
    assert_eq!(
        boundaries[1]["preserves"].as_array().expect("plus-one preservation law").iter().map(|owner| owner.as_str().expect("preserved owner")).collect::<Vec<_>>(),
        ["fuel", "checkpoint", "phase", "ready", "readyIdentity", "colorOwner", "statusLabelOwner", "transientOwners"]
    );
    assert_eq!((maximum_bytes, maximum_plus_one_bytes), (FILL_PREVIEW_JSON_MAX_BYTES, FILL_PREVIEW_JSON_MAX_BYTES + 1));
    let (mut maximum, color, english, _, _, _) = fixture_preview();
    maximum.stage.clear();
    let fixed_bytes = oracle_json_unfenced(&maximum, &color, &english).len();
    maximum.stage = "x".repeat(maximum_bytes - fixed_bytes);
    let maximum_oracle = oracle_json_unfenced(&maximum, &color, &english);
    assert_eq!(maximum_oracle.len(), maximum_bytes);
    assert_eq!(fill_preview_json_wire_bytes(&maximum, &color, &english), Ok(maximum_bytes));
    assert_preview_ready_matches_oracle(maximum, &color, &english);

    let (preview, color, english, _, _, _) = fixture_preview();
    let mut retained = empty_builder();
    retained.preview = preview;
    assert_eq!(drive_preview_json(&mut retained, &color, &english), FillPreviewJsonStep::Ready);
    retained.preview.stage.clear();
    let fixed_bytes = oracle_json_unfenced(&retained.preview, &color, &english).len();
    retained.preview.stage = "x".repeat(maximum_plus_one_bytes - fixed_bytes);
    assert!(oracle_json_scalar_admits(&retained.preview, &color, &english));
    assert_eq!(oracle_json_unfenced(&retained.preview, &color, &english).len(), maximum_plus_one_bytes);
    assert!(!oracle_json_admits(&retained.preview, &color, &english));
    assert_eq!(fill_preview_json_wire_bytes(&retained.preview, &color, &english), Err(()));
    assert_preflight_rejection_preserves_ready(&mut retained, &color, &english);
}

#[test]
fn retained_preview_json_all_native_string_sources_enforce_wire_cap_before_mutation() {
    type Setter = fn(&mut FillBuildPreview, String);
    let setters: [(&str, Setter); 14] = [
        ("stage", |preview, value| preview.stage = value),
        ("targetVortexFullId", |preview, value| preview.target_vortex_full_id = Some(value)),
        ("candidateObjectKindId", |preview, value| preview.candidate_object_kind_id = Some(value)),
        ("candidateGhost.targetVortexFullId", |preview, value| preview.candidate_ghost.as_mut().expect("fixture ghost").target_vortex_full_id = value),
        ("candidateGhost.objectKindId", |preview, value| preview.candidate_ghost.as_mut().expect("fixture ghost").object_kind_id = value),
        ("candidateGhost.meshUrl", |preview, value| preview.candidate_ghost.as_mut().expect("fixture ghost").mesh_url = value),
        ("currentPairObjectId", |preview, value| preview.current_pair_object_id = Some(value)),
        ("candidatePage[0]", |preview, value| preview.candidate_page[0] = Some(value)),
        ("rejectionReason", |preview, value| preview.rejection_reason = Some(value)),
        ("stallReason", |preview, value| preview.stall_reason = Some(value)),
        ("tried[0].reason", |preview, value| preview.tried[0].as_mut().expect("fixture ring entry").reason = Some(value)),
        ("tried[0].ghost.targetVortexFullId", |preview, value| preview.tried[0].as_mut().expect("fixture ring entry").ghost.target_vortex_full_id = value),
        ("tried[0].ghost.objectKindId", |preview, value| preview.tried[0].as_mut().expect("fixture ring entry").ghost.object_kind_id = value),
        ("tried[0].ghost.meshUrl", |preview, value| preview.tried[0].as_mut().expect("fixture ring entry").ghost.mesh_url = value),
    ];
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("language-neutral law fixture");
    let mut declared = law["boundaryLaws"]["aggregateSourceStrings"].as_array().expect("aggregate source strings").iter().map(|field| field.as_str().expect("source string")).collect::<Vec<_>>();
    let mut covered = setters.iter().map(|(field, _)| *field).chain(["color", "statusLabel"]).collect::<Vec<_>>();
    declared.sort_unstable();
    covered.sort_unstable();
    assert_eq!(covered, declared);

    for (_, set) in setters {
        let (preview, color, english, _, _, _) = fixture_preview();
        let mut retained = empty_builder();
        retained.preview = preview;
        assert_eq!(drive_preview_json(&mut retained, &color, &english), FillPreviewJsonStep::Ready);
        set(&mut retained.preview, "x".repeat(FILL_PREVIEW_JSON_MAX_BYTES + 1));
        assert!(oracle_json_scalar_admits(&retained.preview, &color, &english));
        assert!(oracle_json_unfenced(&retained.preview, &color, &english).len() > FILL_PREVIEW_JSON_MAX_BYTES);
        assert_eq!(fill_preview_json_wire_bytes(&retained.preview, &color, &english), Err(()));
        assert_preflight_rejection_preserves_ready(&mut retained, &color, &english);
    }
}

#[test]
fn retained_preview_json_rejects_malformed_and_omitted_schema_fields() {
    assert!(serde_json::from_str::<FillBuildPreview>(r#"{"operation":1}"#).is_err());
    let mut nine_items = serde_json::to_value(fixture_preview().0).expect("fixture value");
    nine_items["candidatePage"].as_array_mut().expect("candidate page").push(serde_json::Value::Null);
    assert!(serde_json::from_value::<FillBuildPreview>(nine_items).is_err());
    let mut malformed = empty_builder();
    malformed.preview.last_sample = Some([f32::NAN, 0.0, 0.0]);
    assert_eq!(drive_preview_json(&mut malformed, "#fff", "Fill progress"), FillPreviewJsonStep::Rejected);
    assert!(malformed.preview_json.ready().is_none());
    let mut missing_locale = empty_builder();
    assert_eq!(drive_preview_json(&mut missing_locale, "#fff", ""), FillPreviewJsonStep::Rejected);
    let mut oversized_locale = empty_builder();
    assert_eq!(drive_preview_json(&mut oversized_locale, "#fff", &"x".repeat(FILL_PREVIEW_JSON_MAX_STATUS_LABEL_BYTES + 1)), FillPreviewJsonStep::Rejected);
}

#[test]
fn retained_preview_json_zero_fuel_deadline_and_stale_generation_make_no_publication() {
    let mut builder = empty_builder();
    let mut fuel = 0;
    let before = builder.preview_json.checkpoint();
    assert!(matches!(builder.preview_json_step("#fff", "Fill progress", &mut fuel, false, false), FillPreviewJsonStep::Pending { .. }));
    assert_eq!(builder.preview_json.checkpoint(), before);
    fuel = 1;
    assert!(matches!(builder.preview_json_step("#fff", "Fill progress", &mut fuel, false, true), FillPreviewJsonStep::Pending { .. }));
    assert_eq!(builder.preview_json.checkpoint(), before);
    for _ in 0..32 {
        fuel = 1;
        let _ = builder.preview_json_step("#fff", "Fill progress", &mut fuel, false, false);
    }
    assert!(matches!(builder.preview_json.phase, FillPreviewJsonPhase::Census | FillPreviewJsonPhase::Reserve | FillPreviewJsonPhase::Encode));
    builder.preview.sequence += 1;
    fuel = 1;
    let _ = builder.preview_json_step("#fff", "Fill progress", &mut fuel, false, false);
    assert!(builder.preview_json.ready().is_none());
    assert_eq!(builder.preview_json.identity.map(|identity| identity.sequence), Some(builder.preview.sequence));
}

#[test]
fn retained_preview_json_cancellation_at_each_transfer_preserves_last_valid_page() {
    for target in [FillPreviewJsonPhase::Census, FillPreviewJsonPhase::Reserve, FillPreviewJsonPhase::Encode, FillPreviewJsonPhase::Validate] {
        let mut builder = empty_builder();
        for _ in 0..PREVIEW_JSON_GRANT_CEILING {
            if builder.preview_json.phase == target {
                break;
            }
            let mut fuel = 1;
            let _ = builder.preview_json_step("#fff", "Fill progress", &mut fuel, false, false);
        }
        let mut fuel = 1;
        assert_eq!(builder.preview_json_step("#fff", "Fill progress", &mut fuel, true, false), FillPreviewJsonStep::Cancelled);
        assert!(builder.preview_json.ready().is_none());
    }
    let mut builder = empty_builder();
    assert_eq!(drive_preview_json(&mut builder, "#fff", "Fill progress"), FillPreviewJsonStep::Ready);
    let ready = builder.preview_json_ready().expect("ready page").to_string();
    builder.preview.sequence += 1;
    let mut fuel = 1;
    assert_eq!(builder.preview_json_step("#fff", "Fill progress", &mut fuel, true, false), FillPreviewJsonStep::Cancelled);
    assert_eq!(builder.preview_json_ready(), Some(ready.as_str()));
}

#[test]
fn retained_preview_json_reuses_exact_ready_page_during_locale_invalidated_encode_and_closes_idempotently() {
    let mut builder = empty_builder();
    assert_eq!(drive_preview_json(&mut builder, "#fff", "Fill progress"), FillPreviewJsonStep::Ready);
    let old_pointer = builder.preview_json.ready.as_ref().expect("ready").as_ptr();
    let old_page = builder.preview_json_ready().expect("English page").to_string();
    assert!(old_page.contains("\"statusLabel\":\"Fill progress\""));
    let old_identity = builder.preview_json_ready_identity().expect("identity");
    for _ in 0..32 {
        let mut fuel = 1;
        assert!(matches!(builder.preview_json_step("#fff", "Füllfortschritt", &mut fuel, false, false), FillPreviewJsonStep::Pending { .. }));
        assert_eq!(builder.preview_json.ready.as_ref().expect("last valid page retained").as_ptr(), old_pointer);
        assert_eq!(builder.preview_json_ready(), Some(old_page.as_str()));
        assert_eq!(builder.preview_json_ready_identity(), Some(old_identity));
    }
    assert_eq!(drive_preview_json(&mut builder, "#fff", "Füllfortschritt"), FillPreviewJsonStep::Ready);
    assert!(builder.preview_json_ready().is_some_and(|page| page.contains("\"statusLabel\":\"Füllfortschritt\"")));
    assert_eq!(builder.preview_json_ready_identity(), Some(old_identity), "locale invalidates bytes without inventing a fill generation");
    assert!(!builder.preview_json.close_step(), "interrupted close releases one owner");
    for _ in 0..8 {
        if builder.preview_json.close_step() {
            break;
        }
    }
    assert!(builder.preview_json.close_step());
    assert!(builder.preview_json.close_step(), "terminal close is idempotent");
    assert!(builder.preview_json.terminal_owners_empty());
}

#[test]
fn retained_owner_census_advances_one_fixed_unit_and_rejects_collection_cap_plus_one() {
    let mut builder = empty_builder();
    let mut tags = Vec::with_capacity(FILL_BUILDER_NESTED_ITEMS);
    tags.extend((0..FILL_BUILDER_NESTED_ITEMS).map(|index| format!("tag-{index}")));
    let _ = builder.catalogs.objects.try_push(ObjectKind {
        id: "bounded-kind".into(),
        representations: vec![ObjectKindRepresentation { id: "r".into(), name: "n".into(), url: "u".into(), mime: "m".into(), tags, lod: Some("l".into()), description: "d".into() }],
        scale: Some(dsl::DslValue::Array(vec![dsl::DslValue::String("nested".into())])),
        vortices: Vec::new(),
    });
    let mut cursor = FillBuilderOwnerCensusCursor::default();
    let mut grants = 0;
    loop {
        let before = cursor.credit;
        match cursor.step(&builder, usize::MAX, usize::MAX) {
            FillBuilderOwnerCensusStep::Pending => {
                assert!(cursor.credit.items.saturating_sub(before.items) <= 7, "one grant visits one entry or fixed schema unit");
                assert!(cursor.credit.bytes.saturating_sub(before.bytes) <= DOCUMENT_OWNER_PAGE_BYTES, "one grant accounts at most one exact page");
                grants += 1;
            }
            FillBuilderOwnerCensusStep::Complete(_) => break,
            FillBuilderOwnerCensusStep::Rejected => panic!("fixed boundary must admit"),
        }
    }
    assert!(grants > FILL_BUILDER_NESTED_ITEMS, "max-cardinality tags and nested DSL cannot be scanned in one admission grant");

    let mut rejected = empty_builder();
    let mut tags = Vec::with_capacity(FILL_BUILDER_NESTED_ITEMS + 1);
    tags.extend((0..=FILL_BUILDER_NESTED_ITEMS).map(|index| format!("tag-{index}")));
    let _ = rejected.catalogs.objects.try_push(ObjectKind {
        id: "rejected-kind".into(),
        representations: vec![ObjectKindRepresentation { id: String::new(), name: String::new(), url: String::new(), mime: String::new(), tags, lod: None, description: String::new() }],
        scale: None,
        vortices: Vec::new(),
    });
    let mut cursor = FillBuilderOwnerCensusCursor::default();
    assert!((0..256).any(|_| matches!(cursor.step(&rejected, usize::MAX, usize::MAX), FillBuilderOwnerCensusStep::Rejected)), "collection cap + 1 rejects before admission credit publication");
}

#[test]
fn constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently() {
    #[derive(Clone, Copy)]
    enum HostileRoot {
        FixtureObjects,
        FixtureAttractions,
        FixtureTargetVolumes,
        Meshes,
        CatalogObjects,
        CatalogVortices,
        CatalogCables,
        KindCompatibility,
        ObjectWeights,
        VortexWeights,
    }
    let object = |index| FixtureObject { id: format!("object-{index:02}"), object_kind: None, anchor: Default::default(), mesh_url: None, origin: [0.0; 3], orientation: None, scale: None, vortices: Vec::new(), reveal_index: None };
    let body = collision_body_from_buffers(&[0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 4.0, 0.0], &[0, 1, 2]).expect("body");
    let roots = |branch: HostileRoot, count| {
        let mut scene =
            SceneConfig { fixture: Fixture::default(), kind_catalogs: Some(KindCatalogBundle::default()), kind_compatibility: Vec::new(), overlap_budget: 0.0, seed: 31, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() };
        let mut meshes = HashMap::new();
        match branch {
            HostileRoot::FixtureObjects => scene.fixture.objects.extend((0..count).map(object)),
            HostileRoot::FixtureAttractions => scene.fixture.attractions.extend((0..count).map(|index| AttractionProps {
                id: format!("attraction-{index:02}"),
                attracting: format!("a-{index:02}"),
                attracted: format!("b-{index:02}"),
                gap: 0.0,
                shift: 0.0,
                rise: 0.0,
                rotation: 0.0,
                turn: 0.0,
                tilt: 0.0,
                x: 0.0,
                y: 0.0,
            })),
            HostileRoot::FixtureTargetVolumes => scene.fixture.target_volumes.extend((0..count).map(|index| WorldVolumeProps { id: format!("volume-{index:02}"), origin: [0.0; 3], orientation: None, scale: None })),
            HostileRoot::Meshes => meshes.extend((0..count).map(|index| (format!("mesh-{index:02}"), body.clone()))),
            HostileRoot::CatalogObjects => scene.kind_catalogs.as_mut().expect("catalogs").objects.extend((0..count).map(|index| ObjectKind { id: format!("catalog-object-{index:02}"), ..Default::default() })),
            HostileRoot::CatalogVortices => scene.kind_catalogs.as_mut().expect("catalogs").vortices.extend((0..count).map(|index| VortexKindCatalog { id: format!("catalog-vortex-{index:02}"), ..Default::default() })),
            HostileRoot::CatalogCables => scene.kind_catalogs.as_mut().expect("catalogs").cables.extend((0..count).map(|index| CableKindCatalog { id: format!("catalog-cable-{index:02}"), ..Default::default() })),
            HostileRoot::KindCompatibility => {
                scene.kind_compatibility.extend((0..count).map(|index| KindCompatEntry { source: format!("compat-{index:02}"), target: format!("target-{index:02}"), bidirectional: false, important: false, specificity: None }))
            }
            HostileRoot::ObjectWeights => scene.weights.object_weights.extend((0..count).map(|index| (format!("object-weight-{index:04}"), index as f64 + 0.25))),
            HostileRoot::VortexWeights => scene.weights.vortex_weights.extend((0..count).map(|index| (format!("vortex-weight-{index:04}"), index as f64 + 0.5))),
        }
        FillPreparationRoots::new(Arc::new(scene), Arc::new(meshes))
    };
    let branches = [
        (HostileRoot::FixtureObjects, "fixture-objects", DOCUMENT_OBJECT_SLOTS),
        (HostileRoot::FixtureAttractions, "fixture-attractions", DOCUMENT_ATTRACTION_SLOTS),
        (HostileRoot::FixtureTargetVolumes, "fixture-target-volumes", DOCUMENT_VOLUME_SLOTS),
        (HostileRoot::Meshes, "meshes", DOCUMENT_KIND_SLOTS),
        (HostileRoot::CatalogObjects, "catalog-objects", DOCUMENT_KIND_SLOTS),
        (HostileRoot::CatalogVortices, "catalog-vortices", DOCUMENT_KIND_SLOTS),
        (HostileRoot::CatalogCables, "catalog-cables", DOCUMENT_KIND_SLOTS),
        (HostileRoot::KindCompatibility, "kind-compatibility", DOCUMENT_KIND_SLOTS),
        (HostileRoot::ObjectWeights, "object-weights", DOCUMENT_KIND_SLOTS),
        (HostileRoot::VortexWeights, "vortex-weights", DOCUMENT_KIND_SLOTS),
    ];
    for (offset, (branch, expected_branch, cap)) in branches.into_iter().enumerate() {
        let operation = Operation::new(OperationId(31 + offset as u64), RevisionId(1), Generation(1), 31);
        let mut accepted = FillBuilder::begin_preparation(roots(branch, cap), operation, TEST_REQUESTED_COUNT);
        let mut turns = 0;
        while accepted.stage != FillJobStage::PrepareTargets {
            accepted.prepare_one();
            turns += 1;
            assert!(turns < 16 * DOCUMENT_OBJECT_SLOTS, "{expected_branch} cap preparation must advance in bounded turns");
        }
        assert!(turns >= cap, "{expected_branch} cap must be installed cooperatively");

        let mut rejected = FillBuilder::begin_preparation(roots(branch, cap + 1), operation, TEST_REQUESTED_COUNT);
        let (actual_branch, exact_index, exact_owner, exact_weight) = rejected.preparation_refusal_owner_for_test().expect("attributable omitted owner");
        assert_eq!(actual_branch, expected_branch);
        assert_eq!(exact_index, cap);
        assert!(!exact_owner.is_empty());
        let omitted_object_weight = format!("object-weight-{cap:04}");
        let omitted_vortex_weight = format!("vortex-weight-{cap:04}");
        match branch {
            HostileRoot::ObjectWeights => assert_eq!((exact_owner.as_str(), exact_weight), (omitted_object_weight.as_str(), Some(cap as f64 + 0.25))),
            HostileRoot::VortexWeights => assert_eq!((exact_owner.as_str(), exact_weight), (omitted_vortex_weight.as_str(), Some(cap as f64 + 0.5))),
            _ => assert_eq!(exact_weight, None),
        }
        assert_eq!(
            (
                rejected.base.objects.len(),
                rejected.base.attractions.len(),
                rejected.base.target_volumes.len(),
                rejected.catalogs.objects.len(),
                rejected.catalogs.vortices.len(),
                rejected.catalogs.cables.len(),
                rejected.kind_compatibility.len(),
                rejected.meshes.len(),
                rejected.weights.object_weights.len(),
                rejected.weights.vortex_weights.len()
            ),
            (0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
        );
        let mut preview_sequence = 0;
        let mut context = test_context(&rejected, root_cancel_token(), &mut preview_sequence);
        assert!(matches!(rejected.step(&mut context), StepOutcome::PreviewReady(_)));
        assert_eq!(rejected.preview.rejection_reason.as_deref(), Some(format!("preparation-capacity:{expected_branch}:{cap}").as_str()));
        assert_eq!(rejected.preview.stall_reason.as_deref(), Some(FILL_STALL_DOCUMENT_CAPACITY));
        assert!(rejected.preview.candidate_ghost.is_none());
        assert!(matches!(rejected.step(&mut context), StepOutcome::Complete(_)), "a full document page stalls visibly instead of faulting");
        assert!(rejected.stalled && rejected.stage == FillJobStage::Complete);
        assert_eq!(
            (
                rejected.base.objects.len(),
                rejected.base.attractions.len(),
                rejected.base.target_volumes.len(),
                rejected.catalogs.objects.len(),
                rejected.catalogs.vortices.len(),
                rejected.catalogs.cables.len(),
                rejected.kind_compatibility.len(),
                rejected.meshes.len(),
                rejected.weights.object_weights.len(),
                rejected.weights.vortex_weights.len()
            ),
            (0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
        );
    }
}

#[test]
fn capacity_refusal_publishes_generation_qualified_no_ghost_diagnostic_before_stalling() {
    let objects = (0..=DOCUMENT_OBJECT_SLOTS)
        .map(|index| FixtureObject { id: format!("rejected-{index:04}"), object_kind: None, anchor: Default::default(), mesh_url: None, origin: [0.0; 3], orientation: None, scale: None, vortices: Vec::new(), reveal_index: None })
        .collect();
    let scene = Arc::new(SceneConfig {
        fixture: Fixture { objects, attractions: Vec::new(), target_volumes: Vec::new() },
        kind_catalogs: Some(KindCatalogBundle::default()),
        kind_compatibility: Vec::new(),
        overlap_budget: 0.0,
        seed: 37,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    });
    let mut builder = FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(HashMap::new())), Operation::new(OperationId(37), RevisionId(9), Generation(11), 37), TEST_REQUESTED_COUNT);
    builder.preview.registry_generation = 13;
    let mut sequence = 0;
    let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
    assert!(matches!(builder.step(&mut context), StepOutcome::PreviewReady(_)));
    assert_eq!((builder.preview.operation, builder.preview.base_revision, builder.preview.registry_generation, builder.preview.generation), (37, 9, 13, 11));
    assert_eq!(builder.preview.rejection_reason.as_deref(), Some(format!("preparation-capacity:fixture-objects:{DOCUMENT_OBJECT_SLOTS}").as_str()));
    assert!(builder.preview.candidate_ghost.is_none());
    assert_eq!(builder.preview.stall_reason.as_deref(), Some(FILL_STALL_DOCUMENT_CAPACITY));
    assert_eq!(builder.preview.sequence, 0, "the refusal is published under the operation's own first preview sequence");
    assert!(matches!(builder.step(&mut context), StepOutcome::Complete(_)));
    drop(context);
    assert_eq!(sequence, 1, "publishing the refusal consumed exactly one preview sequence, and the completion that follows consumes none");
}

#[test]
fn stale_generation_stops_preparation_before_installing_any_entry() {
    let mut builder = empty_builder();
    let before = (builder.base.objects.len(), builder.placed.len(), builder.placed_lookup.len());
    let mut sequence = 0;
    let mut context = StepContext::new(builder.operation.operation, Generation(builder.operation.generation.0 + 1), StepBudget::new(1, 1), root_cancel_token(), || Some(0), &mut sequence);
    assert!(faulted(builder.step(&mut context)));
    assert_eq!((builder.base.objects.len(), builder.placed.len(), builder.placed_lookup.len()), before);
}

#[test]
fn retained_owner_census_credits_each_actual_fixed_slot_page_not_a_layout_heuristic() {
    let builder = empty_builder();
    let expected = [
        builder.placed_lookup.backing_credit().expect("placed page").1,
        builder.candidate_cache.backing_credit().expect("cache page").1,
        builder.seed_object_ids.backing_credit().expect("seed page").1,
        builder.weights.object_weights.backing_credit().expect("object-weight page").1,
        builder.weights.vortex_weights.backing_credit().expect("vortex-weight page").1,
        builder.meshes.backing_credit().expect("mesh page").1,
        builder.blocked_vortex_ids.backing_credit().expect("blocked page").1,
        builder.candidate_seen.backing_credit().expect("seen page").1,
        builder.candidate_cross.backing_credit().expect("cross page").1,
        builder.candidate_same.backing_credit().expect("same page").1,
    ];
    let mut cursor = FillBuilderOwnerCensusCursor::default();
    assert_eq!(cursor.step(&builder, usize::MAX, usize::MAX), FillBuilderOwnerCensusStep::Pending);
    for (page, expected_bytes) in expected.into_iter().enumerate() {
        let before = cursor.credit;
        assert_eq!(cursor.step(&builder, usize::MAX, usize::MAX), FillBuilderOwnerCensusStep::Pending);
        assert_eq!(cursor.credit.items - before.items, 1, "fixed backing page {page} has one exact owner");
        assert_eq!(cursor.credit.bytes - before.bytes, expected_bytes, "fixed backing credit equals the actual slot array allocation");
        assert!(expected_bytes <= DOCUMENT_OWNER_PAGE_BYTES);
    }
}

#[test]
fn all_fill_fixed_collections_store_max_entries_in_the_credited_page_and_return_plus_one() {
    fn map_boundary<V>(mut value: impl FnMut(usize) -> V) {
        let mut map = FixedOwnerMap::<String, V>::new();
        let page = map.backing_ptr().expect("actual fixed page");
        let credit = map.backing_credit().expect("credited fixed page");
        assert_eq!(credit, (1, FixedOwnerMap::<String, V>::page_bytes()));
        assert!(credit.1 <= FIXED_OWNER_PAGE_BYTES);
        for index in 0..FIXED_OWNER_SLOTS {
            assert!(matches!(map.try_insert(format!("key-{index:02}"), value(index)), Ok(FixedOwnerMapInsert::Inserted)));
        }
        let rejected = String::from("key-plus-one");
        let rejected_ptr = rejected.as_ptr();
        let Err((rejected, _)) = map.try_insert(rejected, value(FIXED_OWNER_SLOTS)) else { panic!("cap + 1 must reject") };
        assert_eq!(rejected.as_ptr(), rejected_ptr, "cap + 1 returns the identical key owner");
        assert_eq!(map.backing_ptr(), Some(page), "no second backing can be allocated");
        for _ in 0..FIXED_OWNER_SLOTS {
            drop(map.pop_first().expect("one semantic owner per close grant"));
            assert_eq!(map.backing_ptr(), Some(page));
        }
        assert!(map.retire_backing(), "the same actual slot page returns after semantic owners");
        assert!(map.terminal_owners_empty());
    }

    fn set_boundary() {
        let mut set = FixedOwnerSet::<String>::new();
        let page = set.backing_ptr().expect("actual fixed page");
        for index in 0..FIXED_OWNER_SLOTS {
            assert!(matches!(set.try_insert(format!("set-{index:02}")), Ok(FixedOwnerSetInsert::Inserted)));
        }
        let rejected = String::from("set-plus-one");
        let rejected_ptr = rejected.as_ptr();
        let Err(rejected) = set.try_insert(rejected) else { panic!("cap + 1 must reject") };
        assert_eq!(rejected.as_ptr(), rejected_ptr, "cap + 1 returns the identical set owner");
        assert_eq!(set.backing_ptr(), Some(page));
        for _ in 0..FIXED_OWNER_SLOTS {
            drop(set.pop_first().expect("one semantic owner per close grant"));
        }
        assert!(set.retire_backing());
        assert!(set.terminal_owners_empty());
    }

    fn vec_boundary() {
        let mut values = FixedOwnerVec::<String>::new();
        let page = values.backing_ptr().expect("actual fixed vector page");
        for index in 0..FIXED_OWNER_SLOTS {
            assert!(values.try_push(format!("vector-{index:02}")).is_ok());
        }
        let rejected = String::from("vector-plus-one");
        let rejected_ptr = rejected.as_ptr();
        let Err(rejected) = values.try_push(rejected) else { panic!("vector cap + 1 must reject") };
        assert_eq!(rejected.as_ptr(), rejected_ptr, "cap + 1 returns the exact omitted vector owner");
        assert_eq!(values.backing_ptr(), Some(page));
        for _ in 0..FIXED_OWNER_SLOTS {
            drop(values.pop().expect("one semantic vector owner per close grant"));
        }
        assert!(values.retire_backing());
        assert!(values.terminal_owners_empty());
    }

    map_boundary(|index| index);
    map_boundary(|index| vec![BrushCompatibleCandidate { object_kind_id: format!("cache-{index}"), source_vortex_index: index }]);
    set_boundary();
    map_boundary(|index| index as f64);
    map_boundary(|index| index as f64);
    let body = collision_body_from_buffers(&[0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 4.0, 0.0], &[0, 1, 2]).expect("body");
    map_boundary(|_| body.clone());
    set_boundary();
    set_boundary();
    map_boundary(|index| BrushCompatibleCandidate { object_kind_id: format!("cross-{index}"), source_vortex_index: index });
    map_boundary(|index| BrushCompatibleCandidate { object_kind_id: format!("same-{index}"), source_vortex_index: index });
    vec_boundary();

    let mut cache = FixedOwnerMap::<String, Vec<BrushCompatibleCandidate>>::new();
    for index in 0..FIXED_OWNER_SLOTS {
        assert!(matches!(cache.try_insert(format!("cache-{index:02}"), Vec::new()), Ok(FixedOwnerMapInsert::Inserted)));
    }
    let rejected_key = String::from("cache-plus-one");
    let rejected_key_ptr = rejected_key.as_ptr();
    let rejected_value = vec![BrushCompatibleCandidate { object_kind_id: "identical-value".into(), source_vortex_index: 0 }];
    let rejected_value_ptr = rejected_value.as_ptr();
    let rejected_nested_ptr = rejected_value[0].object_kind_id.as_ptr();
    let Err((rejected_key, rejected_value)) = cache.try_insert(rejected_key, rejected_value) else { panic!("cache cap + 1") };
    assert_eq!(rejected_key.as_ptr(), rejected_key_ptr);
    assert_eq!(rejected_value.as_ptr(), rejected_value_ptr, "cap + 1 returns the identical nested value owner");
    assert_eq!(rejected_value[0].object_kind_id.as_ptr(), rejected_nested_ptr);
    drop(rejected_key);
    drop(rejected_value);
    for _ in 0..FIXED_OWNER_SLOTS {
        drop(cache.pop_first().expect("one retained cache entry per close grant"));
    }
    assert!(cache.retire_backing());
    assert!(cache.terminal_owners_empty());
}

#[test]
fn occupied_fixed_slot_returns_the_distinct_input_owners_without_replacing_stored_owners() {
    let mut map = FixedOwnerMap::<String, Vec<String>>::new();
    let mut stored_key = String::with_capacity(64);
    stored_key.push_str("equal-key");
    let stored_key_ptr = stored_key.as_ptr();
    let stored_value = vec![String::from("stored-value")];
    let stored_value_ptr = stored_value.as_ptr();
    assert!(matches!(map.try_insert(stored_key, stored_value), Ok(FixedOwnerMapInsert::Inserted)));

    let mut input_key = String::with_capacity(256);
    input_key.push_str("equal-key");
    let input_key_ptr = input_key.as_ptr();
    let input_value = vec![String::from("input-value")];
    let input_value_ptr = input_value.as_ptr();
    let Ok(FixedOwnerMapInsert::Occupied { input_key, input_value }) = map.try_insert(input_key, input_value) else { panic!("equal key must return a typed occupied outcome") };
    assert_eq!(input_key.as_ptr(), input_key_ptr);
    assert_eq!(input_value.as_ptr(), input_value_ptr);
    let (retained_key, retained_value) = map.iter().next().expect("stored owner remains retained");
    assert_eq!(retained_key.as_ptr(), stored_key_ptr);
    assert_eq!(retained_value.as_ptr(), stored_value_ptr);

    drop(input_key);
    drop(input_value);
    drop(map.pop_first().expect("stored pair retires as one semantic owner"));
    assert!(map.retire_backing(), "actual page retires only after its stored pair");
    assert!(map.terminal_owners_empty());
}

#[test]
fn cancellation_is_observed_before_the_next_transition() {
    let mut builder = empty_builder();
    let cancel = root_cancel_token();
    cancel.cancel_now();
    let mut sequence = 0;
    let mut context = test_context(&builder, cancel, &mut sequence);
    assert_eq!(builder.step(&mut context), StepOutcome::Cancelled);
}

#[test]
fn preview_payload_is_typed_revisioned_and_bounded() {
    let mut builder = empty_builder();
    builder.operation = Operation::new(OperationId(41), RevisionId(7), Generation(3), 17);
    builder.current_preview =
        Some(BrushPreviewState { target_vortex_full_id: "host:v0".into(), object_kind_id: "candidate".into(), source_vortex_index: 2, mesh_url: "/candidate.glb".into(), origin: [1.0, 2.0, 3.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None });
    builder.preview.candidate_ghost = builder.current_preview.clone();
    builder.preview.candidate_page[0] = Some("a".into());
    builder.preview.candidate_page[1] = Some("b".into());
    builder.preview.current_pair_object_id = Some("a".into());
    builder.preview.collision_count = 1;
    builder.preview.rejection_reason = Some("solid-overlap".into());
    builder.transition_count = 23;
    builder.rejected_count = 4;
    let mut sequence = 0;
    let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
    let StepOutcome::PreviewReady(bytes) = builder.publish_preview(&mut context) else { panic!("preview") };
    assert!(bytes.is_empty(), "the retained envelope observes the shared preview without serializing it");
    assert_eq!((builder.preview.operation, builder.preview.base_revision, builder.preview.generation), (41, 7, 3));
    assert_eq!(builder.preview.candidate_ghost.as_ref().map(|ghost| ghost.mesh_url.as_str()), Some("/candidate.glb"));
    assert_eq!(builder.preview.collision_count, 1);
    assert_eq!((builder.preview.search_count, builder.preview.rejected_count), (23, 4));
}

#[test]
fn stale_generation_faults_without_progress() {
    fn now() -> Option<u64> {
        Some(0)
    }
    let mut builder = empty_builder();
    let mut sequence = 0;
    let mut context = StepContext::new(OperationId(builder.operation.operation.0), Generation(builder.operation.generation.0 + 1), StepBudget::new(100, 10), root_cancel_token(), now, &mut sequence);
    let StepOutcome::Fault(fault) = builder.step(&mut context) else { panic!("a stale generation must fault") };
    assert_eq!(fault.detail.single_page(), Some(b"stale-fill-operation".as_slice()));
    assert!(faulted(StepOutcome::Fault(fault)));
    assert_eq!(builder.operation.base_revision, RevisionId(1));
}

/// 📏️ Turns an EMPTY scene's cursorized planner needs to reach `Complete`: seven preparation stages
/// (three of them walking three empty roots apiece), then target preparation and the stall, one
/// bounded unit per turn. Doubled as headroom for a stage split.
const EMPTY_FILL_TRANSITION_TURNS: usize = 64;

#[test]
fn empty_fill_transition_stays_below_watchdog_ceiling() {
    let mut builder = empty_builder();
    let mut sequence = 0;
    for _ in 0..EMPTY_FILL_TRANSITION_TURNS {
        let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
        let started = Instant::now();
        let _ = builder.step(&mut context);
        assert!(started.elapsed() < Duration::from_millis(8));
        if builder.stage == FillJobStage::Complete {
            break;
        }
    }
    assert_eq!(builder.stage, FillJobStage::Complete);
}

#[test]
fn adversarial_broad_phase_fill_is_end_to_end_resumable_below_eight_ms() {
    let representation = |id: &str| ObjectKindRepresentation { id: id.into(), name: String::new(), url: "/stress/box.glb".into(), mime: String::new(), tags: Vec::new(), lod: None, description: String::new() };
    let candidate_vortex = ObjectKindVortexTemplate { vortex_kind: Some("port-a".into()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]), ..Default::default() };
    let catalogs = KindCatalogBundle {
        objects: vec![
            ObjectKind { id: "Host".into(), representations: vec![representation("host")], scale: None, vortices: Vec::new() },
            ObjectKind { id: "Obstacle".into(), representations: vec![representation("obstacle")], scale: None, vortices: Vec::new() },
            ObjectKind { id: "Placed".into(), representations: vec![representation("placed")], scale: None, vortices: vec![candidate_vortex] },
        ],
        vortices: Vec::new(),
        cables: Vec::new(),
    };
    let host = FixtureObject {
        id: "host".into(),
        object_kind: Some("Host".into()),
        anchor: Default::default(),
        mesh_url: Some("/stress/box.glb".into()),
        origin: [0.0, 0.0, 0.0],
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        vortices: vec![VortexProps { id: "v0".into(), vortex_kind: Some("port-a".into()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
        reveal_index: None,
    };
    let mut objects = vec![host];
    objects.extend((0..30).map(|index| FixtureObject {
        id: format!("obstacle-{index:04}"),
        object_kind: Some("Obstacle".into()),
        anchor: Default::default(),
        mesh_url: Some("/stress/box.glb".into()),
        origin: [10_000.0 + index as f64 * 16.0, 0.0, 0.0],
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        vortices: Vec::new(),
        reveal_index: None,
    }));
    let positions = [-4.0, -4.0, 0.0, 4.0, -4.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 8.0];
    let indices = [0, 1, 2, 0, 1, 3, 1, 2, 3, 2, 0, 3];
    let body = collision_body_from_buffers(&positions, &indices).expect("stress body");
    let meshes = HashMap::from([("/stress/box.glb".to_string(), body)]);
    let scene = Arc::new(SceneConfig {
        fixture: Fixture { objects, attractions: Vec::new(), target_volumes: Vec::new() },
        kind_catalogs: Some(catalogs),
        kind_compatibility: Vec::new(),
        overlap_budget: 0.0,
        seed: 29,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    });
    let mut builder = FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(meshes)), Operation::new(OperationId(29), RevisionId(1), Generation(1), 29), TEST_REQUESTED_COUNT);
    let mut sequence = 0;
    let started = Instant::now();
    let mut first_candidate = None;
    let mut max_step = Duration::ZERO;
    for _ in 0..50_000 {
        let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
        let step_started = Instant::now();
        let outcome = builder.step(&mut context);
        let step_elapsed = step_started.elapsed();
        max_step = max_step.max(step_elapsed);
        assert!(step_elapsed < Duration::from_millis(8), "stage {:?} reached the 8ms ceiling", builder.stage);
        if first_candidate.is_none() && builder.preview.candidate_ghost.is_some() {
            first_candidate = Some(started.elapsed());
        }
        if outcome.is_terminal() {
            break;
        }
    }
    assert!(first_candidate.is_some_and(|elapsed| elapsed < Duration::from_millis(50)), "adversarial fill did not publish its first candidate within 50ms: {first_candidate:?}");
    assert_eq!(builder.stage, FillJobStage::Complete);
    assert_eq!(builder.sequence.len(), 1);
}

#[test]
fn document_capacities_match_the_language_neutral_capacity_law() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("language-neutral law fixture");
    let capacities = &law["documentCapacities"];
    let declared = |field: &str| capacities[field].as_u64().unwrap_or_else(|| panic!("{field} capacity")) as usize;
    assert_eq!([declared("bookkeepingSlots"), declared("bookkeepingPageBytes"), declared("documentPageBytes")], [FIXED_OWNER_SLOTS, FIXED_OWNER_PAGE_BYTES, DOCUMENT_OWNER_PAGE_BYTES]);
    assert_eq!(
        [declared("objectSlots"), declared("attractionSlots"), declared("vortexSlots"), declared("volumeSlots"), declared("kindSlots"), declared("candidateSlots"), declared("cellSlots")],
        [DOCUMENT_OBJECT_SLOTS, DOCUMENT_ATTRACTION_SLOTS, DOCUMENT_VORTEX_SLOTS, DOCUMENT_VOLUME_SLOTS, DOCUMENT_KIND_SLOTS, DOCUMENT_CANDIDATE_SLOTS, DOCUMENT_CELL_SLOTS]
    );
    let nakagin = |field: &str| capacities["nakagin"][field].as_u64().unwrap_or_else(|| panic!("nakagin {field}")) as usize;
    assert!(nakagin("objects") < DOCUMENT_OBJECT_SLOTS, "the flagship fixture leaves the object capacity room to plan into");
    assert!(nakagin("attractions") < DOCUMENT_ATTRACTION_SLOTS);
    assert!(nakagin("vortices") <= DOCUMENT_VORTEX_SLOTS && nakagin("objects") <= nakagin("vortices"), "the measured vortices-per-object ratio backs the vortex capacity");
    assert!(nakagin("objectKinds").max(nakagin("vortexKinds")).max(nakagin("compatibilityRows")) <= DOCUMENT_KIND_SLOTS);
    assert!(DOCUMENT_CELL_SLOTS > FIXED_OWNER_SLOTS && DOCUMENT_OBJECT_SLOTS > FIXED_OWNER_SLOTS, "document capacities are never the bookkeeping batch");
}

#[test]
fn document_scale_fixed_pages_are_admitted_by_the_fill_envelope_reservation() {
    let builder = empty_builder();
    for (page, (_, bytes, _)) in builder.fixed_backing_witness_for_test().into_iter().enumerate() {
        assert!(bytes <= DOCUMENT_OWNER_PAGE_BYTES, "document page {page} claims {bytes} bytes beyond the declared page ceiling");
    }
    let mut cursor = FillBuilderOwnerCensusCursor::default();
    let credit = loop {
        match cursor.step(&builder, FILL_ENVELOPE_MAX_ITEMS, FILL_ENVELOPE_MAX_BYTES) {
            FillBuilderOwnerCensusStep::Pending => {}
            FillBuilderOwnerCensusStep::Complete(credit) => break credit,
            FillBuilderOwnerCensusStep::Rejected => panic!("a document-scale builder must fit one fill envelope reservation"),
        }
    };
    assert!(credit.items <= FILL_ENVELOPE_MAX_ITEMS && credit.bytes <= FILL_ENVELOPE_MAX_BYTES);
    assert!(credit.bytes > 10 * FIXED_OWNER_PAGE_BYTES, "document pages are actually credited, not silently absent: {credit:?}");
}

const NAKAGIN_OBJECTS: usize = 180;
const NAKAGIN_OBJECT_KINDS: usize = 12;
const NAKAGIN_VORTEX_KINDS: usize = 18;
const NAKAGIN_COMPATIBILITY_ROWS: usize = 14;
const NAKAGIN_MESH_URL: &str = "/nakagin/capsule.glb";

/// 🏢️ The flagship fixture at document scale — 180 capsules, 360 attractions, 12 object kinds, 18
/// vortex kinds, 14 compatibility rows — as the fill lane's own preparation roots. Two laws drive
/// it: one on an unconstrained guest, one under a fragmented guest's reservation ceiling.
fn nakagin_scale_roots() -> FillPreparationRoots {
    let template = ObjectKindVortexTemplate { vortex_kind: Some("port-00".into()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]), ..Default::default() };
    let catalogs = KindCatalogBundle {
        objects: (0..NAKAGIN_OBJECT_KINDS)
            .map(|index| ObjectKind {
                id: format!("capsule-kind-{index:02}"),
                representations: vec![ObjectKindRepresentation { id: format!("capsule-representation-{index:02}"), name: String::new(), url: NAKAGIN_MESH_URL.into(), mime: String::new(), tags: Vec::new(), lod: None, description: String::new() }],
                scale: None,
                vortices: vec![template.clone()],
            })
            .collect(),
        vortices: (0..NAKAGIN_VORTEX_KINDS).map(|index| VortexKindCatalog { id: format!("port-{index:02}"), ..Default::default() }).collect(),
        cables: Vec::new(),
    };
    let objects: Vec<FixtureObject> = (0..NAKAGIN_OBJECTS)
        .map(|index| FixtureObject {
            id: format!("capsule-{index:03}"),
            object_kind: Some(format!("capsule-kind-{:02}", index % NAKAGIN_OBJECT_KINDS)),
            anchor: Default::default(),
            mesh_url: Some(NAKAGIN_MESH_URL.into()),
            origin: [(index % 12) as f64 * 64.0, (index / 12) as f64 * 64.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            vortices: vec![
                VortexProps { id: "v0".into(), vortex_kind: Some("port-00".into()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) },
                VortexProps { id: "v1".into(), vortex_kind: Some("port-00".into()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, 1.0]) },
            ],
            reveal_index: None,
        })
        .collect();
    let attraction = |index: usize, vortex: &str| AttractionProps {
        id: format!("cable-{vortex}-{index:03}"),
        attracting: puzzle3d_vortex_full_id(&format!("capsule-{index:03}"), vortex),
        attracted: puzzle3d_vortex_full_id(&format!("capsule-{:03}", (index + 1) % NAKAGIN_OBJECTS), vortex),
        gap: 0.0,
        shift: 0.0,
        rise: 0.0,
        rotation: 0.0,
        turn: 0.0,
        tilt: 0.0,
        x: 0.0,
        y: 0.0,
    };
    let attractions: Vec<AttractionProps> = (0..NAKAGIN_OBJECTS).map(|index| attraction(index, "connected-a")).chain((0..NAKAGIN_OBJECTS).map(|index| attraction(index, "connected-b"))).collect();
    let kind_compatibility: Vec<KindCompatEntry> = (0..NAKAGIN_COMPATIBILITY_ROWS)
        .map(|index| KindCompatEntry { source: format!("port-{index:02}"), target: format!("port-{index:02}"), bidirectional: true, important: false, specificity: Some("vortex".into()) })
        .collect();
    let body = collision_body_from_buffers(&[-4.0, -4.0, 0.0, 4.0, -4.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 8.0], &[0, 1, 2, 0, 1, 3, 1, 2, 3, 2, 0, 3]).expect("capsule body");
    let scene = Arc::new(SceneConfig {
        fixture: Fixture { objects, attractions, target_volumes: Vec::new() },
        kind_catalogs: Some(catalogs),
        kind_compatibility,
        overlap_budget: 0.0,
        seed: 43,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    });
    FillPreparationRoots::new(scene, Arc::new(HashMap::from([(NAKAGIN_MESH_URL.to_string(), body)])))
}

/// 🏢️ Drives a document-scale preparation until it places its first object, refusing to accept a
/// fault, a capacity refusal, or a terminal outcome that placed nothing.
fn drive_nakagin_scale_fill() -> FillBuilder {
    let mut builder = FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(43), RevisionId(1), Generation(1), 43), TEST_REQUESTED_COUNT);
    assert_eq!(builder.preview.rejection_reason, None, "a Nakagin-scale document must not be refused before preparation starts");
    let mut sequence = 0;
    let mut turns = 0;
    while builder.sequence.is_empty() {
        let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
        let outcome = builder.step(&mut context);
        turns += 1;
        let terminal = outcome.is_terminal();
        assert!(!faulted(outcome), "Nakagin-scale fill faulted at stage {:?} after {turns} turns: {:?}", builder.stage, builder.preview.rejection_reason);
        assert!(builder.preview.rejection_reason.as_deref().is_none_or(|reason| !reason.starts_with("preparation-capacity")), "document scale must not publish a capacity refusal: {:?}", builder.preview.rejection_reason);
        assert!(!terminal || !builder.sequence.is_empty(), "Nakagin-scale fill ended after {turns} turns without placing an object: {:?}", builder.preview.rejection_reason);
        assert!(turns < 400_000, "Nakagin-scale fill did not place an object in bounded turns");
    }
    builder
}

#[test]
fn nakagin_scale_fill_is_not_refused_and_places_at_least_one_object() {
    let builder = drive_nakagin_scale_fill();
    assert_eq!((builder.base.objects.len(), builder.base.attractions.len()), (NAKAGIN_OBJECTS, 2 * NAKAGIN_OBJECTS));
    assert_eq!((builder.catalogs.objects.len(), builder.catalogs.vortices.len(), builder.kind_compatibility.len()), (NAKAGIN_OBJECT_KINDS, NAKAGIN_VORTEX_KINDS, NAKAGIN_COMPATIBILITY_ROWS));
    assert_eq!(builder.placed_lookup.len(), NAKAGIN_OBJECTS + 1);
    assert_eq!(builder.appended_objects.len(), 1);
}

/// ⚖️ LAW: a fill session on a guest that refuses every contiguous request over
/// `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` still prepares a Nakagin-scale document and places an
/// object — no owner of this lane asks for a single block the fragmented guest cannot serve.
///
/// 🧊️ A wasm guest runs on one linear memory that grows and never shrinks, served by `dlmalloc` with
/// a 64 KiB granularity: the FIRST request a fragmented or nearly-full guest refuses is one larger
/// than a granularity unit. The fill session's own owners used to be exactly that — one 432 KiB
/// block for `FixedOwnerVec<FixtureObject, DOCUMENT_OBJECT_SLOTS>`, one 96 KiB block for the
/// collision entry map — so ~44 s into a Nakagin fill run the guest refused them and the whole plan
/// was abandoned (`CollisionMutationStep::Rejected(Capacity)`, ticket 26/09/02 build #29 and W-F6
/// §8 item 2). A native suite cannot exhaust a 512 MiB linear memory, so the law installs the
/// fragmented guest as a reservation policy instead.
#[test]
fn nakagin_scale_fill_places_an_object_under_a_fragmented_guest_reservation_ceiling() {
    let _fragmented = OwnerReservationLimit::install(GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, usize::MAX);
    let builder = drive_nakagin_scale_fill();
    assert_eq!((builder.base.objects.len(), builder.base.attractions.len()), (NAKAGIN_OBJECTS, 2 * NAKAGIN_OBJECTS));
    assert_eq!(builder.appended_objects.len(), 1, "the fragmented guest still places its first object");
    assert_eq!(builder.preview.rejection_reason, None);
}

/// 🔁️ Drives a planner until it settles — either it reached what was asked for or it named a stall.
fn drive_until_settled(builder: &mut FillBuilder, turns: usize) {
    let mut sequence = 0;
    for _ in 0..turns {
        if builder.stage == FillJobStage::Complete {
            return;
        }
        let mut context = test_context(builder, root_cancel_token(), &mut sequence);
        let outcome = builder.step(&mut context);
        assert!(!faulted(outcome), "a settled planner never faults: {:?} / {:?}", builder.preview.rejection_reason, builder.preview.stall_reason);
    }
    panic!("planner did not settle in {turns} turns at stage {:?} with {} placements", builder.stage, builder.sequence.len());
}

fn nakagin_plan(requested: usize, turns: usize) -> FillBuilder {
    let mut builder = FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(61), RevisionId(1), Generation(1), 43), requested);
    drive_until_settled(&mut builder, turns);
    builder
}

fn plan_identity(builder: &FillBuilder) -> Vec<(String, String, usize)> {
    builder.sequence.iter().map(|payload| (payload.target_vortex_full_id.clone(), payload.object_kind_id.clone(), payload.source_vortex_index)).collect()
}

/// ⚖️ LAW: asking for more never rewinds the RNG stream — the longer plan keeps the shorter one as
/// its exact prefix, so raising the count continues the placements the user already sees instead of
/// replanning them somewhere else.
#[test]
fn raising_the_requested_count_continues_the_plan_as_an_exact_prefix() {
    const SHORT: usize = 100;
    const LONG: usize = 150;
    let mut raised = nakagin_plan(SHORT, 4_000_000);
    assert_eq!(raised.sequence.len(), SHORT, "the short plan stops exactly at what was asked for");
    assert_eq!(raised.requested_count(), SHORT);
    assert_eq!(raised.preview.requested_count, SHORT);
    let short_identity = plan_identity(&raised);
    let short_rng = raised.rng_state;

    raised.set_requested_count(LONG);
    assert_eq!((raised.requested_count(), raised.stage, raised.stalled), (LONG, FillJobStage::PrepareTargets, false), "a completed planner wakes up at target selection");
    assert_eq!(raised.rng_state, short_rng, "raising never rewinds the stream");
    drive_until_settled(&mut raised, 4_000_000);
    assert_eq!(raised.sequence.len(), LONG);

    let long = nakagin_plan(LONG, 4_000_000);
    assert_eq!(long.sequence.len(), LONG);
    assert_eq!(plan_identity(&raised), plan_identity(&long), "the resumed plan and the one long plan are the same sequence");
    assert_eq!(&plan_identity(&long)[..SHORT], short_identity.as_slice(), "the short plan is the long plan's prefix");
}

/// ⚖️ LAW: asking for less hands the surplus tail back — the plan itself shrinks, each discarded
/// placement withdraws its own collision owner, and asking for more again continues from there.
#[test]
fn lowering_the_requested_count_discards_the_planned_tail_and_raising_continues() {
    const PLANNED: usize = 100;
    const LOWERED: usize = 60;
    const RAISED: usize = 80;
    let mut builder = nakagin_plan(PLANNED, 4_000_000);
    assert_eq!(builder.sequence.len(), PLANNED);
    let owners_before = builder.placed_lookup.len();

    builder.set_requested_count(LOWERED);
    assert_eq!((builder.requested_count(), builder.stage), (LOWERED, FillJobStage::DiscardTail));
    drive_until_settled(&mut builder, 4_000_000);
    assert_eq!((builder.sequence.len(), builder.appended_objects.len(), builder.appended_attractions.len()), (LOWERED, LOWERED, LOWERED));
    assert_eq!(builder.preview.accepted_count, LOWERED);
    assert_eq!(builder.placed_lookup.len(), owners_before - (PLANNED - LOWERED), "every discarded placement withdrew its own spatial owner");
    assert_eq!(builder.stage, FillJobStage::Complete, "a plan that already holds what was asked for stops there");

    builder.set_requested_count(RAISED);
    drive_until_settled(&mut builder, 4_000_000);
    assert_eq!(builder.sequence.len(), RAISED, "raising after a discard keeps planning instead of stalling");
}

/// ⚖️ LAW: every candidate the planner builds a pose for reaches the tried ring, and the verdict it
/// carries is the one the pipeline actually reached — a collision never reads as a plain rejection,
/// and an accepted placement never stays "testing".
#[test]
fn every_tried_candidate_reaches_the_ring_with_its_own_verdict() {
    let mut builder = drive_nakagin_scale_fill();
    let mut sequence = 0;
    for _ in 0..200_000 {
        if builder.tested_count as usize > FILL_TRIED_RING && builder.sequence.len() >= 2 {
            break;
        }
        if builder.stage == FillJobStage::Complete {
            break;
        }
        let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
        let outcome = builder.step(&mut context);
        assert!(!faulted(outcome));
    }
    let entries: Vec<&FillTriedCandidate> = builder.preview.tried.iter().flatten().collect();
    assert_eq!(entries.len(), (builder.tested_count as usize).min(FILL_TRIED_RING), "the ring holds every constructed candidate up to its own width");
    assert!(builder.tested_count as usize > FILL_TRIED_RING, "this run constructed more candidates than the ring is wide, so the ring actually wrapped");
    assert!(entries.iter().all(|entry| entry.sequence > 0 && entry.sequence <= builder.preview.sequence), "each entry carries the preview sequence it was published under");
    for entry in &entries {
        match entry.verdict {
            FillCandidateVerdict::Collision => assert_eq!(entry.reason.as_deref(), Some("solid-overlap"), "a collision names the overlap that caused it"),
            FillCandidateVerdict::Rejected => assert!(entry.reason.as_deref().is_some_and(|reason| reason != "solid-overlap"), "a plain rejection names its own reason"),
            FillCandidateVerdict::Free | FillCandidateVerdict::Accepted => assert_eq!(entry.reason, None, "a clean candidate carries no refusal reason"),
            FillCandidateVerdict::Testing => assert_eq!(Some(entry.ghost.clone()), builder.preview.candidate_ghost, "only the live ghost is still under test"),
        }
        assert!(!entry.ghost.mesh_url.is_empty() && !entry.ghost.target_vortex_full_id.is_empty());
    }
    let accepted = entries.iter().filter(|entry| entry.verdict == FillCandidateVerdict::Accepted).count();
    assert!(accepted > 0, "a run that placed objects shows them accepted in the ring");
    assert!(accepted <= builder.sequence.len());
    assert!(builder.collisions <= builder.rejected_count, "every collision is also counted as a refusal");
}

/// ⚖️ LAW: the tried ring is exactly [`FILL_TRIED_RING`] wide on the wire, the verdict vocabulary is
/// the declared one, and an arbitrarily large ask still encodes — the count is unbounded now.
#[test]
fn retained_preview_json_tried_ring_and_unbounded_request_match_the_language_neutral_law() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("language-neutral law fixture");
    let ring = law["boundaryLaws"]["triedRing"].as_array().expect("tried ring laws");
    assert_eq!((ring[0]["items"].as_u64(), ring[0]["admitted"].as_bool()), (Some(FILL_TRIED_RING as u64), Some(true)));
    assert_eq!((ring[1]["items"].as_u64(), ring[1]["admitted"].as_bool()), (Some(FILL_TRIED_RING as u64 + 1), Some(false)));
    let mut oversized = serde_json::to_value(fixture_preview().0).expect("fixture value");
    oversized["tried"].as_array_mut().expect("tried ring").push(serde_json::Value::Null);
    assert!(serde_json::from_value::<FillBuildPreview>(oversized).is_err(), "a thirteenth ring slot is refused by the schema itself");

    let (preview, color, english, _, _, _) = fixture_preview();
    assert_eq!(preview.tried.iter().flatten().count(), FILL_TRIED_RING, "the law fixture publishes a full ring");
    let declared: Vec<&str> = law["boundaryLaws"]["verdicts"].as_array().expect("verdict laws").iter().map(|value| value.as_str().expect("verdict")).collect();
    for verdict in [FillCandidateVerdict::Testing, FillCandidateVerdict::Free, FillCandidateVerdict::Collision, FillCandidateVerdict::Rejected, FillCandidateVerdict::Accepted] {
        assert!(declared.contains(&verdict.wire()));
        let mut owned = preview.clone();
        owned.verdict = verdict;
        assert_preview_ready_matches_oracle(owned, &color, &english);
    }

    for boundary in law["boundaryLaws"]["requestedCount"].as_array().expect("requested count laws") {
        let requested = boundary["value"].as_u64().expect("requested count");
        assert_eq!(boundary["admitted"].as_bool(), Some(true), "no requested count is refused any more");
        let Ok(requested) = usize::try_from(requested) else { continue };
        let mut owned = preview.clone();
        owned.requested_count = requested;
        assert_preview_ready_matches_oracle(owned, &color, &english);
    }
}

//#region ⏯️FillRunJob
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use semio_framework_tool_run::{ToolRunId, ToolRunTraceCursor, ToolRunTraceStore, TOOL_RUN_TRACE_PAGE_BYTES_MAX};

const FILL_RUN_FIXTURE: &str = include_str!("../../🧫️fixtures/🎞️fill-run.json");
const FILL_RUN_BOX_SCALE: f32 = 4.0;

fn fill_run_identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 7, run: 1 }, [3; 32])
}

fn never() -> Option<u64> {
    Some(0)
}

/// 🏙️ A shipped example document through the app's own scene bridge, every mesh identity backed by the
/// app's scaled box fallback; answers the roots, the sorted mesh lane and each mesh's raw positions.
fn example_fill_roots(document: &str, seed: u32) -> (FillPreparationRoots, Vec<String>, Vec<f32>) {
    let text = match document {
        "nakagin" => crate::standards::v1::subsets::any::schema::snapshot::text::PUZZLE3D_NAKAGIN_EXAMPLE_TEXT,
        "concrete-forest" => crate::standards::v1::subsets::any::schema::snapshot::text::PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT,
        other => panic!("unknown example document {other}"),
    };
    let snapshot = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text).expect("example parses");
    let envelope = crate::editor::puzzle3d::scene_from_snapshot(&snapshot, Default::default(), "fill");
    let mut scene: SceneConfig = dsl::FromValue::from_value(crate::editor::puzzle3d::scene_config_value(&envelope)).expect("scene config decodes");
    scene.seed = seed;
    let fallback = semio_framework_plugin::mesh_from_kind(crate::editor::puzzle3d::PUZZLE3D_FALLBACK_MESH_KIND);
    let positions: Vec<f32> = fallback.positions.iter().map(|value| value * FILL_RUN_BOX_SCALE).collect();
    let body = collision_body_from_buffers(&positions, &fallback.indices).expect("fallback body");
    let mut lane = crate::editor::puzzle3d::collect_mesh_urls(&envelope.fixture);
    lane.push(crate::editor::puzzle3d::PUZZLE3D_FALLBACK_MESH_KIND.to_string());
    lane.sort();
    lane.dedup();
    let meshes = lane.iter().map(|url| (url.clone(), body.clone())).collect();
    (FillPreparationRoots::new(Arc::new(scene), Arc::new(meshes)), lane, positions)
}

fn fill_run_job(roots: FillPreparationRoots, lane: Vec<String>, seed: u64, requested: usize) -> FillRunJob {
    FillRunJob::new(FillBuilder::begin_preparation(roots, Operation::new(OperationId(71), RevisionId(1), Generation(1), seed), requested), fill_run_identity(), lane)
}

fn close_payload(mut payload: RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}

/// 🚦️ What one run job turn handed its driver, with every retained page returned to its ledger.
#[derive(Debug)]
enum FillRunTurn {
    Tick(ToolRunTick),
    Checkpoint(Vec<u8>),
    Complete,
    Yield,
}

fn settle_fill_run_outcome(outcome: StepOutcome) -> FillRunTurn {
    match outcome {
        StepOutcome::PreviewReady(payload) => {
            let page = payload.single_page().expect("a tick is one payload page").to_vec();
            close_payload(payload);
            assert!(page.len() <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES && page.len() <= TOOL_RUN_TRACE_PAGE_BYTES_MAX);
            FillRunTurn::Tick(ToolRunTick::decode(&page).expect("tick decodes"))
        }
        StepOutcome::CheckpointReady(checkpoint) => {
            let bytes = checkpoint.state.single_page().expect("checkpoint page").to_vec();
            close_payload(checkpoint.state);
            FillRunTurn::Checkpoint(bytes)
        }
        StepOutcome::Complete(candidate) => {
            close_payload(candidate.state);
            close_payload(candidate.output);
            FillRunTurn::Complete
        }
        StepOutcome::Yield => FillRunTurn::Yield,
        other => {
            let described = format!("{other:?}");
            assert!(faulted(other), "unexpected run job outcome {described}");
            panic!("the fill run job faulted: {described}");
        }
    }
}

fn fill_run_turn(job: &mut FillRunJob, fuel: u64, sequence: &mut u64) -> FillRunTurn {
    let operation = job.operation();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, u64::MAX), root_cancel_token(), never, sequence);
    settle_fill_run_outcome(job.step(&mut context))
}

/// 🪞️ The ledger side of a run as the contract folds it: provisional ops and entities, the resident
/// trace, and every verdict in the order the ticks carried it.
struct FillRunMirror {
    ops: Vec<Vec<u8>>,
    entities: Vec<u64>,
    trace: ToolRunTraceStore,
    verdicts: Vec<(u64, ToolRunVerdict, u16)>,
    steps: Vec<ToolRunStep>,
    retractions: Vec<u32>,
    checkpoints: Vec<Vec<u8>>,
    last_sequence: Option<u64>,
    progress: Option<ToolRunProgress>,
    ticks: usize,
}

impl FillRunMirror {
    fn new() -> Self {
        Self { ops: Vec::new(), entities: Vec::new(), trace: ToolRunTraceStore::new(fill_run_identity()), verdicts: Vec::new(), steps: Vec::new(), retractions: Vec::new(), checkpoints: Vec::new(), last_sequence: None, progress: None, ticks: 0 }
    }

    fn apply(&mut self, tick: ToolRunTick) {
        assert!(self.last_sequence.is_none_or(|last| tick.sequence > last), "tick sequences are monotone");
        self.last_sequence = Some(tick.sequence);
        self.ticks += 1;
        if let Some(retract_to) = tick.retract_to {
            self.retractions.push(retract_to);
            self.ops.truncate(retract_to as usize);
            self.entities.truncate((retract_to / FILL_RUN_OPS_PER_PLACEMENT) as usize);
        }
        self.ops.extend(tick.append_ops);
        self.entities.extend(tick.append_entities);
        assert_eq!(self.ops.len(), self.entities.len() * FILL_RUN_OPS_PER_PLACEMENT as usize, "every placement carries exactly its two ops and one entity");
        for page in &tick.trace {
            for op in &page.ops {
                if let ToolRunTraceOp::Upsert { key, verdict, reason, .. } = op {
                    if *verdict != ToolRunVerdict::Testing {
                        self.verdicts.push((*key, *verdict, *reason));
                    }
                }
            }
            self.trace.apply_page(page).expect("pages of the one run and generation");
        }
        self.steps.extend(tick.steps);
        if tick.progress.is_some() {
            self.progress = tick.progress;
        }
    }

    fn drive(&mut self, job: &mut FillRunJob, fuel: u64, turns: usize) -> usize {
        let mut sequence = 0;
        for turn in 0..turns {
            match fill_run_turn(job, fuel, &mut sequence) {
                FillRunTurn::Tick(tick) => self.apply(tick),
                FillRunTurn::Checkpoint(bytes) => self.checkpoints.push(bytes),
                FillRunTurn::Complete => return turn + 1,
                FillRunTurn::Yield => {}
            }
        }
        panic!("the fill run job did not complete in {turns} turns at stage {:?}", job.builder().stage);
    }

    fn verdict_words(&self) -> Vec<String> {
        self.verdicts.iter().map(|(_, verdict, reason)| format!("{}:{}", verdict.as_str(), FillRunReason::from_code(*reason).map_or("framework", FillRunReason::id))).collect()
    }
}

fn fill_run_summary(job: &FillRunJob, mirror: &FillRunMirror, prefix: usize) -> serde_json::Value {
    let [tested, locked, collisions, rejected] = job.counters();
    let stall = mirror.steps.iter().rev().find(|step| step.kind == ToolRunStepKind::Warning).and_then(|step| FillRunReason::from_code(step.reason)).map(FillRunReason::id);
    serde_json::json!({
        "verdictPrefix": mirror.verdict_words().into_iter().take(prefix).collect::<Vec<_>>(),
        "tested": tested,
        "locked": locked,
        "collisions": collisions,
        "rejected": rejected,
        "appendOps": mirror.ops.len(),
        "appendEntities": mirror.entities.len(),
        "checkpoints": mirror.checkpoints.len(),
        "stall": stall,
    })
}

/// ⚖️ LAW (language-neutral fixture `🎞️fill-run.json`): a seeded shipped document and a requested count
/// produce exactly the declared verdict prefix, counters, op and entity counts — and the laws that hold
/// for every run: two ops and one entity per placement, one `success` per placement, one `danger` per
/// collision, one `warning` per rule refusal, ops alternating `create_object` / `connect_vortices` whose
/// entity is the created object's own id digest.
#[test]
fn fill_run_job_matches_the_language_neutral_fill_run_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run fixture");
    assert_eq!(fixture["laws"]["opsPerPlacement"].as_u64(), Some(u64::from(FILL_RUN_OPS_PER_PLACEMENT)));
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧬️schema/🔣️.json")).expect("schema");
    let vocabulary = &schema["$defs"]["Puzzle3dFillRun"]["x-semio-toolRun"];
    assert_eq!(vocabulary["stages"].as_array().map(|stages| stages.iter().filter_map(serde_json::Value::as_str).collect::<Vec<_>>()), Some(FillRunStage::ALL.iter().map(|stage| stage.id()).collect()));
    assert_eq!(vocabulary["counters"].as_array().map(|counters| counters.iter().filter_map(serde_json::Value::as_str).collect::<Vec<_>>()), Some(FillRunCounter::ALL.iter().map(|counter| counter.id()).collect()));
    let reasons: Vec<(u64, String, String)> = vocabulary["reasons"].as_array().expect("reasons").iter().map(|reason| (reason["code"].as_u64().expect("code"), reason["id"].as_str().expect("id").into(), reason["verdict"].as_str().expect("verdict").into())).collect();
    assert_eq!(reasons, FillRunReason::ALL.iter().map(|reason| (u64::from(reason.code()), reason.id().to_string(), reason.verdict().as_str().to_string())).collect::<Vec<_>>());
    assert_eq!(vocabulary["checkpoint"]["bytes"].as_u64(), Some(FillRunCheckpoint::BYTES as u64));
    let mut disagreements = Vec::new();
    for case in fixture["cases"].as_array().expect("cases") {
        let document = case["document"].as_str().expect("document");
        let seed = case["seed"].as_u64().expect("seed");
        let requested = case["requested"].as_u64().expect("requested") as usize;
        let (roots, lane, _) = example_fill_roots(document, seed as u32);
        let mut job = fill_run_job(roots, lane, seed, requested);
        let mut mirror = FillRunMirror::new();
        mirror.drive(&mut job, u64::MAX, 1_000_000);
        let expected = &case["expected"];
        let prefix = expected["verdictPrefix"].as_array().map_or(0, Vec::len);
        let actual = fill_run_summary(&job, &mirror, prefix);
        if &actual != expected {
            disagreements.push(format!("{document} seed {seed} requested {requested}: actual {actual}"));
        }
        let [tested, locked, collisions, rejected] = job.counters();
        let count = |wanted: ToolRunVerdict| mirror.verdicts.iter().filter(|(_, verdict, _)| *verdict == wanted).count() as u64;
        assert_eq!((count(ToolRunVerdict::Success), count(ToolRunVerdict::Danger), count(ToolRunVerdict::Warning)), (locked, collisions, rejected));
        assert_eq!(mirror.verdicts.len() as u64, tested, "every constructed candidate reached exactly one verdict");
        assert_eq!(mirror.trace.len() as u64, tested, "every tested candidate stays resident");
        for (index, pair) in mirror.ops.chunks(2).enumerate() {
            let Ok(Puzzle3dMutation::CreateObject(create)) = crate::standards::v1::subsets::any::schema::mutations::binary::decode_op(&pair[0]) else { panic!("op {} is create_object", 2 * index) };
            let Ok(Puzzle3dMutation::ConnectVortices(connect)) = crate::standards::v1::subsets::any::schema::mutations::binary::decode_op(&pair[1]) else { panic!("op {} is connect_vortices", 2 * index + 1) };
            assert_eq!(mirror.entities[index], fill_run_entity(&create.object.id));
            assert_eq!(connect.attracted.split(':').next(), Some(create.object.id.as_str()), "the attraction docks the created object");
            assert_eq!(create.object.id, job.builder().appended_objects[index].id);
        }
        let progress = mirror.progress.as_ref().expect("progress");
        assert_eq!((progress.state, progress.completed, progress.total), (ToolRunState::Complete, locked, Some(requested as u64)));
        assert_eq!(progress.counters.iter().map(|counter| counter.value).collect::<Vec<_>>(), vec![tested, locked, collisions, rejected]);
    }
    assert!(disagreements.is_empty(), "the fill run fixture disagrees:\n{}", disagreements.join("\n"));
}

fn parry_hull(pose: &Pose3d, positions: &[f32]) -> parry3d::shape::ConvexPolyhedron {
    let points: Vec<parry3d::math::Point<f32>> = positions
        .chunks(3)
        .map(|vertex| {
            let world = pose.transform_point(&crate::editor::puzzle3d::precompute::geometry::Point3d::new(vertex[0], vertex[1], vertex[2]));
            parry3d::math::Point::new(world.x(), world.y(), world.z())
        })
        .collect();
    parry3d::shape::ConvexPolyhedron::from_convex_hull(&points).expect("box hull")
}

/// 📦️ Overlap volume of two world-space hulls by `parry3d` point containment on a regular grid over
/// their bounding-box intersection, plus that intersection's volume.
fn parry_overlap(a: &parry3d::shape::ConvexPolyhedron, b: &parry3d::shape::ConvexPolyhedron, cells: usize) -> (f64, f64) {
    use parry3d::query::PointQuery;
    use parry3d::shape::Shape;
    let (left, right) = (a.compute_local_aabb(), b.compute_local_aabb());
    let min = left.mins.sup(&right.mins);
    let max = left.maxs.inf(&right.maxs);
    let size = max - min;
    if size.iter().any(|extent| *extent <= 0.0) {
        return (0.0, 0.0);
    }
    let box_volume = f64::from(size.x) * f64::from(size.y) * f64::from(size.z);
    let mut inside = 0usize;
    for x in 0..cells {
        for y in 0..cells {
            for z in 0..cells {
                let at = |index: usize, axis: usize| min[axis] + size[axis] * ((index as f32 + 0.5) / cells as f32);
                let point = parry3d::math::Point::new(at(x, 0), at(y, 1), at(z, 2));
                inside += usize::from(a.contains_local_point(&point) && b.contains_local_point(&point));
            }
        }
    }
    (box_volume * inside as f64 / (cells * cells * cells) as f64, box_volume)
}

/// ⚖️ ORACLE (`parry3d`): every candidate the run marked `danger` (solid overlap) or `success` (fits) is
/// recomputed against every body placed before it — the document's own bodies plus the run's earlier
/// placements, the docking host excluded — as exact convex hulls whose pairwise overlap volume
/// `parry3d` measures by point containment. The planner collides when one pair overlaps beyond the
/// scene's overlap budget, estimated from `COLLISION_SAMPLES` samples of the pair's bounding-box
/// intersection; a verdict is decisive when the true overlap is at least twice (collision) or at most
/// half (fit) the budget and the sample estimate cannot plausibly land across it, or when parry
/// separates the hulls outright. Every decisive verdict must agree.
#[test]
fn fill_run_job_collision_verdicts_agree_with_the_parry3d_oracle() {
    const COLLISION_SAMPLES: f64 = 512.0;
    const DECISIVE_HITS: f64 = 16.0;
    const GRID_CELLS: usize = 16;
    let fixture: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run fixture");
    let oracle = &fixture["laws"]["parryOracle"];
    let seed = oracle["seed"].as_u64().expect("seed");
    let (roots, lane, positions) = example_fill_roots(oracle["document"].as_str().expect("document"), seed as u32);
    let budget = roots.scene.overlap_budget;
    let mut job = fill_run_job(roots, lane, seed, oracle["requested"].as_u64().expect("requested") as usize);
    let mut mirror = FillRunMirror::new();
    mirror.drive(&mut job, u64::MAX, 1_000_000);
    let placed = &job.builder().placed;
    let base = placed.len() - job.builder().sequence.len();
    let identity = parry3d::math::Isometry::identity();
    let hulls: Vec<parry3d::shape::ConvexPolyhedron> = placed.iter().map(|entry| parry_hull(&entry.world, &positions)).collect();
    let (mut decisive, mut ambiguous, mut collisions, mut fits) = (0usize, 0usize, 0usize, 0usize);
    let mut disagreements = Vec::new();
    for record in job.verdicts.iter().filter(|record| matches!(record.reason, FillRunReason::SolidOverlap | FillRunReason::Fits)) {
        let candidate = parry_hull(&pose_isometry(record.origin, record.orientation, &None), &positions);
        let (mut collides, mut uncertain) = (false, false);
        for (_, other) in placed[..base + record.placements_before].iter().zip(&hulls).filter(|(entry, _)| Some(&entry.object_id) != record.host.as_ref()) {
            if !parry3d::bounding_volume::BoundingVolume::intersects(&parry3d::shape::Shape::compute_local_aabb(&candidate), &parry3d::shape::Shape::compute_local_aabb(other)) || parry3d::query::distance(&identity, &candidate, &identity, other).expect("convex distance") > 0.0 {
                continue;
            }
            let (volume, box_volume) = parry_overlap(&candidate, other, GRID_CELLS);
            let expected_hits = COLLISION_SAMPLES * volume / box_volume.max(f64::MIN_POSITIVE);
            let threshold_hits = COLLISION_SAMPLES * budget / box_volume.max(f64::MIN_POSITIVE);
            if volume >= 2.0 * budget && expected_hits >= DECISIVE_HITS {
                collides = true;
            } else if !(volume <= 0.5 * budget && threshold_hits >= DECISIVE_HITS) {
                uncertain = true;
            }
        }
        let ours = record.reason == FillRunReason::SolidOverlap;
        collisions += usize::from(ours);
        fits += usize::from(!ours);
        if !collides && uncertain {
            ambiguous += 1;
            continue;
        }
        decisive += 1;
        if ours != collides {
            disagreements.push(format!("candidate {} ours={:?} parry collides={collides}", record.key, record.reason));
        }
    }
    assert!(disagreements.is_empty(), "{} of {decisive} decisive verdicts disagree with parry3d:\n{}", disagreements.len(), disagreements.join("\n"));
    assert!(collisions > 0 && fits > 0, "the oracle run must decide both collisions ({collisions}) and fits ({fits})");
    assert!(ambiguous * 10 <= decisive, "at most one in ten verdicts may fall inside the sampling band: {ambiguous} of {decisive}");
}

/// ⚖️ LAW: a run of at least 5 000 tested candidates delivers every trace record — the ledger's resident
/// store and a renderer that only ever reads byte-budgeted deltas through its echoed cursor hold exactly
/// the key set the job reported, with the verdict the job reported last.
#[test]
fn fill_run_job_delivers_every_trace_record_of_a_5000_candidate_run() {
    let fixture: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run fixture");
    let law = &fixture["laws"]["delivery"];
    let minimum = law["candidates"].as_u64().expect("candidates");
    let seed = law["seed"].as_u64().expect("seed");
    let (roots, lane, _) = example_fill_roots(law["document"].as_str().expect("document"), seed as u32);
    let mut job = fill_run_job(roots, lane, seed, law["requested"].as_u64().expect("requested") as usize);
    let mut ledger = FillRunMirror::new();
    let mut renderer = ToolRunTraceStore::new(fill_run_identity());
    let mut cursor: Option<ToolRunTraceCursor> = None;
    let mut expected: HashMap<u64, ToolRunVerdict> = HashMap::new();
    let mut sequence = 0;
    let budget = law["deltaBudgetBytes"].as_u64().expect("delta budget") as usize;
    for _ in 0..10_000_000 {
        match fill_run_turn(&mut job, 64, &mut sequence) {
            FillRunTurn::Tick(tick) => {
                for op in tick.trace.iter().flat_map(|page| &page.ops) {
                    match op {
                        ToolRunTraceOp::Upsert { key, verdict, .. } => {
                            expected.insert(*key, *verdict);
                        }
                        ToolRunTraceOp::Retire { key } => {
                            expected.remove(key);
                        }
                        ToolRunTraceOp::Clear => expected.clear(),
                    }
                }
                ledger.apply(tick);
                let delta = ledger.trace.delta_after(cursor, budget);
                if delta.clear {
                    renderer = ToolRunTraceStore::new(fill_run_identity());
                }
                for page in &delta.pages {
                    renderer.apply_ops(&page.ops);
                }
                cursor = Some(ToolRunTraceCursor { run: delta.identity.id.run, generation: delta.identity.generation, page: delta.next });
            }
            FillRunTurn::Checkpoint(_) | FillRunTurn::Yield => {}
            FillRunTurn::Complete => break,
        }
        if job.counters()[0] >= minimum {
            break;
        }
    }
    loop {
        let delta = ledger.trace.delta_after(cursor, budget);
        if delta.clear {
            renderer = ToolRunTraceStore::new(fill_run_identity());
        }
        let caught_up = delta.pages.is_empty();
        for page in &delta.pages {
            renderer.apply_ops(&page.ops);
        }
        cursor = Some(ToolRunTraceCursor { run: delta.identity.id.run, generation: delta.identity.generation, page: delta.next });
        if caught_up {
            break;
        }
    }
    assert!(job.counters()[0] >= minimum, "the delivery law needs at least {minimum} tested candidates, the run reached {:?}", job.counters());
    let keys = |store: &ToolRunTraceStore| store.records().map(|(key, record)| (key, record.verdict)).collect::<HashMap<_, _>>();
    assert_eq!(expected.len() as u64, job.counters()[0]);
    assert_eq!(keys(&ledger.trace), expected, "the ledger holds every reported record");
    assert_eq!(keys(&renderer), expected, "a cursor-driven renderer holds every reported record");
}

/// ⏱️ Turn clock of the interactive law: records where the wall slice expired in the first cold run
/// and replays exactly those expiries in the later runs, so every run takes the same bounded turns.
#[derive(Default)]
struct FillRunTurnClock {
    reads: u64,
    deadline: u64,
    first_expired: Option<u64>,
    replay_expiry: Option<u64>,
}

thread_local! {
    static FILL_RUN_TURN_CLOCK: std::cell::RefCell<FillRunTurnClock> = std::cell::RefCell::new(FillRunTurnClock::default());
}

fn fill_run_recording_clock() -> Option<u64> {
    let now = semio_framework_job::default_now_us();
    FILL_RUN_TURN_CLOCK.with(|clock| {
        let mut clock = clock.borrow_mut();
        clock.reads += 1;
        if clock.first_expired.is_none() && now.is_none_or(|now| now >= clock.deadline) {
            clock.first_expired = Some(clock.reads);
        }
    });
    now
}

fn fill_run_replaying_clock() -> Option<u64> {
    FILL_RUN_TURN_CLOCK.with(|clock| {
        let mut clock = clock.borrow_mut();
        clock.reads += 1;
        Some(if clock.replay_expiry.is_some_and(|expiry| clock.reads >= expiry) { u64::MAX } else { 0 })
    })
}

/// ⏱️ LAW (red→green row 1, part a): on the shipped Nakagin document every `drive_step` of the fill run
/// job under the interactive lane's wall slice stays below the artifact's 2 ms budget over at least 771
/// turns. Like the artifact's other interactive laws it takes each turn's best of several cold runs;
/// the first run slices by the real clock and the others replay its slice boundaries exactly.
#[test]
fn fill_run_job_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let fixture: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run fixture");
    let law = &fixture["laws"]["interactive"];
    let minimum_turns = law["turns"].as_u64().expect("turns") as usize;
    let budget = Duration::from_micros(law["budgetUs"].as_u64().expect("budget"));
    let runs = law["coldRuns"].as_u64().expect("cold runs") as usize;
    let seed = law["seed"].as_u64().expect("seed");
    let mut expiries: Vec<Option<u64>> = Vec::new();
    let mut best: Vec<Duration> = Vec::new();
    let mut counters = [0; 4];
    for run in 0..runs {
        let (roots, lane, _) = example_fill_roots(law["document"].as_str().expect("document"), seed as u32);
        let mut job = fill_run_job(roots, lane, seed, law["requested"].as_u64().expect("requested") as usize);
        let operation = job.operation();
        let mut sequence = 0;
        let mut verdict = None;
        for turn in 0.. {
            let recording = run == 0;
            assert!(recording || turn < expiries.len(), "cold run {run} took more turns than the recorded run");
            let (step_budget, clock): (StepBudget, fn() -> Option<u64>) = if recording {
                let start = semio_framework_job::default_now_us().expect("clock");
                FILL_RUN_TURN_CLOCK.with(|clock| *clock.borrow_mut() = FillRunTurnClock { deadline: start + semio_framework_job::INTERACTIVE_LANE_WALL_US, ..FillRunTurnClock::default() });
                (StepBudget::new(semio_framework_job::INTERACTIVE_LANE_FUEL, start + semio_framework_job::INTERACTIVE_LANE_WALL_US), fill_run_recording_clock)
            } else {
                FILL_RUN_TURN_CLOCK.with(|clock| *clock.borrow_mut() = FillRunTurnClock { replay_expiry: expiries[turn], ..FillRunTurnClock::default() });
                (StepBudget::new(semio_framework_job::INTERACTIVE_LANE_FUEL, 1), fill_run_replaying_clock)
            };
            let started = Instant::now();
            let outcome = semio_framework_job::drive_step(&mut job, "puzzle3d-fill-run", operation.operation, operation.generation, semio_framework_job::InteractiveStage::InteractiveStep, step_budget, root_cancel_token(), clock, &mut sequence, &mut verdict);
            let elapsed = started.elapsed();
            if recording {
                expiries.push(FILL_RUN_TURN_CLOCK.with(|clock| clock.borrow().first_expired));
                best.push(elapsed);
            } else {
                best[turn] = best[turn].min(elapsed);
            }
            if matches!(settle_fill_run_outcome(outcome), FillRunTurn::Complete) {
                assert!(recording || turn + 1 == expiries.len(), "cold run {run} completed after {} turns, the recorded run after {}", turn + 1, expiries.len());
                break;
            }
        }
        counters = job.counters();
    }
    let (turn, worst) = best.iter().enumerate().max_by_key(|(_, elapsed)| **elapsed).map_or((0, Duration::ZERO), |(turn, elapsed)| (turn + 1, *elapsed));
    assert!(best.len() >= minimum_turns, "the law measures at least {minimum_turns} turns, the run took {}", best.len());
    assert!(counters[1] > 0 && counters[0] > counters[1], "the measured run tested and placed objects: {counters:?}");
    assert!(worst < budget, "fill run job worst drive_step {worst:?} at turn {turn} of {} exceeds {budget:?}", best.len());
}

/// ⚖️ LAW: with one unit of fuel a run job step reaches exactly one candidate verdict — the tick carries
/// that candidate's `testing` and final upsert under one key — except the final tick of a completed run.
#[test]
fn fill_run_job_step_with_one_unit_of_fuel_reaches_exactly_one_candidate_verdict() {
    let mut job = FillRunJob::new(FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(73), RevisionId(1), Generation(1), 43), 24), fill_run_identity(), vec![NAKAGIN_MESH_URL.to_string()]);
    let mut sequence = 0;
    let (mut ticks, mut verdicts) = (0usize, 0u64);
    for _ in 0..1_000_000 {
        match fill_run_turn(&mut job, 1, &mut sequence) {
            FillRunTurn::Tick(tick) => {
                ticks += 1;
                let ops: Vec<&ToolRunTraceOp> = tick.trace.iter().flat_map(|page| &page.ops).collect();
                let finals: Vec<u64> = ops.iter().filter_map(|op| match op {
                    ToolRunTraceOp::Upsert { key, verdict, .. } if *verdict != ToolRunVerdict::Testing => Some(*key),
                    _ => None,
                }).collect();
                let testing: Vec<u64> = ops.iter().filter_map(|op| match op {
                    ToolRunTraceOp::Upsert { key, verdict: ToolRunVerdict::Testing, .. } => Some(*key),
                    _ => None,
                }).collect();
                let complete = tick.progress.as_ref().is_some_and(|progress| progress.state == ToolRunState::Complete);
                if complete && finals.is_empty() {
                    continue;
                }
                assert_eq!(finals.len(), 1, "one fuel unit is one candidate verdict, tick {} carried {finals:?}", tick.sequence);
                assert!(testing.is_empty() || testing == finals, "the verdict closes the candidate this step tested: {testing:?} vs {finals:?}");
                verdicts += 1;
            }
            FillRunTurn::Checkpoint(_) | FillRunTurn::Yield => {}
            FillRunTurn::Complete => break,
        }
    }
    assert_eq!(verdicts, job.counters()[0], "every tested candidate took exactly one fuel-one step");
    assert!(ticks as u64 >= verdicts && job.counters()[1] == 24);
}

fn nakagin_scale_run(requested: usize) -> (FillRunJob, FillRunMirror) {
    let mut job = FillRunJob::new(FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(79), RevisionId(1), Generation(1), 43), requested), fill_run_identity(), vec![NAKAGIN_MESH_URL.to_string()]);
    let mut mirror = FillRunMirror::new();
    mirror.drive(&mut job, u64::MAX, 1_000_000);
    (job, mirror)
}

/// ⚖️ LAW: a completed run resumed from its own checkpoint with a raised count continues the same
/// deterministic sequence — ops, entities and verdicts equal one run to the raised count — and resumed
/// with a lowered count retracts the provisional tail to exactly the lowered count; a foreign or
/// malformed checkpoint is refused.
#[test]
fn fill_run_job_resume_raise_continues_the_sequence_and_lower_retracts_the_tail() {
    const SHORT: usize = 30;
    const LONG: usize = 45;
    const LOWERED: usize = 12;
    let (mut raised, mut raised_mirror) = nakagin_scale_run(SHORT);
    assert_eq!(raised_mirror.checkpoints.len(), SHORT, "one checkpoint per placement");
    let checkpoint = raised_mirror.checkpoints.last().cloned().expect("checkpoint");
    assert_eq!(FillRunCheckpoint::decode(&checkpoint), Some(raised.checkpoint()), "the last checkpoint is where the completed run stands");
    let short_ops = raised_mirror.ops.clone();
    raised.resume(&checkpoint, LONG).expect("resume raise");
    raised_mirror.drive(&mut raised, u64::MAX, 1_000_000);
    let (long, long_mirror) = nakagin_scale_run(LONG);
    assert_eq!(raised_mirror.ops, long_mirror.ops, "the raised run appends the same ops as one long run");
    assert_eq!(raised_mirror.entities, long_mirror.entities);
    assert_eq!(raised_mirror.verdicts, long_mirror.verdicts, "and reaches the same verdicts in the same order");
    assert_eq!(&long_mirror.ops[..short_ops.len()], short_ops.as_slice());
    assert_eq!(raised.counters(), long.counters());

    let (mut lowered, mut lowered_mirror) = nakagin_scale_run(SHORT);
    let checkpoint = lowered_mirror.checkpoints.last().cloned().expect("checkpoint");
    let before = lowered_mirror.ops.clone();
    lowered.resume(&checkpoint, LOWERED).expect("resume lower");
    lowered_mirror.drive(&mut lowered, u64::MAX, 1_000_000);
    assert_eq!(lowered_mirror.retractions.iter().min().copied(), Some(LOWERED as u32 * FILL_RUN_OPS_PER_PLACEMENT), "the lowered run retracts to exactly the lowered count");
    assert_eq!(lowered_mirror.ops.as_slice(), &before[..LOWERED * 2]);
    assert_eq!((lowered_mirror.entities.len(), lowered.counters()[1], lowered.builder().sequence.len()), (LOWERED, LOWERED as u64, LOWERED));
    let success = lowered_mirror.trace.records().filter(|(_, record)| record.verdict == ToolRunVerdict::Success).count();
    assert_eq!(success, LOWERED, "every retracted placement's success record was retired");

    let mut foreign = FillRunCheckpoint::decode(&checkpoint).expect("checkpoint");
    foreign.next_key += 1_000;
    assert_eq!(lowered.resume(&foreign.encode(), SHORT), Err(FillRunResumeError::Foreign));
    assert_eq!(lowered.resume(&checkpoint[..8], SHORT), Err(FillRunResumeError::Malformed));
}

fn drive_revalidation(job: &mut FillRevalidateJob, operation: Operation) -> Vec<ToolRunTick> {
    let mut ticks = Vec::new();
    let mut sequence = 0;
    for _ in 0..1_000_000 {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX), root_cancel_token(), never, &mut sequence);
        match settle_fill_run_outcome(job.step(&mut context)) {
            FillRunTurn::Tick(tick) => ticks.push(tick),
            FillRunTurn::Complete => return ticks,
            FillRunTurn::Checkpoint(_) | FillRunTurn::Yield => {}
        }
    }
    panic!("revalidation did not complete");
}

/// ⚖️ LAW: revalidating provisional placements against an unchanged head keeps every one; against a head
/// that gained a body where one placement stands, that placement turns `danger` with the framework
/// conflict reason, the final tick retracts to it and re-appends every later survivor's exact ops and
/// entity, and one `danger` conflict step counts the conflicts.
#[test]
fn fill_revalidate_job_retracts_conflicting_placements_and_reappends_survivors() {
    const PLACED: usize = 8;
    const INTRUDED: usize = 3;
    let (job, mirror) = nakagin_scale_run(PLACED);
    let placements = job.provisional_placements();
    assert_eq!(placements.len(), PLACED);
    let operation = Operation::new(OperationId(83), RevisionId(2), Generation(1), 43);
    let head = nakagin_scale_roots();
    let mut clean = FillRevalidateJob::new(operation, fill_run_identity(), FillPreparationRoots::new(head.scene.clone(), head.meshes.clone()), placements.clone(), 1_000);
    let ticks = drive_revalidation(&mut clean, operation);
    assert!(clean.conflicts().iter().all(|conflict| !conflict) && clean.conflicts().len() == PLACED);
    assert!(ticks.iter().all(|tick| tick.retract_to.is_none() && tick.append_ops.is_empty()));

    let mut scene = (*head.scene).clone();
    let mut intruder = placements[INTRUDED].object.clone();
    intruder.id = "intruder".into();
    intruder.vortices.clear();
    scene.fixture.objects.push(intruder);
    let mut intruded = FillRevalidateJob::new(operation, fill_run_identity(), FillPreparationRoots::new(Arc::new(scene), head.meshes.clone()), placements.clone(), 1_000);
    let ticks = drive_revalidation(&mut intruded, operation);
    let conflicts = intruded.conflicts().to_vec();
    assert!(conflicts[INTRUDED], "the intruded placement conflicts: {conflicts:?}");
    let first = conflicts.iter().position(|conflict| *conflict).expect("a conflict");
    let last = ticks.last().expect("final tick");
    assert_eq!(last.retract_to, Some(first as u32 * FILL_RUN_OPS_PER_PLACEMENT));
    let survivors: Vec<usize> = (first..PLACED).filter(|index| !conflicts[*index]).collect();
    assert_eq!(last.append_ops, survivors.iter().flat_map(|index| mirror.ops[index * 2..index * 2 + 2].to_vec()).collect::<Vec<_>>(), "survivors re-append their exact ops");
    assert_eq!(last.append_entities, survivors.iter().map(|index| mirror.entities[*index]).collect::<Vec<_>>());
    let conflict_count = conflicts.iter().filter(|conflict| **conflict).count() as u64;
    assert!(last.steps.iter().any(|step| step.kind == ToolRunStepKind::Danger && step.reason == TOOL_RUN_REASON_CONFLICT && step.args == vec![ToolRunStepArg::Unsigned(conflict_count)]));
    let danger: Vec<u64> = ticks.iter().flat_map(|tick| tick.trace.iter().flat_map(|page| page.ops.clone())).filter_map(|op| match op {
        ToolRunTraceOp::Upsert { key, verdict: ToolRunVerdict::Danger, reason: TOOL_RUN_REASON_CONFLICT, .. } => Some(key),
        _ => None,
    }).collect();
    assert_eq!(danger, conflicts.iter().zip(&placements).filter(|(conflict, _)| **conflict).map(|(_, placement)| placement.key).collect::<Vec<_>>());
}
//#endregion ⏯️FillRunJob

/// ⚖️ LAW: a rule refusal is a `warning` record, never a collision — a document whose target volume
/// lies away from every open vortex refuses each constructed candidate as `outside-target-volume`,
/// appends nothing and ends with the `no-free-placement` warning step.
#[test]
fn fill_run_job_reports_rule_refusals_as_warnings_and_the_stall_as_a_warning_step() {
    let roots = nakagin_scale_roots();
    let mut scene = (*roots.scene).clone();
    scene.fixture.target_volumes.push(WorldVolumeProps { id: "elsewhere".into(), origin: [1.0e6, 1.0e6, 1.0e6], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None });
    let mut job = FillRunJob::new(FillBuilder::begin_preparation(FillPreparationRoots::new(Arc::new(scene), roots.meshes.clone()), Operation::new(OperationId(89), RevisionId(1), Generation(1), 43), 10), fill_run_identity(), vec![NAKAGIN_MESH_URL.to_string()]);
    let mut mirror = FillRunMirror::new();
    mirror.drive(&mut job, u64::MAX, 1_000_000);
    let [tested, locked, collisions, rejected] = job.counters();
    assert!(tested > 0 && tested == rejected && locked == 0 && collisions == 0, "{:?}", job.counters());
    assert!(mirror.verdicts.iter().all(|(_, verdict, reason)| *verdict == ToolRunVerdict::Warning && *reason == FillRunReason::OutsideTargetVolume.code()));
    assert!(mirror.ops.is_empty() && mirror.entities.is_empty());
    let last = mirror.steps.last().expect("stall step");
    assert_eq!((last.kind, last.reason, last.args.clone()), (ToolRunStepKind::Warning, FillRunReason::NoFreePlacement.code(), vec![ToolRunStepArg::Unsigned(0)]));
    assert_eq!(mirror.progress.as_ref().map(|progress| progress.state), Some(ToolRunState::Complete));
}
