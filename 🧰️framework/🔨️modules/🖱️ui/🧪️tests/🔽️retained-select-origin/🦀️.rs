//! 🔽️ LAW: a retained Select keeps one window-local popup authority and translates it once at frame publication.

use super::*;
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{UiControlNode, UiPresence, UiSelectItem, UiSelectNode, UiStackNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
use crate::wgpu::draw::KIND_GLYPH;
use crate::wgpu::events::PointerButton;
use crate::wgpu::geometry::Rect;
use crate::wgpu::scene_slots::{SceneHost, ScenePaintCursor, ScenePaintStep, SceneSlot};
use crate::wgpu::tree::AcceptedLayout;
use crate::wgpu::{Label, Theme};

struct EmptySceneHost;

impl SceneHost for EmptySceneHost {
    fn paint_slot_step(&mut self, _slot: &SceneSlot<'_>, _cursor: &mut ScenePaintCursor, _draw: &mut DrawList, _atlas: &mut FontAtlas, _icons: Option<&IconAtlas>) -> ScenePaintStep {
        ScenePaintStep::Fault
    }
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔽️retained-select-origin/🔣️.json")).expect("retained Select origin fixture")
}

fn rect(value: &serde_json::Value) -> Rect {
    let values = value.as_array().expect("rect tuple");
    Rect::new(values[0].as_f64().unwrap() as f32, values[1].as_f64().unwrap() as f32, values[2].as_f64().unwrap() as f32, values[3].as_f64().unwrap() as f32)
}

fn assert_rect(actual: [f32; 4], expected: Rect, label: &str) {
    assert!((actual[0] - expected.x).abs() < 0.01, "{label} x: {} != {}", actual[0], expected.x);
    assert!((actual[1] - expected.y).abs() < 0.01, "{label} y: {} != {}", actual[1], expected.y);
    assert!((actual[2] - expected.w).abs() < 0.01, "{label} width: {} != {}", actual[2], expected.w);
    assert!((actual[3] - expected.h).abs() < 0.01, "{label} height: {} != {}", actual[3], expected.h);
}

fn select_node(item_count: usize) -> UiSelectNode {
    let labels = ["Alpha", "Beta", "Gamma"];
    UiSelectNode {
        id: "fixture.select".into(),
        value: "alpha".into(),
        items: labels.iter().take(item_count).map(|label| UiSelectItem { value: label.to_lowercase(), label: Label::data(*label) }).collect(),
        placeholder: None,
        on_change: ActionDescriptor { controller_id: "fixture".into(), action: "select".into(), args: None },
        presence: UiPresence::default(),
        menu: None,
    }
}

fn select_tree(item_count: usize) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: None,
        padding: None,
        id: Some("fixture.root".into()),
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children: vec![UiNode::Select(select_node(item_count))],
        menu: None,
    })
}

fn upward_tree_select(item_count: usize) -> UiNode {
    let mut item = UiTreeItemNode::base("fixture.item", Label::data("Fixture item"));
    item.control = Some(UiControlNode::Select(select_node(item_count)));
    UiNode::Tree(UiTreeNode { presentation: Default::default(),
        sections: vec![UiTreeSectionNode { id: "fixture.section".into(), label: Some(Label::data("Fixture section")), default_open: Some(true), presence: UiPresence::default(), items: vec![item], window: None }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
}

fn find_select(tree: &UiTree, id: crate::wgpu::arena::NodeId) -> Option<crate::wgpu::arena::NodeId> {
    if tree.node(id).is_some_and(|node| matches!(&node.spec.0, UiNode::Select(_))) {
        return Some(id);
    }
    tree.children(id).find_map(|child| find_select(tree, child))
}

fn place_fixture_layout(ui: &mut Ui, trigger: Rect, viewport: Rect) {
    let window = ui.windows.get_mut("fixture").expect("fixture window");
    let root = window.tree.root.expect("fixture root");
    let select = window.tree.children(root).next().expect("fixture Select");
    let generation = window.tree.accepted_layout_generation().checked_add(1).expect("fixture layout generation");
    assert!(window.tree.write_inactive_layout(root, generation, AcceptedLayout { x: 0.0, y: 0.0, width: viewport.w, height: viewport.h }));
    assert!(window.tree.write_inactive_layout(select, generation, AcceptedLayout { x: trigger.x, y: trigger.y, width: trigger.w, height: trigger.h }));
    window.tree.commit_inactive_layout(generation);
}

fn settle_layout(ui: &mut Ui, atlas: &mut FontAtlas, viewport: Rect) {
    ui.set_viewport("fixture", viewport.w, viewport.h);
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    let mut preview_sequence = 0;
    'settle: for _ in 0..64 {
        for _ in 0..16_384 {
            let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), || Some(0), &mut preview_sequence);
            if matches!(ui.step_layouts(&pool, atlas, &mut cx), UiLayoutStep::Idle) {
                break;
            }
        }
        if !ui.layout_is_dirty("fixture") {
            break 'settle;
        }
        ui.request_layout("fixture");
    }
    assert!(!ui.layout_is_dirty("fixture"), "retained Select layout never settled");
}

fn settle_frame(ui: &mut Ui, atlas: &mut FontAtlas, viewport: Rect) -> DrawList {
    let mut draw = DrawList::default();
    for _ in 0..262_144 {
        match ui.frame_into_step::<EmptySceneHost>("fixture", viewport, atlas, None, None, &mut draw) {
            UiFrameStep::Pending => {}
            UiFrameStep::Ready => return draw,
            step => panic!("retained Select frame answered {step:?}: {}", ui.paint_stall_census("fixture")),
        }
    }
    panic!("retained Select frame did not settle: {}", ui.paint_stall_census("fixture"));
}

#[test]
fn actual_pointer_open_translates_bottom_and_top_popup_paint_and_hits_by_the_frame_origin_once() {
    let law = fixture();
    let origin = &law["viewport"]["origin"];
    let size = &law["viewport"]["size"];
    let viewport = Rect::new(origin[0].as_f64().unwrap() as f32, origin[1].as_f64().unwrap() as f32, size[0].as_f64().unwrap() as f32, size[1].as_f64().unwrap() as f32);
    let item_count = law["row"]["itemCount"].as_u64().unwrap() as usize;

    for test_case in law["cases"].as_array().expect("geometry cases") {
        let trigger = rect(&test_case["triggerLocal"]);
        let expected_menu = rect(&test_case["menuGlobal"]);
        let expected_option = rect(&test_case["firstOptionGlobal"]);
        let mut ui = Ui::new();
        ui.apply_tree("fixture", &select_tree(item_count));
        let mut atlas = FontAtlas::builtin();
        settle_layout(&mut ui, &mut atlas, viewport);
        place_fixture_layout(&mut ui, trigger, viewport);
        let pointer_x = trigger.x + trigger.w * 0.5;
        let pointer_y = trigger.y + trigger.h * 0.5;
        ui.dispatch_event("fixture", UiEvent::PointerDown { x: pointer_x, y: pointer_y, button: PointerButton::Primary, modifiers: Default::default() });
        ui.dispatch_event("fixture", UiEvent::PointerUp { x: pointer_x, y: pointer_y, button: PointerButton::Primary, modifiers: Default::default() });
        settle_layout(&mut ui, &mut atlas, viewport);
        place_fixture_layout(&mut ui, trigger, viewport);
        let draw = settle_frame(&mut ui, &mut atlas, viewport);

        let glass = draw.glass_regions.last().expect("open Select paints popup glass");
        assert_rect(glass.rect, expected_menu, test_case["id"].as_str().unwrap());
        let body_glyphs = draw
            .layers
            .iter()
            .flat_map(|layer| layer.ui_instances.iter())
            .filter(|instance| {
                instance.params[2] == KIND_GLYPH && instance.rect[0] >= expected_menu.x && instance.rect[1] >= expected_menu.y && instance.rect[0] < expected_menu.x + expected_menu.w && instance.rect[1] < expected_menu.y + expected_menu.h
            })
            .count();
        assert!(body_glyphs >= item_count, "every mounted option paints glyphs inside the translated popup, got {body_glyphs}");
        let option = ui.window_hit_targets("fixture").iter().find(|hit| hit.control_id == "alpha").expect("first option hit");
        assert_rect([option.rect.x, option.rect.y, option.rect.w, option.rect.h], expected_option, "first option hit");
        let tree = ui.tree("fixture").expect("fixture retained tree");
        let select = tree.root.and_then(|root| tree.children(root).next()).expect("fixture Select");
        assert_eq!(tree.node(option.node).and_then(|node| node.parent), Some(select), "the published hit remains owned by the open Select");
    }
}

#[test]
fn mounted_layout_pointer_open_frame_and_option_activation_share_one_local_authority() {
    let law = fixture();
    let origin = &law["viewport"]["origin"];
    let size = &law["viewport"]["size"];
    let viewport = Rect::new(origin[0].as_f64().unwrap() as f32, origin[1].as_f64().unwrap() as f32, size[0].as_f64().unwrap() as f32, size[1].as_f64().unwrap() as f32);
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("fixture", &upward_tree_select(law["row"]["itemCount"].as_u64().unwrap() as usize));
    ui.set_window_flow("fixture", ui_contract::UiFlow::for_anchor(ui_contract::Anchor::Bottom));
    settle_layout(&mut ui, &mut atlas, viewport);
    let tree = ui.tree("fixture").expect("fixture retained tree");
    let select = tree.root.and_then(|root| find_select(tree, root)).expect("fixture Select");
    let trigger = tree.absolute_rect(select).expect("mounted Select trigger");
    assert!(trigger.w > 0.0 && trigger.h > 0.0, "mounted Select trigger must be targetable: {trigger:?}");
    let x = trigger.x + trigger.w * 0.5;
    let y = trigger.y + trigger.h * 0.5;
    ui.dispatch_event("fixture", UiEvent::PointerDown { x, y, button: PointerButton::Primary, modifiers: Default::default() });
    let opened = ui.dispatch_event("fixture", UiEvent::PointerUp { x, y, button: PointerButton::Primary, modifiers: Default::default() });
    let window = ui.windows.get("fixture").expect("fixture window");
    assert!(window.tree.node(select).is_some_and(|node| node.state.open), "actual pointer release must open the mounted Select: trigger={trigger:?} commands={opened:?}");
    assert!(window.router.open_overlays().iter().any(|overlay| overlay.root == select && overlay.kind == crate::wgpu::events::OverlayKind::SelectPopup), "the mounted Select owns one open popup after the pointer sequence");
    settle_layout(&mut ui, &mut atlas, viewport);
    let _ = settle_frame(&mut ui, &mut atlas, viewport);

    let hits = ui.window_hit_targets("fixture");
    let option = hits.iter().find(|hit| hit.control_id == "alpha").unwrap_or_else(|| {
        let tree = ui.tree("fixture").expect("fixture retained tree");
        let root = tree.root.and_then(|root| tree.absolute_rect(root));
        let trigger = tree.absolute_rect(select);
        let popup = tree.node(select).and_then(|node| node.state.select_popup);
        panic!("mounted first option hit: root={root:?} trigger={trigger:?} popup={popup:?} hits={hits:?}")
    });
    let option_x = option.rect.x - viewport.x + option.rect.w * 0.5;
    let option_y = option.rect.y - viewport.y + option.rect.h * 0.5;
    ui.dispatch_event("fixture", UiEvent::PointerDown { x: option_x, y: option_y, button: PointerButton::Primary, modifiers: Default::default() });
    let commands = ui.dispatch_event("fixture", UiEvent::PointerUp { x: option_x, y: option_y, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(commands.iter().any(|command| matches!(command, UiCommand::App { intent, .. } if intent.descriptor().action == "select")), "activating a globally published option routes through its window-local Select action");
    assert!(commands.iter().any(|command| matches!(command, UiCommand::OverlayClosed { kind: crate::wgpu::events::OverlayKind::SelectPopup, .. })), "option activation retires the popup authority");
}

#[test]
fn normal_upward_tree_frame_paints_rows_and_each_inline_control_once_at_the_mounted_band() {
    let law = fixture();
    let origin = &law["viewport"]["origin"];
    let size = &law["viewport"]["size"];
    let viewport = Rect::new(origin[0].as_f64().unwrap() as f32, origin[1].as_f64().unwrap() as f32, size[0].as_f64().unwrap() as f32, size[1].as_f64().unwrap() as f32);
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("fixture", &upward_tree_select(law["row"]["itemCount"].as_u64().unwrap() as usize));
    ui.set_window_flow("fixture", ui_contract::UiFlow::for_anchor(ui_contract::Anchor::Bottom));
    settle_layout(&mut ui, &mut atlas, viewport);
    let tree = ui.tree("fixture").expect("fixture retained tree");
    let select = tree.root.and_then(|root| find_select(tree, root)).expect("fixture Select");
    let trigger = tree.absolute_rect(select).expect("mounted Select trigger");
    assert!(trigger.y > viewport.h * 0.5, "the real Up layout places the inline control in the lower half: {trigger:?}");

    let draw = settle_frame(&mut ui, &mut atlas, viewport);
    let glyphs: Vec<_> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| instance.params[2] == KIND_GLYPH).collect();
    let first_glyph_y = glyphs.iter().map(|glyph| glyph.rect[1]).fold(f32::INFINITY, f32::min);
    let trigger_global = Rect::new(viewport.x + trigger.x, viewport.y + trigger.y, trigger.w, trigger.h);
    let theme = Theme::default();
    let metrics = crate::wgpu::layout::TreeRowMetrics::from_theme(&theme);
    let row_global_y = trigger_global.y - (metrics.row_height - theme.control_height_small) * 0.5;
    let ink_ascent_slack = theme.font_size_body * 0.5;
    assert!(first_glyph_y >= row_global_y - ink_ascent_slack, "an Up Tree's label ink must share its mounted row band while allowing font ascent above the line box: first glyph y={first_glyph_y}, row y={row_global_y}, trigger={trigger_global:?}");
    let selected_value_glyphs = glyphs
        .iter()
        .filter(|glyph| {
            let centre_x = glyph.rect[0] + glyph.rect[2] * 0.5;
            let centre_y = glyph.rect[1] + glyph.rect[3] * 0.5;
            centre_x >= trigger_global.x && centre_x < trigger_global.x + trigger_global.w && centre_y >= trigger_global.y && centre_y < trigger_global.y + trigger_global.h
        })
        .count();
    assert_eq!(selected_value_glyphs, "Alpha".chars().count(), "the Tree row delegates its inline Select face to the mounted control exactly once");
}
