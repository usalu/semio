//! 📂️ LAW: retained disclosure state removes descendants from every interactive frame lane.

use super::*;
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{UiButtonNode, UiPresence, UiSectionNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
use crate::wgpu::events::{EventModifiers, PointerButton};
use crate::wgpu::geometry::Rect;
use crate::wgpu::scene_slots::{SceneHost, ScenePaintCursor, ScenePaintStep, SceneSlot};
use crate::wgpu::IconName;
use crate::wgpu::Label;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📂️retained-section-collapse/🔣️.json")).expect("retained Section fixture")
}

struct EmptySceneHost;

impl SceneHost for EmptySceneHost {
    fn paint_slot_step(&mut self, _slot: &SceneSlot<'_>, _cursor: &mut ScenePaintCursor, _draw: &mut DrawList, _atlas: &mut FontAtlas, _icons: Option<&IconAtlas>) -> ScenePaintStep {
        ScenePaintStep::Fault
    }
}

fn disclosure(fixture: &Value) -> UiNode {
    UiNode::Section(UiSectionNode {
        id: fixture["section"]["id"].as_str().expect("section id").to_string(),
        label: Some(Label::data(fixture["section"]["label"].as_str().expect("section label"))),
        default_open: fixture["section"]["defaultOpen"].as_bool(),
        presence: UiPresence::default(),
        children: vec![UiNode::Button(UiButtonNode {
            id: Some(fixture["section"]["childId"].as_str().expect("child id").to_string()),
            icon_id: IconName::CircleDot,
            label: Label::data(fixture["section"]["childLabel"].as_str().expect("child label")),
            action: ActionDescriptor { controller_id: "fixture".into(), action: "activateChild".into(), args: None },
            style: None,
            presence: UiPresence::default(),
            menu: None,
        })],
        menu: None,
    })
}

fn drive_layout_with(ui: &mut Ui, atlas: &mut FontAtlas, pool: &semio_framework_async::WorkerPool) {
    ui.set_viewport("fixture", 320.0, 240.0);
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    for _ in 0..262_144 {
        if ui.layout_is_dirty("fixture") {
            ui.request_layout("fixture");
        }
        let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), || Some(0), &mut preview_sequence);
        if matches!(ui.step_layouts(pool, atlas, &mut cx), UiLayoutStep::Idle) && !ui.layout_is_dirty("fixture") {
            return;
        }
    }
    panic!("retained disclosure layout never settled");
}

fn advance_to_checked_out_layout(ui: &mut Ui, atlas: &mut FontAtlas, pool: &semio_framework_async::WorkerPool) -> u64 {
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    for _ in 0..262_144 {
        if ui.layout_is_dirty("fixture") {
            ui.request_layout("fixture");
        }
        let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), || Some(0), &mut preview_sequence);
        let _ = ui.step_layouts(pool, atlas, &mut cx);
        let checked_out = ui
            .windows
            .get("fixture")
            .and_then(|window| window.layout_session.as_ref().filter(|session| session.poll() == semio_framework_job::WorkerJobPoll::CheckedOut).map(|_| window.layout_generation));
        if let Some(generation) = checked_out {
            return generation;
        }
        std::thread::yield_now();
    }
    panic!("retained disclosure layout never reached a checked-out session");
}

fn assert_layout_terminal(ui: &Ui) {
    let window = ui.windows.get("fixture").expect("fixture window");
    assert!(!ui.layout_is_dirty("fixture"), "the final disclosure state has no dirty layout root");
    assert!(window.layout_job.is_none() && window.layout_session.is_none() && window.layout_rejected.is_none() && !window.layout_closing && !window.queued, "no superseded disclosure layout owner remains queued or checked out");
}

fn drive_frame(ui: &mut Ui, atlas: &mut FontAtlas) {
    let mut draw = DrawList::default();
    for _ in 0..131_072 {
        match ui.frame_into_step::<EmptySceneHost>("fixture", Rect::new(0.0, 0.0, 320.0, 240.0), atlas, None, None, &mut draw) {
            UiFrameStep::Pending => {}
            UiFrameStep::Ready => return,
            step => panic!("retained disclosure paint answered {step:?}"),
        }
    }
    panic!("retained disclosure paint never settled");
}

fn click(ui: &mut Ui, rect: Rect) {
    let x = rect.x + rect.w * 0.5;
    let y = rect.y + rect.h * 0.5;
    ui.dispatch_event("fixture", UiEvent::PointerDown { x, y, button: PointerButton::Primary, modifiers: EventModifiers::default() });
    ui.dispatch_event("fixture", UiEvent::PointerUp { x, y, button: PointerButton::Primary, modifiers: EventModifiers::default() });
}

fn upward_tree() -> UiNode {
    let mut empty_nested = UiTreeItemNode::base("empty-nested", Label::data("Empty nested"));
    empty_nested.default_open = Some(true);
    empty_nested.items = Some(Vec::new());
    let mut populated_nested = UiTreeItemNode::base("populated-nested", Label::data("Populated nested"));
    populated_nested.default_open = Some(true);
    populated_nested.items = Some(vec![UiTreeItemNode::base("nested-child", Label::data("Nested child"))]);
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode { id: "empty-section".into(), label: Some(Label::data("Empty section")), default_open: Some(true), presence: UiPresence::default(), items: Vec::new(), window: None },
            UiTreeSectionNode {
                id: "nested-section".into(),
                label: Some(Label::data("Nested section")),
                default_open: Some(true),
                presence: UiPresence::default(),
                items: vec![empty_nested, populated_nested],
                window: None,
            },
        ],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
}

#[test]
fn closed_open_and_rapidly_reclosed_sections_publish_only_reachable_descendants() {
    let fixture = fixture();
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    ui.apply_tree("fixture", &disclosure(&fixture));
    drive_layout_with(&mut ui, &mut atlas, &pool);
    drive_frame(&mut ui, &mut atlas);

    let section_id = fixture["section"]["id"].as_str().unwrap();
    let child_id = fixture["section"]["childId"].as_str().unwrap();
    let closed_hits = ui.window_hit_targets("fixture");
    let header = closed_hits.iter().find(|hit| hit.control_id == section_id).expect("closed section header remains interactive").rect;
    assert!(!closed_hits.iter().any(|hit| hit.control_id == child_id), "closed descendants publish no hit target");
    let closed_height = ui.surface_content_height("fixture").expect("closed intrinsic height");
    let closed_paint = ui.paint_census("fixture").expect("closed paint census");

    click(&mut ui, header);
    let opening_generation = advance_to_checked_out_layout(&mut ui, &mut atlas, &pool);
    click(&mut ui, header);
    assert!(ui.windows.get("fixture").is_some_and(|window| window.layout_generation > opening_generation), "closing supersedes the checked-out opening generation");
    drive_layout_with(&mut ui, &mut atlas, &pool);
    drive_frame(&mut ui, &mut atlas);
    assert!(!ui.window_hit_targets("fixture").iter().any(|hit| hit.control_id == child_id), "open then close during a checked-out layout publishes only the final closed state");
    assert_layout_terminal(&ui);

    let header = ui.window_hit_targets("fixture").iter().find(|hit| hit.control_id == section_id).expect("reclosed section header remains interactive").rect;
    click(&mut ui, header);
    drive_layout_with(&mut ui, &mut atlas, &pool);
    drive_frame(&mut ui, &mut atlas);
    assert!(ui.window_hit_targets("fixture").iter().any(|hit| hit.control_id == child_id), "opening admits the descendant into the frame hit walk");
    assert!(ui.surface_content_height("fixture").expect("open intrinsic height") > closed_height, "opening adds the descendant to intrinsic layout");
    let open_paint = ui.paint_census("fixture").expect("open paint census");
    assert!(open_paint.quads > closed_paint.quads || open_paint.glyphs > closed_paint.glyphs, "opening admits descendant paint output");

    let header = ui.window_hit_targets("fixture").iter().find(|hit| hit.control_id == section_id).expect("open section header remains interactive").rect;
    click(&mut ui, header);
    let closing_generation = advance_to_checked_out_layout(&mut ui, &mut atlas, &pool);
    click(&mut ui, header);
    assert!(ui.windows.get("fixture").is_some_and(|window| window.layout_generation > closing_generation), "reopening supersedes the checked-out closing generation");
    drive_layout_with(&mut ui, &mut atlas, &pool);
    drive_frame(&mut ui, &mut atlas);
    assert!(ui.window_hit_targets("fixture").iter().any(|hit| hit.control_id == child_id), "close then open during a checked-out layout publishes only the final open state");
    assert_layout_terminal(&ui);
}

#[test]
fn upward_tree_frame_completes_after_empty_nested_and_terminal_cursors() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    ui.apply_tree("fixture", &upward_tree());
    ui.set_window_flow("fixture", ui_contract::UiFlow::for_anchor(ui_contract::Anchor::Bottom));
    drive_layout_with(&mut ui, &mut atlas, &pool);
    drive_frame(&mut ui, &mut atlas);

    assert_eq!(ui.paint_frame_phase("fixture"), None, "the Up tree publishes and retires its complete frame");
    let hits = ui.window_hit_targets("fixture");
    assert!(hits.iter().any(|hit| hit.control_id == "section.chevron.empty-section"), "an empty section header remains in the completed Up frame");
    assert!(hits.iter().any(|hit| hit.control_id == "tree.label.empty-nested"), "an item whose nested list is empty remains in the completed Up frame");
    assert!(hits.iter().any(|hit| hit.control_id == "tree.label.nested-child"), "the walker resumes after the empty nested list and reaches a populated sibling subtree");
}
