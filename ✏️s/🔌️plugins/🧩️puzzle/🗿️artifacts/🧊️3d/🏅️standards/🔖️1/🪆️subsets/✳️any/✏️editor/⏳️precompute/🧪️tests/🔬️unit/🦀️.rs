use super::*;
use crate::editor::puzzle3d::precompute::fill::FillJobStage;
use crate::standards::v1::subsets::any::schema::precompute_model_tests::context::*;
use crate::standards::v1::subsets::any::schema::{BrushHostRules, BrushKindWeights, CableKindCatalog, FixtureObject, KindCompatEntry, ObjectKind, ObjectKindRepresentation, ObjectKindVortexTemplate, VortexKindCatalog, VortexProps};

use std::time::{Duration, Instant};

fn fill_capable_engine() -> Puzzle3dCollision {
    let mut engine = Puzzle3dCollision::new();
    let (positions, indices) = unit_cube_mesh_buffers();
    engine.register_mesh("/test/host.glb".to_string(), &positions, &indices);
    engine.register_mesh("/test/candidate.glb".to_string(), &positions, &indices);
    let scene = SceneConfig {
        fixture: Fixture {
            attractions: vec![],
            target_volumes: vec![],
            objects: vec![FixtureObject {
                id: "host".to_string(),
                object_kind: Some("Host".to_string()),
                anchor: Default::default(),
                mesh_url: Some("/test/host.glb".to_string()),
                origin: [0.0, 0.0, 0.0],
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [4.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                reveal_index: None,
            }],
        },
        kind_catalogs: Some(KindCatalogBundle {
            objects: vec![
                ObjectKind {
                    id: "Host".to_string(),
                    representations: vec![ObjectKindRepresentation { id: "host".into(), name: String::new(), url: "/test/host.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                    scale: None,
                    vortices: vec![],
                },
                ObjectKind {
                    id: "Candidate".to_string(),
                    representations: vec![ObjectKindRepresentation { id: "candidate".into(), name: String::new(), url: "/test/candidate.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                    scale: None,
                    vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, 1.0]), ..Default::default() }],
                },
            ],
            vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None, ..Default::default() }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None, ..Default::default() }],
            cables: vec![],
        }),
        kind_compatibility: vec![KindCompatEntry { source: "port-b".to_string(), target: "port-a".to_string(), bidirectional: true, important: false, specificity: Some("vortex".to_string()) }],
        overlap_budget: DEFAULT_OVERLAP_BUDGET,
        seed: 1,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    };
    engine.set_scene(&serde_json::to_string(&scene).expect("fill scene")).expect("set fill scene");
    engine.fill.as_ref().expect("fill").lock().expect("fill lock").max_count = 1;
    engine
}

fn fill_builder_for_test(base: Fixture, seed: u32, catalogs: &KindCatalogBundle) -> FillBuilder {
    let scene = Arc::new(SceneConfig { fixture: base, kind_catalogs: Some(catalogs.clone()), kind_compatibility: Vec::new(), overlap_budget: 0.0, seed, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() });
    let mut fill = FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(HashMap::new())), Operation::new(semio_framework_job::allocate_operation_id(), RevisionId(1), Generation(1), seed as u64));
    while matches!(
        fill.stage,
        FillJobStage::PrepareFixture | FillJobStage::PrepareCatalogs | FillJobStage::PrepareMeshes | FillJobStage::PrepareEntries | FillJobStage::PrepareSpatial | FillJobStage::PrepareLookup | FillJobStage::PrepareConfiguration
    ) {
        fill.prepare_one();
    }
    fill
}

#[test]
fn brush_candidates_allow_separated_boxes() {
    let mut engine = Puzzle3dCollision::new();
    let positions: Vec<f32> = vec![-4.0, -4.0, -4.0, 4.0, -4.0, -4.0, 4.0, 4.0, -4.0, -4.0, 4.0, -4.0, -4.0, -4.0, 4.0, 4.0, -4.0, 4.0, 4.0, 4.0, 4.0, -4.0, 4.0, 4.0, 4.0];
    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
    engine.register_mesh("/test/obstacle.glb".to_string(), &positions, &indices);
    engine.register_mesh("/test/preview.glb".to_string(), &positions, &indices);
    let scene = SceneConfig {
        fixture: Fixture {
            attractions: vec![],
            target_volumes: vec![],
            objects: vec![
                FixtureObject {
                    id: "obstacle".to_string(),
                    object_kind: Some("Kind".to_string()),
                    anchor: Default::default(),
                    mesh_url: Some("/test/obstacle.glb".to_string()),
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                    reveal_index: None,
                },
                FixtureObject {
                    id: "host".to_string(),
                    object_kind: Some("Host".to_string()),
                    anchor: Default::default(),
                    mesh_url: Some("/test/unregistered.glb".to_string()),
                    origin: [12.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                    reveal_index: None,
                },
            ],
        },
        kind_catalogs: Some(KindCatalogBundle {
            objects: vec![ObjectKind {
                id: "Kind".to_string(),
                representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/test/preview.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                scale: None,
                vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]), ..Default::default() }],
            }],
            vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None, ..Default::default() }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None, ..Default::default() }],
            cables: vec![CableKindCatalog { id: "cable.link".to_string(), default_attraction_kind: None, ..Default::default() }],
        }),
        kind_compatibility: vec![KindCompatEntry { source: "port-b".to_string(), target: "port-a".to_string(), bidirectional: true, important: false, specificity: Some("vortex".to_string()) }],
        overlap_budget: DEFAULT_OVERLAP_BUDGET,
        seed: 1,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    };
    engine.scene = Some(Arc::new(scene));
    let result = engine.compute_brush_cache_entry("host:v0");
    assert!(!result.unknown_pending, "expected mesh-ready result");
    assert_eq!(result.free.len(), 1, "expected one collision-free candidate");
}

/// 🪪️ Regression: `set_scene` used to unconditionally `rebuild_queue()`, wiping `brush_cache`/`fill`
/// progress on every resync — the app's `sync_precompute_session` calls `set_scene` on *every*
/// action, so this made suggestion/fill precompute restart from zero on every single tick, freezing
/// the UI. A resync with byte-identical scene JSON must be a no-operation.
#[test]
fn compose_fill_display_is_read_only_and_matches_apply_prefix() {
    let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
    let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
    let mut fill = fill_builder_for_test(base, 7, &catalogs);
    fill.applied_count = 2;
    fill.sequence = (0..5).map(fill_plan_payload).collect();
    fill.appended_objects = (0..5).map(|index| fill_plan_object(&format!("p{index}"))).collect();
    fill.appended_attractions = (0..5).map(fill_plan_attraction).collect();
    let mut engine = Puzzle3dCollision::new();
    engine.fill = Some(Arc::new(Mutex::new(fill)));

    let display = engine.compose_fill_display(4).expect("semio_compose_rs display");
    assert_eq!(display.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base", "p0", "p1", "p2", "p3"]);
    assert_eq!(engine.fill.as_ref().expect("fill").lock().expect("fill lock").applied_count, 2, "semio_compose_rs must not mutate applied_count");

    let applied = engine.apply_fill_count(4).expect("apply fill count");
    assert_eq!(applied.objects.len(), display.objects.len());
    assert_eq!(engine.fill.as_ref().expect("fill").lock().expect("fill lock").applied_count, 4);
}

#[test]
fn fill_options_paths_are_millisecond_scale() {
    let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
    let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
    let mut fill = fill_builder_for_test(base.clone(), 7, &catalogs);
    fill.applied_count = 0;
    fill.sequence = (0..10).map(fill_plan_payload).collect();
    fill.appended_objects = (0..10).map(|index| fill_plan_object(&format!("p{index}"))).collect();
    fill.appended_attractions = (0..10).map(fill_plan_attraction).collect();

    let mut engine = Puzzle3dCollision::new();
    let base_scene = SceneConfig { fixture: base, kind_catalogs: Some(catalogs), kind_compatibility: vec![], overlap_budget: 0.0, seed: 7, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() };
    engine.set_scene(&serde_json::to_string(&base_scene).unwrap()).expect("seed");
    engine.fill = Some(Arc::new(Mutex::new(fill)));

    let count_start = Instant::now();
    let _ = engine.apply_fill_count(5).expect("apply fill count");
    let count_ms = count_start.elapsed().as_secs_f64() * 1000.0;
    assert!(count_ms < 5.0, "fill count apply took {count_ms}ms");
    assert_eq!(engine.fill.as_ref().expect("fill").lock().expect("fill lock").applied_count, 5);

    let weight_start = Instant::now();
    let mut object_weights = std::collections::BTreeMap::new();
    object_weights.insert("Placed".to_string(), 1.0);
    let mut vortex_weights = std::collections::BTreeMap::new();
    vortex_weights.insert("c-b".to_string(), 0.5);
    vortex_weights.insert("b-s".to_string(), 0.5);
    engine.update_kind_weights(object_weights, vortex_weights);
    let weight_ms = weight_start.elapsed().as_secs_f64() * 1000.0;
    assert!(weight_ms < 50.0, "weight update took {weight_ms}ms");
    let fill_owner = engine.fill.as_ref().expect("fill").clone();
    let fill = fill_owner.lock().expect("fill lock");
    let fill_steps = engine.fill_steps_pending_for_test();
    assert_eq!(fill_steps, fill.max_count - fill.applied_count, "weight update must soft-replan the tail without a full queue wipe");
    assert_eq!(fill.applied_count, 5, "applied fill objects must survive weight edits");
}

#[test]
fn apply_fill_count_downward_move_keeps_the_plan_intact() {
    // 🔽️ Moving the count DOWN must never discard the already-planned sequence/appended objects/
    // placed entries or re-enqueue FillSteps — only `applied_count` (and the returned document-prefix
    // fixture) may change. Otherwise a jittery drag forces expensive replanning on every dip.
    let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
    let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
    let mut fill = fill_builder_for_test(base.clone(), 7, &catalogs);
    fill.applied_count = 0;
    fill.sequence = (0..10).map(fill_plan_payload).collect();
    fill.appended_objects = (0..10).map(|index| fill_plan_object(&format!("p{index}"))).collect();
    fill.appended_attractions = (0..10).map(fill_plan_attraction).collect();
    fill.placed = fill
        .appended_objects
        .iter()
        .map(|object| PlacedCollisionEntry { object_id: object.id.clone(), mesh_url: "/test/placed.glb".into(), world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale) })
        .collect();

    let mut engine = Puzzle3dCollision::new();
    let base_scene = SceneConfig { fixture: base.clone(), kind_catalogs: Some(catalogs), kind_compatibility: vec![], overlap_budget: 0.0, seed: 7, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() };
    engine.set_scene(&serde_json::to_string(&base_scene).unwrap()).expect("seed");
    engine.fill = Some(Arc::new(Mutex::new(fill)));

    engine.apply_fill_count(8).expect("apply up to 8");
    let queue_before = engine.work_pending_for_test();
    let placed_before = engine.fill.as_ref().unwrap().lock().expect("fill lock").placed.len();
    let sequence_before = engine.fill.as_ref().unwrap().lock().expect("fill lock").sequence.len();

    engine.apply_fill_count(3).expect("apply down to 3");
    let fill_owner = engine.fill.as_ref().expect("fill").clone();
    let fill = fill_owner.lock().expect("fill lock");
    assert_eq!(fill.applied_count, 3);
    assert_eq!(fill.sequence.len(), sequence_before, "the plan is prefix-stable — downward moves never truncate it");
    assert_eq!(fill.appended_objects.len(), sequence_before);
    assert_eq!(fill.appended_attractions.len(), sequence_before);
    assert_eq!(fill.placed.len(), placed_before, "placed collision entries survive a downward move");
    drop(fill);
    assert_eq!(engine.work_pending_for_test(), queue_before, "no FillSteps get re-enqueued on a downward move");

    let fixture = engine.apply_fill_count(7).expect("apply back up to 7");
    assert_eq!(fixture.objects.len(), base.objects.len() + 7, "moving back up is instant — the plan was never discarded");
}

#[test]
fn update_kind_weights_soft_replans_tail_without_rebuilding_queue() {
    let mut engine = Puzzle3dCollision::new();
    let json = single_object_scene_json();
    engine.set_scene(&json).expect("seed scene");
    let progress_after_seed = engine.precompute_progress_for_test();
    let queue_len_after_seed = engine.work_pending_for_test();
    engine.precompute_step(8);
    let queue_len_after_step = engine.work_pending_for_test();
    assert!(engine.precompute_progress_for_test() > progress_after_seed, "a precompute turn must complete work in at least one lane");
    assert_eq!(queue_len_after_seed, FILL_COUNT_MAX, "the seed arms one fill step per planned placement");

    let mut object_weights = std::collections::BTreeMap::new();
    object_weights.insert("Host".to_string(), 0.25);
    object_weights.insert("Placed".to_string(), 0.75);
    let mut vortex_weights = std::collections::BTreeMap::new();
    vortex_weights.insert("c-b".to_string(), 0.5);
    vortex_weights.insert("b-s".to_string(), 0.5);
    engine.update_kind_weights(object_weights, vortex_weights);

    assert_eq!(engine.fill.as_ref().and_then(|fill| fill.lock().ok()).map_or(0, |fill| fill.applied_count), 0, "weight-only edits must not change applied count");
    assert_eq!(engine.fill.as_ref().and_then(|fill| fill.lock().ok()).map_or(0, |fill| fill.sequence.len()), 0, "planned tail must be discarded for replanning");
    assert!(engine.work_pending_for_test() >= queue_len_after_step, "fill steps must be re-enqueued without a full queue wipe");
    assert!(engine.fill_steps_pending_for_test() > 0, "fill planning must continue after weight edits");
}

/// 🗺️ A scene sync invalidates the brush derivation PER OBJECT: an identical scene invalidates nothing,
/// a new object enqueues exactly its own targets and nothing else, and only a fill-plan member change
/// invalidates every candidate.
///
/// 🧾️ This law used to read `assert_ne!(work_pending, before)` for "a changed scene must rebuild the
/// queue" — it pinned the whole-document wipe itself, which is the defect: on the 340-object
/// brush-painted document every sync (and the 120 ms `suggestionsTick`/`fillBuildTick` cadence behind an
/// interactive mutation) threw away every resolved candidate and re-walked the whole object × vortex
/// product (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B54).
#[test]
fn a_scene_sync_invalidates_the_brush_derivation_per_object() {
    let mut engine = Puzzle3dCollision::new();
    let json = single_object_scene_json();
    engine.set_scene(&json).expect("first set_scene should succeed");
    let queue_len_before = engine.work_pending_for_test();
    assert!(queue_len_before > 0, "the first sync arms at least the fill steps");
    let progress_before = engine.precompute_progress_for_test();
    engine.precompute_step(4);
    let queue_len_after_step = engine.work_pending_for_test();
    assert!(engine.precompute_progress_for_test() > progress_before, "precompute_step should have completed some precompute work");

    engine.set_scene(&json).expect("resync with identical json should succeed");
    assert_eq!(engine.work_pending_for_test(), queue_len_after_step, "an identical scene invalidates nothing at all");

    let mut scene: serde_json::Value = serde_json::from_str(&json).unwrap();
    scene["fixture"]["objects"].as_array_mut().unwrap().push(serde_json::json!({ "id": "extra", "objectKind": "Host", "meshUrl": "/test/host.glb", "origin": [5.0, 0.0, 0.0], "orientation": [0.0, 0.0, 0.0, 1.0], "vortices": [{ "id": "v0", "vortexKind": "port-a", "position": [0.0, 0.0, 0.0], "direction": [0.0, 0.0, -1.0] }] }));
    let grown_json = serde_json::to_string(&scene).unwrap();
    engine.set_scene(&grown_json).expect("set_scene with one added object should succeed");
    assert_eq!(engine.work_pending_for_test(), queue_len_after_step + 1, "one added object enqueues exactly its own one brush target, and no other object's");

    let mut replanned: serde_json::Value = serde_json::from_str(&grown_json).unwrap();
    replanned["overlapBudget"] = serde_json::json!(0.5);
    engine.set_scene(&serde_json::to_string(&replanned).unwrap()).expect("set_scene with a new overlap budget should succeed");
    assert_eq!(engine.work_pending_for_test(), engine.fill_steps_pending_for_test(), "a fill-plan member change invalidates every candidate and clears the queue for a whole-scene rebuild");
}

#[test]
fn decreasing_fill_count_keeps_the_plan_intact_and_does_not_replan() {
    // 🔽️ Downward moves are prefix-stable (see `apply_fill_count`) — the plan/sequence/appended
    // objects/queue must never be discarded or re-enqueued just because the applied prefix shrank;
    // that used to force expensive replanning on every jittery drag dip.
    let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
    let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
    let mut fill = fill_builder_for_test(base, 7, &catalogs);
    fill.applied_count = 3;
    fill.sequence = (0..3).map(fill_plan_payload).collect();
    fill.appended_objects = (0..3).map(|index| fill_plan_object(&format!("p{index}"))).collect();
    fill.appended_attractions = (0..3).map(fill_plan_attraction).collect();
    fill.stalled = true;
    let rng_state = fill.rng_state;
    let mut engine = Puzzle3dCollision::new();
    engine.fill = Some(Arc::new(Mutex::new(fill)));

    let fixture = engine.apply_fill_count(1).expect("fill session");
    assert_eq!(fixture.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base", "p0"], "the returned document prefix reflects the new applied count");
    let fill_owner = engine.fill.as_ref().expect("fill builder").clone();
    let fill = fill_owner.lock().expect("fill lock");
    assert_eq!(fill.appended_objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["p0", "p1", "p2"], "the full plan survives — a downward move never discards the tail");
    assert_eq!(fill.sequence.len(), 3, "the planned sequence is never truncated by a downward move");
    assert_eq!(fill.applied_count, 1);
    assert!(fill.stalled, "apply_fill_count never touches stalled — only actual planning does");
    assert_eq!(fill.rng_state, rng_state, "no replanning happens, so the random stream is untouched");
    assert_eq!(engine.fill_steps_pending_for_test(), 0, "no FillSteps get enqueued by a downward move");
    drop(fill);

    let fixture = engine.apply_fill_count(0).expect("zero fill count");
    assert_eq!(fixture.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base"], "zero applies nothing to the document");
    assert_eq!(engine.fill.as_ref().expect("fill builder").lock().expect("fill lock").sequence.len(), 3, "even at count 0, the plan is preserved for instant re-apply");
}

#[test]
fn set_scene_with_applied_fill_projection_preserves_slider_session() {
    let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
    let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
    let mut fill = fill_builder_for_test(base.clone(), 7, &catalogs);
    fill.applied_count = 3;
    fill.sequence = (0..3).map(fill_plan_payload).collect();
    fill.appended_objects = (0..3).map(|index| fill_plan_object(&format!("p{index}"))).collect();
    fill.appended_attractions = (0..3).map(fill_plan_attraction).collect();
    fill.stalled = true;

    let mut engine = Puzzle3dCollision::new();
    let base_scene = SceneConfig { fixture: base, kind_catalogs: Some(catalogs), kind_compatibility: vec![], overlap_budget: 0.0, seed: 7, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() };
    let base_json = serde_json::to_string(&base_scene).unwrap();
    engine.set_scene(&base_json).expect("seed base scene");
    // 🪣️ Replace the fresh FillBuilder from rebuild_queue with the already-applied session under test.
    engine.fill = Some(Arc::new(Mutex::new(fill)));

    let mut applied_scene = base_scene;
    applied_scene.fixture.objects.extend((0..3).map(|index| fill_plan_object(&format!("p{index}"))));
    applied_scene.fixture.attractions.extend((0..3).map(fill_plan_attraction));
    // 🪪️ Pose drift on the base object (attraction rederive) must not count as a new scene.
    applied_scene.fixture.objects[0].origin = [1.0, 2.0, 3.0];
    let applied_json = serde_json::to_string(&applied_scene).unwrap();
    engine.set_scene(&applied_json).expect("re-syncing the applied fill projection must succeed");

    let fill_owner = engine.fill.as_ref().expect("fill session must survive the applied-projection re-sync").clone();
    let fill = fill_owner.lock().expect("fill lock");
    assert_eq!(fill.applied_count, 3, "applied fill count must survive incidental set_scene syncs");
    assert_eq!(fill.sequence.len(), 3, "planned fill sequence must survive incidental set_scene syncs");
    assert_eq!(fill.base.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base"]);
    drop(fill);

    let reduced = engine.apply_fill_count(1).expect("decreasing after sync");
    assert_eq!(reduced.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base", "p0"], "slider must still be able to remove fill objects after a document re-sync");
    let cleared = engine.apply_fill_count(0).expect("clear after sync");
    assert_eq!(cleared.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base"]);
}

/// 🪪️ Regression: registering a mesh must invalidate any cached brush candidates computed against a
/// different (e.g. fallback-box) body for the same url, but a no-operation re-registration must not matter
/// once the cache already reflects the current mesh set (the everyday case: every action re-seeds the
/// fallback body, and the app's `sync_precompute_session` already guards that with `has_mesh`).
#[test]
fn register_mesh_invalidates_cached_precompute_state() {
    let mut engine = Puzzle3dCollision::new();
    engine.set_scene(&single_object_scene_json()).expect("set_scene should succeed");
    let applied_before = engine.fill.as_ref().and_then(|fill| fill.lock().ok()).map_or(0, |fill| fill.applied_count);
    let positions: Vec<f32> = vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0];
    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
    engine.register_mesh("/test/host.glb".to_string(), &positions, &indices);
    assert!(engine.brush_cache.is_empty(), "mesh registration must invalidate stale brush cache entries");
    assert_eq!(engine.fill.as_ref().and_then(|fill| fill.lock().ok()).map(|fill| fill.applied_count), Some(applied_before), "mesh registration must not reset applied fill count");
}

#[test]
fn engine_precompute_step_is_false_with_no_scene() {
    let mut engine = Puzzle3dCollision::new();
    assert!(!engine.precompute_step(10));
    assert!(engine.fill.is_none());
}

#[test]
fn engine_apply_brush_placement_none_without_scene_or_catalogs() {
    let mut engine = Puzzle3dCollision::new();
    let payload = BrushPlacePayload { target_vortex_full_id: "host:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
    assert!(engine.apply_brush_placement(&payload).is_none(), "no scene means no placement");

    engine.set_scene(&single_object_scene_json()).expect("seed");
    if let Some(scene) = &mut engine.scene {
        Arc::make_mut(scene).kind_catalogs = None;
    }
    assert!(engine.apply_brush_placement(&payload).is_none(), "no catalogs means no placement");
}

#[test]
fn engine_has_mesh_invalidate_and_refresh_brush_candidates() {
    let mut engine = Puzzle3dCollision::new();
    engine.set_scene(&single_object_scene_json()).expect("seed");
    assert!(!engine.has_mesh("/test/host.glb"));
    let (positions, indices) = unit_cube_mesh_buffers();
    engine.register_mesh("/test/host.glb".to_string(), &positions, &indices);
    assert!(engine.has_mesh("/test/host.glb"));

    engine.invalidate_brush_target("host:v0");
    assert_eq!(engine.brush_queue.front().map(String::as_str), Some("host:v0"), "invalidated brush target must be requeued at the front");
    assert!(!engine.brush_cache.contains_key("host:v0"));

    engine.refresh_brush_candidates("host:v0");
    assert!(engine.brush_cache.contains_key("host:v0"));
    assert_eq!(engine.brush_preview("host:v0", 0), None, "the catalog's Host kind has no vortices, so there are no free candidates");
}

#[test]
fn precompute_session_native_wrapper_exercises_public_methods() {
    let mut session = Puzzle3dPrecomputeSession::default();
    session.set_scene(&single_object_scene_json()).expect("set_scene");
    assert!(!session.has_mesh("/test/host.glb"));
    let (positions, indices) = unit_cube_mesh_buffers();
    session.register_mesh("/test/host.glb", &positions, &indices);
    assert!(session.has_mesh("/test/host.glb"));
    assert!(!session.fill_is_done(), "a freshly (re)seeded fill session has not stalled or hit max_count yet");

    session.precompute_step(50);
    session.invalidate_brush_target("host:v0");
    session.refresh_brush_candidates("host:v0");
    let _candidates: BrushCollisionFreeResult = session.brush_candidates("host:v0");
    assert!(session.brush_preview("host:v0", 0).is_none());

    assert_eq!(session.fill_progress().max_count, FILL_COUNT_MAX);
    assert_eq!(session.fill_available_count(), 0);

    let mut object_weights = std::collections::BTreeMap::new();
    object_weights.insert("Host".to_string(), 1.0);
    session.dispatch(Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights: std::collections::BTreeMap::new() }).expect("update kind weights");

    let missing_payload = BrushPlacePayload { target_vortex_full_id: "missing:v0".to_string(), object_kind_id: "Nonexistent".to_string(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
    assert!(session.dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload: missing_payload }).is_err());

    let outcome = session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).expect("fill session available");
    let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
    assert!(fixture.objects.iter().any(|object| object.id == "host"));
    let outcome = session.dispatch(Puzzle3dEngineCommand::ComposeFillDisplay { count: 0 }).expect("fill session available");
    let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
    assert!(fixture.objects.iter().any(|object| object.id == "host"));
}

/// ⏳️ Drives the fill lane until the planner's cursorized preparation has copied the scene into the
/// builder's fixed pages and stops there — the state every admission law below describes. A builder
/// straight out of `set_scene` still owns its preparation roots and holds EMPTY catalog/base pages,
/// so admitting one measures nothing and the nested-owner injections have no row to reach.
fn drive_fill_preparation(engine: &mut Puzzle3dCollision) {
    for _ in 0..FILL_PREPARATION_DRIVE_TURNS {
        let preparing = engine.fill.as_ref().and_then(|fill| fill.try_lock().ok()).is_some_and(|fill| {
            matches!(
                fill.stage,
                FillJobStage::PrepareFixture | FillJobStage::PrepareCatalogs | FillJobStage::PrepareMeshes | FillJobStage::PrepareEntries | FillJobStage::PrepareSpatial | FillJobStage::PrepareLookup | FillJobStage::PrepareConfiguration
            )
        });
        // 🧵️ The lane leaves the last outcome checked out of the worker session; admission refuses a
        // session in that state, so the drive only stops on a fully handed-back worker.
        if !preparing && engine.fill_worker_outcome.is_none() {
            return;
        }
        engine.precompute_step_lane(PrecomputeLane::Fill, 1);
    }
    panic!("fill preparation must finish within its own bounded turns");
}

/// 📏️ Turns the widest test scene's cursorized preparation needs, with an order of magnitude of
/// headroom: one turn per fixture/catalog/mesh/entry/placement/weight row plus one per stage
/// transition, each one job step behind three lane turns (step, outcome close, resume).
const FILL_PREPARATION_DRIVE_TURNS: usize = 4096;

fn fill_worker_session(seed: u32) -> Puzzle3dPrecomputeSession {
    let mut engine = fill_capable_engine();
    if seed != 1 {
        let mut scene = (*engine.scene.clone().expect("scene")).clone();
        scene.seed = seed;
        engine.set_scene(&serde_json::to_string(&scene).expect("scene json")).expect("reseed scene");
    }
    drive_fill_preparation(&mut engine);
    engine.fill.as_ref().expect("fill").lock().expect("fill lock").max_count = FILL_COUNT_MAX;
    Puzzle3dPrecomputeSession { engine, fill_job: None, fill_admission: None, fill_terminal: None, fill_observation: FillObservation::default(), fill_applied_count: 0, fill_faulted: false, fill_fault_notice: false, last_emitted_fill_checkpoint: RefCell::new(Vec::new()), brush_live_target: None }
}

fn close_fill_envelope(session: &mut Puzzle3dPrecomputeSession) {
    let request = session.fill_job.clone().expect("fill request");
    let _ = session.cancel_fill_job();
    request_fill_envelope_terminal(&request, FillEnvelopeTerminalReason::Closed);
    let _ = session.drive_fill_job(&request);
    let mut terminal = session.take_terminal_fill_job().expect("terminal handle");
    while !matches!(terminal.close_step(), FillEnvelopeCloseStep::Complete) {}
    assert!(terminal.terminal_is_empty());
}

fn enqueue_measured_fill_job(session: &mut Puzzle3dPrecomputeSession) -> Option<(u64, Vec<u8>)> {
    for _ in 0..FILL_ENVELOPE_MAX_ITEMS {
        if let Some(admitted) = session.enqueue_fill_job() {
            return Some(admitted);
        }
    }
    None
}

/// 📏️ Close grants one envelope spends before the admitted fill reaches its own retirement cursor:
/// `FillEnvelopeTerminalHandle::close_step` retires the last worker outcome (stage 0) and then the
/// whole worker session — its job shell, batch params and payload pages (stage 1) — one owner per
/// grant, before stage 2 can unwrap the fill. Measured at exactly nine; doubled as headroom for one
/// more owner in either stage.
const FILL_ENVELOPE_CLOSE_GRANTS_TO_RETIREMENT: usize = 18;

/// ♻️ Grants close steps until the admitted fill has moved into its own retirement cursor, asserting
/// every grant stays incremental, and answers how many that took.
fn close_until_fill_retirement(terminal: &mut FillEnvelopeTerminalHandle, request: &FillJobRequest) -> usize {
    for grant in 1..=FILL_ENVELOPE_CLOSE_GRANTS_TO_RETIREMENT {
        assert_eq!(terminal.close_step(), FillEnvelopeCloseStep::Pending, "one close grant retires one owner and never bulk-closes");
        if fill_envelope_registry().lock().expect("registry").authority_mut(request).is_some_and(|authority| authority.fill_retirement.is_some()) {
            return grant;
        }
    }
    panic!("the admitted fill must reach its retirement cursor within the declared close grants");
}

/// ♻️ Retires one abandoned envelope the way PRODUCTION does — through the process-wide reaper the
/// framework's maintenance ladder grants one unit per turn — and states that the named slot came back
/// and that nothing finished is left standing. A session's `Drop` only ASKS for the terminal (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B42): draining a plan-sized ladder inside a `Drop` is the
/// unyielding turn the host watchdog killed a shard over, so the drain is no longer the dying session's.
///
/// 🔓️ The registry lock is released before every assertion on purpose: an assert that fires while
/// holding it POISONS the process-wide mutex, and one failing law then failed twelve more with
/// `try_lock` errors instead of their own verdicts.
fn drain_orphaned_fill_envelope(request: &FillJobRequest) {
    let grants = crate::editor::puzzle3d::precompute::reap_fill_envelopes_for_test();
    let standing = fill_envelope_registry().lock().expect("registry").slots[usize::from(request.slot)].is_some();
    assert!(!standing, "the granted reaper retires the exact orphan to terminal empty within {grants} grants");
    assert!(crate::editor::puzzle3d::precompute::fill_envelope_terminal_is_empty(), "the same terminal intent cannot mount twice after readiness is cleared");
}

#[test]
fn fill_worker_token_reopens_the_exact_retained_owner_and_drives_one_turn() {
    let _guard = fill_envelope_test_guard();
    let mut admitted = fill_worker_session(7);
    let source = admitted.engine.fill.as_ref().expect("fill owner").clone();
    let source_pointer = Arc::as_ptr(&source);
    let (_, input) = enqueue_measured_fill_job(&mut admitted).expect("fill job");
    assert_eq!(input.len(), FILL_ENVELOPE_TOKEN_BYTES);
    let request = decode_fill_envelope_token(&input).expect("fixed token");
    let registry = fill_envelope_registry().lock().expect("registry");
    let retained = registry.slots[usize::from(request.slot)].as_ref().and_then(|authority| authority.fill.as_ref()).expect("retained fill owner");
    assert_eq!(Arc::as_ptr(retained), source_pointer, "admission moves the same FillBuilder authority without a whole-state clone");
    drop(registry);
    let mut reopened = Puzzle3dPrecomputeSession::new();
    assert!(reopened.restore_persisted_fill(&input));
    assert!(reopened.engine.fill.is_none(), "restore mounts only the immutable registry authority and cannot recreate a mutable engine alias");
    let before = reopened.fill_observation;
    let slice = reopened.drive_fill_job(&request).expect("one retained worker turn");
    assert!(slice.progress.is_none() || reopened.poll_fill_job() || reopened.fill_observation != before);
    drop(source);
    close_fill_envelope(&mut reopened);
}

#[test]
fn fill_worker_cross_generation_restore_rejects_measuring_and_every_live_terminal_phase() {
    let _guard = fill_envelope_test_guard();
    let mut measuring = fill_worker_session(45);
    assert!(measuring.enqueue_fill_job_spending(1).is_none(), "one census unit leaves a multi-owner envelope in Measuring");
    assert!(measuring.fill_admission.is_some(), "the 4096-unit production grant is what finishes the census, not the first owner");
    let measuring_request = measuring.fill_job.clone().expect("measuring request");
    let measuring_cursor = measuring.fill_admission.as_ref().map(|admission| admission.request.clone()).expect("measurement cursor");
    let mut producer = fill_worker_session(47);
    let (_, producer_token) = enqueue_measured_fill_job(&mut producer).expect("producer request");
    let producer_request = decode_fill_envelope_token(&producer_token).expect("producer identity");
    assert!(!measuring.restore_persisted_fill(&producer_token));
    assert_eq!(measuring.fill_job.as_ref(), Some(&measuring_request));
    assert_eq!(measuring.fill_admission.as_ref().map(|admission| &admission.request), Some(&measuring_cursor));
    drop(measuring);
    drop(producer);
    drain_orphaned_fill_envelope(&measuring_request);
    drain_orphaned_fill_envelope(&producer_request);

    for phase in [
        FillEnvelopePhase::Admitted,
        FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Complete),
        FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Cancelled),
        FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Fault),
        FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Closed),
    ] {
        let mut mounted = fill_worker_session(49);
        let (_, mounted_token) = enqueue_measured_fill_job(&mut mounted).expect("mounted request");
        let mounted_request = decode_fill_envelope_token(&mounted_token).expect("mounted identity");
        let mut other = fill_worker_session(51);
        let (_, other_token) = enqueue_measured_fill_job(&mut other).expect("other request");
        let other_request = decode_fill_envelope_token(&other_token).expect("other identity");
        let aggregate_before = {
            let mut registry = fill_envelope_registry().lock().expect("registry");
            let authority = registry.authority_mut(&mounted_request).expect("mounted authority");
            authority.phase = phase;
            if matches!(phase, FillEnvelopePhase::Terminal(_)) {
                authority.observation.done = true;
            }
            registry.aggregate_bytes
        };
        // 🪪️ The session's OWN observation is what a rejected restore must not disturb; the registry's
        // copy is forced above precisely to prove the rejection never adopts it.
        let mounted_observation = mounted.fill_observation;
        assert!(!mounted.restore_persisted_fill(&other_token));
        assert_eq!(mounted.fill_job.as_ref(), Some(&mounted_request), "restore cannot replace the exact mounted producer");
        assert_eq!(mounted.fill_observation, mounted_observation, "restore rejection leaves mounted observation unchanged");
        let registry = fill_envelope_registry().lock().expect("registry");
        let authority = registry.slots[usize::from(mounted_request.slot)].as_ref().expect("mounted authority remains registered");
        assert_eq!(authority.request, mounted_request);
        assert_eq!(authority.phase, phase);
        assert_eq!(registry.aggregate_bytes, aggregate_before, "restore rejection neither reserves nor releases credit");
        drop(registry);
        drop(mounted);
        drop(other);
        drain_orphaned_fill_envelope(&mounted_request);
        drain_orphaned_fill_envelope(&other_request);
        let registry = fill_envelope_registry().lock().expect("registry");
        assert!(registry.slots.iter().all(Option::is_none));
        assert_eq!(registry.aggregate_bytes, 0, "both producers close once to exact zero aggregate credit");
    }
}

#[test]
fn fill_worker_cross_generation_restore_preserves_dropped_closing_handle_and_zero_credit() {
    let _guard = fill_envelope_test_guard();
    let mut mounted = fill_worker_session(53);
    let (_, mounted_token) = enqueue_measured_fill_job(&mut mounted).expect("mounted request");
    let mounted_request = decode_fill_envelope_token(&mounted_token).expect("mounted identity");
    let mut other = fill_worker_session(55);
    let (_, other_token) = enqueue_measured_fill_job(&mut other).expect("other request");
    let other_request = decode_fill_envelope_token(&other_token).expect("other identity");
    terminalize_fill_envelope(&mounted_request, FillEnvelopeTerminalReason::Closed);
    let mut terminal = mounted.take_terminal_fill_job().expect("mounted terminal");
    close_until_fill_retirement(&mut terminal, &mounted_request);
    let retirement_pointer =
        fill_envelope_registry().lock().expect("registry").authority_mut(&mounted_request).and_then(|authority| authority.fill_retirement.as_ref()).map(|cursor| cursor as *const FillBuilderRetirementCursor as usize).expect("retained close cursor");
    drop(terminal);
    assert!(!mounted.restore_persisted_fill(&other_token));
    let registry = fill_envelope_registry().lock().expect("registry");
    let authority = registry.slots[usize::from(mounted_request.slot)].as_ref().expect("closing authority");
    assert_eq!(authority.request, mounted_request);
    assert!(matches!(authority.phase, FillEnvelopePhase::Closing));
    assert!(!authority.checked_out.load(Ordering::Acquire));
    assert_eq!(authority.fill_retirement.as_ref().map(|cursor| cursor as *const FillBuilderRetirementCursor as usize), Some(retirement_pointer));
    drop(registry);
    drop(mounted);
    drop(other);
    drain_orphaned_fill_envelope(&mounted_request);
    drain_orphaned_fill_envelope(&other_request);
    let registry = fill_envelope_registry().lock().expect("registry");
    assert!(registry.slots.iter().all(Option::is_none));
    assert_eq!(registry.aggregate_bytes, 0);
}

#[test]
fn fill_worker_fixed_cap_rejects_plus_one_and_reused_slot_rejects_aba() {
    let _guard = fill_envelope_test_guard();
    let mut sessions = [fill_worker_session(11), fill_worker_session(13), fill_worker_session(17), fill_worker_session(19), fill_worker_session(23)];
    let tokens = [0, 1, 2, 3].map(|index| enqueue_measured_fill_job(&mut sessions[index]).expect("within fixed operation cap").1);
    let rejected_pointer = Arc::as_ptr(sessions[4].engine.fill.as_ref().expect("rejected source remains in session"));
    assert!(enqueue_measured_fill_job(&mut sessions[4]).is_none(), "operation cap + 1 is rejected before ownership transfer");
    assert_eq!(Arc::as_ptr(sessions[4].engine.fill.as_ref().expect("exact rejected owner")), rejected_pointer);

    let first = decode_fill_envelope_token(&tokens[0]).expect("first token");
    assert!(sessions[0].cancel_fill_job());
    let _ = sessions[0].drive_fill_job(&first);
    let returned = sessions[0].take_terminal_fill_job().expect("cancelled terminal owner");
    assert_eq!(returned.reason(), Some("cancelled"));
    drop(returned);
    let mut terminal = sessions[0].take_terminal_fill_job().expect("Drop atomically returns the checked-out terminal authority");
    while !matches!(terminal.close_step(), FillEnvelopeCloseStep::Complete) {}

    let replacement = enqueue_measured_fill_job(&mut sessions[4]).expect("capacity re-arms after exact close").1;
    let replacement = decode_fill_envelope_token(&replacement).expect("replacement token");
    assert_eq!(replacement.slot, first.slot);
    assert_ne!(replacement.registry_generation, first.registry_generation);
    assert!(matches!(drive_fill_envelope(&first), FillEnvelopeDrive::Stale), "a stale generation cannot consume the reused slot");
    close_fill_envelope(&mut sessions[1]);
    close_fill_envelope(&mut sessions[2]);
    close_fill_envelope(&mut sessions[3]);
    close_fill_envelope(&mut sessions[4]);
}

#[test]
fn fill_worker_item_and_byte_plus_one_reject_before_owner_transfer() {
    let mut engine = fill_capable_engine();
    let fill = engine.fill.take().expect("fill owner");
    let pointer = Arc::as_ptr(&fill);
    let operation = fill.lock().expect("fill lock").operation;
    let mut registry = FillEnvelopeRegistry::default();
    let fill = registry.reserve(1, operation, FILL_ENVELOPE_MAX_ITEMS + 1, FILL_ENVELOPE_MAX_BYTES, fill, root_cancel_token(), 1, 0, FillObservation::default()).expect_err("item cap + 1");
    assert_eq!(Arc::as_ptr(&fill), pointer);
    let fill = registry.reserve(2, operation, FILL_ENVELOPE_MAX_ITEMS, FILL_ENVELOPE_MAX_BYTES + 1, fill, root_cancel_token(), 1, 0, FillObservation::default()).expect_err("byte cap + 1");
    assert_eq!(Arc::as_ptr(&fill), pointer, "both preflight failures return the exact source authority");
}

#[test]
fn fill_worker_actual_owner_census_rejects_cap_plus_one_with_exact_handback() {
    let _guard = fill_envelope_test_guard();
    let mut session = fill_worker_session(29);
    let source = session.engine.fill.as_ref().expect("fill owner").clone();
    let pointer = Arc::as_ptr(&source);
    source.lock().expect("fill lock").inject_nested_owner_page_plus_one_for_test();
    assert!(enqueue_measured_fill_job(&mut session).is_none());
    assert!(session.engine.fill.is_none(), "registered fault ownership cannot recreate a mutable engine alias");
    assert!(session.fill_admission.is_none(), "rejected census cannot strand a partially measured owner");
    let request = session.fill_job.clone().expect("rejected registered owner");
    let mut registry = fill_envelope_registry().lock().expect("registry");
    let rejected = registry.authority_mut(&request).and_then(|authority| authority.fill.as_ref()).expect("registered rejected owner");
    assert_eq!(Arc::as_ptr(rejected), pointer, "nested ObjectKind backing cap + 1 keeps the exact source authority in the registered fault owner");
    drop(registry);
    drop(source);
    drop(session);
    drain_orphaned_fill_envelope(&request);
}

#[test]
fn fill_worker_admitted_fixed_pages_survive_replan_and_mesh_supersession_until_retained_close() {
    let _guard = fill_envelope_test_guard();
    let mut session = fill_worker_session(43);
    let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
    let request = decode_fill_envelope_token(&token).expect("request");
    let (admitted_pointer, admitted_credit, admitted_backing) = {
        let registry = fill_envelope_registry().lock().expect("registry");
        let authority = registry.slots[usize::from(request.slot)].as_ref().filter(|authority| authority.request == request).expect("admitted authority");
        let fill = authority.fill.as_ref().expect("exclusive fill").lock().expect("fill lock");
        (Arc::as_ptr(authority.fill.as_ref().expect("fill")), (authority.reserved_items, authority.reserved_bytes), fill.fixed_backing_witness_for_test())
    };
    assert!(session.engine.fill.is_none(), "admission removes the last mutable session alias");

    let mut object_weights = std::collections::BTreeMap::new();
    object_weights.insert("Host".to_string(), 3.0);
    session.dispatch(Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights: std::collections::BTreeMap::new() }).expect("superseding weights");
    let weight_replacement_pointer = Arc::as_ptr(session.engine.fill.as_ref().expect("separate weight replacement candidate"));
    assert_ne!(weight_replacement_pointer, admitted_pointer, "weight supersession builds a distinct unadmitted candidate");
    let (positions, indices) = unit_cube_mesh_buffers();
    session.register_mesh("/test/superseding.glb", &positions, &indices);
    let replacement_pointer = Arc::as_ptr(session.engine.fill.as_ref().expect("separate mesh replacement candidate"));
    assert_ne!(replacement_pointer, weight_replacement_pointer, "mesh supersession replaces the unadmitted weight candidate without touching the admitted owner");

    let registry = fill_envelope_registry().lock().expect("registry");
    let authority = registry.slots[usize::from(request.slot)].as_ref().filter(|authority| authority.request == request).expect("old admitted authority");
    let fill = authority.fill.as_ref().expect("old exclusive fill").lock().expect("fill lock");
    assert_eq!(Arc::as_ptr(authority.fill.as_ref().expect("fill")), admitted_pointer);
    assert_eq!((authority.reserved_items, authority.reserved_bytes), admitted_credit, "the old exact admission credit remains assigned until close");
    assert_eq!(fill.fixed_backing_witness_for_test(), admitted_backing, "weight and mesh refresh cannot clear, replace, or drop any admitted fixed page or semantic entry");
    drop(fill);
    drop(registry);

    close_fill_envelope(&mut session);
    assert_eq!(Arc::as_ptr(session.engine.fill.as_ref().expect("replacement survives old close")), replacement_pointer);
    let (_, replacement_token) = enqueue_measured_fill_job(&mut session).expect("replacement is independently re-censused and admitted");
    let replacement = decode_fill_envelope_token(&replacement_token).expect("replacement request");
    assert_ne!(replacement, request, "the 4096-unit re-census admits a new identity, not the closed owner");
    if replacement.slot == request.slot {
        assert!(replacement.registry_generation > request.registry_generation, "same-slot reuse must bump the per-slot generation");
    }
    close_fill_envelope(&mut session);
}

#[test]
fn fill_worker_session_drop_during_measurement_mounts_the_same_terminal_once() {
    let _guard = fill_envelope_test_guard();
    let mut session = fill_worker_session(31);
    assert!(session.enqueue_fill_job_spending(1).is_none(), "one census unit only begins exact owner measurement");
    let request = session.fill_job.clone().expect("measurement has a registered exact owner");
    assert!(session.fill_admission.is_some());
    let registry = fill_envelope_registry().lock().expect("registry contention");
    drop(session);
    assert_eq!(fill_envelope_terminal_intents()[usize::from(request.slot)].reason.load(Ordering::Acquire), FillEnvelopeTerminalReason::Closed.code());
    drop(registry);
    drain_orphaned_fill_envelope(&request);
}

#[test]
fn fill_admission_census_one_unit_is_pending_and_4096_units_spawn_once() {
    let _guard = fill_envelope_test_guard();
    let mut pending = fill_worker_session(33);
    assert!(pending.enqueue_fill_job_spending(1).is_none(), "one owner-census unit cannot finish a prepared fill envelope");
    assert!(pending.fill_admission.is_some());
    let measuring = pending.fill_job.clone().expect("measuring identity");
    assert_eq!(FILL_ENVELOPE_CENSUS_UNITS_PER_TURN, 4_096, "production census spend is the 4096-unit rule");
    assert!(pending.enqueue_fill_job().is_some(), "one production turn of 4096 census units admits and spawns exactly once");
    assert!(pending.fill_admission.is_none(), "the finished census hands the envelope to its bounded job");
    assert_eq!(pending.fill_job.as_ref(), Some(&measuring), "the spawn keeps the measuring identity — it does not re-admit");
    let mut second = fill_worker_session(35);
    assert!(second.enqueue_fill_job().is_some(), "a fresh session on the same registry gets its own spawn under the four-slot cap");
    drop(pending);
    drop(second);
}

#[test]
fn fill_worker_completed_before_session_drop_is_reclassified_and_mounted_once() {
    let _guard = fill_envelope_test_guard();
    let mut session = fill_worker_session(33);
    let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
    let request = decode_fill_envelope_token(&token).expect("request");
    {
        let mut registry = fill_envelope_registry().lock().expect("registry");
        let authority = registry.authority_mut(&request).expect("authority");
        authority.phase = FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Complete);
        authority.observation.done = true;
    }
    drop(session);
    // ♻️ The dying session ASKS for the `Closed` terminal and stops there — draining the ladder inside
    // a `Drop` is the unyielding turn of wave B42. What reclassifies the completed envelope and returns
    // its slot is the granted reaper, exactly once, and a fresh session must not race it.
    let standing = fill_envelope_registry().lock().expect("registry").slots[usize::from(request.slot)].is_some();
    assert!(standing, "a dying session may only ASK for the terminal; the slot comes back on a granted turn");
    drain_orphaned_fill_envelope(&request);
    let mut mounted = Puzzle3dPrecomputeSession::new();
    assert!(!mounted.poll_fill_job(), "the same terminal cannot mount a second time");
    assert!(mounted.fill_terminal.is_none());
    drop(mounted);
}

#[test]
fn fill_worker_session_drop_during_partial_close_rearms_the_same_cursor_once() {
    let _guard = fill_envelope_test_guard();
    let mut session = fill_worker_session(34);
    let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
    let request = decode_fill_envelope_token(&token).expect("request");
    terminalize_fill_envelope(&request, FillEnvelopeTerminalReason::Closed);
    let mut terminal = session.take_terminal_fill_job().expect("terminal handle");
    close_until_fill_retirement(&mut terminal, &request);
    let registry = fill_envelope_registry().lock().expect("registry contention");
    let authority = registry.slots[usize::from(request.slot)].as_ref().expect("closing authority");
    assert!(matches!(authority.phase, FillEnvelopePhase::Closing));
    let retirement_pointer = authority.fill_retirement.as_ref().map(|cursor| cursor as *const FillBuilderRetirementCursor as usize).expect("partial retirement cursor");
    session.fill_terminal = Some(terminal);
    drop(session);
    drop(registry);

    let mut mounted = Puzzle3dPrecomputeSession::new();
    assert!(mounted.poll_fill_job(), "the durable close intent re-arms the abandoned Closing generation");
    assert_eq!(mounted.fill_terminal.as_ref().map(|terminal| &terminal.request), Some(&request));
    let registry = fill_envelope_registry().lock().expect("registry");
    let authority = registry.slots[usize::from(request.slot)].as_ref().expect("same closing authority");
    assert_eq!(authority.fill_retirement.as_ref().map(|cursor| cursor as *const FillBuilderRetirementCursor as usize), Some(retirement_pointer), "reclamation resumes rather than resets or duplicates the cursor");
    drop(registry);
    for _ in 0..FILL_ENVELOPE_MAX_ITEMS {
        mounted.poll_fill_job();
        if fill_envelope_registry().lock().expect("registry").slots[usize::from(request.slot)].is_none() {
            break;
        }
    }
    let mut registry = fill_envelope_registry().lock().expect("registry");
    assert!(registry.slots[usize::from(request.slot)].is_none(), "the resumed close reaches terminal empty");
    assert!(registry.take_closed().is_none(), "the same Closing generation is never rediscovered twice");
}

#[test]
fn fill_worker_terminal_resume_contention_returns_then_rearms_the_exact_owner() {
    let _guard = fill_envelope_test_guard();
    let mut session = fill_worker_session(35);
    let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
    let request = decode_fill_envelope_token(&token).expect("request");
    terminalize_fill_envelope(&request, FillEnvelopeTerminalReason::Fault);
    let terminal = session.take_terminal_fill_job().expect("terminal owner");
    let registry = fill_envelope_registry().lock().expect("contended registry");
    let terminal = terminal.resume().expect_err("contention returns the same checked-out terminal handle");
    assert_eq!(terminal.request, request);
    drop(registry);
    let resumed = match terminal.resume() {
        Ok(token) => token,
        Err(_) => panic!("capacity change must re-arm the exact owner once"),
    };
    assert_eq!(resumed, token);
    terminalize_fill_envelope(&request, FillEnvelopeTerminalReason::Closed);
    drop(session);
    drain_orphaned_fill_envelope(&request);
}

#[test]
fn fill_worker_malformed_token_faults_exact_raw_owner_not_wrong_context_owner() {
    let _guard = fill_envelope_test_guard();
    let mut session = fill_worker_session(39);
    let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
    let request = decode_fill_envelope_token(&token).expect("request");
    let mut unrelated = fill_worker_session(57);
    let (_, unrelated_token) = enqueue_measured_fill_job(&mut unrelated).expect("unrelated job");
    let unrelated_request = decode_fill_envelope_token(&unrelated_token).expect("unrelated request");
    let mut malformed = token;
    malformed[0] ^= 0xff;
    let mut entry = FillEnvelopeJobEntryCursor::new(unrelated_request.job, malformed);
    assert_eq!(entry.step(), Err("fill worker token header is malformed"));
    drop(entry);
    assert_eq!(session.take_terminal_fill_job().and_then(|terminal| terminal.reason()), Some("fault"), "malformed production ingress preserves the exact registered owner before returning the fault");
    let registry = fill_envelope_registry().lock().expect("registry");
    assert!(matches!(registry.slots[usize::from(unrelated_request.slot)].as_ref().map(|authority| authority.phase), Some(FillEnvelopePhase::Admitted)), "malformed ingress cannot fault the wrong context owner");
    drop(registry);
    drop(session);
    drop(unrelated);
    drain_orphaned_fill_envelope(&request);
    drain_orphaned_fill_envelope(&unrelated_request);
}

#[test]
fn fill_worker_wrong_context_identity_faults_decoded_producer_before_drive() {
    let _guard = fill_envelope_test_guard();
    let mut producer = fill_worker_session(59);
    let (_, token) = enqueue_measured_fill_job(&mut producer).expect("producer job");
    let request = decode_fill_envelope_token(&token).expect("producer request");
    let mut unrelated = fill_worker_session(61);
    let (_, unrelated_token) = enqueue_measured_fill_job(&mut unrelated).expect("unrelated job");
    let unrelated_request = decode_fill_envelope_token(&unrelated_token).expect("unrelated request");
    let mut entry = FillEnvelopeJobEntryCursor::new(unrelated_request.job, token);
    let decoded = loop {
        if let Some(decoded) = entry.step().expect("one-field decode") {
            break decoded;
        }
    };
    assert_eq!(decoded, request);
    assert_eq!(entry.bind(&decoded), Err("fill worker context job does not match the decoded request owner"));
    drop(entry);
    assert_eq!(producer.take_terminal_fill_job().and_then(|terminal| terminal.reason()), Some("fault"));
    let registry = fill_envelope_registry().lock().expect("registry");
    assert!(matches!(registry.slots[usize::from(unrelated_request.slot)].as_ref().map(|authority| authority.phase), Some(FillEnvelopePhase::Admitted)), "wrong context identity cannot transition another producer");
    drop(registry);
    drop(producer);
    drop(unrelated);
    drain_orphaned_fill_envelope(&request);
    drain_orphaned_fill_envelope(&unrelated_request);
}

#[test]
fn fill_worker_stale_envelope_identity_is_rejected_without_faulting_replacement() {
    let _guard = fill_envelope_test_guard();
    let mut stale = fill_worker_session(63);
    let (_, stale_token) = enqueue_measured_fill_job(&mut stale).expect("stale job");
    let stale_request = decode_fill_envelope_token(&stale_token).expect("stale request");
    close_fill_envelope(&mut stale);
    let mut replacement = fill_worker_session(65);
    let (_, replacement_token) = enqueue_measured_fill_job(&mut replacement).expect("replacement job");
    let replacement_request = decode_fill_envelope_token(&replacement_token).expect("replacement request");
    let mut entry = FillEnvelopeJobEntryCursor::new(stale_request.job, stale_token);
    let decoded = loop {
        if let Some(decoded) = entry.step().expect("one-field decode") {
            break decoded;
        }
    };
    assert_eq!(entry.bind(&decoded), Err("fill worker envelope owner is stale"));
    drop(entry);
    let registry = fill_envelope_registry().lock().expect("registry");
    assert!(matches!(registry.slots[usize::from(replacement_request.slot)].as_ref().map(|authority| authority.phase), Some(FillEnvelopePhase::Admitted)), "stale identity cannot fault the live replacement or any no-owner slot");
    drop(registry);
    close_fill_envelope(&mut replacement);
}

#[test]
fn fill_worker_mounted_terminal_pump_closes_completed_slot_and_rearms_capacity() {
    let _guard = fill_envelope_test_guard();
    let mut session = fill_worker_session(37);
    let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
    let request = decode_fill_envelope_token(&token).expect("request");
    assert!(session.cancel_fill_job());
    let _ = session.drive_fill_job(&request);
    for _ in 0..4096 {
        session.poll_fill_job();
        if session.fill_job.is_none() && session.fill_terminal.is_none() {
            break;
        }
    }
    assert!(session.fill_job.is_none());
    assert!(session.fill_terminal.is_none());
    assert!(fill_envelope_registry().lock().expect("registry").slots[usize::from(request.slot)].is_none());
}

#[test]
fn fill_worker_early_fault_guard_terminalizes_and_deep_retirement_is_incremental() {
    let _guard = fill_envelope_test_guard();
    let mut session = fill_worker_session(41);
    let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
    let request = decode_fill_envelope_token(&token).expect("request");
    {
        let _fault = FillEnvelopeWorkerFaultGuard::new(&token);
    }
    let mut terminal = session.take_terminal_fill_job().expect("fault terminal");
    assert_eq!(terminal.reason(), Some("fault"));
    assert_eq!(terminal.close_step(), FillEnvelopeCloseStep::Pending, "the first close grant only transfers the final builder authority into its retirement cursor");
    assert!(!terminal.terminal_is_empty());
    let mut grants = 1;
    while !matches!(terminal.close_step(), FillEnvelopeCloseStep::Complete) {
        grants += 1;
    }
    assert!(grants > 8, "a populated builder cannot be bulk-dropped by one close grant");
    assert!(terminal.terminal_is_empty());
    assert!(fill_envelope_registry().lock().expect("registry").slots[usize::from(request.slot)].is_none());
}

#[test]
fn fill_worker_token_decode_advances_exactly_one_field_per_grant() {
    let request = FillJobRequest { job: 3, operation: 5, generation: 7, base_revision: 11, slot: 1, registry_generation: 13 };
    let mut cursor = FillEnvelopeTokenCursor::new(fill_envelope_token(&request).to_vec());
    assert_eq!(cursor.step().expect("header"), None);
    assert_eq!(cursor.step().expect("registry generation"), None);
    assert_eq!(cursor.step().expect("job"), None);
    assert_eq!(cursor.step().expect("operation"), None);
    assert_eq!(cursor.step().expect("generation"), None);
    assert_eq!(cursor.step().expect("base revision"), None);
    assert_eq!(cursor.step().expect("publish"), Some(request));
}

#[test]
fn fill_operation_identity_checked_nonzero_exhaustion_permanently_refuses_aba() {
    let mut generation_engine = Puzzle3dCollision::new();
    assert_eq!(generation_engine.allocate_fill_identity(true), Some((RevisionId(1), Generation(1))), "zero counters allocate the first exact nonzero identity");
    generation_engine.fill_generation = u64::MAX - 1;
    assert_eq!(generation_engine.allocate_fill_identity(false), Some((RevisionId(1), Generation(u64::MAX))));
    assert_eq!(generation_engine.allocate_fill_identity(false), None, "generation max + 1 is permanently refused");
    assert_eq!(generation_engine.allocate_fill_identity(false), None);
    assert_eq!((generation_engine.fill_revision, generation_engine.fill_generation), (1, u64::MAX));

    let mut revision_engine = Puzzle3dCollision::new();
    revision_engine.fill_revision = u64::MAX - 1;
    revision_engine.fill_generation = 1;
    assert_eq!(revision_engine.allocate_fill_identity(true), Some((RevisionId(u64::MAX), Generation(2))));
    assert_eq!(revision_engine.allocate_fill_identity(true), None, "revision max + 1 is permanently refused without consuming a generation");
    assert_eq!(revision_engine.allocate_fill_identity(true), None);
    assert_eq!((revision_engine.fill_revision, revision_engine.fill_generation), (u64::MAX, 2));
}

#[test]
fn fill_worker_zero_semantic_counters_and_exhausted_stale_tokens_never_alias() {
    let request = FillJobRequest { job: 3, operation: 5, generation: u64::MAX, base_revision: u64::MAX, slot: 1, registry_generation: 13 };
    let exhausted = fill_envelope_token(&request);
    assert_eq!(decode_fill_envelope_token(&exhausted), Some(request.clone()));
    let mut zero_generation = exhausted;
    zero_generation[40..48].copy_from_slice(&0_u64.to_le_bytes());
    assert!(decode_fill_envelope_token(&zero_generation).is_none());
    let mut zero_revision = exhausted;
    zero_revision[48..56].copy_from_slice(&0_u64.to_le_bytes());
    assert!(decode_fill_envelope_token(&zero_revision).is_none());
    assert_ne!(zero_generation, exhausted, "exhaustion cannot reset to a zero token that aliases the permanent max identity");
    assert_ne!(zero_revision, exhausted);
}

#[test]
fn precompute_session_native_wrapper_errors_without_scene() {
    let mut session = Puzzle3dPrecomputeSession::new();
    assert!(session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).is_err());
    assert!(session.dispatch(Puzzle3dEngineCommand::ComposeFillDisplay { count: 0 }).is_err());
    let payload = BrushPlacePayload { target_vortex_full_id: "a:v0".to_string(), object_kind_id: "b".to_string(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
    assert!(session.dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload }).is_err());
    assert!(session.fill_is_done());
    assert_eq!(session.fill_available_count(), 0);
}

#[test]
fn fill_lane_advances_while_brush_targets_remain_queued() {
    let mut engine = Puzzle3dCollision::new();
    engine.set_scene(&single_object_scene_json()).expect("seed");
    assert!(engine.fill_steps_pending_for_test() > 0, "seed scene must schedule fill steps");
    assert!(engine.brush_lane_active(), "seed scene must schedule brush targets — the queue itself fills from the preparation cursor, one target per lane turn");
    let before = engine.fill_progress_summary().count;
    for _ in 0..24 {
        engine.precompute_step_lane(PrecomputeLane::Fill, 4);
    }
    let after = engine.fill_progress_summary().count;
    assert!(after > before || engine.fill_progress_summary().done, "fill lane must make planning progress without draining brush first");
}

#[test]
fn brush_candidates_cold_cache_returns_pending_without_populating_cache() {
    let mut session = Puzzle3dPrecomputeSession::new();
    session.set_scene(&single_object_scene_json()).expect("seed");
    let result = session.brush_candidates("host:v0");
    assert!(result.unknown_pending, "cold cache must surface pending state: {result:?}");
    assert!(session.brush_preview("host:v0", 0).is_none());
}

/// 🧰️ `enqueue_brush_target` is the app-facing append (vs. `invalidate_brush_target`'s
/// front-of-queue jump) — appending an already-queued id must be a no-operation.
#[test]
fn enqueue_brush_target_appends_once() {
    let mut engine = Puzzle3dCollision::new();
    engine.enqueue_brush_target("host:v0");
    engine.enqueue_brush_target("host:v0");
    assert_eq!(engine.brush_queue.len(), 1);
}

/// 🖐️ Compile-guard for the 🖐️5d app, which builds its own `Puzzle5dPrecomputeSession` on top of
/// this one (relocated from the former `⚙️engine` root's `the_5d_facing_engine_surface_stays_public`,
/// ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): every item it names must stay
/// publicly reachable — pure data under `crate::standards::v1::subsets::any::schema::…`, the session/dispatch
/// surface under `crate::editor::puzzle3d::precompute::…`. A rename or a visibility narrowing breaks
/// this test long before it breaks 5d.
#[test]
fn the_5d_facing_precompute_surface_stays_public() {
    use crate::Puzzle3dError as GuardError;

    let mut session = Puzzle3dPrecomputeSession::new();
    assert!(session.set_scene("{ not json").is_err(), "set_scene surfaces a Puzzle3dError");
    session.register_mesh("/probe.glb", &[], &[]);
    assert!(!session.has_mesh("/probe.glb"));
    assert!(!session.precompute_step(1));
    let _: BrushCollisionFreeResult = session.brush_candidates("probe:v0");
    let _: Option<BrushPreviewState> = session.brush_preview("probe:v0", 0);
    let _: FillBuildProgress = session.fill_progress();
    assert!(session.precompute_step_lane(PrecomputeLane::Brush, 1) || true);
    let payload = BrushPlacePayload { target_vortex_full_id: "probe:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
    let rejected: Result<Puzzle3dEngineOutcome, GuardError> = session.dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload });
    assert!(matches!(rejected, Err(GuardError::BrushPlacementRejected)));
    assert!(session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).is_err());
    let _: fn(&Fixture, &BrushPlacePayload, &KindCatalogBundle) -> Fixture = apply_brush_placement_to_fixture;
}

/// 🔗️ Minimal scene JSON matching `SceneConfig`'s real wire shape (camelCase, per its
/// `#[serde(rename = ...)]` attrs) — relocated from `🧬️mutations/💾️binary/🦀️.rs`'s own
/// `sample_scene_config` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): that file's
/// own copy stays for the pure-data wire-format-guard tests that need no session, and this copy feeds
/// the two dispatch tests below, since a schema-side test module must not depend on the app.
fn sample_scene_config() -> SceneConfig {
    let json = r#"{
            "fixture": {
                "objects": [{"id": "host", "objectKind": "Host", "meshUrl": "/test/host.glb", "origin": [0,0,0], "orientation": [0,0,0,1], "vortices": [{"id": "v0", "vortexKind": "port-a", "position": [0,0,0], "direction": [0,0,-1]}]}],
                "attractions": [],
                "targetVolumes": []
            },
            "kindCatalogs": {"objects": [{"id": "Host", "representations": [{"id": "r0", "name": "default", "url": "/test/host.glb"}], "vortices": []}], "vortices": [{"id": "port-a"}], "cables": []},
            "kindCompatibility": [],
            "overlapBudget": 0.02,
            "seed": 1
        }"#;
    serde_json::from_str(json).expect("sample scene config parses")
}

/// 🎯️ Behavioral parity: `dispatch` must reach the exact same engine logic the old JSON-string
/// wasm-bindgen methods delegated to — `SetScene` seeds a fill session, `ApplyFillCount`/
/// `ComposeFillDisplay` read/apply its prefix, matching what this module's own
/// `precompute_session_native_wrapper_exercises_public_methods` test already asserts for the
/// pre-dispatch API. Relocated from `🧬️mutations/💾️binary/🦀️.rs`'s
/// `dispatch_set_scene_then_apply_and_compose_fill_count_round_trip` — that test constructed
/// `Puzzle3dPrecomputeSession` directly, which is now an app type a schema test file must not reach.
#[test]
fn dispatch_set_scene_then_apply_and_compose_fill_count_round_trip() {
    let mut session = Puzzle3dPrecomputeSession::new();
    session.dispatch(Puzzle3dEngineCommand::SetScene { scene: sample_scene_config() }).expect("set scene");
    assert!(!session.fill_is_done(), "a freshly seeded fill session has not stalled or hit max_count yet");

    session.precompute_step(50);

    let outcome = session.dispatch(Puzzle3dEngineCommand::ComposeFillDisplay { count: 0 }).expect("compose fill display");
    let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
    assert!(fixture.objects.iter().any(|object| object.id == "host"), "the base scene's host object must survive compose_fill_display(0)");

    let outcome = session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).expect("apply fill count");
    let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
    assert!(fixture.objects.iter().any(|object| object.id == "host"));
}

/// 🎯️ Relocated from `🧬️mutations/💾️binary/🦀️.rs`'s
/// `dispatch_brush_preview_without_scene_returns_none` (same reason as the test above).
#[test]
fn dispatch_brush_preview_without_scene_returns_none() {
    let mut session = Puzzle3dPrecomputeSession::new();
    let outcome = session.dispatch(Puzzle3dEngineCommand::BrushPreview { vortex_full_id: "host:v0".to_string(), candidate_index: 0 }).expect("brush preview never errors");
    assert_eq!(outcome, Puzzle3dEngineOutcome::BrushPreview(None), "no scene means no cached brush candidates yet");
}

#[test]
fn fill_job_checkpoint_is_a_fixed_generation_token_not_a_whole_state_buffer() {
    let _guard = fill_envelope_test_guard();
    let mut session = fill_worker_session(31);
    let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill envelope");
    assert_eq!(token.len(), FILL_ENVELOPE_TOKEN_BYTES);
    assert_eq!(session.fill_checkpoint_bytes(), token);
    assert!(serde_json::from_slice::<SceneConfig>(&token).is_err(), "the job checkpoint cannot regress to whole-scene serde");
    close_fill_envelope(&mut session);
}

#[test]
fn fill_first_substantive_preview_arrives_below_fifty_ms_and_every_step_below_eight_ms() {
    let mut engine = fill_capable_engine();
    let started = Instant::now();
    let mut first_preview = None;
    let mut completed = false;
    let mut max_step = Duration::ZERO;
    for _ in 0..10_000 {
        let step_started = Instant::now();
        let active = engine.precompute_step_lane(PrecomputeLane::Fill, 1);
        let step_elapsed = step_started.elapsed();
        max_step = max_step.max(step_elapsed);
        assert!(step_elapsed < Duration::from_millis(8), "fill resume step reached the 8ms ceiling");
        if first_preview.is_none() && engine.fill.as_ref().and_then(|fill| fill.lock().ok()).is_some_and(|fill| fill.preview.candidate_ghost.is_some()) {
            first_preview = Some(started.elapsed());
        }
        if !active {
            completed = true;
            break;
        }
    }
    assert!(
        first_preview.is_some_and(|elapsed| elapsed < Duration::from_millis(50)),
        "first substantive fill preview exceeded 50ms: {first_preview:?}; stage={:?}; rejected={}",
        engine.fill.as_ref().and_then(|fill| fill.lock().ok()).map(|fill| fill.stage),
        engine.fill.as_ref().and_then(|fill| fill.lock().ok()).map_or(0, |fill| fill.preview.rejected_count)
    );
    assert!(completed, "fill did not complete within the bounded resume budget");
}

/// 🗺️ Wave W-P: a Nakagin-scale scene (180 objects, one per spatial cell) driven through the persistent
/// interactive brush broad phase. The former implementation rebuilt a `Vec<PlacedCollisionEntry>` from a
/// full fixture scan and linear-scanned it for every candidate — `O(N × C)` per popped vortex. The index
/// must instead visit only the cells the preview's own bounds span, which is what the examined-cell and
/// examined-member witnesses assert: strictly fewer members than the scene holds.
#[test]
fn nakagin_scale_brush_broad_phase_visits_only_the_queried_cells() {
    const OBJECTS: usize = 180;
    const SPACING: f64 = 12.0;
    let mut engine = Puzzle3dCollision::new();
    let (positions, indices) = unit_cube_mesh_buffers();
    engine.register_mesh("/test/host.glb".to_string(), &positions, &indices);
    let objects = (0..OBJECTS)
        .map(|index| FixtureObject {
            id: format!("object-{index}"),
            object_kind: Some("Host".to_string()),
            anchor: Default::default(),
            mesh_url: Some("/test/host.glb".to_string()),
            origin: [SPACING * index as f64, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
            reveal_index: None,
        })
        .collect::<Vec<_>>();
    let scene = SceneConfig {
        fixture: Fixture { objects, attractions: vec![], target_volumes: vec![] },
        kind_catalogs: Some(KindCatalogBundle {
            objects: vec![ObjectKind {
                id: "Host".to_string(),
                representations: vec![ObjectKindRepresentation { id: "host".into(), name: String::new(), url: "/test/host.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                scale: None,
                vortices: vec![],
            }],
            vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None, ..Default::default() }],
            cables: vec![],
        }),
        kind_compatibility: vec![],
        overlap_budget: DEFAULT_OVERLAP_BUDGET,
        seed: 1,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    };
    engine.set_scene(&serde_json::to_string(&scene).expect("scene json")).expect("set scene");

    let mut steps = 0_usize;
    while engine.step_brush_index() {
        steps += 1;
        assert!(steps < 64 * OBJECTS, "brush index reconciliation must terminate in bounded steps");
    }
    assert!(engine.brush_index_ready, "the broad phase must report ready once reconciliation drains");
    assert_eq!(engine.brush_index.entry_len(), OBJECTS, "every meshed object owns exactly one indexed entry");
    assert_eq!(engine.brush_placed.len(), OBJECTS, "every indexed owner resolves to a placement in constant time");

    let preview = BrushPreviewState {
        target_vortex_full_id: "object-0:v0".to_string(),
        object_kind_id: "Host".to_string(),
        source_vortex_index: 0,
        mesh_url: "/test/host.glb".to_string(),
        origin: [SPACING, 0.0, 0.0],
        orientation: [0.0, 0.0, 0.0, 1.0],
        scale: None,
    };
    let deadline = puzzle3d_deadline(PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US * 64).expect("clock");
    let (page, (cells, members)) = engine.brush_broad_phase_page(&preview, "object-0", deadline).expect("broad phase page");
    assert!(cells <= 32, "a one-cube query may only walk its own cell span, walked {cells}");
    assert!(members < OBJECTS, "the query examined {members} members of a {OBJECTS}-object scene — that is a linear scan");
    assert!(page.iter().all(|entry| entry.object_id != "object-0"), "the queried host is never its own collision pair");
    assert!(page.len() <= 2, "only the immediate spatial neighbours may reach the narrow phase, got {}", page.len());
}

/// 🔁️ Wave W-P: moving one object must reconcile the persistent index incrementally — the moved owner's
/// entry follows it and every other entry stays put — instead of rebuilding the whole scene.
#[test]
fn brush_broad_phase_follows_one_moved_object_without_a_rebuild() {
    let mut engine = fill_capable_engine();
    while engine.step_brush_index() {}
    let before = *engine.brush_index.entry_bounds("host").expect("host entry");
    let mut scene = (*engine.scene.clone().expect("scene")).clone();
    scene.fixture.objects[0].origin = [64.0, 0.0, 0.0];
    engine.set_scene(&serde_json::to_string(&scene).expect("scene json")).expect("moved scene");
    while engine.step_brush_index() {}
    let after = *engine.brush_index.entry_bounds("host").expect("host entry after move");
    assert_ne!(before.min, after.min, "the moved owner's indexed bounds must follow it");
    assert_eq!(engine.brush_index.entry_len(), 1, "an incremental move must not leave a stale duplicate entry");
    assert!(engine.brush_placed.contains_key("host"), "the placement lookup follows the same move");
}

/// 🗑️ Wave W-P: an owner the new scene no longer carries must leave the broad phase through the
/// production `CollisionIndexRemoval` path — the same withdrawal that used to be `#[cfg(test)]`-gated. The
/// object is re-identified rather than deleted outright, because a fixture that drops to zero objects is
/// indistinguishable from an already-applied fill projection to `set_scene`'s own heuristic and would not
/// install a new scene at all.
#[test]
fn brush_broad_phase_withdraws_an_owner_the_scene_dropped() {
    let mut engine = fill_capable_engine();
    while engine.step_brush_index() {}
    assert!(engine.brush_index.entry_bounds("host").is_some(), "the original owner is indexed");
    let mut scene = (*engine.scene.clone().expect("scene")).clone();
    scene.fixture.objects[0].id = "successor".to_string();
    engine.set_scene(&serde_json::to_string(&scene).expect("scene json")).expect("re-identified scene");
    while engine.step_brush_index() {}
    assert!(engine.brush_index.entry_bounds("host").is_none(), "the dropped owner must be withdrawn, not orphaned");
    assert!(engine.brush_index.entry_bounds("successor").is_some(), "the new owner is indexed in its place");
    assert_eq!(engine.brush_index.entry_len(), 1, "withdrawal and insertion leave exactly one live entry");
    assert!(!engine.brush_placed.contains_key("host") && engine.brush_placed.contains_key("successor"), "the placement lookup follows the same withdrawal");
}

/// 🥽️ Wave W-P: one mesh identity decodes once per process through the `puzzle3d.mesh-decode` engine, and
/// a brand-new session adopts the derived geometry by id alone — the wire never carries buffers twice.
#[test]
fn a_registered_mesh_is_shared_by_id_across_sessions() {
    let (positions, indices) = unit_cube_mesh_buffers();
    let url = "/test/shared-by-id.glb";
    let (derived_positions, derived_indices) = derive_brush_mesh(url, &positions, &indices).expect("derived geometry");
    assert_eq!(derived_positions, positions, "the decode kernel returns the exact validated geometry");
    assert_eq!(derived_indices, indices);
    assert_eq!(shared_brush_mesh(url).expect("cached geometry").0, positions, "a second read hits the content-addressed cache");
    let mut fresh = Puzzle3dCollision::new();
    assert!(!fresh.has_mesh(url), "a fresh engine starts without any mesh");
    assert!(fresh.adopt_shared_mesh(url, None), "the id alone is enough to install real geometry");
    assert!(fresh.has_mesh(url), "the adopted mesh is live in the collision engine");
    assert!(derive_brush_mesh(url, &positions[..6], &indices).is_none(), "a malformed upload is refused by the kernel, not cached");
}

/// 🛑 Wave W-P: the user-facing fill cancel is identity-guarded. A cancel naming a superseded run must
/// not touch the live job; the exact `(job, operation, generation)` triple must stop it and drive its
/// envelope to terminal-empty. `cancel_fill_job` had zero callers before `cancelFillBuild`.
#[test]
fn fill_cancel_stops_only_the_named_job_and_a_stale_cancel_is_a_no_operation() {
    let _guard = fill_envelope_test_guard();
    let mut session = fill_worker_session(37);
    let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill envelope");
    let request = decode_fill_envelope_token(&token).expect("request");
    let identity = session.fill_job_identity().expect("live fill identity");
    assert_eq!(identity, (request.job, request.operation, request.generation));
    let (job, operation, generation) = identity;
    assert!(!session.cancel_fill_job_for(job.wrapping_add(1), operation, generation), "a stale job id must not cancel the live run");
    assert!(!session.cancel_fill_job_for(job, operation.wrapping_add(1), generation), "a stale operation must not cancel the live run");
    assert!(!session.cancel_fill_job_for(job, operation, generation.wrapping_add(1)), "a stale generation must not cancel the live run");
    let live = {
        let registry = fill_envelope_registry().lock().expect("registry");
        registry.slots[usize::from(request.slot)].as_ref().and_then(|authority| authority.cancel.clone()).expect("live cancel token")
    };
    assert!(!live.is_cancelled_now(), "every stale cancel left the live job running");
    assert!(session.cancel_fill_job_for(job, operation, generation), "the exact identity cancels the live run");
    assert!(live.is_cancelled_now(), "the named cancel reached the job's own cancel token");
    let _ = session.drive_fill_job(&request);
    let mut terminal = session.take_terminal_fill_job().expect("terminal handle");
    while !matches!(terminal.close_step(), FillEnvelopeCloseStep::Complete) {}
    assert!(terminal.terminal_is_empty());
}

//#region 🧩️PagedBrushMeshUploads
/// 🥽️ The language-neutral upload contract both ends implement: this decoder, and the renderer's pager
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`),
/// which is held to the same fixture by `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`.
const BRUSH_MESH_UPLOAD_FIXTURE: &str = include_str!("../../../../🧫️fixtures/🥽️brush-mesh-upload/🔣️.json");

fn brush_mesh_upload_fixture() -> serde_json::Value {
    serde_json::from_str(BRUSH_MESH_UPLOAD_FIXTURE).expect("brush mesh upload fixture")
}

fn fixture_count(value: &serde_json::Value) -> usize {
    usize::try_from(value.as_u64().expect("whole count")).expect("count fits this target")
}

/// 🧮️ One client page run encoded exactly as `puzzle3dBrushMeshPages` encodes it: positions fill each
/// page first, the indices stream continues in whatever of the page's value budget is left.
fn brush_mesh_pages(positions: &[f32], indices: &[u32]) -> Vec<(String, String)> {
    let total = positions.len() + indices.len();
    let mut pages = Vec::new();
    let mut start = 0usize;
    while start < total {
        let end = (start + PUZZLE3D_MESH_PAGE_VALUES).min(total);
        let position_slice = &positions[start.min(positions.len())..end.min(positions.len())];
        let index_slice = &indices[start.saturating_sub(positions.len())..end.saturating_sub(positions.len())];
        let position_bytes: Vec<u8> = position_slice.iter().flat_map(|value| value.to_le_bytes()).collect();
        let index_bytes: Vec<u8> = index_slice.iter().flat_map(|value| value.to_le_bytes()).collect();
        pages.push((semio_framework_io_base64::base64_standard_encode(&position_bytes), semio_framework_io_base64::base64_standard_encode(&index_bytes)));
        start = end;
    }
    pages
}

/// 🚚️ One page as the plugin receives it: decoded out of the two base64 payloads under the same value
/// budget the `registerBrushMesh` arm applies, then admitted into the staging area.
fn admit_brush_mesh_page(url: &str, digest: &str, page: u32, page_count: u32, positions_b64: &str, indices_b64: &str) -> Result<Puzzle3dMeshUploadStep, Puzzle3dMeshUploadFault> {
    let positions: Vec<f32> = decode_brush_mesh_page_values(positions_b64, PUZZLE3D_MESH_PAGE_VALUES).expect("positions payload").iter().map(|bytes| f32::from_le_bytes(*bytes)).collect();
    let indices: Vec<u32> = decode_brush_mesh_page_values(indices_b64, PUZZLE3D_MESH_PAGE_VALUES - positions.len()).expect("indices payload").iter().map(|bytes| u32::from_le_bytes(*bytes)).collect();
    stage_brush_mesh_page(url, digest, page, page_count, &positions, &indices)
}

/// 📏️ The wire one page actually costs, byte-for-byte what the plugin host writes for a UI dispatch:
/// `to_json_string(&(verb, args))` over the args the world layer sends.
fn brush_mesh_page_wire_bytes(url: &str, digest: &str, page: u32, page_count: u32, positions_b64: &str, indices_b64: &str) -> usize {
    serde_json::to_string(&serde_json::json!(["registerBrushMesh", { "surfaceId": "world-3d", "url": url, "digest": digest, "page": page, "pageCount": page_count, "positionsB64": positions_b64, "indicesB64": indices_b64 }])).expect("wire json").len()
}

/// 🥽️ Wave W-M2: a document-scale GLB — the Nakagin capsule `🧊️placeholder.glb`, 25 344 positions and
/// 48 384 indices — reaches the plugin only as a page run, because one whole mesh is 64 KB of JSON and
/// the shared retained command admits 8 192 raw bytes. Every page fits that wire, the run closes on its
/// last page, and the reassembled geometry is byte-identical to what the client loaded.
#[test]
fn a_document_scale_mesh_uploads_in_pages_and_registers() {
    let fixture = brush_mesh_upload_fixture();
    let scale = &fixture["documentScale"][0];
    let position_count = fixture_count(&scale["positions"]);
    let index_count = fixture_count(&scale["indices"]);
    let positions: Vec<f32> = (0..position_count).map(|value| value as f32 * 0.5).collect();
    let vertices = u32::try_from(position_count / 3).expect("vertex count");
    let indices: Vec<u32> = (0..index_count).map(|value| u32::try_from(value).expect("index") % vertices).collect();
    let url = "/test/document-scale.glb";
    let digest = brush_mesh_digest(&positions, &indices);
    let pages = brush_mesh_pages(&positions, &indices);
    assert_eq!(pages.len(), fixture_count(&scale["pages"]), "the document-scale mesh pages exactly as the language-neutral fixture declares");
    let page_count = u32::try_from(pages.len()).expect("page count");
    let mut closed = None;
    for (index, (positions_b64, indices_b64)) in pages.iter().enumerate() {
        let page = u32::try_from(index).expect("page");
        assert!(positions_b64.len() <= PUZZLE3D_MESH_PAGE_BASE64_CHARS && indices_b64.len() <= PUZZLE3D_MESH_PAGE_BASE64_CHARS, "page {page} payload stays inside one page's base64 budget");
        let wire = brush_mesh_page_wire_bytes(url, &digest, page, page_count, positions_b64, indices_b64);
        assert!(wire <= crate::retained_command::PUZZLE_COMMAND_RAW_BYTES, "page {page} costs {wire} raw wire bytes; the shared retained command admits {}", crate::retained_command::PUZZLE_COMMAND_RAW_BYTES);
        match admit_brush_mesh_page(url, &digest, page, page_count, positions_b64, indices_b64).expect("page admitted") {
            Puzzle3dMeshUploadStep::Staged { next_page, page_count: staged_count } => {
                assert_eq!((next_page, staged_count), (page + 1, page_count), "an open run advances exactly one page");
                assert!(staged_brush_mesh_uploads().iter().any(|(staged_url, staged_digest, _, _)| staged_url == url && staged_digest == &digest), "the open run holds exactly one staging slot");
            }
            Puzzle3dMeshUploadStep::Complete(staged_positions, staged_indices) => closed = Some((page, staged_positions, staged_indices)),
        }
    }
    let (last_page, staged_positions, staged_indices) = closed.expect("the run closes");
    assert_eq!(last_page, page_count - 1, "only the last page closes the run");
    assert_eq!(staged_positions, positions, "the reassembled positions are the client's own buffer");
    assert_eq!(staged_indices, indices, "the reassembled indices are the client's own buffer");
    assert!(!staged_brush_mesh_uploads().iter().any(|(staged_url, _, _, _)| staged_url == url), "a closed run leaves no staging slot behind");
    let mut session = Puzzle3dPrecomputeSession::new();
    session.register_mesh(url, &staged_positions, &staged_indices);
    assert!(session.has_mesh(url), "the reassembled mesh is live collision geometry, not just staged bytes");
}

/// 🕳️ Wave W-M2: a run that skips a page, contradicts its own page count, or reassembles into bytes the
/// client never announced is refused with a named fault — never silently half-installed.
#[test]
fn a_gapped_or_mismatched_page_run_is_refused() {
    let (positions, indices) = unit_cube_mesh_buffers();
    let url = "/test/gapped.glb";
    let digest = brush_mesh_digest(&positions, &indices);
    assert_eq!(stage_brush_mesh_page(url, "", 0, 3, &positions, &[]), Err(Puzzle3dMeshUploadFault::Envelope), "a page without a declared identity is refused");
    assert_eq!(stage_brush_mesh_page(url, &digest, 3, 3, &positions, &[]), Err(Puzzle3dMeshUploadFault::Envelope), "a page beyond its own run is refused");
    assert_eq!(stage_brush_mesh_page(url, &digest, 0, PUZZLE3D_MESH_UPLOAD_MAX_PAGES + 1, &positions, &[]), Err(Puzzle3dMeshUploadFault::Envelope), "a run longer than the staging ceiling is refused");
    assert_eq!(stage_brush_mesh_page(url, &digest, 1, 3, &positions, &[]), Err(Puzzle3dMeshUploadFault::Gap), "a run that does not open at page 0 is refused");
    assert!(matches!(stage_brush_mesh_page(url, &digest, 0, 3, &positions, &[]), Ok(Puzzle3dMeshUploadStep::Staged { next_page: 1, page_count: 3 })), "the run opens on page 0");
    assert_eq!(stage_brush_mesh_page(url, &digest, 2, 3, &[], &indices), Err(Puzzle3dMeshUploadFault::Gap), "a skipped page is refused");
    assert_eq!(stage_brush_mesh_page(url, &digest, 1, 4, &[], &indices), Err(Puzzle3dMeshUploadFault::Gap), "a page contradicting the run's own page count is refused");
    assert_eq!(decode_brush_mesh_page_values("not base64!!", PUZZLE3D_MESH_PAGE_VALUES), None, "a payload that is not base64 never reaches the staging area");
    assert_eq!(decode_brush_mesh_page_values(&"A".repeat(PUZZLE3D_MESH_PAGE_BASE64_CHARS + 4), PUZZLE3D_MESH_PAGE_VALUES), None, "a payload longer than one page is refused before decoding");
    let mismatched = "/test/mismatched.glb";
    assert!(matches!(stage_brush_mesh_page(mismatched, &digest, 0, 2, &positions, &[]), Ok(Puzzle3dMeshUploadStep::Staged { next_page: 1, page_count: 2 })));
    let foreign: Vec<u32> = indices.iter().map(|index| index % 4).collect();
    assert_eq!(stage_brush_mesh_page(mismatched, &digest, 1, 2, &[], &foreign), Err(Puzzle3dMeshUploadFault::Digest), "a run that closes on bytes the client never announced is refused");
    assert!(!staged_brush_mesh_uploads().iter().any(|(staged_url, _, _, _)| staged_url == url || staged_url == mismatched), "every refused run releases its staging slot");
}

/// 🪪️ Wave W-M2: a mesh this process already paged is adopted by `(url, digest)` alone — the wire never
/// carries the buffers twice — and a stale digest never adopts foreign bytes.
///
/// 🪢️ Wave B22 widens the second half: the transfer is CONTENT-addressed, so a mesh id this process
/// never paged still adopts when its announced DIGEST is resident under another id. Every
/// `dist/mesh/*.glb` in this repo is the same capsule, and keying the transfer by id alone made the
/// client page byte-identical geometry once per id — 72 commands each. Only the transfer is shared: the
/// collision engine still holds the two ids as two identities.
#[test]
fn an_uploaded_mesh_is_adopted_by_url_and_digest() {
    let (positions, indices) = unit_cube_mesh_buffers();
    let url = "/test/adopt-by-digest.glb";
    let digest = brush_mesh_digest(&positions, &indices);
    let mut session = Puzzle3dPrecomputeSession::new();
    assert_eq!(session.stage_mesh_page(url, &digest, 0, 1, &positions, &indices), Ok(None), "a one-page run installs the mesh the moment it closes");
    assert!(session.has_mesh(url), "the closed run is live collision geometry");
    let mut fresh = Puzzle3dCollision::new();
    assert!(fresh.adopt_shared_mesh(url, Some(&digest)), "the id plus its announced digest installs the shared geometry");
    assert!(fresh.has_mesh(url), "the adopted mesh is live in a session that never saw a page");
    let mut foreign = Puzzle3dCollision::new();
    assert!(!foreign.adopt_shared_mesh(url, Some(&"0".repeat(64))), "a stale digest never adopts foreign bytes");
    assert!(!foreign.has_mesh(url), "a refused adoption installs nothing");
    let mut aliased = Puzzle3dCollision::new();
    let sibling = "/test/adopt-by-digest-sibling.glb";
    assert!(aliased.adopt_shared_mesh(sibling, Some(&digest)), "a second id over resident geometry adopts it instead of paging the bytes again");
    assert!(aliased.has_mesh(sibling), "the aliased id is its own live collision identity");
    assert!(shared_brush_mesh(sibling).is_some(), "the alias is resident for every later session too");
    let (unseen, unseen_indices) = seeded_cube_mesh_buffers(97.0);
    let mut unknown = Puzzle3dCollision::new();
    assert!(!unknown.adopt_shared_mesh("/test/never-uploaded.glb", Some(&brush_mesh_digest(&unseen, &unseen_indices))), "geometry this process never derived has nothing to adopt, by id or by digest");
}

/// 🪢️ Wave B22, the reason the digest index exists: a page run that opens on geometry the process
/// already holds closes on its FIRST page, so a second mesh id costs one command instead of the whole
/// run. Browser-measured before this: 202 `registerBrushMesh` commands for one example switch.
#[test]
fn a_run_over_resident_geometry_closes_on_its_first_page() {
    let (positions, indices) = seeded_cube_mesh_buffers(41.0);
    let digest = brush_mesh_digest(&positions, &indices);
    let mut session = Puzzle3dPrecomputeSession::new();
    assert_eq!(session.stage_mesh_page("/test/b22-first.glb", &digest, 0, 1, &positions, &indices), Ok(None), "the first id pages its one-page run");
    let mut second = Puzzle3dPrecomputeSession::new();
    let half = positions.len() / 2;
    assert_eq!(
        second.stage_mesh_page("/test/b22-second.glb", &digest, 0, 8, &positions[..half], &[]),
        Ok(None),
        "a second id announcing resident geometry closes on page 0 — the other seven pages are never needed"
    );
    assert!(second.has_mesh("/test/b22-second.glb"), "the short-circuited run still installs live collision geometry");
    assert!(!staged_brush_mesh_uploads().iter().any(|(url, _, _, _)| url == "/test/b22-second.glb"), "a run that never opened holds no staging slot");
}

/// 🔁️ Wave B22: a page the run already admitted is a retransmission, not a gap. Dropping the run on a
/// duplicate made one retried page cost the client all 72 of them.
#[test]
fn a_retransmitted_page_is_acknowledged_without_dropping_the_run() {
    let (positions, indices) = seeded_cube_mesh_buffers(53.0);
    let digest = brush_mesh_digest(&positions, &indices);
    let url = "/test/b22-retransmit.glb";
    let half = positions.len() / 2;
    assert!(matches!(stage_brush_mesh_page(url, &digest, 0, 3, &positions[..half], &[]), Ok(Puzzle3dMeshUploadStep::Staged { next_page: 1, page_count: 3 })));
    assert!(matches!(stage_brush_mesh_page(url, &digest, 1, 3, &positions[half..], &[]), Ok(Puzzle3dMeshUploadStep::Staged { next_page: 2, page_count: 3 })));
    assert!(
        matches!(stage_brush_mesh_page(url, &digest, 1, 3, &positions[half..], &[]), Ok(Puzzle3dMeshUploadStep::Staged { next_page: 2, page_count: 3 })),
        "a page already admitted is acknowledged at the cursor the run actually stands on"
    );
    assert!(matches!(stage_brush_mesh_page(url, &digest, 2, 3, &[], &indices), Ok(Puzzle3dMeshUploadStep::Complete(..))), "the run still closes on exactly the announced bytes after the retransmission");
    assert_eq!(stage_brush_mesh_page(url, &digest, 2, 3, &[], &indices), Err(Puzzle3dMeshUploadFault::Gap), "a page for a run that is gone is still a gap");
}

/// 🚚️ Wave W-H: what a guest holds does not outlive the guest. A client whose own bookkeeping survived
/// a restart re-announces identities by id alone, and this instantiation can serve none of them — so the
/// refusal must be a REQUEST for the bytes, not a report. The identity is recorded once however often it
/// is re-announced, survives the session hand-off that carries the brush lane between workers, and
/// retires the instant real geometry installs, so a steady state never carries a standing request.
#[test]
fn an_identity_this_guest_cannot_serve_becomes_a_request_for_the_bytes() {
    // 🎲️ Geometry no other law derives: the store is content-addressed since B22, so the shared cube
    // would be adoptable by digest and the refusal this law is about could never happen.
    let (positions, indices) = seeded_cube_mesh_buffers(11.0);
    let digest = brush_mesh_digest(&positions, &indices);
    let url = "/test/reupload-requested.glb";
    let mut session = Puzzle3dPrecomputeSession::new();
    assert!(!session.adopt_shared_mesh(url, Some(&digest)), "an identity this instantiation never derived cannot be adopted by id alone");
    assert!(session.mesh_reupload_requests().is_empty(), "a refusal is only a request once the arm records it");
    assert!(session.request_mesh_reupload(url), "the FIRST refusal records a request the world body has not published yet");
    assert!(!session.request_mesh_reupload(url), "a re-announcement of a pending id changes nothing, so it may not claim a world-body republication (wave B32: widening the scope for it is a refresh storm)");
    assert_eq!(session.mesh_reupload_requests(), [url.to_string()], "a re-announced dead identity is one standing request, never a growing list");
    let mut moved = Puzzle3dPrecomputeSession::new();
    moved.install_collision_session(session.take_collision_session());
    assert_eq!(moved.mesh_reupload_requests(), [url.to_string()], "the request travels with the brush lane's session hand-off");
    assert!(session.mesh_reupload_requests().is_empty(), "the checked-out session keeps nothing");
    let installs_before = shared_brush_mesh_installs();
    assert_eq!(moved.stage_mesh_page(url, &digest, 0, 1, &positions, &indices), Ok(None), "the client answers the request with the page run");
    assert!(moved.has_mesh(url), "the answered request is live collision geometry");
    assert!(moved.mesh_reupload_requests().is_empty(), "an answered request retires the moment the geometry installs");
    assert!(shared_brush_mesh_installs() > installs_before, "a newly derived identity raises the residency counter the client watches");
}

/// 🚚️ Wave B48 LAW: residency is the authority, so a standing re-upload request retires on EVERY path
/// that answers the announcement — not only on the one path that happens to write geometry.
///
/// 🐛️ The invariant `mesh_reupload_requests`' own field doc asserts ("an entry retires the instant that
/// identity's geometry installs, so the set is empty in every steady state and a refusal can never
/// become a standing request") was implemented by a single `retain` inside `place_collision_mesh`,
/// BEHIND that function's already-resident bail. So the three exits that satisfy an announcement without
/// writing geometry — `adopt_shared_mesh`'s resident fast return, `stage_mesh_page`'s page-0 short
/// circuit, and the bail itself — all left the request standing, the world body published the id in
/// `interactionJson.meshReuploadUrls` forever, and the client re-paged the whole 72-command run on every
/// residency climb. Browser-measured at wasm #58 on the 180-object Nakagin document: 123 of 285 console
/// lines were `registerBrushMesh`, seq 22 → 124 over 306 s, still arriving 8 minutes after the example
/// switch (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B46 §5, wave B48 §1.2).
///
/// 🎯️ What this forbids, stated as the client sees it: a mesh this session can serve is never named in
/// the world body's re-upload lane, however the client announced it and however often.
#[test]
fn a_resident_identity_never_stays_in_the_re_upload_request_set() {
    // 🎲️ A seed no other law derives: the process-wide store outlives one test, and
    // `an_uploaded_mesh_is_adopted_by_url_and_digest` asserts that seed 97 geometry is UNKNOWN to it.
    let (positions, indices) = seeded_cube_mesh_buffers(149.0);
    let digest = brush_mesh_digest(&positions, &indices);
    let mut session = Puzzle3dPrecomputeSession::new();

    let paged = "/test/b48-retire-paged.glb";
    assert!(session.request_mesh_reupload(paged), "a first refusal records the request");
    assert_eq!(session.stage_mesh_page(paged, &digest, 0, 1, &positions, &indices), Ok(None), "the client answers with the page run");
    assert!(session.mesh_reupload_requests().is_empty(), "an installed identity retires its own request");

    // 🪢️ Path 1 — the resident FAST RETURN: this identity is already non-fallback, so `adopt_shared_mesh`
    // answers `true` without installing anything.
    assert!(session.request_mesh_reupload(paged), "a client whose bookkeeping is stale re-announces a resident id");
    assert!(session.adopt_shared_mesh(paged, Some(&digest)), "and this session can serve it");
    assert!(session.mesh_reupload_requests().is_empty(), "so the request it answered must be gone, not left for the world body to republish");

    // 🪢️ Path 2 — `stage_mesh_page`'s page-0 SHORT CIRCUIT over geometry the process already derived.
    let aliased = "/test/b48-retire-aliased.glb";
    assert!(session.request_mesh_reupload(aliased), "a sibling id over the same geometry is refused by id alone and recorded");
    assert_eq!(session.stage_mesh_page(aliased, &digest, 0, 8, &positions, &indices), Ok(None), "its run closes on page 0 by digest");
    assert!(session.has_mesh(aliased), "the short circuit still installs live collision geometry");
    assert!(session.mesh_reupload_requests().is_empty(), "a run the guest closed early still answers its request");

    // 🪢️ Path 3 — `place_collision_mesh`'s ALREADY-RESIDENT bail: real geometry is never overwritten, and
    // the refusal to overwrite is not a reason to keep asking for bytes this session already holds.
    assert!(session.request_mesh_reupload(paged), "the client asks once more");
    session.register_mesh_fallback(paged, &positions, &indices);
    assert!(session.mesh_reupload_requests().is_empty(), "a refused overwrite of resident geometry retires the request it cannot improve on");
}

/// 🔢️ Wave W-H: the residency counter is the client's ONLY evidence that its "already uploaded" map is
/// void, so it may only ever climb inside one instantiation — a counter that could fall for any reason
/// other than a fresh guest would void the map on a healthy one. The request set is bounded by the same
/// mesh ceiling the collision engine admits, so a client re-announcing junk can never grow the world body.
#[test]
fn the_residency_counter_only_climbs_and_the_request_set_is_bounded() {
    let (positions, indices) = unit_cube_mesh_buffers();
    let digest = brush_mesh_digest(&positions, &indices);
    let mut session = Puzzle3dPrecomputeSession::new();
    let mut residency = shared_brush_mesh_installs();
    for identity in 0..3 {
        let url = format!("/test/residency-{identity}.glb");
        assert_eq!(session.stage_mesh_page(&url, &digest, 0, 1, &positions, &indices), Ok(None));
        let observed = shared_brush_mesh_installs();
        assert!(observed > residency, "identity {identity} raised the residency counter");
        residency = observed;
    }
    assert!(!session.adopt_shared_mesh("/test/residency-0.glb", Some(&"0".repeat(64))), "a stale digest is refused even for a resident identity");
    assert_eq!(shared_brush_mesh_installs(), residency, "a refusal derives nothing, so the counter stands still");
    let mut bounded = Puzzle3dPrecomputeSession::new();
    for identity in 0..(FILL_WORKER_MAX_MESHES + 8) {
        let recorded = bounded.request_mesh_reupload(&format!("/test/bounded-{identity:04}.glb"));
        assert_eq!(recorded, identity < FILL_WORKER_MAX_MESHES, "identity {identity} may only claim a republication while the bounded set still has room for it");
    }
    assert_eq!(bounded.mesh_reupload_requests().len(), FILL_WORKER_MAX_MESHES, "the request set stops at the mesh ceiling instead of growing with the client's noise");
    let published: Vec<String> = bounded.mesh_reupload_requests().to_vec();
    let mut sorted = published.clone();
    sorted.sort();
    assert_eq!(published, sorted, "requests publish in a stable order, so an unchanged set hashes to an unchanged world-body lane");
    assert!(!bounded.request_mesh_reupload(&"x".repeat(FILL_WORKER_MAX_URL_BYTES + 1)), "an over-long id records nothing and therefore claims no republication");
    assert_eq!(bounded.mesh_reupload_requests().len(), FILL_WORKER_MAX_MESHES, "an id longer than the engine admits is never recorded");
}

/// 🧾️ Wave W-M2: the paged wire is one contract, written down once. Every constant, fault code, page
/// payload and digest the plugin decodes is the one the renderer's pager encodes.
#[test]
fn the_paged_upload_contract_matches_the_language_neutral_fixture() {
    let fixture = brush_mesh_upload_fixture();
    assert_eq!(fixture_count(&fixture["commandRawBytes"]), crate::retained_command::PUZZLE_COMMAND_RAW_BYTES);
    assert_eq!(fixture_count(&fixture["pageValues"]), PUZZLE3D_MESH_PAGE_VALUES);
    assert_eq!(fixture_count(&fixture["pageBase64Chars"]), PUZZLE3D_MESH_PAGE_BASE64_CHARS);
    assert_eq!(fixture_count(&fixture["uploadSlots"]), PUZZLE3D_MESH_UPLOAD_SLOTS);
    assert_eq!(fixture_count(&fixture["maxPages"]), usize::try_from(PUZZLE3D_MESH_UPLOAD_MAX_PAGES).expect("page ceiling"));
    let declared: Vec<&str> = fixture["faults"].as_array().expect("fault codes").iter().map(|code| code.as_str().expect("fault code")).collect();
    let owned = [Puzzle3dMeshUploadFault::Envelope, Puzzle3dMeshUploadFault::Payload, Puzzle3dMeshUploadFault::Gap, Puzzle3dMeshUploadFault::Capacity, Puzzle3dMeshUploadFault::Digest, Puzzle3dMeshUploadFault::Geometry];
    assert_eq!(declared, owned.iter().map(|fault| fault.code()).collect::<Vec<&str>>(), "every fault the plugin can raise is named in the shared contract");
    let example = &fixture["example"];
    let positions: Vec<f32> = example["positions"].as_array().expect("positions").iter().map(|value| value.as_f64().expect("position") as f32).collect();
    let indices: Vec<u32> = example["indices"].as_array().expect("indices").iter().map(|value| u32::try_from(value.as_u64().expect("index")).expect("index fits")).collect();
    let url = example["url"].as_str().expect("url");
    let digest = example["digest"].as_str().expect("digest");
    assert_eq!(brush_mesh_digest(&positions, &indices), digest, "the Rust digest is the digest the TypeScript pager announced");
    assert_eq!(brush_mesh_pages(&positions, &indices).len(), example["pages"].as_array().expect("pages").len());
    let page = &example["pages"][0];
    let positions_b64 = page["positionsB64"].as_str().expect("positions payload");
    let indices_b64 = page["indicesB64"].as_str().expect("indices payload");
    assert_eq!(brush_mesh_pages(&positions, &indices)[0], (positions_b64.to_string(), indices_b64.to_string()), "the Rust encoding is the payload the fixture pins");
    let staged = admit_brush_mesh_page(url, digest, 0, 1, positions_b64, indices_b64).expect("the fixture page closes its own run");
    assert_eq!(staged, Puzzle3dMeshUploadStep::Complete(positions, indices), "the fixture page reassembles into the fixture's own geometry");
}

/// 🧹️ Wave W-M2: a page run a closed document abandoned costs one staging slot until the session
/// registry's own retirement sweeps it — never unbounded memory, and never a live run.
#[test]
fn an_abandoned_page_run_is_retired_and_a_live_one_survives() {
    let (positions, indices) = unit_cube_mesh_buffers();
    let abandoned = "/test/abandoned.glb";
    let live = "/test/live.glb";
    let digest = brush_mesh_digest(&positions, &indices);
    assert!(matches!(stage_brush_mesh_page(abandoned, &digest, 0, 2, &positions, &[]), Ok(Puzzle3dMeshUploadStep::Staged { .. })));
    retire_abandoned_brush_mesh_uploads();
    assert!(staged_brush_mesh_uploads().iter().any(|(url, _, _, _)| url == abandoned), "one sweep is not yet a whole abandonment cycle");
    assert!(matches!(stage_brush_mesh_page(live, &digest, 0, 2, &positions, &[]), Ok(Puzzle3dMeshUploadStep::Staged { .. })));
    retire_abandoned_brush_mesh_uploads();
    assert!(!staged_brush_mesh_uploads().iter().any(|(url, _, _, _)| url == abandoned), "the run that did not advance across a whole cycle is dropped");
    assert!(staged_brush_mesh_uploads().iter().any(|(url, _, _, _)| url == live), "the run that advanced inside the cycle survives");
    assert!(matches!(stage_brush_mesh_page(live, &digest, 1, 2, &[], &indices), Ok(Puzzle3dMeshUploadStep::Complete(_, _))), "the surviving run still closes");
}
//#endregion 🧩️PagedBrushMeshUploads

//#region 💼️BoundedFillJob
use semio_framework_plugin::reactor::jobs as reactor_jobs;

/// 📏️ The smallest budget the host can grant. The bounded fill owner spends exactly one cursor
/// field or one `drive_fill_envelope` call per `step-job` regardless of it, which is precisely what
/// makes it sliceable; a law that passed only on a fat budget would prove nothing.
const FILL_JOB_LAW_BUDGET: JobBudget = JobBudget { fuel: 1, deadline_ms: 1 };

/// 📏️ Slices one fill run may spend before a law calls it non-terminating. The measured run on the
/// single-host fixture stalls in far fewer; this is an order of magnitude of headroom.
const FILL_JOB_LAW_SLICES: usize = 8192;

/// 📏️ Ticks one law drives through the tick path — more than four times the ≈115 the browser needed
/// to exhaust the guest heap on 2026-09-09.
const FILL_TICK_LAW_TICKS: usize = 512;

const FILL_JOB_LAW_EXPLICIT_KIND: &str = "semio.puzzle3d.fill-law-explicit";

/// 🧪️ A deliberately plain `JobFn` registration — the exact shape `FILL_JOB_KIND` used to have. It
/// is the negative control for `fill_job_kind_is_bounded_and_a_plain_job_fn_is_not`: without it the
/// bounded assertion could pass against a runtime that never rejected anything.
fn explicit_control_job(_context: reactor_jobs::JobCtx, _input: Vec<u8>, _restored: Option<Vec<u8>>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<u8>, semio_framework::Fault>>>> {
    Box::pin(async move { Ok(Vec::new()) })
}

/// 🧩️ W-J law (b). `semio-framework-plugin` is NOT built in `cfg(test)` when this crate's tests run,
/// so `spawn_job` takes exactly the production branch: a kind with no `BoundedJobFactory` becomes
/// `JobBody::ExplicitStateMachineRequired` and every `step-job` answers
/// `job.explicit-state-machine-required`. That is the whole defect this wave fixes — the fill tool's
/// planned count sat at zero in the browser because its kind took that branch on every one of the
/// 120 ms tick loop's spawns.
#[semio_framework_async_macros::async_test]
async fn fill_job_kind_is_bounded_and_a_plain_job_fn_is_not() {
    let _guard = fill_envelope_test_guard();
    initialize();
    let mut admitted = fill_worker_session(61);
    let (job, input) = enqueue_measured_fill_job(&mut admitted).expect("fill job");
    reactor_jobs::start_job(job, FILL_JOB_KIND, &input).await;
    let bounded = reactor_jobs::step_job(job, FILL_JOB_LAW_BUDGET).await;
    assert!(matches!(bounded, JobStep::Running(_)), "the registered fill kind must step as a bounded state machine");

    reactor_jobs::register_job_kind(FILL_JOB_LAW_EXPLICIT_KIND, explicit_control_job as reactor_jobs::JobFn);
    let control = job.checked_add(1).expect("control job identity");
    reactor_jobs::start_job(control, FILL_JOB_LAW_EXPLICIT_KIND, &input).await;
    let JobStep::Failed(refusal) = reactor_jobs::step_job(control, FILL_JOB_LAW_BUDGET).await else {
        panic!("a plain JobFn registration must never step in a production build");
    };
    assert!(String::from_utf8_lossy(&refusal).contains("explicit-state-machine-required"), "the control proves the runtime really does refuse un-bounded kinds");

    reactor_jobs::cancel_job(job).await;
    close_fill_envelope(&mut admitted);
}

/// 🧩️ W-J law (a) + the checkpoint half of the restore contract. `BoundedJobFactory` receives only
/// `(job, input)` — `restore_job`'s `restored` bytes never reach it — so the fill kind's checkpoint
/// MUST equal its own spawn input for a restored actor to rebind the identical envelope. Pinned
/// here rather than assumed.
#[semio_framework_async_macros::async_test]
async fn bounded_fill_job_reaches_done_publishing_its_envelope_token_as_progress_and_checkpoint() {
    let _guard = fill_envelope_test_guard();
    initialize();
    let mut admitted = fill_worker_session(63);
    let (job, input) = enqueue_measured_fill_job(&mut admitted).expect("fill job");
    reactor_jobs::start_job(job, FILL_JOB_KIND, &input).await;
    let mut progress = 0_usize;
    let mut done: Option<Vec<u8>> = None;
    let mut checkpointed = false;
    for _ in 0..FILL_JOB_LAW_SLICES {
        match reactor_jobs::step_job(job, FILL_JOB_LAW_BUDGET).await {
            JobStep::Running(Some(bytes)) => {
                assert_eq!(bytes, input, "published progress is the envelope token the host already holds");
                progress += 1;
            }
            JobStep::Running(None) => {}
            JobStep::Done(bytes) => {
                done = Some(bytes);
                break;
            }
            JobStep::Failed(bytes) => panic!("bounded fill job failed: {}", String::from_utf8_lossy(&bytes)),
        }
        if !checkpointed {
            if let Some(entry) = reactor_jobs::checkpoint_jobs().await.into_iter().find(|entry| entry.job == job) {
                assert_eq!(entry.input, input, "the checkpoint pack carries the spawn input verbatim");
                if entry.checkpoint.is_some() {
                    assert_eq!(entry.checkpoint.as_deref(), Some(input.as_slice()), "the fill checkpoint must equal its spawn input, because a bounded factory only ever sees the input");
                    checkpointed = true;
                }
            }
        }
    }
    assert_eq!(done.as_deref(), Some(input.as_slice()), "a bounded fill run terminates on its own envelope token");
    assert!(progress > 0, "a bounded fill run publishes at least one progress slice");
    assert!(checkpointed, "a live bounded fill run always offers a checkpoint");
    let request = decode_fill_envelope_token(&input).expect("fill identity");
    let published = fill_envelope_registry().lock().expect("registry").observation(&request).expect("the finished envelope still publishes its observation");
    assert!(published.done, "the registry publication the session reads is what marks the plan finished");
    assert!(fill_envelope_registry().lock().expect("registry").slots[usize::from(request.slot)].is_some(), "Complete stays readable until the session closes it");
    close_fill_envelope(&mut admitted);
    assert!(fill_envelope_registry().lock().expect("registry").slots[usize::from(request.slot)].is_none(), "an explicit close vacates the slot the 4096-unit spawn occupied");
}

/// ♻️ Pumps ONE session's own terminal cursor until its envelope slot is empty — the mounted
/// counterpart of `drain_orphaned_fill_envelope`, for a run whose terminal this session already
/// checked out through `poll_fill_job`.
fn drain_fill_envelope(session: &mut Puzzle3dPrecomputeSession, request: &FillJobRequest) {
    for _ in 0..FILL_ENVELOPE_MAX_ITEMS {
        session.poll_fill_job();
        if fill_envelope_registry().lock().expect("registry").slots[usize::from(request.slot)].is_none() {
            return;
        }
    }
    panic!("a terminal fill envelope must reach an empty slot within its own bounded close turns");
}

/// 🧯️ W-J law (c), first half. A job the host never steps — exactly what the React target did to
/// every isolated job — must cost ONE spawn and ONE envelope, no matter how long the 120 ms tick
/// loop runs.
#[test]
fn an_unstepped_fill_job_is_enqueued_once_across_five_hundred_ticks() {
    let _guard = fill_envelope_test_guard();
    let mut session = fill_worker_session(65);
    let mut spawns = 0_usize;
    let mut admitted_bytes = None;
    for _ in 0..FILL_TICK_LAW_TICKS {
        session.poll_fill_job();
        if session.enqueue_fill_job().is_some() {
            spawns += 1;
            admitted_bytes = Some(fill_envelope_registry().lock().expect("registry").aggregate_bytes);
        }
    }
    assert_eq!(spawns, 1, "a live fill job must never be re-enqueued by a later tick");
    assert_eq!(fill_envelope_registry().lock().expect("registry").aggregate_bytes, admitted_bytes.expect("one admission"), "an unstepped fill job reserves no further process bytes per tick");
    close_fill_envelope(&mut session);
}

/// 🧯️ W-J law (c), second half. A FAULTED envelope used to be closed and immediately re-measured by
/// the next tick — an unbounded measure→admit→spawn→fault→close cycle, each turn rebuilding a whole
/// `FillBuilder` preparation, which is what exhausted the guest heap. It must now stop, once, and
/// say so.
#[test]
fn a_faulted_fill_envelope_latches_one_notice_and_never_silently_retries() {
    let _guard = fill_envelope_test_guard();
    let mut session = fill_worker_session(67);
    let (_, input) = enqueue_measured_fill_job(&mut session).expect("fill job");
    let request = decode_fill_envelope_token(&input).expect("fill identity");
    terminalize_fill_envelope(&request, FillEnvelopeTerminalReason::Fault);
    let mut spawns = 0_usize;
    for _ in 0..FILL_TICK_LAW_TICKS {
        session.poll_fill_job();
        if session.enqueue_fill_job().is_some() {
            spawns += 1;
        }
    }
    assert_eq!(spawns, 0, "a faulted fill job must never be silently re-enqueued");
    assert!(session.fill_is_faulted(), "the fault latches until an edit supersedes the plan");
    assert!(session.take_fill_fault_notice(), "the fault surfaces as exactly one user-visible notice");
    assert!(!session.take_fill_fault_notice(), "the notice is taken once, never repeated per tick");
    assert_eq!(fill_envelope_registry().lock().expect("registry").aggregate_bytes, 0, "the faulted envelope returns its whole process byte credit");
}
//#endregion 💼️BoundedFillJob
