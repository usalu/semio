//! 📂️ LAW: retained disclosure state removes descendants from every interactive frame lane.

use super::*;
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{UiButtonNode, UiPresence, UiSectionNode, UiStackNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
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

fn upward_nested_disclosure() -> UiNode {
    let mut branch = UiTreeItemNode::base("branch", Label::data("Branch"));
    branch.default_open = Some(false);
    branch.items = Some(vec![UiTreeItemNode::base("child", Label::data("Child"))]);
    let tree = UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "section".into(),
            label: Some(Label::data("Section")),
            default_open: Some(true),
            presence: UiPresence::default(),
            items: vec![branch],
            window: None,
        }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    });
    UiNode::Stack(UiStackNode {
        id: Some("panel".into()),
        direction: "vertical".into(),
        gap: None,
        padding: None,
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        menu: None,
        children: vec![tree],
    })
}

fn nested_rows(ui: &Ui) -> (&UiTree, crate::wgpu::arena::NodeId, crate::wgpu::arena::NodeId) {
    let window = ui.windows.get("fixture").expect("fixture window");
    let root = window.tree.root.expect("fixture root");
    let tree = window.tree.children(root).next().expect("tree node");
    let section = window.tree.explicit_child(tree, "section").expect("section row");
    let branch = window.tree.explicit_child(section, "branch").expect("branch row");
    let child = window.tree.explicit_child(branch, "child").expect("child row");
    (&window.tree, branch, child)
}

fn nested_child_height(ui: &Ui) -> f32 {
    let (tree, _, child) = nested_rows(ui);
    tree.accepted_layout(child).expect("child layout").height
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

/// 🌳️ A nested disclosure invalidates the published frame as well as mounted layout: its Up-flow
/// header stays on one exact physical row while the child is admitted above it, then the same
/// physical close retires that hit and collapses the mounted descendant back to zero height.
#[test]
fn upward_nested_disclosure_republishes_one_stable_header_and_retires_its_child() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    ui.apply_tree("fixture", &upward_nested_disclosure());
    ui.set_window_flow("fixture", ui_contract::UiFlow::for_anchor(ui_contract::Anchor::Bottom));
    drive_layout_with(&mut ui, &mut atlas, &pool);
    drive_frame(&mut ui, &mut atlas);

    let branch_id = "tree.chevron.branch";
    let child_id = "tree.label.child";
    let closed_header = ui.window_hit_targets("fixture").iter().find(|hit| hit.control_id == branch_id).expect("closed branch gutter").rect;
    assert!(!ui.window_hit_targets("fixture").iter().any(|hit| hit.control_id == child_id), "closed branch publishes no child hit");
    assert_eq!(nested_child_height(&ui), 0.0, "closed branch leaves its mounted child at zero height");

    click(&mut ui, closed_header);
    let (tree, branch, _) = nested_rows(&ui);
    assert_eq!(tree.disclosure_open(branch), Some(true), "the first physical gutter click opens the nested branch");
    drive_layout_with(&mut ui, &mut atlas, &pool);
    drive_frame(&mut ui, &mut atlas);
    let open_header = ui.window_hit_targets("fixture").iter().find(|hit| hit.control_id == branch_id).expect("open branch gutter").rect;
    let child_rect = ui.window_hit_targets("fixture").iter().find(|hit| hit.control_id == child_id).expect("open child hit").rect;
    assert_eq!(open_header, closed_header, "opening admits children above the exact same Up-flow header row");
    assert_eq!(child_rect.y + child_rect.h, open_header.y, "the child ends exactly where the stable branch header begins");
    assert!(nested_child_height(&ui) > 0.0, "open branch gives its mounted child a real row");

    click(&mut ui, open_header);
    let (tree, branch, _) = nested_rows(&ui);
    assert_eq!(tree.disclosure_open(branch), Some(false), "the second physical gutter click closes the nested branch");
    drive_layout_with(&mut ui, &mut atlas, &pool);
    drive_frame(&mut ui, &mut atlas);
    let reclosed_header = ui.window_hit_targets("fixture").iter().find(|hit| hit.control_id == branch_id).expect("reclosed branch gutter").rect;
    assert_eq!(reclosed_header, closed_header, "closing preserves the exact Up-flow header row");
    assert!(!ui.window_hit_targets("fixture").iter().any(|hit| hit.control_id == child_id), "closing retires the child from the published hit generation");
    assert_eq!(nested_child_height(&ui), 0.0, "closing returns the mounted child to zero height");
}

/// 🌲️ The same nested disclosure round trip remains exact in normal Down flow: the child begins
/// after one stable header row and a second physical gutter click retires both geometry and hit.
#[test]
fn downward_nested_disclosure_republishes_one_stable_header_and_retires_its_child() {
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    ui.apply_tree("fixture", &upward_nested_disclosure());
    ui.set_window_flow("fixture", ui_contract::UiFlow::for_anchor(ui_contract::Anchor::Top));
    drive_layout_with(&mut ui, &mut atlas, &pool);
    drive_frame(&mut ui, &mut atlas);

    let branch_id = "tree.chevron.branch";
    let child_id = "tree.label.child";
    let closed_header = ui.window_hit_targets("fixture").iter().find(|hit| hit.control_id == branch_id).expect("closed branch gutter").rect;
    assert!(!ui.window_hit_targets("fixture").iter().any(|hit| hit.control_id == child_id), "closed branch publishes no child hit");
    assert_eq!(nested_child_height(&ui), 0.0, "closed branch leaves its mounted child at zero height");

    click(&mut ui, closed_header);
    let (tree, branch, _) = nested_rows(&ui);
    assert_eq!(tree.disclosure_open(branch), Some(true), "the first physical gutter click opens the nested branch");
    drive_layout_with(&mut ui, &mut atlas, &pool);
    drive_frame(&mut ui, &mut atlas);
    let open_header = ui.window_hit_targets("fixture").iter().find(|hit| hit.control_id == branch_id).expect("open branch gutter").rect;
    let child_rect = ui.window_hit_targets("fixture").iter().find(|hit| hit.control_id == child_id).expect("open child hit").rect;
    assert_eq!(open_header, closed_header, "opening admits children after the exact same Down-flow header row");
    assert_eq!(open_header.y + open_header.h, child_rect.y, "the child begins exactly where the stable branch header ends");
    assert!(nested_child_height(&ui) > 0.0, "open branch gives its mounted child a real row");

    click(&mut ui, open_header);
    let (tree, branch, _) = nested_rows(&ui);
    assert_eq!(tree.disclosure_open(branch), Some(false), "the second physical gutter click closes the nested branch");
    drive_layout_with(&mut ui, &mut atlas, &pool);
    drive_frame(&mut ui, &mut atlas);
    let reclosed_header = ui.window_hit_targets("fixture").iter().find(|hit| hit.control_id == branch_id).expect("reclosed branch gutter").rect;
    assert_eq!(reclosed_header, closed_header, "closing preserves the exact Down-flow header row");
    assert!(!ui.window_hit_targets("fixture").iter().any(|hit| hit.control_id == child_id), "closing retires the child from the published hit generation");
    assert_eq!(nested_child_height(&ui), 0.0, "closing returns the mounted child to zero height");
}
