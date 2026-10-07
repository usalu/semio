use super::*;

fn collect_text_editor_actions_accepted(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let _ = crate::engine_canvas::drive_text_editor_outbox_step(input).expect("text editor outbox");
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("action authority") {
        let queued = action.into_envelope().expect("action envelope");
        if let Some(receipt) = queued.receipt {
            crate::engine_canvas::settle_text_editor_action_receipt(receipt, crate::engine_canvas::TextEditorActionOutcome::Accepted);
        }
        actions.push(queued.descriptor);
    }
    actions
}

fn install_fixture_text_editor_focus(window_id: &str, window_generation: u64, node: NodeId, host_id: &str) -> FocusedTextEditor {
    if UI_ENGINE.with(|cell| cell.borrow().presented_document_id(window_id, node).is_none()) {
        present_seeded_scene_window(window_id);
    }
    let focus = UI_ENGINE.with(|cell| text_editor_focus_in(&cell.borrow(), window_id, window_generation, node, host_id)).expect("the accepted TextEditor fixture has an exact surface and document identity");
    install_text_editor_focus(focus.clone());
    focus
}

#[test]
fn retained_document_close_queue_is_bounded_and_same_window_replacement_costs_no_credit() {
    let mut queue = UiDocumentCloseQueue::default();
    let mut engine = ui_wgpu::wgpu::Ui::new(semio_framework_ui_locale::Locale::En);
    for index in 0..ui_wgpu::wgpu::engine::UI_LAYOUT_SURFACE_SLOTS {
        let window_id = SurfaceId::try_from(format!("close-{index}").as_str()).unwrap();
        let token = engine.try_admit_surface(window_id.as_ref()).unwrap();
        assert!(queue.try_upsert(UiDocumentCloseOwner { window_id, token, generation: 1 }));
    }
    let token = engine.surface_token("close-0").unwrap();
    assert!(!queue.try_upsert(UiDocumentCloseOwner { window_id: SurfaceId::try_from("close-overflow").unwrap(), token, generation: 1 }), "a sixty-fifth retained surface close is refused before its owner is lost");
    assert!(queue.try_upsert(UiDocumentCloseOwner { window_id: SurfaceId::try_from("close-0").unwrap(), token, generation: 2 }), "the same window replaces its stale close generation without another credit");
    assert_eq!(queue.owners.len(), ui_wgpu::wgpu::engine::UI_LAYOUT_SURFACE_SLOTS);
    assert_eq!(queue.owners.front().map(|owner| owner.generation), Some(2));
}

fn ingress_opportunity_document() -> UiDocumentLease {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🌳️document-tree-reconcile/🔣️.json")).unwrap();
    let source = &fixture["document"];
    let mut builder = ui_contract::UiDocumentBuilder::try_new(
        1,
        ui_contract::SurfaceId::try_from(source["surface"].as_str().unwrap()).unwrap(),
        ui_contract::UiRevision(1),
        Some(ui_contract::UiNodeId(source["root"].as_u64().unwrap())),
        source["layoutEpoch"].as_u64().unwrap(),
    )
    .unwrap();
    for record in source["nodes"].as_array().unwrap() {
        builder.try_push(serde_json::from_value(record.clone()).unwrap()).unwrap();
    }
    builder.finish().unwrap()
}

#[test]
fn retained_document_page_budget_refusal_preserves_the_cursor_and_retries_the_same_page() {
    for (fuel, deadline, clock, cancelled) in [(0, u64::MAX, (|| Some(0)) as fn() -> Option<u64>, false), (1, 1, (|| Some(2)) as fn() -> Option<u64>, false), (1, u64::MAX, (|| Some(0)) as fn() -> Option<u64>, true)] {
        let mut document = ingress_opportunity_document();
        let header = document.header().unwrap();
        let mut engine = ui_wgpu::wgpu::Ui::new(semio_framework_ui_locale::Locale::En);
        let mut cursor = UiDocumentFrameCursor::default();
        let mut sequence = 0;
        let cancel = semio_framework_job::CancelToken::root_now();
        let mut admitted = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), || Some(0), &mut sequence);
        engine.begin_document("budget-refusal", header, &mut admitted).unwrap();
        if cancelled {
            cancel.cancel_now();
        }
        let mut refused = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fuel, deadline), cancel.clone(), clock, &mut sequence);
        apply_retained_document_page(&mut engine, &mut cursor, "budget-refusal", document.read_node_page(0).unwrap().unwrap(), &mut refused);
        assert!(!cursor.terminal_is_fault(), "a refused opportunity must retain the page cursor for a fresh grant");
        assert!(matches!(engine.document_status("budget-refusal", 1), ui_wgpu::wgpu::engine::UiDocumentIngressStatus::Pending { next_page: 0, .. }));
        let mut retry =
            semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::CancelToken::root_now(), || Some(0), &mut sequence);
        apply_retained_document_page(&mut engine, &mut cursor, "budget-refusal", document.read_node_page(0).unwrap().unwrap(), &mut retry);
        assert!(!cursor.terminal_is_fault());
        assert!(matches!(engine.document_status("budget-refusal", 1), ui_wgpu::wgpu::engine::UiDocumentIngressStatus::Pending { next_page: 1, .. }));
        while !document.close_step() {}
    }
}

#[test]
fn retained_document_page_with_a_live_wrong_generation_remains_a_terminal_fault() {
    let mut document = ingress_opportunity_document();
    let mut engine = ui_wgpu::wgpu::Ui::new(semio_framework_ui_locale::Locale::En);
    let mut cursor = UiDocumentFrameCursor::default();
    let mut sequence = 0;
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut admitted = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), || Some(0), &mut sequence);
    engine.begin_document("wrong-generation", document.header().unwrap(), &mut admitted).unwrap();
    let mut stale = semio_framework_job::StepContext::new(semio_framework_job::OperationId(2), semio_framework_job::Generation(2), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, || Some(0), &mut sequence);
    apply_retained_document_page(&mut engine, &mut cursor, "wrong-generation", document.read_node_page(0).unwrap().unwrap(), &mut stale);
    assert!(cursor.terminal_is_fault());
    while !document.close_step() {}
}

fn action(name: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: "ctrl".into(), action: name.into(), args: args.map(semio_framework::DslValue::from) }
}

/** 🎬️ A `UiIntentCommand` standing in for one the renderer fired: the case's descriptor with a
 * distinct per-surface `seq`, so `apply_ui_commands`' own admission gate accepts each case instead of
 * de-duplicating the second one as a replay of the first. */
fn fixture_intent(descriptor: ActionDescriptor, seq: usize) -> ui_wgpu::wgpu::UiIntentCommand {
    ui_wgpu::wgpu::UiIntentCommand {
        address: ui_wgpu::wgpu::UiIntentAddress { surface: "fixture".into(), revision: 1, node: 1, node_key: "control".into() },
        trigger: ui_contract::Trigger::Change,
        action: ui_contract::ActionId::try_v1(&descriptor.controller_id, &descriptor.action).expect("fixture action id"),
        args: descriptor.args,
        input: None,
        seq: seq as u64 + 1,
    }
}

#[test]
fn window_action_context_retained_commands_preserve_the_clicked_window() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🔬️window-action-context/🔣️.json")).unwrap();
    let window_id = fixture["clickedWindowId"].as_str().unwrap();
    for (case_index, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let descriptor = action("setValue", (!case["args"].is_null()).then(|| case["args"].clone()));
        apply_ui_commands(&[ui_wgpu::wgpu::UiCommand::App { window_id: window_id.into(), intent: fixture_intent(descriptor, case_index) }], None, &mut input);
        let queued = crate::collect_fixture_actions(&mut input);
        assert_eq!(queued.len(), 1);
        let actual: Option<Value> = queued[0].args.as_ref().map(|args| serde_json::from_str(&semio_framework_pack_json::from_dsl_value(args).to_string()).unwrap());
        assert_eq!(actual, Some(case["expected"].clone()));
    }
}

fn stack_with(id: &str, drop_action: Option<ActionDescriptor>, children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(ui_wgpu::wgpu::UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some(id.into()), presence: UiPresence::default(), activate: None, drop_action, drop_overlay: None, children, menu: None })
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
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_drop_committed(window_id, target, &payload, &mut input).expect("fixture drop admission succeeds");

    let queued = crate::collect_fixture_actions(&mut input);
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
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_drop_committed(window_id, target, &DragPayload::new(), &mut input).expect("fixture drop admission succeeds");

    assert!(crate::collect_fixture_actions(&mut input).is_empty());
}
//#endregion 🔖️DropCommittedTests

//#region 🔖️ClipboardTests
#[test]
fn clipboard_copy_and_cut_commands_write_through_the_mocked_os_clipboard() {
    MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow_mut().clear());
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(&[ui_wgpu::wgpu::UiCommand::ClipboardCopy { window_id: "w".into(), text: "hello".into() }, ui_wgpu::wgpu::UiCommand::ClipboardCut { window_id: "w".into(), text: "world".into() }], None, &mut input);

    assert_eq!(MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow().clone()), vec!["hello".to_string(), "world".to_string()]);
}

#[test]
fn clipboard_paste_requested_reads_the_mocked_clipboard_and_inserts_it_at_the_focused_caret() {
    let window_id = "apply-ui-commands-clipboard-paste-test";
    publish_presented_document(window_id, 1, 401, "clipboard", clipboard_input_document(window_id));
    let focus_commands = UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(window_id, ui_wgpu::wgpu::UiEvent::KeyDown { key: "Tab".into(), modifiers: ui_wgpu::wgpu::EventModifiers::default() }));
    let focused = focus_commands
        .iter()
        .find_map(|cmd| match cmd {
            ui_wgpu::wgpu::UiCommand::FocusChanged { node: Some(id), .. } => Some(*id),
            _ => None,
        })
        .expect("Tab should focus the only focusable node (the Input)");
    MOCK_CLIPBOARD_READ.with(|cell| *cell.borrow_mut() = Some("pasted".to_string()));
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

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
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_clipboard_paste_requested(window_id, &mut input);

    assert!(crate::collect_fixture_actions(&mut input).is_empty());
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
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &[
            ui_wgpu::wgpu::UiCommand::DropCancelled { window_id: window_id.into(), source: node },
            ui_wgpu::wgpu::UiCommand::OverlayClosed { window_id: window_id.into(), root: node, kind: ui_wgpu::wgpu::OverlayKind::Tooltip },
            ui_wgpu::wgpu::UiCommand::FocusChanged { window_id: window_id.into(), node: Some(node) },
        ],
        None,
        &mut input,
    );

    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "none of these three commands should ever queue an ActionDescriptor");
}
//#endregion 🔖️NoOpCommandTests

//#region 🔖️SceneCommandTests
/// 🎬️ A minimal `ComponentScene` leaf — every optional per-`SurfaceKind` payload left `None`,
/// matching `ui_wgpu::wgpu::events::tests::component_scene_ui`'s own fixture shape (that one is private
/// to the sibling `ui_wgpu` crate, so this is a separate copy for this crate's own tests).
fn component_scene_ui(surface_id: &str, kind: ui_wgpu::wgpu::SurfaceKind) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        host_id: surface_id.into(),
        surface_id: surface_id.into(),
        controller_id: "ctrl".into(),
        component_kind: kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    })
}

/// 🌱️ `apply_tree`s a single-child `Stack(scene_node)` into `window_id` and returns the scene
/// leaf's own `NodeId` — every scene-command test below needs a real, live tree node since
/// `apply_scene_ui_command` re-fetches it by `(window_id, node)` from `UI_ENGINE`.
fn scene_surface_props(scene: &UiComponentSceneNode) -> ui_contract::SurfaceProps {
    let kind = match scene.component_kind {
        ui_wgpu::wgpu::SurfaceKind::Canvas2d => ui_contract::SurfaceKind::Canvas2d,
        ui_wgpu::wgpu::SurfaceKind::World3d => ui_contract::SurfaceKind::World3d,
        ui_wgpu::wgpu::SurfaceKind::NodeGraph => ui_contract::SurfaceKind::NodeGraph,
        ui_wgpu::wgpu::SurfaceKind::TextEditor => ui_contract::SurfaceKind::TextEditor,
        ui_wgpu::wgpu::SurfaceKind::Table => ui_contract::SurfaceKind::Table,
        ui_wgpu::wgpu::SurfaceKind::Paint2d => ui_contract::SurfaceKind::Paint2d,
        ui_wgpu::wgpu::SurfaceKind::VirtualFileSystem => ui_contract::SurfaceKind::VirtualFileSystem,
        ui_wgpu::wgpu::SurfaceKind::TiledMap => ui_contract::SurfaceKind::TiledMap,
        ui_wgpu::wgpu::SurfaceKind::Board2d => ui_contract::SurfaceKind::Board2d,
        ui_wgpu::wgpu::SurfaceKind::IconRender => ui_contract::SurfaceKind::IconRender,
        ui_wgpu::wgpu::SurfaceKind::InkCanvas => ui_contract::SurfaceKind::InkCanvas,
        ui_wgpu::wgpu::SurfaceKind::GraphTimeline => ui_contract::SurfaceKind::GraphTimeline,
        ui_wgpu::wgpu::SurfaceKind::BlockList => ui_contract::SurfaceKind::BlockList,
        ui_wgpu::wgpu::SurfaceKind::DiffView => ui_contract::SurfaceKind::DiffView,
        ui_wgpu::wgpu::SurfaceKind::EventFeed => ui_contract::SurfaceKind::EventFeed,
    };
    let encoded = match scene.component_kind {
        ui_wgpu::wgpu::SurfaceKind::Canvas2d => scene.canvas_2d.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::World3d => scene.world_3d.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::NodeGraph => scene.node_graph.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::TextEditor => scene.text_editor.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::Table => scene.table.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::Paint2d => scene.paint_2d.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::VirtualFileSystem => scene.virtual_file_system.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::TiledMap => scene.tiled_map.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::Board2d => scene.board2d.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::IconRender => scene.icon_render.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::InkCanvas => scene.ink_canvas.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::GraphTimeline => scene.graph_timeline.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::BlockList => scene.block_list.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::DiffView => scene.diff_view.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::EventFeed => scene.event_feed.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
    };
    encoded.transpose().expect("the bounded scene fixture encodes").unwrap_or_else(|| ui_contract::SurfaceProps {
        kind,
        doc_schema: ui_contract::UiText::try_from_str(&format!("{}@1", scene.component_kind.as_str())).expect("bounded fixture schema"),
        doc: Default::default(),
        bindings: Default::default(),
    })
}

fn scene_identity_document(window_id: &str, scene: &UiComponentSceneNode) -> ui_wgpu::wgpu::tree::UiDocumentTree {
    let header = ui_contract::UiDocumentLeaseHeader { generation: 1, surface: ui_contract::SurfaceId::try_from(window_id).unwrap(), revision: ui_contract::UiRevision(1), root: ui_contract::UiNodeId(0), layout_epoch: 1, node_count: 2 };
    let mut document = ui_wgpu::wgpu::tree::UiDocumentTree::new(header).unwrap();
    let mut children = ui_contract::UiFixedList::default();
    children.try_push(ui_contract::UiNodeId(1)).unwrap();
    let root = ui_contract::UiNodeRecord {
        id: ui_contract::UiNodeId(0),
        key: ui_contract::UiText::try_from_str("seed/root").unwrap(),
        component: ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
        layout: ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() }),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: Default::default(),
        menu: None,
        children,
    };
    let scene = ui_contract::UiNodeRecord {
        id: ui_contract::UiNodeId(1),
        key: ui_contract::UiText::try_from_str("seed/scene").unwrap(),
        component: ui_contract::Component::Surface(scene_surface_props(scene)),
        layout: ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() }),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: Default::default(),
        menu: None,
        children: Default::default(),
    };
    document.try_upsert_record(root).unwrap();
    document.try_upsert_record(scene).unwrap();
    document
}

fn clipboard_input_document(window_id: &str) -> ui_wgpu::wgpu::tree::UiDocumentTree {
    let records = serde_json::json!([
        {
            "id": 0,
            "key": "clipboard/root",
            "component": { "type": "container" },
            "layout": { "kind": "stack", "axis": "vertical", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "grow": true, "wrap": false },
            "style": {}, "activity": "idle", "accessibility": {}, "children": [1]
        },
        {
            "id": 1,
            "key": "clipboard/name",
            "component": { "type": "input", "kind": "text", "value": "" },
            "layout": { "kind": "leaf", "width": "fill", "height": "hug" },
            "style": {}, "activity": "idle", "accessibility": { "label": "Clipboard input" },
            "bindings": [{ "trigger": "change", "action": { "scope": "clipboard", "name": "onChange", "version": 1 } }]
        }
    ]);
    let rows = records.as_array().unwrap();
    let header = ui_contract::UiDocumentLeaseHeader { generation: 1, surface: ui_contract::SurfaceId::try_from(window_id).unwrap(), revision: ui_contract::UiRevision(1), root: ui_contract::UiNodeId(0), layout_epoch: 1, node_count: rows.len() };
    let mut document = ui_wgpu::wgpu::tree::UiDocumentTree::new(header).unwrap();
    for row in rows {
        document.try_upsert_record(serde_json::from_value(row.clone()).unwrap()).unwrap();
    }
    document
}

fn publish_presented_document(window_id: &str, generation: u64, witness: u64, controller_id: &str, document: ui_wgpu::wgpu::tree::UiDocumentTree) {
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, document)));
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    let complete = (0..4096).any(|_| {
        let mut step = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(64, u64::MAX), cancel.clone(), || Some(0), &mut sequence);
        match UI_ENGINE.with(|cell| cell.borrow_mut().step_document_reconcile(window_id, controller_id, &mut step)) {
            ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Pending => false,
            ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Complete => true,
            ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Fault(fault) => panic!("presented document reconcile fault: {fault:?}"),
        }
    });
    assert!(complete, "presented fixture reconcile reaches terminal within its fixed opportunity ceiling");
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    assert!(seal_presented_input_candidate(witness));
    assert!(acknowledge_presented_input(witness));
}

fn drive_scene_seed_reconcile(window_id: &str, controller_id: &str) {
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    for _ in 0..4096 {
        let mut step = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(64, u64::MAX), cancel.clone(), || Some(0), &mut sequence);
        match UI_ENGINE.with(|cell| cell.borrow_mut().step_document_reconcile(window_id, controller_id, &mut step)) {
            ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Pending => {}
            ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Complete => return,
            ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Fault(fault) => panic!("scene seed reconcile fault: {fault:?}"),
        }
    }
    panic!("scene seed reconcile exceeded its fixed opportunity ceiling");
}

fn seed_scene_window_with(window_id: &str, scene_node: UiNode) -> NodeId {
    let UiNode::ComponentScene(scene) = scene_node else { panic!("the seed scene fixture requires ComponentScene") };
    let controller_id = scene.controller_id.clone();
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, scene_identity_document(window_id, &scene))));
    drive_scene_seed_reconcile(window_id, &controller_id);
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let tree = engine.tree(window_id).expect("the retained scene window exists");
        let node = tree.children(tree.root.expect("the scene document has a root")).next().expect("the scene document has a child");
        assert!(tree.node(node).is_some_and(|retained| matches!(retained.spec.0, UiNode::ComponentScene(_))));
        node
    })
}

/// 🎞️ The presenter witness a seeded scene window's first presentation is accepted under.
const SEEDED_SCENE_PRESENTATION_WITNESS: u64 = 0x5EED_5CE7;

/// 🎞️ Presents a seeded window exactly as a frame presents it (seal + accepted pixels): since 09-27 every input, caret and
/// accessibility address resolves in the PRESENTED tree, so a law driving one needs the window presented.
fn present_seeded_scene_window(window_id: &str) {
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    assert!(seal_presented_input_candidate(SEEDED_SCENE_PRESENTATION_WITNESS), "the seeded scene window seals its presentation");
    assert!(acknowledge_presented_input(SEEDED_SCENE_PRESENTATION_WITNESS), "and its pixels are accepted");
}

/// 🪪️ The ComponentScene a seeded window mounted. Its host id is engine-minted (`reconcile::component_scene_host_id`, 09-21),
/// so every scene-scoped key, staged control and focus address reads it from here, never from the authored fixture scene.
fn retained_scene(window_id: &str, node: NodeId) -> UiComponentSceneNode {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let retained = engine.tree(window_id).and_then(|tree| tree.node(node)).expect("the retained scene node remains mounted");
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { panic!("the retained node is a ComponentScene") };
        scene.clone()
    })
}

/// 🔑️ A fixture key authored under the fixture's host id, rebased onto the engine-minted host id its retained scene carries.
fn rebased_scene_key(key: &str, authored_host: &str, retained_host: &str) -> String {
    format!("{retained_host}{}", key.strip_prefix(authored_host).expect("the fixture key is scoped by its authored host"))
}

/// ♿️ Stages, seals and accepts the editable-text cells a seeded Table window's paint publishes (09-27: only accepted cells
/// reach the accessibility tree).
fn accept_table_editable_text_cells(window_id: &str, node: NodeId, epoch: u64) {
    let retained = retained_scene(window_id, node);
    crate::scenes::remember_scene_theme(&Theme::default());
    let cells = crate::scenes::table_editable_text_accessibility_cells(&retained, Rect::new(0.0, 0.0, 200.0, 200.0), ui_wgpu::wgpu::UiDriverDrag::Handle);
    assert!(!cells.is_empty(), "the seeded Table paints its editable cells");
    crate::scenes::stage_table_editable_text_accessibility_cells(&retained.host_id, cells);
    crate::scenes::seal_table_editable_text_accessibility_candidates(epoch);
    crate::scenes::acknowledge_table_editable_text_accessibility_candidates(epoch);
}

fn seed_scene_window(window_id: &str, surface_id: &str, kind: ui_wgpu::wgpu::SurfaceKind) -> NodeId {
    seed_scene_window_with(window_id, component_scene_ui(surface_id, kind))
}

fn retained_scene_host_id(window_id: &str, node: NodeId) -> String {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let retained = engine.tree(window_id).and_then(|tree| tree.node(node)).expect("the retained scene node remains mounted");
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { panic!("the retained node is a ComponentScene") };
        scene.host_id.clone()
    })
}

fn retained_scene_surface_id(window_id: &str, node: NodeId) -> String {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let retained = engine.tree(window_id).and_then(|tree| tree.node(node)).expect("the retained scene node remains mounted");
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { panic!("the retained node is a ComponentScene") };
        scene.surface_id.clone()
    })
}

fn focus_rebase_document_with_ink(window_id: &str, generation: u64, kind: ui_wgpu::wgpu::SurfaceKind, children: &[u64], ink: Option<&ui_wgpu::wgpu::InkCanvasScene>) -> ui_wgpu::wgpu::tree::UiDocumentTree {
    let mut root: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": "focus-rebase/root",
        "component": { "type": "container" },
        "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
        "style": {}, "activity": "idle", "accessibility": {}, "children": children
    }))
    .unwrap();
    root.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Horizontal, grow: true, ..Default::default() });
    let mut document = ui_wgpu::wgpu::tree::UiDocumentTree::new(ui_contract::UiDocumentLeaseHeader {
        generation,
        surface: ui_contract::SurfaceId::try_from(window_id).unwrap(),
        revision: ui_contract::UiRevision(generation),
        root: ui_contract::UiNodeId(1),
        layout_epoch: generation,
        node_count: children.len() + 1,
    })
    .unwrap();
    document.try_upsert_record(root).unwrap();
    for id in children {
        let mut record: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
            "id": id,
            "key": format!("focus-rebase/{id}"),
            "component": { "type": "container" },
            "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
            "style": {}, "activity": "idle", "accessibility": {}, "children": []
        }))
        .unwrap();
        record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
        if *id == 4 {
            let surface = match kind {
                ui_wgpu::wgpu::SurfaceKind::Canvas2d => ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::Canvas2d, &ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into())).unwrap(),
                ui_wgpu::wgpu::SurfaceKind::TextEditor => ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::TextEditor, &ui_wgpu::wgpu::TextEditorScene::base("focus".into(), None, None)).unwrap(),
                ui_wgpu::wgpu::SurfaceKind::Table => ui_wgpu::wgpu::encode_surface_doc(
                    ui_contract::SurfaceKind::Table,
                    &ui_wgpu::wgpu::TableScene::base(
                        serde_json::json!([{ "id": "count", "label": "Count" }]).to_string(),
                        serde_json::json!([{ "id": "r1", "count": { "kind": "stepper", "value": 2.0, "min": 0.0, "max": 5.0, "step": 0.5, "action": { "controllerId": "focus-rebase", "action": "setCount", "args": { "objectId": "r1" } } } }])
                            .to_string(),
                    ),
                )
                .unwrap(),
                ui_wgpu::wgpu::SurfaceKind::InkCanvas => {
                    ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::InkCanvas, &ink.cloned().unwrap_or_else(|| ui_wgpu::wgpu::InkCanvasScene::base("[]".into(), "pen".into(), "document".into(), true))).unwrap()
                }
                _ => panic!("focus rebase fixture supports Canvas, Table and editor surfaces"),
            };
            record.component = ui_contract::Component::Surface(surface);
        }
        document.try_upsert_record(record).unwrap();
    }
    document
}

fn focus_rebase_document(window_id: &str, generation: u64, kind: ui_wgpu::wgpu::SurfaceKind, children: &[u64]) -> ui_wgpu::wgpu::tree::UiDocumentTree {
    focus_rebase_document_with_ink(window_id, generation, kind, children, None)
}

fn drive_focus_rebase_reconcile(window_id: &str, generation: u64) {
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    for _ in 0..4096 {
        let mut step = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(64, u64::MAX), cancel.clone(), || Some(0), &mut sequence);
        match UI_ENGINE.with(|cell| cell.borrow_mut().step_document_reconcile(window_id, "focus-rebase", &mut step)) {
            ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Pending => {}
            ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Complete => return,
            ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Fault(fault) => panic!("focus rebase reconcile fault: {fault:?}"),
        }
    }
    panic!("focus rebase reconcile exceeded its fixed opportunity ceiling");
}

fn focus_rebase_node(tree: &ui_wgpu::wgpu::UiTree) -> Option<NodeId> {
    tree.children(tree.root?).find(|node| tree.node(*node).is_some_and(|node| node.key == ui_wgpu::wgpu::NodeKey::Explicit("focus-rebase/4".into())))
}

fn publish_new_focus_rebase_document(window_id: &str, generation: u64, witness: u64, kind: ui_wgpu::wgpu::SurfaceKind, children: &[u64]) -> Option<NodeId> {
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, focus_rebase_document(window_id, generation, kind, children))));
    drive_focus_rebase_reconcile(window_id, generation);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    assert!(seal_presented_input_candidate(witness));
    assert!(acknowledge_presented_input(witness));
    UI_ENGINE.with(|cell| cell.borrow().tree(window_id).and_then(focus_rebase_node))
}

fn publish_focus_rebase_document(window_id: &str, generation: u64, witness: u64, kind: ui_wgpu::wgpu::SurfaceKind, children: &[u64]) -> Option<NodeId> {
    if generation > 1 {
        drive_focus_rebase_reconcile(window_id, generation - 1);
    }
    publish_new_focus_rebase_document(window_id, generation, witness, kind, children)
}

fn reconcile_focus_rebase_document(window_id: &str, generation: u64, kind: ui_wgpu::wgpu::SurfaceKind, children: &[u64]) {
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, focus_rebase_document(window_id, generation, kind, children))));
    drive_focus_rebase_reconcile(window_id, generation);
}

fn stage_focus_rebase_document(window_id: &str, generation: u64, witness: u64, kind: ui_wgpu::wgpu::SurfaceKind, children: &[u64]) -> NodeId {
    drive_focus_rebase_reconcile(window_id, generation - 1);
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, focus_rebase_document(window_id, generation, kind, children))));
    drive_focus_rebase_reconcile(window_id, generation);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    assert!(seal_presented_input_candidate(witness));
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let presented = engine.tree(window_id).and_then(focus_rebase_node).expect("presented editor node");
        let candidate = engine.candidate_tree(window_id).and_then(focus_rebase_node).expect("candidate editor node");
        assert_eq!(engine.candidate_scene_node_for_presented_node(window_id, witness, presented), Some(candidate));
        candidate
    })
}

fn stage_new_focus_rebase_document(window_id: &str, generation: u64, witness: u64, kind: ui_wgpu::wgpu::SurfaceKind, children: &[u64]) -> NodeId {
    drive_focus_rebase_reconcile(window_id, generation - 1);
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, focus_rebase_document(window_id, generation, kind, children))));
    drive_focus_rebase_reconcile(window_id, generation);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    assert!(seal_presented_input_candidate(witness));
    UI_ENGINE.with(|cell| cell.borrow().candidate_tree(window_id).and_then(focus_rebase_node).expect("candidate editor node"))
}

fn ink_intent_scene(law: &Value, utility: &str, document_id: &str) -> ui_wgpu::wgpu::InkCanvasScene {
    let mut document = law["document"].clone();
    document["id"] = Value::String(document_id.into());
    document["activeUtility"] = Value::String(utility.into());
    let mut scene = ui_wgpu::wgpu::InkCanvasScene::base(serde_json::to_string(&document).unwrap(), utility.into(), "edit".into(), true);
    scene.selection_json = law["scene"]["inkCanvas"]["selectionJson"].as_str().unwrap().into();
    scene
}

fn publish_ink_intent_document(window_id: &str, generation: u64, witness: u64, children: &[u64], ink: &ui_wgpu::wgpu::InkCanvasScene) -> NodeId {
    publish_presented_document(window_id, generation, witness, "focus-rebase", focus_rebase_document_with_ink(window_id, generation, ui_wgpu::wgpu::SurfaceKind::InkCanvas, children, Some(ink)));
    UI_ENGINE.with(|cell| cell.borrow().tree(window_id).and_then(focus_rebase_node).expect("presented Ink intent receiver"))
}

fn stage_ink_intent_document(window_id: &str, generation: u64, witness: u64, children: &[u64], ink: &ui_wgpu::wgpu::InkCanvasScene) -> NodeId {
    drive_focus_rebase_reconcile(window_id, generation - 1);
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, focus_rebase_document_with_ink(window_id, generation, ui_wgpu::wgpu::SurfaceKind::InkCanvas, children, Some(ink)))));
    drive_focus_rebase_reconcile(window_id, generation);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    assert!(seal_presented_input_candidate(witness));
    UI_ENGINE.with(|cell| cell.borrow().candidate_tree(window_id).and_then(focus_rebase_node).expect("candidate Ink intent receiver"))
}

fn begin_pending_ink_intent(window_id: &str, node: NodeId, pointer: ui_render::PointerId, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    apply_scene_ui_command(
        window_id,
        node,
        ui_wgpu::wgpu::SurfaceKind::InkCanvas,
        Rect::new(0.0, 0.0, 320.0, 240.0),
        &ui_wgpu::wgpu::UiEvent::PointerDown { x: 220.0, y: 190.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        Some(pointer),
        input,
    );
    assert!(drive_scene_interaction_step(input));
    assert!(SCENE_INTENTS.with(|cell| cell.borrow().slots.iter().flatten().any(|intent| intent.pointer_id == Some(pointer) && intent.ink_job.is_some() && !intent.retiring)));
}

#[test]
fn scene_command_dispatches_a_canvas2d_pointer_down_action() {
    let window_id = "apply-ui-commands-scene-canvas2d-pointer-down";
    let node = seed_scene_window(window_id, "s1", ui_wgpu::wgpu::SurfaceKind::Canvas2d);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &[ui_wgpu::wgpu::UiCommand::Scene {
            window_id: window_id.into(),
            node,
            surface_id: "s1".into(),
            kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
            rect,
            event: ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        }],
        Some(ui_render::PointerId(1)),
        &mut input,
    );

    let queued = crate::collect_fixture_actions(&mut input);
    assert!(queued.iter().any(|action| action.action == "canvasPointerDown"), "a real per-event PointerDown over a canvas-2d scene should reach the same handler apply_scene_pointer used to sample, got {queued:?}");
    crate::scenes::cancel_canvas_pointer_gesture(&mut input);
    crate::collect_fixture_actions(&mut input);
}

/// ⚖️ Law (packet W2j): the canvas-2d pointer wire is `📐️Canvas2dHost`'s wire. React's gesture lane
/// publishes `{ x, y, button, shift, ctrl, meta, alt, width, height }` with `x`/`y` SCREEN-LOGICAL
/// inside the canvas and `width`/`height` its logical size, because every plugin reader converts them
/// itself (`🖍️draw`'s `canvas_point_to_world(viewport, x, y, width, height)`, `📏️layout`'s
/// `hit_test_at(doc, cfg, x, y, width, height)`). wgpu used to publish world coordinates in the `x`/`y`
/// slots with no `width`/`height` at all, which converted twice against a 0×0 viewport.
#[test]
fn canvas2d_pointer_payload_is_react_shaped_screen_logical_with_a_world_lane() {
    let window_id = "apply-ui-commands-scene-canvas2d-pointer-payload";
    let node = seed_scene_window(window_id, "s1", ui_wgpu::wgpu::SurfaceKind::Canvas2d);
    let rect = Rect::new(30.0, 40.0, 200.0, 120.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &[ui_wgpu::wgpu::UiCommand::Scene {
            window_id: window_id.into(),
            node,
            surface_id: "s1".into(),
            kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
            rect,
            event: ui_wgpu::wgpu::UiEvent::PointerDown { x: 50.0, y: 70.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        }],
        Some(ui_render::PointerId(1)),
        &mut input,
    );

    let queued = crate::collect_fixture_actions(&mut input);
    let down = queued.iter().find(|action| action.action == "canvasPointerDown").expect("the press publishes canvasPointerDown");
    let args = down.args.as_ref().expect("the pointer payload is present");
    let number = |key: &str| args.get(key).and_then(semio_framework::DslValue::as_f64);
    assert_eq!(number("x"), Some(20.0), "x is logical pixels inside the surface rect, not a world coordinate");
    assert_eq!(number("y"), Some(30.0), "y is logical pixels inside the surface rect, not a world coordinate");
    assert_eq!(number("width"), Some(200.0), "width is the surface's logical width — draw/layout divide by it");
    assert_eq!(number("height"), Some(120.0), "height is the surface's logical height");
    assert_eq!(number("button"), Some(0.0));
    for flag in ["shift", "ctrl", "meta", "alt", "extend"] {
        assert_eq!(args.get(flag).and_then(semio_framework::DslValue::as_bool), Some(false), "{flag} must be present under React's own name");
    }
    assert!(number("worldX").is_some() && number("worldY").is_some(), "the optional world lane 🖍️draw declares rides along, got {args:?}");
    crate::scenes::cancel_canvas_pointer_gesture(&mut input);
    crate::collect_fixture_actions(&mut input);
}

/// 🎨️ `ink_wheel` reads straight from the scene's own `ink_canvas` payload (unlike TextEditor,
/// it needs no separate lazily-render-created host state) — mirrors `RenderEntry::ink_scene`'s
/// own fixture (`apply_scene_wheel_dispatches_actions_for_a_previously_dead_surface`).
#[test]
fn scene_command_dispatches_an_ink_canvas_scroll_action() {
    let window_id = "apply-ui-commands-scene-ink-canvas-scroll";
    let mut scene_node = component_scene_ui("s1", ui_wgpu::wgpu::SurfaceKind::InkCanvas);
    if let UiNode::ComponentScene(scene) = &mut scene_node {
        scene.ink_canvas = Some(ui_wgpu::wgpu::InkCanvasScene { document_json: "{}".into(), selection_json: "[]".into(), hovered_id: None, active_utility: String::new(), view_mode: "canvas".into(), interactive: true, interaction_domain: None });
    }
    let node = seed_scene_window_with(window_id, scene_node);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &[ui_wgpu::wgpu::UiCommand::Scene {
            window_id: window_id.into(),
            node,
            surface_id: "s1".into(),
            kind: ui_wgpu::wgpu::SurfaceKind::InkCanvas,
            rect,
            event: ui_wgpu::wgpu::UiEvent::Scroll { x: 10.0, y: 10.0, delta_x: 0.0, delta_y: -1.0, modifiers: Default::default() },
        }],
        None,
        &mut input,
    );

    let queued = crate::collect_fixture_actions(&mut input);
    assert!(queued.iter().any(|action| action.action == "setCamera"), "a real per-event Scroll over an ink-canvas scene should reach handle_scene_wheel, got {queued:?}");
}

fn ink_editing_law() -> Value {
    serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🖋️ink-canvas-editing/🔣️.json")).expect("shared InkCanvas editing law parses")
}

fn ink_editing_scene(law: &Value) -> UiNode {
    let mut scene_node = component_scene_ui(law["scene"]["surfaceId"].as_str().expect("surface id"), ui_wgpu::wgpu::SurfaceKind::InkCanvas);
    if let UiNode::ComponentScene(scene) = &mut scene_node {
        scene.controller_id = law["scene"]["controllerId"].as_str().expect("controller id").into();
        scene.ink_canvas = Some(ui_wgpu::wgpu::InkCanvasScene {
            document_json: serde_json::to_string(&law["document"]).expect("InkCanvas document serializes"),
            selection_json: law["scene"]["inkCanvas"]["selectionJson"].as_str().expect("selection").into(),
            hovered_id: None,
            active_utility: law["scene"]["inkCanvas"]["activeUtility"].as_str().expect("utility").into(),
            view_mode: law["scene"]["inkCanvas"]["viewMode"].as_str().expect("view mode").into(),
            interactive: law["scene"]["inkCanvas"]["interactive"].as_bool().expect("interactive"),
            interaction_domain: None,
        });
    }
    scene_node
}

#[track_caller]
fn drive_ink_scene_terminal(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    for _ in 0..4096 {
        if !drive_scene_interaction_step(input) {
            assert!(scene_interaction_terminal_is_empty());
            return;
        }
    }
    panic!("InkCanvas interaction did not reach a terminal state");
}

#[track_caller]
fn retire_presented_document(window_id: &str) {
    assert!(request_ui_document_close(window_id));
    drain_presented_document_close(window_id);
}

#[track_caller]
fn drain_presented_document_close(window_id: &str) {
    for _ in 0..262_144 {
        if !ui_document_close_pending() {
            break;
        }
        assert!(close_ui_document_one());
    }
    assert!(!ui_document_close_pending());
    assert!(UI_ENGINE.with(|cell| cell.borrow().tree(window_id).is_none()));
}

fn dispatch_ink_pointer(window_id: &str, node: NodeId, surface_id: &str, rect: Rect, x: f32, y: f32, down: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    let event = if down {
        ui_wgpu::wgpu::UiEvent::PointerDown { x, y, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() }
    } else {
        ui_wgpu::wgpu::UiEvent::PointerUp { x, y, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() }
    };
    apply_ui_commands(&[ui_wgpu::wgpu::UiCommand::Scene { window_id: window_id.into(), node, surface_id: surface_id.into(), kind: ui_wgpu::wgpu::SurfaceKind::InkCanvas, rect, event }], Some(ui_render::PointerId(1)), input);
    drive_ink_scene_terminal(input);
}

#[test]
fn ink_pointer_cancel_retires_only_the_exact_in_progress_owner_without_publishing() {
    let cancellation: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🛑️scene-pointer-cancellation/🔣️.json")).expect("shared scene cancellation fixture");
    let ink_case = cancellation["cases"].as_array().unwrap().iter().find(|case| case["family"] == "ink").expect("Ink cancellation case");
    assert_eq!(ink_case["terminal"], "discard");
    assert_eq!(ink_case["preservePublished"], serde_json::json!(["selection", "camera"]));
    assert_eq!(ink_case["invokeNormalPointerUp"], false);
    assert_eq!(ink_case["publishOnCancel"], serde_json::json!(["gesture-abort"]), "only a gesture that streamed publishes its abort");

    SCENE_INTENTS.with(|cell| *cell.borrow_mut() = SceneIntentQueue::default());
    SCENE_POINTER_OWNERS.with(|cell| *cell.borrow_mut() = ScenePointerOwners::default());
    let law = ink_editing_law();
    assert_eq!(cancellation["owner"]["zeroGeneration"], "invalid");
    let window_id = "ink-exact-pointer-cancel";
    let authored_surface_id = law["scene"]["surfaceId"].as_str().unwrap();
    let ink = ink_intent_scene(&law, law["scene"]["inkCanvas"]["activeUtility"].as_str().unwrap(), law["document"]["id"].as_str().unwrap());
    let node = publish_ink_intent_document(window_id, 1, 451, &[4], &ink);
    let surface_id = retained_scene_surface_id(window_id, node);
    assert_eq!(surface_id, window_id);
    assert_ne!(surface_id, authored_surface_id, "the mounted document owns the public action surface");
    let rect = Rect::new(0.0, 0.0, 640.0, 480.0);
    let pointer = ui_render::PointerId(41);
    let UiNode::ComponentScene(zero_generation_scene) = ink_editing_scene(&law) else { unreachable!() };
    assert!(matches!(
        crate::scenes::InkInteractionJob::new(1, 0, Some(pointer), &zero_generation_scene, crate::scenes::InkInteractionEvent::PointerDown { x: 64.0, y: 64.0, button: 0, shift: false }),
        Err(ui_wgpu::wgpu::BoundedActionFault::Structure)
    ));
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let down = ui_wgpu::wgpu::UiCommand::Scene {
        window_id: window_id.into(),
        node,
        surface_id: surface_id.clone(),
        kind: ui_wgpu::wgpu::SurfaceKind::InkCanvas,
        rect,
        event: ui_wgpu::wgpu::UiEvent::PointerDown { x: 64.0, y: 64.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
    };
    apply_ui_commands(&[down.clone()], Some(pointer), &mut input);
    let installed = (0..4096).any(|_| {
        let installed = SCENE_INTENTS.with(|cell| cell.borrow().slots.iter().flatten().any(|intent| intent.pointer_id == Some(pointer) && intent.ink_job.is_some() && !intent.retiring));
        installed || (drive_scene_interaction_step(&mut input) && SCENE_INTENTS.with(|cell| cell.borrow().slots.iter().flatten().any(|intent| intent.pointer_id == Some(pointer) && intent.ink_job.is_some() && !intent.retiring)))
    });
    assert!(installed, "the real scene worker installs an in-progress Ink job for the exact owner");

    assert!(cancel_scene_pointer(ui_render::PointerId(42), &mut input).is_empty(), "a sibling pointer cannot retire the Ink owner");
    assert!(SCENE_INTENTS.with(|cell| cell.borrow().slots.iter().flatten().any(|intent| intent.pointer_id == Some(pointer) && !intent.retiring)));
    let cancelled = cancel_scene_pointer(pointer, &mut input);
    assert_eq!(cancelled.len(), 1);
    assert_eq!(cancelled[0].surface_id, surface_id);
    assert_eq!(cancelled[0].kind, ui_wgpu::wgpu::SurfaceKind::InkCanvas);
    drive_ink_scene_terminal(&mut input);
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "a press that never streamed cannot synthesize PointerUp, publish Ink events or abort");

    let next_pointer = ui_render::PointerId(43);
    apply_ui_commands(&[down], Some(next_pointer), &mut input);
    assert!(SCENE_POINTER_OWNERS.with(|cell| cell.borrow().slots.iter().any(|owner| owner.pointer_id == next_pointer)), "the next Down is accepted after exact cancellation");
    cancel_scene_pointer(next_pointer, &mut input);
    drive_ink_scene_terminal(&mut input);
    retire_presented_document(window_id);
}

#[test]
fn a_streamed_note_ink_gesture_aborts_on_physical_cancel_while_stale_continuation_is_inert() {
    let surface_behavior: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🎬️surface-behavior/🔣️.json")).expect("shared app surface behavior");
    let behavior = surface_behavior["cases"].as_array().unwrap().iter().find(|entry| entry["id"] == "note-ink-canvas").expect("Note Ink behavior");
    assert_eq!(behavior["controllerId"], "s.note.note@1/*#editor");
    assert_eq!(behavior["terminalPolicy"], "cancelled-action-discards-draft");

    SCENE_INTENTS.with(|cell| *cell.borrow_mut() = SceneIntentQueue::default());
    SCENE_POINTER_OWNERS.with(|cell| *cell.borrow_mut() = ScenePointerOwners::default());
    let law = ink_editing_law();
    let window_id = "note-ink-app-backed-cancel";
    let ink = ink_intent_scene(&law, behavior["utility"].as_str().unwrap(), "note-demo");
    let node = publish_ink_intent_document(window_id, 1, 453, &[4], &ink);
    let surface_id = retained_scene_surface_id(window_id, node);
    let host_id = retained_scene_host_id(window_id, node);
    let rect = Rect::new(0.0, 0.0, 320.0, 240.0);
    let pointer = ui_render::PointerId(71);
    let point = |name: &str| (behavior["points"][name]["x"].as_f64().unwrap() as f32, behavior["points"][name]["y"].as_f64().unwrap() as f32);
    let down_point = point("down");
    let move_point = point("move");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let command = |event| ui_wgpu::wgpu::UiCommand::Scene { window_id: window_id.into(), node, surface_id: surface_id.clone(), kind: ui_wgpu::wgpu::SurfaceKind::InkCanvas, rect, event };

    apply_ui_commands(&[command(ui_wgpu::wgpu::UiEvent::PointerDown { x: down_point.0, y: down_point.1, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() })], Some(pointer), &mut input);
    drive_ink_scene_terminal(&mut input);
    let begin = crate::collect_fixture_actions(&mut input);
    assert_eq!(begin.len(), 1);
    assert_eq!(begin[0].action, "inkApplyEvents");
    assert_eq!(begin[0].args.as_ref().and_then(|args| args.get("phase")).and_then(semio_framework::DslValue::as_str), behavior["acceptedBeforeCancel"][0]["phase"].as_str());

    apply_ui_commands(&[command(ui_wgpu::wgpu::UiEvent::PointerMove { x: move_point.0, y: move_point.1, modifiers: Default::default() })], Some(pointer), &mut input);
    drive_ink_scene_terminal(&mut input);
    let live = crate::collect_fixture_actions(&mut input);
    assert_eq!(live.len(), 1);
    assert_eq!(live[0].args.as_ref().and_then(|args| args.get("phase")).and_then(semio_framework::DslValue::as_str), behavior["acceptedBeforeCancel"][1]["phase"].as_str());

    let cancelled = cancel_scene_pointer(pointer, &mut input);
    assert_eq!(cancelled.len(), 1);
    assert_eq!(cancelled[0].host_id, host_id);
    let aborted = crate::collect_fixture_actions(&mut input);
    assert_eq!(aborted.len(), 1, "the streamed gesture publishes exactly its abort: {aborted:?}");
    assert_eq!(aborted[0].action, behavior["publishedOnCancel"][0]["action"].as_str().unwrap());
    assert_eq!(aborted[0].args.as_ref().and_then(|args| args.get("phase")).and_then(semio_framework::DslValue::as_str), behavior["publishedOnCancel"][0]["phase"].as_str());
    assert_eq!(aborted[0].args.as_ref().and_then(|args| args.get("reason")).and_then(semio_framework::DslValue::as_str), Some("captureLost"));
    assert!(captured_scene_pointer(pointer).is_none(), "the old physical pointer cannot route a later move or up");
    assert!(crate::scenes::ink_pointer_state_is_clear(&host_id), "the retained preview is cleared and the plugin drops the gesture with zero trace");
    assert_eq!(behavior["blockedAfterCancel"], serde_json::json!(["inkApplyEvents:stream", "inkApplyEvents:commit"]));

    let successor = ui_render::PointerId(72);
    apply_ui_commands(&[command(ui_wgpu::wgpu::UiEvent::PointerDown { x: down_point.0 + 24.0, y: down_point.1 + 24.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() })], Some(successor), &mut input);
    drive_ink_scene_terminal(&mut input);
    let fresh = crate::collect_fixture_actions(&mut input);
    assert_eq!(fresh.len(), 1);
    assert_eq!(fresh[0].args.as_ref().and_then(|args| args.get("phase")).and_then(semio_framework::DslValue::as_str), Some("stream"));
    cancel_scene_pointer(successor, &mut input);
    retire_presented_document(window_id);
}

fn open_ink_editor(window_id: &str, node: NodeId, surface_id: &str, rect: Rect, point: &Value, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    let x = point["x"].as_f64().expect("gesture x") as f32;
    let y = point["y"].as_f64().expect("gesture y") as f32;
    for _ in 0..2 {
        dispatch_ink_pointer(window_id, node, surface_id, rect, x, y, true, input);
        dispatch_ink_pointer(window_id, node, surface_id, rect, x, y, false, input);
    }
}

fn assert_atomic_ink_update(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, expected: &Value) {
    let queued = crate::collect_fixture_actions(input);
    let updates: Vec<&ActionDescriptor> = queued.iter().filter(|action| action.action == "inkApplyEvents").collect();
    assert_eq!(updates.len(), 1, "one commit publishes exactly one inkApplyEvents action, got {queued:?}");
    let args = updates[0].args.as_ref().expect("inkApplyEvents args");
    assert_eq!(args.get("phase"), None, "an inline edit commits as a one-shot");
    let events: Value = serde_json::from_str(args.get("eventsJson").and_then(semio_framework::DslValue::as_str).expect("eventsJson string")).expect("eventsJson parses");
    assert_eq!(events, Value::Array(vec![expected.clone()]), "the commit is one exact updateBlock event");
}

/// 🖋️ The shared law enters the native renderer through actual retained scene commands: double-click
/// focus, blur/Enter atomic commits, table advance, Escape cancellation, and stale-generation
/// retirement all run through the same interpreter functions the window renderer calls.
#[test]
fn ink_canvas_text_and_table_editing_matches_the_react_host_lifecycle() {
    let law = ink_editing_law();
    let window_id = "ink-canvas-editing-law";
    let authored_surface_id = law["scene"]["surfaceId"].as_str().expect("surface id");
    let ink = ink_intent_scene(&law, law["scene"]["inkCanvas"]["activeUtility"].as_str().unwrap(), law["document"]["id"].as_str().unwrap());
    let node = publish_ink_intent_document(window_id, 1, 452, &[4], &ink);
    let surface_id = retained_scene_surface_id(window_id, node);
    let host_id = retained_scene_host_id(window_id, node);
    assert_eq!(surface_id, window_id);
    assert_ne!(host_id, authored_surface_id);
    let viewport = &law["viewport"];
    let rect = Rect::new(viewport["x"].as_f64().unwrap() as f32, viewport["y"].as_f64().unwrap() as f32, viewport["width"].as_f64().unwrap() as f32, viewport["height"].as_f64().unwrap() as f32);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    open_ink_editor(window_id, node, &surface_id, rect, &law["gestures"]["text"], &mut input);
    assert_eq!(input.focused_id.as_deref(), Some(format!("{host_id}.ink.text.text-a.input").as_str()));
    crate::collect_fixture_actions(&mut input);
    assert!(apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Char(law["gestures"]["text"]["replacement"].as_str().unwrap().into()), &Default::default(), &mut input));
    assert!(blur_focused_ink_editor_at(law["gestures"]["blur"]["x"].as_f64().unwrap() as f32, law["gestures"]["blur"]["y"].as_f64().unwrap() as f32, &mut input));
    assert_atomic_ink_update(&mut input, &law["expected"]["textEvent"]);

    open_ink_editor(window_id, node, &surface_id, rect, &law["gestures"]["table"], &mut input);
    crate::collect_fixture_actions(&mut input);
    assert!(apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Char(law["gestures"]["table"]["replacement"].as_str().unwrap().into()), &Default::default(), &mut input));
    assert!(apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Enter, &Default::default(), &mut input));
    assert_atomic_ink_update(&mut input, &law["expected"]["tableEvent"]);
    let advanced_suffix = law["expected"]["advancedControlId"].as_str().unwrap().strip_prefix(authored_surface_id).expect("neutral control id begins with the authored surface");
    assert_eq!(input.focused_id.as_deref(), Some(format!("{host_id}{advanced_suffix}").as_str()));
    assert!(apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Char("z".into()), &Default::default(), &mut input));
    assert!(apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Tab, &Default::default(), &mut input));
    assert_atomic_ink_update(&mut input, &law["expected"]["tableTabEvent"]);
    assert!(input.focused_id.is_none(), "Tab from the final table cell commits and retires the editor");

    open_ink_editor(window_id, node, &surface_id, rect, &law["gestures"]["text"], &mut input);
    crate::collect_fixture_actions(&mut input);
    assert!(apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Char("cancelled".into()), &Default::default(), &mut input));
    assert!(apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Escape, &Default::default(), &mut input));
    assert!(crate::collect_fixture_actions(&mut input).iter().all(|action| action.action != "inkApplyEvents"), "Escape cancels without publishing an update");

    open_ink_editor(window_id, node, &surface_id, rect, &law["gestures"]["text"], &mut input);
    crate::collect_fixture_actions(&mut input);
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).expect("window generation");
    assert!(request_ui_document_close(window_id));
    assert!(!apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Char("closing".into()), &Default::default(), &mut input), "a closing document immediately fences and clears its focused editor");
    assert!(input.focused_id.is_none());
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
    drain_presented_document_close(window_id);
    assert!(publish_new_focus_rebase_document(window_id, 2, 453, ui_wgpu::wgpu::SurfaceKind::TextEditor, &[4]).is_some());
    assert!(UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).is_some_and(|next| next > generation));
    assert!(!apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Char("stale".into()), &Default::default(), &mut input), "a prior window generation cannot mutate the reopened document");
    assert!(input.focused_id.is_none());
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
    retire_presented_document(window_id);
}

#[test]
fn stale_scene_revision_retires_without_mutation_or_action_publication() {
    while !close_scene_interaction_step() {}
    let window_id = "apply-ui-commands-stale-scene-revision";
    let node = seed_scene_window(window_id, "original", ui_wgpu::wgpu::SurfaceKind::Canvas2d);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_scene_ui_command(
        window_id,
        node,
        ui_wgpu::wgpu::SurfaceKind::Canvas2d,
        rect,
        &ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        Some(ui_render::PointerId(1)),
        &mut input,
    );
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("root", None, vec![component_scene_ui("replacement", ui_wgpu::wgpu::SurfaceKind::Canvas2d)])));

    assert!(drive_scene_interaction_step(&mut input));
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
    assert!(scene_interaction_terminal_is_empty());
}

#[test]
fn closing_window_fences_queued_scene_intent_before_tree_retirement() {
    let window_id = "closing-window-queued-canvas";
    let node = seed_scene_window(window_id, "closing-canvas", ui_wgpu::wgpu::SurfaceKind::Canvas2d);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_scene_ui_command(
        window_id,
        node,
        ui_wgpu::wgpu::SurfaceKind::Canvas2d,
        rect,
        &ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        Some(ui_render::PointerId(1)),
        &mut input,
    );
    assert!(!scene_interaction_terminal_is_empty());
    assert!(request_ui_document_close(window_id));
    assert!(UI_ENGINE.with(|cell| cell.borrow().tree(window_id).is_some()));
    assert!(drive_scene_interaction_step(&mut input));
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "queued input cannot activate the old scene after its window closes");
    assert!(scene_interaction_terminal_is_empty());
}

#[test]
fn closing_ink_editor_rejects_a_commit_before_its_scene_retires() {
    let law = ink_editing_law();
    let window_id = "closing-focused-ink-editor";
    let surface_id = law["scene"]["surfaceId"].as_str().unwrap();
    let node = seed_scene_window_with(window_id, ink_editing_scene(&law));
    let viewport = &law["viewport"];
    let rect = Rect::new(viewport["x"].as_f64().unwrap() as f32, viewport["y"].as_f64().unwrap() as f32, viewport["width"].as_f64().unwrap() as f32, viewport["height"].as_f64().unwrap() as f32);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    open_ink_editor(window_id, node, surface_id, rect, &law["gestures"]["text"], &mut input);
    crate::collect_fixture_actions(&mut input);
    assert!(apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Char("uncommitted".into()), &Default::default(), &mut input));
    assert!(request_ui_document_close(window_id));
    assert!(!apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Enter, &Default::default(), &mut input));
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn closing_window_retires_an_unfinished_ink_clipboard_stream_before_reopening() {
    let law = ink_editing_law();
    let window_id = "closing-ink-clipboard-stream";
    let surface_id = law["scene"]["surfaceId"].as_str().unwrap();
    let node = seed_scene_window_with(window_id, ink_editing_scene(&law));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let host_id = retained_scene_host_id(window_id, node);
    assert_ne!(host_id, surface_id, "the fixture exercises the retained host identity rather than the wire surface id");
    focus_ink_surface(window_id, generation, node, &host_id, Rect::new(0.0, 0.0, 200.0, 200.0));
    assert_eq!(start_focused_ink_clipboard_stream(41, false, 100), Ok(true));
    assert_eq!(push_focused_ink_clipboard_stream(41, "unfinished"), Ok(true));
    assert!(request_ui_document_close(window_id));
    assert!(!with_live_ink_surface(&focused_ink_surface().unwrap(), |_| ()).is_some(), "closing immediately revokes the clipboard address");
    for _ in 0..262_144 {
        if !ui_document_close_pending() {
            break;
        }
        assert!(close_ui_document_one());
    }
    assert!(!ui_document_close_pending());
    assert!(INK_CLIPBOARD_STREAMS.with(|cell| cell.borrow().iter().flatten().all(|stream| stream.address.window_id != window_id)));
    assert!(focused_ink_surface().is_none());
    seed_scene_window_with(window_id, ink_editing_scene(&law));
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(!commit_focused_ink_clipboard_stream(41, &mut input));
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn closing_one_window_retires_only_its_ink_clipboard_owner() {
    let law = ink_editing_law();
    let first_window = "closing-one-ink-owner-a";
    let second_window = "closing-one-ink-owner-b";
    let first_node = seed_scene_window_with(first_window, ink_editing_scene(&law));
    let second_node = seed_scene_window_with(second_window, ink_editing_scene(&law));
    let first_generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(first_window)).unwrap();
    let second_generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(second_window)).unwrap();
    focus_ink_surface(first_window, first_generation, first_node, &retained_scene_host_id(first_window, first_node), Rect::new(0.0, 0.0, 200.0, 200.0));
    assert_eq!(start_focused_ink_clipboard_stream(142, false, 12), Ok(true));
    focus_ink_surface(second_window, second_generation, second_node, &retained_scene_host_id(second_window, second_node), Rect::new(0.0, 0.0, 200.0, 200.0));
    assert_eq!(start_focused_ink_clipboard_stream(143, false, 12), Ok(true));

    assert!(request_ui_document_close(first_window));
    for _ in 0..262_144 {
        if !ui_document_close_pending() {
            break;
        }
        assert!(close_ui_document_one());
    }
    assert!(!ui_document_close_pending());
    INK_CLIPBOARD_STREAMS.with(|cell| {
        let slots = cell.borrow();
        assert!(slots.iter().flatten().all(|stream| stream.id != 142));
        assert!(slots.iter().flatten().any(|stream| stream.id == 143), "closing one document cannot consume a sibling host's stream");
    });
    assert!(abort_focused_ink_clipboard_stream(143));
}

#[test]
fn a_late_native_clipboard_callback_cannot_complete_a_reused_slot() {
    let mut arena = ui_wgpu::wgpu::arena::Arena::<()>::new();
    let node = arena.insert(());
    let address = |host_id: &str| InkClipboardAddress { window_id: "native-ink-clipboard-aba".into(), window_generation: 1, node, host_id: host_id.into(), rect: Rect::new(0.0, 0.0, 100.0, 100.0) };
    let stale = reserve_pending_native_ink_clipboard(address("old")).expect("old slot reserves");
    PENDING_NATIVE_INK_CLIPBOARD.with(|cell| cell.borrow_mut()[usize::from(stale.slot)].mounted = None);
    let successor_token = reserve_pending_native_ink_clipboard(address("successor")).expect("successor reuses the free slot");
    assert_eq!(stale.slot, successor_token.slot);
    assert_ne!(stale.generation, successor_token.generation);
    complete_pending_native_ink_clipboard(stale, Ok(Some(ui_wgpu::wgpu::ClipboardContent::Text("stale".into()))));
    PENDING_NATIVE_INK_CLIPBOARD.with(|cell| {
        let mut slots = cell.borrow_mut();
        let successor = slots[usize::from(successor_token.slot)].mounted.take().expect("successor slot remains mounted");
        assert_eq!(successor.address.host_id, "successor");
        assert!(successor.outcome.is_none(), "the old completion token cannot write through a reused slot index");
    });
}

#[test]
fn stale_window_close_cannot_clear_a_successor_text_editor_focus() {
    let window = "text-focus-reopen";
    let node = seed_scene_window(window, "editor", ui_wgpu::wgpu::SurfaceKind::TextEditor);
    let target = retained_scene_target(window, node).expect("mounted editor target");
    let focus = install_fixture_text_editor_focus(window, target.window_generation, node, &target.host_id);

    assert!(!close_window_focus_clipboard_one(window, target.window_generation.saturating_sub(1)), "a stale window lifetime cannot consume its successor's exact text focus");
    assert!(FOCUSED_TEXT_EDITOR.with(|cell| cell.borrow().as_ref().is_some_and(|mounted| {
        mounted.window_id == focus.window_id && mounted.window_generation == focus.window_generation && mounted.node == focus.node && mounted.host_id == focus.host_id && mounted.surface == focus.surface && mounted.document_id == focus.document_id
    })));
    FOCUSED_TEXT_EDITOR.with(|cell| *cell.borrow_mut() = None);
}

#[test]
fn text_editor_focus_transfers_only_on_primary_presses_outside_its_mounted_target() {
    let window = "text-editor-primary-focus";
    let node = seed_scene_window(window, "editor", ui_wgpu::wgpu::SurfaceKind::TextEditor);
    let target = retained_scene_target(window, node).expect("mounted editor target");
    install_fixture_text_editor_focus(window, target.window_generation, node, &target.host_id);
    for (down, button) in [(false, 0), (true, 1), (true, 2)] {
        assert!(!blur_focused_text_editor_for_pointer(None, down, button));
        assert!(FOCUSED_TEXT_EDITOR.with(|cell| cell.borrow().is_some()));
    }
    assert!(!blur_focused_text_editor_for_pointer(Some(&target), true, 0));
    assert!(blur_focused_text_editor_for_pointer(None, true, 0));
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(!apply_focused_text_editor_key(&ui_wgpu::wgpu::KeyAction::Char("x".into()), &Default::default(), &mut input));
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
    install_fixture_text_editor_focus(window, target.window_generation, node, &target.host_id);
    let mut successor = target.clone();
    successor.window_generation += 1;
    assert!(blur_focused_text_editor_for_pointer(Some(&successor), true, 0));
    assert!(FOCUSED_TEXT_EDITOR.with(|cell| cell.borrow().is_none()));
}

#[test]
fn accepted_node_graph_note_caret_arms_resets_and_retires_with_its_exact_scene() {
    let _serialized = crate::engine_canvas::engine_surface_law_guard();
    let window = "node-graph-note-caret";
    let mut scene = component_scene_ui(window, ui_wgpu::wgpu::SurfaceKind::NodeGraph);
    let UiNode::ComponentScene(scene) = &mut scene else { unreachable!() };
    scene.node_graph = Some(ui_wgpu::wgpu::NodeGraphScene {
        editable: Some(true),
        host_snapshot_json: Some(
            serde_json::json!({
                "schema": "flow.hostSnapshot",
                "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
                "widgets": [{ "kind": "inputNote", "id": "note", "text": "hello" }],
                "synapses": [],
                "layout": { "note": { "x": 0.0, "y": 0.0 } }
            })
            .to_string(),
        ),
        capabilities_json: Some(serde_json::json!({ "engine": "flow" }).to_string()),
        ..ui_wgpu::wgpu::NodeGraphScene::base(Vec::new(), Vec::new(), semio_framework_os_kernel::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 })
    });
    let node = seed_scene_window_with(window, UiNode::ComponentScene(scene.clone()));
    present_seeded_scene_window(window);
    let target = retained_scene_target(window, node).expect("accepted NodeGraph target");
    let retained_scene = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let retained = engine.tree(window).and_then(|tree| tree.node(node)).expect("accepted NodeGraph remains mounted");
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { unreachable!() };
        scene.clone()
    });
    assert!(crate::engine_canvas::sync_engine_scene(&retained_scene, window, Rect::new(0.0, 0.0, 640.0, 480.0), &Theme::default()));
    assert!(crate::engine_canvas::component_scene_begin_note_edit_fixture(&retained_scene, "note"));
    assert!(refresh_node_graph_caret_after_pointer(&target));
    assert!(FOCUSED_NODE_GRAPH_CARET.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.target.same_component_host(&target))));
    assert!(UI_ENGINE.with(|cell| cell.borrow().window_next_clock_deadline(window)).and_then(|(_, deadline)| deadline).is_some(), "accepted note editing owns one cadence deadline");

    assert!(apply_focused_node_graph_note_key(&ui_wgpu::wgpu::KeyAction::Char("!".into()), &Default::default()), "an edit reaches only the focused accepted Flow host and resets its phase");
    assert!(crate::engine_canvas::component_scene_has_editable_caret(&retained_scene));
    assert!(apply_focused_node_graph_note_key(&ui_wgpu::wgpu::KeyAction::Enter, &Default::default()), "commit is consumed by the focused note");
    assert!(FOCUSED_NODE_GRAPH_CARET.with(|cell| cell.borrow().is_none()));
    assert!(!crate::engine_canvas::component_scene_has_editable_caret(&retained_scene));
    assert_eq!(UI_ENGINE.with(|cell| cell.borrow().window_next_clock_deadline(window)).and_then(|(_, deadline)| deadline), None, "committing the note cancels the cadence wake");

    crate::engine_canvas::retire_engine_surface_fixture(&target.host_id);
}

#[test]
fn text_editor_space_reaches_the_focused_editor_before_the_runtime_pan_modifier() {
    use std::future::Future;
    let _serialized = crate::engine_canvas::engine_surface_law_guard();
    let law: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/✍️editor/🧫️fixtures/⌨️text-input/🔣️.json"))).unwrap();
    let law = law["rendererKeys"].as_array().unwrap().iter().find(|law| law["id"] == "space-is-a-printable-key").unwrap();
    let window = "text-editor-runtime-space";
    let UiNode::ComponentScene(mut scene) = component_scene_ui(window, ui_wgpu::wgpu::SurfaceKind::TextEditor) else { unreachable!() };
    scene.text_editor = Some(ui_wgpu::wgpu::TextEditorScene::base(law["text"].as_str().unwrap().into(), None, Some(serde_json::json!({ "start": law["selection"][0], "end": law["selection"][1] }).to_string())));
    let node = seed_scene_window_with(window, UiNode::ComponentScene(scene));
    let target = retained_scene_target(window, node).unwrap();
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let UiNode::ComponentScene(scene) = &engine.tree(window).unwrap().node(node).unwrap().spec.0 else { unreachable!() };
        assert!(crate::engine_canvas::sync_engine_scene(scene, window, Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
    });
    install_fixture_text_editor_focus(window, target.window_generation, node, &target.host_id);
    let mut runtime = crate::AppInteractionState {
        shell: crate::shell::ShellState::new(Vec::new(), "text-editor-runtime-space".into(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native),
        input: Default::default(),
        theme: Default::default(),
        theme_dark: false,
        last_pointer_x: 0.0,
        last_pointer_y: 0.0,
        pointer_down: false,
        pointer_button: 0,
        pointer_capture: Default::default(),
        modifiers: Default::default(),
        space_pressed: false,
        wheel_zoom_deadline_ms: 0.0,
        text_streams: std::array::from_fn(|_| None),
        text_fault: None,
        frame_fault: None,
        text_cancel_pending: false,
        last_sync_pump_ms: 0.0,
    };
    {
        let mut key = Box::pin(runtime.handle_key(ui_wgpu::wgpu::KeyAction::Space(true), Default::default()));
        assert!(key.as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())).is_ready());
    }
    assert!(!runtime.space_pressed);
    let actions = collect_text_editor_actions_accepted(&mut runtime.input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"]);
    assert_eq!(serde_json::to_value(actions[0].args.as_ref().unwrap()).unwrap()["text"], law["expect"]["text"]);
    assert_eq!(serde_json::to_value(actions[1].args.as_ref().unwrap()).unwrap(), serde_json::json!({ "surfaceId": target.surface_id, "start": law["expect"]["selection"][0], "end": law["expect"]["selection"][1] }));
    assert!(blur_focused_text_editor_for_pointer(None, true, 0));
    for pressed in [true, false] {
        {
            let mut key = Box::pin(runtime.handle_key(ui_wgpu::wgpu::KeyAction::Space(pressed), Default::default()));
            assert!(key.as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())).is_ready());
        }
        assert_eq!(runtime.space_pressed, pressed);
        assert!(crate::collect_fixture_actions(&mut runtime.input).is_empty());
    }
    crate::engine_canvas::retire_engine_surface_fixture(&target.host_id);
    assert!(request_ui_document_close(window));
    for _ in 0..262_144 {
        if !ui_document_close_pending() {
            break;
        }
        assert!(close_ui_document_one());
    }
    assert!(!ui_document_close_pending());
}

#[test]
fn focused_text_editor_clipboard_composition_and_accessibility_share_the_accepted_host() {
    let _serialized = crate::engine_canvas::engine_surface_law_guard();
    let law: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/✍️editor/🧫️fixtures/⌨️text-input/🔣️.json"))).unwrap();
    let paste = law["sequences"].as_array().unwrap().iter().find(|law| law["id"] == "a-paste-replaces-the-selection").unwrap();
    let window = "text-editor-clipboard-ime-accessibility";
    let UiNode::ComponentScene(mut scene) = component_scene_ui(window, ui_wgpu::wgpu::SurfaceKind::TextEditor) else { unreachable!() };
    scene.text_editor = Some(ui_wgpu::wgpu::TextEditorScene::base(paste["text"].as_str().unwrap().into(), None, Some(serde_json::json!({ "start": paste["selection"][0], "end": paste["selection"][1] }).to_string())));
    let node = seed_scene_window_with(window, UiNode::ComponentScene(scene));
    let target = retained_scene_target(window, node).unwrap();
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let UiNode::ComponentScene(scene) = &engine.tree(window).unwrap().node(node).unwrap().spec.0 else { unreachable!() };
        assert!(crate::engine_canvas::sync_engine_scene(scene, window, Rect::new(10.0, 20.0, 480.0, 320.0), &Theme::default()));
    });
    install_fixture_text_editor_focus(window, target.window_generation, node, &target.host_id);
    MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow_mut().clear());
    let chord = ui_wgpu::wgpu::PointerModifiers { ctrl: true, ..Default::default() };
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    assert!(apply_focused_text_editor_key(&ui_wgpu::wgpu::KeyAction::Char("c".into()), &chord, &mut input));
    assert_eq!(MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow().clone()), ["b"]);
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "copy publishes no guest action");

    let pasted = paste["steps"][0]["paste"].as_str().unwrap();
    assert!(apply_focused_text_editor_text(pasted, &mut input));
    let actions = collect_text_editor_actions_accepted(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"]);
    assert_eq!(serde_json::to_value(actions[0].args.as_ref().unwrap()).unwrap()["text"], paste["expect"]["text"]);
    assert_eq!(serde_json::to_value(actions[1].args.as_ref().unwrap()).unwrap()["start"], paste["expect"]["selection"][0]);

    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window);
    publish_accessibility_visible_documents();
    let textbox = published_accessibility_nodes_for_test(window).into_iter().find(|node| node.role == "textbox").expect("accepted TextEditor textbox");
    assert_eq!(textbox.label.as_deref(), Some("Editor"));
    assert_eq!(textbox.value_text.as_deref(), paste["expect"]["text"].as_str());
    assert!(textbox.focusable && textbox.actionable && textbox.focused && textbox.editable && textbox.multiline && textbox.rect.is_some());
    assert!(dispatch_accessibility_event(window, target.window_generation, textbox.node_id, &textbox.key, ui_wgpu::wgpu::AccessibilityUiEvent::Value("日本".into()), &mut input).is_some());
    let actions = collect_text_editor_actions_accepted(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"]);
    assert_eq!(serde_json::to_value(actions[0].args.as_ref().unwrap()).unwrap()["text"], "日本");

    assert!(apply_focused_text_editor_key(&ui_wgpu::wgpu::KeyAction::Char("a".into()), &chord, &mut input));
    let actions = collect_text_editor_actions_accepted(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"]);
    assert_eq!(actions[0].controller_id, "ctrl");
    let commit = serde_json::to_value(actions[0].args.as_ref().unwrap()).unwrap();
    assert_eq!(commit, serde_json::json!({ "surfaceId": target.surface_id, "typing": target.surface_id, "typingCommit": "selectionJump" }));
    assert!(commit.get("text").is_none());
    assert_eq!(actions[1].controller_id, "ctrl");
    assert_eq!(serde_json::to_value(actions[1].args.as_ref().unwrap()).unwrap(), serde_json::json!({ "surfaceId": target.surface_id, "start": 0, "end": 6 }));
    assert!(apply_focused_text_editor_key(&ui_wgpu::wgpu::KeyAction::Char("x".into()), &chord, &mut input));
    assert_eq!(MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow().clone()), ["b", "日本"]);
    let actions = collect_text_editor_actions_accepted(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"]);
    assert_eq!(serde_json::to_value(actions[0].args.as_ref().unwrap()).unwrap()["text"], "");

    MOCK_CLIPBOARD_READ.with(|cell| *cell.borrow_mut() = Some("clipboard".into()));
    assert!(apply_focused_text_editor_key(&ui_wgpu::wgpu::KeyAction::Char("v".into()), &chord, &mut input));
    let actions = collect_text_editor_actions_accepted(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"]);
    assert_eq!(serde_json::to_value(actions[0].args.as_ref().unwrap()).unwrap()["text"], "clipboard");

    assert!(start_focused_text_editor_stream(701, 8).unwrap());
    assert!(push_focused_text_editor_stream(701, "retired").unwrap());
    FOCUSED_TEXT_EDITOR.with(|cell| *cell.borrow_mut() = None);
    assert!(!commit_focused_text_editor_stream(701, &mut input), "a delayed stream cannot edit after focus retires");
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
    eprintln!("[DEBUG] Native text editor accessibility, composition and clipboard used canonical UTF-8 byte selection offsets");
}

fn table_stepper_scene_node(value: f64) -> UiNode {
    let mut node = component_scene_ui("table-stepper-focus", ui_wgpu::wgpu::SurfaceKind::Table);
    let UiNode::ComponentScene(scene) = &mut node else { unreachable!() };
    scene.controller_id = "sourcing-curation".into();
    scene.table = Some(ui_wgpu::wgpu::TableScene::base(
        serde_json::json!([{ "id": "curated", "label": "Curated" }]).to_string(),
        serde_json::json!([{ "id": "beam-glulam-gl24h", "curated": { "kind": "stepper", "value": value, "min": 0.0, "max": 5.0, "step": 0.5, "action": { "controllerId": "sourcing-curation", "action": "curationSetCount", "args": { "objectId": "beam-glulam-gl24h" } } } }]).to_string(),
    ));
    node
}

fn table_editable_text_scene_node_with_value(value: &str) -> UiNode {
    let mut node = component_scene_ui("table-editable-text-focus", ui_wgpu::wgpu::SurfaceKind::Table);
    let UiNode::ComponentScene(scene) = &mut node else { unreachable!() };
    scene.controller_id = "csv-editor".into();
    scene.table = Some(ui_wgpu::wgpu::TableScene::base(
        serde_json::json!([{ "id": "value", "label": "Value" }]).to_string(),
        serde_json::json!([{ "id": "row-2", "value": { "kind": "editableText", "value": value, "action": { "controllerId": "csv-editor", "action": "set-cell", "args": { "row": 2, "column": 1 } } } }]).to_string(),
    ));
    node
}

/// 📤️ Every action an input published: the inline queue AND the retained (paged) publication an editable value takes.
fn collect_published_actions(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = crate::collect_fixture_actions(input);
    for _ in 0..4096 {
        if !input.retained_action_pending() {
            break;
        }
        if let Some(action) = input.drive_retained_action_step().expect("retained publication page") {
            actions.push(action.descriptor);
        }
    }
    actions
}

fn table_editable_text_scene_node() -> UiNode {
    table_editable_text_scene_node_with_value("Grüße\n世界")
}

#[test]
fn accepted_table_editable_text_commits_enter_and_discards_escape() {
    let window_id = "accepted-table-editable-text-focus";
    let node = seed_scene_window_with(window_id, table_editable_text_scene_node());
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let theme = ui_wgpu::wgpu::Theme::default();
    let y = theme.control_height * 1.33 + theme.control_height * 0.5;
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let focus = |input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>| {
        apply_scene_ui_command(
            window_id,
            node,
            ui_wgpu::wgpu::SurfaceKind::Table,
            rect,
            &ui_wgpu::wgpu::UiEvent::PointerDown { x: 100.0, y, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
            Some(ui_render::PointerId(18)),
            input,
        );
        assert!(drive_scene_interaction_step(input));
    };
    focus(&mut input);
    assert!(FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.window_id == window_id && focus.row_id == "row-2" && focus.column_id == "value")));
    while input.drive_text_step().expect("the focused cell's own value projects before the edit") {}
    let control_id = input.focused_id.clone().expect("native cell input focus");
    input.focus_input_owned(control_id, "mehrzeilig\nΩ🙂".into());
    while input.drive_text_step().expect("text projection") {}
    sync_focused_table_editable_text_draft(&input);
    assert_eq!(focused_table_editable_text_draft(window_id, node, &retained_scene_host_id(window_id, node), "row-2", "value").as_deref(), Some("mehrzeilig\nΩ🙂"));
    assert!(apply_focused_table_editable_text_key(&ui_wgpu::wgpu::KeyAction::Enter, &Default::default(), &mut input));
    assert!(input.retained_action_pending());
    let action = loop {
        if let Some(action) = input.drive_retained_action_step().expect("retained cell page") {
            break action.descriptor;
        }
    };
    assert_eq!(
        serde_json::to_value(&action).unwrap(),
        serde_json::json!({
            "controllerId": "csv-editor",
            "action": "set-cell",
            "args": { "row": 2, "column": 1, "value": "mehrzeilig\nΩ🙂" }
        })
    );
    clear_focused_table_editable_text(&mut input);
    assert!(input.focused_id.is_none());

    focus(&mut input);
    while input.drive_text_step().expect("the refocused cell's own value projects before the edit") {}
    let control_id = input.focused_id.clone().expect("refocused cell input");
    input.focus_input_owned(control_id, "discard me".into());
    while input.drive_text_step().expect("text projection") {}
    assert!(apply_focused_table_editable_text_key(&ui_wgpu::wgpu::KeyAction::Escape, &Default::default(), &mut input));
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
    assert!(input.focused_id.is_none());
}

#[test]
fn accepted_accessibility_table_value_echo_blurs_while_refusal_preserves_the_draft() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧱️elements/📊️Table/🧫️fixtures/✏️editable-text/🔣️.json"))).expect("editable table fixture");
    let acceptance = &fixture["acceptance"];
    let base = acceptance["base"].as_str().unwrap();
    let draft = acceptance["draft"].as_str().unwrap();
    let awaiting_echo = acceptance["awaitingEcho"].as_str().unwrap();
    let target = || crate::scenes::TableEditableTextFocusTarget { row_id: "row-2".into(), column_id: "value".into(), value: base.into() };

    let pending_window = "pending-accessibility-table-value";
    let pending_node = seed_scene_window_with(pending_window, table_editable_text_scene_node_with_value(base));
    accept_table_editable_text_cells(pending_window, pending_node, 931);
    let pending_host = retained_scene_host_id(pending_window, pending_node);
    let pending_generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(pending_window)).unwrap();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(pending_window);
    publish_accessibility_visible_documents();
    let textbox = published_accessibility_nodes_for_test(pending_window).into_iter().find(|node| node.role == "textbox").expect("editable table textbox");
    assert!(dispatch_accessibility_event(pending_window, pending_generation, textbox.node_id, &textbox.key, ui_wgpu::wgpu::AccessibilityUiEvent::Focus, &mut input).is_some());
    assert!(dispatch_accessibility_event(pending_window, pending_generation, textbox.node_id, &textbox.key, ui_wgpu::wgpu::AccessibilityUiEvent::Value(draft.into()), &mut input).is_some());
    let actions = collect_published_actions(&mut input);
    let publication_count_before_echo = acceptance["publicationCountBeforeEcho"].as_u64().unwrap();
    assert_eq!(actions.len() as u64, publication_count_before_echo, "the accepted accessibility value publishes once");
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(pending_window);
    publish_accessibility_visible_documents();
    let pending_textbox = published_accessibility_nodes_for_test(pending_window).into_iter().find(|node| node.key == textbox.key).expect("pending editable table textbox");
    assert_eq!(pending_textbox.busy, acceptance["busyBeforeEcho"].as_bool().unwrap());
    assert_eq!(pending_textbox.value_text.as_deref(), Some(draft));
    assert!(dispatch_accessibility_event(pending_window, pending_generation, textbox.node_id, &textbox.key, ui_wgpu::wgpu::AccessibilityUiEvent::Blur, &mut input).is_some());
    let blur_actions = collect_published_actions(&mut input);
    assert_eq!(publication_count_before_echo + blur_actions.len() as u64, acceptance["publicationCountAfterBlur"].as_u64().unwrap(), "blur before the scene echo cannot republish the accepted accessibility value");
    assert_eq!(focused_table_editable_text_draft(pending_window, pending_node, &pending_host, "row-2", "value").as_deref(), Some(draft));
    assert!(FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.awaiting_echo.as_deref() == Some(awaiting_echo))));

    let echoed_node = seed_scene_window_with(pending_window, table_editable_text_scene_node_with_value(acceptance["acceptedPersisted"].as_str().unwrap()));
    accept_table_editable_text_cells(pending_window, echoed_node, 932);
    let echoed_generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(pending_window)).unwrap();
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(pending_window);
    publish_accessibility_visible_documents();
    let echoed_textbox = published_accessibility_nodes_for_test(pending_window).into_iter().find(|node| node.role == "textbox").expect("echoed editable table textbox");
    assert!(dispatch_accessibility_event(pending_window, echoed_generation, echoed_textbox.node_id, &echoed_textbox.key, ui_wgpu::wgpu::AccessibilityUiEvent::Blur, &mut input).is_some());
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "the authoritative scene echo cannot publish the accepted value again");
    assert!(FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| cell.borrow().is_none()), "the authoritative scene echo rebases the accessibility draft and releases focus");
    assert!(!input.retained_action_pending());

    let refused_window = "refused-accessibility-table-value";
    let refused_node = seed_scene_window_with(refused_window, table_editable_text_scene_node_with_value(acceptance["refusedPersisted"].as_str().unwrap()));
    let refused_host = retained_scene_host_id(refused_window, refused_node);
    let refused_generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(refused_window)).unwrap();
    UI_ENGINE.with(|cell| focus_table_editable_text(&cell.borrow(), refused_window, refused_generation, refused_node, &refused_host, target(), &mut input)).expect("refused cell focus");
    set_focused_table_editable_text_draft(refused_window, refused_node, &refused_host, "row-2", "value", draft);
    assert!(blur_focused_table_editable_text_for_pointer(None, true, 0, &mut input));
    assert!(input.retained_action_pending(), "an unacknowledged persisted value remains a retryable publication");
    assert_eq!(focused_table_editable_text_draft(refused_window, refused_node, &refused_host, "row-2", "value").as_deref(), Some(draft));
    input.cancel_retained_action();
    assert_eq!(focused_table_editable_text_draft(refused_window, refused_node, &refused_host, "row-2", "value").as_deref(), Some(draft), "cancellation preserves the refused draft");
    let other = crate::scenes::TableEditableTextFocusTarget { row_id: "other-row".into(), column_id: "value".into(), value: "other".into() };
    UI_ENGINE.with(|cell| focus_table_editable_text(&cell.borrow(), refused_window, refused_generation, refused_node, &refused_host, other, &mut input)).expect("focus transfer preserves the unacknowledged edit");
    assert!(!input.retained_action_pending(), "focus transfer cannot duplicate a publication awaiting its scene echo");
    assert_eq!(focused_table_editable_text_draft(refused_window, refused_node, &refused_host, "row-2", "value").as_deref(), Some(draft), "an unacknowledged edit keeps the original cell and draft focused");
    input.cancel_retained_action();
    clear_focused_table_editable_text(&mut input);
}

#[test]
fn accepted_table_stepper_centre_owns_navigation_keys_and_a_missing_row_retires_focus() {
    let window_id = "accepted-table-stepper-focus";
    let node = seed_scene_window_with(window_id, table_stepper_scene_node(2.0));
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let theme = ui_wgpu::wgpu::Theme::default();
    let y = theme.control_height * 1.33 + theme.control_height * 0.5;
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_scene_ui_command(
        window_id,
        node,
        ui_wgpu::wgpu::SurfaceKind::Table,
        rect,
        &ui_wgpu::wgpu::UiEvent::PointerDown { x: 100.0, y, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        Some(ui_render::PointerId(17)),
        &mut input,
    );
    assert!(drive_scene_interaction_step(&mut input));
    assert!(FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.window_id == window_id && focus.row_id == "beam-glulam-gl24h" && focus.column_id == "curated")));
    let scene = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let UiNode::ComponentScene(scene) = &engine.tree(window_id).unwrap().node(node).unwrap().spec.0 else { unreachable!() };
        scene.clone()
    });
    crate::scenes::stage_table_stepper_accessibility_cells(&scene.host_id, crate::scenes::table_stepper_accessibility_cells(&scene, rect, ui_wgpu::wgpu::UiDriverDrag::Handle));
    crate::scenes::seal_table_stepper_accessibility_candidates(800);
    crate::scenes::acknowledge_table_stepper_accessibility_candidates(800);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    let spinbutton = published_accessibility_nodes_for_test(window_id).into_iter().find(|node| node.role == "spinbutton").expect("accepted Table stepper spinbutton");
    assert_eq!(spinbutton.label.as_deref(), Some("Curated"));
    assert_eq!((spinbutton.value_min, spinbutton.value_now, spinbutton.value_max, spinbutton.value_text.as_deref()), (Some(0.0), Some(2.0), Some(5.0), Some("2")));
    assert!(spinbutton.focusable && spinbutton.actionable && spinbutton.focused && spinbutton.rect.is_some());
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    assert!(dispatch_accessibility_event(window_id, generation, spinbutton.node_id, &spinbutton.key, ui_wgpu::wgpu::AccessibilityUiEvent::Blur, &mut input).is_some());
    assert!(FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow().is_none()));
    assert!(dispatch_accessibility_event(window_id, generation, spinbutton.node_id, &spinbutton.key, ui_wgpu::wgpu::AccessibilityUiEvent::Focus, &mut input).is_some());

    assert!(apply_focused_table_stepper_key(&ui_wgpu::wgpu::KeyAction::PageUp, &mut input));
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    let args = actions[0].args.as_ref().expect("stepper args");
    assert_eq!(args.get("objectId").and_then(semio_framework::DslValue::as_str), Some("beam-glulam-gl24h"));
    assert_eq!(args.get("delta").and_then(semio_framework::DslValue::as_f64), Some(3.0));
    assert!(!apply_focused_table_stepper_key(&ui_wgpu::wgpu::KeyAction::Enter, &mut input), "an unrelated key remains available to the generic shell route");

    FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow_mut().as_mut().expect("focused Table stepper").row_id = "retired-row".into());
    assert!(!apply_focused_table_stepper_key(&ui_wgpu::wgpu::KeyAction::ArrowUp, &mut input));
    assert!(FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow().is_none()));
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn accepted_vfs_controls_publish_localized_buttons_and_reject_a_retired_row_action() {
    let window_id = "accepted-vfs-controls";
    let authored_host_id = "accepted-vfs-host";
    let mut scene_node = component_scene_ui(authored_host_id, ui_wgpu::wgpu::SurfaceKind::VirtualFileSystem);
    let UiNode::ComponentScene(scene) = &mut scene_node else { unreachable!() };
    scene.controller_id = "vfs-controller".into();
    scene.virtual_file_system = Some(ui_wgpu::wgpu::VirtualFileSystemScene {
        schema_json: "{}".into(),
        rows_json: serde_json::json!([
            { "id": "folder", "name": "Modelle", "hasChildren": true },
            { "id": "successor", "name": "Nachfolger" }
        ])
        .to_string(),
        selected_row_ids_json: None,
        hovered_row_id: None,
        empty_message: None,
        drag_drop_enabled: None,
    });
    let node = seed_scene_window_with(window_id, scene_node);
    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained VFS target")));
    let host_id = retained_scene_host_id(window_id, node);
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let row =
        crate::scenes::VfsAccessibilityControl { key: format!("{host_id}.vfs.folder"), row_id: "folder".into(), label: "Modelle".into(), kind: crate::scenes::VfsAccessibilityControlKind::Row, rect: Rect::new(0.0, 40.0, 320.0, 24.0), expanded: None };
    let chevron = crate::scenes::VfsAccessibilityControl {
        key: format!("{host_id}.vfs.chevron.folder"),
        row_id: "folder".into(),
        label: "Einklappen".into(),
        kind: crate::scenes::VfsAccessibilityControlKind::Chevron,
        rect: Rect::new(8.0, 40.0, 14.0, 24.0),
        expanded: Some(true),
    };
    crate::scenes::stage_vfs_accessibility_controls(&host_id, vec![row, chevron]);
    crate::scenes::seal_vfs_accessibility_candidates(801);
    crate::scenes::acknowledge_vfs_accessibility_candidates(801);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    let nodes = published_accessibility_nodes_for_test(window_id);
    let folder = nodes.iter().find(|node| node.key == format!("{host_id}.vfs.folder")).expect("accepted VFS row button");
    let toggle = nodes.iter().find(|node| node.key == format!("{host_id}.vfs.chevron.folder")).expect("accepted VFS chevron button");
    assert_eq!((folder.role.as_str(), folder.label.as_deref(), folder.focusable, folder.actionable), ("button", Some("Modelle"), true, true));
    assert_eq!((toggle.role.as_str(), toggle.label.as_deref(), toggle.expanded), ("button", Some("Einklappen"), Some(true)));

    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(dispatch_accessibility_event(window_id, generation, toggle.node_id, &toggle.key, ui_wgpu::wgpu::AccessibilityUiEvent::Focus, &mut input).is_some());
    assert!(apply_focused_vfs_control_key(&ui_wgpu::wgpu::KeyAction::Enter, &mut input));
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "chevron keyboard activation owns renderer-local expansion only");
    assert!(dispatch_accessibility_event(window_id, generation, folder.node_id, &folder.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_some());
    assert_eq!(crate::collect_fixture_actions(&mut input).iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["selectRows"]);

    let successor = crate::scenes::VfsAccessibilityControl {
        key: format!("{host_id}.vfs.successor"),
        row_id: "successor".into(),
        label: "Nachfolger".into(),
        kind: crate::scenes::VfsAccessibilityControlKind::Row,
        rect: Rect::new(0.0, 40.0, 320.0, 24.0),
        expanded: None,
    };
    crate::scenes::stage_vfs_accessibility_controls(&host_id, vec![successor]);
    crate::scenes::seal_vfs_accessibility_candidates(802);
    crate::scenes::acknowledge_vfs_accessibility_candidates(802);
    assert!(!apply_focused_vfs_control_key(&ui_wgpu::wgpu::KeyAction::Enter, &mut input));
    assert!(FOCUSED_VFS_CONTROL.with(|cell| cell.borrow().is_none()), "a row missing from the newly accepted semantic frame retires focus");
    assert!(dispatch_accessibility_event(window_id, generation, folder.node_id, &folder.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(), "the stale accepted address cannot dispatch");
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    let successor = published_accessibility_nodes_for_test(window_id).into_iter().find(|node| node.key == format!("{host_id}.vfs.successor")).expect("successor accepted row");
    assert!(dispatch_accessibility_event(window_id, generation, successor.node_id, &successor.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_some());
    assert_eq!(crate::collect_fixture_actions(&mut input).iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["selectRows"]);
}

#[test]
fn accepted_block_list_buttons_publish_localized_actions_and_reject_a_retired_palette_entry() {
    let window_id = "accepted-block-list-controls";
    let authored_host_id = "accepted-block-list-host";
    let mut scene_node = component_scene_ui(authored_host_id, ui_wgpu::wgpu::SurfaceKind::BlockList);
    let UiNode::ComponentScene(scene) = &mut scene_node else { unreachable!() };
    scene.controller_id = "block-list-controller".into();
    scene.block_list = Some(ui_wgpu::wgpu::BlockListScene {
        steps_json: serde_json::json!([{ "id": "basics", "title": "Basics", "blocks": [] }]).to_string(),
        palette_json: serde_json::json!([{ "blockKind": "filter", "label": "Filter", "iconId": "funnel" }]).to_string(),
        selected_id: None,
        dragging_id: None,
        domain_id: None,
    });
    let node = seed_scene_window_with(window_id, scene_node);
    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained BlockList target")));
    let host_id = retained_scene_host_id(window_id, node);
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let add_step = crate::scenes::BlockListAccessibilityControl {
        key: format!("{host_id}.addStep"),
        label: "Schritt hinzufügen".into(),
        rect: Rect::new(180.0, 4.0, 120.0, 24.0),
        action: ActionDescriptor { controller_id: "block-list-controller".into(), action: "addStep".into(), args: crate::action_args_json!({}) },
    };
    let palette = crate::scenes::BlockListAccessibilityControl {
        key: format!("{host_id}.palette.filter"),
        label: "Filter".into(),
        rect: Rect::new(320.0, 4.0, 100.0, 24.0),
        action: ActionDescriptor { controller_id: "block-list-controller".into(), action: "addBlock".into(), args: crate::action_args_json!({ "stepId": "basics", "kind": "filter" }) },
    };
    crate::scenes::stage_block_list_accessibility_controls(&host_id, vec![add_step.clone(), palette.clone()]);
    crate::scenes::seal_block_list_accessibility_candidates(803);
    crate::scenes::acknowledge_block_list_accessibility_candidates(803);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    let nodes = published_accessibility_nodes_for_test(window_id);
    let add_step_node = nodes.iter().find(|entry| entry.key == add_step.key).expect("accepted add-step button");
    let palette_node = nodes.iter().find(|entry| entry.key == palette.key).expect("accepted palette button");
    assert_eq!((add_step_node.role.as_str(), add_step_node.label.as_deref(), add_step_node.tabbable, add_step_node.actionable), ("button", Some("Schritt hinzufügen"), true, true));
    assert_eq!((palette_node.role.as_str(), palette_node.label.as_deref()), ("button", Some("Filter")));

    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(dispatch_accessibility_event(window_id, generation, palette_node.node_id, &palette_node.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_some());
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].action, "addBlock");
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("stepId")).and_then(semio_framework::DslValue::as_str), Some("basics"));
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("kind")).and_then(semio_framework::DslValue::as_str), Some("filter"));

    crate::scenes::stage_block_list_accessibility_controls(&host_id, vec![add_step]);
    crate::scenes::seal_block_list_accessibility_candidates(804);
    crate::scenes::acknowledge_block_list_accessibility_candidates(804);
    assert!(dispatch_accessibility_event(window_id, generation, palette_node.node_id, &palette_node.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none());
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn accepted_event_feed_rows_publish_tabbable_buttons_read_only_text_and_exact_actions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧱️elements/📡️EventFeedHost/🧫️fixtures/♿️accessible-entry/🔣️.json"))).expect("EventFeed accessible entry fixture");
    let actionable = &fixture["cases"][0];
    let passive = &fixture["cases"][1];
    let window_id = "accepted-event-feed-controls";
    let host_id = fixture["hostId"].as_str().unwrap();
    let mut scene_node = component_scene_ui(host_id, ui_wgpu::wgpu::SurfaceKind::EventFeed);
    let UiNode::ComponentScene(scene) = &mut scene_node else { unreachable!() };
    scene.surface_id = fixture["surfaceId"].as_str().unwrap().into();
    scene.controller_id = fixture["controllerId"].as_str().unwrap().into();
    scene.event_feed = Some(ui_wgpu::wgpu::EventFeedScene {
        entries_json: serde_json::json!([actionable["entry"].clone()]).to_string(),
        follow: None,
        activate_action: actionable["activateAction"].as_str().map(str::to_owned),
        domain_id: None,
    });
    let node = seed_scene_window_with(window_id, scene_node);
    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained EventFeed target")));
    let retained_host = retained_scene_host_id(window_id, node);
    let retained_surface = retained_scene(window_id, node).surface_id;
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let expected_action = &actionable["expected"]["action"];
    let control = crate::scenes::EventFeedAccessibilityControl {
        key: rebased_scene_key(actionable["expected"]["key"].as_str().unwrap(), host_id, &retained_host),
        entry_id: actionable["entry"]["id"].as_str().unwrap().into(),
        label: actionable["expected"]["accessibleName"].as_str().unwrap().into(),
        rect: Rect::new(0.0, 12.0, 320.0, 24.0),
        action: Some(ActionDescriptor {
            controller_id: expected_action["controllerId"].as_str().unwrap().into(),
            action: expected_action["action"].as_str().unwrap().into(),
            args: crate::action_args_json!({
                "surfaceId": retained_surface.as_str(),
                "id": expected_action["args"]["id"].as_str().unwrap()
            }),
        }),
    };
    crate::scenes::stage_event_feed_accessibility_controls(&retained_host, vec![control.clone()]);

    let passive_window_id = "accepted-passive-event-feed-controls";
    let passive_host_id = "feed-ax-passive-host";
    let mut passive_scene_node = component_scene_ui(passive_host_id, ui_wgpu::wgpu::SurfaceKind::EventFeed);
    let UiNode::ComponentScene(passive_scene) = &mut passive_scene_node else { unreachable!() };
    passive_scene.surface_id = fixture["surfaceId"].as_str().unwrap().into();
    passive_scene.controller_id = fixture["controllerId"].as_str().unwrap().into();
    passive_scene.event_feed = Some(ui_wgpu::wgpu::EventFeedScene { entries_json: serde_json::json!([passive["entry"].clone()]).to_string(), follow: None, activate_action: None, domain_id: None });
    let passive_scene_node = seed_scene_window_with(passive_window_id, passive_scene_node);
    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(passive_window_id, passive_scene_node).expect("retained passive EventFeed target")));
    let passive_retained_host = retained_scene_host_id(passive_window_id, passive_scene_node);
    let passive_key = format!("{passive_retained_host}.feed.{}", passive["entry"]["id"].as_str().unwrap());
    crate::scenes::stage_event_feed_accessibility_controls(
        &passive_retained_host,
        vec![crate::scenes::EventFeedAccessibilityControl {
            key: passive_key.clone(),
            entry_id: passive["entry"]["id"].as_str().unwrap().into(),
            label: passive["expected"]["accessibleName"].as_str().unwrap().into(),
            rect: Rect::new(0.0, 12.0, 320.0, 24.0),
            action: None,
        }],
    );

    crate::scenes::seal_event_feed_accessibility_candidates(805);
    crate::scenes::acknowledge_event_feed_accessibility_candidates(805);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    note_accessibility_visible_document(passive_window_id);
    publish_accessibility_visible_documents();
    let button = published_accessibility_nodes_for_test(window_id).into_iter().find(|entry| entry.key == control.key).expect("accepted EventFeed button");
    assert_eq!((button.role.as_str(), button.label.as_deref(), button.focusable, button.tabbable, button.actionable), ("button", Some(control.label.as_str()), true, true, true));
    let paragraph = published_accessibility_nodes_for_test(passive_window_id).into_iter().find(|entry| entry.key == passive_key).expect("accepted passive EventFeed text");
    assert_eq!((paragraph.role.as_str(), paragraph.focusable, paragraph.tabbable, paragraph.actionable), ("paragraph", false, false, false));

    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_some());
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].controller_id, expected_action["controllerId"].as_str().unwrap());
    assert_eq!(actions[0].action, expected_action["action"].as_str().unwrap());
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str), Some(retained_surface.as_str()), "the feed's action names the surface its document mounted in");
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("id")).and_then(semio_framework::DslValue::as_str), expected_action["args"]["id"].as_str());

    crate::scenes::stage_event_feed_accessibility_controls(&retained_host, Vec::new());
    crate::scenes::seal_event_feed_accessibility_candidates(806);
    crate::scenes::acknowledge_event_feed_accessibility_candidates(806);
    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none());
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn accepted_table_row_button_publishes_one_tabbable_exact_action_and_rejects_stale_addresses() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧱️elements/📊️Table/🧫️fixtures/🔘️button-accessibility/🔣️.json"))).expect("Table button accessibility fixture");
    let window_id = "accepted-table-row-button";
    let host_id = fixture["hostId"].as_str().unwrap();
    let mut scene_node = component_scene_ui(host_id, ui_wgpu::wgpu::SurfaceKind::Table);
    let UiNode::ComponentScene(scene) = &mut scene_node else { unreachable!() };
    scene.surface_id = fixture["surfaceId"].as_str().unwrap().into();
    scene.controller_id = fixture["controllerId"].as_str().unwrap().into();
    scene.table = Some(ui_wgpu::wgpu::TableScene::base(fixture["columns"].to_string(), serde_json::Value::Array(vec![fixture["row"].clone()]).to_string()));
    crate::scenes::remember_scene_theme(&Theme::default());
    let node = seed_scene_window_with(window_id, scene_node);
    let retained = retained_scene(window_id, node);
    let controls = crate::scenes::table_button_accessibility_cells(&retained, Rect::new(0.0, 0.0, 400.0, 300.0), ui_wgpu::wgpu::UiDriverDrag::Handle);
    assert_eq!(controls.len(), 1);
    let control = controls[0].clone();
    assert_eq!(control.key, rebased_scene_key(fixture["expected"]["key"].as_str().unwrap(), host_id, &retained.host_id), "the button key is the fixture's, scoped by the engine-minted host");
    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained Table target")));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    crate::scenes::stage_table_button_accessibility_cells(&retained.host_id, controls);
    crate::scenes::seal_table_button_accessibility_candidates(902);
    crate::scenes::acknowledge_table_button_accessibility_candidates(902);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    let button = published_accessibility_nodes_for_test(window_id).into_iter().find(|entry| entry.key == control.key).expect("accepted Table row button");
    assert_eq!((button.role.as_str(), button.label.as_deref(), button.focusable, button.tabbable, button.actionable), ("button", fixture["expected"]["label"].as_str(), true, true, true));

    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(dispatch_accessibility_event(window_id, generation + 1, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(), "a stale window generation is inert");
    assert!(
        dispatch_accessibility_event(window_id, generation, button.node_id, &rebased_scene_key("table-ax-host.row.stale.actions.0", host_id, &retained.host_id), ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(),
        "a stale key is inert"
    );
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_some());
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(serde_json::to_value(&actions[0]).expect("action json"), fixture["expected"]["action"]);

    crate::scenes::stage_table_button_accessibility_cells(&retained.host_id, Vec::new());
    crate::scenes::seal_table_button_accessibility_candidates(903);
    crate::scenes::acknowledge_table_button_accessibility_candidates(903);
    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none());
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn accepted_graph_checkpoint_publishes_one_tabbable_exact_action_and_rejects_stale_addresses() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧱️elements/🌳️GraphTimelineHost/🧫️fixtures/🎯️checkpoint-hit/🔣️.json"))).expect("GraphTimeline checkpoint fixture");
    let expected = &fixture["accessibility"];
    let window_id = "accepted-graph-checkpoint";
    let host_id = fixture["hostId"].as_str().unwrap();
    let mut scene_node = component_scene_ui(host_id, ui_wgpu::wgpu::SurfaceKind::GraphTimeline);
    let UiNode::ComponentScene(scene) = &mut scene_node else { unreachable!() };
    scene.surface_id = fixture["surfaceId"].as_str().unwrap().into();
    scene.controller_id = fixture["controllerId"].as_str().unwrap().into();
    scene.graph_timeline = Some(ui_wgpu::wgpu::GraphTimelineScene { columns_json: fixture["columns"].to_string() });
    let values = fixture["bounds"].as_array().unwrap();
    let bounds = Rect::new(values[0].as_f64().unwrap() as f32, values[1].as_f64().unwrap() as f32, values[2].as_f64().unwrap() as f32, values[3].as_f64().unwrap() as f32);
    let node = seed_scene_window_with(window_id, scene_node);
    let retained = retained_scene(window_id, node);
    let label_track = crate::scenes::graph_timeline_scene_label_track(&retained, &Theme::default(), &mut ui_wgpu::wgpu::FontAtlas::shaped_default());
    let controls = crate::scenes::graph_timeline_accessibility_controls(&retained, bounds, &Theme::default(), label_track);
    let control = controls.iter().find(|control| control.key == rebased_scene_key(expected["key"].as_str().unwrap(), host_id, &retained.host_id)).expect("expected checkpoint").clone();
    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained GraphTimeline target")));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    crate::scenes::stage_graph_timeline_accessibility_controls(&retained.host_id, controls);
    crate::scenes::seal_graph_timeline_accessibility_candidates(922);
    crate::scenes::acknowledge_graph_timeline_accessibility_candidates(922);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    let button = published_accessibility_nodes_for_test(window_id).into_iter().find(|entry| entry.key == control.key).expect("accepted GraphTimeline checkpoint button");
    assert_eq!((button.role.as_str(), button.label.as_deref(), button.focusable, button.tabbable, button.actionable), (expected["role"].as_str().unwrap(), expected["accessibleName"].as_str(), true, true, true));

    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(dispatch_accessibility_event(window_id, generation + 1, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(), "a stale window generation is inert");
    assert!(
        dispatch_accessibility_event(window_id, generation, button.node_id, &rebased_scene_key("history-host.history.stale", host_id, &retained.host_id), ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(),
        "a stale key is inert"
    );
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_some());
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(serde_json::to_value(&actions[0]).expect("action json"), expected["action"]);

    crate::scenes::stage_graph_timeline_accessibility_controls(&retained.host_id, Vec::new());
    crate::scenes::seal_graph_timeline_accessibility_candidates(923);
    crate::scenes::acknowledge_graph_timeline_accessibility_candidates(923);
    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none());
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn presented_editor_text_focus_rebases_only_after_same_host_pixels_are_accepted() {
    let window_id = "presented-text-focus-rebase";
    let kind = ui_wgpu::wgpu::SurfaceKind::TextEditor;
    assert!(publish_focus_rebase_document(window_id, 1, 501, kind, &[2]).is_none());
    drive_focus_rebase_reconcile(window_id, 1);
    reconcile_focus_rebase_document(window_id, 2, kind, &[2, 3]);
    let old = stage_new_focus_rebase_document(window_id, 3, 502, kind, &[2, 4]);
    assert!(acknowledge_presented_input(502));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let host_id = retained_scene_host_id(window_id, old);
    install_fixture_text_editor_focus(window_id, generation, old, &host_id);

    let successor = stage_focus_rebase_document(window_id, 4, 503, kind, &[2, 3, 4]);
    assert_ne!(old, successor, "the accepted sibling sequence exercises distinct arena addresses");
    assert!(FOCUSED_TEXT_EDITOR.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.node == old && focus.host_id == host_id)), "candidate construction cannot mutate presented focus");
    assert_eq!(UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)), Some(generation));
    assert!(acknowledge_presented_input(503));
    assert!(FOCUSED_TEXT_EDITOR.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.node == successor && focus.host_id == host_id)), "accepted pixels rebase the same mounted Text editor focus");
    FOCUSED_TEXT_EDITOR.with(|cell| *cell.borrow_mut() = None);
}

#[test]
fn presented_table_stepper_focus_rebases_only_after_same_host_pixels_are_accepted() {
    let window_id = "presented-table-stepper-focus-rebase";
    let kind = ui_wgpu::wgpu::SurfaceKind::Table;
    assert!(publish_focus_rebase_document(window_id, 1, 521, kind, &[2]).is_none());
    drive_focus_rebase_reconcile(window_id, 1);
    reconcile_focus_rebase_document(window_id, 2, kind, &[2, 3]);
    let old = stage_new_focus_rebase_document(window_id, 3, 522, kind, &[2, 4]);
    assert!(acknowledge_presented_input(522));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let host_id = retained_scene_host_id(window_id, old);
    focus_table_stepper(window_id, generation, old, &host_id, crate::scenes::TableStepperFocusTarget { row_id: "r1".into(), column_id: "count".into() });

    let successor = stage_focus_rebase_document(window_id, 4, 523, kind, &[2, 3, 4]);
    assert_ne!(old, successor);
    assert!(FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.node == old && focus.host_id == host_id)), "candidate construction cannot mutate accepted focus");
    assert!(acknowledge_presented_input(523));
    assert!(FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.node == successor && focus.host_id == host_id)), "accepted pixels rebase the same Table stepper focus");

    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(apply_focused_table_stepper_key(&ui_wgpu::wgpu::KeyAction::End, &mut input));
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    let args = actions[0].args.as_ref().expect("stepper args");
    assert_eq!(args.get("objectId").and_then(semio_framework::DslValue::as_str), Some("r1"));
    assert_eq!(args.get("delta").and_then(semio_framework::DslValue::as_f64), Some(3.0));
    FOCUSED_TABLE_STEPPER.with(|cell| *cell.borrow_mut() = None);
}

#[test]
fn presented_editor_ink_focus_rebases_only_after_same_host_pixels_are_accepted() {
    let window_id = "presented-ink-focus-rebase";
    let kind = ui_wgpu::wgpu::SurfaceKind::InkCanvas;
    assert!(publish_focus_rebase_document(window_id, 1, 511, kind, &[2]).is_none());
    drive_focus_rebase_reconcile(window_id, 1);
    reconcile_focus_rebase_document(window_id, 2, kind, &[2, 3]);
    let old = stage_new_focus_rebase_document(window_id, 3, 512, kind, &[2, 4]);
    assert!(acknowledge_presented_input(512));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let host_id = retained_scene_host_id(window_id, old);
    focus_ink_editor(window_id, generation, old, &host_id);

    let successor = stage_focus_rebase_document(window_id, 4, 513, kind, &[2, 3, 4]);
    assert_ne!(old, successor, "the accepted sibling sequence exercises distinct arena addresses");
    assert_eq!(focused_ink_editor(), Some((window_id.into(), generation, old, host_id.clone())), "candidate construction cannot mutate presented Ink edit focus");
    assert!(acknowledge_presented_input(513));
    assert_eq!(focused_ink_editor(), Some((window_id.into(), generation, successor, host_id.clone())), "accepted pixels rebase the same mounted Ink editor focus");
    FOCUSED_INK_EDITOR.with(|cell| *cell.borrow_mut() = None);
}

#[test]
fn presented_editor_ink_clipboard_owners_rebase_only_after_same_host_pixels_are_accepted() {
    let window_id = "presented-ink-clipboard-rebase";
    let kind = ui_wgpu::wgpu::SurfaceKind::InkCanvas;
    assert!(publish_focus_rebase_document(window_id, 1, 521, kind, &[2]).is_none());
    drive_focus_rebase_reconcile(window_id, 1);
    reconcile_focus_rebase_document(window_id, 2, kind, &[2, 3]);
    let old = stage_new_focus_rebase_document(window_id, 3, 522, kind, &[2, 4]);
    assert!(acknowledge_presented_input(522));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let host_id = retained_scene_host_id(window_id, old);
    let address = InkClipboardAddress { window_id: window_id.into(), window_generation: generation, node: old, host_id: host_id.clone(), rect: Rect::new(0.0, 0.0, 100.0, 100.0) };
    FOCUSED_INK_SURFACE.with(|cell| *cell.borrow_mut() = Some(address.clone()));
    INK_CLIPBOARD_STREAMS.with(|cell| cell.borrow_mut()[0] = Some(InkClipboardStream { id: 525, address: address.clone(), image_data_url: false, text: "retained".into() }));
    let pending = reserve_pending_native_ink_clipboard(address).expect("native clipboard owner reserves");

    let successor = stage_focus_rebase_document(window_id, 4, 523, kind, &[2, 3, 4]);
    assert_ne!(old, successor, "the accepted sibling sequence exercises distinct arena addresses");
    assert_eq!(focused_ink_surface().map(|address| address.node), Some(old), "candidate construction cannot mutate presented clipboard focus");
    assert!(acknowledge_presented_input(523));
    assert_eq!(focused_ink_surface().map(|address| (address.node, address.host_id)), Some((successor, host_id.clone())));
    assert!(INK_CLIPBOARD_STREAMS.with(|cell| cell.borrow()[0].as_ref().is_some_and(|stream| stream.address.node == successor && stream.address.host_id == host_id)));
    assert!(PENDING_NATIVE_INK_CLIPBOARD.with(|cell| cell.borrow()[usize::from(pending.slot)].mounted.as_ref().is_some_and(|mounted| mounted.address.node == successor && mounted.address.host_id == host_id)));
    FOCUSED_INK_SURFACE.with(|cell| *cell.borrow_mut() = None);
    INK_CLIPBOARD_STREAMS.with(|cell| cell.borrow_mut()[0] = None);
    PENDING_NATIVE_INK_CLIPBOARD.with(|cell| cell.borrow_mut()[usize::from(pending.slot)].mounted = None);
}

#[test]
fn presented_ink_intent_finishes_before_an_unchanged_same_host_sibling_refresh_is_accepted() {
    while !close_scene_interaction_step() {}
    let contract: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
    let packet = &contract["sceneIntentPresentation"];
    let records = packet["records"].as_array().unwrap();
    let children = |index: usize| records[index].as_array().unwrap().iter().map(|id| id.as_u64().unwrap()).collect::<Vec<_>>();
    let law = ink_editing_law();
    let ink = ink_intent_scene(&law, packet["unchangedUtilities"][0].as_str().unwrap(), "intent-unchanged");
    let window_id = "presented-ink-intent-unchanged";
    let old = publish_ink_intent_document(window_id, 1, 601, &children(0), &ink);
    let host_id = retained_scene_host_id(window_id, old);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let pointer = ui_render::PointerId(61);
    begin_pending_ink_intent(window_id, old, pointer, &mut input);

    let successor = stage_ink_intent_document(window_id, 2, 602, &children(1), &ink);
    let candidate_host = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let retained = engine.candidate_tree(window_id).and_then(|tree| tree.node(successor)).unwrap();
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { panic!("candidate receiver is an Ink scene") };
        scene.host_id.clone()
    });
    assert_eq!(candidate_host, host_id, "the sibling insertion keeps the exact mounted Ink host");
    assert!(!acknowledge_presented_input(602), "a checked-out presented Ink job bars its candidate pixels until the old event reaches a terminal state");
    drive_ink_scene_terminal(&mut input);
    let admitted = crate::collect_fixture_actions(&mut input);
    assert_eq!(admitted.iter().filter(|action| action.action == "inkApplyEvents").count(), packet["admittedActions"].as_u64().unwrap() as usize);
    cancel_scene_pointer(pointer, &mut input);
    assert!(acknowledge_presented_input(602));

    let successor_pointer = ui_render::PointerId(62);
    begin_pending_ink_intent(window_id, successor, successor_pointer, &mut input);
    drive_ink_scene_terminal(&mut input);
    let successor_actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(successor_actions.iter().filter(|action| action.action == "inkApplyEvents").count(), packet["successorActions"][0].as_u64().unwrap() as usize);
    cancel_scene_pointer(successor_pointer, &mut input);
    retire_presented_document(window_id);
}

#[test]
fn presented_ink_intent_uses_old_authored_content_before_a_changed_same_host_successor_starts() {
    while !close_scene_interaction_step() {}
    let contract: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
    let packet = &contract["sceneIntentPresentation"];
    let records = packet["records"].as_array().unwrap();
    let children = |index: usize| records[index].as_array().unwrap().iter().map(|id| id.as_u64().unwrap()).collect::<Vec<_>>();
    let law = ink_editing_law();
    let old_ink = ink_intent_scene(&law, packet["changedUtilities"][0].as_str().unwrap(), "intent-before-change");
    let successor_ink = ink_intent_scene(&law, packet["changedUtilities"][1].as_str().unwrap(), "intent-after-change");
    let window_id = "presented-ink-intent-authored-change";
    let old = publish_ink_intent_document(window_id, 1, 611, &children(0), &old_ink);
    let host_id = retained_scene_host_id(window_id, old);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let pointer = ui_render::PointerId(63);
    begin_pending_ink_intent(window_id, old, pointer, &mut input);

    let successor = stage_ink_intent_document(window_id, 2, 612, &children(1), &successor_ink);
    assert!(UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let retained = engine.candidate_tree(window_id).and_then(|tree| tree.node(successor)).unwrap();
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { return false };
        scene.host_id == host_id && scene.ink_canvas.as_ref().is_some_and(|ink| ink.document_json.contains("intent-after-change"))
    }));
    assert!(!acknowledge_presented_input(612), "new authored pixels cannot mix with a partially stepped job that owns the old document snapshot");
    drive_ink_scene_terminal(&mut input);
    let admitted = crate::collect_fixture_actions(&mut input);
    assert_eq!(admitted.iter().filter(|action| action.action == "inkApplyEvents").count(), packet["admittedActions"].as_u64().unwrap() as usize);
    cancel_scene_pointer(pointer, &mut input);
    assert!(acknowledge_presented_input(612));

    let successor_pointer = ui_render::PointerId(64);
    begin_pending_ink_intent(window_id, successor, successor_pointer, &mut input);
    drive_ink_scene_terminal(&mut input);
    let successor_actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(successor_actions.iter().filter(|action| action.action == "inkApplyEvents").count(), packet["successorActions"][1].as_u64().unwrap() as usize);
    cancel_scene_pointer(successor_pointer, &mut input);
    retire_presented_document(window_id);
}

#[test]
fn presented_ink_intent_in_a_hidden_window_does_not_block_an_unrelated_candidate() {
    while !close_scene_interaction_step() {}
    let contract: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
    let packet = &contract["sceneIntentPresentation"];
    let rows = contract["barrierScope"].as_array().unwrap();
    assert!(rows.iter().any(|row| row["name"] == "hidden-window" && row["blocks"] == false));
    let records = packet["records"].as_array().unwrap();
    let children = |index: usize| records[index].as_array().unwrap().iter().map(|id| id.as_u64().unwrap()).collect::<Vec<_>>();
    let law = ink_editing_law();
    let ink = ink_intent_scene(&law, "text", "intent-hidden-window");
    let hidden_window = "presented-ink-intent-hidden-owner";
    let hidden = publish_ink_intent_document(hidden_window, 1, 621, &children(0), &ink);
    let pointer = ui_render::PointerId(65);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    begin_pending_ink_intent(hidden_window, hidden, pointer, &mut input);

    let participant = "presented-ink-intent-visible-candidate";
    publish_ink_intent_document(participant, 1, 622, &children(0), &ink);
    let _ = stage_ink_intent_document(participant, 2, 623, &children(1), &ink);
    assert!(acknowledge_presented_input(623), "an exact hidden-window owner cannot veto a candidate that does not contain its window");
    assert!(SCENE_INTENTS.with(|cell| cell.borrow().slots.iter().flatten().any(|intent| intent.window_id == hidden_window && intent.pointer_id == Some(pointer))));

    drive_ink_scene_terminal(&mut input);
    crate::collect_fixture_actions(&mut input);
    cancel_scene_pointer(pointer, &mut input);
    retire_presented_document(hidden_window);
    retire_presented_document(participant);
}

#[test]
fn presented_ink_intent_from_a_stale_revision_does_not_block_the_current_candidate() {
    while !close_scene_interaction_step() {}
    let contract: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
    let packet = &contract["sceneIntentPresentation"];
    let rows = contract["barrierScope"].as_array().unwrap();
    assert!(rows.iter().any(|row| row["name"] == "stale-revision" && row["blocks"] == false));
    let records = packet["records"].as_array().unwrap();
    let children = |index: usize| records[index].as_array().unwrap().iter().map(|id| id.as_u64().unwrap()).collect::<Vec<_>>();
    let law = ink_editing_law();
    let ink = ink_intent_scene(&law, "text", "intent-stale-revision");
    let window_id = "presented-ink-intent-stale-revision";
    let old = publish_ink_intent_document(window_id, 1, 631, &children(0), &ink);
    let pointer = ui_render::PointerId(66);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    begin_pending_ink_intent(window_id, old, pointer, &mut input);
    let _ = stage_ink_intent_document(window_id, 2, 632, &children(1), &ink);
    SCENE_INTENTS.with(|cell| {
        let mut queue = cell.borrow_mut();
        let stale = queue.slots.iter_mut().flatten().find(|intent| intent.pointer_id == Some(pointer)).expect("pending stale-revision owner");
        stale.tree_revision = stale.tree_revision.saturating_sub(1);
        assert_ne!(UI_ENGINE.with(|cell| cell.borrow().tree_revision(window_id)), Some(stale.tree_revision));
    });
    assert!(acknowledge_presented_input(632), "an intent that no longer belongs to the presented revision cannot veto its candidate");
    drive_ink_scene_terminal(&mut input);
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "the stale intent closes instead of dispatching against successor content");
    cancel_scene_pointer(pointer, &mut input);
    retire_presented_document(window_id);
}

#[test]
fn presented_ink_intent_barrier_has_an_independent_bounded_progress_owner() {
    while !close_scene_interaction_step() {}
    let contract: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
    let packet = &contract["sceneIntentPresentation"];
    let records = packet["records"].as_array().unwrap();
    let children = |index: usize| records[index].as_array().unwrap().iter().map(|id| id.as_u64().unwrap()).collect::<Vec<_>>();
    let law = ink_editing_law();
    let ink = ink_intent_scene(&law, "text", "intent-presenter-progress");
    let window_id = "presented-ink-intent-presenter-progress";
    let old = publish_ink_intent_document(window_id, 1, 641, &children(0), &ink);
    let pointer = ui_render::PointerId(67);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    begin_pending_ink_intent(window_id, old, pointer, &mut input);
    let _ = stage_ink_intent_document(window_id, 2, 642, &children(1), &ink);

    for _ in 0..4096 {
        match progress_presented_input_candidate(642, &mut input) {
            PresentedInputCandidateProgress::Pending { advanced: true } => continue,
            PresentedInputCandidateProgress::Pending { advanced: false } => panic!("the exact blocking intent must advance one bounded unit"),
            PresentedInputCandidateProgress::Ready => break,
            PresentedInputCandidateProgress::Stale => panic!("the exact candidate witness remains live while its old interaction drains"),
        }
    }
    assert!(scene_interaction_terminal_is_empty());
    assert!(acknowledge_presented_input(642));
    assert_eq!(crate::collect_fixture_actions(&mut input).iter().filter(|action| action.action == "inkApplyEvents").count(), 1);
    cancel_scene_pointer(pointer, &mut input);
    retire_presented_document(window_id);
}

#[test]
fn window_close_retires_queued_scene_and_capture_owners_before_slot_release() {
    let window_id = "closing-window-scene-owners";
    let node = seed_scene_window(window_id, "closing-canvas", ui_wgpu::wgpu::SurfaceKind::Canvas2d);
    let target = retained_scene_target(window_id, node).unwrap();
    let generation = target.window_generation;
    assert!(claim_scene_pointer_owner(target, ui_render::PointerId(1)));
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_scene_ui_command(
        window_id,
        node,
        ui_wgpu::wgpu::SurfaceKind::Canvas2d,
        rect,
        &ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        Some(ui_render::PointerId(1)),
        &mut input,
    );
    assert!(request_ui_document_close(window_id));
    for _ in 0..262_144 {
        if !ui_document_close_pending() {
            break;
        }
        assert!(close_ui_document_one());
    }
    assert!(!ui_document_close_pending());
    assert!(UI_ENGINE.with(|cell| cell.borrow().surface_token(window_id).is_none()));
    assert!(scene_interaction_terminal_is_empty(), "the sole window close owner also retires its queued scene work");
    assert!(SCENE_POINTER_OWNERS.with(|cell| cell.borrow().slots.iter().all(|owner| owner.target.window_id != window_id || owner.target.window_generation != generation)));
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn closing_window_silently_retires_an_active_canvas_gesture() {
    let window_id = "closing-window-active-canvas";
    let node = seed_scene_window(window_id, "active-canvas", ui_wgpu::wgpu::SurfaceKind::Canvas2d);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_scene_ui_command(
        window_id,
        node,
        ui_wgpu::wgpu::SurfaceKind::Canvas2d,
        Rect::new(0.0, 0.0, 200.0, 200.0),
        &ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        Some(ui_render::PointerId(1)),
        &mut input,
    );
    assert!(drive_scene_interaction_step(&mut input));
    assert_eq!(crate::collect_fixture_actions(&mut input).iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown"]);
    assert!(request_ui_document_close(window_id));
    crate::scenes::request_canvas_pointer_gesture_cancel_for_window(window_id);
    drive_scene_interaction_step(&mut input);
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "closing a mounted Canvas cannot publish a synthetic pointer terminal action");
    for _ in 0..262_144 {
        if !ui_document_close_pending() {
            break;
        }
        assert!(close_ui_document_one());
    }
    assert!(!ui_document_close_pending());
    assert!(!crate::scenes::cancel_canvas_pointer_gesture_for(ui_render::PointerId(1), &mut input));
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn live_canvas_gesture_cancels_when_its_published_generation_is_replaced() {
    while !close_scene_interaction_step() {}
    let mut discarded = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    crate::scenes::cancel_canvas_pointer_gesture(&mut discarded);
    let window_id = "canvas-generation-cancellation";
    let node = publish_focus_rebase_document(window_id, 1, 454, ui_wgpu::wgpu::SurfaceKind::Canvas2d, &[4]).expect("presented Canvas receiver");
    assert_eq!(retained_scene_surface_id(window_id, node), window_id);
    let canvas_host = retained_scene_host_id(window_id, node);
    let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_scene_ui_command(
        window_id,
        node,
        ui_wgpu::wgpu::SurfaceKind::Canvas2d,
        rect,
        &ui_wgpu::wgpu::UiEvent::PointerDown { x: 20.0, y: 20.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: ui_wgpu::wgpu::EventModifiers::default() },
        Some(ui_render::PointerId(1)),
        &mut input,
    );
    assert!(drive_scene_interaction_step(&mut input));
    assert_eq!(crate::collect_fixture_actions(&mut input).iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown"]);

    let successor = publish_focus_rebase_document(window_id, 2, 455, ui_wgpu::wgpu::SurfaceKind::TextEditor, &[4]).expect("presented non-Canvas successor");
    assert_ne!(retained_scene_host_id(window_id, successor), canvas_host, "component-kind replacement receives a fresh private host identity");
    assert!(drive_scene_interaction_step(&mut input), "the terminal lane observes the replaced generation even when the old surface no longer paints");
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerUp"]);
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("cancelled")).and_then(semio_framework::DslValue::as_bool), Some(true));
    retire_presented_document(window_id);
}

#[test]
fn scene_command_skips_bespoke_surface_kinds_to_avoid_double_dispatch() {
    let window_id = "apply-ui-commands-scene-bespoke-skip";
    let node = seed_scene_window(window_id, "s1", ui_wgpu::wgpu::SurfaceKind::NodeGraph);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &[ui_wgpu::wgpu::UiCommand::Scene {
            window_id: window_id.into(),
            node,
            surface_id: "s1".into(),
            kind: ui_wgpu::wgpu::SurfaceKind::NodeGraph,
            rect,
            event: ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        }],
        None,
        &mut input,
    );

    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "node-graph already gets real input through its own bespoke dock/engine_canvas host and must not be double-dispatched via UiCommand::Scene");
}

#[test]
fn pointer_button_code_round_trips_through_pointer_button_from_code_for_every_dom_button_code() {
    for code in [0i16, 1, 2] {
        let button = pointer_button_from_code(code);
        assert_eq!(pointer_button_code(button), code, "code {code} should round-trip through PointerButton unchanged (regression guard for the W4 1/2-swap fix)");
    }
}

/// 🧭️ Smoke-tests every one of the 11 non-bespoke `SurfaceKind`s through the real `UiCommand::Scene`
/// path (PointerDown, PointerMove, PointerUp, Scroll) — proving each one is actually reachable
/// through `apply_scene_ui_command` (no panics, no silently-skipped kind) before the per-frame
/// `apply_scene_wheel`/`apply_scene_pointer` sampling fallback they used to depend on is deleted.
#[test]
fn scene_command_reaches_every_generic_fallback_surface_kind_without_panicking() {
    let kinds = [
        ui_wgpu::wgpu::SurfaceKind::Canvas2d,
        ui_wgpu::wgpu::SurfaceKind::Paint2d,
        ui_wgpu::wgpu::SurfaceKind::TextEditor,
        ui_wgpu::wgpu::SurfaceKind::InkCanvas,
        ui_wgpu::wgpu::SurfaceKind::GraphTimeline,
        ui_wgpu::wgpu::SurfaceKind::Table,
        ui_wgpu::wgpu::SurfaceKind::VirtualFileSystem,
        ui_wgpu::wgpu::SurfaceKind::IconRender,
        ui_wgpu::wgpu::SurfaceKind::BlockList,
        ui_wgpu::wgpu::SurfaceKind::DiffView,
        ui_wgpu::wgpu::SurfaceKind::EventFeed,
    ];
    for kind in kinds {
        assert!(!crate::scenes::scene_has_bespoke_pointer_dispatch(kind), "{kind:?} must stay in the generic-fallback set for this smoke test to be meaningful");
        let window_id = format!("apply-ui-commands-scene-smoke-{kind:?}");
        let node = seed_scene_window(&window_id, "s1", kind);
        let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let events = [
            ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
            ui_wgpu::wgpu::UiEvent::PointerMove { x: 12.0, y: 12.0, modifiers: Default::default() },
            ui_wgpu::wgpu::UiEvent::PointerUp { x: 12.0, y: 12.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
            ui_wgpu::wgpu::UiEvent::Scroll { x: 10.0, y: 10.0, delta_x: 0.0, delta_y: 4.0, modifiers: Default::default() },
        ];
        for event in events {
            apply_ui_commands(&[ui_wgpu::wgpu::UiCommand::Scene { window_id: window_id.clone(), node, surface_id: "s1".into(), kind, rect, event }], None, &mut input);
        }
    }
}

/// 🖱️➡️ W4 fix regression guard: a right-click (`button == 2`) `PointerDown` on a `✏️TextEditor`
/// scene must route through `apply_scene_ui_command` -> `handle_scene_pointer_button` ->
/// `engine_canvas::text_editor_pointer_down` with the REAL button code — before this ticket's fix,
/// `pointer_button_from_code`'s 1/2 swap would have handed it `1`, not `2`. This test can't observe
/// `EditorHost`'s own caret state from here: `text_editor_pointer_down` only reaches a live
/// `EditorHost` once `engine_canvas::paint_text_editor` has lazily created one in `ENGINE_SURFACES`
/// (a real render pass this unit test intentionally doesn't run), so it correctly no-ops gracefully
/// here. `framework_editor::pointer_down_screen_repositions_caret_for_non_primary_button_but_does_
/// not_start_a_drag_selection` (that crate's own test) is the real assertion on `EditorHost`'s
/// behavior; `pointer_button_code_round_trips_through_pointer_button_from_code_for_every_dom_
/// button_code` (above) is the real assertion on the button-code plumbing. This test's only job is
/// proving the `UiCommand::Scene` route reaches that call site end-to-end without panicking.
#[test]
fn scene_command_right_click_on_text_editor_does_not_panic_and_stays_a_graceful_no_op_without_a_rendered_host() {
    let window_id = "apply-ui-commands-scene-text-editor-right-click";
    let node = seed_scene_window(window_id, "s1", ui_wgpu::wgpu::SurfaceKind::TextEditor);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &[ui_wgpu::wgpu::UiCommand::Scene {
            window_id: window_id.into(),
            node,
            surface_id: "s1".into(),
            kind: ui_wgpu::wgpu::SurfaceKind::TextEditor,
            rect,
            event: ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Secondary, modifiers: Default::default() },
        }],
        None,
        &mut input,
    );

    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "no ENGINE_SURFACES entry exists without a real paint pass, so this should no-op rather than panic or queue a stale action");
}
//#endregion 🔖️SceneCommandTests


/// 🧾️ Publishes an owned typed input record for the shared retained-draft corpus.
fn retained_input_document(window: &str, kind: &str, value: serde_json::Value) -> ui_wgpu::wgpu::tree::UiDocumentTree {
    let mut document = clipboard_input_document(window);
    document.try_upsert_record(serde_json::from_value(serde_json::json!({
        "id":1,"key":"clipboard/name","component":{"type":"input","kind":kind,"value":value,"commit":"blur"},
        "layout":{"kind":"leaf","width":"fill","height":"hug"},"style":{},"activity":"idle","accessibility":{"label":"Draft input"},
        "bindings":[{"trigger":"commit","action":{"scope":"fixture","name":"setValue","version":1}}]
    })).unwrap()).unwrap();
    document
}

#[test]
fn retained_input_commit_has_a_generation_bound_completion_receipt() {
    let window = "input-draft-receipt";
    let document = retained_input_document(window, "text", "Original".into());
    publish_presented_document(window, 1, 9801, "fixture", document);
    UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(window, ui_wgpu::wgpu::UiEvent::KeyDown { key: "Tab".into(), modifiers: Default::default() }));
    UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(window, ui_wgpu::wgpu::UiEvent::KeyDown { key: "a".into(), modifiers: ui_wgpu::wgpu::EventModifiers { ctrl: true, ..Default::default() } }));
    UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(window, ui_wgpu::wgpu::UiEvent::TextInput { text: "Refused".into() }));
    let commands = UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(window, ui_wgpu::wgpu::UiEvent::KeyDown { key: "Enter".into(), modifiers: Default::default() }));
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_ui_commands(&commands, None, &mut input);
    let queued = input.take_action_step().unwrap().expect("Enter submits one input commit").into_envelope().unwrap();
    assert!(queued.receipt.is_some(), "focused input retains a receipt until dispatch settles");
    let receipt = queued.receipt.unwrap();
    settle_retained_input_receipt(ui_wgpu::wgpu::ActionQueueReceipt { source: ui_wgpu::wgpu::ActionQueueReceiptSource::CanvasTextEditor, ..receipt }, true);
    assert!(RETAINED_INPUT_RECEIPTS.with(|cell| cell.borrow().slots.iter().any(Option::is_some)), "equal tokens of other receipt sources cannot consume an input completion");
    crate::settle_renderer_action_receipt(receipt, crate::engine_canvas::TextEditorActionOutcome::Cancelled);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🚦️input-draft-disposition/🔣️.json")).unwrap();
    let capacity = corpus["receiptCapacity"].as_u64().unwrap() as usize;
    assert_eq!(capacity, ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY);
    let intent = commands.iter().find_map(|command| match command { ui_wgpu::wgpu::UiCommand::App { intent, .. } => Some(intent), _ => None }).unwrap();
    let mut held: Vec<_> = (0..capacity).map(|_| reserve_retained_input_receipt(window, intent).unwrap()).collect();
    assert!(matches!(reserve_retained_input_receipt(window, intent), Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)));
    let retired = held[0].0.take().unwrap();
    settle_retained_input_receipt(retired, false);
    let replacement = reserve_retained_input_receipt(window, intent).unwrap();
    assert_ne!(replacement.0.unwrap().token, retired.token);
    settle_retained_input_receipt(retired, true);
    assert!(UI_ENGINE.with(|cell| cell.borrow().retained_input_draft(window, intent).unwrap().text == "Refused"));
    drop(replacement);
    drop(held);
    assert!(RETAINED_INPUT_RECEIPTS.with(|cell| cell.borrow().slots.iter().all(Option::is_none)));
    let pending = reserve_retained_input_receipt(window, intent).unwrap();
    retire_presented_document(window);
    let replacement_document = retained_input_document(window, "text", corpus["replacementBase"].clone());
    publish_presented_document(window, 2, 9802, "fixture", replacement_document);
    let replacement_focus = UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(window, ui_wgpu::wgpu::UiEvent::KeyDown { key: "Tab".into(), modifiers: Default::default() })).into_iter().find_map(|command| match command { ui_wgpu::wgpu::UiCommand::FocusChanged { node: Some(node), .. } => Some(node), _ => None }).unwrap();
    UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(window, ui_wgpu::wgpu::UiEvent::KeyDown { key: "a".into(), modifiers: ui_wgpu::wgpu::EventModifiers { ctrl: true, ..Default::default() } }));
    UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(window, ui_wgpu::wgpu::UiEvent::TextInput { text: "Refused".into() }));
    settle_retained_input_receipt(pending.0.unwrap(), true);
    let replacement_text = UI_ENGINE.with(|cell| {
        let ui = cell.borrow();
        let tree = ui.tree(window).unwrap();
        tree.node(replacement_focus).unwrap().state.edit.as_ref().unwrap().text.clone()
    });
    assert_eq!(replacement_text, "Refused", "an older surface receipt cannot restore its replacement's buffer");
    drop(pending);
    retire_presented_document(window);
}

#[test]
fn retained_input_refusal_settlement_obeys_the_shared_draft_disposition_law() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🚦️input-draft-disposition/🔣️.json" )).unwrap();
    for (index, law) in corpus["cases"].as_array().unwrap().iter().enumerate() {
        let window = format!("draft-disposition-{index}");
        let document = retained_input_document(&window, law["kind"].as_str().unwrap(), law["base"].clone());
        publish_presented_document(&window, 1, 9900 + index as u64, "fixture", document);
        let keys = |key: &str, ctrl: bool| ui_wgpu::wgpu::UiEvent::KeyDown { key: key.into(), modifiers: ui_wgpu::wgpu::EventModifiers { ctrl, ..Default::default() } };
        let focused = UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(&window, keys("Tab", false))).into_iter().find_map(|command| match command { ui_wgpu::wgpu::UiCommand::FocusChanged { node: Some(node), .. } => Some(node), _ => None }).unwrap();
        UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(&window, keys("a", true)));
        UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(&window, ui_wgpu::wgpu::UiEvent::TextInput { text: law["submitted"].as_str().unwrap().into() }));
        let commands = UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(&window, keys("Enter", law["kind"] == "longText")));
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        apply_ui_commands(&commands, None, &mut input);
        let receipt = input.take_action_step().unwrap().expect("commit submits").into_envelope().unwrap().receipt.expect("addressed commit owns one completion");
        assert_eq!(receipt.source, ui_wgpu::wgpu::ActionQueueReceiptSource::RetainedInput);
        if law["draft"] != law["submitted"] {
            UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(&window, keys("a", true)));
            UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(&window, ui_wgpu::wgpu::UiEvent::TextInput { text: law["draft"].as_str().unwrap().into() }));
        }
        let outcome = if law["cancelled"] == true { crate::engine_canvas::TextEditorActionOutcome::Cancelled } else if law["disposition"] == "discard" { crate::engine_canvas::TextEditorActionOutcome::Refused("timeTravel.frozen: draft refused") } else { crate::engine_canvas::TextEditorActionOutcome::Refused("validation.recoverable: repair the draft") };
        crate::settle_renderer_action_receipt(receipt, outcome);
        let text = UI_ENGINE.with(|cell| cell.borrow().tree(&window).unwrap().node(focused).unwrap().state.edit.as_ref().unwrap().text.clone());
        assert_eq!(text, law["expected"].as_str().unwrap(), "{}", law["name"]);
        crate::settle_renderer_action_receipt(receipt, crate::engine_canvas::TextEditorActionOutcome::Refused("timeTravel.frozen: duplicate completion"));
        let repeated = UI_ENGINE.with(|cell| cell.borrow().tree(&window).unwrap().node(focused).unwrap().state.edit.as_ref().unwrap().text.clone());
        assert_eq!(repeated, text, "a settled receipt cannot alter a later buffer");
        retire_presented_document(&window);
    }
    assert!(RETAINED_INPUT_RECEIPTS.with(|cell| cell.borrow().slots.iter().all(Option::is_none)));
}
