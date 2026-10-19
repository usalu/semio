/// 🧈️ Checked defects: relocate, clipboard and snapshot export and import laws.

//#region 🩹️CheckedDefects

fn first_target_volume_id(app: &Puzzle3dApp) -> String {
    let projection = projection_of(app);
    projection
        .get("targetVolumes")
        .or_else(|| projection.get("target_volumes"))
        .and_then(Value::as_array)
        .and_then(|volumes| volumes.first())
        .and_then(|volume| volume.get("id").and_then(Value::as_str))
        .expect("a target volume is present")
        .to_string()
}

fn volume_origin(app: &Puzzle3dApp, volume_id: &str) -> Vec<f64> {
    let projection = projection_of(app);
    let volumes = projection.get("targetVolumes").or_else(|| projection.get("target_volumes")).and_then(Value::as_array).cloned().unwrap_or_default();
    volumes
        .iter()
        .find(|volume| volume.get("id").and_then(Value::as_str) == Some(volume_id))
        .and_then(|volume| volume.get("origin").or_else(|| volume.get("position")).and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

#[semio_framework_async_macros::async_test]
async fn relocate_target_volume_undoes_and_redoes_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let volume_id = first_target_volume_id(&app);
    let start = volume_origin(&app, &volume_id);
    let pose = |position: [f64; 3]| json!({ "position": position, "quaternion": [0.0, 0.0, 0.0, 1.0], "scale": [1.0, 1.0, 1.0] });
    let (before, after) = ([start[0], start[1], start[2]], [start[0] + 3.0, start[1] + 3.0, start[2] + 3.0]);
    let (_, settled) = dispatch_reporting(&mut app, "relocateTargetVolume", Some(&json!({ "volumeId": volume_id, "mode": "translate", "before": pose(before), "after": pose(after) })), None).await;
    let rows = edit_rows(&settled);
    assert!(rows.len() == 1 && rows[0].op_lines[0].starts_with("drag-selection") && rows[0].transaction.is_some(), "the volume gumball is one relative drag transaction: {rows:?}");
    let moved = volume_origin(&app, &volume_id);
    assert!(moved.iter().zip(after).all(|(landed, wanted)| (landed - wanted).abs() < 1e-9), "relocateTargetVolume must land the gumball's after pose, got {moved:?} from {start:?}");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(volume_origin(&app, &volume_id), start, "undo restores the volume pose");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(volume_origin(&app, &volume_id), moved, "redo reapplies the volume pose");
}

#[semio_framework_async_macros::async_test]
async fn world_relocate_undoes_and_redoes_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let start = object_origin(&app, &object_id);
    dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id, "position": [9.0, 8.0, 7.0] })), None).await.expect("worldRelocate");
    let moved = object_origin(&app, &object_id);
    assert!((moved[0] - 9.0).abs() < 1e-9 && (moved[1] - 8.0).abs() < 1e-9 && (moved[2] - 7.0).abs() < 1e-9, "worldRelocate must write the world position, got {moved:?} from {start:?}");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "undo restores the object origin");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(object_origin(&app, &object_id), moved, "redo reapplies the object origin");
}

/// 🚚️ Wave W-Y: Nakagin-scale `worldRelocate` of one object must admit and complete inside the
/// command work budget — the extent is the true per-item vortex walk, not `objects + attractions`.
#[semio_framework_async_macros::async_test]
async fn world_relocate_on_nakagin_admits_and_completes() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin");
    let object_id = first_object_id(&app);
    let start = object_origin(&app, &object_id);
    dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id, "position": [4.0, 5.0, 6.0] })), None).await.expect("worldRelocate nakagin");
    let moved = object_origin(&app, &object_id);
    assert!((moved[0] - 4.0).abs() < 1e-6 && (moved[1] - 5.0).abs() < 1e-6 && (moved[2] - 6.0).abs() < 1e-6, "nakagin relocate must land, got {moved:?} from {start:?}");
}

/// 🚚️ Wave B5: the Relocate utility's single commit, both answers. An UNLOCKED object takes the absolute
/// world origin the host's pointer-up sends and publishes a real document edit; the SAME verb on a LOCKED
/// object refuses with exactly one `selection_locked` notice, no edit and no movement — the identical
/// refusal `translateSelection` already owes a locked selection, which this verb used to skip silently
/// (the host's relocate ghost just snapped back with nothing said). The declared `ActionKind` is pinned
/// here too: `worldRelocate` writes the document, so undo/redo and the history panel must see a
/// `Mutation`, never the `View` the 2026-09-10 checklist reverification claimed.
#[semio_framework_async_macros::async_test]
async fn world_relocate_moves_an_unlocked_object_and_refuses_a_locked_one_with_one_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let moved = dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id.as_str(), "position": [3.0, -2.0, 1.5] })), None).await.expect("worldRelocate unlocked");
    assert!(notices(&moved).is_empty(), "an unlocked relocate must not refuse: {:?}", moved.requested_effects);
    let landed = object_origin(&app, &object_id);
    assert!((landed[0] - 3.0).abs() < 1e-9 && (landed[1] + 2.0).abs() < 1e-9 && (landed[2] - 1.5).abs() < 1e-9, "worldRelocate must write the absolute origin, got {landed:?}");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.as_str()], "flag": "locked", "value": true })), None).await.expect("lock");
    let refused = dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id.as_str(), "position": [9.0, 9.0, 9.0] })), None).await.expect("worldRelocate locked");
    let raised = notices(&refused);
    assert_eq!(raised.len(), 1, "a locked relocate must raise exactly one notice: {:?}", refused.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so the refusal must be real prose");
    assert!(refused.mutations.is_empty(), "a locked relocate must emit no edit: {:?}", refused.mutations);
    assert_eq!(object_origin(&app, &object_id), landed, "a locked object must not move");
    let definition = create_puzzle3d_app();
    let declared = dispatchable_actions(&definition).into_iter().find(|action| action.id == "worldRelocate").map(|action| action.kind).expect("worldRelocate is declared");
    assert_eq!(declared, ActionKind::Mutation, "worldRelocate edits the document, so history and undo must see a Mutation");
}

/// 📋️ Wave W-Y: copy then paste clones the selection with new ids as one Mutation edit.
#[semio_framework_async_macros::async_test]
async fn copy_then_paste_clones_selection_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("add");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let copied = dispatch(&mut app, "copy", None, None).await.expect("copy");
    let fragment = copied.requested_effects.iter().find_map(|effect| match effect {
        Effect::ClipboardWrite { fragment } => Some(fragment.clone()),
        _ => None,
    }).expect("copy must emit ClipboardWrite");
    let before = object_count(&app);
    let paste_args = json!({ "fragment": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&fragment)) });
    let pasted = dispatch(&mut app, "paste", Some(&paste_args), None).await.expect("paste");
    assert!(!pasted.mutations.is_empty(), "paste must emit one mutation edit: {:?}", pasted.mutations);
    assert_eq!(object_count(&app), before + 1, "paste clones the selection");
    let ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).unwrap().iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    assert!(ids.iter().any(|id| id != &object_id), "the clone must carry a new id: {ids:?}");
}

/// ✂️ Wave W-Y: cut is copy + delete as one undo step.
#[semio_framework_async_macros::async_test]
async fn cut_undoes_as_one_step() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let before = object_count(&app);
    dispatch(&mut app, "cut", None, None).await.expect("cut");
    assert_eq!(object_count(&app), before - 1, "cut removes the selection");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), before, "one undo restores the cut");
}

/// ⬇️ Wave W-Y: export carries the full round-trippable scene_snapshot JSON.
#[semio_framework_async_macros::async_test]
async fn export_snapshot_downloads_round_trippable_json() {
    let mut app = app().await;
    let before = projection_of(&app);
    let result = dispatch(&mut app, "exportSnapshot", None, None).await.expect("export");
    let data = result.requested_effects.iter().find_map(|effect| match effect {
        Effect::DownloadMediaExport { filename, mime_type, data, .. } => {
            assert_eq!(filename, "concrete-forest.json", "a fresh session exports under the example its document was seeded from");
            assert_eq!(mime_type, "application/json");
            Some(data.clone())
        }
        _ => None,
    }).expect("export must emit DownloadMediaExport");
    let exported: Value = parse(&data, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("export JSON parses");
    assert_eq!(exported.get("schema"), before.get("schema"), "export schema must match the live scene_snapshot");
    assert_eq!(exported.get("objects").and_then(Value::as_array).map(Vec::len), before.get("objects").and_then(Value::as_array).map(Vec::len));
}

/// 🧬️ Identity of imported objects without scene_snapshot/snapshot projection twins (anchor / null scale).
fn object_cores(value: &Value) -> Vec<(String, String, String, Value, Vec<String>)> {
    value
        .get("objects")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|object| {
            let id = object.get("id").and_then(Value::as_str)?.to_string();
            let kind = object.get("objectKind").and_then(Value::as_str).unwrap_or_default().to_string();
            let mesh = object.get("meshUrl").and_then(Value::as_str).unwrap_or_default().to_string();
            let origin = object.get("origin").cloned().unwrap_or(Value::Null);
            let vortices = object
                .get("vortices")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|vortex| vortex.get("id").and_then(Value::as_str).map(str::to_string))
                .collect();
            Some((id, kind, mesh, origin, vortices))
        })
        .collect()
}

/// 📥️ Wave W-Y: importing that JSON reproduces the document as one Mutation edit.
#[semio_framework_async_macros::async_test]
async fn import_snapshot_reproduces_the_exported_document() {
    let mut app = app().await;
    let source = projection_of(&app);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0);
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&source) })), None).await;
    imported.expect("import");
    assert_eq!(loads_in(&settled), 1, "import is one whole-document load");
    assert!(history_row_labels(&settled).iter().all(|row| row.contains("applied=false") && row.contains("ops=0")), "a whole-document load journals no applied history row: {:?}", history_row_labels(&settled));
    assert_eq!(object_cores(&imported_projection(&settled)), object_cores(&source), "import reproduces the exported objects");
}

/// 📥️ Wave W-AB: workspace Import is `openImportSnapshot` (file picker). `importSnapshot` stays
/// dispatchable for the host re-dispatch after the pick, but is not a file-menu row.
#[test]
fn file_menu_import_row_opens_the_file_picker() {
    let definition = create_puzzle3d_app();
    let file: Vec<(&str, bool)> = dispatchable_actions(&definition)
        .into_iter()
        .filter(|action| action.category.as_deref() == Some("file"))
        .map(|action| (action.id.as_str(), action.in_palette))
        .collect();
    assert!(file.iter().any(|(id, _)| *id == "exportSnapshot"), "file menu keeps Export: {file:?}");
    assert!(file.iter().any(|(id, in_palette)| *id == "openImportSnapshot" && *in_palette), "file menu Import is openImportSnapshot: {file:?}");
    assert!(!file.iter().any(|(id, _)| *id == "importSnapshot"), "importSnapshot is the picker completion, not a menu row: {file:?}");
    let import = dispatchable_actions(&definition).into_iter().find(|action| action.id == "importSnapshot").expect("importSnapshot stays dispatchable");
    assert!(!import.in_palette);
    assert_eq!(import.category.as_deref(), None);
}

/// 📥️ Wave W-AB: activating Import requests a file open; completing it with scene_snapshot JSON imports.
#[semio_framework_async_macros::async_test]
async fn open_import_snapshot_requests_file_open_then_import_applies_payload() {
    let mut app = app().await;
    let source = projection_of(&app);
    let opened = dispatch(&mut app, "openImportSnapshot", None, None).await.expect("openImportSnapshot");
    let req = opened.requested_effects.iter().find_map(|effect| match effect {
        Effect::RequestFileOpen { accept, read_as, import_action, multiple, .. } => {
            assert!(accept.contains("json"), "picker accepts JSON: {accept}");
            assert_eq!(read_as.as_deref(), Some("text"));
            assert_eq!(import_action, "importSnapshot");
            assert!(!*multiple);
            Some(())
        }
        _ => None,
    });
    assert!(req.is_some(), "Import must emit RequestFileOpen: {:?}", opened.requested_effects);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0);
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&source) })), None).await;
    imported.expect("import");
    assert_eq!(loads_in(&settled), 1, "import is one whole-document load");
    assert_eq!(object_cores(&imported_projection(&settled)), object_cores(&source), "picked payload reproduces the exported objects");
}

/// 📥️ Wave W-AB: re-importing the live scene_snapshot is a store identity, not a guest payload dedupe.
/// `import_snapshot` always assigns. The store folds equal content into no edit, so the admitting
/// settle carries one unapplied `importSnapshot` row (`ops=0`) — the empty-emit command log — and
/// the object cores do not move.
#[semio_framework_async_macros::async_test]
async fn import_snapshot_of_the_live_document_records_whether_identical_content_is_an_edit() {
    let mut app = app().await;
    let source = projection_of(&app);
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&source) })), None).await;
    imported.expect("reimport");
    let objects_after = object_cores(&projection_of(&app));
    assert_eq!(objects_after, object_cores(&source), "identical payload must not rewrite object cores");
    let rows = history_row_labels(&settled);
    assert_eq!(history_rows(&settled), 1, "identical live scene_snapshot logs the command and no edit: {rows:?}");
    assert!(rows.iter().all(|row| row.contains("importSnapshot") && row.contains("applied=false") && row.contains("ops=0")), "{rows:?}");
}

/// 📥️ Wave W-AB #44: leftover `importSnapshot` against a one-object live scene_snapshot applies a distinct two-object JSON.
#[semio_framework_async_macros::async_test]
async fn import_snapshot_of_a_distinct_two_object_json_against_a_one_object_live_fixture_emits_operations() {
    let source = include_str!("../../../🦀️.rs");
    assert!(
        source.contains(r#""importSnapshot" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance))"#),
        "importSnapshot must leftover-commit through Puzzle3dWindowCommandWork, not BoundedFirstStep"
    );
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [0.0, 0.0, 0.0] })), None).await.expect("seed");
    assert_eq!(object_count(&app), 1, "live scene_snapshot must start as one object");
    let live_id = first_object_id(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("distinct");
    let two = projection_of(&app);
    let cores = object_cores(&two);
    assert_eq!(cores.len(), 2, "distinct payload must carry two objects");
    let distinct_id = cores.iter().map(|row| row.0.as_str()).find(|id| *id != live_id).expect("distinct object id").to_string();
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&two) })), None).await;
    imported.expect("import distinct");
    assert_eq!(loads_in(&settled), 1, "distinct two-object import is one whole-document load");
    let after = object_cores(&imported_projection(&settled));
    assert_eq!(after.len(), 2, "after-snapshot must contain both objects");
    assert!(after.iter().any(|row| row.0 == distinct_id), "after-snapshot must contain the distinct object id {distinct_id}");
}

/// 📥️ Wave B9 lane 3: the picker's REAL round trip — the bytes `exportSnapshot` hands the host's
/// download are the bytes the file chooser hands `importSnapshot` back. A distinct export must upsert
/// its objects and record one history row; re-importing the SAME file against the document it
/// describes must stay an identity no-op. The browser measured `effects:0 historyUpserts:0` on the
/// distinct half, which is also exactly what a silently-dropped refusal notice looks like — so this
/// law asserts the absence of a refusal as well as the presence of the edit.
#[semio_framework_async_macros::async_test]
async fn exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity() {
    let exported = |result: &semio_framework_plugin::InvocationResult| -> String {
        result
            .requested_effects
            .iter()
            .find_map(|effect| match effect {
                Effect::DownloadMediaExport { data, .. } => Some(data.clone()),
                _ => None,
            })
            .expect("exportSnapshot emits one DownloadMediaExport")
    };
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { message } => Some(message.clone()),
                _ => None,
            })
            .collect()
    };
    let mut app = app().await;
    let one_object = exported(&dispatch(&mut app, "exportSnapshot", None, None).await.expect("export the boot document"));
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("seed a second object");
    let two_objects = exported(&dispatch(&mut app, "exportSnapshot", None, None).await.expect("export the distinct document"));
    assert_ne!(one_object, two_objects, "the two exports must be distinct files");
    let (restored, restored_settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": one_object.as_str(), "name": "puzzle-3d.json" })), None).await;
    let restored = restored.expect("restore the one-object document");
    assert!(notices(&restored).is_empty(), "restoring an exported file must not refuse: {:?}", notices(&restored));
    assert_eq!(object_cores(&imported_projection(&restored_settled)).len(), 1, "the restored file is the one-object document");
    let (distinct, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": two_objects.as_str(), "name": "puzzle-3d-distinct.json" })), None).await;
    let distinct = distinct.expect("import the distinct file");
    assert!(notices(&distinct).is_empty(), "a distinct exported file must not refuse: {:?}", notices(&distinct));
    assert_eq!(loads_in(&settled), 1, "a distinct exported file is one whole-document load");
    let after_distinct = object_cores(&imported_projection(&settled));
    assert_eq!(after_distinct.len(), 2, "a distinct exported file must carry its objects");
    let (again, again_settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": two_objects.as_str(), "name": "puzzle-3d-distinct.json" })), None).await;
    let again = again.expect("re-import the same file");
    assert!(notices(&again).is_empty(), "re-importing the same file must not refuse: {:?}", notices(&again));
    assert_eq!(object_cores(&imported_projection(&again_settled)), after_distinct, "re-importing the same file loads the same document");
}

/// 🔢️ The payload an import actually receives was written by a BROWSER, not by this crate's own writer:
/// the file chooser hands back whatever `JSON.stringify` produced, which spells a whole float as `1` (no
/// `.0`), a tiny one in exponent form (`-5.551115123125783e-17`), an absent optional as `null`, and a
/// non-ASCII label as a `\u` escape. The law above only ever fed `import_snapshot` this crate's own export
/// text, so none of those spellings was under test — and the browser's `import-distinct` read
/// `effects:0 historyUpserts:0` with NO refusal notice, which is exactly what `import_snapshot`'s
/// `ctx.abort` path looks like when `parse` rejects the text it was handed (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B57 §2). One unreadable spelling is a silently dropped document.
#[semio_framework_async_macros::async_test]
async fn a_browser_serialized_fixture_payload_imports_every_json_number_spelling() {
    let mut app = app().await;
    let payload = r#"{"schema":"puzzle.3d.scene.v1","domain":"architecture","objects":[{"id":"browser-clone","label":"Distinct Capsule J · cs_sl1","objectKind":"Object","origin":[-16.75,-3.6499999999999986,0],"orientation":[-5.551115123125783e-17,5.551115123125783e-17,0.7071067811865475,0.7071067811865475],"scale":null,"vortices":[{"id":"browser-clone:v0","position":[1,0,2]}],"hidden":false,"locked":false}],"attractions":[],"targetVolumes":[],"references":[]}"#;
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": payload, "name": "browser-stringify.json" })), None).await;
    let imported = imported.expect("import a browser-serialized payload");
    let notices: Vec<String> = imported
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect();
    assert!(notices.is_empty(), "a browser-serialized payload must not be refused: {notices:?}");
    assert_eq!(loads_in(&settled), 1, "a browser-serialized payload is one whole-document load");
    let objects = object_cores(&imported_projection(&settled));
    assert_eq!(objects.len(), 1, "the imported document is exactly the payload's objects: {objects:?}");
    assert_eq!(objects.first().map(|(id, ..)| id.as_str()), Some("browser-clone"), "the payload's own object id survives the import: {objects:?}");
}

/// 🧱️ Wave B59: the payload the browser hands `importSnapshot` for a REAL example is 145 714 B — the
/// Nakagin Capsule Tower export, 180 objects — and the live `import-distinct` verdict read
/// `paneObjects=180→180` with no notice at all (wave B57 §2.3). Every import law before this one fed a
/// payload of a few hundred bytes, so the size class the product actually imports was never under test.
///
/// 🧩️ The payload reaches the action the way the framework hands it once it has reassembled the host's
/// chunks (`semio_framework::kernel::ImportStaging`): ONE whole `{payload, name}`. The witnesses are the
/// browser's own three: the object census moves, exactly ONE applied history row lands, nothing is refused.
#[semio_framework_async_macros::async_test]
async fn a_one_hundred_forty_five_kilobyte_distinct_fixture_imports_inside_one_settle() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [220.0, 0.0, 0.0] })), None).await.expect("seed one more object");
    let distinct = crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json(&puzzle3d_scene_snapshot_from_document(app.snapshot().expect("typed app snapshot").typed()));
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("return to the example document");
    assert_eq!(object_count(&app), seeded, "the document is back at the example census before the import");
    assert!(distinct.len() > 140_000, "the payload under test must be the product's own size class; observed {} B", distinct.len());
    let (result, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": distinct.as_str(), "name": "nakagin-capsule-tower-distinct.json" })), None).await;
    let result = result.expect("a product-sized import inside the budget is admitted");
    let notices: Vec<String> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect();
    assert!(notices.is_empty(), "a payload inside the declared import budget must not be refused: {notices:?}");
    assert_eq!(loads_in(&settled), 1, "the whole import is exactly one whole-document load");
    assert_eq!(object_cores(&imported_projection(&settled)).len(), seeded + 1, "a product-sized distinct payload must carry the document it replaces");
}

/// 🧩️ The framework half of an import, end to end for this app: the host's chunk envelope
/// (`semio_framework::kernel::import_chunk_arguments`) goes through the SDK's own action dispatch, whose
/// instance-scoped `ImportStaging` stages every chunk but the last — no command, no edit — and hands
/// `importSnapshot` the whole file once the run closes: ONE applied history row, the census moved.
#[semio_framework_async_macros::async_test]
async fn a_chunked_pick_reaches_import_snapshot_as_one_whole_file_through_the_framework_staging() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [220.0, 0.0, 0.0] })), None).await.expect("seed one more object");
    let distinct = crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json(&puzzle3d_scene_snapshot_from_document(app.snapshot().expect("typed app snapshot").typed()));
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("return to the example document");
    let chunks = semio_framework::kernel::import_payload_chunks(&distinct);
    assert!(chunks.len() > 1, "a product-sized payload spans several host chunks; observed {}", chunks.len());
    let mut loaded = None;
    for chunk in &chunks {
        let args = semio_framework::kernel::import_chunk_arguments("nakagin-capsule-tower-distinct.json", chunk, None);
        let (result, settled) = dispatch_action_reporting(&mut app, "importSnapshot", &args).await;
        result.expect("every chunk of a product-sized pick is admitted");
        let rows = history_row_labels(&settled);
        if chunk.chunk + 1 < chunk.chunk_count {
            assert!(rows.is_empty(), "a staged chunk is no command and no edit; chunk {}: {rows:?}", chunk.chunk);
            assert_eq!(object_count(&app), seeded, "a staged chunk must not move the document; chunk {}", chunk.chunk);
        }
        if loads_in(&settled) == 1 {
            assert!(loaded.is_none(), "the whole chunked pick is exactly one whole-document load");
            loaded = Some(imported_projection(&settled));
        }
    }
    let loaded = loaded.expect("the closing chunk publishes the whole-document load");
    assert_eq!(object_cores(&loaded).len(), seeded + 1, "the reassembled file carries the document it replaces");
}

/// 🧯️ An import above the budget one export may stream is REFUSED with a notice, never silently dropped
/// and never a fault — the framework hands the whole file, so this budget is the app's one size rule.
#[semio_framework_async_macros::async_test]
async fn an_import_above_the_export_budget_refuses_with_a_notice() {
    use crate::retained_command::PUZZLE_IMPORT_TOTAL_BYTES;
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    let oversized = "x".repeat(PUZZLE_IMPORT_TOTAL_BYTES + 1);
    let (result, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": oversized.as_str(), "name": "oversized.json" })), None).await;
    let result = result.expect("an over-budget import is an ANSWER, never a fault");
    let notices: Vec<String> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(notices.len(), 1, "an over-budget import publishes exactly one notice: {notices:?}");
    let rows = history_row_labels(&settled);
    assert_eq!(history_rows(&settled), 1, "a refused import logs the command and no edit: {rows:?}");
    assert!(rows.iter().all(|row| row.contains("importSnapshot") && row.contains("applied=false") && row.contains("ops=0")), "{rows:?}");
    assert_eq!(object_count(&app), seeded, "a refused import leaves the document alone");
}

/// 📤️📥️ LAW: the Nakagin export re-imports as the very document it came from — the whole exported file,
/// read by `puzzle3d_import_value`, rebuilds a scene_snapshot whose own export is byte-identical.
#[test]
fn the_nakagin_export_reimports_byte_for_byte() {
    use crate::editor::puzzle3d::commands::import_snapshot::puzzle3d_import_value;
    let payload = crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json(&NAKAGIN_EXAMPLE_SNAPSHOT.clone());
    assert!(payload.len() > semio_framework::kernel::IMPORT_CHUNK_BYTES, "the payload under test spans several host chunks: {} B", payload.len());
    let root = puzzle3d_import_value(&json!({ "payload": payload.as_str(), "name": "nakagin.json" })).expect("the whole export is a puzzle 3D document");
    assert_eq!(
        crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json(&Puzzle3dSceneSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&root)).expect("the imported root IS a puzzle 3D document")),
        payload,
        "the import is byte-identical to the document that was exported"
    );
}

/// 📥️ Wave B16: leftover `exportSnapshot` must emit `DownloadMediaExport`.
/// 🏷️ Wave B30: a fresh session's document IS the Concrete Forest example
/// (`ArtifactApp::initial_snapshot`), and since the config lane says so from the first render the
/// boot download is named after it. The app-generic `puzzle-3d.json` belongs to a document that came
/// from no example at all — see `export_snapshot_names_the_download_after_the_active_example`, which
/// reaches that state by clearing the picker.
#[semio_framework_async_macros::async_test]
async fn leftover_export_snapshot_downloads_the_boot_example_json() {
    let source = include_str!("../../../🦀️.rs");
    assert!(
        source.contains(r#""exportSnapshot" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance))"#),
        "exportSnapshot must leftover-commit through Puzzle3dWindowCommandWork, not BoundedFirstStep"
    );
    let mut app = app().await;
    let result = dispatch(&mut app, "exportSnapshot", None, None).await.expect("leftover export");
    let filename = result.requested_effects.iter().find_map(|effect| match effect {
        Effect::DownloadMediaExport { filename, .. } => Some(filename.as_str()),
        _ => None,
    });
    assert_eq!(filename, Some("concrete-forest.json"), "leftover export must name the boot example: {:?}", result.requested_effects);
}

/// 🏷️ Wave B26: exporting Concrete Forest and exporting Nakagin must not both land as one constant
/// filename. The download is named after the ACTIVE EXAMPLE — the id `set_active_example` persists on
/// the shared config, restored into `Puzzle3dRuntime` by `window::runtime` — and a document that came
/// from no example at all keeps the app-generic `puzzle-3d.json`. Both picker aliases (`concrete`,
/// `nakagin`) resolve to the canonical id, so the filename never depends on how the row was spelled.
#[semio_framework_async_macros::async_test]
async fn export_snapshot_names_the_download_after_the_active_example() {
    /// 🏷️ The name is asserted across BOTH publication lanes, because the lane is chosen by payload size
    /// and the naming contract is not: an over-budget example publishes a segmented handle rather than an
    /// inline `DownloadMediaExport`, and it must carry the same filename (wave B38).
    async fn exported_filename(app: &mut Puzzle3dApp) -> String {
        let (result, settled) = dispatch_reporting(app, "exportSnapshot", None, None).await;
        result.expect("export");
        let inline = settled.effects.iter().find_map(|effect| match effect {
            Effect::DownloadMediaExport { filename, .. } => Some(filename.clone()),
            _ => None,
        });
        let segmented = settled.downloads.first().map(|download| download.filename.clone());
        assert!(inline.is_none() || segmented.is_none(), "one export publishes exactly one lane, never both: inline={inline:?} segmented={segmented:?}");
        inline.or(segmented).expect("export must publish a download on one of the two lanes")
    }
    let mut app = app().await;
    assert_eq!(exported_filename(&mut app).await, "concrete-forest.json", "a fresh session already names the example its document was seeded from");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("clear to a blank document");
    assert_eq!(exported_filename(&mut app).await, "puzzle-3d.json", "a document that came from no example keeps the app-generic name");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "concrete-forest" })), None).await.expect("load concrete forest");
    assert_eq!(exported_filename(&mut app).await, "concrete-forest.json", "Concrete Forest must export under its own id");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "nakagin" })), None).await.expect("load nakagin by alias");
    assert_eq!(exported_filename(&mut app).await, "nakagin-capsule-tower.json", "the picker alias must still export the canonical example id");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("clear to a blank document");
    assert_eq!(exported_filename(&mut app).await, "puzzle-3d.json", "clearing the example clears the name it exported under");
}

/// ⬇️ Wave B43: an export above what ONE segmented download may carry is refused with a NOTICE, and the
/// budget it is measured against is the framework's own end-to-end cap — never a literal of this app's.
///
/// 🧯 A fault here would land as a dead job; before B43 the whole lane's over-cap answer was a producer
/// `Fault`. The refusal names the file, its size and the budget, so the answer is readable.
#[test]
fn export_refuses_a_payload_above_the_declared_segmented_budget_with_a_notice() {
    use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_refusal;
    use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_segmented;
    use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_segmented_budget_bytes;
    use crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES;
    use semio_framework_plugin::app::ArtifactOutputChunks;
    let budget = puzzle3d_export_segmented_budget_bytes().expect("the declared output budget is within the framework cap");
    assert_eq!(budget, PUZZLE_COMMAND_OUTPUT_BYTES, "the segmented budget IS this command's declared contract output budget");
    assert!(budget <= ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES, "no app may declare a download larger than the wire admits");
    let over = "x".repeat(budget + 1);
    assert!(puzzle3d_export_segmented("nakagin-capsule-tower.json".into(), &over).is_err(), "a payload above the budget must never open a download handle");
    let notice = puzzle3d_export_refusal("nakagin-capsule-tower.json", budget + 1, budget);
    assert!(notice.contains("nakagin-capsule-tower.json") && notice.contains(&(budget + 1).to_string()) && notice.contains(&budget.to_string()), "the refusal must name the file, its size and the budget: {notice}");
}
//#endregion 🩹️CheckedDefects
