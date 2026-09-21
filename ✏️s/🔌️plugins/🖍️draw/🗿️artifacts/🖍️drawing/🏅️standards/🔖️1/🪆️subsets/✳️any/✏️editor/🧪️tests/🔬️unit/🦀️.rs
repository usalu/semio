pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::{ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel};

    pub type DrawingApp = VcsArtifactApp<EditorApp<DrawingPlayApp>>;

    pub struct DrawingAppFixture(DrawingApp);

    impl std::ops::Deref for DrawingAppFixture {
        type Target = DrawingApp;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for DrawingAppFixture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for DrawingAppFixture {
        fn drop(&mut self) {
            if std::thread::panicking() || self.0.close_terminal_is_empty() {
                return;
            }
            semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0);
        }
    }

    /// ✏️ `DrawingPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<DrawingPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<DrawingPlayApp>` builds it.

    /// 🧪️ Draw fixtures carry the production manifest and its exact registered factories.
    pub async fn drawing_app() -> DrawingAppFixture {
        let mut app = new_app_with_registry::<EditorApp<DrawingPlayApp>>(|| App { definition: create_drawing_app(), examples: Vec::new() }).await;
        app.bind_instance_id(meta("local").instance_id).await;
        DrawingAppFixture(app)
    }

    /// 🧰️ Captures the host-owned active utility in one operation's invocation context.
    pub fn meta_with_utility(utility: &str) -> ActionMeta {
        let mut action_meta = meta("local");
        action_meta.view_state = Some(ViewModel { active_utility_id: Some(utility.into()), ..Default::default() });
        action_meta
    }
}

use super::*;
use crate::schema::{default_drawing_document, layer_id, semio_drawing_example_json};
use crate::DrawingLayerNode;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{artifact_app_laws as artifact_laws, PluginApp, ViewModel, SET_ACTIVE_UTILITY_ACTION_ID};
use context::{drawing_app, meta_with_utility, DrawingApp, DrawingAppFixture};

fn canvas_scene(tree: semio_framework_plugin::ComponentTree) -> semio_framework_plugin::Canvas2dScene {
    let decoded = match &tree.root.component {
        semio_framework_plugin::Component::Surface(surface) => semio_framework_ui_scene::decode(surface).map_err(|_| "canvas payload"),
        _ => Err("canvas surface"),
    };
    artifact_laws::project_and_retire_fixture_tree(tree).expect("retire canvas tree");
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
    let mutation = DrawingMutation::RenameLayer(crate::mutations::RenameLayer { layer_id: retained_target.clone(), new_name: "Retained Path".into() });
    let mutation_hex = crate::spr::encode_op(&mutation).expect("Drawing fixture mutation pack").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": DRAWING_DOCUMENT_SCHEMA,
        "id": "drawing-retained-load",
        "vcs": {
            "initialSnapshot": snapshot_hex,
            "edits": [{
                "id": "drawing-retained-edit-final",
                "actor": "drawing-retained-actor",
                "forwards": [mutation_hex],
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
    let poll = drive_drawing_load(&mut app, handle);
    assert_eq!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready, "valid retained Drawing load refusal: {:?}", app.artifact_store_replacement_refusal(handle));
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
    let poll = drive_drawing_load(&mut app, handle);
    assert_eq!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready, "valid staged Drawing load refusal: {:?}", app.artifact_store_replacement_refusal(handle));
    let stale = semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle { operation: handle.operation, generation: semio_framework_job::Generation(handle.generation.0 + 1) };
    assert!(!app.acknowledge_artifact_store_replacement(stale).expect("stale Drawing ACK is refused without retiring the exact owner"));
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
    let json = artifact_laws::project_and_retire_fixture_tree(node).expect("retire semantic tree");
    assert!(json.contains("drawing-play-layers.add.path"));
    assert!(json.contains("Layer 1"));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_panel_lists_boolean_operations() {
    let mut app = drawing_app().await;
    let node = app.render(DRAWING_PLAY_BODY_CATALOGUE, None, &ViewModel::default()).await.expect("render");
    let json = artifact_laws::project_and_retire_fixture_tree(node).expect("retire semantic tree");
    assert!(json.contains("drawing-play-catalogue.path"));
    assert!(json.contains("Boolean union"));
}

#[semio_framework_async_macros::async_test]
async fn add_layer_action_emits_op_and_appends_path() {
    let mut app = drawing_app().await;
    let before = app.snapshot().unwrap().layers.len();
    let meta = artifact_laws::meta("local");
    let (result, receipt) = settled(&mut app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &meta).await;
    assert!(result.mutations.is_empty(), "the migrated route retains its operation until publication");
    assert_one_artifact_publication(&receipt);
    let projection = app.snapshot().unwrap();
    assert_eq!(projection.layers.len(), before + 1);
    assert!(projection.layers.iter().any(|layer| matches!(layer, DrawingLayerNode::Shape(shape) if shape.shape_kind == "rect")));
}

#[semio_framework_async_macros::async_test]
async fn patch_layers_opacity_emits_granular_operation() {
    let mut app = drawing_app().await;
    let id = first_layer_id(&app);
    let meta = artifact_laws::meta("local");
    let (result, receipt) = settled(&mut app, DrawingCommand::PatchLayers(patch_layers::PatchLayers { layer_ids: vec![id], field: "opacity".into(), value: "0.5".into() }), &meta).await;
    assert!(result.mutations.is_empty(), "the migrated route retains its operation until publication");
    assert_one_artifact_publication(&receipt);
    let projection = app.snapshot().unwrap();
    assert!((crate::schema::layer_base(&projection.layers[0]).opacity - 0.5).abs() < f64::EPSILON);
}

#[semio_framework_async_macros::async_test]
async fn patch_layer_name_emits_op_and_changes_projection() {
    let mut app = drawing_app().await;
    let id = first_layer_id(&app);
    let meta = artifact_laws::meta("local");
    let (result, receipt) = settled(&mut app, DrawingCommand::PatchLayer(patch_layer::PatchLayer { layer_id: id, field: "name".into(), value: "Renamed".into() }), &meta).await;
    assert!(result.mutations.is_empty(), "the migrated route retains its operation until publication");
    assert_one_artifact_publication(&receipt);
    assert_eq!(crate::schema::layer_base(&app.snapshot().unwrap().layers[0]).name, "Renamed");
}

#[semio_framework_async_macros::async_test]
async fn host_utility_change_clears_scratch_and_emits_no_history_entry() {
    let mut app = drawing_app().await;
    let shape_meta = meta_with_utility("shapeRect");
    settled(
        &mut app,
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
    .await;
    let before = app.snapshot().unwrap();
    let pen_meta = meta_with_utility("pen");
    let pen_view = pen_meta.view_state.as_ref().expect("host view");
    let tree = app.render(DRAWING_PLAY_BODY_COMPOSITE, None, pen_view).await.expect("render after utility change");
    artifact_laws::project_and_retire_fixture_tree(tree).expect("retire render tree");
    assert_eq!(app.snapshot().unwrap(), before, "utility switching does not mutate the document");
    let (up, _) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 40.0, y: 40.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: false }), &pen_meta).await;
    assert!(up.mutations.is_empty(), "the in-progress shape draft was cleared on utility switch");
}

#[semio_framework_async_macros::async_test]
async fn combine_boolean_creates_boolean_layer() {
    let mut app = drawing_app().await;
    let first_id = first_layer_id(&app);
    let meta = artifact_laws::meta("local");
    let (_, add_receipt) = settled(&mut app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &meta).await;
    assert_one_artifact_publication(&add_receipt);
    let second_id = last_layer_id(&app);
    let (result, receipt) = settled(&mut app, DrawingCommand::CombineBoolean(combine_boolean::CombineBoolean { operation: "union".into(), ids: vec![first_id, second_id] }), &meta).await;
    assert!(result.mutations.is_empty(), "the migrated route retains its operation until publication");
    assert_one_artifact_publication(&receipt);
    assert!(app.snapshot().unwrap().layers.iter().any(|layer| matches!(layer, DrawingLayerNode::Boolean(_))));
}

#[semio_framework_async_macros::async_test]
async fn canvas_point_to_world_matches_host_formula() {
    let camera = store::Viewport2d { x: 100.0, y: 50.0, zoom: 2.0 };
    let (world_x, world_y) = canvas_pointer_down::canvas_point_to_world(&camera, 420.0, 310.0, 800.0, 600.0);
    assert!((world_x - 110.0).abs() < 1e-9);
    assert!((world_y - 55.0).abs() < 1e-9);
}

/// 🧰️ The React host arms utilities PER WINDOW (`active_utility_by_window_id`, keyed by the window
/// instance) and mirrors only the shell's active window into the flat `active_utility_id`; a drag
/// whose view state carries the map alone must still draw the rectangle, not marquee-select.
#[semio_framework_async_macros::async_test]
async fn shape_rect_drag_commits_with_the_per_window_utility_map_alone() {
    let (mut app, meta) = inline_selection_app().await;
    let mut utility_meta = meta.clone();
    let view = meta.view_state.clone().expect("canvas window view");
    utility_meta.view_state = Some(ViewModel {
        active_utility_by_window_id: std::collections::HashMap::from([("drawing-canvas".to_string(), "shapeRect".to_string())]),
        active_utility_id: None,
        ..view
    });
    let before = app.snapshot().unwrap().layers.len();
    settled(
        &mut app,
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 500.0, y: 400.0, width: 1000.0, height: 800.0, shift: false, ctrl: false, meta: false, generation: None, checkpoint_completed_work: None, checkpoint_pending_work: None, ..Default::default() }),
        &utility_meta,
    )
    .await;
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 600.0, y: 500.0, width: 1000.0, height: 800.0, samples: Vec::new() }), &utility_meta).await;
    let (_result, receipt) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 600.0, y: 500.0, width: 1000.0, height: 800.0, shift: false, ctrl: false, meta: false, cancelled: false }), &utility_meta).await;
    // 🛣️ The retained gesture lane publishes its commit through the store lane (the receipt), never
    // through the invocation result's `mutations`.
    let projection = app.snapshot().unwrap();
    assert_eq!(projection.layers.len(), before + 1, "the per-window map arms the rectangle utility: {:?}", receipt.lanes);
    assert!(projection.layers.iter().any(|layer| matches!(layer, DrawingLayerNode::Shape(shape) if shape.shape_kind == "rect")));
    assert!(receipt.effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { utility_id, .. } if utility_id == "selectDirect")), "the canvas returns to select-direct: {:?}", receipt.effects);
    assert_eq!(drawing_active_utility(&ViewModel { active_utility_id: Some("pen".into()), ..Default::default() }), "pen", "the flat field still resolves when no map entry addresses the window");
    assert_eq!(drawing_active_utility(&ViewModel::default()), DRAWING_DEFAULT_UTILITY);
    artifact_laws::close_registered_fixture_app(&mut *app);
}

/// 📤️ `exportDocument` (palette default `pdf`) hands the host one `DownloadMediaExport` whose base64
/// body is a PDF that `s.stdio.pdf`'s 1.4 reader opens on the artboard page — no document operation.
#[semio_framework_async_macros::async_test]
async fn export_document_downloads_a_real_pdf_of_the_document() {
    let (mut app, meta) = inline_selection_app().await;
    let (result, receipt) = settled(&mut app, DrawingCommand::ExportDocument(export_document::ExportDocument { format: "pdf".into() }), &meta).await;
    assert!(result.mutations.is_empty(), "an export is not a document operation");
    let [Effect::DownloadMediaExport { filename, mime_type, data, encoding }] = receipt.effects.as_slice() else { panic!("one download effect, got {:?}", receipt.effects) };
    assert!(filename.ends_with(".pdf"), "{filename}");
    assert_eq!(mime_type, "application/pdf");
    assert_eq!(encoding.as_deref(), Some("base64"));
    let bytes = base64_codec::base64_standard_decode(data).expect("base64 body");
    assert!(bytes.starts_with(b"%PDF-1.4\n"));
    let read = semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::io::decode_pdf(&bytes).expect("stdio's reader opens the download");
    assert_eq!(read.pages.len(), 1);
    let (_result, receipt) = settled(&mut app, DrawingCommand::ExportDocument(export_document::ExportDocument { format: "svg".into() }), &meta).await;
    assert!(matches!(receipt.effects.as_slice(), [Effect::DownloadMediaExport { mime_type, data, encoding: None, .. }] if mime_type == "image/svg+xml" && data.starts_with("<svg")), "{:?}", receipt.effects);
    artifact_laws::close_registered_fixture_app(&mut *app);
}

#[semio_framework_async_macros::async_test]
async fn shape_rect_drag_commits_one_layer_and_requests_utility_reset() {
    let mut app = drawing_app().await;
    let utility_meta = meta_with_utility("shapeRect");
    settled(
        &mut app,
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
    .await;
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 600.0, y: 500.0, width: 1000.0, height: 800.0, samples: Vec::new() }), &utility_meta).await;
    let (result, receipt) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 600.0, y: 500.0, width: 1000.0, height: 800.0, shift: false, ctrl: false, meta: false, cancelled: false }), &utility_meta).await;
    assert!(result.mutations.is_empty(), "the migrated gesture retains its operation until publication");
    assert_one_artifact_publication(&receipt);
    let projection = app.snapshot().unwrap();
    assert!(projection.layers.iter().any(|layer| matches!(layer, DrawingLayerNode::Shape(shape) if shape.shape_kind == "rect")));
    assert!(
        matches!(
            receipt.effects.as_slice(),
            [Effect::SetActiveUtility { window_id, utility_id }] if window_id == DRAWING_PLAY_WINDOW_CANVAS && utility_id == "selectDirect"
        ),
        "the canvas returns to select-direct via a host effect, not a document operation"
    );
}

#[semio_framework_async_macros::async_test]
async fn pen_draft_commits_path_layer_on_enter() {
    let mut app = drawing_app().await;
    let utility_meta = meta_with_utility("pen");
    settled(
        &mut app,
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
    .await;
    settled(
        &mut app,
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
    .await;
    let (result, receipt) = settled(&mut app, DrawingCommand::CanvasCommitDraft(canvas_commit_draft::CanvasCommitDraft {}), &utility_meta).await;
    assert!(result.mutations.is_empty(), "the migrated gesture retains its operation until publication");
    assert_one_artifact_publication(&receipt);
    let projection = app.snapshot().unwrap();
    assert!(projection.layers.iter().any(|layer| matches!(layer, DrawingLayerNode::Path(path) if !path.segments.is_empty())));
    assert!(matches!(receipt.effects.as_slice(), [Effect::SetActiveUtility { utility_id, .. }] if utility_id == "selectDirect"));
}

#[semio_framework_async_macros::async_test]
async fn canvas_escape_cancels_draft_without_committing() {
    let mut app = drawing_app().await;
    let before = app.snapshot().unwrap().layers.len();
    let utility_meta = meta_with_utility("pen");
    settled(
        &mut app,
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
    .await;
    let (result, _) = settled(&mut app, DrawingCommand::CanvasEscape(canvas_escape::CanvasEscape {}), &utility_meta).await;
    assert!(result.mutations.is_empty());
    assert_eq!(app.snapshot().unwrap().layers.len(), before);
}

/// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C: the `interactionSelect` a gesture emits as
/// `Effect::ReplayShellCommand` is folded in-reactor by the typed-operation ladder, so these two
/// selection tests witness the selection snapshot itself instead of the effect. They need what the
/// plugin host supplies: the self-closing `drawing_app()` binds the live instance, while this helper
/// adds a `ViewModel` naming the canvas window (`setCamera` addresses it) with the active utility and the host's
/// settle protocol after every dispatch (`settle_registered_typed_operation`) — exactly
/// `🎚️config/🧪️tests/🔬️window-ownership`'s recipe.
async fn inline_selection_app() -> (DrawingAppFixture, semio_framework_plugin::ActionMeta) {
    use semio_framework_plugin::{ViewWindowInstance, WindowConfigOwner};
    let app = drawing_app().await;
    let view = ViewModel {
        window_instances: vec![ViewWindowInstance { id: "drawing-canvas".into(), window_kind_id: crate::editor::drawing::modes::edit::windows::canvas::config::DrawingCanvasWindowConfigOwner::WINDOW_KIND_ID.into() }],
        ..Default::default()
    };
    let meta = semio_framework_plugin::ActionMeta { view_state: Some(view.for_window_instance("drawing-canvas").expect("canvas window instance")), ..artifact_laws::meta("local") };
    (app, meta)
}

/// 🔁️ One dispatch settled the way the plugin host settles it; the receipt's `effects` are exactly
/// what the host would have been handed.
async fn settled(app: &mut DrawingApp, command: DrawingCommand, meta: &semio_framework_plugin::ActionMeta) -> (semio_framework_plugin::InvocationResult, artifact_laws::TypedOperationFixtureReceipt) {
    let command_id = command.command_id();
    let result = app.dispatch_typed(command, meta).await.unwrap_or_else(|fault| panic!("dispatch {command_id}: {fault:?}"));
    let receipt = artifact_laws::settle_registered_typed_operation(app, meta.instance_id).await.unwrap_or_else(|fault| panic!("retained publication of {command_id} settles: {fault:?}"));
    (result, receipt)
}

fn assert_one_artifact_publication(receipt: &artifact_laws::TypedOperationFixtureReceipt) {
    assert_artifact_publication_units(receipt, 1);
}

fn assert_artifact_publication_units(receipt: &artifact_laws::TypedOperationFixtureReceipt, expected_completions: usize) {
    assert_eq!(receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::Artifact).count(), 1, "one retained artifact result page: {:?}", receipt.lanes);
    assert_eq!(receipt.completions, expected_completions, "every retained component operation reaches one exact terminal witness");
    assert_eq!(receipt.completion_operations.len(), expected_completions, "every terminal witness names its component operation");
    assert_eq!(receipt.revisions.len(), expected_completions, "every terminal witness carries its committed revision");
    let operations = receipt.completion_operations.iter().copied().collect::<std::collections::HashSet<_>>();
    assert_eq!(operations.len(), expected_completions, "component operation terminal witnesses are distinct: {:?}", receipt.completion_operations);
}

fn drawing_composite_shape_meta() -> semio_framework_plugin::ActionMeta {
    use semio_framework_plugin::{ViewSessionIdentity, ViewWindowInstance};
    let view = ViewModel {
        active_utility_by_window_id: std::collections::HashMap::from([(DRAWING_PLAY_WINDOW_CANVAS.to_string(), "shapeRect".to_string())]),
        focused_window_id: Some(DRAWING_PLAY_WINDOW_CANVAS.into()),
        window_instances: vec![ViewWindowInstance { id: DRAWING_PLAY_WINDOW_CANVAS.into(), window_kind_id: DRAWING_PLAY_WINDOW_CANVAS.into() }],
        session_identity: Some(ViewSessionIdentity { user_id: "draw-repeat-owner".into(), display_name: "Draw Repeat Owner".into() }),
        ..Default::default()
    };
    semio_framework_plugin::ActionMeta { view_state: view.for_window_instance(DRAWING_PLAY_WINDOW_CANVAS), ..artifact_laws::meta("local") }
}

fn drawing_gesture_observation(meta: &semio_framework_plugin::ActionMeta, receipt: &artifact_laws::TypedOperationFixtureReceipt) -> String {
    let view = meta.view_state.as_ref().expect("drawing composite view");
    format!(
        "utility={} window={:?} session={:?} operations={:?} revisions={:?} lanes={:?} completions={}",
        drawing_active_utility(view),
        view.window_id,
        view.session_identity,
        receipt.completion_operations,
        receipt.revisions,
        receipt.lanes,
        receipt.completions
    )
}

fn has_direct_select_reset(receipt: &artifact_laws::TypedOperationFixtureReceipt) -> bool {
    receipt.effects.iter().any(|effect| {
        matches!(effect, Effect::SetActiveUtility { window_id, utility_id } if window_id == DRAWING_PLAY_WINDOW_CANVAS && utility_id == DRAWING_DEFAULT_UTILITY)
    })
}

/// 🖱️ Draw10's real retained-owner order: idle hover, rectangle drag, published revision, a fresh
/// per-window view rearm, then one non-overlapping rectangle drag through the same production owner.
#[semio_framework_async_macros::async_test]
async fn repeated_shape_rect_gestures_from_fresh_published_views_commit_distinct_layers_and_reset_twice() {
    let mut app = drawing_app().await;
    let first_meta = drawing_composite_shape_meta();
    let first_view = first_meta.view_state.as_ref().expect("first drawing-composite view");
    let initial_tree = app.render(DRAWING_PLAY_BODY_COMPOSITE, None, first_view).await.expect("initial drawing-composite render");
    artifact_laws::project_and_retire_fixture_tree(initial_tree).expect("retire initial drawing-composite tree");
    let before = app.snapshot().expect("initial Drawing snapshot").layers.len();
    let width = 1587.0;
    let height = 907.0;

    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 396.75, y: 317.45, width, height, samples: Vec::new() }), &first_meta).await;
    settled(
        &mut app,
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 396.75,
            y: 317.45,
            width,
            height,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &first_meta,
    )
    .await;
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 603.06, y: 453.5, width, height, samples: Vec::new() }), &first_meta).await;
    let (_first_up, first_receipt) = settled(
        &mut app,
        DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 603.06, y: 453.5, width, height, shift: false, ctrl: false, meta: false, cancelled: false }),
        &first_meta,
    )
    .await;
    let first_observation = drawing_gesture_observation(&first_meta, &first_receipt);
    let first_layer_count = app.snapshot().expect("first published Drawing snapshot").layers.len();
    let first_reset = has_direct_select_reset(&first_receipt);

    let second_meta = drawing_composite_shape_meta();
    let second_view = second_meta.view_state.as_ref().expect("fresh post-publication drawing-composite view");
    let refreshed_tree = app.render(DRAWING_PLAY_BODY_COMPOSITE, None, second_view).await.expect("post-publication drawing-composite render");
    artifact_laws::project_and_retire_fixture_tree(refreshed_tree).expect("retire post-publication drawing-composite tree");
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 825.24, y: 317.45, width, height, samples: Vec::new() }), &second_meta).await;
    settled(
        &mut app,
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 825.24,
            y: 317.45,
            width,
            height,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &second_meta,
    )
    .await;
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 1031.55, y: 453.5, width, height, samples: Vec::new() }), &second_meta).await;
    let (_second_up, second_receipt) = settled(
        &mut app,
        DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 1031.55, y: 453.5, width, height, shift: false, ctrl: false, meta: false, cancelled: false }),
        &second_meta,
    )
    .await;
    let second_observation = drawing_gesture_observation(&second_meta, &second_receipt);
    let second_reset = has_direct_select_reset(&second_receipt);
    let second_layer_count = app.snapshot().expect("twice-published Drawing snapshot").layers.len();

    let third_meta = drawing_composite_shape_meta();
    let third_view = third_meta.view_state.as_ref().expect("fresh identical-geometry drawing-composite view");
    let third_tree = app.render(DRAWING_PLAY_BODY_COMPOSITE, None, third_view).await.expect("identical-geometry drawing-composite render");
    artifact_laws::project_and_retire_fixture_tree(third_tree).expect("retire identical-geometry drawing-composite tree");
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 825.24, y: 317.45, width, height, samples: Vec::new() }), &third_meta).await;
    settled(
        &mut app,
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 825.24,
            y: 317.45,
            width,
            height,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &third_meta,
    )
    .await;
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 1031.55, y: 453.5, width, height, samples: Vec::new() }), &third_meta).await;
    let (_third_up, third_receipt) = settled(
        &mut app,
        DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 1031.55, y: 453.5, width, height, shift: false, ctrl: false, meta: false, cancelled: false }),
        &third_meta,
    )
    .await;
    let third_observation = drawing_gesture_observation(&third_meta, &third_receipt);
    let third_reset = has_direct_select_reset(&third_receipt);

    let projection = app.snapshot().expect("three-times-published Drawing snapshot");
    artifact_laws::close_registered_fixture_app(&mut *app);
    assert_eq!(first_layer_count, before + 1, "first rectangle publication: {first_observation}");
    assert!(first_reset, "first PointerUp returns its exact drawing-composite utility to Direct Select: {first_observation}");
    assert!(second_reset, "second PointerUp returns its exact drawing-composite utility to Direct Select: {second_observation}");
    assert_eq!(second_layer_count, before + 2, "two retained-owner gestures publish two layers; first=[{first_observation}] second=[{second_observation}]");
    assert!(third_reset, "identical-geometry PointerUp returns its exact drawing-composite utility to Direct Select: {third_observation}");
    assert_eq!(projection.layers.len(), before + 3, "a separate identical-geometry creation still publishes a third layer; second=[{second_observation}] third=[{third_observation}]");
    let [DrawingLayerNode::Shape(first), DrawingLayerNode::Shape(second), DrawingLayerNode::Shape(third)] = &projection.layers[before..] else {
        panic!("all retained-owner additions are shape layers; first=[{first_observation}] second=[{second_observation}] third=[{third_observation}]")
    };
    let first_rect = first.rect.as_ref().expect("first rectangle geometry");
    let second_rect = second.rect.as_ref().expect("second rectangle geometry");
    let third_rect = third.rect.as_ref().expect("identical rectangle geometry");
    assert_eq!((first.shape_kind.as_str(), second.shape_kind.as_str(), third.shape_kind.as_str()), ("rect", "rect", "rect"));
    assert_ne!(first.base.id, second.base.id, "two commits own distinct layer identities; first=[{first_observation}] second=[{second_observation}]");
    assert!(first_rect.x + first_rect.width < second_rect.x, "the two physical coordinate ranges remain non-overlapping; first={first_rect:?} second={second_rect:?}");
    assert_eq!(second_rect, third_rect, "the third creation intentionally repeats the second geometry");
    assert_ne!(second.base.id, third.base.id, "separate identical-geometry creations own distinct identities; second=[{second_observation}] third=[{third_observation}]");
}

fn with_utility(meta: &semio_framework_plugin::ActionMeta, utility: &str) -> semio_framework_plugin::ActionMeta {
    let mut meta = meta.clone();
    meta.view_state = Some(ViewModel { active_utility_id: Some(utility.into()), ..meta.view_state.clone().unwrap_or_default() });
    meta
}

async fn selected_strokes(app: &DrawingApp) -> Vec<String> {
    app.interaction_state().await.selection.get(DRAWING_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default()
}

#[semio_framework_async_macros::async_test]
async fn marquee_select_covers_contained_layer_only() {
    // 🔖 Built through dispatched commands (`add-layer` + `patch-layer` transform fields), never
    // a whole-document swap — `SetSnapshot` is banned vocabulary now (see
    // `🧬️mutations/🦀️.rs`'s module doc); this exercises the same real semantic
    // `create-layer`/`update-layer-transform` mutations a live editor session would emit.
    let (mut app, meta) = inline_selection_app().await;
    let utility_meta = with_utility(&meta, "selectMarquee");
    let initial_id = layer_id(&app.snapshot().unwrap().layers[0]).to_string();
    settled(&mut app, DrawingCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: initial_id }), &meta).await;

    settled(&mut app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &meta).await;
    let rect_a_id = layer_id(app.snapshot().unwrap().layers.last().unwrap()).to_string();
    for (field, value) in [("transformX", "10"), ("transformY", "10"), ("transformScaleX", "0.15625"), ("transformScaleY", "0.208333")] {
        settled(&mut app, DrawingCommand::PatchLayer(patch_layer::PatchLayer { layer_id: rect_a_id.clone(), field: field.into(), value: value.into() }), &meta).await;
    }

    settled(&mut app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:ellipse".into() }), &meta).await;
    let ellipse_b_id = layer_id(app.snapshot().unwrap().layers.last().unwrap()).to_string();
    for (field, value) in [("transformX", "200"), ("transformY", "200")] {
        settled(&mut app, DrawingCommand::PatchLayer(patch_layer::PatchLayer { layer_id: ellipse_b_id.clone(), field: field.into(), value: value.into() }), &meta).await;
    }

    settled(&mut app, DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } }), &meta).await;
    settled(
        &mut app,
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
    .await;
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 460.0, y: 360.0, width: 800.0, height: 600.0, samples: Vec::new() }), &utility_meta).await;
    let (result, receipt) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 460.0, y: 360.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: false }), &utility_meta).await;
    // 🕹️ Selection is framework-owned (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
    // the marquee hit-test emits `interactionSelect` for exactly the contained rect via an
    // `Effect::ReplayShellCommand` — folded in-reactor (§2 C), so the host never sees it and the
    // selection already holds the rect when the release settles.
    assert!(result.mutations.is_empty(), "a pure marquee-select gesture is not a document operation");
    assert!(!receipt.effects.iter().any(|effect| matches!(effect, Effect::ReplayShellCommand { .. })), "interactionSelect is folded in-reactor, never handed to the host: {:?}", receipt.effects);
    assert_eq!(selected_strokes(&app).await, vec![rect_a_id.clone()], "only the contained rect is selected, not the outside ellipse");
    artifact_laws::close_registered_fixture_app(&mut *app);
}

#[semio_framework_async_macros::async_test]
async fn set_camera_writes_runtime_and_emits_no_operations() {
    let (mut app, meta) = inline_selection_app().await;
    let before = app.snapshot().expect("projection");
    let (result, _) = settled(&mut app, DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 5.0, y: 5.0, zoom: 2.0 } }), &meta).await;
    assert!(result.mutations.is_empty(), "camera is a view action and emits no operations");
    assert_eq!(app.snapshot().expect("projection"), before, "camera never mutates the document");
    let scene = canvas_scene(app.render(DRAWING_PLAY_BODY_COMPOSITE, None, meta.view_state.as_ref().expect("addressed canvas view")).await.expect("render"));
    assert_eq!([scene.camera_x, scene.camera_y, scene.zoom], [5.0, 5.0, 2.0]);
}

#[semio_framework_async_macros::async_test]
async fn set_camera_zoom_updates_zoom_and_keeps_pan_via_runtime() {
    let (mut app, meta) = inline_selection_app().await;
    settled(&mut app, DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 4.0, y: 5.0, zoom: 1.0 } }), &meta).await;
    let (result, _) = settled(&mut app, DrawingCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 3.0 }), &meta).await;
    assert!(result.mutations.is_empty(), "camera zoom is a view action and emits no operations");
    let scene = canvas_scene(app.render(DRAWING_PLAY_BODY_COMPOSITE, None, meta.view_state.as_ref().expect("addressed canvas view")).await.expect("render"));
    assert_eq!([scene.camera_x, scene.camera_y, scene.zoom], [4.0, 5.0, 3.0]);
}

#[semio_framework_async_macros::async_test]
async fn add_layer_undo_round_trip_through_wrapper() {
    let mut app = drawing_app().await;
    let before = app.snapshot().unwrap().layers.len();
    artifact_laws::assert_undo_redo_round_trip(&mut *app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "path".into() }), |app| app.snapshot().unwrap().layers.len(), before, before + 1).await;
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
    let set_active_utility = definition.actions.iter().find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).expect("setActiveUtility app action");
    assert!(matches!(set_active_utility.kind, ActionKind::View));
    assert!(semio_framework::window_kind_actions(&definition, scene).iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID));
    assert!(!scene.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID), "app action stays canonical instead of being copied into the canvas window");
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
async fn canvas_pointer_up_direct_pick_selects_inline() {
    let (mut app, meta) = inline_selection_app().await;
    // 🔖 The default document's one layer is an empty-segment path (no bounds to hit-test against
    // — see `default_drawing_document`), so a real shape is added first, mirroring
    // `marquee_select_covers_contained_layer_only`'s own setup.
    let initial_id = first_layer_id(&app);
    settled(&mut app, DrawingCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: initial_id }), &meta).await;
    settled(&mut app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &meta).await;
    let rect_id = last_layer_id(&app);
    settled(&mut app, DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } }), &meta).await;
    // 🎯️ Default `shape:rect` geometry is world (0,0)-(128,96); screen (110,110) on a 200x200
    // viewport with the identity camera above maps to world (10,10) — inside the rect.
    let (result, receipt) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 110.0, y: 110.0, width: 200.0, height: 200.0, shift: false, ctrl: false, meta: false, cancelled: false }), &meta).await;
    assert!(result.mutations.is_empty(), "a direct pick is not a document operation");
    assert!(!receipt.effects.iter().any(|effect| matches!(effect, Effect::ReplayShellCommand { .. })), "interactionSelect is folded in-reactor, never handed to the host: {:?}", receipt.effects);
    assert_eq!(selected_strokes(&app).await, vec![rect_id], "the picked rect is selected inside the carrying operation");
    artifact_laws::close_registered_fixture_app(&mut *app);
}

#[semio_framework_async_macros::async_test]
async fn set_selected_opacity_reads_the_framework_interaction_selection() {
    let (mut app, meta) = inline_selection_app().await;
    let id = first_layer_id(&app);
    let targets = serde_json::to_string(&vec![serde_json::json!({ "granularity": DRAWING_INTERACTION_GRANULARITY, "id": id })]).unwrap();
    let admission = app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&dsl::json::to_dsl_value(&dsl::json!({ "domainId": DRAWING_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" }))), &meta).await.expect("select");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app, admission).await.expect("selection publication settles");
    assert_eq!(selected_strokes(&app).await, vec![id.clone()], "the retained opacity command reads the framework-owned selection published for its canvas window");
    let (result, receipt) = settled(&mut app, DrawingCommand::SetSelectedOpacity(set_selected_opacity::SetSelectedOpacity { value: 0.25 }), &meta).await;
    assert!(result.mutations.is_empty(), "the migrated route retains its operation until publication");
    assert_artifact_publication_units(&receipt, 2);
    assert!((crate::schema::layer_base(&app.snapshot().unwrap().layers[0]).opacity - 0.25).abs() < f64::EPSILON);
}

#[semio_framework_async_macros::async_test]
async fn drawing_labels_resolve_native_by_default() {
    let mut app = drawing_app().await;
    let node = app.render(DRAWING_PLAY_BODY_LAYERS, None, &ViewModel::default()).await.expect("render");
    let json = artifact_laws::project_and_retire_fixture_tree(node).expect("retire semantic tree");
    assert!(json.contains("Add Path"));
    assert!(json.contains("Add Rectangle"));
    assert!(!json.contains("Pfad hinzufügen"));
}

#[semio_framework_async_macros::async_test]
async fn drawing_labels_translate_panels_in_german() {
    let mut app = drawing_app().await;
    let view_state = ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let layers_node = app.render(DRAWING_PLAY_BODY_LAYERS, None, &view_state).await.expect("render");
    let layers_json = artifact_laws::project_and_retire_fixture_tree(layers_node).expect("retire layers tree");
    assert!(layers_json.contains("Pfad hinzufügen"));
    assert!(layers_json.contains("Rechteck hinzufügen"));
    assert!(!layers_json.contains("Add Path"));
    let catalogue_node = app.render(DRAWING_PLAY_BODY_CATALOGUE, None, &view_state).await.expect("render");
    let catalogue_json = artifact_laws::project_and_retire_fixture_tree(catalogue_node).expect("retire catalogue tree");
    assert!(catalogue_json.contains("\"Ellipse\""));
    assert!(catalogue_json.contains("Nachzeichnung"));
}

#[semio_framework_async_macros::async_test]
async fn drawing_io_declares_vector_out_and_export_media_covers_both_ports() {
    let mut app = drawing_app().await;
    let meta = artifact_laws::meta("local");
    let (_, receipt) = settled(&mut app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &meta).await;
    assert_one_artifact_publication(&receipt);
    let projection = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let vector = DrawingPlayApp::export_media("vector:out", &doc).expect("vector:out");
    let MediaPayload::Structured { schema, json } = vector.payload else { panic!("expected structured svg payload") };
    assert_eq!(schema, "2d.drawing");
    assert!(json.starts_with("<svg"));
    assert!(DrawingPlayApp::export_media("artifact:out", &doc).is_ok());
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
    let config = NoConfig::default();

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
    let config = NoConfig::default();
    session.step_gesture(canvas_pointer_down::drawing_gesture::Event::PointerDown { utility: "shapeRect".into(), world: [1.0, 2.0], shift: false, ctrl: false, meta: false }, &document, &config);
    let context_before = session.gesture.context.clone();
    let _ = session.preview();
    let _ = session.preview();
    assert_eq!(session.gesture.context, context_before, "preview must never mutate the live gesture scratch it reads");
}

//#region 🧵️BatchedSamplesAndCancel
fn session_with(utility: &str) -> (DrawingSession, DrawingSnapshot, NoConfig, semio_framework_plugin::HistoryView) {
    (DrawingSession::with_active_utility(utility), default_drawing_document("empty", None), NoConfig::default(), semio_framework_plugin::HistoryView::empty())
}

/// 🧵️ LAW (design L4 / §2 D): a batch of four samples leaves the shape drag exactly where four
/// separate moves left it — the cursor follows the LAST sample, not an intermediate one.
#[semio_framework_async_macros::async_test]
async fn a_batched_move_drives_the_gesture_to_its_last_sample() {
    let path = [[420.0, 320.0], [480.0, 300.0], [520.0, 380.0], [460.0, 360.0]];
    let run = |batched: bool| {
        let (mut session, document, config, history) = session_with("shapeRect");
        let view = semio_framework_plugin::ArtifactView::new(&document, &history);
        let cfg = semio_framework_plugin::ConfigView { snapshot: &config, window: None };
        session.step_gesture(canvas_pointer_down::drawing_gesture::Event::PointerDown { utility: "shapeRect".into(), world: [0.0, 0.0], shift: false, ctrl: false, meta: false }, &document, &config);
        if batched {
            let [x, y] = path[3];
            let emit = canvas_pointer_move::handle(&canvas_pointer_move::CanvasPointerMove { x, y, width: 800.0, height: 600.0, samples: path.to_vec() }, &view, &cfg, &mut session).expect("batched move");
            assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty(), "a mid-drag batch emits no operation");
        } else {
            for [x, y] in path {
                canvas_pointer_move::handle(&canvas_pointer_move::CanvasPointerMove { x, y, width: 800.0, height: 600.0, samples: Vec::new() }, &view, &cfg, &mut session).expect("move");
            }
        }
        session.gesture.context.clone()
    };
    let one_per_event = run(false);
    let one_batch = run(true);
    assert_eq!(one_batch.cursor, one_per_event.cursor, "the batch ends on the same cursor as four separate moves");
    assert_eq!(one_batch.start, one_per_event.start);
    let (session, ..) = session_with("shapeRect");
    let expected = canvas_pointer_down::canvas_point_to_world(&session.window_config.viewport, 460.0, 360.0, 800.0, 600.0);
    assert_eq!(one_batch.cursor, [expected.0, expected.1], "the cursor is the LAST sample of the batch");
}

/// 🚫️ LAW (design §2 D): a cancelled release drops a live drag and commits/selects nothing.
#[semio_framework_async_macros::async_test]
async fn a_cancelled_release_commits_nothing_and_leaves_the_gesture_idle() {
    for utility in ["shapeRect", "selectMarquee"] {
        let (mut session, document, config, history) = session_with(utility);
        let view = semio_framework_plugin::ArtifactView::new(&document, &history);
        let cfg = semio_framework_plugin::ConfigView { snapshot: &config, window: None };
        session.step_gesture(canvas_pointer_down::drawing_gesture::Event::PointerDown { utility: utility.into(), world: [0.0, 0.0], shift: false, ctrl: false, meta: false }, &document, &config);
        canvas_pointer_move::handle(&canvas_pointer_move::CanvasPointerMove { x: 600.0, y: 500.0, width: 800.0, height: 600.0, samples: Vec::new() }, &view, &cfg, &mut session).expect("move");
        assert!(!session.gesture.matches("idle"), "{utility}: the drag is live before the cancel");
        let emit = canvas_pointer_up::handle(&canvas_pointer_up::CanvasPointerUp { x: 600.0, y: 500.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: true }, &view, &cfg, &mut session).expect("cancel");
        assert!(emit.artifact_mutations.is_empty(), "{utility}: a cancel never commits");
        assert!(emit.effects.is_empty(), "{utility}: a cancel never selects or resets the utility");
        assert!(session.gesture.matches("idle"), "{utility}: no gesture survives a cancel");
        assert!(session.point_query.is_none(), "{utility}: no marquee/pick query is retained");
    }
    // 🎯️ An idle cancel (select-direct) must not fall back to a pick either.
    let (mut session, document, config, history) = session_with("selectDirect");
    let view = semio_framework_plugin::ArtifactView::new(&document, &history);
    let cfg = semio_framework_plugin::ConfigView { snapshot: &config, window: None };
    let emit = canvas_pointer_up::handle(&canvas_pointer_up::CanvasPointerUp { x: 400.0, y: 300.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: true }, &view, &cfg, &mut session).expect("cancel");
    assert!(emit.effects.is_empty() && emit.artifact_mutations.is_empty());
    assert!(session.gesture.matches("idle"));
}

/// 🧵️ LAW: a legacy one-per-event wire (no `samples`, no `cancelled`) decodes as one sample at
/// `(x, y)` and a real release.
#[semio_framework_async_macros::async_test]
async fn canvas_pointer_wire_defaults_samples_and_cancelled() {
    use dsl::FromValue;
    let f = dsl::DslValue::float;
    let legacy = dsl::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0)), ("width".into(), f(800.0)), ("height".into(), f(600.0))]);
    let moved = canvas_pointer_move::CanvasPointerMove::from_value(legacy.clone()).expect("legacy move decodes");
    assert!(moved.samples.is_empty());
    assert_eq!(moved.samples_or_last(), vec![[5.0, 6.0]], "an absent `samples` is the single (x, y)");
    assert_eq!(moved.last_sample(), [5.0, 6.0]);
    let pair = |x: f64, y: f64| dsl::DslValue::Array(vec![f(x), f(y)]);
    let batched = dsl::DslValue::Object(vec![("x".into(), f(3.0)), ("y".into(), f(4.0)), ("width".into(), f(800.0)), ("height".into(), f(600.0)), ("samples".into(), dsl::DslValue::Array(vec![pair(1.0, 1.5), pair(2.0, 2.5), pair(3.0, 4.0)]))]);
    let moved = canvas_pointer_move::CanvasPointerMove::from_value(batched).expect("batched move decodes");
    assert_eq!(moved.samples, vec![[1.0, 1.5], [2.0, 2.5], [3.0, 4.0]]);
    assert_eq!(moved.last_sample(), [3.0, 4.0]);
    let legacy_up = dsl::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0)), ("width".into(), f(800.0)), ("height".into(), f(600.0)), ("shift".into(), dsl::DslValue::Bool(false)), ("ctrl".into(), dsl::DslValue::Bool(false)), ("meta".into(), dsl::DslValue::Bool(false))]);
    let released = canvas_pointer_up::CanvasPointerUp::from_value(legacy_up).expect("legacy release decodes");
    assert!(!released.cancelled, "an absent `cancelled` is a real release");
}
//#endregion 🧵️BatchedSamplesAndCancel
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
        DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 1.0, y: 2.0, zoom: 1.5 } }),
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
        DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 1.0, y: 2.0, width: 800.0, height: 600.0, samples: vec![[0.5, 1.5], [1.0, 2.0]] }),
        DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 1.0, y: 2.0, width: 800.0, height: 600.0, shift: false, ctrl: true, meta: false, cancelled: false }),
        DrawingCommand::CanvasDoubleClick(canvas_double_click::CanvasDoubleClick {}),
        DrawingCommand::CanvasCommitDraft(canvas_commit_draft::CanvasCommitDraft {}),
        DrawingCommand::CanvasEscape(canvas_escape::CanvasEscape {}),
        DrawingCommand::ExportDocument(export_document::ExportDocument { format: "pdf".into() }),
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
        "export-document",
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
    assert_eq!(DRAWING_BOUNDED_TOOL_IDS.len(), 19);
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
    assert!(<DrawingPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_none(), "NoConfig owns no document-config publication lane");
    let mut window_config_owners = semio_framework_plugin::WindowConfigOwnerRegistry::default();
    <DrawingPlayApp as ArtifactEditor>::register_window_config_owners(&mut window_config_owners).expect("register canvas window config owner");
    assert!(!window_config_owners.is_empty(), "the canvas window config owns its own retained publication authority");

    let definition = create_drawing_app();
    let classified = definition
        .actions
        .iter()
        .map(|action| (action.id.as_str(), action.semantics.execution.interactive_job))
        .chain(
            definition
        .window_kinds
        .iter()
        .flat_map(|window| window.actions.iter())
        .map(|action| (action.id.as_str(), action.semantics.execution.interactive_job)),
        )
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
    let config = NoConfig::default();
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
