// 🧱️ Engine fixtures and scene lifetime laws: the stack, button, clock and layout pool fixtures, the scene lifetime, slider, stepper and tooltip documents, the candidate retirement, the window clock, the caret and the pointer capture transfer laws.

//#region 🔖️FacadeTests
fn stack_ui(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some("root".into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}

fn action() -> ActionDescriptor {
    ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None }
}

fn button_ui(id: &str, label: &str) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: IconName::CircleDot, label: Label::data(label), action: action(), style: None, presence: UiPresence::default(), menu: None })
}

fn test_clock() -> Option<u64> {
    Some(0)
}

fn test_layout_pool() -> semio_framework_async::WorkerPool {
    semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1))
}

fn scene_lifetime_document(window_id: &str, generation: u64, scene: bool) -> UiDocumentTree {
    let component = if scene { serde_json::json!({ "type": "surface", "kind": "world-3d", "docSchema": "world3d@1", "doc": { "bytes": [] } }) } else { serde_json::json!({ "type": "container" }) };
    let record: UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": "scene-lifetime/root",
        "component": component,
        "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
        "style": {},
        "activity": "idle",
        "accessibility": {},
        "children": []
    }))
    .expect("scene lifetime record");
    let mut document = UiDocumentTree::new(UiDocumentLeaseHeader { generation, surface: SurfaceId::try_from(window_id).expect("scene lifetime surface"), revision: UiRevision(generation), root: UiNodeId(1), layout_epoch: generation, node_count: 1 })
        .expect("scene lifetime document");
    document.try_upsert_record(record).expect("scene lifetime record admits");
    document
}

fn captured_slider_document(window_id: &str, generation: u64, value: f64, children: &[u64]) -> UiDocumentTree {
    let mut root: UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": "captured-slider/root",
        "component": { "type": "container" },
        "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
        "style": {},
        "activity": "idle",
        "accessibility": {},
        "children": children
    }))
    .expect("captured slider root");
    root.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Horizontal, grow: true, ..Default::default() });
    let mut document = UiDocumentTree::new(UiDocumentLeaseHeader {
        generation,
        surface: SurfaceId::try_from(window_id).expect("captured slider surface"),
        revision: UiRevision(generation),
        root: UiNodeId(1),
        layout_epoch: generation,
        node_count: children.len() + 1,
    })
    .expect("captured slider document");
    document.try_upsert_record(root).expect("captured slider root admits");
    for id in children {
        let component = if *id == 4 { serde_json::json!({ "type": "slider", "value": value, "min": 0, "max": 10, "step": 1 }) } else { serde_json::json!({ "type": "container" }) };
        let bindings = if *id == 4 { serde_json::json!([{ "trigger": "change", "action": { "scope": "fixture", "name": "setValue", "version": 1 } }]) } else { serde_json::json!([]) };
        let mut record: UiNodeRecord = serde_json::from_value(serde_json::json!({
            "id": id,
            "key": format!("captured-slider/{id}"),
            "component": component,
            "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
            "style": {},
            "activity": "idle",
            "accessibility": if *id == 4 { serde_json::json!({ "label": "Captured slider" }) } else { serde_json::json!({}) },
            "bindings": bindings,
            "children": []
        }))
        .expect("captured slider child");
        record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
        document.try_upsert_record(record).expect("captured slider child admits");
    }
    document
}

fn held_stepper_document(window_id: &str, generation: u64, hold: &serde_json::Value) -> UiDocumentTree {
    let mut root: UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": "held-stepper/root",
        "component": { "type": "container" },
        "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
        "style": {},
        "activity": "idle",
        "accessibility": {},
        "children": [2]
    }))
    .expect("held stepper root");
    root.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Horizontal, grow: true, ..Default::default() });
    let mut stepper: UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 2,
        "key": "held-stepper/control",
        "component": { "type": "numberStepper", "value": hold["start"], "step": 1, "uniform": true, "min": 0, "max": 5 },
        "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
        "style": {},
        "activity": "idle",
        "accessibility": { "label": "Held value" },
        "bindings": [{ "trigger": "change", "action": { "scope": "fixture", "name": "setHeldValue", "version": 1 } }],
        "children": []
    }))
    .expect("held stepper control");
    stepper.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
    let mut document = UiDocumentTree::new(UiDocumentLeaseHeader { generation, surface: SurfaceId::try_from(window_id).expect("held stepper surface"), revision: UiRevision(generation), root: UiNodeId(1), layout_epoch: generation, node_count: 2 })
        .expect("held stepper document");
    document.try_upsert_record(root).expect("held stepper root admits");
    document.try_upsert_record(stepper).expect("held stepper control admits");
    document
}

fn tooltip_document(window_id: &str, generation: u64, label: &str, shortcut: &str) -> UiDocumentTree {
    let records = serde_json::json!([
        {
            "id": 1,
            "key": "tooltip/root",
            "component": { "type": "container" },
            "layout": { "kind": "stack", "axis": "vertical", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "grow": false, "wrap": false },
            "style": {},
            "activity": "idle",
            "accessibility": {},
            "children": [2]
        },
        {
            "id": 2,
            "key": "tooltip/close",
            "component": { "type": "button", "icon": "x", "label": label },
            "layout": { "kind": "leaf", "width": "hug", "height": "hug" },
            "style": {},
            "activity": "idle",
            "accessibility": { "label": label, "shortcut": shortcut },
            "bindings": [{ "trigger": "activate", "action": { "scope": "fixture", "name": "close", "version": 1 } }],
            "children": []
        }
    ]);
    let records = records.as_array().expect("tooltip records");
    let mut document =
        UiDocumentTree::new(UiDocumentLeaseHeader { generation, surface: SurfaceId::try_from(window_id).expect("tooltip surface"), revision: UiRevision(generation), root: UiNodeId(1), layout_epoch: generation, node_count: records.len() })
            .expect("tooltip document");
    for record in records {
        document.try_upsert_record(serde_json::from_value(record.clone()).expect("tooltip record")).expect("tooltip record admits");
    }
    document
}

fn place_tooltip_anchor(tree: &mut UiTree, anchor: [f32; 4], viewport: [f32; 2]) {
    let root = tree.document_node(UiNodeId(1)).expect("tooltip root");
    let control = tree.document_node(UiNodeId(2)).expect("tooltip control");
    let generation = tree.accepted_layout_generation().saturating_add(1);
    assert!(tree.write_inactive_layout(root, generation, semio_framework_ui_retained_tree::tree::AcceptedLayout { x: 0.0, y: 0.0, width: viewport[0], height: viewport[1], inline_suffix_width: 0.0 }));
    assert!(tree.write_inactive_layout(control, generation, semio_framework_ui_retained_tree::tree::AcceptedLayout { x: anchor[0], y: anchor[1], width: anchor[2], height: anchor[3], inline_suffix_width: 0.0 },));
    tree.commit_inactive_layout(generation);
}

fn reconcile_scene_lifetime_candidate(ui: &mut Ui, window_id: &str, generation: u64, scene: bool) {
    assert!(ui.publish_document(window_id, scene_lifetime_document(window_id, generation, scene)));
    drive_scene_lifetime_reconcile(ui, window_id, generation);
}

fn drive_scene_lifetime_reconcile(ui: &mut Ui, window_id: &str, generation: u64) {
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    for _ in 0..4096 {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(64, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), test_clock, &mut sequence,&mut actual_retained_progress);
        match ui.step_document_reconcile(window_id, "scene-lifetime", &mut cx) {
            UiDocumentReconcileStep::Pending => {}
            UiDocumentReconcileStep::Complete => return,
            UiDocumentReconcileStep::Fault(fault) => panic!("scene lifetime reconcile fault: {fault:?}"),
        }
    }
    panic!("scene lifetime reconcile exceeded its fixed budget");
}

fn acknowledge_scene_lifetime_candidate(ui: &mut Ui, window_id: &str, witness: u64) {
    assert!(ui.seal_presented_input_candidate(witness, &[window_id.to_string()]));
    assert!(ui.acknowledge_presented_input(witness));
}

fn root_scene_host(tree: &UiTree) -> Option<String> {
    let node = tree.root.and_then(|root| tree.node(root))?;
    let UiNode::ComponentScene(scene) = &node.spec.0 else { return None };
    Some(scene.host_id.clone())
}

#[test]
fn candidate_scene_removal_retires_only_after_presentation_acknowledgement() {
    let window_id = "candidate-scene-removal";
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    reconcile_scene_lifetime_candidate(&mut ui, window_id, 1, true);
    acknowledge_scene_lifetime_candidate(&mut ui, window_id, 11);
    drive_scene_lifetime_reconcile(&mut ui, window_id, 1);
    let presented_host = ui.windows.get(window_id).and_then(|window| root_scene_host(&window.presented_tree)).expect("presented scene host");

    reconcile_scene_lifetime_candidate(&mut ui, window_id, 2, false);
    assert_eq!(ui.windows.get(window_id).and_then(|window| root_scene_host(&window.presented_tree)).as_deref(), Some(presented_host.as_str()));
    assert!(ui.take_retired_component_scene(window_id).is_none(), "candidate topology cannot retire still-presented pixels");
    assert!(ui.seal_presented_input_candidate(12, &[window_id.to_string()]));
    assert!(ui.discard_presented_input_candidate(12));
    assert!(ui.take_retired_component_scene(window_id).is_none(), "discarding the presentation witness preserves the presented scene");
    assert!(ui.seal_presented_input_candidate(13, &[window_id.to_string()]));
    assert!(ui.acknowledge_presented_input(13));
    assert_eq!(ui.take_retired_component_scene(window_id).map(|retired| retired.host_id), Some(presented_host));
}

#[test]
fn superseded_candidate_rechecks_deferred_scene_retirement() {
    let window_id = "superseded-scene-removal";
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    reconcile_scene_lifetime_candidate(&mut ui, window_id, 1, true);
    acknowledge_scene_lifetime_candidate(&mut ui, window_id, 15);
    drive_scene_lifetime_reconcile(&mut ui, window_id, 1);
    let presented_host = ui.windows.get(window_id).and_then(|window| root_scene_host(&window.presented_tree)).expect("presented scene host");

    reconcile_scene_lifetime_candidate(&mut ui, window_id, 2, false);
    reconcile_scene_lifetime_candidate(&mut ui, window_id, 3, true);
    assert!(ui.take_retired_component_scene(window_id).is_none(), "superseding an unaccepted removal cannot publish the presented host retirement");
    acknowledge_scene_lifetime_candidate(&mut ui, window_id, 16);
    assert_eq!(ui.windows.get(window_id).and_then(|window| root_scene_host(&window.presented_tree)).as_deref(), Some(presented_host.as_str()));
    assert!(ui.take_retired_component_scene(window_id).is_none(), "the surviving successor cancels its deferred retirement before acknowledgement");
}

#[test]
fn component_scene_host_survives_alternating_arenas_and_readd_gets_a_fresh_mount() {
    let window_id = "alternating-scene-host";
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    reconcile_scene_lifetime_candidate(&mut ui, window_id, 1, true);
    acknowledge_scene_lifetime_candidate(&mut ui, window_id, 21);
    let first = ui.windows.get(window_id).and_then(|window| root_scene_host(&window.presented_tree)).expect("first presented host");

    drive_scene_lifetime_reconcile(&mut ui, window_id, 1);
    reconcile_scene_lifetime_candidate(&mut ui, window_id, 2, true);
    assert!(ui.seal_presented_input_candidate(22, &[window_id.to_string()]));
    let presented_node = ui.windows.get(window_id).and_then(|window| window.presented_tree.root).expect("presented scene node");
    let candidate_node = ui.windows.get(window_id).and_then(|window| window.tree.root).expect("candidate scene node");
    assert!(ui.candidate_is_sealed_for(window_id, 22));
    assert_eq!(ui.candidate_scene_node_for_presented_node(window_id, 22, presented_node), Some(candidate_node));
    assert!(ui.candidate_scene_node_for_presented_node(window_id, 23, presented_node).is_none(), "a different presenter witness cannot expose the candidate mapping");
    assert!(ui.acknowledge_presented_input(22));
    assert_eq!(ui.windows.get(window_id).and_then(|window| root_scene_host(&window.presented_tree)).as_deref(), Some(first.as_str()));

    drive_scene_lifetime_reconcile(&mut ui, window_id, 2);
    reconcile_scene_lifetime_candidate(&mut ui, window_id, 3, false);
    acknowledge_scene_lifetime_candidate(&mut ui, window_id, 23);
    assert_eq!(ui.take_retired_component_scene(window_id).map(|retired| retired.host_id), Some(first.clone()));
    drive_scene_lifetime_reconcile(&mut ui, window_id, 3);
    reconcile_scene_lifetime_candidate(&mut ui, window_id, 4, true);
    acknowledge_scene_lifetime_candidate(&mut ui, window_id, 24);
    let readded = ui.windows.get(window_id).and_then(|window| root_scene_host(&window.presented_tree)).expect("re-added presented host");
    assert_ne!(readded, first);
}

#[test]
fn held_pointer_capture_transfers_to_the_accepted_candidate_and_releases_once() {
    let window_id = "presented-captured-slider";
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    assert!(ui.publish_document(window_id, captured_slider_document(window_id, 1, 2.0, &[2])));
    drive_scene_lifetime_reconcile(&mut ui, window_id, 1);
    let mut atlas = FontAtlas::builtin();
    drive_layout(&mut ui, window_id, 200.0, 40.0, &mut atlas);
    acknowledge_scene_lifetime_candidate(&mut ui, window_id, 30);

    drive_scene_lifetime_reconcile(&mut ui, window_id, 1);
    assert!(ui.publish_document(window_id, captured_slider_document(window_id, 2, 2.0, &[2, 3])));
    drive_scene_lifetime_reconcile(&mut ui, window_id, 2);
    assert!(ui.publish_document(window_id, captured_slider_document(window_id, 3, 2.0, &[2, 4])));
    drive_scene_lifetime_reconcile(&mut ui, window_id, 3);
    drive_layout(&mut ui, window_id, 200.0, 40.0, &mut atlas);
    acknowledge_scene_lifetime_candidate(&mut ui, window_id, 31);
    let presented_node = ui.windows.get(window_id).and_then(|window| window.presented_tree.document_node(UiNodeId(4))).expect("presented slider node");
    let (x, y, w, h) = ui.windows.get(window_id).and_then(|window| window.presented_tree.mounted_layout(presented_node)).expect("presented slider layout");
    ui.dispatch_pointer_event(window_id, 77, UiEvent::PointerDown { x: x + w * 0.5, y: y + h * 0.5, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(ui.windows.get(window_id).and_then(|window| window.presented_router.capture()).is_some());

    drive_scene_lifetime_reconcile(&mut ui, window_id, 3);
    assert!(ui.publish_document(window_id, captured_slider_document(window_id, 4, 3.0, &[2, 3, 4])));
    drive_scene_lifetime_reconcile(&mut ui, window_id, 4);
    drive_layout(&mut ui, window_id, 200.0, 40.0, &mut atlas);
    assert!(ui.seal_presented_input_candidate(32, &[window_id.to_string()]), "a surviving captured control cannot block pixel publication");
    let candidate_node = ui.windows.get(window_id).and_then(|window| window.tree.document_node(UiNodeId(4))).expect("candidate slider node");
    assert!(ui.candidate_is_sealed_for(window_id, 32));
    assert_eq!(ui.candidate_scene_node_for_presented_node(window_id, 32, presented_node), None, "the generic retained mapper refuses a non-scene node");
    assert!(!ui.candidate_is_sealed_for(window_id, 33));
    assert_ne!(presented_node, candidate_node, "the capture law exercises separate retained arenas");
    assert!(ui.acknowledge_presented_input(32));
    assert!(ui.windows.get(window_id).and_then(|window| window.presented_router.capture()).is_some(), "the exact pointer owner transfers with accepted pixels");

    ui.dispatch_pointer_event(window_id, 77, UiEvent::PointerMove { x: 120.0, y: 20.0, modifiers: Default::default() });
    ui.dispatch_pointer_event(window_id, 77, UiEvent::PointerUp { x: 120.0, y: 20.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(ui.windows.get(window_id).and_then(|window| window.presented_router.capture()).is_none());
}

#[test]
fn window_clock_keeps_a_noop_candidate_sealed_and_invalidates_it_once_when_hold_repeat_changes_state() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/⌨️number-stepper-editing/🔣️.json")).expect("number stepper fixture");
    let hold = &fixture["hold"];
    let window_id = "clock-held-stepper";
    let generation = 1;
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    assert!(ui.publish_document(window_id, held_stepper_document(window_id, generation, hold)));
    drive_scene_lifetime_reconcile(&mut ui, window_id, generation);
    let mut atlas = FontAtlas::builtin();
    drive_layout(&mut ui, window_id, 160.0, 24.0, &mut atlas);
    acknowledge_scene_lifetime_candidate(&mut ui, window_id, 70);
    let surface = ui.surface_token(window_id).expect("clock surface token");
    assert_eq!(ui.clock_surface_at(0), Some((window_id, surface)));
    let clock_origin = hold["clockOriginSeconds"].as_f64().unwrap();
    assert!(!ui.advance_window_clock(window_id, clock_origin).unwrap().interaction_changed);

    let presented = ui.windows.get(window_id).and_then(|window| window.presented_tree.document_node(UiNodeId(2))).expect("presented stepper");
    let (x, y, width, height) = ui.windows.get(window_id).and_then(|window| window.presented_tree.mounted_layout(presented)).expect("presented stepper bounds");
    ui.dispatch_pointer_event(window_id, 1, UiEvent::PointerDown { x: x + width - height * 0.5, y: y + height * 0.5, button: PointerButton::Primary, modifiers: Default::default() });
    let tooltip_deadline = clock_origin + f64::from(TOOLTIP_DWELL_SECONDS);
    let caret_deadline = clock_origin + semio_framework_ui_wgpu_events::CARET_BLINK_SECONDS;
    let first_deadline = clock_origin + (hold["delayMs"].as_f64().unwrap() + hold["intervalMs"].as_f64().unwrap()) / 1000.0;
    assert_eq!(ui.window_next_clock_deadline(window_id), Some((surface, Some(tooltip_deadline))));
    let accepted = vec![(window_id.to_string(), surface)];
    assert_eq!(ui.surfaces_next_clock_deadline(&accepted), Some(tooltip_deadline));

    let revealed = ui.advance_window_clock(window_id, tooltip_deadline).expect("tooltip clock step");
    assert!(revealed.interaction_changed);
    assert_eq!(revealed.tooltip, TooltipStep::Reveal(presented));
    assert_eq!(revealed.next_deadline, Some(caret_deadline));

    drive_scene_lifetime_reconcile(&mut ui, window_id, generation);
    assert!(ui.seal_presented_input_candidate(71, &[window_id.to_string()]));
    let noop = ui.advance_window_clock(window_id, tooltip_deadline + (caret_deadline - tooltip_deadline) * 0.5).expect("clock step");
    assert_eq!((noop.surface, noop.interaction_changed, noop.tooltip, noop.next_deadline), (surface, false, TooltipStep::Idle, Some(caret_deadline)));
    assert!(ui.candidate_is_sealed_for(window_id, 71), "pure elapsed time does not starve an already sealed candidate");

    let blink = ui.advance_window_clock(window_id, caret_deadline).expect("caret step");
    assert!(blink.interaction_changed);
    assert_eq!((blink.tooltip, blink.caret.is_some(), blink.next_deadline), (TooltipStep::Idle, true, Some(first_deadline)));
    assert!(!ui.candidate_is_sealed_for(window_id, 71), "the due accepted caret invalidates the stale candidate once");
    assert!(ui.discard_presented_input_candidate(71));
    drive_scene_lifetime_reconcile(&mut ui, window_id, generation);
    assert!(ui.seal_presented_input_candidate(72, &[window_id.to_string()]));

    let repeated = ui.advance_window_clock(window_id, first_deadline).expect("repeat step");
    assert!(repeated.interaction_changed);
    assert_eq!(repeated.tooltip, TooltipStep::Idle);
    assert!(!ui.candidate_is_sealed_for(window_id, 72), "the due repeat changes presented state and invalidates the stale candidate exactly once");
    let live_text = ui.windows.get(window_id).and_then(|window| window.presented_tree.node(presented)).and_then(|node| node.state.edit.as_ref()).map(|edit| edit.text.as_str());
    let expected = hold["atFirstInterval"].as_i64().unwrap().to_string();
    assert_eq!(live_text, Some(expected.as_str()));

    assert!(ui.discard_presented_input_candidate(72));
    assert!(ui.seal_presented_input_candidate(73, &[]));
    assert_eq!(ui.window_next_clock_deadline(window_id), Some((surface, Some(first_deadline + hold["intervalMs"].as_f64().unwrap() / 1000.0))), "sealing an unaccepted hidden roster cannot mutate the still-visible accepted interaction");
    assert_eq!(ui.surfaces_next_clock_deadline(&accepted), Some(first_deadline + hold["intervalMs"].as_f64().unwrap() / 1000.0));
    assert!(ui.discard_presented_input_candidate(73));
    assert!(ui.window_next_clock_deadline(window_id).is_some_and(|(_, deadline)| deadline.is_some()), "discarding the hidden candidate preserves the accepted hold");
    assert!(ui.surfaces_next_clock_deadline(&accepted).is_some(), "discard preserves the accepted roster deadline");
    let window = ui.windows.get_mut(window_id).expect("clock window");
    window.presented_tooltip = Some(PresentedTooltip { surface, document_id: UiNodeId(2), label: "Increase".into(), accessibility_generation: window.presented_accessibility_generation, interaction_epoch: window.presented_interaction_epoch, row_action: None });
    assert!(ui.seal_presented_input_candidate(74, &[]));
    assert!(ui.acknowledge_presented_input(74), "accepting an empty visibility roster retires hidden clock owners");
    assert_eq!(ui.window_next_clock_deadline(window_id), Some((surface, None)));
    assert_eq!(ui.surfaces_next_clock_deadline(&accepted), None);
    assert!(ui.windows.get(window_id).expect("hidden clock window").presented_tooltip.is_none(), "hidden acceptance clears the presented tooltip with its clock state");
    assert!(!ui.advance_window_clock(window_id, clock_origin + 1_000_000.0).unwrap().interaction_changed, "a hidden held press cannot catch up when the surface becomes clocked again");
}

#[test]
fn a_late_hidden_visibility_ack_cannot_suspend_a_reused_surface_id() {
    let window_id = "reused-clock-surface";
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    ui.apply_tree(window_id, &stack_ui(Vec::new()));
    let retired = ui.surface_token(window_id).expect("first surface token");
    assert!(ui.seal_presented_input_candidate(80, &[]));
    close_surface_to_terminal(&mut ui, retired);

    let stepper =
        UiNode::NumberStepper(UiNumberStepperNode { id: "reused.stepper".into(), value: 2.0, step: 1.0, uniform: true, min: Some(0.0), max: Some(5.0), precision: None, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None, ..Default::default() });
    ui.apply_tree(window_id, &stack_ui(vec![stepper]));
    let successor = ui.surface_token(window_id).expect("successor surface token");
    assert_ne!(successor, retired);
    let mut atlas = FontAtlas::builtin();
    drive_layout(&mut ui, window_id, 160.0, 24.0, &mut atlas);
    let origin = 31_536_000.123_456;
    assert!(!ui.advance_window_clock(window_id, origin).unwrap().interaction_changed);
    let stepper = ui.windows.get(window_id).and_then(|window| window.tree.root).and_then(|root| ui.windows.get(window_id)?.tree.node(root)?.first_child).expect("successor stepper");
    let bounds = ui.windows.get(window_id).and_then(|window| window.tree.absolute_rect(stepper)).expect("successor bounds");
    ui.dispatch_pointer_event(window_id, 1, UiEvent::PointerDown { x: bounds.x + bounds.w - 1.0, y: bounds.y + bounds.h * 0.5, button: PointerButton::Primary, modifiers: Default::default() });
    let deadline = ui.window_next_clock_deadline(window_id).expect("successor clock deadline");
    assert!(deadline.1.is_some());
    assert_eq!(ui.surfaces_next_clock_deadline(&[(window_id.to_string(), retired)]), None, "an accepted roster carrying the retired token cannot adopt its id-reusing successor's deadline");
    assert_eq!(ui.surfaces_next_clock_deadline(&[(window_id.to_string(), successor)]), deadline.1);

    assert!(ui.acknowledge_presented_input(80));
    assert_eq!(ui.window_next_clock_deadline(window_id), Some(deadline), "the old token-qualified receipt cannot suspend the successor that reused its id and slot");
}

#[test]
fn accepted_input_caret_starts_solid_blinks_resets_and_cancels_from_the_shared_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧬️contract/⌨️caret-cadence/🧪️tests/🧫️fixtures/🔣️.json")).expect("caret cadence fixture");
    let half_period = fixture["halfPeriodMs"].as_f64().expect("half period") / 1_000.0;
    assert_eq!(half_period, semio_framework_ui_wgpu_events::CARET_BLINK_SECONDS);
    for (case_index, case) in fixture["cases"].as_array().expect("caret cases").iter().enumerate() {
        let window_id = format!("caret-cadence-{case_index}");
        let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
        publish_blur_commit_input_document(&mut ui, &window_id);
        let mut atlas = FontAtlas::builtin();
        drive_layout(&mut ui, &window_id, 180.0, 28.0, &mut atlas);
        acknowledge_scene_lifetime_candidate(&mut ui, &window_id, 200 + case_index as u64);
        let generation = ui.surface_generation(&window_id).expect("accepted generation");
        let surface = ui.surface_token(&window_id).expect("accepted surface");
        let input = ui.windows.get(&window_id).and_then(|window| window.presented_tree.document_node(UiNodeId(2))).expect("accepted input");
        let mut now = 31_536_000.125;
        assert!(!ui.advance_window_clock(&window_id, now).expect("clock origin").interaction_changed);

        for step in case["steps"].as_array().expect("caret steps") {
            now += step["advanceMs"].as_f64().expect("advance") / 1_000.0;
            match step["event"].as_str().expect("event") {
                "focus" => {
                    ui.dispatch_accessibility_event(&window_id, generation, 2, "framework.settings.driver.saveLabel", AccessibilityUiEvent::Focus).expect("focus accepted");
                }
                "tick" => {
                    ui.advance_window_clock(&window_id, now).expect("clock tick");
                }
                "edit" => {
                    ui.advance_window_clock(&window_id, now).expect("edit clock stamp");
                    ui.dispatch_event(&window_id, UiEvent::TextInput { text: "x".into() });
                }
                "blur" => {
                    ui.dispatch_accessibility_event(&window_id, generation, 2, "framework.settings.driver.saveLabel", AccessibilityUiEvent::Blur).expect("blur accepted");
                }
                "hide" => {
                    let witness = 300 + case_index as u64;
                    assert!(ui.seal_presented_input_candidate(witness, &[]));
                    assert!(ui.acknowledge_presented_input(witness));
                }
                event => panic!("unknown caret event {event}"),
            }
            let visible = ui.windows.get(&window_id).and_then(|window| window.presented_tree.node(input)).is_some_and(|node| node.state.caret_visible);
            assert_eq!(visible, step["visible"].as_bool().expect("visible"), "{} {step:?}", case["id"]);
            let armed = ui.surfaces_next_clock_deadline(&[(window_id.clone(), surface)]).is_some();
            assert_eq!(armed, step["armed"].as_bool().expect("armed"), "{} {step:?}", case["id"]);
        }
    }
}

#[test]
fn scene_caret_requires_exact_accepted_surface_and_document_identity() {
    let window_id = "accepted-scene-caret";
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let component = serde_json::json!({ "type": "surface", "kind": "text-editor", "docSchema": "text-editor@1", "doc": { "bytes": [13,1,6,6,98,117,102,102,101,114,6,0] } });
    let record: UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 1, "key": "caret/editor", "component": component,
        "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
        "style": {}, "activity": "idle", "accessibility": {}, "children": []
    }))
    .expect("text editor scene record");
    let mut document = UiDocumentTree::new(UiDocumentLeaseHeader { generation: 1, surface: SurfaceId::try_from(window_id).expect("surface"), revision: UiRevision(1), root: UiNodeId(1), layout_epoch: 1, node_count: 1 }).expect("scene document");
    document.try_upsert_record(record).expect("scene record admits");
    assert!(ui.publish_document(window_id, document));
    drive_scene_lifetime_reconcile(&mut ui, window_id, 1);
    let mut atlas = FontAtlas::builtin();
    drive_layout(&mut ui, window_id, 180.0, 80.0, &mut atlas);
    acknowledge_scene_lifetime_candidate(&mut ui, window_id, 400);
    let surface = ui.surface_token(window_id).expect("accepted surface");
    let origin = 31_536_000.25;
    let armed = ui.arm_presented_scene_caret(window_id, surface, UiNodeId(1), origin).expect("exact scene arms");
    assert_eq!((armed.source, armed.visible), (UiCaretSource::Scene, true));
    assert_eq!(ui.window_next_clock_deadline(window_id), Some((surface, Some(origin + semio_framework_ui_wgpu_events::CARET_BLINK_SECONDS))));
    let blink = ui.advance_window_clock(window_id, origin + semio_framework_ui_wgpu_events::CARET_BLINK_SECONDS).expect("scene blink").caret.expect("scene directive");
    assert_eq!((blink.surface, blink.document_id, blink.source, blink.visible), (surface, UiNodeId(1), UiCaretSource::Scene, false));
    let cleared = ui.clear_presented_scene_caret(window_id, surface, UiNodeId(1)).expect("exact scene clears");
    assert!(!cleared.visible);
    assert_eq!(ui.window_next_clock_deadline(window_id), Some((surface, None)));

    close_surface_to_terminal(&mut ui, surface);
    ui.apply_tree(window_id, &stack_ui(Vec::new()));
    let successor = ui.surface_token(window_id).expect("successor surface");
    assert_ne!(successor, surface);
    assert!(ui.arm_presented_scene_caret(window_id, surface, UiNodeId(1), origin + 1.0).is_none(), "a retired exact token cannot arm its id-reusing successor");
}
//#endregion 🔖️FacadeTests
