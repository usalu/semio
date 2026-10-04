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

    semio_framework_plugin::history_edit_acceptance_law!("draw", DrawingPlayApp, || App { definition: create_drawing_app(), examples: Vec::new() }, "../..");

    /// 🧰️ Captures the host-owned active utility in one operation's invocation context.
    pub fn meta_with_utility(utility: &str) -> ActionMeta {
        let mut action_meta = meta("local");
        action_meta.view_state = Some(ViewModel { active_utility_id: Some(utility.into()), ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) });
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

async fn rendered_drawing_canvas(app:&mut DrawingApp,fixture:Option<&str>,view:&ViewModel)->Result<semio_framework_plugin::ComponentTree,semio_framework_plugin::Fault>{
    use semio_framework_plugin::reactor::jobs::{start_job,step_job,cancel_job,JobBudget,JobStep};
    for effect in app.pending_effects(Some(view)).await{match effect{
        Effect::CancelJob{job}=>cancel_job(job).await,
        Effect::SpawnJob{job,kind,input,..}if kind=="semio.draw.mounted-vector"=>{start_job(job,&kind,&input).await;let mut complete=false;for _ in 0..100000{match step_job(job,JobBudget{fuel:65536,deadline_ms:8}).await{JobStep::Running(_)=>{},JobStep::Done(_)=>{complete=true;break;},JobStep::Failed(error)=>panic!("registered Drawing geometry failed: {}",String::from_utf8_lossy(&error))}}assert!(complete,"registered Drawing geometry must settle before its canvas witness");eprintln!("[DEBUG] Actual Drawing app pending effects drove registered reactor geometry job {job} to complete before canvas inspection");},
        _=>{},
    }}
    app.render(DRAWING_PLAY_BODY_COMPOSITE,fixture,view).await
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
                "line": null,
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
    let node = rendered_drawing_canvas(&mut app, Some(example_json.as_str()), &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
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
    let node = rendered_drawing_canvas(&mut app, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    let scene = canvas_scene(node);
    let layers_json = scene.layers_json.as_str();
    assert!(layers_json.contains("1024 × 1024"), "blank documents show default artboard dimensions");
}

#[semio_framework_async_macros::async_test]
async fn layers_panel_lists_default_layer() {
    let mut app = drawing_app().await;
    let node = app.render(DRAWING_PLAY_BODY_LAYERS, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    let json = artifact_laws::project_and_retire_fixture_tree(node).expect("retire semantic tree");
    assert!(json.contains("drawing-play-layers.add.path"));
    assert!(json.contains("Layer 1"));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_panel_lists_boolean_operations() {
    let mut app = drawing_app().await;
    let node = app.render(DRAWING_PLAY_BODY_CATALOGUE, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
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
    let created=projection.layers.last().unwrap();
    assert_eq!(selected_strokes(&app).await,vec![layer_id(created).to_string()]);
    assert!(crate::schema::layer_base(created).attributes.fill.is_some());
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
    let tree = rendered_drawing_canvas(&mut app, None, pen_view).await.expect("render after utility change");
    artifact_laws::project_and_retire_fixture_tree(tree).expect("retire render tree");
    assert_eq!(app.snapshot().unwrap(), before, "utility switching does not mutate the document");
    let (up, _) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 40.0, y: 40.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: false }), &pen_meta).await;
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
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 600.0, y: 500.0, width: 1000.0, height: 800.0, samples: Vec::new() }), &utility_meta).await;
    let (_result, receipt) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 600.0, y: 500.0, width: 1000.0, height: 800.0, shift: false, ctrl: false, meta: false, cancelled: false }), &utility_meta).await;
    // 🛣️ The retained gesture lane publishes its commit through the store lane (the receipt), never
    // through the invocation result's `mutations`.
    let projection = app.snapshot().unwrap();
    assert_eq!(projection.layers.len(), before + 1, "the per-window map arms the rectangle utility: {:?}", receipt.lanes);
    assert!(projection.layers.iter().any(|layer| matches!(layer, DrawingLayerNode::Shape(shape) if shape.shape_kind == "rect")));
    assert!(receipt.effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { utility_id, .. } if utility_id == "selectDirect")), "the canvas returns to select-direct: {:?}", receipt.effects);
    assert_eq!(drawing_active_utility(&ViewModel { active_utility_id: Some("pen".into()), ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }), "pen", "the flat field still resolves when no map entry addresses the window");
    assert_eq!(drawing_active_utility(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), DRAWING_DEFAULT_UTILITY);
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
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 600.0, y: 500.0, width: 1000.0, height: 800.0, samples: Vec::new() }), &utility_meta).await;
    let (result, receipt) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 600.0, y: 500.0, width: 1000.0, height: 800.0, shift: false, ctrl: false, meta: false, cancelled: false }), &utility_meta).await;
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
    let created=projection.layers.last().unwrap();
    assert_eq!(selected_strokes(&app).await,vec![layer_id(created).to_string()]);
    assert!(crate::schema::layer_base(created).attributes.stroke.as_ref().is_some_and(|stroke|stroke.width>0.0));
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
/// `🎚️config/🧪️tests/🔬️window`'s recipe.
async fn inline_selection_app() -> (DrawingAppFixture, semio_framework_plugin::ActionMeta) {
    use semio_framework_plugin::{ViewWindowInstance, WindowConfigOwner};
    let app = drawing_app().await;
    let view = ViewModel {
        window_instances: vec![ViewWindowInstance { id: "drawing-canvas".into(), window_kind_id: crate::editor::drawing::modes::edit::windows::canvas::config::DrawingCanvasWindowConfigOwner::WINDOW_KIND_ID.into() }],
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
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

/// 📸️ Loads test geometry through the same retained envelope owner as the host.
fn load_drawing_fixture(app: &mut DrawingApp, snapshot: &DrawingSnapshot) {
    use store::ArtifactPack;
    let pack = snapshot.encode_pack().iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": DRAWING_DOCUMENT_SCHEMA, "id": snapshot.id,
        "vcs": { "initialSnapshot": pack, "edits": [], "changes": [], "checkpoints": [], "alternatives": [] },
        "editMessages": [], "conflicts": []
    })).unwrap();
    let handle = admit_drawing_envelope(app, &wire);
    assert_eq!(drive_drawing_load(app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready, "fixture admission: {:?}", app.artifact_store_replacement_refusal(handle));
    assert!(app.acknowledge_artifact_store_replacement(handle).unwrap());
    assert_eq!(app.snapshot().unwrap(), *snapshot);
}

fn assert_one_artifact_publication(receipt: &artifact_laws::TypedOperationFixtureReceipt) {
    assert_artifact_publication_units(receipt, 1);
}

#[semio_framework_async_macros::async_test]
async fn direct_drag_projects_without_editing_and_publishes_only_on_release() {
    let surface_behavior: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🎬️surface-behavior/🔣️.json"))).expect("shared surface behavior fixture");
    let law = surface_behavior["cases"].as_array().unwrap().iter().find(|entry| entry["id"] == "draw-canvas2d").expect("Draw surface behavior");
    assert_eq!(law["appId"], "s.draw.drawing@1/*#editor");
    assert_eq!(law["terminalPolicy"], "cancelled-action-discards-draft");
    assert_eq!(law["publishedOnCancel"][0]["action"], "canvasPointerUp");
    assert_eq!(law["publishedOnCancel"][0]["cancelled"], true);
    for cancelled in [true, false] {
        let (mut app, mut meta) = inline_selection_app().await;
        meta.view_state.as_mut().unwrap().active_utility_id = Some("selectDirect".into());
        let layer = crate::schema::create_drawing_shape_layer_rect("Drag target");
        let id = layer_id(&layer).to_string();
        let snapshot = DrawingSnapshot { id: "direct-drag".into(), layers: vec![layer], ..Default::default() };
        load_drawing_fixture(&mut app, &snapshot);
        settled(&mut app, DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } }), &meta).await;
        let before = app.snapshot().unwrap();
        let view = meta.view_state.as_ref().unwrap();
        let scene = canvas_scene(rendered_drawing_canvas(&mut app, None, view).await.unwrap());
        let records: Vec<serde_json::Value> = serde_json::from_str(&scene.layers_json).unwrap();
        let original = records.iter().find(|record| record["id"] == id).unwrap()["transform"].clone();
        let (_, down) = settled(&mut app, DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 400.0, y: 300.0, width: 800.0, height: 600.0, ..Default::default() }), &meta).await;
        assert!(!down.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact));
        assert_eq!(app.snapshot().unwrap(), before);
        let (_, moved) = settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 430.0, y: 320.0, width: 800.0, height: 600.0, samples: vec![[410.0,310.0],[430.0,320.0]] }), &meta).await;
        assert!(!moved.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact));
        assert_eq!(app.snapshot().unwrap(), before);
        let scene = canvas_scene(rendered_drawing_canvas(&mut app, None, view).await.unwrap());
        let records: Vec<serde_json::Value> = serde_json::from_str(&scene.layers_json).unwrap();
        let preview = &records.iter().find(|record| record["id"] == id).unwrap()["transform"];
        assert_eq!(preview[4].as_f64().unwrap(), original[4].as_f64().unwrap() + 30.0);
        assert_eq!(preview[5].as_f64().unwrap(), original[5].as_f64().unwrap() + 20.0);
        let (_, released) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 430.0, y: 320.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled }), &meta).await;
        if cancelled {
            assert!(!released.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact));
            assert_eq!(app.snapshot().unwrap(), before);
            assert_eq!(law["artifactAfterCancel"], "unchanged");
            let (_, stale_move) = settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 450.0, y: 340.0, width: 800.0, height: 600.0, samples: vec![[450.0,340.0]] }), &meta).await;
            let (_, stale_up) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 450.0, y: 340.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: false }), &meta).await;
            assert!(!stale_move.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact));
            assert!(!stale_up.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact));
            assert_eq!(app.snapshot().unwrap(), before, "a stale move/up that crosses the renderer fence still cannot mutate the Draw document");
            settled(&mut app, DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 400.0, y: 300.0, width: 800.0, height: 600.0, ..Default::default() }), &meta).await;
            settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 430.0, y: 320.0, width: 800.0, height: 600.0, samples: vec![[430.0,320.0]] }), &meta).await;
            let (_, fresh) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 430.0, y: 320.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: false }), &meta).await;
            assert_one_artifact_publication(&fresh);
            let after = app.snapshot().unwrap();
            let transform = &crate::schema::layer_base(&after.layers[0]).transform;
            assert_eq!((transform.x, transform.y), (30.0, 20.0));
        } else {
            assert_one_artifact_publication(&released);
            assert_eq!(law["freshGestureResult"], "one-artifact-publication");
            let after = app.snapshot().unwrap();
            let transform = &crate::schema::layer_base(&after.layers[0]).transform;
            assert_eq!(transform.x, 30.0);
            assert_eq!(transform.y, 20.0);
        }
    }
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
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
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
    let initial_tree = rendered_drawing_canvas(&mut app, None, first_view).await.expect("initial drawing-composite render");
    artifact_laws::project_and_retire_fixture_tree(initial_tree).expect("retire initial drawing-composite tree");
    let before = app.snapshot().expect("initial Drawing snapshot").layers.len();
    let width = 1587.0;
    let height = 907.0;

    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 396.75, y: 317.45, width, height, samples: Vec::new() }), &first_meta).await;
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
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 603.06, y: 453.5, width, height, samples: Vec::new() }), &first_meta).await;
    let (_first_up, first_receipt) = settled(
        &mut app,
        DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 603.06, y: 453.5, width, height, shift: false, ctrl: false, meta: false, cancelled: false }),
        &first_meta,
    )
    .await;
    let first_observation = drawing_gesture_observation(&first_meta, &first_receipt);
    let first_layer_count = app.snapshot().expect("first published Drawing snapshot").layers.len();
    let first_reset = has_direct_select_reset(&first_receipt);

    let second_meta = drawing_composite_shape_meta();
    let second_view = second_meta.view_state.as_ref().expect("fresh post-publication drawing-composite view");
    let refreshed_tree = rendered_drawing_canvas(&mut app, None, second_view).await.expect("post-publication drawing-composite render");
    artifact_laws::project_and_retire_fixture_tree(refreshed_tree).expect("retire post-publication drawing-composite tree");
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 825.24, y: 317.45, width, height, samples: Vec::new() }), &second_meta).await;
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
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 1031.55, y: 453.5, width, height, samples: Vec::new() }), &second_meta).await;
    let (_second_up, second_receipt) = settled(
        &mut app,
        DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 1031.55, y: 453.5, width, height, shift: false, ctrl: false, meta: false, cancelled: false }),
        &second_meta,
    )
    .await;
    let second_observation = drawing_gesture_observation(&second_meta, &second_receipt);
    let second_reset = has_direct_select_reset(&second_receipt);
    let second_layer_count = app.snapshot().expect("twice-published Drawing snapshot").layers.len();

    let third_meta = drawing_composite_shape_meta();
    let third_view = third_meta.view_state.as_ref().expect("fresh identical-geometry drawing-composite view");
    let third_tree = rendered_drawing_canvas(&mut app, None, third_view).await.expect("identical-geometry drawing-composite render");
    artifact_laws::project_and_retire_fixture_tree(third_tree).expect("retire identical-geometry drawing-composite tree");
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 825.24, y: 317.45, width, height, samples: Vec::new() }), &third_meta).await;
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
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 1031.55, y: 453.5, width, height, samples: Vec::new() }), &third_meta).await;
    let (_third_up, third_receipt) = settled(
        &mut app,
        DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 1031.55, y: 453.5, width, height, shift: false, ctrl: false, meta: false, cancelled: false }),
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
    meta.view_state = Some(ViewModel { active_utility_id: Some(utility.into()), ..meta.view_state.clone().unwrap_or_else(||ViewModel::new(semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native)) });
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
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 460.0, y: 360.0, width: 800.0, height: 600.0, samples: Vec::new() }), &utility_meta).await;
    let (result, receipt) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 460.0, y: 360.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: false }), &utility_meta).await;
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
    let scene = canvas_scene(rendered_drawing_canvas(&mut app, None, meta.view_state.as_ref().expect("addressed canvas view")).await.expect("render"));
    assert_eq!([scene.camera_x, scene.camera_y, scene.zoom], [5.0, 5.0, 2.0]);
}

#[semio_framework_async_macros::async_test]
async fn set_camera_zoom_updates_zoom_and_keeps_pan_via_runtime() {
    let (mut app, meta) = inline_selection_app().await;
    settled(&mut app, DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 4.0, y: 5.0, zoom: 1.0 } }), &meta).await;
    let (result, _) = settled(&mut app, DrawingCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 3.0 }), &meta).await;
    assert!(result.mutations.is_empty(), "camera zoom is a view action and emits no operations");
    let scene = canvas_scene(rendered_drawing_canvas(&mut app, None, meta.view_state.as_ref().expect("addressed canvas view")).await.expect("render"));
    assert_eq!([scene.camera_x, scene.camera_y, scene.zoom], [4.0, 5.0, 3.0]);
}

#[semio_framework_async_macros::async_test]
async fn add_layer_undo_round_trip_through_wrapper() {
    let mut app = drawing_app().await;
    let before = app.snapshot().unwrap().layers.len();
    artifact_laws::assert_undo_redo_round_trip(&mut *app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "path".into() }), |app| app.snapshot().unwrap().layers.len(), before, before + 1).await;
}

#[semio_framework_async_macros::async_test]
async fn path_join_conversion_position_and_translation_each_undo_as_one_edit() {
    use crate::schema::geometry::editing::{edit_path,PathEdit,PathPoint,PathPointRef,SegmentType};
    for edit in [PathEdit::Join { index:1,other:2 },PathEdit::Convert { index:1,target:SegmentType::Cubic },PathEdit::Position {index:1,point:PathPoint::Anchor,to:[15.0,5.0]},PathEdit::Translate {points:vec![PathPointRef {index:0,point:PathPoint::Anchor},PathPointRef {index:1,point:PathPoint::Anchor}],delta:[3.0,-2.0]}] {
        let mut app=drawing_app().await;
        let before=vec![crate::PathSegment::Move { to:[0.0,0.0] },crate::PathSegment::Line { to:[10.0,0.0] },crate::PathSegment::Move { to:[20.0,0.0] },crate::PathSegment::Line { to:[30.0,0.0] }];
        let after=edit_path(&before,&edit).unwrap();
        let layer=crate::schema::create_drawing_path_layer("Editable",before.clone());
        let id=layer_id(&layer).to_string();
        let snapshot=DrawingSnapshot { id:"path-history".into(),layers:vec![layer],..Default::default() };
        load_drawing_fixture(&mut app,&snapshot);
        let command=DrawingCommand::EditPath(edit_path::EditPath { layer_id:id,edit:Box::new(edit) });
        artifact_laws::assert_undo_redo_round_trip(&mut *app,command,|app| {
            let snapshot=app.snapshot().unwrap();
            let DrawingLayerNode::Path(path)=&snapshot.layers[0] else { panic!("Expected path") };
            path.segments.clone()
        },before,after).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn shape_conversion_restores_the_primitive_with_one_undo() {
    let mut app=drawing_app().await;
    let layer=crate::schema::create_drawing_shape_layer_rect("Convert me");
    let id=layer_id(&layer).to_string();
    let before=DrawingSnapshot { id:"conversion-history".into(),layers:vec![layer],..Default::default() };
    load_drawing_fixture(&mut app,&before);
    let mut after=before.clone();
    for mutation in edit_selection::plan(&before,&[id.clone()],"toPath").unwrap() { crate::mutations::apply_drawing_mutation(&mut after,&mutation).unwrap(); }
    artifact_laws::assert_undo_redo_round_trip(&mut *app,DrawingCommand::EditSelection(edit_selection::EditSelection { operation:"toPath".into(),ids:vec![id] }),|app|app.snapshot().unwrap(),before,after).await;
}

#[semio_framework_async_macros::async_test]
async fn utility_registry_declares_all_canvas_utilities_scoped_to_the_window() {
    let definition = create_drawing_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utility_ids, ["selectMarquee", "selectLasso", "selectDirect", "editNodes", "pen", "shapeRect", "shapeEllipse", "shapeLine", "shapePolygon", "booleanCombine", "trace", "transformMove"],);
    let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
    assert_eq!(selects, ["selectMarquee", "selectLasso", "selectDirect", "editNodes"]);
    let scene = definition.window_kinds.iter().find(|window| window.id == DRAWING_PLAY_WINDOW_CANVAS).expect("canvas window");
    assert_eq!(scene.utilities.len(), definition.utilities.len(), "every utility is scoped to the canvas window kind");
    let set_active_utility = definition.actions.iter().find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).expect("setActiveUtility app action");
    assert!(matches!(set_active_utility.kind, ActionKind::View));
    assert!(semio_framework::window_kind_actions(&definition, scene).iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID));
    assert!(!scene.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID), "app action stays canonical instead of being copied into the canvas window");
}

#[semio_framework_async_macros::async_test]
async fn strokes_interaction_domain_enumerates_document_layers_on_the_canvas_window() {
    let definition = create_drawing_app();
    let domain = definition.interactions.iter().find(|interaction| interaction.id == DRAWING_INTERACTION_DOMAIN).expect("strokes interaction domain declared");
    assert!(matches!(domain.hierarchy, HierarchyProvider::Topology));
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
    let (result, receipt) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 110.0, y: 110.0, width: 200.0, height: 200.0, shift: false, ctrl: false, meta: false, cancelled: false }), &meta).await;
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
    let admission = app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "domainId": DRAWING_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" }))), &meta).await.expect("select");
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
    let node = app.render(DRAWING_PLAY_BODY_LAYERS, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    let json = artifact_laws::project_and_retire_fixture_tree(node).expect("retire semantic tree");
    assert!(json.contains("Add Path"));
    assert!(json.contains("Add Rectangle"));
    assert!(!json.contains("Pfad hinzufügen"));
}

#[semio_framework_async_macros::async_test]
async fn drawing_labels_translate_panels_in_german() {
    let mut app = drawing_app().await;
    let view_state = ViewModel { locale: semio_framework_ui_locale::Locale::De, ..ViewModel::new(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Native) };
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

/// 🖱️ A press of `utility` at `world` without modifiers.
fn pointer(utility: &str, world: [f64; 2]) -> canvas_pointer_down::DrawingPointer {
    canvas_pointer_down::DrawingPointer { utility: utility.into(), world, shift: false, alt: false, ctrl: false, meta: false }
}

/// 🧱️ The committed base a release yields against.
fn tool_base(document: &DrawingSnapshot) -> canvas_pointer_down::DrawingToolBase {
    canvas_pointer_down::DrawingToolBase { document: std::sync::Arc::new(document.clone()), operation: None }
}

/// 🛠️ LAW (design §5): a shape drag previews from the tool context, publishes nothing per tick and commits ONE
/// transaction of one `create-layer` under `<appId>#shapeRect`, labelled from the leaf in English and German.
#[semio_framework_async_macros::async_test]
async fn gesture_preview_reflects_live_shape_drag_and_clears_on_commit() {
    let mut session = DrawingSession::new("shapeRect", "preview-seed");
    let document = default_drawing_document("empty", None);
    let down = session.press(pointer("shapeRect", [10.0, 10.0])).expect("press");
    assert!(down.artifact_mutations.is_empty() && down.transaction.is_none(), "pointer-down opens no document operation");
    let preview = session.preview();
    let seq_after_down = preview.sequence;
    assert_eq!(preview.context.start, [10.0, 10.0]);
    assert_eq!(preview.context.cursor, [10.0, 10.0]);

    let moved = session.sample([40.0, 30.0], false, false).expect("move");
    assert!(moved.artifact_mutations.is_empty(), "mid-drag ticks emit zero operations");
    let preview = session.preview();
    assert_eq!(preview.context.cursor, [40.0, 30.0], "preview tracks the live cursor, not the drag start");
    assert!(preview.sequence > seq_after_down, "seq is monotone per tick, for staleness detection on the receiving end");

    let up = session.release("canvasPointerUp", pointer("shapeRect", [40.0, 30.0]), tool_base(&document)).expect("release").expect("a shape release commits");
    assert_eq!(up.artifact_mutations.len(), 1, "pointer-up commits the shape as one real DrawingMutation");
    let transaction = up.transaction.as_ref().expect("the shape commits as one tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.draw.drawing@1/*#editor#shapeRect", "{transaction:?}");

    let DrawingMutation::CreateLayer(created) = &up.artifact_mutations[0] else { panic!("a shape drag creates a layer") };
    let label = <DrawingMutation as protocol::SemanticMutation<DrawingSnapshot>>::label(&up.artifact_mutations[0]);
    let id = layer_id(&created.layer).to_string();
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), format!("Create layer \"{id}\""));
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), format!("Ebene \"{id}\" erstellen"));
    assert_eq!(session.preview().phase, canvas_pointer_down::DrawingGesturePreviewPhase::Idle, "the committed projection is terminal idle");
    assert!(session.tool.at_rest());
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_is_a_pure_read_never_mutating_the_tool_context() {
    let mut session = DrawingSession::new("shapeRect", "");
    session.press(pointer("shapeRect", [1.0, 2.0])).expect("press");
    let context_before = session.tool.context().clone();
    let _ = session.preview();
    let _ = session.preview();
    assert_eq!(session.tool.context(), &context_before, "preview must never mutate the live tool context it reads");
}

/// 🔁️ LAW (design §5): two gestures are two transactions with distinct refs; a cancelled gesture leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn two_gestures_are_two_transactions_and_a_cancel_is_zero_trace() {
    let document = default_drawing_document("empty", None);
    let mut session = DrawingSession::new("shapeRect", "two-gestures");
    let mut transactions = Vec::new();
    for (start, end) in [([0.0, 0.0], [20.0, 10.0]), ([30.0, 30.0], [60.0, 50.0])] {
        session.press(pointer("shapeRect", start)).expect("press");
        session.sample(end, false, false).expect("move");
        let emit = session.release("canvasPointerUp", pointer("shapeRect", end), tool_base(&document)).expect("release").expect("commit");
        transactions.push(emit.transaction.expect("each gesture commits its own transaction"));
    }
    assert_ne!(transactions[0].id, transactions[1].id);
    assert_eq!(transactions[0].tool, transactions[1].tool);
    session.press(pointer("shapeRect", [0.0, 0.0])).expect("press");
    session.sample([40.0, 40.0], false, false).expect("move");
    let cancelled = session.cancel();
    assert!(cancelled.artifact_mutations.is_empty() && cancelled.transaction.is_none() && cancelled.effects.is_empty());
    assert!(session.tool.at_rest() && session.tool.provisional().is_none());
}

//#region 🧵️BatchedSamplesAndCancel
fn session_with(utility: &str) -> (DrawingSession, DrawingSnapshot, NoConfig, semio_framework_plugin::HistoryView) {
    (DrawingSession::new(utility, ""), default_drawing_document("empty", None), NoConfig::default(), semio_framework_plugin::HistoryView::empty())
}

/// 🧵️ LAW (design L4 / §2 D): a batch of four samples leaves the shape drag exactly where four
/// separate moves left it — the cursor follows the LAST sample, not an intermediate one.
#[semio_framework_async_macros::async_test]
async fn a_batched_move_drives_the_gesture_to_its_last_sample() {
    let path = [[420.0, 320.0], [480.0, 300.0], [520.0, 380.0], [460.0, 360.0]];
    let run = |batched: bool| {
        let (mut session, document, config, history) = session_with("shapeRect");
        let view = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        session.press(pointer("shapeRect", [0.0, 0.0])).expect("press");
        if batched {
            let [x, y] = path[3];
            let emit = canvas_pointer_move::handle(&canvas_pointer_move::CanvasPointerMove { shift: false, alt: false,  x, y, width: 800.0, height: 600.0, samples: path.to_vec() }, &view, &cfg, &mut session).expect("batched move");
            assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty(), "a mid-drag batch emits no operation");
        } else {
            for [x, y] in path {
                canvas_pointer_move::handle(&canvas_pointer_move::CanvasPointerMove { shift: false, alt: false,  x, y, width: 800.0, height: 600.0, samples: Vec::new() }, &view, &cfg, &mut session).expect("move");
            }
        }
        session.tool.context().clone()
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
        let view = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        session.press(pointer(utility, [0.0, 0.0])).expect("press");
        canvas_pointer_move::handle(&canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 600.0, y: 500.0, width: 800.0, height: 600.0, samples: Vec::new() }, &view, &cfg, &mut session).expect("move");
        assert!(!session.tool.at_rest(), "{utility}: the drag is live before the cancel");
        let emit = canvas_pointer_up::handle(&canvas_pointer_up::CanvasPointerUp { alt: false, x: 600.0, y: 500.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: true }, &view, &cfg, &mut session).expect("cancel");
        assert!(emit.artifact_mutations.is_empty() && emit.transaction.is_none(), "{utility}: a cancel never commits");
        assert!(emit.effects.is_empty(), "{utility}: a cancel never selects or resets the utility");
        assert!(session.tool.at_rest(), "{utility}: no gesture survives a cancel");
        assert!(session.point_query.is_none(), "{utility}: no marquee/pick query is retained");
    }
    let (mut session, document, config, history) = session_with("selectDirect");
    let view = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = canvas_pointer_up::handle(&canvas_pointer_up::CanvasPointerUp { alt: false, x: 400.0, y: 300.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: true }, &view, &cfg, &mut session).expect("cancel");
    assert!(emit.effects.is_empty() && emit.artifact_mutations.is_empty(), "an idle cancel never falls back to a pick");
    assert!(session.tool.at_rest());
}

/// 🧵️ LAW: a legacy one-per-event wire (no `samples`, no `cancelled`) decodes as one sample at
/// `(x, y)` and a real release.
#[semio_framework_async_macros::async_test]
async fn canvas_pointer_wire_defaults_samples_and_cancelled() {
    use semio_framework_value::FromValue;
    let f = semio_framework_value::DslValue::float;
    let legacy = semio_framework_value::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0)), ("width".into(), f(800.0)), ("height".into(), f(600.0))]);
    let moved = canvas_pointer_move::CanvasPointerMove::from_value(legacy.clone()).expect("legacy move decodes");
    assert!(moved.samples.is_empty());
    assert_eq!(moved.samples_or_last(), vec![[5.0, 6.0]], "an absent `samples` is the single (x, y)");
    assert_eq!(moved.last_sample(), [5.0, 6.0]);
    let pair = |x: f64, y: f64| semio_framework_value::DslValue::Array(vec![f(x), f(y)]);
    let batched = semio_framework_value::DslValue::Object(vec![("x".into(), f(3.0)), ("y".into(), f(4.0)), ("width".into(), f(800.0)), ("height".into(), f(600.0)), ("samples".into(), semio_framework_value::DslValue::Array(vec![pair(1.0, 1.5), pair(2.0, 2.5), pair(3.0, 4.0)]))]);
    let moved = canvas_pointer_move::CanvasPointerMove::from_value(batched).expect("batched move decodes");
    assert_eq!(moved.samples, vec![[1.0, 1.5], [2.0, 2.5], [3.0, 4.0]]);
    assert_eq!(moved.last_sample(), [3.0, 4.0]);
    let modified = canvas_pointer_move::CanvasPointerMove::from_value(semio_framework_value::DslValue::Object(vec![("x".into(),f(3.0)),("y".into(),f(4.0)),("width".into(),f(800.0)),("height".into(),f(600.0)),("shift".into(),semio_framework_value::DslValue::Bool(true)),("alt".into(),semio_framework_value::DslValue::Bool(true))])).expect("live modifiers decode");
    assert!(modified.shift && modified.alt);
    let legacy_up = semio_framework_value::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0)), ("width".into(), f(800.0)), ("height".into(), f(600.0)), ("shift".into(), semio_framework_value::DslValue::Bool(false)), ("ctrl".into(), semio_framework_value::DslValue::Bool(false)), ("meta".into(), semio_framework_value::DslValue::Bool(false))]);
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
        DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 1.0, y: 2.0, width: 800.0, height: 600.0, samples: vec![[0.5, 1.5], [1.0, 2.0]] }),
        DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 1.0, y: 2.0, width: 800.0, height: 600.0, shift: false, ctrl: true, meta: false, cancelled: false }),
        DrawingCommand::CanvasDoubleClick(canvas_double_click::CanvasDoubleClick {}),
        DrawingCommand::CanvasCommitDraft(canvas_commit_draft::CanvasCommitDraft {}),
        DrawingCommand::CanvasEscape(canvas_escape::CanvasEscape {}),
        DrawingCommand::ExportDocument(export_document::ExportDocument { format: "pdf".into() }),
        DrawingCommand::EditSelection(edit_selection::EditSelection { operation: "group".into(), ids: vec!["a".into(), "b".into()] }),
        DrawingCommand::EditPath(edit_path::EditPath { layer_id: "path".into(), edit: Box::new(crate::schema::geometry::editing::PathEdit::Reverse) }),
        DrawingCommand::EditFill(edit_fill::EditFill { layer_id: "a".into(), edit: Box::new(crate::schema::fill::FillEdit::Type { value: crate::schema::fill::FillType::LinearGradient }) }),
        DrawingCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        DrawingCommand::NudgeSelectionLeft(nudge_selection_left::NudgeSelectionLeft {}),
        DrawingCommand::NudgeSelectionLeftFast(nudge_selection_left_fast::NudgeSelectionLeftFast {}),
        DrawingCommand::NudgeSelectionRight(nudge_selection_right::NudgeSelectionRight {}),
        DrawingCommand::NudgeSelectionRightFast(nudge_selection_right_fast::NudgeSelectionRightFast {}),
        DrawingCommand::NudgeSelectionUp(nudge_selection_up::NudgeSelectionUp {}),
        DrawingCommand::NudgeSelectionUpFast(nudge_selection_up_fast::NudgeSelectionUpFast {}),
        DrawingCommand::NudgeSelectionDown(nudge_selection_down::NudgeSelectionDown {}),
        DrawingCommand::NudgeSelectionDownFast(nudge_selection_down_fast::NudgeSelectionDownFast {}),
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
        "edit-selection",
        "edit-path",
        "edit-fill",
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
    assert_eq!(DRAWING_BOUNDED_TOOL_IDS.len(), 31);
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

#[semio_framework_async_macros::async_test]
async fn selected_group_and_layer_drag_preserves_selection_and_one_history_edit() {
    for cancelled in [true,false] {
        let (mut app,mut meta)=inline_selection_app().await;
        meta.view_state.as_mut().unwrap().active_utility_id=Some("selectDirect".into());
        let mut child=crate::schema::create_drawing_shape_layer_rect("Child");
        crate::schema::layer_base_mut(&mut child).id="child".into();
        crate::schema::layer_base_mut(&mut child).attributes.fill=Some(crate::FillStyle::Solid {color:[1.0,0.0,0.0,1.0]});
        let mut group=crate::schema::create_drawing_group_layer("Group");
        crate::schema::layer_base_mut(&mut group).id="group".into();
        crate::schema::layer_base_mut(&mut group).transform.scale_x=2.0;
        if let DrawingLayerNode::Group(body)=&mut group { body.children.push(child); }
        let mut other=crate::schema::create_drawing_shape_layer_rect("Other");
        crate::schema::layer_base_mut(&mut other).id="other".into();
        crate::schema::layer_base_mut(&mut other).attributes.fill=Some(crate::FillStyle::Solid {color:[0.0,0.0,1.0,1.0]});
        crate::schema::layer_base_mut(&mut other).transform.x=400.0;
        let before=DrawingSnapshot { id:"selection-drag".into(),layers:vec![group,other],..Default::default() };
        load_drawing_fixture(&mut app,&before);
        settled(&mut app,DrawingCommand::SetCamera(set_camera::SetCamera { camera:store::Viewport2d { x:0.0,y:0.0,zoom:1.0 } }),&meta).await;
        let ids=vec!["group".to_string(),"child".to_string(),"other".to_string()];
        let targets=serde_json::to_string(&ids.iter().map(|id|serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":id})).collect::<Vec<_>>()).unwrap();
        let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))),&meta).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();
        let selected=selected_strokes(&app).await;
        assert_eq!(selected.len(),3);
        settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x:420.0,y:320.0,width:800.0,height:600.0,..Default::default() }),&meta).await;
        assert_eq!(selected_strokes(&app).await,selected);
        settled(&mut app,DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x:450.0,y:340.0,width:800.0,height:600.0,samples:vec![] }),&meta).await;
        assert_eq!(app.snapshot().unwrap(),before);
        let scene=canvas_scene(rendered_drawing_canvas(&mut app,None,meta.view_state.as_ref().unwrap()).await.unwrap());
        let records:Vec<serde_json::Value>=serde_json::from_str(&scene.layers_json).unwrap();
        for (id,x,scale) in [("child",30.0,2.0),("other",430.0,1.0)] {
            let node=records.iter().find(|node|node["id"]==id).unwrap();
            assert_eq!(node["transform"][0].as_f64(),Some(scale),"interior drag preserves scale");
            assert!((node["transform"][4].as_f64().unwrap()-x).abs()<1e-10);
            assert!((node["transform"][5].as_f64().unwrap()-20.0).abs()<1e-10);
        }
        let (_,receipt)=settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x:450.0,y:340.0,width:800.0,height:600.0,shift:false,ctrl:false,meta:false,cancelled }),&meta).await;
        if cancelled {
            assert_eq!(app.snapshot().unwrap(),before);
            assert!(app.history_snapshot().await.expect("history").upserts.iter().all(|row|row.transaction.is_none()),"a cancelled drag leaves zero trace");
        } else {
            assert_one_artifact_publication(&receipt);
            let rows=app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row|row.transaction.is_some()).collect::<Vec<_>>();
            assert_eq!(rows.len(),1,"one drag is one transaction row: {rows:?}");
            let transaction=rows[0].transaction.as_ref().unwrap();
            assert!(transaction.id.starts_with("tx-") && transaction.tool=="s.draw.drawing@1/*#editor#selectDirect","{transaction:?}");
            assert!(rows[0].op_lines.len()==1 && rows[0].op_lines[0].starts_with("drag-layers"),"one relative drag leaf: {:?}",rows[0].op_lines);
            assert_eq!(rows[0].label.resolve(semio_framework_ui_locale::Terminology::Native,semio_framework_ui_locale::Locale::En),"Drag 2 layers by (30, 20)");
            assert_eq!(rows[0].label.resolve(semio_framework_ui_locale::Terminology::Native,semio_framework_ui_locale::Locale::De),"2 Ebenen um (30; 20) ziehen");
            let after=app.snapshot().unwrap();
            assert!((crate::schema::layer_base(&after.layers[0]).transform.x-30.0).abs()<1e-10);
            assert!((crate::schema::layer_base(&after.layers[1]).transform.x-430.0).abs()<1e-10);
            assert_eq!(crate::schema::layer_base(crate::schema::find_drawing_layer(&after,"child").unwrap()).transform.x,0.0);
            artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id).await;
            assert_eq!(app.snapshot().unwrap(),before);
            artifact_laws::settle_history_verb(&mut *app,"redo",meta.instance_id).await;
            assert_eq!(app.snapshot().unwrap(),after);
        }
        assert_eq!(selected_strokes(&app).await,selected);
    }
}

#[semio_framework_async_macros::async_test]
async fn created_layers_are_painted_and_selected_for_immediate_editing() {
    let (mut app,meta)=inline_selection_app().await;
    for kind in ["shape:rect","shape:ellipse","shape:line","shape:polygon","text"] {
        settled(&mut app,DrawingCommand::AddLayer(add_layer::AddLayer {kind:kind.into()}),&meta).await;
        let snapshot=app.snapshot().unwrap();
        let layer=snapshot.layers.last().unwrap();
        let base=crate::schema::layer_base(layer);
        assert_eq!(selected_strokes(&app).await,vec![base.id.clone()]);
        let scene=crate::schema::flatten_drawing_document_to_scene_nodes(&snapshot);
        let node=scene.iter().find(|node|node.id==base.id).unwrap();
        assert!(node.fill.is_some() || node.stroke.as_ref().is_some_and(|stroke|stroke.width>0.0));
    }
}

#[test]
fn drawing_canvas_initial_framing_uses_world_bounds_and_respects_restored_navigation() {
    let layer = crate::schema::create_drawing_path_layer("Off origin",vec![crate::PathSegment::Move { to: [-20.0,-30.0] },crate::PathSegment::Line { to: [80.0,70.0] }]);
    let mut document = crate::DrawingSnapshot { layers: vec![layer],artboard: None,..Default::default() };
    let mut config = canvas_window::config::DrawingCanvasWindowConfig::default();
    let preview = DrawingGesturePreview::default();
    let mut producer=crate::schema::scene_preparation::DocumentVectorJob::new(&document,geometry_session::limits(),geometry_session::algorithms()).unwrap();
    while !producer.advance(4096).unwrap().done{}
    let plan=producer.result().unwrap();
    drop(producer);
    let scene = canvas_scene(semio_framework_plugin::built_to_component_tree(canvas_window::render(Some(&plan),1,&document,&config,&preview,DRAWING_DEFAULT_UTILITY,&[],&[]).unwrap()));
    assert_eq!(scene.framing.as_ref().unwrap().bounds,[-20.0,-30.0,80.0,70.0]);
    document.artboard = Some(crate::schema::DrawingArtboard { width: 1024.0,height: 1024.0 });
    let scene = canvas_scene(semio_framework_plugin::built_to_component_tree(canvas_window::render(Some(&plan),1,&document,&config,&preview,DRAWING_DEFAULT_UTILITY,&[],&[]).unwrap()));
    assert_eq!(scene.framing.as_ref().unwrap().bounds,[-20.0,-30.0,1024.0,1024.0]);
    config.framed = true;
    config.viewport = store::Viewport2d { x: 777.0,y: -333.0,zoom: 2.0 };
    let scene = canvas_scene(semio_framework_plugin::built_to_component_tree(canvas_window::render(Some(&plan),1,&document,&config,&preview,DRAWING_DEFAULT_UTILITY,&[],&[]).unwrap()));
    assert!(scene.framing.is_none());
    assert_eq!((scene.camera_x,scene.camera_y,scene.zoom),(777.0,-333.0,2.0));
}

#[semio_framework_async_macros::async_test]
async fn text_content_and_size_each_undo_as_one_selection_edit() {
    for (field, value) in [("textContent", "Grüße 🌍\n123"), ("textSize", "36")] {
        let mut app = drawing_app().await;
        let mut first = crate::schema::create_drawing_text_layer("First");
        let mut second = crate::schema::create_drawing_text_layer("Second");
        crate::schema::layer_base_mut(&mut first).id = "first".into();
        crate::schema::layer_base_mut(&mut second).id = "second".into();
        let DrawingLayerNode::Text(text) = &mut second else { unreachable!() };
        text.content = "Different".into();
        text.size = 18.0;
        let before = DrawingSnapshot { id: "text-history".into(), layers: vec![first, second], ..Default::default() };
        let mut after = before.clone();
        for layer in &mut after.layers {
            let DrawingLayerNode::Text(text) = layer else { unreachable!() };
            if field == "textContent" { text.content = value.into(); } else { text.size = 36.0; }
        }
        load_drawing_fixture(&mut app, &before);
        artifact_laws::assert_undo_redo_round_trip(&mut *app, DrawingCommand::PatchLayers(patch_layers::PatchLayers { layer_ids: vec!["first".into(), "second".into()], field: field.into(), value: value.into() }), |app| app.snapshot().unwrap(), before, after).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn select_all_discovers_unvisited_layers_and_prunes_deleted_selection() {
    let (mut app, meta) = inline_selection_app().await;
    let first = crate::schema::create_drawing_shape_layer_rect("First");
    let second = crate::schema::create_drawing_text_layer("Second");
    let ids = vec![layer_id(&first).to_string(), layer_id(&second).to_string()];
    let snapshot = DrawingSnapshot { id: "select-all-topology".into(), layers: vec![first, second], ..Default::default() };
    load_drawing_fixture(&mut app, &snapshot);
    let admission=app.handle_action("selectAll", None, &meta).await.expect("Select All admission");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.expect("Select All publication");
    assert_eq!(selected_strokes(&app).await, ids);
    assert_eq!(app.snapshot().unwrap(), snapshot, "selection must not edit the drawing");
    settled(&mut app, DrawingCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: ids[0].clone() }), &meta).await;
    assert_eq!(selected_strokes(&app).await, vec![ids[1].clone()]);
    settled(&mut app, DrawingCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: ids[1].clone() }), &meta).await;
    assert!(selected_strokes(&app).await.is_empty());
    let admission=app.handle_action("selectAll", None, &meta).await.expect("empty Select All admission");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.expect("empty Select All publication");
    assert!(selected_strokes(&app).await.is_empty());
}


#[semio_framework_async_macros::async_test]
async fn transform_handles_render_and_commit_once_through_the_registered_editor() {
    for handle in [4_usize,8_usize] {
        for (cancelled,shift,alt) in [(false,false,false),(true,false,false),(false,true,true),(true,true,true)] {
            let (mut app,mut meta)=inline_selection_app().await;
            meta.view_state.as_mut().unwrap().active_utility_id=Some("selectDirect".into());
            let mut child=crate::schema::create_drawing_shape_layer_rect("Transform target");
            crate::schema::layer_base_mut(&mut child).id="child".into();
            let mut parent=crate::schema::create_drawing_group_layer("Affine parent");
            crate::schema::layer_base_mut(&mut parent).transform=crate::DrawingTransform {x:20.0,y:30.0,scale_x:2.0,scale_y:3.0,rotation:0.3,shear:0.5};
            let DrawingLayerNode::Group(group)=&mut parent else {unreachable!()};
            group.children.push(child);
            let before=DrawingSnapshot {id:"handle-gesture".into(),layers:vec![parent],..Default::default()};
            load_drawing_fixture(&mut app,&before);
            settled(&mut app,DrawingCommand::SetCamera(set_camera::SetCamera {camera:store::Viewport2d {x:0.0,y:0.0,zoom:1.0}}),&meta).await;
            let targets=serde_json::to_string(&[serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":"child"})]).unwrap();
            let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))),&meta).await.unwrap();
            semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();
            let scene=canvas_scene(rendered_drawing_canvas(&mut app,None,meta.view_state.as_ref().unwrap()).await.unwrap());
            let records:Vec<serde_json::Value>=serde_json::from_str(&scene.layers_json).unwrap();
            assert_eq!(records.iter().filter(|row|row["id"].as_str().is_some_and(|id|id.starts_with("overlay:transform-handle:"))).count(),9);
            let bounds=canvas_pointer_down::selected_transform_bounds(&before,&["child".into()]).unwrap();
            let start=crate::schema::geometry::handles::handle_points(bounds,1.0)[handle];
            let center=[bounds[0]+bounds[2]*0.5,bounds[1]+bounds[3]*0.5];
            let end=if handle==8 {[center[0]-(start[1]-center[1]),center[1]+start[0]-center[0]]} else {[start[0]+30.0,start[1]+20.0]};
            let expected=crate::schema::geometry::handles::handle_matrix(handle,bounds,start,end,shift,alt).unwrap();
            settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x:start[0]+400.0,y:start[1]+300.0,width:800.0,height:600.0,..Default::default()}),&meta).await;
            settled(&mut app,DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {shift:!shift,alt:!alt,x:end[0]+400.0,y:end[1]+300.0,width:800.0,height:600.0,samples:vec![]}),&meta).await;
            let (_,moved)=settled(&mut app,DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift, alt, x:end[0]+400.0,y:end[1]+300.0,width:800.0,height:600.0,samples:vec![]}),&meta).await;
            assert!(!moved.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact));
            assert_eq!(app.snapshot().unwrap(),before);
            let (_,released)=settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt, x:end[0]+400.0,y:end[1]+300.0,width:800.0,height:600.0,shift,ctrl:false,meta:false,cancelled}),&meta).await;
            if cancelled {assert_eq!(app.snapshot().unwrap(),before);} else {
                assert_one_artifact_publication(&released);
                let after=app.snapshot().unwrap();
                let original=crate::schema::flatten_drawing_document_to_scene_nodes(&before)[0].transform;
                let actual=crate::schema::flatten_drawing_document_to_scene_nodes(&after)[0].transform;
                let wanted=crate::schema::geometry::multiply(expected,original);
                for index in 0..6 {assert!((actual[index]-wanted[index]).abs()<1e-9);}
                artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id).await;
                assert_eq!(app.snapshot().unwrap(),before);
                artifact_laws::settle_history_verb(&mut *app,"redo",meta.instance_id).await;
                assert_eq!(app.snapshot().unwrap(),after);
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn node_drag_previews_without_mutation_and_commits_one_undoable_edit() {
    for cancelled in [true,false] {
        let (mut app,mut meta)=inline_selection_app().await;
        meta.view_state.as_mut().unwrap().active_utility_id=Some("editNodes".into());
        let source=vec![crate::PathSegment::Move {to:[0.0,0.0]},crate::PathSegment::Cubic {ctrl1:[10.0,20.0],ctrl2:[30.0,20.0],to:[40.0,0.0]}];
        let mut path=crate::schema::create_drawing_path_layer("Curve",source.clone());
        crate::schema::layer_base_mut(&mut path).id="curve".into();
        crate::schema::layer_base_mut(&mut path).transform.scale_x=2.0;
        let before=DrawingSnapshot {id:"node-drag".into(),layers:vec![path],..Default::default()};
        load_drawing_fixture(&mut app,&before);
        settled(&mut app,DrawingCommand::SetCamera(set_camera::SetCamera {camera:store::Viewport2d {x:0.0,y:0.0,zoom:1.0}}),&meta).await;
        let targets=serde_json::to_string(&vec![serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":"curve"})]).unwrap();
        let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))),&meta).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();
        artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id).await.unwrap();
        settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x:421.0,y:321.0,width:800.0,height:600.0,..Default::default()}),&meta).await;
        let selected=app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.clone();
        assert_eq!(selected.len(),1);
        let selected_point=interaction::points::parse_point_id(&selected[0]).unwrap();
        assert_eq!((selected_point.layer_id,selected_point.index,selected_point.point),("curve",1,crate::schema::geometry::editing::PathPoint::Control1));
        assert_eq!(selected_point.geometry,interaction::points::geometry_id(&source).unwrap());
        settled(&mut app,DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {x:441.0,y:331.0,width:800.0,height:600.0,shift:false,alt:false,samples:vec![]}),&meta).await;
        assert_eq!(app.snapshot().unwrap(),before);
        let scene=canvas_scene(rendered_drawing_canvas(&mut app,None,meta.view_state.as_ref().unwrap()).await.unwrap());
        let records:Vec<serde_json::Value>=serde_json::from_str(&scene.layers_json).unwrap();
        let node=records.iter().find(|node|node["id"]=="curve").unwrap();
        assert_eq!(node["segments"][1]["ctrl1"],serde_json::json!([20.0,30.0]));
        assert!(records.iter().any(|node|node["id"].as_str().is_some_and(|id|id.starts_with("overlay:node:"))));
        assert!(records.iter().any(|node|node["id"]=="overlay:node:curve:selected-controls" && node["segments"].as_array().is_some_and(|segments|!segments.is_empty())));
        let (_,receipt)=settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {x:441.0,y:331.0,width:800.0,height:600.0,shift:false,alt:false,ctrl:false,meta:false,cancelled}),&meta).await;
        if cancelled {assert_eq!(app.snapshot().unwrap(),before);assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids,selected);} else {
            assert_one_artifact_publication(&receipt);
            let after=app.snapshot().unwrap();
            let DrawingLayerNode::Path(path)=&after.layers[0] else {panic!("Expected path")};
            let rebound=app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.clone();
            assert_eq!(rebound.len(),1);
            assert_ne!(rebound,selected);
            assert_eq!(interaction::points::parse_point_id(&rebound[0]).unwrap().geometry,interaction::points::geometry_id(&path.segments).unwrap());
            assert_eq!(path.segments[1],crate::PathSegment::Cubic {ctrl1:[20.0,30.0],ctrl2:[30.0,20.0],to:[40.0,0.0]});
            artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id).await;assert_eq!(app.snapshot().unwrap(),before);
            artifact_laws::settle_history_verb(&mut *app,"redo",meta.instance_id).await;assert_eq!(app.snapshot().unwrap(),after);
        }
        assert_eq!(selected_strokes(&app).await,vec!["curve".to_string()]);
    }
}

#[semio_framework_async_macros::async_test]
async fn point_click_persists_and_local_position_rebinds_but_topology_edit_prunes() {
    use crate::schema::geometry::editing::{PathEdit,PathPoint,PathAxis};
    let (mut app,mut meta)=inline_selection_app().await;
    meta.view_state.as_mut().unwrap().active_utility_id=Some("editNodes".into());
    let mut layer=crate::schema::create_drawing_path_layer("Line",vec![crate::PathSegment::Move {to:[0.0,0.0]},crate::PathSegment::Line {to:[10.0,0.0]}]);
    crate::schema::layer_base_mut(&mut layer).id="path".into();
    let before=DrawingSnapshot {id:"point-selection".into(),layers:vec![layer],..Default::default()};
    load_drawing_fixture(&mut app,&before);
    settled(&mut app,DrawingCommand::SetCamera(set_camera::SetCamera {camera:store::Viewport2d {x:0.0,y:0.0,zoom:1.0}}),&meta).await;
    let targets=serde_json::to_string(&vec![serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":"path"})]).unwrap();
    let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))),&meta).await.unwrap();
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();
    artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id).await.unwrap();
    settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x:410.0,y:300.0,width:800.0,height:600.0,..Default::default()}),&meta).await;
    settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {x:410.0,y:300.0,width:800.0,height:600.0,shift:false,alt:false,ctrl:false,meta:false,cancelled:false}),&meta).await;
    assert_eq!(app.snapshot().unwrap(),before);
    let selected=app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.clone();
    assert_eq!(selected.len(),1);
    let point=interaction::points::parse_point_id(&selected[0]).unwrap();
    assert_eq!((point.layer_id,point.index,point.point),("path",1,PathPoint::Anchor));
    settled(&mut app,DrawingCommand::EditPath(edit_path::EditPath {layer_id:"path".into(),edit:Box::new(PathEdit::Coordinate {index:1,point:PathPoint::Anchor,axis:PathAxis::X,value:15.0})}),&meta).await;
    let rebound=app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.clone();
    assert_eq!(rebound.len(),1);assert_ne!(selected,rebound);
    let point=interaction::points::parse_point_id(&rebound[0]).unwrap();
    assert_eq!((point.layer_id,point.index,point.point),("path",1,PathPoint::Anchor));
    settled(&mut app,DrawingCommand::EditPath(edit_path::EditPath {layer_id:"path".into(),edit:Box::new(PathEdit::Reverse)}),&meta).await;
    assert!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.is_empty());
    assert_eq!(selected_strokes(&app).await,vec!["path".to_owned()]);
}

#[semio_framework_async_macros::async_test]
async fn every_keyboard_nudge_moves_document_axes_and_undoes_once() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🎮️commands/🕹️nudge-selection/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture.as_array().unwrap() {
        let (mut app,meta)=inline_selection_app().await;
        let mut layer=crate::schema::create_layer_by_kind("shape:rect");
        crate::schema::layer_base_mut(&mut layer).id="shape".into();
        let before=DrawingSnapshot {id:"nudge".into(),layers:vec![layer],..Default::default()};
        load_drawing_fixture(&mut app,&before);
        let targets=serde_json::to_string(&[serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":"shape"})]).unwrap();
        let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))),&meta).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();
        artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id).await.unwrap();
        let command=args_bridge::command_from_action(row["action"].as_str().unwrap(),None).unwrap();
        store::os_store::test_support::assert_op_line_round_trip(&command);
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        let (_,receipt)=settled(&mut app,command,&meta).await;
        assert_one_artifact_publication(&receipt);
        let after=app.snapshot().unwrap();
        let transform=&crate::schema::layer_base(&after.layers[0]).transform;
        assert_eq!([transform.x,transform.y],serde_json::from_value::<[f64;2]>(row["delta"].clone()).unwrap());
        artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id).await;
        assert_eq!(app.snapshot().unwrap(),before);
        artifact_laws::settle_history_verb(&mut *app,"redo",meta.instance_id).await;
        assert_eq!(app.snapshot().unwrap(),after);
    }
}

#[semio_framework_async_macros::async_test]
async fn node_keyboard_nudges_rebind_selection_and_use_parent_axes() {
    let (mut app,mut meta)=inline_selection_app().await;
    meta.view_state.as_mut().unwrap().active_utility_id=Some("editNodes".into());
    let source=vec![crate::PathSegment::Move {to:[0.0,0.0]},crate::PathSegment::Cubic {ctrl1:[2.0,0.0],ctrl2:[8.0,0.0],to:[10.0,0.0]}];
    let mut path=crate::schema::create_drawing_path_layer("Curve",source.clone());
    crate::schema::layer_base_mut(&mut path).id="path".into();
    let mut group=crate::schema::create_layer_by_kind("group");
    if let DrawingLayerNode::Group(group)=&mut group {
        group.base.id="group".into();group.base.transform.rotation=std::f64::consts::FRAC_PI_2;group.base.transform.scale_x=2.0;group.base.transform.scale_y=4.0;group.base.transform.x=100.0;group.base.transform.y=200.0;group.children=vec![path];
    }
    let before=DrawingSnapshot {id:"point-nudge".into(),layers:vec![group],..Default::default()};
    load_drawing_fixture(&mut app,&before);
    let point=interaction::points::point_id("path",&interaction::points::geometry_id(&source).unwrap(),1,crate::schema::geometry::editing::PathPoint::Anchor).unwrap();
    for (domain,granularity,id) in [(DRAWING_INTERACTION_DOMAIN,DRAWING_INTERACTION_GRANULARITY,"path".to_owned()),(DRAWING_POINT_DOMAIN,DRAWING_POINT_GRANULARITY,point)] {
        let targets=serde_json::to_string(&[serde_json::json!({"granularity":granularity,"id":id})]).unwrap();
        let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":domain,"targets":targets,"merge":"replace","method":"pick"}))),&meta).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();
        artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id).await.unwrap();
    }
    for step in 1..=2 {
        let (_,receipt)=settled(&mut app,DrawingCommand::NudgeSelectionRightFast(nudge_selection_right_fast::NudgeSelectionRightFast {}),&meta).await;
        assert_one_artifact_publication(&receipt);
        let after=app.snapshot().unwrap();
        let Some(DrawingLayerNode::Path(path))=crate::schema::find_drawing_layer(&after,"path") else {panic!("Path missing")};
        let crate::PathSegment::Cubic {ctrl1,ctrl2,to}=path.segments[1] else {panic!("Cubic missing")};
        assert_eq!(ctrl1,[2.0,0.0]);
        assert!((to[0]-10.0).abs()<1e-10 && (to[1]+2.5*f64::from(step)).abs()<1e-10);
        assert!((ctrl2[0]-8.0).abs()<1e-10 && (ctrl2[1]-to[1]).abs()<1e-10);
        let selection=app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.clone();
        assert_eq!(selection.len(),1);
        assert_eq!(interaction::points::parse_point_id(&selection[0]).unwrap().geometry,interaction::points::geometry_id(&path.segments).unwrap());
    }
    artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id).await;
    artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(),before);
}

#[semio_framework_async_macros::async_test]
async fn modified_node_picks_and_combined_drag_preserve_layers_and_one_history_edit() {
    for cancelled in [true,false] {
        let (mut app,mut meta)=inline_selection_app().await;
        meta.view_state.as_mut().unwrap().active_utility_id=Some("editNodes".into());
        let segments=vec![crate::PathSegment::Move {to:[0.0,0.0]},crate::PathSegment::Line {to:[10.0,0.0]}];
        let mut first=crate::schema::create_drawing_path_layer("First",segments.clone());
        crate::schema::layer_base_mut(&mut first).id="first".into();
        let mut second=crate::schema::create_drawing_path_layer("Second",segments);
        let base=crate::schema::layer_base_mut(&mut second);base.id="second".into();base.transform.x=40.0;base.transform.scale_x=2.0;base.transform.scale_y=2.0;
        let before=DrawingSnapshot {id:"multi-node-drag".into(),layers:vec![first,second],..Default::default()};
        load_drawing_fixture(&mut app,&before);
        settled(&mut app,DrawingCommand::SetCamera(set_camera::SetCamera {camera:store::Viewport2d {x:0.0,y:0.0,zoom:1.0}}),&meta).await;
        let targets=serde_json::to_string(&[serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":"first"}),serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":"second"})]).unwrap();
        let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))),&meta).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();
        artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id).await.unwrap();
        for (x,y,shift,count) in [(410.0,300.0,false,1),(460.0,300.0,true,2),(700.0,500.0,true,2),(460.0,300.0,true,1),(460.0,300.0,true,2)] {
            settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x,y,shift,width:800.0,height:600.0,..Default::default()}),&meta).await;
            settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {x,y,shift,width:800.0,height:600.0,alt:false,ctrl:false,meta:false,cancelled:false}),&meta).await;
            assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.len(),count);
            assert_eq!(selected_strokes(&app).await,vec!["first".to_owned(),"second".to_owned()]);
            assert_eq!(app.snapshot().unwrap(),before);
        }
        let selected=app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.clone();
        settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x:410.0,y:300.0,width:800.0,height:600.0,..Default::default()}),&meta).await;
        assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids,selected);
        settled(&mut app,DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {x:420.0,y:310.0,width:800.0,height:600.0,shift:false,alt:false,samples:vec![]}),&meta).await;
        assert_eq!(app.snapshot().unwrap(),before);
        let scene=canvas_scene(rendered_drawing_canvas(&mut app,None,meta.view_state.as_ref().unwrap()).await.unwrap());
        let records:Vec<serde_json::Value>=serde_json::from_str(&scene.layers_json).unwrap();
        assert_eq!(records.iter().find(|row|row["id"]=="first").unwrap()["segments"][1]["to"],serde_json::json!([20.0,10.0]));
        assert_eq!(records.iter().find(|row|row["id"]=="second").unwrap()["segments"][1]["to"],serde_json::json!([15.0,5.0]));
        let (_,receipt)=settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {x:420.0,y:310.0,width:800.0,height:600.0,shift:false,alt:false,ctrl:false,meta:false,cancelled}),&meta).await;
        if cancelled {assert_eq!(app.snapshot().unwrap(),before);assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids,selected);} else {
            assert_one_artifact_publication(&receipt);
            let after=app.snapshot().unwrap();assert_ne!(after,before);
            assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.len(),2);
            artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id).await;assert_eq!(app.snapshot().unwrap(),before);
            artifact_laws::settle_history_verb(&mut *app,"redo",meta.instance_id).await;assert_eq!(app.snapshot().unwrap(),after);
        }
        settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x:700.0,y:500.0,width:800.0,height:600.0,..Default::default()}),&meta).await;
        settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {x:700.0,y:500.0,width:800.0,height:600.0,shift:false,alt:false,ctrl:false,meta:false,cancelled:false}),&meta).await;
        assert!(app.interaction_state().await.selection.get(DRAWING_POINT_DOMAIN).is_none_or(|selection|selection.ids.is_empty()));
        assert_eq!(selected_strokes(&app).await,vec!["first".to_owned(),"second".to_owned()]);
    }
}

#[semio_framework_async_macros::async_test]
async fn fill_rule_selection_edit_undoes_as_one_history_entry() {
    let mut app=drawing_app().await;
    let before:DrawingSnapshot=serde_json::from_str(include_str!("../../../../🎨️style/🧫️fixtures/🧬️mutations/🌀️set-layer-fill-rule/🌀️evenodd/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let mut before=before;
    let mut second=before.layers[0].clone();
    crate::schema::layer_base_mut(&mut second).id="shape-b".into();
    before.layers.push(second);
    let mut after=before.clone();
    for layer in &mut after.layers {crate::schema::layer_base_mut(layer).attributes.fill_rule=crate::FillRule::Nonzero;}
    let ids=before.layers.iter().map(|layer|crate::schema::layer_id(layer).to_string()).collect();
    load_drawing_fixture(&mut app,&before);
    artifact_laws::assert_undo_redo_round_trip(&mut *app,DrawingCommand::PatchLayers(patch_layers::PatchLayers {layer_ids:ids,field:"fillRule".into(),value:"nonzero".into()}),|app|app.snapshot().unwrap(),before,after).await;
}

#[semio_framework_async_macros::async_test]
async fn alignment_across_groups_undoes_as_one_history_entry() {
    let mut app=drawing_app().await;
    let mut first=crate::schema::create_drawing_shape_layer_rect("First");
    crate::schema::layer_base_mut(&mut first).id="first".into();
    let mut second=crate::schema::create_drawing_shape_layer_rect("Second");
    let base=crate::schema::layer_base_mut(&mut second);base.id="second".into();base.transform.x=20.0;base.transform.y=20.0;
    let mut group=crate::schema::create_drawing_group_layer("Group");
    let DrawingLayerNode::Group(body)=&mut group else {unreachable!()};
    body.base.id="group".into();body.base.transform.scale_x=2.0;body.base.transform.scale_y=3.0;body.children=vec![first,second];
    let mut third=crate::schema::create_drawing_shape_layer_rect("Third");
    let base=crate::schema::layer_base_mut(&mut third);base.id="third".into();base.transform.x=-20.0;
    let before=DrawingSnapshot {id:"alignment-history".into(),layers:vec![group,third],..Default::default()};
    let mut after=before.clone();
    let DrawingLayerNode::Group(body)=&mut after.layers[0] else {unreachable!()};
    for layer in &mut body.children {crate::schema::layer_base_mut(layer).transform.x=-10.0;}
    load_drawing_fixture(&mut app,&before);
    artifact_laws::assert_undo_redo_round_trip(&mut *app,DrawingCommand::EditSelection(edit_selection::EditSelection {ids:vec!["first".into(),"second".into(),"third".into()],operation:"alignLeft".into()}),|app|app.snapshot().unwrap(),before,after).await;
}

#[semio_framework_async_macros::async_test]
async fn layer_stack_steps_undo_as_one_history_entry() {
    for (operation,order) in [("bringForward",["a","d","b","c"]),("sendBackward",["b","c","a","d"])] {
        let mut app=drawing_app().await;
        let layers=["a","b","c","d"].into_iter().map(|id| {
            let mut layer=crate::schema::create_drawing_shape_layer_rect(id);
            crate::schema::layer_base_mut(&mut layer).id=id.into();layer
        }).collect::<Vec<_>>();
        let before=DrawingSnapshot {id:"stack-history".into(),layers,..Default::default()};
        let after=DrawingSnapshot {layers:order.into_iter().map(|id|before.layers.iter().find(|layer|crate::schema::layer_id(layer)==id).unwrap().clone()).collect(),..before.clone()};
        load_drawing_fixture(&mut app,&before);
        artifact_laws::assert_undo_redo_round_trip(&mut *app,DrawingCommand::EditSelection(edit_selection::EditSelection {ids:vec!["c".into(),"b".into()],operation:operation.into()}),|app|app.snapshot().unwrap(),before,after).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn ungroup_selects_promoted_children_and_undoes_as_one_history_entry() {
    let (mut app,meta)=inline_selection_app().await;
    let mut first=crate::schema::create_drawing_shape_layer_rect("First");
    let base=crate::schema::layer_base_mut(&mut first);base.id="first".into();base.transform.x=2.0;
    let mut second=crate::schema::create_drawing_shape_layer_rect("Second");
    crate::schema::layer_base_mut(&mut second).id="second".into();
    let mut group=crate::schema::create_drawing_group_layer("Group");
    let DrawingLayerNode::Group(body)=&mut group else {unreachable!()};
    body.base.id="group".into();body.base.transform.x=5.0;body.children=vec![first.clone(),second.clone()];
    let before=DrawingSnapshot {id:"ungroup-history".into(),layers:vec![group],..Default::default()};
    crate::schema::layer_base_mut(&mut first).transform.x=7.0;
    crate::schema::layer_base_mut(&mut second).transform.x=5.0;
    let after=DrawingSnapshot {layers:vec![first,second],..before.clone()};
    load_drawing_fixture(&mut app,&before);
    let (_,receipt)=settled(&mut app,DrawingCommand::EditSelection(edit_selection::EditSelection {ids:vec!["group".into()],operation:"ungroup".into()}),&meta).await;
    assert_one_artifact_publication(&receipt);
    assert_eq!(app.snapshot().unwrap(),after);
    assert_eq!(selected_strokes(&app).await,vec!["first".to_owned(),"second".to_owned()]);
    artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id).await;assert_eq!(app.snapshot().unwrap(),before);
    artifact_laws::settle_history_verb(&mut *app,"redo",meta.instance_id).await;assert_eq!(app.snapshot().unwrap(),after);
}

#[semio_framework_async_macros::async_test]
async fn group_isolation_selection_edit_undoes_as_one_history_entry() {
    let mut app=drawing_app().await;
    let before:DrawingSnapshot=serde_json::from_str(include_str!("../../../../🎨️style/🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let after:DrawingSnapshot=serde_json::from_str(include_str!("../../../../🎨️style/🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/📸️snapshot/➡️after/🔣️.json")).unwrap();
    load_drawing_fixture(&mut app,&before);
    artifact_laws::assert_undo_redo_round_trip(&mut *app,DrawingCommand::PatchLayers(patch_layers::PatchLayers {layer_ids:vec!["group-a".into()],field:"isolation".into(),value:"true".into()}),|app|app.snapshot().unwrap(),before,after).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_selected_nodes_commits_once_clears_points_and_undoes_exactly() {
    let (mut app,mut meta)=inline_selection_app().await;
    meta.view_state.as_mut().unwrap().active_utility_id=Some("editNodes".into());
    let segments=vec![crate::PathSegment::Move {to:[0.0,0.0]},crate::PathSegment::Line {to:[10.0,0.0]},crate::PathSegment::Line {to:[10.0,10.0]}];
    let mut path=crate::schema::create_drawing_path_layer("Path",segments.clone());
    crate::schema::layer_base_mut(&mut path).id="path".into();
    let before=DrawingSnapshot {id:"node-delete-history".into(),layers:vec![path],..Default::default()};
    load_drawing_fixture(&mut app,&before);
    let geometry=interaction::points::geometry_id(&segments).unwrap();
    let points=[0,1].iter().map(|index|interaction::points::point_id("path",&geometry,*index,crate::schema::geometry::editing::PathPoint::Anchor).unwrap()).collect::<Vec<_>>();
    for (domain,granularity,ids) in [(DRAWING_INTERACTION_DOMAIN,DRAWING_INTERACTION_GRANULARITY,vec!["path".into()]),(DRAWING_POINT_DOMAIN,DRAWING_POINT_GRANULARITY,points)] {
        let targets=serde_json::to_string(&ids.into_iter().map(|id|serde_json::json!({"granularity":granularity,"id":id})).collect::<Vec<_>>()).unwrap();
        let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":domain,"targets":targets,"merge":"replace","method":"pick"}))),&meta).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();
        artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id).await.unwrap();
    }
    let (_,receipt)=settled(&mut app,DrawingCommand::DeleteSelection(delete_selection::DeleteSelection {}),&meta).await;
    assert_one_artifact_publication(&receipt);
    let after=app.snapshot().unwrap();
    let Some(DrawingLayerNode::Path(path))=crate::schema::find_drawing_layer(&after,"path") else {panic!("Path missing")};
    assert_eq!(path.segments,vec![crate::PathSegment::Move {to:[10.0,10.0]}]);
    assert!(app.interaction_state().await.selection.get(DRAWING_POINT_DOMAIN).is_none_or(|selection|selection.ids.is_empty()));
    assert_eq!(selected_strokes(&app).await,vec!["path".to_owned()]);
    artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id).await;assert_eq!(app.snapshot().unwrap(),before);
    artifact_laws::settle_history_verb(&mut *app,"redo",meta.instance_id).await;assert_eq!(app.snapshot().unwrap(),after);
}

#[semio_framework_async_macros::async_test]
async fn node_marquee_preserves_layers_and_supports_merge_and_cancellation() {
    for (shift,ctrl,cancelled,initial,expected) in [(false,false,false,vec![0],vec![1,2]),(true,false,false,vec![1],vec![2]),(false,true,false,vec![0],vec![0,1,2]),(false,false,true,vec![0],vec![0])] {
        let (mut app,mut meta)=inline_selection_app().await;
        meta.view_state.as_mut().unwrap().active_utility_id=Some("editNodes".into());
        let segments=vec![crate::PathSegment::Move {to:[0.0,0.0]},crate::PathSegment::Line {to:[10.0,0.0]},crate::PathSegment::Line {to:[10.0,10.0]}];
        let geometry=interaction::points::geometry_id(&segments).unwrap();
        let point=|index|interaction::points::point_id("path",&geometry,index,crate::schema::geometry::editing::PathPoint::Anchor).unwrap();
        let mut path=crate::schema::create_drawing_path_layer("Path",segments);crate::schema::layer_base_mut(&mut path).id="path".into();
        let before=DrawingSnapshot {id:"node-marquee".into(),layers:vec![path],..Default::default()};load_drawing_fixture(&mut app,&before);
        settled(&mut app,DrawingCommand::SetCamera(set_camera::SetCamera {camera:store::Viewport2d {x:0.0,y:0.0,zoom:1.0}}),&meta).await;
        for (domain,granularity,ids) in [(DRAWING_INTERACTION_DOMAIN,DRAWING_INTERACTION_GRANULARITY,vec!["path".into()]),(DRAWING_POINT_DOMAIN,DRAWING_POINT_GRANULARITY,initial.iter().copied().map(point).collect())] {
            let targets=serde_json::to_string(&ids.into_iter().map(|id|serde_json::json!({"granularity":granularity,"id":id})).collect::<Vec<_>>()).unwrap();
            let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":domain,"targets":targets,"merge":"replace","method":"pick"}))),&meta).await.unwrap();
            semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id).await.unwrap();
        }
        settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x:404.0,y:292.0,width:800.0,height:600.0,shift,ctrl,..Default::default()}),&meta).await;
        settled(&mut app,DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {x:412.0,y:312.0,width:800.0,height:600.0,shift,alt:false,samples:vec![]}),&meta).await;
        assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids,initial.iter().copied().map(point).collect::<Vec<_>>());
        assert_eq!(app.snapshot().unwrap(),before);
        settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {x:412.0,y:312.0,width:800.0,height:600.0,shift,ctrl,meta:false,alt:false,cancelled}),&meta).await;
        assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids,expected.iter().copied().map(point).collect::<Vec<_>>());
        assert_eq!(selected_strokes(&app).await,vec!["path".to_owned()]);assert_eq!(app.snapshot().unwrap(),before);
    }
}

#[semio_framework_async_macros::async_test]
async fn mounted_vector_editor_closes_its_registered_read_without_a_live_maintenance_tick(){let mut app=drawing_app().await;let layer=crate::schema::create_drawing_shape_layer_rect("Registered geometry");let snapshot=DrawingSnapshot{id:"mounted-read-close".into(),layers:vec![layer],..Default::default()};load_drawing_fixture(&mut app,&snapshot);let view=ViewModel::new(semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native);let scene=canvas_scene(rendered_drawing_canvas(&mut app,None,&view).await.unwrap());let records:serde_json::Value=serde_json::from_str(&scene.layers_json).unwrap();assert!(records.as_array().unwrap().iter().any(|record|record["id"]==layer_id(&snapshot.layers[0])));semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);assert!(app.close_terminal_is_empty());eprintln!("[DEBUG] Actual registered Drawing app returned and acknowledged its mounted source read and closed without relying on a live maintenance tick");}
