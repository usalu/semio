use super::*;
use ui_wgpu::wgpu::{BlockListScene, DrawList, FontAtlas, IconAtlas, InputState};

fn block_list_scene(surface_id: &str, controller_id: &str, steps: Value, palette: Value) -> UiComponentSceneNode {
    UiComponentSceneNode {
        host_id: surface_id.into(),
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

fn presentation_fixture() -> Value {
    serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🧩️block-list-presentation/🔣️.json"))).expect("shared block-list presentation fixture")
}

#[test]
fn authored_selection_targets_support_pointer_and_accessibility() {
    let fixture = presentation_fixture();
    let mut node = block_list_scene("form-design", "forms", fixture["steps"].clone(), fixture["palette"].clone());
    node.block_list.as_mut().unwrap().domain_id = Some("fields".into());
    let bounds = Rect::new(0.0, 0.0, 600.0, 400.0);
    let theme = Theme::default();
    let plan = block_list_plan(&node, bounds, &theme, UiDriverDrag::Handle);
    let controls = block_list_accessibility_controls(&plan, bounds, BlockListChromeLabels { steps: "Steps", add_step: "Add Step", delete: "Delete" });
    for case in fixture["selectionCases"].as_array().unwrap() {
        let control = controls.iter().find(|control| control.label == case["label"].as_str().unwrap()).expect("selectable card");
        assert_eq!(control.action.action, "interactionSelect");
        let args = control.action.args.as_ref().unwrap();
        assert_eq!(args.get("domainId").and_then(semio_framework::DslValue::as_str), Some("fields"));
        assert_eq!(args.get("merge").and_then(semio_framework::DslValue::as_str), Some("replace"));
        let targets: Value = serde_json::from_str(args.get("targets").and_then(semio_framework::DslValue::as_str).unwrap()).unwrap();
        assert_eq!(targets, json!([case["target"]]));
        assert!(block_list_accessibility_action_is_current(&node, &control.action));
        let hit = block_list_hit(&node, bounds, control.rect.x + control.rect.w * 0.5, control.rect.y + theme.control_height * 0.5, &theme, UiDriverDrag::Handle).expect("pointer card target");
        assert_eq!(hit.control_id, control.key);
        let mut retired = node.clone();
        retired.block_list.as_mut().unwrap().steps_json = "[]".into();
        assert!(!block_list_accessibility_action_is_current(&retired, &control.action));
    }
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
        render_block_list(node, Rect::new(0.0, 0.0, 600.0, 400.0), &mut ctx, driver_drag, BlockListChromeLabels { steps: "Steps", add_step: "Add Step", delete: "Delete" });
    }
    input
}

fn role_rect(plan: &BlockListPlan, accepts: impl Fn(&BlockListRole) -> bool) -> Rect {
    plan.targets.iter().find(|target| accepts(&target.role)).expect("role target").rect
}

fn pointer(node: &UiComponentSceneNode, input: &mut InputState<ActionDescriptor>, point: (f32, f32), down: bool, generation: u64, driver_drag: UiDriverDrag) {
    passive_scene_pointer_button(node, Rect::new(0.0, 0.0, 600.0, 400.0), ui_render::PointerId(1), point.0, point.1, down, 0, SceneModifiers::default(), "window.pipeline", generation, driver_drag, input).expect("bounded pointer action");
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
    passive_scene_pointer_move(&node, bounds, ui_render::PointerId(1), step_target.0, step_target.1, "window.pipeline", 11, UiDriverDrag::Handle);
    pointer(&node, &mut input, step_target, false, 11, UiDriverDrag::Handle);
    let action = drain_actions(&mut input).pop().expect("moveStep action");
    assert_eq!(serde_json::to_value(action).unwrap(), fixture["journeys"][4]["expectedAction"]);

    let block_source = center(role_rect(&plan, |role| matches!(role, BlockListRole::BlockHandle { block_id, .. } if block_id == "load")));
    let block_target = center(role_rect(&plan, |role| matches!(role, BlockListRole::Block { block_id, .. } if block_id == "clean")));
    pointer(&node, &mut input, block_source, true, 12, UiDriverDrag::Handle);
    passive_scene_pointer_move(&node, bounds, ui_render::PointerId(1), block_target.0, block_target.1, "window.pipeline", 12, UiDriverDrag::Handle);
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
    passive_scene_pointer_move(&node, bounds, ui_render::PointerId(1), publish.0, publish.1, "window.pipeline", 13, UiDriverDrag::Handle);
    pointer(&node, &mut input, publish, false, 13, UiDriverDrag::Handle);
    let action = drain_actions(&mut input).pop().expect("addBlock action");
    assert_eq!(serde_json::to_value(action).unwrap(), fixture["journeys"][6]["expectedAction"]);

    pointer(&node, &mut input, palette, true, 14, UiDriverDrag::Handle);
    assert!(cancel_scene_list_transfer(), "Escape cancellation consumes the active authority");
    pointer(&node, &mut input, publish, false, 14, UiDriverDrag::Handle);
    assert!(drain_actions(&mut input).is_empty());

    pointer(&node, &mut input, palette, true, 15, UiDriverDrag::Handle);
    passive_scene_pointer_move(&node, bounds, ui_render::PointerId(1), publish.0, publish.1, "window.pipeline", 16, UiDriverDrag::Handle);
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

#[test]
fn shared_palette_target_contract_drives_pointer_drag_and_accessibility_from_the_current_steps() {
    let fixture = presentation_fixture();
    let node = block_list_scene("presentation-block-list", "controller.block-list", fixture["steps"].clone(), fixture["palette"].clone());
    let bounds = Rect::new(0.0, 0.0, 600.0, 400.0);
    let theme = Theme::default();
    let plan = block_list_plan(&node, bounds, &theme, UiDriverDrag::Handle);
    let palette = role_rect(&plan, |role| matches!(role, BlockListRole::Palette { kind } if kind == "filter"));
    assert_eq!(palette.y, bounds.y + theme.padding_standard, "the palette begins at React's rail padding without a heading band");
    assert!(!plan.body_range.is_empty(), "the shared target fixture carries current steps");
    for target_case in fixture["targetCases"].as_array().expect("target cases") {
        let steps = if target_case["steps"] == "empty" { json!([]) } else { fixture["steps"].clone() };
        let mut candidate = block_list_scene("presentation-block-list", "controller.block-list", steps, fixture["palette"].clone());
        candidate.block_list.as_mut().expect("block list").selected_id = target_case["selectedId"].as_str().map(str::to_owned);
        let candidate_plan = block_list_plan(&candidate, bounds, &theme, UiDriverDrag::Handle);
        assert_eq!(candidate_plan.palette_target_step_id.as_deref(), target_case["expectedStepId"].as_str(), "target case {}", target_case["name"]);
        let palette_target = candidate_plan.targets.iter().find(|target| matches!(&target.role, BlockListRole::Palette { kind } if kind == "filter")).expect("painted palette row");
        assert_eq!(
            palette_target.action.as_ref().and_then(|action| action.args.as_ref()).and_then(|args| args.get("stepId")).and_then(semio_framework::DslValue::as_str),
            target_case["expectedStepId"].as_str(),
            "pointer action follows the neutral resolver"
        );
        if target_case["expectedStepId"].is_null() {
            assert!(!candidate_plan.targets.iter().any(|target| matches!(target.role, BlockListRole::PaletteHandle { .. })), "an empty list publishes no transfer handle");
            let point = center(palette_target.rect);
            assert!(block_list_transfer_start(&candidate, bounds, point.0, point.1, &theme, UiDriverDrag::Surface).is_none(), "an empty list cannot arm surface drag");
        }
    }

    let locales = fixture["locales"].as_array().expect("locales");
    for (index, (locale, labels)) in locales.iter().zip([BlockListChromeLabels { steps: "Steps", add_step: "Add Step", delete: "Delete" }, BlockListChromeLabels { steps: "Schritte", add_step: "Schritt hinzufügen", delete: "Löschen" }]).enumerate()
    {
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut scroll = HashMap::new();
        let mut collapsed = HashMap::new();
        let mut selects = HashMap::new();
        {
            let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
            render_block_list(&node, bounds, &mut ctx, UiDriverDrag::Handle, labels);
        }
        let epoch = 900 + index as u64;
        seal_block_list_accessibility_candidates(epoch);
        acknowledge_block_list_accessibility_candidates(epoch);
        let controls = accepted_block_list_accessibility_controls(&node.host_id);
        assert_eq!(controls.iter().find(|control| control.key.ends_with(".addStep")).map(|control| control.label.as_str()), locale["addStep"].as_str());
        assert_eq!(controls.iter().find(|control| control.key.ends_with(".palette.filter")).map(|control| control.label.as_str()), Some("Filter"));
        for deletion in fixture["deletionCases"].as_array().unwrap() {
            let label = deletion["labels"][locale["locale"].as_str().unwrap()].as_str().unwrap();
            let control = controls.iter().find(|control| control.label == label).expect("delete control names its exact target");
            let action = serde_json::to_value(&control.action).unwrap();
            assert_eq!(action["action"], deletion["action"]);
            assert_eq!(action["args"], deletion["args"]);
        }
    }

    let mut input = InputState::<ActionDescriptor>::default();
    block_list_accessibility_activate(&node, "presentation-block-list.addStep", &mut input).expect("accepted add step").expect("bounded add step");
    block_list_accessibility_activate(&node, "presentation-block-list.palette.filter", &mut input).expect("accepted palette entry").expect("bounded add block");
    let actions = drain_actions(&mut input);
    assert_eq!(actions.len(), 2);
    assert_eq!(actions[0].action, fixture["actions"][0]["action"].as_str().expect("add step action"));
    assert_eq!(actions[1].action, fixture["actions"][1]["action"].as_str().expect("add block action"));
    assert_eq!(serde_json::to_value(&actions[1]).expect("action value")["args"], fixture["actions"][1]["args"]);

    let controller_successor = block_list_scene("presentation-block-list", "controller.successor", fixture["steps"].clone(), fixture["palette"].clone());
    assert!(block_list_accessibility_activate(&controller_successor, "presentation-block-list.palette.filter", &mut input).is_none(), "a successor controller cannot replay the prior accepted action");

    let mut target_successor = block_list_scene("presentation-block-list", "controller.block-list", fixture["steps"].clone(), fixture["palette"].clone());
    target_successor.block_list.as_mut().expect("block list").selected_id = Some("schedule".into());
    assert!(block_list_accessibility_activate(&target_successor, "presentation-block-list.palette.filter", &mut input).is_none(), "a changed current target cannot replay the prior accepted action");

    let successor = block_list_scene("presentation-block-list", "controller.block-list", json!([]), fixture["palette"].clone());
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_block_list(&successor, bounds, &mut ctx, UiDriverDrag::Handle, BlockListChromeLabels { steps: "Steps", add_step: "Add Step", delete: "Delete" });
    }
    seal_block_list_accessibility_candidates(902);
    acknowledge_block_list_accessibility_candidates(902);
    let controls = accepted_block_list_accessibility_controls(&successor.host_id);
    assert!(!controls.iter().any(|control| control.key.ends_with(".palette.filter")), "an empty list publishes no enabled virtual palette control");
    assert!(block_list_accessibility_activate(&successor, "presentation-block-list.palette.filter", &mut input).is_none(), "the accepted empty successor retires the old palette action");
}
