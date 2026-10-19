/// ⌨️ Text editor focus and clipboard: stale window close against a successor focus, focus transfer on primary presses, the node graph note caret, the space key and the accepted host clipboard, composition and accessibility.

//#region 🔖️SceneCommandTests

#[test]
fn stale_window_close_cannot_clear_a_successor_text_editor_focus() {
    let window = "text-focus-reopen";
    let node = seed_scene_window(window, "editor", semio_framework_ui_declaration::ui::SurfaceKind::TextEditor);
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
    let node = seed_scene_window(window, "editor", semio_framework_ui_declaration::ui::SurfaceKind::TextEditor);
    let target = retained_scene_target(window, node).expect("mounted editor target");
    install_fixture_text_editor_focus(window, target.window_generation, node, &target.host_id);
    for (down, button) in [(false, 0), (true, 1), (true, 2)] {
        assert!(!blur_focused_text_editor_for_pointer(None, down, button));
        assert!(FOCUSED_TEXT_EDITOR.with(|cell| cell.borrow().is_some()));
    }
    assert!(!blur_focused_text_editor_for_pointer(Some(&target), true, 0));
    assert!(blur_focused_text_editor_for_pointer(None, true, 0));
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    assert!(!apply_focused_text_editor_key(&semio_framework_ui_wgpu_input::KeyAction::Char("x".into()), &Default::default(), &mut input));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    install_fixture_text_editor_focus(window, target.window_generation, node, &target.host_id);
    let mut successor = target.clone();
    successor.window_generation += 1;
    assert!(blur_focused_text_editor_for_pointer(Some(&successor), true, 0));
    assert!(FOCUSED_TEXT_EDITOR.with(|cell| cell.borrow().is_none()));
}

#[test]
fn accepted_node_graph_note_caret_arms_resets_and_retires_with_its_exact_scene() {
    let _serialized = semio_framework_os_renderer_wgpu_engine_registry::engine_surface_law_guard();
    let window = "node-graph-note-caret";
    let mut scene = component_scene_ui(window, semio_framework_ui_declaration::ui::SurfaceKind::NodeGraph);
    let UiNode::ComponentScene(scene) = &mut scene else { unreachable!() };
    scene.node_graph = Some(semio_framework_ui_scene_node_graph::NodeGraphScene {
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
        ..semio_framework_ui_declaration::ui::NodeGraphScene::base(Vec::new(), Vec::new(), semio_framework_os_kernel::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 })
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
    assert!(semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::sync_engine_scene(&retained_scene, window, Rect::new(0.0, 0.0, 640.0, 480.0), &Theme::default()));
    assert!(semio_framework_os_renderer_wgpu_engine_graph::node_graph::component_scene_begin_note_edit_fixture(&retained_scene, "note"));
    assert!(refresh_node_graph_caret_after_pointer(&target));
    assert!(FOCUSED_NODE_GRAPH_CARET.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.target.same_component_host(&target))));
    assert!(UI_ENGINE.with(|cell| cell.borrow().window_next_clock_deadline(window)).and_then(|(_, deadline)| deadline).is_some(), "accepted note editing owns one cadence deadline");

    assert!(apply_focused_node_graph_note_key(&semio_framework_ui_wgpu_input::KeyAction::Char("!".into()), &Default::default()), "an edit reaches only the focused accepted Flow host and resets its phase");
    assert!(semio_framework_os_renderer_wgpu_engine_graph::node_graph::component_scene_has_editable_caret(&retained_scene));
    assert!(apply_focused_node_graph_note_key(&semio_framework_ui_wgpu_input::KeyAction::Enter, &Default::default()), "commit is consumed by the focused note");
    assert!(FOCUSED_NODE_GRAPH_CARET.with(|cell| cell.borrow().is_none()));
    assert!(!semio_framework_os_renderer_wgpu_engine_graph::node_graph::component_scene_has_editable_caret(&retained_scene));
    assert_eq!(UI_ENGINE.with(|cell| cell.borrow().window_next_clock_deadline(window)).and_then(|(_, deadline)| deadline), None, "committing the note cancels the cadence wake");

    semio_framework_os_renderer_wgpu_engine_attach::retire_engine_surface_fixture(&target.host_id);
}

#[test]
fn text_editor_space_reaches_the_focused_editor_before_the_runtime_pan_modifier() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    use std::future::Future;
    let _serialized = semio_framework_os_renderer_wgpu_engine_registry::engine_surface_law_guard();
    let law: Value = serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🔨️modules/✍️editor/🧪️tests/🧫️fixtures/⌨️text-input/🔣️.json")).unwrap();
    let law = law["rendererKeys"].as_array().unwrap().iter().find(|law| law["id"] == "space-is-a-printable-key").unwrap();
    let window = "text-editor-runtime-space";
    let UiNode::ComponentScene(mut scene) = component_scene_ui(window, semio_framework_ui_declaration::ui::SurfaceKind::TextEditor) else { unreachable!() };
    scene.text_editor = Some(semio_framework_ui_scene_views::TextEditorScene::base(law["text"].as_str().unwrap().into(), None, Some(serde_json::json!({ "start": law["selection"][0], "end": law["selection"][1] }).to_string())));
    let node = seed_scene_window_with(window, UiNode::ComponentScene(scene));
    let target = retained_scene_target(window, node).unwrap();
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let UiNode::ComponentScene(scene) = &engine.tree(window).unwrap().node(node).unwrap().spec.0 else { unreachable!() };
        assert!(semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::sync_engine_scene(scene, window, Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
    });
    install_fixture_text_editor_focus(window, target.window_generation, node, &target.host_id);
    let mut runtime = semio_framework_os_renderer_wgpu_frame_actions::AppInteractionState {
        shell: semio_framework_os_renderer_wgpu_shell_state::ShellState::new(Vec::new(), "text-editor-runtime-space".into(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native),
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
        let mut key = Box::pin(runtime.handle_key(semio_framework_ui_wgpu_input::KeyAction::Space(true), Default::default()));
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
            let mut key = Box::pin(runtime.handle_key(semio_framework_ui_wgpu_input::KeyAction::Space(pressed), Default::default()));
            assert!(key.as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())).is_ready());
        }
        assert_eq!(runtime.space_pressed, pressed);
        assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut runtime.input).is_empty());
    }
    semio_framework_os_renderer_wgpu_engine_attach::retire_engine_surface_fixture(&target.host_id);
    assert!(request_ui_document_close(window));
    for _ in 0..262_144 {
        if !ui_document_close_pending() {
            break;
        }
        assert!(caller.turn(|cx|close_ui_document_one(cx)).expect("original fixture caller turn"));
    }
    assert!(!ui_document_close_pending());
    caller.finish();
}

#[test]
fn focused_text_editor_clipboard_composition_and_accessibility_share_the_accepted_host() {
    let _serialized = semio_framework_os_renderer_wgpu_engine_registry::engine_surface_law_guard();
    let law: Value = serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🔨️modules/✍️editor/🧪️tests/🧫️fixtures/⌨️text-input/🔣️.json")).unwrap();
    let paste = law["sequences"].as_array().unwrap().iter().find(|law| law["id"] == "a-paste-replaces-the-selection").unwrap();
    let window = "text-editor-clipboard-ime-accessibility";
    let UiNode::ComponentScene(mut scene) = component_scene_ui(window, semio_framework_ui_declaration::ui::SurfaceKind::TextEditor) else { unreachable!() };
    scene.text_editor = Some(semio_framework_ui_scene_views::TextEditorScene::base(paste["text"].as_str().unwrap().into(), None, Some(serde_json::json!({ "start": paste["selection"][0], "end": paste["selection"][1] }).to_string())));
    let node = seed_scene_window_with(window, UiNode::ComponentScene(scene));
    let target = retained_scene_target(window, node).unwrap();
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let UiNode::ComponentScene(scene) = &engine.tree(window).unwrap().node(node).unwrap().spec.0 else { unreachable!() };
        assert!(semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::sync_engine_scene(scene, window, Rect::new(10.0, 20.0, 480.0, 320.0), &Theme::default()));
    });
    install_fixture_text_editor_focus(window, target.window_generation, node, &target.host_id);
    MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow_mut().clear());
    let chord = semio_framework_ui_wgpu_input::PointerModifiers { ctrl: true, ..Default::default() };
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();

    assert!(apply_focused_text_editor_key(&semio_framework_ui_wgpu_input::KeyAction::Char("c".into()), &chord, &mut input));
    assert_eq!(MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow().clone()), ["b"]);
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "copy publishes no guest action");

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
    assert!(dispatch_accessibility_event(window, target.window_generation, textbox.node_id, &textbox.key, semio_framework_ui_wgpu_events::AccessibilityUiEvent::Value("日本".into()), &mut input).is_some());
    let actions = collect_text_editor_actions_accepted(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"]);
    assert_eq!(serde_json::to_value(actions[0].args.as_ref().unwrap()).unwrap()["text"], "日本");

    assert!(apply_focused_text_editor_key(&semio_framework_ui_wgpu_input::KeyAction::Char("a".into()), &chord, &mut input));
    let actions = collect_text_editor_actions_accepted(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"]);
    assert_eq!(actions[0].controller_id, "ctrl");
    let commit = serde_json::to_value(actions[0].args.as_ref().unwrap()).unwrap();
    assert_eq!(commit, serde_json::json!({ "surfaceId": target.surface_id, "typing": target.surface_id, "typingCommit": "selectionJump" }));
    assert!(commit.get("text").is_none());
    assert_eq!(actions[1].controller_id, "ctrl");
    assert_eq!(serde_json::to_value(actions[1].args.as_ref().unwrap()).unwrap(), serde_json::json!({ "surfaceId": target.surface_id, "start": 0, "end": 6 }));
    assert!(apply_focused_text_editor_key(&semio_framework_ui_wgpu_input::KeyAction::Char("x".into()), &chord, &mut input));
    assert_eq!(MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow().clone()), ["b", "日本"]);
    let actions = collect_text_editor_actions_accepted(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"]);
    assert_eq!(serde_json::to_value(actions[0].args.as_ref().unwrap()).unwrap()["text"], "");

    MOCK_CLIPBOARD_READ.with(|cell| *cell.borrow_mut() = Some("clipboard".into()));
    assert!(apply_focused_text_editor_key(&semio_framework_ui_wgpu_input::KeyAction::Char("v".into()), &chord, &mut input));
    let actions = collect_text_editor_actions_accepted(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"]);
    assert_eq!(serde_json::to_value(actions[0].args.as_ref().unwrap()).unwrap()["text"], "clipboard");

    assert!(start_focused_text_editor_stream(701, 8).unwrap());
    assert!(push_focused_text_editor_stream(701, "retired").unwrap());
    FOCUSED_TEXT_EDITOR.with(|cell| *cell.borrow_mut() = None);
    assert!(!commit_focused_text_editor_stream(701, &mut input), "a delayed stream cannot edit after focus retires");
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
}
//#endregion 🔖️SceneCommandTests
