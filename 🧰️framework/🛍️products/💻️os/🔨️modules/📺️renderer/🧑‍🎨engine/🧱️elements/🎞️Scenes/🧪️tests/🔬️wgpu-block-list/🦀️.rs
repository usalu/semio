
use super::*;
use ui_wgpu::wgpu::{BlockListScene, DrawList, FontAtlas, IconAtlas, InputState};

fn block_list_scene(surface_id: &str, block_list: BlockListScene) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::BlockList,
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
        diff_view: None,
        event_feed: None,
        block_list: Some(block_list),
        menu: None,
    }
}

fn step_json(id: &str, blocks: &[(&str, &str, &str)]) -> Value {
    json!({
        "id": id,
        "title": format!("Step {id}"),
        "blocks": blocks.iter().map(|(bid, label, kind)| json!({ "id": bid, "label": label, "kind": kind })).collect::<Vec<_>>(),
    })
}

/// 🧪️ Renders `node` and returns the `InputState` so tests can inspect registered hit targets.
fn render(node: &UiComponentSceneNode) -> InputState<ActionDescriptor> {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
        render_block_list(node, Rect::new(0.0, 0.0, 600.0, 400.0), &mut ctx);
    }
    input
}

fn hit<'a>(input: &'a InputState<ActionDescriptor>, control_id: &str) -> &'a HitTarget<ActionDescriptor> {
    input.hit_targets.iter().find(|target| target.control_id.as_deref() == Some(control_id)).unwrap_or_else(|| panic!("no hit target registered for control_id {control_id:?}"))
}

fn find_hit<'a>(input: &'a InputState<ActionDescriptor>, control_id: &str) -> Option<&'a HitTarget<ActionDescriptor>> {
    input.hit_targets.iter().find(|target| target.control_id.as_deref() == Some(control_id))
}

#[test]
fn missing_scene_renders_placeholder_without_panicking() {
    let node = UiComponentSceneNode {
        surface_id: "s1".into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::BlockList,
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
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    };
    render(&node);
}

#[test]
fn add_step_button_dispatches_add_step() {
    let scene = BlockListScene { steps_json: "[]".into(), palette_json: "[]".into(), selected_id: None, dragging_id: None, domain_id: None };
    let node = block_list_scene("s1", scene);
    let input = render(&node);
    let target = hit(&input, "s1.addStep");
    let action = target.event.as_ref().expect("addStep action");
    assert_eq!(action.action, "addStep");
}

#[test]
fn palette_entry_dispatches_add_block_with_kind() {
    let palette = json!([{ "blockKind": "text", "label": "Text", "iconId": "type" }]).to_string();
    let scene = BlockListScene { steps_json: "[]".into(), palette_json: palette, selected_id: None, dragging_id: None, domain_id: None };
    let node = block_list_scene("s1", scene);
    let input = render(&node);
    let target = hit(&input, "s1.palette.text");
    let action = target.event.as_ref().expect("addBlock action");
    assert_eq!(action.action, "addBlock");
    assert_eq!(action.args.as_ref().and_then(|args| args.get("kind")).and_then(semio_framework::DslValue::as_str), Some("text"));
}

#[test]
fn first_step_has_no_move_up_but_has_move_down_when_a_second_step_exists() {
    let steps = json!([step_json("a", &[]), step_json("b", &[])]).to_string();
    let scene = BlockListScene { steps_json: steps, palette_json: "[]".into(), selected_id: None, dragging_id: None, domain_id: None };
    let node = block_list_scene("s1", scene);
    let input = render(&node);
    assert!(hit(&input, "s1.step.a.moveUp").event.is_none(), "the first step must not be able to move further up");
    let down = hit(&input, "s1.step.a.moveDown");
    let action = down.event.as_ref().expect("moveStep action");
    assert_eq!(action.action, "moveStep");
    assert_eq!(action.args.as_ref().and_then(|args| args.get("stepId")).and_then(semio_framework::DslValue::as_str), Some("a"));
    assert_eq!(action.args.as_ref().and_then(|args| args.get("index")).and_then(semio_framework::DslValue::as_f64), Some(1.0));
}

#[test]
fn last_step_has_no_move_down() {
    let steps = json!([step_json("a", &[]), step_json("b", &[])]).to_string();
    let scene = BlockListScene { steps_json: steps, palette_json: "[]".into(), selected_id: None, dragging_id: None, domain_id: None };
    let node = block_list_scene("s1", scene);
    let input = render(&node);
    assert!(hit(&input, "s1.step.b.moveDown").event.is_none(), "the last step must not be able to move further down");
}

#[test]
fn remove_step_button_dispatches_remove_step_with_step_id() {
    let steps = json!([step_json("a", &[])]).to_string();
    let scene = BlockListScene { steps_json: steps, palette_json: "[]".into(), selected_id: None, dragging_id: None, domain_id: None };
    let node = block_list_scene("s1", scene);
    let input = render(&node);
    let target = hit(&input, "s1.step.a.remove");
    let action = target.event.as_ref().expect("removeStep action");
    assert_eq!(action.action, "removeStep");
    assert_eq!(action.args.as_ref().and_then(|args| args.get("stepId")).and_then(semio_framework::DslValue::as_str), Some("a"));
}

#[test]
fn block_move_and_remove_dispatch_expected_action_shapes() {
    let steps = json!([step_json("a", &[("b1", "Block One", "text"), ("b2", "Block Two", "number")])]).to_string();
    let scene = BlockListScene { steps_json: steps, palette_json: "[]".into(), selected_id: None, dragging_id: None, domain_id: None };
    let node = block_list_scene("s1", scene);
    let input = render(&node);

    assert!(hit(&input, "s1.block.b1.moveUp").event.is_none(), "the first block in a step must not move further up");
    let move_down = hit(&input, "s1.block.b1.moveDown");
    let move_action = move_down.event.as_ref().expect("moveBlock action");
    assert_eq!(move_action.action, "moveBlock");
    assert_eq!(move_action.args.as_ref().and_then(|args| args.get("blockId")).and_then(semio_framework::DslValue::as_str), Some("b1"));
    assert_eq!(move_action.args.as_ref().and_then(|args| args.get("fromStepId")).and_then(semio_framework::DslValue::as_str), Some("a"));
    assert_eq!(move_action.args.as_ref().and_then(|args| args.get("toStepId")).and_then(semio_framework::DslValue::as_str), Some("a"));
    assert_eq!(move_action.args.as_ref().and_then(|args| args.get("index")).and_then(semio_framework::DslValue::as_f64), Some(1.0));

    assert!(hit(&input, "s1.block.b2.moveDown").event.is_none(), "the last block in a step must not move further down");

    let remove = hit(&input, "s1.block.b1.remove");
    let remove_action = remove.event.as_ref().expect("removeBlock action");
    assert_eq!(remove_action.action, "removeBlock");
    assert_eq!(remove_action.args.as_ref().and_then(|args| args.get("stepId")).and_then(semio_framework::DslValue::as_str), Some("a"));
    assert_eq!(remove_action.args.as_ref().and_then(|args| args.get("blockId")).and_then(semio_framework::DslValue::as_str), Some("b1"));
}

#[test]
fn empty_steps_registers_no_step_hit_targets() {
    let scene = BlockListScene { steps_json: "[]".into(), palette_json: "[]".into(), selected_id: None, dragging_id: None, domain_id: None };
    let node = block_list_scene("s1", scene);
    let input = render(&node);
    assert!(find_hit(&input, "s1.step.a.remove").is_none());
}

//#region BlockListPaintTests
#[test]
fn step_card_draws_a_full_four_sided_border_not_just_top_and_bottom() {
    // 🖼️ Unit-tests `draw_ink_rect_outline` directly (the helper `render_block_list`'s step-card
    // border calls) rather than filtering `render_block_list`'s full draw output by color: `theme
    // .separator` and `theme.border_normal` are byte-identical by design (both derive from
    // `chrome.border_normal` in `Theme::from_chrome`), and the card's right edge sits only a few
    // px from the unrelated main/palette divider line — too tight a margin for a position filter
    // to reliably separate the two from the full scene, so isolate the helper instead.
    let mut draw = DrawList::default();
    let color = Theme::default().border_normal;
    draw_ink_rect_outline(&mut draw, 10.0, 20.0, 200.0, 80.0, color, 1.0);
    let border_vertex_count = draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).filter(|v| v.color == [color.r, color.g, color.b, color.a]).count();
    assert_eq!(border_vertex_count, 24, "4 lines (top/right/bottom/left) * 6 vertices should emit 24, got {border_vertex_count}");
}
//#endregion BlockListPaintTests
