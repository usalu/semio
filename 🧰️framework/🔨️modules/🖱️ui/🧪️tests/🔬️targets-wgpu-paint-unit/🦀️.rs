
use super::*;
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{UiFieldNode, UiNumberStepperNode, UiSectionNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiTreeItemAction, UiTreeSectionNode};
use crate::wgpu::draw::{KIND_GLYPH, KIND_LOADING_BORDER, KIND_SOLID, KIND_WAITING_BORDER};
use crate::wgpu::flex::LayoutEngine;
use crate::wgpu::tree::EditState;

fn action() -> ActionDescriptor {
    ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None }
}

fn text(value: &str) -> UiNode {
    UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
}

fn stack(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: Some("none".into()), padding: Some("none".into()), id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}

fn loading_button(id: &str) -> UiNode {
    UiNode::Button(UiButtonNode {
        id: Some(id.into()),
        icon_id: IconName::CircleDot,
        label: Label::data(id),
        action: ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None },
        style: None,
        presence: UiPresence::status(UiStatus::Loading),
        menu: None,
    })
}

fn waiting_button(id: &str) -> UiNode {
    UiNode::Button(UiButtonNode {
        id: Some(id.into()),
        icon_id: IconName::CircleDot,
        label: Label::data(id),
        action: ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None },
        style: None,
        presence: UiPresence::status(UiStatus::Waiting),
        menu: None,
    })
}

fn setup(ui: &UiNode) -> (UiTree, NodeId, Theme, FontAtlas) {
    let mut tree = UiTree::new();
    tree.apply_tree(ui);
    let root = tree.root.unwrap();
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let mut engine = LayoutEngine::new();
    engine.compute(&mut tree, root, &mut atlas, &theme, 400.0, 400.0);
    (tree, root, theme, atlas)
}

#[test]
fn painting_a_text_node_emits_glyph_instances() {
    let (mut tree, root, theme, mut atlas) = setup(&text("hi"));
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let total_instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    assert!(total_instances > 0, "text node should emit at least one glyph instance");
}

#[test]
fn painting_a_stack_recurses_into_every_child() {
    let ui = stack(vec![text("a"), UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None }), text("b")]);
    let (mut tree, root, theme, mut atlas) = setup(&ui);
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let total_instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    let total_vectors: usize = draw.layers.iter().map(|layer| layer.vector_vertices.len()).sum();
    assert!(total_instances > 0, "text children should have emitted glyphs");
    assert!(total_vectors > 0, "separator child should have emitted a line");
}

#[test]
fn paint_tree_clears_dirty_paint_but_leaves_layout_dirt_flags_untouched() {
    let (mut tree, root, theme, mut atlas) = setup(&text("hi"));
    assert!(tree.node(root).unwrap().flags.contains(NodeFlags::DIRTY_PAINT));
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let after_first = tree.node(root).unwrap().flags;
    assert!(!after_first.contains(NodeFlags::DIRTY_PAINT));
    assert!(!after_first.contains(NodeFlags::DIRTY_LAYOUT), "paint must not touch DIRTY_LAYOUT, that's flex's job");
    assert!(!after_first.contains(NodeFlags::SUBTREE_DIRTY), "flex::compute already cleared SUBTREE_DIRTY before paint ran");

    // Second call must be a no-operation w.r.t. these flags — repeat of the M3 SUBTREE_DIRTY bug class
    // (calling twice shouldn't set or double-clear something it shouldn't).
    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);
    let after_second = tree.node(root).unwrap().flags;
    assert_eq!(after_first, after_second);
}

#[test]
fn painting_a_loading_button_emits_a_loading_border_instance() {
    let (mut tree, root, theme, mut atlas) = setup(&loading_button("save"));
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let has_loading_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_LOADING_BORDER).abs() < 0.01);
    assert!(has_loading_border, "loading button should emit a KIND_LOADING_BORDER instance");
}

#[test]
fn painting_a_waiting_button_emits_a_waiting_border_instance() {
    let (mut tree, root, theme, mut atlas) = setup(&waiting_button("save"));
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let has_waiting_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_WAITING_BORDER).abs() < 0.01);
    assert!(has_waiting_border, "waiting button should emit a KIND_WAITING_BORDER instance");
}

// 🚫️ `painting_a_loading_and_waiting_button_prefers_the_loading_border` deleted: `status` is now a
// single `UiStatus` enum, so "loading and waiting both set" is unrepresentable — which is the point.

//#region 🔖️FidelityFixes
// 🩹️ One test per fidelity gap this pass closed (see `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w1c-paint-parity.md`),
// additive to the pre-existing tests above.

fn button(id: &str, disabled: bool) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: IconName::CircleDot, label: Label::data(id), action: action(), style: None, presence: UiPresence::disabled_if(disabled), menu: None })
}

#[test]
fn painting_a_disabled_button_dims_its_border_alpha() {
    let (mut tree, root, theme, mut atlas) = setup(&button("btn", true));
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let dimmed = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.color[3] - theme.border_normal.a * 0.5).abs() < 0.01);
    assert!(dimmed, "a disabled button should paint its border at half alpha");
}

fn loading_section(id: &str) -> UiNode {
    UiNode::Section(UiSectionNode { id: id.into(), label: Some(Label::data("Sec")), default_open: Some(true), presence: UiPresence::status(UiStatus::Loading), children: vec![text("child")], menu: None })
}

#[test]
fn painting_a_loading_section_emits_a_loading_border_instance() {
    let (mut tree, root, theme, mut atlas) = setup(&loading_section("sec"));
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let has_loading_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_LOADING_BORDER).abs() < 0.01);
    assert!(has_loading_border, "a loading section should emit a KIND_LOADING_BORDER instance");
}

fn waiting_section(id: &str) -> UiNode {
    UiNode::Section(UiSectionNode { id: id.into(), label: Some(Label::data("Sec")), default_open: Some(true), presence: UiPresence::status(UiStatus::Waiting), children: vec![text("child")], menu: None })
}

#[test]
fn painting_a_waiting_section_emits_a_waiting_border_instance() {
    let (mut tree, root, theme, mut atlas) = setup(&waiting_section("sec"));
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let has_waiting_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_WAITING_BORDER).abs() < 0.01);
    assert!(has_waiting_border, "a waiting section should emit a KIND_WAITING_BORDER instance");
}

fn loading_stack(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: Some("none".into()),
        padding: Some("none".into()),
        id: None,
        presence: UiPresence::status(UiStatus::Loading),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children,
        menu: None,
    })
}

#[test]
fn painting_a_loading_stack_emits_a_loading_border_instance() {
    let (mut tree, root, theme, mut atlas) = setup(&loading_stack(vec![text("a")]));
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let has_loading_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_LOADING_BORDER).abs() < 0.01);
    assert!(has_loading_border, "a loading stack should emit a KIND_LOADING_BORDER instance");
}

fn waiting_stack(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: Some("none".into()),
        padding: Some("none".into()),
        id: None,
        presence: UiPresence::status(UiStatus::Waiting),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children,
        menu: None,
    })
}

#[test]
fn painting_a_waiting_stack_emits_a_waiting_border_instance() {
    let (mut tree, root, theme, mut atlas) = setup(&waiting_stack(vec![text("a")]));
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let has_waiting_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_WAITING_BORDER).abs() < 0.01);
    assert!(has_waiting_border, "a waiting stack should emit a KIND_WAITING_BORDER instance");
}

fn loading_tree() -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", Label::data("Item"))] }],
        presence: UiPresence::status(UiStatus::Loading),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
}

fn waiting_tree() -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", Label::data("Item"))] }],
        presence: UiPresence::status(UiStatus::Waiting),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
}

#[test]
fn painting_a_loading_tree_emits_a_loading_border_instance() {
    let (mut tree, root, theme, mut atlas) = setup(&loading_tree());
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let has_loading_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_LOADING_BORDER).abs() < 0.01);
    assert!(has_loading_border, "a loading tree should emit a KIND_LOADING_BORDER instance");
}

#[test]
fn painting_a_waiting_tree_emits_a_waiting_border_instance() {
    let (mut tree, root, theme, mut atlas) = setup(&waiting_tree());
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let has_waiting_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_WAITING_BORDER).abs() < 0.01);
    assert!(has_waiting_border, "a waiting tree should emit a KIND_WAITING_BORDER instance");
}

fn stepper(id: &str, value: f64, uniform: bool) -> UiNode {
    UiNode::NumberStepper(UiNumberStepperNode { id: id.into(), value, step: 1.0, uniform, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None })
}

#[test]
fn painting_a_uniform_number_stepper_nests_a_border_around_its_center_value() {
    let (mut tree, root, theme, mut atlas) = setup(&stepper("ns", 2.0, true));
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let total: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    // Outer control border (bg + 4 edges = 5) + 2 divider lines + nested center-value border
    // (bg + 4 edges = 5) + minus/"2.000"/plus glyphs (1 + 5 + 1 = 7) = 19 — the exact instance
    // count `golden_number_stepper_known_gap`'s doc comment measured `widgets::render_number_stepper`
    // emitting (vs this region's pre-fix 14), now matched by porting the nested border.
    assert_eq!(total, 19, "uniform NumberStepper should now nest a border around its center value, matching widgets' 19-instance output");
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
    UiNode::Slider(UiSliderNode { id: id.into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, unit: unit.map(String::from), on_change: action(), presence: UiPresence::default(), menu: None })
}

#[test]
fn painting_a_slider_with_a_unit_emits_extra_glyphs_for_the_readout() {
    let (mut plain_tree, plain_root, theme, mut plain_atlas) = setup(&slider("sl", None));
    let mut plain_draw = DrawList::default();
    paint_tree(&mut plain_tree, plain_root, &theme, &mut plain_atlas, None, false, &mut plain_draw);

    let (mut unit_tree, unit_root, theme2, mut unit_atlas) = setup(&slider("sl", Some("mm")));
    let mut unit_draw = DrawList::default();
    paint_tree(&mut unit_tree, unit_root, &theme2, &mut unit_atlas, None, false, &mut unit_draw);

    let plain_total: usize = plain_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    let unit_total: usize = unit_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    assert!(unit_total > plain_total, "a slider with a unit should paint extra glyphs for its value+unit readout");
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
    item.actions = Some(vec![UiTreeItemAction { icon_id: IconName::Sparkles, label: None, action: action(), placement: None }]);
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
}

fn tree_with_bare_item() -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", Label::data("Item One"))] }],
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

//#region 🔖️W2InteractivityFixes
// 🔽️🎴️🌳️ Tests for `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`'s W2 pass: Select popup painting
// (`paint_select` + `sync_select_popup_rows`), Stack `activate`/`selected`/`drop_action`
// (`paint_stack_frame` + `sync_interactive_state`'s `DROP_TARGET` sync), and Tree row real layout
// + `DRAG_SOURCE` sync (`sync_tree_row_layout`/`sync_tree_item_layout`).

fn select(id: &str, value: &str) -> UiNode {
    UiNode::Select(UiSelectNode {
        id: id.into(),
        value: value.into(),
        items: vec![UiSelectItem { value: "a".into(), label: Label::data("Alpha") }, UiSelectItem { value: "b".into(), label: Label::data("Beta") }],
        placeholder: None,
        on_change: action(),
        presence: UiPresence::default(),
        menu: None,
    })
}

#[test]
fn painting_an_open_select_popup_emits_more_instances_than_a_closed_one_and_highlights_the_value() {
    let (mut tree, root, theme, mut atlas) = setup(&select("sel", "b"));
    let mut closed_draw = DrawList::default();
    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut closed_draw);
    let closed_total: usize = closed_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

    tree.node_mut(root).unwrap().state.open = true;
    tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
    let mut open_draw = DrawList::default();
    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut open_draw);
    let open_total: usize = open_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

    assert!(open_total > closed_total, "an open Select should paint its popup rows in addition to the closed trigger");
    // "Beta" (value "b") is the current value — its row should paint a `row_hover`-colored
    // highlight rect (a KIND_ROUNDED instance in exactly `theme.row_hover`).
    let has_selected_highlight = open_draw
        .layers
        .iter()
        .flat_map(|layer| layer.ui_instances.iter())
        .any(|instance| (instance.color[0] - theme.row_hover.r).abs() < 0.001 && (instance.color[1] - theme.row_hover.g).abs() < 0.001 && (instance.color[2] - theme.row_hover.b).abs() < 0.001);
    assert!(has_selected_highlight, "the popup row matching the Select's current value should paint a row_hover highlight");
}

#[test]
fn opening_a_selects_popup_gives_its_synthesized_item_rows_real_hit_testable_layout() {
    let (mut tree, root, theme, mut atlas) = setup(&select("sel", "a"));
    tree.node_mut(root).unwrap().state.open = true;
    tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let row_a = find_child_by_key(&tree, root, &NodeKey::Explicit("a".into())).expect("reconcile should have synthesized a retained row for item \"a\"");
    let row_b = find_child_by_key(&tree, root, &NodeKey::Explicit("b".into())).expect("reconcile should have synthesized a retained row for item \"b\"");
    let bucket_a = &tree.node(row_a).unwrap().layout;
    let bucket_b = &tree.node(row_b).unwrap().layout;
    assert!(bucket_a.width > 0.0 && bucket_a.height > 0.0, "an open Select's row should get real (non-zero) layout so events::hit_test can find it");
    assert!(bucket_b.y > bucket_a.y, "row \"b\" should be laid out below row \"a\"");
}

fn drop_stack(drop_action: Option<ActionDescriptor>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some("dz".into()), presence: UiPresence::default(), activate: None, drop_action, drop_overlay: None, children: vec![text("child")], menu: None })
}

#[test]
fn a_stacks_drop_target_flag_tracks_its_drop_action() {
    let (mut tree, root, theme, mut atlas) = setup(&drop_stack(Some(action())));
    let mut draw = DrawList::default();
    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);
    assert!(tree.node(root).unwrap().flags.contains(NodeFlags::DROP_TARGET), "a Stack with a drop_action should be flagged NodeFlags::DROP_TARGET");

    let (mut plain_tree, plain_root, plain_theme, mut plain_atlas) = setup(&drop_stack(None));
    let mut plain_draw = DrawList::default();
    paint_tree(&mut plain_tree, plain_root, &plain_theme, &mut plain_atlas, None, false, &mut plain_draw);
    assert!(!plain_tree.node(plain_root).unwrap().flags.contains(NodeFlags::DROP_TARGET), "a Stack without a drop_action must not be flagged DROP_TARGET");
}

fn activatable_stack(selected: bool) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: None,
        padding: None,
        id: Some("card".into()),
        presence: UiPresence::selected(selected),
        activate: Some(action()),
        drop_action: None,
        drop_overlay: None,
        children: vec![text("child")],
        menu: None,
    })
}

#[test]
fn an_activatable_stack_paints_a_frame_and_a_selected_one_paints_an_extra_ring() {
    let (mut bare_tree, bare_root, theme, mut bare_atlas) = setup(&stack(vec![text("child")]));
    let mut bare_draw = DrawList::default();
    paint_tree(&mut bare_tree, bare_root, &theme, &mut bare_atlas, None, false, &mut bare_draw);
    let bare_total: usize = bare_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

    let (mut card_tree, card_root, theme2, mut card_atlas) = setup(&activatable_stack(false));
    let mut card_draw = DrawList::default();
    paint_tree(&mut card_tree, card_root, &theme2, &mut card_atlas, None, false, &mut card_draw);
    let card_total: usize = card_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    assert!(card_total > bare_total, "an activatable Stack should paint a bg+border frame a bare Stack doesn't");

    let (mut selected_tree, selected_root, theme3, mut selected_atlas) = setup(&activatable_stack(true));
    let mut selected_draw = DrawList::default();
    paint_tree(&mut selected_tree, selected_root, &theme3, &mut selected_atlas, None, false, &mut selected_draw);
    let selected_total: usize = selected_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    assert!(selected_total > card_total, "a selected activatable Stack should paint an extra ring border beyond the plain activate frame");
}

fn tree_with_draggable_item() -> UiNode {
    let mut item = UiTreeItemNode::base("i1", Label::data("Item One"));
    item.draggable = Some(true);
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
}

#[test]
fn a_trees_draggable_item_gets_real_row_layout_and_the_drag_source_flag() {
    let (mut tree, root, theme, mut atlas) = setup(&tree_with_draggable_item());
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    let section = find_child_by_key(&tree, root, &NodeKey::Explicit("s1".into())).expect("reconcile should have synthesized a retained row for section \"s1\"");
    let row = find_child_by_key(&tree, section, &NodeKey::Explicit("i1".into())).expect("reconcile should have synthesized a retained row for item \"i1\"");
    let bucket = &tree.node(row).unwrap().layout;
    assert!(bucket.width > 0.0 && bucket.height > 0.0, "a Tree row should get real (non-zero) layout so events::hit_test can find it");
    assert!(tree.node(row).unwrap().flags.contains(NodeFlags::DRAG_SOURCE), "a draggable Tree item's row should be flagged NodeFlags::DRAG_SOURCE");
}
//#endregion 🔖️W2InteractivityFixes

//#region 🔖️W2WidgetVisuals
// 🖱️✍️🎯️ Tests for `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`'s W2 widget-visuals pass: a
// focused `Input`'s caret/selection-highlight (`paint_input`, sourced from `tree::EditState`),
// and the `formControlFocusBorderClass`-matching focus ring ported onto every remaining
// focusable control kind (`Button`/`Select`/`Toggle`/`NumberStepper`/`IconSelect`, plus a
// `NumberStepper` hover tint) that only `paint_input` had before this pass.
fn input(id: &str, value: &str) -> UiNode {
    UiNode::Input(UiInputNode { id: id.into(), input_kind: "text".into(), value: value.into(), placeholder: None, commit: None, min: None, max: None, step: None, accept: None, on_change: action(), presence: UiPresence::default(), menu: None })
}

fn focus(tree: &mut UiTree, id: NodeId) {
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

fn toggle(id: &str) -> UiNode {
    UiNode::Toggle(UiToggleNode { id: id.into(), icon_id: IconName::CircleDot, text: Some(Label::data("Toggle")), on_change: action(), presence: UiPresence::default(), menu: None })
}

fn icon_select(id: &str) -> UiNode {
    UiNode::IconSelect(UiIconSelectNode { id: id.into(), value: "star".into(), uniform: true, classifier_kind: "generic".into(), on_change: action(), presence: UiPresence::default(), menu: None })
}

/// 🎯️ Shared assertion for the border-swap-on-focus fix: an otherwise-identical pair of trees,
/// one with `NodeFlags::FOCUSED` set on the root, should differ in at least one border instance's
/// color (`theme.border_emphasized` replacing `theme.border_normal`) — mirrors
/// `formControlFocusBorderClass`'s `focus-visible:border-accent` (`ui/js/react/index.tsx`).
fn assert_focus_swaps_border_color(make: impl Fn() -> UiNode, label: &str) {
    let (mut unfocused_tree, unfocused_root, theme, mut unfocused_atlas) = setup(&make());
    let mut unfocused_draw = DrawList::default();
    paint_tree(&mut unfocused_tree, unfocused_root, &theme, &mut unfocused_atlas, None, false, &mut unfocused_draw);

    let (mut focused_tree, focused_root, focused_theme, mut focused_atlas) = setup(&make());
    focus(&mut focused_tree, focused_root);
    let mut focused_draw = DrawList::default();
    paint_tree(&mut focused_tree, focused_root, &focused_theme, &mut focused_atlas, None, false, &mut focused_draw);

    assert!(!has_solid_instance_colored(&unfocused_draw, theme.border_emphasized), "{label}: an unfocused control must not paint its border_emphasized color");
    assert!(has_solid_instance_colored(&focused_draw, theme.border_emphasized), "{label}: a focused control should swap its border to theme.border_emphasized");
}

#[test]
fn painting_a_focused_button_swaps_its_border_to_border_emphasized() {
    assert_focus_swaps_border_color(|| button("btn", false), "Button");
}

#[test]
fn painting_a_focused_select_swaps_its_border_to_border_emphasized() {
    assert_focus_swaps_border_color(|| select("sel", "a"), "Select");
}

#[test]
fn painting_a_focused_toggle_swaps_its_border_to_border_emphasized() {
    assert_focus_swaps_border_color(|| toggle("tog"), "Toggle");
}

#[test]
fn painting_a_focused_number_stepper_swaps_its_outer_border_to_border_emphasized() {
    assert_focus_swaps_border_color(|| stepper("ns", 2.0, true), "NumberStepper");
}

#[test]
fn painting_a_focused_icon_select_swaps_its_border_to_border_emphasized() {
    assert_focus_swaps_border_color(|| icon_select("ic"), "IconSelect");
}

#[test]
fn painting_a_hovered_number_stepper_tints_its_outer_background() {
    let (mut tree, root, theme, mut atlas) = setup(&stepper("ns", 2.0, true));
    tree.node_mut(root).unwrap().flags.set(NodeFlags::HOVERED, true);
    tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
    let mut draw = DrawList::default();

    paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

    assert!(has_solid_instance_colored(&draw, theme.button_hover), "a hovered NumberStepper should tint its shared minus/plus background to theme.button_hover");
}
//#endregion 🔖️W2WidgetVisuals

//#region 🧵️RetainedPaintLaws
fn retained_glyph_count(draw: &DrawList) -> usize {
    draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).count()
}

#[test]
fn retained_text_paint_emits_at_most_one_glyph_per_grant() {
    let (tree, root, theme, mut atlas) = setup(&text("ab"));
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
    let before = retained_glyph_count(&draw);
    assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
    assert_eq!(retained_glyph_count(&draw), before + 1);
    assert_eq!(cursor.glyph.byte(), 1);
    assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
    assert_eq!(retained_glyph_count(&draw), before + 2);
    assert_eq!(cursor.glyph.byte(), 2);
}

#[test]
fn retained_multi_megabyte_input_advances_one_scalar_per_grant() {
    let value = "x".repeat(2 * 1_024 * 1_024);
    let (tree, root, theme, mut atlas) = setup(&input("huge-input", &value));
    let identity = match tree.node(root).map(|node| &node.spec.0) {
        Some(UiNode::Input(input)) => input.value.as_ptr(),
        _ => panic!("retained input owner"),
    };
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
    assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
    let before = retained_glyph_count(&draw);
    assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
    assert_eq!(retained_glyph_count(&draw), before + 1);
    assert_eq!(cursor.glyph.byte(), 1);
    assert_eq!(
        tree.node(root).and_then(|node| match &node.spec.0 {
            UiNode::Input(input) => Some(input.value.as_ptr()),
            _ => None,
        }),
        Some(identity)
    );
}

#[test]
fn retained_select_max_plus_one_refuses_before_output_without_moving_tree_owner() {
    let mut authored = select("select-max-plus-one", "0");
    let UiNode::Select(select) = &mut authored else { panic!("select fixture") };
    select.items = (0..=RETAINED_NODE_COLLECTION_ITEMS).map(|index| UiSelectItem { value: index.to_string(), label: Label::data(index.to_string()) }).collect();
    let (tree, root, theme, mut atlas) = setup(&authored);
    let identity = match tree.node(root).map(|node| &node.spec.0) {
        Some(UiNode::Select(select)) => select.items.as_ptr(),
        _ => panic!("retained select owner"),
    };
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
    assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Fault);
    assert_eq!(draw.layers.iter().map(|layer| layer.ui_instances.len()).sum::<usize>(), 0);
    assert_eq!(
        tree.node(root).and_then(|node| match &node.spec.0 {
            UiNode::Select(select) => Some(select.items.as_ptr()),
            _ => None,
        }),
        Some(identity)
    );
}

#[test]
fn retained_select_sync_writes_at_most_one_row_per_grant() {
    let (mut tree, root, theme, _) = setup(&select("retained-sync-select", "a"));
    let Some(root_node) = tree.node_mut(root) else { panic!("retained select root") };
    root_node.state.open = true;
    let children: Vec<NodeId> = tree.children(root).collect();
    let mut cursor = RetainedInteractiveSyncCursor::default();
    let mut complete = false;
    for _ in 0..64 {
        let before: Vec<(f32, f32, f32, f32)> = children.iter().filter_map(|child| tree.node(*child).map(|node| (node.layout.x, node.layout.y, node.layout.width, node.layout.height))).collect();
        let step = sync_interactive_state_node_step(&mut tree, root, &theme, &mut cursor);
        let changed = children.iter().zip(before.iter()).filter(|(child, before)| tree.node(**child).is_some_and(|node| (node.layout.x, node.layout.y, node.layout.width, node.layout.height) != **before)).count();
        assert!(changed <= 1);
        if step == RetainedInteractiveSyncStep::Complete {
            complete = true;
            break;
        }
        assert_ne!(step, RetainedInteractiveSyncStep::Fault);
    }
    assert!(complete);
    assert!(cursor.terminal_is_empty());
}

#[test]
fn retained_select_sync_max_plus_one_fault_closes_exact_cursor_owner() {
    let mut authored = select("retained-sync-max-plus-one", "0");
    let UiNode::Select(select) = &mut authored else { panic!("retained select fixture") };
    select.items = (0..=RETAINED_SYNC_COLLECTION_ITEMS).map(|index| UiSelectItem { value: index.to_string(), label: Label::data(index.to_string()) }).collect();
    let (mut tree, root, theme, _) = setup(&authored);
    let Some(root_node) = tree.node_mut(root) else { panic!("retained select root") };
    root_node.state.open = true;
    let identity = match tree.node(root).map(|node| &node.spec.0) {
        Some(UiNode::Select(select)) => select.items.as_ptr(),
        _ => panic!("retained select owner"),
    };
    let mut cursor = RetainedInteractiveSyncCursor::default();
    assert_eq!(sync_interactive_state_node_step(&mut tree, root, &theme, &mut cursor), RetainedInteractiveSyncStep::Pending);
    assert_eq!(sync_interactive_state_node_step(&mut tree, root, &theme, &mut cursor), RetainedInteractiveSyncStep::Fault);
    assert!(!cursor.close_step());
    assert!(cursor.close_step());
    assert!(cursor.terminal_is_empty());
    assert_eq!(
        tree.node(root).and_then(|node| match &node.spec.0 {
            UiNode::Select(select) => Some(select.items.as_ptr()),
            _ => None,
        }),
        Some(identity)
    );
}

#[test]
fn retained_tree_sync_abandonment_releases_one_record_or_depth_owner_per_grant() {
    let mut nested = UiTreeItemNode::base("nested", Label::data("Nested"));
    nested.default_open = Some(true);
    nested.items = Some(vec![UiTreeItemNode::base("leaf", Label::data("Leaf"))]);
    let authored = UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode { id: "section".into(), label: Some(Label::data("Section")), default_open: Some(true), presence: UiPresence::default(), items: vec![nested] }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    });
    let (mut tree, root, theme, _) = setup(&authored);
    let mut cursor = RetainedInteractiveSyncCursor::default();
    for _ in 0..32 {
        assert_ne!(sync_interactive_state_node_step(&mut tree, root, &theme, &mut cursor), RetainedInteractiveSyncStep::Fault);
        if cursor.tree_record_len >= 3 {
            break;
        }
    }
    assert!(cursor.tree_record_len >= 3);
    while !cursor.terminal_is_empty() {
        let records = cursor.tree_record_len;
        let frames = cursor.tree_frames.iter().flatten().count();
        assert!(!cursor.close_step());
        let closed = records.saturating_sub(cursor.tree_record_len) + frames.saturating_sub(cursor.tree_frames.iter().flatten().count());
        assert!(closed <= 1);
    }
    assert!(cursor.close_step());
}

#[test]
fn retained_text_multi_megabyte_max_plus_one_preserves_tree_owner_identity() {
    let authored = text(&"x".repeat(RETAINED_NODE_TEXT_MAX_BYTES + 1));
    let mut tree = UiTree::new();
    tree.apply_tree(&authored);
    let Some(root) = tree.root else { panic!("retained text root") };
    let identity = match tree.node(root).map(|node| &node.spec.0) {
        Some(UiNode::Text(text)) => text.value.as_str().as_ptr(),
        _ => panic!("retained text owner"),
    };
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
    assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Fault);
    let after = match tree.node(root).map(|node| &node.spec.0) {
        Some(UiNode::Text(text)) => text.value.as_str().as_ptr(),
        _ => panic!("retained text owner"),
    };
    assert_eq!(after, identity);
    assert_eq!(retained_glyph_count(&draw), 0);
    assert!(!cursor.terminal_is_empty());
}

#[test]
fn retained_text_cancel_close_preserves_exact_terminal_witness() {
    let (tree, root, theme, mut atlas) = setup(&text("cancel"));
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
    assert!(!cursor.terminal_is_empty());
    assert!(!cursor.close_step());
    assert!(cursor.terminal_is_empty());
    assert!(cursor.close_step());
}
//#endregion 🧵️RetainedPaintLaws
