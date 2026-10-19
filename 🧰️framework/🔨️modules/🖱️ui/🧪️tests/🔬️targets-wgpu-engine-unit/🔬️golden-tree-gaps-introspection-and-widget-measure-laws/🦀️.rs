// 🔬️ Golden tree, gaps, introspection and widget measure laws: the tree action rows, the known gap goldens, the widget kit coverage, the private window state and the widget wrap and measure laws.

//#region 🔖️GoldenHarness
/// 🌳️ Clickable identity rows preserve the label pixels painted by their owning Tree.
#[test]
fn retained_tree_action_rows_do_not_overpaint_their_labels() {
    let samples: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧱️elements/🌳️Tree/🧪️tests/🧫️fixtures/♿️actions/🔣️.json")).unwrap();
    for sample in samples.as_array().unwrap() {
        let label = sample["label"].as_str().unwrap();
        let item = UiTreeItemNode {
            tone: None,
            window: None,
            granularity: None,
            id: sample["id"].as_str().unwrap().into(),
            label: Label::data(label),
            description: None,
            icon_id: None,
            presence: UiPresence::default(),
            default_open: None,
            action: Some(action()),
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            content_lines: None,
            inline_toolbar: None,
            detail: None,
            dimmed: None,
            menu: None,
        };
        let node = UiNode::Tree(UiTreeNode {
            presentation: Default::default(),
            sections: vec![UiTreeSectionNode { header_toolbar: None, window: None, id: "section".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }],
            presence: UiPresence::default(),
            drop_action: None,
            menu: None,
            interaction_domain: None,
        });
        let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
        let mut atlas = FontAtlas::builtin();
        let mut draw = DrawList::default();
        let bounds = Rect::new(120.0, 300.0, 300.0, 48.0);
        ui.apply_tree("tree-paint", &node);
        drive_layout(&mut ui, "tree-paint", bounds.w, bounds.h, &mut atlas);
        let mut completed = false;
        for _ in 0..16_384 {
            match ui.frame_into_step::<RecordingSceneHost>("tree-paint", bounds, &mut atlas, None, None, &mut draw) {
                UiFrameStep::Pending => {}
                UiFrameStep::Ready => {
                    completed = true;
                    break;
                }
                step => panic!("tree paint failed: {step:?}"),
            }
        }
        assert!(completed, "{label}: retained frame completes");
        let instances: Vec<_> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).collect();
        assert!(instances.iter().filter(|item| item.params[2] == semio_framework_ui_draw_contract::KIND_GLYPH).count() >= label.chars().filter(|ch| !ch.is_whitespace()).count(), "{label}: label glyphs are emitted");
        assert!(instances.iter().all(|item| item.params[2] == semio_framework_ui_draw_contract::KIND_GLYPH), "{label}: an idle action row has no opaque control chrome covering its glyphs");
        assert!(ui.window_hit_targets("tree-paint").iter().any(|hit| hit.action.is_some()), "{label}: suppressing identity paint must retain activation");
    }
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
            precision: None,
            snaps: Vec::new(),
            on_change: action(),
            on_submit: None,
            on_abort: None,
            on_repeat_last: None,
            presence: UiPresence::default(),
            menu: None,
            ..Default::default()
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
    let node = UiNode::ExternalSlot(UiExternalSlotNode { plugin_id: "plug".into(), app_id: "app".into(), body_key: "body".into(), params_json: "{}".into(), host_status: None, presence: UiPresence::default(), menu: None });
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

    let slot = UiNode::ExternalSlot(UiExternalSlotNode { plugin_id: "plug".into(), app_id: "app".into(), body_key: "body".into(), params_json: "{}".into(), host_status: None, presence: UiPresence::default(), menu: None });
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
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
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
//#endregion 🧩️WidgetsInternalsTests
