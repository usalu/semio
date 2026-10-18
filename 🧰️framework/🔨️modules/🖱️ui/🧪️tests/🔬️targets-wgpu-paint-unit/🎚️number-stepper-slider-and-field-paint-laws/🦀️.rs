// 🎚️ Number stepper, slider and field paint laws: the stepper borders and mixed placeholder, the slider readout, rail and compact edit and the field and tree item description paint.

//#region 🔖️FidelityFixes
fn stepper(id: &str, value: f64, uniform: bool) -> UiNode {
    UiNode::NumberStepper(UiNumberStepperNode { id: id.into(), value, step: 1.0, uniform, min: None, max: None, precision: None, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None, ..Default::default() })
}

#[test]
fn painting_a_uniform_number_stepper_uses_one_outer_border_and_two_dividers() {
    let (mut tree, root, theme, mut atlas) = setup(&stepper("ns", 2.0, true));
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let total: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    assert_eq!(total, 8, "one control border, two dividers, and the value glyph without an icon atlas");
}

#[test]
fn painting_a_mixed_number_stepper_shows_the_mixed_placeholder_in_muted_color() {
    let (mut tree, root, theme, mut atlas) = setup(&stepper("ns", 2.0, false));
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let has_muted_glyph = draw
        .layers
        .iter()
        .flat_map(|layer| layer.ui_instances.iter())
        .any(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01 && (instance.color[0] - theme.text_muted.r).abs() < 0.001 && (instance.color[1] - theme.text_muted.g).abs() < 0.001 && (instance.color[2] - theme.text_muted.b).abs() < 0.001);
    assert!(has_muted_glyph, "a non-uniform (mixed) NumberStepper should paint its center value's glyphs in theme.text_muted (the 'Mixed' placeholder)");
}

fn slider(id: &str, unit: Option<&str>) -> UiNode {
    UiNode::Slider(UiSliderNode { id: id.into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, unit: unit.map(String::from), snaps: Vec::new(), on_change: action(), presence: UiPresence::default(), menu: None, ..Default::default() })
}

#[test]
fn painting_a_slider_keeps_its_numeric_readout_independent_from_the_external_unit_sibling() {
    let (mut plain_tree, plain_root, theme, mut plain_atlas) = setup(&slider("sl", None));
    let plain_draw = paint_retained_node_for_test(&plain_tree, plain_root, &theme, &mut plain_atlas);

    let (mut unit_tree, unit_root, theme2, mut unit_atlas) = setup(&slider("sl", Some("mm")));
    let unit_draw = paint_retained_node_for_test(&unit_tree, unit_root, &theme2, &mut unit_atlas);

    let plain_glyphs: Vec<_> = plain_draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).collect();
    let unit_glyphs: Vec<_> = unit_draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).collect();
    assert_eq!(plain_glyphs.iter().filter(|glyph| glyph.color == [theme.text_element.r, theme.text_element.g, theme.text_element.b, theme.text_element.a]).count(), 3, "the internal Slider readout stays numeric only");
    assert_eq!(
        unit_glyphs.iter().filter(|glyph| glyph.color == [theme2.text_muted.r, theme2.text_muted.g, theme2.text_muted.b, theme2.text_muted.a]).count(),
        "0.5 mm".chars().count(),
        "the external sibling advances and paints every formatted scalar, including its separating space"
    );
    assert!(unit_glyphs.len() > plain_glyphs.len());
}

fn compact_tree_slider(unit: Option<&str>) -> UiNode {
    let mut item = UiTreeItemNode::base("slider.item", Label::data("Distance"));
    let UiNode::Slider(slider) = slider("slider.control", unit) else { unreachable!() };
    item.control = Some(UiControlNode::Slider(slider));
    UiNode::Tree(UiTreeNode {
        presentation: ui_contract::TreePresentation::Compact,
        sections: vec![UiTreeSectionNode { header_toolbar: None, window: None, id: "slider.section".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
}

#[test]
fn inline_tree_slider_paints_the_same_external_unit_sibling_and_preserves_its_accepted_width() {
    let (mut tree, root, theme, mut atlas) = setup(&compact_tree_slider(Some("mm")));
    let section = tree.explicit_child(root, "slider.section").unwrap();
    let item = tree.explicit_child(section, "slider.item").unwrap();
    let slider = tree.explicit_child(item, "slider.control").unwrap();
    assert!(tree.accepted_inline_suffix_width(slider).unwrap() > 0.0);
    let draw = paint_retained_node_for_test(&tree, slider, &theme, &mut atlas);
    let unit_glyphs =
        draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01 && instance.color == [theme.text_muted.r, theme.text_muted.g, theme.text_muted.b, theme.text_muted.a]).count();
    assert_eq!(unit_glyphs, "0.5 mm".chars().count(), "the inline Tree control paints the same external '0.5 mm' sibling as a standalone Slider");
}

#[test]
fn compact_slider_edit_uses_the_unscoped_input_font_while_its_resting_readout_stays_compact() {
    let paint = |editing: bool| {
        let (mut tree, root, theme, mut atlas) = setup(&compact_tree_slider(None));
        let section = tree.explicit_child(root, "slider.section").unwrap();
        let item = tree.explicit_child(section, "slider.item").unwrap();
        let slider = tree.explicit_child(item, "slider.control").unwrap();
        if editing {
            tree.node_mut(slider).unwrap().state.edit = Some(EditState { text: "0.5".into(), caret: 3, anchor: 3, composition: None, scroll_x: 0.0 });
        }
        let draw = paint_retained_node_for_test(&tree, slider, &theme, &mut atlas);
        let mut positions: Vec<f32> = draw
            .layers
            .iter()
            .flat_map(|layer| layer.ui_instances.iter())
            .filter(|instance| {
                (instance.params[2] - KIND_GLYPH).abs() < 0.01
                    && if editing { instance.color == [theme.text.r, theme.text.g, theme.text.b, theme.text.a] } else { instance.color == [theme.text_element.r, theme.text_element.g, theme.text_element.b, theme.text_element.a] }
            })
            .map(|instance| instance.rect[0])
            .collect();
        positions.sort_by(f32::total_cmp);
        positions.last().copied().unwrap_or_default() - positions.first().copied().unwrap_or_default()
    };
    let resting = paint(false);
    let editing = paint(true);
    assert!(editing > resting, "React's compact rest text is 9.6px, while its numeric Input edit text uses the wider unscoped 12.8px body advance ({editing} <= {resting})");
}

#[test]
fn slider_paints_the_shared_rail_range_thumb_and_numeric_readout() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎚️slider-presentation/🔣️.json")).unwrap();
    let case = law["cases"].as_array().unwrap().iter().find(|case| case["id"] == "fractional-middle").unwrap();
    let rect = |value: &serde_json::Value| [value[0].as_f64().unwrap() as f32, value[1].as_f64().unwrap() as f32, value[2].as_f64().unwrap() as f32, value[3].as_f64().unwrap() as f32];
    let node =
        UiSliderNode { id: "slider".into(), value: case["value"].as_f64().unwrap(), min: case["min"].as_f64().unwrap(), max: case["max"].as_f64().unwrap(), step: 0.1, unit: None, snaps: Vec::new(), on_change: action(), presence: UiPresence::default(), menu: None, ..Default::default() };
    let bounds = Rect::new(case["bounds"][0].as_f64().unwrap() as f32, case["bounds"][1].as_f64().unwrap() as f32, case["bounds"][2].as_f64().unwrap() as f32, case["bounds"][3].as_f64().unwrap() as f32);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let mut draw = DrawList::default();
    paint_slider(&node, bounds, &theme, &mut atlas, &mut draw);
    let instances: Vec<_> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).collect();
    let rounded: Vec<_> = instances.iter().copied().filter(|instance| (instance.params[2] - KIND_ROUNDED).abs() < 0.01).collect();
    assert_eq!(rounded.len(), 3);
    for (instance, expected) in rounded.iter().zip([rect(&case["rail"]), rect(&case["range"]), rect(&case["thumb"])]) {
        assert!(instance.rect.iter().zip(expected).all(|(left, right)| (*left - right).abs() < 0.001));
    }
    assert_eq!(rounded[0].color, [theme.muted.r, theme.muted.g, theme.muted.b, theme.muted.a]);
    assert_eq!(rounded[1].color, [theme.text_element.r, theme.text_element.g, theme.text_element.b, theme.text_element.a]);
    assert_eq!(rounded[2].color, [theme.text_element.r, theme.text_element.g, theme.text_element.b, theme.text_element.a]);
    let glyphs: Vec<_> = instances.iter().copied().filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).collect();
    assert_eq!(glyphs.len(), case["formatted"].as_str().unwrap().chars().count());
    assert!(glyphs.iter().all(|glyph| glyph.rect[0] >= case["valueCell"][0].as_f64().unwrap() as f32));
}

fn field(description: Option<&str>, required: bool, error: Option<&str>) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: "f".into(),
        label: Label::data("Label"),
        description: description.map(String::from),
        required: Some(required),
        error: error.map(String::from),
        child: Box::new(text("child")),
        presence: UiPresence::default(),
        menu: None,
    })
}

#[test]
fn painting_a_field_with_description_required_and_error_emits_extra_glyphs() {
    let (mut bare_tree, bare_root, theme, mut bare_atlas) = setup(&field(None, false, None));
    let mut bare_draw = DrawList::default();
    paint_tree(&mut bare_tree, bare_root, &theme, &mut bare_atlas, None, false, &mut bare_draw);

    let (mut rich_tree, rich_root, theme2, mut rich_atlas) = setup(&field(Some("desc"), true, Some("bad")));
    let mut rich_draw = DrawList::default();
    paint_tree(&mut rich_tree, rich_root, &theme2, &mut rich_atlas, None, false, &mut rich_draw);

    let bare_total: usize = bare_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    let rich_total: usize = rich_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    assert!(rich_total > bare_total, "description/required-marker/error should each add glyph instances beyond the bare label");
}

fn tree_with_item_description() -> UiNode {
    let mut item = UiTreeItemNode::base("i1", Label::data("Item One"));
    item.description = Some("desc".into());
    item.actions = Some(vec![UiTreeItemAction { icon_id: IconName::Sparkles, label: None, action: action(), placement: None, disabled: false, reason: None }]);
    UiNode::Tree(UiTreeNode {
        presentation: Default::default(),
        sections: vec![UiTreeSectionNode { header_toolbar: None, window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
}

fn tree_with_bare_item() -> UiNode {
    UiNode::Tree(UiTreeNode {
        presentation: Default::default(),
        sections: vec![UiTreeSectionNode { header_toolbar: None, window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", Label::data("Item One"))] }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
}

#[test]
fn painting_a_tree_item_with_description_emits_more_than_a_bare_item() {
    let (mut bare_tree, bare_root, theme, mut bare_atlas) = setup(&tree_with_bare_item());
    let mut bare_draw = DrawList::default();
    paint_tree(&mut bare_tree, bare_root, &theme, &mut bare_atlas, None, false, &mut bare_draw);

    let (mut rich_tree, rich_root, theme2, mut rich_atlas) = setup(&tree_with_item_description());
    let mut rich_draw = DrawList::default();
    paint_tree(&mut rich_tree, rich_root, &theme2, &mut rich_atlas, None, false, &mut rich_draw);

    let bare_total: usize = bare_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    let rich_total: usize = rich_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    assert!(rich_total > bare_total, "a tree item's description text should paint extra glyphs beyond a bare item (icons are None here, so its always-visible action doesn't add its own icon instance in this fixture)");
}
//#endregion 🔖️FidelityFixes
