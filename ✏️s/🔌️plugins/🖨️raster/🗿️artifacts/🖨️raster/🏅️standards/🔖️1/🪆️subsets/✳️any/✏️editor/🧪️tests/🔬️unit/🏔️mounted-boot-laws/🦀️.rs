/// 🔬️ Mounted boot laws: 1 test covering structural commands refuse protected sources and destination parents.
use semio_framework_test_fixtures::fixture_text;

/// 🏔️ The mounted app boot, history, selection and close laws.

//#region 🔖️MountedBoot
/// 🧪️ The registered, MOUNTED app the react shell drives — bound to instance `1` so typed commands
/// reach their retained routes (the registry-less `context::app()` rejects every tool row, see
/// memory `project-registryless-testkit-new-app-unusable`), settled after each dispatch, and retired
/// through the framework's exact close loop (the store's `Drop` demands the terminal-empty witness).
pub mod mounted {
    use super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

    pub const RASTER_TEST_INSTANCE: u32 = 1;

    /// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
    pub struct MountedRasterApp(pub VcsArtifactApp<EditorApp<RasterPlayApp>>, pub semio_framework_plugin::artifact_app_laws::OriginalFixtureScope);

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
                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");
            }
        }
    }

    fn manifest() -> App {
        App { definition: create_raster_app(), examples: Vec::new() }
    }

    pub fn mounted_app() -> MountedRasterApp {
        let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
        let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry::<EditorApp<RasterPlayApp>>(manifest, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()));
        ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(RASTER_TEST_INSTANCE));
        MountedRasterApp(app, scope)
    }

    pub async fn dispatch(app: &mut MountedRasterApp, command: RasterCommand) -> InvocationResult {
        let id = "raster-composite";
        let mut action = meta("local");
        action.view_state = Some(ViewModel { window_id: Some(id.into()), window_instances: vec![ViewWindowInstance { id: id.into(), window_kind_id: semio_s_artifact_raster_raster_editor::modes::edit::windows::composite::RASTER_PLAY_WINDOW_COMPOSITE.into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) });
        let mut result = app.0.dispatch_typed(command, &action, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");
        let settled = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, RASTER_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("settle the typed operation");
        result.requested_effects.extend(settled.effects);
        result
    }

    /// ⏪️ One framework-reserved history verb (`undo`/`redo`) driven the whole way a shell drives it:
    /// the admission comes back as a `SpawnJob` receipt the caller must commit before the store ever
    /// sees `ArtifactCommand::Undo`, and only then does the typed publication settle. Pressing it
    /// without that ladder reads back as a silent no-op.
    pub async fn history(app: &mut MountedRasterApp, action: &str) {
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app.0, action, RASTER_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
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
    mounted::dispatch(&mut app, RasterCommand::SetActiveExample(semio_s_artifact_raster_raster_editor::commands::set_active_example::SetActiveExample { example_id: semio_s_artifact_raster_raster_examples::art_raster_demo::ID.into() })).await;
    // 🔁️ The boot replay over an already-demo document is a no-op (no second history patch). Emblem
    // pixels and layer structure are covered by `example_media_operations` and `boot_document` tests.
    let second = mounted::dispatch(&mut app, RasterCommand::SetActiveExample(semio_s_artifact_raster_raster_editor::commands::set_active_example::SetActiveExample { example_id: semio_s_artifact_raster_raster_examples::art_raster_demo::ID.into() })).await;
    assert!(second.requested_effects.is_empty(), "re-selecting the boot example must not rewrite the document");
    // 🖼️ The shell's first publication after the boot replay renders the composite window over the
    // demo document, whose asset pool now holds the planted emblem: the scene projection must read
    // the populated owned map through its entries (play-grid boot trap, 2026-09-19).
    let composite = packed_scene_text(&render(&mut app, semio_s_artifact_raster_raster_editor::modes::edit::windows::composite::RASTER_PLAY_BODY_COMPOSITE).await);
    // 🖼️ The pixels themselves, not just the lane: `Paint2dHost` uploads one texture per `assetsJson`
    // entry, so a boot whose asset pool holds handles WITHOUT their materialization renders an empty
    // canvas even though every lane, layer row and viewport looks right (play pane measured blank on
    // :6033, 2026-09-21 — `assetsJson` parsed as `{}`, zero textures uploaded). The scene's own
    // projection is read here because the published surface carries both JSON lanes out of band.
    let booted = app.snapshot().expect("snapshot");
    let scene = raster_scene(&booted, &semio_s_artifact_raster_raster_editor::config::RasterConfig::default(), "brush", "composite", &[], None);
    let assets_json = scene.assets_json.clone();
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(booted);
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
    mounted::dispatch(&mut app, RasterCommand::SetActiveExample(semio_s_artifact_raster_raster_editor::commands::set_active_example::SetActiveExample { example_id: semio_s_artifact_raster_raster_examples::art_raster_demo::ID.into() })).await;
    let json = render(&mut app, semio_s_artifact_raster_raster_editor::modes::edit::windows::composite::RASTER_PLAY_BODY_COMPOSITE).await;
    std::mem::forget(app);

    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<Paint2dScene>(&json).expect("the composite window publishes a paint-2d surface");
    let manifest = scene.lanes.iter().find(|lane| lane.lane == "assets").unwrap_or_else(|| panic!("the published spine declares an assets lane: {:?}", scene.lanes));
    assert!(manifest.bytes > 8 * 1_024, "the GUEST measured real emblem pixels onto the assets lane before paging, got {} bytes: {}", manifest.bytes, scene.assets_json);

    assert_eq!(scene.assets_json.len() as u32, manifest.bytes, "the assets carrier reassembles to exactly the payload the lane manifest promised");
    let assets: Value = semio_framework_pack_json::parse(&scene.assets_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("assets json");
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
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(snapshot);
    count
}

#[semio_framework_async_macros::async_test]
async fn mounted_paint_undo_redo_undo_never_reaches_an_owned_map_drop() {
    let mut app = mounted::mounted_app();
    mounted::dispatch(&mut app, RasterCommand::SetActiveExample(semio_s_artifact_raster_raster_editor::commands::set_active_example::SetActiveExample { example_id: semio_s_artifact_raster_raster_examples::art_raster_demo::ID.into() })).await;
    let planted = observed_layer_count(&app);
    assert!(planted > 0, "the demo carrier plants its layer forest");
    mounted::dispatch(&mut app, RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer { kind: "pixel".into() })).await;
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
    mounted::dispatch(&mut app, RasterCommand::SetCompositeViewport(semio_s_artifact_raster_raster_editor::commands::set_composite_viewport::SetCompositeViewport { width: 1024.0, height: 807.0 })).await;
    mounted::dispatch(&mut app, RasterCommand::SetCamera(semio_s_artifact_raster_raster_editor::commands::set_camera::SetCamera { camera: semio_s_artifact_raster_raster_model::RasterCamera { x: 12.0, y: -4.0, zoom: 2.0 } })).await;
    let text = packed_scene_text(&render(&mut app, semio_s_artifact_raster_raster_editor::modes::edit::windows::composite::RASTER_PLAY_BODY_COMPOSITE).await);
    assert!(text.contains("\"zoom\":2") || text.contains("zoom=2"), "the wheel camera must land in the config store and reach the composite scene: {text}");
    assert!(text.contains("1024"), "the boot viewport must land in the config store: {text}");
}

#[semio_framework_async_macros::async_test]
async fn layer_tree_and_scene_keep_exact_domain_selection_ids() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧫️fixtures/🖱️selection/🔣️.json")).unwrap();
    let selected: Vec<String> = serde_json::from_value(fixture["selectedIds"].clone()).unwrap();
    let hovered = fixture["hoveredId"].as_str();
    let mut document = empty_raster_document();
    let layer: RasterLayerNode = semio_framework_pack_json::from_json_str(&serde_json::json!({"kind":"pixel","id":selected[0],"name":fixture["name"],"mask":null,"width":32,"height":32,"imageKey":null}).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(layer_row_id(&layer), selected[0]);
    document.layers.push(layer);
    let scene = raster_scene(&document, &RasterConfig::default(), "paintBrush", "composite", &selected, hovered);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.selection_json).unwrap(), fixture["selectedIds"]);
    assert_eq!(scene.hovered_id.as_deref(), hovered);
    semio_s_artifact_raster_raster_model::retire_raster_value_cold(document);
}

#[semio_framework_async_macros::async_test]
async fn inspector_patch_values_preserve_numeric_names_and_typed_controls() {
    let mut app = app().await;
    dispatch(&mut app, RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer { kind: "pixel".into() })).await;
    let snapshot = app.snapshot().unwrap();
    let id = semio_s_artifact_raster_raster_model::layer_node_id(&snapshot.layers[0]).to_string();
    semio_s_artifact_raster_raster_model::retire_raster_value_cold(snapshot);
    for (field, value) in [("name", serde_json::json!("123")), ("opacity", serde_json::json!(0.25)), ("visible", serde_json::json!(false))] {
        let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&serde_json::json!({"layerIds":[id],"field":field,"value":value}).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
        let command = <RasterPlayApp as ArtifactEditor>::command_from_action("patchLayers", Some((args).clone()), None).unwrap();
        dispatch(&mut app, command).await;
    }
    let snapshot = app.snapshot().unwrap();
    let layer = semio_s_artifact_raster_raster_model::find_layer(&snapshot.layers, &id).unwrap();
    assert_eq!(semio_s_artifact_raster_raster_model::layer_name(layer), "123");
    assert_eq!(semio_s_artifact_raster_raster_model::layer_opacity(layer), 0.25);
    assert!(!semio_s_artifact_raster_raster_model::layer_visible(layer));
    semio_s_artifact_raster_raster_model::retire_raster_value_cold(snapshot);
}

#[semio_framework_async_macros::async_test]
async fn selection_mask_retained_publication_survives_undo_and_redo() {
    use semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot;
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🎮️commands/🎭️mask-from-selection/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut app=context::app().await;
    context::dispatch(&mut app,RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer {kind:"pixel".into()})).await;
    let snapshot=app.snapshot().unwrap();
    let id=semio_s_artifact_raster_raster_model::layer_node_id(&snapshot.layers[0]).to_owned();
    retire_raster_snapshot(snapshot);
    for field in ["width","height"] {
        context::dispatch(&mut app,RasterCommand::PatchLayer(semio_s_artifact_raster_raster_editor::commands::patch_layer::PatchLayer {layer_id:id.clone(),field:field.into(),value:fixture[field].to_string()})).await;
    }
    context::dispatch(&mut app,RasterCommand::MaskFromSelection(semio_s_artifact_raster_raster_editor::commands::mask_from_selection::MaskFromSelection {layer_id:id,expected_image_key:None,selection:fixture["selection"].to_string()})).await;
    let snapshot=app.snapshot().unwrap();
    let semio_s_artifact_raster_raster_model::RasterLayerNode::Pixel {mask:Some(mask),image_key:None,..}=&snapshot.layers[0] else {panic!("mask creation must preserve source pixels")};
    let key=mask.image_key.clone().unwrap();
    let asset=semio_s_artifact_raster_raster_io::raster_asset(&snapshot.assets,&key).unwrap();
    assert_eq!(serde_json::to_value(semio_framework_pixels::decode_png(&asset.data).unwrap().pixels).unwrap(),fixture["expectedRgba"]);
    retire_raster_snapshot(snapshot);
    context::history(&mut app,"undo").await;
    let snapshot=app.snapshot().unwrap();
    assert!(matches!(&snapshot.layers[0],semio_s_artifact_raster_raster_model::RasterLayerNode::Pixel {mask:None,image_key:None,..}));
    assert!(!snapshot.assets.contains_key(&key));retire_raster_snapshot(snapshot);
    context::history(&mut app,"redo").await;
    let snapshot=app.snapshot().unwrap();
    assert!(matches!(&snapshot.layers[0],semio_s_artifact_raster_raster_model::RasterLayerNode::Pixel {mask:Some(mask),image_key:None,..} if mask.image_key.as_ref()==Some(&key)));
    assert!(semio_s_artifact_raster_raster_io::raster_asset(&snapshot.assets,&key).is_some());retire_raster_snapshot(snapshot);
}

#[semio_framework_async_macros::async_test]
async fn adjustment_parameter_controls_publish_and_restore_history() {
    use semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot;
    let mut app=context::app().await;
    context::dispatch(&mut app,RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer {kind:"adjustment".into()})).await;
    let snapshot=app.snapshot().unwrap();let id=semio_s_artifact_raster_raster_model::layer_node_id(&snapshot.layers[0]).to_owned();retire_raster_snapshot(snapshot);
    for (field,value) in [("brightness","0.25"),("contrast","-0.3"),("brightness","null")] {
        context::dispatch(&mut app,RasterCommand::PatchLayer(semio_s_artifact_raster_raster_editor::commands::patch_layer::PatchLayer {layer_id:id.clone(),field:field.into(),value:value.into()})).await;
    }
    let snapshot=app.snapshot().unwrap();
    let semio_s_artifact_raster_raster_model::RasterLayerNode::Adjustment {params,..}=&snapshot.layers[0] else {panic!("adjustment layer")};
    assert!(!params.contains_key("brightness"));assert_eq!(params.get("contrast").and_then(semio_framework_value::DslValue::as_f64),Some(-0.3));retire_raster_snapshot(snapshot);
    context::history(&mut app,"undo").await;
    let snapshot=app.snapshot().unwrap();
    let semio_s_artifact_raster_raster_model::RasterLayerNode::Adjustment {params,..}=&snapshot.layers[0] else {panic!("adjustment layer")};
    assert_eq!(params.get("brightness").and_then(semio_framework_value::DslValue::as_f64),Some(0.25));assert_eq!(params.get("contrast").and_then(semio_framework_value::DslValue::as_f64),Some(-0.3));retire_raster_snapshot(snapshot);
    context::history(&mut app,"redo").await;
    let snapshot=app.snapshot().unwrap();let semio_s_artifact_raster_raster_model::RasterLayerNode::Adjustment {params,..}=&snapshot.layers[0] else {panic!("adjustment layer")};
    assert!(!params.contains_key("brightness"));assert_eq!(params.get("contrast").and_then(semio_framework_value::DslValue::as_f64),Some(-0.3));retire_raster_snapshot(snapshot);
}

#[semio_framework_async_macros::async_test]
async fn flatten_retained_publication_restores_the_original_stack_through_history() {
    use semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot;
    let mut app=context::app().await;
    let initial=app.snapshot().unwrap();let initial_count=initial.layers.len();retire_raster_snapshot(initial);
    for _ in 0..2 {context::dispatch(&mut app,RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer {kind:"pixel".into()})).await;}
    let before=app.snapshot().unwrap();assert_eq!(before.layers.len(),initial_count+2);
    context::dispatch(&mut app,RasterCommand::FlattenLayers(semio_s_artifact_raster_raster_editor::commands::flatten_layers::FlattenLayers {name:"Flattened Image".into()})).await;
    let after=app.snapshot().unwrap();assert_eq!(after.layers.len(),1);
    assert!(matches!(&after.layers[0],RasterLayerNode::Pixel {image_key:Some(_),name,..} if name=="Flattened Image"));
    context::history(&mut app,"undo").await;
    let undone=app.snapshot().unwrap();assert_eq!(undone,before);retire_raster_snapshot(undone);
    context::history(&mut app,"redo").await;
    let redone=app.snapshot().unwrap();assert_eq!(redone,after);retire_raster_snapshot(redone);
    retire_raster_snapshot(before);retire_raster_snapshot(after);
}

#[semio_framework_async_macros::async_test]
async fn merge_down_retained_publication_keeps_other_layers_and_round_trips_history() {
    use semio_s_artifact_raster_raster_model::layer_node_id;
    use semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot;
    let mut app=context::app().await;
    for _ in 0..2 {context::dispatch(&mut app,RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer {kind:"pixel".into()})).await;}
    let before=app.snapshot().unwrap();let id=layer_node_id(before.layers.last().unwrap()).to_owned();
    context::dispatch(&mut app,RasterCommand::MergeDown(semio_s_artifact_raster_raster_editor::commands::merge_down::MergeDown {layer_id:id.clone()})).await;
    let after=app.snapshot().unwrap();assert_eq!(after.layers.len(),before.layers.len()-1);
    assert_eq!(&after.layers[..after.layers.len()-1],&before.layers[..before.layers.len()-2]);
    assert!(matches!(after.layers.last().unwrap(),RasterLayerNode::Pixel {image_key:Some(_),..}));
    assert_eq!(layer_node_id(after.layers.last().unwrap()),id);
    context::history(&mut app,"undo").await;let undone=app.snapshot().unwrap();assert_eq!(undone,before);retire_raster_snapshot(undone);
    context::history(&mut app,"redo").await;let redone=app.snapshot().unwrap();assert_eq!(redone,after);retire_raster_snapshot(redone);
    retire_raster_snapshot(before);retire_raster_snapshot(after);
}

#[semio_framework_async_macros::async_test]
async fn protection_controls_publish_and_restore_retained_history() {
    use semio_s_artifact_raster_raster_model::{layer_locked, layer_node_id};
    use semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot;
    let mut app=context::app().await;
    context::dispatch(&mut app,RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer {kind:"pixel".into()})).await;
    let before=app.snapshot().unwrap();let id=layer_node_id(before.layers.last().unwrap()).to_owned();
    context::dispatch(&mut app,RasterCommand::PatchLayer(semio_s_artifact_raster_raster_editor::commands::patch_layer::PatchLayer {layer_id:id.clone(),field:"locked".into(),value:"true".into()})).await;
    let after=app.snapshot().unwrap();assert!(layer_locked(semio_s_artifact_raster_raster_model::find_layer(&after.layers,&id).unwrap()));
    context::history(&mut app,"undo").await;
    let undone=app.snapshot().unwrap();assert_eq!(undone,before);retire_raster_snapshot(undone);
    context::history(&mut app,"redo").await;
    let redone=app.snapshot().unwrap();assert_eq!(redone,after);retire_raster_snapshot(redone);
    context::dispatch(&mut app,RasterCommand::PatchLayer(semio_s_artifact_raster_raster_editor::commands::patch_layer::PatchLayer {layer_id:id.clone(),field:"locked".into(),value:"false".into()})).await;
    let unlocked=app.snapshot().unwrap();assert_eq!(unlocked,before);retire_raster_snapshot(unlocked);
    retire_raster_snapshot(before);retire_raster_snapshot(after);
}

#[test]
fn structural_commands_refuse_protected_sources_and_destination_parents() {
    use protocol::Mutation;
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    let mut document=semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot();document.layers=semio_framework_pack_json::from_json_str(&fixture["layers"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let history=semio_framework_plugin::HistoryView::empty();let doc=ArtifactView::new(&document,&history);
    let config=RasterConfig::default();let cfg=semio_framework_plugin::ConfigView {snapshot:&config,window:None};
    for case in fixture["cases"].as_array().unwrap() {
        let id=case["id"].as_str().unwrap();let accepted=case["expected"]["structural"].as_bool().unwrap();
        for outcome in [semio_s_artifact_raster_raster_editor::commands::delete_layer::handle(&semio_s_artifact_raster_raster_editor::commands::delete_layer::DeleteLayer {layer_id:id.into()},&doc,&cfg),semio_s_artifact_raster_raster_editor::commands::move_layer::handle(&semio_s_artifact_raster_raster_editor::commands::move_layer::MoveLayer {layer_id:id.into(),target_row_id:"outside".into(),drop_position:"before".into()},&doc,&cfg)] {
            assert_eq!(outcome.is_ok(),accepted,"{id}");if let Ok(emit)=outcome {for operation in emit.artifact_mutations {operation.retire_cold();}}
        }
    }
    for (target,position) in [("locked-group","inside"),("inherited-pixel","before")] {
        assert!(semio_s_artifact_raster_raster_editor::commands::move_layer::handle(&semio_s_artifact_raster_raster_editor::commands::move_layer::MoveLayer {layer_id:"outside".into(),target_row_id:target.into(),drop_position:position.into()},&doc,&cfg).is_err());
    }
    drop(doc);semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(document);
}

/// 📥️ LAW (Import Document hang, ticket 26/09/23 S20 14c): the demo document's op log — ONE edit whose ops plant the
/// 25 KB `semio-emblem` asset and the carrier layers — comes back through the host document-archive door. Live,
/// Import Document of that archive answered `1/1 running` forever while an empty raster imported in 1.2 s
/// (`.tmp-ticket/wp-s20/generated/probe-raster-2.txt`). Two bounded halves pin where a stall lives: the raster store
/// initializer alone (the edit replay), then the whole archive load.
#[semio_framework_async_macros::async_test]
async fn a_demo_edit_archive_loads_back_through_the_document_archive_door() {
    use semio_framework_job::InteractiveJob;
    let mut app = mounted::mounted_app();
    mounted::dispatch(&mut app, RasterCommand::SetActiveExample(semio_s_artifact_raster_raster_editor::commands::set_active_example::SetActiveExample { example_id: semio_s_artifact_raster_raster_examples::art_raster_demo::ID.into() })).await;
    let archive = PluginApp::document_archive(&mut app.0).await.expect("the demo document reads as one archive");
    let envelope = store::os_store::text_format::parse_document_pack::<semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot, semio_s_artifact_raster_raster_mutations::RasterMutation>(&archive.parent_pack, &archive.parent_spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("the archive's pack/spr pair parses").into_envelope();
    let edits = envelope.vcs.edits.len();
    let operation = semio_framework_job::OperationId(4_401);
    let generation = semio_framework_job::Generation(1);
    let mut job = semio_s_artifact_raster_raster_host::raster_document_store_initialization_job(envelope, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()));
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut steps = 0usize;
    let wallet = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 64 };
    let mut completed = None;
    while steps < 400_000 {
        let mut progress = semio_framework_value::retained_clone::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4_096, u64::MAX, wallet), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut progress);
        let outcome = job.step(&mut context).expect("initializer turn");
        steps += 1;
        if let Some(outcome) = outcome {
            if outcome.is_terminal() {
                completed = Some(matches!(outcome, semio_framework_job::JobOutcomeBorrow::Complete { .. }));
                break;
            }
        }
    }
    let completed = completed.unwrap_or_else(|| panic!("the raster store initializer replaying {edits} edit(s) of the demo archive reached no terminal in {steps} steps"));
    assert!(completed, "the initializer completes the demo archive");
    job.begin_close();
    for _ in 0..400_000 {
        if job.terminal_is_empty() {
            break;
        }
        let demand = job.retirement_demands(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("the initializer quotes its close turn");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES.max(demand.copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let _ = job.close_step(grant);
    }
    assert!(job.terminal_is_empty(), "the completed initializer closes to terminal-empty");
    drop(job);
    PluginApp::begin_document_archive_load(&mut app.0, 91, archive).expect("archive admission");
    let started = std::time::Instant::now();
    let mut status = None;
    let mut polls = 0usize;
    let mut last = (0u64, 0u64);
    while started.elapsed() < std::time::Duration::from_secs(240) {
        let polled = PluginApp::poll_document_archive_load(&mut app.0, 91, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("archive status");
        polls += 1;
        last = (polled.completed, polled.total);
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            status = Some(polled);
            break;
        }
        let _ = PluginApp::maintenance_step(&mut app.0, app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("archive maintenance step");
    }
    let status = status.unwrap_or_else(|| panic!("the demo archive load reached no terminal in {polls} polls (last progress {}/{})", last.0, last.1));
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}", String::from_utf8_lossy(&status.fault));
    PluginApp::acknowledge_document_archive_load(&mut app.0, 91).expect("archive acknowledgement");
    let snapshot = app.snapshot().expect("loaded snapshot");
    assert!(semio_s_artifact_raster_raster_io::raster_asset(&snapshot.assets, "semio-emblem").is_some(), "the emblem survives the archive door");
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(snapshot);
}

#[semio_framework_async_macros::async_test]
async fn raster_close_diagnosis_registered_add_layer_releases_all_owners() {
    use semio_framework_plugin::PluginApp;
    let mut app=context::app().await;
    context::dispatch(&mut app,RasterCommand::AddLayer(commands::add_layer::AddLayer{kind:"pixel".into()})).await;
    for _ in 0..100000 {if app.close_terminal_is_empty(){break;}app.0.close_step(app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).unwrap();semio_framework_async::yield_once().await;}
    assert!(app.close_terminal_is_empty(),"registered AddLayer closure must release all owners");
}
