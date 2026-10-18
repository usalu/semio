// 🖱️ Widget interaction and render laws: the interaction maps, the widget render registrations for inputs, selects, sliders, steppers, rings, fields, sections and trees, the scroll regions and the draw text channels.

//#region 🧩️WidgetsInternalsTests
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
    let presentation = crate::wgpu::layout::slider_control_presentation(VIEWPORT, 0.5, 0.0, 1.0, None, Theme::default().gap_standard, ui_contract::FlowInline::Ltr).slider;
    let meta = h.maps.slider_metas.get("sl").expect("enabled slider meta");
    assert_eq!((meta.bounds_x, meta.bounds_w), (presentation.track_cell.x, presentation.track_cell.w));
    assert!(h.maps.slider_live_values.contains_key("sl"));
    let hit = h.input.staged_hits().iter().find(|hit| hit.kind == HitKind::Slider).expect("enabled slider hit");
    assert_eq!(hit.rect, presentation.track_cell, "immediate pointer authority excludes the inert readout cell");

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
