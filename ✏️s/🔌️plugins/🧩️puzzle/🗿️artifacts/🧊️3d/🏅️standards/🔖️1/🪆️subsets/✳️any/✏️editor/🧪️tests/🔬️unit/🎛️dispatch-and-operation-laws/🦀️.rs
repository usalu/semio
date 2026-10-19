/// 🎛️ Dispatch and operation laws: gumball gestures, the reserved refresh sections, the remaining cursorized routes, the operations on the concrete forest and the command envelope round trip.

/// 🧲️ Both hosts paint a gumball drag locally and dispatch ONE pose delta on release, so there is no gesture
/// bracket left to declare: `transformBegin`/`transformEnd` are neither retained tools, nor manifest actions,
/// nor publication contracts.
#[test]
fn gumball_gestures_declare_no_bracket_verbs() {
    let manifest = create_puzzle3d_app();
    let contracts = <Puzzle3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    for action in ["transformBegin", "transformEnd"] {
        assert!(!PUZZLE3D_RETAINED_TOOL_IDS.contains(&action), "{action} must not be a retained tool id");
        assert!(action_declarations(&manifest, action).is_empty(), "{action} must not be declared");
        assert!(!contracts.iter().any(|contract| contract.tool_id == action), "{action} must carry no publication contract");
        assert!(Puzzle3dCommand::from_action(action, None, None).is_none(), "{action} must not decode as a command");
    }
}

/// 🌉️ ticket 26/09/02/PUZZLE-3D-END-TO-END: exercises the wiring this ticket adds — the
/// classification flip to `Migrated`, `PUZZLE3D_RETAINED_TOOL_IDS` registration, the
/// `ArtifactToolPublicationContract` for the Artifact+Config lanes, and
/// `Puzzle3dArtifactStorePreparationFactory` — by dispatching `setActiveExample` through the
/// real `InteractiveJob`/tool-job path (`Puzzle3dRetainedCommandJobFactory` ->
/// `RetainedPuzzleCommandJob` -> `ArtifactToolCompletion` -> the shared publication loop's
/// `self.store.begin_apply_batch(..., self.artifact_one_item_factory.as_ref(), None)`), driving the
/// resulting typed operation to completion via repeated `maintenance_step` turns exactly as a
/// real host does every actor tick, then asserting the document was actually swapped. Uses
/// the registry-backed, instance-bound `app()`: this plugin declares
/// `bounded_first_step_tool_proofs!`, so the bare registry-less `artifact_app_laws::new_app` faults closed
/// with `interactive-job.catalog-authority` before any dispatch is even attempted.
#[semio_framework_async_macros::async_test]
async fn set_active_example_dispatches_through_the_tool_job_path_and_swaps_the_document() {
    use semio_framework_plugin::PluginApp;
    let mut app = app().await;
    let before_first_id = first_object_id(&app);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("dispatch setActiveExample through the migrated tool-job path");
    let mut ticks = 0usize;
    while ticks < 5_000 && (object_count(&app) == 0 || first_object_id(&app) == before_first_id) {
        app.measure_maintenance_step(1_048_576, 1_048_576).expect("maintenance step drives the pending typed operation forward");
        ticks += 1;
    }
    assert!(object_count(&app) > 0, "nakagin document did not land after {ticks} maintenance turns");
    assert_ne!(first_object_id(&app), before_first_id, "setActiveExample did not actually swap the document's objects through the tool-job path");
}

/// 📏️ Sizes this artifact's three reserved refresh sections against the retained section carrier that
/// now publishes them (`semio_framework_plugin::section_component_tree`): the engagements, window
/// measure and tool measure maps must all admit into the bounded chunk tree and round-trip
/// byte-exactly, with the fill tool's count slider present in the tool payload.
#[semio_framework_async_macros::async_test]
async fn reserved_refresh_section_payloads_admit_into_the_retained_section_carrier() {
    use semio_framework_plugin::PluginApp;
    let mut app = app().await;
    let view = semio_framework_plugin::ViewModel { window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: main::WINDOW_KIND_ID.to_string(), window_kind_id: main::WINDOW_KIND_ID.to_string() }], ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let payloads = [
        (semio_framework_plugin::UiRefreshSection::Engagements, serde_json::to_string(&app.window_engagements(&view).await).expect("engagements serialize")),
        (semio_framework_plugin::UiRefreshSection::Measures, serde_json::to_string(&app.window_measures(&view).await).expect("measures serialize")),
        (semio_framework_plugin::UiRefreshSection::Tools, serde_json::to_string(&app.tool_measures(&view).await).expect("tool measures serialize")),
    ];
    assert!(payloads[2].1.contains("puzzle3d-fill-count"), "the fill tool must contribute its count slider to the tools section");
    assert!(payloads[1].1.contains(main::WINDOW_KIND_ID), "the main window instance must key its own entry in the measures section");
    for (section, payload) in payloads {
        let tree = semio_framework_plugin::section_component_tree(section, &payload).expect("reserved section payload admits into the bounded carrier");
        let projected: serde_json::Value = serde_json::from_str(&semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("carrier projects")).expect("carrier projection is json");
        assert_eq!(projected["key"], section.body_key());
        assert_eq!(semio_framework_plugin::artifact_app_laws::fixture_carrier_text(&projected), payload, "{} carrier must round-trip byte-exactly", section.key());
    }
}

fn add_brush_object_is_cursorized(source: &str) -> bool {
    source.contains(r#""addBrushObject" => owned!(Puzzle3dAddBrushObjectWork, |request, scope| Puzzle3dAddBrushObjectWork::from_original_construction(scope))"#)
        && source.contains("Puzzle3dAddBrushObjectStage::Decode")
        && source.contains("Puzzle3dAddBrushObjectStage::Kind")
        && source.contains("Puzzle3dAddBrushObjectStage::Representation")
        && source.contains("Puzzle3dAddBrushObjectStage::Vortices")
        && source.contains("Puzzle3dAddBrushObjectStage::ExistingAttractions")
        && source.contains("Puzzle3dAddBrushObjectStage::PublishObject")
        && source.contains("Puzzle3dAddBrushObjectStage::PublishAttraction")
        && !source.contains(r#""addBrushObject" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn add_brush_object_hostile_static_law_rejects_engine_run_to_completion() {
    let source: &str = &editor_source();
    assert!(add_brush_object_is_cursorized(source));
    let direct = source
        .replace(r#""addBrushObject" => owned!(Puzzle3dAddBrushObjectWork, |request, scope| Puzzle3dAddBrushObjectWork::from_original_construction(scope))"#, r#""addBrushObject" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!add_brush_object_is_cursorized(&direct));
    for marker in [
        "Puzzle3dAddBrushObjectStage::Decode",
        "Puzzle3dAddBrushObjectStage::Kind",
        "Puzzle3dAddBrushObjectStage::Representation",
        "Puzzle3dAddBrushObjectStage::Vortices",
        "Puzzle3dAddBrushObjectStage::ExistingAttractions",
        "Puzzle3dAddBrushObjectStage::PublishObject",
        "Puzzle3dAddBrushObjectStage::PublishAttraction",
    ] {
        assert!(!add_brush_object_is_cursorized(&source.replace(marker, "cursor-removed")), "missing brush cursor was falsely accepted: {marker}");
    }
}

fn add_object_kind_is_cursorized(source: &str) -> bool {
    source.contains(r#""addObjectKind" => owned!(Puzzle3dAddObjectKindWork, |request, scope| Puzzle3dAddObjectKindWork::from_original_construction(scope))"#)
        && source.contains("Puzzle3dAddObjectKindStage::Decode")
        && source.contains("Puzzle3dAddObjectKindStage::Kind")
        && source.contains("Puzzle3dAddObjectKindStage::Representation")
        && source.contains("Puzzle3dAddObjectKindStage::Vortex")
        && source.contains("Puzzle3dAddObjectKindStage::Publish")
        && source.contains("PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT")
        && !source.contains(r#""addObjectKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn add_object_kind_hostile_static_law_rejects_whole_catalog_conversion() {
    let source: &str = &editor_source();
    assert!(add_object_kind_is_cursorized(source));
    let direct =
        source.replace(r#""addObjectKind" => owned!(Puzzle3dAddObjectKindWork, |request, scope| Puzzle3dAddObjectKindWork::from_original_construction(scope))"#, r#""addObjectKind" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!add_object_kind_is_cursorized(&direct));
    for marker in ["Puzzle3dAddObjectKindStage::Kind", "Puzzle3dAddObjectKindStage::Representation", "Puzzle3dAddObjectKindStage::Vortex", "Puzzle3dAddObjectKindStage::Publish", "PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT"] {
        assert!(!add_object_kind_is_cursorized(&source.replace(marker, "cursor-removed")), "missing add-object-kind cursor was falsely accepted: {marker}");
    }
}

fn exact_window_routes_capture_instance_owners(source: &str) -> bool {
    source.contains("struct Puzzle3dWindowCommandWork")
        && source.contains("ArtifactCommandWorkStep::CompleteWithEphemeral")
        && source.contains("config_from_snapshot(window_config_snapshot)")
        && source.contains("transient_from_snapshot(window_transient_snapshot)")
        && source.contains("=> bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance))")
}

#[test]
fn exact_window_routes_reject_missing_owner_capture() {
    let source: &str = &editor_source();
    assert!(exact_window_routes_capture_instance_owners(source));
    for marker in ["ArtifactCommandWorkStep::CompleteWithEphemeral", "config_from_snapshot(window_config_snapshot)", "transient_from_snapshot(window_transient_snapshot)"] {
        assert!(!exact_window_routes_capture_instance_owners(&source.replace(marker, "owner-capture-removed")));
    }
}

fn window_config_publish_does_not_silently_drop(source: &str) -> bool {
    source.contains("addressed_config_for(&wid, window_after)")
        && source.contains("addressed_transient_for(&wid, transient_after)")
        && !source.contains("addressed_config(view, window_after).ok()).into_iter()")
}

#[test]
fn window_config_publish_hostile_static_law_rejects_silent_ok_into_iter_drop() {
    let source: &str = &editor_source();
    assert!(window_config_publish_does_not_silently_drop(source));
    let dropped = source.replace(
        "vec![window_ownership::addressed_config_for(&wid, window_after)]",
        "view_state.and_then(|view| window_ownership::addressed_config(view, window_after).ok()).into_iter().collect()",
    );
    assert!(!window_config_publish_does_not_silently_drop(&dropped), "a silent .ok().into_iter() drop must fail the law");
}

fn kind_weight_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""setObjectKindWeight" | "setVortexKindWeight" => owned!(Puzzle3dKindWeightWork, |request, scope| Puzzle3dKindWeightWork::from_original_construction(tool_id, scope))"#)
        && source.contains("Puzzle3dKindWeightStage::Catalog")
        && source.contains("Puzzle3dKindWeightStage::Validate")
        && source.contains("Puzzle3dKindWeightStage::SumOthers")
        && source.contains("Puzzle3dKindWeightStage::Build")
        && source.contains("Puzzle3dConfigMutation::SetObjectKindWeights")
        && source.contains("Puzzle3dConfigMutation::SetVortexKindWeights")
        && !source.contains(r#""setObjectKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn kind_weight_hostile_static_law_rejects_whole_normalizer_and_missing_cursors() {
    let source: &str = &editor_source();
    assert!(kind_weight_route_is_cursorized(source));
    let direct = source.replace(
        r#""setObjectKindWeight" | "setVortexKindWeight" => owned!(Puzzle3dKindWeightWork, |request, scope| Puzzle3dKindWeightWork::from_original_construction(tool_id, scope))"#,
        r#""setObjectKindWeight" | "setVortexKindWeight" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!kind_weight_route_is_cursorized(&direct));
    assert!(!source.contains("puzzle3d_normalize_kind_weight_group(self.weights"));
}

#[test]
fn app_config_serialization_excludes_operation_and_window_state() {
    let config = Puzzle3dConfig::default();
    let spr = to_json_string(&config);
    let oracle: serde_json::Value = serde_json::from_str(&spr).expect("third-party JSON oracle accepts Puzzle 3D config");
    assert_eq!(oracle.as_object().map(serde_json::Map::len), Some(5));
    assert!(spr.contains("activeExampleId"));
    for forbidden in ["fillCheckpoint", "fillApplyGeneration", "windowOptions", "camera", "suggestionMenu", "engagementInput"] {
        assert!(!spr.contains(forbidden));
    }
}

//#region 🔖️Operations
#[semio_framework_async_macros::async_test]
async fn renders_world_scene() {
    let mut app = app().await;
    assert!(render_composite(&mut app).await.to_string().contains("world-3d"));
}

#[semio_framework_async_macros::async_test]
async fn initial_snapshot_is_the_concrete_forest_fixture() {
    let app = app().await;
    assert_eq!(projection_of(&app).get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_SCENE_SNAPSHOT_SCHEMA));
    assert!(object_count(&app) > 0, "the concrete-forest default scene_snapshot ships with objects");
}

/// 📦️ `Puzzle3dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
/// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
/// `serde_json::Value` bridge impls), reusing the default concrete-forest scene_snapshot.
#[semio_framework_async_macros::async_test]
async fn puzzle3d_play_projection_pack_round_trips() {
    let app = app().await;
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&app.snapshot().expect("projection"));
}

/// 🧬️ Wave B9 lane 2: the TYPED half of the snapshot is the authority
/// `ArtifactApp::interaction_topology` reads, and `protocol::validate_state` prunes every selected id
/// the resulting topology does not contain — so a snapshot whose typed half decoded to the DEFAULT
/// (empty) document answers every pick with an empty selection while its `value()` projection still
/// renders the whole world. That is exactly the boundary wave B6 proved in the browser. `PartialEq`
/// compares the typed half alone, so [`assert_dsl_pack_equivalence`] above reads two empty documents
/// as equal and cannot state this; the object count on BOTH halves can.
#[semio_framework_async_macros::async_test]
async fn play_snapshot_typed_authority_survives_every_store_round_trip() {
    use store::pack::snapshot::ArtifactDsl;
    use store::pack::snapshot::ArtifactPack;
    let app = app().await;
    let snapshot = app.snapshot().expect("projection");
    let objects = snapshot.value().get("objects").and_then(|value| value.as_array()).map(|items| items.len()).unwrap_or(0);
    assert!(objects > 0, "the boot scene_snapshot ships objects");
    assert_eq!(snapshot.typed().objects.len(), objects, "the live snapshot's typed authority must carry the projection's objects");
    let via_pack = Puzzle3dPlaySnapshot::decode_pack(&snapshot.encode_pack()).expect("pack decode");
    assert_eq!(via_pack.typed().objects.len(), objects, "a store pack round trip must not empty the typed authority");
    let via_dsl = Puzzle3dPlaySnapshot::parse_dsl(&snapshot.print_dsl()).expect("dsl parse");
    assert_eq!(via_dsl.typed().objects.len(), objects, "a dsl round trip must not empty the typed authority");
    let via_value = Puzzle3dPlaySnapshot::new(semio_framework_value::FromValue::from_value(snapshot.value().clone()).expect("typed fixture admits"));
    assert_eq!(via_value.typed().objects.len(), objects, "rebuilding from the projection must not empty the typed authority");
}

/// 🕳️ Wave B9 lane 2: an ABSENT scene_snapshot-meta member must project as absent, never as `Null`. The
/// persisted twin types `meta.kindCompatibility` as a real array and refuses a `Null` for it, and
/// `Puzzle3dPlaySnapshot`'s decode is all-or-nothing — so one `null` member replaced the ENTIRE typed
/// authority with an empty document while the projection kept every object. `PartialEq` compares the
/// typed half alone, so no round-trip law above can see it; the object count on BOTH halves can.
#[test]
fn an_absent_fixture_meta_member_never_empties_the_typed_authority() {
    let mut seeded = empty_scene_snapshot();
    seeded.objects.clone_from(&CONCRETE_FOREST_EXAMPLE_SNAPSHOT.objects);
    assert!(seeded.meta.kind_catalogs.is_none() && seeded.meta.kind_compatibility.is_none(), "this law is about the meta-less scene_snapshot shape");
    let projection: serde_json::Value = (&semio_framework_value::ToValue::to_value(&seeded)).into();
    let meta = projection.get("meta").expect("the projection carries a meta object");
    assert!(!meta.get("kindCatalogs").is_some_and(serde_json::Value::is_null), "an absent kind catalog must be absent, not null: {meta}");
    assert!(!meta.get("kindCompatibility").is_some_and(serde_json::Value::is_null), "an absent compatibility table must be absent, not null: {meta}");
    let snapshot = Puzzle3dPlaySnapshot::new(semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&(projection))).expect("typed fixture admits"));
    assert_eq!(snapshot.typed().objects.len(), seeded.objects.len(), "both halves of one snapshot must describe the same document");
}

/// 🔭️ Wave B9 lane 2: the pickable universe `protocol::validate_state` prunes against must name
/// every id the world lane paints. Measured against a document whose TYPED authority is empty while
/// its projection is whole — the exact divergence a refused meta decode produces, and the reason the
/// browser's first pick landed on the leftover and then vanished from the Inspection panel.
#[test]
fn interaction_topology_names_every_id_the_world_lane_paints() {
    let mut divergent = empty_scene_snapshot();
    divergent.objects.clone_from(&CONCRETE_FOREST_EXAMPLE_SNAPSHOT.objects);
    divergent.meta.kind_compatibility = Some(semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::object([("target".to_string(), semio_framework_value::DslValue::String("b-l".to_string()))])]));
    let projection: serde_json::Value = (&semio_framework_value::ToValue::to_value(&divergent)).into();
    let refusal=<semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(&projection));
    assert!(refusal.is_err(),"invalid catalogs must refuse rather than discard objects");
    divergent.meta.kind_compatibility=None;
    let snapshot=Puzzle3dPlaySnapshot::new(puzzle3d_snapshot_from_host_snapshot(&divergent).expect("valid host admits"));
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = Puzzle3dConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = Puzzle3dPlayApp::interaction_topology(&doc, &cfg).expect("valid retained interaction scene_snapshot");
    let domain = topology.domains.get(PUZZLE3D_INTERACTION_DOMAIN).expect("the vortex domain is declared");
    let ids: Vec<&str> = domain.ordered.iter().map(|node| node.id.as_str()).collect();
    for object in &divergent.objects {
        assert!(ids.contains(&object.id.as_str()), "a painted object must be pickable: {} missing from {ids:?}", object.id);
        for vortex in &object.vortices {
            let full = puzzle3d_vortex_full_id(&object.id, &vortex.id);
            assert!(ids.iter().any(|id| *id == full.as_str()), "a painted vortex marker must be pickable: {full} missing");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn open_add_object_dialog_emits_the_open_dialog_effect_with_no_document_change() {
    let mut app = app().await;
    let before = object_count(&app);
    let result = dispatch(&mut app, "openAddObjectDialog", None, None).await.expect("openAddObjectDialog");
    assert!(
        matches!(result.requested_effects.as_slice(), [Effect::OpenDialog { dialog_id, args, .. }] if dialog_id == "addObject" && args.is_none()),
        "expected a single OpenDialog effect for the addObject dialog, got {:?}",
        result.requested_effects,
    );
    assert_eq!(object_count(&app), before, "opening the dialog does not mutate the document");
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_swaps_the_document_and_undo_restores_it() {
    let mut app = app().await;
    let loaded = object_count(&app);
    assert!(loaded > 0);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0, "empty example clears the objects");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), loaded, "undo restores the concrete-forest objects");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(object_count(&app), 0);
}

/// 🛰️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B7: `undo` is a TWO-half framework-reserved gesture
/// and neither half may be silent. The ADMISSION half must change nothing and say so out loud — one
/// Isolated [`semio_framework_plugin::app::FRAMEWORK_RESERVED_JOB_KIND`] spawn job and
/// `UiDirtyScope::None`, never a success that quietly committed nothing; the HOST half
/// (`complete_reserved_spawned_job` → `commit_framework_history_route`) is the only thing that may
/// move the document. Pins the regression wave B5 reported: a caller that stops at the admission
/// sees `dispatch("undo")` succeed while the store never moves, so every undo law degrades into a
/// no-op that still passes its dispatch.
#[semio_framework_async_macros::async_test]
async fn undo_admits_one_reserved_job_that_alone_restores_the_document() {
    let mut app = app().await;
    let loaded = object_count(&app);
    assert!(loaded > 0);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0, "empty example clears the objects");
    let admitted = dispatch_reserved_unsettled(&mut app, "undo", None, None).await.expect("undo admits");
    let spawned = admitted.requested_effects.iter().filter(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == semio_framework_plugin::app::FRAMEWORK_RESERVED_JOB_KIND)).count();
    assert_eq!(spawned, 1, "the admission half must ask the host for exactly one Isolated reserved job: {:?}", admitted.requested_effects);
    assert!(matches!(admitted.ui_scope, UiDirtyScope::None), "an undo that has not run yet must not claim a dirty scope: {:?}", admitted.ui_scope);
    assert_eq!(object_count(&app), 0, "the admission half must never move the document on its own");
    let committed = settle_reserved(&mut app, admitted).await.expect("the host half commits the history route");
    assert!(matches!(committed.ui_scope, UiDirtyScope::Full), "a committed undo republishes the whole document: {:?}", committed.ui_scope);
    assert_eq!(object_count(&app), loaded, "only the driven reserved job restores the concrete-forest objects");
}

#[semio_framework_async_macros::async_test]
async fn nakagin_example_loads_via_operations() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin");
    let projection = projection_of(&app);
    assert_eq!(projection.get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_SCENE_SNAPSHOT_SCHEMA));
    assert!(projection.get("objects").and_then(|value| value.as_array()).is_some_and(|objects| !objects.is_empty()));
}

/// 🧾️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B21: a mutating verb lands its OBJECT **and** its
/// history row inside the settle of its OWN dispatch — no second command. The browser measured the
/// opposite on wasm #47: an `addObjectKind` click settled in 23 continuations with
/// `frameKinds:["Invocation","Ephemeral"]` and `historyUpserts:0`, and the `create-object` row only
/// arrived inside the NEXT command's settle, which is why `catalogue-add-object-kind`,
/// `delete-selection`, `duplicate-selection` and `volume-brush-add-target-volume` all read
/// `before == after` at their own verdict (`📓️2026-09-12-wave-B19-mutation-lane-regression.md` §3).
///
/// The row rides [`Puzzle3dCompletion::history_patch`] — the `AppFrame::OperationCompleted` the
/// renderer applies through `subscribeOperationCompletions`. A typed operation never calls
/// `record_command`: its edit reaches the command log only through `backfill_command_log`, inside
/// `refresh_cache`, inside `history_patch` — so reading `history_dirty_sequences` BEFORE that
/// backfill answered "nothing dirty" on the very call whose publication had just landed.
#[semio_framework_async_macros::async_test]
async fn a_mutating_verb_lands_its_object_and_its_history_row_inside_its_own_settle() {
    for (action, args) in [("addObjectKind", json!({ "objectKind": "Object", "origin": [3.0, 0.0, 0.0] })), ("duplicateSelection", json!({}))] {
        let mut app = app().await;
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty example");
        dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("seed one object");
        let object_id = first_object_id(&app);
        select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select the seed object");
        let before = object_count(&app);
        dispatch_unsettled(&mut app, action, Some(&args), None).await.unwrap_or_else(|fault| panic!("{action} must admit: {fault:?}"));
        let settled = settle(&mut app).await;
        assert_eq!(object_count(&app), before + 1, "{action} must land its object inside its own settle");
        let rows = settled.completions.iter().filter(|completion| completion.history_patch.as_ref().is_some_and(|patch| !patch.upserts.is_empty())).count();
        assert_eq!(rows, 1, "{action} must hand back exactly one completion carrying its history patch inside its own settle, got {:?}", settled.completions);
    }
}

#[semio_framework_async_macros::async_test]
async fn document_and_inspector_panels_render() {
    let mut app = app().await;
    for body in [artifact::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY, settings_panel::BODY_KEY] {
        assert!(!render_body(&mut app, body).await.to_string().is_empty());
    }
}
//#endregion 🔖️Operations

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`). Deliberately
/// dispatches through a standalone typed `Puzzle3dStore` — NOT through `Puzzle3dPlayApp`/
/// `Puzzle3dPlaySnapshot` (the `🔖️ValueBridge` `serde_json::Value` wrapper this app still uses)
/// — since `Puzzle3dMutation`'s canonical `Mutation<Puzzle3dSnapshot>` impl (not its
/// `Mutation<Value>` bridge impl) is what the CW7 law is about.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use semio_s_artifact_puzzle_3d_host::close_puzzle3d_store;
    use semio_s_artifact_puzzle_3d_host::puzzle3d_store;
    use {semio_s_artifact_puzzle_3d_model::Puzzle3dObject as TypedObject, semio_s_artifact_puzzle_3d_model::PUZZLE_3D_SCHEMA};
    use protocol::ArtifactId;
    use protocol::Edit;
    use protocol::SchemaId;
    use store::os_store::document_materialize::create_document_envelope;
    use store::os_store::store_schema::ArtifactCommand;

    let mut store = puzzle3d_store(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", Puzzle3dSnapshot::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("store");
    let object = TypedObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    store.dispatch(ArtifactCommand::Apply { mutations: vec![semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(object, None)], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle3dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle3dSnapshot, Puzzle3dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())).await;
    close_puzzle3d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
}
//#endregion 🔖️CommandEnvelopeTests
