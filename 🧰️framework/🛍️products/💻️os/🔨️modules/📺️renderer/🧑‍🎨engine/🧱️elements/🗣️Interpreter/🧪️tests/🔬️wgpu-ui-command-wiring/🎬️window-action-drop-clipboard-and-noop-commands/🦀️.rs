/// 🎬️ Window action, drop, clipboard and no-op commands: the action fixtures, the clicked window context, the drop committed region, the mocked OS clipboard laws and the explicit no-op commands.

fn action(name: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: "ctrl".into(), action: name.into(), args: args.map(semio_framework::DslValue::from) }
}

/** 🎬️ A `UiIntentCommand` standing in for one the renderer fired: the case's descriptor with a
 * distinct per-surface `seq`, so `apply_ui_commands`' own admission gate accepts each case instead of
 * de-duplicating the second one as a replay of the first. */
fn fixture_intent(descriptor: ActionDescriptor, seq: usize) -> semio_framework_ui_action::UiIntentCommand {
    semio_framework_ui_action::UiIntentCommand {
        address: semio_framework_ui_action::UiIntentAddress { surface: "fixture".into(), revision: 1, node: 1, node_key: "control".into() },
        trigger: ui_contract::Trigger::Change,
        action: ui_contract::ActionId::try_v1(&descriptor.controller_id, &descriptor.action).expect("fixture action id"),
        args: descriptor.args,
        input: None,
        seq: seq as u64 + 1,
    }
}

#[test]
fn window_action_context_retained_commands_preserve_the_clicked_window() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🔬️window-action-context/🔣️.json")).unwrap();
    let window_id = fixture["clickedWindowId"].as_str().unwrap();
    for (case_index, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
        let descriptor = action("setValue", (!case["args"].is_null()).then(|| case["args"].clone()));
        apply_ui_commands(&mut vec![semio_framework_ui_wgpu_events::UiCommand::App { window_id: window_id.into(), intent: fixture_intent(descriptor, case_index) }], None, &mut input);
        let queued = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
        assert_eq!(queued.len(), 1);
        let actual: Option<Value> = queued[0].args.as_ref().map(|args| serde_json::from_str(&semio_framework_pack_json::from_dsl_value(args).to_string()).unwrap());
        assert_eq!(actual, Some(case["expected"].clone()));
    }
}

fn stack_with(id: &str, drop_action: Option<ActionDescriptor>, children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(semio_framework_ui_declaration::ui::UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some(id.into()), presence: UiPresence::default(), activate: None, drop_action, drop_overlay: None, children, menu: None })
}

//#region 🔖️DropCommittedTests
#[test]
fn decode_drop_payload_extracts_the_first_semio_mime_entry_as_a_json_object() {
    let mut payload = DragPayload::new();
    payload.insert("application/x-semio-catalogue-item".into(), "{\"id\":\"abc\"}".into());
    let decoded = decode_drop_payload(&payload).expect("a well-formed semio-mime JSON object payload should decode");
    assert_eq!(decoded.get("id").and_then(Value::as_str), Some("abc"));
}

#[test]
fn decode_drop_payload_ignores_non_semio_mimes_and_rejects_non_object_or_blank_json() {
    let mut no_semio_mime = DragPayload::new();
    no_semio_mime.insert("text/plain".into(), "\"abc\"".into());
    assert!(decode_drop_payload(&no_semio_mime).is_none(), "no application/x-semio-* entry present");

    let mut non_object = DragPayload::new();
    non_object.insert("application/x-semio-catalogue-item".into(), "\"not-an-object\"".into());
    assert!(decode_drop_payload(&non_object).is_none(), "a non-object JSON payload must not decode");

    let mut blank = DragPayload::new();
    blank.insert("application/x-semio-catalogue-item".into(), "   ".into());
    assert!(decode_drop_payload(&blank).is_none(), "a blank payload value must not decode");
}

#[test]
fn merge_action_args_lets_the_patch_win_over_existing_args() {
    let existing = serde_json::json!({"id": "abc", "kept": true});
    let existing_dsl = existing.into();
    let mut patch = serde_json::Map::new();
    patch.insert("id".to_string(), Value::from("overridden"));
    patch.insert("targetId".to_string(), Value::from("t1"));

    let merged: Value = serde_json::from_str(&semio_framework_pack_json::from_dsl_value(&merge_action_args(Some(&existing_dsl), patch).expect("merged args")).to_string()).expect("json args");

    assert_eq!(merged.get("id").and_then(Value::as_str), Some("overridden"));
    assert_eq!(merged.get("kept").and_then(Value::as_bool), Some(true));
    assert_eq!(merged.get("targetId").and_then(Value::as_str), Some("t1"));
}

#[test]
fn drop_target_action_reads_a_stacks_drop_action_from_the_retained_tree() {
    let window_id = "apply-ui-commands-drop-target-test";
    let expected = action("onDrop", None);
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("dz", Some(expected.clone()), vec![])));
    let target = UI_ENGINE.with(|cell| cell.borrow().tree(window_id).unwrap().root.unwrap());

    assert_eq!(drop_target_action(window_id, target), Some(expected));
}

#[test]
fn drop_target_action_is_none_for_a_stack_without_a_drop_action() {
    let window_id = "apply-ui-commands-drop-target-none-test";
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("dz", None, vec![])));
    let target = UI_ENGINE.with(|cell| cell.borrow().tree(window_id).unwrap().root.unwrap());

    assert!(drop_target_action(window_id, target).is_none());
}

#[test]
fn apply_drop_committed_queues_the_merged_action_into_input() {
    let window_id = "apply-ui-commands-drop-committed-test";
    let drop_action = action("onDrop", Some(serde_json::json!({"kept": true})));
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("dz", Some(drop_action), vec![])));
    let target = UI_ENGINE.with(|cell| cell.borrow().tree(window_id).unwrap().root.unwrap());
    let mut payload = DragPayload::new();
    payload.insert("application/x-semio-catalogue-item".into(), "{\"id\":\"abc\",\"windowId\":\"unrelated-window\"}".into());
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();

    apply_drop_committed(window_id, target, &payload, &mut input).expect("fixture drop admission succeeds");

    let queued = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(queued.len(), 1);
    assert_eq!(queued[0].controller_id, "ctrl");
    assert_eq!(queued[0].action, "onDrop");
    let args = queued[0].args.as_ref().expect("merged args");
    assert_eq!(args.get("id").and_then(semio_framework::DslValue::as_str), Some("abc"), "the decoded payload should flow through");
    assert_eq!(args.get("kept").and_then(semio_framework::DslValue::as_bool), Some(true), "the drop_action's own existing args should survive");
    assert_eq!(args.get("windowId").and_then(semio_framework::DslValue::as_str), Some(window_id), "the drop payload cannot replace its host-owned window target");
}

#[test]
fn apply_drop_committed_is_a_no_op_without_a_decodable_semio_payload() {
    let window_id = "apply-ui-commands-drop-committed-no-payload-test";
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("dz", Some(action("onDrop", None)), vec![])));
    let target = UI_ENGINE.with(|cell| cell.borrow().tree(window_id).unwrap().root.unwrap());
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();

    apply_drop_committed(window_id, target, &DragPayload::new(), &mut input).expect("fixture drop admission succeeds");

    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
}
//#endregion 🔖️DropCommittedTests

//#region 🔖️ClipboardTests
#[test]
fn clipboard_copy_and_cut_commands_write_through_the_mocked_os_clipboard() {
    MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow_mut().clear());
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();

    apply_ui_commands(&mut vec![semio_framework_ui_wgpu_events::UiCommand::ClipboardCopy { window_id: "w".into(), text: "hello".into() }, semio_framework_ui_wgpu_events::UiCommand::ClipboardCut { window_id: "w".into(), text: "world".into() }], None, &mut input);

    assert_eq!(MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow().clone()), vec!["hello".to_string(), "world".to_string()]);
}

#[test]
fn clipboard_paste_requested_reads_the_mocked_clipboard_and_inserts_it_at_the_focused_caret() {
    let window_id = "apply-ui-commands-clipboard-paste-test";
    publish_presented_document(window_id, 1, 401, "clipboard", clipboard_input_document(window_id));
    let focus_commands = UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(window_id, semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "Tab".into(), modifiers: semio_framework_ui_wgpu_events::EventModifiers::default() }));
    let focused = focus_commands
        .iter()
        .find_map(|cmd| match cmd {
            semio_framework_ui_wgpu_events::UiCommand::FocusChanged { node: Some(id), .. } => Some(*id),
            _ => None,
        })
        .expect("Tab should focus the only focusable node (the Input)");
    MOCK_CLIPBOARD_READ.with(|cell| *cell.borrow_mut() = Some("pasted".to_string()));
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();

    apply_clipboard_paste_requested(window_id, &mut input);

    let text = UI_ENGINE.with(|cell| cell.borrow().tree(window_id).unwrap().node(focused).unwrap().state.edit.clone().unwrap().text);
    assert_eq!(text, "pasted");
    retire_presented_document(window_id);
}

#[test]
fn clipboard_paste_requested_is_a_no_op_when_the_mocked_clipboard_is_empty() {
    let window_id = "apply-ui-commands-clipboard-paste-empty-test";
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("root", None, vec![])));
    MOCK_CLIPBOARD_READ.with(|cell| *cell.borrow_mut() = None);
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();

    apply_clipboard_paste_requested(window_id, &mut input);

    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn native_rgba_clipboard_content_encodes_a_natural_png_data_url() {
    use base64::Engine as _;

    let data_url = rgba_png_data_url(1, 2, vec![255, 0, 0, 255, 0, 255, 0, 255]).expect("exact RGBA dimensions encode");
    let png = base64::engine::general_purpose::STANDARD.decode(data_url.strip_prefix("data:image/png;base64,").expect("PNG data URL discriminator")).expect("base64 payload");
    let decoded = image::load_from_memory(&png).expect("third-party PNG decoder accepts renderer output").to_rgba8();
    assert_eq!((decoded.width(), decoded.height()), (1, 2));
    assert_eq!(decoded.into_raw(), [255, 0, 0, 255, 0, 255, 0, 255]);
    assert!(rgba_png_data_url(2, 2, vec![0; 4]).is_none(), "mismatched RGBA dimensions are refused rather than padded or truncated");
}
//#endregion 🔖️ClipboardTests

//#region 🔖️NoOpCommandTests
#[test]
fn drop_cancelled_overlay_closed_and_focus_changed_commands_are_explicit_no_ops() {
    let window_id = "apply-ui-commands-noop-test";
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("root", None, vec![])));
    let node = UI_ENGINE.with(|cell| cell.borrow().tree(window_id).unwrap().root.unwrap());
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &mut vec![
            semio_framework_ui_wgpu_events::UiCommand::DropCancelled { window_id: window_id.into(), source: node },
            semio_framework_ui_wgpu_events::UiCommand::OverlayClosed { window_id: window_id.into(), root: node, kind: semio_framework_ui_contract_presentation::OverlayKind::Tooltip },
            semio_framework_ui_wgpu_events::UiCommand::FocusChanged { window_id: window_id.into(), node: Some(node) },
        ],
        None,
        &mut input,
    );

    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "none of these three commands should ever queue an ActionDescriptor");
}
//#endregion 🔖️NoOpCommandTests
