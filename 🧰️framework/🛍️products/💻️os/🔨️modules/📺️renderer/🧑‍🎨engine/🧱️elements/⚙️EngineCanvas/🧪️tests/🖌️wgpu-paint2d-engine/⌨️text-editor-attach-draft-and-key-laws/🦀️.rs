/// ⌨️ Text editor attach, draft and key laws: the editor host attach, the explicit draft, large draft paging, the keys the shell owns and the renderer keys against the React fixture.

#[test]
fn text_editor_window_attaches_the_editor_host_and_a_key_commits_the_react_action_pair() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "text-editor-attach";
    drop_engine_surface(surface_id);
    let scene = text_editor_scene(surface_id, "alpha");
    let bounds = Rect { x: 0.0, y: 0.0, w: 480.0, h: 320.0 };

    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "the production dispatcher attaches a text-editor surface");
    let text = ENGINE_SURFACES.with(|cell| cell.borrow().get(surface_id).and_then(|entry| entry.editor.as_ref()).map(|host| host.text().to_owned()));
    assert_eq!(text.as_deref(), Some("alpha"), "the scene buffer reaches the EditorHost whole");
    assert!(stage_engine_scene_paint(&scene, bounds, Theme::default().panel), "the editor host stages a composited engine packet");

    let mut input = InputState::<ActionDescriptor>::default();
    let modifiers = PointerModifiers::default();
    assert!(text_editor_apply_key_into(&scene, &KeyAction::Char("!".into()), &modifiers, &mut input).expect("bounded publish"), "a printable key is consumed by the focused editor");
    let actions = drain_editor_actions_accepted(&mut input);

    let names: Vec<&str> = actions.iter().map(|action| action.action.as_str()).collect();
    assert_eq!(names, vec!["textEdit", "textSelect"], "the edited document precedes its selection");
    let edit = actions.iter().find(|action| action.action == "textEdit").expect("textEdit");
    let fields = action_fields(edit);
    assert_eq!(fields.iter().find(|(key, _)| key == "text").map(|(_, value)| value.as_str()), Some("alpha!"), "the edit carries the current document in the authored action field");
    assert_eq!(fields.iter().find(|(key, _)| key == "surfaceId").map(|(_, value)| value.as_str()), Some(surface_id));
    assert_eq!(fields.iter().find(|(key, _)| key == semio_framework_ui_scene_views::TEXT_EDITOR_TYPING_BUFFER_ARG).map(|(_, value)| value.as_str()), Some(surface_id), "a live typed edit names its buffer, so the window folds it into ONE typing run");
    let select = actions.iter().find(|action| action.action == "textSelect").expect("textSelect");
    let select_fields = serde_json::to_value(select.args.as_ref().unwrap()).expect("selection args");
    assert_eq!(select_fields, json!({ "surfaceId": surface_id, "start": 6, "end": 6 }));

    drop_engine_surface(surface_id);
}

#[test]
fn text_editor_explicit_draft_preserves_local_text_until_apply_or_discard() {
    let _serialized = engine_surface_law_guard();
    let fixture: Value = serde_json::from_str(fixture_text!("../../../../../🧱️elements/✏️TextEditor/🧪️tests/🧫️fixtures/📝️explicit-draft/🔣️.json")).expect("neutral explicit draft fixture");
    let law = &fixture["cases"][1];
    let id = "text-editor-explicit-draft";
    drop_engine_surface(id);
    let mut scene = text_editor_scene(id, law["initial"].as_str().unwrap());
    scene.text_editor.as_mut().unwrap().settings_json = Some(law["settings"].to_string());
    let bounds = Rect::new(0.0, 0.0, 480.0, 320.0);
    assert!(sync_engine_scene(&scene, "explicit-draft-law", bounds, &Theme::default()));

    let mut input = InputState::<ActionDescriptor>::default();
    assert_eq!(text_editor_apply_key_into(&scene, &KeyAction::Char("!".into()), &PointerModifiers::default(), &mut input), Ok(true));
    assert!(drain(&mut input).is_empty(), "typing stays local until Apply");
    assert!(text_editor_explicit_draft_is_dirty(&scene));
    assert!(text_editor_move_explicit_draft_history(&scene, false));
    assert_eq!(text_editor_accessibility_value(&scene).as_deref(), Some("Alpha"));
    assert!(text_editor_move_explicit_draft_history(&scene, true));
    assert_eq!(text_editor_accessibility_value(&scene).as_deref(), Some("Alpha!"));

    let mut unrelated = scene.clone();
    unrelated.text_editor.as_mut().unwrap().diagnostics_json = Some("[]".into());
    assert!(sync_engine_scene(&unrelated, "explicit-draft-law", bounds, &Theme::default()));
    assert_eq!(text_editor_accessibility_value(&unrelated).as_deref(), Some("Alpha!"), "unrelated scene refresh preserves the draft");

    assert_eq!(text_editor_apply_explicit_draft_into(&unrelated, &mut input), Ok(true));
    assert!(drain(&mut input).is_empty(), "Apply uses the retained lane for every draft size");
    let action = loop {
        match drive_text_editor_retained_action_step().expect("retained draft page") {
            TextEditorRetainedActionStep::Pending => {}
            TextEditorRetainedActionStep::Ready(actions) => break actions.first,
            TextEditorRetainedActionStep::Idle => panic!("publication disappeared"),
        }
    };
    assert_eq!(action.descriptor.action, law["expected"]["action"].as_str().unwrap());
    assert_eq!(serde_json::to_value(action.descriptor.args.as_ref().unwrap()).unwrap(), json!({ "path": "/description", "value": "Alpha!" }));
    settle_text_editor_action_receipt(action.receipt.unwrap(), TextEditorActionOutcome::Accepted);
    assert!(text_editor_explicit_draft_is_dirty(&unrelated), "dispatch completion does not discard an unvalidated draft");

    let mut conflicted = unrelated.clone();
    conflicted.text_editor.as_mut().unwrap().buffer = fixture["preservation"]["collaboratorScene"].as_str().unwrap().into();
    assert!(sync_engine_scene(&conflicted, "explicit-draft-law", bounds, &Theme::default()));
    assert!(text_editor_explicit_draft_has_conflict(&conflicted));
    assert_eq!(text_editor_accessibility_value(&conflicted).as_deref(), Some("Alpha!"), "a collaborator refresh preserves the local draft");
    assert_eq!(text_editor_apply_explicit_draft_into(&conflicted, &mut input), Ok(true));
    assert!(drain(&mut input).is_empty(), "a conflicted draft cannot overwrite collaborator data");

    assert!(text_editor_discard_explicit_draft(&conflicted));
    assert_eq!(text_editor_accessibility_value(&conflicted).as_deref(), fixture["preservation"]["collaboratorScene"].as_str());
    assert!(!text_editor_explicit_draft_is_dirty(&conflicted));
    assert!(!text_editor_explicit_draft_has_conflict(&conflicted));
    drop_engine_surface(id);
}

#[test]
fn text_editor_large_explicit_draft_pages_cancel_and_refusal_without_losing_text() {
    let _serialized = engine_surface_law_guard();
    let fixture: Value = serde_json::from_str(fixture_text!("../../../../../🧱️elements/✏️TextEditor/🧪️tests/🧫️fixtures/📝️explicit-draft/🔣️.json")).expect("neutral explicit draft fixture");
    let id = "text-editor-large-explicit-draft";
    drop_engine_surface(id);
    let initial = "a".repeat(65_535);
    let mut scene = text_editor_scene(id, &initial);
    scene.text_editor.as_mut().unwrap().settings_json = Some(fixture["cases"][0]["settings"].to_string());
    assert!(sync_engine_scene(&scene, "large-explicit-draft-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
    let mut input = InputState::<ActionDescriptor>::default();
    assert_eq!(text_editor_apply_key_into(&scene, &KeyAction::Char("🚀".into()), &PointerModifiers::default(), &mut input), Ok(true));
    let expected = format!("{initial}🚀");
    assert_eq!(text_editor_accessibility_value(&scene).as_deref(), Some(expected.as_str()));
    assert_eq!(text_editor_apply_explicit_draft_into(&scene, &mut input), Ok(true));
    assert!(drain(&mut input).is_empty(), "large draft never enters the inline bounded string queue");
    assert!(matches!(drive_text_editor_retained_action_step(), Ok(TextEditorRetainedActionStep::Pending)));
    assert!(text_editor_cancel_explicit_publication(&scene));
    assert_eq!(text_editor_accessibility_value(&scene).as_deref(), Some(expected.as_str()), "cancellation keeps the local draft");

    assert_eq!(text_editor_apply_explicit_draft_into(&scene, &mut input), Ok(true));
    let ready = loop {
        match drive_text_editor_retained_action_step().expect("retained source page") {
            TextEditorRetainedActionStep::Pending => {}
            TextEditorRetainedActionStep::Ready(actions) => break actions.first,
            TextEditorRetainedActionStep::Idle => panic!("publication disappeared"),
        }
    };
    assert_eq!(ready.descriptor.args.as_ref().and_then(|args| args.get("value")).and_then(semio_framework::DslValue::as_str), Some(expected.as_str()));
    let oracle = serde_json::to_value(ready.descriptor.args.as_ref().unwrap()).unwrap();
    assert_eq!(oracle["value"].as_str(), Some(expected.as_str()), "serde_json independently observes the same atomic value");
    settle_text_editor_action_receipt(ready.receipt.unwrap(), TextEditorActionOutcome::Refused("invalid-source"));
    let status = text_editor_explicit_draft_status(&scene);
    assert_eq!(status.error.as_deref(), Some("invalid-source"));
    assert!(!status.pending);
    assert_eq!(text_editor_accessibility_value(&scene).as_deref(), Some(expected.as_str()), "a refused action keeps the draft available for correction");
    drop_engine_surface(id);
}

#[test]
fn text_editor_refuses_the_keys_the_shell_owns() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "text-editor-chords";
    drop_engine_surface(surface_id);
    let scene = text_editor_scene(surface_id, "alpha");
    let bounds = Rect { x: 0.0, y: 0.0, w: 480.0, h: 320.0 };
    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "attach");

    let mut input = InputState::<ActionDescriptor>::default();
    let mut modifiers = PointerModifiers::default();
    modifiers.meta = true;
    assert!(!text_editor_apply_key_into(&scene, &KeyAction::Char("k".into()), &modifiers, &mut input).expect("bounded"), "cmd+k stays a shell chord");
    assert!(!text_editor_apply_key_into(&scene, &KeyAction::Escape, &PointerModifiers::default(), &mut input).expect("bounded"), "Escape stays a shell chord");
    assert!(drain(&mut input).is_empty());

    drop_engine_surface(surface_id);
}

#[test]
fn text_editor_renderer_keys_match_the_actual_react_fixture() {
    let _serialized = engine_surface_law_guard();
    let fixture: Value = serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🔨️modules/✍️editor/🧪️tests/🧫️fixtures/⌨️text-input/🔣️.json")).expect("neutral keyboard fixture");
    for law in fixture["rendererKeys"].as_array().expect("renderer keys") {
        let id = law["id"].as_str().unwrap();
        drop_engine_surface(id);
        let mut scene = text_editor_scene(id, law["text"].as_str().unwrap());
        let editor = scene.text_editor.as_mut().unwrap();
        editor.selection_json = Some(json!({ "start": law["selection"][0], "end": law["selection"][1] }).to_string());
        editor.settings_json = Some(json!({ "tabSize": law["tabSize"].as_u64().unwrap_or(2) }).to_string());
        editor.newline_gates_json = law.get("newlineGates").map(Value::to_string);
        assert!(sync_engine_scene(&scene, "text-key-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
        let key = match law["key"].as_str().unwrap() {
            "ArrowLeft" => KeyAction::ArrowLeft,
            "ArrowDown" => KeyAction::ArrowDown,
            "Home" => KeyAction::Home,
            "End" => KeyAction::End,
            "Tab" => KeyAction::Tab,
            "Enter" => KeyAction::Enter,
            " " => KeyAction::Space(true),
            key => KeyAction::Char(key.into()),
        };
        let modifiers = PointerModifiers { shift: law["shift"].as_bool().unwrap_or(false), alt: law["alt"].as_bool().unwrap_or(false), ..Default::default() };
        let mut input = InputState::<ActionDescriptor>::default();
        assert_eq!(text_editor_apply_key_into(&scene, &key, &modifiers, &mut input).expect("bounded key"), !modifiers.alt, "{id}");
        ENGINE_SURFACES.with(|cell| {
            let map = cell.borrow();
            let host = map.get(id).unwrap().editor.as_ref().unwrap();
            assert_eq!(host.text(), law["expect"]["text"].as_str().unwrap(), "{id}");
            assert_eq!(json!([host.anchor(), host.caret()]), law["expect"]["selection"], "{id}");
        });
        let actions = if law["operation"].is_null() { drain(&mut input) } else { drain_editor_actions_accepted(&mut input) };
        let expected = match law["operation"].as_str() {
            Some("insertText") => vec!["textEdit", "textSelect"],
            Some(_) => vec!["textSelect"],
            None => vec![],
        };
        assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), expected, "{id}");
        if let Some(select) = actions.last() {
            assert_eq!(serde_json::to_value(select.args.as_ref().unwrap()).unwrap(), json!({ "surfaceId": id, "start": law["expect"]["selection"][0], "end": law["expect"]["selection"][1] }), "{id}");
        }
        drop_engine_surface(id);
    }
}
