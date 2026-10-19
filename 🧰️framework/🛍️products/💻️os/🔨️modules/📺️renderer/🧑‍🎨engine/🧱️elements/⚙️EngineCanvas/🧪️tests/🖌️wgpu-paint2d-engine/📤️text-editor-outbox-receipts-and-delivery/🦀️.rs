/// 📤️ Text editor outbox, receipts and delivery: credit pressure, typing run signals, ordered pages, the neutral receipt laws, scene echoes, the local echo ledger and the production key route.

#[test]
fn text_editor_outbox_preserves_local_echo_during_temporary_action_credit_pressure() {
    let _serialized = engine_surface_law_guard();
    let id = "text-editor-action-credits";
    drop_engine_surface(id);
    let scene = text_editor_scene(id, "alpha");
    assert!(sync_engine_scene(&scene, "editor-credit-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
    let mut input = InputState::<ActionDescriptor>::default();
    for _ in 0..semio_framework_ui_action::ACTION_QUEUE_ITEM_CAPACITY - 1 {
        input.publish_action("c", "a", 2, |_, _| Ok(())).unwrap();
    }
    assert_eq!(text_editor_apply_key_into(&scene, &KeyAction::Char("!".into()), &PointerModifiers::default(), &mut input), Ok(true));
    assert_eq!(drive_text_editor_outbox_step(&mut input), Ok(false));
    assert!(has_pending_text_editor_outbox());
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let host = map.get(id).unwrap().editor.as_ref().unwrap();
        assert_eq!((host.text(), host.anchor(), host.caret()), ("alpha!", 6, 6));
    });
    let _ = drain(&mut input);
    let actions = drain_editor_actions_accepted(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"]);
    assert!(has_pending_text_editor_outbox(), "the open typing run keeps the outbox driven until it ends");
    assert_eq!(text_editor_apply_key_into(&scene, &KeyAction::ArrowLeft, &PointerModifiers::default(), &mut input), Ok(true));
    let actions = drain_editor_actions_accepted(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"], "a pure caret move ends the typing run with ONE commit signal before its selection");
    assert_eq!(serde_json::to_value(actions[0].args.as_ref().unwrap()).unwrap(), json!({ "surfaceId": id, "typing": id, "typingCommit": "selectionJump" }));
    let selection = actions.last().unwrap();
    assert_eq!(serde_json::to_value(selection.args.as_ref().unwrap()).unwrap(), json!({ "surfaceId": id, "start": 5, "end": 5 }));
    assert!(!has_pending_text_editor_outbox(), "the ended run leaves nothing to drive");
    drop_engine_surface(id);
}

/// ⚖️ LAW (text-splice corpus `hostSignals`, shared with the React host): every host signal the wgpu editor owns ends its open
/// typing run with the corpus `typingCommit` reason — a pure caret move, the editor losing keyboard focus, the page hidden — as
/// ONE commit signal sent once the run's last keystroke left, and nothing is left to drive afterwards. The idle bound is the
/// tool-machine owner's constant.
#[test]
fn text_editor_host_signals_end_the_typing_run_with_the_corpus_reasons() {
    let _serialized = engine_surface_law_guard();
    let corpus: serde_json::Value = serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/✂️text-splice/🔣️.json")).expect("text-splice corpus");
    for row in corpus["hostSignals"].as_array().expect("host signals") {
        let (signal, commit) = (row["signal"].as_str().expect("signal"), row["commit"].as_str().expect("commit"));
        if signal == "idle" {
            assert_eq!((commit, semio_framework_ui_scene_views::TEXT_EDITOR_TYPING_IDLE_MS), ("idle", 750), "the idle bound ends the run as `idle`");
            continue;
        }
        let id = format!("text-editor-signal-{signal}");
        drop_engine_surface(&id);
        let scene = text_editor_scene(&id, "alpha");
        assert!(sync_engine_scene(&scene, "editor-signal-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
        let mut input = InputState::<ActionDescriptor>::default();
        assert_eq!(text_editor_apply_key_into(&scene, &KeyAction::Char("!".into()), &PointerModifiers::default(), &mut input), Ok(true));
        match signal {
            "caretMove" => {}
            "blur" => assert!(end_text_editor_typing(&id, TextEditorTypingEnd::Blur), "a keystroke in flight is a run the blur ends"),
            "hidden" => assert!(end_every_text_editor_typing(TextEditorTypingEnd::Hidden) >= 1, "the hidden page ends the run"),
            other => panic!("unknown host signal {other:?}"),
        }
        let delivered = drain_editor_actions_accepted(&mut input);
        assert_eq!(delivered.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"], "{signal}: the keystroke leaves before the run ends");
        if signal == "caretMove" {
            assert_eq!(text_editor_apply_key_into(&scene, &KeyAction::ArrowLeft, &PointerModifiers::default(), &mut input), Ok(true));
        }
        let ended = drain_editor_actions_accepted(&mut input);
        assert_eq!(serde_json::to_value(ended[0].args.as_ref().expect("commit args")).expect("args"), json!({ "surfaceId": id, "typing": id, "typingCommit": commit }), "{signal}");
        assert_eq!(ended.len(), if signal == "caretMove" { 2 } else { 1 }, "{signal}: ONE commit signal");
        assert!(drain_editor_actions_accepted(&mut input).is_empty() && !has_pending_text_editor_outbox(), "{signal}: the ended run leaves nothing to drive");
        assert!(!end_text_editor_typing(&id, TextEditorTypingEnd::Blur), "{signal}: no run is left to end");
        drop_engine_surface(&id);
    }
}

#[test]
fn text_editor_outbox_pages_a_large_document_into_one_ordered_edit_and_selection_pair() {
    let _serialized = engine_surface_law_guard();
    let id = "text-editor-large-outbox";
    drop_engine_surface(id);
    let scene = text_editor_scene(id, "seed");
    assert!(sync_engine_scene(&scene, "editor-large-outbox-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
    let expected = format!("{}🚀", "a".repeat(65_535));
    let mut input = InputState::<ActionDescriptor>::default();
    assert_eq!(text_editor_replace_all_into(&scene, &expected, &mut input), Ok(true));
    assert_eq!(drive_text_editor_outbox_step(&mut input), Ok(true));
    assert!(drain(&mut input).is_empty(), "a large document never enters the bounded string ring");
    let actions = loop {
        match drive_text_editor_retained_action_step().expect("retained text page") {
            TextEditorRetainedActionStep::Pending => {}
            TextEditorRetainedActionStep::Ready(actions) => break actions,
            TextEditorRetainedActionStep::Idle => panic!("publication disappeared"),
        }
    };
    assert_eq!(actions.first.descriptor.action, "textEdit");
    assert_eq!(actions.first.descriptor.args.as_ref().and_then(|args| args.get("text")).and_then(semio_framework::DslValue::as_str), Some(expected.as_str()));
    assert_eq!(serde_json::to_value(actions.first.descriptor.args.as_ref().unwrap()).unwrap()["text"], expected);
    let selection = actions.second.expect("ordered selection");
    assert_eq!(selection.descriptor.action, "textSelect");
    assert_eq!(serde_json::to_value(selection.descriptor.args.as_ref().unwrap()).unwrap(), json!({ "surfaceId": id, "start": expected.len(), "end": expected.len() }));
    settle_text_editor_action_receipt(actions.first.receipt.unwrap(), TextEditorActionOutcome::Accepted);
    settle_text_editor_action_receipt(selection.receipt.unwrap(), TextEditorActionOutcome::Accepted);
    assert!(!has_pending_text_editor_outbox());
    drop_engine_surface(id);
}

#[test]
fn text_editor_receipts_match_the_neutral_first_latest_refusal_and_read_only_laws() {
    let _serialized = engine_surface_law_guard();
    let fixture: Value = serde_json::from_str(fixture_text!("../../../../../🧱️elements/✏️TextEditor/🧪️tests/🧫️fixtures/📮️delivery/🔣️.json")).expect("neutral delivery fixture");
    for law in fixture["cases"].as_array().expect("delivery cases") {
        let id = law["id"].as_str().unwrap();
        drop_engine_surface(id);
        let scene = text_editor_scene(id, law["initial"].as_str().unwrap());
        assert!(sync_engine_scene(&scene, "editor-delivery-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
        let mut input = InputState::<ActionDescriptor>::default();
        let typed = law["typed"].as_array().unwrap();
        assert!(text_editor_apply_key_into(&scene, &KeyAction::Char(typed[0].as_str().unwrap().into()), &PointerModifiers::default(), &mut input).unwrap());
        assert!(drive_text_editor_outbox_step(&mut input).unwrap());
        let first_edit = input.take_action_step().unwrap().unwrap().into_envelope().unwrap();
        let mut dispatched = vec![format!("textEdit:{}", serde_json::to_value(first_edit.descriptor.args.as_ref().unwrap()).unwrap()["text"].as_str().unwrap())];
        assert_eq!(dispatched, law["expected"][0].as_array().unwrap().iter().map(|row| row.as_str().unwrap().to_owned()).collect::<Vec<_>>(), "{id}: leading edit");
        for value in &typed[1..] {
            assert!(text_editor_apply_key_into(&scene, &KeyAction::Char(value.as_str().unwrap().into()), &PointerModifiers::default(), &mut input).unwrap());
        }
        let after_new_owner = law["typedAfterNewOwner"].as_array().cloned().unwrap_or_default();
        if !after_new_owner.is_empty() {
            assert!(sync_engine_scene(&text_editor_scene(id, law["initial"].as_str().unwrap()), "editor-delivery-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()), "{id}: the same host re-renders mid-flight with its guest's stale buffer");
            for value in &after_new_owner {
                assert!(text_editor_apply_key_into(&scene, &KeyAction::Char(value.as_str().unwrap().into()), &PointerModifiers::default(), &mut input).unwrap());
            }
        }
        let first_receipt = first_edit.receipt.unwrap();
        let accepted = law["outcome"] == "accepted";
        settle_text_editor_action_receipt(first_receipt, if accepted { TextEditorActionOutcome::Accepted } else { TextEditorActionOutcome::Refused(law["outcome"].as_str().unwrap()) });
        let first_selection = input.take_action_step().unwrap().unwrap().into_envelope().unwrap();
        if accepted {
            let args = serde_json::to_value(first_selection.descriptor.args.as_ref().unwrap()).unwrap();
            dispatched.push(format!("textSelect:{}:{}", args["start"], args["end"]));
            settle_text_editor_action_receipt(first_selection.receipt.unwrap(), TextEditorActionOutcome::Accepted);
        } else {
            settle_text_editor_action_receipt(first_selection.receipt.unwrap(), TextEditorActionOutcome::Cancelled);
        }
        if drive_text_editor_outbox_step(&mut input).unwrap() {
            let latest_edit = input.take_action_step().unwrap().unwrap().into_envelope().unwrap();
            let args = serde_json::to_value(latest_edit.descriptor.args.as_ref().unwrap()).unwrap();
            dispatched.push(format!("textEdit:{}", args["text"].as_str().unwrap()));
            assert_eq!(dispatched, law["expected"][1].as_array().unwrap().iter().map(|row| row.as_str().unwrap().to_owned()).collect::<Vec<_>>(), "{id}: latest edit");
            settle_text_editor_action_receipt(latest_edit.receipt.unwrap(), TextEditorActionOutcome::Accepted);
            let latest_selection = input.take_action_step().unwrap().unwrap().into_envelope().unwrap();
            let args = serde_json::to_value(latest_selection.descriptor.args.as_ref().unwrap()).unwrap();
            dispatched.push(format!("textSelect:{}:{}", args["start"], args["end"]));
            settle_text_editor_action_receipt(latest_selection.receipt.unwrap(), TextEditorActionOutcome::Accepted);
        }
        assert_eq!(dispatched, law["expected"][2].as_array().unwrap().iter().map(|row| row.as_str().unwrap().to_owned()).collect::<Vec<_>>(), "{id}: settled pair");
        let text = ENGINE_SURFACES.with(|cell| cell.borrow().get(id).unwrap().editor.as_ref().unwrap().text().to_owned());
        let expected_text = if accepted { format!("{}{}", law["initial"].as_str().unwrap(), typed.iter().chain(after_new_owner.iter()).map(|row| row.as_str().unwrap()).collect::<String>()) } else { law["initial"].as_str().unwrap().to_owned() };
        assert_eq!(text, expected_text, "{id}: local echo");
        assert_eq!(text_editor_is_read_only(&scene), law["readOnly"].as_bool().unwrap(), "{id}: refusal mode");
        drop_engine_surface(id);
    }
}

#[test]
fn text_editor_scene_echoes_never_overwrite_newer_unsent_local_text() {
    let _serialized = engine_surface_law_guard();
    let id = "text-editor-local-echo";
    drop_engine_surface(id);
    let mut scene = text_editor_scene(id, "a");
    let bounds = Rect::new(0.0, 0.0, 480.0, 320.0);
    assert!(sync_engine_scene(&scene, "editor-echo-law", bounds, &Theme::default()));
    let mut input = InputState::<ActionDescriptor>::default();
    assert!(text_editor_apply_key_into(&scene, &KeyAction::Char("b".into()), &PointerModifiers::default(), &mut input).unwrap());
    assert!(drive_text_editor_outbox_step(&mut input).unwrap());
    assert!(text_editor_apply_key_into(&scene, &KeyAction::Char("c".into()), &PointerModifiers::default(), &mut input).unwrap());
    let _ = sync_text_editor_scene(&scene, bounds, &Theme::default());
    assert_eq!(ENGINE_SURFACES.with(|cell| cell.borrow().get(id).unwrap().editor.as_ref().unwrap().text().to_owned()), "abc", "an identical stale scene cannot revert local text");
    scene.text_editor.as_mut().unwrap().buffer = "ab".into();
    assert!(sync_text_editor_scene(&scene, bounds, &Theme::default()));
    let text = ENGINE_SURFACES.with(|cell| cell.borrow().get(id).unwrap().editor.as_ref().unwrap().text().to_owned());
    assert_eq!(text, "abc", "the first edit's echo acknowledges it without overwriting unsent latest text");
    scene.text_editor.as_mut().unwrap().buffer = "external".into();
    assert!(sync_text_editor_scene(&scene, bounds, &Theme::default()));
    let text = ENGINE_SURFACES.with(|cell| cell.borrow().get(id).unwrap().editor.as_ref().unwrap().text().to_owned());
    assert_eq!(text, "external", "an unrelated guest buffer remains an authoritative external edit");
    drop_engine_surface(id);
}

#[test]
fn text_editor_delivery_state_matches_the_neutral_local_echo_ledger() {
    let fixture: Value = serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧪️tests/🧫️fixtures/🔁️local-echo/🔣️.json")).expect("neutral local echo fixture");
    for law in fixture["cases"].as_array().unwrap() {
        let id = law["id"].as_str().unwrap();
        let mut state = TextEditorDeliveryState::default();
        let mut shown = law["initial"].as_str().unwrap().to_owned();
        let mut guest = shown.clone();
        let mut sequence = 1_u64;
        assert!(state.reconcile_buffer(&shown));
        for event in law["events"].as_array().unwrap() {
            if let Some(text) = event["local"].as_str() {
                if !state.read_only {
                    assert!(!state.guest_has_text(text), "{id}: fixture local must require textEdit");
                    shown = text.to_owned();
                    state.offer(TextEditorDeliverySnapshot { controller_id: "writer".into(), surface_id: id.into(), text: shown.clone(), start: shown.len(), end: shown.len() });
                    let token = std::num::NonZeroU64::new(sequence).unwrap();
                    assert!(state.note_dispatched(EngineSurfaceToken { slot: 0, generation: 1 }, token, true, true));
                    state.active = None;
                    sequence += 1;
                }
            }
            if let Some(text) = event["typed"].as_str() {
                if !state.read_only {
                    shown = text.to_owned();
                    state.offer(TextEditorDeliverySnapshot { controller_id: "writer".into(), surface_id: id.into(), text: shown.clone(), start: shown.len(), end: shown.len() });
                }
            }
            if let Some(text) = event["echo"].as_str() {
                guest = text.to_owned();
                if state.reconcile_buffer(text) {
                    shown = text.to_owned();
                }
            }
            if let Some(text) = event["refuse"].as_str() {
                let reason = event["reason"].as_str().unwrap_or("dispatch-failed");
                if matches!(reason, "undeclared-action" | "viewer-read-only") {
                    state.read_only = true;
                }
                if state.remove_pending_echo(text) && state.pending_echoes.is_empty() {
                    shown = guest.clone();
                }
            }
            assert_eq!(shown, event["expect"].as_str().unwrap(), "{id}: visible text");
            assert_eq!(state.pending_echoes.iter().map(String::as_str).collect::<Vec<_>>(), event["pending"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>(), "{id}: pending echo ledger");
            if let Some(read_only) = event["readOnly"].as_bool() {
                assert_eq!(state.read_only, read_only, "{id}: refusal mode");
            }
        }
    }
}

#[test]
fn text_editor_production_key_route_replays_chromiums_neutral_typing_sequences() {
    let _serialized = engine_surface_law_guard();
    let fixture: Value = serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🔨️modules/✍️editor/🧪️tests/🧫️fixtures/⌨️text-input/🔣️.json")).unwrap();
    for law in fixture["sequences"].as_array().unwrap() {
        let id = law["id"].as_str().unwrap();
        drop_engine_surface(id);
        let mut scene = text_editor_scene(id, law["text"].as_str().unwrap());
        scene.text_editor.as_mut().unwrap().selection_json = Some(json!({ "start": law["selection"][0], "end": law["selection"][1] }).to_string());
        assert!(sync_engine_scene(&scene, "text-input-oracle", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
        let mut input = InputState::<ActionDescriptor>::default();
        for step in law["steps"].as_array().unwrap() {
            let keys = if let Some(text) = step["type"].as_str() {
                text.chars()
                    .map(|ch| match ch {
                        '\n' => KeyAction::Enter,
                        ' ' => KeyAction::Space(true),
                        ch => KeyAction::Char(ch.to_string()),
                    })
                    .collect::<Vec<_>>()
            } else if let Some(text) = step["paste"].as_str().or_else(|| step["compose"].as_str()) {
                vec![KeyAction::Char(text.into())]
            } else {
                vec![match step["key"].as_str().unwrap() {
                    "ArrowLeft" => KeyAction::ArrowLeft,
                    "ArrowRight" => KeyAction::ArrowRight,
                    "ArrowUp" => KeyAction::ArrowUp,
                    "ArrowDown" => KeyAction::ArrowDown,
                    "Home" => KeyAction::Home,
                    "End" => KeyAction::End,
                    "Backspace" => KeyAction::Backspace,
                    "Delete" => KeyAction::Delete,
                    key => panic!("unexpected key {key}"),
                }]
            };
            for key in keys {
                assert!(text_editor_apply_key_into(&scene, &key, &PointerModifiers::default(), &mut input).expect("bounded key"), "{id}");
                let _ = drain_editor_actions_accepted(&mut input);
            }
        }
        ENGINE_SURFACES.with(|cell| {
            let map = cell.borrow();
            let host = map.get(id).unwrap().editor.as_ref().unwrap();
            assert_eq!(host.text(), law["expect"]["text"].as_str().unwrap(), "{id}");
            assert_eq!(json!([host.text()[..host.anchor()].chars().count(), host.text()[..host.caret()].chars().count()]), law["expect"]["selection"], "{id}");
        });
        drop_engine_surface(id);
    }
}
