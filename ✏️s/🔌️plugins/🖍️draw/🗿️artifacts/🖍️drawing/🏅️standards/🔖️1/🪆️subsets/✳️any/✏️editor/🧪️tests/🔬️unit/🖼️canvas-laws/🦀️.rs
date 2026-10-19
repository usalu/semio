//! 🖼️ Drawing editor unit laws over the canvas and the panels: scene rendering, artboard, layers and catalogue panels, add and patch layer, host utility change, boolean combine, world point and PDF export.

use super::*;

pub(super) fn canvas_scene_of_tree(tree: semio_framework_plugin::ComponentTree) -> semio_framework_plugin::Canvas2dScene {
    let decoded = match &tree.root.component {
        semio_framework_plugin::Component::Surface(surface) => semio_framework_ui_scene::decode(surface).map_err(|_| "canvas payload"),
        _ => Err("canvas surface"),
    };
    artifact_laws::project_and_retire_fixture_tree(tree).expect("retire canvas tree");
    decoded.expect("typed canvas")
}

pub(super) fn canvas_scene(projection: String) -> semio_framework_plugin::Canvas2dScene {
    artifact_laws::decode_fixture_scene::<semio_framework_plugin::Canvas2dScene>(&projection).expect("typed canvas")
}

pub(super) async fn settle_drawing_geometry(app:&mut DrawingAppFixture,view:Option<&ViewModel>){
    use semio_framework_plugin::reactor::jobs::{start_job,step_job,cancel_job,JobBudget,JobStep};
    for effect in app.pending_effects(view).await{match effect{
        Effect::CancelJob{job}=>cancel_job(job).await,
        Effect::SpawnJob{job,kind,input,..}if kind=="semio.draw.mounted-vector"=>{start_job(job,&kind,&input).await;let mut complete=false;for _ in 0..100000{match step_job(job,JobBudget{fuel:65536,deadline_ms:8}).await{JobStep::Running(_)=>{},JobStep::Done(_)=>{complete=true;break;},JobStep::Failed(error)=>panic!("registered Drawing geometry failed: {}",String::from_utf8_lossy(&error))}}assert!(complete,"registered Drawing geometry must settle before its canvas witness");},
        _=>{},
    }}
}

pub(super) async fn rendered_drawing_canvas(app:&mut DrawingAppFixture,fixture:Option<&str>,view:&ViewModel)->Result<String,String>{
    settle_drawing_geometry(app,Some(view)).await;
    artifact_laws::render_registered_fixture_app_over(&mut app.0,DRAWING_PLAY_BODY_COMPOSITE,fixture,view,app.1.cx()).await.map_err(|error|format!("{error:?}"))
}

pub(super) fn first_layer_id(app: &DrawingAppFixture) -> String {
    layer_id(&app.snapshot().expect("materialize projection").layers[0]).to_string()
}

pub(super) fn last_layer_id(app: &DrawingAppFixture) -> String {
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
    let json = semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, DRAWING_PLAY_BODY_LAYERS, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx()).await.expect("retire semantic tree");
    assert!(json.contains("drawing-play-layers.add.path"));
    assert!(json.contains("Layer 1"));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_panel_lists_boolean_operations() {
    let mut app = drawing_app().await;
    let json = semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, DRAWING_PLAY_BODY_CATALOGUE, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx()).await.expect("retire semantic tree");
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
    assert!(semio_s_artifact_draw_drawing_model::layer_base(created).attributes.fill.is_some());
}

#[semio_framework_async_macros::async_test]
async fn patch_layers_opacity_emits_granular_operation() {
    let mut app = drawing_app().await;
    let id = first_layer_id(&app);
    let meta = artifact_laws::meta("local");
    let (result, receipt) = settled(&mut app, DrawingCommand::PatchLayers(patch_layers::PatchLayers { index:None, layer_ids: vec![id], field: "opacity".into(), value: "0.5".into() }), &meta).await;
    assert!(result.mutations.is_empty(), "the migrated route retains its operation until publication");
    assert_one_artifact_publication(&receipt);
    let projection = app.snapshot().unwrap();
    assert!((semio_s_artifact_draw_drawing_model::layer_base(&projection.layers[0]).opacity - 0.5).abs() < f64::EPSILON);
}

#[semio_framework_async_macros::async_test]
async fn patch_layer_name_emits_op_and_changes_projection() {
    let mut app = drawing_app().await;
    let id = first_layer_id(&app);
    let meta = artifact_laws::meta("local");
    let (result, receipt) = settled(&mut app, DrawingCommand::PatchLayer(patch_layer::PatchLayer { layer_id: id, field: "name".into(), value: "Renamed".into() }), &meta).await;
    assert!(result.mutations.is_empty(), "the migrated route retains its operation until publication");
    assert_one_artifact_publication(&receipt);
    assert_eq!(semio_s_artifact_draw_drawing_model::layer_base(&app.snapshot().unwrap().layers[0]).name, "Renamed");
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
    drop(rendered_drawing_canvas(&mut app, None, pen_view).await.expect("render after utility change"));
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
        active_utility_by_window_id: semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap::from([("drawing-canvas".to_string(), "shapeRect".to_string())]),
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
    artifact_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

/// 📤️ `exportDocument` (palette default `pdf`) hands the host one `DownloadMediaExport` whose base64
/// body is a PDF that `s.stdio.pdf`'s 1.4 reader opens on the artboard page — no document operation.
#[semio_framework_async_macros::async_test]
async fn export_document_downloads_a_real_pdf_of_the_document() {
    let (mut app, meta) = inline_selection_app().await;
    let (result, receipt) = settled(&mut app, DrawingCommand::ExportDocument(export_document::ExportDocument { format: "pdf".into(), ..Default::default() }), &meta).await;
    assert!(result.mutations.is_empty(), "an export is not a document operation");
    let [Effect::DownloadMediaExport { filename, mime_type, data, encoding }] = receipt.effects.as_slice() else { panic!("one download effect, got {:?}", receipt.effects) };
    assert!(filename.ends_with(".pdf"), "{filename}");
    assert_eq!(mime_type, "application/pdf");
    assert_eq!(encoding.as_deref(), Some("base64"));
    let bytes = base64_codec::base64_standard_decode(data).expect("base64 body");
    assert!(bytes.starts_with(b"%PDF-1.4\n"));
    let read = semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::io::decode_pdf(&bytes).expect("stdio's reader opens the download");
    assert_eq!(read.pages.len(), 1);
    let (_result, receipt) = settled(&mut app, DrawingCommand::ExportDocument(export_document::ExportDocument { format: "svg".into(), ..Default::default() }), &meta).await;
    let [Effect::DownloadMediaExport { mime_type, data, encoding, .. }] = receipt.effects.as_slice() else { panic!("one SVG download required") };
    assert_eq!(mime_type,"image/svg+xml");assert_eq!(encoding.as_deref(),Some("base64"));assert!(base64_codec::base64_standard_decode(data).unwrap().starts_with(b"<svg"));
    artifact_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}
