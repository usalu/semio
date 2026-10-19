/// 🧭️ Layer, camera, convergence, utility and import laws of the raster editor.

/// 🛡️ Play-grid boot regression (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP): the composite
/// and navigator windows build their `Paint2dScene` through `raster_scene` on every render, and the
/// demo document carries a populated asset pool plus a populated adjustment `params` map. The sync
/// projection used to serialize the whole snapshot through `RasterOwnedMap`'s `ToValue`, whose
/// populated-map guard trapped the wasm guest right after boot.
#[semio_framework_async_macros::async_test]
async fn raster_scene_projects_populated_owned_maps_without_wholesale_serialization() {
    let document = semio_s_artifact_raster_raster_io::testing::raster_image_test_snapshot();
    assert!(!document.assets.is_empty(), "the regression needs a populated asset pool");
    let scene = raster_scene(&document, &semio_s_artifact_raster_raster_editor::config::RasterConfig::default(), "brush", "composite", &[], None);
    let sync_value: Value = semio_framework_pack_json::parse(&scene.document_sync_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("sync json");
    assert!(sync_value.get("assets").is_none(), "sync json must omit assets");
    assert_eq!(sync_value.get("id").and_then(Value::as_str), Some("semio-demo"));
    assert_eq!(sync_value.get("title").and_then(Value::as_str), Some("Semio Raster Demo"));
    let layers = sync_value.get("layers").and_then(Value::as_array).expect("layers");
    assert_eq!(layers.len(), 2);
    assert_eq!(layers[0].get("kind").and_then(Value::as_str), Some("pixel"));
    assert_eq!(layers[0].get("imageKey").and_then(Value::as_str), Some("semio-emblem"));
    let brighten = &layers[1];
    assert_eq!(brighten.get("kind").and_then(Value::as_str), Some("adjustment"));
    assert_eq!(brighten.get("adjustmentKind").and_then(Value::as_str), Some("brightnessContrast"));
    let params = brighten.get("params").expect("adjustment params");
    assert_eq!(params.get("brightness").and_then(Value::as_f64), Some(0.12));
    assert_eq!(params.get("contrast").and_then(Value::as_f64), Some(0.08));
    assert!(matches!(semio_framework_pack_json::parse(&scene.assets_json, semio_framework_pack_json::JsonMemberPolicy::Reject), Ok(Value::Object(_))), "assets json stays a well-formed object");
    semio_s_artifact_raster_raster_model::retire_raster_value_cold(document);
}

#[semio_framework_async_macros::async_test]
async fn semio_example_preserves_adjustment_params() {
    let document = semio_s_artifact_raster_raster_io::testing::raster_image_test_snapshot();
    let RasterLayerNode::Adjustment { params, adjustment_kind, .. } = document.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Adjustment { id, .. } if id == "brighten")).expect("brighten adjustment") else {
        panic!("expected adjustment");
    };
    assert_eq!(adjustment_kind, "brightnessContrast");
    assert!(params.contains_key("brightness"), "fixture brightness must roundtrip");
    assert!(params.contains_key("contrast"), "fixture contrast must roundtrip");
    semio_s_artifact_raster_raster_model::retire_raster_value_cold(document);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: layer hover/selection dispatch
/// through the framework-injected `interactionHover`/`interactionSelect` verbs against the
/// `"layers"` domain now (`semio-framework-plugin`'s own suite covers that generic machinery);
/// this app's contribution is declaring the domain and binding the tree to it.
#[semio_framework_async_macros::async_test]
async fn document_tree_binds_the_layers_interaction_domain() {
    let mut app = semio_app().await;
    let json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS).await;
    assert!(json.contains("\"interactionDomain\":\"layers\""), "layer tree must bind the framework-owned layers domain: {json}");
}

#[semio_framework_async_macros::async_test]
async fn set_composite_viewport_feeds_navigator_scene() {
    let mut app = app().await;
    dispatch(&mut app, RasterCommand::SetCompositeViewport(semio_s_artifact_raster_raster_editor::commands::set_composite_viewport::SetCompositeViewport { width: 640.0, height: 480.0 })).await;
    let text = packed_scene_text(&render(&mut app, semio_s_artifact_raster_raster_editor::modes::edit::windows::navigator::RASTER_PLAY_BODY_NAVIGATOR).await);
    assert!(text.contains("compositeViewportJson"), "{text}");
    assert!(text.contains(r#""width":640.0"#), "{text}");
    assert!(text.contains(r#""height":480.0"#), "{text}");
}

#[semio_framework_async_macros::async_test]
async fn set_camera_mutates_runtime_and_emits_no_operations() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot");
    let result = dispatch(&mut app, RasterCommand::SetCamera(semio_s_artifact_raster_raster_editor::commands::set_camera::SetCamera { camera: semio_s_artifact_raster_raster_model::RasterCamera { x: 4.0, y: 5.0, zoom: 2.0 } })).await;
    assert!(result.mutations.is_empty(), "camera is a view action and emits no operations");
    assert_eq!(app.snapshot().expect("snapshot"), before, "camera never mutates the document");
    let text = packed_scene_text(&render(&mut app, semio_s_artifact_raster_raster_editor::modes::edit::windows::composite::RASTER_PLAY_BODY_COMPOSITE).await);
    assert!(text.contains(r#""zoom":2.0"#), "composite scene camera reflects runtime state: {text}");
    assert!(text.contains(r#""x":4.0"#), "composite scene camera reflects runtime state: {text}");
}

#[semio_framework_async_macros::async_test]
async fn set_camera_zoom_updates_zoom_and_keeps_pan_via_runtime() {
    let mut app = app().await;
    dispatch(&mut app, RasterCommand::SetCamera(semio_s_artifact_raster_raster_editor::commands::set_camera::SetCamera { camera: semio_s_artifact_raster_raster_model::RasterCamera { x: 4.0, y: 5.0, zoom: 1.0 } })).await;
    let result = dispatch(&mut app, RasterCommand::SetCameraZoom(semio_s_artifact_raster_raster_editor::commands::set_camera_zoom::SetCameraZoom { zoom: 3.0 })).await;
    assert!(result.mutations.is_empty(), "camera zoom is a view action and emits no operations");
    let text = packed_scene_text(&render(&mut app, semio_s_artifact_raster_raster_editor::modes::edit::windows::composite::RASTER_PLAY_BODY_COMPOSITE).await);
    assert!(text.contains(r#""zoom":3.0"#), "zoom updated: {text}");
    assert!(text.contains(r#""x":4.0"#), "pan preserved across zoom-only update: {text}");
}

#[semio_framework_async_macros::async_test]
async fn add_layer_action_appends_and_undo_removes() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot").layers.len();
    dispatch(&mut app, RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer { kind: "group".into() })).await;
    let projection = app.snapshot().expect("snapshot");
    assert_eq!(projection.layers.len(), before + 1);
    assert!(matches!(projection.layers.last().unwrap(), RasterLayerNode::Group { .. }));
    history(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("snapshot").layers.len(), before);
}

#[semio_framework_async_macros::async_test]
async fn patch_layer_renames_and_toggles_visibility_round_trip() {
    let mut app = app().await;
    // 🌱️ The mounted store boots on the empty shell, so this test plants the layer it addresses.
    dispatch(&mut app, RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer { kind: "pixel".into() })).await;
    let layer_id = semio_s_artifact_raster_raster_model::layer_node_id(&app.snapshot().expect("snapshot").layers[0]).to_string();
    dispatch(&mut app, RasterCommand::PatchLayer(semio_s_artifact_raster_raster_editor::commands::patch_layer::PatchLayer { layer_id: layer_id.clone(), field: "name".into(), value: "Renamed".into() })).await;
    assert_eq!(layer_name(&app.snapshot().expect("snapshot").layers[0]), "Renamed");
    dispatch(&mut app, RasterCommand::ToggleLayerVisible(semio_s_artifact_raster_raster_editor::commands::toggle_layer_visible::ToggleLayerVisible { layer_id })).await;
    assert!(!layer_visible(&app.snapshot().expect("snapshot").layers[0]));
    history(&mut app, "undo").await;
    assert!(layer_visible(&app.snapshot().expect("snapshot").layers[0]));
}

#[semio_framework_async_macros::async_test]
async fn move_layer_into_group() {
    let mut app = app().await;
    // 🌱️ The mounted store boots on the empty shell: both the group and the pixel it receives are
    // planted here.
    dispatch(&mut app, RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer { kind: "pixel".into() })).await;
    dispatch(&mut app, RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer { kind: "group".into() })).await;
    let (group_id, pixel_id) = {
        let projection = app.snapshot().expect("snapshot");
        let group = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Group { .. })).unwrap();
        let pixel = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Pixel { .. })).unwrap();
        (semio_s_artifact_raster_raster_model::layer_node_id(group).to_string(), semio_s_artifact_raster_raster_model::layer_node_id(pixel).to_string())
    };
    let target_row = group_id.clone();
    dispatch(&mut app, RasterCommand::MoveLayer(semio_s_artifact_raster_raster_editor::commands::move_layer::MoveLayer { layer_id: pixel_id.clone(), target_row_id: target_row, drop_position: "inside".into() })).await;
    let projection = app.snapshot().expect("snapshot");
    let RasterLayerNode::Group { children, .. } = projection.layers.iter().find(|layer| semio_s_artifact_raster_raster_model::layer_node_id(layer) == group_id).unwrap() else {
        panic!("expected group");
    };
    assert_eq!(children.len(), 1);
    assert_eq!(semio_s_artifact_raster_raster_model::layer_node_id(&children[0]), pixel_id);
}

/// 🧪️ The definitional merge proof: A adds a layer while B renames the background layer — disjoint
/// tree edits on one backbone that must both survive on both instances.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_layer_edits_via_backbone() {
    let mut instance_a = app().await;
    let mut instance_b = app().await;
    // Seed both from an identical base projection (a background layer with a fixed id) so B's
    // rename targets the same layer A holds — per-instance `initial_snapshot` mints fresh ids.
    let mut base = semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot();
    base.layers = vec![RasterLayerNode::Pixel {
        id: "bg".into(),
        name: "Background".into(),
        visible: true, locked: false,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: semio_s_artifact_raster_raster_model::RasterTransform::default(),
        mask: None,
        width: Some(512),
        height: Some(512),
        image_key: None,
    }];
    let base_envelope = store::os_store::document_materialize::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster", base, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let base_files = store::os_store::text_format::print_document_pack(&base_envelope).await.expect("print document pack");
    context::retire_raster_envelope(base_envelope);
    semio_framework_plugin::artifact_app_laws::load_document(&mut instance_a.0, &base_files, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_a.1.cx()).await.expect("load a");
    semio_framework_plugin::artifact_app_laws::load_document(&mut instance_b.0, &base_files, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_b.1.cx()).await.expect("load b");
    let background_id = "bg".to_string();
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://raster-convergence", "mem://raster-convergence").await;
    instance_a.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_b)).await.expect("attach b");

    dispatch(&mut instance_a, RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer { kind: "pixel".into() })).await;
    dispatch(&mut instance_b, RasterCommand::PatchLayer(semio_s_artifact_raster_raster_editor::commands::patch_layer::PatchLayer { layer_id: background_id, field: "name".into(), value: "Renamed By B".into() })).await;

    // 🔀️ Publishing an edit only PUTS it on the backbone; a replica folds what the other side put
    // there when it ticks. `commitCheckpoint` is a history verb on the LOCAL store and never pulled
    // anything in, so without these two ticks each instance still read only its own edit — the exact
    // exchange `artifact_app_laws::assert_two_registered_instances_converge` performs for the apps
    // whose genesis document is enough to carry both commands.
    instance_a.0.tick_backbone(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_a.1.cx()).await.expect("a folds b's events");
    instance_b.0.tick_backbone(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_b.1.cx()).await.expect("b folds a's events");

    let projection_a = instance_a.snapshot().expect("projection a");
    let projection_b = instance_b.snapshot().expect("projection b");
    assert_eq!(projection_a.layers.len(), 2, "A keeps its added layer");
    assert_eq!(projection_b.layers.len(), 2, "B converges on A's added layer");
    assert_eq!(layer_name(&projection_a.layers[0]), "Renamed By B", "A converges on B's rename");
    assert_eq!(layer_name(&projection_b.layers[0]), "Renamed By B", "B keeps its rename");

    // 🔀️ A replicated CHECKPOINT keeps both sides converged too — the second half of the framework's
    // own convergence law, run here against the shared seeded base.
    let local = artifact_app_laws::meta("local");
    let admitted = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut instance_a.0, "commitCheckpoint", None, &local, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_a.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("a commits a checkpoint");
    semio_framework_plugin::plugin_app_close_prelude::settle_framework_reserved_admission(&mut instance_a.0, admitted).await.expect("a's checkpoint commit settles");
    artifact_app_laws::settle_registered_typed_operation(&mut instance_a.0, local.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_a.1.cx()).await.expect("a's checkpoint publication settles");
    instance_b.0.tick_backbone(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_b.1.cx()).await.expect("b folds a's checkpoint");
    assert_eq!(instance_a.snapshot().expect("projection a").layers.len(), instance_b.snapshot().expect("projection b").layers.len(), "a replicated checkpoint keeps both instances converged");

    // 🧷️ Paired backbone ends block the fixtures' close ladder until both are released.
    instance_a.detach_backbone().await.expect("a releases its backbone");
    instance_b.detach_backbone().await.expect("b releases its backbone");
}

/// 🔁️ The REGISTERED idempotency law: raster publishes bounded tool proofs, so the registry-less
/// `assert_ingest_idempotent` faults `interactive-job.catalog-authority` while it builds its sender.
#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent() {
    artifact_app_laws::assert_registered_ingest_idempotent::<EditorApp<RasterPlayApp>, usize, _, _>(
        || async { context::raster_app_manifest_for_tests() },
        RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer { kind: "pixel".into() }),
        |app| app.snapshot().unwrap().layers.len(),semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn utility_registry_declares_utilities_scoped_to_the_composite_window() {
    let definition = create_raster_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utility_ids, ["selectMarquee", "paintBrush", "paintEraser", "paintBucket"]);
    // The marquee carries the Selection category; the paint utilities are Tools.
    let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
    assert_eq!(selects, ["selectMarquee"]);
    let composite = definition.window_kinds.iter().find(|window| window.id == semio_s_artifact_raster_raster_editor::modes::edit::windows::composite::RASTER_PLAY_WINDOW_COMPOSITE).expect("composite window");
    assert_eq!(composite.utilities.len(), definition.utilities.len(), "every utility is scoped to the composite window kind");
    // The framework auto-injects the setActiveUtility View action once utilities are declared; no doc operation survives.
    // 🧩️ The framework injects `setActiveUtility` into the APP definition's action surface (the
    // builder's `!self.utilities.is_empty()` arm), not into each window kind's own list.
    assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, ActionKind::View)));
    assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == "setActiveUtility" && !matches!(action.kind, ActionKind::View)));
}

#[semio_framework_async_macros::async_test]
async fn raster_io_declares_image_in_out_and_export_media_covers_all_ports() {
    let projection = empty_raster_document();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let image_out = RasterPlayApp::export_media("image:out", &doc).expect("image:out");
    let MediaPayload::Structured { schema, json } = image_out.payload else { panic!("expected structured payload") };
    assert_eq!(schema, "2d.image");
    assert!(!json.is_empty());
    assert!(RasterPlayApp::export_media("artifact:out", &doc).is_ok());
    assert!(matches!(RasterPlayApp::export_media("unknown:out", &doc), Err(MediaError::NotImplemented)));
}

#[semio_framework_async_macros::async_test]
async fn raster_import_media_appends_layer_from_incoming_image() {
    let mut app = app().await;
    let boot = app.snapshot().expect("snapshot");
    let before = boot.layers.len();
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(boot);
    // 🖼️ A REAL PNG — the crate's own committed emblem. The former `"aGVsbG8="` ("hello") decoded to
    // five bytes with no PNG signature, so `mint_raster_asset_child` fell through to its content-less
    // fallback and the law proved nothing about the pixels an import is for.
    let media = Media {
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        payload: MediaPayload::Structured { schema: "2d.image".into(), json: base64_codec::base64_standard_encode(semio_s_artifact_raster_raster_examples::art_raster_demo::emblem_image_asset().data) },
    };
    // 🔌️ A MOUNTED app's `InvocationResult.mutations` is always EMPTY (the host settles the typed
    // publication), so the import is read off the DOCUMENT, exactly as sequence/animate read theirs.
    app.0.import_media("image:in", media, &artifact_app_laws::meta("local"), app.1.cx()).await.expect("import image:in");
    // 🧹️ `ArtifactApp::snapshot` hands back an OWNED `RasterSnapshot`, and after the import that
    // snapshot owns a POPULATED asset pool: reading `.layers.len()` off the temporary dropped a
    // `RasterOwnedMap` that still held its page backing, which is exactly what its fail-closed `Drop`
    // refuses ("Raster owned map reached Drop before every entry and page backing was explicitly
    // retired", `🗑️generated/raster/test-26.txt`). Every observation is taken first, then the
    // snapshot is retired through the artifact's own helper.
    let imported = app.snapshot().expect("snapshot");
    let layers = imported.layers.len();
    let image_key = match imported.layers.last().expect("imported layer") {
        RasterLayerNode::Pixel { image_key, .. } => image_key.clone().expect("the imported pixel layer names its asset"),
        other => panic!("image:in must import a pixel layer, got {other:?}"),
    };
    // 🖼️ The imported asset resolves to real pixels through the same composer the composite reads —
    // an import that lands a content-less handle renders an empty layer.
    let pixels = semio_s_artifact_raster_raster_io::raster_asset(&imported.assets, &image_key).is_some_and(|asset| asset.mime == "image/png" && !asset.data.is_empty());
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(imported);
    assert_eq!(layers, before + 1, "image:in must append one pixel layer");
    assert!(pixels, "the imported asset resolves to real pixels");
}
