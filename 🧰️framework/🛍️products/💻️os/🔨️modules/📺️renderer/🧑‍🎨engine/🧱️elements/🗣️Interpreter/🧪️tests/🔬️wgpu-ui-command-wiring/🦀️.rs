use super::*;

#[test]
fn retained_document_close_queue_is_bounded_and_same_window_replacement_costs_no_credit() {
    let mut queue = UiDocumentCloseQueue::default();
    let mut engine = ui_wgpu::wgpu::Ui::new();
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
        let mut engine = ui_wgpu::wgpu::Ui::new();
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
    eprintln!("[DEBUG] retained document pages retry exact source records after fuel, deadline and cancellation refusal");
}

#[test]
fn retained_document_page_with_a_live_wrong_generation_remains_a_terminal_fault() {
    let mut document = ingress_opportunity_document();
    let mut engine = ui_wgpu::wgpu::Ui::new();
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
    ActionDescriptor { controller_id: "ctrl".into(), action: name.into(), args: semio_framework::optional_json_to_dsl(args) }
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
        let actual: Option<Value> = queued[0].args.as_ref().map(|args| serde_json::from_str(&dsl::json::from_dsl_value(args).to_string()).unwrap());
        assert_eq!(actual, Some(case["expected"].clone()));
    }
    eprintln!("[DEBUG] retained native actions preserved clicked window identity and replaced conflicting descriptor targets");
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

    let merged: Value = serde_json::from_str(&dsl::json::from_dsl_value(&merge_action_args(Some(&existing_dsl), patch).expect("merged args")).to_string()).expect("json args");

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
                ui_wgpu::wgpu::SurfaceKind::InkCanvas => {
                    ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::InkCanvas, &ink.cloned().unwrap_or_else(|| ui_wgpu::wgpu::InkCanvasScene::base("[]".into(), "pen".into(), "document".into(), true))).unwrap()
                }
                _ => panic!("focus rebase fixture supports Canvas and editor surfaces"),
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

#[test]
fn scene_command_dispatches_an_ink_canvas_scroll_action() {
    let window_id = "apply-ui-commands-scene-ink-canvas-scroll";
    // 🎨️ `ink_wheel` reads straight from the scene's own `ink_canvas` payload (unlike TextEditor,
    // it needs no separate lazily-render-created host state) — mirrors `RenderEntry::ink_scene`'s
    // own fixture (`apply_scene_wheel_dispatches_actions_for_a_previously_dead_surface`).
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
    serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧪️fixtures/🖋️ink-canvas-editing/🔣️.json")).expect("shared InkCanvas editing law parses")
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
    assert_eq!(ink_case["invokeNormalPointerUp"], false);
    assert_eq!(ink_case["publishOnCancel"], serde_json::json!([]));

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
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "cancel cannot synthesize PointerUp or publish Ink events");

    let next_pointer = ui_render::PointerId(43);
    apply_ui_commands(&[down], Some(next_pointer), &mut input);
    assert!(SCENE_POINTER_OWNERS.with(|cell| cell.borrow().slots.iter().any(|owner| owner.pointer_id == next_pointer)), "the next Down is accepted after exact cancellation");
    cancel_scene_pointer(next_pointer, &mut input);
    drive_ink_scene_terminal(&mut input);
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
    assert_eq!(args.get("phase").and_then(semio_framework::DslValue::as_str), Some("atomic"));
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
    let mut arena = ui_wgpu::wgpu::arena::Arena::<()>::new();
    let node = arena.insert(());
    FOCUSED_TEXT_EDITOR.with(|cell| {
        *cell.borrow_mut() = Some(FocusedTextEditor { window_id: "text-focus-reopen".into(), window_generation: 2, node, host_id: "scene.2.1".into() });
    });

    assert!(!close_window_focus_clipboard_one("text-focus-reopen", 1), "a stale window lifetime cannot consume its successor's exact text focus");
    assert!(FOCUSED_TEXT_EDITOR.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.host_id == "scene.2.1")));
    FOCUSED_TEXT_EDITOR.with(|cell| *cell.borrow_mut() = None);
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
    focus_text_editor(window_id, generation, old, &host_id);

    let successor = stage_focus_rebase_document(window_id, 4, 503, kind, &[2, 3, 4]);
    assert_ne!(old, successor, "the accepted sibling sequence exercises distinct arena addresses");
    assert!(FOCUSED_TEXT_EDITOR.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.node == old && focus.host_id == host_id)), "candidate construction cannot mutate presented focus");
    assert_eq!(UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)), Some(generation));
    assert!(acknowledge_presented_input(503));
    assert!(FOCUSED_TEXT_EDITOR.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.node == successor && focus.host_id == host_id)), "accepted pixels rebase the same mounted Text editor focus");
    FOCUSED_TEXT_EDITOR.with(|cell| *cell.borrow_mut() = None);
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
