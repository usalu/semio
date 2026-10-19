/// 🥓️ Checked defects: segmented export, leftover verbs and brush work budget.

//#region 🩹️CheckedDefects

/// ⬇️ Wave B38: an export larger than one wire page must reach the user as ONE download carrying the WHOLE
/// scene_snapshot, through the framework's segmented lane — never as an inline effect field.
///
/// 🧊️ The lane is picked by size against the guest's own contiguous-request ceiling
/// (`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`, one wasm page), derived from the wire constant rather than
/// spelled as a literal: Concrete Forest (7 542 B) rides inline, Nakagin Capsule Tower (145 714 B) does not.
/// The browser measured exactly that split — the inline Nakagin effect left the guest (`"effects":1`) and
/// reached no file (`📓️2026-09-12-wave-B36-full-run-bisect-2.md` §5).
///
/// 🧾️ The assertion is the REASSEMBLED payload, drained one bounded chunk at a time exactly as
/// `drainSegmentedMediaExport` drains it, parsed back to JSON and compared object-for-object with the live
/// document — so a lane that publishes a handle but loses, reorders or truncates chunks fails here.
#[semio_framework_async_macros::async_test]
async fn export_over_the_inline_budget_streams_one_segmented_download_carrying_the_whole_fixture() {
    use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_inline_budget_bytes;
    use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json;
    assert_eq!(
        puzzle3d_export_inline_budget_bytes(),
        semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
        "the inline budget is the guest's own contiguous-request ceiling, never an independent literal"
    );
    let mut app = app().await;

    let (small, small_settled) = dispatch_reporting(&mut app, "exportSnapshot", None, None).await;
    small.expect("concrete forest export");
    let small_bytes = puzzle3d_export_json(&puzzle3d_scene_snapshot_from_document(app.snapshot().expect("live snapshot").typed())).len();
    assert!(small_bytes <= puzzle3d_export_inline_budget_bytes(), "Concrete Forest must sit under the inline budget, got {small_bytes} B");
    assert!(small_settled.downloads.is_empty(), "a payload that fits one page must not open a segmented handle: {:?}", small_settled.downloads);
    let inline = small_settled.effects.iter().find_map(|effect| match effect {
        Effect::DownloadMediaExport { filename, data, encoding, .. } => Some((filename.clone(), data.len(), encoding.clone())),
        _ => None,
    });
    assert_eq!(inline, Some(("concrete-forest.json".to_string(), small_bytes, Some("utf-8".to_string()))), "the inline lane carries the whole payload and the utf-8 encoding");

    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "nakagin" })), None).await.expect("load nakagin");
    let expected = puzzle3d_export_json(&puzzle3d_scene_snapshot_from_document(app.snapshot().expect("live snapshot").typed()));
    assert!(expected.len() > puzzle3d_export_inline_budget_bytes(), "Nakagin must exceed the inline budget, got {} B", expected.len());
    let (large, large_settled) = dispatch_reporting(&mut app, "exportSnapshot", None, None).await;
    large.expect("nakagin export");
    assert!(
        !large_settled.effects.iter().any(|effect| matches!(effect, Effect::DownloadMediaExport { .. })),
        "an over-budget export must publish no inline download effect: {:?}",
        large_settled.effects
    );
    assert_eq!(large_settled.downloads.len(), 1, "exactly one segmented download handle: {:?}", large_settled.downloads);
    let handle = large_settled.downloads[0].clone();
    assert_eq!(handle.filename, "nakagin-capsule-tower.json", "the segmented lane keeps the per-example filename");
    assert_eq!(handle.mime_type, "application/json");
    assert_eq!(handle.encoding.as_deref(), Some("identity"), "UTF-8 JSON streams as identity, never base64");
    assert_eq!(handle.bytes, expected.len(), "the handle declares the whole payload");
    let drained = drain_segmented_download(&mut app, &handle).await;
    assert_eq!(drained.len(), expected.len(), "the drained bytes are the whole payload");
    assert_eq!(String::from_utf8(drained).expect("drained payload is UTF-8"), expected, "the drained payload IS the export, byte for byte");
    let reassembled: Value = parse(&expected, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the reassembled export parses");
    let live = projection_of(&app);
    assert_eq!(object_cores(&reassembled), object_cores(&live), "the streamed export reproduces every object of the live document");
    assert_eq!(reassembled.get("schema"), live.get("schema"));
}

/// 📥️ Wave B16: leftover `openImportSnapshot` must emit the file picker that completes as `importSnapshot`.
#[semio_framework_async_macros::async_test]
async fn leftover_open_import_snapshot_requests_file_open() {
    let source = include_str!("../../../🦀️.rs");
    assert!(
        source.contains(r#""openImportSnapshot" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance))"#),
        "openImportSnapshot must leftover-commit through Puzzle3dWindowCommandWork, not BoundedFirstStep"
    );
    let mut app = app().await;
    let opened = dispatch(&mut app, "openImportSnapshot", None, None).await.expect("leftover openImportSnapshot");
    let req = opened.requested_effects.iter().find_map(|effect| match effect {
        Effect::RequestFileOpen { accept, import_action, .. } => {
            assert!(accept.contains("json"), "picker accepts JSON: {accept}");
            assert_eq!(import_action, "importSnapshot");
            Some(())
        }
        _ => None,
    });
    assert!(req.is_some(), "leftover Import must emit RequestFileOpen: {:?}", opened.requested_effects);
}

/// 📥️ Wave B16: leftover `importSnapshot` of a distinct two-object JSON replaces the live one-object document.
#[semio_framework_async_macros::async_test]
async fn leftover_import_snapshot_replaces_live_document_with_distinct_two_object_json() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [0.0, 0.0, 0.0] })), None).await.expect("seed");
    assert_eq!(object_count(&app), 1, "live scene_snapshot must start as one object");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("distinct");
    let two = projection_of(&app);
    assert_eq!(object_cores(&two).len(), 2, "distinct payload must carry two objects");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty again");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [0.0, 0.0, 0.0] })), None).await.expect("one-object live");
    assert_eq!(object_count(&app), 1, "live scene_snapshot is one object before import");
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&two), "name": "puzzle-3d-distinct.json" })), None).await;
    imported.expect("leftover distinct import");
    assert_eq!(loads_in(&settled), 1, "distinct leftover import is one whole-document load");
    assert_eq!(object_cores(&imported_projection(&settled)).len(), 2, "distinct leftover import must carry both objects");
}

/// 🖱️ Wave W-Y: a selected-object context menu is puzzle-owned — never the shell fallback vocabulary.
#[semio_framework_async_macros::async_test]
async fn object_context_menu_owns_puzzle_rows_not_shell_fallback() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("add");
    let object_id = first_object_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await;
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("duplicateSelection"), "object menu must own Duplicate: {menu_json}");
    assert!(menu_json.contains("selectSameKindSelection"), "object menu must own Select same kind: {menu_json}");
    assert!(!menu_json.contains("setActiveExample"), "object menu must not dress itself as the shell fallback: {menu_json}");
}

/// 📏️ Wave W-Y: gumball scale on a locked volume refuses with a notice and emits no edit.
#[semio_framework_async_macros::async_test]
async fn gumball_scale_on_locked_volume_refuses_without_edit() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("volume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    dispatch(&mut app, "setTargetVolumeFlag", Some(&json!({ "id": volume_id, "flag": "locked", "value": true })), None).await.expect("lock");
    select_id(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await.expect("select volume");
    let before = projection_of(&app);
    let result = dispatch(&mut app, "scaleSelection", Some(&json!({ "sx": 2.0, "sy": 2.0, "sz": 2.0 })), None).await.expect("scale locked");
    let raised = notices(&result);
    assert_eq!(raised.len(), 1, "locked volume must raise exactly one notice: {:?}", result.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED);
    assert!(result.mutations.is_empty(), "locked volume must emit no edit: {:?}", result.mutations);
    assert_eq!(projection_of(&app).get("targetVolumes"), before.get("targetVolumes"));
}

/// 📋️ leftover vortex-granularity selection still captures the object, and paste clones it with a new id.
#[test]
fn leftover_copy_paste_clones_selected_object() {
    let mut scene_snapshot = empty_scene_snapshot();
    scene_snapshot.objects.push(Puzzle3dObject {
        id: "seed-left-001".into(),
        label: Some("seed".into()),
        object_kind: Some("Object".into()),
        origin: [1.0, 2.0, 3.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: Vec::new(),
        hidden: false,
        locked: false,
    });
    let marks = Puzzle3dInteractionSnapshot {
        granularity: PUZZLE3D_GRANULARITY_VORTEX.into(),
        selected: vec!["seed-left-001".into()],
        hovered: Vec::new(),
        referenced: Vec::new(),
    };
    let objects = puzzle3d_selected_objects_from(&marks, &scene_snapshot);
    assert_eq!(objects.len(), 1, "leftover selected object id must copy even when granularity is vortex: {objects:?}");
    assert_eq!(objects[0].id, "seed-left-001");
    assert_eq!(objects[0].origin, [1.0, 2.0, 3.0]);
    let fragment = puzzle3d_copy_fragment_from(&scene_snapshot, objects).expect("copy fragment");
    let mutations = puzzle3d_paste_operations(&fragment, &semio_framework_plugin::kernel::PastePlacement::default()).expect("paste");
    let created: Vec<_> = mutations
        .iter()
        .filter_map(|op| match op {
            Puzzle3dMutation::CreateObject(create) => Some(create),
            _ => None,
        })
        .collect();
    assert_eq!(created.len(), 1, "paste must commit one create-object, got {mutations:?}");
    assert_ne!(created[0].object.id, "seed-left-001", "clone must mint a fresh id");
    assert!((created[0].object.origin[0] - 1.5).abs() < 1e-9, "clone origin must keep source fields plus paste offset: {:?}", created[0].object.origin);
    assert_eq!(created[0].object.object_kind.as_deref(), Some("Object"));
    assert_eq!(created[0].object.label.as_deref(), Some("seed"));
}

/// 📋️ leftover selected vortex uuid (not object.id) still captures the parent object for copy.
#[test]
fn leftover_copy_paste_clones_object_from_selected_vortex_uuid() {
    let mut scene_snapshot = empty_scene_snapshot();
    scene_snapshot.objects.push(Puzzle3dObject {
        id: "seed-left-001".into(),
        label: Some("seed".into()),
        object_kind: Some("Object".into()),
        origin: [1.0, 2.0, 3.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: vec![Puzzle3dVortex {
            id: "5de35caa-0f02-43d7-ae74-aa730efd3386".into(),
            vortex_kind: None,
            position: [0.0, 0.0, 0.0],
            direction: None,
            radius: None,
            hidden: false,
            locked: false,
        }],
        hidden: false,
        locked: false,
    });
    let marks = Puzzle3dInteractionSnapshot {
        granularity: PUZZLE3D_GRANULARITY_VORTEX.into(),
        selected: vec!["5de35caa-0f02-43d7-ae74-aa730efd3386".into()],
        hovered: Vec::new(),
        referenced: Vec::new(),
    };
    let objects = puzzle3d_selected_objects_from(&marks, &scene_snapshot);
    assert_eq!(objects.len(), 1, "leftover selected vortex uuid must resolve to the parent object: {objects:?}");
    assert_eq!(objects[0].id, "seed-left-001");
    let fragment = puzzle3d_copy_fragment_from(&scene_snapshot, objects).expect("copy fragment");
    let mutations = puzzle3d_paste_operations(&fragment, &semio_framework_plugin::kernel::PastePlacement::default()).expect("paste");
    let created: Vec<_> = mutations
        .iter()
        .filter_map(|op| match op {
            Puzzle3dMutation::CreateObject(create) => Some(create),
            _ => None,
        })
        .collect();
    assert_eq!(created.len(), 1, "vortex-uuid leftover copy must paste one create-object, got {mutations:?}");
    assert_ne!(created[0].object.id, "seed-left-001");
}

/// 🕹️ leftover overlay → translateSelection on a locked object must refuse with a visible notice.
#[semio_framework_async_macros::async_test]
async fn leftover_translate_selection_on_locked_object_refuses_with_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.as_str()], "flag": "locked", "value": true })), None).await.expect("lock");
    let before = object_origin_x(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 4.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("translate locked");
    let raised = notices(&result);
    assert_eq!(raised.len(), 1, "locked object must raise exactly one notice: {:?}", result.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED);
    assert!(result.mutations.is_empty(), "locked object must emit no edit: {:?}", result.mutations);
    assert!((object_origin_x(&app, &object_id) - before).abs() < 1e-9, "locked object origin must not move");
}

/// 🕹️ leftover overlay → translateSelection carries explicit ids; guest snapshot granularity may be empty.
#[semio_framework_async_macros::async_test]
async fn leftover_overlay_translate_selection_moves_unlocked_object() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let before = object_origin(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 4.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("leftover overlay translate");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::Notify { .. })), "unlocked leftover overlay must not refuse: {:?}", result.requested_effects);
    assert!((object_origin(&app, &object_id)[0] - before[0] - 4.0).abs() < 1e-9, "leftover overlay ids must move the unlocked object by dx");
}

/// 🕹️ leftover `translateSelection` with explicit ids + mesh mode commits a pose delta.
#[semio_framework_async_macros::async_test]
async fn leftover_translate_selection_mesh_mode_commits_pose_delta() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let before = object_origin(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "mode": "mesh", "dx": 3.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("leftover mesh translate");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::Notify { .. })), "mesh leftover translate must pre-admit: {:?}", result.requested_effects);
    assert!((object_origin(&app, &object_id)[0] - before[0] - 3.0).abs() < 1e-9, "leftover mesh-mode translateSelection must commit a pose delta");
}

//#endregion 🩹️CheckedDefects

/// 📏️ Wave W-M2: one whole `registerBrushMesh` page validates inside one command's own work budget.
/// The page scan is cursorized at [`PUZZLE3D_MESH_PAGE_SCAN_CHARS`] characters per step over both
/// payload streams, and the largest page the plugin admits is
/// `PUZZLE3D_MESH_PAGE_BASE64_CHARS` characters per stream — so the extent a page claims can never
/// approach `PUZZLE_COMMAND_WORK_ITEMS`, which is what keeps every per-page unit under the 8 ms law.
#[test]
fn one_brush_mesh_page_validates_inside_one_command_work_budget() {
    let page = crate::editor::puzzle3d::precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS;
    let steps = page.div_ceil(PUZZLE3D_MESH_PAGE_SCAN_CHARS).saturating_mul(2).saturating_add(1);
    assert!(steps <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, "a whole page validates in {steps} bounded steps, inside {}", crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS);
    assert!(page + 8 < crate::retained_command::PUZZLE_COMMAND_RAW_BYTES, "a page's two payload streams — one page's values, split at most once and padded per stream — leave the JSON envelope room inside the raw wire");
}

/// 🚚️ Wave W-H: a client whose "already uploaded" bookkeeping outlived the guest that justified it
/// announces identities this instantiation holds nothing for. The arm may not answer that with a notice
/// — nothing on the host reads a notification's text, so the brush utility would silently keep no
/// collision body until a full page reload. It publishes the identity on the world body instead, and
/// widens its own otherwise-`Quiet` scope to the viewport so that body is actually republished. The
/// client answers with the page run, and the request retires.
#[semio_framework_async_macros::async_test]
async fn an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes() {
    let mut app = app().await;
    let (positions, indices) = semio_s_artifact_puzzle_3d_model::standards::v1::subsets::any::schema::precompute_model_tests::context::seeded_cube_mesh_buffers(23.0);
    let digest = crate::editor::puzzle3d::precompute::brush_mesh_digest(&positions, &indices);
    let url = "/test/restarted-guest-reannounce.glb";
    let announcement = json!({ "surfaceId": "world-3d", "url": url, "digest": digest });
    let refused = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("id-only announcement");
    assert!(!refused.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "a refusal the client cannot act on is not an answer");
    assert!(
        matches!(&refused.ui_scope, UiDirtyScope::Partial { window_bodies, .. } if window_bodies.iter().any(|body| body == main::BODY_KEY)),
        "the request is worthless until the world body carrying it repaints; got {:?}",
        refused.ui_scope
    );
    let requested = interaction_of(&render_composite(&mut app).await);
    assert_eq!(requested.pointer("/meshReuploadUrls").and_then(Value::as_array).cloned().unwrap_or_default(), vec![json!(url)], "the refused identity is published as a request for its bytes");
    let residency = requested.pointer("/meshResidency").and_then(Value::as_u64).expect("the world body publishes the guest's mesh residency");
    let repeated = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("the client re-announces the same dead identity");
    assert!(matches!(repeated.ui_scope, UiDirtyScope::None), "a re-announcement of an ALREADY standing request republishes nothing and must stay Quiet; got {:?}", repeated.ui_scope);
    let unchanged = interaction_of(&render_composite(&mut app).await);
    assert_eq!(unchanged.pointer("/meshReuploadUrls"), requested.pointer("/meshReuploadUrls"), "the second refusal leaves the published request set byte-identical, which is why it owes no repaint");

    let position_bytes: Vec<u8> = positions.iter().flat_map(|value| value.to_le_bytes()).collect();
    let index_bytes: Vec<u8> = indices.iter().flat_map(|value| value.to_le_bytes()).collect();
    let page = json!({
        "surfaceId": "world-3d",
        "url": url,
        "digest": digest,
        "page": 0,
        "pageCount": 1,
        "positionsB64": semio_framework_io_base64::base64_standard_encode(&position_bytes),
        "indicesB64": semio_framework_io_base64::base64_standard_encode(&index_bytes),
    });
    let accepted = dispatch(&mut app, "registerBrushMesh", Some(&page), None).await.expect("the client answers with the page run");
    assert!(!accepted.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "an accepted page run is silent");
    let settled = interaction_of(&render_composite(&mut app).await);
    assert_eq!(settled.pointer("/meshReuploadUrls").and_then(Value::as_array).map(Vec::len), Some(0), "an answered request retires, so a steady state carries no standing request");
    assert!(settled.pointer("/meshResidency").and_then(Value::as_u64).expect("residency") > residency, "the client's proof that this guest holds the geometry is the counter climbing");

    let readopted = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("the identity is now resident");
    assert!(readopted.requested_effects.is_empty(), "an identity this guest holds is adopted by id alone, at no cost on the wire");
    assert!(matches!(readopted.ui_scope, UiDirtyScope::None), "the adopting fast path stays the quiet command it is declared to be");
    assert!(
        !puzzle3d_action_uses_precompute("registerBrushMesh"),
        "registerBrushMesh only WRITES geometry into the precompute session, so it may not owe the sync prologue: that prologue seeds a fallback body for every id the session has no geometry for — precisely the ids this command supplies — clearing the brush cache and rebuilding the queue each time, which the arm's own install then discards, and which is where the 0.65 s → 2.85 s per announcement measured on 2026-09-09 came from"
    );
}
