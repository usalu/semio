//! 🔬️ Layout run laws over the language-neutral fixture: deterministic convergence, slicing independence, one
//! iteration per fuel unit, tick page bounds, compaction to one op per moved node, checkpoint resume, Barnes-Hut
//! accuracy, the step bound on a 5 000-node grid, close and cancellation.

use super::*;
use semio_framework_job::{default_now_us, drive_step, root_cancel_token, CancelToken, Generation, InteractiveStage, OperationId, StepBudget, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_US};
use semio_framework_tool_run::{ToolRunId, ToolRunStep, ToolRunTick, ToolRunTraceOp, ToolRunTraceStore};
use serde_json::Value;
use std::collections::BTreeSet;

pub(crate) const FIXTURE: &str = include_str!("../../🧫️fixtures/🎞️layout-run.json");
const SCHEMA: &str = include_str!("../../🧬️schema/🔣️.json");

pub(crate) type TestEncoder = fn(u32, u64, LayoutRunPoint) -> Result<Vec<u8>, LayoutRunEncodeError>;

#[expect(clippy::unnecessary_wraps, reason = "the signature is the `LayoutRunOpEncoder` closure shape")]
pub(crate) fn encode_move(node: u32, _entity: u64, position: LayoutRunPoint) -> Result<Vec<u8>, LayoutRunEncodeError> {
    let mut bytes = Vec::with_capacity(20);
    bytes.extend_from_slice(&node.to_le_bytes());
    bytes.extend_from_slice(&position.x.to_bits().to_le_bytes());
    bytes.extend_from_slice(&position.y.to_bits().to_le_bytes());
    Ok(bytes)
}

fn decode_move(bytes: &[u8]) -> (u32, [f64; 2]) {
    assert_eq!(bytes.len(), 20, "test move ops are 20 bytes");
    let node = u32::from_le_bytes(bytes[0..4].try_into().expect("node"));
    let x = f64::from_bits(u64::from_le_bytes(bytes[4..12].try_into().expect("x")));
    let y = f64::from_bits(u64::from_le_bytes(bytes[12..20].try_into().expect("y")));
    (node, [x, y])
}

pub(crate) fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("fixture parses")
}

pub(crate) fn identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 7, run: 1 }, [0; 32])
}

pub(crate) fn generate(spec: &Value) -> LayoutRunGraph {
    let radius = spec["radius"].as_f64().expect("radius");
    let count = match spec["kind"].as_str().expect("kind") {
        "grid" => spec["columns"].as_u64().expect("columns") * spec["rows"].as_u64().expect("rows"),
        _ => spec["count"].as_u64().expect("count"),
    } as u32;
    let mut nodes: Vec<LayoutRunNode> = (0..count).map(|index| LayoutRunNode { entity: u64::from(index) + 1, origin: None, radius, pinned: false, anchor: None }).collect();
    let edge = |source: u32, target: u32| LayoutRunEdge { source, target, weight: 1.0 };
    let edges: Vec<LayoutRunEdge> = match spec["kind"].as_str().expect("kind") {
        "grid" => {
            let columns = spec["columns"].as_u64().expect("columns") as u32;
            let rows = spec["rows"].as_u64().expect("rows") as u32;
            let mut edges = Vec::new();
            for row in 0..rows {
                for column in 0..columns {
                    let index = row * columns + column;
                    if column + 1 < columns {
                        edges.push(edge(index, index + 1));
                    }
                    if row + 1 < rows {
                        edges.push(edge(index, index + columns));
                    }
                }
            }
            edges
        }
        "ring" => (0..count).map(|index| edge(index, (index + 1) % count)).collect(),
        "tree" => (0..count).flat_map(|index| [2 * index + 1, 2 * index + 2].into_iter().filter(move |child| *child < count).map(move |child| edge(index, child))).collect(),
        kind => panic!("unknown generator {kind}"),
    };
    for pin in spec["pins"].as_array().expect("pins") {
        let node = &mut nodes[pin["node"].as_u64().expect("pin node") as usize];
        node.pinned = true;
        node.origin = Some(LayoutRunPoint::new(pin["at"]["x"].as_f64().expect("x"), pin["at"]["y"].as_f64().expect("y")));
    }
    LayoutRunGraph { nodes, edges }
}

pub(crate) fn case_graph(case: &Value) -> LayoutRunGraph {
    match case.get("generator") {
        Some(generator) => generate(generator),
        None => serde_json::from_value(case["graph"].clone()).expect("graph parses"),
    }
}

pub(crate) fn case_config(case: &Value) -> LayoutRunConfig {
    serde_json::from_value(case["config"].clone()).expect("config parses")
}

#[derive(Default)]
pub(crate) struct Ledger {
    pub(crate) ops: Vec<(u32, [f64; 2])>,
    pub(crate) entity_marks: Vec<(u32, u64)>,
    pub(crate) trace: Option<ToolRunTraceStore>,
    pub(crate) upserts: Vec<String>,
    pub(crate) steps: Vec<ToolRunStep>,
    pub(crate) progress: Vec<(u16, u64, ToolRunState)>,
    pub(crate) checkpoints: Vec<Vec<u8>>,
    pub(crate) max_tick_bytes: usize,
    pub(crate) retracts: u32,
    pub(crate) iteration_ticks: u32,
}

impl Ledger {
    fn apply(&mut self, bytes: &[u8]) {
        self.max_tick_bytes = self.max_tick_bytes.max(bytes.len());
        let tick = ToolRunTick::decode(bytes).expect("tick decodes");
        if let Some(length) = tick.retract_to {
            self.retracts += 1;
            self.ops.truncate(length as usize);
            self.entity_marks.retain(|(mark, _)| *mark <= length);
        }
        let iteration_tick = tick.progress.as_ref().is_some_and(|progress| progress.stage == LayoutRunStage::Iterate.index() && self.progress.last().is_some_and(|(_, completed, _)| progress.completed == completed + 1));
        if iteration_tick {
            self.iteration_ticks += 1;
            let upserted: BTreeSet<u64> = tick.trace.iter().flat_map(|page| page.ops.iter()).filter_map(|op| if let ToolRunTraceOp::Upsert { key, .. } = op { Some(*key) } else { None }).collect();
            assert!(tick.append_ops.iter().all(|op| upserted.contains(&u64::from(decode_move(op).0))), "every position update of an iteration tick carries its node's trace record");
        }
        self.ops.extend(tick.append_ops.iter().map(|op| decode_move(op)));
        let end = self.ops.len() as u32;
        self.entity_marks.extend(tick.append_entities.iter().map(|entity| (end, *entity)));
        let store = self.trace.get_or_insert_with(|| ToolRunTraceStore::new(tick.identity));
        for page in &tick.trace {
            store.apply_page(page).expect("page belongs to the run");
            for op in &page.ops {
                if let ToolRunTraceOp::Upsert { key, verdict, reason, subject } = op {
                    assert!(matches!(subject, ToolRunTraceSubject::Entity { .. }), "layout trace subjects are entities");
                    if self.upserts.len() < 24 && tick.progress.as_ref().is_some_and(|progress| progress.stage != LayoutRunStage::Initialize.index()) {
                        self.upserts.push(format!("{key}:{}/{}", verdict_id(*verdict), LayoutRunReason::from_code(*reason).expect("reason").id()));
                    }
                }
            }
        }
        self.steps.extend(tick.steps);
        if let Some(progress) = tick.progress {
            self.progress.push((progress.stage, progress.completed, progress.state));
        }
    }

    fn overlay(&self, graph: &LayoutRunGraph) -> Vec<Option<[f64; 2]>> {
        let mut positions: Vec<Option<[f64; 2]>> = graph.nodes.iter().map(|node| node.origin.map(|origin| [origin.x, origin.y])).collect();
        for (node, position) in &self.ops {
            positions[*node as usize] = Some(*position);
        }
        positions
    }
}

fn verdict_id(verdict: ToolRunVerdict) -> &'static str {
    match verdict {
        ToolRunVerdict::Testing => "testing",
        ToolRunVerdict::Success => "success",
        ToolRunVerdict::Warning => "warning",
        ToolRunVerdict::Danger => "danger",
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Driven {
    Complete,
    Checkpoint,
}

#[expect(clippy::unnecessary_wraps, reason = "the signature is the `StepContext` clock shape")]
fn frozen_now() -> Option<u64> {
    Some(0)
}

fn close_outcome(outcome: &mut StepOutcome) {
    while outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES) != semio_framework_job::JobPayloadCloseStep::Complete {}
}

/// 🎛️ One direct `step` call with `fuel` and a frozen clock whose deadline is `deadline_us` (`u64::MAX` = never).
fn step_once(job: &mut LayoutRunJob<TestEncoder>, ledger: &mut Ledger, fuel: u64, deadline_us: u64, cancel: &CancelToken) -> Option<Driven> {
    let mut sequence = 0;
    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(fuel, deadline_us), cancel.clone(), frozen_now, &mut sequence);
    let mut outcome = job.step(&mut cx);
    let driven = observe(&mut outcome, ledger);
    close_outcome(&mut outcome);
    driven
}

fn observe(outcome: &mut StepOutcome, ledger: &mut Ledger) -> Option<Driven> {
    match outcome {
        StepOutcome::PreviewReady(payload) => {
            assert_eq!(payload.page_count(), 1, "one tick is one payload page");
            let bytes = payload.page(0).expect("page").to_vec();
            ledger.apply(&bytes);
            None
        }
        StepOutcome::CheckpointReady(checkpoint) => {
            ledger.checkpoints.push(checkpoint.state.page(0).expect("checkpoint page").to_vec());
            Some(Driven::Checkpoint)
        }
        StepOutcome::Complete(_) => Some(Driven::Complete),
        StepOutcome::Yield => None,
        StepOutcome::Cancelled => panic!("unexpected cancellation"),
        StepOutcome::Fault(fault) => panic!("layout run faulted: {}", String::from_utf8_lossy(fault.detail.page(0).unwrap_or_default())),
    }
}

pub(crate) fn run_to(job: &mut LayoutRunJob<TestEncoder>, ledger: &mut Ledger, fuel: u64, until: Driven) {
    let cancel = root_cancel_token();
    for _ in 0..10_000_000 {
        match step_once(job, ledger, fuel, u64::MAX, &cancel) {
            Some(Driven::Complete) => return,
            Some(Driven::Checkpoint) if until == Driven::Checkpoint => return,
            _ => {}
        }
    }
    panic!("layout run never reached {until:?}");
}

pub(crate) fn new_job(graph: &LayoutRunGraph, config: LayoutRunConfig) -> LayoutRunJob<TestEncoder> {
    LayoutRunJob::new(identity(), graph, config, encode_move as TestEncoder).expect("valid run")
}

fn hex16(value: u64) -> String {
    format!("{value:016x}")
}

fn assert_run_laws(case: &str, graph: &LayoutRunGraph, job: &LayoutRunJob<TestEncoder>, ledger: &Ledger) {
    let positions = job.positions();
    assert!(ledger.max_tick_bytes <= JOB_PAYLOAD_PAGE_BYTES, "{case}: tick of {} bytes exceeds one page", ledger.max_tick_bytes);
    let moved: Vec<usize> = (0..graph.nodes.len()).filter(|index| job.moved(*index)).collect();
    assert_eq!(ledger.ops.len(), moved.len(), "{case}: compaction leaves exactly one op per moved node");
    let op_nodes: BTreeSet<u32> = ledger.ops.iter().map(|(node, _)| *node).collect();
    assert_eq!(op_nodes, moved.iter().map(|index| *index as u32).collect(), "{case}: ops cover exactly the moved nodes");
    let overlay = ledger.overlay(graph);
    for (index, position) in positions.iter().enumerate() {
        assert_eq!(overlay[index], Some([position.x, position.y]), "{case}: overlay of node {index} equals the job's final position");
    }
    let entities: BTreeSet<u64> = ledger.entity_marks.iter().map(|(_, entity)| *entity).collect();
    assert_eq!(entities, moved.iter().map(|index| graph.nodes[*index].entity).collect(), "{case}: provisional entities are the moved nodes");
    let trace = ledger.trace.as_ref().expect("trace");
    assert_eq!(trace.len(), graph.nodes.len(), "{case}: one resident trace record per node");
    for (key, record) in trace.records() {
        let reason = job.final_reason(key as usize);
        assert_eq!((record.verdict, record.reason), (reason.verdict(), reason.code()), "{case}: final trace record of node {key}");
        assert_eq!(record.subject, ToolRunTraceSubject::Entity { entity: graph.nodes[key as usize].entity });
    }
    let (stage, completed, state) = *ledger.progress.last().expect("progress");
    assert_eq!((stage, completed, state), (LayoutRunStage::Settle.index(), u64::from(job.iteration()), ToolRunState::Complete), "{case}: last progress");
    let last = ledger.steps.last().expect("final step");
    let expected = match job.stop().expect("stopped") {
        LayoutRunStop::Converged => (ToolRunStepKind::Success, LayoutRunReason::Converged),
        LayoutRunStop::IterationLimit => (ToolRunStepKind::Warning, LayoutRunReason::IterationLimit),
    };
    assert_eq!((last.kind, last.reason), (expected.0, expected.1.code()), "{case}: final step");
    assert_eq!(ledger.checkpoints.len() as u32, job.checkpoints(), "{case}: every compaction reports one checkpoint");
    assert_eq!(u64::from(ledger.iteration_ticks), u64::from(job.iteration()) - ledger.progress.first().map_or(0, |(_, completed, _)| *completed), "{case}: one iteration tick per iteration");
    assert_eq!(ledger.retracts, job.checkpoints() + 1, "{case}: every checkpoint compaction and the settle compaction retract the provisional list once");
}

fn actual_expectation(job: &LayoutRunJob<TestEncoder>, ledger: &Ledger) -> Value {
    serde_json::json!({
        "stop": match job.stop().expect("stopped") { LayoutRunStop::Converged => "converged", LayoutRunStop::IterationLimit => "iterationLimit" },
        "iterations": job.iteration(),
        "positionsDigest": hex16(job.positions_digest()),
        "movedOps": ledger.ops.len(),
        "settled": job.counters()[3],
        "checkpoints": job.checkpoints(),
        "verdictPrefix": ledger.upserts,
    })
}

#[test]
fn layout_run_converges_deterministically_to_the_language_neutral_fixture() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let graph = case_graph(case);
        let config = case_config(case);
        let mut job = new_job(&graph, config);
        let mut ledger = Ledger::default();
        run_to(&mut job, &mut ledger, INTERACTIVE_LANE_FUEL, Driven::Complete);
        assert_run_laws(id, &graph, &job, &ledger);
        let actual = actual_expectation(&job, &ledger);
        assert_eq!(actual, case["expect"], "{id}: fixture expectation (actual on the left)");
        let mut again = new_job(&graph, config);
        let mut again_ledger = Ledger::default();
        run_to(&mut again, &mut again_ledger, INTERACTIVE_LANE_FUEL, Driven::Complete);
        assert_eq!(again.positions_digest(), job.positions_digest(), "{id}: the same seed converges to the same bits");
        assert_eq!(again_ledger.ops, ledger.ops, "{id}: the same seed publishes the same ops");
    }
}

#[test]
fn layout_run_step_with_one_unit_of_fuel_advances_exactly_one_iteration_per_iterate_tick() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let graph = case_graph(case);
        let config = case_config(case);
        let mut reference = new_job(&graph, config);
        run_to(&mut reference, &mut Ledger::default(), INTERACTIVE_LANE_FUEL, Driven::Complete);
        let mut job = new_job(&graph, config);
        let mut ledger = Ledger::default();
        run_to(&mut job, &mut ledger, 1, Driven::Complete);
        assert_eq!(job.positions_digest(), reference.positions_digest(), "{id}: fuel does not change the layout");
        let iterate: Vec<u64> = ledger.progress.iter().filter(|(stage, _, _)| *stage == LayoutRunStage::Iterate.index()).map(|(_, completed, _)| *completed).collect();
        let mut previous = 0;
        for completed in iterate {
            assert!(completed == previous || completed == previous + 1, "{id}: an iterate tick advances at most one iteration ({previous} → {completed})");
            previous = completed;
        }
        let distinct: BTreeSet<u64> = ledger.progress.iter().map(|(_, completed, _)| *completed).collect();
        assert!((1..=u64::from(job.iteration())).all(|iteration| distinct.contains(&iteration)), "{id}: every iteration reports its own tick");
    }
}

#[test]
fn layout_run_is_independent_of_deadline_slicing() {
    let fixture = fixture();
    let case = &fixture["cases"][0];
    let graph = case_graph(case);
    let config = LayoutRunConfig { pairwise_max_bodies: 0, ..case_config(case) };
    let mut reference = new_job(&graph, config);
    run_to(&mut reference, &mut Ledger::default(), INTERACTIVE_LANE_FUEL, Driven::Complete);
    let mut job = new_job(&graph, config);
    let mut ledger = Ledger::default();
    let cancel = root_cancel_token();
    let mut yields = 0;
    loop {
        let mut sequence = 0;
        let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(INTERACTIVE_LANE_FUEL, 0), cancel.clone(), frozen_now, &mut sequence);
        let mut outcome = job.step(&mut cx);
        if outcome == StepOutcome::Yield {
            yields += 1;
        }
        let driven = observe(&mut outcome, &mut ledger);
        close_outcome(&mut outcome);
        if driven == Some(Driven::Complete) {
            break;
        }
    }
    assert!(yields > 0, "an expired deadline slices iterations into several steps");
    assert_eq!(job.positions_digest(), reference.positions_digest(), "slicing by deadline does not change the layout");
}

#[test]
fn layout_run_resumes_from_its_checkpoint_to_the_same_layout_and_refuses_foreign_checkpoints() {
    let fixture = fixture();
    let case = fixture["cases"].as_array().expect("cases").iter().find(|case| case["id"] == "ring-24-seed-3").expect("ring case");
    let graph = case_graph(case);
    let config = case_config(case);
    let mut reference = new_job(&graph, config);
    let mut reference_ledger = Ledger::default();
    run_to(&mut reference, &mut reference_ledger, INTERACTIVE_LANE_FUEL, Driven::Complete);
    assert!(reference.checkpoints() >= 2, "the ring compacts more than once");
    let mut first = new_job(&graph, config);
    let mut ledger = Ledger::default();
    run_to(&mut first, &mut ledger, INTERACTIVE_LANE_FUEL, Driven::Checkpoint);
    let checkpoint = ledger.checkpoints.last().expect("checkpoint").clone();
    assert_eq!(checkpoint.len(), LayoutRunCheckpoint::BYTES);
    let decoded = LayoutRunCheckpoint::decode(&checkpoint).expect("checkpoint decodes");
    assert_eq!(decoded.provisional_len as usize, ledger.ops.len(), "the checkpoint follows the compaction");
    let positions = first.positions();
    let provisional = ledger.ops.len() as u32;
    let checkpoint_ops = ledger.ops.clone();
    let mut resumed = LayoutRunJob::resume(identity(), &graph, config, encode_move as TestEncoder, &positions, &checkpoint, provisional).expect("own checkpoint resumes");
    run_to(&mut resumed, &mut ledger, INTERACTIVE_LANE_FUEL, Driven::Complete);
    assert_eq!(resumed.iteration(), reference.iteration(), "resume continues the iteration count");
    assert_eq!(resumed.positions_digest(), reference.positions_digest(), "resume from the checkpoint reproduces the uninterrupted layout");
    assert_eq!(ledger.ops, reference_ledger.ops, "resume publishes the same final ops");
    assert!(ledger.steps.iter().any(|step| step.reason == LayoutRunReason::Resumed.code()), "resume reports a resumed step");
    let mut other = graph.clone();
    other.edges.pop();
    let resume = |graph: &LayoutRunGraph, bytes: &[u8], provisional: u32| LayoutRunJob::resume(identity(), graph, config, encode_move as TestEncoder, &positions, bytes, provisional).err();
    assert_eq!(resume(&other, &checkpoint, provisional), Some(LayoutRunResumeError::Foreign));
    assert_eq!(resume(&graph, &checkpoint, provisional.saturating_sub(1)), Some(LayoutRunResumeError::Foreign));
    assert_eq!(resume(&graph, &checkpoint[..43], provisional), Some(LayoutRunResumeError::Malformed));
    let mut wrong_magic = checkpoint.clone();
    wrong_magic[0] ^= 1;
    assert_eq!(resume(&graph, &wrong_magic, provisional), Some(LayoutRunResumeError::Malformed));
    assert_eq!(LayoutRunJob::resume(identity(), &graph, config, encode_move as TestEncoder, &positions[1..], &checkpoint, provisional).err(), Some(LayoutRunResumeError::Positions));
    let consumer = |bytes: &[u8], provisional_len: u32, positions: Vec<LayoutRunPoint>| layout_run_job(identity(), &graph, config, || encode_move as TestEncoder, Some(LayoutRunResume { checkpoint: bytes, positions, provisional_len })).expect("the consumer entry always yields a job");
    assert_eq!(consumer(&checkpoint, provisional, positions.clone()).iteration(), decoded.iteration, "the consumer entry resumes its own checkpoint");
    assert_eq!(consumer(&checkpoint[..43], provisional, positions.clone()).iteration(), 0, "a malformed checkpoint starts fresh");
    assert_eq!(consumer(&checkpoint, provisional, layout_run_overlay_positions(&graph, [])).iteration(), 0, "unplaced overlay positions start fresh");
    assert_eq!(layout_run_overlay_positions(&graph, [(1, LayoutRunPoint::new(4.0, 5.0))])[1], LayoutRunPoint::new(4.0, 5.0));
    assert_eq!(layout_run_job(identity(), &other, LayoutRunConfig { max_iterations: 0, ..config }, || encode_move as TestEncoder, None).err(), Some(LayoutRunStartError::Config(LayoutRunConfigError { field: "maxIterations" })));
    assert_eq!(layout_run_entity("node-1"), u64::from_le_bytes(semio_framework_hash::hash(b"node-1").as_bytes()[..8].try_into().expect("digest")));
    let reconfigured = LayoutRunConfig { ideal_edge_length: 60.0, ..config };
    let mut retargeted = LayoutRunJob::resume(identity(), &graph, reconfigured, encode_move as TestEncoder, &positions, &checkpoint, provisional).expect("changed settings resume");
    let mut retargeted_ledger = Ledger { ops: checkpoint_ops, ..Ledger::default() };
    run_to(&mut retargeted, &mut retargeted_ledger, INTERACTIVE_LANE_FUEL, Driven::Complete);
    assert_ne!(retargeted.positions_digest(), reference.positions_digest(), "new settings change the continued layout");
    assert_run_laws("ring-24-retargeted", &graph, &retargeted, &retargeted_ledger);
}

#[test]
fn layout_run_barnes_hut_repulsion_tracks_exact_pairwise_repulsion() {
    let count = 2000;
    let mut state = 42u64;
    let positions: Vec<[f64; 2]> = (0..count).map(|_| [unit_interval(&mut state) * 4000.0, unit_interval(&mut state) * 4000.0]).collect();
    let radii: Vec<f64> = (0..count).map(|_| 10.0 + unit_interval(&mut state) * 20.0).collect();
    let bounds = positions.iter().fold([f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY], |b, p| [b[0].min(p[0]), b[1].min(p[1]), b[2].max(p[0]), b[3].max(p[1])]);
    let mut tree = LayoutRunTree::default();
    tree.reset(bounds, count);
    for body in 0..count {
        tree.insert(body as u32, &positions, &radii);
    }
    assert_eq!(tree.cells[0].count as usize, count, "the root aggregates every body");
    let corner = [[1.0, 1.0], [99.0, 99.0], [98.0, 99.0]];
    let corner_radii = [10.0; 3];
    let mut corner_tree = LayoutRunTree::default();
    corner_tree.reset([1.0, 1.0, 99.0, 99.0], 3);
    for body in 0..3 {
        corner_tree.insert(body, &corner, &corner_radii);
    }
    let corner_exact = [1, 2].iter().fold([0.0, 0.0], |sum, other| {
        let push = pairwise_repulsion(corner[0], corner[*other], 10.0, 10.0, 1.0, LayoutRunFalloff::Inverse);
        [sum[0] + push[0], sum[1] + push[1]]
    });
    let corner_approximate = corner_tree.repulsion(0, &corner, &corner_radii, 1.0, LayoutRunFalloff::Inverse, 4.0);
    let corner_error = (corner_approximate[0] - corner_exact[0]).hypot(corner_approximate[1] - corner_exact[1]) / corner_exact[0].hypot(corner_exact[1]);
    assert!(corner_error < 0.05, "a body never feels an aggregate that contains itself, even at the widest opening angle (error {corner_error})");
    let theta = LayoutRunConfig::default().barnes_hut_theta;
    for falloff in [LayoutRunFalloff::Inverse, LayoutRunFalloff::InverseSquare] {
        let mut errors: Vec<f64> = Vec::with_capacity(count);
        let (mut error_sum, mut magnitude_sum, mut closed_worst) = (0.0, 0.0, 0.0f64);
        for body in 0..count {
            let reference = (0..count).filter(|other| *other != body).fold([0.0, 0.0], |sum, other| {
                let push = pairwise_repulsion(positions[body], positions[other], radii[body], radii[other], 1.0, falloff);
                [sum[0] + push[0], sum[1] + push[1]]
            });
            let magnitude = reference[0].hypot(reference[1]);
            let approximate = tree.repulsion(body as u32, &positions, &radii, 1.0, falloff, theta * theta);
            let error = (approximate[0] - reference[0]).hypot(approximate[1] - reference[1]);
            errors.push(error / magnitude);
            error_sum += error;
            magnitude_sum += magnitude;
            let closed = tree.repulsion(body as u32, &positions, &radii, 1.0, falloff, 1e-6);
            closed_worst = closed_worst.max((closed[0] - reference[0]).hypot(closed[1] - reference[1]) / magnitude);
        }
        errors.sort_by(f64::total_cmp);
        let (median, p95, aggregate) = (errors[count / 2], errors[count * 95 / 100], error_sum / magnitude_sum);
        assert!(closed_worst < 1e-9, "{falloff:?}: a closed opening angle reproduces exact repulsion (worst {closed_worst})");
        assert!(aggregate < 0.01, "{falloff:?}: Barnes-Hut aggregate relative force error {aggregate}");
        if falloff == LayoutRunFalloff::Inverse {
            assert!(median < 0.01 && p95 < 0.04, "{falloff:?}: Barnes-Hut relative force error median {median}, p95 {p95}");
        }
    }
}

thread_local! {
    static CLOCK: std::cell::RefCell<(bool, Vec<u64>, usize)> = const { std::cell::RefCell::new((false, Vec::new(), 0)) };
}

/// ⏱️ Attempt 1 reads the real clock and records every read; later attempts replay the same reads, so every attempt
/// slices the run into identical steps and each step's best wall time across attempts filters preemption noise.
fn replay_now() -> Option<u64> {
    CLOCK.with(|clock| {
        let (replay, reads, cursor) = &mut *clock.borrow_mut();
        if *replay {
            let value = reads.get(*cursor).copied();
            *cursor += 1;
            value
        } else {
            let value = default_now_us()?;
            reads.push(value);
            Some(value)
        }
    })
}

#[test]
fn layout_run_step_stays_below_the_interactive_target_on_a_5000_node_grid() {
    let fixture = fixture();
    let law = &fixture["stepLaw"];
    let spec = serde_json::json!({ "kind": "grid", "columns": law["columns"], "rows": law["rows"], "radius": 20, "pins": [] });
    let graph = generate(&spec);
    let iterations = law["iterations"].as_u64().expect("iterations") as u32;
    let config = LayoutRunConfig { checkpoint_iterations: iterations, ..serde_json::from_value(fixture["defaultConfig"].clone()).expect("default config") };
    let budget_us = law["worstStepMicros"].as_u64().expect("worst");
    let attempts = law["attempts"].as_u64().expect("attempts");
    let mut best: Vec<u64> = Vec::new();
    let mut digests = BTreeSet::new();
    for attempt in 0..attempts {
        CLOCK.with(|clock| {
            let (replay, _, cursor) = &mut *clock.borrow_mut();
            *replay = attempt > 0;
            *cursor = 0;
        });
        let mut job = new_job(&graph, config);
        let mut ledger = Ledger::default();
        let cancel = root_cancel_token();
        let mut sequence = 0;
        let mut verdict = None;
        let mut step = 0;
        while ledger.checkpoints.is_empty() {
            let now = replay_now().expect("clock");
            let budget = StepBudget::from_duration(INTERACTIVE_LANE_FUEL, now, INTERACTIVE_LANE_WALL_US).expect("budget");
            let started = std::time::Instant::now();
            let mut outcome = drive_step(&mut job, "layout-run-step-law", OperationId(1), Generation(1), InteractiveStage::InteractiveStep, budget, cancel.clone(), replay_now, &mut sequence, &mut verdict);
            let elapsed = started.elapsed().as_micros() as u64;
            match best.get_mut(step) {
                Some(slot) => *slot = (*slot).min(elapsed),
                None => best.push(elapsed),
            }
            step += 1;
            observe(&mut outcome, &mut ledger);
            close_outcome(&mut outcome);
        }
        assert_eq!(job.iteration(), iterations, "the law measures the configured iterations plus one 5 000-node compaction");
        assert_eq!(ledger.ops.len(), graph.nodes.len(), "the compaction publishes every seeded node once");
        assert!(ledger.max_tick_bytes <= JOB_PAYLOAD_PAGE_BYTES);
        assert_eq!(step, best.len(), "every attempt replays the same step slicing");
        digests.insert(job.positions_digest());
    }
    assert_eq!(digests.len(), 1, "replayed attempts lay out identically");
    let worst = best.iter().copied().max().expect("steps");
    assert!(worst < budget_us, "worst best-of-{attempts} drive_step {worst} µs over {} steps on {} nodes must stay below {budget_us} µs", best.len(), graph.nodes.len());
}

#[test]
fn layout_run_emits_the_settled_trace_and_pinned_nodes_never_move() {
    let fixture = fixture();
    let case = fixture["cases"].as_array().expect("cases").iter().find(|case| case["id"] == "anchored-chain").expect("chain case");
    let graph = case_graph(case);
    let config = case_config(case);
    let mut job = new_job(&graph, config);
    let mut ledger = Ledger::default();
    run_to(&mut job, &mut ledger, INTERACTIVE_LANE_FUEL, Driven::Complete);
    assert_eq!(job.positions()[0], LayoutRunPoint::new(0.0, 0.0), "the pinned node stays at its origin");
    assert!(ledger.ops.iter().all(|(node, _)| *node != 0), "a pinned node never gets a move op");
    assert_eq!(job.stop(), Some(LayoutRunStop::IterationLimit), "a tiny settle threshold runs into the iteration limit");
    let trace = ledger.trace.as_ref().expect("trace");
    assert_eq!(trace.record(0).map(|record| record.reason), Some(LayoutRunReason::Pinned.code()));
    assert!(trace.records().any(|(_, record)| record.verdict == ToolRunVerdict::Warning), "nodes still moving at the limit are warnings");
    let anchored = job.positions()[2];
    assert!(anchored.x > 100.0, "the anchor pulls node 2 toward x=300 (x={})", anchored.x);
}

#[test]
fn layout_run_rejects_invalid_graphs_and_configs() {
    let node = LayoutRunNode { entity: 1, origin: None, radius: 10.0, pinned: false, anchor: None };
    let graph = |nodes: Vec<LayoutRunNode>, edges: Vec<LayoutRunEdge>| LayoutRunGraph { nodes, edges };
    let start = |graph: LayoutRunGraph, config: LayoutRunConfig| LayoutRunJob::new(identity(), &graph, config, encode_move as TestEncoder).err();
    let config = LayoutRunConfig::default();
    assert_eq!(start(graph(vec![LayoutRunNode { pinned: true, ..node }], vec![]), config), Some(LayoutRunStartError::Graph(LayoutRunGraphError::PinnedWithoutOrigin { node: 0 })));
    assert_eq!(start(graph(vec![LayoutRunNode { radius: 0.0, ..node }], vec![]), config), Some(LayoutRunStartError::Graph(LayoutRunGraphError::NonPositiveRadius { node: 0 })));
    assert_eq!(start(graph(vec![LayoutRunNode { origin: Some(LayoutRunPoint::new(f64::NAN, 0.0)), ..node }], vec![]), config), Some(LayoutRunStartError::Graph(LayoutRunGraphError::NonFiniteOrigin { node: 0 })));
    assert_eq!(start(graph(vec![node], vec![LayoutRunEdge { source: 0, target: 1, weight: 1.0 }]), config), Some(LayoutRunStartError::Graph(LayoutRunGraphError::EdgeOutOfRange { edge: 0 })));
    assert_eq!(start(graph(vec![node, node], vec![LayoutRunEdge { source: 0, target: 1, weight: -1.0 }]), config), Some(LayoutRunStartError::Graph(LayoutRunGraphError::InvalidWeight { edge: 0 })));
    assert_eq!(start(graph(vec![node; LAYOUT_RUN_NODES_MAX + 1], vec![]), config), Some(LayoutRunStartError::Graph(LayoutRunGraphError::TooManyNodes)));
    assert_eq!(start(graph(vec![node], vec![]), LayoutRunConfig { velocity_damping: 1.5, ..config }), Some(LayoutRunStartError::Config(LayoutRunConfigError { field: "velocityDamping" })));
    assert_eq!(start(graph(vec![node], vec![]), LayoutRunConfig { compact_ops: 0, ..config }), Some(LayoutRunStartError::Config(LayoutRunConfigError { field: "compactOps" })));
    let mut empty = new_job(&LayoutRunGraph::default(), config);
    let mut ledger = Ledger::default();
    run_to(&mut empty, &mut ledger, INTERACTIVE_LANE_FUEL, Driven::Complete);
    assert_eq!((empty.iteration(), empty.stop(), ledger.ops.len()), (0, Some(LayoutRunStop::Converged), 0), "an empty graph settles at once");
}

#[test]
fn layout_run_encoder_faults_and_cancellation_end_the_run() {
    let fixture = fixture();
    let graph = case_graph(&fixture["cases"][0]);
    let failing: TestEncoder = |_, _, _| Err(LayoutRunEncodeError("refused".to_string()));
    let mut job = LayoutRunJob::new(identity(), &graph, LayoutRunConfig::default(), failing).expect("valid run");
    let cancel = root_cancel_token();
    let mut faulted = false;
    for _ in 0..1000 {
        let mut sequence = 0;
        let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(1, u64::MAX), cancel.clone(), frozen_now, &mut sequence);
        let mut outcome = job.step(&mut cx);
        if let StepOutcome::Fault(fault) = &outcome {
            assert_eq!(fault.detail.page(0), Some(&b"refused"[..]));
            faulted = true;
        }
        close_outcome(&mut outcome);
        if faulted {
            break;
        }
    }
    assert!(faulted, "an encoder error faults the run");
    let mut cancelled = new_job(&graph, LayoutRunConfig::default());
    let token = root_cancel_token();
    token.cancel_now();
    let mut sequence = 0;
    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(1, u64::MAX), token, frozen_now, &mut sequence);
    assert_eq!(cancelled.step(&mut cx), StepOutcome::Cancelled);
}

#[test]
fn layout_run_close_releases_every_owned_buffer_in_bounded_steps() {
    let fixture = fixture();
    let graph = case_graph(&fixture["cases"][0]);
    let mut job = new_job(&graph, LayoutRunConfig::default());
    run_to(&mut job, &mut Ledger::default(), INTERACTIVE_LANE_FUEL, Driven::Checkpoint);
    assert_eq!(job.close_step(1, JOB_PAYLOAD_PAGE_BYTES), InteractiveJobCloseStep::Blocked, "close waits for begin_close");
    job.begin_close();
    let mut steps = 0;
    loop {
        match job.close_step(1, JOB_PAYLOAD_PAGE_BYTES) {
            InteractiveJobCloseStep::Pending { released_items, .. } => assert_eq!(released_items, 1),
            InteractiveJobCloseStep::Complete => break,
            InteractiveJobCloseStep::Blocked => panic!("close never blocks once begun"),
        }
        steps += 1;
        assert!(steps < 64, "close is bounded");
    }
    assert!(job.terminal_is_empty());
}

#[test]
fn layout_run_definition_matches_the_schema_tool_run_table() {
    let schema: Value = serde_json::from_str(SCHEMA).expect("schema parses");
    let table = &schema["x-semio-toolRun"];
    let definition = layout_run_definition(JobKindId::new("graph.layoutRun"));
    definition.validate().expect("definition is valid");
    let text = |label: &LocalizedLabel, locale: ui::wgpu::Locale| label.resolve(ui::wgpu::Terminology::Native, locale).to_string();
    let locales = |label: &LocalizedLabel| serde_json::json!({ "en": text(label, ui::wgpu::Locale::En), "de": text(label, ui::wgpu::Locale::De) });
    let stages: Vec<Value> = LayoutRunStage::ALL.iter().zip(&definition.stages).map(|(stage, row)| serde_json::json!({ "index": stage.index(), "id": row.id, "label": locales(&row.label) })).collect();
    assert_eq!(Value::Array(stages), table["stages"]);
    let counters: Vec<Value> = LayoutRunCounter::ALL.iter().zip(&definition.counters).map(|(counter, row)| serde_json::json!({ "index": counter.index(), "id": row.id, "fixedPoint": counter.fixed_point(), "label": locales(&row.label) })).collect();
    assert_eq!(Value::Array(counters), table["counters"]);
    let reasons: Vec<Value> = definition.reasons.iter().map(|row| serde_json::json!({ "code": row.code, "id": row.id, "verdict": verdict_id(row.verdict), "template": locales(&row.template) })).collect();
    assert_eq!(Value::Array(reasons), table["reasons"]);
    assert_eq!(locales(&definition.unit), table["unit"]);
    assert_eq!((definition.mutating, definition.rebase, definition.reconfigure, definition.trace), (true, ToolRunRebasePolicy::Restart, ToolRunReconfigurePolicy::Resume, ToolRunTraceKind::Entity));
    assert_eq!(table["checkpoint"]["bytes"], LayoutRunCheckpoint::BYTES);
    assert_eq!(table["checkpoint"]["fields"][0]["value"], LAYOUT_RUN_CHECKPOINT_MAGIC);
    assert_eq!(table["limits"]["nodesMax"], LAYOUT_RUN_NODES_MAX);
    assert_eq!(table["limits"]["opBytesMax"], LAYOUT_RUN_OP_BYTES_MAX);
    assert_eq!(table["limits"]["tickFlushBytes"], LAYOUT_RUN_TICK_FLUSH_BYTES);
    let default_config: LayoutRunConfig = serde_json::from_value(fixture()["defaultConfig"].clone()).expect("default config");
    assert_eq!(default_config, LayoutRunConfig::default(), "the fixture's default config is the Rust default");
    let config = LayoutRunConfig { center: Some(LayoutRunPoint::new(1.0, 2.0)), repulsion_falloff: LayoutRunFalloff::InverseSquare, spring_law: LayoutRunSpringLaw::Linear, ..LayoutRunConfig::default() };
    assert_eq!(LayoutRunConfig::from_value(config.to_value()).expect("config value round trip"), config);
    let graph = case_graph(&fixture()["cases"][3]);
    assert_eq!(LayoutRunGraph::from_value(graph.to_value()).expect("graph value round trip"), graph);
    let boxed: Box<dyn InteractiveJob + Send> = Box::new(new_job(&graph, config));
    assert!(!boxed.terminal_is_empty(), "the layout run boxes as the ToolRun ledger's job type");
    let checkpoint = LayoutRunCheckpoint { node_count: 3, graph_digest: 9, iteration: 4, settle_streak: 2, provisional_len: 5, center: [1.5, -2.0] };
    assert_eq!(LayoutRunCheckpoint::decode(&checkpoint.encode()), Some(checkpoint));
}

#[test]
fn layout_run_resets_diverged_nodes_and_never_calls_them_settled() {
    let node = |x: f64| LayoutRunNode { entity: x as u64 + 1, origin: Some(LayoutRunPoint::new(x, 0.0)), radius: 10.0, pinned: false, anchor: None };
    let graph = LayoutRunGraph { nodes: vec![node(0.0), node(1.0)], edges: vec![] };
    let config = LayoutRunConfig { repulsion_strength: f64::MAX, max_iterations: 5, ..LayoutRunConfig::default() };
    let mut job = new_job(&graph, config);
    let mut ledger = Ledger::default();
    run_to(&mut job, &mut ledger, INTERACTIVE_LANE_FUEL, Driven::Complete);
    assert_eq!(job.stop(), Some(LayoutRunStop::IterationLimit), "diverging nodes never converge");
    assert_eq!(job.positions(), vec![LayoutRunPoint::new(0.0, 0.0), LayoutRunPoint::new(1.0, 0.0)], "diverged nodes keep their last finite position");
    assert_eq!(job.counters()[3], 0, "diverged nodes are not settled");
    let danger: Vec<&ToolRunStep> = ledger.steps.iter().filter(|step| step.reason == LayoutRunReason::Diverged.code()).collect();
    assert_eq!(danger.len(), 5, "every diverging iteration reports one danger step");
    assert!(danger.iter().all(|step| step.kind == ToolRunStepKind::Danger && step.args == vec![ToolRunStepArg::Unsigned(2)]));
    let trace = ledger.trace.as_ref().expect("trace");
    assert!(trace.records().all(|(_, record)| (record.verdict, record.reason) == (ToolRunVerdict::Danger, LayoutRunReason::Diverged.code())));
    assert!(ledger.ops.is_empty(), "nodes that never left their origin get no move op");
}
