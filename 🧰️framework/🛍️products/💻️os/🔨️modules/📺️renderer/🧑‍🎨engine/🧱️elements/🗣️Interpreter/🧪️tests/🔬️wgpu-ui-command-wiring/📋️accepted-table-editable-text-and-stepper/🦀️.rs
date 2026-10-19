/// 📋️ Accepted table editable text and stepper: the table scene nodes, enter and escape on editable text cells, the accessibility value echo and the stepper navigation keys.

//#region 🔖️SceneCommandTests

fn table_stepper_scene_node(value: f64) -> UiNode {
    let mut node = component_scene_ui("table-stepper-focus", semio_framework_ui_declaration::ui::SurfaceKind::Table);
    let UiNode::ComponentScene(scene) = &mut node else { unreachable!() };
    scene.controller_id = "sourcing-curation".into();
    scene.table = Some(semio_framework_ui_scene_views::TableScene::base(
        serde_json::json!([{ "id": "curated", "label": "Curated" }]).to_string(),
        serde_json::json!([{ "id": "beam-glulam-gl24h", "curated": { "kind": "stepper", "value": value, "min": 0.0, "max": 5.0, "step": 0.5, "action": { "controllerId": "sourcing-curation", "action": "curationSetCount", "args": { "objectId": "beam-glulam-gl24h" } } } }]).to_string(),
    ));
    node
}

fn table_editable_text_scene_node_with_value(value: &str) -> UiNode {
    let mut node = component_scene_ui("table-editable-text-focus", semio_framework_ui_declaration::ui::SurfaceKind::Table);
    let UiNode::ComponentScene(scene) = &mut node else { unreachable!() };
    scene.controller_id = "csv-editor".into();
    scene.table = Some(semio_framework_ui_scene_views::TableScene::base(
        serde_json::json!([{ "id": "value", "label": "Value" }]).to_string(),
        serde_json::json!([{ "id": "row-2", "value": { "kind": "editableText", "value": value, "action": { "controllerId": "csv-editor", "action": "set-cell", "args": { "row": 2, "column": 1 } } } }]).to_string(),
    ));
    node
}

/// 📤️ Every action an input published: the inline queue AND the retained (paged) publication an editable value takes.
fn collect_published_actions(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(input);
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
    let theme = semio_framework_ui_theme::Theme::default();
    let y = theme.control_height * 1.33 + theme.control_height * 0.5;
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let focus = |input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>| {
        apply_scene_ui_command(
            window_id,
            node,
            semio_framework_ui_declaration::ui::SurfaceKind::Table,
            rect,
            &semio_framework_ui_wgpu_events::UiEvent::PointerDown { x: 100.0, y, button: semio_framework_ui_wgpu_events::PointerButton::Primary, modifiers: Default::default() },
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
    assert!(apply_focused_table_editable_text_key(&semio_framework_ui_wgpu_input::KeyAction::Enter, &Default::default(), &mut input));
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
    assert!(apply_focused_table_editable_text_key(&semio_framework_ui_wgpu_input::KeyAction::Escape, &Default::default(), &mut input));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    assert!(input.focused_id.is_none());
}

#[test]
fn accepted_accessibility_table_value_echo_blurs_while_refusal_preserves_the_draft() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../📊️Table/🧪️tests/🧫️fixtures/✏️editable-text/🔣️.json")).expect("editable table fixture");
    let acceptance = &fixture["acceptance"];
    let base = acceptance["base"].as_str().unwrap();
    let draft = acceptance["draft"].as_str().unwrap();
    let awaiting_echo = acceptance["awaitingEcho"].as_str().unwrap();
    let target = || semio_framework_os_renderer_wgpu_scene_state::TableEditableTextFocusTarget { row_id: "row-2".into(), column_id: "value".into(), value: base.into() };

    let pending_window = "pending-accessibility-table-value";
    let pending_node = seed_scene_window_with(pending_window, table_editable_text_scene_node_with_value(base));
    accept_table_editable_text_cells(pending_window, pending_node, 931);
    let pending_host = retained_scene_host_id(pending_window, pending_node);
    let pending_generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(pending_window)).unwrap();
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(pending_window);
    publish_accessibility_visible_documents();
    let textbox = published_accessibility_nodes_for_test(pending_window).into_iter().find(|node| node.role == "textbox").expect("editable table textbox");
    assert!(dispatch_accessibility_event(pending_window, pending_generation, textbox.node_id, &textbox.key, semio_framework_ui_wgpu_events::AccessibilityUiEvent::Focus, &mut input).is_some());
    assert!(dispatch_accessibility_event(pending_window, pending_generation, textbox.node_id, &textbox.key, semio_framework_ui_wgpu_events::AccessibilityUiEvent::Value(draft.into()), &mut input).is_some());
    let actions = collect_published_actions(&mut input);
    let publication_count_before_echo = acceptance["publicationCountBeforeEcho"].as_u64().unwrap();
    assert_eq!(actions.len() as u64, publication_count_before_echo, "the accepted accessibility value publishes once");
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(pending_window);
    publish_accessibility_visible_documents();
    let pending_textbox = published_accessibility_nodes_for_test(pending_window).into_iter().find(|node| node.key == textbox.key).expect("pending editable table textbox");
    assert_eq!(pending_textbox.busy, acceptance["busyBeforeEcho"].as_bool().unwrap());
    assert_eq!(pending_textbox.value_text.as_deref(), Some(draft));
    assert!(dispatch_accessibility_event(pending_window, pending_generation, textbox.node_id, &textbox.key, semio_framework_ui_wgpu_events::AccessibilityUiEvent::Blur, &mut input).is_some());
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
    assert!(dispatch_accessibility_event(pending_window, echoed_generation, echoed_textbox.node_id, &echoed_textbox.key, semio_framework_ui_wgpu_events::AccessibilityUiEvent::Blur, &mut input).is_some());
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "the authoritative scene echo cannot publish the accepted value again");
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
    let other = semio_framework_os_renderer_wgpu_scene_state::TableEditableTextFocusTarget { row_id: "other-row".into(), column_id: "value".into(), value: "other".into() };
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
    let theme = semio_framework_ui_theme::Theme::default();
    let y = theme.control_height * 1.33 + theme.control_height * 0.5;
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    apply_scene_ui_command(
        window_id,
        node,
        semio_framework_ui_declaration::ui::SurfaceKind::Table,
        rect,
        &semio_framework_ui_wgpu_events::UiEvent::PointerDown { x: 100.0, y, button: semio_framework_ui_wgpu_events::PointerButton::Primary, modifiers: Default::default() },
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
    semio_framework_os_renderer_wgpu_scene_feeds::stage_table_stepper_accessibility_cells(&scene.host_id, semio_framework_os_renderer_wgpu_scene_tables::table_stepper_accessibility_cells(&scene, rect, semio_framework_ui_wgpu_layout::chrome::UiDriverDrag::Handle));
    semio_framework_os_renderer_wgpu_scene_feeds::seal_table_stepper_accessibility_candidates(800);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_table_stepper_accessibility_candidates(800);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    let spinbutton = published_accessibility_nodes_for_test(window_id).into_iter().find(|node| node.role == "spinbutton").expect("accepted Table stepper spinbutton");
    assert_eq!(spinbutton.label.as_deref(), Some("Curated"));
    assert_eq!((spinbutton.value_min, spinbutton.value_now, spinbutton.value_max, spinbutton.value_text.as_deref()), (Some(0.0), Some(2.0), Some(5.0), Some("2")));
    assert!(spinbutton.focusable && spinbutton.actionable && spinbutton.focused && spinbutton.rect.is_some());
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    assert!(dispatch_accessibility_event(window_id, generation, spinbutton.node_id, &spinbutton.key, semio_framework_ui_wgpu_events::AccessibilityUiEvent::Blur, &mut input).is_some());
    assert!(FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow().is_none()));
    assert!(dispatch_accessibility_event(window_id, generation, spinbutton.node_id, &spinbutton.key, semio_framework_ui_wgpu_events::AccessibilityUiEvent::Focus, &mut input).is_some());

    assert!(apply_focused_table_stepper_key(&semio_framework_ui_wgpu_input::KeyAction::PageUp, &mut input));
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    let args = actions[0].args.as_ref().expect("stepper args");
    assert_eq!(args.get("objectId").and_then(semio_framework::DslValue::as_str), Some("beam-glulam-gl24h"));
    assert_eq!(args.get("delta").and_then(semio_framework::DslValue::as_f64), Some(3.0));
    assert!(!apply_focused_table_stepper_key(&semio_framework_ui_wgpu_input::KeyAction::Enter, &mut input), "an unrelated key remains available to the generic shell route");

    FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow_mut().as_mut().expect("focused Table stepper").row_id = "retired-row".into());
    assert!(!apply_focused_table_stepper_key(&semio_framework_ui_wgpu_input::KeyAction::ArrowUp, &mut input));
    assert!(FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow().is_none()));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
}
//#endregion 🔖️SceneCommandTests
