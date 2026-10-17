/// 🪄️ Presented focus rebases and ink intent: focus rebasing only after the same host pixels are accepted, and the ink intent barrier, its successors, hidden windows, stale revisions and progress owner.

//#region 🔖️SceneCommandTests

#[test]
fn presented_editor_text_focus_rebases_only_after_same_host_pixels_are_accepted() {
    let window_id = "presented-text-focus-rebase";
    let kind = semio_framework_ui_declaration::ui::SurfaceKind::TextEditor;
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
    let kind = semio_framework_ui_declaration::ui::SurfaceKind::Table;
    assert!(publish_focus_rebase_document(window_id, 1, 521, kind, &[2]).is_none());
    drive_focus_rebase_reconcile(window_id, 1);
    reconcile_focus_rebase_document(window_id, 2, kind, &[2, 3]);
    let old = stage_new_focus_rebase_document(window_id, 3, 522, kind, &[2, 4]);
    assert!(acknowledge_presented_input(522));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let host_id = retained_scene_host_id(window_id, old);
    focus_table_stepper(window_id, generation, old, &host_id, semio_framework_os_renderer_wgpu_scene_state::TableStepperFocusTarget { row_id: "r1".into(), column_id: "count".into() });

    let successor = stage_focus_rebase_document(window_id, 4, 523, kind, &[2, 3, 4]);
    assert_ne!(old, successor);
    assert!(FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.node == old && focus.host_id == host_id)), "candidate construction cannot mutate accepted focus");
    assert!(acknowledge_presented_input(523));
    assert!(FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.node == successor && focus.host_id == host_id)), "accepted pixels rebase the same Table stepper focus");

    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(apply_focused_table_stepper_key(&ui_wgpu::wgpu::KeyAction::End, &mut input));
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    let args = actions[0].args.as_ref().expect("stepper args");
    assert_eq!(args.get("objectId").and_then(semio_framework::DslValue::as_str), Some("r1"));
    assert_eq!(args.get("delta").and_then(semio_framework::DslValue::as_f64), Some(3.0));
    FOCUSED_TABLE_STEPPER.with(|cell| *cell.borrow_mut() = None);
}

#[test]
fn presented_editor_ink_focus_rebases_only_after_same_host_pixels_are_accepted() {
    let window_id = "presented-ink-focus-rebase";
    let kind = semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas;
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
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let source=caller.turn(|cx|ui_wgpu::wgpu::ClipboardIntentSource{operation:cx.operation(),generation:cx.generation(),epoch:cx.original_source_epoch().expect("fixture issued input epoch")});
    let window_id = "presented-ink-clipboard-rebase";
    let kind = semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas;
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
    let pending = reserve_pending_native_ink_clipboard(address,source).expect("native clipboard owner reserves");

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
    caller.finish();
}

#[test]
fn presented_ink_intent_finishes_before_an_unchanged_same_host_sibling_refresh_is_accepted() {
    while !close_scene_interaction_step() {}
    let contract: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
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
    let admitted = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(admitted.iter().filter(|action| action.action == "inkApplyEvents").count(), packet["admittedActions"].as_u64().unwrap() as usize);
    cancel_scene_pointer(pointer, &mut input);
    assert!(acknowledge_presented_input(602));

    let successor_pointer = ui_render::PointerId(62);
    begin_pending_ink_intent(window_id, successor, successor_pointer, &mut input);
    drive_ink_scene_terminal(&mut input);
    let successor_actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(successor_actions.iter().filter(|action| action.action == "inkApplyEvents").count(), packet["successorActions"][0].as_u64().unwrap() as usize);
    cancel_scene_pointer(successor_pointer, &mut input);
    retire_presented_document(window_id);
}

#[test]
fn presented_ink_intent_uses_old_authored_content_before_a_changed_same_host_successor_starts() {
    while !close_scene_interaction_step() {}
    let contract: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
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
    let admitted = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(admitted.iter().filter(|action| action.action == "inkApplyEvents").count(), packet["admittedActions"].as_u64().unwrap() as usize);
    cancel_scene_pointer(pointer, &mut input);
    assert!(acknowledge_presented_input(612));

    let successor_pointer = ui_render::PointerId(64);
    begin_pending_ink_intent(window_id, successor, successor_pointer, &mut input);
    drive_ink_scene_terminal(&mut input);
    let successor_actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(successor_actions.iter().filter(|action| action.action == "inkApplyEvents").count(), packet["successorActions"][1].as_u64().unwrap() as usize);
    cancel_scene_pointer(successor_pointer, &mut input);
    retire_presented_document(window_id);
}

#[test]
fn presented_ink_intent_in_a_hidden_window_does_not_block_an_unrelated_candidate() {
    while !close_scene_interaction_step() {}
    let contract: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
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
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    cancel_scene_pointer(pointer, &mut input);
    retire_presented_document(hidden_window);
    retire_presented_document(participant);
}

#[test]
fn presented_ink_intent_from_a_stale_revision_does_not_block_the_current_candidate() {
    while !close_scene_interaction_step() {}
    let contract: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
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
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "the stale intent closes instead of dispatching against successor content");
    cancel_scene_pointer(pointer, &mut input);
    retire_presented_document(window_id);
}

#[test]
fn presented_ink_intent_barrier_has_an_independent_bounded_progress_owner() {
    while !close_scene_interaction_step() {}
    let contract: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
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
    assert_eq!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).iter().filter(|action| action.action == "inkApplyEvents").count(), 1);
    cancel_scene_pointer(pointer, &mut input);
    retire_presented_document(window_id);
}
//#endregion 🔖️SceneCommandTests
