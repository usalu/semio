// ✍️ Input caret and focus ring paint laws: the input fixtures, the caret, selection and live edit buffer paint, the retained caret cadence and the accent focus ring on every control.

//#region 🔖️W2WidgetVisuals
// 🖱️✍️🎯️ Tests for `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`'s W2 widget-visuals pass: a
// focused `Input`'s caret/selection-highlight (`paint_input`, sourced from `tree::EditState`),
// and the `formControlFocusBorderClass`-matching focus ring ported onto every remaining
// focusable control kind (`Button`/`Select`/`Toggle`/`NumberStepper`, plus a
// `NumberStepper` hover tint) that only `paint_input` had before this pass.
fn input(id: &str, value: &str) -> UiNode {
    UiNode::Input(UiInputNode {
        id: id.into(),
        input_kind: "text".into(),
        value: value.into(),
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
    })
}

// ⌨️ Keyboard focus: BOTH bits, the way `EventRouter::set_focus` stamps them for a `Tab` move.
// The accent ring is `focus-visible:border-accent`, so `FOCUSED` alone (a pointer press) must not
// paint one — `focus_pointer` below is the counter-case (ticket 26/09/17 packet W2k).
fn focus(tree: &mut UiTree, id: NodeId) {
    tree.node_mut(id).unwrap().flags.set(NodeFlags::FOCUSED, true);
    tree.node_mut(id).unwrap().flags.set(NodeFlags::FOCUS_VISIBLE, true);
    tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
}

// 👆️ Pointer focus: `FOCUSED` without `FOCUS_VISIBLE`, which is what a click produces.
fn focus_pointer(tree: &mut UiTree, id: NodeId) {
    tree.node_mut(id).unwrap().flags.set(NodeFlags::FOCUSED, true);
    tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
}

fn has_solid_instance_colored(draw: &DrawList, color: Rgba) -> bool {
    draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| {
        (instance.params[2] - KIND_SOLID).abs() < 0.01 && (instance.color[0] - color.r).abs() < 0.001 && (instance.color[1] - color.g).abs() < 0.001 && (instance.color[2] - color.b).abs() < 0.001 && (instance.color[3] - color.a).abs() < 0.001
    })
}

#[test]
fn painting_an_unfocused_input_emits_no_caret_or_selection() {
    let (mut tree, root, theme, mut atlas) = setup(&input("in", "hello"));
    tree.node_mut(root).unwrap().state.edit = Some(EditState { text: "hello".into(), caret: 5, anchor: 0, composition: None, scroll_x: 0.0 });
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    assert!(!has_solid_instance_colored(&draw, theme.accent), "an unfocused Input must not paint a caret even if a stale EditState lingers on it");
    assert!(!has_solid_instance_colored(&draw, theme.accent.with_alpha(0.3)), "an unfocused Input must not paint a selection highlight");
}

#[test]
fn painting_a_focused_input_with_a_collapsed_selection_emits_a_caret_line_but_no_highlight() {
    let (mut tree, root, theme, mut atlas) = setup(&input("in", "hello"));
    focus(&mut tree, root);
    tree.node_mut(root).unwrap().state.edit = Some(EditState { text: "hello".into(), caret: 5, anchor: 5, composition: None, scroll_x: 0.0 });
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    assert!(has_solid_instance_colored(&draw, theme.accent), "a focused Input should paint its caret as a theme.accent solid instance");
    assert!(!has_solid_instance_colored(&draw, theme.accent.with_alpha(0.3)), "a collapsed selection (anchor == caret) must not paint a highlight rect");
}

#[test]
fn painting_a_focused_input_with_a_real_selection_emits_a_translucent_highlight() {
    let (mut tree, root, theme, mut atlas) = setup(&input("in", "hello world"));
    focus(&mut tree, root);
    tree.node_mut(root).unwrap().state.edit = Some(EditState { text: "hello world".into(), caret: 5, anchor: 0, composition: None, scroll_x: 0.0 });
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    assert!(has_solid_instance_colored(&draw, theme.accent.with_alpha(0.3)), "a real anchor..caret selection should paint a theme.accent-at-0.3-alpha highlight rect");
}

#[test]
fn painting_a_focused_input_shows_the_live_edit_buffer_text_not_the_stale_declarative_value() {
    let (mut tree, root, theme, mut atlas) = setup(&input("in", "old"));
    focus(&mut tree, root);
    tree.node_mut(root).unwrap().state.edit = Some(EditState { text: "a much longer buffer".into(), caret: 20, anchor: 20, composition: None, scroll_x: 0.0 });
    let mut draw = DrawList::default();

    let (mut stale_tree, stale_root, stale_theme, mut stale_atlas) = setup(&input("in", "old"));
    let mut stale_draw = DrawList::default();
    paint_tree(&mut stale_tree, stale_root, &stale_theme, &mut stale_atlas, None, false, &mut stale_draw);
    let stale_total: usize = stale_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);
    let focused_total: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

    assert!(focused_total > stale_total, "a focused Input with a live EditState should paint its live (longer) buffer text, not the stale shorter declarative value");
}

#[test]
fn retained_input_paints_a_caret_only_while_the_accepted_cadence_is_visible() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧬️contract/⌨️caret-cadence/🧪️tests/🧫️fixtures/🔣️.json")).expect("caret cadence fixture");
    assert_eq!(fixture["halfPeriodMs"].as_u64(), Some(500));
    let (mut tree, root, theme, mut atlas) = setup(&input("in", "hello"));
    let node = tree.node_mut(root).expect("retained input");
    node.state.edit = Some(EditState { text: "hello".into(), caret: 5, anchor: 5, composition: None, scroll_x: 0.0 });
    node.state.caret_visible = true;
    let visible = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    assert!(has_solid_instance_colored(&visible, theme.text), "the accepted visible phase paints one caret primitive in the text colour, as a native input does");

    tree.node_mut(root).expect("retained input").state.caret_visible = false;
    let hidden = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    assert!(!has_solid_instance_colored(&hidden, theme.text), "the hidden phase keeps the edit buffer while omitting its caret primitive");
}

//#region 🔖️RetainedTextInputLaws
fn instances_of(draw: &DrawList, kind: f32, color: Rgba) -> Vec<[f32; 4]> {
    draw.layers
        .iter()
        .flat_map(|layer| layer.ui_instances.iter())
        .filter(|instance| (instance.params[2] - kind).abs() < 0.01 && (instance.color[0] - color.r).abs() < 0.001 && (instance.color[1] - color.g).abs() < 0.001 && (instance.color[2] - color.b).abs() < 0.001 && (instance.color[3] - color.a).abs() < 0.001)
        .map(|instance| instance.rect)
        .collect()
}

fn fills_of(draw: &DrawList, color: Rgba) -> Vec<[f32; 4]> {
    instances_of(draw, KIND_SOLID, color).into_iter().filter(|rect| rect[2] > 1.5 && rect[3] > 1.5).collect()
}

fn input_of(kind: &str, value: &str) -> UiNode {
    let UiNode::Input(mut field) = input("in", value) else { unreachable!() };
    field.input_kind = kind.into();
    UiNode::Input(field)
}

fn editing_input(kind: &str, value: &str, text: &str, anchor: usize, caret: usize) -> (UiTree, NodeId, Theme, FontAtlas) {
    let (mut tree, root, theme, atlas) = setup_flowed(&input_of(kind, value));
    focus(&mut tree, root);
    let node = tree.node_mut(root).expect("retained input");
    node.state.edit = Some(EditState { text: text.into(), caret, anchor, composition: None, scroll_x: 0.0 });
    node.state.caret_visible = true;
    (tree, root, theme, atlas)
}

fn inner_origin(tree: &UiTree, root: NodeId, theme: &Theme) -> (f32, f32, f32) {
    let bounds = tree.absolute_rect(root).expect("bounds");
    (bounds.x + theme.padding_standard, bounds.y, bounds.w - theme.padding_standard * 2.0)
}

#[test]
fn a_retained_selection_paints_the_accent_highlight_under_text_in_the_selection_colour() {
    let (tree, root, theme, mut atlas) = editing_input("text", "hello world", "hello world", 0, 5);
    let theme = Theme { border_emphasized: theme.error, ..theme };
    let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    let (x, _, _) = inner_origin(&tree, root, &theme);
    let highlights = fills_of(&draw, theme.accent);
    assert_eq!(highlights.len(), 1, "one highlight for a one-line selection");
    let width = atlas.measure_range("hello world", 0, 5, theme.font_size_body);
    assert!((highlights[0][0] - x).abs() < 0.01 && (highlights[0][2] - width).abs() < 0.05, "the highlight spans exactly the selected scalars: {:?} vs x {x} width {width}", highlights[0]);
    assert!((highlights[0][3] - semio_ui_text::line_height(theme.font_size_body)).abs() < 0.01, "one line box tall");
    assert_eq!(instances_of(&draw, KIND_GLYPH, theme.border_emphasized).len(), 5, "React's ::selection text colour on the selected scalars only");
    assert!(instances_of(&draw, KIND_SOLID, theme.text).is_empty(), "a real selection paints no caret");
}

#[test]
fn a_retained_caret_sits_after_the_scalars_before_it_in_the_text_colour() {
    let (tree, root, theme, mut atlas) = editing_input("text", "hello", "hello", 3, 3);
    let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    let (x, _, _) = inner_origin(&tree, root, &theme);
    let carets = instances_of(&draw, KIND_SOLID, theme.text);
    assert_eq!(carets.len(), 1);
    assert!((carets[0][0] - (x + atlas.measure_range("hello", 0, 3, theme.font_size_body))).abs() < 0.05 && (carets[0][2] - 1.0).abs() < 0.001, "{:?}", carets[0]);
}

#[test]
fn a_retained_preedit_is_underlined_and_the_caret_rests_inside_it() {
    let (mut tree, root, theme, mut atlas) = editing_input("text", "ab", "ab", 1, 1);
    let node = tree.node_mut(root).expect("retained input");
    node.state.edit.as_mut().expect("edit").composition = Some("ねこ".into());
    node.state.composition_cursor = Some(3);
    let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    let (x, _, _) = inner_origin(&tree, root, &theme);
    let size = theme.font_size_body;
    let solids = instances_of(&draw, KIND_SOLID, theme.text);
    let underline = solids.iter().find(|rect| rect[3] < 2.5).expect("the preedit underline");
    let display = "aねこb";
    assert!((underline[0] - (x + atlas.measure_range(display, 0, 1, size))).abs() < 0.05 && (underline[2] - atlas.measure_range(display, 1, 7, size)).abs() < 0.05, "the underline spans the composition: {underline:?}");
    let caret = solids.iter().find(|rect| (rect[2] - 1.0).abs() < 0.001).expect("the caret");
    assert!((caret[0] - (x + atlas.measure_range(display, 0, 4, size))).abs() < 0.05, "the caret rests after the first preedit scalar: {caret:?}");
    assert!(fills_of(&draw, theme.accent).is_empty(), "a preedit paints no selection highlight");
}

#[test]
fn a_long_text_input_keeps_a_four_and_a_half_rem_box_and_wraps_to_the_carets_line() {
    let text = "word ".repeat(120);
    let (mut tree, root, theme, mut atlas) = editing_input("longText", "", &text, text.len(), text.len());
    semio_ui_wgpu_paint::refresh_input_text_layout(&mut tree, root, &theme, &mut atlas);
    assert_eq!(tree.accepted_layout(root).expect("layout").height, semio_ui_wgpu_layout::layout::TEXTAREA_MIN_HEIGHT, "React's min-h-[4.5rem]");
    let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    let (x, top, width) = inner_origin(&tree, root, &theme);
    let size = theme.font_size_body;
    let lines = atlas.wrap_lines(&text, width, size);
    assert!(lines.len() > 3, "the text wraps over several lines");
    let line_height = semio_ui_text::line_height(size);
    let box_height = tree.absolute_rect(root).expect("bounds").h - theme.padding_standard * 2.0;
    let scroll = ((lines.len() as f32) * line_height - box_height).max(0.0);
    let laid = tree.node(root).expect("input").state.edit_layout.clone().expect("the refresh parks the layout");
    assert_eq!((laid.lines, laid.stops.len(), laid.stops.last().map(|stop| stop.byte)), (lines.len(), text.chars().count() + 1, Some(text.len())), "one stop per scalar plus the end, over the very lines the wrap law gives");
    assert!((tree.node(root).expect("input").state.scroll_offset.1 - scroll).abs() < 0.01, "the caret at the end scrolls the box to its last line");
    let carets = instances_of(&draw, KIND_SOLID, theme.text);
    assert_eq!(carets.len(), 1);
    let expected_top = top + theme.padding_standard - scroll + line_height * (lines.len() - 1) as f32;
    assert!((carets[0][1] - expected_top).abs() < 0.05, "the caret is on the last wrapped line, scrolled into the box: {:?} vs {expected_top}", carets[0]);
    assert!(carets[0][1] >= top && carets[0][1] + carets[0][3] <= top + 72.0 + 0.01, "inside the box");
    let last = &text[lines.last().expect("a line").clone()];
    assert!((carets[0][0] - (x + atlas.measure_range(last, 0, last.len(), size)).min(x + width - 1.0)).abs() < 0.1, "the caret (kept inside the box) follows the last line's pen: {:?} vs x {x} measure {}", carets[0], atlas.measure_range(last, 0, last.len(), size));
}

#[test]
fn a_long_text_caret_after_a_line_break_starts_the_next_line() {
    let (tree, root, theme, mut atlas) = editing_input("longText", "", "ab\ncd", 4, 4);
    let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    let (x, top, _) = inner_origin(&tree, root, &theme);
    let size = theme.font_size_body;
    let caret = instances_of(&draw, KIND_SOLID, theme.text)[0];
    assert!((caret[1] - (top + theme.padding_standard + semio_ui_text::line_height(size))).abs() < 0.05, "second line: {caret:?}");
    assert!((caret[0] - (x + atlas.measure_range("cd", 0, 1, size))).abs() < 0.05);
}

#[test]
fn a_long_text_selection_over_two_lines_paints_one_highlight_per_line() {
    let (tree, root, theme, mut atlas) = editing_input("longText", "", "ab\ncd", 1, 4);
    let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    let highlights = fills_of(&draw, theme.accent);
    assert_eq!(highlights.len(), 2, "{highlights:?}");
    assert!(highlights[1][1] > highlights[0][1] && highlights[1][0] < highlights[0][0] + highlights[0][2], "the second line starts at the line's left edge");
}

#[test]
fn a_single_line_input_scrolls_sideways_to_keep_its_caret_inside() {
    let text = "m".repeat(200);
    let (mut tree, root, theme, mut atlas) = editing_input("text", "", &text, 200, 200);
    semio_ui_wgpu_paint::refresh_input_text_layout(&mut tree, root, &theme, &mut atlas);
    assert!(tree.node(root).expect("input").state.scroll_offset.0 > 0.0, "the caret beyond the box scrolls it sideways");
    let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    let (x, _, width) = inner_origin(&tree, root, &theme);
    let caret = instances_of(&draw, KIND_SOLID, theme.text)[0];
    assert!(caret[0] >= x && caret[0] <= x + width, "the caret stays inside the text box: {caret:?} in {x}..{}", x + width);
}

#[test]
fn a_refused_number_draft_paints_its_message_below_the_field_in_the_error_colour() {
    let (mut tree, root, theme, mut atlas) = editing_input("number", "4", "40", 2, 2);
    tree.node_mut(root).expect("retained input").state.number_refusal = Some("Must be at most 10 mm".into());
    let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    let bounds = tree.absolute_rect(root).expect("bounds");
    let row = semio_ui_wgpu_layout::layout::input_refusal_row_height(theme.font_size_small);
    let below: Vec<_> = instances_of(&draw, KIND_GLYPH, theme.error).into_iter().filter(|rect| rect[1] >= bounds.y + bounds.h - row - 1.0).collect();
    assert!(below.len() >= 16, "every visible scalar of the message is penned below the field: {}", below.len());
    let silent = {
        tree.node_mut(root).expect("retained input").state.number_refusal = Some(String::new());
        paint_retained_node_for_test(&tree, root, &theme, &mut atlas)
    };
    assert!(instances_of(&silent, KIND_GLYPH, theme.error).iter().all(|rect| rect[1] < bounds.y + bounds.h - 1.0), "an unlabelled refusal paints the red field only");
}

#[test]
fn a_refusal_reserves_its_row_under_the_box_and_the_layout_pass_is_asked_for_it() {
    let (mut tree, root, theme, mut atlas) = editing_input("number", "4", "40", 2, 2);
    let base = tree.accepted_layout(root).expect("layout").height;
    tree.node_mut(root).expect("retained input").state.number_refusal = Some("Must be at most 10 mm".into());
    let _ = tree.take_disclosure_changed();
    semio_ui_wgpu_paint::sync_input_refusal_rows(&mut tree, root);
    assert!(tree.node(root).expect("input").state.refusal_row_laid && tree.take_disclosure_changed(), "a refusal appearing asks for a layout pass once");
    semio_ui_wgpu_paint::sync_input_refusal_rows(&mut tree, root);
    assert!(!tree.take_disclosure_changed(), "and not again while it stays");
    let top = tree.root.expect("top");
    assert!(semio_ui_wgpu_mounted_layout::layout_tree_now(&mut tree, top, theme, 400.0, 400.0), "layout pass");
    let row = semio_ui_wgpu_layout::layout::input_refusal_row_height(theme.font_size_small);
    assert!((tree.accepted_layout(root).expect("layout").height - (base + row)).abs() < 0.01, "the field grows by React's text-xs leading-tight row");
    let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    let bounds = tree.absolute_rect(root).expect("bounds");
    assert!(instances_of(&draw, KIND_GLYPH, theme.error).iter().all(|rect| rect[1] + rect[3] <= bounds.y + bounds.h + 4.0), "the message sits inside the reserved row");
    tree.node_mut(root).expect("retained input").state.number_refusal = None;
    semio_ui_wgpu_paint::sync_input_refusal_rows(&mut tree, root);
    assert!(tree.take_disclosure_changed(), "the row going away asks again");
}

fn paint_retained_node_with_presentation_for_test(tree: &UiTree, node: NodeId, theme: &Theme, atlas: &mut FontAtlas, presentation: semio_ui_wgpu_paint::RetainedPaintPresentation) -> DrawList {
    let layout = tree.accepted_layout(node).expect("retained node layout");
    let absolute = tree.absolute_rect(node).expect("retained node bounds");
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    for _ in 0..16_384 {
        match paint_node_step_with_driver(tree, node, absolute.x - layout.x, absolute.y - layout.y, theme, atlas, None, false, presentation, false, ui_contract::FlowInline::Ltr, None, &mut draw, &mut cursor) {
            RetainedNodePaintStep::Pending => {}
            RetainedNodePaintStep::Complete => return draw,
            RetainedNodePaintStep::Fault => panic!("retained node paint fault"),
        }
    }
    panic!("retained node paint exceeded its fixed budget")
}

#[test]
fn a_mixed_stepper_shows_the_localized_caption_or_a_dash_when_no_language_is_named() {
    let (tree, root, theme, mut atlas) = setup_flowed(&stepper("ns", 0.0, false));
    for (locale, scalars) in [(None, 1), (Some(semio_ui_locale::Locale::En), 5), (Some(semio_ui_locale::Locale::De), 8)] {
        let draw = paint_retained_node_with_presentation_for_test(&tree, root, &theme, &mut atlas, semio_ui_wgpu_paint::RetainedPaintPresentation { locale, ..Default::default() });
        assert_eq!(instances_of(&draw, KIND_GLYPH, theme.text_muted).len(), scalars, "{locale:?}");
    }
}

#[test]
fn a_stepper_selection_and_preedit_paint_beneath_the_centred_value() {
    let (mut tree, root, theme, mut atlas) = setup_flowed(&stepper("ns", 12345.0, true));
    let theme = Theme { border_emphasized: theme.error, ..theme };
    focus(&mut tree, root);
    let node = tree.node_mut(root).expect("stepper");
    node.state.edit = Some(EditState { text: "12345".into(), caret: 3, anchor: 1, composition: None, scroll_x: 0.0 });
    let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    assert_eq!(fills_of(&draw, theme.accent).len(), 1, "the stepper's selection is the accent highlight too");
    assert_eq!(instances_of(&draw, KIND_GLYPH, theme.border_emphasized).len(), 2);
}
//#endregion 🔖️RetainedTextInputLaws

fn toggle(id: &str) -> UiNode {
    UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, id: id.into(), icon_id: IconName::CircleDot, text: Some(Label::data("Toggle")), on_change: action(), presence: UiPresence::default(), menu: None })
}


/// 🎯️ Shared assertion for the border-swap-on-focus fix: an otherwise-identical pair of trees,
/// one with `NodeFlags::FOCUSED` set on the root, should differ in at least one border instance's
/// color — `theme.selected` (`--active-base`) replacing `theme.border_normal`, the stroke React's border compositor paints
/// on a `:focus-visible` cell (`sharedBorderStroke`, `🧱️elements/🗂️WindowChrome/shared-border.ts`).
fn assert_focus_swaps_border_color(make: impl Fn() -> UiNode, label: &str) {
    let (mut unfocused_tree, unfocused_root, theme, mut unfocused_atlas) = setup(&make());
    let mut unfocused_draw = DrawList::default();
    paint_tree(&mut unfocused_tree, unfocused_root, &theme, &mut unfocused_atlas, None, false, &mut unfocused_draw);

    let (mut focused_tree, focused_root, focused_theme, mut focused_atlas) = setup(&make());
    focus(&mut focused_tree, focused_root);
    let mut focused_draw = DrawList::default();
    paint_tree(&mut focused_tree, focused_root, &focused_theme, &mut focused_atlas, None, false, &mut focused_draw);

    let (mut pointer_tree, pointer_root, pointer_theme, mut pointer_atlas) = setup(&make());
    focus_pointer(&mut pointer_tree, pointer_root);
    let mut pointer_draw = DrawList::default();
    paint_tree(&mut pointer_tree, pointer_root, &pointer_theme, &mut pointer_atlas, None, false, &mut pointer_draw);

    assert!(!has_solid_instance_colored(&unfocused_draw, theme.selected), "{label}: an unfocused control must not paint its focus-ring color");
    assert!(has_solid_instance_colored(&focused_draw, theme.selected), "{label}: a keyboard-focused control should swap its border to theme.selected, React's compositor focus stroke");
    assert!(!has_solid_instance_colored(&pointer_draw, theme.selected), "{label}: a POINTER-focused control must not paint the ring — React's selector is `:focus-visible`, not `:focus` (ticket 26/09/17 packet W2k)");
}

#[test]
fn painting_a_focused_button_swaps_its_border_to_the_accent_focus_ring() {
    assert_focus_swaps_border_color(|| button("btn", false), "Button");
}

#[test]
fn painting_a_focused_select_swaps_its_border_to_the_accent_focus_ring() {
    assert_focus_swaps_border_color(|| select("sel", "a"), "Select");
}

#[test]
fn painting_a_focused_toggle_swaps_its_border_to_the_accent_focus_ring() {
    assert_focus_swaps_border_color(|| toggle("tog"), "Toggle");
}

#[test]
fn painting_a_focused_number_stepper_swaps_its_outer_border_to_the_accent_focus_ring() {
    for focused in [false, true] {
        for keyboard in [false, true] {
            let (mut tree, root, theme, mut atlas) = setup(&stepper("ns", 2.0, true));
            if focused {
                if keyboard {
                    focus(&mut tree, root);
                } else {
                    focus_pointer(&mut tree, root);
                }
            }
            let mut draw = DrawList::default();
            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);
            assert_eq!(has_solid_instance_colored(&draw, theme.selected), focused, "React Stepper uses focus-within for both input modalities");
        }
    }
}

#[test]
fn painting_a_hovered_number_stepper_preserves_its_center_background() {
    let (mut tree, root, theme, mut atlas) = setup(&stepper("ns", 2.0, true));
    tree.node_mut(root).unwrap().flags.set(NodeFlags::HOVERED, true);
    tree.node_mut(root).unwrap().flags.set(NodeFlags::HOVER_OWNER, true);
    tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    assert!(!has_solid_instance_colored(&draw, theme.button_hover), "whole-control hover must not tint the center input");
}
//#endregion 🔖️W2WidgetVisuals

//#region 🔖️ClosedStateOutlineLaws
fn resolved_border_instances(draw: &mut DrawList) -> Vec<([f32; 4], [f32; 4])> {
    while !draw.finalize_shared_borders_step().unwrap() {}
    draw.layers.iter().flat_map(|layer| layer.ui_instances.iter().chain(&layer.overlay_ui_instances)).filter(|item| item.rect[2] > 0.0 && item.rect[3] > 0.0).map(|item| (item.rect, item.color)).collect()
}

fn closed_perimeter(rect: [f32; 4], stroke: f32) -> [[f32; 4]; 4] {
    let [x, y, w, h] = rect;
    [[x, y, w, stroke], [x, y + h - stroke, w, stroke], [x, y, stroke, h], [x + w - stroke, y, stroke, h]]
}

#[test]
fn a_hovered_cell_of_a_shared_group_paints_its_whole_perimeter_in_hover_colour_and_the_group_stays_normal() {
    let (_, _, theme, _) = setup(&toggle("t"));
    let stroke = theme.stroke_hairline;
    let (normal, hover) = (Rgba { r: 0.2, g: 0.3, b: 0.4, a: 1.0 }, Rgba { r: 0.9, g: 0.8, b: 0.7, a: 1.0 });
    for hovered in 0..3 {
        let mut draw = DrawList::default();
        for cell in 0..3 {
            let (color, state) = if cell == hovered { (hover, semio_ui_draw_contract::SharedBorderState::Hover) } else { (normal, semio_ui_draw_contract::SharedBorderState::Normal) };
            semio_ui_wgpu_layout::chrome::push_control_border(&mut draw, Rect::new(20.0 * cell as f32, 0.0, 20.0, 20.0), &theme, color, Rgba::TRANSPARENT, state);
        }
        let lines = resolved_border_instances(&mut draw);
        let paint = |color: Rgba| lines.iter().filter(|(_, c)| *c == [color.r, color.g, color.b, color.a]).map(|(rect, _)| *rect).collect::<Vec<_>>();
        let hover_lines = paint(hover);
        for edge in closed_perimeter([20.0 * hovered as f32, 0.0, 20.0, 20.0], stroke) {
            assert!(hover_lines.contains(&edge), "hovered cell {hovered}: {edge:?} in {hover_lines:?}");
        }
        assert_eq!(hover_lines.len(), 4, "no second parallel hover line beside the seams");
        assert!(paint(normal).iter().all(|rect| !hover_lines.contains(rect)));
    }
}

#[test]
fn the_selected_presence_outline_stays_on_the_boxes_own_seams() {
    let (_, _, theme, _) = setup(&toggle("t"));
    let bounds = Rect::new(20.0, 10.0, 30.0, 20.0);
    let presence = UiPresence { selected: true, ..Default::default() };
    let mut draw = DrawList::default();
    presence_overlay(&mut draw, bounds, &theme, &presence, true);
    let lines = resolved_border_instances(&mut draw);
    assert!(!lines.is_empty());
    for (rect, _) in &lines {
        assert!(rect[0] >= bounds.x && rect[1] >= bounds.y && rect[0] + rect[2] <= bounds.x + bounds.w && rect[1] + rect[3] <= bounds.y + bounds.h, "{rect:?} leaves {bounds:?}");
    }
    let active: Vec<_> = lines.iter().map(|(rect, _)| *rect).collect();
    for edge in closed_perimeter([bounds.x, bounds.y, bounds.w, bounds.h], theme.stroke_hairline) {
        assert!(active.contains(&edge), "{edge:?} missing in {active:?}");
    }
}

#[test]
fn an_ancestor_in_the_hover_chain_without_ownership_does_not_paint_hover() {
    let paint_with = |flags: &[NodeFlags]| {
        let (mut tree, root, theme, mut atlas) = setup(&toggle("t"));
        for flag in flags { tree.node_mut(root).unwrap().flags.set(*flag, true); }
        tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
        let mut draw = DrawList::default();
        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);
        resolved_border_instances(&mut draw)
    };
    assert_eq!(paint_with(&[NodeFlags::HOVERED]), paint_with(&[]));
    assert_ne!(paint_with(&[NodeFlags::HOVERED, NodeFlags::HOVER_OWNER]), paint_with(&[]));
}
//#endregion 🔖️ClosedStateOutlineLaws

#[test]
fn a_focused_cell_next_to_a_selected_cell_keeps_its_full_perimeter_in_focus_colour() {
    let (_, _, theme, _) = setup(&toggle("t"));
    let stroke = theme.stroke_hairline;
    let (selected, focus) = (Rgba { r: 0.9, g: 0.1, b: 0.1, a: 1.0 }, Rgba { r: 0.1, g: 0.2, b: 0.9, a: 1.0 });
    for focus_first in [true, false] {
        let mut draw = DrawList::default();
        let cells = [(0.0, if focus_first { (focus, semio_ui_draw_contract::SharedBorderState::Focus) } else { (selected, semio_ui_draw_contract::SharedBorderState::Active) }), (20.0, if focus_first { (selected, semio_ui_draw_contract::SharedBorderState::Active) } else { (focus, semio_ui_draw_contract::SharedBorderState::Focus) })];
        for (x, (color, state)) in cells {
            semio_ui_wgpu_layout::chrome::push_control_border(&mut draw, Rect::new(x, 0.0, 20.0, 20.0), &theme, color, Rgba::TRANSPARENT, state);
        }
        let lines = resolved_border_instances(&mut draw);
        let focus_lines: Vec<_> = lines.iter().filter(|(_, c)| *c == [focus.r, focus.g, focus.b, focus.a]).map(|(rect, _)| *rect).collect();
        let focused_x = if focus_first { 0.0 } else { 20.0 };
        for edge in closed_perimeter([focused_x, 0.0, 20.0, 20.0], stroke) {
            assert!(focus_lines.contains(&edge), "focus first {focus_first}: {edge:?} in {focus_lines:?}");
        }
        assert_eq!(focus_lines.len(), 4);
    }
}
