//! ⏱️ The BUDGET law of the `evaluate` capability, replayed from
//! `🧫️fixtures/⏱️evaluate-budget.json` against the real `FlowEvalSession`, the real fold
//! and the real status projection.
//!
//! The companion `🔬️unit` module pins what ONE finished answer does; this pins what an UNFINISHED
//! one does, which is the whole reason a sixteen-second boolean no longer kills its worker
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`).

use super::*;
use crate::editor_domain::editor_laws::context::retire_flow_eval_session;
use crate::preview_reply;
use semio_framework_test_fixtures::repo_text;

fn evaluate_budget_fixture_json() -> &'static str { repo_text!("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/⏱️evaluate-budget.json") }
const GEOMETRY_EXTENSION_PLUGIN_ID: &str = "flow-extension-brep";
const BUDGET_WINDOW_ID: &str = "procedural-preview-test";
const BUDGET_NODE_HASH: u64 = 0x_b0_07_ca_11_u64;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct EvaluateBudgetFixture {
    format: String,
    version: u8,
    capability: String,
    cancel_capability: String,
    response_action: String,
    phase_labels: std::collections::BTreeMap<String, PhaseLabel>,
    job_phase_tags: std::collections::BTreeMap<String, String>,
    stepped_operator: SteppedOperator,
}

#[derive(serde::Deserialize)]
struct PhaseLabel {
    en: String,
    de: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExpectedStatus {
    phase: String,
    in_flight: u64,
    eval_units_done: u64,
    eval_units_total: u64,
    ratio: f64,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SteppedOperator {
    operator_id: String,
    minimum_round_trips_under_a_tight_budget: usize,
    phase_order: Vec<String>,
}

fn fixture() -> EvaluateBudgetFixture {
    let fixture: EvaluateBudgetFixture = serde_json::from_str(evaluate_budget_fixture_json()).expect("evaluate budget fixture");
    assert_eq!(fixture.format, "semio.generation3d.evaluate-budget");
    assert_eq!(fixture.version, 1);
    fixture
}

/// 📈️ The status object the preview window would publish right now, through the same pure
/// projection the surface uses.
fn observed_status(session: &FlowEvalSession) -> serde_json::Value {
    serde_json::from_str(&semio_s_artifact_procedural_3d_preview_eval::preview_progress_status_json_for(Some(session), None, Ok(GEOMETRY_EXTENSION_PLUGIN_ID.to_string()))).expect("status json")
}

/// ⏱️ The hop the run dispatched before an answer can land: armed, begun, recorded unfinished, one
/// answer outstanding — the latch state every `flowEvalResolve` folds into.
fn park_one_evaluate_hop(session: &mut FlowEvalSession) {
    assert!(session.arm_window_tick(BUDGET_WINDOW_ID, crate::preview_reply::TURN).expect("window tick admission").0, "the run arms the hop");
    session.begin_window_tick(BUDGET_WINDOW_ID, crate::preview_reply::TURN).map(drop).expect("original window latch turn");
    session.note_window_tick_outcome(BUDGET_WINDOW_ID, semio_s_artifact_procedural_3d_preview_eval::tick_is_unfinished(true, 1), crate::preview_reply::TURN).map(drop).expect("original window latch turn");
    session.note_window_extensions_in_flight(BUDGET_WINDOW_ID, 1, crate::preview_reply::TURN).map(drop).expect("original window latch turn");
}

/// ⚖️ LAW: every row of the fixture, folded through the real command handler — what it seeds, whether
/// the window still owes the run a hop, and the exact status the preview window publishes afterwards.
#[test]
fn the_evaluate_budget_envelope_obeys_its_fixture_end_to_end() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let fixture = fixture();
    assert_eq!(fixture.capability, "evaluate", "the budgeted capability");
    assert_eq!(fixture.cancel_capability, "evaluateCancel", "the door a cancel reaches a parked evaluation through");
    assert_eq!(fixture.response_action, "flowEvalResolve", "the command this fold is bound to");
    let window = (BUDGET_WINDOW_ID, semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
    let cases = preview_reply::evaluation_fixture();
    for row in cases["rows"].as_array().unwrap() {
        let name = row["id"].as_str().unwrap();
        let expected: ExpectedStatus = serde_json::from_value(row["status"].clone()).unwrap();
        let mut session = FlowEvalSession::new();
        let cache = session.neural_cache();
        park_one_evaluate_hop(&mut session);
        if row["reply"]["phase"] == "cancelled" { preview_reply::cancel(&mut session, window.0); }
        let payload = preview_reply::evaluation_payload(window, BUDGET_NODE_HASH, GEOMETRY_EXTENSION_PLUGIN_ID,
            preview_reply::dto(row["reply"].clone()));
        preview_reply::receive_evaluation(&mut session, payload).expect("typed evaluation reply");
        let cached = cache.get(BUDGET_NODE_HASH);
        let expected_body = &row["reply"]["output"];
        assert_eq!(cached.is_some(), !expected_body.is_null(), "{name}: cache publication");
        if let Some(cached) = cached {
            let observed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&cached)).unwrap();
            assert_eq!(observed, *expected_body, "{name}: cached Dictionary");
            semio_os_flow::neural::ColdRetire::retire_cold(cached);
        }
        drop(cache);
        assert_eq!(session.window_tick_owed(window.0), row["owesHop"].as_bool().unwrap(), "{name}: continuation");
        let status = observed_status(&session);
        assert_eq!(status["phase"].as_str(), Some(expected.phase.as_str()), "{name}: phase");
        let labels = &fixture.phase_labels[&expected.phase];
        assert_eq!(status["phaseLabel"]["en"].as_str(), Some(labels.en.as_str()), "{name}: English phase");
        assert_eq!(status["phaseLabel"]["de"].as_str(), Some(labels.de.as_str()), "{name}: German phase");
        assert_eq!(status["progress"]["inFlight"].as_u64(), Some(expected.in_flight), "{name}: inFlight");
        assert_eq!(status["progress"]["evalUnitsDone"].as_u64(), Some(expected.eval_units_done), "{name}: evalUnitsDone");
        assert_eq!(status["progress"]["evalUnitsTotal"].as_u64(), Some(expected.eval_units_total), "{name}: evalUnitsTotal");
        let ratio = status["progress"]["ratio"].as_f64().unwrap();
        assert!((ratio - expected.ratio).abs() < 1e-9, "{name}: ratio {ratio} != {}", expected.ratio);
        assert_eq!(status["cancelAction"].as_str(), Some(semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID));
        retire_flow_eval_session(session);
    }
}

#[test]
fn invalid_original_evaluation_replies_leave_the_cache_and_window_unchanged() {
    let window = (BUDGET_WINDOW_ID, semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
    let cases = preview_reply::evaluation_fixture();
    for reply in cases["refused"].as_array().unwrap() {
        let mut session = FlowEvalSession::new();
        let cache = session.neural_cache();
        park_one_evaluate_hop(&mut session);
        let payload = preview_reply::evaluation_payload(window, BUDGET_NODE_HASH, GEOMETRY_EXTENSION_PLUGIN_ID, preview_reply::dto(reply.clone()));
        let failure = preview_reply::receive_evaluation(&mut session, payload).expect_err("invalid reply must be refused");
        assert_eq!(failure.primary_value().unwrap().kind, semio_framework_value::ValueRefusalKind::InvalidValue);
        assert!(!cache.contains(BUDGET_NODE_HASH));
        assert_eq!(session.window_extensions_in_flight(window.0), 1);
        assert!(!session.window_tick_owed(window.0));
        drop(cache);
        retire_flow_eval_session(session);
    }
}

/// ⚖️ LAW: the fixture's declared job phase vocabulary IS the one the session projects. A tag the
/// vocabulary does not know reads as `computing`, never as `idle` — the envelope that carried it
/// said the job is still working.
#[test]
fn every_declared_job_phase_tag_projects_to_its_declared_surface_phase() {
    let fixture = fixture();
    for (job_tag, surface_tag) in &fixture.job_phase_tags {
        assert_eq!(semio_os_flow::PreviewEvalPhase::from_job_tag(job_tag).tag(), surface_tag.as_str(), "job phase `{job_tag}` projects to `{surface_tag}`");
    }
    assert_eq!(semio_os_flow::PreviewEvalPhase::from_job_tag("a-phase-no-surface-has-heard-of").tag(), "computing", "an unknown tag is live work, never idle");
    for (tag, labels) in &fixture.phase_labels {
        let phase = semio_os_flow::PreviewEvalPhase::from_job_tag(match tag.as_str() {
            "imprinting" => "imprint",
            "splitting" => "applyA",
            "classifying" => "classifyA",
            "stitching" => "stitch",
            "validating" => "validate",
            other => other,
        });
        assert_eq!(phase.tag(), tag.as_str(), "the fixture's label table is keyed by surface phase");
        let (english, german) = phase.labels();
        assert_eq!(english, labels.en, "english label for `{tag}`");
        assert_eq!(german, labels.de, "german label for `{tag}`");
    }
}

/// ⚖️ LAW: progress is monotone across the round trips of one evaluation, and a working envelope
/// never seeds. Replayed as a real SEQUENCE (the fixture's rows are each a single fold), because
/// monotonicity is a property of the sequence, not of any one answer.
#[test]
fn progress_is_monotone_across_the_round_trips_of_one_evaluation() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let fixture = fixture();
    let window = (BUDGET_WINDOW_ID, semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
    let mut session = FlowEvalSession::new();
    let cache = session.neural_cache();
    let mut previous_done = 0_u64;
    let total_steps = fixture.stepped_operator.phase_order.len();
    assert!(total_steps >= fixture.stepped_operator.minimum_round_trips_under_a_tight_budget, "the declared phase order must admit at least the declared minimum of round trips");
    for (index, phase) in fixture.stepped_operator.phase_order.iter().enumerate() {
        let done = index == total_steps - 1;
        let units_done = (index + 1) as u64;
        let output = if done { serde_json::json!({"solid":"brep:solid-9"}) } else { serde_json::Value::Null };
        park_one_evaluate_hop(&mut session);
        let reply = preview_reply::evaluation_response(done, phase, (units_done, total_steps as u64), output);
        let payload = preview_reply::evaluation_payload(window, BUDGET_NODE_HASH, GEOMETRY_EXTENSION_PLUGIN_ID, reply);
        preview_reply::receive_evaluation(&mut session, payload).expect("typed evaluation progression");
        let status = observed_status(&session);
        let observed_done = status["progress"]["evalUnitsDone"].as_u64().expect("evalUnitsDone");
        if done {
            assert!(cache.contains(BUDGET_NODE_HASH), "the terminal answer seeds the node cache");
            assert_eq!(observed_done, 0, "a finished evaluation leaves no in-flight row behind");
        } else {
            assert!(!cache.contains(BUDGET_NODE_HASH), "`{phase}` is still working, so nothing may be seeded");
            assert!(observed_done >= previous_done, "unitsDone never decreases ({previous_done} -> {observed_done} at `{phase}`)");
            assert_eq!(status["progress"]["inFlight"].as_u64(), Some(1), "`{phase}` is live work, and the surface must say so");
            assert_ne!(status["phase"].as_str(), Some("idle"), "`{phase}` must never publish as idle");
            previous_done = observed_done;
        }
        assert!(session.window_tick_owed(BUDGET_WINDOW_ID), "`{phase}` owes the run exactly one continuation");
    }
    if let Some(cached) = cache.get(BUDGET_NODE_HASH) {
        semio_os_flow::neural::ColdRetire::retire_cold(cached);
    }
    drop(cache);
    retire_flow_eval_session(session);
}

/// 🔁️ A real Pack progress answer owes the mounted editor one continuation with the same evaluation identity.
#[semio_framework_async_macros::async_test]
async fn the_preview_reissues_working_evaluations_as_compact_resumes() {
    use crate::editor_domain::editor_laws::context::{app_with_registry, drive_preview_run_serving, shell_views};

    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (view, _) = shell_views("procedural-resume-main", "procedural-resume-preview");
    let mut working = std::collections::BTreeSet::new();
    let mut resumes = 0;
    let run = drive_preview_run_serving(&mut app, &view, &[], &mut |pending| {
        let answer = crate::brep_extension::serve(pending)?;
        if pending.capability != "evaluate" {
            return Ok(answer);
        }
        let request: serde_json::Value = serde_json::from_str(&pending.request_json).expect("evaluate request");
        let text = |field: &str| request[field].as_str().expect("evaluate text field");
        let key = (pending.extension_id.clone(), text("windowId").to_owned(), text("operatorId").to_owned(), request["nodeHash"].as_u64().expect("evaluate node hash"));
        if request["resume"].as_bool().expect("evaluate resume flag") {
            assert!(working.remove(&key), "resume must follow a working Pack answer for {key:?}");
            assert_eq!(text("inputJson"), "");
            assert_eq!(text("dependencyJson"), "");
            resumes += 1;
        }
        let reply = semio_os_flow::os_pack::pack_rt::decode_wire_value(&answer).expect("evaluate Pack reply");
        if matches!(reply.get("done"), Some(semio_framework_value::DslValue::Bool(false))) {
            assert!(working.insert(key), "one progress answer owes one continuation");
        }
        Ok(answer)
    }).await;
    assert_eq!(run.state.as_deref(), Some("finalized"));
    assert!(resumes > 0, "default evaluation cadence must exercise a resume");
    assert!(working.is_empty(), "every working Pack answer must be resumed");
}

/// ⚖️ LAW: the release a closed run owes reaches BOTH retained-work registries in the geometry
/// extension — the parked budgeted evaluations and the mesh jobs — because a boolean stopped
/// mid-validation lives only in the first.
#[test]
fn the_kernel_release_reaches_the_evaluation_registry_too() {
    let fixture = fixture();
    let payload = semio_s_artifact_procedural_3d_preview_eval::FlowEvalRelease { window_id: BUDGET_WINDOW_ID.into(), window_kind_id: semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() };
    let invocations = semio_s_artifact_procedural_3d_preview_eval::release_invocations_for(&payload, Ok(GEOMETRY_EXTENSION_PLUGIN_ID.to_string()));
    let capabilities: Vec<&str> = invocations.iter().map(|invocation| invocation.capability.as_str()).collect();
    assert!(capabilities.contains(&fixture.cancel_capability.as_str()), "the release emits the fixture's evaluation-cancel capability, got {capabilities:?}");
    assert!(capabilities.contains(&"tessellateCancel"), "the release still emits the mesh-job cancel, got {capabilities:?}");
}

//#region 🔁️InlineContinuation
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InlineContinuationFixture {
    examples: Vec<InlineContinuationExample>,
    rows: Vec<InlineContinuationRow>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InlineContinuationExample {
    id: String,
    contributed_nodes: usize,
    wave_widths: Vec<usize>,
    before_coalescing: usize,
    after_coalescing: usize,
    after_inline: usize,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InlineContinuationRow {
    id: String,
    wave_widths: Vec<usize>,
    #[serde(default)]
    cancel_before_hop: Option<usize>,
    #[serde(default)]
    spent_turn_before_hop: Option<usize>,
    expected_dispatched_hops: usize,
    expected_inline_waves: usize,
    #[serde(default)]
    expected_continuations_after_cancel: Option<usize>,
}

fn inline_continuation_fixture() -> InlineContinuationFixture {
    let root: serde_json::Value = serde_json::from_str(evaluate_budget_fixture_json()).expect("evaluate budget fixture");
    serde_json::from_value(root["inlineContinuation"].clone()).expect("the fixture declares its inline-continuation law")
}

/// 🔢️ What one replayed chain cost: hops the HOST dispatched, waves a fold ran inline, and
/// continuations a cancelled session nevertheless admitted (which must always be zero).
#[derive(Debug, Default, PartialEq, Eq)]
struct ChainLadder {
    dispatched_hops: usize,
    inline_waves: usize,
    continuations_after_cancel: usize,
}

/// 🕰️ The instant a replayed guest turn began, and the instant its fold asks the admission question —
/// stated in microseconds, never read off a clock, so the branch a browser turn takes is drivable in a
/// native test (which installs no clock at all).
const REPLAY_TURN_STARTED_US: u64 = 1_000_000;

fn replay_now_us(spent: bool) -> Option<u64> {
    Some(if spent { REPLAY_TURN_STARTED_US + semio_os_flow::FLOW_EVAL_TICK_ELAPSED_CEILING_US } else { REPLAY_TURN_STARTED_US })
}

/// ⛓️ The hop ladder of ONE chain, driven through the real latch exactly as the run job and the two
/// window-addressed folds drive it: the scheduler dispatches a hop only for a debt no fold took over,
/// a hop parks its wave, and each answer of that wave asks
/// [`FlowEvalSession::inline_continuation_admitted`] in its own guest turn.
fn replay_chain(wave_widths: &[usize], cancel_before_hop: Option<usize>, spent_turn_before_hop: Option<usize>) -> ChainLadder {
    let mut session = FlowEvalSession::new();
    let mut ladder = ChainLadder::default();
    let mut cancelled = false;
    assert!(session.window_tick_owed(BUDGET_WINDOW_ID), "the gesture that changed the document left the window owing its first hop");
    assert!(session.arm_window_tick(BUDGET_WINDOW_ID, crate::preview_reply::TURN).expect("window tick admission").0, "the run job dispatches that first hop");
    ladder.dispatched_hops += 1;
    for (index, width) in wave_widths.iter().enumerate() {
        let hop = index + 1;
        session.begin_window_tick(BUDGET_WINDOW_ID, crate::preview_reply::TURN).map(drop).expect("original window latch turn");
        session.note_window_tick_outcome(BUDGET_WINDOW_ID, semio_s_artifact_procedural_3d_preview_eval::tick_is_unfinished(false, *width), crate::preview_reply::TURN).map(drop).expect("original window latch turn");
        session.note_window_extensions_in_flight(BUDGET_WINDOW_ID, *width, crate::preview_reply::TURN).map(drop).expect("original window latch turn");
        let mut continued = false;
        for answer in 0..*width {
            let last = answer + 1 == *width;
            if last && cancel_before_hop == Some(hop + 1) {
                preview_reply::cancel(&mut session, BUDGET_WINDOW_ID);
                cancelled = true;
            }
            session.settle_window_extension(BUDGET_WINDOW_ID, crate::preview_reply::TURN).map(drop).expect("original window latch turn");
            let spent = last && spent_turn_before_hop == Some(hop + 1);
            if session.inline_continuation_admitted(BUDGET_WINDOW_ID, Some(REPLAY_TURN_STARTED_US), replay_now_us(spent)) {
                assert!(session.arm_window_tick(BUDGET_WINDOW_ID, crate::preview_reply::TURN).expect("window tick admission").0, "a continuation CLAIMS the hop it takes over");
                continued = true;
                ladder.inline_waves += 1;
                if cancelled {
                    ladder.continuations_after_cancel += 1;
                }
            }
        }
        if cancelled {
            break;
        }
        if !continued {
            assert!(session.window_tick_owed(BUDGET_WINDOW_ID), "a declined continuation leaves the debt exactly where the scheduler reads it");
            assert!(session.arm_window_tick(BUDGET_WINDOW_ID, crate::preview_reply::TURN).expect("window tick admission").0, "so the run job dispatches the round trip instead");
            ladder.dispatched_hops += 1;
        }
    }
    if !cancelled {
        session.begin_window_tick(BUDGET_WINDOW_ID, crate::preview_reply::TURN).map(drop).expect("original window latch turn");
        session.note_window_tick_outcome(BUDGET_WINDOW_ID, false, crate::preview_reply::TURN).map(drop).expect("original window latch turn");
        assert!(!session.window_tick_owed(BUDGET_WINDOW_ID), "the terminal walk owes nothing and the run settles");
    }
    retire_flow_eval_session(session);
    ladder
}

/// ⚖️ LAW: every bundled example's chain costs ONE dispatched `flowEvalTick` hop, whatever its depth —
/// the wave coalescing removed the per-NODE hop, and the inline continuation removes the per-LEVEL one.
///
/// 📐️ Fixture-driven and honest about what it is: the wave shapes come from each example's own graph,
/// and the claim asserted here is about the LATCH ladder, not about a browser. The browser count is
/// `performInvocation settled {"actionId":"flowEvalTick"}` per example in
/// `📓️flow-inline-continuation-2026-09-14.md`.
#[test]
fn every_example_chain_costs_one_dispatched_hop_once_the_folds_continue_it_inline() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let fixture = inline_continuation_fixture();
    assert_eq!(fixture.examples.len(), 8, "all eight bundled examples declare their wave shape");
    let mut ladders = Vec::new();
    for example in &fixture.examples {
        assert_eq!(example.contributed_nodes, example.wave_widths.iter().sum::<usize>(), "{}: the waves account for every contributed node", example.id);
        assert_eq!(example.after_coalescing, example.wave_widths.len() + 1, "{}: coalescing bottoms out at one hop per LEVEL plus a terminal one", example.id);
        assert!(example.before_coalescing >= example.after_coalescing, "{}: the coalescing lane never made an example worse", example.id);
        let ladder = replay_chain(&example.wave_widths, None, None);
        assert_eq!(ladder.dispatched_hops, example.after_inline, "{}: dispatched hops", example.id);
        assert_eq!(ladder.dispatched_hops, 1, "{}: one gesture, one hop, however deep the graph", example.id);
        assert_eq!(ladder.inline_waves, example.wave_widths.len(), "{}: every wave after the first hop runs inside an answer's own turn", example.id);
        ladders.push((example.id.clone(), example.before_coalescing, example.after_coalescing, ladder.dispatched_hops));
    }
    let total_before: usize = fixture.examples.iter().map(|example| example.before_coalescing).sum();
    let total_after: usize = fixture.examples.iter().map(|example| example.after_inline).sum();
    assert!(total_after * 4 < total_before, "the eight example loads must fall by far more than a quarter: {total_before} -> {total_after}");
}

/// ⚖️ LAW: the fixture's interference rows — a cancel landing between two waves, and a turn with no
/// wall left — each answered by the real latch.
///
/// 🚨️ These are the two ways the continuation must give the round trip back. A cancel must stop the
/// chain even though the answer that would have continued it was already crossing when the gesture
/// landed; a spent turn must park rather than stretch the 8 ms interactive hold by another ceiling.
#[test]
fn a_cancel_between_waves_and_a_spent_turn_each_hand_the_round_trip_back_to_the_host() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let fixture = inline_continuation_fixture();
    assert_eq!(fixture.rows.len(), 3, "the fixture declares all three interference rows");
    for row in &fixture.rows {
        let ladder = replay_chain(&row.wave_widths, row.cancel_before_hop, row.spent_turn_before_hop);
        assert_eq!(ladder.dispatched_hops, row.expected_dispatched_hops, "{}: dispatched hops", row.id);
        assert_eq!(ladder.inline_waves, row.expected_inline_waves, "{}: inline waves", row.id);
        if let Some(expected) = row.expected_continuations_after_cancel {
            assert_eq!(ladder.continuations_after_cancel, expected, "{}: a cancelled chain admits no continuation at all", row.id);
        }
    }
}
//#endregion 🔁️InlineContinuation
