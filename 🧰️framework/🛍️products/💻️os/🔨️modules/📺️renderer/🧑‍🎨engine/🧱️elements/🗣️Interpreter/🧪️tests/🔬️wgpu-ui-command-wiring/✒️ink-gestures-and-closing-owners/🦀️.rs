/// ✒️ Ink gestures and closing owners: pointer cancel, streamed note gestures, the ink canvas text and table editing lifecycle, stale scene revisions and the closing window and editor owner laws.

//#region 🔖️SceneCommandTests

#[test]
fn ink_pointer_cancel_retires_only_the_exact_in_progress_owner_without_publishing() {
    let cancellation: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🛑️scene-pointer-cancellation/🔣️.json")).expect("shared scene cancellation fixture");
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
        semio_framework_os_renderer_wgpu_scene_ink::InkInteractionJob::new(1, 0, Some(pointer), &zero_generation_scene, semio_framework_os_renderer_wgpu_scene_ink::InkInteractionEvent::PointerDown { x: 64.0, y: 64.0, button: 0, shift: false }),
        Err(semio_framework_ui_action::BoundedActionFault::Structure)
    ));
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let down = ui_wgpu::wgpu::UiCommand::Scene {
        window_id: window_id.into(),
        node,
        surface_id: surface_id.clone(),
        kind: semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas,
        rect,
        event: ui_wgpu::wgpu::UiEvent::PointerDown { x: 64.0, y: 64.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
    };
    apply_ui_commands(&mut vec![down.clone()], Some(pointer), &mut input);
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
    assert_eq!(cancelled[0].kind, semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas);
    drive_ink_scene_terminal(&mut input);
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "a press that never streamed cannot synthesize PointerUp, publish Ink events or abort");

    let next_pointer = ui_render::PointerId(43);
    apply_ui_commands(&mut vec![down], Some(next_pointer), &mut input);
    assert!(SCENE_POINTER_OWNERS.with(|cell| cell.borrow().slots.iter().any(|owner| owner.pointer_id == next_pointer)), "the next Down is accepted after exact cancellation");
    cancel_scene_pointer(next_pointer, &mut input);
    drive_ink_scene_terminal(&mut input);
    retire_presented_document(window_id);
}

#[test]
fn a_streamed_note_ink_gesture_aborts_on_physical_cancel_while_stale_continuation_is_inert() {
    let surface_behavior: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🎬️surface-behavior/🔣️.json")).expect("shared app surface behavior");
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
    let command = |event| ui_wgpu::wgpu::UiCommand::Scene { window_id: window_id.into(), node, surface_id: surface_id.clone(), kind: semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas, rect, event };

    apply_ui_commands(&mut vec![command(ui_wgpu::wgpu::UiEvent::PointerDown { x: down_point.0, y: down_point.1, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() })], Some(pointer), &mut input);
    drive_ink_scene_terminal(&mut input);
    let begin = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(begin.len(), 1);
    assert_eq!(begin[0].action, "inkApplyEvents");
    assert_eq!(begin[0].args.as_ref().and_then(|args| args.get("phase")).and_then(semio_framework::DslValue::as_str), behavior["acceptedBeforeCancel"][0]["phase"].as_str());

    apply_ui_commands(&mut vec![command(ui_wgpu::wgpu::UiEvent::PointerMove { x: move_point.0, y: move_point.1, modifiers: Default::default() })], Some(pointer), &mut input);
    drive_ink_scene_terminal(&mut input);
    let live = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(live.len(), 1);
    assert_eq!(live[0].args.as_ref().and_then(|args| args.get("phase")).and_then(semio_framework::DslValue::as_str), behavior["acceptedBeforeCancel"][1]["phase"].as_str());

    let cancelled = cancel_scene_pointer(pointer, &mut input);
    assert_eq!(cancelled.len(), 1);
    assert_eq!(cancelled[0].host_id, host_id);
    let aborted = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(aborted.len(), 1, "the streamed gesture publishes exactly its abort: {aborted:?}");
    assert_eq!(aborted[0].action, behavior["publishedOnCancel"][0]["action"].as_str().unwrap());
    assert_eq!(aborted[0].args.as_ref().and_then(|args| args.get("phase")).and_then(semio_framework::DslValue::as_str), behavior["publishedOnCancel"][0]["phase"].as_str());
    assert_eq!(aborted[0].args.as_ref().and_then(|args| args.get("reason")).and_then(semio_framework::DslValue::as_str), Some("captureLost"));
    assert!(captured_scene_pointer(pointer).is_none(), "the old physical pointer cannot route a later move or up");
    assert!(semio_framework_os_renderer_wgpu_scene_ink::ink_pointer_state_is_clear(&host_id), "the retained preview is cleared and the plugin drops the gesture with zero trace");
    assert_eq!(behavior["blockedAfterCancel"], serde_json::json!(["inkApplyEvents:stream", "inkApplyEvents:commit"]));

    let successor = ui_render::PointerId(72);
    apply_ui_commands(&mut vec![command(ui_wgpu::wgpu::UiEvent::PointerDown { x: down_point.0 + 24.0, y: down_point.1 + 24.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() })], Some(successor), &mut input);
    drive_ink_scene_terminal(&mut input);
    let fresh = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
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
    let queued = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(input);
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
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert!(apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Char(law["gestures"]["text"]["replacement"].as_str().unwrap().into()), &Default::default(), &mut input));
    assert!(blur_focused_ink_editor_at(law["gestures"]["blur"]["x"].as_f64().unwrap() as f32, law["gestures"]["blur"]["y"].as_f64().unwrap() as f32, &mut input));
    assert_atomic_ink_update(&mut input, &law["expected"]["textEvent"]);

    open_ink_editor(window_id, node, &surface_id, rect, &law["gestures"]["table"], &mut input);
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
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
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert!(apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Char("cancelled".into()), &Default::default(), &mut input));
    assert!(apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Escape, &Default::default(), &mut input));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).iter().all(|action| action.action != "inkApplyEvents"), "Escape cancels without publishing an update");

    open_ink_editor(window_id, node, &surface_id, rect, &law["gestures"]["text"], &mut input);
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).expect("window generation");
    assert!(request_ui_document_close(window_id));
    assert!(!apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Char("closing".into()), &Default::default(), &mut input), "a closing document immediately fences and clears its focused editor");
    assert!(input.focused_id.is_none());
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    drain_presented_document_close(window_id);
    assert!(publish_new_focus_rebase_document(window_id, 2, 453, semio_framework_ui_declaration::ui::SurfaceKind::TextEditor, &[4]).is_some());
    assert!(UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).is_some_and(|next| next > generation));
    assert!(!apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Char("stale".into()), &Default::default(), &mut input), "a prior window generation cannot mutate the reopened document");
    assert!(input.focused_id.is_none());
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    retire_presented_document(window_id);
}

#[test]
fn stale_scene_revision_retires_without_mutation_or_action_publication() {
    while !close_scene_interaction_step() {}
    let window_id = "apply-ui-commands-stale-scene-revision";
    let node = seed_scene_window(window_id, "original", semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_scene_ui_command(
        window_id,
        node,
        semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d,
        rect,
        &ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        Some(ui_render::PointerId(1)),
        &mut input,
    );
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("root", None, vec![component_scene_ui("replacement", semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d)])));

    assert!(drive_scene_interaction_step(&mut input));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    assert!(scene_interaction_terminal_is_empty());
}

#[test]
fn closing_window_fences_queued_scene_intent_before_tree_retirement() {
    let window_id = "closing-window-queued-canvas";
    let node = seed_scene_window(window_id, "closing-canvas", semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_scene_ui_command(
        window_id,
        node,
        semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d,
        rect,
        &ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        Some(ui_render::PointerId(1)),
        &mut input,
    );
    assert!(!scene_interaction_terminal_is_empty());
    assert!(request_ui_document_close(window_id));
    assert!(UI_ENGINE.with(|cell| cell.borrow().tree(window_id).is_some()));
    assert!(drive_scene_interaction_step(&mut input));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "queued input cannot activate the old scene after its window closes");
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
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert!(apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Char("uncommitted".into()), &Default::default(), &mut input));
    assert!(request_ui_document_close(window_id));
    assert!(!apply_focused_ink_editor_key(&ui_wgpu::wgpu::KeyAction::Enter, &Default::default(), &mut input));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn closing_window_retires_an_unfinished_ink_clipboard_stream_before_reopening() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
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
        assert!(caller.turn(|cx|close_ui_document_one(cx)).expect("original fixture caller turn"));
    }
    assert!(!ui_document_close_pending());
    assert!(INK_CLIPBOARD_STREAMS.with(|cell| cell.borrow().iter().flatten().all(|stream| stream.address.window_id != window_id)));
    assert!(focused_ink_surface().is_none());
    seed_scene_window_with(window_id, ink_editing_scene(&law));
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(!commit_focused_ink_clipboard_stream(41, &mut input));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    caller.finish();
}

#[test]
fn closing_one_window_retires_only_its_ink_clipboard_owner() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
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
        assert!(caller.turn(|cx|close_ui_document_one(cx)).expect("original fixture caller turn"));
    }
    assert!(!ui_document_close_pending());
    INK_CLIPBOARD_STREAMS.with(|cell| {
        let slots = cell.borrow();
        assert!(slots.iter().flatten().all(|stream| stream.id != 142));
        assert!(slots.iter().flatten().any(|stream| stream.id == 143), "closing one document cannot consume a sibling host's stream");
    });
    assert!(abort_focused_ink_clipboard_stream(143));
    caller.finish();
}

#[test]
fn a_late_native_clipboard_callback_cannot_complete_a_reused_slot() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let source=caller.turn(|cx|ui_wgpu::wgpu::ClipboardIntentSource{operation:cx.operation(),generation:cx.generation(),epoch:cx.original_source_epoch().expect("fixture issued input epoch")});
    let mut arena = semio_framework_ui_arena::Arena::<()>::new();
    let node = arena.insert(());
    let address = |host_id: &str| InkClipboardAddress { window_id: "native-ink-clipboard-aba".into(), window_generation: 1, node, host_id: host_id.into(), rect: Rect::new(0.0, 0.0, 100.0, 100.0) };
    let stale = reserve_pending_native_ink_clipboard(address("old"),source).expect("old slot reserves");
    PENDING_NATIVE_INK_CLIPBOARD.with(|cell| cell.borrow_mut()[usize::from(stale.slot)].mounted = None);
    let successor_token = reserve_pending_native_ink_clipboard(address("successor"),source).expect("successor reuses the free slot");
    assert_eq!(stale.slot, successor_token.slot);
    assert_ne!(stale.generation, successor_token.generation);
    complete_pending_native_ink_clipboard(stale, Ok(Some(ui_wgpu::wgpu::ClipboardContent::Text("stale".into()))));
    PENDING_NATIVE_INK_CLIPBOARD.with(|cell| {
        let mut slots = cell.borrow_mut();
        let successor = slots[usize::from(successor_token.slot)].mounted.take().expect("successor slot remains mounted");
        assert_eq!(successor.address.host_id, "successor");
        assert!(successor.outcome.is_none(), "the old completion token cannot write through a reused slot index");
    });
    caller.finish();
}
//#endregion 🔖️SceneCommandTests
