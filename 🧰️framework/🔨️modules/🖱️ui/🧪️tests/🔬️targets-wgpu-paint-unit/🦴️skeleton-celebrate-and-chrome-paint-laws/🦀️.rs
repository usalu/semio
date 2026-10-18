// 🦴️ Skeleton, celebrate and chrome paint laws: the skeleton predicate and blocks, the celebrate cycle, ring thickness and colour, the section and field chrome bands and the number stepper catalog icons.

//#region 🦴️SkeletonPaint
// 🦴️ React's `interpretUiNodeBusyShell` (`🗣️Interpreter/🟦️.tsx:2147-2154`) swaps a busy element for
// `elementSkeleton(kind)` entirely. These pin the same predicate, the same per-kind shape family, and
// that the retained paint step really draws blocks instead of the element's own content.

#[test]
fn the_skeleton_predicate_matches_reacts_busy_shell() {
    let busy = UiPresence::status(UiStatus::Loading);
    let waiting = UiPresence::status(UiStatus::Waiting);
    let idle = UiPresence::default();
    assert!(skeleton_replaces_content(&text("hi"), &busy));
    assert!(skeleton_replaces_content(&text("hi"), &waiting));
    assert!(!skeleton_replaces_content(&text("hi"), &idle));
    assert!(!skeleton_replaces_content(&UiNode::Progress(progress(1.0, Some(2.0))), &busy), "a progress bar IS the busy affordance — React exempts it");
}

#[test]
fn each_element_kind_picks_reacts_own_skeleton_shape() {
    assert_eq!(skeleton_kind(&text("x")), SkeletonKind::Line);
    assert_eq!(skeleton_kind(&button("b", false)), SkeletonKind::Control);
    assert_eq!(skeleton_kind(&image_node("i", "")), SkeletonKind::Image);
    assert_eq!(skeleton_kind(&stack(Vec::new())), SkeletonKind::Fill);
}

#[test]
fn a_skeletons_blocks_stay_inside_their_bounds_and_within_the_fixed_capacity() {
    let theme = Theme::default();
    let bounds = Rect::new(4.0, 8.0, 200.0, 120.0);
    for kind in [SkeletonKind::Line, SkeletonKind::Control, SkeletonKind::Separator, SkeletonKind::Image, SkeletonKind::Rows, SkeletonKind::Ring, SkeletonKind::Field, SkeletonKind::Panel, SkeletonKind::Fill] {
        let (blocks, count) = skeleton_blocks(kind, bounds, &theme);
        assert!((1..=SKELETON_MAX_BLOCKS).contains(&count), "{kind:?} draws between one block and the fixed ceiling");
        for block in blocks.iter().take(count) {
            assert!(block.x >= bounds.x && block.y >= bounds.y, "{kind:?} never starts outside its rect");
            assert!(block.w >= 0.0 && block.h >= 0.0, "{kind:?} never emits a negative extent");
            assert!(block.x + block.w <= bounds.x + bounds.w + 0.01, "{kind:?} never overflows its width");
        }
    }
}

#[test]
fn a_loading_text_node_paints_skeleton_blocks_instead_of_its_glyphs() {
    let mut node = text("hello");
    let UiNode::Text(inner) = &mut node else { panic!("text fixture") };
    inner.presence = UiPresence::status(UiStatus::Loading);
    let (tree, root, theme, mut atlas) = setup(&node);
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    for _ in 0..8 {
        if paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor) == RetainedNodePaintStep::Complete {
            break;
        }
    }
    let instances: Vec<_> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).collect();
    assert_eq!(instances.iter().filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).count(), 0, "React never paints half-built text under `activity: loading`");
    assert!(instances.iter().any(|instance| (instance.params[2] - KIND_LOADING_BORDER).abs() < 0.01), "the loading border treatment stays");
    assert!(instances.len() > 1, "a shaped placeholder block is drawn, not just the border tint");
}
//#endregion 🦴️SkeletonPaint

//#region 🎉️CelebratePaint
// 🎉️ W2k: a celebrating element used to paint a STATIC `theme.accent` hairline ring, because `Theme`
// carried no primary/secondary/tertiary triad and no frame clock reached paint. Both now exist, so
// the ring spins through React's own conic stops and bursts to `--stroke-focus` at React's timing.

#[test]
fn the_celebrate_cycle_wraps_once_per_react_duration_and_never_divides_by_zero() {
    let theme = Theme::default();
    assert_eq!(theme.celebrate_duration_seconds, 1.2, "React's `--celebrate-border-duration`");
    assert_eq!(celebrate_turns(0.0, theme.celebrate_duration_seconds), 0.0);
    assert!((celebrate_turns(0.6, theme.celebrate_duration_seconds) - 0.5).abs() < 1e-6);
    assert!(celebrate_turns(1.2, theme.celebrate_duration_seconds).abs() < 1e-6, "one duration is exactly one turn");
    assert!((celebrate_turns(3.0, theme.celebrate_duration_seconds) - 0.5).abs() < 1e-6, "the cycle is infinite, so it wraps");
    assert_eq!(celebrate_turns(1.0, 0.0), 0.0, "a zero duration freezes at the start");
    assert_eq!(celebrate_turns(f32::NAN, 1.2), 0.0);
}

#[test]
fn the_ring_thickness_bursts_from_hairline_to_the_focus_stroke_and_back() {
    let theme = Theme::default();
    assert!(theme.stroke_focus > theme.stroke_hairline, "React's `--stroke-focus` is three hairlines");
    assert!((celebrate_stroke(&theme, 0.0) - theme.stroke_hairline).abs() < 1e-5, "0 % is hairline");
    assert!((celebrate_stroke(&theme, 0.5) - theme.stroke_focus).abs() < 1e-5, "50 % is the focus stroke");
    assert!((celebrate_stroke(&theme, 1.0) - theme.stroke_hairline).abs() < 1e-5, "100 % is hairline again");
    assert!(celebrate_stroke(&theme, 0.25) > theme.stroke_hairline && celebrate_stroke(&theme, 0.25) < theme.stroke_focus, "the ramp is eased, not stepped");
}

#[test]
fn the_ring_colour_cycles_the_three_conic_stops_and_wraps_back_to_the_first() {
    let theme = Theme::default();
    let [primary, secondary, tertiary] = theme.celebrate;
    assert_ne!(primary, secondary);
    assert_ne!(secondary, tertiary);
    let near = |left: Rgba, right: Rgba| (left.r - right.r).abs() < 1e-5 && (left.g - right.g).abs() < 1e-5 && (left.b - right.b).abs() < 1e-5 && (left.a - right.a).abs() < 1e-5;
    assert!(near(celebrate_color(&theme, 0.0), primary));
    assert!(near(celebrate_color(&theme, 1.0 / 3.0), secondary));
    assert!(near(celebrate_color(&theme, 2.0 / 3.0), tertiary));
    assert!(near(celebrate_color(&theme, 1.0), primary), "the conic wraps through primary again");
    let midway = celebrate_color(&theme, 1.0 / 6.0);
    assert!(midway != primary && midway != secondary, "a sample between two stops is interpolated, not snapped");
}

#[test]
fn a_celebrating_element_paints_a_different_ring_as_the_clock_advances_while_an_introducing_one_does_not() {
    let ring = |state: UiState, seconds: f32| {
        let mut node = button("b", false);
        if let UiNode::Button(button) = &mut node {
            button.presence.state = state;
        }
        let (mut tree, root, theme, mut atlas) = setup(&node);
        let mut draw = DrawList::default();
        draw.set_clock_seconds(seconds);
        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);
        draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| (instance.color, instance.params)).collect::<Vec<_>>()
    };
    assert_ne!(ring(UiState::Celebrating, 0.0), ring(UiState::Celebrating, 0.6), "the celebrate ring is time-varying — that IS the spin and the burst");
    assert_eq!(ring(UiState::Introducing, 0.0), ring(UiState::Introducing, 0.6), "the introduce pulse is shader-side, so paint emits the same instance at any clock");
}
//#endregion 🎉️CelebratePaint

#[test]
fn section_and_field_chrome_paint_only_inside_their_measured_bands() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📐️section-field-presentation/🔣️.json")).expect("section field fixture");
    let number = |path: &[&str]| path.iter().fold(&fixture, |value, key| &value[*key]).as_f64().expect("fixture number") as f32;
    let text_value = |path: &[&str]| path.iter().fold(&fixture, |value, key| &value[*key]).as_str().expect("fixture text");
    let control = || UiNode::Button(UiButtonNode { id: Some("geometry.control".into()), icon_id: IconName::CircleDot, label: Label::data("Control"), action: action(), style: None, presence: UiPresence::default(), menu: None });

    let section = UiNode::Section(UiSectionNode { id: "geometry-section".into(), label: Some(Label::data(text_value(&["section", "title"]))), default_open: Some(true), presence: UiPresence::default(), children: vec![control()], menu: None });
    let mut section_tree = UiTree::new();
    section_tree.apply_tree(&section);
    let section_root = section_tree.root.expect("section root");
    let theme = Theme::default();
    let mut atlas = FontAtlas::shaped_default();
    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut section_tree, section_root, theme, number(&["section", "width"]), 400.0));
    let section_child = section_tree.node(section_root).and_then(|node| node.first_child).expect("section child");
    let title_bottom = section_tree.accepted_layout(section_child).expect("section child layout").y - crate::wgpu::flex::SECTION_TITLE_BODY_GAP;
    let mut section_draw = DrawList::default();
    paint_tree(&mut section_tree, section_root, &theme, &mut atlas, None, false, &mut section_draw);
    let title_glyphs: Vec<_> = section_draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01 && instance.rect[1] < title_bottom).collect();
    assert!(!title_glyphs.is_empty());
    assert!(title_glyphs.iter().all(|glyph| glyph.rect[1] + glyph.rect[3] <= title_bottom + 0.01), "title glyphs stay inside the wrapped title rect");
    assert!(title_glyphs.len() >= text_value(&["section", "title"]).chars().filter(|scalar| !scalar.is_whitespace()).count() * 2, "the Section title uses semibold double strikes");

    let field = UiNode::Field(UiFieldNode {
        id: "geometry-field".into(),
        label: Label::data(text_value(&["field", "label"])),
        description: Some(text_value(&["field", "description"]).into()),
        required: Some(false),
        error: Some(text_value(&["field", "error"]).into()),
        child: Box::new(control()),
        presence: UiPresence::default(),
        menu: None,
    });
    let mut field_tree = UiTree::new();
    field_tree.apply_tree(&field);
    let field_root = field_tree.root.expect("field root");
    let field_control = field_tree.node(field_root).and_then(|node| node.first_child).expect("field control");
    let mut field_atlas = FontAtlas::shaped_default();
    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut field_tree, field_root, theme, number(&["field", "width"]), 400.0));
    let control_layout = field_tree.accepted_layout(field_control).expect("field control layout");
    let mut field_draw = DrawList::default();
    paint_tree(&mut field_tree, field_root, &theme, &mut field_atlas, None, false, &mut field_draw);
    let color_matches = |instance: &&crate::wgpu::draw_types::UiInstance, color: crate::wgpu::theme::Rgba| (instance.color[0] - color.r).abs() < 0.001 && (instance.color[1] - color.g).abs() < 0.001 && (instance.color[2] - color.b).abs() < 0.001;
    let glyphs: Vec<_> = field_draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).collect();
    let description: Vec<_> = glyphs.iter().copied().filter(|instance| color_matches(instance, theme.text_muted)).collect();
    let error: Vec<_> = glyphs.iter().copied().filter(|instance| color_matches(instance, theme.error)).collect();
    assert!(!description.is_empty() && !error.is_empty());
    assert!(description.iter().all(|glyph| glyph.rect[1] >= crate::wgpu::flex::FIELD_LABEL_LINE_HEIGHT && glyph.rect[1] + glyph.rect[3] <= control_layout.y + 0.01));
    assert!(error.iter().all(|glyph| glyph.rect[1] >= control_layout.y + control_layout.height + theme.gap_standard - 0.01));
}

#[test]
fn number_stepper_catalog_icons_match_chromiums_measured_content_boxes() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️stepper-presentation/🔣️.json")).unwrap();
    let geometry = law["geometry"].as_array().unwrap().iter().find(|case| case["id"] == law["icons"]["case"]).unwrap();
    let rect = |value: &serde_json::Value| Rect::new(value[0].as_f64().unwrap() as f32, value[1].as_f64().unwrap() as f32, value[2].as_f64().unwrap() as f32, value[3].as_f64().unwrap() as f32);
    let theme = Theme { stroke_hairline: geometry["border"].as_f64().unwrap() as f32, ..Theme::default() };
    let [minus, _, plus] = crate::wgpu::layout::number_stepper_segments(rect(&geometry["bounds"]), ui_contract::FlowInline::Ltr, theme.stroke_hairline);
    let icons = IconAtlas::from_packed(2, 1, vec![255; 8], vec![("minus".into(), [0.0, 0.0, 0.5, 1.0]), ("plus".into(), [0.5, 0.0, 0.5, 1.0])]);
    let mut draw = DrawList::default();
    push_stepper_icon(&mut draw, Some(&icons), "minus", minus, ui_contract::FlowInline::Ltr, &theme);
    push_stepper_icon(&mut draw, Some(&icons), "plus", plus, ui_contract::FlowInline::Ltr, &theme);
    let instances: Vec<_> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).collect();
    assert_eq!(instances.len(), 2);
    for (instance, expected) in instances.iter().zip(law["icons"]["rects"].as_array().unwrap()) {
        for (actual, expected) in instance.rect.iter().zip(expected.as_array().unwrap()) {
            assert!((f64::from(*actual) - expected.as_f64().unwrap()).abs() <= 1.0 / 64.0, "Chromium layout-unit tolerance");
        }
    }
}
