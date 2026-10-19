/// ⏯️ Fill run laws: the language-neutral run fixture, the parry3d oracle, the trace delivery, the interactive ceilings, resume and checkpoint replay, the capacity danger step, revalidation and the own-mesh bodies.

//#region ⏯️FillRunJob
use crate::Puzzle3dPlaySnapshot;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_framework_tool_run::ToolRunId;
use semio_framework_tool_run::ToolRunTraceCursor;
use semio_framework_tool_run::ToolRunTraceStore;
use semio_framework_tool_run::TOOL_RUN_TRACE_PAGE_BYTES_MAX;

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
        "nakagin" => std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio")).expect("committed example reads"),
        "concrete-forest" => std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio")).expect("committed example reads"),
        other => panic!("unknown example document {other}"),
    };
    let snapshot = semio_s_artifact_puzzle_3d_model::io::text::snapshot::parse_dsl(&text).expect("example parses");
    let envelope = semio_s_artifact_puzzle_3d_editor_snapshot::scene_from_snapshot(&snapshot, Default::default(), "fill");
    let mut scene: SceneConfig = semio_framework_value::FromValue::from_value(semio_s_artifact_puzzle_3d_editor_session::scene_config_value(&envelope)).expect("scene config decodes");
    scene.seed = seed;
    let fallback = semio_framework_plugin::mesh_from_kind(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FALLBACK_MESH_KIND);
    let positions: Vec<f32> = fallback.positions.iter().map(|value| value * FILL_RUN_BOX_SCALE).collect();
    let body = collision_body_from_buffers(&positions, &fallback.indices).expect("fallback body");
    let mut lane = semio_s_artifact_puzzle_3d_editor_snapshot::collect_mesh_urls(&envelope.scene_snapshot);
    lane.push(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FALLBACK_MESH_KIND.to_string());
    lane.sort();
    lane.dedup();
    let meshes = lane.iter().map(|url| (url.clone(), body.clone())).collect();
    (FillPreparationRoots::new(Arc::new(scene), Arc::new(meshes)), lane, positions)
}

fn fill_run_job(roots: FillPreparationRoots, lane: Vec<String>, seed: u64, requested: usize) -> FillRunFixture {
    fill_run_job_on_clock(roots, lane, seed, requested, never)
}

fn fill_run_job_on_clock(roots: FillPreparationRoots, lane: Vec<String>, seed: u64, requested: usize, clock: fn() -> Option<u64>) -> FillRunFixture {
    FillRunFixture::new(FillRunJob::new(FillBuilder::begin_preparation(roots, Operation::new(OperationId(71), RevisionId(1), Generation(1), seed), requested), fill_run_identity(), lane, [0; 32]), clock)
}

/// 🚦️ What one run job turn handed its driver, with every retained page returned to its ledger.
#[derive(Debug)]
enum FillRunTurn {
    Tick(ToolRunTick),
    Checkpoint(Vec<u8>),
    Complete,
    Yield,
}

fn settle_fill_run_outcome(turn: JobTurn) -> FillRunTurn {
    match turn {
        JobTurn::Preview(page) => {
            assert!(page.len() <= semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES && page.len() <= TOOL_RUN_TRACE_PAGE_BYTES_MAX);
            FillRunTurn::Tick(ToolRunTick::decode(&page).expect("tick decodes"))
        }
        JobTurn::Checkpoint { state, .. } => FillRunTurn::Checkpoint(state),
        JobTurn::Complete => FillRunTurn::Complete,
        JobTurn::Yield => FillRunTurn::Yield,
        other => {
            let described = format!("{other:?}");
            assert!(faulted(other), "unexpected run job outcome {described}");
            panic!("the fill run job faulted: {described}");
        }
    }
}

struct FillRunFixture {
    job: Option<FillRunJob>,
    original: Option<semio_s_plugin_puzzle_jobs::testing::OriginalFixture>,
}

impl FillRunFixture {
    fn new(job: FillRunJob, clock: fn() -> Option<u64>) -> Self {
        let operation = job.operation();
        Self { job: Some(job), original: Some(semio_s_plugin_puzzle_jobs::testing::OriginalFixture::new(operation.operation, operation.generation, semio_s_plugin_puzzle_jobs::testing::fixture_stock(), clock, u64::MAX, 1 << 48)) }
    }

    fn advance(&mut self, fuel: u64, deadline: u64) -> JobTurn {
        let job = self.job.as_mut().expect("original fill run remains live");
        self.original.as_mut().expect("original fill caller remains live").turn(semio_s_plugin_puzzle_jobs::testing::fixture_stock(), |cx| {
            let mut output=None;
            cx.with_original_work_loan(fuel,deadline,|cx|{output=Some(job.turn(cx));Ok(())})?;
            Ok(output.unwrap())
        }).expect("original fill run step")
    }

    fn drive_outcome(&mut self, fuel: u64, deadline: u64, verdict: &mut Option<semio_framework_trace::CallbackVerdict>) -> Option<JobTurn> {
        let job = self.job.as_mut().expect("original fill run remains live");
        self.original.as_mut().expect("original fill caller remains live").turn(semio_s_plugin_puzzle_jobs::testing::fixture_stock(), |cx| {
            let mut output=None;
            cx.with_original_work_loan(fuel,deadline,|cx|{output=semio_framework_job::drive_step(job,cx,"puzzle3d-fill-run",semio_framework_job::InteractiveStage::InteractiveStep,verdict)?.map(semio_s_plugin_puzzle_jobs::testing::owned_turn);Ok(())})?;
            Ok(output)
        }).expect("original fill run callback")
    }
}

impl std::ops::Deref for FillRunFixture {
    type Target = FillRunJob;
    fn deref(&self) -> &Self::Target { self.job.as_ref().expect("original fill run remains live") }
}

impl std::ops::DerefMut for FillRunFixture {
    fn deref_mut(&mut self) -> &mut Self::Target { self.job.as_mut().expect("original fill run remains live") }
}

impl Drop for FillRunFixture {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            if let Some(mut original) = self.original.take() {
                if let Some(job) = self.job.as_mut() { original.close_job(job); }
                self.job.take();
                original.finish();
            }
        }
    }
}

fn fill_run_turn(job: &mut FillRunFixture, fuel: u64) -> FillRunTurn {
    settle_fill_run_outcome(job.advance(fuel, u64::MAX))
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

    fn drive(&mut self, job: &mut FillRunFixture, fuel: u64, turns: usize) -> usize {
        let mut sequence = 0;
        for turn in 0..turns {
            match fill_run_turn(job, fuel) {
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

fn fill_run_summary(job: &FillRunFixture, mirror: &FillRunMirror, prefix: usize) -> serde_json::Value {
    let [tested, locked, collisions, rejected, marked] = job.counters();
    let stall = mirror.steps.iter().rev().find(|step| step.kind == ToolRunStepKind::Warning).and_then(|step| FillRunReason::from_code(step.reason)).map(FillRunReason::id);
    serde_json::json!({
        "verdictPrefix": mirror.verdict_words().into_iter().take(prefix).collect::<Vec<_>>(),
        "tested": tested,
        "locked": locked,
        "collisions": collisions,
        "rejected": rejected,
        "marked": marked,
        "appendOps": mirror.ops.len(),
        "appendEntities": mirror.entities.len(),
        "checkpoints": mirror.checkpoints.len(),
        "stall": stall,
    })
}

/// ⚖️ LAW (language-neutral scene_snapshot `🎞️fill-run.json`): a seeded shipped document and a requested count
/// produce exactly the declared verdict prefix, counters, op and entity counts — and the laws that hold
/// for every run: two ops and one entity per placement, one `success` per placement, one `danger` per
/// collision, one `warning` per rule refusal, ops alternating `create_object` / `connect_vortices` whose
/// entity is the created object's own id digest.
#[test]
fn fill_run_job_matches_the_language_neutral_fill_run_fixture() {
    let scene_snapshot: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run scene_snapshot");
    assert_eq!(scene_snapshot["laws"]["opsPerPlacement"].as_u64(), Some(u64::from(FILL_RUN_OPS_PER_PLACEMENT)));
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧬️schema/🔣️.json")).expect("schema");
    let vocabulary = &schema["$defs"]["Puzzle3dFillRun"]["x-semio-toolRun"];
    assert_eq!(vocabulary["stages"].as_array().map(|stages| stages.iter().filter_map(serde_json::Value::as_str).collect::<Vec<_>>()), Some(FillRunStage::ALL.iter().map(|stage| stage.id()).collect()));
    assert_eq!(vocabulary["counters"].as_array().map(|counters| counters.iter().filter_map(serde_json::Value::as_str).collect::<Vec<_>>()), Some(FillRunCounter::ALL.iter().map(|counter| counter.id()).collect()));
    let reasons: Vec<(u64, String, String)> = vocabulary["reasons"].as_array().expect("reasons").iter().map(|reason| (reason["code"].as_u64().expect("code"), reason["id"].as_str().expect("id").into(), reason["verdict"].as_str().expect("verdict").into())).collect();
    assert_eq!(reasons, FillRunReason::ALL.iter().map(|reason| (u64::from(reason.code()), reason.id().to_string(), reason.verdict().as_str().to_string())).collect::<Vec<_>>());
    assert_eq!(vocabulary["checkpoint"]["bytes"].as_u64(), Some(FillRunCheckpoint::BYTES as u64));
    let mut disagreements = Vec::new();
    for case in scene_snapshot["cases"].as_array().expect("cases") {
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
        let [tested, locked, collisions, rejected, marked] = job.counters();
        let count = |wanted: ToolRunVerdict| mirror.verdicts.iter().filter(|(_, verdict, _)| *verdict == wanted).count() as u64;
        assert_eq!((count(ToolRunVerdict::Success), count(ToolRunVerdict::Danger), count(ToolRunVerdict::Warning)), (locked, collisions + marked, rejected));
        assert_eq!(mirror.verdicts.len() as u64, tested + marked, "every constructed candidate reached exactly one verdict and every marked vortex one danger record");
        let candidates = mirror.trace.len() as u64 - marked;
        assert!(mirror.trace.len() as u64 >= marked && candidates <= 1, "only the last tested candidate stays resident next to every marked vortex: {candidates} candidates, {marked} marked");
        for (index, pair) in mirror.ops.chunks(2).enumerate() {
            let Ok(Puzzle3dMutation::CreateObject(create)) = semio_s_artifact_puzzle_3d_mutations::io::binary::mutations::decode_op(&pair[0]) else { panic!("op {} is create_object", 2 * index) };
            let Ok(Puzzle3dMutation::ConnectVortices(connect)) = semio_s_artifact_puzzle_3d_mutations::io::binary::mutations::decode_op(&pair[1]) else { panic!("op {} is connect_vortices", 2 * index + 1) };
            assert_eq!(mirror.entities[index], fill_run_entity(&create.object.id));
            assert_eq!(connect.attracted.split(':').next(), Some(create.object.id.as_str()), "the attraction docks the created object");
            assert_eq!(create.object.id, job.builder().appended_objects[index].id);
        }
        let progress = mirror.progress.as_ref().expect("progress");
        assert_eq!((progress.state, progress.completed, progress.total), (ToolRunState::Complete, locked, Some(requested as u64)));
        assert_eq!(progress.counters.iter().map(|counter| counter.value).collect::<Vec<_>>(), vec![tested, locked, collisions, rejected, marked]);
    }
    assert!(disagreements.is_empty(), "the fill run scene_snapshot disagrees:\n{}", disagreements.join("\n"));
}

fn parry_hull(pose: &Pose3d, positions: &[f32]) -> parry3d::shape::ConvexPolyhedron {
    let points: Vec<parry3d::math::Point<f32>> = positions
        .chunks(3)
        .map(|vertex| {
            let world = pose.transform_point(&semio_s_artifact_puzzle_3d_precompute_geometry::Point3d::new(vertex[0], vertex[1], vertex[2]));
            parry3d::math::Point::new(world.x(), world.y(), world.z())
        })
        .collect();
    parry3d::shape::ConvexPolyhedron::from_convex_hull(&points).expect("box hull")
}

/// 🧥️ The mesh identity every second body of an own-mesh document variant carries instead of its kind's.
const FILL_RUN_OWN_MESH_URL: &str = "/own-mesh/body.glb";

/// 🧥️ A shipped example document in which every second body carries its own mesh `FILL_RUN_OWN_MESH_URL`, `scale`
/// times the box its kind renders; answers the roots, the mesh lane and each mesh identity's raw positions.
fn own_mesh_fill_roots(document: &str, seed: u32, scale: f32) -> (FillPreparationRoots, Vec<String>, HashMap<String, Vec<f32>>) {
    let (roots, mut lane, positions) = example_fill_roots(document, seed);
    let mut scene = (*roots.scene).clone();
    for object in scene.scene_snapshot.objects.iter_mut().step_by(2) {
        object.mesh_url = Some(FILL_RUN_OWN_MESH_URL.into());
    }
    let own: Vec<f32> = positions.iter().map(|value| value * scale).collect();
    let fallback = semio_framework_plugin::mesh_from_kind(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FALLBACK_MESH_KIND);
    let mut meshes = (*roots.meshes).clone();
    meshes.cold_insert(FILL_RUN_OWN_MESH_URL.into(), collision_body_from_buffers(&own, &fallback.indices).expect("own mesh body"));
    lane.push(FILL_RUN_OWN_MESH_URL.into());
    lane.sort();
    lane.dedup();
    let mut by_url: HashMap<String, Vec<f32>> = lane.iter().map(|url| (url.clone(), positions.clone())).collect();
    by_url.insert(FILL_RUN_OWN_MESH_URL.into(), own);
    (FillPreparationRoots::new(Arc::new(scene), Arc::new(meshes)), lane, by_url)
}

/// ⚖️ What one oracle comparison decided.
#[derive(Default)]
struct FillRunOracleTally {
    decisive: usize,
    ambiguous: usize,
    collisions: usize,
    fits: usize,
    disagreements: Vec<String>,
}

/// ⚖️ ORACLE (`parry3d`): every candidate the run marked `danger` (solid overlap) or `success` (fits) — the first `records` of
/// them — is recomputed against EVERY body placed before it, the docking host included: the document's own bodies plus the
/// run's earlier placements, as exact convex hulls whose penetration depth `parry3d::query::contact` measures. A document
/// body's hull is built from the mesh it renders, its own `meshUrl` first, independently of the planner's resolution. The
/// planner collides when one pair's surface dives deeper than the scene's contact tolerance; a verdict is decisive when some
/// pair penetrates at least twice the tolerance (collision) or every pair stays within half of it (fit).
fn fill_run_parry3d_tally(roots: FillPreparationRoots, lane: Vec<String>, positions: &HashMap<String, Vec<f32>>, seed: u64, requested: usize, records: usize) -> FillRunOracleTally {
    let scene = roots.scene.clone();
    let tolerance = scene.contact_tolerance;
    let mut job = fill_run_job(roots, lane, seed, requested);
    let mut mirror = FillRunMirror::new();
    mirror.drive(&mut job, u64::MAX, 1_000_000);
    let placed = &job.builder().placed;
    let base = placed.len() - job.builder().sequence.len();
    assert_eq!(base, scene.scene_snapshot.objects.len(), "every document body is a collision body");
    let box_positions = &positions[semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FALLBACK_MESH_KIND];
    let rendered = |url: Option<&str>| url.map(str::trim).filter(|url| !url.is_empty()).and_then(|url| positions.get(url)).unwrap_or(box_positions);
    let bodies: Vec<(String, parry3d::shape::ConvexPolyhedron)> = scene
        .scene_snapshot
        .objects
        .iter()
        .map(|object| (object.id.clone(), parry_hull(&pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale), rendered(object.mesh_url.as_deref()))))
        .chain(placed[base..].iter().map(|entry| (entry.object_id.clone(), parry_hull(&entry.world, rendered(Some(&entry.mesh_url))))))
        .collect();
    let identity = parry3d::math::Isometry::identity();
    let mut tally = FillRunOracleTally::default();
    for record in job.verdicts.iter().filter(|record| matches!(record.reason, FillRunReason::SolidOverlap | FillRunReason::Fits)).take(records) {
        let candidate = parry_hull(&pose_isometry(record.origin, record.orientation, &None), rendered(Some(&record.mesh_url)));
        let center = parry3d::shape::Shape::compute_local_aabb(&candidate).center();
        let deepest = bodies[..base + record.placements_before]
            .iter()
            .map(|(_, other)| {
                if parry3d::query::PointQuery::contains_point(other, &identity, &center) {
                    return f64::INFINITY;
                }
                parry3d::query::contact(&identity, &candidate, &identity, other, 0.0).ok().flatten().map_or(0.0, |contact| f64::from(-contact.dist))
            })
            .fold(0.0, f64::max);
        let (collides, fits) = (deepest >= 2.0 * tolerance, deepest <= 0.5 * tolerance);
        let ours = record.reason == FillRunReason::SolidOverlap;
        tally.collisions += usize::from(ours);
        tally.fits += usize::from(!ours);
        if !collides && !fits {
            tally.ambiguous += 1;
            continue;
        }
        tally.decisive += 1;
        if ours != collides {
            tally.disagreements.push(format!("candidate {} ours={:?} parry penetration {deepest:.4} m (tolerance {tolerance} m)", record.key, record.reason));
        }
    }
    tally
}

/// ⚖️ ORACLE (`parry3d`, see [`fill_run_parry3d_tally`]): the shipped Concrete Forest law and the own-mesh variants
/// of Concrete Forest and Nakagin, whose bodies render meshes that differ from their kinds'. Every decisive verdict
/// of every document agrees, each document keeps at most one in ten verdicts inside the tolerance band, and the
/// documents together decide both collisions and fits.
#[test]
fn fill_run_job_collision_verdicts_agree_with_the_parry3d_oracle() {
    let scene_snapshot: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run scene_snapshot");
    let oracle = &scene_snapshot["laws"]["parryOracle"];
    let (mut collisions, mut fits) = (0, 0);
    let mut documents = vec![(oracle, false)];
    documents.extend(oracle["ownMeshVariants"].as_array().expect("own mesh variants").iter().map(|variant| (variant, true)));
    for (law, own_mesh) in documents {
        let (document, seed, requested) = (law["document"].as_str().expect("document"), law["seed"].as_u64().expect("seed"), law["requested"].as_u64().expect("requested") as usize);
        let (roots, lane, positions) = if own_mesh {
            own_mesh_fill_roots(document, seed as u32, law["scale"].as_f64().expect("own mesh scale") as f32)
        } else {
            let (roots, lane, positions) = example_fill_roots(document, seed as u32);
            let by_url = lane.iter().map(|url| (url.clone(), positions.clone())).collect();
            (roots, lane, by_url)
        };
        let tally = fill_run_parry3d_tally(roots, lane, &positions, seed, requested, law["records"].as_u64().map_or(usize::MAX, |records| records as usize));
        let name = format!("{document} seed {seed}{}", if own_mesh { " own-mesh" } else { "" });
        assert!(tally.disagreements.is_empty(), "{name}: {} of {} decisive verdicts disagree with parry3d:\n{}", tally.disagreements.len(), tally.decisive, tally.disagreements.join("\n"));
        assert!(tally.decisive > 0 && tally.ambiguous * 10 <= tally.decisive, "{name}: at most one in ten verdicts may fall inside the tolerance band: {} of {}", tally.ambiguous, tally.decisive);
        collisions += tally.collisions;
        fits += tally.fits;
    }
    assert!(collisions > 0 && fits > 0, "the oracle documents must decide both collisions ({collisions}) and fits ({fits})");
}

/// ⚖️ LAW: a long run (at least the scene_snapshot's `candidates` tested) delivers every trace record — the ledger's resident
/// store and a renderer that only ever reads byte-budgeted deltas through its echoed cursor hold exactly the key set the
/// job reported (every marked vortex and the ONE candidate still on screen — each newly tested candidate retires the
/// previous one), with the verdict the job reported last.
#[test]
fn fill_run_job_delivers_every_trace_record_of_a_long_run() {
    let scene_snapshot: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run scene_snapshot");
    let law = &scene_snapshot["laws"]["delivery"];
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
        match fill_run_turn(&mut job, 64) {
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
    assert!(expected.len() as u64 >= job.counters()[4] && expected.len() as u64 <= job.counters()[4] + 1, "marked vortices plus at most the current candidate stay resident: {} records, counters {:?}", expected.len(), job.counters());
    assert_eq!(keys(&ledger.trace), expected, "the ledger holds every reported record");
    assert_eq!(keys(&renderer), expected, "a cursor-driven renderer holds every reported record");
}

/// 🪞️ The tool run ledger's overlay append of one op (`📋️tool-run-contract.md` §2.7.2): decode, diff against the
/// running overlay, apply — `None` for an op the overlay no longer admits.
fn fold_overlay_op(overlay: &Puzzle3dPlaySnapshot, bytes: &[u8]) -> Option<Puzzle3dPlaySnapshot> {
    let op = <Puzzle3dMutation as protocol::OpBinary>::decode_op(bytes).ok()?;
    let outcome = protocol::Mutation::<Puzzle3dPlaySnapshot>::diff(&op, overlay);
    if !outcome.is_applicable(protocol::MergePolicy::default()) {
        return None;
    }
    protocol::apply_diff_cold(outcome.diff(), overlay).ok()
}

/// ⏱️ LAW (red→green row 1, the successor of the deleted `fillBuildTick` law): on the shipped Nakagin document the fill run
/// job is driven with the thread's primitive-work meter as its clock ([`precompute_work_clock`]) and a slice of
/// `budgetWork` units, over at least `turns` turns: (a) every `drive_step` stays inside `stepWorkCeiling` units — no unit
/// between two deadline checks outgrows the slice — and (b) every tick appends at most `appendOpsCeiling` provisional ops,
/// each of which folds onto the running overlay (the O(k) append the tool run ledger performs). The counts are
/// deterministic: the same turns, ticks and costs on an idle and a saturated machine.
#[test]
fn fill_run_job_step_and_overlay_append_stay_below_the_interactive_ceiling_for_nakagin() {
    let scene_snapshot: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run scene_snapshot");
    let law = &scene_snapshot["laws"]["interactive"];
    let number = |key: &str| law[key].as_u64().unwrap_or_else(|| panic!("interactive law {key}"));
    let (budget, ceiling, append_ceiling, minimum_turns, seed) = (number("budgetWork"), number("stepWorkCeiling"), number("appendOpsCeiling") as usize, number("turns") as usize, number("seed"));
    let (roots, lane, _) = example_fill_roots(law["document"].as_str().expect("document"), seed as u32);
    let mut overlay = Puzzle3dPlaySnapshot::new(semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(&semio_s_artifact_puzzle_3d_model::io::text::snapshot::parse_dsl(&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio")).expect("committed example reads")).expect("example parses"))).expect("typed fixture admits"));
    let mut job = fill_run_job_on_clock(roots, lane, seed, number("requested") as usize,precompute_work_clock);
    let mut verdict=None;
    let (mut worst, mut worst_turn, mut turns, mut worst_append, mut appends) = (0_u64, 0_usize, 0_usize, 0_usize, 0_usize);
    loop {
        let start = precompute_work_done();
        let outcome = job.drive_outcome(semio_framework_job::INTERACTIVE_LANE_FUEL, start + budget, &mut verdict);
        let work = precompute_work_done() - start;
        turns += 1;
        if work > worst {
            (worst, worst_turn) = (work, turns);
        }
        let Some(outcome) = outcome else { continue };
        match settle_fill_run_outcome(outcome) {
            FillRunTurn::Complete => break,
            FillRunTurn::Tick(tick) if !tick.append_ops.is_empty() => {
                worst_append = worst_append.max(tick.append_ops.len());
                for bytes in &tick.append_ops {
                    overlay = fold_overlay_op(&overlay, bytes).expect("every appended op applies onto the overlay it was planned against");
                }
                appends += 1;
            }
            _ => {}
        }
    }
    let counters = job.counters();
    assert!(appends > 0, "the measured run appended at least one tick of provisional ops");
    assert!(worst_append <= append_ceiling, "one tick appended {worst_append} provisional ops, over {append_ceiling}");
    assert!(turns >= minimum_turns, "the law measures at least {minimum_turns} turns, the run took {turns}");
    assert!(counters[1] > 0 && counters[0] > counters[1], "the measured run tested and placed objects: {counters:?}");
    assert!(worst <= ceiling, "fill run job worst drive_step cost {worst} units at turn {worst_turn} of {turns}, over {ceiling} (slice {budget})");
}

/// ⚖️ LAW: with one unit of fuel a run job step publishes exactly one visible unit — the `testing` upsert of the
/// candidate it just constructed, or that candidate's final verdict — so every tested candidate is on screen as
/// `testing` in a tick of its own before the tick that marks it (collision, fit or refusal).
#[test]
fn fill_run_job_step_with_one_unit_of_fuel_shows_each_candidate_before_its_verdict() {
    let mut job = FillRunFixture::new(FillRunJob::new(FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(73), RevisionId(1), Generation(1), 43), 24), fill_run_identity(), vec![NAKAGIN_MESH_URL.to_string()], [0; 32]), never);
    let mut sequence = 0;
    let (mut shown, mut verdicts) = (0u64, 0u64);
    let mut under_test: Option<u64> = None;
    for _ in 0..1_000_000 {
        match fill_run_turn(&mut job, 1) {
            FillRunTurn::Tick(tick) => {
                let ops: Vec<&ToolRunTraceOp> = tick.trace.iter().flat_map(|page| &page.ops).collect();
                let finals: Vec<u64> = ops.iter().filter_map(|op| match op {
                    ToolRunTraceOp::Upsert { key, verdict, .. } if *verdict != ToolRunVerdict::Testing => Some(*key),
                    _ => None,
                }).collect();
                let testing: Vec<u64> = ops.iter().filter_map(|op| match op {
                    ToolRunTraceOp::Upsert { key, verdict: ToolRunVerdict::Testing, .. } => Some(*key),
                    _ => None,
                }).collect();
                assert!(finals.len() + testing.len() <= 1, "one fuel unit is one visible unit, tick {} carried testing {testing:?} and verdicts {finals:?}", tick.sequence);
                if let Some(&key) = testing.first() {
                    under_test = Some(key);
                    shown += 1;
                }
                if let Some(&key) = finals.first() {
                    assert_eq!(under_test.take(), Some(key), "tick {} marks a candidate an earlier tick showed under test", tick.sequence);
                    verdicts += 1;
                }
            }
            FillRunTurn::Checkpoint(_) | FillRunTurn::Yield => {}
            FillRunTurn::Complete => break,
        }
    }
    assert_eq!((shown, verdicts), (job.counters()[0], job.counters()[0]), "every tested candidate was shown, then marked");
    assert_eq!(job.counters()[1], 24);
}

/// ⚖️ LAW: the run shows ONE candidate at a time. After every tick the resident trace holds the marked vortices and at
/// most one candidate record — the newest tested one, `testing` while under test, then its collision (`danger`) or fit
/// (`success`) — because each newly constructed candidate retires the previous one. A placement stays visible as its
/// provisional document instance, never as a pile of past collision records.
#[test]
fn fill_run_job_keeps_only_the_current_candidate_on_screen() {
    let (roots, lane, _) = example_fill_roots("concrete-forest", 3);
    let mut job = fill_run_job(roots, lane, 3, 40);
    let mut mirror = FillRunMirror::new();
    let mut sequence = 0;
    let mut newest: Option<u64> = None;
    let (mut ticks, mut danger_shown) = (0usize, false);
    for _ in 0..1_000_000 {
        match fill_run_turn(&mut job, 8) {
            FillRunTurn::Tick(tick) => {
                for op in tick.trace.iter().flat_map(|page| &page.ops) {
                    if let ToolRunTraceOp::Upsert { key, reason, .. } = op {
                        if *reason != FillRunReason::VortexExhausted.code() {
                            newest = Some(newest.map_or(*key, |current| current.max(*key)));
                        }
                    }
                }
                mirror.apply(tick);
                ticks += 1;
                let candidates: Vec<(u64, ToolRunVerdict)> = mirror.trace.records().filter(|(_, record)| record.reason != FillRunReason::VortexExhausted.code()).map(|(key, record)| (key, record.verdict)).collect();
                assert!(candidates.len() <= 1, "tick {ticks}: only the current candidate may be on screen, resident {candidates:?}");
                if let Some((key, verdict)) = candidates.first() {
                    assert_eq!(Some(*key), newest, "tick {ticks}: the resident candidate is the newest tested one");
                    danger_shown |= *verdict == ToolRunVerdict::Danger;
                }
            }
            FillRunTurn::Checkpoint(bytes) => mirror.checkpoints.push(bytes),
            FillRunTurn::Yield => {}
            FillRunTurn::Complete => break,
        }
    }
    let [tested, locked, collisions, _, marked] = job.counters();
    assert_eq!(locked, 40);
    assert!(marked > 0, "the law needs a marked vortex next to the current candidate");
    assert!(collisions > 0 && danger_shown, "the law needs a colliding candidate on screen ({collisions} collisions)");
    assert!(tested > locked + 1, "the law needs more tested candidates than the one on screen ({tested} tested)");
    assert_eq!(mirror.trace.records().filter(|(_, record)| record.reason == FillRunReason::VortexExhausted.code()).count() as u64, marked, "every marked vortex stays marked");
}

fn nakagin_scale_run(requested: usize) -> (FillRunFixture, FillRunMirror) {
    let mut job = FillRunFixture::new(FillRunJob::new(FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(79), RevisionId(1), Generation(1), 43), requested), fill_run_identity(), vec![NAKAGIN_MESH_URL.to_string()], [0; 32]), never);
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
    assert!(
        lowered_mirror.trace.records().filter(|(_, record)| record.verdict == ToolRunVerdict::Success).all(|(key, _)| lowered.placement_keys.iter().any(|(kept, _)| *kept == key)),
        "no retracted placement's success record stays resident"
    );

    let mut foreign = FillRunCheckpoint::decode(&checkpoint).expect("checkpoint");
    foreign.next_key += 1_000;
    assert_eq!(lowered.resume(&foreign.encode(), SHORT), Err(FillRunResumeError::Foreign));
    assert_eq!(lowered.resume(&checkpoint[..8], SHORT), Err(FillRunResumeError::Malformed));
}

fn nakagin_scale_builder(operation: u64, requested: usize) -> FillBuilder {
    FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(operation), RevisionId(1), Generation(1), 43), requested)
}

/// ⏪️ LAW: a run job rebuilt from its predecessor's last checkpoint — the framework closes a run's job on
/// every settings change and builds a new one, whose tick sequence starts over under the newer generation —
/// replays silently to that checkpoint, with no tick leaving it
/// on the way, and then continues exactly like the resident job: a raise appends the long run's ops and
/// verdicts, a lower retracts to the lowered count. A job whose inputs differ restarts instead: its first tick
/// clears the trace and retracts every provisional op, and it then runs like a fresh run.
#[test]
fn fill_run_job_rebuilt_from_a_checkpoint_replays_silently_and_continues_like_the_resident_job() {
    const SHORT: usize = 30;
    const LONG: usize = 45;
    const LOWERED: usize = 12;
    let lane = || vec![NAKAGIN_MESH_URL.to_string()];
    let (_, mut raised) = nakagin_scale_run(SHORT);
    let checkpoint = FillRunCheckpoint::decode(raised.checkpoints.last().expect("checkpoint")).expect("checkpoint decodes");
    let provisional = raised.ops.len() as u32;
    let mut rebuilt = FillRunFixture::new(FillRunJob::replaying(nakagin_scale_builder(81, checkpoint.requested as usize), fill_run_identity(), lane(), checkpoint, LONG, provisional), never);
    raised.last_sequence = None;
    raised.drive(&mut rebuilt, u64::MAX, 1_000_000);
    let (long, long_mirror) = nakagin_scale_run(LONG);
    assert!(raised.retractions.is_empty(), "a raise retracts nothing");
    assert_eq!(raised.ops, long_mirror.ops, "the rebuilt run appends exactly the long run's ops after its predecessor's");
    assert_eq!(raised.entities, long_mirror.entities);
    assert_eq!(raised.verdicts, long_mirror.verdicts, "the replay published no verdict twice and missed none");
    assert_eq!(rebuilt.counters(), long.counters());

    let (_, mut lowered) = nakagin_scale_run(SHORT);
    let before = lowered.ops.clone();
    let mut rebuilt = FillRunFixture::new(FillRunJob::replaying(nakagin_scale_builder(82, checkpoint.requested as usize), fill_run_identity(), lane(), checkpoint, LOWERED, provisional), never);
    lowered.last_sequence = None;
    lowered.drive(&mut rebuilt, u64::MAX, 1_000_000);
    assert_eq!(lowered.retractions.iter().min().copied(), Some(LOWERED as u32 * FILL_RUN_OPS_PER_PLACEMENT), "the rebuilt lower retracts to exactly the lowered count");
    assert_eq!(lowered.ops.as_slice(), &before[..LOWERED * FILL_RUN_OPS_PER_PLACEMENT as usize]);
    assert!(
        lowered.trace.records().filter(|(_, record)| record.verdict == ToolRunVerdict::Success).all(|(key, _)| rebuilt.placement_keys.iter().any(|(kept, _)| *kept == key)),
        "no retracted placement's success record stays resident"
    );

    let (_, mut restarted) = nakagin_scale_run(SHORT);
    let mut foreign = checkpoint;
    foreign.inputs = [9; 32];
    let mut rebuilt = FillRunFixture::new(FillRunJob::restarting(nakagin_scale_builder(83, LOWERED), fill_run_identity(), lane(), foreign.inputs, provisional), never);
    restarted.last_sequence = None;
    restarted.drive(&mut rebuilt, u64::MAX, 1_000_000);
    let (_, fresh) = nakagin_scale_run(LOWERED);
    assert_eq!(restarted.retractions.first().copied(), Some(0), "a restart retracts every provisional op first");
    assert_eq!((restarted.ops.clone(), restarted.entities.clone()), (fresh.ops.clone(), fresh.entities.clone()), "and then runs like a fresh run");
    assert_eq!(restarted.trace.len(), fresh.trace.len(), "its first tick cleared the predecessor's trace");
}

/// 🚧️ LAW (P4e): a document too large for the planner's fixed pages publishes ONE `danger` step naming the
/// refused capacity through the run's tick, and the next step faults the run — nothing was placed.
#[test]
fn fill_run_job_capacity_refusal_publishes_a_danger_step_before_faulting() {
    let objects = (0..=DOCUMENT_OBJECT_SLOTS).map(|index| EngineSceneObject { id: format!("rejected-{index:04}"), object_kind: None, anchor: Default::default(), mesh_url: None, origin: [0.0; 3], orientation: None, scale: None, vortices: Vec::new() }).collect();
    let scene = Arc::new(SceneConfig { scene_snapshot: EngineSceneSnapshot { objects, attractions: Vec::new(), target_volumes: Vec::new() }, kind_catalogs: Some(KindCatalogBundle::default()), kind_compatibility: Vec::new(), contact_tolerance: 0.0, seed: 37, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() });
    let builder = FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(FillMeshMap::new())), Operation::new(OperationId(37), RevisionId(9), Generation(11), 37), TEST_REQUESTED_COUNT);
    let mut job = FillRunFixture::new(FillRunJob::new(builder, fill_run_identity(), Vec::new(), [0; 32]), never);
    let mut sequence = 0;
    let FillRunTurn::Tick(tick) = fill_run_turn(&mut job, u64::MAX) else { panic!("the refusal is published as a tick first") };
    assert!(tick.append_ops.is_empty(), "nothing is placed");
    assert_eq!(tick.steps.iter().map(|step| (step.kind, step.reason, step.args.clone())).collect::<Vec<_>>(), vec![(ToolRunStepKind::Danger, FillRunReason::ArtifactCapacity.code(), vec![ToolRunStepArg::Unsigned(DOCUMENT_OBJECT_SLOTS as u64)])]);
    let operation = job.operation();
    let JobTurn::Fault(fault) = job.advance(u64::MAX, u64::MAX) else { panic!("the step after the danger step faults the run") };
    assert_eq!(fault.as_slice(), format!("preparation-capacity:scene_snapshot-objects:{DOCUMENT_OBJECT_SLOTS}").as_bytes());
    assert!(faulted(JobTurn::Fault(fault)));
}

/// 🧱️ LAW: the placements a finalize revalidation rebuilds from the ledger's provisional ops alone are the
/// run's own placements, in op order — same object, attraction, entity and mesh-lane subject — keyed
/// consecutively from the allocated first key; a list that is not whole placements is refused.
#[test]
fn fill_run_placements_rebuild_the_provisional_placements_from_their_ops() {
    let (roots, lane, _) = example_fill_roots("concrete-forest", 7);
    let mut job = fill_run_job(roots, lane.clone(), 7, 5);
    let mut mirror = FillRunMirror::new();
    mirror.drive(&mut job, u64::MAX, 1_000_000);
    let ops: Vec<Puzzle3dMutation> = mirror.ops.iter().map(|bytes| <Puzzle3dMutation as protocol::OpBinary>::decode_op(bytes).expect("op decodes")).collect();
    let rebuilt = fill_run_placements(&ops, job.mesh_lane(), REVALIDATE_FIRST_KEY).expect("whole placements");
    let original = job.provisional_placements();
    assert_eq!(rebuilt.len(), 5);
    for (index, (rebuilt, original)) in rebuilt.iter().zip(&original).enumerate() {
        assert_eq!((&rebuilt.object.id, &rebuilt.attraction, rebuilt.entity), (&original.object.id, &original.attraction, original.entity));
        assert_eq!(rebuilt.key, REVALIDATE_FIRST_KEY + index as u64, "placements take consecutive keys from the allocated range");
        let (ToolRunTraceSubject::Instance3d { mesh: rebuilt_mesh, .. }, ToolRunTraceSubject::Instance3d { mesh: original_mesh, .. }) = (rebuilt.subject, original.subject) else { panic!("instance subjects") };
        assert_eq!(rebuilt_mesh, original_mesh, "the rebuilt subject indexes the same mesh lane entry");
    }
    assert!(fill_run_placements(&ops[..ops.len() - 1], job.mesh_lane(), REVALIDATE_FIRST_KEY).is_none(), "half a placement is refused");
}

/// 🔑️ The first key of the range a revalidation's trace key allocator hands the placement law.
const REVALIDATE_FIRST_KEY: u64 = 4_096;

struct FillRevalidateFixture{job:Option<FillRevalidateJob>,original:Option<semio_s_plugin_puzzle_jobs::testing::OriginalFixture>}
impl FillRevalidateFixture{
 fn new(job:FillRevalidateJob,operation:Operation)->Self{Self{job:Some(job),original:Some(semio_s_plugin_puzzle_jobs::testing::OriginalFixture::new(operation.operation,operation.generation,semio_s_plugin_puzzle_jobs::testing::fixture_stock(),never,u64::MAX,1<<48))}}
 fn advance(&mut self)->JobTurn{let job=self.job.as_mut().unwrap();let mut turn=None;self.original.as_mut().unwrap().turn(semio_s_plugin_puzzle_jobs::testing::fixture_stock(),|cx|cx.with_original_work_loan(u64::MAX,u64::MAX,|cx|{turn=Some(job.turn(cx));Ok(())})).expect("same original fill revalidation turn");turn.unwrap()}
}
impl std::ops::Deref for FillRevalidateFixture{type Target=FillRevalidateJob;fn deref(&self)->&Self::Target{self.job.as_ref().unwrap()}}
impl std::ops::DerefMut for FillRevalidateFixture{fn deref_mut(&mut self)->&mut Self::Target{self.job.as_mut().unwrap()}}
impl Drop for FillRevalidateFixture{fn drop(&mut self){if !std::thread::panicking(){if let Some(mut original)=self.original.take(){original.close_job(self.job.as_mut().unwrap());self.job.take();original.finish();}}}}
fn drive_revalidation(job:&mut FillRevalidateFixture)->Vec<ToolRunTick>{
 let mut ticks=Vec::new();for _ in 0..1_000_000{match settle_fill_run_outcome(job.advance()){
  FillRunTurn::Tick(tick)=>ticks.push(tick),FillRunTurn::Complete=>return ticks,
  FillRunTurn::Checkpoint(_)|FillRunTurn::Yield=>{},
 }}panic!("revalidation did not complete");
}

/// ⚖️ LAW: revalidating provisional placements against an unchanged head keeps every one; against a head
/// that gained a body where one placement stands, that placement turns `danger` with the framework
/// conflict reason, the final tick retracts to it and re-appends every later survivor's exact ops and
/// entity, and one `danger` conflict step counts the conflicts. Its trace shows one placement at a time and none once done.
#[test]
fn fill_revalidate_job_retracts_conflicting_placements_and_reappends_survivors() {
    const PLACED: usize = 8;
    const INTRUDED: usize = 3;
    let (job, mirror) = nakagin_scale_run(PLACED);
    let placements = job.provisional_placements();
    assert_eq!(placements.len(), PLACED);
    let operation = Operation::new(OperationId(83), RevisionId(2), Generation(1), 43);
    let head = nakagin_scale_roots();
    let mut clean = FillRevalidateFixture::new(FillRevalidateJob::new(operation, fill_run_identity(), FillPreparationRoots::new(head.scene.clone(), head.meshes.clone()), placements.clone(), 1_000),operation);
    let ticks = drive_revalidation(&mut clean);
    assert!(clean.conflicts().iter().all(|conflict| !conflict) && clean.conflicts().len() == PLACED);
    assert!(ticks.iter().all(|tick| tick.retract_to.is_none() && tick.append_ops.is_empty()));
    let mut screen = ToolRunTraceStore::new(fill_run_identity());
    for tick in &ticks {
        for page in &tick.trace {
            screen.apply_ops(&page.ops);
            assert!(screen.len() <= 1, "revalidation shows one placement at a time, resident {:?}", screen.records().map(|(key, record)| (key, record.verdict)).collect::<Vec<_>>());
        }
    }
    assert_eq!(screen.len(), 0, "the finished revalidation leaves no verdict record on screen");

    let mut scene = (*head.scene).clone();
    let mut intruder = placements[INTRUDED].object.clone();
    intruder.id = "intruder".into();
    intruder.vortices.clear();
    scene.scene_snapshot.objects.push(intruder);
    let mut intruded = FillRevalidateFixture::new(FillRevalidateJob::new(operation, fill_run_identity(), FillPreparationRoots::new(Arc::new(scene), head.meshes.clone()), placements.clone(), 1_000),operation);
    let ticks = drive_revalidation(&mut intruded);
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

/// 🧊️ A cube of half extent 4 shifted `offset` along x, as raw positions and indices.
fn own_mesh_cube(offset: f32) -> (Vec<f32>, Vec<u32>) {
    (
        [-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0].iter().enumerate().map(|(index, unit)| unit * 4.0 + if index % 3 == 0 { offset } else { 0.0 }).collect(),
        vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2],
    )
}

fn own_mesh_kind(id: &str, url: &str, vortices: Vec<ObjectKindVortexTemplate>) -> ObjectKind {
    ObjectKind { id: id.into(), representations: vec![ObjectKindRepresentation { id: id.to_lowercase(), name: String::new(), url: url.into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }], scale: None, vortices }
}

/// 🧥️ The `ownMesh` law's scene (`🎞️fill-run.json`): one host vortex, two compatible kinds, and with `blocker` a
/// body of the host's kind carrying its own mesh where both candidates dock, while the kind's mesh is the same cube
/// shifted `kind_offset` away.
fn own_mesh_roots(blocker: bool, kind_offset: f32) -> FillPreparationRoots {
    let host = EngineSceneObject {
        id: "host".into(),
        object_kind: Some("Host".into()),
        anchor: Default::default(),
        mesh_url: None,
        origin: [12.0, 0.0, 0.0],
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        vortices: vec![VortexProps { id: "v0".into(), vortex_kind: Some("port-a".into()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
    };
    let mut objects = vec![host.clone()];
    if blocker {
        objects.push(EngineSceneObject { id: "blocker".into(), mesh_url: Some("/test/blocker.glb".into()), vortices: Vec::new(), ..host });
    }
    let port = || vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".into()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]), ..Default::default() }];
    let scene = SceneConfig {
        scene_snapshot: EngineSceneSnapshot { attractions: Vec::new(), target_volumes: Vec::new(), objects },
        kind_catalogs: Some(KindCatalogBundle {
            objects: vec![own_mesh_kind("Host", "/test/host-kind.glb", Vec::new()), own_mesh_kind("Near", "/test/near.glb", port()), own_mesh_kind("Late", "/test/late.glb", port())],
            vortices: vec![VortexKindCatalog { id: "port-a".into(), default_cable_kind: None, ..Default::default() }, VortexKindCatalog { id: "port-b".into(), default_cable_kind: None, ..Default::default() }],
            cables: vec![CableKindCatalog { id: "cable.link".into(), default_attraction_kind: None, ..Default::default() }],
        }),
        kind_compatibility: vec![KindCompatEntry { source: "port-b".into(), target: "port-a".into(), bidirectional: true, important: false, specificity: Some("vortex".into()) }],
        contact_tolerance: 0.02,
        seed: 1,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    };
    let body = |offset: f32| {
        let (positions, indices) = own_mesh_cube(offset);
        collision_body_from_buffers(&positions, &indices).expect("cube body")
    };
    let meshes = [("/test/host-kind.glb", body(kind_offset)), ("/test/blocker.glb", body(0.0)), ("/test/near.glb", body(0.0)), ("/test/late.glb", body(0.0))].into_iter().map(|(url, body)| (url.to_string(), body)).collect();
    FillPreparationRoots::new(Arc::new(scene), Arc::new(meshes))
}

fn own_mesh_lane() -> Vec<String> {
    ["/test/blocker.glb", "/test/host-kind.glb", "/test/late.glb", "/test/near.glb"].map(str::to_string).to_vec()
}

fn all_objects_roots(case: &serde_json::Value) -> FillPreparationRoots {
    let roots = own_mesh_roots(case["blocker"].as_bool().unwrap_or(false), case["kindMeshOffset"].as_f64().unwrap_or(100.0) as f32);
    let mut scene = (*roots.scene).clone();
    let template = scene.scene_snapshot.objects[0].vortices[0].clone();
    scene.scene_snapshot.objects[0].vortices = case["targets"].as_array().expect("targets").iter().enumerate().map(|(index, offset)| VortexProps {
        id: format!("v{index}"), position: [offset.as_f64().expect("offset"), 0.0, 0.0], ..template.clone()
    }).collect();
    for index in 0..case["farObjects"].as_u64().unwrap_or(0) {
        let host = scene.scene_snapshot.objects[0].clone();
        scene.scene_snapshot.objects.insert(1, EngineSceneObject { id: format!("far-{index}"), origin: [1000.0 + index as f64 * 20.0, 0.0, 0.0], vortices: Vec::new(), ..host });
    }
    let mut meshes = (*roots.meshes).clone();
    for missing in case["missing"].as_array().expect("missing meshes") {
        meshes.cold_remove(missing.as_str().expect("mesh url"));
    }
    FillPreparationRoots::new(Arc::new(scene), Arc::new(meshes))
}

fn all_objects_run(roots: FillPreparationRoots, requested: usize) -> FillRunFixture {
    let mut job = fill_run_job(roots, own_mesh_lane(), 1, requested);
    FillRunMirror::new().drive(&mut job, u64::MAX, 1_000_000);
    job
}

#[test]
fn fill_checks_all_objects_before_each_placement() {
    let scene_snapshot: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill law");
    for case in scene_snapshot["laws"]["allObjects"]["cases"].as_array().expect("cases") {
        let roots = all_objects_roots(case);
        let scene = roots.scene.clone();
        let job = all_objects_run(roots, case["targets"].as_array().expect("targets").len());
        let placements = job.provisional_placements();
        assert_eq!(placements.len() as u64, case["expectedLocked"].as_u64().expect("locked"), "{}", case["name"]);
        let mut peers = scene.scene_snapshot.objects.clone();
        for placement in placements {
            let hull = |object: &EngineSceneObject| {
                let offset = if object.mesh_url.as_deref().unwrap_or("/test/host-kind.glb") == "/test/host-kind.glb" { case["kindMeshOffset"].as_f64().unwrap() as f32 } else { 0.0 };
                parry_hull(&pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale), &own_mesh_cube(offset).0)
            };
            let identity = parry3d::math::Isometry::identity();
            for peer in &peers {
                assert!(!parry3d::query::intersection_test(&identity, &hull(&placement.object), &identity, &hull(peer)).expect("oracle intersection"), "{}: {} overlaps {}", case["name"], placement.object.id, peer.id);
            }
            peers.push(placement.object);
        }
    }
}

#[test]
fn fill_revalidation_checks_all_objects_with_current_meshes() {
    let scene_snapshot: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill law");
    let law = &scene_snapshot["laws"]["allObjects"];
    let roots = all_objects_roots(&law["cases"][0]);
    let placements = all_objects_run(FillPreparationRoots::new(roots.scene.clone(), roots.meshes.clone()), 2).provisional_placements();
    assert_eq!(placements.len(), 2);
    for case in law["revalidation"].as_array().expect("cases") {
        let (mut positions, indices) = own_mesh_cube(0.0);
        positions.iter_mut().for_each(|value| *value *= case["candidateScale"].as_f64().expect("scale") as f32);
        let mut meshes = (*roots.meshes).clone();
        for url in ["/test/near.glb", "/test/late.glb"] {
            meshes.cold_insert(url.into(), collision_body_from_buffers(&positions, &indices).expect("current mesh"));
        }
        for missing in case["missing"].as_array().expect("missing meshes") {
            meshes.cold_remove(missing.as_str().expect("mesh url"));
        }
        let mut oracle = Vec::new();
        let mut survivors = vec![parry_hull(&pose_isometry([12.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], &None), &own_mesh_cube(100.0).0)];
        let identity = parry3d::math::Isometry::identity();
        for placement in &placements {
            let candidate = parry_hull(&pose_isometry(placement.object.origin, placement.object.orientation.unwrap(), &placement.object.scale), &positions);
            let conflict = meshes.get("/test/host-kind.glb").is_none() || meshes.get(placement.object.mesh_url.as_ref().unwrap()).is_none()
                || survivors.iter().any(|peer| parry3d::query::intersection_test(&identity, &candidate, &identity, peer).expect("oracle intersection"));
            oracle.push(conflict);
            if !conflict {
                survivors.push(candidate);
            }
        }
        assert_eq!(serde_json::json!(oracle), case["expectedConflicts"], "oracle {}", case["name"]);
        let operation = Operation::new(OperationId(111), RevisionId(2), Generation(1), 1);
        let mut job = FillRevalidateFixture::new(FillRevalidateJob::new(operation, fill_run_identity(), FillPreparationRoots::new(roots.scene.clone(), Arc::new(meshes)), placements.clone(), 1_000),operation);
        drive_revalidation(&mut job);
        assert_eq!(job.conflicts(), oracle, "{}", case["name"]);
    }
}

/// 🐛️ Red→green (`📓️wave-W2-C.md` §7.2, language-neutral law `ownMesh` of `🎞️fill-run.json`): a placed body carrying
/// its own mesh collides with that mesh, not its kind's. The fill planner resolved document bodies by kind, so a body
/// whose kind renders a different (here shifted) mesh never blocked the candidates docking into it and they read `fits`;
/// the revalidation job re-tested provisional placements against the same wrong body. Both now resolve a placed
/// object's own mesh first, the renderer's law.
#[test]
fn fill_run_and_revalidation_collide_with_a_placed_body_carrying_its_own_mesh() {
    let scene_snapshot: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run scene_snapshot");
    let law = &scene_snapshot["laws"]["ownMesh"];
    let kind_offset = law["kindMeshOffset"].as_f64().expect("kind mesh offset") as f32;
    let run = |blocker: bool, requested: usize| {
        let mut job = FillRunFixture::new(FillRunJob::new(FillBuilder::begin_preparation(own_mesh_roots(blocker, kind_offset), Operation::new(OperationId(97), RevisionId(1), Generation(1), 1), requested), fill_run_identity(), own_mesh_lane(), [0; 32]), never);
        let mut mirror = FillRunMirror::new();
        mirror.drive(&mut job, u64::MAX, 1_000_000);
        (job, mirror)
    };
    let (clear, _) = run(false, 1);
    let placements = clear.provisional_placements();
    for case in law["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("name");
        let blocker = case["blocker"].as_bool().expect("blocker");
        let expected = &case["expected"];
        let (job, mirror) = run(blocker, case["requested"].as_u64().expect("requested") as usize);
        let [_, locked, collisions, _, _] = job.counters();
        let stall = mirror.steps.iter().rev().find(|step| step.kind == ToolRunStepKind::Warning).and_then(|step| FillRunReason::from_code(step.reason)).map(FillRunReason::id);
        let mut actual = serde_json::json!({ "verdicts": mirror.verdict_words(), "locked": locked, "collisions": collisions, "stall": stall });
        let operation = Operation::new(OperationId(98), RevisionId(2), Generation(1), 1);
        let mut revalidation = FillRevalidateFixture::new(FillRevalidateJob::new(operation, fill_run_identity(), own_mesh_roots(blocker, kind_offset), placements.clone(), 1_000),operation);
        drive_revalidation(&mut revalidation);
        actual["revalidateConflicts"] = serde_json::json!(revalidation.conflicts());
        assert_eq!(&actual, expected, "{name}");
    }
}
//#endregion ⏯️FillRunJob
