//! 👆️ and every gesture the plugin turns into a document edit is the concrete mutation kinds it consists of. Each law
//! 🧭️ Host gesture protocol laws: the renderer and the panels report GESTURES (verbs with ids and values), never a scene,
//! builds the kinds from a typed base alone, replays them forward through the central applier and the recorded
//! inverse rows last-to-first, and requires the base back.

use super::*;
use semio_plugin::artifact_app_laws;
use {semio_s_artifact_puzzle_3d_mutations_object::change_object_hidden::mutation::change_object_hidden, semio_s_artifact_puzzle_3d_mutations_object::change_object_locked::mutation::change_object_locked, semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object, semio_s_artifact_puzzle_3d_mutations_object::delete_object::mutation::delete_object, semio_s_artifact_puzzle_3d_mutations_vortex::disconnect_vortices::mutation::disconnect_vortices};
#[cfg(test)] use semio_s_artifact_puzzle_3d_commands_document::set_selection_flag;
#[cfg(test)] use semio_s_artifact_puzzle_3d_commands_document::add_target_volume;

fn vortex(id: &str) -> Puzzle3dVortex {
    Puzzle3dVortex { label: None, id: id.into(), vortex_kind: None, position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false }
}

fn object(id: &str, origin: [f64; 3]) -> Puzzle3dObject {
    Puzzle3dObject { anchor: Default::default(), id: id.into(), label: None, object_kind: Some("Object".into()), origin, orientation: None, scale: None, mesh_url: None, vortices: vec![vortex("v0")], hidden: false, locked: false }
}

fn base() -> Puzzle3dSnapshot {
    let mut scene = empty_scene_snapshot();
    scene.objects = vec![object("a", [0.0, 0.0, 0.0]), object("b", [1.0, 0.0, 0.0]), object("c", [2.0, 0.0, 0.0])];
    scene.attractions = vec![
        Puzzle3dAttraction { id: "t1".into(), attracting: "a:v0".into(), attracted: "b:v0".into(), ..Default::default() },
        Puzzle3dAttraction { id: "t2".into(), attracting: "b:v0".into(), attracted: "c:v0".into(), ..Default::default() },
    ];
    puzzle3d_snapshot_from_host_snapshot(&scene).expect("fixture admits")
}

fn replay(before: &Puzzle3dSnapshot, operations: &[Puzzle3dMutation]) -> (Puzzle3dSnapshot, Puzzle3dSnapshot) {
    let mut state = before.clone();
    let mut inverses = Vec::new();
    for operation in operations {
        inverses.extend(protocol::ColdMutation::<Puzzle3dSnapshot>::inverse(operation, &state).expect("concrete inverse"));
        state = protocol::apply_diff_cold(protocol::ColdMutation::<Puzzle3dSnapshot>::diff(operation, &state).diff(), &state).expect("central apply");
    }
    let after = state.clone();
    for inverse in inverses.iter().rev() {
        state = protocol::apply_diff_cold(protocol::ColdMutation::<Puzzle3dSnapshot>::diff(inverse, &state).diff(), &state).expect("central inverse apply");
    }
    (after, state)
}

#[test]
fn a_delete_gesture_on_an_object_is_one_delete_object_and_undo_restores_the_document() {
    let before = base();
    let operations = delete_selection::delete_selection_mutations(&before, &["b".to_string()], &HashSet::new(), &[], &[], &[]);
    assert_eq!(operations, vec![delete_object("b".into())]);
    let (after, restored) = replay(&before, &operations);
    assert_eq!(after.objects.len(), 2);
    assert!(after.attractions.is_empty(), "the object's delete severs both attractions it carries");
    assert_eq!(restored, before);
}

#[test]
fn a_delete_gesture_on_an_attraction_is_one_disconnect_and_a_cascaded_attraction_adds_no_second_row() {
    let before = base();
    let alone = delete_selection::delete_selection_mutations(&before, &[], &HashSet::new(), &["t1".to_string()], &[], &[]);
    assert_eq!(alone, vec![disconnect_vortices("t1".into())]);
    let (after, restored) = replay(&before, &alone);
    assert_eq!(after.attractions.len(), 1);
    assert_eq!(restored, before);
    let with_owner = delete_selection::delete_selection_mutations(&before, &["a".to_string()], &HashSet::new(), &["t1".to_string()], &[], &[]);
    assert_eq!(with_owner, vec![delete_object("a".into())], "t1 is severed by deleting a, so it names no row of its own");
    let (_, restored) = replay(&before, &with_owner);
    assert_eq!(restored, before);
}

#[test]
fn a_delete_gesture_on_a_vortex_removes_only_that_vortex_of_a_surviving_object() {
    let before = base();
    let vortices: HashSet<String> = [puzzle3d_vortex_full_id("c", "v0")].into_iter().collect();
    let operations = delete_selection::delete_selection_mutations(&before, &[], &vortices, &[], &[], &[]);
    assert_eq!(operations, vec![semio_s_artifact_puzzle_3d_mutations_vortex::remove_object_vortex::mutation::remove_object_vortex("c".into(), "v0".into())]);
    let (after, restored) = replay(&before, &operations);
    assert!(after.objects[2].vortices.is_empty());
    assert_eq!(after.attractions.len(), 1, "the vortex's attraction is severed with it");
    assert_eq!(restored, before);
}

#[test]
fn a_flag_gesture_emits_one_row_per_entity_whose_flag_differs_and_none_when_it_already_holds() {
    let before = base();
    let hide = set_selection_flag::selection_flag_mutations(&before, "object", &["a".to_string(), "c".to_string()], "hidden", true);
    assert_eq!(hide, vec![change_object_hidden("a".into(), true), change_object_hidden("c".into(), true)]);
    let (after, restored) = replay(&before, &hide);
    assert!(after.objects[0].hidden && !after.objects[1].hidden && after.objects[2].hidden);
    assert_eq!(restored, before);
    assert!(set_selection_flag::selection_flag_mutations(&after, "object", &["a".to_string()], "hidden", true).is_empty());
    assert_eq!(set_selection_flag::selection_flag_mutations(&before, "object", &["b".to_string()], "locked", true), vec![change_object_locked("b".into(), true)]);
}

#[test]
fn a_vortex_flag_gesture_is_one_replace_object_vortex_carrying_the_base_vortex_with_the_flag() {
    let before = base();
    let operations = set_selection_flag::selection_flag_mutations(&before, "vortex", &[puzzle3d_vortex_full_id("b", "v0")], "locked", true);
    assert_eq!(operations.len(), 1);
    let (after, restored) = replay(&before, &operations);
    assert!(after.objects[1].vortices[0].locked);
    assert_eq!(restored, before);
}

#[test]
fn a_cut_gesture_is_the_delete_kinds_of_the_selected_objects() {
    let before = base();
    let marks = Puzzle3dInteractionSnapshot { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), selected: vec!["a".into(), "c".into()], hovered: Vec::new(), referenced: Vec::new() };
    let operations = puzzle3d_cut_operations_from(&before, &marks);
    assert_eq!(operations, vec![delete_object("a".into()), delete_object("c".into())]);
    let (after, restored) = replay(&before, &operations);
    assert_eq!(after.objects.len(), 1);
    assert_eq!(restored, before);
}

#[test]
fn a_paste_gesture_is_one_create_object_per_clone_with_a_fresh_id() {
    let before = base();
    let marks = Puzzle3dInteractionSnapshot { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), selected: vec!["a".into()], hovered: Vec::new(), referenced: Vec::new() };
    let fragment = puzzle3d_copy_fragment_from(&before, &puzzle3d_selected_objects_from(&marks, &before)).expect("copy fragment");
    let operations = puzzle3d_paste_operations(&before, &fragment, &PastePlacement::default()).expect("paste");
    assert_eq!(operations.len(), 1);
    assert!(matches!(&operations[0], Puzzle3dMutation::CreateObject(create) if create.object.id != "a" && create.object.origin == [0.5, 0.5, 0.0]), "{operations:?}");
    let (after, restored) = replay(&before, &operations);
    assert_eq!(after.objects.len(), 4);
    assert_eq!(restored, before);
}

#[test]
fn clipboard_paste_preserves_relative_origins_and_only_internal_attractions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️clipboard.json")).unwrap();
    let mut before = base();
    let source = fixture["internalAttraction"].as_str().unwrap();
    let gap = fixture["gap"].as_f64().unwrap();
    let vortex_ids: Vec<String> = serde_json::from_value(fixture["qualifiedVortices"].clone()).unwrap();
    let internal = before.attractions.iter_mut().find(|attraction| attraction.id == source).unwrap();
    internal.gap = gap;
    internal.attracted = vortex_ids[0].clone();
    before.attractions[1].attracting = vortex_ids[0].clone();
    let original_vortex = before.objects[1].vortices[0].clone();
    before.objects[1].vortices = vortex_ids.iter().map(|id| semio_s_artifact_puzzle_3d_model::Puzzle3dVortex {
        id: id.clone(), ..original_vortex.clone()
    }).collect();
    let scale = fixture["uniformScale"].as_f64().unwrap();
    before.objects[1].scale = Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Uniform(scale));
    let selected = serde_json::from_value(fixture["selected"].clone()).unwrap();
    let marks = Puzzle3dInteractionSnapshot { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), selected, ..Default::default() };
    let fragment = puzzle3d_copy_fragment_from(&before, &puzzle3d_selected_objects_from(&marks, &before)).expect("copy fragment");
    let payload: serde_json::Value = serde_json::from_str(&fragment.dsl_text).expect("external clipboard decoder");
    let attractions = payload["attractions"].as_array().unwrap();
    assert_eq!(payload["objects"].as_array().unwrap().len(), 2);
    assert_eq!(payload["objects"][1]["scale"], fixture["uniformScale"]);
    assert_eq!(attractions.len(), 1);
    assert_eq!(attractions[0]["id"], fixture["internalAttraction"]);
    assert_ne!(attractions[0]["id"], fixture["excludedAttraction"]);

    let placement = PastePlacement {
        anchor: semio_plugin::kernel::PasteAnchor::Centroid,
        position: serde_json::from_value(fixture["position"].clone()).unwrap(),
    };
    let operations = puzzle3d_paste_operations(&before, &fragment, &placement).expect("paste");
    let (after, restored) = replay(&before, &operations);
    let clones = &after.objects[before.objects.len()..];
    let expected: Vec<[f64; 3]> = serde_json::from_value(fixture["origins"].clone()).unwrap();
    assert_eq!(clones.iter().map(|object| object.origin).collect::<Vec<_>>(), expected);
    assert_eq!(clones[1].scale, before.objects[1].scale);
    let projected = semio_s_artifact_puzzle_3d_editor_snapshot::scene_object_from_snapshot(&clones[1]);
    assert_eq!(semio_s_artifact_puzzle_3d_editor_snapshot::object_scale_json(&projected), [scale; 3]);
    assert_eq!(after.attractions.len(), before.attractions.len() + 1);
    let connection = after.attractions.last().unwrap();
    assert_ne!(connection.id, source);
    assert_eq!(connection.gap, gap);
    assert_eq!(connection.attracting, puzzle3d_vortex_full_id(&clones[0].id, &clones[0].vortices[0].id));
    assert_eq!(connection.attracted, puzzle3d_vortex_full_id(&clones[1].id, &clones[1].vortices[0].id));
    assert_ne!(connection.attracted, vortex_ids[0]);
    let expected_suffixes: Vec<String> = serde_json::from_value(fixture["clonedVortexSuffixes"].clone()).unwrap();
    let expected_vortices: Vec<_> = expected_suffixes.iter().map(|suffix| format!("{}:{suffix}", clones[1].id)).collect();
    assert_eq!(clones[1].vortices.iter().map(|vortex| &vortex.id).collect::<Vec<_>>(), expected_vortices.iter().collect::<Vec<_>>());
    assert_eq!(restored, before);
}

#[semio_framework_async_macros::async_test]
async fn clipboard_actions_apply_anchor_and_respect_locked_cut() {
    let mut app = app().await;
    let before = projection_of(&app);
    let originals = before.get("objects").and_then(Value::as_array).unwrap();
    let original_ids: HashSet<&str> = originals.iter().filter_map(|object| object.get("id").and_then(Value::as_str)).collect();
    let source = originals.iter().find(|object| object.get("locked").and_then(Value::as_bool) != Some(true))
        .and_then(|object| object.get("id").and_then(Value::as_str)).expect("an unlocked fixture object");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, source).await.expect("select");
    let copied = dispatch(&mut app, "copy", None, None).await.expect("copy");
    let fragment = copied.requested_effects.iter().find_map(|effect| match effect {
        Effect::ClipboardWrite { fragment } => Some(semio_framework_value::ToValue::to_value(fragment)),
        _ => None,
    }).expect("clipboard fragment");

    let position = [4.0, -3.0, 2.0];
    let args = json!({ "fragment": semio_framework_pack_json::from_dsl_value(&fragment), "anchor": "centroid", "position": position });
    dispatch(&mut app, "paste", Some(&args), None).await.expect("paste");
    assert_eq!(object_count(&app), originals.len() + 1);
    let projection = projection_of(&app);
    let clone = projection.get("objects").and_then(Value::as_array).unwrap().iter().find_map(|object| {
        object.get("id").and_then(Value::as_str).filter(|id| !original_ids.contains(id))
    }).expect("fresh clone");
    let origin = object_origin(&app, clone);
    assert_eq!(origin.len(), position.len());
    assert!(origin.iter().zip(position).all(|(actual, expected)| (actual - expected).abs() < 1e-9), "{origin:?} should land at {position:?}");
    let interaction = app.interaction_state().await;
    let selection = &interaction.selection[PUZZLE3D_INTERACTION_DOMAIN];
    assert_eq!(selection.ids, [clone.to_owned()]);
    let archive = app.document_pack().await.expect("save the anchored paste");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), originals.len());

    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, source).await.expect("select original");
    let cut = dispatch(&mut app, "cut", None, None).await.expect("cut");
    assert_eq!(object_count(&app), originals.len() - 1);
    assert!(cut.requested_effects.iter().any(|effect| matches!(effect, Effect::ClipboardWrite { .. })));
    let interaction = app.interaction_state().await;
    assert!(interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()));
    dispatch(&mut app, "undo", None, None).await.expect("undo cut");
    assert_eq!(object_count(&app), originals.len());

    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, source).await.expect("select restored original");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [source], "flag": "locked", "value": true })), None).await.expect("lock");
    let refused = dispatch(&mut app, "cut", None, None).await.expect("cut locked selection");
    assert_eq!(object_count(&app), originals.len());
    assert!(!refused.requested_effects.iter().any(|effect| matches!(effect, Effect::ClipboardWrite { .. })));
    assert!(refused.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })));
    assert!(crate::context::close_witness(&mut app).expect("retire the original instance"));

    let mut reopened = crate::context::app().await;
    let (raw, cx) = reopened.parts();
    let mut identity = artifact_app_laws::fixture_identity();
    artifact_app_laws::load_document(raw, &archive, &mut identity, cx).await.expect("reopen the saved world");
    assert_eq!(projection_of(&reopened), projection);
    dispatch(&mut reopened, "undo", None, None).await.expect("undo paste after reopening");
    assert_eq!(projection_of(&reopened), before);
    dispatch(&mut reopened, "redo", None, None).await.expect("redo paste after reopening");
    assert_eq!(projection_of(&reopened), projection);
    assert!(crate::context::close_witness(&mut reopened).expect("retire the reopened instance"));
}

#[semio_framework_async_macros::async_test]
async fn cancelling_clipboard_admission_releases_the_unpublished_mount() {
    cancelled_clipboard_mount_at_phase(0).await;
}

#[semio_framework_async_macros::async_test]
async fn cancelling_clipboard_admission_restores_the_unpublished_operation_slot() {
    cancelled_clipboard_mount_at_phase(4).await;
}

async fn cancelled_clipboard_mount_at_phase(phase: u8) {
    let mut app = app().await;
    let selected = crate::context::first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &selected).await.expect("select the object to cut");
    let view = app.window_view(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
    let mut meta = Some(semio_plugin::ActionMeta { view_state: Some(std::sync::Arc::new(view)), ..app.meta("local") });
    let mut args = None;
    let mut identity = artifact_app_laws::fixture_identity();
    let (raw, cx) = app.parts();
    let policy = artifact_app_laws::fixture_mounted_policy();
    let fuel = artifact_app_laws::OriginalFixtureCaller::turn_fuel();
    let grant = cx.retained_grant();
    artifact_app_laws::ensure_fixture_cancellation_issuer(raw, cx).expect("register the initiating caller");
    raw.render_ctx().refresh_cache().await.expect("publish the current document projection");
    assert!(raw.render_ctx().current_reserved_cache().is_some());
    let original_reservations = raw.tools.typed_operation_reservations;

    for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
        if raw.media.reserved_mount.as_ref().is_some_and(|owner| owner.phase == phase) { break; }
        artifact_app_laws::fixture_renew_turn(cx, fuel, grant).expect("fund the next admission turn");
        semio_framework_job::testkit::renew_payload_page_grant(cx);
        let answered = raw.handle_action("cut", &mut args, &mut meta, &mut identity, cx).await.expect("enter the registered Cut mount");
        assert!(answered.is_none());
        raw.maintenance_step(cx, policy.maintenance).expect("refresh the reserved snapshot cache");
    }
    assert_eq!(raw.media.reserved_mount.as_ref().map(|owner| owner.phase), Some(phase), "{}", raw.typed_operation_diagnostic());
    assert!(raw.media.reserved_receipt.is_none());
    assert_ne!(raw.tools.typed_operation_reservations, original_reservations);
    assert!(raw.ingress.owns_original_action_ingress(cx), "the registered Cut must hold the caller's action ingress");
    cx.original_cancel_token().cancel_now();

    for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
        if raw.maintenance_source_terminal_is_empty(cx) { break; }
        artifact_app_laws::fixture_renew_turn(cx, fuel, grant).expect("fund the next source return turn");
        cx.with_original_retirement(|cx| raw.maintenance_source_close_step(cx, policy.close).map_err(Into::into))
            .expect("return the cancelled caller's original sources");
    }
    assert!(raw.maintenance_source_terminal_is_empty(cx), "pending source close demand {:?}, phase policy {:?}, remaining {:?}, fuel {}, preparation slot {} bytes, {}",
        raw.maintenance_source_close_demands(policy.close.maximum_copy_bytes, cx),
        policy.close, cx.retained_grant(), cx.fuel_remaining(), size_of_val(&*raw.pending.reserved_preparation),
        raw.typed_operation_diagnostic());
    assert!(raw.media.reserved_mount.is_none(), "the cancelled caller must release its empty mount shell");
    assert!(!raw.ingress.owns_original_action_ingress(cx), "the cancelled caller must release its action ingress record");
    assert!(!raw.ingress.action_source_pending(), "the cancelled caller must leave the parser available for the next action");
    assert!(raw.ingress.action_ingress_sources.allocation_admitted, "source retirement must keep the live registry backing");
    assert_eq!(raw.tools.typed_operation_reservations, original_reservations);
}

#[test]
fn a_duplicate_clone_is_a_create_object_and_a_painted_volume_is_a_create_target_volume() {
    let before = base();
    let clone = create_object(semio_s_artifact_puzzle_3d_model::Puzzle3dObject { anchor: Default::default(), id: "a-copy".into(), origin: [0.5, 0.5, 0.0], ..before.objects[0].clone() }, None);
    let volume = add_target_volume::add_target_volume_mutation([1.1, 2.0, 2.9], 1.0, [2, 2, 2]);
    let (after, restored) = replay(&before, &[clone, volume]);
    assert_eq!(after.objects.len(), 4);
    assert_eq!(after.target_volumes.len(), 1);
    assert_eq!(after.target_volumes[0].origin, [1.0, 2.0, 3.0]);
    assert_eq!(restored, before);
}
