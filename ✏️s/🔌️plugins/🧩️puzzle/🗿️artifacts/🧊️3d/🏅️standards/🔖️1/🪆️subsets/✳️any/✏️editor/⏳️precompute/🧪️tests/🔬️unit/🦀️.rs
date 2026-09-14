use super::*;
use crate::standards::v1::subsets::any::schema::precompute_model_tests::context::*;
use crate::standards::v1::subsets::any::schema::{BrushHostRules, BrushKindWeights, CableKindCatalog, FixtureObject, KindCompatEntry, ObjectKind, ObjectKindRepresentation, ObjectKindVortexTemplate, VortexKindCatalog, VortexProps};

fn catalog_host_engine() -> Puzzle3dCollision {
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
    engine.set_scene(&serde_json::to_string(&scene).expect("catalog host scene")).expect("set catalog host scene");
    engine
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

/// 🗺️ A scene sync invalidates the brush derivation PER OBJECT: an identical scene invalidates nothing,
/// a new object enqueues exactly its own targets and nothing else, and only a change to what every
/// candidate is derived from (overlap budget, catalogs, weights) invalidates every candidate.
///
/// 🧾️ This law used to read `assert_ne!(work_pending, before)` for "a changed scene must rebuild the
/// queue" — it pinned the whole-document wipe itself, which is the defect: on the 340-object
/// brush-painted document every sync (and the 120 ms `suggestionsTick` cadence behind an
/// interactive mutation) threw away every resolved candidate and re-walked the whole object × vortex
/// product (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B54).
#[test]
fn a_scene_sync_invalidates_the_brush_derivation_per_object() {
    let mut engine = Puzzle3dCollision::new();
    let json = single_object_scene_json();
    engine.set_scene(&json).expect("first set_scene should succeed");
    let queue_len_before = engine.work_pending_for_test();
    assert!(queue_len_before > 0 || engine.brush_lane_active(), "the first sync arms the brush lane");
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
    assert_eq!(engine.work_pending_for_test(), 0, "a candidate-derivation change invalidates every candidate and clears the queue for a whole-scene rebuild");
}

/// 🪪️ Regression: registering a mesh must invalidate any cached brush candidates computed against a
/// different (e.g. fallback-box) body for the same url, but a no-operation re-registration must not matter
/// once the cache already reflects the current mesh set (the everyday case: every action re-seeds the
/// fallback body, and the app's `sync_precompute_session` already guards that with `has_mesh`).
#[test]
fn register_mesh_invalidates_cached_precompute_state() {
    let mut engine = Puzzle3dCollision::new();
    engine.set_scene(&single_object_scene_json()).expect("set_scene should succeed");
    let positions: Vec<f32> = vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0];
    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
    engine.register_mesh("/test/host.glb".to_string(), &positions, &indices);
    assert!(engine.brush_cache.is_empty(), "mesh registration must invalidate stale brush cache entries");
}

#[test]
fn engine_precompute_step_is_false_with_no_scene() {
    let mut engine = Puzzle3dCollision::new();
    assert!(!engine.precompute_step(10));
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

    session.precompute_step(50);
    session.invalidate_brush_target("host:v0");
    session.refresh_brush_candidates("host:v0");
    let _candidates: BrushCollisionFreeResult = session.brush_candidates("host:v0");
    assert!(session.brush_preview("host:v0", 0).is_none());

    let mut object_weights = std::collections::BTreeMap::new();
    object_weights.insert("Host".to_string(), 1.0);
    session.dispatch(Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights: std::collections::BTreeMap::new() }).expect("update kind weights");

    let missing_payload = BrushPlacePayload { target_vortex_full_id: "missing:v0".to_string(), object_kind_id: "Nonexistent".to_string(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
    assert!(session.dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload: missing_payload }).is_err());
}

#[test]
fn precompute_session_native_wrapper_errors_without_scene() {
    let mut session = Puzzle3dPrecomputeSession::new();
    let payload = BrushPlacePayload { target_vortex_full_id: "a:v0".to_string(), object_kind_id: "b".to_string(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
    assert!(session.dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload }).is_err());
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
    let payload = BrushPlacePayload { target_vortex_full_id: "probe:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
    let rejected: Result<Puzzle3dEngineOutcome, GuardError> = session.dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload });
    assert!(matches!(rejected, Err(GuardError::BrushPlacementRejected)));
    let _: fn(&Fixture, &BrushPlacePayload, &KindCatalogBundle) -> Fixture = apply_brush_placement_to_fixture;
}

/// 🎯️ Relocated from `🧬️mutations/💾️binary/🦀️.rs`'s
/// `dispatch_brush_preview_without_scene_returns_none` (same reason as the test above).
#[test]
fn dispatch_brush_preview_without_scene_returns_none() {
    let mut session = Puzzle3dPrecomputeSession::new();
    let outcome = session.dispatch(Puzzle3dEngineCommand::BrushPreview { vortex_full_id: "host:v0".to_string(), candidate_index: 0 }).expect("brush preview never errors");
    assert_eq!(outcome, Puzzle3dEngineOutcome::BrushPreview(None), "no scene means no cached brush candidates yet");
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
    let mut engine = catalog_host_engine();
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
/// production `CollisionIndexRemoval` path — the same withdrawal that used to be `#[cfg(test)]`-gated.
#[test]
fn brush_broad_phase_withdraws_an_owner_the_scene_dropped() {
    let mut engine = catalog_host_engine();
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
    for identity in 0..(COLLISION_MESH_MAX_MESHES + 8) {
        let recorded = bounded.request_mesh_reupload(&format!("/test/bounded-{identity:04}.glb"));
        assert_eq!(recorded, identity < COLLISION_MESH_MAX_MESHES, "identity {identity} may only claim a republication while the bounded set still has room for it");
    }
    assert_eq!(bounded.mesh_reupload_requests().len(), COLLISION_MESH_MAX_MESHES, "the request set stops at the mesh ceiling instead of growing with the client's noise");
    let published: Vec<String> = bounded.mesh_reupload_requests().to_vec();
    let mut sorted = published.clone();
    sorted.sort();
    assert_eq!(published, sorted, "requests publish in a stable order, so an unchanged set hashes to an unchanged world-body lane");
    assert!(!bounded.request_mesh_reupload(&"x".repeat(COLLISION_MESH_MAX_URL_BYTES + 1)), "an over-long id records nothing and therefore claims no republication");
    assert_eq!(bounded.mesh_reupload_requests().len(), COLLISION_MESH_MAX_MESHES, "an id longer than the engine admits is never recorded");
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
