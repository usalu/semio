// ✍️ Input caret and focus ring paint laws: the input fixtures, the caret, selection and live edit buffer paint, the retained caret cadence and the accent focus ring on every control.

//#region 🔖️W2WidgetVisuals
// 🖱️✍️🎯️ Tests for `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`'s W2 widget-visuals pass: a
// focused `Input`'s caret/selection-highlight (`paint_input`, sourced from `tree::EditState`),
// and the `formControlFocusBorderClass`-matching focus ring ported onto every remaining
// focusable control kind (`Button`/`Select`/`Toggle`/`NumberStepper`/`IconSelect`, plus a
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
    assert!(has_solid_instance_colored(&visible, theme.accent), "the accepted visible phase paints one caret primitive");

    tree.node_mut(root).expect("retained input").state.caret_visible = false;
    let hidden = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    assert!(!has_solid_instance_colored(&hidden, theme.accent), "the hidden phase keeps the edit buffer while omitting its caret primitive");
}

fn toggle(id: &str) -> UiNode {
    UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, id: id.into(), icon_id: IconName::CircleDot, text: Some(Label::data("Toggle")), on_change: action(), presence: UiPresence::default(), menu: None })
}

fn icon_select(id: &str) -> UiNode {
    UiNode::IconSelect(UiIconSelectNode { id: id.into(), value: "star".into(), uniform: true, classifier_kind: "generic".into(), on_change: action(), presence: UiPresence::default(), menu: None })
}

/// 🎯️ Shared assertion for the border-swap-on-focus fix: an otherwise-identical pair of trees,
/// one with `NodeFlags::FOCUSED` set on the root, should differ in at least one border instance's
/// color — `theme.accent` replacing `theme.border_normal`, the literal color React's shared
/// `formControlFocusBorderClass` swaps in (`focus-visible:border-accent`,
/// `🔨️modules/📝️form-control-presentation/🟦️.ts:14`); it used to be the unrelated
/// `border_emphasized` token (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1o).
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

    assert!(!has_solid_instance_colored(&unfocused_draw, theme.accent), "{label}: an unfocused control must not paint its focus-ring color");
    assert!(has_solid_instance_colored(&focused_draw, theme.accent), "{label}: a keyboard-focused control should swap its border to theme.accent, React's own focus-visible border");
    assert!(!has_solid_instance_colored(&pointer_draw, theme.accent), "{label}: a POINTER-focused control must not paint the ring — React's selector is `:focus-visible`, not `:focus` (ticket 26/09/17 packet W2k)");
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
            assert_eq!(has_solid_instance_colored(&draw, theme.accent), focused, "React Stepper uses focus-within for both input modalities");
        }
    }
}

#[test]
fn painting_a_focused_icon_select_swaps_its_border_to_the_accent_focus_ring() {
    assert_focus_swaps_border_color(|| icon_select("ic"), "IconSelect");
}

#[test]
fn painting_a_hovered_number_stepper_preserves_its_center_background() {
    let (mut tree, root, theme, mut atlas) = setup(&stepper("ns", 2.0, true));
    tree.node_mut(root).unwrap().flags.set(NodeFlags::HOVERED, true);
    tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    assert!(!has_solid_instance_colored(&draw, theme.button_hover), "whole-control hover must not tint the center input");
}
//#endregion 🔖️W2WidgetVisuals
