pub(crate) mod context {
    //! 🧪️ Shared harness for every `editor::raster` node's tests — mirrors TEMPLATE.md §7.
    use super::super::*;
    use semio_framework_plugin::{artifact_app_laws as artifact_laws, InvocationResult, VcsArtifactApp, ViewModel, ViewWindowInstance};
    
    pub type RasterApp = VcsArtifactApp<EditorApp<RasterPlayApp>>;

    use semio_framework_plugin::PluginApp;

    /// 🧹️ A live app fixture that CLOSES itself. `VcsArtifactApp`'s `ArtifactStore` owns an
    /// `ArtifactStoreCursorDisposer` whose `Drop` asserts terminal-empty ownership, so a plainly
    /// dropped fixture panics with "artifact store reached Drop without its exact terminal-empty
    /// shallow-shell witness". Dereferences to the app and drains the exact retained close ladder on
    /// the way out — the same law the runtime uses, and the same shape `mounted_app` already had.
    pub struct RasterAppFixture(RasterApp);

    impl std::ops::Deref for RasterAppFixture {
        type Target = RasterApp;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for RasterAppFixture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for RasterAppFixture {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                artifact_laws::close_registered_fixture_app(&mut self.0);
            }
        }
    }

    /// ✏️ Adapts `create_raster_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `new_app_with_registry` still expects — framework test context gap.
    pub fn raster_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_raster_app(), examples: Vec::new() }
    }

    /// 🧪️ The app instance every test builds — registry-backed, because there is no other kind.
    /// `EditorApp<RasterPlayApp>` publishes a `bounded_first_step_tool_proofs!` roster, and
    /// `with_registry_on_bus` joins that roster against the registry's `Migrated` tool ids
    /// (`AppActionRegistry::validate_tool_job_rows`): an empty registry declares none of them, so the
    /// registry-LESS `artifact_app_laws::new_app` fails construction outright with
    /// `interactive-job.catalog-authority … generated_migrated=false, migrated={}`.
    /// 🔌️ Mounted (`bind_instance_id`): `dispatch_typed` refuses `interactive-job.live-instance` for
    /// every typed command until the app carries the live runtime instance the `ActionMeta` names, so
    /// an unmounted fixture can construct and render but never mutate.
    pub async fn app() -> RasterAppFixture {
        let mut app = artifact_laws::new_app_with_registry::<EditorApp<RasterPlayApp>>(raster_app_manifest_for_tests).await;
        app.bind_instance_id(artifact_laws::meta("local").instance_id).await;
        RasterAppFixture(app)
    }

    /// 🪟️ The view state the runtime always carries: the composite window INSTANCE the action is
    /// dispatched in. Without it the app publishes no window-scoped chrome and every measure
    /// projection comes back empty.
    pub fn raster_view_state() -> ViewModel {
        let id = "raster-composite";
        ViewModel {
            window_id: Some(id.into()),
            window_instances: vec![ViewWindowInstance { id: id.into(), window_kind_id: composite::RASTER_PLAY_WINDOW_COMPOSITE.into() }],
            ..Default::default()
        }
    }

    /// 🔌️ A MOUNTED app (`bind_instance_id`) QUEUES every typed command: the dispatch only admits it
    /// and nothing reaches the document until the host continuation settles it. A test that stopped at
    /// `dispatch_typed` read an unchanged document, which made every action test below look like a
    /// silent no-op.
    pub async fn dispatch(app: &mut RasterApp, command: RasterCommand) -> InvocationResult {
        let mut action = artifact_laws::meta("local");
        action.view_state = Some(raster_view_state());
        let mut result = app.dispatch_typed(command, &action).await.expect("dispatch");
        let settled = artifact_laws::settle_registered_typed_operation(app, artifact_laws::meta("local").instance_id).await.expect("settle the typed operation");
        result.requested_effects.extend(settled.effects);
        result
    }
    
    /// ⏪️ One framework-reserved history verb driven the whole way a shell drives it: the admission
    /// comes back as a `SpawnJob` receipt the caller must commit before the store ever sees
    /// `ArtifactCommand::Undo`, and only then does the typed publication settle. Pressing it with a
    /// bare `handle_action` reads back as a silent no-op on a MOUNTED app.
    pub async fn history(app: &mut RasterApp, action: &str) {
        artifact_laws::settle_history_verb(app, action, artifact_laws::meta("local").instance_id).await;
    }

    pub async fn render(app: &mut RasterApp, body_key: &str) -> String {
        render_with_view(app, body_key, &ViewModel::default()).await
    }
    
    pub async fn render_with_view(app: &mut RasterApp, body_key: &str, view_state: &ViewModel) -> String {
        let tree = app.render(body_key, None, view_state).await.expect("render");
        artifact_laws::project_and_retire_fixture_tree(tree).expect("rendered fixture observation and retirement")
    }
    
    pub async fn main_window_measures(app: &mut RasterApp) -> Vec<WindowMeasure> {
        app.window_measures(&raster_view_state()).await.remove(composite::RASTER_PLAY_WINDOW_COMPOSITE).unwrap_or_default()
    }
    

    /// 🧹️ A document envelope a test builds only to print a pack is a TERMINAL SHELL: every nested
    /// owner below it (its `ArtifactVcs`, its history ledger, its initial snapshot — which for a
    /// raster document owns a populated asset pool) asserts in `Drop` that the bounded protocol ran
    /// first, and only a store ever runs it. The artifact's own owner catalog retires it instead.
    pub fn retire_raster_envelope(envelope: store::ArtifactEnvelope<RasterSnapshot, RasterMutation>) {
        let mut retirement = crate::spr::raster_document_store_owners().retire_envelope_uninstalled(envelope).expect("an uninstalled raster owner catalog retires one envelope");
        // ⛽️ A `RasterOwnedMap` page backing is one 16 KiB allocation released whole.
        let grant = crate::RASTER_OWNED_MAP_PAGE_BACKING_BYTES;
        for _ in 0..1_000_000 {
            if store::ErasedSnapshotRetirement::terminal_is_empty(retirement.as_ref()) {
                return;
            }
            store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), 1, grant).expect("raster test envelope retires within its exact grant");
        }
        panic!("raster test envelope did not reach its terminal-empty shell")
    }

    pub async fn semio_app() -> RasterAppFixture {
        let mut app = app().await;
        let document = crate::standards::v1::subsets::any::schema::semio_example_document();
        let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster", document, None);
        let files = store::print_document_pack(&envelope).await.expect("print document pack");
        retire_raster_envelope(envelope);
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
    assert_eq!(io.artifact_schema, RASTER_DOCUMENT_SCHEMA);
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
    // 🧾️ A `scene_surface` node publishes its payload as a PACKED document (`"kind":"paint-2d"`,
    // `"docSchema":"paint-2d@1"`, `doc.bytes`), never as inline tree text.
    assert!(json.contains("\"kind\":\"paint-2d\""), "{json}");
    assert!(packed_scene_text(&json).contains("navigator"), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn parses_semio_example_document() {
    let document = crate::standards::v1::subsets::any::schema::semio_example_document();
    assert!(!document.layers.is_empty());
    crate::standards::v1::subsets::any::schema::mutations::binary::unit_tests::retirement::retire_raster_snapshot(document);
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
    // 🧾️ A `scene_surface` node publishes the scene's SPINE inside the fixed-capacity surface doc and
    // every lane `Paint2dScene::split_lanes` declares (`documentSync`, `assets`) as its own
    // `paged_text_carrier` child BESIDE that doc — unconditionally, both lanes, every render. Reading
    // them off the packed doc bytes (`packed_scene_text`) therefore never saw a single asset byte: the
    // spine carries only the lane manifest (`SceneLaneRef { lane, bytes, hash }`). The assembled scene
    // a render host actually reads is the doc PLUS its lanes.
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::Paint2dScene>(&json).expect("paint-2d scene with its lanes");
    assert!(json.contains("\"kind\":\"paint-2d\""), "{json}");
    assert_eq!(scene.view_mode, "composite");
    assert!(scene.document_sync_json.contains("\"layers\""), "the documentSync lane carries the layer forest: {}", scene.document_sync_json);
    // 🖼️ The pixels themselves: a document that was packed (`print_document_pack`) and reopened
    // (`load_document_pack`) must still resolve its composed asset children to real bytes, because
    // `Paint2dHost` uploads one texture per `assetsJson` entry and a pixel-less handle pool renders a
    // blank canvas (play pane measured blank on :6033, 2026-09-21).
    assert!(scene.assets_json.contains("semio-emblem"), "the reopened document keeps its asset entry: {}", scene.assets_json);
    assert!(scene.assets_json.contains("image/png"), "the semio fixture's embedded asset must resolve to real pixels: {}", scene.assets_json);
    let document = crate::standards::v1::subsets::any::schema::semio_example_document();
    let sync_json = document_sync_json(&document);
    assert!(!sync_json.contains("\"assets\""), "sync json must omit assets");
    assert!(sync_json.contains("\"params\""), "adjustment params must survive document→sync roundtrip for the paint host");
    let sync_value: Value = dsl::os_pack::json::parse(&sync_json).expect("sync json");
    let layers = sync_value.get("layers").and_then(Value::as_array).expect("layers");
    assert!(layers.iter().any(|layer| layer.get("kind").and_then(Value::as_str) == Some("adjustment") && layer.get("params").is_some()));
    assert!(document.assets.contains_key("semio-emblem"));
    crate::standards::v1::subsets::any::schema::mutations::binary::unit_tests::retirement::retire_raster_snapshot(document);
}

/// 🛡️ Play-grid boot regression (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP): the composite
/// and navigator windows build their `Paint2dScene` through `raster_scene` on every render, and the
/// demo document carries a populated asset pool plus a populated adjustment `params` map. The sync
/// projection used to serialize the whole snapshot through `RasterOwnedMap`'s `ToValue`, whose
/// populated-map guard trapped the wasm guest right after boot.
#[semio_framework_async_macros::async_test]
async fn raster_scene_projects_populated_owned_maps_without_wholesale_serialization() {
    let document = crate::standards::v1::subsets::any::schema::semio_example_document();
    assert!(!document.assets.is_empty(), "the regression needs a populated asset pool");
    let scene = raster_scene(&document, &crate::editor::raster::config::RasterConfig::default(), "brush", "composite");
    let sync_value: Value = dsl::os_pack::json::parse(&scene.document_sync_json).expect("sync json");
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
    assert!(matches!(dsl::os_pack::json::parse(&scene.assets_json), Ok(Value::Object(_))), "assets json stays a well-formed object");
    crate::standards::v1::subsets::any::schema::mutations::binary::unit_tests::retirement::retire_raster_snapshot(document);
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
    crate::standards::v1::subsets::any::schema::mutations::binary::unit_tests::retirement::retire_raster_snapshot(document);
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
    let text = packed_scene_text(&render(&mut app, navigator::RASTER_PLAY_BODY_NAVIGATOR).await);
    assert!(text.contains("compositeViewportJson"), "{text}");
    assert!(text.contains(r#""width":640.0"#), "{text}");
    assert!(text.contains(r#""height":480.0"#), "{text}");
}

#[semio_framework_async_macros::async_test]
async fn set_camera_mutates_runtime_and_emits_no_operations() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot");
    let result = dispatch(&mut app, RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::RasterCamera { x: 4.0, y: 5.0, zoom: 2.0 } })).await;
    assert!(result.mutations.is_empty(), "camera is a view action and emits no operations");
    assert_eq!(app.snapshot().expect("snapshot"), before, "camera never mutates the document");
    let text = packed_scene_text(&render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await);
    assert!(text.contains(r#""zoom":2.0"#), "composite scene camera reflects runtime state: {text}");
    assert!(text.contains(r#""x":4.0"#), "composite scene camera reflects runtime state: {text}");
}

#[semio_framework_async_macros::async_test]
async fn set_camera_zoom_updates_zoom_and_keeps_pan_via_runtime() {
    let mut app = app().await;
    dispatch(&mut app, RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::RasterCamera { x: 4.0, y: 5.0, zoom: 1.0 } })).await;
    let result = dispatch(&mut app, RasterCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { zoom: 3.0 })).await;
    assert!(result.mutations.is_empty(), "camera zoom is a view action and emits no operations");
    let text = packed_scene_text(&render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await);
    assert!(text.contains(r#""zoom":3.0"#), "zoom updated: {text}");
    assert!(text.contains(r#""x":4.0"#), "pan preserved across zoom-only update: {text}");
}

#[semio_framework_async_macros::async_test]
async fn add_layer_action_appends_and_undo_removes() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot").layers.len();
    dispatch(&mut app, RasterCommand::AddLayer(add_layer::AddLayer { kind: "group".into() })).await;
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
    dispatch(&mut app, RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() })).await;
    let layer_id = crate::standards::v1::subsets::any::schema::layer_node_id(&app.snapshot().expect("snapshot").layers[0]).to_string();
    dispatch(&mut app, RasterCommand::PatchLayer(patch_layer::PatchLayer { layer_id: layer_id.clone(), field: "name".into(), value: "Renamed".into() })).await;
    assert_eq!(layer_name(&app.snapshot().expect("snapshot").layers[0]), "Renamed");
    dispatch(&mut app, RasterCommand::ToggleLayerVisible(toggle_layer_visible::ToggleLayerVisible { layer_id })).await;
    assert!(!layer_visible(&app.snapshot().expect("snapshot").layers[0]));
    history(&mut app, "undo").await;
    assert!(layer_visible(&app.snapshot().expect("snapshot").layers[0]));
}

#[semio_framework_async_macros::async_test]
async fn move_layer_into_group() {
    let mut app = app().await;
    // 🌱️ The mounted store boots on the empty shell: both the group and the pixel it receives are
    // planted here.
    dispatch(&mut app, RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() })).await;
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
    context::retire_raster_envelope(base_envelope);
    instance_a.load_document_pack(&base_files).await.expect("load a");
    instance_b.load_document_pack(&base_files).await.expect("load b");
    let background_id = "bg".to_string();
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://raster-convergence", "mem://raster-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");

    dispatch(&mut instance_a, RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() })).await;
    dispatch(&mut instance_b, RasterCommand::PatchLayer(patch_layer::PatchLayer { layer_id: background_id, field: "name".into(), value: "Renamed By B".into() })).await;

    // 🔀️ Publishing an edit only PUTS it on the backbone; a replica folds what the other side put
    // there when it ticks. `commitCheckpoint` is a history verb on the LOCAL store and never pulled
    // anything in, so without these two ticks each instance still read only its own edit — the exact
    // exchange `artifact_app_laws::assert_two_registered_instances_converge` performs for the apps
    // whose genesis document is enough to carry both commands.
    instance_a.tick_backbone().await.expect("a folds b's events");
    instance_b.tick_backbone().await.expect("b folds a's events");

    let projection_a = instance_a.snapshot().expect("projection a");
    let projection_b = instance_b.snapshot().expect("projection b");
    assert_eq!(projection_a.layers.len(), 2, "A keeps its added layer");
    assert_eq!(projection_b.layers.len(), 2, "B converges on A's added layer");
    assert_eq!(layer_name(&projection_a.layers[0]), "Renamed By B", "A converges on B's rename");
    assert_eq!(layer_name(&projection_b.layers[0]), "Renamed By B", "B keeps its rename");

    // 🔀️ A replicated CHECKPOINT keeps both sides converged too — the second half of the framework's
    // own convergence law, run here against the shared seeded base.
    let local = artifact_app_laws::meta("local");
    let admitted = instance_a.handle_action("commitCheckpoint", None, &local).await.expect("a commits a checkpoint");
    semio_framework_plugin::plugin_app_close_prelude::settle_framework_reserved_admission(&mut *instance_a, admitted).await.expect("a's checkpoint commit settles");
    artifact_app_laws::settle_registered_typed_operation(&mut *instance_a, local.instance_id).await.expect("a's checkpoint publication settles");
    instance_b.tick_backbone().await.expect("b folds a's checkpoint");
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
        RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() }),
        |app| app.snapshot().unwrap().layers.len(),
    )
    .await;
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
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(boot);
    // 🖼️ A REAL PNG — the crate's own committed emblem. The former `"aGVsbG8="` ("hello") decoded to
    // five bytes with no PNG signature, so `mint_raster_asset_child` fell through to its content-less
    // fallback and the law proved nothing about the pixels an import is for.
    let media = Media {
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        payload: MediaPayload::Structured { schema: "2d.image".into(), json: base64_codec::base64_standard_encode(crate::examples::art_raster_demo::emblem_image_asset().data) },
    };
    // 🔌️ A MOUNTED app's `InvocationResult.mutations` is always EMPTY (the host settles the typed
    // publication), so the import is read off the DOCUMENT, exactly as sequence/animate read theirs.
    app.import_media("image:in", media, &artifact_app_laws::meta("local")).await.expect("import image:in");
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
    let pixels = crate::raster_asset(&imported.assets, &image_key).is_some_and(|asset| asset.mime == "image/png" && !asset.data.is_empty());
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(imported);
    assert_eq!(layers, before + 1, "image:in must append one pixel layer");
    assert!(pixels, "the imported asset resolves to real pixels");
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
/// fifteen ids — the exact join `validate_tool_job_rows` demands
/// (`interactive-job.catalog-authority` / `interactive-job.catalog-incomplete`). Mirrors block2d's
/// `retained_route_dispositions_are_exact_and_exhaustive`.
#[semio_framework_async_macros::async_test]
async fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};
    use std::collections::BTreeSet;
    assert_eq!(RASTER_RETAINED_TOOL_IDS.len(), 15);
    assert_eq!(<RasterPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 15);
    assert_eq!(RasterRetainedCommandJobFactory::PUBLICATION_CONTRACTS.len(), 15);
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
        let action = definition
            .actions
            .iter()
            .chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter()))
            .find(|action| action.id == *tool_id)
            .unwrap_or_else(|| panic!("action {tool_id} declared"));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id} must be UI-dispatchable");
    }
    // ⚖️ No declaration in raster's catalog may stay `Unclassified` — that is the release-blocking
    // gate `validate_interactive_job_classification` enforces. Every action lands on a window kind
    // (both app-declared and framework-injected ones), so this sweep sees the whole action surface.
    for action in definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())) {
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
    let migrated_ids: BTreeSet<&str> = definition
        .actions
        .iter()
        .chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter()))
        .filter(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated)
        .map(|action| action.id.as_str())
        .collect();
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
    assert_eq!(commands.len(), 15, "every RasterCommand row must be covered by every_command()");
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

//#region 🔖️ActionBridge
/// 🌉️ Every command row the shells reach by action id must decode through `command_from_action`
/// (camelCase host keys → the payloads' own snake_case `FromValue` names) and its `command_id` must
/// round-trip — the boundary that was missing before the 2026-09-16 boot wave of ticket
/// 26/09/05/RASTER-PLUGIN-END-TO-END (every shell action was refused as "not framework-reserved").
#[test]
fn command_from_action_round_trips_every_command_id() {
    for command in every_command() {
        let id = command.command_id();
        let args = dsl::ToValue::to_value(&command);
        // 🔁️ The `DslOps` wire shape is `{keyword: payload}`; the shell sends the bare payload object.
        let payload = match &args {
            dsl::DslValue::Object(entries) if entries.len() == 1 => entries[0].1.clone(),
            other => other.clone(),
        };
        let camel = camel_case_keys(&payload);
        let bridged = <RasterPlayApp as semio_framework_plugin::ArtifactEditor>::command_from_action(id, Some(&camel)).unwrap_or_else(|error| panic!("action {id} failed to bridge: {}", error.message));
        assert_eq!(bridged.command_id(), id, "command_id mismatch for action {id}");
        assert_eq!(bridged, command, "payload mismatch for action {id}");
    }
    assert!(<RasterPlayApp as semio_framework_plugin::ArtifactEditor>::command_from_action("nonsense", None).is_err());
}

/// 🐫️ The shell's spelling of the payload keys.
fn camel_case_keys(value: &dsl::DslValue) -> dsl::DslValue {
    match value {
        dsl::DslValue::Object(entries) => dsl::DslValue::Object(
            entries
                .iter()
                .map(|(key, value)| {
                    let mut camel = String::new();
                    let mut upper = false;
                    for ch in key.chars() {
                        if ch == '_' { upper = true; } else if upper { camel.push(ch.to_ascii_uppercase()); upper = false; } else { camel.push(ch); }
                    }
                    (camel, value.clone())
                })
                .collect(),
        ),
        other => other.clone(),
    }
}

/// 🧱️ The tree/host verbs' own spellings (bare `id`, `exampleId`, the Paint2dHost's `{camera:{…}}`)
/// reach the same rows.
#[test]
fn command_from_action_bridges_host_spellings() {
    let args = |json: &str| dsl::os_pack::json_to_dsl_value(&dsl::os_pack::json::parse(json).expect("fixture JSON"));
    let bridge = |action: &str, json: &str| <RasterPlayApp as semio_framework_plugin::ArtifactEditor>::command_from_action(action, Some(&args(json))).expect(action);
    assert_eq!(bridge("setActiveExample", r#"{"exampleId":"demo"}"#), RasterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }));
    assert_eq!(bridge("deleteLayer", r#"{"id":"layer-1"}"#), RasterCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: "layer-1".into() }));
    assert_eq!(bridge("setCamera", r#"{"camera":{"x":1,"y":2,"zoom":3}}"#), RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::RasterCamera { x: 1.0, y: 2.0, zoom: 3.0 } }));
    assert_eq!(bridge("setCompositeViewport", r#"{"width":640,"height":480}"#), RasterCommand::SetCompositeViewport(set_composite_viewport::SetCompositeViewport { width: 640.0, height: 480.0 }));
    assert_eq!(bridge("addLayer", r#"{"kind":"pixel"}"#), RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() }));
}
//#endregion 🔖️ActionBridge

//#region 🔖️MountedBoot
/// 🧪️ The registered, MOUNTED app the react shell drives — bound to instance `1` so typed commands
/// reach their retained routes (the registry-less `context::app()` rejects every tool row, see
/// memory `project-registryless-testkit-new-app-unusable`), settled after each dispatch, and retired
/// through the framework's exact close loop (the store's `Drop` demands the terminal-empty witness).
pub(crate) mod mounted {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

    pub const RASTER_TEST_INSTANCE: u32 = 1;

    pub struct MountedRasterApp(VcsArtifactApp<EditorApp<RasterPlayApp>>);

    impl std::ops::Deref for MountedRasterApp {
        type Target = VcsArtifactApp<EditorApp<RasterPlayApp>>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for MountedRasterApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for MountedRasterApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0);
            }
        }
    }

    fn manifest() -> App {
        App { definition: create_raster_app(), examples: Vec::new() }
    }

    pub fn mounted_app() -> MountedRasterApp {
        let mut app = semio_framework_plugin::resolve_ready(new_app_with_registry::<EditorApp<RasterPlayApp>>(manifest));
        semio_framework_plugin::resolve_ready(app.bind_instance_id(RASTER_TEST_INSTANCE));
        MountedRasterApp(app)
    }

    pub async fn dispatch(app: &mut MountedRasterApp, command: RasterCommand) -> InvocationResult {
        let id = "raster-composite";
        let mut action = meta("local");
        action.view_state = Some(ViewModel { window_id: Some(id.into()), window_instances: vec![ViewWindowInstance { id: id.into(), window_kind_id: composite::RASTER_PLAY_WINDOW_COMPOSITE.into() }], ..Default::default() });
        let mut result = app.dispatch_typed(command, &action).await.expect("dispatch");
        let settled = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, RASTER_TEST_INSTANCE).await.expect("settle the typed operation");
        result.requested_effects.extend(settled.effects);
        result
    }

    /// ⏪️ One framework-reserved history verb (`undo`/`redo`) driven the whole way a shell drives it:
    /// the admission comes back as a `SpawnJob` receipt the caller must commit before the store ever
    /// sees `ArtifactCommand::Undo`, and only then does the typed publication settle. Pressing it
    /// without that ladder reads back as a silent no-op.
    pub async fn history(app: &mut MountedRasterApp, action: &str) {
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app.0, action, RASTER_TEST_INSTANCE).await;
    }
}

/// 🔡️ The composite scene is a packed record (`"bytes":[…]` arrays in the projected tree) — decodes
/// every byte array lossily and returns the flattened text.
fn packed_scene_text(json: &str) -> String {
    let mut text = String::new();
    let mut rest = json;
    while let Some(start) = rest.find("\"bytes\":[") {
        let body = &rest[start + 9..];
        let end = body.find(']').unwrap_or(body.len());
        let bytes: Vec<u8> = body[..end].split(',').filter_map(|token| token.trim().parse::<u8>().ok()).collect();
        text.push_str(&String::from_utf8_lossy(&bytes));
        rest = &body[end..];
    }
    text
}

/// 🚀️ The react shell's boot sequence: the store boots on the empty shell and the shell replays
/// `setActiveExample demo`, which must plant the Semio-logo carrier (two root layers) through the
/// retained route without any owned-map clone or un-retired drop (react boots of 2026-09-16).
#[semio_framework_async_macros::async_test]
async fn mounted_boot_replays_the_demo_example_through_the_retained_route() {
    let mut app = mounted::mounted_app();
    assert!(app.snapshot().expect("snapshot").layers.is_empty(), "the store boots on the empty shell");
    mounted::dispatch(&mut app, RasterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::art_raster_demo::ID.into() })).await;
    // 🔁️ The boot replay over an already-demo document is a no-op (no second history patch). Emblem
    // pixels and layer structure are covered by `example_media_operations` and `boot_document` tests.
    let second = mounted::dispatch(&mut app, RasterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::art_raster_demo::ID.into() })).await;
    assert!(second.requested_effects.is_empty(), "re-selecting the boot example must not rewrite the document");
    // 🖼️ The shell's first publication after the boot replay renders the composite window over the
    // demo document, whose asset pool now holds the planted emblem: the scene projection must read
    // the populated owned map through its entries (play-grid boot trap, 2026-09-19).
    let composite = packed_scene_text(&render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await);
    // 🖼️ The pixels themselves, not just the lane: `Paint2dHost` uploads one texture per `assetsJson`
    // entry, so a boot whose asset pool holds handles WITHOUT their materialization renders an empty
    // canvas even though every lane, layer row and viewport looks right (play pane measured blank on
    // :6033, 2026-09-21 — `assetsJson` parsed as `{}`, zero textures uploaded). The scene's own
    // projection is read here because the published surface carries both JSON lanes out of band.
    let booted = app.snapshot().expect("snapshot");
    let scene = raster_scene(&booted, &crate::editor::raster::config::RasterConfig::default(), "brush", "composite");
    let assets_json = scene.assets_json.clone();
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(booted);
    std::mem::forget(app);
    assert!(composite.contains("composite") && composite.contains("documentSync"), "the composite window publishes its document-sync lane: {composite}");
    assert!(assets_json.contains("semio-emblem") && assets_json.contains("image/png"), "the booted document's asset pool must resolve to real pixels, not pixel-less handles: {assets_json}");
}

/// ⚖️ LAW — the play pane's ACTUAL boot, read off the PUBLISHED surface rather than off
/// `raster_scene`. The law above reads the scene as the app builds it, i.e. BEFORE `scene_surface`
/// splits it into the fixed-capacity `SurfaceProps.doc` spine plus one `paged_text_carrier` child per
/// declared lane. Everything that can go wrong between those two points — the guest's UI arena
/// refusing the carrier, a lane dropped from the manifest, a carrier that reassembles to something
/// other than what the manifest promised — was therefore invisible to this suite while the pane on
/// :6033 rendered an empty composite with ZERO console errors: `Paint2dHost` never called `atob`,
/// meaning `parsePaint2dAssets` got `{}` and uploaded no texture
/// (`.🧬semio/🦑️repo/⚡️cache/play-fleet/raster/browser9/page.png`, 2026-09-22).
///
/// The two assertions are deliberately split so a failure says WHICH half broke:
/// the spine's own `SceneLaneRef` manifest is what the GUEST measured before paging, and the
/// reassembled `assets_json` is what a host gets back out of the carrier.
#[semio_framework_async_macros::async_test]
async fn mounted_boot_publishes_the_emblem_pixels_on_the_composite_assets_lane() {
    use semio_framework_plugin::Paint2dScene;

    let mut app = mounted::mounted_app();
    mounted::dispatch(&mut app, RasterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::art_raster_demo::ID.into() })).await;
    let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await;
    std::mem::forget(app);

    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<Paint2dScene>(&json).expect("the composite window publishes a paint-2d surface");
    let manifest = scene.lanes.iter().find(|lane| lane.lane == "assets").unwrap_or_else(|| panic!("the published spine declares an assets lane: {:?}", scene.lanes));
    assert!(manifest.bytes > 8 * 1_024, "the GUEST measured real emblem pixels onto the assets lane before paging, got {} bytes: {}", manifest.bytes, scene.assets_json);

    assert_eq!(scene.assets_json.len() as u32, manifest.bytes, "the assets carrier reassembles to exactly the payload the lane manifest promised");
    let assets: Value = dsl::os_pack::json::parse(&scene.assets_json).expect("assets json");
    let entry = assets.get("semio-emblem").unwrap_or_else(|| panic!("the published assets lane names the emblem: {}", scene.assets_json));
    assert_eq!(entry.get("mime").and_then(Value::as_str), Some("image/png"));
    let data = entry.get("data").and_then(Value::as_str).expect("the emblem's base64 payload");
    // 🖼️ A NON-TRIVIAL image, read off the PNG's own IHDR (bytes 16..24, big-endian) after decoding the
    // exact base64 the host would hand `atob`. The curated demo used to publish a 75-byte 2×2 swatch,
    // which travelled the whole lane correctly and still drew four pixels — indistinguishable in the
    // browser from the blank pane the missing `paint-2d` lane route caused.
    let bytes = base64_codec::base64_standard_decode(data.as_bytes()).expect("the emblem payload is valid base64");
    assert!(bytes.len() > 8 * 1_024, "the emblem's payload is a real image, got {} bytes", bytes.len());
    let width = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    let height = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
    assert!(width >= 256 && height >= 256, "the published emblem is large enough to see: {width}x{height}");
}

/// ↩️ The five-clause bar's undo/redo half on the LIVE document shape: boot the demo carrier, paint
/// a layer, then walk `undo → redo → undo`. Every step displaces a projection that owns the demo's
/// asset pool, so each one must reach its owner instead of `RasterOwnedMap`'s fail-closed `Drop`
/// (slice B3e fixed the undo half; the redo press still aborted the guest at `🦀️.rs:301` on the
/// live bar, measured 2026-09-20 by B3f).
/// 📸️ Reads the live layer count and hands the materialized projection straight back to the
/// artifact's own retirement seam — `PluginApp::snapshot` CLONES, and a clone of a demo-shaped
/// document owns a populated asset pool, so a test that lets it fall off the end of a statement
/// trips the same fail-closed `Drop` the law below is about.
fn observed_layer_count(app: &mounted::MountedRasterApp) -> usize {
    let snapshot = app.snapshot().expect("snapshot");
    let count = snapshot.layers.len();
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
    count
}

#[semio_framework_async_macros::async_test]
async fn mounted_paint_undo_redo_undo_never_reaches_an_owned_map_drop() {
    let mut app = mounted::mounted_app();
    mounted::dispatch(&mut app, RasterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::art_raster_demo::ID.into() })).await;
    let planted = observed_layer_count(&app);
    assert!(planted > 0, "the demo carrier plants its layer forest");
    mounted::dispatch(&mut app, RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() })).await;
    assert_eq!(observed_layer_count(&app), planted + 1, "the painted layer lands");
    mounted::history(&mut app, "undo").await;
    assert_eq!(observed_layer_count(&app), planted, "undo withdraws the painted layer");
    mounted::history(&mut app, "redo").await;
    assert_eq!(observed_layer_count(&app), planted + 1, "redo reinstates the painted layer");
    mounted::history(&mut app, "undo").await;
    assert_eq!(observed_layer_count(&app), planted, "the second undo withdraws it again");
    std::mem::forget(app);
}

//#endregion 🔖️MountedBoot

/// 🎥 The config lane end to end: the Paint2dHost's boot `setCompositeViewport` and a wheel
/// `setCamera` publish into the config store through the retained route (`raster-boot-5` refused
/// both with "batched item candidate failed its exact fixed fold contract" while the config
/// `preflight` under-declared its work items).
#[semio_framework_async_macros::async_test]
async fn mounted_config_lane_publishes_viewport_and_camera() {
    let mut app = mounted::mounted_app();
    mounted::dispatch(&mut app, RasterCommand::SetCompositeViewport(set_composite_viewport::SetCompositeViewport { width: 1024.0, height: 807.0 })).await;
    mounted::dispatch(&mut app, RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::RasterCamera { x: 12.0, y: -4.0, zoom: 2.0 } })).await;
    let text = packed_scene_text(&render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await);
    assert!(text.contains("\"zoom\":2") || text.contains("zoom=2"), "the wheel camera must land in the config store and reach the composite scene: {text}");
    assert!(text.contains("1024"), "the boot viewport must land in the config store: {text}");
}
