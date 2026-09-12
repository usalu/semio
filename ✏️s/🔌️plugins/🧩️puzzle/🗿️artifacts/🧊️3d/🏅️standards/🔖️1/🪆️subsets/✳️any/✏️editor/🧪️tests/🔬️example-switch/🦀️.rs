use super::testkit::*;
use super::*;

/// 🏷️ Wave B30: the ACTIVE EXAMPLE must be known from the first render, not only after the user
/// switches. `ArtifactApp::initial_snapshot` seeds a fresh document from `default_fixture()` — the
/// Concrete Forest example — while `Puzzle3dConfig::active_example_id` defaulted to `""`, i.e. "this
/// document came from no example at all". Everything downstream reads that one field: `export_fixture`
/// names its download after it (Concrete Forest downloaded as the generic `puzzle-3d.json`), and the
/// shell's navbar picker has nothing else to agree with. The law pins the two together — the seeded
/// DOCUMENT and the seeded CONFIG must name the same example — on all three ways a config comes into
/// existence (the Rust `Default`, a decode of a record that omits the field, and a live instance),
/// plus the switch that moves it.
#[semio_framework_async_macros::async_test]
async fn a_fresh_session_config_names_the_example_its_document_was_seeded_from() {
    assert_eq!(Puzzle3dConfig::default().active_example_id, PUZZLE3D_EXAMPLE_CONCRETE_FOREST, "the shared config default must name the example `initial_snapshot` seeds");
    assert_eq!(Puzzle3dRuntime::default().active_example_id, PUZZLE3D_EXAMPLE_CONCRETE_FOREST, "the runtime projection default must agree with the config default");
    let omitted = <Puzzle3dConfig as store::ArtifactDsl>::parse_dsl("{}").expect("a config record that omits every field decodes");
    assert_eq!(omitted.active_example_id, PUZZLE3D_EXAMPLE_CONCRETE_FOREST, "a record that omits the field must decode to the seeded example, never to the blank id");

    let mut app = app().await;
    let seeded = object_count(&app);
    assert!(seeded > 0, "a fresh session boots on a real example document");
    let object_ids = |fixture: &Puzzle3dFixture| fixture.objects.iter().map(|object| object.id.clone()).collect::<Vec<_>>();
    assert_eq!(
        object_ids(&puzzle3d_fixture_from_projection(&projection_of(&app))),
        object_ids(&default_fixture()),
        "the document a fresh session boots with must BE the example the config names"
    );
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin switch");
    assert_ne!(object_count(&app), seeded, "the switch must actually replace the document it was measured against");
    eprintln!("[DEBUG] seeded example id={} objects={}", Puzzle3dConfig::default().active_example_id, seeded);
}

/// 🎵️ Wave W-X: a whole-fixture switch must stay cursorized (hostile law) but land as ONE
/// coalesced document-replacement emit — chunked by mutation kind, not one ingress per item.
#[test]
fn set_active_example_chunks_by_kind_and_emits_one_coalesced_gesture() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&CONCRETE_FOREST_EXAMPLE_FIXTURE.clone())).into());
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).expect("setActiveExample command decodes");
    let mut work = Puzzle3dSetActiveExampleWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin example fits the fixed bounded work envelope");
    let mut progress_steps = 0usize;
    let emit = loop {
        assert!(progress_steps <= extent + 8, "setActiveExample work did not reach Complete within its own declared extent");
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => progress_steps += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    let document = snapshot.typed();
    let items = document.attractions.len()
        + document.objects.len()
        + document.target_volumes.len()
        + document.references.len()
        + document.meta.kind_compatibility.len()
        + NAKAGIN_EXAMPLE_FIXTURE.objects.len()
        + NAKAGIN_EXAMPLE_FIXTURE.attractions.len()
        + NAKAGIN_EXAMPLE_FIXTURE.target_volumes.len()
        + NAKAGIN_EXAMPLE_FIXTURE.references.len()
        + Puzzle3dSetActiveExampleWork::compatibility_rows(&NAKAGIN_EXAMPLE_FIXTURE).len()
        + 2;
    assert!(emit.artifact_mutations.len() > 1, "the completed emit still carries one mutation per deleted/created item; observed {}", emit.artifact_mutations.len());
    assert_eq!(emit.artifact_mutations.len(), items, "chunking must not drop or fuse mutation kinds (items plus domain and catalogs)");
    assert_eq!(emit.coalesce_key.as_deref(), Some(PUZZLE3D_SET_ACTIVE_EXAMPLE_COALESCE_KEY));
    assert_eq!(emit.description.as_deref(), Some(PUZZLE3D_SET_ACTIVE_EXAMPLE_DESCRIPTION));
    assert!(emit.window_config_mutations.is_empty(), "a fixture switch must not emit a separate window-resize undo step");
    assert_eq!(emit.ui_scope, puzzle3d_scope(Puzzle3dScopeClass::Chrome));
    assert!(progress_steps < items, "a whole-fixture switch must not take one ingress per document item; observed {progress_steps} steps for {items} items");
    assert!(
        progress_steps <= PUZZLE3D_SET_ACTIVE_EXAMPLE_FIXED_STEPS + items.div_ceil(PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK) + 10,
        "per-stage ceil(len/chunk) plus transitions must stay a bounded handful; observed {progress_steps} for {items} items"
    );
}

/// 🎵️ Wave W-X: the migrated tool-job path publishes that one emit as one history entry.
/// One undo restores the previous fixture, and the play window's world-3d surface republishes
/// Nakagin's instance lane — the same guest encode `refreshUi` of `puzzle3d.play.composite` runs.
#[semio_framework_async_macros::async_test]
async fn set_active_example_lands_as_one_edit_and_republishes_the_world_scene() {
    let mut app = app().await;
    let before = object_count(&app);
    assert!(before > 0, "the fixture boots on Concrete Forest");
    let forest = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    let forest_instances = forest.assembled.instances_json.clone();
    let forest_hash = forest.lane("framework.scene.world3d.instances").map(|lane| lane.declared_hash.clone()).unwrap_or_default();
    assert!(!forest_instances.is_empty(), "Concrete Forest must publish an instance lane: {}", forest.report());

    let result = dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin switch");
    assert_eq!(object_count(&app), NAKAGIN_EXAMPLE_FIXTURE.objects.len(), "the document must swap through the tool-job path");
    assert_ne!(object_count(&app), before);
    assert_eq!(result.ui_scope, puzzle3d_scope(Puzzle3dScopeClass::Chrome));

    let nakagin = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    let nakagin_instances = nakagin.assembled.instances_json.clone();
    let nakagin_hash = nakagin.lane("framework.scene.world3d.instances").map(|lane| lane.declared_hash.clone()).unwrap_or_default();
    assert!(!nakagin_instances.is_empty(), "Nakagin must republish its instance lane on the composite window body: {}", nakagin.report());
    assert_ne!(nakagin_hash, forest_hash, "a whole-document swap must republish a new instance-lane hash, not reuse the previous fixture's carrier");
    assert_ne!(nakagin_instances, forest_instances, "assembled world-3d instances must leave the previous fixture");
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&nakagin_instances).expect("instance lane is JSON");
    assert_eq!(parsed.len(), NAKAGIN_EXAMPLE_FIXTURE.objects.len(), "the republished instance lane must carry every Nakagin object");

    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), before, "one undo restores the previous fixture — the switch is one history entry");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(object_count(&app), NAKAGIN_EXAMPLE_FIXTURE.objects.len());
}


/// 🧱 Wave W-X §6: a Nakagin world-3d surface must stay inside `UI_DOCUMENT_NODES`. Unpacked
/// 512-byte leaves presented 129 nodes (one over the cap) and both viewports kept Concrete Forest.
#[semio_framework_async_macros::async_test]
async fn nakagin_world3d_surface_fits_reconcile_node_cap() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin switch");
    let census = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    assert!(census.nodes <= semio_framework_ui_contract::UI_DOCUMENT_NODES, "Nakagin world-3d surface presented {} nodes over UI_DOCUMENT_NODES: {}", census.nodes, census.report());
    eprintln!("[DEBUG] Nakagin world-3d surface presented {} nodes (cap {}) {}", census.nodes, semio_framework_ui_contract::UI_DOCUMENT_NODES, census.report());
}

/// 🚚️ Wave W-P5 — the switch is only done when what a RENDER HOST assembles is drawable. The lane
/// laws next door measure the split (spine ≤ `UI_FIXED_BYTES`, one carrier per lane, byte/hash
/// manifest); this one measures the reassembly the way `World3dHost` reads it: every declared lane
/// merged back onto the spine, `instancesJson` carrying one record per Nakagin object, and every
/// `meshId` those records name resolving inside the SAME publication's `meshesJson`. A dangling
/// `meshId` is the one shape that survives every byte-level lane law and still paints an empty world
/// — `WorldInstancesLayer` looks the id up in `meshById`, finds nothing, and renders neither a GLB
/// (no `meshRecord.url`) nor a fallback box (no `meshData`). The spine's own content hash for the
/// instances lane is re-derived here from the assembled text, so a manifest that describes anything
/// other than what the host reassembles fails too.
#[semio_framework_async_macros::async_test]
async fn the_nakagin_switch_assembles_every_object_onto_a_mesh_the_same_publication_declares() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin switch");
    let census = world_surface_carrier_census(&mut app, main::BODY_KEY).await;

    let instances: Vec<serde_json::Value> = serde_json::from_str(&census.assembled.instances_json).expect("the assembled instances lane is json");
    assert_eq!(instances.len(), NAKAGIN_EXAMPLE_FIXTURE.objects.len(), "the assembled scene must carry one instance per Nakagin object: {}", census.report());
    assert!(instances.len() >= 180, "the Nakagin catalog is the widest document this editor publishes; observed {} objects", instances.len());

    let meshes: Vec<serde_json::Value> = serde_json::from_str(&census.assembled.meshes_json).expect("the assembled meshes lane is json");
    let declared_meshes: std::collections::HashSet<&str> = meshes.iter().filter_map(|mesh| mesh.get("id").and_then(serde_json::Value::as_str)).collect();
    let dangling: Vec<&str> = instances.iter().filter_map(|instance| instance.get("meshId").and_then(serde_json::Value::as_str)).filter(|id| !declared_meshes.contains(id)).collect();
    assert!(dangling.is_empty(), "every instance must name a mesh the same publication declares; unresolved {dangling:?} against {declared_meshes:?}");
    assert!(meshes.iter().all(|mesh| mesh.get("kind").and_then(serde_json::Value::as_str).is_some() || mesh.get("url").and_then(serde_json::Value::as_str).is_some()), "every declared mesh resolves by kind or by url: {meshes:?}");

    let instances_lane = census.lane(semio_framework_plugin::World3dSceneLane::Instances.body_key()).expect("the instances lane always publishes");
    assert_eq!(instances_lane.declared_hash, semio_framework_plugin::world3d_scene_lane_hash(&census.assembled.instances_json), "the spine manifest must describe the very text the host reassembles");
    assert_eq!(instances_lane.bytes, census.assembled.instances_json.len(), "the carrier text and the assembled lane are the same bytes");
    eprintln!("[DEBUG] Nakagin switch assembled {} instances over {} declared meshes ({} unresolved) from a {}-byte instances lane in {} leaves", instances.len(), declared_meshes.len(), dangling.len(), instances_lane.bytes, instances_lane.leaves);
}

/// 🎵️ Wave W-AA: `VcsArtifactApp::backfill_command_log` labels an unpublished tool-job edit from
/// `Emit.description`, else the first op `print_op()` (`delete-object id=…`). The coalesced
/// Complete emit must carry "Set Active Example" and no window-config resize — one history row,
/// not a Shell `Resize Window` step of the switch. One-undo restore is
/// `set_active_example_lands_as_one_edit_and_republishes_the_world_scene`.
#[test]
fn set_active_example_history_is_one_set_active_example_row() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&CONCRETE_FOREST_EXAMPLE_FIXTURE.clone())).into());
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).expect("setActiveExample command decodes");
    let mut work = Puzzle3dSetActiveExampleWork::default();
    let emit = loop {
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => {}
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert_eq!(emit.description.as_deref(), Some(PUZZLE3D_SET_ACTIVE_EXAMPLE_DESCRIPTION));
    assert!(emit.window_config_mutations.is_empty(), "a fixture switch must not emit a separate window-resize undo step");
    assert_eq!(emit.coalesce_key.as_deref(), Some(PUZZLE3D_SET_ACTIVE_EXAMPLE_COALESCE_KEY));
}

/// 🎯️ Wave W-S2: a document swap must hand the render host a NEW camera-fit revision, and an
/// ordinary edit must not. `WorldAutoFit` (`🌐️World3dHost`) refits once per `${revision}:${meshes}`
/// key: with no `fit` lane at all — the state of this editor until this wave — the camera after a
/// fixture switch stays wherever the previous document left it, so a fixture centred elsewhere is
/// simply off-screen (W-P5 §7); with a revision derived from GEOMETRY instead of identity it would
/// yank the camera on every object move. The law therefore pins both directions, and pins them on
/// the lane the host actually reads (the reassembled scene), not on the helper alone.
#[semio_framework_async_macros::async_test]
async fn a_document_swap_republishes_the_camera_fit_lane_and_an_object_edit_does_not() {
    let mut app = app().await;
    let forest = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    let fit_of = |census: &WorldSceneCarrierCensus| -> serde_json::Value {
        let text = census.assembled.fit_json.clone().unwrap_or_else(|| panic!("the world scene must publish a fit lane: {}", census.report()));
        serde_json::from_str(&text).expect("the fit lane is json")
    };
    let forest_fit = fit_of(&forest);
    assert_eq!(forest_fit["enabled"], serde_json::json!(true), "the fit lane must arm the host's one-shot auto-fit");
    assert!(forest_fit["revision"].as_u64().is_some(), "the fit lane must carry a document revision: {forest_fit}");
    assert!(forest.lane(semio_framework_plugin::World3dSceneLane::Fit.body_key()).is_some(), "the fit payload rides as its own carrier: {}", forest.report());

    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin switch");
    let nakagin = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    let nakagin_fit = fit_of(&nakagin);
    assert_ne!(nakagin_fit["revision"], forest_fit["revision"], "a whole-document swap must refit the camera exactly once");

    let object = first_object_id(&app);
    dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object.as_str()], "dx": 4.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("move one object");
    let edited = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    assert_eq!(fit_of(&edited)["revision"], nakagin_fit["revision"], "an ordinary edit must never move the camera");
    eprintln!("[DEBUG] fit lane revision forest={} nakagin={} after-edit={}", forest_fit["revision"], nakagin_fit["revision"], fit_of(&edited)["revision"]);
}
