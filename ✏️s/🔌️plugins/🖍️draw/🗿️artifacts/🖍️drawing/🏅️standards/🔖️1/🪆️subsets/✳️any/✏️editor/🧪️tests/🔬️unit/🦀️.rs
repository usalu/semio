use super::*;
use crate::schema::{default_drawing_document, layer_id, semio_drawing_example_json};
use crate::DrawingLayerNode;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{testkit as fw_testkit, PluginApp, ViewModel, SET_ACTIVE_UTILITY_ACTION_ID};
use testkit::{drawing_app, meta_with_utility, DrawingApp};

fn canvas_scene(tree: semio_framework_plugin::ComponentTree) -> semio_framework_plugin::Canvas2dScene {
    let decoded = match &tree.root.component {
        semio_framework_plugin::Component::Surface(surface) => semio_framework_ui_scene::decode(surface).map_err(|_| "canvas payload"),
        _ => Err("canvas surface"),
    };
    fw_testkit::project_and_retire_fixture_tree(tree).expect("retire canvas tree");
    decoded.expect("typed canvas")
}

fn drawing_envelope_wire() -> Vec<u8> {
    use store::ArtifactPack;

    let mut snapshot = default_drawing_document("drawing-retained-load", None);
    let mut group = crate::schema::create_drawing_group_layer("Nested");
    if let DrawingLayerNode::Group(value) = &mut group {
        value.children.push(crate::schema::create_drawing_path_layer("Path", vec![crate::PathSegment::Move { to: [1.0, 2.0] }, crate::PathSegment::Line { to: [3.0, 4.0] }]));
    }
    let retained_target = match &group {
        DrawingLayerNode::Group(value) => layer_id(&value.children[0]).to_string(),
        _ => unreachable!("retained Drawing fixture group remains exact"),
    };
    snapshot.layers.push(group);
    snapshot.assets.insert("image-a".into(), crate::DrawingImageAsset { mime: "image/png".into(), data: "AA==".into(), width: Some(1), height: Some(1) });
    let snapshot_pack = snapshot.encode_pack();
    let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": DRAWING_DOCUMENT_SCHEMA,
        "id": "drawing-retained-load",
        "vcs": {
            "initialSnapshot": snapshot_hex,
            "edits": [{
                "id": "drawing-retained-edit-final",
                "actor": "drawing-retained-actor",
                "forwards": [DrawingMutation::RenameLayer(crate::mutations::RenameLayer { layer_id: retained_target.clone(), new_name: "Retained Path".into() })],
                "inverse": [],
                "sequenceNumber": 1,
                "startedAt": "2026-08-23T00:00:00.000Z"
            }],
            "changes": [],
            "checkpoints": [],
            "alternatives": []
        },
        "editMessages": [],
        "conflicts": []
    }))
    .expect("schema-first Drawing fixture envelope");
    let envelope = store::create_document_envelope(DRAWING_DOCUMENT_SCHEMA, "drawing-retained-load", snapshot, None);
    let mut retirement = crate::spr::drawing_envelope_decode_owner_bundle().retire_envelope(envelope);
    for _ in 0..100_000 {
        match retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Drawing fixture envelope retirement") {
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return wire;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
            }
            store::SnapshotRetirementStep::Blocked => panic!("unshared Drawing fixture envelope retirement blocked"),
        }
    }
    panic!("Drawing fixture envelope retirement did not reach terminal")
}

fn admit_drawing_envelope(app: &mut DrawingApp, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Drawing live envelope ingress credits");
    for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Drawing envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Drawing envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("Drawing envelope seal"));
    handle
}

fn drive_drawing_load(app: &mut DrawingApp, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..100_000 {
        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one Drawing maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle).expect("Drawing load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("Drawing retained envelope load did not reach terminal")
}

#[semio_framework_async_macros::async_test]
async fn drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed() {
    let mut app = drawing_app().await;
    let base_generation = app.artifact_generation_now();
    let handle = admit_drawing_envelope(&mut app, &drawing_envelope_wire());
    assert_eq!(handle.generation, base_generation);
    assert_eq!(drive_drawing_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1);
    let projection = app.snapshot().expect("Drawing retained mutation publication");
    let renamed = crate::schema::find_drawing_layer(&projection, &crate::schema::create_drawing_id("path", b"Path")).expect("retained Drawing target");
    assert_eq!(crate::schema::layer_base(renamed).name, "Retained Path");
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("first Drawing acknowledgement"));
    assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate Drawing acknowledgement"));
}

#[semio_framework_async_macros::async_test]
async fn drawing_live_envelope_cancel_closes_retained_pages_without_publication() {
    let mut app = drawing_app().await;
    let base_generation = app.artifact_generation_now();
    let wire = drawing_envelope_wire();
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled Drawing ingress credits");
    let first = &wire[..wire.len().min(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
    let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    bytes[..first.len()].copy_from_slice(first);
    let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled Drawing first page");
    app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("cancelled Drawing page admission failed: {fault:?}"));
    app.cancel_artifact_envelope_load(handle).expect("cancel Drawing ingress");
    assert_eq!(drive_drawing_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
    assert_eq!(app.artifact_generation_now(), base_generation);
}

#[semio_framework_async_macros::async_test]
async fn drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle() {
    for turns in [0usize, 1, 2, 8] {
        let mut app = drawing_app().await;
        let base_generation = app.artifact_generation_now();
        let base_id = app.snapshot().expect("Drawing last-valid snapshot").id;
        let handle = admit_drawing_envelope(&mut app, &drawing_envelope_wire());
        for _ in 0..turns {
            app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("bounded Drawing staged maintenance");
        }
        let stale = semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle { operation: handle.operation, generation: semio_framework_job::Generation(handle.generation.0 + 1) };
        assert!(app.advance_artifact_envelope_load(stale).is_err(), "stale staged handle cannot consume the exact operation owner");
        app.cancel_artifact_envelope_load(handle).expect("exact Drawing staged cancellation");
        assert!(matches!(drive_drawing_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled));
        assert_eq!(app.artifact_generation_now(), base_generation);
        assert_eq!(app.snapshot().expect("Drawing last-valid survives staged cancel").id, base_id);
    }

    let mut app = drawing_app().await;
    let handle = admit_drawing_envelope(&mut app, &drawing_envelope_wire());
    assert_eq!(drive_drawing_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    let stale = semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle { operation: handle.operation, generation: semio_framework_job::Generation(handle.generation.0 + 1) };
    assert!(app.acknowledge_artifact_store_replacement(stale).is_err(), "stale ACK cannot retire the exact committed owner");
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("exact staged Drawing ACK"));
    assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate staged Drawing ACK is idempotent"));
}

#[semio_framework_async_macros::async_test]
async fn drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate() {
    for final_edit in [false, true] {
        let mut value: serde_json::Value = serde_json::from_slice(&drawing_envelope_wire()).expect("Drawing retained fixture JSON");
        let edits = value.pointer_mut("/vcs/edits").and_then(serde_json::Value::as_array_mut).expect("Drawing retained edits");
        if final_edit {
            let mut first = edits[0].clone();
            first["id"] = serde_json::Value::String("drawing-retained-edit-first".into());
            edits.insert(0, first);
        }
        edits.last_mut().expect("Drawing final edit")["id"] = serde_json::Value::String("x".repeat(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES + 1));
        let wire = serde_json::to_vec(&value).expect("hostile Drawing edit fixture");
        let mut app = drawing_app().await;
        let generation = app.artifact_generation_now();
        let handle = admit_drawing_envelope(&mut app, &wire);
        assert_eq!(drive_drawing_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(app.artifact_generation_now(), generation);
    }
}

fn first_layer_id(app: &DrawingApp) -> String {
    layer_id(&app.snapshot().expect("materialize projection").layers[0]).to_string()
}

fn last_layer_id(app: &DrawingApp) -> String {
    let projection = app.snapshot().expect("materialize projection");
    layer_id(projection.layers.last().expect("layer")).to_string()
}

#[semio_framework_async_macros::async_test]
async fn renders_canvas_scene_with_segments() {
    let mut app = drawing_app().await;
    let example_json = semio_drawing_example_json();
    let node = app.render(DRAWING_PLAY_BODY_COMPOSITE, Some(example_json.as_str()), &ViewModel::default()).await.expect("render");
    let scene = canvas_scene(node);
    let layers_json = scene.layers_json.as_str();
    assert!(layers_json.contains("segments"));
    let records: Vec<serde_json::Value> = serde_json::from_str(layers_json).unwrap();
    assert!(records.iter().any(|record| record.get("role").and_then(|value| value.as_str()) == Some("meta")));
    assert!(records.iter().any(|record| record.get("id").and_then(|value| value.as_str()) == Some("artboard:frame")), "canvas must show the document artboard frame");
    assert!(
        records.iter().any(|record| { record.get("id").and_then(|value| value.as_str()) == Some("artboard:dimensions") && record.pointer("/text/content").and_then(|value| value.as_str()).is_some_and(|label| label.contains('×')) }),
        "canvas must show document dimension label"
    );
    assert!(layers_json.contains("200 × 200"), "example artboard dimensions must be visible");
}

#[semio_framework_async_macros::async_test]
async fn default_document_exposes_artboard_dimensions_on_canvas() {
    let mut app = drawing_app().await;
    let node = app.render(DRAWING_PLAY_BODY_COMPOSITE, None, &ViewModel::default()).await.expect("render");
    let scene = canvas_scene(node);
    let layers_json = scene.layers_json.as_str();
    assert!(layers_json.contains("1024 × 1024"), "blank documents show default artboard dimensions");
}

#[semio_framework_async_macros::async_test]
async fn layers_panel_lists_default_layer() {
    let mut app = drawing_app().await;
    let node = app.render(DRAWING_PLAY_BODY_LAYERS, None, &ViewModel::default()).await.expect("render");
    let json = fw_testkit::project_and_retire_fixture_tree(node).expect("retire semantic tree");
    assert!(json.contains("drawing-play-layers.add.path"));
    assert!(json.contains("Layer 1"));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_panel_lists_boolean_operations() {
    let mut app = drawing_app().await;
    let node = app.render(DRAWING_PLAY_BODY_CATALOGUE, None, &ViewModel::default()).await.expect("render");
    let json = fw_testkit::project_and_retire_fixture_tree(node).expect("retire semantic tree");
    assert!(json.contains("drawing-play-catalogue.path"));
    assert!(json.contains("Boolean union"));
}

#[semio_framework_async_macros::async_test]
async fn add_layer_action_emits_op_and_appends_path() {
    let mut app = drawing_app().await;
    let before = app.snapshot().unwrap().layers.len();
    let result = app.dispatch_typed(DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).await.expect("add layer");
    assert_eq!(result.mutations.len(), 1);
    let projection = app.snapshot().unwrap();
    assert_eq!(projection.layers.len(), before + 1);
    assert!(projection.layers.iter().any(|layer| matches!(layer, DrawingLayerNode::Shape(shape) if shape.shape_kind == "rect")));
}

#[semio_framework_async_macros::async_test]
async fn patch_layers_opacity_emits_granular_operation() {
    let mut app = drawing_app().await;
    let id = first_layer_id(&app);
    let result = app.dispatch_typed(DrawingCommand::PatchLayers(patch_layers::PatchLayers { layer_ids: vec![id], field: "opacity".into(), value: "0.5".into() }), &fw_testkit::meta("local")).await.expect("patch");
    assert_eq!(result.mutations.len(), 1);
    let projection = app.snapshot().unwrap();
    assert!((crate::schema::layer_base(&projection.layers[0]).opacity - 0.5).abs() < f64::EPSILON);
}

#[semio_framework_async_macros::async_test]
async fn patch_layer_name_emits_op_and_changes_projection() {
    let mut app = drawing_app().await;
    let id = first_layer_id(&app);
    let result = app.dispatch_typed(DrawingCommand::PatchLayer(patch_layer::PatchLayer { layer_id: id, field: "name".into(), value: "Renamed".into() }), &fw_testkit::meta("local")).await.expect("patch");
    assert_eq!(result.mutations.len(), 1);
    assert_eq!(crate::schema::layer_base(&app.snapshot().unwrap().layers[0]).name, "Renamed");
}

#[semio_framework_async_macros::async_test]
async fn host_utility_change_clears_scratch_and_emits_no_history_entry() {
    let mut app = drawing_app().await;
    let shape_meta = meta_with_utility("shapeRect");
    app.dispatch_typed(
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 10.0,
            y: 10.0,
            width: 800.0,
            height: 600.0,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &shape_meta,
    )
    .await
    .expect("down");
    let before = app.snapshot().unwrap();
    let pen_meta = meta_with_utility("pen");
    let pen_view = pen_meta.view_state.as_ref().expect("host view");
    let tree = app.render(DRAWING_PLAY_BODY_COMPOSITE, None, pen_view).await.expect("render after utility change");
    fw_testkit::project_and_retire_fixture_tree(tree).expect("retire render tree");
    assert_eq!(app.snapshot().unwrap(), before, "utility switching does not mutate the document");
    let up = app.dispatch_typed(DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 40.0, y: 40.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }), &pen_meta).await.expect("up");
    assert!(up.mutations.is_empty(), "the in-progress shape draft was cleared on utility switch");
}

#[semio_framework_async_macros::async_test]
async fn combine_boolean_creates_boolean_layer() {
    let mut app = drawing_app().await;
    let first_id = first_layer_id(&app);
    app.dispatch_typed(DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).await.expect("add rect");
    let second_id = last_layer_id(&app);
    let result = app.dispatch_typed(DrawingCommand::CombineBoolean(combine_boolean::CombineBoolean { operation: "union".into(), ids: vec![first_id, second_id] }), &fw_testkit::meta("local")).await.expect("combine");
    assert_eq!(result.mutations.len(), 1);
    assert!(app.snapshot().unwrap().layers.iter().any(|layer| matches!(layer, DrawingLayerNode::Boolean(_))));
}

#[semio_framework_async_macros::async_test]
async fn canvas_point_to_world_matches_host_formula() {
    let camera = crate::DrawingCamera { x: 100.0, y: 50.0, zoom: 2.0 };
    let (world_x, world_y) = canvas_pointer_down::canvas_point_to_world(&camera, 420.0, 310.0, 800.0, 600.0);
    assert!((world_x - 110.0).abs() < 1e-9);
    assert!((world_y - 55.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn shape_rect_drag_commits_one_layer_and_requests_utility_reset() {
    let mut app = drawing_app().await;
    let utility_meta = meta_with_utility("shapeRect");
    app.dispatch_typed(
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 500.0,
            y: 400.0,
            width: 1000.0,
            height: 800.0,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &utility_meta,
    )
    .await
    .expect("down");
    app.dispatch_typed(DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 600.0, y: 500.0, width: 1000.0, height: 800.0 }), &utility_meta).await.expect("move");
    let result = app.dispatch_typed(DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 600.0, y: 500.0, width: 1000.0, height: 800.0, shift: false, ctrl: false, meta: false }), &utility_meta).await.expect("up");
    assert_eq!(result.mutations.len(), 1, "a shape drag commits as one edit adding exactly the layer");
    let projection = app.snapshot().unwrap();
    assert!(projection.layers.iter().any(|layer| matches!(layer, DrawingLayerNode::Shape(shape) if shape.shape_kind == "rect")));
    assert!(
        matches!(
            result.requested_effects.as_slice(),
            [Effect::SetActiveUtility { window_id, utility_id }] if window_id == DRAWING_PLAY_WINDOW_CANVAS && utility_id == "selectDirect"
        ),
        "the canvas returns to select-direct via a host effect, not a document operation"
    );
}

#[semio_framework_async_macros::async_test]
async fn pen_draft_commits_path_layer_on_enter() {
    let mut app = drawing_app().await;
    let utility_meta = meta_with_utility("pen");
    app.dispatch_typed(
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 400.0,
            y: 300.0,
            width: 800.0,
            height: 600.0,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &utility_meta,
    )
    .await
    .expect("p1");
    app.dispatch_typed(
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 500.0,
            y: 300.0,
            width: 800.0,
            height: 600.0,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &utility_meta,
    )
    .await
    .expect("p2");
    let result = app.dispatch_typed(DrawingCommand::CanvasCommitDraft(canvas_commit_draft::CanvasCommitDraft {}), &utility_meta).await.expect("commit");
    assert_eq!(result.mutations.len(), 1, "the draft commits as exactly one AddLayer edit");
    let projection = app.snapshot().unwrap();
    assert!(projection.layers.iter().any(|layer| matches!(layer, DrawingLayerNode::Path(path) if !path.segments.is_empty())));
    assert!(matches!(result.requested_effects.as_slice(), [Effect::SetActiveUtility { utility_id, .. }] if utility_id == "selectDirect"));
}

#[semio_framework_async_macros::async_test]
async fn canvas_escape_cancels_draft_without_committing() {
    let mut app = drawing_app().await;
    let before = app.snapshot().unwrap().layers.len();
    let utility_meta = meta_with_utility("pen");
    app.dispatch_typed(
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 400.0,
            y: 300.0,
            width: 800.0,
            height: 600.0,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &utility_meta,
    )
    .await
    .expect("p1");
    let result = app.dispatch_typed(DrawingCommand::CanvasEscape(canvas_escape::CanvasEscape {}), &utility_meta).await.expect("escape");
    assert!(result.mutations.is_empty());
    assert_eq!(app.snapshot().unwrap().layers.len(), before);
}

#[semio_framework_async_macros::async_test]
async fn marquee_select_covers_contained_layer_only() {
    // 🔖 Built through dispatched commands (`add-layer` + `patch-layer` transform fields), never
    // a whole-document swap — `SetSnapshot` is banned vocabulary now (see
    // `🧬️mutations/🦀️.rs`'s module doc); this exercises the same real semantic
    // `create-layer`/`update-layer-transform` mutations a live editor session would emit.
    let mut app = drawing_app().await;
    let utility_meta = meta_with_utility("selectMarquee");
    let initial_id = layer_id(&app.snapshot().unwrap().layers[0]).to_string();
    app.dispatch_typed(DrawingCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: initial_id }), &fw_testkit::meta("local")).await.expect("clear default layer");

    app.dispatch_typed(DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).await.expect("add rect");
    let rect_a_id = layer_id(app.snapshot().unwrap().layers.last().unwrap()).to_string();
    for (field, value) in [("transformX", "10"), ("transformY", "10"), ("transformScaleX", "0.15625"), ("transformScaleY", "0.208333")] {
        app.dispatch_typed(DrawingCommand::PatchLayer(patch_layer::PatchLayer { layer_id: rect_a_id.clone(), field: field.into(), value: value.into() }), &fw_testkit::meta("local")).await.expect("position rect a");
    }

    app.dispatch_typed(DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:ellipse".into() }), &fw_testkit::meta("local")).await.expect("add ellipse");
    let ellipse_b_id = layer_id(app.snapshot().unwrap().layers.last().unwrap()).to_string();
    for (field, value) in [("transformX", "200"), ("transformY", "200")] {
        app.dispatch_typed(DrawingCommand::PatchLayer(patch_layer::PatchLayer { layer_id: ellipse_b_id.clone(), field: field.into(), value: value.into() }), &fw_testkit::meta("local")).await.expect("position ellipse b");
    }

    app.dispatch_typed(DrawingCommand::SetCamera(set_camera::SetCamera { camera: crate::DrawingCamera { x: 0.0, y: 0.0, zoom: 1.0 } }), &fw_testkit::meta("local")).await.expect("camera");
    app.dispatch_typed(
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 400.0,
            y: 300.0,
            width: 800.0,
            height: 600.0,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &utility_meta,
    )
    .await
    .expect("down");
    app.dispatch_typed(DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 460.0, y: 360.0, width: 800.0, height: 600.0 }), &utility_meta).await.expect("move");
    let result = app.dispatch_typed(DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 460.0, y: 360.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }), &utility_meta).await.expect("up");
    // 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
    // the marquee hit-test requests `interactionSelect` for exactly the contained rect via a
    // `Effect::ReplayShellCommand`, instead of writing a `DrawingConfigMutation::SetSelection`.
    assert!(result.mutations.is_empty(), "a pure marquee-select gesture is not a document operation");
    assert_eq!(result.requested_effects, vec![canvas_pointer_down::interaction_select_effect(&[rect_a_id.clone()], "replace")], "only the contained rect is requested, not the outside ellipse");
}

#[semio_framework_async_macros::async_test]
async fn set_camera_writes_runtime_and_emits_no_operations() {
    let mut app = drawing_app().await;
    let before = app.snapshot().expect("projection");
    let result = app.dispatch_typed(DrawingCommand::SetCamera(set_camera::SetCamera { camera: crate::DrawingCamera { x: 5.0, y: 5.0, zoom: 2.0 } }), &fw_testkit::meta("local")).await.expect("camera");
    assert!(result.mutations.is_empty(), "camera is a view action and emits no operations");
    assert_eq!(app.snapshot().expect("projection"), before, "camera never mutates the document");
    let scene = canvas_scene(app.render(DRAWING_PLAY_BODY_COMPOSITE, None, &ViewModel::default()).await.expect("render"));
    assert_eq!([scene.camera_x, scene.camera_y, scene.zoom], [5.0, 5.0, 2.0]);
}

#[semio_framework_async_macros::async_test]
async fn set_camera_zoom_updates_zoom_and_keeps_pan_via_runtime() {
    let mut app = drawing_app().await;
    app.dispatch_typed(DrawingCommand::SetCamera(set_camera::SetCamera { camera: crate::DrawingCamera { x: 4.0, y: 5.0, zoom: 1.0 } }), &fw_testkit::meta("local")).await.expect("set camera");
    let result = app.dispatch_typed(DrawingCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 3.0 }), &fw_testkit::meta("local")).await.expect("set camera zoom");
    assert!(result.mutations.is_empty(), "camera zoom is a view action and emits no operations");
    let scene = canvas_scene(app.render(DRAWING_PLAY_BODY_COMPOSITE, None, &ViewModel::default()).await.expect("render"));
    assert_eq!([scene.camera_x, scene.camera_y, scene.zoom], [4.0, 5.0, 3.0]);
}

#[semio_framework_async_macros::async_test]
async fn add_layer_undo_round_trip_through_wrapper() {
    let mut app = drawing_app().await;
    let before = app.snapshot().unwrap().layers.len();
    fw_testkit::assert_undo_redo_round_trip(&mut app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "path".into() }), |app| app.snapshot().unwrap().layers.len(), before, before + 1).await;
}

#[semio_framework_async_macros::async_test]
async fn utility_registry_declares_all_canvas_utilities_scoped_to_the_window() {
    let definition = create_drawing_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utility_ids, ["selectMarquee", "selectLasso", "selectDirect", "pen", "shapeRect", "shapeEllipse", "shapeLine", "shapePolygon", "booleanCombine", "trace", "transformMove"],);
    let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
    assert_eq!(selects, ["selectMarquee", "selectLasso", "selectDirect"]);
    let scene = definition.window_kinds.iter().find(|window| window.id == DRAWING_PLAY_WINDOW_CANVAS).expect("canvas window");
    assert_eq!(scene.utilities.len(), definition.utilities.len(), "every utility is scoped to the canvas window kind");
    assert!(scene.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, ActionKind::View)));
    assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == "setActiveUtility" && !matches!(action.kind, ActionKind::View)));
}

#[semio_framework_async_macros::async_test]
async fn strokes_interaction_domain_is_declared_flat_pick_rectangle_lasso_on_the_canvas_window() {
    let definition = create_drawing_app();
    let domain = definition.interactions.iter().find(|interaction| interaction.id == DRAWING_INTERACTION_DOMAIN).expect("strokes interaction domain declared");
    assert!(matches!(domain.hierarchy, HierarchyProvider::Flat));
    assert_eq!(domain.selection.methods, vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso]);
    let canvas_window = definition.window_kinds.iter().find(|window| window.id == DRAWING_PLAY_WINDOW_CANVAS).expect("canvas window");
    assert!(canvas_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == DRAWING_INTERACTION_DOMAIN));
}

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_up_direct_pick_requests_interaction_select() {
    let mut app = drawing_app().await;
    // 🔖 The default document's one layer is an empty-segment path (no bounds to hit-test against
    // — see `default_drawing_document`), so a real shape is added first, mirroring
    // `marquee_select_covers_contained_layer_only`'s own setup.
    let initial_id = first_layer_id(&app);
    app.dispatch_typed(DrawingCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: initial_id }), &fw_testkit::meta("local")).await.expect("clear default layer");
    app.dispatch_typed(DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).await.expect("add rect");
    let rect_id = last_layer_id(&app);
    app.dispatch_typed(DrawingCommand::SetCamera(set_camera::SetCamera { camera: crate::DrawingCamera { x: 0.0, y: 0.0, zoom: 1.0 } }), &fw_testkit::meta("local")).await.expect("camera");
    // 🎯️ Default `shape:rect` geometry is world (0,0)-(128,96); screen (110,110) on a 200x200
    // viewport with the identity camera above maps to world (10,10) — inside the rect.
    let result = app.dispatch_typed(DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 110.0, y: 110.0, width: 200.0, height: 200.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local")).await.expect("pick");
    assert!(result.mutations.is_empty(), "a direct pick is not a document operation");
    assert_eq!(result.requested_effects, vec![canvas_pointer_down::interaction_select_effect(&[rect_id], "replace")]);
}

#[semio_framework_async_macros::async_test]
async fn set_selected_opacity_reads_the_framework_interaction_selection() {
    let mut app = drawing_app().await;
    let id = first_layer_id(&app);
    let targets = serde_json::to_string(&vec![serde_json::json!({ "granularity": DRAWING_INTERACTION_GRANULARITY, "id": id })]).unwrap();
    app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&dsl::json::to_dsl_value(&dsl::json!({ "domainId": DRAWING_INTERACTION_DOMAIN, "targets": targets, "merge": "replace" }))), &fw_testkit::meta("local")).await.expect("select");
    let result = app.dispatch_typed(DrawingCommand::SetSelectedOpacity(set_selected_opacity::SetSelectedOpacity { value: 0.25 }), &fw_testkit::meta("local")).await.expect("opacity");
    assert_eq!(result.mutations.len(), 1);
    assert!((crate::schema::layer_base(&app.snapshot().unwrap().layers[0]).opacity - 0.25).abs() < f64::EPSILON);
}

#[semio_framework_async_macros::async_test]
async fn drawing_labels_resolve_native_by_default() {
    let mut app = drawing_app().await;
    let node = app.render(DRAWING_PLAY_BODY_LAYERS, None, &ViewModel::default()).await.expect("render");
    let json = fw_testkit::project_and_retire_fixture_tree(node).expect("retire semantic tree");
    assert!(json.contains("Add Path"));
    assert!(json.contains("Add Rectangle"));
    assert!(!json.contains("Pfad hinzufügen"));
}

#[semio_framework_async_macros::async_test]
async fn drawing_labels_translate_panels_in_german() {
    let mut app = drawing_app().await;
    let view_state = ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let layers_node = app.render(DRAWING_PLAY_BODY_LAYERS, None, &view_state).await.expect("render");
    let layers_json = fw_testkit::project_and_retire_fixture_tree(layers_node).expect("retire layers tree");
    assert!(layers_json.contains("Pfad hinzufügen"));
    assert!(layers_json.contains("Rechteck hinzufügen"));
    assert!(!layers_json.contains("Add Path"));
    let catalogue_node = app.render(DRAWING_PLAY_BODY_CATALOGUE, None, &view_state).await.expect("render");
    let catalogue_json = fw_testkit::project_and_retire_fixture_tree(catalogue_node).expect("retire catalogue tree");
    assert!(catalogue_json.contains("\"Ellipse\""));
    assert!(catalogue_json.contains("Nachzeichnung"));
}

#[semio_framework_async_macros::async_test]
async fn drawing_io_declares_vector_out_and_export_media_covers_both_ports() {
    let mut app = drawing_app().await;
    app.dispatch_typed(DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).await.expect("add");
    let projection = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let vector = DrawingPlayApp::export_media("vector:out", &doc).expect("vector:out");
    let MediaPayload::Structured { schema, json } = vector.payload else { panic!("expected structured svg payload") };
    assert_eq!(schema, "2d.drawing");
    assert!(json.starts_with("<svg"));
    assert!(DrawingPlayApp::export_media("document:out", &doc).is_ok());
    assert!(matches!(DrawingPlayApp::export_media("unknown:out", &doc), Err(MediaError::NotImplemented)));
}

//#region 🔖️GesturePreview
#[semio_framework_async_macros::async_test]
async fn gesture_preview_is_none_while_idle() {
    let session = DrawingSession::default();
    assert_eq!(session.preview().phase, canvas_pointer_down::DrawingGesturePreviewPhase::Idle, "idle has an empty fixed projection");
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_reflects_live_shape_drag_and_clears_on_commit() {
    let mut session = DrawingSession::default();
    let document = default_drawing_document("empty", None);
    let config = DrawingConfig::default();

    let down = session.step_gesture(canvas_pointer_down::drawing_gesture::Event::PointerDown { utility: "shapeRect".into(), world: [10.0, 10.0], shift: false, ctrl: false, meta: false }, &document, &config);
    assert!(down.artifact_mutations.is_empty(), "pointer-down starts a scratch drag, not a document operation");
    let preview = session.preview();
    let seq_after_down = preview.sequence;
    assert_eq!(preview.context.start, [10.0, 10.0]);
    assert_eq!(preview.context.cursor, [10.0, 10.0]);

    let moved = session.step_gesture(canvas_pointer_down::drawing_gesture::Event::PointerMove { world: [40.0, 30.0], marquee_threshold_world: 4.0 }, &document, &config);
    assert!(moved.artifact_mutations.is_empty(), "mid-drag ticks emit zero operations (scratch-commit pattern)");
    let preview = session.preview();
    assert_eq!(preview.context.cursor, [40.0, 30.0], "preview tracks the live cursor, not the drag start");
    assert!(preview.sequence > seq_after_down, "seq is monotone per tick, for staleness detection on the receiving end");

    let up = session.step_gesture(canvas_pointer_down::drawing_gesture::Event::PointerUp { utility: "shapeRect".into(), world: [40.0, 30.0], shift: false, ctrl: false, meta: false }, &document, &config);
    assert_eq!(up.artifact_mutations.len(), 1, "pointer-up commits the shape as one real DrawingMutation");
    assert_eq!(session.preview().phase, canvas_pointer_down::DrawingGesturePreviewPhase::Idle, "the committed projection is terminal idle");
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_is_a_pure_read_never_mutating_gesture_context() {
    let mut session = DrawingSession::default();
    let document = default_drawing_document("empty", None);
    let config = DrawingConfig::default();
    session.step_gesture(canvas_pointer_down::drawing_gesture::Event::PointerDown { utility: "shapeRect".into(), world: [1.0, 2.0], shift: false, ctrl: false, meta: false }, &document, &config);
    let context_before = session.gesture.context.clone();
    let _ = session.preview();
    let _ = session.preview();
    assert_eq!(session.gesture.context, context_before, "preview must never mutate the live gesture scratch it reads");
}
//#endregion 🔖️GesturePreview

//#region 🔖️WireGuards
/// 🔖️ One `DrawingCommand` value per row, in binary-variant-ordinal order — feeds both the
/// op-text/binary equivalence loop and the "printed line starts with the row's wire keyword"
/// assertion. Permanent wire guard: appending a variant is safe, reordering breaks the format.
fn every_command() -> Vec<DrawingCommand> {
    vec![
        DrawingCommand::SetSnapshot(set_snapshot::SetSnapshot { snapshot: default_drawing_document("cmd-doc", None) }),
        DrawingCommand::CommitDocument(commit_document::CommitDocument { snapshot: default_drawing_document("cmd-doc-2", None) }),
        DrawingCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "{}".into() }),
        DrawingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }),
        DrawingCommand::SetSelectedOpacity(set_selected_opacity::SetSelectedOpacity { value: 0.5 }),
        DrawingCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("Renamed \"layer\"".into()) }),
        DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }),
        DrawingCommand::DropLayerKind(drop_layer_kind::DropLayerKind { kind: "path".into(), target_row_id: "drawing-play-layers".into(), drop_position: "inside".into() }),
        DrawingCommand::MoveLayer(move_layer::MoveLayer { layer_id: "layer-1".into(), target_row_id: "drawing-play-layers".into(), drop_position: "after".into() }),
        DrawingCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: "layer-1".into() }),
        DrawingCommand::DuplicateLayer(duplicate_layer::DuplicateLayer { layer_id: "layer-1".into() }),
        DrawingCommand::ToggleLayerVisible(toggle_layer_visible::ToggleLayerVisible { layer_id: "layer-1".into() }),
        DrawingCommand::CombineBoolean(combine_boolean::CombineBoolean { operation: "union".into(), ids: vec!["a".into(), "b".into()] }),
        DrawingCommand::PatchLayer(patch_layer::PatchLayer { layer_id: "layer-1".into(), field: "opacity".into(), value: "0.4".into() }),
        DrawingCommand::PatchLayers(patch_layers::PatchLayers { layer_ids: vec!["a".into(), "b".into()], field: "blendMode".into(), value: "\"multiply\"".into() }),
        DrawingCommand::SetCamera(set_camera::SetCamera { camera: crate::DrawingCamera { x: 1.0, y: 2.0, zoom: 1.5 } }),
        DrawingCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 2.0 }),
        DrawingCommand::EngagementInput(engagement_input::EngagementInput { value: "typing".into() }),
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 1.0,
            y: 2.0,
            width: 800.0,
            height: 600.0,
            shift: true,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 1.0, y: 2.0, width: 800.0, height: 600.0 }),
        DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 1.0, y: 2.0, width: 800.0, height: 600.0, shift: false, ctrl: true, meta: false }),
        DrawingCommand::CanvasDoubleClick(canvas_double_click::CanvasDoubleClick {}),
        DrawingCommand::CanvasCommitDraft(canvas_commit_draft::CanvasCommitDraft {}),
        DrawingCommand::CanvasEscape(canvas_escape::CanvasEscape {}),
    ]
}

#[semio_framework_async_macros::async_test]
async fn drawing_command_op_text_round_trips_every_variant() {
    for command in every_command() {
        store::os_store::test_support::assert_op_line_round_trip(&command);
    }
    // The `None`-field variant missing from `every_command` (kept distinct from its `Some`
    // counterpart above, matching the pre-migration wire-baseline capture).
    store::os_store::test_support::assert_op_line_round_trip(&DrawingCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None }));
}

#[semio_framework_async_macros::async_test]
async fn drawing_command_op_binary_round_trips_every_variant() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🔖️ Pins the exact pre-migration hex for the two rows whose `Option` fields make `None`/`Some`
/// distinct wire cases — copied verbatim from the `wire-baseline-before.txt` capture taken from
/// the OLD `drawing_protocol` crate before this migration. A byte-for-byte diff, not just a
/// round-trip law, since round-trip alone would happily pass on a changed-but-consistent format.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    use protocol::OpBinary;
    let engagement_submit_some = DrawingCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("Renamed \"layer\"".into()) });
    assert_eq!(engagement_submit_some.encode_op().expect("encode"), hex_bytes("0105010f52656e616d656420226c617965722201000600"));
    let engagement_submit_none = DrawingCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None });
    assert_eq!(engagement_submit_none.encode_op().expect("encode"), hex_bytes("01050000"));
}

fn hex_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("valid hex")).collect()
}

#[semio_framework_async_macros::async_test]
async fn every_command_row_prints_starting_with_its_wire_keyword() {
    use protocol::OpText;
    let expected_keywords = [
        "set-snapshot",
        "commit-document",
        "fixture-json",
        "active-example",
        "selected-opacity",
        "engagement-submit",
        "add-layer",
        "drop-layer-kind",
        "move-layer",
        "delete-layer",
        "duplicate-layer",
        "toggle-layer-visible",
        "combine-boolean",
        "patch-layer",
        "patch-layers",
        "camera",
        "camera-zoom",
        "engagement-input",
        "canvas-pointer-down",
        "canvas-pointer-move",
        "canvas-pointer-up",
        "canvas-double-click",
        "canvas-commit-draft",
        "canvas-escape",
    ];
    for (command, keyword) in every_command().into_iter().zip(expected_keywords) {
        let printed = command.print_op();
        assert!(printed.starts_with(keyword), "expected '{printed}' to start with '{keyword}'");
    }
}
//#endregion 🔖️WireGuards

//#region 🔖️CommandSurface
/// ⚖️ The dispatch law: the two owned factories partition the command enum exactly, every route
/// carries a publication contract, and every proof row has an owner. A regression in any of the
/// four lists below is exactly the class of defect that made twenty of draw's twenty-six commands
/// unreachable from the client.
#[semio_framework_async_macros::async_test]
async fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory as _;

    assert_eq!(DRAWING_GESTURE_TOOL_IDS.len(), 6);
    assert_eq!(DRAWING_BOUNDED_TOOL_IDS.len(), 18);
    let mut routes = DRAWING_GESTURE_TOOL_IDS.iter().chain(DRAWING_BOUNDED_TOOL_IDS).copied().collect::<Vec<_>>();
    routes.sort_unstable();
    let mut declared = every_command().into_iter().map(|command| command.command_id()).collect::<Vec<_>>();
    declared.sort_unstable();
    assert_eq!(routes, declared, "every declared command owns exactly one retained route, and no route names a command that does not exist");
    assert_eq!(routes.windows(2).filter(|pair| pair[0] == pair[1]).count(), 0, "a tool id may be owned by exactly one factory");

    assert_eq!(DrawingGestureOperationJobFactory::PUBLICATION_CONTRACTS.len(), DRAWING_GESTURE_TOOL_IDS.len());
    assert_eq!(DrawingBoundedCommandJobFactory::PUBLICATION_CONTRACTS.len(), DRAWING_BOUNDED_TOOL_IDS.len());
    for contract in DrawingGestureOperationJobFactory::PUBLICATION_CONTRACTS.iter().chain(DrawingBoundedCommandJobFactory::PUBLICATION_CONTRACTS) {
        assert!(routes.contains(&contract.tool_id), "{} publishes without owning a route", contract.tool_id);
        assert!(!contract.lanes.is_empty(), "{} declares no publication lane", contract.tool_id);
        assert!(!contract.lanes.contains(&semio_framework_plugin::ArtifactToolPublicationLane::HostOnly) || contract.lanes.len() == 1, "{} pairs HostOnly with another lane, which the registry rejects", contract.tool_id);
    }

    assert_eq!(<DrawingPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), routes.len());
    assert!(<DrawingPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some(), "the artifact lane needs its one-item preparation authority");
    assert!(<DrawingPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some(), "the config lane needs its one-item preparation authority");

    let definition = create_drawing_app();
    let classified = definition
        .window_kinds
        .iter()
        .flat_map(|window| window.actions.iter())
        .map(|action| (action.id.as_str(), action.semantics.execution.interactive_job))
        .chain(definition.commands.iter().map(|command| (command.id.as_str(), command.semantics.execution.interactive_job)))
        .collect::<Vec<_>>();
    for route in &routes {
        assert!(classified.iter().any(|(id, classification)| id == route && *classification == semio_framework_plugin::InteractiveJobClassification::Migrated), "{route} owns a retained route but the manifest does not classify it Migrated");
    }
}

/// 🖼️ `setActiveExample` resolves against the registered catalogue rather than a hardcoded id, so
/// the switcher's ids load a document and an unregistered id faults instead of silently doing
/// nothing.
#[semio_framework_async_macros::async_test]
async fn set_active_example_resolves_the_registered_catalogue() {
    let examples = crate::standards::v1::subsets::any::examples();
    assert!(examples.iter().any(|source| source.id() == "demo"), "the subset registers its demo example");
    let doc = default_drawing_document("example-probe", None);
    let history = semio_framework_plugin::HistoryView::empty();
    let config = DrawingConfig::default();
    let view = ArtifactView::new(&doc, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = DrawingSession::default();
    for source in examples {
        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: source.id().into() }, &view, &cfg, &mut session).expect("a registered example id loads its document");
        assert_eq!(emit.effects.len(), 1, "{} must load exactly one document", source.id());
    }
    let reset = set_active_example::handle(&set_active_example::SetActiveExample { example_id: String::new() }, &view, &cfg, &mut session).expect("the empty id resets to a blank drawing");
    assert_eq!(reset.effects.len(), 1);
    assert!(set_active_example::handle(&set_active_example::SetActiveExample { example_id: "semio".into() }, &view, &cfg, &mut session).is_err(), "an unregistered id faults instead of silently doing nothing");
}
//#endregion 🔖️CommandSurface
