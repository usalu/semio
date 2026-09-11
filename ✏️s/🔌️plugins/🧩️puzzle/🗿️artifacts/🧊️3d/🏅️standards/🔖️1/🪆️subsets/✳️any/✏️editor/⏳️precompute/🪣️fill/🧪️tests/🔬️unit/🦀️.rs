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
    FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(HashMap::new())), Operation::new(OperationId(1), RevisionId(1), Generation(1), 17))
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
    total_count: usize,
    search_count: u64,
    rejected_count: u64,
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
    fill_build_preview: OracleDiagnostic<'a>,
}

fn oracle_ghost(ghost: &BrushPreviewState) -> OracleGhost<'_> {
    OracleGhost { target_vortex_full_id: &ghost.target_vortex_full_id, object_kind_id: &ghost.object_kind_id, source_vortex_index: ghost.source_vortex_index, mesh_url: &ghost.mesh_url, origin: ghost.origin, orientation: ghost.orientation }
}

fn oracle_json_scalar_admits(preview: &FillBuildPreview, color: &str, status_label: &str) -> bool {
    let safe_u64 = |value: u64, minimum: u64| value >= minimum && value <= 9_007_199_254_740_991;
    let safe_usize = |value: usize| value as u128 <= 9_007_199_254_740_991;
    let source_authority = preview.candidate_ghost.as_ref().map_or(true, |ghost| {
        let root_source_vortex_index = ghost.source_vortex_index as u128;
        let candidate_ghost_source_vortex_index = ghost.source_vortex_index as u128;
        root_source_vortex_index <= 9_007_199_254_740_991 && candidate_ghost_source_vortex_index <= 9_007_199_254_740_991
    });
    color.len() <= 128
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
        && safe_usize(preview.total_count)
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
            total_count: preview.total_count,
            search_count: preview.search_count,
            rejected_count: preview.rejected_count,
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
    assert!(text.len() <= 4096, "owned full-wire byte semantics guard the test-only serde oracle");
    text
}

fn drive_preview_json(builder: &mut FillBuilder, color: &str, status_label: &str) -> FillPreviewJsonStep {
    for _ in 0..20_000 {
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
    let numeric_fields = law["diagnosticNumericFields"].as_array().expect("diagnostic numeric laws");
    assert_eq!(numeric_fields.len(), 14);
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
    let u64_fields: [(&str, U64Setter); 7] = [
        ("operation", |preview, value| preview.operation = value),
        ("baseRevision", |preview, value| preview.base_revision = value),
        ("registryGeneration", |preview, value| preview.registry_generation = value),
        ("sequence", |preview, value| preview.sequence = value),
        ("generation", |preview, value| preview.generation = value),
        ("searchCount", |preview, value| preview.search_count = value),
        ("rejectedCount", |preview, value| preview.rejected_count = value),
    ];
    let usize_fields: [(&str, UsizeSetter); 7] = [
        ("collisionCount", |preview, value| preview.collision_count = value),
        ("sampleCursor", |preview, value| preview.sample_cursor = value),
        ("insideBoth", |preview, value| preview.inside_both = value),
        ("targetCursor", |preview, value| preview.target_cursor = value),
        ("candidateCursor", |preview, value| preview.candidate_cursor = value),
        ("acceptedCount", |preview, value| preview.accepted_count = value),
        ("totalCount", |preview, value| preview.total_count = value),
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
    let setters: [(&str, Setter); 9] = [
        ("stage", |preview, value| preview.stage = value),
        ("targetVortexFullId", |preview, value| preview.target_vortex_full_id = Some(value)),
        ("candidateObjectKindId", |preview, value| preview.candidate_object_kind_id = Some(value)),
        ("candidateGhost.targetVortexFullId", |preview, value| preview.candidate_ghost.as_mut().expect("fixture ghost").target_vortex_full_id = value),
        ("candidateGhost.objectKindId", |preview, value| preview.candidate_ghost.as_mut().expect("fixture ghost").object_kind_id = value),
        ("candidateGhost.meshUrl", |preview, value| preview.candidate_ghost.as_mut().expect("fixture ghost").mesh_url = value),
        ("currentPairObjectId", |preview, value| preview.current_pair_object_id = Some(value)),
        ("candidatePage[0]", |preview, value| preview.candidate_page[0] = Some(value)),
        ("rejectionReason", |preview, value| preview.rejection_reason = Some(value)),
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
        for _ in 0..20_000 {
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
        let mut accepted = FillBuilder::begin_preparation(roots(branch, cap), operation);
        let mut turns = 0;
        while accepted.stage != FillJobStage::PrepareTargets {
            accepted.prepare_one();
            turns += 1;
            assert!(turns < 16 * DOCUMENT_OBJECT_SLOTS, "{expected_branch} cap preparation must advance in bounded turns");
        }
        assert!(turns >= cap, "{expected_branch} cap must be installed cooperatively");

        let mut rejected = FillBuilder::begin_preparation(roots(branch, cap + 1), operation);
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
        assert!(rejected.preview.candidate_ghost.is_none());
        assert!(faulted(rejected.step(&mut context)));
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
fn capacity_refusal_publishes_generation_qualified_no_ghost_diagnostic_before_fault() {
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
    let mut builder = FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(HashMap::new())), Operation::new(OperationId(37), RevisionId(9), Generation(11), 37));
    builder.preview.registry_generation = 13;
    let mut sequence = 0;
    let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
    assert!(matches!(builder.step(&mut context), StepOutcome::PreviewReady(_)));
    assert_eq!((builder.preview.operation, builder.preview.base_revision, builder.preview.registry_generation, builder.preview.generation), (37, 9, 13, 11));
    assert_eq!(builder.preview.rejection_reason.as_deref(), Some(format!("preparation-capacity:fixture-objects:{DOCUMENT_OBJECT_SLOTS}").as_str()));
    assert!(builder.preview.candidate_ghost.is_none());
    assert_eq!(builder.preview.sequence, 0, "the refusal is published under the operation's own first preview sequence");
    assert!(faulted(builder.step(&mut context)));
    drop(context);
    assert_eq!(sequence, 1, "publishing the refusal consumed exactly one preview sequence, and the fault that follows consumes none");
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
    let mut builder = FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(meshes)), Operation::new(OperationId(29), RevisionId(1), Generation(1), 29));
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
    assert_eq!([declared("bookkeepingSlots"), declared("bookkeepingPageBytes"), declared("documentPageBytes"), declared("fillCountMax")], [FIXED_OWNER_SLOTS, FIXED_OWNER_PAGE_BYTES, DOCUMENT_OWNER_PAGE_BYTES, FILL_COUNT_MAX]);
    assert_eq!(
        [declared("objectSlots"), declared("attractionSlots"), declared("vortexSlots"), declared("volumeSlots"), declared("kindSlots"), declared("candidateSlots"), declared("cellSlots")],
        [DOCUMENT_OBJECT_SLOTS, DOCUMENT_ATTRACTION_SLOTS, DOCUMENT_VORTEX_SLOTS, DOCUMENT_VOLUME_SLOTS, DOCUMENT_KIND_SLOTS, DOCUMENT_CANDIDATE_SLOTS, DOCUMENT_CELL_SLOTS]
    );
    let nakagin = |field: &str| capacities["nakagin"][field].as_u64().unwrap_or_else(|| panic!("nakagin {field}")) as usize;
    assert!(nakagin("objects") + FILL_COUNT_MAX <= DOCUMENT_OBJECT_SLOTS, "the flagship fixture plus a full plan must fit the object capacity");
    assert!(nakagin("attractions") + FILL_COUNT_MAX <= DOCUMENT_ATTRACTION_SLOTS);
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
    let mut builder = FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(43), RevisionId(1), Generation(1), 43));
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
