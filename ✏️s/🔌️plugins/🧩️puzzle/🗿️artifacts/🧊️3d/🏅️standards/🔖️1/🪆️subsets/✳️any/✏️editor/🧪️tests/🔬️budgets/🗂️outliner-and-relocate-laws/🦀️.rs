/// 🗂️ Outliner and relocate laws: the inspection flag rows, the outliner verbs, the target volume and object relocation undo and redo.

//#region 🩹️CheckedDefects
/// 🙈️ `📓️2026-09-09-user-feature-checklist.md` §16 (re-checked for W-D4's §17 law): the inspection
/// panel's `patchInspector` flag rows must carry the INVERSE of the row's current state, so an object
/// hidden from here can be un-hidden from here. Driven end to end — hide through the panel's own
/// dispatch shape, then re-read the panel and show again.
#[semio_framework_async_macros::async_test]
async fn inspection_flag_rows_toggle_back_off() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select the object");
    let hidden_of = |app: &Puzzle3dApp, id: &str| {
        projection_of(app)
            .get("objects")
            .and_then(Value::as_array)
            .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(id)).cloned())
            .and_then(|object| object.get("hidden").and_then(Value::as_bool))
            .unwrap_or(false)
    };
    for expected in [true, false] {
        let panel = to_json_string(&render_body(&mut app, inspection::BODY_KEY).await);
        assert!(panel.contains("puzzle3d-play-inspector.object.hidden"), "the inspector must render the object's hidden row: {panel}");
        dispatch(&mut app, "patchInspector", Some(&json!({ "entity": PUZZLE3D_GRANULARITY_OBJECT, "field": "hidden", "ids": [object_id.as_str()], "value": expected })), None).await.expect("patchInspector hidden");
        assert_eq!(hidden_of(&app, &object_id), expected, "the inspector's hidden row must reach {expected}");
    }
}

/// 🙈️ `📓️2026-09-09-user-feature-checklist.md` §17: the outliner row's own Hide/Show control, driven
/// through the target the panel itself declares — never a hand-written literal, since the reported defect
/// ("Hide does nothing, Show can never un-hide") was exactly a wrong value baked into the row. Hide,
/// re-read the panel, and dispatch the row's `setSelectionHidden` with its NEW target args to show again;
/// the flag must round-trip both ways, exactly as the host binds a row verb to the row's one target.
#[semio_framework_async_macros::async_test]
async fn outliner_row_hide_and_show_round_trip_through_their_own_declared_args() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    let hidden_of = |app: &Puzzle3dApp, id: &str| {
        projection_of(app)
            .get("objects")
            .and_then(Value::as_array)
            .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(id)).cloned())
            .and_then(|object| object.get("hidden").and_then(Value::as_bool))
            .unwrap_or(false)
    };
    assert!(!hidden_of(&app, &object_id), "a freshly added object starts visible");
    for expected in [true, false] {
        let panel: Value = from_json_str(&to_json_string(&render_body(&mut app, artifact::BODY_KEY).await), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the outliner renders parseable ui json");
        let args = outliner_verb_row(&panel, &object_id, "setSelectionHidden").and_then(|row| row.pointer("/target/args").cloned()).unwrap_or_else(|| panic!("the outliner row must declare a setSelectionHidden target for {object_id}: {panel}"));
        assert_eq!(args.get("hidden").and_then(Value::as_bool), Some(expected), "the row's own target must ask for the INVERSE of the state it renders: {args}");
        dispatch(&mut app, "setSelectionHidden", Some(&args), None).await.expect("setSelectionHidden from the outliner row's own target");
        assert_eq!(hidden_of(&app, &object_id), expected, "the outliner row's hidden flag must reach {expected}");
    }
}

/// 🙈️ The hop AFTER the one above: hiding an object from the outliner row must also reach the WORLD
/// lane the viewport paints from, and the row itself must re-render with the inverse control. Wave B20
/// measured the browser publishing an unchanged row AND an unchanged `data-instances-json` while the
/// host's history had already recorded `change-object-hidden id=seed-left-001 new-hidden=true`, so
/// "the flag reached the projection" (the law above) is NOT the same claim as "the two surfaces the
/// user looks at moved". `world_instances_geometry_json` encodes hidden as a zero scale — the id stays
/// in the array so no other object's index shifts — and the row's own control flips Hide↔Show.
#[semio_framework_async_macros::async_test]
async fn outliner_hide_reaches_the_world_instance_lane_and_flips_the_row_control() {
    fn instance_scale(world: &Value, object_id: &str) -> Vec<f64> {
        let instances: Value = parse(world.pointer("/world3d/instancesJson").and_then(Value::as_str).expect("the world body publishes an instances lane"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("instances lane is json");
        instances
            .as_array()
            .expect("instances lane is an array")
            .iter()
            .find(|instance| instance.get("id").and_then(Value::as_str) == Some(object_id))
            .and_then(|instance| instance.get("scale").and_then(Value::as_array))
            .map(|scale| scale.iter().filter_map(Value::as_f64).collect())
            .unwrap_or_default()
    }
    /// 🙈️ The visibility row action's rendered face: the icon of the `setSelectionHidden` action on the row whose
    /// target names this object. Keyed on the target rather than on a row id so the law survives any reshaping
    /// of the tree above the control.
    fn visibility_control_icon(panel: &Value, object_id: &str) -> Option<String> {
        let row = outliner_verb_row(panel, object_id, "setSelectionHidden")?;
        let actions = row.get("rowActions").and_then(Value::as_array)?;
        let action = actions.iter().find(|action| action.get("verb").and_then(Value::as_str) == Some("setSelectionHidden"))?;
        action.get("icon").and_then(Value::as_str).filter(|icon| matches!(*icon, "eye" | "eye-off")).map(str::to_string)
    }
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let visible = render_body(&mut app, main::BODY_KEY).await;
    assert_eq!(instance_scale(&visible, &object_id), vec![1.0, 1.0, 1.0], "a visible object publishes its real scale");
    let before_icon = visibility_control_icon(&render_body(&mut app, artifact::BODY_KEY).await, &object_id);
    assert_eq!(before_icon.as_deref(), Some("eye"), "a visible object's row renders the Hide control");

    dispatch(&mut app, "setSelectionHidden", Some(&json!({ "entity": "object", "hidden": true, "ids": [object_id.clone()] })), None).await.expect("setSelectionHidden");

    let hidden = render_body(&mut app, main::BODY_KEY).await;
    assert_eq!(instance_scale(&hidden, &object_id), vec![0.0, 0.0, 0.0], "a hidden object must publish a zero scale into the world instance lane");
    let after_icon = visibility_control_icon(&render_body(&mut app, artifact::BODY_KEY).await, &object_id);
    assert_eq!(after_icon.as_deref(), Some("eye-off"), "a hidden object's row must re-render as the Show control");
}

/// 👁️ The RESTORE half of the law above, in the SAME settle: the row's Show control must put the object's
/// real scale back into the world instance lane, not merely flip the row's label back. The lane is served
/// from [`Puzzle3dInstanceResidency`], which re-serializes only the records whose own
/// `instance_record_fingerprint` moved and caches every other record verbatim — so a key that failed to
/// read `hidden` would leave an un-hidden object at `[0,0,0]` for the rest of the session while the row,
/// the projection and the history all read `hidden=false`, and nothing but the world would disagree.
/// Browser battery #64 reported `outliner-show-restores restored=false` while the `seed-left-001` row had
/// already come back to `Hide` (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B57), and no row-text verdict
/// can tell those two apart.
#[semio_framework_async_macros::async_test]
async fn outliner_show_restores_the_world_instance_scale_in_the_same_settle() {
    fn instance_scale(world: &Value, object_id: &str) -> Vec<f64> {
        let instances: Value = parse(world.pointer("/world3d/instancesJson").and_then(Value::as_str).expect("the world body publishes an instances lane"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("instances lane is json");
        instances
            .as_array()
            .expect("instances lane is an array")
            .iter()
            .find(|instance| instance.get("id").and_then(Value::as_str) == Some(object_id))
            .and_then(|instance| instance.get("scale").and_then(Value::as_array))
            .map(|scale| scale.iter().filter_map(Value::as_f64).collect())
            .unwrap_or_default()
    }
    let flag_args = |object_id: &str, value: bool| json!({ "entity": "object", "hidden": value, "ids": [object_id] });
    let mut app = app().await;
    let object_id = first_object_id(&app);
    assert_eq!(instance_scale(&render_body(&mut app, main::BODY_KEY).await, &object_id), vec![1.0, 1.0, 1.0], "a visible object publishes its real scale");
    dispatch(&mut app, "setSelectionHidden", Some(&flag_args(&object_id, true)), None).await.expect("setSelectionHidden hidden=true");
    assert_eq!(instance_scale(&render_body(&mut app, main::BODY_KEY).await, &object_id), vec![0.0, 0.0, 0.0], "the hide half must still reach the world lane");
    dispatch(&mut app, "setSelectionHidden", Some(&flag_args(&object_id, false)), None).await.expect("setSelectionHidden hidden=false");
    let shown = render_body(&mut app, main::BODY_KEY).await;
    assert_eq!(object_flag(&app, &object_id, "hidden"), Some(false), "the show write must reach the document");
    assert_eq!(instance_scale(&shown, &object_id), vec![1.0, 1.0, 1.0], "un-hiding must republish the object's real scale — a residency that cached the zero-scale record would leave it invisible for good");
}

/// 🎯️ An outliner row's flag write names its OWN entity, so the live selection must not decide what it
/// hits. `set_selection_flag` chooses between the explicit `{entity, ids}` the row declares and the whole
/// live selection, and the browser only ever exercised the branch with NOTHING selected: with a sibling
/// entity selected, `outliner-hide-applies` failed on the same row with the same args while the command
/// still settled `command-complete` with `historyUpserts: 0` (ticket 26/09/02/PUZZLE-3D-END-TO-END wave
/// B47 §6 — PASS with an empty selection at 2 345 ms, FAIL with a sibling selected at 30 237 ms). Locked,
/// too: a lock guards an entity's GEOMETRY, and a row that says "Hide" must still hide.
#[semio_framework_async_macros::async_test]
async fn an_explicit_outliner_flag_write_ignores_whatever_is_selected() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select a vortex, i.e. NOT the object the row names");
    dispatch(&mut app, "setSelectionLocked", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "locked": true })), None).await.expect("lock the object by its own row");
    assert_eq!(object_flag(&app, &object_id, "locked"), Some(true), "an explicit lock write lands while a vortex is the live selection");

    dispatch(&mut app, "setSelectionHidden", Some(&json!({ "entity": "object", "hidden": true, "ids": [object_id.clone()] })), None).await.expect("hide the object by its own row");

    assert_eq!(object_flag(&app, &object_id, "hidden"), Some(true), "the row's explicit id decides what is hidden — never the live selection, and never its lock");
}

/// 🎯️ The rendered outliner row that offers `verb` and whose ONE target names `object_id` — the row a host binds that verb to.
fn outliner_verb_row<'a>(node: &'a Value, object_id: &str, verb: &str) -> Option<&'a Value> {
    let offers = node.get("rowActions").and_then(Value::as_array).is_some_and(|actions| actions.iter().any(|action| action.get("verb").and_then(Value::as_str) == Some(verb)));
    let names = node.pointer("/target/args/ids").and_then(Value::as_array).is_some_and(|ids| ids.iter().any(|id| id.as_str() == Some(object_id)));
    if offers && names {
        return Some(node);
    }
    match node {
        Value::Object(fields) => fields.iter().find_map(|(_, child)| outliner_verb_row(child, object_id, verb)),
        Value::Array(items) => items.iter().find_map(|item| outliner_verb_row(item, object_id, verb)),
        _ => None,
    }
}

/// 🙈️ LAW: the outliner's set-verbs are idempotent by value — replaying `setSelectionHidden`/`setSelectionLocked` with the
/// same `{entity, ids, flag: true}` (a stale view's second click) leaves exactly the first dispatch's one applied history
/// row and a byte-identical document; and a set-verb without its value is refused at the command boundary, never defaulted.
#[semio_framework_async_macros::async_test]
async fn outliner_set_verbs_are_idempotent_by_value_and_refuse_a_missing_value() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let bare = json!({ "entity": "object", "ids": [object_id.clone()] });
    for (verb, flag, args) in [
        ("setSelectionHidden", "hidden", json!({ "entity": "object", "hidden": true, "ids": [object_id.clone()] })),
        ("setSelectionLocked", "locked", json!({ "entity": "object", "ids": [object_id.clone()], "locked": true })),
    ] {
        assert!(Puzzle3dPlayApp::command_from_action(verb, Some((semio_framework_pack_json::to_dsl_value(&bare)).clone()), None).is_err(), "{verb} without its {flag} value must be refused");
        let (first, settled) = dispatch_reporting(&mut app, verb, Some(&args), None).await;
        first.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
        let rows = history_row_labels(&settled);
        assert_eq!(rows.iter().filter(|row| row.contains("applied=true") && !row.contains("ops=0")).count(), 1, "{verb} commits exactly one document edit: {rows:?}");
        assert_eq!(object_flag(&app, &object_id, flag), Some(true), "{verb} sets {flag}");
        let set = committed_document(&app);
        let (replay, settled) = dispatch_reporting(&mut app, verb, Some(&args), None).await;
        replay.unwrap_or_else(|fault| panic!("{verb} replay: {fault:?}"));
        let rows = history_row_labels(&settled);
        assert!(rows.iter().all(|row| !row.contains("applied=true") || row.contains("ops=0")), "replaying {verb} with the same value commits no second edit: {rows:?}");
        assert_eq!(committed_document(&app), set, "replaying {verb} with the same value leaves the document byte-identical");
    }
}

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

//#endregion 🩹️CheckedDefects
