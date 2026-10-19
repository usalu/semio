/// 📥️ Retained envelope ingress, io ports, window measures and scene rendering laws.

//#region 🔖️RetainedEnvelopeIngress
/// 📨️ One live document-envelope wire, built from the artifact's own empty output shell — the only
/// snapshot `ArtifactPack::encode_pack` admits, since a populated Raster document reaches output
/// exclusively through the retained page authority. The envelope the wire describes is retired here
/// through the same owner bundle the app's decode hook installs, one bounded grant per turn.
fn raster_envelope_wire() -> Vec<u8> {
    use pack::snapshot::ArtifactPack;

    let snapshot = semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot();
    let snapshot_pack = snapshot.encode_pack();
    let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let wire = semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
        ("schema".to_string(), Value::String(RASTER_DOCUMENT_SCHEMA.to_string())),
        ("id".to_string(), Value::String("raster-live-load".to_string())),
        (
            "vcs".to_string(),
            semio_framework_pack_json::object([
                ("initialPack".to_string(), Value::String(snapshot_hex)),
                ("edits".to_string(), semio_framework_pack_json::array([])),
                ("changes".to_string(), semio_framework_pack_json::array([])),
                ("checkpoints".to_string(), semio_framework_pack_json::array([])),
                ("alternatives".to_string(), semio_framework_pack_json::array([])),
            ]),
        ),
        ("editMessages".to_string(), semio_framework_pack_json::array([])),
        ("conflicts".to_string(), semio_framework_pack_json::array([])),
    ]))
    .into_bytes();
    let envelope = store::os_store::document_materialize::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster-live-load", snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    semio_s_artifact_raster_raster_model::close_raster_owner_cold(semio_s_artifact_raster_raster_host::raster_envelope_decode_owner_bundle().retire_envelope(envelope));
    wire
}

/// 🎟️ Reserves page/byte credits first, then feeds the wire as fixed-size pages and seals — the
/// caller never holds a growable buffer and never sees the store the decode will publish into.
fn admit_raster_envelope(app: &mut RasterAppFixture, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Raster live envelope ingress credits");
    for chunk in wire.chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Raster live envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Raster live envelope page admission failed: {}: {}", fault.code.0, fault.message));
    }
    assert!(app.0.seal_artifact_envelope_ingress(handle, app.1.cx()).expect("Raster live envelope seal/submit"));
    handle
}

/// 🔄️ Pumps the load one bounded maintenance turn at a time and polls after each — never inline,
/// never unbounded.
fn drive_raster_live_load(app: &mut RasterAppFixture, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..100_000 {
        app.0.maintenance_step(app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("one Raster live maintenance turn");
        let poll = app.0.advance_artifact_envelope_load(handle, app.1.cx()).expect("Raster live load advancement");
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
    let pages = wire.len().div_ceil(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled Raster ingress credits");
    let first = &wire[..wire.len().min(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
    let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    bytes[..first.len()].copy_from_slice(first);
    let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled Raster first page");
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
async fn window_measures_expose_brush_eraser_and_bucket_option_groups() {
    let mut app = app().await;
    let measures = main_window_measures(&mut app).await;
    assert_eq!(measures.len(), 3);
    for utility in ["paintBrush", "paintEraser", "paintBucket"] {
        assert!(measures.iter().any(|m| matches!(m, WindowMeasure::Group { id, .. } if id == &format!("raster-utility-options-{utility}"))), "{utility} option group");
    }
}

#[semio_framework_async_macros::async_test]
async fn renders_raster_scene() {
    let mut app = app().await;
    let json = render(&mut app, semio_s_artifact_raster_raster_editor::modes::edit::windows::composite::RASTER_PLAY_BODY_COMPOSITE).await;
    assert!(json.contains("raster"));
}

#[semio_framework_async_macros::async_test]
async fn renders_navigator_scene() {
    let mut app = app().await;
    let json = render(&mut app, semio_s_artifact_raster_raster_editor::modes::edit::windows::navigator::RASTER_PLAY_BODY_NAVIGATOR).await;
    // 🧾️ A `scene_surface` node publishes its payload as a PACKED document (`"kind":"paint-2d"`,
    // `"docSchema":"paint-2d@1"`, `doc.bytes`), never as inline tree text.
    assert!(json.contains("\"kind\":\"paint-2d\""), "{json}");
    assert!(packed_scene_text(&json).contains("navigator"), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn parses_raster_image_test_snapshot() {
    let document = semio_s_artifact_raster_raster_io::testing::raster_image_test_snapshot();
    assert!(!document.layers.is_empty());
    semio_s_artifact_raster_raster_model::retire_raster_value_cold(document);
}

#[semio_framework_async_macros::async_test]
async fn empty_document_background_layer_has_identity_scale() {
    let document = empty_raster_document();
    let json = document_sync_json(&document);
    assert!(json.contains(r#""a":1.0"#), "expected identity scale in {json}");
    assert!(json.contains(r#""d":1.0"#), "expected identity scale in {json}");
    assert!(!json.contains(r#""a":0.0"#), "layer must not collapse to zero size");
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
    assert!(properties_json.contains("raster-inspector.foreground.input"));
    assert!(properties_json.contains("Foreground"));
    assert!(properties_json.contains("setBrushColor"));
}

#[semio_framework_async_macros::async_test]
async fn raster_labels_resolve_german_locale() {
    let mut app = app().await;
    let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::De, ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Native) };
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
    let json = render(&mut app, semio_s_artifact_raster_raster_editor::modes::edit::windows::composite::RASTER_PLAY_BODY_COMPOSITE).await;
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
    let document = semio_s_artifact_raster_raster_io::testing::raster_image_test_snapshot();
    let sync_json = document_sync_json(&document);
    assert!(!sync_json.contains("\"assets\""), "sync json must omit assets");
    assert!(sync_json.contains("\"params\""), "adjustment params must survive document→sync roundtrip for the paint host");
    let sync_value: Value = semio_framework_pack_json::parse(&sync_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("sync json");
    let layers = sync_value.get("layers").and_then(Value::as_array).expect("layers");
    assert!(layers.iter().any(|layer| layer.get("kind").and_then(Value::as_str) == Some("adjustment") && layer.get("params").is_some()));
    assert!(document.assets.contains_key("semio-emblem"));
    semio_s_artifact_raster_raster_model::retire_raster_value_cold(document);
}
