use super::*;
use crate::editor::puzzle2d::config::PUZZLE2D_DEFAULT_SUGGESTION_OFFSET;
use crate::editor::puzzle2d::unit_tests::context::*;
use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::{fixture_nodes, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID};
use geo::{coord, Intersects, Rect};
use semio_framework_job::{drive_step, InteractiveStage, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_US};
use semio_framework_plugin::{DslValue, PluginApp};
use semio_framework_tool_run::{ToolRunId, ToolRunStep, ToolRunTick, ToolRunTraceOp};
use serde_json::json;
use std::collections::BTreeMap;

const FILL_RUN_FIXTURE: &str = include_str!("../../🧫️fixtures/🎞️fill-run.json");

fn fixture() -> Value {
    let fixture: Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run fixture parses");
    assert_eq!(fixture["schema"], "s.puzzle2d.fill-run.v1");
    fixture
}

fn number(value: &Value) -> u64 {
    value.as_u64().unwrap_or_else(|| panic!("fixture number expected, found {value}"))
}

fn text(value: &Value) -> &str {
    value.as_str().unwrap_or_else(|| panic!("fixture text expected, found {value}"))
}

/// 🛍️ The committed document of a fixture `document` spec: an example load (or the empty board), optionally reduced
/// to its first `keepNodes` nodes and the edges between them, or with every edge detached.
fn example(spec: &Value) -> Arc<Puzzle2dPlaySnapshot> {
    let mut app = app_with_registry();
    if text(&spec["example"]) != "empty" {
        load_example(&mut app, text(&spec["example"]));
    }
    let mut document = fixture_of(&app);
    close_app(&mut app);
    if let Some(keep) = spec["keepNodes"].as_u64() {
        let nodes = document["nodes"].as_array_mut().expect("nodes");
        nodes.truncate(keep as usize);
        let handles: HashSet<String> = nodes.iter().flat_map(|node| node["handles"].as_array().cloned().unwrap_or_default()).filter_map(|handle| handle["id"].as_str().map(str::to_string)).collect();
        document["edges"].as_array_mut().expect("edges").retain(|edge| edge["source"].as_str().is_some_and(|id| handles.contains(id)) && edge["target"].as_str().is_some_and(|id| handles.contains(id)));
    }
    if spec["detachEdges"].as_bool() == Some(true) {
        document["edges"] = json!([]);
    }
    Arc::new(Puzzle2dPlaySnapshot(document))
}

fn identity(run: u64, generation: u32) -> ToolRunIdentity {
    ToolRunIdentity { id: ToolRunId { app_instance_id: 1, run }, generation, base_revision: [0; 32] }
}

fn fresh(document: &Arc<Puzzle2dPlaySnapshot>, run: u64, requested: u64) -> Puzzle2dFillRunJob {
    Puzzle2dFillRunJob::new(identity(run, 0), Arc::clone(document), PUZZLE2D_DEFAULT_SUGGESTION_OFFSET, requested as u32, None, &[]).expect("fresh fill run")
}

fn payload_bytes(payload: &RetainedJobPayload) -> Vec<u8> {
    (0..payload.page_count()).flat_map(|index| payload.page(index).unwrap_or_default().to_vec()).collect()
}

/// 📼️ Everything a sequence of run ticks told the ledger, applied the way the ledger applies it.
#[derive(Default)]
struct RunLog {
    finals: Vec<(u64, ToolRunVerdict, u16, ToolRunTraceSubject)>,
    tick_finals: Vec<usize>,
    testing: Vec<u64>,
    retired: Vec<u64>,
    ops: Vec<Vec<u8>>,
    entities: Vec<u64>,
    steps: Vec<ToolRunStep>,
    retracts: Vec<u32>,
    progress: Option<ToolRunProgress>,
    checkpoints: Vec<Vec<u8>>,
    complete: bool,
}

impl RunLog {
    fn continuing(ops: &[Vec<u8>]) -> Self {
        Self { ops: ops.to_vec(), entities: vec![0; ops.len() / FILL_RUN_OPS_PER_PLACEMENT], ..Self::default() }
    }

    fn apply(&mut self, tick: ToolRunTick) {
        if let Some(length) = tick.retract_to {
            self.retracts.push(length);
            self.ops.truncate(length as usize);
            self.entities.truncate(length as usize / FILL_RUN_OPS_PER_PLACEMENT);
        }
        let mut finals = 0;
        for op in tick.trace.into_iter().flat_map(|page| page.ops) {
            match op {
                ToolRunTraceOp::Upsert { key, verdict: ToolRunVerdict::Testing, .. } => self.testing.push(key),
                ToolRunTraceOp::Upsert { key, verdict, reason, subject } => {
                    finals += 1;
                    self.finals.push((key, verdict, reason, subject));
                }
                ToolRunTraceOp::Retire { key } => self.retired.push(key),
                ToolRunTraceOp::Clear => {}
            }
        }
        self.tick_finals.push(finals);
        self.ops.extend(tick.append_ops);
        self.entities.extend(tick.append_entities);
        self.steps.extend(tick.steps);
        if tick.progress.is_some() {
            self.progress = tick.progress;
        }
    }

    fn verdicts(&self) -> BTreeMap<u64, (ToolRunVerdict, u16)> {
        self.finals.iter().map(|(key, verdict, reason, _)| (*key, (*verdict, *reason))).collect()
    }

    fn counters(&self) -> Vec<u64> {
        self.progress.as_ref().expect("a run reports progress").counters.iter().map(|counter| counter.value).collect()
    }
}

fn close_job(job: &mut dyn InteractiveJob) {
    job.begin_close();
    for _ in 0..1 << 20 {
        if matches!(job.close_step(1, JOB_PAYLOAD_PAGE_BYTES), InteractiveJobCloseStep::Complete) && job.terminal_is_empty() {
            return;
        }
    }
    panic!("fill run job never reached terminal-empty");
}

fn step_job(job: &mut dyn InteractiveJob, budget: StepBudget, now_us: fn() -> Option<u64>, log: &mut RunLog, sequence: &mut u64) {
    let mut context = StepContext::new(OperationId(1), Generation(0), budget, semio_framework_job::root_cancel_token(), now_us, sequence);
    let mut outcome = job.step(&mut context);
    drop(context);
    let failure = match &outcome {
        StepOutcome::PreviewReady(payload) => ToolRunTick::decode(&payload_bytes(payload)).map(|tick| log.apply(tick)).err().map(|error| format!("tick decode {error:?}")),
        StepOutcome::CheckpointReady(checkpoint) => {
            log.checkpoints.push(payload_bytes(&checkpoint.state));
            None
        }
        StepOutcome::Complete(_) => {
            log.complete = true;
            None
        }
        StepOutcome::Yield => None,
        StepOutcome::Cancelled => Some("cancelled".to_string()),
        StepOutcome::Fault(fault) => Some(String::from_utf8_lossy(&payload_bytes(&fault.detail)).into_owned()),
    };
    close_outcome(&mut outcome);
    if let Some(failure) = failure {
        close_job(job);
        panic!("fill run failed: {failure}");
    }
}

fn run_to_complete(job: &mut dyn InteractiveJob, fuel: u64, log: &mut RunLog) {
    let mut sequence = 0;
    for _ in 0..1 << 24 {
        if log.complete {
            close_job(job);
            return;
        }
        step_job(job, StepBudget::new(fuel, u64::MAX), fill_run_monotonic_zero, log, &mut sequence);
    }
    panic!("fill run never completed");
}

fn run(document: &Arc<Puzzle2dPlaySnapshot>, run: u64, requested: u64) -> RunLog {
    let mut job = fresh(document, run, requested);
    let mut log = RunLog::default();
    run_to_complete(&mut job, INTERACTIVE_LANE_FUEL, &mut log);
    log
}

fn verdict_id(verdict: ToolRunVerdict) -> String {
    serde_json::to_value(verdict).expect("verdict serializes").as_str().unwrap_or_default().to_string()
}

fn reason_id(code: u16) -> &'static str {
    FillRunReason::from_code(code).map_or("reserved", FillRunReason::id)
}

fn decoded(ops: &[Vec<u8>]) -> Vec<Puzzle2dMutation> {
    ops.iter().map(|op| <Puzzle2dMutation as protocol::OpBinary>::decode_op(op).expect("provisional op decodes")).collect()
}

/// 📜️ The language-neutral case summary of one run.
fn summary(log: &RunLog, prefix: usize) -> Value {
    let counters = log.counters();
    let stall = log.steps.last().filter(|step| step.kind == ToolRunStepKind::Warning).map(|step| reason_id(step.reason));
    json!({
        "verdictPrefix": log.finals.iter().take(prefix).map(|(key, verdict, reason, _)| format!("{key}:{}:{}", verdict_id(*verdict), reason_id(*reason))).collect::<Vec<_>>(),
        "decisionPrefix": log.finals.iter().filter(|(_, verdict, ..)| *verdict != ToolRunVerdict::Warning).take(prefix).map(|(key, verdict, reason, _)| format!("{key}:{}:{}", verdict_id(*verdict), reason_id(*reason))).collect::<Vec<_>>(),
        "tested": counters[0],
        "accepted": counters[1],
        "collisions": counters[2],
        "rejected": counters[3],
        "appendOps": log.ops.len(),
        "appendEntities": log.entities.len(),
        "checkpoints": log.checkpoints.len(),
        "stall": stall,
    })
}

/// 🎞️ Every fixture case reproduces its exact verdict prefix, counters, op/entity/checkpoint counts and stall, and
/// every run keeps the per-run laws: two ops and one entity per placement, verdict counts equal the counters,
/// every final verdict was first `testing`, ops alternate `create_node`/`connect_handles`, entities digest the
/// created node ids, placements never reuse a document id, and the run ends `complete` with one closing step.
#[test]
fn fill_run_job_matches_the_language_neutral_fill_run_fixture() {
    let fixture = fixture();
    let prefix = number(&fixture["verdictPrefix"]) as usize;
    for case in fixture["cases"].as_array().expect("cases") {
        let document = example(&case["document"]);
        let log = run(&document, number(&case["run"]), number(&case["requested"]));
        assert_eq!(summary(&log, prefix), case["expected"], "{}", text(&case["id"]));
        let counters = log.counters();
        assert_eq!(log.ops.len() as u64, counters[1] * number(&fixture["opsPerPlacement"]));
        assert_eq!(log.entities.len() as u64, counters[1]);
        let count = |wanted: ToolRunVerdict| log.finals.iter().filter(|(_, verdict, ..)| *verdict == wanted).count() as u64;
        assert_eq!((log.finals.len() as u64, count(ToolRunVerdict::Success), count(ToolRunVerdict::Danger), count(ToolRunVerdict::Warning)), (counters[0], counters[1], counters[2], counters[3]));
        assert!(log.finals.iter().all(|(key, ..)| log.testing.contains(key)), "every final verdict was announced as testing first");
        let existing: HashSet<String> = document.0["nodes"].as_array().into_iter().flatten().filter_map(|node| node["id"].as_str().map(str::to_string)).collect();
        for (placement, pair) in decoded(&log.ops).chunks(FILL_RUN_OPS_PER_PLACEMENT).enumerate() {
            let [Puzzle2dMutation::CreateNode(create), Puzzle2dMutation::ConnectHandles(_)] = pair else { panic!("placement {placement} is not create_node then connect_handles") };
            assert_eq!(log.entities[placement], fill_run_entity(&create.node.id));
            assert!(!existing.contains(&create.node.id), "{} reuses a document id", create.node.id);
        }
        let progress = log.progress.as_ref().expect("progress");
        assert_eq!((progress.state, progress.completed, progress.total), (ToolRunState::Complete, counters[1], Some(number(&case["requested"]))));
        assert!(log.complete);
        let closing = log.steps.last().expect("a closing step");
        assert_eq!(closing.args, vec![ToolRunStepArg::Unsigned(counters[1])]);
        assert!(matches!((closing.kind, FillRunReason::from_code(closing.reason)), (ToolRunStepKind::Success, Some(FillRunReason::RequestedReached)) | (ToolRunStepKind::Warning, Some(FillRunReason::NoOpenHandle | FillRunReason::NoCompatibleKind | FillRunReason::NoFreePlacement | FillRunReason::ArtifactCapacity))));
        let checkpoint = FillRunCheckpoint::decode(log.checkpoints.last().expect("final checkpoint")).expect("checkpoint decodes");
        assert_eq!((checkpoint.tested, checkpoint.placements.len() as u64, checkpoint.collisions, checkpoint.rejected), (counters[0], counters[1], counters[2], counters[3]));
    }
}

/// 🧮️ The checkpoint codec is exact: a round trip is lossless and any length but header + 12·placements is refused.
/// 🎯️ Every node this run places lies fully inside the one visible target region the board declares,
/// the same run without that region provably leaves it, at least one candidate is refused
/// `outside-target-region`, and HIDING the region takes the constraint away again — the four halves
/// of puzzle3d's target-volume rule, stated by `🎞️fill-run.json`'s own `targetRegion` vector.
#[test]
fn fill_run_job_places_only_inside_visible_target_regions() {
    let fixture = fixture();
    let vector = &fixture["targetRegion"];
    let expected = &vector["expected"];
    let base = example(&vector["document"]);
    let seed = fixture_nodes(&base.0).first().cloned().expect("the reduced board keeps its seed node");
    let (cx, cy) = (seed["x"].as_f64().expect("seed x"), seed["y"].as_f64().expect("seed y"));
    let half = vector["halfSpan"].as_f64().expect("half span");
    let bounds = [cx - half, cy - half, cx + half, cy + half];
    let painted = |hidden: bool| {
        let mut document = base.0.clone();
        document["targetRegions"] = json!([{ "id": "region-1", "x": bounds[0], "y": bounds[1], "width": half * 2.0, "height": half * 2.0, "hidden": hidden, "locked": false }]);
        Arc::new(Puzzle2dPlaySnapshot(document))
    };
    let (run_id, requested) = (number(&vector["run"]), number(&vector["requested"]));
    let inside = |log: &RunLog| {
        decoded(&log.ops)
            .into_iter()
            .filter_map(|mutation| match mutation {
                Puzzle2dMutation::CreateNode(payload) => Some(payload.node),
                _ => None,
            })
            .map(|node| {
                let rectangle = node.shape.as_deref() == Some("rectangle");
                fill_node_bounds(node.x, node.y, node.scale, rectangle, if rectangle { node.width } else { node.radius }, node.height)
            })
            .collect::<Vec<_>>()
    };

    let constrained = run(&painted(false), run_id, requested);
    let placed = inside(&constrained);
    assert!(!placed.is_empty(), "a region-constrained run must still place something, or this vector proves nothing");
    if expected["everyPlacementInsideRegion"].as_bool() == Some(true) {
        for placement in &placed {
            assert!(fill_regions_admit(&[bounds], *placement), "a region-constrained run placed {placement:?} outside the only visible target region {bounds:?}");
        }
    }
    if expected["atLeastOneOutsideRejection"].as_bool() == Some(true) {
        let reason = text(&expected["reasonId"]);
        let verdict = text(&expected["reasonVerdict"]);
        let refusals: Vec<&(u64, ToolRunVerdict, u16, ToolRunTraceSubject)> = constrained.finals.iter().filter(|(_, _, code, _)| reason_id(*code) == reason).collect();
        assert!(!refusals.is_empty(), "the constrained run refused nothing as {reason}, so the constraint never bit");
        for (_, produced, ..) in &refusals {
            assert_eq!(verdict_id(*produced), verdict, "an {reason} refusal carries the verdict the fixture declares");
        }
    }
    if expected["unconstrainedRunLeavesTheRegion"].as_bool() == Some(true) {
        let unconstrained = run(&base, run_id, requested);
        assert!(inside(&unconstrained).iter().any(|placement| !fill_regions_admit(&[bounds], *placement)), "the same run without a region stayed inside it anyway, so this vector cannot tell a constraint from a coincidence");
    }
    if expected["hiddenRegionConstrainsNothing"].as_bool() == Some(true) {
        let hidden = run(&painted(true), run_id, requested);
        assert_eq!(inside(&hidden), inside(&run(&base, run_id, requested)), "a hidden region must constrain exactly nothing");
    }
}

#[test]
fn fill_run_checkpoint_codec_round_trips_and_refuses_malformed_lengths() {
    let checkpoint = FillRunCheckpoint { requested: 45, tested: 310, collisions: 201, rejected: 79, next_key: 311, placements: vec![FillRunPlacementKey { key: 3, shape: 2 }, FillRunPlacementKey { key: 17, shape: u32::MAX }] };
    let bytes = checkpoint.encode();
    assert_eq!(bytes.len(), FillRunCheckpoint::HEADER_BYTES + 2 * FillRunCheckpoint::PLACEMENT_BYTES);
    assert_eq!(FillRunCheckpoint::decode(&bytes), Some(checkpoint));
    assert_eq!(FillRunCheckpoint::decode(&bytes[..bytes.len() - 1]), None);
    assert_eq!(FillRunCheckpoint::decode(&[bytes.as_slice(), &[0]].concat()), None);
    assert_eq!(FillRunCheckpoint::decode(&[]), None);
}

/// 📐️ `(min_x, min_y, max_x, max_y)` of a board node, derived here from the document JSON on its own.
fn oracle_node_rect(node: &Value) -> Rect<f64> {
    let value = |key: &str, default: f64| node[key].as_f64().filter(|value| value.is_finite()).unwrap_or(default).max(f64::EPSILON);
    let scale = value("scale", 1.0);
    let (half_x, half_y) = if node["shape"].as_str() == Some("rectangle") { (value("width", 1.0) * scale / 2.0, value("height", 1.0) * scale / 2.0) } else { (value("radius", 1.0) * scale, value("radius", 1.0) * scale) };
    let (x, y) = (node["x"].as_f64().unwrap_or(0.0), node["y"].as_f64().unwrap_or(0.0));
    Rect::new(coord! { x: x - half_x, y: y - half_y }, coord! { x: x + half_x, y: y + half_y })
}

/// 📏️ Signed overlap depth of two rectangles: positive when they overlap, negative when apart.
fn depth(left: &Rect<f64>, right: &Rect<f64>) -> f64 {
    let x = left.max().x.min(right.max().x) - left.min().x.max(right.min().x);
    let y = left.max().y.min(right.max().y) - left.min().y.max(right.min().y);
    x.min(y)
}

/// 🌍️ Third-party oracle: every collision and fit verdict of a concrete forest run is recomputed with `geo` polygon
/// intersection — the candidate's footprint (kind catalog size at the traced position) against every board node
/// (host collision) and then every placement accepted before it (virtual collision). Verdicts whose deciding
/// overlap depth lies within the traced `f32` position precision are ambiguous and excluded; no decisive verdict
/// may disagree.
#[test]
fn fill_run_job_collision_verdicts_agree_with_the_geo_oracle() {
    let law = &fixture()["geoOracle"];
    let document = example(&law["document"]);
    let log = run(&document, number(&law["run"]), number(&law["requested"]));
    let hosts: Vec<Rect<f64>> = document.0["nodes"].as_array().expect("nodes").iter().map(oracle_node_rect).collect();
    let kinds = fill_kind_rows(&document.0);
    let mut accepted: Vec<Rect<f64>> = Vec::new();
    let (mut decisive, mut ambiguous, mut disagreements, mut collisions, mut fits) = (0, 0, Vec::new(), 0, 0);
    for (key, _, reason, subject) in &log.finals {
        let Some(reason @ (FillRunReason::Fits | FillRunReason::HostCollision | FillRunReason::VirtualCollision)) = FillRunReason::from_code(*reason) else { continue };
        let ToolRunTraceSubject::Placement2d { shape, position, .. } = subject else { panic!("fill traces placement2d subjects") };
        let half = 48.0 * kinds[*shape as usize]["scale"].as_f64().filter(|scale| scale.is_finite()).unwrap_or(1.0).max(f64::EPSILON);
        let (x, y) = (f64::from(position[0]), f64::from(position[1]));
        let candidate = Rect::new(coord! { x: x - half, y: y - half }, coord! { x: x + half, y: y + half });
        let polygon = candidate.to_polygon();
        let host_hit = hosts.iter().any(|host| host.to_polygon().intersects(&polygon));
        let virtual_hit = accepted.iter().any(|placed| placed.to_polygon().intersects(&polygon));
        let tolerance = 1e-3 * (1.0 + x.abs().max(y.abs()));
        let margin = |rects: &[Rect<f64>]| rects.iter().map(|rect| depth(rect, &candidate)).fold(f64::NEG_INFINITY, f64::max);
        let oracle = if host_hit { FillRunReason::HostCollision } else if virtual_hit { FillRunReason::VirtualCollision } else { FillRunReason::Fits };
        if margin(&hosts).abs() < tolerance || margin(&accepted).abs() < tolerance {
            ambiguous += 1;
        } else {
            decisive += 1;
            if oracle != reason {
                disagreements.push(format!("{key}: ours {} geo {}", reason.id(), oracle.id()));
            }
        }
        match reason {
            FillRunReason::Fits => {
                fits += 1;
                accepted.push(candidate);
            }
            _ => collisions += 1,
        }
    }
    assert!(disagreements.is_empty(), "geo disagrees with the engine: {disagreements:?}");
    assert!(collisions > 0 && fits > 0 && decisive > ambiguous, "the oracle run must decide both collisions and fits ({decisive} decisive, {ambiguous} ambiguous)");
}

/// 👣️ With one unit of fuel per step every tick carries at most one final verdict, and exactly `tested` ticks carry one.
#[test]
fn fill_run_job_step_with_one_unit_of_fuel_reaches_exactly_one_candidate_verdict() {
    let document = example(&fixture()["resume"]["document"]);
    let mut job = fresh(&document, 1, 6);
    let mut log = RunLog::default();
    run_to_complete(&mut job, 1, &mut log);
    let tested = log.counters()[0];
    assert!(tested > 0);
    assert!(log.tick_finals.iter().all(|finals| *finals <= 1), "a single unit of fuel decided more than one candidate: {:?}", log.tick_finals);
    assert_eq!(log.tick_finals.iter().filter(|finals| **finals == 1).count() as u64, tested);
    assert_eq!(log.verdicts(), run(&document, 1, 6).verdicts(), "single-stepping decides exactly what a free run decides");
}

/// ⏩️ A job rebuilt from a run's checkpoint and provisional ops with a raised count continues the same sequence (ops,
/// verdicts and counters equal one run to the raised count, and no earlier candidate is re-announced); with a lowered
/// count it retracts the provisional tail to exactly the kept placements, retires their success records and completes.
#[test]
fn fill_run_job_resume_raise_continues_the_sequence_and_lower_retracts_the_tail() {
    let law = &fixture()["resume"];
    let document = example(&law["document"]);
    let (run_id, first, raise, lower) = (number(&law["run"]), number(&law["first"]), number(&law["raise"]), number(&law["lower"]));
    let base = run(&document, run_id, first);
    let checkpoint = base.checkpoints.last().expect("base checkpoint").clone();
    let decoded_checkpoint = FillRunCheckpoint::decode(&checkpoint).expect("base checkpoint decodes");
    let provisional = decoded(&base.ops);

    let mut raised_job = Puzzle2dFillRunJob::new(identity(run_id, 1), Arc::clone(&document), PUZZLE2D_DEFAULT_SUGGESTION_OFFSET, raise as u32, Some(&checkpoint), &provisional).expect("raised resume");
    let mut raised = RunLog::continuing(&base.ops);
    run_to_complete(&mut raised_job, INTERACTIVE_LANE_FUEL, &mut raised);
    let direct = run(&document, run_id, raise);
    assert_eq!(raised.ops, direct.ops, "a raise continues the deterministic sequence");
    assert!(raised.retracts.is_empty());
    let mut verdicts = base.verdicts();
    verdicts.extend(raised.verdicts());
    assert_eq!(verdicts, direct.verdicts());
    assert_eq!(raised.counters(), direct.counters());
    assert!(raised.testing.iter().all(|key| *key >= decoded_checkpoint.next_key), "a raise never re-announces a candidate the base run already decided");

    let mut lowered_job = Puzzle2dFillRunJob::new(identity(run_id, 2), Arc::clone(&document), PUZZLE2D_DEFAULT_SUGGESTION_OFFSET, lower as u32, Some(&checkpoint), &provisional).expect("lowered resume");
    let mut lowered = RunLog::continuing(&base.ops);
    run_to_complete(&mut lowered_job, INTERACTIVE_LANE_FUEL, &mut lowered);
    let kept = (lower as usize) * FILL_RUN_OPS_PER_PLACEMENT;
    assert_eq!(lowered.retracts, vec![kept as u32]);
    assert_eq!(lowered.ops, base.ops[..kept].to_vec());
    assert_eq!(lowered.retired, decoded_checkpoint.placements[lower as usize..].iter().map(|placement| placement.key).collect::<Vec<_>>());
    assert!(lowered.finals.is_empty() && lowered.testing.is_empty(), "a lower decides nothing new");
    assert!(lowered.steps.iter().any(|step| step.kind == ToolRunStepKind::Info && step.reason == FillRunReason::Retracted.code() && step.args == vec![ToolRunStepArg::Unsigned(lower)]));
    let progress = lowered.progress.as_ref().expect("lowered progress");
    assert_eq!((progress.state, progress.completed, progress.total), (ToolRunState::Complete, lower, Some(lower)));
}

/// 🔍️ Revalidation against an unchanged head keeps every placement; against a head where a node now sits on one
/// placement it marks exactly that placement `danger`, retracts to it and re-appends every later survivor.
#[test]
fn fill_revalidate_job_retracts_conflicting_placements_and_reappends_survivors() {
    let law = &fixture()["revalidate"];
    let document = example(&law["document"]);
    let base = run(&document, number(&law["run"]), number(&law["requested"]));
    let checkpoint = base.checkpoints.last().expect("checkpoint").clone();
    let keys = FillRunCheckpoint::decode(&checkpoint).expect("checkpoint decodes").placements;
    let provisional = decoded(&base.ops);

    let mut clean_job = Puzzle2dFillRevalidateJob::new(identity(1, 3), Arc::clone(&document), &provisional, Some(&checkpoint), 0.0);
    let mut clean = RunLog::continuing(&base.ops);
    run_to_complete(&mut clean_job, INTERACTIVE_LANE_FUEL, &mut clean);
    assert!(clean.retracts.is_empty() && clean.steps.is_empty());
    assert_eq!(clean.ops, base.ops);
    assert_eq!(clean.finals.iter().map(|(key, verdict, ..)| (*key, *verdict)).collect::<Vec<_>>(), keys.iter().map(|placement| (placement.key, ToolRunVerdict::Success)).collect::<Vec<_>>());

    let conflicted = number(&law["conflicted"]) as usize;
    let Puzzle2dMutation::CreateNode(create) = &provisional[conflicted * FILL_RUN_OPS_PER_PLACEMENT] else { panic!("create_node") };
    let mut head = document.0.clone();
    head["nodes"].as_array_mut().expect("nodes").push(json!({ "id": "intruder", "shape": "circle", "x": create.node.x, "y": create.node.y, "radius": 1.0, "handles": [] }));
    let mut conflict_job = Puzzle2dFillRevalidateJob::new(identity(1, 3), Arc::new(Puzzle2dPlaySnapshot(head)), &provisional, Some(&checkpoint), 0.0);
    let mut conflict = RunLog::continuing(&base.ops);
    run_to_complete(&mut conflict_job, INTERACTIVE_LANE_FUEL, &mut conflict);
    let kept = conflicted * FILL_RUN_OPS_PER_PLACEMENT;
    assert_eq!(conflict.retracts, vec![kept as u32]);
    assert_eq!(conflict.ops, [&base.ops[..kept], &base.ops[kept + FILL_RUN_OPS_PER_PLACEMENT..]].concat());
    assert_eq!(conflict.entities[conflicted..], base.entities[conflicted + 1..]);
    let dangers: Vec<u64> = conflict.finals.iter().filter(|(_, verdict, ..)| *verdict == ToolRunVerdict::Danger).map(|(key, ..)| *key).collect();
    assert_eq!(dangers, vec![keys[conflicted].key]);
    assert!(conflict.steps.iter().any(|step| step.kind == ToolRunStepKind::Danger && step.reason == TOOL_RUN_REASON_CONFLICT && step.args == vec![ToolRunStepArg::Unsigned(1)]));
}

thread_local! {
    static REPLAY_CLOCK: std::cell::RefCell<(Vec<u64>, Option<usize>)> = const { std::cell::RefCell::new((Vec::new(), None)) };
}

/// 🕰️ Run 1 reads and records the real clock; later runs replay exactly those readings, so every cold run slices
/// the job at the same points and per-turn timings are comparable.
fn replay_clock() -> Option<u64> {
    REPLAY_CLOCK.with(|clock| {
        let mut clock = clock.borrow_mut();
        match clock.1 {
            None => {
                let now = semio_framework_job::default_now_us()?;
                clock.0.push(now);
                Some(now)
            }
            Some(cursor) => {
                clock.1 = Some(cursor + 1);
                clock.0.get(cursor).copied().or_else(|| clock.0.last().copied())
            }
        }
    })
}

/// ⏱️ Interactive ceiling on the largest 2d example: every `drive_step` of the Nakagin fill run job stays below the
/// 2 ms budget, per turn taking the best of several cold runs that replay one clock so load spikes cannot pose
/// as job cost.
#[test]
fn fill_run_job_drive_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let law = &fixture()["interactive"];
    let document = example(&law["document"]);
    let mut best: Vec<u128> = Vec::new();
    for cold in 0..number(&law["coldRuns"]) {
        REPLAY_CLOCK.with(|clock| clock.borrow_mut().1 = (cold > 0).then_some(0));
        let mut job = fresh(&document, number(&law["run"]), number(&law["requested"]));
        let (mut log, mut preview_sequence, mut turn) = (RunLog::default(), 0_u64, 0);
        while !log.complete {
            let start = replay_clock().expect("clock");
            let budget = StepBudget::from_duration(INTERACTIVE_LANE_FUEL, start, INTERACTIVE_LANE_WALL_US).expect("budget");
            let mut verdict = None;
            let began = std::time::Instant::now();
            let mut outcome = drive_step(&mut job, "puzzle2d.fill.run.test", OperationId(1), Generation(0), InteractiveStage::InteractiveStep, budget, semio_framework_job::root_cancel_token(), replay_clock, &mut preview_sequence, &mut verdict);
            let elapsed = began.elapsed().as_micros();
            let failure = match &outcome {
                StepOutcome::PreviewReady(payload) => ToolRunTick::decode(&payload_bytes(payload)).map(|tick| log.apply(tick)).err().map(|error| format!("{error:?}")),
                StepOutcome::Complete(_) => {
                    log.complete = true;
                    None
                }
                StepOutcome::Fault(fault) => Some(String::from_utf8_lossy(&payload_bytes(&fault.detail)).into_owned()),
                _ => None,
            };
            close_outcome(&mut outcome);
            if let Some(failure) = failure {
                close_job(&mut job);
                panic!("interactive fill run failed: {failure}");
            }
            if cold == 0 {
                best.push(elapsed);
            } else if let Some(slot) = best.get_mut(turn) {
                *slot = (*slot).min(elapsed);
            }
            turn += 1;
        }
        close_job(&mut job);
        assert_eq!(turn, best.len(), "cold run {cold} sliced the job differently");
    }
    let worst = best.iter().copied().max().unwrap_or(0);
    assert!(best.len() as u64 >= number(&law["minimumTurns"]), "only {} turns", best.len());
    assert!(worst < u128::from(number(&law["budgetUs"])), "worst drive_step {worst} µs");
}

fn tool_run_args(entries: &[(&str, DslValue)]) -> DslValue {
    DslValue::Object(entries.iter().map(|(key, value)| ((*key).to_string(), value.clone())).collect())
}

fn tool_run_action(app: &mut Puzzle2dApp, action: &str, entries: &[(&str, DslValue)]) -> DslValue {
    semio_framework::io::resolve_ready(app.handle_action(action, Some(&tool_run_args(entries)), &meta("local"))).unwrap_or_else(|fault| panic!("{action}: {fault:?}")).output
}

fn run_state(app: &Puzzle2dApp) -> Option<&'static str> {
    app.tool_run_presence().map(|presence| presence.state.wire_name())
}

/// 🔁️ Host continuation turns (driver turn, result page acknowledgement, outboxes) until `done` holds.
fn pump_until(app: &mut Puzzle2dApp, what: &str, done: impl Fn(&Puzzle2dApp) -> bool) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    while std::time::Instant::now() < deadline {
        if done(app) {
            return;
        }
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).unwrap_or_else(|fault| panic!("{what}: maintenance faulted: {fault:?}"));
        semio_framework::io::resolve_ready(app.advance_typed_operation_publication()).unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
        if let Some(page) = app.take_typed_operation_result_page(1) {
            assert_ne!(page.lane, semio_framework_plugin::app::TypedOperationResultLane::Fault, "{what}: typed operation faulted: {}", String::from_utf8_lossy(page.bytes()));
            app.acknowledge_typed_operation_result(page.token).expect("acknowledge result page");
        }
        let _ = app.take_typed_operation_effect();
        let _ = app.take_typed_operation_event();
        let _ = semio_framework::io::resolve_ready(app.take_typed_operation_completion()).expect("completion");
        let _ = app.take_typed_operation_ui_scope();
    }
    panic!("{what} never settled; state {:?}", run_state(app));
}

fn history_len(app: &mut Puzzle2dApp) -> usize {
    semio_framework::io::resolve_ready(app.history_snapshot()).expect("history").upserts.len()
}

fn document_pack(app: &Puzzle2dApp) -> store::ArtifactPackFiles {
    semio_framework::io::resolve_ready(app.document_pack()).expect("document pack")
}

/// 🧰️ An app holding the fixture `document` spec's example reduced to its first `keepNodes` nodes — the removed
/// nodes deleted by ordinary local `delete-node` edits — with the fill count set in config.
fn fill_app(spec: &Value, requested: u64) -> Puzzle2dApp {
    let mut app = app_with_registry();
    load_example(&mut app, text(&spec["example"]));
    let keep = number(&spec["keepNodes"]) as usize;
    let deletes: Vec<String> = fixture_nodes(&fixture_of(&app)).iter().skip(keep).filter_map(|node| node["id"].as_str()).map(|id| protocol::OpText::print_op(&crate::standards::v1::subsets::any::schema::mutations::delete_node(id.to_string()))).collect();
    semio_framework::io::resolve_ready(app.ingest_operations_text(&deletes.join("\n"))).expect("delete the nodes past keepNodes");
    assert_eq!(fixture_of(&app), example(spec).0, "the app commits exactly the fixture document");
    dispatch(&mut app, "setFillCount", Some(&json!({ "count": requested })), None).expect("set fill count");
    app
}

/// 🏁️ start → complete → finalize: the committed document never moves while the run holds provisional placements,
/// finalize publishes them as exactly one history entry, and one undo removes every placed node.
#[test]
fn fill_run_start_complete_finalize_is_one_undo_entry() {
    let law = &fixture()["finalize"];
    let requested = number(&law["requested"]);
    let mut app = fill_app(&law["document"], requested);
    let before = fixture_of(&app);
    let (nodes, history) = (fixture_nodes(&before).len(), history_len(&mut app));
    assert_eq!(tool_run_action(&mut app, "toolRunStart", &[("toolId", DslValue::String(fill::TOOL_ID.into()))]).get("toolRun").and_then(DslValue::as_str), Some("spawnJob"));
    pump_until(&mut app, "fill run completes", |app| run_state(app) == Some("complete"));
    assert_eq!(app.tool_run_presence().map(|presence| (presence.completed, presence.total)), Some((requested, Some(requested))));
    assert_eq!(fixture_of(&app), before, "a complete run has committed nothing");
    assert_eq!(history_len(&mut app), history);
    assert!(app.tool_run_trace_delta(None).is_some_and(|delta| !delta.is_empty()), "the run published trace pages");
    let run = [("runId", DslValue::String("1".into())), ("generation", DslValue::String("0".into()))];
    assert_eq!(tool_run_action(&mut app, "toolRunFinalize", &run).get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"));
    pump_until(&mut app, "fill run finalizes", |app| run_state(app) == Some("finalized"));
    assert_eq!(fixture_nodes(&fixture_of(&app)).len(), nodes + requested as usize);
    assert_eq!((history_len(&mut app) - history) as u64, number(&law["historyEntriesAdded"]));
    dispatch(&mut app, "undo", None, None).expect("undo the finalized fill run");
    assert_eq!(fixture_nodes(&fixture_of(&app)).len(), nodes, "one undo removes every placement");
    close_app(&mut app);
}

/// 🪣️ The browser battery's own fill: 100 requested placements on concrete-forest. However far the
/// plan reaches, the finalize is exactly ONE history entry — one of the 64 edit-ledger slots — one
/// undo restores the pre-fill document byte-for-byte and one redo re-applies the whole run
/// (2026-09-17 battery: `20-history/undo-changes-document` never moved the document after a fill).
#[test]
fn a_hundred_placement_fill_is_one_history_entry_that_undoes_and_redoes() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    dispatch(&mut app, "setFillCount", Some(&json!({ "count": 100 })), None).expect("set fill count");
    let before = fixture_of(&app);
    let (nodes, history) = (fixture_nodes(&before).len(), history_len(&mut app));
    tool_run_action(&mut app, "toolRunStart", &[("toolId", DslValue::String(fill::TOOL_ID.into()))]);
    pump_until(&mut app, "hundred-placement fill run completes", |app| run_state(app) == Some("complete"));
    let run = [("runId", DslValue::String("1".into())), ("generation", DslValue::String("0".into()))];
    tool_run_action(&mut app, "toolRunFinalize", &run);
    pump_until(&mut app, "hundred-placement fill run finalizes", |app| run_state(app) == Some("finalized"));
    let filled = fixture_of(&app);
    let placed = fixture_nodes(&filled).len();
    assert!(placed > nodes, "the run must place at least one node, placed {placed} from {nodes}");
    assert_eq!(history_len(&mut app) - history, 1, "a whole fill run costs the history exactly one entry");
    dispatch(&mut app, "undo", None, None).expect("undo the finalized fill run");
    assert_eq!(fixture_of(&app), before, "one undo restores the pre-fill document");
    dispatch(&mut app, "redo", None, None).expect("redo the finalized fill run");
    assert_eq!(fixture_of(&app), filled, "one redo re-applies the whole run");
    close_app(&mut app);
}

/// 🛑️ Abort after provisional placements exist leaves the document pack byte-identical and the history untouched.
#[test]
fn fill_run_abort_leaves_the_document_byte_identical() {
    let law = &fixture()["finalize"];
    let mut app = fill_app(&law["document"], number(&law["requested"]));
    let (pack, history) = (document_pack(&app), history_len(&mut app));
    tool_run_action(&mut app, "toolRunStart", &[("toolId", DslValue::String(fill::TOOL_ID.into()))]);
    let placements = number(&law["abortAfterPlacements"]);
    pump_until(&mut app, "provisional placements exist", |app| app.tool_run_presence().is_some_and(|presence| presence.completed >= placements));
    let run = [("runId", DslValue::String("1".into())), ("generation", DslValue::String("0".into()))];
    assert_eq!(tool_run_action(&mut app, "toolRunAbort", &run).get("toolRun").and_then(DslValue::as_str), Some("closeJob"));
    pump_until(&mut app, "abort settles", |app| run_state(app) == Some("aborted"));
    let after = document_pack(&app);
    assert_eq!((after.pack, after.spr), (pack.pack, pack.spr), "abort leaves the document byte-identical");
    assert_eq!(history_len(&mut app), history);
    close_app(&mut app);
}

//#region 🚧️PlacementSlack
/// 🚧️ LAW: the collision test is widened by the net of the two placement-tuning settings. Two
/// footprints one world unit apart do NOT collide at zero slack; a contact tolerance of one unit per
/// side closes the gap and they do; an overlap budget of one unit re-opens it. A budget can never
/// shrink a footprint past its own centre, so genuinely coincident footprints always collide.
#[test]
fn the_fill_collision_test_reads_the_placement_tuning_slack() {
    let left = fill_node_bounds(0.0, 0.0, Some(1.0), true, Some(2.0), Some(2.0));
    let right = fill_node_bounds(3.0, 0.0, Some(1.0), true, Some(2.0), Some(2.0));
    assert!(!fill_bounds_overlap_with(left, right, 0.0), "a one-unit gap is clear at zero slack");
    assert!(fill_bounds_overlap_with(left, right, 0.5), "a contact tolerance of half a unit per side closes a one-unit gap");
    assert!(!fill_bounds_overlap_with(left, right, -1.0), "an overlap budget re-opens the gap");
    let coincident = fill_node_bounds(0.0, 0.0, Some(1.0), true, Some(2.0), Some(2.0));
    assert!(fill_bounds_overlap_with(left, coincident, -1_000.0), "no budget may shrink a footprint past its centre");
    assert!(fill_bounds_overlap_with(left, left, 0.0), "a footprint always collides with itself at zero slack");
}
//#endregion 🚧️PlacementSlack
