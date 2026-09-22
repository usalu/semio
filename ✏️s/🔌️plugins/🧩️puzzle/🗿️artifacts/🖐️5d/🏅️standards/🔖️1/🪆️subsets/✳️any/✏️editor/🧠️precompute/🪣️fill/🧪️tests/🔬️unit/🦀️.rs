use super::*;
use crate::editor::puzzle5d::config::Puzzle5dConfig;
use crate::editor::puzzle5d::precompute::{puzzle3d_snapshot, puzzle5d_placement_entity, Puzzle5dPlannerToolRunJob, PUZZLE5D_PLACEMENT_OPS, PUZZLE5D_PLANNER_TRACE_SHAPE_CIRCLE, PUZZLE5D_PLANNER_TRACE_TWIN_BIT};
use crate::editor::puzzle5d::puzzle5d_grip_full_id;
use semio_framework_job::{InteractiveJob, RetainedJobPayload, StepOutcome, JOB_PAYLOAD_PAGE_BYTES};
use semio_framework_tool_run::{ToolRunTick, ToolRunTraceOp, ToolRunTraceSubject, ToolRunVerdict};
use semio_s_artifact_puzzle_3d::editor::puzzle3d::config::Puzzle3dConfig;
use std::collections::HashMap;
use crate::editor::puzzle5d::modes::edit::tools::fill::run_definition;
use crate::editor::puzzle5d::modes::edit::windows::{board2d, world3d as world3d_window};
use crate::editor::puzzle5d::unit_tests::context::{app_with_registry, close_app, dispatch, meta, projection_of, render_body, window_view, Puzzle5dApp};
use crate::editor::puzzle5d::{capsule_dream_example_document, concrete_forest_example_document, nakagin_example_document};
use crate::standards::v1::subsets::any::schema::mutations::text::Puzzle5dPlaySnapshot;
use semio_framework_job::{CancelToken, Generation, InteractiveStage, OperationId, StepBudget};
use semio_framework_plugin::{ActionMeta, ArtifactApp, ArtifactInstanceOperationOwnerHandle, PluginApp, ToolRunJobPort, ToolRunJobPurpose, ToolRunTraceKeys};
use semio_framework_tool_run::{ToolRunId, ToolRunIdentity, ToolRunStep, TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_FINALIZE_ACTION_ID, TOOL_RUN_START_ACTION_ID};
use semio_s_artifact_puzzle_3d::editor::puzzle3d::Puzzle3dPlayApp;
use semio_s_artifact_puzzle_3d::standards::v1::subsets::any::schema::FillRunReason;
use std::time::{Duration, Instant};

const FILL_RUN_FIXTURE: &str = include_str!("../../🧫️fixtures/🎞️fill-run.json");
pub(crate) const FILL_RUN_TURNS: usize = 262_144;

fn fixture() -> serde_json::Value {
    serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run fixture parses")
}

fn example(id: &str) -> Puzzle5dDocument {
    match id {
        "concrete-forest" => concrete_forest_example_document(),
        "nakagin-capsule-tower" => nakagin_example_document(),
        "capsule-dream" => capsule_dream_example_document(),
        other => panic!("unknown example {other}"),
    }
}

fn identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [0; 32])
}

fn never() -> Option<u64> {
    Some(0)
}

/// 🧊️ The scale the planner's own box fallback applies to a mesh identity it must stand in for.
const PUZZLE5D_FILL_LAW_BOX_SCALE: f32 = 4.0;

/// 🧊️ Derives a collision body for EVERY mesh identity `document` names into the process-wide brush-mesh
/// store, so the planner's preparation can resolve them.
///
/// 🐛️ Without this every candidate of every 5d fill law read `warning:mesh-unavailable` and no run placed
/// anything. The 5d fill delegates to puzzle 3d's planner through the real `fill_run_job` →
/// `fill3d::build_run_job`, and that preparation resolves each lane url through `shared_brush_mesh(&url)`
/// — the PROCESS-WIDE store the browser fills with `registerBrushMesh`'s GLB round trip. A url with no
/// registered mesh takes the scaled-box substitute ONLY when it is literally `PUZZLE3D_FALLBACK_MESH_KIND`;
/// every other url is dropped from `FillPreparationRoots::meshes`, and `FillBuilder` then rejects its
/// candidates `mesh-unavailable`. `🧊️3d`'s own fill laws never meet this because they build
/// `FillPreparationRoots::new(scene, meshes)` directly and seed a body per lane url; the 5d laws go
/// through the preparation with an EMPTY store.
///
/// The store is a process-global `Mutex` every accessor reaches with `try_lock().ok()?`, so a contended
/// lock reads as "not resident" — hence the bounded retry rather than one call.
fn seed_planner_meshes(document: &Puzzle5dDocument) {
    use semio_s_artifact_puzzle_3d::editor::puzzle3d::precompute::derive_brush_mesh;
    let fallback = semio_framework_plugin::mesh_from_kind(crate::editor::puzzle5d::PUZZLE5D_FALLBACK_MESH_KIND);
    let positions: Vec<f32> = fallback.positions.iter().map(|value| value * PUZZLE5D_FILL_LAW_BOX_SCALE).collect();
    let mut urls = crate::editor::puzzle5d::collect_mesh_urls(document);
    urls.push(crate::editor::puzzle5d::PUZZLE5D_FALLBACK_MESH_KIND.to_string());
    urls.sort();
    urls.dedup();
    for url in urls {
        for _ in 0..4_096 {
            if derive_brush_mesh(&url, &positions, &fallback.indices).is_some() {
                break;
            }
            std::thread::yield_now();
        }
    }
}

/// 🧵️ A fill run or revalidate job over `document` exactly as the ledger builds it through the app hook.
fn job(document: &Puzzle5dDocument, requested: u32, purpose: ToolRunJobPurpose, provisional: &[Puzzle5dMutation]) -> Puzzle5dPlannerToolRunJob {
    seed_planner_meshes(document);
    let definition = run_definition();
    let request = ToolRunJobRequest::<EditorApp<Puzzle5dPlayApp>> {
        tool_id: TOOL_ID,
        definition: &definition,
        purpose,
        identity: identity(),
        snapshot: Arc::new(Puzzle5dPlaySnapshot(serde_json::to_value(document).expect("document serializes"))),
        config: Arc::new(Puzzle5dConfig { fill_count: requested, ..Puzzle5dConfig::default() }),
        window_id: None,
        window_config: None,
        checkpoint: None,
        provisional,
        instance_owner: ArtifactInstanceOperationOwnerHandle::new(<EditorApp<Puzzle5dPlayApp> as ArtifactApp>::build_instance_operation_owner()),
        port: ToolRunJobPort::default(),
        trace_keys: ToolRunTraceKeys::default(),
        entity_marks: &[],
    };
    fill_run_job(request).expect("the fill job builds").expect("the fill tool has a run job")
}

fn close(mut payload: RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
}

/// 🪞️ What the ledger folds from a run: provisional 5d ops and entities, every trace op in order, steps.
#[derive(Default)]
struct Mirror {
    ops: Vec<Puzzle5dMutation>,
    entities: Vec<u64>,
    trace: Vec<ToolRunTraceOp>,
    steps: Vec<ToolRunStep>,
    ticks: usize,
    complete: bool,
    fault: Option<String>,
}

impl Mirror {
    fn apply(&mut self, tick: ToolRunTick) {
        if let Some(length) = tick.retract_to {
            self.ops.truncate(length as usize);
            self.entities.truncate(length as usize / PUZZLE5D_PLACEMENT_OPS);
        }
        self.ops.extend(tick.append_ops.iter().map(|bytes| <Puzzle5dMutation as protocol::OpBinary>::decode_op(bytes).expect("a 5d op decodes")));
        self.entities.extend(tick.append_entities);
        self.trace.extend(tick.trace.into_iter().flat_map(|page| page.ops));
        self.steps.extend(tick.steps);
        self.ticks += 1;
    }

    /// 🎬️ One turn: `true` once the job completed.
    fn turn(&mut self, job: &mut dyn InteractiveJob, budget: StepBudget, clock: fn() -> Option<u64>, sequence: &mut u64) -> bool {
        let mut verdict = None;
        match semio_framework_job::drive_step(job, "puzzle5d-fill-run", OperationId(5), Generation(1), InteractiveStage::InteractiveStep, budget, CancelToken::root_now(), clock, sequence, &mut verdict) {
            StepOutcome::PreviewReady(payload) => {
                let bytes: Vec<u8> = (0..payload.page_count()).flat_map(|index| payload.page(index).expect("tick page").to_vec()).collect();
                close(payload);
                self.apply(ToolRunTick::decode(&bytes).expect("a translated tick decodes"));
                false
            }
            StepOutcome::CheckpointReady(checkpoint) => {
                close(checkpoint.state);
                false
            }
            StepOutcome::Complete(candidate) => {
                close(candidate.state);
                close(candidate.output);
                self.complete = true;
                true
            }
            StepOutcome::Yield => false,
            StepOutcome::Fault(mut fault) => {
                let detail: Vec<u8> = (0..fault.detail.page_count()).flat_map(|index| fault.detail.page(index).expect("fault page").to_vec()).collect();
                while !fault.detail.terminal_is_empty() {
                    fault.detail.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
                }
                self.fault = Some(String::from_utf8_lossy(&detail).into_owned());
                true
            }
            other => panic!("the fill run job ended unexpectedly: {other:?}"),
        }
    }

    fn primaries(&self) -> Vec<(u64, ToolRunVerdict, u16, ToolRunTraceSubject)> {
        self.trace
            .iter()
            .filter_map(|op| match op {
                ToolRunTraceOp::Upsert { key, verdict, reason, subject: subject @ ToolRunTraceSubject::Instance3d { .. } } => Some((*key, *verdict, *reason, *subject)),
                _ => None,
            })
            .collect()
    }
}

fn run_to_completion(mut job: Puzzle5dPlannerToolRunJob) -> Mirror {
    let mut mirror = Mirror::default();
    let mut sequence = 0;
    for _ in 0..FILL_RUN_TURNS {
        if mirror.turn(&mut job, StepBudget::new(semio_framework_job::INTERACTIVE_LANE_FUEL, u64::MAX), never, &mut sequence) {
            assert!(mirror.fault.is_none(), "the fill run job faulted: {:?}", mirror.fault);
            return mirror;
        }
    }
    panic!("the fill run job never completed");
}

fn reason_id(code: u16) -> String {
    FillRunReason::from_code(code).map_or_else(|| format!("{code:#06x}"), |reason| reason.id().to_string())
}

fn verdict_id(verdict: ToolRunVerdict) -> &'static str {
    verdict.as_str()
}

/// ⚖️ LAW (language-neutral fixture): every seeded example run reaches the fixture's exact verdict prefix,
/// tested and placed counts and stall; it appends one `create-part` and one `connect-grips` per placement with
/// the part's entity; every `instance3d` record is followed by its `placement2d` twin under the twin key with
/// the same verdict, and a placed part's twin sits on the part; every fastener joins the new part to a part
/// placed before it. The planner itself is the oracle for the 3d half: the 5d run's `instance3d` trace equals
/// the puzzle 3d fill job's trace over the same document.
#[test]
fn fill_run_job_matches_the_language_neutral_fill_run_fixture() {
    let fixture = fixture();
    let twin_bit = fixture["twinBit"].as_u64().expect("twin bit");
    assert_eq!(twin_bit, PUZZLE5D_PLANNER_TRACE_TWIN_BIT);
    assert_eq!(fixture["opsPerPlacement"].as_u64(), Some(PUZZLE5D_PLACEMENT_OPS as u64));
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let document = example(case["example"].as_str().expect("example"));
        let requested = case["requested"].as_u64().expect("requested") as u32;
        let run = run_to_completion(job(&document, requested, ToolRunJobPurpose::Run, &[]));
        let primaries = run.primaries();
        let finals: Vec<String> = primaries.iter().filter(|(_, verdict, _, _)| *verdict != ToolRunVerdict::Testing).map(|(_, verdict, reason, _)| format!("{}:{}", verdict_id(*verdict), reason_id(*reason))).collect();
        let tested = primaries.iter().filter(|(_, verdict, _, _)| *verdict == ToolRunVerdict::Testing).count();
        let placed = run.ops.len() / PUZZLE5D_PLACEMENT_OPS;
        let stall = run.steps.iter().rev().find(|step| step.kind == semio_framework_tool_run::ToolRunStepKind::Warning).map(|step| reason_id(step.reason));
        let expected: Vec<String> = case["verdictPrefix"].as_array().expect("verdict prefix").iter().map(|entry| entry.as_str().expect("verdict entry").to_string()).collect();
        assert_eq!(&finals[..expected.len().min(finals.len())], &expected[..], "{id}: verdict prefix");
        assert_eq!(tested as u64, case["tested"].as_u64().expect("tested"), "{id}: tested");
        assert_eq!(placed as u64, case["placed"].as_u64().expect("placed"), "{id}: placed");
        assert_eq!(stall.as_deref(), case["stall"].as_str(), "{id}: stall");
        assert!(run.complete, "{id}: the run completes");

        let mut board: HashMap<String, [f64; 2]> = document.parts.iter().map(|part| (part.id.clone(), [part.part_2d.x, part.part_2d.y])).collect();
        let mut grips: std::collections::HashSet<String> = document.parts.iter().flat_map(|part| part.grips.iter().map(|grip| puzzle5d_grip_full_id(&part.id, &grip.id))).collect();
        assert_eq!(run.entities.len(), placed, "{id}: one entity per placement");
        for (index, pair) in run.ops.chunks(PUZZLE5D_PLACEMENT_OPS).enumerate() {
            let [Puzzle5dMutation::CreatePart(create), Puzzle5dMutation::ConnectGrips(connect)] = pair else { panic!("{id}: placement {index} is create-part then connect-grips, got {pair:?}") };
            assert_eq!(run.entities[index], puzzle5d_placement_entity(&create.part.id), "{id}: entity of placement {index}");
            let own: Vec<String> = create.part.grips.iter().map(|grip| puzzle5d_grip_full_id(&create.part.id, &grip.id)).collect();
            let host = [&connect.source, &connect.target].into_iter().find(|grip| !own.contains(grip)).expect("a fastener joins the part to a host");
            assert!(grips.contains(host), "{id}: placement {index} docks at a grip placed before it");
            assert!([&connect.source, &connect.target].into_iter().any(|grip| own.contains(grip)), "{id}: placement {index} fastens one of its own grips");
            grips.extend(own);
            board.insert(create.part.id.clone(), [create.part.part_2d.x, create.part.part_2d.y]);
        }
        let twins: HashMap<u64, (ToolRunVerdict, u16, [f32; 2])> = run
            .trace
            .iter()
            .filter_map(|op| match op {
                ToolRunTraceOp::Upsert { key, verdict, reason, subject: ToolRunTraceSubject::Placement2d { shape, position, .. } } => {
                    assert_eq!(*shape, PUZZLE5D_PLANNER_TRACE_SHAPE_CIRCLE);
                    Some((*key, (*verdict, *reason, *position)))
                }
                _ => None,
            })
            .collect();
        for window in run.trace.windows(2) {
            if let [ToolRunTraceOp::Upsert { key, verdict, reason, subject: ToolRunTraceSubject::Instance3d { .. } }, next] = window {
                assert!(matches!(next, ToolRunTraceOp::Upsert { key: twin, verdict: twin_verdict, reason: twin_reason, subject: ToolRunTraceSubject::Placement2d { .. } } if *twin == key | twin_bit && twin_verdict == verdict && twin_reason == reason), "{id}: record {key} is followed by its twin");
            }
        }
        let placed_centers: Vec<[f32; 2]> = run.ops.chunks(PUZZLE5D_PLACEMENT_OPS).filter_map(|pair| match &pair[0] {
            Puzzle5dMutation::CreatePart(create) => Some([create.part.part_2d.x as f32, create.part.part_2d.y as f32]),
            _ => None,
        }).collect();
        let success_twins: Vec<[f32; 2]> = primaries.iter().filter(|(_, verdict, _, _)| *verdict == ToolRunVerdict::Success).map(|(key, _, _, _)| twins[&(key | twin_bit)].2).collect();
        assert_eq!(success_twins, placed_centers, "{id}: a placed part's board twin sits on the part");

        let planner = fill3d::build_run_job(ToolRunJobRequest::<EditorApp<Puzzle3dPlayApp>> {
            tool_id: fill3d::TOOL_ID,
            definition: &fill3d::run_definition(),
            purpose: ToolRunJobPurpose::Run,
            identity: identity(),
            snapshot: Arc::new(puzzle3d_snapshot(&document, None).expect("the 3d document builds")),
            config: Arc::new(Puzzle3dConfig { fill_count: requested, ..Puzzle3dConfig::default() }),
            window_id: None,
            window_config: None,
            checkpoint: None,
            provisional: &[],
            instance_owner: ArtifactInstanceOperationOwnerHandle::new(<EditorApp<Puzzle3dPlayApp> as ArtifactApp>::build_instance_operation_owner()),
            port: ToolRunJobPort::default(),
            trace_keys: ToolRunTraceKeys::default(),
            entity_marks: &[],
        })
        .expect("the planner job builds")
        .expect("the planner has a run job");
        let mut sequence = 0;
        let mut planner = planner;
        let mut oracle_trace = Vec::new();
        let mut oracle_ops = 0;
        let mut oracle_complete = false;
        for _ in 0..FILL_RUN_TURNS {
            let mut verdict = None;
            match semio_framework_job::drive_step(planner.as_mut(), "puzzle3d-fill-run", OperationId(5), Generation(1), InteractiveStage::InteractiveStep, StepBudget::new(semio_framework_job::INTERACTIVE_LANE_FUEL, u64::MAX), CancelToken::root_now(), never, &mut sequence, &mut verdict) {
                StepOutcome::PreviewReady(payload) => {
                    let bytes: Vec<u8> = (0..payload.page_count()).flat_map(|index| payload.page(index).expect("tick page").to_vec()).collect();
                    close(payload);
                    let tick = ToolRunTick::decode(&bytes).expect("a planner tick decodes");
                    oracle_ops = tick.retract_to.map_or(oracle_ops, |length| length as usize) + tick.append_ops.len();
                    oracle_trace.extend(tick.trace.into_iter().flat_map(|page| page.ops));
                }
                StepOutcome::CheckpointReady(checkpoint) => close(checkpoint.state),
                StepOutcome::Complete(candidate) => {
                    close(candidate.state);
                    close(candidate.output);
                    oracle_complete = true;
                    break;
                }
                StepOutcome::Yield => {}
                other => panic!("the planner ended unexpectedly: {other:?}"),
            }
        }
        assert!(oracle_complete, "{id}: the planner completes");
        let primary_trace: Vec<ToolRunTraceOp> = run.trace.iter().filter(|op| !matches!(op, ToolRunTraceOp::Upsert { subject: ToolRunTraceSubject::Placement2d { .. }, .. }) && !matches!(op, ToolRunTraceOp::Retire { key } if key & twin_bit != 0)).copied().collect();
        assert_eq!(primary_trace, oracle_trace, "{id}: the 5d run reports exactly the planner's verdicts");
        assert_eq!(run.ops.len(), oracle_ops, "{id}: one 5d op per planner op");
    }
}

/// ⚖️ LAW: a revalidation over a clean head retracts nothing and re-reports every provisional placement as
/// `success`; a head that already holds a placement's part id makes that placement conflict.
#[test]
fn fill_revalidate_job_translates_provisional_placements_and_their_conflicts() {
    let document = concrete_forest_example_document();
    let run = run_to_completion(job(&document, 3, ToolRunJobPurpose::Run, &[]));
    assert_eq!(run.ops.len(), 6, "three provisional placements");
    let clean = run_to_completion(job(&document, 3, ToolRunJobPurpose::Revalidate, &run.ops));
    assert!(clean.ops.is_empty(), "a clean head retracts and re-appends nothing");
    assert_eq!(clean.primaries().iter().filter(|(_, verdict, _, _)| *verdict == ToolRunVerdict::Success).count(), 3);
    let mut head = document.clone();
    let Puzzle5dMutation::CreatePart(first) = &run.ops[0] else { panic!("create-part") };
    head.parts.push(editor_part(&first.part).expect("editor part"));
    let conflicted = run_to_completion(job(&head, 3, ToolRunJobPurpose::Revalidate, &run.ops));
    assert!(conflicted.primaries().iter().any(|(_, verdict, _, _)| *verdict == ToolRunVerdict::Danger), "the duplicated placement conflicts");
    assert!(conflicted.trace.iter().any(|op| matches!(op, ToolRunTraceOp::Upsert { verdict: ToolRunVerdict::Danger, subject: ToolRunTraceSubject::Placement2d { .. }, .. })), "its board twin conflicts too");
}

//#region ⏱️Interactive
#[derive(Default)]
struct TurnClock {
    reads: u64,
    deadline: u64,
    first_expired: Option<u64>,
    replay_expiry: Option<u64>,
}

thread_local! {
    static TURN_CLOCK: std::cell::RefCell<TurnClock> = std::cell::RefCell::new(TurnClock::default());
}

fn recording_clock() -> Option<u64> {
    let now = semio_framework_job::default_now_us();
    TURN_CLOCK.with(|clock| {
        let mut clock = clock.borrow_mut();
        clock.reads += 1;
        if clock.first_expired.is_none() && now.is_none_or(|now| now >= clock.deadline) {
            clock.first_expired = Some(clock.reads);
        }
    });
    now
}

fn replaying_clock() -> Option<u64> {
    TURN_CLOCK.with(|clock| {
        let mut clock = clock.borrow_mut();
        clock.reads += 1;
        Some(if clock.replay_expiry.is_some_and(|expiry| clock.reads >= expiry) { u64::MAX } else { 0 })
    })
}

/// ⏱️ LAW: on the plugin's largest examples every `drive_step` of the 5d fill run job — the planner's slice plus
/// the tick translation onto the 5d document — stays below the interactive ceiling. Each turn takes its best of
/// several cold runs: the first slices by the real clock and the others replay its slice boundaries exactly,
/// so every run yields the same ticks in the same order.
#[test]
fn fill_run_job_step_stays_below_the_interactive_ceiling_on_the_largest_examples() {
    let fixture = fixture();
    let law = &fixture["laws"]["interactive"];
    let budget = Duration::from_micros(law["budgetUs"].as_u64().expect("budget"));
    let slice = law["sliceUs"].as_u64().expect("slice");
    let runs = law["coldRuns"].as_u64().expect("cold runs") as usize;
    let requested = law["requested"].as_u64().expect("requested") as u32;
    for name in law["examples"].as_array().expect("examples").iter().map(|entry| entry.as_str().expect("example")) {
        let document = example(name);
        let mut expiries: Vec<Option<u64>> = Vec::new();
        let mut best: Vec<Duration> = Vec::new();
        let mut placed = 0;
        for run in 0..runs {
            let mut job = job(&document, requested, ToolRunJobPurpose::Run, &[]);
            let mut mirror = Mirror::default();
            let mut sequence = 0;
            for turn in 0..FILL_RUN_TURNS {
                let recording = run == 0;
                assert!(recording || turn < expiries.len(), "{name}: cold run {run} took more turns than the recorded run");
                let (step_budget, clock): (StepBudget, fn() -> Option<u64>) = if recording {
                    let start = semio_framework_job::default_now_us().expect("clock");
                    TURN_CLOCK.with(|clock| *clock.borrow_mut() = TurnClock { deadline: start + slice, ..TurnClock::default() });
                    (StepBudget::new(semio_framework_job::INTERACTIVE_LANE_FUEL, start + slice), recording_clock)
                } else {
                    TURN_CLOCK.with(|clock| *clock.borrow_mut() = TurnClock { replay_expiry: expiries[turn], ..TurnClock::default() });
                    (StepBudget::new(semio_framework_job::INTERACTIVE_LANE_FUEL, 1), replaying_clock)
                };
                job.clock = clock;
                let started = Instant::now();
                let complete = mirror.turn(&mut job, step_budget, clock, &mut sequence);
                let elapsed = started.elapsed();
                if recording {
                    expiries.push(TURN_CLOCK.with(|clock| clock.borrow().first_expired));
                    best.push(elapsed);
                } else {
                    best[turn] = best[turn].min(elapsed);
                }
                if complete {
                    break;
                }
            }
            placed = mirror.ops.len() / PUZZLE5D_PLACEMENT_OPS;
            match law["capacityRefusals"][name].as_str() {
                Some(refusal) => {
                    assert_eq!(mirror.fault.as_deref(), Some(refusal), "{name}: cold run {run} refuses the document past the planner's fixed capacity");
                    assert!(mirror.steps.iter().any(|step| step.kind == semio_framework_tool_run::ToolRunStepKind::Danger && step.reason == FillRunReason::ArtifactCapacity.code()), "{name}: the refusal is a visible danger step before the fault");
                }
                None => assert!(mirror.complete && mirror.fault.is_none(), "{name}: cold run {run} completes: {:?}", mirror.fault),
            }
        }
        let (turn, worst) = best.iter().enumerate().max_by_key(|(_, elapsed)| **elapsed).map_or((0, Duration::ZERO), |(turn, elapsed)| (turn + 1, *elapsed));
        assert!(worst < budget, "{name}: fill run job worst drive_step {worst:?} at turn {turn} of {} exceeds {budget:?}", best.len());
    }
}
//#endregion ⏱️Interactive

//#region ⏯️App
pub(crate) fn host_turn(app: &mut Puzzle5dApp) {
    PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("maintenance step");
    semio_framework::io::resolve_ready(app.advance_typed_operation_publication()).expect("advance one typed operation publication unit");
    if let Some(page) = app.take_typed_operation_result_page(1) {
        assert_ne!(page.lane, semio_framework_plugin::app::TypedOperationResultLane::Fault, "retained operation faulted: {}", String::from_utf8_lossy(page.bytes()));
        app.acknowledge_typed_operation_result(page.token).expect("acknowledge one presented result page");
    }
    let _ = app.take_typed_operation_event();
    let _ = semio_framework::io::resolve_ready(app.take_typed_operation_completion());
    let _ = app.take_typed_operation_effect();
    let _ = app.take_typed_operation_ui_scope();
}

pub(crate) fn world_meta() -> ActionMeta {
    ActionMeta { view_state: Some(window_view(world3d_window::WINDOW_KIND_ID, world3d_window::WINDOW_KIND_ID)), ..meta("local") }
}

pub(crate) fn tool_run_action(app: &mut Puzzle5dApp, action: &str, args: serde_json::Value) -> serde_json::Value {
    let args = dsl::DslValue::from(&args);
    let result = semio_framework::io::resolve_ready(app.handle_action(action, Some(&args), &world_meta())).unwrap_or_else(|fault| panic!("{action} faulted: {fault:?}"));
    serde_json::Value::from(&result.output)
}

fn start(app: &mut Puzzle5dApp) -> serde_json::Value {
    // 🧊️ The app-driven laws run the planner through the LIVE tool-run ledger, not through `job()`, so they
    // need the same process-wide mesh identities seeded — see `seed_planner_meshes`. Without them every
    // candidate reads `mesh-unavailable` and the run completes having placed nothing
    // (`PresenceToolRun { state: Complete, completed: 0, total: Some(40) }`).
    seed_planner_meshes(&crate::editor::puzzle5d::default_document());
    tool_run_action(app, TOOL_RUN_START_ACTION_ID, serde_json::json!({ TOOL_RUN_ARG_TOOL_ID: TOOL_ID }))
}

fn pump(app: &mut Puzzle5dApp, what: &str, until: impl Fn(&protocol::PresenceToolRun) -> bool) -> usize {
    for turn in 0..FILL_RUN_TURNS {
        if app.tool_run_presence().is_some_and(|presence| until(&presence)) {
            return turn;
        }
        host_turn(app);
    }
    panic!("{what} never happened; presence {:?}", app.tool_run_presence());
}

/// 🧭️ The live fill run's `{runId, generation}` exactly as the world window's Escape binding carries it.
fn escape_identity(app: &mut Puzzle5dApp) -> Option<serde_json::Value> {
    let view = window_view(world3d_window::WINDOW_KIND_ID, world3d_window::WINDOW_KIND_ID);
    let engagements = semio_framework::io::resolve_ready(app.window_engagements(&view));
    let abort = engagements.get(world3d_window::WINDOW_KIND_ID)?.input.as_ref()?.on_abort.clone()?;
    (abort.action == TOOL_RUN_ABORT_ACTION_ID).then(|| serde_json::Value::from(abort.args.as_ref().expect("a tool run abort carries its identity")))
}

pub(crate) fn committed(app: &Puzzle5dApp) -> String {
    projection_of(app).to_string()
}

fn provisional_instances(app: &mut Puzzle5dApp) -> (usize, usize) {
    let body: serde_json::Value = serde_json::from_str(&render_body(app, world3d_window::BODY_KEY)).expect("world body");
    let instances: Vec<serde_json::Value> = serde_json::from_str(body["scene"]["instancesJson"].as_str().expect("instances json")).expect("instances parse");
    (instances.len(), instances.iter().filter(|instance| instance["provisional"] == serde_json::json!(true)).count())
}

fn set_fill_count(app: &mut Puzzle5dApp, count: u64) {
    dispatch(app, "setFillCount", Some(&dsl::json!({ "value": count })), Some(board2d::WINDOW_KIND_ID)).expect("setFillCount");
}

/// ⏯️ LAW (start → complete → finalize = one undo entry): the tool declares its run through the manifest; a
/// complete run has committed nothing while the world window renders committed ⊕ provisional parts stamped
/// `provisional`; finalize publishes every placement as ONE edit that one undo removes and one redo restores.
#[test]
fn fill_run_finalize_publishes_one_edit_with_every_provisional_placement() {
    let law = &fixture()["laws"]["finalize"];
    let requested = law["requested"].as_u64().expect("requested");
    let mut app = app_with_registry();
    set_fill_count(&mut app, requested);
    let before = committed(&app);
    let parts = projection_of(&app)["parts"].as_array().map_or(0, Vec::len);
    let fasteners = projection_of(&app)["fasteners"].as_array().map_or(0, Vec::len);
    assert_eq!(start(&mut app)["toolRun"], serde_json::json!("spawnJob"));
    pump(&mut app, "the run completes", |presence| presence.state == protocol::PresenceToolRunState::Complete);
    assert_eq!(app.tool_run_presence().expect("presence").completed, requested, "the run places what was requested");
    assert_eq!(committed(&app), before, "a complete run has committed nothing");
    assert_eq!(provisional_instances(&mut app), (parts + requested as usize, requested as usize), "the world renders committed ⊕ provisional, the provisional parts stamped");
    let identity = escape_identity(&mut app).expect("a complete run is live until it is finalized");
    assert_eq!(tool_run_action(&mut app, TOOL_RUN_FINALIZE_ACTION_ID, identity)["toolRun"], serde_json::json!("beginFinalize"));
    pump(&mut app, "finalize publishes", |presence| presence.state == protocol::PresenceToolRunState::Finalized);
    let finalized = projection_of(&app);
    let published = finalized["parts"].as_array().expect("parts");
    assert_eq!(published.len(), parts + requested as usize, "finalize publishes every placement");
    assert_eq!(finalized["fasteners"].as_array().map_or(0, Vec::len), fasteners + requested as usize, "and one fastener per placement");
    for part in &published[parts..] {
        let flat = &part["2d"];
        let spatial = &part["3d"];
        assert!(flat["x"].as_f64().is_some() && flat["y"].as_f64().is_some(), "a published placement carries its FLAT pose: {part}");
        assert_eq!(spatial["origin"].as_array().map_or(0, Vec::len), 3, "a published placement carries its SPATIAL pose: {part}");
        assert!(part["grips"].as_array().is_some_and(|grips| !grips.is_empty()), "a published placement carries its grips: {part}");
    }
    assert!(escape_identity(&mut app).is_none(), "a finalized run is terminal, so Escape no longer aborts it");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(committed(&app), before, "one undo removes every placement: start → complete → finalize is ONE undo entry");
    dispatch(&mut app, "redo", None, None).expect("redo");
    assert_eq!(projection_of(&app), finalized, "one redo restores all of them");
    close_app(&mut app);
}

/// 🛑 LAW (abort leaves the document byte-identical): Escape carries the live run's identity from `starting` on;
/// aborting after provisional parts are on screen leaves zero provisional parts, a byte-identical committed
/// document and no undo entry.
#[test]
fn aborting_a_fill_run_leaves_the_document_byte_identical() {
    let law = &fixture()["laws"]["abort"];
    let mut app = app_with_registry();
    set_fill_count(&mut app, law["requested"].as_u64().expect("requested"));
    let before = committed(&app);
    let parts = projection_of(&app)["parts"].as_array().map_or(0, Vec::len);
    start(&mut app);
    assert!(escape_identity(&mut app).is_some(), "abort is offered while the run is still starting");
    pump(&mut app, "a provisional placement", |presence| presence.completed > 0);
    assert!(provisional_instances(&mut app).1 > 0, "provisional parts are on screen before the abort");
    let identity = escape_identity(&mut app).expect("abort carries the run's current identity");
    assert_eq!(tool_run_action(&mut app, TOOL_RUN_ABORT_ACTION_ID, identity)["toolRun"], serde_json::json!("closeJob"));
    pump(&mut app, "the abort settles", |presence| presence.state == protocol::PresenceToolRunState::Aborted);
    assert_eq!(committed(&app), before, "an aborted run leaves the committed document byte-identical");
    assert_eq!(provisional_instances(&mut app), (parts, 0), "an aborted run holds zero provisional parts");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(committed(&app), before, "and left no undo entry behind");
    close_app(&mut app);
}

/// 🎚️ LAW: `setFillCount` is configuration only and, during a run, reconfigures that same run under a newer
/// generation until the raised count is placed — never an artifact mutation.
#[test]
fn raising_the_fill_count_during_a_run_reconfigures_the_same_run() {
    let law = &fixture()["laws"]["reconfigure"];
    let mut app = app_with_registry();
    set_fill_count(&mut app, law["requested"].as_u64().expect("requested"));
    let before = committed(&app);
    assert_eq!(committed(&app), before, "the count is configuration: no artifact mutation");
    start(&mut app);
    pump(&mut app, "the first run completes", |presence| presence.state == protocol::PresenceToolRunState::Complete);
    let first = escape_identity(&mut app).expect("a live run");
    let raised = law["raised"].as_u64().expect("raised");
    set_fill_count(&mut app, raised);
    pump(&mut app, "the reconfigured run completes", |presence| presence.state == protocol::PresenceToolRunState::Complete && presence.completed == raised);
    let second = escape_identity(&mut app).expect("the same run is still live");
    assert_eq!(second["runId"], first["runId"], "a count change resumes the SAME run");
    assert!(second["generation"].as_u64() > first["generation"].as_u64(), "under a newer generation");
    assert_eq!(committed(&app), before, "a reconfigured run still committed nothing");
    close_app(&mut app);
}

/// 🪜️ LAW: lowering the count during a run retracts the TAIL of the SAME run — the placements below the new
/// count stay exactly as they were, so a lower is a retraction, never a restart.
#[test]
fn lowering_the_fill_count_during_a_run_retracts_the_tail_of_the_same_run() {
    let law = &fixture()["laws"]["reconfigure"];
    let mut app = app_with_registry();
    let requested = law["raised"].as_u64().expect("raised");
    let lowered = law["lowered"].as_u64().expect("lowered");
    set_fill_count(&mut app, requested);
    let before = committed(&app);
    start(&mut app);
    pump(&mut app, "the first run completes", |presence| presence.state == protocol::PresenceToolRunState::Complete);
    let first = escape_identity(&mut app).expect("a live run");
    assert_eq!(provisional_instances(&mut app).1, requested as usize, "the run holds what was requested");
    set_fill_count(&mut app, lowered);
    pump(&mut app, "the lowered run settles", |presence| presence.state == protocol::PresenceToolRunState::Complete && presence.completed == lowered);
    let second = escape_identity(&mut app).expect("the same run is still live");
    assert_eq!(second["runId"], first["runId"], "a lower retracts within the SAME run");
    assert_eq!(provisional_instances(&mut app).1, lowered as usize, "only the tail above the new count is retracted");
    assert_eq!(committed(&app), before, "a retracted run still committed nothing");
    close_app(&mut app);
}

/// 🧱️ LAW: a document past the planner's fixed object capacity (capsule-dream, 2 880 parts against the 3d
/// planner's `DOCUMENT_OBJECT_SLOTS` = 2 048) REFUSES visibly — a `danger` step carrying the artifact-capacity
/// reason before the declared preparation-capacity fault — instead of silently truncating the plan.
#[test]
fn a_document_past_the_planner_capacity_refuses_with_a_visible_danger_step() {
    let law = &fixture()["laws"]["interactive"];
    let refusal = law["capacityRefusals"]["capsule-dream"].as_str().expect("the capsule dream refusal");
    let mut mirror = Mirror::default();
    let mut job = job(&example("capsule-dream"), law["requested"].as_u64().expect("requested") as u32, ToolRunJobPurpose::Run, &[]);
    let mut sequence = 0;
    for _ in 0..FILL_RUN_TURNS {
        if mirror.turn(&mut job, StepBudget::new(semio_framework_job::INTERACTIVE_LANE_FUEL, u64::MAX), never, &mut sequence) {
            break;
        }
    }
    assert_eq!(mirror.fault.as_deref(), Some(refusal), "the run refuses the oversized document by its declared capacity code");
    assert!(
        mirror.steps.iter().any(|step| step.kind == semio_framework_tool_run::ToolRunStepKind::Danger && step.reason == FillRunReason::ArtifactCapacity.code()),
        "the refusal is a visible danger step before the fault"
    );
    assert!(mirror.ops.is_empty(), "a refused run places nothing");
}
//#endregion ⏯️App

