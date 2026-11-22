// 🔘️ Segmented select, button label and default caption laws: React's radio `ToggleGroup` segments, the `labels` driver axis on buttons and segments, and the localized `ui.common.select` placeholder.

//#region 🔘️SegmentedSelectLaws
fn segmented_fixture() -> serde_json::Value {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🔘️segmented-select/🔣️.json")).expect("segmented select fixture");
    assert_eq!(fixture["schema"].as_str(), Some("semio.ui.segmented-select.v1"));
    fixture
}

fn segmented_select(value: &str) -> UiNode {
    UiNode::Select(UiSelectNode {
        appearance: ui_contract::SelectAppearance::Segmented,
        id: "segmented".into(),
        value: value.into(),
        items: vec![
            UiSelectItem { value: "a".into(), label: Label::data("Alpha") },
            UiSelectItem { value: "b".into(), label: Label::data("Beta") },
            UiSelectItem { value: "c".into(), label: Label::data("Gamma") },
        ],
        placeholder: None,
        on_change: action(),
        presence: UiPresence::default(),
        menu: None,
    })
}

fn paint_with_presentation(tree: &UiTree, node: NodeId, theme: &Theme, atlas: &mut FontAtlas, presentation: RetainedPaintPresentation, hovered_at: Option<(f32, f32)>) -> DrawList {
    let layout = tree.accepted_layout(node).expect("retained node layout");
    let absolute = tree.absolute_rect(node).expect("retained node bounds");
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    for _ in 0..16_384 {
        match paint_node_step_with_driver(tree, node, absolute.x - layout.x, absolute.y - layout.y, theme, atlas, None, false, presentation, false, ui_contract::FlowInline::Ltr, hovered_at, &mut draw, &mut cursor) {
            RetainedNodePaintStep::Pending => {}
            RetainedNodePaintStep::Complete => return draw,
            RetainedNodePaintStep::Fault => panic!("retained node paint fault"),
        }
    }
    panic!("retained node paint exceeded its fixed budget")
}

fn glyph_count(draw: &DrawList) -> usize {
    draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).count()
}

fn solid_colors_at(draw: &DrawList, rect: Rect) -> Vec<[f32; 4]> {
    draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_SOLID).abs() < 0.01 && instance.rect == [rect.x, rect.y, rect.w, rect.h]).map(|instance| instance.color).collect()
}

fn token_color(color: Rgba) -> [f32; 4] {
    [color.r, color.g, color.b, color.a]
}

fn presentation_with_labels(labels: semio_ui_wgpu_layout::chrome::UiDriverLabels) -> RetainedPaintPresentation {
    let mut presentation = RetainedPaintPresentation::default();
    presentation.driver.labels = labels;
    presentation
}

#[test]
fn segmented_select_behavior_follows_the_toggle_group_fixture() {
    let fixture = segmented_fixture();
    let bounds = &fixture["bounds"];
    let rect = Rect::new(bounds["x"].as_f64().unwrap() as f32, bounds["y"].as_f64().unwrap() as f32, bounds["w"].as_f64().unwrap() as f32, bounds["h"].as_f64().unwrap() as f32);
    let count = fixture["count"].as_u64().unwrap() as usize;
    for case in fixture["rects"].as_array().unwrap() {
        let got = semio_ui_select_behavior::select_segment_rect(rect, count, case["index"].as_u64().unwrap() as usize, case["rtl"].as_bool().unwrap());
        assert_eq!((got.x, got.w, got.y, got.h), (case["x"].as_f64().unwrap() as f32, case["w"].as_f64().unwrap() as f32, rect.y, rect.h), "{case}");
    }
    for case in fixture["hits"].as_array().unwrap() {
        let got = semio_ui_select_behavior::select_segment_at(rect, count, case["x"].as_f64().unwrap() as f32, case["rtl"].as_bool().unwrap());
        assert_eq!(got, case["segment"].as_u64().map(|index| index as usize), "{case}");
    }
    for case in fixture["keys"].as_array().unwrap() {
        let disabled: Vec<usize> = case["disabled"].as_array().unwrap().iter().map(|index| index.as_u64().unwrap() as usize).collect();
        let got = semio_ui_select_behavior::select_segment_key(Some(case["current"].as_u64().unwrap() as usize), count, case["key"].as_str().unwrap(), case["rtl"].as_bool().unwrap(), |index| disabled.contains(&index));
        assert_eq!(got, case["next"].as_u64().map(|index| index as usize), "{case}");
    }
}

#[test]
fn a_segmented_select_paints_every_option_with_the_chosen_one_pressed_and_the_hovered_one_lit() {
    let (mut tree, root, theme, mut atlas) = setup(&segmented_select("b"));
    let bounds = tree.absolute_rect(root).expect("bounds");
    let segment = |index| semio_ui_select_behavior::select_segment_rect(bounds, 3, index, false);
    let at_rest = paint_with_presentation(&tree, root, &theme, &mut atlas, RetainedPaintPresentation::default(), None);
    assert_eq!(solid_colors_at(&at_rest, segment(0)), vec![token_color(item_bg(&theme, false, false))], "an unchosen segment rests");
    assert_eq!(solid_colors_at(&at_rest, segment(1)), vec![token_color(item_bg(&theme, true, false))], "the chosen segment is pressed");
    assert_eq!(solid_colors_at(&at_rest, segment(2)), vec![token_color(item_bg(&theme, false, false))]);
    let center = segment(2);
    tree.node_mut(root).expect("node").flags.set(NodeFlags::HOVER_OWNER, true);
    let hovered = paint_with_presentation(&tree, root, &theme, &mut atlas, RetainedPaintPresentation::default(), Some((center.x + center.w * 0.5, center.y + center.h * 0.5)));
    assert_eq!(solid_colors_at(&hovered, segment(2)), vec![token_color(item_bg(&theme, false, true))], "the segment under the pointer is lit");
    assert_eq!(solid_colors_at(&hovered, segment(0)), vec![token_color(item_bg(&theme, false, false))], "the others stay at rest");
}

#[test]
fn the_labels_driver_axis_hides_captions_on_segments_and_buttons_but_keeps_the_icons_path() {
    let (tree, root, theme, mut atlas) = setup(&segmented_select("a"));
    let full = paint_with_presentation(&tree, root, &theme, &mut atlas, presentation_with_labels(semio_ui_wgpu_layout::chrome::UiDriverLabels::Full), None);
    let icons = paint_with_presentation(&tree, root, &theme, &mut atlas, presentation_with_labels(semio_ui_wgpu_layout::chrome::UiDriverLabels::Icons), None);
    assert!(glyph_count(&full) > 0, "the full driver paints every segment's caption");
    assert_eq!(glyph_count(&icons), 0, "the icons driver paints no caption");
    let button = UiNode::Button(UiButtonNode { id: Some("save".into()), icon_id: IconName::CircleDot, label: Label::data("Save"), action: action(), style: None, presence: UiPresence::default(), menu: None });
    let (tree, root, theme, mut atlas) = setup(&button);
    let full = paint_with_presentation(&tree, root, &theme, &mut atlas, presentation_with_labels(semio_ui_wgpu_layout::chrome::UiDriverLabels::Full), None);
    let icons = paint_with_presentation(&tree, root, &theme, &mut atlas, presentation_with_labels(semio_ui_wgpu_layout::chrome::UiDriverLabels::Icons), None);
    assert_eq!(glyph_count(&full), 4, "the full driver paints the label");
    assert_eq!(glyph_count(&icons), 0, "the icons driver hides it; the accessible name stays on the contract record");
}

#[test]
fn an_empty_select_shows_the_localized_common_select_caption_and_never_a_guessed_language() {
    let fixture = segmented_fixture();
    let menu = UiNode::Select(UiSelectNode { appearance: Default::default(), id: "menu".into(), value: String::new(), items: vec![UiSelectItem { value: "a".into(), label: Label::data("Alpha") }], placeholder: None, on_change: action(), presence: UiPresence::default(), menu: None });
    let (tree, root, theme, mut atlas) = setup(&menu);
    let caption = |locale: Option<semio_ui_locale::Locale>, atlas: &mut FontAtlas| {
        let mut presentation = RetainedPaintPresentation::default();
        presentation.locale = locale;
        glyph_count(&paint_with_presentation(&tree, root, &theme, atlas, presentation, None))
    };
    let expected = |key: &str, language: &str| fixture["labels"][key][language].as_str().unwrap().chars().count();
    assert_eq!(caption(Some(semio_ui_locale::Locale::En), &mut atlas), expected("select", "en"));
    assert_eq!(caption(Some(semio_ui_locale::Locale::De), &mut atlas), expected("select", "de"));
    assert_eq!(caption(None, &mut atlas), 0, "no locale named, no caption");
    for (locale, language) in [(semio_ui_locale::Locale::En, "en"), (semio_ui_locale::Locale::De, "de")] {
        use semio_ui_locale::AppLabels;
        let labels = semio_ui_locale::UiCommonLabels::labels(locale, semio_ui_locale::Terminology::Native);
        assert_eq!(labels.select.as_str(), fixture["labels"]["select"][language].as_str().unwrap());
        assert_eq!(labels.mixed_values.as_str(), fixture["labels"]["mixedValues"][language].as_str().unwrap());
    }
}

#[test]
fn the_icons_driver_lays_buttons_and_segments_out_as_icon_cells_not_label_width() {
    let theme = Theme::default();
    let button = |label: &str| UiNode::Button(UiButtonNode { id: Some("b".into()), icon_id: IconName::CircleDot, label: Label::data(label), action: action(), style: None, presence: UiPresence::default(), menu: None });
    let width = |node: &UiNode, icons_only: bool| {
        let mut tree = UiTree::new();
        tree.apply_tree(&stack(vec![node.clone()]));
        let root = tree.root.expect("root");
        assert!(semio_ui_wgpu_mounted_layout::layout_tree_presenting_now(&mut tree, root, theme, 400.0, 400.0, false, ui_contract::FlowInline::Ltr, icons_only), "layout pass");
        let child = tree.node(root).and_then(|node| node.first_child).expect("child");
        tree.accepted_layout(child).expect("accepted layout").width
    };
    let long = button("A considerably long caption");
    assert!(width(&long, false) > theme.control_height * 3.0, "the full driver reserves the caption");
    assert_eq!(width(&long, true), theme.control_height, "the icons driver reserves one `min-w-medium` cell");
    assert_eq!(width(&button("x"), true), width(&long, true), "icon cells do not depend on the caption");
    assert_eq!(width(&segmented_select("a"), true), theme.control_height * 3.0, "an icons-only segmented group is one cell per option");
}
//#endregion 🔘️SegmentedSelectLaws
