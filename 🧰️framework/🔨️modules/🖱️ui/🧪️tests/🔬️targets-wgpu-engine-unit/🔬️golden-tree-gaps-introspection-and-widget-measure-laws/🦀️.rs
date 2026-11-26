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
        let mut ui = UiFixture::new(semio_ui_locale::Locale::En);
        let mut atlas = FontAtlas::builtin();
        let mut draw = DrawList::default();
        let bounds = Rect::new(120.0, 300.0, 300.0, 48.0);
        ui.apply_tree("tree-paint", &node);
        drive_layout(&mut ui, "tree-paint", bounds.w, bounds.h, &mut atlas);
        let mut completed = false;
        for _ in 0..16_384 {
            match ui.frame_into_step::<RecordingSceneHost>("tree-paint", bounds, semio_ui_theme::Level::Window, &mut atlas, None, None, &mut draw, ui_contract::UI_WORKER_RETIREMENT_POLICY).0 {
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
        assert!(instances.iter().filter(|item| item.params[2] == semio_ui_draw_contract::KIND_GLYPH).count() >= label.chars().filter(|ch| !ch.is_whitespace()).count(), "{label}: label glyphs are emitted");
        let overlaps = |a: &[f32; 4], b: &[f32; 4]| a[0] < b[0] + b[2] && b[0] < a[0] + a[2] && a[1] < b[1] + b[3] && b[1] < a[1] + a[3];
        let glyphs: Vec<_> = instances.iter().filter(|item| item.params[2] == semio_ui_draw_contract::KIND_GLYPH).collect();
        assert!(instances.iter().filter(|item| item.params[2] != semio_ui_draw_contract::KIND_GLYPH).all(|chrome| !glyphs.iter().any(|glyph| overlaps(&chrome.rect, &glyph.rect))), "{label}: an idle action row has no opaque control chrome covering its glyphs (guide lines may sit beside them)");
        assert!(ui.window_hit_targets("tree-paint").iter().any(|hit| hit.action.is_some()), "{label}: suppressing identity paint must retain activation");
    }
}

/// ↔️ Field and Section expand into a retained child; these two tests verify the retained side
/// produces non-empty output.
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

/// 🖼️ The Image, ComponentScene and ExternalSlot placeholders are documented stand-ins; these tests
/// only verify the retained side produces the placeholder chrome its own doc comments promise.
#[test]
fn golden_image_known_gap() {
    let node = UiNode::Image(UiImageNode { source_page: None, id: "img".into(), src: String::new(), alt: Some(Label::data("alt text")), presence: UiPresence::default(), menu: None });
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

//#endregion 🔖️GoldenHarness

//#region 🔬️IntrospectionTests
#[test]
fn window_ids_viewport_tree_and_theme_expose_private_window_state() {
    let mut ui = UiFixture::new(semio_ui_locale::Locale::En);
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

//#endregion 🧩️WidgetsInternalsTests
