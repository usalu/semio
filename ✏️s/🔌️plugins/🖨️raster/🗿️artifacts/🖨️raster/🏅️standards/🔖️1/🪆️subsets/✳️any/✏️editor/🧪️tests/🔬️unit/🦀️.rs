pub(crate) mod context {
    //! 🧪️ Shared harness for every `editor::raster` node's tests — mirrors TEMPLATE.md §7.
    use super::super::*;
    use semio_framework_plugin::{artifact_app_laws as artifact_laws, InvocationResult, VcsArtifactApp, ViewModel};
    
    pub type RasterApp = VcsArtifactApp<EditorApp<RasterPlayApp>>;
    
    use semio_framework_plugin::PluginApp;
    
    pub async fn app() -> RasterApp {
        artifact_laws::new_app::<EditorApp<RasterPlayApp>>().await
    }
    
    pub async fn dispatch(app: &mut RasterApp, command: RasterCommand) -> InvocationResult {
        app.dispatch_typed(command, &artifact_laws::meta("local")).await.expect("dispatch")
    }
    
    pub async fn render(app: &mut RasterApp, body_key: &str) -> String {
        render_with_view(app, body_key, &ViewModel::default()).await
    }
    
    pub async fn render_with_view(app: &mut RasterApp, body_key: &str, view_state: &ViewModel) -> String {
        let tree = app.render(body_key, None, view_state).await.expect("render");
        artifact_laws::project_and_retire_fixture_tree(tree).expect("rendered fixture observation and retirement")
    }
    
    pub async fn main_window_measures(app: &mut RasterApp) -> Vec<WindowMeasure> {
        app.window_measures(&ViewModel::default()).await.remove(composite::RASTER_PLAY_WINDOW_COMPOSITE).unwrap_or_default()
    }
    
    pub async fn semio_app() -> RasterApp {
        let mut app = artifact_laws::new_app::<EditorApp<RasterPlayApp>>().await;
        let document = crate::standards::v1::subsets::any::schema::semio_example_document();
        let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster", document, None);
        let files = store::print_document_pack(&envelope).await.expect("print document pack");
        app.load_document_pack(&files).await.expect("load semio");
        app
    }
}

use context::*;
use super::*;
use crate::editor::raster::panels::{catalogue, document, inspection, masks};
use crate::standards::v1::subsets::any::schema::{empty_raster_document, layer_name, layer_visible};
use semio_framework_plugin::{artifact_app_laws, PluginApp, SET_ACTIVE_UTILITY_ACTION_ID};
use store::MemoryBackbone;

//#region 🔖️RetainedEnvelopeIngress
/// 📨️ One live document-envelope wire, built from the artifact's own empty output shell — the only
/// snapshot `ArtifactPack::encode_pack` admits, since a populated Raster document reaches output
/// exclusively through the retained page authority. The envelope the wire describes is retired here
/// through the same owner bundle the app's decode hook installs, one bounded grant per turn.
fn raster_envelope_wire() -> Vec<u8> {
    use store::ArtifactPack;

    let snapshot = crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
    let snapshot_pack = snapshot.encode_pack();
    let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let wire = dsl::json::to_string(&dsl::json::object([
        ("schema".to_string(), Value::String(RASTER_DOCUMENT_SCHEMA.to_string())),
        ("id".to_string(), Value::String("raster-live-load".to_string())),
        (
            "vcs".to_string(),
            dsl::json::object([
                ("initialSnapshot".to_string(), Value::String(snapshot_hex)),
                ("edits".to_string(), dsl::json::array([])),
                ("changes".to_string(), dsl::json::array([])),
                ("checkpoints".to_string(), dsl::json::array([])),
                ("alternatives".to_string(), dsl::json::array([])),
            ]),
        ),
        ("editMessages".to_string(), dsl::json::array([])),
        ("conflicts".to_string(), dsl::json::array([])),
    ]))
    .into_bytes();
    let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster-live-load", snapshot, None);
    let mut retirement = crate::spr::raster_envelope_decode_owner_bundle().retire_envelope(envelope);
    for _ in 0..100_000 {
        match retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Raster fixture envelope retirement") {
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return wire;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
            }
            store::SnapshotRetirementStep::Blocked => panic!("unshared Raster fixture envelope retirement blocked"),
        }
    }
    panic!("Raster fixture envelope retirement did not reach terminal")
}

/// 🎟️ Reserves page/byte credits first, then feeds the wire as fixed-size pages and seals — the
/// caller never holds a growable buffer and never sees the store the decode will publish into.
fn admit_raster_envelope(app: &mut RasterApp, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Raster live envelope ingress credits");
    for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Raster live envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Raster live envelope page admission failed: {}: {}", fault.code.0, fault.message));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("Raster live envelope seal/submit"));
    handle
}

/// 🔄️ Pumps the load one bounded maintenance turn at a time and polls after each — never inline,
/// never unbounded.
fn drive_raster_live_load(app: &mut RasterApp, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..100_000 {
        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one Raster live maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle).expect("Raster live load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("Raster live envelope load did not reach terminal")
}

#[semio_framework_async_macros::async_test]
async fn raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed() {
    let mut app = app().await;
    let base_generation = app.artifact_generation_now();
    let handle = admit_raster_envelope(&mut app, &raster_envelope_wire());
    assert_eq!(handle.generation, base_generation);
    assert_eq!(drive_raster_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("first exact Raster load acknowledgement"));
    assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate Raster load acknowledgement is a no-op"));
}

#[semio_framework_async_macros::async_test]
async fn raster_live_envelope_cancel_closes_retained_pages_without_publication() {
    let mut app = app().await;
    let base_generation = app.artifact_generation_now();
    let wire = raster_envelope_wire();
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled Raster ingress credits");
    let first = &wire[..wire.len().min(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
    let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    bytes[..first.len()].copy_from_slice(first);
    let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled Raster first page");
    app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("cancelled Raster page admission failed: {}: {}", fault.code.0, fault.message));
    app.cancel_artifact_envelope_load(handle).expect("cancel exact Raster ingress");
    assert_eq!(drive_raster_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
    assert_eq!(app.artifact_generation_now(), base_generation);
}
//#endregion 🔖️RetainedEnvelopeIngress

/// 🌱️ Relocated verbatim from `⚙️engine`'s own test module (rule 4: `raster_io`/`raster_composite_media`
/// now live in this file's own `🔖️Io` region).
#[semio_framework_async_macros::async_test]
async fn raster_io_declares_image_in_and_image_out() {
    let io = raster_io();
    assert_eq!(io.document_schema, RASTER_DOCUMENT_SCHEMA);
    assert_eq!(io.artifact.id, "2d.raster");
    assert!(io.ports.iter().any(|p| p.id == "image:in"));
    let out_port = raster_image_out_port();
    assert_eq!(out_port.kind_id.as_deref(), Some("2d.image"));
}

#[semio_framework_async_macros::async_test]
async fn raster_composite_media_exports_structured_2d_image_payload() {
    let document = empty_raster_document();
    let media = raster_composite_media(&document).expect("export image:out");
    let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
    assert_eq!(schema, "2d.image");
    assert!(!json.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn window_measures_expose_brush_and_eraser_option_groups() {
    let mut app = app().await;
    let measures = main_window_measures(&mut app).await;
    assert_eq!(measures.len(), 2);
    assert!(measures.iter().any(|m| matches!(m, WindowMeasure::Group { id, .. } if id == "raster-utility-options-paintBrush")));
}

#[semio_framework_async_macros::async_test]
async fn renders_raster_scene() {
    let mut app = app().await;
    let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await;
    assert!(json.contains("raster"));
}

#[semio_framework_async_macros::async_test]
async fn renders_navigator_scene() {
    let mut app = app().await;
    let json = render(&mut app, navigator::RASTER_PLAY_BODY_NAVIGATOR).await;
    assert!(json.contains("\"componentKind\":\"paint-2d\""));
    assert!(json.contains("\"viewMode\":\"navigator\""));
}

#[semio_framework_async_macros::async_test]
async fn parses_semio_example_document() {
    let document = crate::standards::v1::subsets::any::schema::semio_example_document();
    assert!(!document.layers.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn empty_document_background_layer_has_identity_scale() {
    let document = empty_raster_document();
    let json = document_sync_json(&document);
    assert!(json.contains(r#""scaleX":1.0"#), "expected identity scale in {json}");
    assert!(json.contains(r#""scaleY":1.0"#), "expected identity scale in {json}");
    assert!(!json.contains(r#""scaleX":0.0"#), "layer must not collapse to zero size");
}

#[semio_framework_async_macros::async_test]
async fn renders_layers_tree() {
    let mut app = semio_app().await;
    let json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS).await;
    assert!(json.contains("\"type\":\"tree\""));
    assert!(json.contains("Backdrop"));
}

#[semio_framework_async_macros::async_test]
async fn raster_labels_resolve_native_english_by_default() {
    let mut app = app().await;
    let layers_json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS).await;
    assert!(layers_json.contains("Add Pixel"));
    assert!(layers_json.contains("Add Group"));
    let masks_json = render(&mut app, masks::RASTER_PLAY_BODY_MASKS).await;
    assert!(masks_json.contains("Masks"));
    assert!(masks_json.contains("No masks"));
    let catalogue_json = render(&mut app, catalogue::RASTER_PLAY_BODY_CATALOGUE).await;
    assert!(catalogue_json.contains("Layer kinds"));
    assert!(catalogue_json.contains("raster-catalogue.pixel"));
    assert!(catalogue_json.contains("raster-catalogue.group"));
    assert!(catalogue_json.contains("raster-catalogue.adjustment"));
    let properties_json = render(&mut app, inspection::RASTER_PLAY_BODY_PROPERTIES).await;
    assert!(properties_json.contains("raster-play-inspector.schema"));
    assert!(properties_json.contains(RASTER_DOCUMENT_SCHEMA));
    assert!(properties_json.contains("raster-play-inspector.brush"));
}

#[semio_framework_async_macros::async_test]
async fn raster_labels_resolve_german_locale() {
    let mut app = app().await;
    let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let layers_json = render_with_view(&mut app, document::RASTER_PLAY_BODY_LAYERS, &view_state).await;
    assert!(layers_json.contains("Pixel hinzufügen"));
    assert!(layers_json.contains("Gruppe hinzufügen"));
    let masks_json = render_with_view(&mut app, masks::RASTER_PLAY_BODY_MASKS, &view_state).await;
    assert!(masks_json.contains("Masken"));
    assert!(masks_json.contains("Keine Masken"));
    let catalogue_json = render_with_view(&mut app, catalogue::RASTER_PLAY_BODY_CATALOGUE, &view_state).await;
    assert!(catalogue_json.contains("Ebenenarten"));
}

#[semio_framework_async_macros::async_test]
async fn composite_scene_syncs_document_and_assets() {
    let mut app = semio_app().await;
    let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await;
    assert!(json.contains("\"componentKind\":\"paint-2d\""));
    assert!(json.contains("\"viewMode\":\"composite\""));
    assert!(!json.contains("\"assetsJson\":\"{}\""), "semio fixture has embedded assets");
    let document = crate::standards::v1::subsets::any::schema::semio_example_document();
    let sync_json = document_sync_json(&document);
    assert!(!sync_json.contains("\"assets\""), "sync json must omit assets");
    assert!(sync_json.contains("\"params\""), "adjustment params must survive document→sync roundtrip for the paint host");
    let sync_value: Value = dsl::os_pack::json::parse(&sync_json).expect("sync json");
    let layers = sync_value.get("layers").and_then(Value::as_array).expect("layers");
    assert!(layers.iter().any(|layer| layer.get("kind").and_then(Value::as_str) == Some("adjustment") && layer.get("params").is_some()));
    assert!(document.assets.contains_key("semio-emblem"));
}

#[semio_framework_async_macros::async_test]
async fn semio_example_preserves_adjustment_params() {
    let document = crate::standards::v1::subsets::any::schema::semio_fixture_snapshot();
    let RasterLayerNode::Adjustment { params, adjustment_kind, .. } = document.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Adjustment { id, .. } if id == "brighten")).expect("brighten adjustment") else {
        panic!("expected adjustment");
    };
    assert_eq!(adjustment_kind, "brightnessContrast");
    assert!(params.contains_key("brightness"), "fixture brightness must roundtrip");
    assert!(params.contains_key("contrast"), "fixture contrast must roundtrip");
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
    dispatch(&mut app, RasterCommand::SetCompositeViewport(set_composite_viewport::SetCompositeViewport { width: 640.0, height: 480.0 })).await;
    let json = render(&mut app, navigator::RASTER_PLAY_BODY_NAVIGATOR).await;
    assert!(json.contains("compositeViewportJson"));
    assert!(json.contains(r#"\"width\":640.0"#));
    assert!(json.contains(r#"\"height\":480.0"#));
}

#[semio_framework_async_macros::async_test]
async fn set_camera_mutates_runtime_and_emits_no_operations() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot");
    let result = dispatch(&mut app, RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::RasterCamera { x: 4.0, y: 5.0, zoom: 2.0 } })).await;
    assert!(result.mutations.is_empty(), "camera is a view action and emits no operations");
    assert_eq!(app.snapshot().expect("snapshot"), before, "camera never mutates the document");
    let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await;
    assert!(json.contains(r#"\"zoom\":2.0"#), "composite scene camera reflects runtime state: {json}");
    assert!(json.contains(r#"\"x\":4.0"#), "composite scene camera reflects runtime state: {json}");
}

#[semio_framework_async_macros::async_test]
async fn set_camera_zoom_updates_zoom_and_keeps_pan_via_runtime() {
    let mut app = app().await;
    dispatch(&mut app, RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::RasterCamera { x: 4.0, y: 5.0, zoom: 1.0 } })).await;
    let result = dispatch(&mut app, RasterCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { zoom: 3.0 })).await;
    assert!(result.mutations.is_empty(), "camera zoom is a view action and emits no operations");
    let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await;
    assert!(json.contains(r#"\"zoom\":3.0"#), "zoom updated: {json}");
    assert!(json.contains(r#"\"x\":4.0"#), "pan preserved across zoom-only update: {json}");
}

#[semio_framework_async_macros::async_test]
async fn add_layer_action_appends_and_undo_removes() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot").layers.len();
    dispatch(&mut app, RasterCommand::AddLayer(add_layer::AddLayer { kind: "group".into() })).await;
    let projection = app.snapshot().expect("snapshot");
    assert_eq!(projection.layers.len(), before + 1);
    assert!(matches!(projection.layers.last().unwrap(), RasterLayerNode::Group { .. }));
    app.handle_action("undo", None, &artifact_app_laws::meta("local")).await.expect("undo");
    assert_eq!(app.snapshot().expect("snapshot").layers.len(), before);
}

#[semio_framework_async_macros::async_test]
async fn patch_layer_renames_and_toggles_visibility_round_trip() {
    let mut app = app().await;
    let layer_id = crate::standards::v1::subsets::any::schema::layer_node_id(&app.snapshot().expect("snapshot").layers[0]).to_string();
    dispatch(&mut app, RasterCommand::PatchLayer(patch_layer::PatchLayer { layer_id: layer_id.clone(), field: "name".into(), value: "Renamed".into() })).await;
    assert_eq!(layer_name(&app.snapshot().expect("snapshot").layers[0]), "Renamed");
    dispatch(&mut app, RasterCommand::ToggleLayerVisible(toggle_layer_visible::ToggleLayerVisible { layer_id })).await;
    assert!(!layer_visible(&app.snapshot().expect("snapshot").layers[0]));
    app.handle_action("undo", None, &artifact_app_laws::meta("local")).await.expect("undo toggle");
    assert!(layer_visible(&app.snapshot().expect("snapshot").layers[0]));
}

#[semio_framework_async_macros::async_test]
async fn move_layer_into_group() {
    let mut app = app().await;
    dispatch(&mut app, RasterCommand::AddLayer(add_layer::AddLayer { kind: "group".into() })).await;
    let (group_id, pixel_id) = {
        let projection = app.snapshot().expect("snapshot");
        let group = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Group { .. })).unwrap();
        let pixel = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Pixel { .. })).unwrap();
        (crate::standards::v1::subsets::any::schema::layer_node_id(group).to_string(), crate::standards::v1::subsets::any::schema::layer_node_id(pixel).to_string())
    };
    let target_row = format!("{RASTER_TREE_PREFIX}.group.{group_id}");
    dispatch(&mut app, RasterCommand::MoveLayer(move_layer::MoveLayer { layer_id: pixel_id.clone(), target_row_id: target_row, drop_position: "after".into() })).await;
    let projection = app.snapshot().expect("snapshot");
    let RasterLayerNode::Group { children, .. } = projection.layers.iter().find(|layer| crate::standards::v1::subsets::any::schema::layer_node_id(layer) == group_id).unwrap() else {
        panic!("expected group");
    };
    assert_eq!(children.len(), 1);
    assert_eq!(crate::standards::v1::subsets::any::schema::layer_node_id(&children[0]), pixel_id);
}

/// 🧪️ The definitional merge proof: A adds a layer while B renames the background layer — disjoint
/// tree edits on one backbone that must both survive on both instances.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_layer_edits_via_backbone() {
    let mut instance_a = app().await;
    let mut instance_b = app().await;
    // Seed both from an identical base projection (a background layer with a fixed id) so B's
    // rename targets the same layer A holds — per-instance `initial_snapshot` mints fresh ids.
    let mut base = crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
    base.layers = vec![RasterLayerNode::Pixel {
        id: "bg".into(),
        name: "Background".into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: crate::RasterTransform::default(),
        mask: None,
        width: Some(512),
        height: Some(512),
        image_key: None,
    }];
    let base_envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster", base, None);
    let base_files = store::print_document_pack(&base_envelope).await.expect("print document pack");
    instance_a.load_document_pack(&base_files).await.expect("load a");
    instance_b.load_document_pack(&base_files).await.expect("load b");
    let background_id = "bg".to_string();
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://raster-convergence", "mem://raster-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");

    dispatch(&mut instance_a, RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() })).await;
    dispatch(&mut instance_b, RasterCommand::PatchLayer(patch_layer::PatchLayer { layer_id: background_id, field: "name".into(), value: "Renamed By B".into() })).await;

    instance_a.handle_action("commitCheckpoint", None, &artifact_app_laws::meta("actor-a")).await.expect("pump a");
    instance_b.handle_action("commitCheckpoint", None, &artifact_app_laws::meta("actor-b")).await.expect("pump b");

    let projection_a = instance_a.snapshot().expect("projection a");
    let projection_b = instance_b.snapshot().expect("projection b");
    assert_eq!(projection_a.layers.len(), 2, "A keeps its added layer");
    assert_eq!(projection_b.layers.len(), 2, "B converges on A's added layer");
    assert_eq!(layer_name(&projection_a.layers[0]), "Renamed By B", "A converges on B's rename");
    assert_eq!(layer_name(&projection_b.layers[0]), "Renamed By B", "B keeps its rename");
}

#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent() {
    artifact_app_laws::assert_ingest_idempotent::<EditorApp<RasterPlayApp>, usize>(RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() }), |app| app.snapshot().unwrap().layers.len()).await;
}

#[semio_framework_async_macros::async_test]
async fn utility_registry_declares_utilities_scoped_to_the_composite_window() {
    let definition = create_raster_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utility_ids, ["selectMarquee", "paintBrush", "paintEraser"]);
    // The marquee carries the Selection category; the paint utilities are Tools.
    let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
    assert_eq!(selects, ["selectMarquee"]);
    let composite = definition.window_kinds.iter().find(|window| window.id == composite::RASTER_PLAY_WINDOW_COMPOSITE).expect("composite window");
    assert_eq!(composite.utilities.len(), definition.utilities.len(), "every utility is scoped to the composite window kind");
    // The framework auto-injects the setActiveUtility View action once utilities are declared; no doc operation survives.
    assert!(composite.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, ActionKind::View)));
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
    assert!(RasterPlayApp::export_media("document:out", &doc).is_ok());
    assert!(matches!(RasterPlayApp::export_media("unknown:out", &doc), Err(MediaError::NotImplemented)));
}

#[semio_framework_async_macros::async_test]
async fn raster_import_media_appends_layer_from_incoming_image() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot").layers.len();
    let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: "aGVsbG8=".into() } };
    let result = app.import_media("image:in", media, &artifact_app_laws::meta("local")).await.expect("import image:in");
    assert!(!result.mutations.is_empty(), "image:in import must emit a real document operation");
    assert_eq!(app.snapshot().expect("snapshot").layers.len(), before + 1);
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order — TEMPLATE.md §7's
/// permanent wire guard, feeding the round-trip/keyword-uniqueness/leading-token laws below.
fn every_command() -> Vec<RasterCommand> {
    vec![
        RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() }),
        RasterCommand::DropLayerKind(drop_layer_kind::DropLayerKind { kind: "group".into() }),
        RasterCommand::SetLayerVisible(set_layer_visible::SetLayerVisible { layer_id: "l1".into(), visible: Some(true) }),
        RasterCommand::ToggleLayerVisible(toggle_layer_visible::ToggleLayerVisible { layer_id: "l1".into() }),
        RasterCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: "l1".into() }),
        RasterCommand::DuplicateLayer(duplicate_layer::DuplicateLayer { layer_id: "l1".into() }),
        RasterCommand::PatchLayer(patch_layer::PatchLayer { layer_id: "l1".into(), field: "opacity".into(), value: "0.4".into() }),
        RasterCommand::PatchLayers(patch_layers::PatchLayers { layer_ids: vec!["a".into(), "b".into()], field: "name".into(), value: "Renamed".into() }),
        RasterCommand::MoveLayer(move_layer::MoveLayer { layer_id: "l1".into(), target_row_id: "raster-play-layers".into(), drop_position: "after".into() }),
        RasterCommand::SetBrushSize(set_brush_size::SetBrushSize { value: 40.0 }),
        RasterCommand::SetBrushOpacity(set_brush_opacity::SetBrushOpacity { value: 0.5 }),
        RasterCommand::SetCompositeViewport(set_composite_viewport::SetCompositeViewport { width: 640.0, height: 480.0 }),
        RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::RasterCamera { x: 1.0, y: 2.0, zoom: 1.5 } }),
        RasterCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { zoom: 2.0 }),
        RasterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::art_raster_demo::ID.into() }),
    ]
}

/// ⚖️ LAW: raster's retained route table, its publication contracts, its bounded-first-step proofs,
/// `RasterCommand::TOOL_JOB_IDS` and the catalog's `Migrated` classifications are the SAME
/// seventeen ids — the exact join `validate_tool_job_rows` demands
/// (`interactive-job.catalog-authority` / `interactive-job.catalog-incomplete`). Mirrors block2d's
/// `retained_route_dispositions_are_exact_and_exhaustive`.
#[semio_framework_async_macros::async_test]
async fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};
    use std::collections::BTreeSet;
    assert_eq!(RASTER_RETAINED_TOOL_IDS.len(), 15);
    assert_eq!(<RasterPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 17);
    assert_eq!(RasterRetainedCommandJobFactory::PUBLICATION_CONTRACTS.len(), 17);
    assert_eq!(raster_retained_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(raster_retained_contract().cancellation, ToolCancellationPolicy::PerOperation);

    let retained: BTreeSet<&str> = RASTER_RETAINED_TOOL_IDS.iter().copied().collect();
    assert_eq!(retained, every_command().iter().map(RasterCommand::command_id).collect::<BTreeSet<_>>(), "every RasterCommand row must be a retained route");
    assert_eq!(retained, RasterCommand::TOOL_JOB_IDS.iter().copied().collect::<BTreeSet<_>>(), "the retained table must equal the generated tool-job id set");

    // 🛣️ Lane discipline, read off the handlers: ten document verbs publish into the artifact lane,
    // five session verbs into the config lane, and no route publishes into both.
    let artifact_lane: BTreeSet<&str> = ["addLayer", "dropLayerKind", "setLayerVisible", "toggleLayerVisible", "deleteLayer", "duplicateLayer", "patchLayer", "patchLayers", "moveLayer", "setActiveExample"].into_iter().collect();
    for tool_id in RASTER_RETAINED_TOOL_IDS {
        let contract = RasterRetainedCommandJobFactory::PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == *tool_id).unwrap_or_else(|| panic!("publication contract for {tool_id}"));
        let expected = if artifact_lane.contains(tool_id) { ArtifactToolPublicationLane::Artifact } else { ArtifactToolPublicationLane::Config };
        assert_eq!(contract.lanes, [expected].as_slice(), "{tool_id} publishes into exactly one lane");
    }
    assert!(<RasterPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some(), "the Artifact lane is rejected outright without a document one-item preparation factory");
    assert!(<RasterPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some(), "the Config lane is rejected outright without a config one-item preparation factory");

    // 🧵️ Every retained plugin command id must be UI-dispatchable.
    let definition = create_raster_app();
    for tool_id in RASTER_RETAINED_TOOL_IDS {
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("action {tool_id} declared"));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id} must be UI-dispatchable");
    }
    // ⚖️ No declaration in raster's catalog may stay `Unclassified` — that is the release-blocking
    // gate `validate_interactive_job_classification` enforces. Every action lands on a window kind
    // (both app-declared and framework-injected ones), so this sweep sees the whole action surface.
    for action in definition.window_kinds.iter().flat_map(|window| window.actions.iter()) {
        assert_ne!(action.semantics.execution.interactive_job, InteractiveJobClassification::Unclassified, "action {} is unclassified", action.id);
    }
    for command in definition.commands.iter() {
        assert_ne!(command.semantics.execution.interactive_job, InteractiveJobClassification::Unclassified, "command {} is unclassified", command.id);
    }

    // ⚖️ The gate's own arithmetic, spelled out: `expected = TOOL_JOB_IDS ∩ migrated` must equal the
    // proof set. `RASTER_RETAINED_TOOL_IDS` names that proof set (its length is asserted equal to
    // `bounded_first_step_tool_proofs().len()` above, and the macro derives the proofs from the same
    // literal list), so a route that is proven but left unclassified — or classified but unproven —
    // fails here instead of at runtime with `interactive-job.catalog-authority`.
    let migrated_ids: BTreeSet<&str> =
        definition.window_kinds.iter().flat_map(|window| window.actions.iter()).filter(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated).map(|action| action.id.as_str()).collect();
    assert_eq!(RasterCommand::TOOL_JOB_IDS.iter().copied().filter(|tool_id| migrated_ids.contains(tool_id)).collect::<BTreeSet<_>>(), retained, "every bounded-first-step proof entry must be Migrated, and every Migrated tool-job row must be proven");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🎫️ Every `app_commands!` row's wire keyword must be distinct — the cross-cutting invariant the
/// macro exists to hold.
#[semio_framework_async_macros::async_test]
async fn command_wire_keywords_are_unique_across_every_row() {
    let commands = every_command();
    assert_eq!(commands.len(), 17, "every RasterCommand row must be covered by every_command()");
    let mut keywords: Vec<String> = commands.iter().map(|command| protocol::OpText::print_op(command).split(' ').next().unwrap_or_default().to_string()).collect();
    keywords.sort();
    keywords.dedup();
    assert_eq!(keywords.len(), commands.len(), "every row's wire keyword must be distinct");
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — what a
/// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
/// keyword at all and no longer parses).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_declared_wire_keyword() {
    let expectations: Vec<(&str, RasterCommand)> = every_command()
        .into_iter()
        .map(|command| {
            let keyword: &'static str = match &command {
                RasterCommand::AddLayer(_) => "add-layer",
                RasterCommand::DropLayerKind(_) => "drop-layer-kind",
                RasterCommand::SetLayerVisible(_) => "set-layer-visible",
                RasterCommand::ToggleLayerVisible(_) => "toggle-layer-visible",
                RasterCommand::DeleteLayer(_) => "delete-layer",
                RasterCommand::DuplicateLayer(_) => "duplicate-layer",
                RasterCommand::PatchLayer(_) => "patch-layer",
                RasterCommand::PatchLayers(_) => "patch-layers",
                RasterCommand::MoveLayer(_) => "move-layer",
                RasterCommand::SetBrushSize(_) => "brush-size",
                RasterCommand::SetBrushOpacity(_) => "brush-opacity",
                RasterCommand::SetCompositeViewport(_) => "composite-viewport",
                RasterCommand::SetCamera(_) => "camera",
                RasterCommand::SetCameraZoom(_) => "camera-zoom",
                RasterCommand::SetActiveExample(_) => "set-active-example",
            };
            (keyword, command)
        })
        .collect();
    for (expected_keyword, command) in expectations {
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for {command:?}: {printed:?}");
    }
}

/// ⚖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to exact
/// bytes so an ACCIDENTAL row reorder is caught. Baseline rebased once, deliberately, by the
/// `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL` ticket: dropping the two leading `setSnapshot`/
/// `setActiveExample` rows (whole-document replace is no longer expressible as a mutation)
/// shifted every later row's binary ordinal down by two — `set-layer-visible` 4→2 (`0104`→`0102`).
/// `setActiveExample` returned in `26/09/05/RASTER-PLUGIN-END-TO-END` as an ordered mutation batch
/// rather than a snapshot swap, APPENDED as the last row, so no earlier ordinal moved.
/// Rebased again by `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`: the `set-selection`/
/// `set-hover`/`select-all` rows (the only other `Option`-carrying case, `set-hover`) are deleted
/// outright — layer selection/hover is the framework-owned `"layers"` interaction domain now.
/// `set-layer-visible`'s ordinal is unaffected (it sits before the deleted rows). Greenfield repo,
/// no persisted wire data to migrate. Any FURTHER drift here is a real format break, not a
/// fixture mismatch.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_declared_wire_bytes() {
    let cases: [(RasterCommand, &str, &str); 1] = [(RasterCommand::SetLayerVisible(set_layer_visible::SetLayerVisible { layer_id: "l1".into(), visible: None }), "set-layer-visible set-layer-visible layer-id=l1", "010201026c3101000600")];
    for (command, text, hex) in cases {
        assert_eq!(protocol::OpText::print_op(&command), text, "printed text drifted for {command:?}");
        let bytes = protocol::OpBinary::encode_op(&command).expect("encode");
        assert_eq!(bytes.iter().map(|b| format!("{b:02x}")).collect::<String>(), hex, "binary bytes drifted for {command:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_across_every_row() {
    let mut seen = std::collections::HashSet::new();
    for command in every_command() {
        assert!(seen.insert(command.command_id().to_string()), "duplicate command_id {}", command.command_id());
    }
}
