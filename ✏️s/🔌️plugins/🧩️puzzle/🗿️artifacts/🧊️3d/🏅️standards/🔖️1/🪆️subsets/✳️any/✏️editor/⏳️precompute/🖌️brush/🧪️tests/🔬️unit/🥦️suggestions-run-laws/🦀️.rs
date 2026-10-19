/// 🥦️ Suggestions run laws: the brush candidate search tool run, its language-neutral fixture, link-following, close retirement and collision verdicts.

//#region ⏯️BrushSuggestionsRun
/// ⏯️ Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS lane W2-C — the brush candidate search is a read-only
/// tool run: every candidate it tests is a trace record, every verdict reaches the instance's link, and the job
/// follows the link's target. The scene and the laws are read from `🧫️fixtures/🎞️brush-suggestions-run.json`.
use semio_s_artifact_puzzle_3d_editor_session::Puzzle3dInstanceOperationOwner;
use semio_framework_job::root_cancel_token;
use semio_framework_job::Generation;
use semio_framework_job::OperationId;
use semio_framework_job::StepBudget;
use semio_framework_tool_run::ToolRunId;
use semio_framework_tool_run::ToolRunStep;
use semio_framework_tool_run::ToolRunTick;
use semio_framework_tool_run::ToolRunTraceOp;
use semio_framework_tool_run::ToolRunTraceStore;

const BRUSH_SUGGESTIONS_RUN_FIXTURE: &str = include_str!("../../🧫️fixtures/🎞️brush-suggestions-run.json");
const BRUSH_SEARCH_TARGET: &str = "host:v0";

fn brush_search_cube() -> (Vec<f32>, Vec<u32>) {
    (
        vec![-4.0, -4.0, -4.0, 4.0, -4.0, -4.0, 4.0, 4.0, -4.0, -4.0, 4.0, -4.0, -4.0, -4.0, 4.0, 4.0, -4.0, 4.0, 4.0, 4.0, 4.0, -4.0, 4.0, 4.0],
        vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2],
    )
}

fn brush_search_kind(id: &str, url: &str) -> ObjectKind {
    ObjectKind {
        id: id.to_string(),
        representations: vec![ObjectKindRepresentation { id: id.to_lowercase(), name: String::new(), url: url.to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
        scale: None,
        vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), point: [0.0, 0.0, 4.0], direction: Some([0.0, 0.0, 1.0]), ..Default::default() }],
    }
}

/// 📦️ Half height of the app's fallback box, the host's collision body while its own mesh is unregistered.
fn brush_search_host_half_height() -> f64 {
    f64::from(semio_s_artifact_puzzle_3d_editor_session::puzzle3d_fallback_mesh_buffers().0.chunks(3).map(|vertex| vertex[2]).fold(f32::MIN, f32::max))
}

/// 🧪️ The scene_snapshot scene: one host vortex on the bottom face of the host's (fallback box) body, two compatible kinds that
/// dock flush under it, and optionally a body of the host's own kind carrying its own (larger) mesh parked at the host,
/// whose volume reaches into where both candidates dock.
fn brush_search_scene(blocker: bool) -> SceneConfig {
    let mut objects = vec![EngineSceneObject {
        id: "host".to_string(),
        object_kind: Some("Host".to_string()),
        anchor: Default::default(),
        mesh_url: Some("/test/unregistered.glb".to_string()),
        origin: [12.0, 0.0, 0.0],
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, -brush_search_host_half_height()], direction: Some([0.0, 0.0, -1.0]) }],
    }];
    if blocker {
        objects.push(EngineSceneObject { id: "blocker".to_string(), mesh_url: Some("/test/blocker.glb".to_string()), vortices: vec![], ..objects[0].clone() });
    }
    SceneConfig {
        scene_snapshot: EngineSceneSnapshot { attractions: vec![], target_volumes: vec![], objects },
        kind_catalogs: Some(KindCatalogBundle {
            objects: vec![brush_search_kind("Near", "/test/near.glb"), brush_search_kind("Late", "/test/late.glb")],
            vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None, ..Default::default() }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None, ..Default::default() }],
            cables: vec![CableKindCatalog { id: "cable.link".to_string(), default_attraction_kind: None, ..Default::default() }],
        }),
        kind_compatibility: vec![KindCompatEntry { source: "port-b".to_string(), target: "port-a".to_string(), bidirectional: true, important: false, specificity: Some("vortex".to_string()) }],
        contact_tolerance: 0.02,
        seed: 1,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    }
}

/// 🥽️ The scene_snapshot's mesh store: every registered test mesh is the cube, the host's own mesh is unknown.
fn brush_search_meshes(url: &str) -> Option<(Vec<f32>, Vec<u32>)> {
    (url != "/test/unregistered.glb").then(brush_search_cube)
}

fn brush_search_lane() -> Vec<String> {
    ["box", "/test/blocker.glb", "/test/late.glb", "/test/near.glb", "/test/unregistered.glb"].map(str::to_string).to_vec()
}

fn brush_run_identity(generation: u32) -> ToolRunIdentity {
    ToolRunIdentity { generation, ..ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [0; 32]) }
}

fn brush_run_owner(target: Option<&str>) -> ArtifactInstanceOperationOwnerHandle {
    let owner = ArtifactInstanceOperationOwnerHandle::new(Box::new(Puzzle3dInstanceOperationOwner::default()));
    brush_run_link(&owner, |link| link.hover(target.map(str::to_string)));
    owner
}

fn brush_run_link<R>(owner: &ArtifactInstanceOperationOwnerHandle, apply: impl FnOnce(&mut BrushSuggestionsLink) -> R) -> R {
    owner.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(apply(&mut owner.brush_suggestions))).expect("the brush suggestions link")
}

fn brush_run_job_on_clock(owner: &ArtifactInstanceOperationOwnerHandle, port: &ToolRunJobPort, scene: SceneConfig, lane: Vec<String>, meshes: BrushSuggestionsMeshSource, generation: u32, clock: fn() -> Option<u64>) -> BrushRunFixture {
    let job = BrushSuggestionsRunJob::new(owner.clone(), port.clone(), brush_run_identity(generation), Arc::new(scene), lane, meshes, semio_s_artifact_puzzle_3d_editor_session::puzzle3d_fallback_mesh_buffers()).expect("fallback body");
    BrushRunFixture { job: Some(job), original: Some(semio_s_plugin_puzzle_jobs::testing::OriginalFixture::new(OperationId(91), Generation(1), semio_s_plugin_puzzle_jobs::testing::fixture_stock(), clock, u64::MAX, 1 << 48)) }
}

fn brush_run_job(owner: &ArtifactInstanceOperationOwnerHandle, port: &ToolRunJobPort, scene: SceneConfig, lane: Vec<String>, meshes: BrushSuggestionsMeshSource, generation: u32) -> BrushRunFixture {
    brush_run_job_on_clock(owner, port, scene, lane, meshes, generation, brush_run_never)
}

fn brush_run_never() -> Option<u64> {
    Some(0)
}

/// 🚦️ One job step with `fuel` units and no deadline, its retained tick page decoded and returned to its ledger.
struct BrushRunFixture {
    job: Option<BrushSuggestionsRunJob<Puzzle3dInstanceOperationOwner>>,
    original: Option<semio_s_plugin_puzzle_jobs::testing::OriginalFixture>,
}

impl BrushRunFixture {
    fn advance(&mut self, fuel: u64, deadline: u64) -> JobTurn {
        let job = self.job.as_mut().expect("original brush job remains live");
        let mut outcome = None;
        self.original.as_mut().expect("original brush caller remains live").turn(semio_s_plugin_puzzle_jobs::testing::fixture_stock(), |cx| cx.with_original_work_loan(fuel, deadline, |cx| {
            outcome = Some(job.turn(cx));
            Ok(())
        })).expect("original brush step");
        outcome.expect("original brush loan returned its outcome")
    }

    fn close(&mut self) {
        if let Some(mut original) = self.original.take() {
            if let Some(job) = self.job.as_mut() { original.close_job(job); }
            self.job.take();
            original.finish();
        }
    }
}

impl Drop for BrushRunFixture {
    fn drop(&mut self) { if !std::thread::panicking() { self.close(); } }
}

fn brush_run_step(job: &mut BrushRunFixture, fuel: u64) -> Option<ToolRunTick> {
    brush_run_settle(job.advance(fuel, u64::MAX))
}

fn brush_run_settle(turn: JobTurn) -> Option<ToolRunTick> {
    match turn {
        JobTurn::Preview(page) => Some(ToolRunTick::decode(&page).expect("tick decodes")),
        JobTurn::Yield => None,
        other => panic!("a brush suggestions run only yields ticks: {other:?}"),
    }
}

/// 🪞️ The ledger side of a brush suggestions run: the resident trace, every record in order, the steps.
struct BrushRunMirror {
    trace: ToolRunTraceStore,
    records: Vec<(u64, ToolRunVerdict, u16)>,
    clears: usize,
    steps: Vec<ToolRunStep>,
    progress: Option<ToolRunProgress>,
}

impl BrushRunMirror {
    fn new(generation: u32) -> Self {
        Self { trace: ToolRunTraceStore::new(brush_run_identity(generation)), records: Vec::new(), clears: 0, steps: Vec::new(), progress: None }
    }

    fn apply(&mut self, tick: ToolRunTick) {
        for page in &tick.trace {
            for op in &page.ops {
                match op {
                    ToolRunTraceOp::Upsert { key, verdict, reason, .. } => self.records.push((*key, *verdict, *reason)),
                    ToolRunTraceOp::Clear => self.clears += 1,
                    ToolRunTraceOp::Retire { .. } => {}
                }
            }
            self.trace.apply_page(page).expect("pages of the one run and generation");
        }
        self.steps.extend(tick.steps);
        if tick.progress.is_some() {
            self.progress = tick.progress;
        }
    }

    /// 🔁️ Steps until the job waits on its port; answers the steps it took.
    fn drive(&mut self, job: &mut BrushRunFixture, port: &ToolRunJobPort, fuel: u64) -> usize {
        for step in 0..100_000 {
            if port.is_waiting() {
                return step;
            }
            if let Some(tick) = brush_run_step(job, fuel) {
                self.apply(tick);
            }
        }
        panic!("the brush suggestions run never settled");
    }

    fn finals(&self) -> Vec<(u64, ToolRunVerdict, u16)> {
        self.records.iter().copied().filter(|(_, verdict, _)| *verdict != ToolRunVerdict::Testing).collect()
    }

    fn words(&self) -> Vec<String> {
        self.finals().iter().map(|(_, verdict, reason)| format!("{}:{}", verdict.as_str(), BrushSuggestionsRunReason::ALL[usize::from(*reason)].id())).collect()
    }

    fn step_words(&self) -> Vec<String> {
        self.steps
            .iter()
            .map(|step| {
                let args = step.args.iter().map(|arg| arg.to_plain_string()).collect::<Vec<_>>().join(",");
                let base = format!("{}:{}", step.kind.as_str(), BrushSuggestionsRunReason::ALL[usize::from(step.reason)].id());
                if args.is_empty() { base } else { format!("{base}:{args}") }
            })
            .collect()
    }
}

/// ⚖️ LANGUAGE-NEUTRAL LAW: every scene_snapshot case reaches exactly its verdicts, free kinds, counters and steps; every
/// candidate is first upserted `testing` and then decided exactly once; the link holds the same result.
#[test]
fn brush_suggestions_run_matches_the_language_neutral_fixture() {
    let scene_snapshot: serde_json::Value = serde_json::from_str(BRUSH_SUGGESTIONS_RUN_FIXTURE).expect("brush suggestions run scene_snapshot");
    for case in scene_snapshot["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("name");
        let mut scene = brush_search_scene(case["blocker"].as_bool().expect("blocker"));
        for (kind, weight) in case["vortexWeights"].as_object().expect("vortex weights") {
            scene.weights.vortex_weights.cold_insert(kind.clone(), weight.as_f64().expect("weight"));
        }
        let target = case["target"].as_str().expect("target");
        let (owner, port) = (brush_run_owner(Some(target)), ToolRunJobPort::default());
        let mut job = brush_run_job(&owner, &port, scene, brush_search_lane(), brush_search_meshes, 0);
        let mut mirror = BrushRunMirror::new(0);
        mirror.drive(&mut job, &port, u64::MAX);
        let expect_words = |key: &str| case[key].as_array().expect(key).iter().map(|word| word.as_str().expect("word").to_string()).collect::<Vec<_>>();
        assert_eq!(mirror.words(), expect_words("verdicts"), "{name}: verdicts");
        assert_eq!(mirror.step_words(), expect_words("steps"), "{name}: steps");
        let progress = mirror.progress.as_ref().expect("a settled search reports its progress");
        assert_eq!(progress.counters.iter().map(|counter| counter.value).collect::<Vec<_>>(), case["counters"].as_array().expect("counters").iter().map(|value| value.as_u64().expect("counter")).collect::<Vec<_>>(), "{name}: counters");
        assert_eq!(progress.stage, BrushSuggestionsRunStage::Idle.index(), "{name}: a settled search waits in the idle stage");
        for (key, _, _) in mirror.finals() {
            let upserts: Vec<ToolRunVerdict> = mirror.records.iter().filter(|(record, _, _)| *record == key).map(|(_, verdict, _)| *verdict).collect();
            assert_eq!(upserts.first(), Some(&ToolRunVerdict::Testing), "{name}: candidate {key} is shown under test before it is decided");
            assert_eq!(upserts.iter().filter(|verdict| **verdict != ToolRunVerdict::Testing).count(), 1, "{name}: candidate {key} is decided exactly once");
            assert_eq!(mirror.trace.record(key).map(|record| record.verdict), mirror.finals().iter().find(|(record, _, _)| *record == key).map(|(_, verdict, _)| *verdict), "{name}: the resident trace keeps the verdict");
        }
        let free = brush_run_link(&owner, |link| link.found(target).map(|found| found.free().map(|preview| preview.object_kind_id.clone()).collect::<Vec<_>>()).unwrap_or_default());
        assert_eq!(free, expect_words("free"), "{name}: the link lists exactly the free candidates");
        assert!(brush_run_link(&owner, |link| link.found(target).is_some_and(|found| found.done)), "{name}: the link reads the search as done");
    }
}

/// 🐛️ Red→green (`📓️wave-W1-B.md` §6, `📓️wave-W0-C2.md` §3): a placed body of the host's kind that carries its OWN
/// mesh was resolved by kind — to the host's unregistered mesh — so it was never a collision body and every candidate
/// docking into it read free. Both the run and the precompute lane puzzle 5d still reads resolve a placed object's own
/// mesh first, the renderer's law.
#[test]
fn a_body_carrying_its_own_mesh_where_every_candidate_docks_collides_them_all() {
    let (owner, port) = (brush_run_owner(Some(BRUSH_SEARCH_TARGET)), ToolRunJobPort::default());
    let mut job = brush_run_job(&owner, &port, brush_search_scene(true), brush_search_lane(), brush_search_meshes, 0);
    let mut mirror = BrushRunMirror::new(0);
    mirror.drive(&mut job, &port, u64::MAX);
    assert_eq!(mirror.words(), vec!["danger:collision", "danger:collision"]);
    let (positions, indices) = brush_search_cube();
    let mut engine = Puzzle3dCollision::new();
    for url in ["/test/near.glb", "/test/late.glb", "/test/blocker.glb"] {
        engine.register_mesh(url.to_string(), &positions, &indices);
    }
    engine.scene = Some(Arc::new(brush_search_scene(true)));
    let result = (0..64).map(|_| engine.compute_brush_cache_entry(BRUSH_SEARCH_TARGET)).find(|result| !result.unknown_pending).expect("the precompute lane settles");
    assert!(result.free.is_empty(), "every candidate docks into the parked body: {:?}", result.free);
    let mut clear = Puzzle3dCollision::new();
    for url in ["/test/near.glb", "/test/late.glb", "/test/blocker.glb"] {
        clear.register_mesh(url.to_string(), &positions, &indices);
    }
    clear.scene = Some(Arc::new(brush_search_scene(false)));
    let result = (0..64).map(|_| clear.compute_brush_cache_entry(BRUSH_SEARCH_TARGET)).find(|result| !result.unknown_pending).expect("the precompute lane settles");
    assert_eq!(result.free.len(), 2, "with nothing parked there both candidates are free");
}

/// ⛽️ LAW: one fuel unit is one candidate verdict, so a single `toolRunStep` shows exactly one decided candidate.
#[test]
fn brush_suggestions_run_step_with_one_unit_of_fuel_decides_at_most_one_candidate() {
    let (owner, port) = (brush_run_owner(Some(BRUSH_SEARCH_TARGET)), ToolRunJobPort::default());
    let mut job = brush_run_job(&owner, &port, brush_search_scene(false), brush_search_lane(), brush_search_meshes, 0);
    let mut mirror = BrushRunMirror::new(0);
    let mut decided_ticks = 0;
    for _ in 0..10_000 {
        if port.is_waiting() {
            break;
        }
        let Some(tick) = brush_run_step(&mut job, 1) else { continue };
        let decided = tick.trace.iter().flat_map(|page| &page.ops).filter(|op| matches!(op, ToolRunTraceOp::Upsert { verdict, .. } if *verdict != ToolRunVerdict::Testing)).count();
        assert!(decided <= 1, "a one-unit step decided {decided} candidates");
        decided_ticks += decided;
        mirror.apply(tick);
    }
    assert!(port.is_waiting(), "the search settles one unit at a time");
    assert_eq!(decided_ticks, 2, "both candidates are decided, one per unit");
}

/// 🎯️ LAW: the run follows the link's target. A settled search waits on its port and costs no step; a new target
/// clears the trace and searches it; no target clears the trace, the link's result and waits again. A gesture wakes it.
#[test]
fn brush_suggestions_run_follows_the_link_target_and_waits_while_settled() {
    let (owner, port) = (brush_run_owner(Some(BRUSH_SEARCH_TARGET)), ToolRunJobPort::default());
    let mut scene = brush_search_scene(false);
    scene.scene_snapshot.objects[0].vortices.push(VortexProps { id: "v1".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 40.0, 0.0], direction: Some([0.0, 0.0, -1.0]) });
    let mut job = brush_run_job(&owner, &port, scene, brush_search_lane(), brush_search_meshes, 0);
    let mut mirror = BrushRunMirror::new(0);
    mirror.drive(&mut job, &port, u64::MAX);
    brush_run_link(&owner, |link| assert_eq!(link.port.is_some(), true, "the job hands the link its wake port"));
    assert!(brush_run_step(&mut job, u64::MAX).is_none(), "a settled search on an unchanged target emits nothing");
    assert!(port.is_waiting());
    brush_run_link(&owner, |link| link.hover(Some("host:v1".to_string())));
    assert!(!port.is_waiting(), "a gesture wakes the run");
    let before = mirror.clears;
    mirror.drive(&mut job, &port, u64::MAX);
    assert_eq!(mirror.clears, before + 1, "a new target clears the trace first");
    assert_eq!(mirror.trace.len(), 2, "the trace now holds only the new target's candidates");
    assert!(brush_run_link(&owner, |link| link.found("host:v1").is_some_and(|found| found.done && found.free().count() == 2)));
    assert!(brush_run_link(&owner, |link| link.found(BRUSH_SEARCH_TARGET).is_none()), "the link holds one target's result");
    brush_run_link(&owner, |link| link.hover(None));
    mirror.drive(&mut job, &port, u64::MAX);
    assert_eq!(mirror.trace.len(), 0, "no target, no trace");
    assert!(brush_run_link(&owner, |link| link.found("host:v1").is_none()), "no target, no result");
}

/// 🧹️ LAW: a closing job retires the link result it published and never one its successor (the next generation)
/// already published.
#[test]
fn a_closing_brush_suggestions_job_retires_only_its_own_result() {
    let (owner, port) = (brush_run_owner(Some(BRUSH_SEARCH_TARGET)), ToolRunJobPort::default());
    let mut first = brush_run_job(&owner, &port, brush_search_scene(false), brush_search_lane(), brush_search_meshes, 0);
    BrushRunMirror::new(0).drive(&mut first, &port, u64::MAX);
    port.wake();
    let mut second = brush_run_job(&owner, &port, brush_search_scene(false), brush_search_lane(), brush_search_meshes, 1);
    BrushRunMirror::new(1).drive(&mut second, &port, u64::MAX);
    first.close();
    assert_eq!(brush_run_link(&owner, |link| link.found(BRUSH_SEARCH_TARGET).map(|found| found.writer)), Some((1, 1)), "the successor's result survives");
    second.close();
    assert!(brush_run_link(&owner, |link| link.found(BRUSH_SEARCH_TARGET).is_none()), "the last writer's close retires its result");
}

fn brush_run_example_scene(document: &str) -> (SceneConfig, Vec<String>, Vec<String>) {
    let text = match document {
        "nakagin" => std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio")).expect("committed example reads"),
        "concrete-forest" => std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio")).expect("committed example reads"),
        other => panic!("unknown example document {other}"),
    };
    let snapshot = semio_s_artifact_puzzle_3d_model::io::text::snapshot::parse_dsl(&text).expect("example parses");
    let envelope = semio_s_artifact_puzzle_3d_editor_snapshot::scene_from_snapshot(&snapshot, Default::default(), "brush");
    let scene = semio_s_artifact_puzzle_3d_editor_session::scene_config(&envelope).expect("scene config");
    let targets = scene.scene_snapshot.objects.iter().flat_map(|object| object.vortices.iter().map(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id))).collect();
    (scene, semio_s_artifact_puzzle_3d_editor_session::mesh_lane(&envelope.scene_snapshot), targets)
}

fn brush_run_no_meshes(_url: &str) -> Option<(Vec<f32>, Vec<u32>)> {
    None
}

/// ⚖️ ORACLE (`parry3d`): on Concrete Forest and Nakagin with the app's box fallback, every candidate the run decided for the
/// scene_snapshot's targets is recomputed as an exact convex hull against EVERY placed body, the docking host included, and
/// `parry3d::query::contact` measures how deep they penetrate. A penetration of at least twice the scene's contact
/// tolerance must be a collision, one of at most half of it (or separated hulls) must be free; a candidate whose own center
/// lies inside a body (coincident hulls, where `contact` reads 0) collides. Every decisive verdict must agree.
#[test]
fn brush_suggestions_run_collision_verdicts_agree_with_the_parry3d_oracle() {
    let scene_snapshot: serde_json::Value = serde_json::from_str(BRUSH_SUGGESTIONS_RUN_FIXTURE).expect("brush suggestions run scene_snapshot");
    let law = &scene_snapshot["laws"]["parryOracle"];
    let identity = parry3d::math::Isometry::identity();
    let (mut decisive, mut ambiguous, mut collisions, mut frees) = (0usize, 0usize, 0usize, 0usize);
    let mut disagreements = Vec::new();
    for document in law["documents"].as_array().expect("documents") {
        let (scene, lane, targets) = brush_run_example_scene(document["document"].as_str().expect("document"));
        let tolerance = scene.contact_tolerance;
        let (positions, _) = semio_s_artifact_puzzle_3d_editor_session::puzzle3d_fallback_mesh_buffers();
        let hull = |pose: &Pose3d| {
            let points: Vec<parry3d::math::Point<f32>> = positions.chunks(3).map(|vertex| pose.transform_point(&semio_s_artifact_puzzle_3d_precompute_geometry::Point3d::new(vertex[0], vertex[1], vertex[2]))).map(|world| parry3d::math::Point::new(world.x(), world.y(), world.z())).collect();
            parry3d::shape::ConvexPolyhedron::from_convex_hull(&points).expect("box hull")
        };
        let catalogs = scene.kind_catalogs.clone().unwrap_or_default();
        let placed: Vec<parry3d::shape::ConvexPolyhedron> = scene.scene_snapshot.objects.iter().filter(|object| resolve_placed_object_mesh_url(object, &catalogs, &scene.scene_snapshot).is_some()).map(|object| hull(&pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale))).collect();
        let (owner, port) = (brush_run_owner(None), ToolRunJobPort::default());
        let mut job = brush_run_job(&owner, &port, scene.clone(), lane, brush_run_no_meshes, 0);
        for target in targets.iter().take(document["targets"].as_u64().expect("targets") as usize) {
            brush_run_link(&owner, |link| link.hover(Some(target.clone())));
            BrushRunMirror::new(0).drive(&mut job, &port, u64::MAX);
            let Some(found) = brush_run_link(&owner, |link| link.found(target).cloned()) else { continue };
            for (key, (preview, verdict)) in found.previews.iter().zip(&found.verdicts).enumerate() {
                let (Some(preview), BrushSuggestionVerdict::Free | BrushSuggestionVerdict::Collision) = (preview, verdict) else { continue };
                let candidate = hull(&pose_isometry(preview.origin, preview.orientation, &preview.scale));
                let center = candidate.compute_local_aabb().center();
                let deepest = placed
                    .iter()
                    .map(|other| {
                        if other.contains_local_point(&center) {
                            return f64::INFINITY;
                        }
                        parry3d::query::contact(&identity, &candidate, &identity, other, 0.0).ok().flatten().map_or(0.0, |contact| f64::from(-contact.dist))
                    })
                    .fold(0.0, f64::max);
                let (collides, clear) = (deepest >= 2.0 * tolerance, deepest <= 0.5 * tolerance);
                let ours = *verdict == BrushSuggestionVerdict::Collision;
                collisions += usize::from(ours);
                frees += usize::from(!ours);
                if !collides && !clear {
                    ambiguous += 1;
                    continue;
                }
                decisive += 1;
                if ours != collides {
                    disagreements.push(format!("{target} candidate {key} ({}) ours={verdict:?} parry penetration {deepest:.4} m (tolerance {tolerance} m)", preview.object_kind_id));
                }
            }
        }
    }
    assert!(disagreements.is_empty(), "{} of {decisive} decisive verdicts disagree with parry3d:\n{}", disagreements.len(), disagreements.join("\n"));
    assert!(collisions > 0 && frees > 0, "the oracle must decide both collisions ({collisions}) and free candidates ({frees})");
    assert!(ambiguous * 10 <= decisive, "at most one in ten verdicts may fall inside the tolerance band: {ambiguous} of {decisive}");
}

/// ⏱️ LAW: hovering never blocks. On Nakagin, the largest example, the run is driven across the scene_snapshot's targets with the
/// thread's primitive-work meter as its clock ([`precompute_work_clock`]) and a slice of `budgetWork` units: every step —
/// preparation, target listing and collision units alike — stays inside `stepWorkCeiling` units, so no unit between two
/// deadline checks outgrows the slice. The count is deterministic: the same steps on an idle and a saturated machine.
#[test]
fn brush_suggestions_run_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let scene_snapshot: serde_json::Value = serde_json::from_str(BRUSH_SUGGESTIONS_RUN_FIXTURE).expect("brush suggestions run scene_snapshot");
    let law = &scene_snapshot["laws"]["interactive"];
    let (scene, lane, targets) = brush_run_example_scene(law["document"].as_str().expect("document"));
    let (budget, ceiling) = (law["budgetWork"].as_u64().expect("budget"), law["stepWorkCeiling"].as_u64().expect("ceiling"));
    let (owner, port) = (brush_run_owner(None), ToolRunJobPort::default());
    let mut job = brush_run_job_on_clock(&owner, &port, scene, lane, brush_run_no_meshes, 0, precompute_work_clock);
    let (mut worst, mut steps) = (0_u64, 0_usize);
    for target in targets.iter().take(law["targets"].as_u64().expect("targets") as usize) {
        brush_run_link(&owner, |link| link.hover(Some(target.clone())));
        while !port.is_waiting() {
            let start = precompute_work_done();
            let step_budget = StepBudget::from_duration(semio_framework_job::INTERACTIVE_LANE_FUEL, start, budget, semio_s_plugin_puzzle_jobs::testing::fixture_stock()).expect("budget");
            let outcome = job.advance(step_budget.fuel, step_budget.deadline_us);
            worst = worst.max(precompute_work_done() - start);
            drop(brush_run_settle(outcome));
            steps += 1;
        }
    }
    assert!(worst <= ceiling, "the worst step cost {worst} units of primitive work over {steps} steps against a {ceiling}-unit ceiling (slice {budget})");
}
//#endregion ⏯️BrushSuggestionsRun
