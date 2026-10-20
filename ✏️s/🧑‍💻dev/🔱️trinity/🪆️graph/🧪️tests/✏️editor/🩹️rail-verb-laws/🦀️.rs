//! 🩹️ The rail verbs of the editor laws: patch nodes, undo and redo, select all and the text gesture verbs. See [`super`].

use super::*;
use semio_framework::Fault;

//#region 🩹️RailVerbLaws
/// 🕹️ Dispatches `action` with rail-staged text `args` (the Actions pane's own shape) and settles it.
pub(super) async fn dispatch_rail(app: &mut JackTestApp, action: &str, args: &[(&str, &str)],cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_plugin::PluginLifecycleFailure> {
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::Value::Object(args.iter().map(|(key, value)| ((*key).to_string(), semio_framework_pack_json::Value::String((*value).to_string()))).collect()));
    semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.app,action,Some(args),&meta("local"),&mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx,jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy().close).await?;
    artifact_app_laws::settle_registered_typed_operation(&mut app.app, JACK_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx).await.map(|_| ())
}

/// ⚖️ LAW: `patchNodes` pressed from the rail with an EMPTY `nodeIds` renames the selected nodes, and a
/// comma list in the text field names several nodes (S15: the verb "moved nothing").
#[semio_framework_async_macros::async_test]
async fn patch_nodes_from_the_rail_renames_the_selection_or_the_listed_nodes() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let (first, second) = (node_id_at(&app, 0).await, node_id_at(&app, 1).await);
    select_ast(&mut app, &[&first],cx).await;
    dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "S15 Selected")],cx).await.expect("an empty nodeIds patches the selection");
    assert_eq!(node_name(&app, &first).await, "S15 Selected");
    dispatch_rail(&mut app, "patchNodes", &[("nodeIds", &format!("{first}, {second}")), ("field", "name"), ("value", "S15 Listed")],cx).await.expect("a comma list names both nodes");
    assert_eq!((node_name(&app, &first).await, node_name(&app, &second).await), ("S15 Listed".to_string(), "S15 Listed".to_string()));

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

/// ⏪️ LAW: a rail `patchNodes` is one undoable edit of the local user, whether it renames one node or the whole
/// selection — undo restores every renamed node and redo renames them again (S15, session 12: after `selectAll` the
/// row had no ↶ and undo left the document renamed).
#[semio_framework_async_macros::async_test]
async fn a_rail_patch_nodes_is_undone_and_redone_as_one_local_edit() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    for selected in [1usize, 2] {
        let mut app = new_app().await;
        let mut ids = Vec::new();
        for index in 0..selected {
            ids.push(node_id_at(&app, index).await);
        }
        let before = node_names(&app, Some(&ids)).await;
        select_ast(&mut app, &ids.iter().map(String::as_str).collect::<Vec<_>>(),cx).await;
        dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "S15 Undo")],cx).await.expect("patch the selection");
        assert_eq!(node_names(&app, Some(&ids)).await, vec!["S15 Undo".to_string(); selected], "{selected} selected");
        artifact_app_laws::settle_history_verb(&mut app.app, "undo", JACK_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx).await;
        assert_eq!(node_names(&app, Some(&ids)).await, before, "undo restores all {selected} renamed nodes");
        artifact_app_laws::settle_history_verb(&mut app.app, "redo", JACK_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx).await;
        assert_eq!(node_names(&app, Some(&ids)).await, vec!["S15 Undo".to_string(); selected], "redo renames all {selected} again");
    
    app.close(cx);
}

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

/// 🕹️ Presses one argument-less framework verb (`selectAll`, `clearSelection`, `undo`, `redo`) the whole way a shell
/// does: admission, its reserved job, then the typed publication.
async fn framework_verb(app: &mut JackTestApp, action: &str,cx:&mut semio_framework_job::StepContext<'_>) {
    let admitted = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.app,action,None,&meta("local"),&mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx,jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy().close).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}"));
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.app, admitted).await.unwrap_or_else(|fault| panic!("{action} reserved job: {fault:?}"));
    artifact_app_laws::settle_registered_typed_operation(&mut app.app, JACK_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx).await.unwrap_or_else(|fault| panic!("{action} publication: {fault:?}"));
}

/// ⏪️ LAW: the shell's own round trip — `selectAll`, `patchNodes` with an empty `nodeIds`, a neutral
/// `clearSelection`, then undo and redo — renames every node, restores every name, and renames them again.
#[semio_framework_async_macros::async_test]
async fn the_shells_select_all_patch_undo_redo_round_trip_restores_every_node() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let before = node_names(&app, None).await;
    framework_verb(&mut app, "selectAll",cx).await;
    dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "S15 All")],cx).await.expect("patch the whole selection");
    let patched = node_names(&app, None).await;
    assert!(patched.iter().all(|name| name == "S15 All"), "every selected node renamed: {patched:?}");
    framework_verb(&mut app, "clearSelection",cx).await;
    framework_verb(&mut app, "undo",cx).await;
    assert_eq!(node_names(&app, None).await, before, "undo restores every node");
    framework_verb(&mut app, "redo",cx).await;
    assert_eq!(node_names(&app, None).await, patched, "redo renames them again");

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

/// ⏪️ LAW: after the host LOADS the curated example (envelope ingress, swapped store), the shell's `selectAll` →
/// `patchNodes` → `clearSelection` → undo → redo round trip still treats the rename as the local user's edit.
#[semio_framework_async_macros::async_test]
async fn a_loaded_example_keeps_the_local_users_patch_undoable() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let example = SemioJackComposition::initial_snapshot();
    let handle = admit_jack_envelope(&mut app, &jack_envelope_wire_of(example),cx);
    assert_eq!(drive_jack_live_load(&mut app, handle,cx), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("exact load acknowledgement"));
    let before = node_names(&app, None).await;
    assert!(!before.is_empty(), "the curated example has nodes");
    framework_verb(&mut app, "selectAll",cx).await;
    dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "S15 Loaded")],cx).await.expect("patch the whole selection");
    let patched = node_names(&app, None).await;
    assert!(patched.iter().all(|name| name == "S15 Loaded"), "every selected node renamed: {patched:?}");
    framework_verb(&mut app, "clearSelection",cx).await;
    framework_verb(&mut app, "undo",cx).await;
    assert_eq!(node_names(&app, None).await, before, "undo restores every node of the loaded example");
    framework_verb(&mut app, "redo",cx).await;
    assert_eq!(node_names(&app, None).await, patched, "redo renames them again");

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

/// 🧹️ LAW: `clearSelection` on a graph with nothing selected settles like any other turn and leaves the program's
/// dispatch lane free — the next verb and undo run (S15, session 12: the empty clear never settled and held every
/// later undo, Cmd+Z and patchNodes).
#[semio_framework_async_macros::async_test]
async fn clear_selection_on_an_empty_selection_settles_and_frees_the_lane() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    framework_verb(&mut app, "clearSelection",cx).await;
    let first = node_id_at(&app, 0).await;
    dispatch_rail(&mut app, "patchNodes", &[("nodeIds", &first), ("field", "name"), ("value", "S15 After Clear")],cx).await.expect("the lane is free after an empty clear");
    assert_eq!(node_name(&app, &first).await, "S15 After Clear");
    framework_verb(&mut app, "clearSelection",cx).await;
    framework_verb(&mut app, "undo",cx).await;
    assert_ne!(node_name(&app, &first).await, "S15 After Clear", "undo after a clear still reaches the edit");

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

/// ⚖️ LAW: a `patchNodes` that cannot move the document is refused by name — an unknown id is
/// `mutation.target-missing`, no id and no selection is `app.command.targets-required` (the precondition an agent,
/// which has no selection, meets by naming `nodeIds`), an unsupported field or an empty value is
/// `app.command.invalid-args` — and the document stays untouched. It used to answer an empty emit that read as an
/// accepted edit.
#[semio_framework_async_macros::async_test]
async fn patch_nodes_refuses_what_it_cannot_apply_and_leaves_the_document_untouched() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let first = node_id_at(&app, 0).await;
    let before = node_names(&app, None).await;
    let refusal = |result: Result<(), Fault>| result.err().expect("refused").code.0;
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", "no-such-node"), ("field", "name"), ("value", "x")],cx).await), "mutation.target-missing");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "x")],cx).await), "app.command.targets-required");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", first.as_str()), ("field", "kind"), ("value", "x")],cx).await), "app.command.invalid-args");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", first.as_str()), ("field", "name"), ("value", "  ")],cx).await), "app.command.invalid-args");
    assert_eq!(node_names(&app, None).await, before, "every refusal leaves the content child untouched");

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

/// ⚖️ LAW: the query editor's gesture verbs stay out of the palette and the Actions rail — a press there
/// carries no buffer or caret, so `textSelect` was refused `missing start` (S15) and `textEdit` would
/// have emptied the query — while the editor window still declares both for its text host.
#[semio_framework_async_macros::async_test]
async fn the_text_gesture_verbs_are_kept_off_the_rail() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let definition = create_trinity_jack_app();
    let editor = definition.window_kinds.iter().find(|window| window.id == "trinity-jack-editor").expect("the query editor window");
    let actions = semio_framework_plugin::resolve_window_actions(&definition, editor);
    for gesture in ["textEdit", "textSelect"] {
        let action = actions.iter().find(|action| action.id == gesture).expect("the query editor still declares its text-host gesture");
        assert!(!action.in_palette, "{gesture} is a text-host gesture, never a rail row");
    }
    let patch = actions.iter().find(|action| action.id == "patchNodes").expect("declared");
    assert!(patch.args.iter().any(|arg| arg.id == "nodeIds" && !arg.required), "nodeIds is optional: empty means the selection");

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}
//#endregion 🩹️RailVerbLaws
