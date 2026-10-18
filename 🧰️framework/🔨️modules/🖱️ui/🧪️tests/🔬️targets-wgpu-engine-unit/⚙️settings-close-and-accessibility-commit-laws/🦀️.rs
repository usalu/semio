// ⚙️ Settings, close and accessibility commit laws: the settings document paint and stepper commit, the accessibility dispatch identity, the surface close, the blur commit and the select listbox projection laws.

//#region 🔖️FacadeTests
/// ⚙️ The actual Puzzle3D Settings payload enters through `UiDocumentTree`, completes the
/// production frame ladder, publishes all four controls and retains the clipped outer border,
/// two dividers, catalog icons and value text.
#[test]
fn puzzle3d_settings_document_completes_retained_paint() {
    let law = puzzle3d_settings_law();
    let window_id = law["document"]["surface"].as_str().expect("surface");
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    reconcile_puzzle3d_settings(&mut ui, window_id, &law);

    let mut atlas = FontAtlas::builtin();
    let icons = crate::wgpu::draw_types::IconAtlas::from_packed(2, 1, vec![255; 8], vec![("minus".into(), [0.0, 0.0, 0.5, 1.0]), ("plus".into(), [0.5, 0.0, 0.5, 1.0])]);
    let body = Rect::new(0.0, 0.0, 360.0, 480.0);
    drive_layout(&mut ui, window_id, body.w, body.h, &mut atlas);
    let mut draw = DrawList::default();
    let mut ready = false;
    for _ in 0..262_144 {
        match ui.frame_into_step::<RecordingSceneHost>(window_id, body, &mut atlas, Some(&icons), None, &mut draw) {
            UiFrameStep::Pending => {}
            UiFrameStep::Ready => {
                ready = true;
                break;
            }
            UiFrameStep::Missing | UiFrameStep::Fault => panic!("Puzzle3D Settings frame fault: {}", ui.paint_stall_census(window_id)),
        }
    }
    assert!(ready, "Puzzle3D Settings frame never completed: {}", ui.paint_stall_census(window_id));
    let census = ui.paint_stall_census(window_id);
    assert!(census.contains("frame=false") && census.contains("phase=None") && census.contains("fault-site=None") && census.contains("paint-key=none") && census.contains("cursor=[none]"), "terminal retained census: {census}");

    let all_instances: Vec<_> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).collect();
    let tree = ui.tree(window_id).expect("Puzzle3D Settings mounted tree");
    let hits = ui.window_hit_targets(window_id);
    for control in law["controls"].as_array().expect("controls") {
        let key = control["key"].as_str().expect("control key");
        let id = explicit_node(tree, key);
        let node = tree.node(id).expect("mounted control");
        let UiNode::NumberStepper(stepper) = &node.spec.0 else { panic!("{key} must mount as NumberStepper") };
        assert!(stepper.uniform, "{key} preserves the authored uniform scalar");
        assert_eq!(stepper.on_absolute.action, control["action"].as_str().expect("action"), "{key} keeps its authored Change binding");
        assert!(stepper.on_delta.action.is_empty(), "{key} authors no Delta binding");
        let bounds = tree.absolute_rect(id).expect("accepted absolute layout");
        assert!(bounds.w > 0.0 && bounds.h > 0.0, "{key} has accepted layout");
        assert!(hits.iter().any(|hit| hit.control_id == key && hit.kind == HitKind::NumberStepper), "{key} publishes its retained hit");

        let clip = crate::wgpu::draw_types::ScissorRect::from_rect(bounds, body.h);
        let clipped: Vec<_> = draw.layers.iter().filter(|layer| layer.scissor == Some(clip)).flat_map(|layer| layer.ui_instances.iter()).collect();
        let solids: Vec<_> = clipped.iter().copied().filter(|instance| instance.params[2] == crate::wgpu::draw_types::KIND_SOLID).collect();
        assert_eq!(solids.len(), 7, "{key} retains one five-quad border and two clipped dividers when neither side is hovered");
        assert!(solids.iter().any(|instance| rect_matches(instance.rect, bounds)), "{key} retains the outer background");
        let [minus, value_segment, plus] = crate::wgpu::layout::number_stepper_segments(bounds, ui_contract::FlowInline::Ltr, ui.theme.stroke_hairline);
        assert!(!solids.iter().any(|instance| rect_matches(instance.rect, value_segment)), "{key} center is transparent and borderless");
        let hair = ui.theme.stroke_hairline;
        for divider in [Rect::new(minus.x + minus.w - hair, minus.y, hair, minus.h), Rect::new(plus.x, plus.y, hair, plus.h)] {
            assert!(solids.iter().any(|instance| rect_matches(instance.rect, divider)), "{key} side divider follows its button edge");
        }
        let texture_instances: Vec<_> = clipped.iter().copied().filter(|instance| instance.params[2] == crate::wgpu::draw_types::KIND_TEXTURED).collect();
        assert_eq!(texture_instances.len(), 2, "{key} retains two catalog icons");
        for (id, segment) in [("minus", minus), ("plus", plus)] {
            let size = crate::wgpu::chrome::ICON_TINY;
            let inset = if id == "minus" { -hair } else { hair };
            let expected = Rect::new(segment.x + (segment.w - size + inset) * 0.5, segment.y + (segment.h - size) * 0.5, size, size);
            assert!(texture_instances.iter().any(|instance| instance.uv_rect == icons.icon_uv(id).unwrap() && rect_matches(instance.rect, expected)), "{key} positions the {id} catalog icon");
        }

        let glyphs: Vec<_> = all_instances.iter().copied().filter(|instance| instance.params[2] == crate::wgpu::draw_types::KIND_GLYPH && rect_inside(instance.rect, bounds)).collect();
        let expected_text = control["formatted"].as_str().unwrap();
        assert_eq!(ui_contract::format_ui_number(stepper.value), expected_text);
        let thirds = crate::wgpu::layout::number_stepper_segments(bounds, ui_contract::FlowInline::Ltr, ui.theme.stroke_hairline);
        let in_segment = |segment: Rect| {
            glyphs
                .iter()
                .filter(|instance| {
                    let centre = instance.rect[0] + instance.rect[2] * 0.5;
                    centre >= segment.x && centre <= segment.x + segment.w
                })
                .count()
        };
        assert_eq!(in_segment(thirds[0]), 0, "{key} minus comes from the icon atlas");
        assert!(in_segment(thirds[1]) >= control["formatted"].as_str().expect("formatted value").chars().count(), "{key} retains the formatted value glyph run");
        assert_eq!(in_segment(thirds[2]), 0, "{key} plus comes from the icon atlas");
    }
}

#[test]
fn puzzle3d_settings_stepper_commits_on_press_and_release_only_retires_capture() {
    let law = puzzle3d_settings_law();
    let gestures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️stepper-pointer-commit/🔣️.json")).expect("stepper pointer fixture");
    for gesture in gestures["cases"].as_array().expect("gesture cases") {
        let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
        let window_id = law["document"]["surface"].as_str().expect("surface");
        reconcile_puzzle3d_settings(&mut ui, window_id, &law);
        drive_layout(&mut ui, window_id, 360.0, 480.0, &mut FontAtlas::builtin());
        let control = &law["controls"][0];
        let tree = ui.tree(window_id).expect("mounted tree");
        let id = explicit_node(tree, control["key"].as_str().expect("control key"));
        let bounds = tree.absolute_rect(id).expect("stepper bounds");
        let segment = match gesture["segment"].as_str().unwrap() {
            "minus" => 0,
            "value" => 1,
            "plus" => 2,
            _ => unreachable!(),
        };
        let rect = crate::wgpu::layout::number_stepper_segments(bounds, ui_contract::FlowInline::Ltr, ui.theme.stroke_hairline)[segment];
        let (x, y) = (rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
        let pressed = ui.dispatch_event(window_id, UiEvent::PointerDown { x, y, button: PointerButton::Primary, modifiers: Default::default() });
        let actions = pressed
            .iter()
            .filter_map(|command| match command {
                UiCommand::App { intent, .. } => Some(intent.descriptor()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(actions.len(), gesture["pressActions"].as_u64().unwrap() as usize, "{} commits on press", gesture["id"]);
        if let Some(action) = actions.first() {
            assert_eq!(action.action, control["action"].as_str().unwrap());
            let value = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_f64()).expect("absolute stepper value");
            let expected = control["value"].as_f64().unwrap() + gesture["deltaSteps"].as_f64().unwrap() * control["step"].as_f64().unwrap();
            assert!((value - expected).abs() < 1e-9);
        }
        let (release_x, release_y) = if gesture["terminal"] == "release-outside" { (-100.0, -100.0) } else { (x, y) };
        let terminal = if gesture["terminal"] == "cancel" { UiEvent::PointerCancel } else { UiEvent::PointerUp { x: release_x, y: release_y, button: PointerButton::Primary, modifiers: Default::default() } };
        let released = ui.dispatch_event(window_id, terminal);
        assert_eq!(released.iter().filter(|command| matches!(command, UiCommand::App { .. })).count(), gesture["terminalActions"].as_u64().unwrap() as usize);
        assert!(!ui.tree(window_id).unwrap().node(id).unwrap().flags.contains(NodeFlags::ACTIVE));
        assert!(ui.windows.get(window_id).unwrap().router.capture().is_none());
    }
}

#[test]
fn accessibility_dispatch_uses_current_window_generation_and_node_identity() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let window_id = "conformance.a11y.projection";
    publish_accessibility_document(&mut ui, window_id);
    let first_generation = ui.surface_generation(window_id).expect("published surface generation");
    let focus = ui.dispatch_accessibility_event(window_id, first_generation, 2, "#width", AccessibilityUiEvent::Focus).expect("current focus address is accepted");
    assert!(focus.iter().any(|command| matches!(command, UiCommand::FocusChanged { node: Some(_), .. })));
    assert_eq!(crate::wgpu::accessibility::accessibility_projection(ui.tree(window_id).unwrap()).iter().find(|node| node.node_id == 2).map(|node| node.focused), Some(true));
    let value = ui.dispatch_accessibility_event(window_id, first_generation, 2, "#width", AccessibilityUiEvent::Value("42".into())).expect("current value address is accepted");
    assert_eq!(value.iter().filter(|command| matches!(command, UiCommand::App { .. })).count(), 1, "one AT value event mints one retained action");
    assert!(ui.dispatch_accessibility_event(window_id, first_generation + 1, 2, "#width", AccessibilityUiEvent::Activate).is_none());
    assert!(ui.dispatch_accessibility_event(window_id, first_generation, 2, "#reused", AccessibilityUiEvent::Activate).is_none());
    let token = ui.surface_token(window_id).expect("published surface token");
    close_surface_to_terminal(&mut ui, token);
    assert!(ui.dispatch_accessibility_event(window_id, first_generation, 2, "#width", AccessibilityUiEvent::Activate).is_none(), "a retired document cannot receive a stale activation");
    publish_accessibility_document(&mut ui, window_id);
    let second_generation = ui.surface_generation(window_id).unwrap();
    assert!(second_generation > first_generation);
    assert!(ui.dispatch_accessibility_event(window_id, first_generation, 2, "#width", AccessibilityUiEvent::Activate).is_none(), "a reopened same-id window rejects the prior lifetime");
    assert!(ui.dispatch_accessibility_event(window_id, second_generation, 2, "#width", AccessibilityUiEvent::Focus).is_some());
}

fn close_surface_to_terminal(ui: &mut Ui, token: UiSurfaceToken) {
    let mut sequence=0;let cancel=semio_framework_job::root_cancel_token();
    let id = ui.windows.id(token).unwrap().clone();
    let generation = ui.surface_generation(id.as_ref()).unwrap();
    let tree = ui.tree(id.as_ref()).unwrap();
    let mut pending = tree.root.into_iter().collect::<Vec<_>>();
    let mut expected_scenes = Vec::new();
    while let Some(node_id) = pending.pop() {
        let node = tree.node(node_id).unwrap();
        if let UiNode::ComponentScene(scene) = &node.spec.0 {
            expected_scenes.push((node_id, node.key.clone(), node.component_generation(), scene.component_kind, scene.surface_id.clone()));
        }
        pending.extend(tree.children(node_id));
    }
    assert!(ui.begin_surface_close(token));
    for _ in 0..262_144 {
        let mut receipt=Default::default();let mut cx=semio_framework_job::StepContext::new(semio_framework_job::OperationId(17),semio_framework_job::Generation(9),semio_framework_job::StepBudget::new(1,12345,ui_contract::UI_WORKER_RETIREMENT_POLICY),cancel.clone(),||Some(1),&mut sequence,&mut receipt);
        match ui.close_surface_one(token,&mut cx).unwrap() {
            UiSurfaceCloseStep::Pending => {}
            UiSurfaceCloseStep::Refused(refusal) => panic!("original surface close refused: {refusal:?}"),
            UiSurfaceCloseStep::RetiredScene(retired) => {
                assert_eq!(retired.window_id, id.as_ref());
                assert_eq!(retired.window_generation, generation);
                let actual = (retired.node, retired.key, retired.component_generation, retired.kind, retired.surface_id);
                let index = expected_scenes.iter().position(|scene| *scene == actual).expect("one exact external scene retirement, before slot release");
                expected_scenes.remove(index);
                assert_eq!(ui.surface_token(id.as_ref()), Some(token));
            }
            UiSurfaceCloseStep::Complete => {
                assert!(expected_scenes.is_empty(), "all external scene owners leave before the registry slot");
                return;
            }
        }
    }
    panic!("the complete surface owner exceeded its finite retirement opportunity ceiling");
}

#[test]
fn closed_surface_token_and_document_epoch_cannot_alias_a_same_id_successor() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️surface-lifetime/🔣️.json")).unwrap();
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let window_id = "surface-close-same-id";
    publish_accessibility_document(&mut ui, window_id);
    let old_token = ui.surface_token(window_id).unwrap();
    let old_generation = ui.surface_generation(window_id).unwrap();
    close_surface_to_terminal(&mut ui, old_token);
    assert!(ui.surface_token(window_id).is_none());
    publish_accessibility_document(&mut ui, window_id);
    let token = ui.surface_token(window_id).unwrap();
    let generation = ui.surface_generation(window_id).unwrap();
    assert_ne!(token, old_token);
    assert!(generation > old_generation);
    assert_eq!(law["sameIdSuccessor"]["rejectPreviousIdentity"], true);
    assert!(!ui.begin_surface_close(old_token));
    let mut receipt=Default::default();let mut sequence=0;let mut cx=semio_framework_job::StepContext::new(semio_framework_job::OperationId(17),semio_framework_job::Generation(9),semio_framework_job::StepBudget::new(1,12345,ui_contract::UI_WORKER_RETIREMENT_POLICY),semio_framework_job::root_cancel_token(),||Some(1),&mut sequence,&mut receipt);
    assert!(matches!(ui.close_surface_one(old_token,&mut cx).unwrap(), UiSurfaceCloseStep::Complete));
    assert_eq!(ui.surface_token(window_id), Some(token));
    assert_eq!(law["sameIdSuccessor"]["preserveSuccessor"], true);
    assert!(ui.dispatch_accessibility_event(window_id, old_generation, 2, "#width", AccessibilityUiEvent::Activate).is_none());
    assert!(ui.dispatch_accessibility_event(window_id, generation, 2, "#width", AccessibilityUiEvent::Focus).is_some());
    close_surface_to_terminal(&mut ui, token);
}

#[test]
fn surface_close_releases_queued_layout_capacity_for_every_sequential_window() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️surface-lifetime/🔣️.json")).unwrap();
    let count = law["sequentialMounts"].as_u64().unwrap() as usize;
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    for index in 0..count {
        let id = format!("queued-surface-{index}");
        publish_accessibility_document(&mut ui, &id);
        let token = ui.surface_token(&id).unwrap();
        assert!(ui.windows.get_token(token).unwrap().queued);
        assert!(ui.layout_queues.iter().any(|lane| lane.len() > 0));
        close_surface_to_terminal(&mut ui, token);
        assert_eq!(ui.window_ids().count(), law["closedSurfaces"].as_u64().unwrap() as usize);
        assert!(ui.layout_queues.iter().all(|lane| lane.len() == 0));
        assert!(ui.layout_pressure.is_none());
    }
}

#[test]
fn surface_close_silently_discards_a_focused_blur_commit() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️surface-lifetime/🔣️.json")).unwrap();
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let id = "silent-input-unmount";
    publish_blur_commit_input_document(&mut ui, id);
    let generation = ui.surface_generation(id).unwrap();
    let token = ui.surface_token(id).unwrap();
    assert!(ui.dispatch_accessibility_event(id, generation, 2, "framework.settings.driver.saveLabel", AccessibilityUiEvent::Focus).is_some());
    let staged = ui.dispatch_accessibility_event(id, generation, 2, "framework.settings.driver.saveLabel", AccessibilityUiEvent::Value(law["silentBlurCommit"]["draft"].as_str().unwrap().into())).unwrap();
    assert!(!staged.iter().any(|command| matches!(command, UiCommand::App { .. })));
    ui.drain_commands();
    assert!(ui.begin_surface_close(token));
    assert!(ui.try_admit_surface(id).is_err());
    assert!(ui.dispatch_accessibility_event(id, generation, 2, "framework.settings.driver.saveLabel", AccessibilityUiEvent::Blur).is_none());
    assert!(ui.dispatch_event(id, UiEvent::PointerCancel).is_empty());
    assert!(ui.advance_window_clock(id, 1000.0).is_none());
    close_surface_to_terminal(&mut ui, token);
    assert_eq!(ui.drain_commands().len(), law["silentBlurCommit"]["actions"].as_u64().unwrap() as usize);
    assert_eq!(ui.window_ids().count(), law["closedSurfaces"].as_u64().unwrap() as usize);
}

#[test]
fn accessibility_blur_input_stages_values_and_commits_exactly_once_on_blur() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let window_id = "framework.settings.general";
    publish_blur_commit_input_document(&mut ui, window_id);
    let generation = ui.surface_generation(window_id).unwrap();
    ui.dispatch_accessibility_event(window_id, generation, 2, "framework.settings.driver.saveLabel", AccessibilityUiEvent::Focus).expect("focus");
    let value = ui.dispatch_accessibility_event(window_id, generation, 2, "framework.settings.driver.saveLabel", AccessibilityUiEvent::Value("Focus Flow".into())).expect("value");
    assert!(value.iter().all(|command| !matches!(command, UiCommand::App { .. })), "a blur-committing value remains a draft");
    let blur = ui.dispatch_accessibility_event(window_id, generation, 2, "framework.settings.driver.saveLabel", AccessibilityUiEvent::Blur).expect("blur");
    let actions = blur
        .iter()
        .filter_map(|command| match command {
            UiCommand::App { intent, .. } => Some(intent.descriptor()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].action, "setDriverSaveLabel");
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("value")), Some(&DslValue::String("Focus Flow".into())));
    assert!(blur.iter().any(|command| matches!(command, UiCommand::FocusChanged { node: None, .. })));
    let repeated = ui.dispatch_accessibility_event(window_id, generation, 2, "framework.settings.driver.saveLabel", AccessibilityUiEvent::Blur).expect("repeated blur");
    assert!(repeated.iter().all(|command| !matches!(command, UiCommand::App { .. })), "blur commit is exact once");
}

#[test]
fn accessibility_select_projects_one_live_listbox_and_option_activation_commits_once() {
    let law = select_accessibility_law();
    let window_id = law["window"]["id"].as_str().unwrap();
    let node_id = law["select"]["nodeId"].as_u64().unwrap();
    let node_key = law["select"]["key"].as_str().unwrap();
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    publish_select_accessibility_document(&mut ui, &law);
    let generation = ui.surface_generation(window_id).expect("published Select surface generation");

    let closed: Vec<_> = crate::wgpu::accessibility::accessibility_projection(ui.tree(window_id).unwrap()).into_iter().filter(|node| node.node_id == node_id).map(|node| node.role).collect();
    assert_eq!(closed, law["closedRoles"].as_array().unwrap().iter().map(|role| role.as_str().unwrap().to_string()).collect::<Vec<_>>());
    let opened = ui.dispatch_accessibility_event(window_id, generation, node_id, node_key, AccessibilityUiEvent::Activate).expect("current Select address opens");
    assert!(opened.iter().all(|command| !matches!(command, UiCommand::App { .. })), "opening the listbox commits no value");
    let projection = crate::wgpu::accessibility::accessibility_projection(ui.tree(window_id).unwrap());
    let open: Vec<_> = projection.iter().filter(|node| node.node_id == node_id).map(|node| node.role.as_str()).collect();
    assert_eq!(open, law["openRoles"].as_array().unwrap().iter().map(|role| role.as_str().unwrap()).collect::<Vec<_>>());
    for expected in law["select"]["options"].as_array().unwrap() {
        let option = projection.iter().find(|node| node.key == expected["key"].as_str().unwrap()).expect("open option is projected");
        assert_eq!(option.role, "option");
        assert_eq!(option.label.as_deref(), expected["label"].as_str());
        assert_eq!(option.selected, expected["selected"].as_bool());
    }

    let option_key = law["activation"]["optionKey"].as_str().unwrap();
    let commands = ui.dispatch_accessibility_event(window_id, generation, node_id, option_key, AccessibilityUiEvent::Activate).expect("current virtual option address activates");
    let actions: Vec<_> = commands
        .iter()
        .filter_map(|command| match command {
            UiCommand::App { intent, .. } => Some(intent.descriptor()),
            _ => None,
        })
        .collect();
    assert_eq!(actions.len(), law["activation"]["expectedActions"].as_u64().unwrap() as usize);
    assert_eq!(
        actions[0].args.as_ref().and_then(|args| args.as_object()).and_then(|entries| entries.iter().find(|(key, _)| key == "value")).map(|(_, value)| value),
        Some(&DslValue::String(law["activation"]["expectedValue"].as_str().unwrap().to_string()))
    );
    assert!(commands.iter().any(|command| matches!(command, UiCommand::OverlayClosed { .. })), "option activation closes its one popup authority");
    assert!(ui.dispatch_accessibility_event(window_id, generation, node_id, option_key, AccessibilityUiEvent::Activate).is_none(), "the closed popup rejects its retired virtual option address");
    let closed_again: Vec<_> = crate::wgpu::accessibility::accessibility_projection(ui.tree(window_id).unwrap()).into_iter().filter(|node| node.node_id == node_id).map(|node| node.role).collect();
    assert_eq!(closed_again, closed, "closing retires listbox and options without retiring the Select document");
}
//#endregion 🔖️FacadeTests
