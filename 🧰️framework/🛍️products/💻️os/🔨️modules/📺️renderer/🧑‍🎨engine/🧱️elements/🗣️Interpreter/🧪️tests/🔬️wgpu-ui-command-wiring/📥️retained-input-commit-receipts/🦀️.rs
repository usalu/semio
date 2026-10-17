/// 📥️ Retained input commit receipts: the typed input record, the generation bound completion receipt and the shared draft disposition law.

/// 🧾️ Publishes an owned typed input record for the shared retained-draft corpus.
fn retained_input_document(window: &str, kind: &str, value: serde_json::Value) -> semio_framework_ui_retained_tree::tree::UiDocumentTree {
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
    let mut commands = UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(window, ui_wgpu::wgpu::UiEvent::KeyDown { key: "Enter".into(), modifiers: Default::default() }));
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_ui_commands(&mut commands, None, &mut input);
    let queued = input.take_action_step().unwrap().expect("Enter submits one input commit").into_envelope().unwrap();
    assert!(queued.receipt.is_some(), "focused input retains a receipt until dispatch settles");
    let receipt = queued.receipt.unwrap();
    settle_retained_input_receipt(semio_framework_ui_action::ActionQueueReceipt { source: semio_framework_ui_action::ActionQueueReceiptSource::CanvasTextEditor, ..receipt }, true);
    assert!(RETAINED_INPUT_RECEIPTS.with(|cell| cell.borrow().slots.iter().any(Option::is_some)), "equal tokens of other receipt sources cannot consume an input completion");
    semio_framework_os_renderer_wgpu_frame_actions::settle_renderer_action_receipt(receipt, semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Cancelled);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🚦️input-draft-disposition/🔣️.json")).unwrap();
    let capacity = corpus["receiptCapacity"].as_u64().unwrap() as usize;
    assert_eq!(capacity, semio_framework_ui_action::ACTION_QUEUE_ITEM_CAPACITY);
    let intent = commands.iter().find_map(|command| match command { ui_wgpu::wgpu::UiCommand::App { intent, .. } => Some(intent), _ => None }).unwrap();
    let mut held: Vec<_> = (0..capacity).map(|_| reserve_retained_input_receipt(window, intent).unwrap()).collect();
    assert!(matches!(reserve_retained_input_receipt(window, intent), Err(semio_framework_ui_action::BoundedActionFault::ItemCredits)));
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
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🚦️input-draft-disposition/🔣️.json" )).unwrap();
    for (index, law) in corpus["cases"].as_array().unwrap().iter().enumerate() {
        let window = format!("draft-disposition-{index}");
        let document = retained_input_document(&window, law["kind"].as_str().unwrap(), law["base"].clone());
        publish_presented_document(&window, 1, 9900 + index as u64, "fixture", document);
        let keys = |key: &str, ctrl: bool| ui_wgpu::wgpu::UiEvent::KeyDown { key: key.into(), modifiers: ui_wgpu::wgpu::EventModifiers { ctrl, ..Default::default() } };
        let focused = UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(&window, keys("Tab", false))).into_iter().find_map(|command| match command { ui_wgpu::wgpu::UiCommand::FocusChanged { node: Some(node), .. } => Some(node), _ => None }).unwrap();
        UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(&window, keys("a", true)));
        UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(&window, ui_wgpu::wgpu::UiEvent::TextInput { text: law["submitted"].as_str().unwrap().into() }));
        let mut commands = UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(&window, keys("Enter", law["kind"] == "longText")));
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        apply_ui_commands(&mut commands, None, &mut input);
        let receipt = input.take_action_step().unwrap().expect("commit submits").into_envelope().unwrap().receipt.expect("addressed commit owns one completion");
        assert_eq!(receipt.source, semio_framework_ui_action::ActionQueueReceiptSource::RetainedInput);
        if law["draft"] != law["submitted"] {
            UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(&window, keys("a", true)));
            UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(&window, ui_wgpu::wgpu::UiEvent::TextInput { text: law["draft"].as_str().unwrap().into() }));
        }
        let outcome = if law["cancelled"] == true { semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Cancelled } else if law["disposition"] == "discard" { semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Refused("timeTravel.frozen: draft refused") } else { semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Refused("validation.recoverable: repair the draft") };
        semio_framework_os_renderer_wgpu_frame_actions::settle_renderer_action_receipt(receipt, outcome);
        let text = UI_ENGINE.with(|cell| cell.borrow().tree(&window).unwrap().node(focused).unwrap().state.edit.as_ref().unwrap().text.clone());
        assert_eq!(text, law["expected"].as_str().unwrap(), "{}", law["name"]);
        semio_framework_os_renderer_wgpu_frame_actions::settle_renderer_action_receipt(receipt, semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Refused("timeTravel.frozen: duplicate completion"));
        let repeated = UI_ENGINE.with(|cell| cell.borrow().tree(&window).unwrap().node(focused).unwrap().state.edit.as_ref().unwrap().text.clone());
        assert_eq!(repeated, text, "a settled receipt cannot alter a later buffer");
        retire_presented_document(&window);
    }
    assert!(RETAINED_INPUT_RECEIPTS.with(|cell| cell.borrow().slots.iter().all(Option::is_none)));
}
