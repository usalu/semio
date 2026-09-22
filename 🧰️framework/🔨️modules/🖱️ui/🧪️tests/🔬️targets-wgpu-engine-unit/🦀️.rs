use super::*;
use crate::dsl::DslValue;
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{
    ui_node_to_control, SurfaceKind, UiButtonNode, UiComponentSceneNode, UiControlNode, UiExternalSlotNode, UiFieldNode, UiGroupNode, UiIconSelectNode, UiImageNode, UiInputNode, UiKeyValueEntry, UiKeyValueNode, UiNumberStepperNode, UiPresence,
    UiProgressNode, UiRingNode, UiSectionNode, UiSelectItem, UiSelectNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiState, UiTextNode, UiToggleNode, UiTreeActionPlacement, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
};
use crate::wgpu::events::{AccessibilityUiEvent, PointerButton};
use crate::wgpu::geometry::Rect;
use crate::wgpu::input::{HitKind, InputState};
use crate::wgpu::scene_slots::SceneSlot;
use crate::wgpu::widgets::{
    draw_text_on, draw_text_overlay_on, measure_widget, render_scroll_region, render_widget, wrap_text, ControlNode, InputMeta, KeyValueEntry, RingMeta, SelectItem, SliderMeta, StepperMeta, TreeItem, TreeItemAction, TreeSection, WidgetContext,
    WidgetInteractionMaps, WidgetNode,
};
use crate::wgpu::Label;
use std::collections::HashMap as StdHashMap;
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiNodeId, UiNodeRecord, UiRevision};

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
    let component = if scene {
        serde_json::json!({ "type": "surface", "kind": "world-3d", "docSchema": "world3d@1", "doc": { "bytes": [] } })
    } else {
        serde_json::json!({ "type": "container" })
    };
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
    let mut document = UiDocumentTree::new(UiDocumentLeaseHeader {
        generation,
        surface: SurfaceId::try_from(window_id).expect("scene lifetime surface"),
        revision: UiRevision(generation),
        root: UiNodeId(1),
        layout_epoch: generation,
        node_count: 1,
    })
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
        let component = if *id == 4 {
            serde_json::json!({ "type": "slider", "value": value, "min": 0, "max": 10, "step": 1 })
        } else {
            serde_json::json!({ "type": "container" })
        };
        let bindings = if *id == 4 {
            serde_json::json!([{ "trigger": "change", "action": { "scope": "fixture", "name": "setValue", "version": 1 } }])
        } else {
            serde_json::json!([])
        };
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

fn reconcile_scene_lifetime_candidate(ui: &mut Ui, window_id: &str, generation: u64, scene: bool) {
    assert!(ui.publish_document(window_id, scene_lifetime_document(window_id, generation, scene)));
    drive_scene_lifetime_reconcile(ui, window_id, generation);
}

fn drive_scene_lifetime_reconcile(ui: &mut Ui, window_id: &str, generation: u64) {
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    for _ in 0..4096 {
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(64, u64::MAX), cancel.clone(), test_clock, &mut sequence);
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
    let mut ui = Ui::new();
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
    let mut ui = Ui::new();
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
    let mut ui = Ui::new();
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
    let mut ui = Ui::new();
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
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
        if matches!(ui.step_layouts(&pool, atlas, &mut cx), UiLayoutStep::Idle) && !ui.layout_is_dirty(window_id) {
            return;
        }
    }
    panic!("retained layout never completed: {}", ui.paint_stall_census(window_id));
}

fn accessibility_document() -> UiDocumentTree {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧬️contract/🧫️fixtures/♿️accessibility-projection.json")).expect("accessibility projection fixture parses");
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
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(64, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
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
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(64, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
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
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(64, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
        match ui.step_document_reconcile(window_id, "settings", &mut cx) {
            UiDocumentReconcileStep::Pending => {}
            UiDocumentReconcileStep::Complete => return,
            UiDocumentReconcileStep::Fault(fault) => panic!("select accessibility document reconcile fault: {fault:?}"),
        }
    }
    panic!("select accessibility document reconcile exceeded its fixed budget");
}

fn puzzle3d_settings_law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧪️fixtures/⚙️puzzle3d-settings-document/🔣️.json")).expect("Puzzle3D Settings fixture parses")
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
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(64, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
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

/// ⚙️ The actual Puzzle3D Settings payload enters through `UiDocumentTree`, completes the
/// production frame ladder, publishes all four controls and retains every stepper's two border boxes
/// and three glyph runs. This is the app-level witness for the phase-0 ten-item chrome grant.
#[test]
fn puzzle3d_settings_document_completes_retained_paint() {
    let law = puzzle3d_settings_law();
    let window_id = law["document"]["surface"].as_str().expect("surface");
    let mut ui = Ui::new();
    reconcile_puzzle3d_settings(&mut ui, window_id, &law);

    let mut atlas = FontAtlas::builtin();
    let body = Rect::new(0.0, 0.0, 360.0, 480.0);
    drive_layout(&mut ui, window_id, body.w, body.h, &mut atlas);
    let mut draw = DrawList::default();
    let mut ready = false;
    for _ in 0..262_144 {
        match ui.frame_into_step::<RecordingSceneHost>(window_id, body, &mut atlas, None, None, &mut draw) {
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

        let solids: Vec<_> = all_instances.iter().copied().filter(|instance| instance.params[2] == crate::wgpu::draw::KIND_SOLID && rect_inside(instance.rect, bounds)).collect();
        assert_eq!(solids.len(), crate::wgpu::paint::RETAINED_NUMBER_STEPPER_CHROME_OUTPUT_ITEMS, "{key} retains exactly two five-quad control borders");
        assert!(solids.iter().any(|instance| rect_matches(instance.rect, bounds)), "{key} retains the outer background");
        let value_segment = crate::wgpu::layout::number_stepper_segments(bounds)[1];
        assert!(solids.iter().any(|instance| rect_matches(instance.rect, value_segment)), "{key} retains the nested value background");

        let glyphs: Vec<_> = all_instances.iter().copied().filter(|instance| instance.params[2] == crate::wgpu::draw::KIND_GLYPH && rect_inside(instance.rect, bounds)).collect();
        let thirds = crate::wgpu::layout::number_stepper_segments(bounds);
        let in_segment = |segment: Rect| glyphs.iter().filter(|instance| {
            let centre = instance.rect[0] + instance.rect[2] * 0.5;
            centre >= segment.x && centre <= segment.x + segment.w
        }).count();
        assert!(in_segment(thirds[0]) >= 1, "{key} retains the minus glyph run");
        assert!(in_segment(thirds[1]) >= control["formatted"].as_str().expect("formatted value").chars().count(), "{key} retains the formatted value glyph run");
        assert!(in_segment(thirds[2]) >= 1, "{key} retains the plus glyph run");
        eprintln!("[DEBUG] Puzzle3D Settings retained {key}: bounds={bounds:?} solids={} glyphs={}", solids.len(), glyphs.len());
    }
}

#[test]
fn puzzle3d_settings_stepper_commits_on_press_and_release_only_retires_capture() {
    let law = puzzle3d_settings_law();
    let gestures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️stepper-pointer-commit/🔣️.json")).expect("stepper pointer fixture");
    for gesture in gestures["cases"].as_array().expect("gesture cases") {
        let mut ui = Ui::new();
        let window_id = law["document"]["surface"].as_str().expect("surface");
        reconcile_puzzle3d_settings(&mut ui, window_id, &law);
        drive_layout(&mut ui, window_id, 360.0, 480.0, &mut FontAtlas::builtin());
        let control = &law["controls"][0];
        let tree = ui.tree(window_id).expect("mounted tree");
        let id = explicit_node(tree, control["key"].as_str().expect("control key"));
        let bounds = tree.absolute_rect(id).expect("stepper bounds");
        let segment = match gesture["segment"].as_str().unwrap() { "minus" => 0, "value" => 1, "plus" => 2, _ => unreachable!() };
        let rect = crate::wgpu::layout::number_stepper_segments(bounds)[segment];
        let (x, y) = (rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
        let pressed = ui.dispatch_event(window_id, UiEvent::PointerDown { x, y, button: PointerButton::Primary, modifiers: Default::default() });
        let actions = pressed.iter().filter_map(|command| match command { UiCommand::App { intent, .. } => Some(intent.descriptor()), _ => None }).collect::<Vec<_>>();
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
    let mut ui = Ui::new();
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
        match ui.close_surface_one(token) {
            UiSurfaceCloseStep::Pending => {}
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
    let mut ui = Ui::new();
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
    assert!(matches!(ui.close_surface_one(old_token), UiSurfaceCloseStep::Complete));
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
    let mut ui = Ui::new();
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
    let mut ui = Ui::new();
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
    assert!(ui.advance_clock(1000.0).is_empty());
    close_surface_to_terminal(&mut ui, token);
    assert_eq!(ui.drain_commands().len(), law["silentBlurCommit"]["actions"].as_u64().unwrap() as usize);
    assert_eq!(ui.window_ids().count(), law["closedSurfaces"].as_u64().unwrap() as usize);
}

#[test]
fn accessibility_blur_input_stages_values_and_commits_exactly_once_on_blur() {
    let mut ui = Ui::new();
    let window_id = "framework.settings.general";
    publish_blur_commit_input_document(&mut ui, window_id);
    let generation = ui.surface_generation(window_id).unwrap();
    ui.dispatch_accessibility_event(window_id, generation, 2, "framework.settings.driver.saveLabel", AccessibilityUiEvent::Focus).expect("focus");
    let value = ui.dispatch_accessibility_event(window_id, generation, 2, "framework.settings.driver.saveLabel", AccessibilityUiEvent::Value("Focus Flow".into())).expect("value");
    assert!(value.iter().all(|command| !matches!(command, UiCommand::App { .. })), "a blur-committing value remains a draft");
    let blur = ui.dispatch_accessibility_event(window_id, generation, 2, "framework.settings.driver.saveLabel", AccessibilityUiEvent::Blur).expect("blur");
    let actions = blur.iter().filter_map(|command| match command { UiCommand::App { intent, .. } => Some(intent.descriptor()), _ => None }).collect::<Vec<_>>();
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
    let mut ui = Ui::new();
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
    let actions: Vec<_> = commands.iter().filter_map(|command| match command { UiCommand::App { intent, .. } => Some(intent.descriptor()), _ => None }).collect();
    assert_eq!(actions.len(), law["activation"]["expectedActions"].as_u64().unwrap() as usize);
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.as_object()).and_then(|entries| entries.iter().find(|(key, _)| key == "value")).map(|(_, value)| value), Some(&DslValue::String(law["activation"]["expectedValue"].as_str().unwrap().to_string())));
    assert!(commands.iter().any(|command| matches!(command, UiCommand::OverlayClosed { .. })), "option activation closes its one popup authority");
    assert!(ui.dispatch_accessibility_event(window_id, generation, node_id, option_key, AccessibilityUiEvent::Activate).is_none(), "the closed popup rejects its retired virtual option address");
    let closed_again: Vec<_> = crate::wgpu::accessibility::accessibility_projection(ui.tree(window_id).unwrap()).into_iter().filter(|node| node.node_id == node_id).map(|node| node.role).collect();
    assert_eq!(closed_again, closed, "closing retires listbox and options without retiring the Select document");
}

#[test]
fn apply_tree_then_frame_produces_a_non_empty_draw_list() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("main", &stack_ui(vec![UiNode::Text(UiTextNode { value: Label::data("hi"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })]));

    assert!(ui.needs_frame(), "a freshly applied tree must report needing a frame");
    drive_layout(&mut ui, "main", 400.0, 400.0, &mut atlas);
    let draw = ui.frame::<RecordingSceneHost>("main", 400.0, 400.0, &mut atlas, None, None).expect("frame must produce a draw list once a tree was applied");
    let total: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    assert!(total > 0, "expected the text node to emit at least one glyph instance");
}

#[test]
fn frame_before_any_apply_tree_returns_none() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    assert!(ui.frame::<RecordingSceneHost>("nonexistent", 400.0, 400.0, &mut atlas, None, None).is_none());
}

#[test]
fn retained_paint_walk_yields_one_node_or_scalar_per_step_in_tree_order() {
    let mut tree = UiTree::new();
    let root = tree.insert_child(None, retained_walk_leaf(0, 0, "root"));
    let first = tree.insert_child(Some(root), retained_walk_leaf(1, 0, "first"));
    let second = tree.insert_child(Some(root), retained_walk_leaf(1, 1, "second"));
    let nested = tree.insert_child(Some(first), retained_walk_leaf(2, 0, "nested"));
    let expected = [root, first, nested, second];
    let mut observed = [None; 4];
    let mut observed_len = 0;
    let mut walk = RetainedPaintWalk::new(&tree, root);
    let mut complete = false;
    for _ in 0..16 {
        match walk.step(&tree) {
            RetainedPaintWalkStep::Visit(node, _, _, _) => {
                observed[observed_len] = Some(node);
                observed_len += 1;
            }
            RetainedPaintWalkStep::Scalar => {}
            RetainedPaintWalkStep::Complete => {
                complete = true;
                break;
            }
            RetainedPaintWalkStep::DepthFault => panic!("bounded tree should not exhaust retained depth credits"),
        }
    }
    assert!(complete);
    assert_eq!(observed, expected.map(Some));
}

#[test]
fn retained_paint_walk_depth_cap_plus_one_faults_without_dynamic_spill() {
    let mut tree = UiTree::new();
    let root = tree.insert_child(None, retained_walk_leaf(0, 0, "root"));
    let mut parent = root;
    for ordinal in 1..=RETAINED_PAINT_DEPTH_CREDITS {
        let ordinal = match u32::try_from(ordinal) {
            Ok(ordinal) => ordinal,
            Err(_) => panic!("retained depth credit must fit a node ordinal"),
        };
        parent = tree.insert_child(Some(parent), retained_walk_leaf(1, ordinal, "child"));
    }
    let mut walk = RetainedPaintWalk::new(&tree, root);
    let mut faulted = false;
    for _ in 0..(RETAINED_PAINT_DEPTH_CREDITS * 3) {
        if matches!(walk.step(&tree), RetainedPaintWalkStep::DepthFault) {
            faulted = true;
            break;
        }
    }
    assert!(faulted);
}

#[test]
fn needs_frame_is_false_once_a_stable_tree_has_been_framed() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    let ui_node = stack_ui(vec![UiNode::Text(UiTextNode { value: Label::data("hi"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })]);
    ui.apply_tree("main", &ui_node);
    drive_layout(&mut ui, "main", 400.0, 400.0, &mut atlas);
    ui.frame::<RecordingSceneHost>("main", 400.0, 400.0, &mut atlas, None, None);
    assert!(!ui.needs_frame(), "nothing changed since the last frame, so no frame should be needed");

    ui.apply_tree("main", &ui_node);
    assert!(!ui.needs_frame(), "re-applying an identical tree must set zero dirty flags (reconcile's own golden rule)");
}

#[test]
fn dispatch_event_emits_a_button_click_command_once() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("main", &stack_ui(vec![button_ui("go", "Go")]));
    drive_layout(&mut ui, "main", 400.0, 400.0, &mut atlas);
    ui.frame::<RecordingSceneHost>("main", 400.0, 400.0, &mut atlas, None, None);

    ui.dispatch_event("main", UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    let commands = ui.dispatch_event("main", UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });

    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { intent, .. } if intent.descriptor() == action())));
    assert!(ui.drain_commands().is_empty(), "synchronous commands have the return value as their single consumption owner");
}

#[test]
fn set_window_layout_wires_into_the_facades_shell() {
    let mut ui = Ui::new();
    ui.set_window_layout(crate::wgpu::even_window_layout(&["app.viewport".to_string()]));
    assert!(ui.shell().window_layout().is_some());
}

#[test]
fn resize_storm_coalesces_to_one_latest_surface_job() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("resize", &stack_ui(vec![button_ui("go", "Go")]));
    for width in 1..=2_000 {
        ui.set_viewport("resize", width as f32, 480.0);
    }
    assert_eq!(ui.layout_queues.iter().map(SurfaceLaneRing::len).sum::<usize>(), 1, "a resize storm must retain one coalesced surface entry");

    drive_layout(&mut ui, "resize", 2_000.0, 480.0, &mut atlas);
    let root = ui.tree("resize").and_then(|tree| tree.root).expect("root");
    assert_eq!(ui.tree("resize").and_then(|tree| tree.accepted_layout(root)).expect("accepted root").width, 2_000.0);
}

#[test]
fn interactive_storm_does_not_starve_background_surface_lane() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("interactive", &stack_ui(vec![button_ui("go", "Go")]));
    ui.apply_tree("background", &stack_ui(vec![button_ui("bg", "Background")]));
    ui.set_surface_lane("interactive", SurfaceLane::Interactive);
    ui.set_surface_lane("background", SurfaceLane::Background);
    ui.set_viewport("interactive", 400.0, 400.0);
    ui.set_viewport("background", 400.0, 400.0);

    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let pool = test_layout_pool();
    let mut preview_sequence = 0;
    let mut background_progress_at = None;
    for slice in 0..12 {
        ui.set_viewport("interactive", 401.0 + slice as f32, 400.0);
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
        let step = ui.step_layouts(&pool, &mut atlas, &mut cx);
        if matches!(step, UiLayoutStep::Yielded { ref window_id, .. } | UiLayoutStep::Ready { ref window_id, .. } if window_id.as_ref() == "background") {
            background_progress_at = Some(slice);
            break;
        }
    }
    assert!(background_progress_at.is_some_and(|slice| slice < LANE_WHEEL.len()), "the weighted wheel must service background within one six-slot cycle");
}

/// 🧵️ LAW: a 1,025-node text workload is admitted ONE work unit per slice — never a batch whose size
/// grows with the tree.
///
/// This used to assert a WALL-CLOCK ceiling (`max_slice < 8ms`) and it measured the machine, not the
/// engine: it failed at 22.9 ms, 27.5 ms, 31.5 ms and 174 ms whenever peer cargos were resident and
/// passed on a quiet box, so a red run said nothing about the code
/// (`📓️w2-w6-integration.md` §6, `📓️w8a`, `📓️w9a`). The eight milliseconds were only ever a PROXY for
/// the property that actually holds the frame budget: each `step_layouts` call advances the admission
/// cursor by exactly one node or one scalar, so a slice costs the same whether the tree has ten nodes
/// or ten thousand. That is what is asserted here, in units the engine itself reports
/// (`UiLayoutStep::Yielded { nodes, glyphs }`), and it is machine-independent.
///
/// `admit_node_one` answers `(1, 0)` for the node it admits, `admit_text_one` answers `(0, 1)` for the
/// single scalar it shapes, and `unwind_one` answers `(1, 0)` when it pops a frame — hence two node
/// units per node in the tree, one glyph unit per scalar in every label, and never two of anything in
/// one slice.
///
/// **React ref:** React's own layout is synchronous and unsliced; this engine slices because it owns
/// the frame budget React gets from the browser for free, which is why the chunking — not a duration —
/// is the law.
#[test]
fn large_layout_and_shaping_job_admits_one_work_unit_per_slice() {
    let labels: Vec<String> = (0..1_024).map(|index| format!("node-{index}")).collect();
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    let children = labels.iter().map(|label| UiNode::Text(UiTextNode { value: Label::data(label.clone()), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })).collect();
    ui.apply_tree("large", &stack_ui(children));
    ui.set_viewport("large", 1_920.0, 1_080.0);

    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let pool = test_layout_pool();
    let mut preview_sequence = 0;
    let mut slices = 0;
    let mut widest_slice = (0usize, 0usize);
    let mut admitted = (0usize, 0usize);
    loop {
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
        let step = ui.step_layouts(&pool, &mut atlas, &mut cx);
        slices += 1;
        if let UiLayoutStep::Yielded { stage, nodes, glyphs, .. } = step {
            widest_slice = (widest_slice.0.max(nodes), widest_slice.1.max(glyphs));
            if stage == "Layout.CollectNodes" {
                admitted = (admitted.0 + nodes, admitted.1 + glyphs);
            }
            continue;
        }
        if matches!(step, UiLayoutStep::Idle) {
            break;
        }
    }
    let tree = ui.tree("large").expect("large tree");
    let root = tree.root.expect("large root");
    let children: Vec<_> = tree.children(root).collect();
    let scalars: usize = labels.iter().map(|label| label.chars().count()).sum();
    assert!(slices > 10_000, "the 1,025-node/text workload must be observably chunked, got {slices} slices");
    assert_eq!(widest_slice, (1, 1), "no slice may admit more than one node and one glyph, got {widest_slice:?}");
    assert_eq!(admitted, (2 * (children.len() + 1), scalars), "admission must visit and unwind every node once and shape every label scalar once");
    assert_eq!(tree.accepted_layout(root).expect("accepted root").width, 1_920.0);
    assert!(tree.accepted_layout(*children.last().expect("last child")).expect("accepted last child").y > tree.accepted_layout(children[0]).expect("accepted first child").y);
}

#[test]
fn mounted_layout_surface_max_plus_one_returns_exact_owner_without_mutation() {
    let mut ui = Ui::new();
    for index in 0..UI_LAYOUT_SURFACE_SLOTS {
        let id = SurfaceId::try_from(format!("mounted-{index}")).unwrap_or_else(|_| panic!("bounded mounted surface"));
        assert!(ui.windows.try_admit(id).is_ok());
    }
    let before: UiFixedList<SurfaceId, UI_LAYOUT_SURFACE_SLOTS> = ui.windows.ids().cloned().fold(UiFixedList::default(), |mut ids, id| {
        assert_eq!(ids.try_push(id), Ok(()));
        ids
    });
    let owner = SurfaceId::try_from("mounted-max-plus-one").unwrap_or_else(|_| panic!("bounded rejected surface"));
    let rejected = ui.windows.try_admit(owner.clone()).unwrap_err();
    assert_eq!(rejected.id, owner);
    assert_eq!(ui.windows.ids().cloned().collect::<Vec<_>>(), before.iter().cloned().collect::<Vec<_>>());
}

#[test]
fn mounted_layout_equal_theme_does_not_invalidate_or_requeue() {
    let mut ui = Ui::new();
    ui.apply_tree("theme", &stack_ui(vec![button_ui("same", "Same")]));
    let before_generation = ui.windows.get("theme").map(|window| window.layout_generation);
    let before_theme_revision = ui.windows.get("theme").map(|window| window.theme_revision);
    let before_queue = ui.layout_queues.iter().map(SurfaceLaneRing::len).sum::<usize>();
    ui.set_theme(ui.theme());
    assert_eq!(ui.windows.get("theme").map(|window| window.layout_generation), before_generation);
    assert_eq!(ui.windows.get("theme").map(|window| window.theme_revision), before_theme_revision);
    assert_eq!(ui.layout_queues.iter().map(SurfaceLaneRing::len).sum::<usize>(), before_queue);
}

#[test]
fn changed_theme_propagates_one_fixed_surface_slot_per_opportunity() {
    let mut ui = Ui::new();
    ui.apply_tree("theme-a", &stack_ui(vec![button_ui("a", "A")]));
    ui.apply_tree("theme-b", &stack_ui(vec![button_ui("b", "B")]));
    let before_a = ui.windows.get("theme-a").map(|window| window.theme_revision);
    let before_b = ui.windows.get("theme-b").map(|window| window.theme_revision);
    let mut changed = ui.theme();
    changed.gap_standard += 1.0;
    ui.set_theme(changed);
    assert_eq!(ui.windows.get("theme-a").map(|window| window.theme_revision), before_a);
    assert_eq!(ui.windows.get("theme-b").map(|window| window.theme_revision), before_b);
    assert!(ui.drive_theme_propagation_one());
    assert!(ui.theme_propagation.as_ref().is_some_and(|cursor| cursor.phase == ThemePropagationPhase::Validate && cursor.slot == 1));
    assert_eq!(ui.windows.get("theme-a").map(|window| window.theme_revision), before_a);
    assert_eq!(ui.windows.get("theme-b").map(|window| window.theme_revision), before_b);
}

#[test]
fn mounted_layout_atomic_snapshot_keeps_last_valid_geometry_until_fresh_swap() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("atomic", &stack_ui(vec![UiNode::Text(UiTextNode { value: Label::data("atomic"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }), button_ui("target", "Target")]));
    drive_layout(&mut ui, "atomic", 320.0, 200.0, &mut atlas);
    let tree = ui.tree("atomic").unwrap_or_else(|| panic!("atomic tree"));
    let root = tree.root.unwrap_or_else(|| panic!("atomic root"));
    let old_generation = tree.accepted_layout_generation();
    let old_layout = tree.accepted_layout(root).unwrap_or_default();
    ui.set_viewport("atomic", 960.0, 540.0);
    let pool = test_layout_pool();
    let cancel = semio_framework_job::CancelToken::root_now();
    let operation = semio_framework_job::allocate_operation_id();
    let mut preview_sequence = 0;
    let mut swaps = 0;
    for _ in 0..100_000 {
        let before = ui.tree("atomic").map(UiTree::accepted_layout_generation).unwrap_or_default();
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
        let step = ui.step_layouts(&pool, &mut atlas, &mut cx);
        let tree = ui.tree("atomic").unwrap_or_else(|| panic!("atomic retained tree"));
        let after = tree.accepted_layout_generation();
        swaps += usize::from(before != after);
        if matches!(step, UiLayoutStep::Ready { .. }) {
            assert_eq!(tree.accepted_layout(root).unwrap_or_default().width, 960.0);
            break;
        }
        assert_eq!(after, old_generation);
        assert_eq!(tree.accepted_layout(root).unwrap_or_default(), old_layout);
    }
    assert_eq!(swaps, 1);
    assert!(ui.progressive_layout_preview("atomic").is_some());
    assert!(ui.progressive_glyph_preview("atomic").is_some());
}

#[test]
fn mounted_layout_revision_max_refuses_theme_tree_and_viewport_without_alias() {
    let mut ui = Ui::new();
    let original = stack_ui(vec![button_ui("original", "Original")]);
    ui.apply_tree("max", &original);
    let original_theme = ui.theme();
    if let Some(window) = ui.windows.get_mut("max") {
        window.theme_revision = u64::MAX;
    }
    let mut changed_theme = original_theme;
    changed_theme.gap_standard += 1.0;
    ui.set_theme(changed_theme);
    assert_eq!(theme_layout_identity(&ui.theme()), theme_layout_identity(&original_theme));
    if let Some(window) = ui.windows.get_mut("max") {
        window.theme_revision = 1;
        window.viewport_revision = u64::MAX;
        window.layout_generation = u64::MAX - 1;
    }
    let before_viewport = ui.viewport("max");
    ui.set_viewport("max", 777.0, 333.0);
    assert_eq!(ui.viewport("max"), before_viewport);
    if let Some(window) = ui.windows.get_mut("max") {
        window.viewport_revision = 1;
        window.layout_generation = 7;
        window.revision = u64::MAX;
    }
    ui.apply_tree("max", &stack_ui(vec![button_ui("changed", "Changed")]));
    assert_eq!(ui.tree_revision("max"), Some(u64::MAX));
    assert!(ui.tree("max").and_then(|tree| tree.root).and_then(|root| ui.tree("max").and_then(|tree| tree.node(root))).is_some_and(|node| node.spec.0 == original));
}

#[test]
fn mounted_layout_replay_and_resize_supersede_are_deterministic() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    let input = stack_ui(vec![button_ui("replay", "Replay")]);
    ui.apply_tree("replay", &input);
    drive_layout(&mut ui, "replay", 640.0, 480.0, &mut atlas);
    let root = ui.tree("replay").and_then(|tree| tree.root).unwrap_or_else(|| panic!("replay root"));
    let first = ui.tree("replay").and_then(|tree| tree.accepted_layout(root)).unwrap_or_default();
    ui.set_viewport("replay", 641.0, 480.0);
    ui.set_viewport("replay", 640.0, 480.0);
    drive_layout(&mut ui, "replay", 640.0, 480.0, &mut atlas);
    let second = ui.tree("replay").and_then(|tree| tree.accepted_layout(root)).unwrap_or_default();
    assert_eq!(first, second);
    assert_eq!(second.width, 640.0);
}
//#endregion 🔖️FacadeTests

//#region 🔖️SceneHostTests
fn component_scene_ui(surface_id: &str) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        host_id: surface_id.into(),
        surface_id: surface_id.into(),
        controller_id: "ctrl".into(),
        component_kind: SurfaceKind::World3d,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    })
}

/// 🎬️ A bare-bones `SceneHost` recording every call it receives, so tests can assert `Ui::frame`
/// actually reaches the host (once per slot, with the right payload) instead of just trusting the
/// wiring compiles. Paints a single filled rect per slot so a hosted frame's `DrawList` is
/// distinguishable from an unpainted one.
struct RecordingSceneHost {
    paint_calls: usize,
    last_surface_id: Option<String>,
}

impl SceneHost for RecordingSceneHost {
    fn paint_slot_step(&mut self, slot: &SceneSlot<'_>, cursor: &mut ScenePaintCursor, draw: &mut DrawList, _atlas: &mut FontAtlas, _icons: Option<&IconAtlas>) -> ScenePaintStep {
        match cursor.bind(slot.node) {
            Ok(true) => {}
            Ok(false) => return ScenePaintStep::Pending,
            Err(_) => return ScenePaintStep::Fault,
        }
        self.paint_calls += 1;
        self.last_surface_id = slot.surface().map(|(surface_id, _)| surface_id.to_string());
        draw.push_rounded([slot.rect.x, slot.rect.y, slot.rect.w, slot.rect.h], Theme::default().accent, 0.0);
        cursor.finish()
    }
}

//#region 🖱️SceneAtTests
/// 🖱️ `Ui::scene_at` answers the `ComponentScene` leaf under a window-local point with its identity
/// AND the absolute rect it was laid out at — the two things a right-click needs before any event is
/// dispatched (see [`Ui::scene_at`]).
#[test]
fn scene_at_answers_the_component_scene_leaf_under_the_point() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.at")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    let hit = ui.scene_at("w", 200.0, 200.0).expect("a point inside the scene resolves it");
    assert_eq!(hit.surface_id, "surface.at");
    assert_eq!(hit.controller_id, "ctrl");
    assert_eq!(hit.kind, SurfaceKind::World3d);
    assert!(hit.rect.contains(200.0, 200.0), "the reported rect must be the one the point fell in, got {:?}", hit.rect);
}

/// 🖱️ A point OUTSIDE every scene answers `None`, which is what sends a right-click to the shell's
/// own `"window"` menu instead of inventing a surface for it.
#[test]
fn scene_at_answers_none_outside_every_scene() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![button_ui("b", "Button")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    assert!(ui.scene_at("w", 10.0, 10.0).is_none());
    assert!(ui.scene_at("missing-window", 10.0, 10.0).is_none());
}

/// 🖱️ The query must NOT move focus, arm a press capture or emit a command: a right-click resolving
/// its surface is not a press, and React's `onContextMenu` calls `preventDefault()` rather than
/// letting one through. A real `PointerDown` at the same point DOES emit one — that contrast is the
/// whole reason `scene_at` exists instead of dispatching an event to find out.
#[test]
fn scene_at_is_read_only_where_a_press_is_not() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.readonly")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    let _ = ui.scene_at("w", 200.0, 200.0);
    assert!(ui.dispatch_event("w", UiEvent::PointerMove { x: 380.0, y: 380.0, modifiers: Default::default() }).iter().all(|command| !matches!(command, UiCommand::FocusChanged { .. })));
    let pressed = ui.dispatch_event("w", UiEvent::PointerDown { x: 200.0, y: 200.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(pressed.iter().any(|command| matches!(command, UiCommand::Scene { .. })), "a real press reaches the scene lane, got {pressed:?}");
}
//#endregion 🖱️SceneAtTests

//#region 🪟️OverlayBodyTests
/// 🪟️ The first `ComponentScene` leaf's arena id — overlay tests need a real content root and the
/// façade exposes node ids only through the tree itself.
fn first_component_scene_node(ui: &Ui, window_id: &str) -> crate::wgpu::arena::NodeId {
    let tree = ui.tree(window_id).expect("window has a tree");
    let root = tree.root.expect("tree has a root");
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        if matches!(tree.node(id).map(|node| &node.spec.0), Some(UiNode::ComponentScene(_))) {
            return id;
        }
        stack.extend(tree.children(id));
    }
    panic!("no ComponentScene leaf in {window_id}");
}

/// 🪟️ f32 round-trip tolerance: the walk origin is `placement - layout`, and the walk adds `layout`
/// back, so the recovered coordinate is the placement to within one f32 rounding step.
fn assert_close(left: (f32, f32), right: (f32, f32), what: &str) {
    assert!((left.0 - right.0).abs() < 0.01 && (left.1 - right.1).abs() < 0.01, "{what}: {left:?} != {right:?}");
}

/** @emoji 🪟️ An OPEN floating overlay's BODY is positioned at its resolved placement, not at the
 * in-flow slot it was authored in — and the body's own content keeps painting through the normal node
 * pipeline, which is what `scene_at` resolving the scene INSIDE the overlay proves (the walk that
 * answers it is the paint walk's own geometry). React gets the same from a portal plus
 * `PopoverContent`'s fixed positioning. */
#[test]
fn an_open_overlays_body_is_positioned_at_its_resolved_placement() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.popover")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    let content = first_component_scene_node(&ui, "w");
    let in_flow = ui.scene_at("w", 200.0, 200.0).expect("the scene resolves in flow").rect;

    ui.open_overlay("w", content, OverlayKind::Popover, OverlayAnchor::Point { x: 40.0, y: 30.0 });
    let _ = ui.frame::<RecordingSceneHost>("w", 400.0, 400.0, &mut atlas, None, None);
    let placement = *ui.overlay_placements("w").first().expect("one open overlay");
    assert_eq!(placement.root, content);

    let placed = ui.scene_at("w", placement.x + 2.0, placement.y + 2.0).expect("the overlay body resolves at its placement");
    assert_eq!(placed.surface_id, "surface.popover");
    assert_close((placed.rect.x, placed.rect.y), (placement.x, placement.y), "the overlay body must sit at its placement");
    assert!((in_flow.y - placement.y).abs() > 0.5, "this fixture must actually move the body, in-flow {in_flow:?} vs placement {placement:?}");
}

/// 🪟️ Closing the overlay returns its body to the in-flow position — the origin override is per
/// frame, never a stored offset, so nothing has to be unwound.
#[test]
fn closing_an_overlay_returns_its_body_to_the_in_flow_position() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.popover")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    let content = first_component_scene_node(&ui, "w");
    let in_flow = ui.scene_at("w", 200.0, 200.0).expect("in flow").rect;

    ui.open_overlay("w", content, OverlayKind::Popover, OverlayAnchor::Point { x: 40.0, y: 30.0 });
    let _ = ui.frame::<RecordingSceneHost>("w", 400.0, 400.0, &mut atlas, None, None);
    let _ = ui.close_overlay("w", content);
    let _ = ui.frame::<RecordingSceneHost>("w", 400.0, 400.0, &mut atlas, None, None);

    let restored = ui.scene_at("w", 200.0, 200.0).expect("in flow again").rect;
    assert_close((restored.x, restored.y), (in_flow.x, in_flow.y), "a closed overlay is back in flow");
}

/// 🪟️ A `Dialog`/`CommandPalette` is CENTERED rather than anchored, and its body follows the same
/// one origin rule — `OverlayKind::default_placement`'s `Centered` branch.
#[test]
fn a_modal_overlays_body_is_centered_in_the_viewport() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.dialog")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    let content = first_component_scene_node(&ui, "w");
    ui.open_overlay("w", content, OverlayKind::Dialog, OverlayAnchor::Point { x: 0.0, y: 0.0 });
    let _ = ui.frame::<RecordingSceneHost>("w", 400.0, 400.0, &mut atlas, None, None);
    let placement = *ui.overlay_placements("w").first().expect("one open overlay");
    assert!(placement.backdrop, "a Dialog draws a scrim");
    let placed = ui.scene_at("w", placement.x + 2.0, placement.y + 2.0).expect("the modal body resolves at its placement");
    assert_close((placed.rect.x, placed.rect.y), (placement.x, placement.y), "a centered modal body sits at its placement");
}
/// 🪟️ W15a item 5. `paint_overlay_backdrop`/`paint_overlay_surface` existed but were reachable
/// only through the `🪟️OverlayApi` façade, which no host ever called — an open `Dialog`/`Popover`
/// therefore painted its content with NO surface under it on every frame a host did not hand-pump
/// (W1n gap 3). This law drives the production ladder (`frame_into_step`) and pins three things:
/// the chrome is painted by the ladder itself; it lands in the OVERLAY bucket, which composites
/// above every panel; and the overlay's own content lands there too, AFTER the chrome, so the
/// surface can never hide the content it exists to carry.
#[test]
fn the_frame_ladder_paints_an_open_overlays_own_surface_chrome_under_its_content() {
    fn settle(ui: &mut Ui, atlas: &mut FontAtlas, body: Rect) -> DrawList {
        let mut draw = DrawList::default();
        for _ in 0..262_144 {
            match ui.frame_into_step::<RecordingSceneHost>("w", body, atlas, None, None, &mut draw) {
                UiFrameStep::Pending => {}
                UiFrameStep::Ready => return draw,
                step => panic!("retained paint answered {step:?} (phase {:?})", ui.paint_frame_phase("w")),
            }
        }
        panic!("retained paint never completed: {}", ui.paint_stall_census("w"));
    }
    let overlay_instances = |draw: &DrawList| -> usize { draw.layers.iter().map(|layer| layer.overlay_ui_instances.len()).sum() };
    let body = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };

    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.popover")]));
    drive_layout(&mut ui, "w", body.w, body.h, &mut atlas);
    let closed = settle(&mut ui, &mut atlas, body);
    assert_eq!(overlay_instances(&closed), 0, "a surface with no open overlay puts nothing in the overlay bucket");

    let content = first_component_scene_node(&ui, "w");
    ui.open_overlay("w", content, OverlayKind::Popover, OverlayAnchor::Point { x: 40.0, y: 30.0 });
    drive_layout(&mut ui, "w", body.w, body.h, &mut atlas);
    let opened = settle(&mut ui, &mut atlas, body);
    assert!(overlay_instances(&opened) > 0, "the ladder itself must paint the open Popover's surface chrome — no host pumps `overlay_placements`");
    assert!(!opened.overlay_routed(), "every overlay-routed region the ladder opened is closed again by the time the frame publishes");

    let placement = *ui.overlay_placements("w").first().expect("one open overlay");
    let bucket: Vec<[f32; 4]> = opened.layers.iter().flat_map(|layer| layer.overlay_ui_instances.iter()).map(|instance| instance.rect).collect();
    let surface_at = bucket.iter().position(|rect| (rect[0] - placement.x).abs() < 0.01 && (rect[1] - placement.y).abs() < 0.01 && (rect[2] - placement.width).abs() < 0.01);
    let surface_at = surface_at.expect("the overlay surface is painted at the placement layout resolved");
    assert!(bucket.len() > surface_at + 1, "the overlay's own content paints into the SAME bucket, after the chrome — otherwise the surface covers it");

    // 🔽️ A `SelectPopup` paints its own glass inside `paint_select`, and its overlay ROOT is the
    // trigger's rect, not the popup's — chrome at that placement would draw a menu panel over a
    // closed trigger. The ladder must leave those three kinds alone.
    let _ = ui.close_overlay("w", content);
    ui.open_overlay("w", content, OverlayKind::SelectPopup, OverlayAnchor::Node(content));
    drive_layout(&mut ui, "w", body.w, body.h, &mut atlas);
    let popup = settle(&mut ui, &mut atlas, body);
    assert_eq!(overlay_instances(&popup), 0, "a SelectPopup owns its own surface — the ladder must not paint a second one over it");
}
//#endregion 🪟️OverlayBodyTests

#[test]
fn frame_with_no_scene_host_falls_back_to_the_placeholder_chrome() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.no-host")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    let draw = ui.frame::<RecordingSceneHost>("w", 400.0, 400.0, &mut atlas, None, None).expect("frame must produce a draw list");
    let instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    assert!(instances > 0, "with no scene host registered, paint_component_scene's placeholder chrome should still paint");
}

#[test]
fn frame_with_a_scene_host_routes_the_component_scene_leaf_through_it() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.host-test")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);

    let mut host = RecordingSceneHost { paint_calls: 0, last_surface_id: None };
    let draw = ui.frame("w", 400.0, 400.0, &mut atlas, None, Some(&mut host)).expect("frame must produce a draw list even with a scene host registered");
    let instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

    assert_eq!(host.paint_calls, 1, "the host should be invoked exactly once for the single ComponentScene leaf");
    assert_eq!(host.last_surface_id.as_deref(), Some("surface.host-test"));
    assert!(instances > 0, "the host's own draw call should still land in the frame's DrawList");
}

#[test]
fn frame_with_a_scene_host_still_paints_ancestor_chrome_around_the_hosted_slot() {
    // 🌳️ Nests the ComponentScene under a Group (not just a bare Stack) — regression for the
    // shadow-walk gap this bridge replaces: `collect_scene_slots` must still find it, and the
    // Group's own header/frame chrome (unrelated to the scene leaf) must still paint normally.
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    let group_node = UiNode::Group(UiGroupNode {
        id: "group".into(),
        label: Label::data("Group"),
        default_open: None,
        presence: UiPresence::default(),
        children: vec![UiNode::Text(UiTextNode { value: Label::data("label"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }), component_scene_ui("surface.nested")],
        menu: None,
    });
    ui.apply_tree("w", &group_node);
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);

    let mut host = RecordingSceneHost { paint_calls: 0, last_surface_id: None };
    ui.frame("w", 400.0, 400.0, &mut atlas, None, Some(&mut host)).expect("frame must produce a draw list");

    assert_eq!(host.paint_calls, 1);
    assert_eq!(host.last_surface_id.as_deref(), Some("surface.nested"));
}
//#endregion 🔖️SceneHostTests

//#region 🔖️GoldenHarness
/// 🏆️ Acceptance gate for this workstream: for a curated fixture of every `UiNode` variant, runs
/// the retained façade (`apply_tree` + `frame`) and the immediate-mode path (`render_widget` over
/// a hand-converted `WidgetNode`) and asserts they emit structurally equivalent `DrawList`s
/// (same instance/vector/raster counts — not bit-identical geometry, per this ticket's brief).
/// `to_widget_node`/`control_to_widget`/`tree_*_to_widget` below mirror
/// `framework/renderer/wgpu/rs/lib.rs`'s private `ui_node_to_widget` conversion; they're
/// duplicated here (test-only) rather than shared because that crate depends on `ui_wgpu`, never
/// the reverse — keeping the two in sync is this harness's job.
fn to_widget_node(node: &UiNode) -> WidgetNode<ActionDescriptor> {
    match node {
        UiNode::Stack(stack) => WidgetNode::Stack { direction: stack.direction.clone(), gap: stack.gap.clone(), padding: stack.padding.clone(), children: stack.children.iter().map(to_widget_node).collect() },
        UiNode::Text(text) => WidgetNode::Text { value: text.value.to_string(), emphasize: text.emphasize.unwrap_or(false) },
        UiNode::Separator(_) => WidgetNode::Separator,
        UiNode::Button(button) => WidgetNode::Button { id: button.id.clone(), icon_id: Some(button.icon_id.clone()), label: button.label.to_string(), event: Some(button.action.clone()) },
        UiNode::Input(input) => WidgetNode::Input {
            id: input.id.clone(),
            input_kind: input.input_kind.clone(),
            value: input.value.clone(),
            placeholder: input.placeholder.clone().map(|l| l.to_string()),
            commit: input.commit.clone(),
            min: input.min,
            max: input.max,
            step: input.step,
            accept: input.accept.clone(),
            on_change: Some(input.on_change.clone()),
        },
        UiNode::Select(select) => WidgetNode::Select {
            id: select.id.clone(),
            value: select.value.clone(),
            items: select.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.to_string() }).collect(),
            placeholder: select.placeholder.clone().map(|l| l.to_string()),
            on_change: Some(select.on_change.clone()),
        },
        UiNode::Toggle(toggle) => WidgetNode::Toggle { id: toggle.id.clone(), icon_id: toggle.icon_id.clone(), pressed: toggle.presence.selected, text: toggle.text.clone().map(|l| l.to_string()), on_change: Some(toggle.on_change.clone()) },
        UiNode::KeyValue(kv) => WidgetNode::KeyValue { entries: kv.entries.iter().map(|entry| KeyValueEntry { label: entry.label.to_string(), value: entry.value.clone() }).collect() },
        UiNode::Slider(slider) => WidgetNode::Slider { id: slider.id.clone(), value: slider.value, min: slider.min, max: slider.max, step: slider.step, ready: None, disabled: false, on_change: Some(slider.on_change.clone()) },
        UiNode::NumberStepper(stepper) => {
            WidgetNode::NumberStepper { id: stepper.id.clone(), value: stepper.value, step: stepper.step, uniform: stepper.uniform, on_absolute: Some(stepper.on_absolute.clone()), on_delta: Some(stepper.on_delta.clone()) }
        }
        UiNode::Ring(ring) => WidgetNode::Ring { id: ring.id.clone(), t: ring.t, disabled: ring.presence.state == UiState::Disabled, on_change: Some(ring.on_change.clone()) },
        UiNode::IconSelect(select) => WidgetNode::IconSelect { id: select.id.clone(), value: select.value.clone(), uniform: select.uniform, classifier_kind: select.classifier_kind.clone(), on_change: Some(select.on_change.clone()) },
        UiNode::Field(field) => match ui_node_to_control(&field.child) {
            Some(control) => WidgetNode::Field { id: field.id.clone(), label: field.label.to_string(), child: control_to_widget(&control) },
            None => WidgetNode::Section { id: field.id.clone(), label: Some(field.label.to_string()), default_open: true, children: vec![to_widget_node(&field.child)] },
        },
        UiNode::Section(section) => {
            WidgetNode::Section { id: section.id.clone(), label: section.label.clone().map(|l| l.to_string()), default_open: section.default_open.unwrap_or(true), children: section.children.iter().map(to_widget_node).collect() }
        }
        UiNode::Group(group) => WidgetNode::Group { id: group.id.clone(), label: group.label.to_string(), default_open: group.default_open.unwrap_or(true), children: group.children.iter().map(to_widget_node).collect() },
        UiNode::Tree(tree) => WidgetNode::Tree {
            // 🧭️ Per-item `selected`/`highlighted` (see `tree_item_to_widget`) already carry the
            // full signal from `item.presence` — the tree-level id lists are gone, not re-derived.
            sections: tree.sections.iter().map(tree_section_to_widget).collect(),
            selected_ids: Vec::new(),
            highlighted_ids: Vec::new(),
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: `UiTreeNode.selectionChange`
            // is deleted — selection now flows through `interactionSelect`/`interaction_domain`, not yet
            // wired into this retained-mode engine.
            selection_change: None,
        },
        // 🧩️ CLOSED by ticket 26/09/17 packet W15a (was: "`WidgetNode<E>` has no Image/
        // ComponentScene/ExternalSlot/Progress variant at all", so all four collapsed to an empty
        // placeholder `Text` and no two-pipeline comparison was possible). All five kinds now have a
        // real arm in the kit, so this mapping is like-for-like and
        // `every_ui_node_kind_has_a_widget_kit_arm_that_paints` asserts each one paints.
        UiNode::Progress(progress) => WidgetNode::Progress { id: progress.id.clone(), completed: progress.completed, total: progress.total },
        UiNode::Image(image) => WidgetNode::Image { id: image.id.clone(), src: image.src.clone(), alt: image.alt.clone().map(|alt| alt.to_string()) },
        UiNode::ComponentScene(scene) => WidgetNode::component_scene(scene),
        UiNode::ExternalSlot(slot) => WidgetNode::ExternalSlot { body_key: slot.body_key.clone() },
    }
}

fn control_to_widget(control: &UiControlNode) -> ControlNode<ActionDescriptor> {
    match control {
        UiControlNode::Button(n) => ControlNode::Button { id: n.id.clone(), icon_id: Some(n.icon_id.clone()), label: n.label.to_string(), event: Some(n.action.clone()) },
        UiControlNode::Input(n) => ControlNode::Input {
            id: n.id.clone(),
            input_kind: n.input_kind.clone(),
            value: n.value.clone(),
            placeholder: n.placeholder.clone().map(|l| l.to_string()),
            commit: n.commit.clone(),
            min: n.min,
            max: n.max,
            step: n.step,
            accept: n.accept.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Select(n) => ControlNode::Select {
            id: n.id.clone(),
            value: n.value.clone(),
            items: n.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.to_string() }).collect(),
            placeholder: n.placeholder.clone().map(|l| l.to_string()),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Toggle(n) => ControlNode::Toggle { id: n.id.clone(), icon_id: n.icon_id.clone(), pressed: n.presence.selected, text: n.text.clone().map(|l| l.to_string()), on_change: Some(n.on_change.clone()) },
        UiControlNode::KeyValue(n) => ControlNode::KeyValue { entries: n.entries.iter().map(|entry| KeyValueEntry { label: entry.label.to_string(), value: entry.value.clone() }).collect() },
        UiControlNode::Slider(n) => ControlNode::Slider { id: n.id.clone(), value: n.value, min: n.min, max: n.max, step: n.step, ready: None, disabled: false, on_change: Some(n.on_change.clone()) },
        UiControlNode::NumberStepper(n) => ControlNode::NumberStepper { id: n.id.clone(), value: n.value, step: n.step, uniform: n.uniform, on_absolute: Some(n.on_absolute.clone()), on_delta: Some(n.on_delta.clone()) },
        UiControlNode::Ring(n) => ControlNode::Ring { id: n.id.clone(), t: n.t, disabled: n.presence.state == UiState::Disabled, on_change: Some(n.on_change.clone()) },
        UiControlNode::IconSelect(n) => ControlNode::IconSelect { id: n.id.clone(), value: n.value.clone(), uniform: n.uniform, classifier_kind: n.classifier_kind.clone(), on_change: Some(n.on_change.clone()) },
    }
}

/// 🎛️ Same per-variant field mapping as `control_to_widget`, but into a `WidgetNode` instead of a
/// `ControlNode` — needed for `TreeItem::control: Option<Box<WidgetNode<E>>>`, which (unlike
/// `Field`'s `child: ControlNode<E>`) embeds a full widget, not a bare control payload.
fn control_to_widget_node(control: &UiControlNode) -> WidgetNode<ActionDescriptor> {
    match control {
        UiControlNode::Button(n) => WidgetNode::Button { id: n.id.clone(), icon_id: Some(n.icon_id.clone()), label: n.label.to_string(), event: Some(n.action.clone()) },
        UiControlNode::Input(n) => WidgetNode::Input {
            id: n.id.clone(),
            input_kind: n.input_kind.clone(),
            value: n.value.clone(),
            placeholder: n.placeholder.clone().map(|l| l.to_string()),
            commit: n.commit.clone(),
            min: n.min,
            max: n.max,
            step: n.step,
            accept: n.accept.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Select(n) => WidgetNode::Select {
            id: n.id.clone(),
            value: n.value.clone(),
            items: n.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.to_string() }).collect(),
            placeholder: n.placeholder.clone().map(|l| l.to_string()),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Toggle(n) => WidgetNode::Toggle { id: n.id.clone(), icon_id: n.icon_id.clone(), pressed: n.presence.selected, text: n.text.clone().map(|l| l.to_string()), on_change: Some(n.on_change.clone()) },
        UiControlNode::KeyValue(n) => WidgetNode::KeyValue { entries: n.entries.iter().map(|entry| KeyValueEntry { label: entry.label.to_string(), value: entry.value.clone() }).collect() },
        UiControlNode::Slider(n) => WidgetNode::Slider { id: n.id.clone(), value: n.value, min: n.min, max: n.max, step: n.step, ready: None, disabled: false, on_change: Some(n.on_change.clone()) },
        UiControlNode::NumberStepper(n) => WidgetNode::NumberStepper { id: n.id.clone(), value: n.value, step: n.step, uniform: n.uniform, on_absolute: Some(n.on_absolute.clone()), on_delta: Some(n.on_delta.clone()) },
        UiControlNode::Ring(n) => WidgetNode::Ring { id: n.id.clone(), t: n.t, disabled: n.presence.state == UiState::Disabled, on_change: Some(n.on_change.clone()) },
        UiControlNode::IconSelect(n) => WidgetNode::IconSelect { id: n.id.clone(), value: n.value.clone(), uniform: n.uniform, classifier_kind: n.classifier_kind.clone(), on_change: Some(n.on_change.clone()) },
    }
}

fn tree_action_to_widget(action: &UiTreeItemAction) -> TreeItemAction<ActionDescriptor> {
    TreeItemAction { icon_id: action.icon_id.clone(), label: action.label.clone().map(|l| l.to_string()), event: action.action.clone(), placement: action.placement() }
}

fn tree_item_to_widget(item: &UiTreeItemNode) -> TreeItem<ActionDescriptor> {
    TreeItem {
        id: item.id.clone(),
        label: item.label.to_string(),
        description: item.description.clone(),
        icon_id: item.icon_id.clone(),
        selected: item.presence.selected,
        highlighted: item.presence.state == UiState::Previewed,
        default_open: item.default_open.unwrap_or(false),
        dimmed: item.dimmed.unwrap_or(false),
        event: item.action.clone(),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: `UiTreeItemNode.hoverAction`/
        // `unhoverAction` are deleted — hover is now framework-owned per `UiTreeNode.interactionDomain`.
        hover_event: None,
        unhover_event: None,
        actions: item.actions.as_ref().map(|actions| actions.iter().map(tree_action_to_widget).collect()).unwrap_or_default(),
        draggable: item.draggable.unwrap_or(false),
        drag_data: item.drag_data.clone().unwrap_or_default(),
        control: item.control.as_ref().map(|control| Box::new(control_to_widget_node(control))),
        children: item.items.as_ref().map(|items| items.iter().map(tree_item_to_widget).collect()).unwrap_or_default(),
        window: item.window,
    }
}

fn tree_section_to_widget(section: &UiTreeSectionNode) -> TreeSection<ActionDescriptor> {
    TreeSection { id: section.id.clone(), label: section.label.clone().map(|l| l.to_string()), default_open: section.default_open.unwrap_or(true), items: section.items.iter().map(tree_item_to_widget).collect(), window: section.window }
}

/// 📊️ Total (ui_instances incl. overlay, vector_vertices incl. overlay, raster_instances) across
/// every layer of a `DrawList` — the "structurally equivalent" signal this harness compares,
/// deliberately coarser than exact geometry per this ticket's tolerance allowance.
fn stats(draw: &DrawList) -> (usize, usize, usize) {
    let instances = draw.layers.iter().map(|layer| layer.ui_instances.len() + layer.overlay_ui_instances.len()).sum();
    let vectors = draw.layers.iter().map(|layer| layer.vector_vertices.len() + layer.overlay_vector_vertices.len()).sum();
    let raster = draw.layers.iter().map(|layer| layer.raster_instances.len()).sum();
    (instances, vectors, raster)
}

fn retained_stats(node: &UiNode) -> (usize, usize, usize) {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("golden", node);
    drive_layout(&mut ui, "golden", 400.0, 400.0, &mut atlas);
    let draw = ui.frame::<RecordingSceneHost>("golden", 400.0, 400.0, &mut atlas, None, None).expect("apply_tree then frame must produce a draw list");
    stats(draw)
}

fn immediate_stats(node: &UiNode, bounds: Rect) -> (usize, usize, usize) {
    let widget = to_widget_node(node);
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut scroll_offsets: StdHashMap<String, f32> = StdHashMap::new();
    let mut collapsed_sections: StdHashMap<String, bool> = StdHashMap::new();
    let mut open_selects: StdHashMap<String, bool> = StdHashMap::new();
    let mut ctx = WidgetContext {
        draw: &mut draw,
        overlay: None,
        atlas: &mut atlas,
        icons: None,
        input: &mut input,
        theme: &theme,
        scroll_offsets: &mut scroll_offsets,
        collapsed_sections: &mut collapsed_sections,
        open_selects: &mut open_selects,
        interaction_maps: None,
        pick_clip: None,
        viewport_height: 0.0,
    };
    render_widget(&widget, bounds, &mut ctx);
    stats(&draw)
}

const VIEWPORT: Rect = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };

/// 🧱️ Wraps a leaf `UiNode` as the sole child of a gap-less/padding-less vertical `Stack`: on the
/// retained side, `flex::LayoutEngine` always forces the *root* to the full viewport size
/// (`compute`'s `root_style.size` override) and gives a `Stack`'s only child `flex_grow: 1.0`, so
/// the child's resolved `LayoutBucket` is exactly the full viewport. On the immediate side,
/// `layout::layout_vertical`/`layout_horizontal`'s `extra_per_child` gives a lone child the same
/// full bounds. Wrapping every leaf fixture this way guarantees both pipelines paint it at
/// identical bounds, which is what makes an exact instance/vector-count comparison meaningful
/// instead of an artifact of divergent layout math.
fn leaf(child: UiNode) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: Some("none".into()),
        padding: Some("none".into()),
        id: None,
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children: vec![child],
        menu: None,
    })
}

fn assert_equivalent(kind: &str, node: &UiNode) {
    let retained = retained_stats(node);
    let immediate = immediate_stats(node, VIEWPORT);
    assert_eq!(retained, immediate, "{kind}: retained (instances, vectors, raster) {retained:?} != immediate {immediate:?}");
}

// `action()` is shared with 🔖️FacadeTests above — both sub-regions live in the same `mod tests`.

#[test]
fn golden_stack() {
    let node = UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: Some("none".into()),
        padding: Some("none".into()),
        id: None,
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children: vec![UiNode::Text(UiTextNode { value: Label::data("hello"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }), UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None })],
        menu: None,
    });
    assert_equivalent("Stack", &node);
}

#[test]
fn golden_text() {
    assert_equivalent("Text", &leaf(UiNode::Text(UiTextNode { value: Label::data("hello world"), emphasize: Some(true), data_attributes: None, presence: UiPresence::default(), menu: None })));
}

#[test]
fn golden_button() {
    assert_equivalent("Button", &leaf(UiNode::Button(UiButtonNode { id: Some("btn".into()), icon_id: IconName::CircleDot, label: Label::data("Go"), action: action(), style: None, presence: UiPresence::default(), menu: None })));
}

#[test]
fn golden_separator() {
    assert_equivalent("Separator", &leaf(UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None })));
}

#[test]
fn golden_input() {
    assert_equivalent(
        "Input",
        &leaf(UiNode::Input(UiInputNode {
            id: "in".into(),
            input_kind: "text".into(),
            value: "abc".into(),
            placeholder: None,
            accessibility_label: None,
            commit: None,
            min: None,
            max: None,
            step: None,
            accept: None,
            on_change: action(),
            on_submit: None,
            on_abort: None,
            on_repeat_last: None,
            presence: UiPresence::default(),
            menu: None,
        })),
    );
}

#[test]
fn golden_select() {
    assert_equivalent(
        "Select",
        &leaf(UiNode::Select(UiSelectNode {
            id: "sel".into(),
            value: "a".into(),
            items: vec![UiSelectItem { value: "a".into(), label: Label::data("Alpha") }, UiSelectItem { value: "b".into(), label: Label::data("Beta") }],
            placeholder: None,
            on_change: action(),
            presence: UiPresence::default(),
            menu: None,
        })),
    );
}

#[test]
fn golden_toggle() {
    // 🚫️ `presence.selected` is intentionally NOT exercised here: the shared `presence_overlay`
    // now draws an outset accent ring for ANY selected element (see
    // `selected_presence_draws_an_outset_ring_on_any_element`, below) — a deliberate new
    // capability `widgets::render_toggle` (the immediate-mode reference this harness compares
    // against) never had, so a selected fixture would fail this equivalence check for the wrong
    // reason. This test stays scoped to the base (unselected) toggle's fill/label parity.
    assert_equivalent("Toggle", &leaf(UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::default(), menu: None })));
}

/// ✨️ `presence.selected` draws its outset accent ring universally — proven here on `Toggle`, a
/// non-`Stack` variant, since `selected` used to be a `UiStackNode`-only field. Instance count
/// grows vs. the unselected fixture (the extra `push_chrome_border` edges), confirming the ring
/// is now a shared channel every element gets for free from `presence_overlay`.
#[test]
fn selected_presence_draws_an_outset_ring_on_any_element() {
    let unselected = UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::default(), menu: None });
    let selected = UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::selected(true), menu: None });
    let (unselected_instances, _, _) = retained_stats(&leaf(unselected));
    let (selected_instances, _, _) = retained_stats(&leaf(selected));
    assert!(selected_instances > unselected_instances, "a selected element should paint more instances than an unselected one (the outset accent ring)");
}

#[test]
fn golden_key_value() {
    assert_equivalent("KeyValue", &leaf(UiNode::KeyValue(UiKeyValueNode { entries: vec![UiKeyValueEntry { label: Label::data("Name"), value: Label::data("Semio").to_string() }], presence: UiPresence::default(), menu: None })));
}

#[test]
fn golden_slider() {
    assert_equivalent("Slider", &leaf(UiNode::Slider(UiSliderNode { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, unit: None, on_change: action(), presence: UiPresence::default(), menu: None })));
}

/// KNOWN GAP: `widgets::render_number_stepper` renders its center value segment via a full
/// `render_input` call (which itself calls `push_control_border` — a background fill plus 4
/// border-edge quads, 5 instances), giving the center value its own nested input-style border box.
/// `paint::paint_number_stepper` instead just `draw_text_on`s the formatted value directly with no
/// surrounding border. Confirmed by running this fixture: retained emits 14 instances (one
/// `push_control_border` for the whole control + 2 divider lines + 3 text runs), immediate emits
/// 19 (the same 14 plus the center value's own nested 5-instance border box) — a real, reproducible
/// paint-logic difference, not a fixture/harness artifact. This is real follow-up work for `paint`
/// (either add the nested border to `paint_number_stepper`, or confirm the immediate path's nested
/// border is unintentional and should be dropped there instead — a product decision outside this
/// façade's scope), not something to paper over here.
#[test]
fn golden_number_stepper_known_gap() {
    let (instances, _, _) = retained_stats(&leaf(UiNode::NumberStepper(UiNumberStepperNode { id: "ns".into(), value: 2.0, step: 1.0, uniform: false, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None })));
    assert!(instances > 0, "NumberStepper should paint its minus/value/plus segments");
}

/// 🔒️ Added by `w1c-paint-parity` (see `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w1c-paint-parity.md`):
/// `paint::paint_number_stepper` now ports `widgets::render_number_stepper`'s nested
/// center-value border box (the exact gap `golden_number_stepper_known_gap`'s doc comment
/// above documents), closing the 14-vs-19-instance divergence for the `uniform: true` case.
/// Left `golden_number_stepper_known_gap` itself untouched (still valid, still a `uniform: false`
/// fixture) and added this as a new, additive `assert_equivalent` case for `uniform: true`
/// instead, per this workstream's "don't modify existing tests" rule.
#[test]
fn golden_number_stepper() {
    assert_equivalent("NumberStepper", &leaf(UiNode::NumberStepper(UiNumberStepperNode { id: "ns".into(), value: 2.0, step: 1.0, uniform: true, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None })));
}

#[test]
fn golden_ring() {
    assert_equivalent("Ring", &leaf(UiNode::Ring(UiRingNode { id: "ring".into(), orb_id: "orb".into(), t: 0.25, on_change: action(), presence: UiPresence::default(), menu: None })));
}

#[test]
fn golden_icon_select() {
    assert_equivalent(
        "IconSelect",
        &leaf(UiNode::IconSelect(UiIconSelectNode { id: "ic".into(), value: IconName::Sparkles.to_string(), uniform: false, classifier_kind: "kind".into(), on_change: action(), presence: UiPresence::default(), menu: None })),
    );
}

#[test]
fn golden_tree() {
    let item = |id: &str, label: &str| UiTreeItemNode {
        window: None,
        granularity: None,
        id: id.into(),
        label: Label::data(label),
        description: None,
        icon_id: None,
        presence: UiPresence::default(),
        default_open: None,
        action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        dimmed: None,
        menu: None,
    };
    let node = UiNode::Tree(UiTreeNode { presentation: Default::default(),
        sections: vec![UiTreeSectionNode { window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item("i1", "Item One"), item("i2", "Item Two")] }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    });
    assert_equivalent("Tree", &node);
}

/// KNOWN GAP: `reconcile` only expands `Field`/`Section` into a real retained child for their
/// `child`/`children` payload (per `reconcile`'s own module doc comment — M2 recurses into
/// `Stack`/`Section`/`Field` only), but `flex::LayoutEngine` only grants `flex_grow: 1.0` to a
/// `Stack`'s children (see `style_with_grow`'s `flex_grow_child` param, gated on
/// `matches!(node.spec.0, UiNode::Stack(_))`). A `Field`/`Section`'s synthetic retained child is
/// therefore laid out at its own intrinsic content size instead of filling the label-adjusted
/// remainder the way `widgets::render_widget`'s hand-rolled `Field`/`Section` branches
/// (`Rect::new(bounds.x, bounds.y + label_h + gap, bounds.w, bounds.h - label_h - gap)` for
/// `Field`, per-child accumulated `y` for `Section`) explicitly carve out. The two pipelines'
/// geometry — and therefore instance counts for size-dependent content like wrapped `Text` — can
/// genuinely diverge here. This is real follow-up work for `flex`, not something this façade can
/// paper over; these two tests verify the retained side alone produces sane, non-empty output.
#[test]
fn golden_field_known_gap() {
    let node = UiNode::Field(UiFieldNode {
        id: "f".into(),
        label: Label::data("Label"),
        description: None,
        required: None,
        error: None,
        child: Box::new(UiNode::Input(UiInputNode {
            id: "in".into(),
            input_kind: "text".into(),
            value: "abc".into(),
            placeholder: None,
            accessibility_label: None,
            commit: None,
            min: None,
            max: None,
            step: None,
            accept: None,
            on_change: action(),
            on_submit: None,
            on_abort: None,
            on_repeat_last: None,
            presence: UiPresence::default(),
            menu: None,
        })),
        presence: UiPresence::default(),
        menu: None,
    });
    let (instances, _, _) = retained_stats(&node);
    assert!(instances > 0, "Field should paint its label plus its child control");
}

#[test]
fn golden_section_known_gap() {
    let node = UiNode::Section(UiSectionNode {
        id: "sec".into(),
        label: Some(Label::data("Section")),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![UiNode::Text(UiTextNode { value: Label::data("child"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })],
        menu: None,
    });
    let (instances, _, _) = retained_stats(&node);
    assert!(instances > 0, "Section should paint its header label plus its children");
}

/// KNOWN GAP: see `to_widget_node`'s own `UiNode::Image | UiNode::ComponentScene | UiNode::ExternalSlot`
/// match arm doc comment — `WidgetNode<E>` has no variant for any of these three, so there is no
/// immediate-mode equivalent to compare against at all. `paint::paint_image`/
/// `paint_component_scene`/`paint_external_slot` are themselves documented placeholders (no host
/// texture-upload queue / scene-host / plugin-body wiring exists in `ui_wgpu` yet either) — these
/// tests only verify the retained side produces the placeholder chrome its own doc comments
/// promise, not equivalence with anything immediate-mode.
#[test]
fn golden_image_known_gap() {
    let node = UiNode::Image(UiImageNode { id: "img".into(), src: String::new(), alt: Some(Label::data("alt text")), presence: UiPresence::default(), menu: None });
    let (instances, _, _) = retained_stats(&node);
    assert!(instances > 0, "an empty-src Image should still paint its alt text");
}

#[test]
fn golden_component_scene_known_gap() {
    let node = UiNode::ComponentScene(UiComponentSceneNode {
        host_id: "surf".into(),
        surface_id: "surf".into(),
        controller_id: "ctrl".into(),
        component_kind: SurfaceKind::World3d,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    });
    let (instances, _, _) = retained_stats(&node);
    assert!(instances > 0, "ComponentScene should paint its placeholder border chrome");
}

#[test]
fn golden_external_slot_known_gap() {
    let node = UiNode::ExternalSlot(UiExternalSlotNode { plugin_id: "plug".into(), app_id: "app".into(), body_key: "body".into(), params_json: "{}".into(), presence: UiPresence::default(), menu: None });
    let (instances, _, _) = retained_stats(&node);
    assert!(instances > 0, "ExternalSlot should paint its placeholder chrome plus its body_key label");
}

/// 🧩️ W15a item 2. `WidgetNode<E>` — the second, smaller paint kit a scene-embedded panel and the
/// standalone `🌳️Tree` target draw through — was 15 arms against `UiNode`'s 20, so a panel painted
/// through it could not show a progress bar, an image, a nested group, a scene or an extension slot
/// AT ALL. This law pins the kit to paint SOMETHING for every one of the five, and pins a scene's
/// hit contract to the same `retained_scene_hit` derivation the retained registry uses, so the two
/// kits cannot answer differently for the same node.
#[test]
fn every_ui_node_kind_has_a_widget_kit_arm_that_paints() {
    let bounds = Rect { x: 0.0, y: 0.0, w: 200.0, h: 60.0 };
    let paints = |node: &UiNode| {
        let (instances, vectors, raster) = immediate_stats(node, bounds);
        instances + vectors + raster
    };
    let progress = UiNode::Progress(UiProgressNode { id: "p".into(), completed: 3.0, total: Some(4.0), value_text: Label::data("3 of 4"), presence: UiPresence::default(), menu: None });
    assert!(paints(&progress) > 0, "a determinate Progress must paint its track and its fill");
    let indeterminate = UiNode::Progress(UiProgressNode { id: "p".into(), completed: 0.0, total: None, value_text: Label::data("busy"), presence: UiPresence::default(), menu: None });
    assert!(paints(&indeterminate) > 0, "an indeterminate Progress must still paint its busy band");

    let image = UiNode::Image(UiImageNode { id: "img".into(), src: String::new(), alt: Some(Label::data("alt text")), presence: UiPresence::default(), menu: None });
    assert!(paints(&image) > 0, "an Image with no decodable source must paint its placeholder plus alt text");

    let group = UiNode::Group(UiGroupNode {
        id: "grp".into(),
        label: Label::data("Group"),
        default_open: Some(true),
        presence: UiPresence::default(),
        menu: None,
        children: vec![UiNode::Text(UiTextNode { value: Label::data("inside"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })],
    });
    let open = paints(&group);
    assert!(open > 0, "an open Group must paint its chevron, its label AND its children");
    let mut collapsed_group = group.clone();
    if let UiNode::Group(node) = &mut collapsed_group {
        node.default_open = Some(false);
    }
    assert!(paints(&collapsed_group) < open, "a collapsed Group paints its header only — its children are not drawn");

    let slot = UiNode::ExternalSlot(UiExternalSlotNode { plugin_id: "plug".into(), app_id: "app".into(), body_key: "body".into(), params_json: "{}".into(), presence: UiPresence::default(), menu: None });
    assert!(paints(&slot) > 0, "an ExternalSlot must paint its placeholder chrome and its body_key");

    let scene = component_scene_ui("surf");
    assert!(paints(&scene) > 0, "a ComponentScene must paint its placeholder rect when no host fills it");
    let widget = to_widget_node(&scene);
    match &widget {
        WidgetNode::ComponentScene { hit_kind, hit_control_id, .. } => {
            assert_eq!(*hit_kind, HitKind::World3d, "a World3d surface registers under its own kind, exactly as `retained_hit_registration` does");
            assert_eq!(hit_control_id, "surf");
        }
        other => panic!("a ComponentScene node must map to the kit's own arm, got {other:?}"),
    }
}
//#endregion 🔖️GoldenHarness

//#region 🔬️IntrospectionTests
#[test]
fn window_ids_viewport_tree_and_theme_expose_private_window_state() {
    let mut ui = Ui::new();
    assert_eq!(ui.window_ids().count(), 0);
    assert_eq!(ui.viewport("win"), None);
    assert!(ui.tree("win").is_none());

    let node = UiNode::Text(UiTextNode { value: Label::data("hi"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None });
    ui.apply_tree("win", &node);
    ui.set_viewport("win", 800.0, 600.0);

    let ids: Vec<&str> = ui.window_ids().collect();
    assert_eq!(ids, vec!["win"]);
    assert_eq!(ui.viewport("win"), Some((800.0, 600.0)));
    let tree = ui.tree("win").expect("tree exists after apply_tree");
    assert!(tree.root.is_some());
    assert_eq!(ui.theme().text.a, Theme::default().text.a);
}
//#endregion 🔬️IntrospectionTests

//#region 🧩️WidgetsInternalsTests
/// 🧰️ Owns every piece `widgets::WidgetContext<'_, ActionDescriptor>` borrows, so each test can
/// build one without fighting lifetimes; `ctx()` re-borrows fresh each call (a `WidgetContext`
/// isn't `Clone`/reusable once passed to `render_widget`, which can mutate through it).
struct WidgetHarness {
    draw: DrawList,
    atlas: FontAtlas,
    theme: Theme,
    input: InputState<ActionDescriptor>,
    scroll_offsets: StdHashMap<String, f32>,
    collapsed_sections: StdHashMap<String, bool>,
    open_selects: StdHashMap<String, bool>,
    maps: WidgetInteractionMaps<ActionDescriptor>,
}

impl WidgetHarness {
    fn new() -> Self {
        Self {
            draw: DrawList::default(),
            atlas: FontAtlas::builtin(),
            theme: Theme::default(),
            input: InputState::default(),
            scroll_offsets: StdHashMap::new(),
            collapsed_sections: StdHashMap::new(),
            open_selects: StdHashMap::new(),
            maps: WidgetInteractionMaps::default(),
        }
    }

    fn ctx(&mut self) -> WidgetContext<'_, ActionDescriptor> {
        WidgetContext {
            draw: &mut self.draw,
            overlay: None,
            atlas: &mut self.atlas,
            icons: None,
            input: &mut self.input,
            theme: &self.theme,
            scroll_offsets: &mut self.scroll_offsets,
            collapsed_sections: &mut self.collapsed_sections,
            open_selects: &mut self.open_selects,
            interaction_maps: Some(&mut self.maps),
            pick_clip: None,
            viewport_height: 0.0,
        }
    }
}

#[test]
fn wrap_text_wraps_long_text_across_multiple_lines() {
    let mut atlas = FontAtlas::builtin();
    let long = "word ".repeat(40);
    let lines = wrap_text(&mut atlas, &long, 100.0, 16.0);
    assert!(lines.len() > 1, "text far wider than max_width must wrap into multiple lines");
    for line in &lines {
        assert!(!line.is_empty());
    }
}

#[test]
fn wrap_text_of_empty_string_yields_one_empty_line() {
    let mut atlas = FontAtlas::builtin();
    let lines = wrap_text(&mut atlas, "", 200.0, 16.0);
    assert_eq!(lines, vec![String::new()], "an empty input must still produce a single (empty) line, never zero lines");
}

#[test]
fn measure_widget_stack_vertical_sums_child_heights_and_maxes_width() {
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    let node = WidgetNode::<ActionDescriptor>::Stack { direction: "vertical".into(), gap: Some("none".into()), padding: Some("none".into()), children: vec![WidgetNode::Separator, WidgetNode::Separator] };
    let (_, h) = measure_widget(&mut atlas, &theme, &node);
    let (_, single_h) = measure_widget(&mut atlas, &theme, &WidgetNode::<ActionDescriptor>::Separator);
    assert!((h - single_h * 2.0).abs() < 0.001, "two stacked separators with no gap/padding must measure to exactly twice one separator's height, got {h} vs {single_h}");
}

#[test]
fn measure_widget_stack_horizontal_sums_child_widths() {
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    let button = || WidgetNode::<ActionDescriptor>::Button { id: Some("b".into()), icon_id: None, label: Label::data("Go").to_string(), event: None };
    let node = WidgetNode::Stack { direction: "horizontal".into(), gap: Some("none".into()), padding: Some("none".into()), children: vec![button(), button()] };
    let (w, _) = measure_widget(&mut atlas, &theme, &node);
    assert!((w - theme.control_height * 2.0).abs() < 0.001, "two gap-less horizontal buttons must measure to exactly twice one control's width");
}

#[test]
fn measure_widget_separator_uses_theme_control_height_floor() {
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    let (w, h) = measure_widget(&mut atlas, &theme, &WidgetNode::<ActionDescriptor>::Separator);
    assert_eq!(w, theme.control_height.max(1.0));
    assert_eq!(h, 1.0 + theme.gap_standard);
}

#[test]
fn measure_widget_key_value_grows_with_entry_count() {
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    let one = WidgetNode::<ActionDescriptor>::KeyValue { entries: vec![KeyValueEntry { label: Label::data("A").to_string(), value: "1".into() }] };
    let two = WidgetNode::<ActionDescriptor>::KeyValue { entries: vec![KeyValueEntry { label: Label::data("A").to_string(), value: "1".into() }, KeyValueEntry { label: Label::data("B").to_string(), value: "2".into() }] };
    let (_, h1) = measure_widget(&mut atlas, &theme, &one);
    let (_, h2) = measure_widget(&mut atlas, &theme, &two);
    assert!((h2 - h1 * 2.0).abs() < 0.001, "KeyValue height must scale linearly with entry count");
}

#[test]
fn measure_widget_ring_is_fixed_size() {
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    let (w, h) = measure_widget(&mut atlas, &theme, &WidgetNode::<ActionDescriptor>::Ring { id: "r".into(), t: 0.5, disabled: false, on_change: None });
    assert_eq!((w, h), (80.0, 80.0));
}

#[test]
fn measure_widget_field_combines_label_and_child_height() {
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    let node = WidgetNode::<ActionDescriptor>::Field { id: "f".into(), label: Label::data("Label").to_string(), child: ControlNode::Slider { id: "s".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.1, ready: None, disabled: false, on_change: None } };
    let (_, h) = measure_widget(&mut atlas, &theme, &node);
    assert!(h > theme.control_height, "a Field's total height must be its label plus its child control, so it must exceed the control's own height alone");
}

#[test]
fn measure_widget_section_sums_header_and_children_plus_gap() {
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    let empty = WidgetNode::<ActionDescriptor>::Section { id: "s".into(), label: None, default_open: true, children: vec![] };
    let with_child = WidgetNode::<ActionDescriptor>::Section { id: "s".into(), label: None, default_open: true, children: vec![WidgetNode::Separator] };
    let (_, empty_h) = measure_widget(&mut atlas, &theme, &empty);
    let (_, child_h) = measure_widget(&mut atlas, &theme, &with_child);
    assert!(child_h > empty_h, "adding a child must grow a Section's measured height beyond its bare header height");
}

#[test]
fn measure_widget_tree_skips_dimmed_items_in_height() {
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    let item = |id: &str, dimmed: bool| TreeItem {
        window: None,
        id: id.into(),
        label: Label::data(id).to_string(),
        description: None,
        icon_id: None,
        selected: false,
        highlighted: false,
        default_open: false,
        dimmed,
        event: None,
        hover_event: None,
        unhover_event: None,
        actions: vec![],
        draggable: false,
        drag_data: StdHashMap::new(),
        control: None,
        children: vec![],
    };
    let visible =
        WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { window: None, id: "sec".into(), label: None, default_open: true, items: vec![item("a", false)] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
    let dimmed =
        WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { window: None, id: "sec".into(), label: None, default_open: true, items: vec![item("a", true)] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
    let (_, visible_h) = measure_widget(&mut atlas, &theme, &visible);
    let (_, dimmed_h) = measure_widget(&mut atlas, &theme, &dimmed);
    assert!(dimmed_h < visible_h, "a dimmed tree item must contribute zero height, so the dimmed tree must measure shorter than the visible one");
}

#[test]
fn widget_interaction_maps_clear_frame_empties_every_map() {
    let mut maps = WidgetInteractionMaps::<ActionDescriptor>::default();
    maps.input_metas.insert("i".into(), InputMeta { on_change: action(), commit: None, value: "v".into(), input_kind: "text".into(), min: None, max: None, step: None, accept: None });
    maps.select_metas.insert("s".into(), action());
    maps.toggle_metas.insert("t".into(), (true, action()));
    maps.slider_metas.insert("sl".into(), SliderMeta { on_change: action(), min: 0.0, max: 1.0, step: 0.1, value: 0.5, bounds_x: 0.0, bounds_w: 10.0 });
    maps.stepper_metas.insert("st".into(), StepperMeta { on_absolute: action(), on_delta: action(), step: 1.0, value: 1.0 });
    maps.ring_metas.insert("r".into(), RingMeta { on_change: action(), disabled: false, center_x: 0.0, center_y: 0.0, radius: 10.0 });
    maps.slider_live_values.insert("sl".into(), 0.5);
    maps.ring_live_values.insert("r".into(), 0.5);
    maps.tree_hover_commands.insert("h".into(), action());
    maps.tree_unhover_commands.insert("u".into(), action());
    maps.tree_selection_change = Some(action());

    maps.clear_frame();

    assert!(maps.input_metas.is_empty());
    assert!(maps.select_metas.is_empty());
    assert!(maps.toggle_metas.is_empty());
    assert!(maps.slider_metas.is_empty());
    assert!(maps.stepper_metas.is_empty());
    assert!(maps.ring_metas.is_empty());
    assert!(maps.slider_live_values.is_empty());
    assert!(maps.ring_live_values.is_empty());
    assert!(maps.tree_hover_commands.is_empty());
    assert!(maps.tree_unhover_commands.is_empty());
    assert!(maps.tree_selection_change.is_none());
}

#[test]
fn render_widget_input_registers_interaction_meta_when_maps_present() {
    let mut h = WidgetHarness::new();
    let node = WidgetNode::Input { id: "in".into(), input_kind: "text".into(), value: "hello".into(), placeholder: None, commit: Some("blur".into()), min: None, max: None, step: None, accept: None, on_change: Some(action()) };
    render_widget(&node, VIEWPORT, &mut h.ctx());
    let meta = h.maps.input_metas.get("in").expect("register_input_meta must populate the map when interaction_maps is Some and on_change is Some");
    assert_eq!(meta.value, "hello");
    assert_eq!(meta.commit.as_deref(), Some("blur"));
}

#[test]
fn render_widget_input_with_no_on_change_does_not_register_meta() {
    let mut h = WidgetHarness::new();
    let node = WidgetNode::Input { id: "in".into(), input_kind: "text".into(), value: "hello".into(), placeholder: None, commit: None, min: None, max: None, step: None, accept: None, on_change: None };
    render_widget(&node, VIEWPORT, &mut h.ctx());
    assert!(h.maps.input_metas.is_empty(), "no on_change means nothing should be wired for the host to fire");
}

#[test]
fn render_widget_select_and_toggle_register_interaction_metas() {
    let mut h = WidgetHarness::new();
    let select = WidgetNode::Select { id: "sel".into(), value: "a".into(), items: vec![SelectItem { value: "a".into(), label: Label::data("Alpha").to_string() }], placeholder: None, on_change: Some(action()) };
    render_widget(&select, VIEWPORT, &mut h.ctx());
    assert!(h.maps.select_metas.contains_key("sel"));

    let toggle = WidgetNode::Toggle { id: "tog".into(), icon_id: IconName::CircleDot, pressed: true, text: Some("On".into()), on_change: Some(action()) };
    render_widget(&toggle, VIEWPORT, &mut h.ctx());
    let (pressed, _) = h.maps.toggle_metas.get("tog").expect("toggle meta must be registered");
    assert!(*pressed);
}

#[test]
fn render_widget_slider_registers_meta_and_live_value_unless_disabled() {
    let mut h = WidgetHarness::new();
    let enabled = WidgetNode::Slider { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, ready: None, disabled: false, on_change: Some(action()) };
    render_widget(&enabled, VIEWPORT, &mut h.ctx());
    assert!(h.maps.slider_metas.contains_key("sl"));
    assert!(h.maps.slider_live_values.contains_key("sl"));

    let mut h2 = WidgetHarness::new();
    let disabled = WidgetNode::Slider { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, ready: None, disabled: true, on_change: Some(action()) };
    render_widget(&disabled, VIEWPORT, &mut h2.ctx());
    assert!(h2.maps.slider_metas.is_empty(), "a disabled slider must not register interaction metadata");
    assert!(h2.maps.slider_live_values.is_empty());
}

#[test]
fn render_widget_number_stepper_registers_stepper_meta() {
    let mut h = WidgetHarness::new();
    let node = WidgetNode::NumberStepper { id: "ns".into(), value: 3.0, step: 1.0, uniform: false, on_absolute: Some(action()), on_delta: Some(action()) };
    render_widget(&node, VIEWPORT, &mut h.ctx());
    let meta = h.maps.stepper_metas.get("ns").expect("stepper meta must be registered when both on_absolute and on_delta are Some");
    assert_eq!(meta.value, 3.0);
    assert!(h.maps.input_metas.contains_key("ns.input"), "the stepper's embedded value segment renders through render_input and must also register an input meta");
}

#[test]
fn render_widget_ring_registers_meta_and_live_value() {
    let mut h = WidgetHarness::new();
    let node = WidgetNode::Ring { id: "r".into(), t: 0.25, disabled: false, on_change: Some(action()) };
    render_widget(&node, VIEWPORT, &mut h.ctx());
    assert!(h.maps.ring_metas.contains_key("r"));
    assert_eq!(h.maps.ring_live_values.get("r"), Some(&0.25));
}

#[test]
fn render_widget_field_draws_label_and_delegates_to_control() {
    let mut h = WidgetHarness::new();
    let node = WidgetNode::Field {
        id: "f".into(),
        label: Label::data("Name").to_string(),
        child: ControlNode::Input { id: "in".into(), input_kind: "text".into(), value: "x".into(), placeholder: None, commit: None, min: None, max: None, step: None, accept: None, on_change: Some(action()) },
    };
    render_widget(&node, VIEWPORT, &mut h.ctx());
    assert!(h.maps.input_metas.contains_key("in"), "Field must render its child control (an Input here), which registers its own interaction meta");
    let total: usize = h.draw.layers.iter().map(|l| l.ui_instances.len()).sum();
    assert!(total > 0, "Field must paint its label plus its child control");
}

#[test]
fn render_widget_section_toggles_collapsed_state_from_default_open() {
    let child = || WidgetNode::<ActionDescriptor>::Text { value: "child text".into(), emphasize: false };
    let mut h = WidgetHarness::new();
    let closed = WidgetNode::<ActionDescriptor>::Section { id: "sec".into(), label: Some(Label::data("Sec").to_string()), default_open: false, children: vec![child()] };
    render_widget(&closed, VIEWPORT, &mut h.ctx());
    assert_eq!(h.collapsed_sections.get("section.sec"), Some(&true), "a Section with default_open: false must seed its collapsed_sections entry as collapsed");

    let mut h2 = WidgetHarness::new();
    let open = WidgetNode::<ActionDescriptor>::Section { id: "sec".into(), label: Some(Label::data("Sec").to_string()), default_open: true, children: vec![child()] };
    render_widget(&open, VIEWPORT, &mut h2.ctx());
    assert_eq!(h2.collapsed_sections.get("section.sec"), Some(&false));
    let closed_instances: usize = h.draw.layers.iter().map(|l| l.ui_instances.len()).sum();
    let open_instances: usize = h2.draw.layers.iter().map(|l| l.ui_instances.len()).sum();
    assert!(open_instances > closed_instances, "an open section must also paint its (visible) child's glyphs, a collapsed one must not");
}

#[test]
fn render_widget_tree_populates_hover_and_unhover_commands() {
    let mut h = WidgetHarness::new();
    let item = TreeItem {
        window: None,
        id: "i1".into(),
        label: Label::data("Item").to_string(),
        description: None,
        icon_id: None,
        selected: false,
        highlighted: false,
        default_open: false,
        dimmed: false,
        event: None,
        hover_event: Some(action()),
        unhover_event: Some(action()),
        actions: vec![],
        draggable: false,
        drag_data: StdHashMap::new(),
        control: None,
        children: vec![],
    };
    let node = WidgetNode::<ActionDescriptor>::Tree {
        sections: vec![TreeSection { window: None, id: "s".into(), label: Some(Label::data("Section").to_string()), default_open: true, items: vec![item] }],
        selected_ids: vec![],
        highlighted_ids: vec![],
        selection_change: Some(action()),
    };
    render_widget(&node, VIEWPORT, &mut h.ctx());
    assert!(h.maps.tree_hover_commands.contains_key("i1"));
    assert!(h.maps.tree_unhover_commands.contains_key("i1"));
    assert_eq!(h.maps.tree_selection_change, Some(action()));
}

#[test]
fn render_widget_tree_row_actions_register_hits_without_hover() {
    let mut h = WidgetHarness::new();
    let item = TreeItem {
        window: None,
        id: "i1".into(),
        label: Label::data("Item").to_string(),
        description: None,
        icon_id: None,
        selected: false,
        highlighted: false,
        default_open: false,
        dimmed: false,
        event: None,
        hover_event: None,
        unhover_event: None,
        actions: vec![TreeItemAction { icon_id: IconName::CircleDot, label: Some(Label::data("Del").to_string()), event: action(), placement: UiTreeActionPlacement::Row }],
        draggable: false,
        drag_data: StdHashMap::new(),
        control: None,
        children: vec![],
    };
    let node = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { window: None, id: "s".into(), label: None, default_open: true, items: vec![item] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
    render_widget(&node, VIEWPORT, &mut h.ctx());
    let action_hits = h.input.staged_hits().iter().filter(|t| t.control_id.as_deref() == Some("tree.action.i1.0")).count();
    assert_eq!(action_hits, 1, "row-placement actions must register a hit target even when the row is unhovered");
}

#[test]
fn render_widget_tree_menu_placement_skips_row_action_hits() {
    let mut h = WidgetHarness::new();
    let item = TreeItem {
        window: None,
        id: "i1".into(),
        label: Label::data("Item").to_string(),
        description: None,
        icon_id: None,
        selected: false,
        highlighted: false,
        default_open: false,
        dimmed: false,
        event: None,
        hover_event: None,
        unhover_event: None,
        actions: vec![TreeItemAction { icon_id: IconName::CircleDot, label: Some(Label::data("Del").to_string()), event: action(), placement: UiTreeActionPlacement::Menu }],
        draggable: false,
        drag_data: StdHashMap::new(),
        control: None,
        children: vec![],
    };
    let node = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { window: None, id: "s".into(), label: None, default_open: true, items: vec![item] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
    render_widget(&node, VIEWPORT, &mut h.ctx());
    let action_hits = h.input.staged_hits().iter().filter(|t| t.control_id.as_deref() == Some("tree.action.i1.0")).count();
    assert_eq!(action_hits, 0, "menu-placement actions must not register row hit targets");
}

#[test]
fn render_widget_tree_marks_selected_and_highlighted_ids_via_ids_list() {
    let mut h = WidgetHarness::new();
    let item = TreeItem {
        window: None,
        id: "i1".into(),
        label: Label::data("Item").to_string(),
        description: None,
        icon_id: None,
        selected: false,
        highlighted: false,
        default_open: false,
        dimmed: false,
        event: Some(action()),
        hover_event: None,
        unhover_event: None,
        actions: vec![],
        draggable: false,
        drag_data: StdHashMap::new(),
        control: None,
        children: vec![],
    };
    let node =
        WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { window: None, id: "s".into(), label: None, default_open: true, items: vec![item] }], selected_ids: vec!["i1".into()], highlighted_ids: vec![], selection_change: None };
    render_widget(&node, VIEWPORT, &mut h.ctx());
    let hit = h.input.staged_hits().iter().find(|t| t.control_id.as_deref() == Some("tree.label.i1")).expect("tree item label must register a hit target");
    assert_eq!(hit.event, Some(action()));
}

#[test]
fn render_scroll_region_clamps_stale_offset_to_new_max_scroll() {
    let mut h = WidgetHarness::new();
    h.scroll_offsets.insert("scroll".into(), 500.0);
    let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
    {
        let mut ctx = h.ctx();
        render_scroll_region("scroll", bounds, 150.0, &mut ctx, |_content, _ctx| {});
    }
    assert_eq!(h.scroll_offsets.get("scroll"), Some(&50.0), "offset must clamp to max_scroll (content_height - bounds.h) even if a stale value was larger");
}

#[test]
fn render_scroll_region_registers_a_scroll_region_hit_target() {
    let mut h = WidgetHarness::new();
    let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
    {
        let mut ctx = h.ctx();
        render_scroll_region("myscroll", bounds, 400.0, &mut ctx, |_content, _ctx| {});
    }
    assert!(h.input.staged_hits().iter().any(|t| t.control_id.as_deref() == Some("myscroll")));
}

#[test]
fn draw_text_on_emits_one_glyph_instance_per_character() {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    draw_text_on(&mut draw, &mut atlas, "abc", 0.0, 0.0, 16.0, Theme::default().text);
    let total: usize = draw.layers.iter().map(|l| l.ui_instances.len()).sum();
    assert_eq!(total, 3);
}

#[test]
fn draw_text_overlay_on_writes_to_the_overlay_channel_not_the_main_one() {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    draw_text_overlay_on(&mut draw, &mut atlas, "hi", 0.0, 0.0, 16.0, Theme::default().text);
    let main: usize = draw.layers.iter().map(|l| l.ui_instances.len()).sum();
    let overlay: usize = draw.layers.iter().map(|l| l.overlay_ui_instances.len()).sum();
    assert_eq!(main, 0, "overlay glyphs must not land in the main ui_instances channel");
    assert_eq!(overlay, 2, "one overlay glyph instance per character");
}
//#endregion 🧩️WidgetsInternalsTests

/// 🧱️ The `boxed_fixed_slots` law for this module's fixed slot tables, against the one committed
/// budget every implementation of it reads (`the committed fixed-slot fixture`).
///
/// Asserts the measured shape of each table (capacity, one slot's bytes, the owner's own bytes)
/// against that record, that each owner is smaller than the table it owns — the structural proof the
/// slots are heap-first rather than an inline `[T; N]` field — and then constructs them on a thread
/// holding only the fixture's `boundedThreadStackBytes`. `Builder::stack_size` overrides
/// `RUST_MIN_STACK`, so the repo runner's 128 MiB floor cannot hide a re-inflated frame here.
#[test]
fn ui_surface_slot_table_is_heap_first_and_fits_a_bounded_thread_stack() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json")).expect("🧱️ the committed fixed-slot-table budget parses");
    let declared: Vec<semio_framework_async::FixedSlotTableBudget> = fixture["tables"]
        .as_array()
        .expect("🧱️ the budget lists its tables")
        .iter()
        .filter(|table| table["guard"] == "ui::wgpu_engine")
        .map(|table| {
            semio_framework_async::FixedSlotTableBudget::new(
                table["owner"].as_str().expect("owner"),
                table["capacity"].as_u64().expect("capacity") as usize,
                table["elementSizeBytes"].as_u64().expect("element bytes") as usize,
                table["ownerSizeBytes"].as_u64().expect("owner bytes") as usize,
            )
        })
        .collect();
    let measured = vec![semio_framework_async::FixedSlotTableBudget::new("wgpu::engine::UiSurfaceRegistry", UI_LAYOUT_SURFACE_SLOTS, size_of::<Option<UiSurfaceSlot>>(), size_of::<UiSurfaceRegistry>())];
    semio_framework_async::assert_fixed_slot_tables(
        "ui::wgpu_engine",
        fixture["boundedThreadStackBytes"].as_u64().expect("bounded stack budget") as usize,
        fixture["conversionThresholdBytes"].as_u64().expect("conversion threshold") as usize,
        &declared,
        &measured,
        || {
            drop(UiSurfaceRegistry::default());
        },
    );
}

/// 📐️ `surface_content_height` answers the DOCUMENT's own extent, never the viewport it was laid out
/// against — it is the measure a floating panel hugs its content with (`anchor_panel_rect`'s
/// `content_h`), so answering the caller's own input makes that hug a no-op and leaves every panel at
/// its full column band. Measured live on 6118: the `framework.panel.toolRun` root reported 781.6
/// while its one run group was 233.96 tall, and the Tool-runs panel covered the whole right column
/// and the 3D preview under it (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `📓️w14b-generation3d-labels-preview-layout.md`).
#[test]
fn surface_content_height_measures_the_document_not_the_viewport_it_was_given() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("panel", &stack_ui(vec![button_ui("one", "One"), button_ui("two", "Two")]));
    drive_layout(&mut ui, "panel", 300.0, 780.0, &mut atlas);

    let short = ui.surface_content_height("panel").expect("a laid-out surface answers its content height");
    assert!(short > 0.0, "two buttons take some height, got {short}");
    assert!(short < 780.0, "two buttons do not take a 780 px column, got {short}");

    // 📏️ …and the same document in a TALLER viewport answers the same height, which is what makes the
    // hug converge instead of tracking the band it is clamped to.
    drive_layout(&mut ui, "panel", 300.0, 1_400.0, &mut atlas);
    let tall = ui.surface_content_height("panel").expect("content height");
    assert!((tall - short).abs() <= 1.0, "the document's extent is viewport-independent, got {short} then {tall}");
}

/// 🌲️ Compact content keeps its intrinsic extent when presentation swaps its accepted tree.
#[test]
fn compact_tree_content_height_survives_presentation_and_viewport_changes() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌳️compact-tree-intrinsic/🔣️.json")).expect("compact tree height fixture");
    let surface = law["surface"].as_str().unwrap();
    let nodes = law["nodes"].as_array().unwrap();
    let mut document = UiDocumentTree::new(UiDocumentLeaseHeader {
        generation: 1, surface: SurfaceId::try_from(surface).unwrap(), revision: UiRevision(1), root: UiNodeId(1), layout_epoch: 0, node_count: nodes.len(),
    }).unwrap();
    for node in nodes {
        document.try_upsert_record(serde_json::from_value(node.clone()).expect("compact row record")).unwrap();
    }
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    assert!(ui.publish_document(surface, document));
    drive_scene_lifetime_reconcile(&mut ui, surface, 1);
    let expected = law["expectedHeight"].as_f64().unwrap() as f32;
    for (index, height) in law["viewports"].as_array().unwrap().iter().enumerate() {
        drive_scene_lifetime_reconcile(&mut ui, surface, 1);
        drive_layout(&mut ui, surface, law["width"].as_f64().unwrap() as f32, height.as_f64().unwrap() as f32, &mut atlas);
        let before = ui.surface_content_height(surface).expect("candidate content extent");
        assert!((before - expected).abs() < 0.02, "candidate content height {before} != {expected}");
        acknowledge_scene_lifetime_candidate(&mut ui, surface, index as u64 + 31);
        let after = ui.surface_content_height(surface).expect("presentation preserves content extent");
        assert!((after - expected).abs() < 0.02, "presented content height {after} != {expected}");
    }
    let mut oracle = taffy::TaffyTree::<()>::new();
    oracle.disable_rounding();
    let children = law["visibleRowHeights"].as_array().unwrap().iter().map(|height| {
        oracle.new_leaf(taffy::Style { size: taffy::geometry::Size { width: taffy::style::Dimension::length(300.0), height: taffy::style::Dimension::length(height.as_f64().unwrap() as f32) }, flex_shrink: 0.0, ..Default::default() }).unwrap()
    }).collect::<Vec<_>>();
    let root = oracle.new_with_children(taffy::Style { flex_direction: taffy::style::FlexDirection::Column, ..Default::default() }, &children).unwrap();
    oracle.compute_layout(root, taffy::geometry::Size { width: taffy::style::AvailableSpace::MaxContent, height: taffy::style::AvailableSpace::MaxContent }).unwrap();
    assert!((oracle.layout(root).unwrap().size.height - expected).abs() < 0.02, "independent flex oracle agrees with the measured React row sum");
}

/// 🧾️ A discarded layout cannot lend its height witness to the still-presented document.
#[test]
fn intrinsic_content_height_cannot_leak_from_a_discarded_candidate() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌳️compact-tree-intrinsic/🔣️.json")).unwrap();
    let surface = law["surface"].as_str().unwrap();
    let sequence = law["publicationSequence"].as_array().unwrap();
    let document = |generation: u64, presentation: &str| {
        let nodes = law["nodes"].as_array().unwrap();
        let mut document = UiDocumentTree::new(UiDocumentLeaseHeader {
            generation, surface: SurfaceId::try_from(surface).unwrap(), revision: UiRevision(generation), root: UiNodeId(1), layout_epoch: 0, node_count: nodes.len(),
        }).unwrap();
        for node in nodes {
            let mut node = node.clone();
            if node["id"] == 1 { node["component"]["presentation"] = presentation.into(); }
            document.try_upsert_record(serde_json::from_value(node).unwrap()).unwrap();
        }
        document
    };
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    assert!(ui.publish_document(surface, document(sequence[0]["generation"].as_u64().unwrap(), sequence[0]["presentation"].as_str().unwrap())));
    drive_scene_lifetime_reconcile(&mut ui, surface, 1);
    drive_layout(&mut ui, surface, 300.0, 720.0, &mut atlas);
    acknowledge_scene_lifetime_candidate(&mut ui, surface, 61);
    let presented = ui.surface_content_height(surface).expect("A is presented");
    drive_scene_lifetime_reconcile(&mut ui, surface, 1);
    assert!(ui.publish_document(surface, document(sequence[1]["generation"].as_u64().unwrap(), sequence[1]["presentation"].as_str().unwrap())));
    drive_scene_lifetime_reconcile(&mut ui, surface, 2);
    drive_layout(&mut ui, surface, 300.0, 720.0, &mut atlas);
    let discarded = ui.surface_content_height(surface).expect("B owns a distinct candidate extent");
    assert!((presented - sequence[0]["height"].as_f64().unwrap() as f32).abs() < 0.02);
    assert!((discarded - sequence[1]["height"].as_f64().unwrap() as f32).abs() < 0.02);
    assert!(ui.seal_presented_input_candidate(62, &[surface.to_string()]));
    assert!(ui.discard_presented_input_candidate(62));
    assert!(ui.publish_document(surface, document(sequence[2]["generation"].as_u64().unwrap(), sequence[2]["presentation"].as_str().unwrap())));
    let before_layout = ui.surface_content_height(surface);
    let expected = sequence[2]["heightBeforeLayout"].as_f64().unwrap() as f32;
    assert!(before_layout.is_none_or(|height| (height - expected).abs() < 0.02), "C has no layout; A's {presented} may be reused, never discarded B's {discarded}: {before_layout:?}");
}
