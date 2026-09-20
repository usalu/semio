use super::*;
use ui_wgpu::wgpu::{BlockListScene, DrawList, FontAtlas, IconAtlas, InputState};

fn block_list_scene(surface_id: &str, controller_id: &str, steps: Value, palette: Value) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: controller_id.into(),
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
        block_list: Some(BlockListScene { steps_json: steps.to_string(), palette_json: palette.to_string(), selected_id: None, dragging_id: None, domain_id: None }),
        menu: None,
    }
}

fn shared_fixture() -> Value {
    serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🔀️scene-list-transfer/🔣️.json"))).expect("shared scene-list transfer fixture")
}

fn drain_actions(input: &mut InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("action authority live") {
        actions.push(action.into_descriptor().expect("bounded action materializes"));
    }
    actions
}

fn fixture_scene() -> UiComponentSceneNode {
    let fixture = shared_fixture();
    block_list_scene("pipeline", "controller.block-list", fixture["blockList"]["steps"].clone(), fixture["blockList"]["palette"].clone())
}

fn render(node: &UiComponentSceneNode, driver_drag: UiDriverDrag) -> InputState<ActionDescriptor> {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_block_list(node, Rect::new(0.0, 0.0, 600.0, 400.0), &mut ctx, driver_drag);
    }
    input
}

fn role_rect(plan: &BlockListPlan, accepts: impl Fn(&BlockListRole) -> bool) -> Rect {
    plan.targets.iter().find(|target| accepts(&target.role)).expect("role target").rect
}

fn pointer(node: &UiComponentSceneNode, input: &mut InputState<ActionDescriptor>, point: (f32, f32), down: bool, generation: u64, driver_drag: UiDriverDrag) {
    passive_scene_pointer_button(node, Rect::new(0.0, 0.0, 600.0, 400.0), point.0, point.1, down, 0, SceneModifiers::default(), "window.pipeline", generation, driver_drag, input).expect("bounded pointer action");
}

fn center(rect: Rect) -> (f32, f32) {
    (rect.x + rect.w * 0.5, rect.y + rect.h * 0.5)
}

#[test]
fn missing_scene_renders_placeholder_without_panicking() {
    let mut node = fixture_scene();
    node.block_list = None;
    render(&node, UiDriverDrag::Handle);
}

#[test]
fn shared_driver_fixture_exposes_only_semantic_handles_or_surfaces_and_no_move_buttons() {
    let node = fixture_scene();
    let bounds = Rect::new(0.0, 0.0, 600.0, 400.0);
    let theme = Theme::default();
    let handle = block_list_plan(&node, bounds, &theme, UiDriverDrag::Handle);
    let surface = block_list_plan(&node, bounds, &theme, UiDriverDrag::Surface);
    let handle_roles = handle.targets.iter().filter(|target| matches!(target.role, BlockListRole::StepHandle { .. } | BlockListRole::BlockHandle { .. } | BlockListRole::PaletteHandle { .. })).count();
    assert_eq!(handle_roles, 6, "two steps, three blocks and one palette entry publish six handles");
    assert!(!surface.targets.iter().any(|target| matches!(target.role, BlockListRole::StepHandle { .. } | BlockListRole::BlockHandle { .. } | BlockListRole::PaletteHandle { .. })));
    assert!(handle.targets.iter().all(|target| !target.control_id.ends_with(".moveUp") && !target.control_id.ends_with(".moveDown")), "the alternate reorder button UI is retired");

    let prepare = role_rect(&handle, |role| matches!(role, BlockListRole::Step { step_id, .. } if step_id == "prepare"));
    let handle_point = center(role_rect(&handle, |role| matches!(role, BlockListRole::StepHandle { step_id, .. } if step_id == "prepare")));
    let label_point = (prepare.x + prepare.w * 0.5, prepare.y + theme.control_height * 0.5);
    assert!(block_list_transfer_start(&node, bounds, handle_point.0, handle_point.1, &theme, UiDriverDrag::Handle).is_some());
    assert!(block_list_transfer_start(&node, bounds, label_point.0, label_point.1, &theme, UiDriverDrag::Handle).is_none(), "a Handle label does not arm sorting");
    assert!(block_list_transfer_start(&node, bounds, label_point.0, label_point.1, &theme, UiDriverDrag::Surface).is_some(), "the same row arms under Surface policy");
}

#[test]
fn shared_fixture_step_and_block_reorders_match_closest_center_actions() {
    cancel_scene_list_transfer();
    let fixture = shared_fixture();
    let node = fixture_scene();
    let bounds = Rect::new(0.0, 0.0, 600.0, 400.0);
    let theme = Theme::default();
    remember_scene_theme(&theme);
    let plan = block_list_plan(&node, bounds, &theme, UiDriverDrag::Handle);
    let mut input = InputState::<ActionDescriptor>::default();

    let step_source = center(role_rect(&plan, |role| matches!(role, BlockListRole::StepHandle { step_id, .. } if step_id == "prepare")));
    let step_target = center(role_rect(&plan, |role| matches!(role, BlockListRole::Step { step_id, .. } if step_id == "publish")));
    pointer(&node, &mut input, step_source, true, 11, UiDriverDrag::Handle);
    passive_scene_pointer_move(&node, bounds, step_target.0, step_target.1, "window.pipeline", 11, UiDriverDrag::Handle);
    pointer(&node, &mut input, step_target, false, 11, UiDriverDrag::Handle);
    let action = drain_actions(&mut input).pop().expect("moveStep action");
    assert_eq!(serde_json::to_value(action).unwrap(), fixture["journeys"][4]["expectedAction"]);

    let block_source = center(role_rect(&plan, |role| matches!(role, BlockListRole::BlockHandle { block_id, .. } if block_id == "load")));
    let block_target = center(role_rect(&plan, |role| matches!(role, BlockListRole::Block { block_id, .. } if block_id == "clean")));
    pointer(&node, &mut input, block_source, true, 12, UiDriverDrag::Handle);
    passive_scene_pointer_move(&node, bounds, block_target.0, block_target.1, "window.pipeline", 12, UiDriverDrag::Handle);
    pointer(&node, &mut input, block_target, false, 12, UiDriverDrag::Handle);
    let action = drain_actions(&mut input).pop().expect("moveBlock action");
    assert_eq!(serde_json::to_value(action).unwrap(), fixture["journeys"][5]["expectedAction"]);
}

#[test]
fn shared_fixture_palette_drop_and_cancellation_paths_use_the_one_authority() {
    cancel_scene_list_transfer();
    let fixture = shared_fixture();
    let node = fixture_scene();
    let bounds = Rect::new(0.0, 0.0, 600.0, 400.0);
    let theme = Theme::default();
    remember_scene_theme(&theme);
    let plan = block_list_plan(&node, bounds, &theme, UiDriverDrag::Handle);
    let palette = center(role_rect(&plan, |role| matches!(role, BlockListRole::PaletteHandle { kind } if kind == "filter")));
    let publish = center(role_rect(&plan, |role| matches!(role, BlockListRole::Step { step_id, .. } if step_id == "publish")));
    let mut input = InputState::<ActionDescriptor>::default();

    pointer(&node, &mut input, palette, true, 13, UiDriverDrag::Handle);
    passive_scene_pointer_move(&node, bounds, publish.0, publish.1, "window.pipeline", 13, UiDriverDrag::Handle);
    pointer(&node, &mut input, publish, false, 13, UiDriverDrag::Handle);
    let action = drain_actions(&mut input).pop().expect("addBlock action");
    assert_eq!(serde_json::to_value(action).unwrap(), fixture["journeys"][6]["expectedAction"]);

    pointer(&node, &mut input, palette, true, 14, UiDriverDrag::Handle);
    assert!(cancel_scene_list_transfer(), "Escape cancellation consumes the active authority");
    pointer(&node, &mut input, publish, false, 14, UiDriverDrag::Handle);
    assert!(drain_actions(&mut input).is_empty());

    pointer(&node, &mut input, palette, true, 15, UiDriverDrag::Handle);
    passive_scene_pointer_move(&node, bounds, publish.0, publish.1, "window.pipeline", 16, UiDriverDrag::Handle);
    pointer(&node, &mut input, publish, false, 16, UiDriverDrag::Handle);
    assert!(drain_actions(&mut input).is_empty(), "a subtree revision change retires the source before release");
}

#[test]
fn block_list_actions_match_react_args_without_an_invented_surface_id() {
    let node = fixture_scene();
    let plan = block_list_plan(&node, Rect::new(0.0, 0.0, 600.0, 400.0), &Theme::default(), UiDriverDrag::Handle);
    let remove = plan.targets.iter().find(|target| target.control_id == "pipeline.block.load.remove").and_then(|target| target.action.as_ref()).expect("remove block action");
    let args = remove.args.as_ref().expect("args");
    assert_eq!(remove.action, "removeBlock");
    assert_eq!(args.get("stepId").and_then(semio_framework::DslValue::as_str), Some("prepare"));
    assert_eq!(args.get("blockId").and_then(semio_framework::DslValue::as_str), Some("load"));
    assert!(args.get("surfaceId").is_none(), "React's dispatchBlockListAction does not inject surfaceId");
}

#[test]
fn step_card_draws_a_full_four_sided_border() {
    let mut draw = DrawList::default();
    let color = Theme::default().border_normal;
    draw_ink_rect_outline(&mut draw, 10.0, 20.0, 200.0, 80.0, color, 1.0);
    let border_vertex_count = draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).filter(|vertex| vertex.color == [color.r, color.g, color.b, color.a]).count();
    assert_eq!(border_vertex_count, 24);
}
