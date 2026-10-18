// 💬️ Tooltip and accessibility fixtures laws: the accepted control tooltip dwell and overlay paint, the tooltip text across candidate discard and the retained walk, layout drive and accessibility, blur commit, select and settings document fixtures.

//#region 🔖️FacadeTests
#[test]
fn accepted_control_tooltip_reveals_after_dwell_paints_in_overlay_and_dismisses_immediately() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/💡️retained-control-tooltip/🔣️.json")).expect("retained tooltip fixture");
    let window_id = "retained-tooltip";
    let generation = 1;
    let label = fixture["text"]["label"].as_str().expect("tooltip label");
    let shortcut = fixture["text"]["shortcut"].as_str().expect("tooltip shortcut");
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    assert!(ui.publish_document(window_id, tooltip_document(window_id, generation, label, shortcut)));
    drive_scene_lifetime_reconcile(&mut ui, window_id, generation);
    let mut atlas = FontAtlas::builtin();
    drive_layout(&mut ui, window_id, 240.0, 180.0, &mut atlas);
    acknowledge_scene_lifetime_candidate(&mut ui, window_id, 90);
    drive_scene_lifetime_reconcile(&mut ui, window_id, generation);
    drive_layout(&mut ui, window_id, 240.0, 180.0, &mut atlas);

    for placement in fixture["placements"].as_array().expect("tooltip placements") {
        let numbers = |key: &str| -> Vec<f32> { placement[key].as_array().expect("placement numbers").iter().map(|value| value.as_f64().expect("number") as f32).collect() };
        let anchor = numbers("anchor");
        let content = numbers("content");
        let viewport = numbers("viewport");
        let window = ui.windows.get_mut(window_id).expect("tooltip window");
        place_tooltip_anchor(&mut window.tree, [anchor[0], anchor[1], anchor[2], anchor[3]], [viewport[0], viewport[1]]);
        let control = window.tree.document_node(UiNodeId(2)).expect("tooltip control");
        let resolved = resolve_overlay_placement_side(&window.tree, OverlayAnchor::Node(control), (content[0], content[1]), (viewport[0], viewport[1]), OverlayKind::Tooltip.default_placement(), window.router.flow().inline);
        assert!((resolved.x - placement["expected"]["left"].as_f64().expect("left") as f32).abs() < 0.01);
        assert!((resolved.y - placement["expected"]["top"].as_f64().expect("top") as f32).abs() < 0.01);
        let expected_side = placement["expected"]["side"].as_str().expect("side");
        assert!(matches!((expected_side, resolved.side), ("top", crate::wgpu::events::OverlaySide::Top) | ("bottom", crate::wgpu::events::OverlaySide::Bottom)));
    }

    let placement = &fixture["placements"][0];
    let anchor = placement["anchor"].as_array().expect("anchor");
    let anchor = [anchor[0].as_f64().unwrap() as f32, anchor[1].as_f64().unwrap() as f32, anchor[2].as_f64().unwrap() as f32, anchor[3].as_f64().unwrap() as f32];
    let window = ui.windows.get_mut(window_id).expect("tooltip window");
    place_tooltip_anchor(&mut window.tree, anchor, [240.0, 180.0]);
    place_tooltip_anchor(&mut window.presented_tree, anchor, [240.0, 180.0]);
    let clock_origin = fixture["timing"]["clockOriginSeconds"].as_f64().expect("clock origin");
    let dwell = fixture["timing"]["dwellMs"].as_f64().expect("dwell") / 1000.0;
    assert_eq!(ui.advance_window_clock(window_id, clock_origin).expect("clock starts").tooltip, TooltipStep::Idle);
    ui.dispatch_pointer_event(window_id, 1, UiEvent::PointerMove { x: anchor[0] + anchor[2] * 0.5, y: anchor[1] + anchor[3] * 0.5, modifiers: Default::default() });
    assert_eq!(ui.advance_window_clock(window_id, clock_origin + dwell - 0.001).expect("pre-dwell").tooltip, TooltipStep::Idle);
    assert!(ui.windows.get(window_id).expect("tooltip window").presented_tooltip.is_none());
    assert!(matches!(ui.advance_window_clock(window_id, clock_origin + dwell).expect("dwell").tooltip, TooltipStep::Reveal(_)));
    let expected_label = fixture["text"]["formatted"].as_str().expect("formatted label");
    assert_eq!(ui.windows.get(window_id).and_then(|window| window.presented_tooltip.as_ref()).map(|tooltip| tooltip.label.as_str()), Some(expected_label));

    drive_scene_lifetime_reconcile(&mut ui, window_id, generation);
    drive_layout(&mut ui, window_id, 240.0, 180.0, &mut atlas);
    place_tooltip_anchor(&mut ui.windows.get_mut(window_id).expect("tooltip window").tree, anchor, [240.0, 180.0]);
    let surface = ui.surface_token(window_id).expect("tooltip surface");
    let theme = ui.theme;
    let tooltip = retained_tooltip_paint(ui.windows.get_mut(window_id).expect("tooltip window"), surface, &mut atlas, &theme).expect("accepted tooltip paint");
    let size = tooltip_surface_size(expected_label, &theme, &mut atlas);
    assert!((tooltip.x + size.0 * 0.5 - (anchor[0] + anchor[2] * 0.5)).abs() < 0.01, "tooltip remains centered over its accepted anchor");
    assert!((tooltip.y + size.1 + 8.0 - anchor[1]).abs() < 0.01, "tooltip uses React's top gap");
    let mut tooltip_draw = DrawList::default();
    paint_retained_tooltip(&mut tooltip_draw, &mut atlas, tooltip, (0.0, 0.0), &theme);
    assert!(tooltip_draw.layers.iter().any(|layer| !layer.overlay_ui_instances.is_empty()), "the tooltip surface and text use the overlay route");
    let mut frame_draw = DrawList::default();
    let mut ready = false;
    for _ in 0..262_144 {
        match ui.frame_into_step::<RecordingSceneHost>(window_id, Rect::new(0.0, 0.0, 240.0, 180.0), &mut atlas, None, None, &mut frame_draw) {
            UiFrameStep::Pending => {}
            UiFrameStep::Ready => {
                ready = true;
                break;
            }
            step => panic!("tooltip frame answered {step:?}: {}", ui.paint_stall_census(window_id)),
        }
    }
    assert!(ready, "tooltip frame completes: {}", ui.paint_stall_census(window_id));
    assert!(frame_draw.layers.iter().any(|layer| !layer.overlay_ui_instances.is_empty()), "the production ladder composites the tooltip after retained content");

    ui.dispatch_pointer_event(window_id, 1, UiEvent::PointerMove { x: 0.0, y: 179.0, modifiers: Default::default() });
    assert!(ui.windows.get(window_id).expect("tooltip window").presented_tooltip.is_none(), "pointer leave dismisses without another clock tick");
    let window = ui.windows.get_mut(window_id).expect("tooltip window");
    window.presented_tooltip = Some(PresentedTooltip { surface, document_id: UiNodeId(2), label: expected_label.into(), accessibility_generation: window.presented_accessibility_generation, interaction_epoch: window.presented_interaction_epoch, row_action: None });
    assert!(ui.begin_surface_close(surface));
    assert!(ui.windows.get(window_id).expect("closing tooltip window").presented_tooltip.is_none(), "surface close retires the accepted tooltip record");
}

#[test]
fn tooltip_keeps_accepted_text_across_candidate_discard_and_retires_after_replacement() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/💡️retained-control-tooltip/🔣️.json")).expect("retained tooltip fixture");
    let window_id = "accepted-tooltip-text";
    let label = fixture["text"]["label"].as_str().expect("tooltip label");
    let shortcut = fixture["text"]["shortcut"].as_str().expect("tooltip shortcut");
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    assert!(ui.publish_document(window_id, tooltip_document(window_id, 1, label, shortcut)));
    drive_scene_lifetime_reconcile(&mut ui, window_id, 1);
    let mut atlas = FontAtlas::builtin();
    drive_layout(&mut ui, window_id, 160.0, 120.0, &mut atlas);
    acknowledge_scene_lifetime_candidate(&mut ui, window_id, 91);
    drive_scene_lifetime_reconcile(&mut ui, window_id, 1);
    drive_layout(&mut ui, window_id, 160.0, 120.0, &mut atlas);
    let window = ui.windows.get_mut(window_id).expect("tooltip window");
    place_tooltip_anchor(&mut window.tree, [4.0, 4.0, 32.0, 20.0], [160.0, 120.0]);
    place_tooltip_anchor(&mut window.presented_tree, [4.0, 4.0, 32.0, 20.0], [160.0, 120.0]);
    let origin = fixture["timing"]["clockOriginSeconds"].as_f64().expect("clock origin");
    let dwell = fixture["timing"]["dwellMs"].as_f64().expect("dwell") / 1000.0;
    ui.advance_window_clock(window_id, origin);
    ui.dispatch_pointer_event(window_id, 1, UiEvent::PointerMove { x: 20.0, y: 14.0, modifiers: Default::default() });
    assert!(matches!(ui.advance_window_clock(window_id, origin + dwell).expect("tooltip reveal").tooltip, TooltipStep::Reveal(_)));
    let accepted = fixture["text"]["formatted"].as_str().expect("formatted tooltip");

    drive_scene_lifetime_reconcile(&mut ui, window_id, 1);
    assert!(ui.publish_document(window_id, tooltip_document(window_id, 2, "Fenster schließen", "Strg+W")));
    drive_scene_lifetime_reconcile(&mut ui, window_id, 2);
    place_tooltip_anchor(&mut ui.windows.get_mut(window_id).expect("tooltip window").tree, [4.0, 4.0, 32.0, 20.0], [160.0, 120.0]);
    let surface = ui.surface_token(window_id).expect("tooltip surface");
    let theme = ui.theme;
    let candidate = retained_tooltip_paint(ui.windows.get_mut(window_id).expect("tooltip window"), surface, &mut atlas, &theme).expect("accepted text paints against candidate geometry");
    assert_eq!(candidate.label, accepted, "an unaccepted candidate cannot leak replacement text");
    assert!(ui.seal_presented_input_candidate(92, &[window_id.to_string()]));
    assert!(ui.discard_presented_input_candidate(92));
    assert_eq!(ui.windows.get(window_id).and_then(|window| window.presented_tooltip.as_ref()).map(|tooltip| tooltip.label.as_str()), Some(accepted), "candidate discard preserves the accepted tooltip");

    assert!(ui.publish_document(window_id, tooltip_document(window_id, 3, "Fenster schließen", "Strg+W")));
    drive_scene_lifetime_reconcile(&mut ui, window_id, 3);
    place_tooltip_anchor(&mut ui.windows.get_mut(window_id).expect("tooltip window").tree, [4.0, 4.0, 32.0, 20.0], [160.0, 120.0]);
    acknowledge_scene_lifetime_candidate(&mut ui, window_id, 93);
    assert!(retained_tooltip_paint(ui.windows.get_mut(window_id).expect("tooltip window"), surface, &mut atlas, &theme).is_none());
    assert!(ui.windows.get(window_id).expect("tooltip window").presented_tooltip.is_none(), "accepting the replacement retires the old document label");
}

fn retained_walk_leaf(discriminant: u32, ordinal: u32, value: &str) -> crate::wgpu::tree::Node {
    crate::wgpu::tree::Node::new(
        crate::wgpu::tree::NodeKey::Positional(discriminant, ordinal),
        crate::wgpu::tree::WidgetSpec(UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })),
    )
}

fn drive_layout(ui: &mut Ui, window_id: &str, width: f32, height: f32, atlas: &mut FontAtlas) {
    ui.set_viewport(window_id, width, height);
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let pool = test_layout_pool();
    let mut preview_sequence = 0;
    for _ in 0..262_144 {
        if ui.layout_is_dirty(window_id) {
            ui.request_layout(window_id);
        }
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), test_clock, &mut preview_sequence,&mut actual_retained_progress);
        if matches!(ui.step_layouts(&pool, atlas, &mut cx), UiLayoutStep::Idle) && !ui.layout_is_dirty(window_id) {
            return;
        }
    }
    panic!("retained layout never completed: {}", ui.paint_stall_census(window_id));
}

fn accessibility_document() -> UiDocumentTree {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️contract/🧪️tests/🧫️fixtures/♿️accessibility-projection.json")).expect("accessibility projection fixture parses");
    let source = &law["document"];
    let nodes = source["nodes"].as_array().expect("fixture nodes");
    let header = UiDocumentLeaseHeader {
        generation: 1,
        surface: SurfaceId::try_from(source["surface"].as_str().unwrap()).unwrap(),
        revision: UiRevision(source["revision"].as_u64().unwrap()),
        root: UiNodeId(source["root"].as_u64().unwrap()),
        layout_epoch: source["layoutEpoch"].as_u64().unwrap(),
        node_count: nodes.len(),
    };
    let mut document = UiDocumentTree::new(header).unwrap();
    for node in nodes {
        document.try_upsert_record(serde_json::from_value::<UiNodeRecord>(node.clone()).unwrap()).unwrap();
    }
    document
}

fn publish_accessibility_document(ui: &mut Ui, window_id: &str) {
    assert!(ui.publish_document(window_id, accessibility_document()));
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    for _ in 0..4096 {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(64, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), test_clock, &mut preview_sequence,&mut actual_retained_progress);
        match ui.step_document_reconcile(window_id, "procedural", &mut cx) {
            UiDocumentReconcileStep::Pending => {}
            UiDocumentReconcileStep::Complete => return,
            UiDocumentReconcileStep::Fault(fault) => panic!("accessibility document reconcile fault: {fault:?}"),
        }
    }
    panic!("accessibility document reconcile exceeded its fixed budget");
}

fn publish_blur_commit_input_document(ui: &mut Ui, window_id: &str) {
    let nodes = serde_json::json!([
        {
            "id": 1,
            "key": "root",
            "component": { "type": "container" },
            "layout": { "kind": "stack", "axis": "vertical", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "grow": false, "wrap": false },
            "style": {}, "activity": "idle", "accessibility": {}, "children": [2]
        },
        {
            "id": 2,
            "key": "framework.settings.driver.saveLabel",
            "component": { "type": "input", "kind": "text", "value": "", "commit": "blur" },
            "layout": { "kind": "leaf", "width": "fill", "height": "hug" },
            "style": {}, "activity": "idle", "accessibility": { "label": "Save label" },
            "bindings": [{ "trigger": "commit", "action": { "scope": "framework", "name": "setDriverSaveLabel", "version": 1 } }]
        }
    ]);
    let records = nodes.as_array().unwrap();
    let header = UiDocumentLeaseHeader { generation: 1, surface: SurfaceId::try_from(window_id).unwrap(), revision: UiRevision(0), root: UiNodeId(1), layout_epoch: 0, node_count: records.len() };
    let mut document = UiDocumentTree::new(header).unwrap();
    for record in records {
        document.try_upsert_record(serde_json::from_value::<UiNodeRecord>(record.clone()).unwrap()).unwrap();
    }
    assert!(ui.publish_document(window_id, document));
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    for _ in 0..4096 {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(64, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), test_clock, &mut preview_sequence,&mut actual_retained_progress);
        match ui.step_document_reconcile(window_id, "framework", &mut cx) {
            UiDocumentReconcileStep::Pending => {}
            UiDocumentReconcileStep::Complete => return,
            UiDocumentReconcileStep::Fault(fault) => panic!("blur input reconcile fault: {fault:?}"),
        }
    }
    panic!("blur input reconcile exceeded its fixed budget");
}

fn select_accessibility_law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/♿️retained-select-accessibility/🔣️.json")).expect("retained Select accessibility fixture parses")
}

fn publish_select_accessibility_document(ui: &mut Ui, law: &serde_json::Value) {
    let window_id = law["window"]["id"].as_str().expect("window id");
    let select = &law["select"];
    let nodes = serde_json::json!([
        {
            "id": 10,
            "key": "framework.settings.general/root",
            "component": { "type": "container" },
            "layout": { "kind": "stack", "axis": "vertical", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "grow": false, "wrap": false },
            "style": {}, "activity": "idle", "accessibility": {}, "children": [select["nodeId"]]
        },
        {
            "id": select["nodeId"],
            "key": select["key"],
            "component": { "type": "select", "value": select["value"], "items": select["options"].as_array().expect("options").iter().map(|option| serde_json::json!({ "value": option["value"], "label": option["label"] })).collect::<Vec<_>>() },
            "layout": { "kind": "leaf", "width": "fill", "height": "hug" },
            "style": {}, "activity": "idle", "accessibility": { "label": select["label"] },
            "bindings": [{ "trigger": "change", "action": { "scope": "settings", "name": "setAppearance", "version": 1 } }]
        }
    ]);
    let nodes = nodes.as_array().expect("select document nodes");
    let header = UiDocumentLeaseHeader {
        generation: law["window"]["generation"].as_u64().expect("generation"),
        surface: SurfaceId::try_from(window_id).expect("surface id"),
        revision: UiRevision(0),
        root: UiNodeId(10),
        layout_epoch: 0,
        node_count: nodes.len(),
    };
    let mut document = UiDocumentTree::new(header).expect("select document header");
    for node in nodes {
        document.try_upsert_record(serde_json::from_value::<UiNodeRecord>(node.clone()).expect("select record")).expect("select record admits");
    }
    assert!(ui.publish_document(window_id, document));
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    for _ in 0..4096 {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(64, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), test_clock, &mut preview_sequence,&mut actual_retained_progress);
        match ui.step_document_reconcile(window_id, "settings", &mut cx) {
            UiDocumentReconcileStep::Pending => {}
            UiDocumentReconcileStep::Complete => return,
            UiDocumentReconcileStep::Fault(fault) => panic!("select accessibility document reconcile fault: {fault:?}"),
        }
    }
    panic!("select accessibility document reconcile exceeded its fixed budget");
}

fn puzzle3d_settings_law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/⚙️puzzle3d-settings-document/🔣️.json")).expect("Puzzle3D Settings fixture parses")
}

fn puzzle3d_settings_document(law: &serde_json::Value) -> UiDocumentTree {
    let source = &law["document"];
    let nodes = source["nodes"].as_array().expect("Puzzle3D Settings nodes");
    let header = UiDocumentLeaseHeader {
        generation: source["generation"].as_u64().expect("generation"),
        surface: SurfaceId::try_from(source["surface"].as_str().expect("surface")).expect("surface id"),
        revision: UiRevision(source["revision"].as_u64().expect("revision")),
        root: UiNodeId(source["root"].as_u64().expect("root")),
        layout_epoch: source["layoutEpoch"].as_u64().expect("layout epoch"),
        node_count: nodes.len(),
    };
    let mut document = UiDocumentTree::new(header).expect("Puzzle3D Settings header admits");
    for node in nodes {
        document.try_upsert_record(serde_json::from_value::<UiNodeRecord>(node.clone()).expect("Puzzle3D Settings record deserializes")).expect("Puzzle3D Settings record admits");
    }
    document
}

fn reconcile_puzzle3d_settings(ui: &mut Ui, window_id: &str, law: &serde_json::Value) {
    assert!(ui.publish_document(window_id, puzzle3d_settings_document(law)));
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    for _ in 0..4096 {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(64, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), test_clock, &mut preview_sequence,&mut actual_retained_progress);
        match ui.step_document_reconcile(window_id, "puzzle3d-play", &mut cx) {
            UiDocumentReconcileStep::Pending => {}
            UiDocumentReconcileStep::Complete => return,
            UiDocumentReconcileStep::Fault(fault) => panic!("Puzzle3D Settings reconcile fault: {fault:?}"),
        }
    }
    panic!("Puzzle3D Settings reconcile exceeded its fixed budget");
}

fn explicit_node(tree: &UiTree, key: &str) -> crate::wgpu::arena::NodeId {
    let mut pending = vec![tree.root.expect("mounted document root")];
    while let Some(id) = pending.pop() {
        if matches!(tree.node(id).map(|node| &node.key), Some(crate::wgpu::tree::NodeKey::Explicit(found)) if found == key) {
            return id;
        }
        pending.extend(tree.children(id));
    }
    panic!("mounted document has no `{key}` node");
}

fn rect_inside(inner: [f32; 4], outer: Rect) -> bool {
    inner[0] >= outer.x - 0.01 && inner[1] >= outer.y - 0.01 && inner[0] + inner[2] <= outer.x + outer.w + 0.01 && inner[1] + inner[3] <= outer.y + outer.h + 0.01
}

fn rect_matches(actual: [f32; 4], expected: Rect) -> bool {
    (actual[0] - expected.x).abs() < 0.01 && (actual[1] - expected.y).abs() < 0.01 && (actual[2] - expected.w).abs() < 0.01 && (actual[3] - expected.h).abs() < 0.01
}
//#endregion 🔖️FacadeTests
