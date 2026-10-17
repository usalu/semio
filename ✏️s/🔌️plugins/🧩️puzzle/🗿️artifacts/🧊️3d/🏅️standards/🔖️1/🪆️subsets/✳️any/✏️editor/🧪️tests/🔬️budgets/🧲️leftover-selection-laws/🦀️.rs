/// 🧲️ Leftover selection laws: the object context menu, the locked volume scale refusal, the clipboard clones and the leftover translate routes.

//#region 🩹️CheckedDefects
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
