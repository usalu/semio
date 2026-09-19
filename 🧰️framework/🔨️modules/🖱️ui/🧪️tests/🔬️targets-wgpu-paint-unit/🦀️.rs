
use super::*;
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{UiFieldNode, UiNumberStepperNode, UiSectionNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiTreeItemAction, UiTreeSectionNode};
use crate::wgpu::draw::{KIND_GLYPH, KIND_LOADING_BORDER, KIND_SOLID, KIND_WAITING_BORDER};
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
    let atlas = FontAtlas::builtin();
    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut tree, root, theme, 400.0, 400.0), "layout pass");
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
        sections: vec![UiTreeSectionNode { window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", Label::data("Item"))] }],
        presence: UiPresence::status(UiStatus::Loading),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
}

fn waiting_tree() -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode { window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", Label::data("Item"))] }],
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
        sections: vec![UiTreeSectionNode { window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
}

fn tree_with_bare_item() -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode { window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", Label::data("Item One"))] }],
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

/// 🔽️ A CLOSED `Select` materializes no option rows (`reconcile::children_of` gates on
/// `WidgetState::open`), so the popup's rows only exist after re-reconciling with the bit set.
#[test]
fn opening_a_selects_popup_gives_its_synthesized_item_rows_real_hit_testable_layout() {
    let fixture = select("sel", "a");
    let (mut tree, root, theme, mut atlas) = setup(&fixture);
    tree.node_mut(root).unwrap().state.open = true;
    tree.apply_tree(&fixture);
    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut tree, root, theme, 400.0, 400.0), "layout pass");
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

/// 🔽️ W15a item 7. `reconcile::children_of`/`apply_tree` — the only thing that ever synthesized an
/// open `Select`'s option rows — are `cfg(test, testkit)`, so PRODUCTION materialized none: with
/// `paint_select` itself `cfg(test)` and the retained `UiNode::Select` arm painting only the
/// trigger, an open popup on the live renderer painted nothing, registered no row hit target and
/// projected no accessibility children. This law drives the PRODUCTION stepper
/// (`sync_interactive_state_node_step`, no `apply_tree` anywhere) and pins: the rows appear, they
/// are keyed by item value, they are laid out at the popup geometry paint resolves, re-running the
/// pass does not mint a second copy, closing the popup unmounts them, and the ledger is bounded.
#[test]
fn the_production_sync_pass_materialises_an_open_selects_option_rows_and_unmounts_them_on_close() {
    let fixture = select("sel", "a");
    let (mut tree, root, theme, _atlas) = setup(&fixture);
    assert_eq!(tree.composite_row_count(), 0, "a closed Select owns no synthesized rows");

    let drive = |tree: &mut UiTree, theme: &Theme| {
        let mut cursor = RetainedInteractiveSyncCursor::default();
        for _ in 0..4_096 {
            match sync_interactive_state_node_step(tree, root, theme, &mut cursor) {
                RetainedInteractiveSyncStep::Pending => {}
                RetainedInteractiveSyncStep::Complete => return,
                RetainedInteractiveSyncStep::Fault => panic!("the production sync pass faulted at line {}", cursor.fault_line),
            }
        }
        panic!("the production sync pass never completed");
    };

    drive(&mut tree, &theme);
    assert_eq!(tree.composite_row_count(), 0, "a CLOSED popup mounts nothing, exactly as React mounts no `SelectContent`");

    tree.node_mut(root).unwrap().state.open = true;
    drive(&mut tree, &theme);
    let row_a = find_child_by_key(&tree, root, &NodeKey::Explicit("a".into())).expect("the production pass must synthesize a row for item \"a\"");
    let row_b = find_child_by_key(&tree, root, &NodeKey::Explicit("b".into())).expect("the production pass must synthesize a row for item \"b\"");
    assert_eq!(tree.composite_row_count(), 2, "one ledgered row per item, no more");
    let (bucket_a, bucket_b) = (tree.node(row_a).unwrap().layout.clone(), tree.node(row_b).unwrap().layout.clone());
    assert!(bucket_a.width > 0.0 && bucket_a.height > 0.0, "a synthesized row gets real layout, so `events::hit_test` and the hit registry can find it");
    assert!(bucket_b.y > bucket_a.y, "row \"b\" is laid out below row \"a\", at the popup geometry paint resolves");
    assert!(matches!(tree.node(row_a).map(|node| &node.spec.0), Some(UiNode::Button(_))), "a row is a real retained Button — which is what gives it a paint arm AND a hit registration");

    drive(&mut tree, &theme);
    assert_eq!(tree.composite_row_count(), 2, "a second pass over the SAME open popup re-finds its rows by key instead of minting a second copy");

    tree.node_mut(root).unwrap().state.open = false;
    drive(&mut tree, &theme);
    assert_eq!(tree.composite_row_count(), 0, "a closed popup unmounts its rows");
    assert!(find_child_by_key(&tree, root, &NodeKey::Explicit("a".into())).is_none(), "and leaves no arena node behind");
    assert_eq!(tree.children(root).count(), 0, "the owner's sibling chain is left exactly as the rows found it");
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
        sections: vec![UiTreeSectionNode { window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }],
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
    UiNode::Input(UiInputNode { id: id.into(), input_kind: "text".into(), value: value.into(), placeholder: None, commit: None, min: None, max: None, step: None, accept: None, on_change: action(), on_submit: None, on_abort: None, on_repeat_last: None, presence: UiPresence::default(), menu: None })
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

fn toggle(id: &str) -> UiNode {
    UiNode::Toggle(UiToggleNode { id: id.into(), icon_id: IconName::CircleDot, text: Some(Label::data("Toggle")), on_change: action(), presence: UiPresence::default(), menu: None })
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
    assert!(
        !has_solid_instance_colored(&pointer_draw, theme.accent),
        "{label}: a POINTER-focused control must not paint the ring — React's selector is `:focus-visible`, not `:focus` (ticket 26/09/17 packet W2k)"
    );
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
    assert_focus_swaps_border_color(|| stepper("ns", 2.0, true), "NumberStepper");
}

#[test]
fn painting_a_focused_icon_select_swaps_its_border_to_the_accent_focus_ring() {
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

/// 🅰️ W15a item 3. React's `TextView` swaps `text-sm` for `font-semibold` when `emphasize` is set
/// (`🗣️Interpreter/🟦️.tsx:1168`), so an emphasized run is a real WEIGHT change, not only a size
/// bump. `draw_text_weighted`/`TextWeight::Semibold` were built and unit-tested by W2k with zero
/// callers: this pins the production retained painter to the synthetic double strike, and pins the
/// advance (and with it every wrap point) to be unchanged by it.
#[test]
fn an_emphasized_retained_text_node_strikes_every_glyph_twice_without_moving_the_pen() {
    let emphasized = UiNode::Text(UiTextNode { value: Label::data("ab"), emphasize: Some(true), data_attributes: None, presence: UiPresence::default(), menu: None });
    let strike_run = |node: &UiNode| {
        let (tree, root, theme, mut atlas) = setup(node);
        let mut draw = DrawList::default();
        let mut cursor = RetainedNodePaintCursor::default();
        let mut per_step = Vec::new();
        for _ in 0..4 {
            let before = retained_glyph_count(&draw);
            if paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor) == RetainedNodePaintStep::Fault {
                panic!("retained text paint must not fault");
            }
            per_step.push(retained_glyph_count(&draw) - before);
        }
        let xs: Vec<f32> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).map(|instance| instance.rect[0]).collect();
        (per_step, xs)
    };
    let (regular_steps, regular_xs) = strike_run(&text("ab"));
    let (semibold_steps, semibold_xs) = strike_run(&emphasized);
    assert!(regular_steps.iter().all(|count| *count <= 1), "a regular run still costs at most one strike per grant: {regular_steps:?}");
    assert!(semibold_steps.contains(&2), "an emphasized run must emit BOTH strikes of a glyph inside ONE grant, never split across two: {semibold_steps:?}");
    assert_eq!(semibold_xs.len(), regular_xs.len() * 2, "every emphasized glyph is struck exactly twice");
    let offset = crate::wgpu::text::faux_bold_offset(Theme::default().font_size_emphasized);
    assert!(offset > 0.0);
    assert!(semibold_xs.windows(2).any(|pair| (pair[1] - pair[0] - offset).abs() < 0.001), "the second strike sits exactly `faux_bold_offset` along x");
    assert!(regular_xs.first().is_some_and(|x| semibold_xs.first().is_some_and(|semibold| (semibold - x).abs() < 0.001)), "the FIRST strike lands on the same pen position, so the advance — and every wrap point priced from it — is untouched");
}

/// 🔽️ See `opening_a_selects_popup_...`: the rows exist only once the tree is reconciled OPEN.
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
    let fixture = select("retained-sync-select", "a");
    let (mut tree, root, theme, _) = setup(&fixture);
    let Some(root_node) = tree.node_mut(root) else { panic!("retained select root") };
    root_node.state.open = true;
    tree.apply_tree(&fixture);
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

/// 🌳️ LAW: a declared section/row whose retained child was never mounted is walked past, not
/// faulted. A document-mounted `Tree` carries only the rows its producer published as child records,
/// and one missing row used to answer `Fault` — which the shell reports as `retained document
/// ingress reached its terminal fault` and paints as an empty body, killing the whole panel
/// (`framework.panel.toolRun` on the live generation3d wgpu playground).
#[test]
fn retained_tree_sync_skips_a_declared_row_the_document_never_mounted() {
    let section = |id: &str, item: &str| UiTreeSectionNode {
        window: None,
        id: id.into(),
        label: Some(Label::data(id)),
        default_open: Some(true),
        presence: UiPresence::default(),
        items: vec![UiTreeItemNode::base(item, Label::data(item))],
    };
    let tree_of = |id: &str, item: &str| {
        UiNode::Tree(UiTreeNode { sections: vec![section(id, item)], presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
    };
    let (mut tree, root, theme, _) = setup(&tree_of("mounted", "row"));
    let Some(node) = tree.node_mut(root) else { panic!("retained tree root") };
    node.spec = crate::wgpu::tree::WidgetSpec(tree_of("never-mounted", "ghost"));
    let mut cursor = RetainedInteractiveSyncCursor::default();
    let mut complete = false;
    for _ in 0..256 {
        let step = sync_interactive_state_node_step(&mut tree, root, &theme, &mut cursor);
        assert_ne!(step, RetainedInteractiveSyncStep::Fault);
        if step == RetainedInteractiveSyncStep::Complete {
            complete = true;
            break;
        }
    }
    assert!(complete);
    assert!(cursor.terminal_is_empty());
}

#[test]
fn retained_tree_sync_abandonment_releases_one_record_or_depth_owner_per_grant() {
    let mut nested = UiTreeItemNode::base("nested", Label::data("Nested"));
    nested.default_open = Some(true);
    nested.items = Some(vec![UiTreeItemNode::base("leaf", Label::data("Leaf"))]);
    let authored = UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode { window: None, id: "section".into(), label: Some(Label::data("Section")), default_open: Some(true), presence: UiPresence::default(), items: vec![nested] }],
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

//#region 📶️ProgressPaint
fn progress(completed: f64, total: Option<f64>) -> UiProgressNode {
    UiProgressNode { id: "progress".into(), completed, total, value_text: Label::data("progress"), presence: UiPresence::default(), menu: None }
}

/// 📶️ Every case of the shared progress law fills exactly the fraction it declares, on a centred
/// `--size-tiny` track — React's bar is `h-tiny w-full` (`🗣️Interpreter/🟦️.tsx`'s `ProgressView`),
/// and the same fraction sizes its fill.
#[test]
fn a_progress_bar_fills_the_fraction_the_shared_fixture_declares() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧬️contract/🧫️fixtures/📶️progress.json")).expect("📶️ the progress fixture parses");
    let theme = Theme::default();
    let bounds = Rect::new(10.0, 20.0, 300.0, 40.0);
    for case in fixture["cases"].as_array().expect("progress cases") {
        let component = &case["component"];
        let node = progress(component["completed"].as_f64().expect("completed"), component["total"].as_f64());
        let (track, fill) = progress_bar_rects(&node, bounds, &theme);
        let height = crate::wgpu::chrome::SIZE_TINY;
        assert_eq!(track, [10.0, 20.0 + (40.0 - height) * 0.5, 300.0, height], "{}: the track spans the bounds, vertically centred", case["id"]);
        match case["fraction"].as_f64() {
            Some(fraction) => assert_eq!(fill, [track[0], track[1], 300.0 * fraction as f32, track[3]], "{}: determinate fill", case["id"]),
            None => assert_eq!(fill, [10.0 + 100.0, track[1], 100.0, track[3]], "{}: a still, centred indeterminate sweep", case["id"]),
        }
    }
}

/// 🎨️ A progress bar paints with theme tokens only: the `separator` track, then the `accent` fill
/// React's own `bg-muted`/`bg-accent` pair resolves to — and an empty determinate bar paints no
/// zero-width fill instance at all.
#[test]
fn a_progress_bar_paints_its_track_and_fill_from_theme_tokens() {
    let theme = Theme::default();
    let bounds = Rect::new(0.0, 0.0, 200.0, 24.0);
    let colors = |node: &UiProgressNode| {
        let mut draw = DrawList::default();
        paint_progress(node, bounds, &theme, &mut draw);
        draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.color).collect::<Vec<_>>()
    };
    let token = |color: Rgba| [color.r, color.g, color.b, color.a];
    assert_eq!(colors(&progress(12.0, Some(100.0))), vec![token(theme.muted), token(theme.accent)]);
    assert_eq!(colors(&progress(3.0, None)), vec![token(theme.muted), token(theme.accent)]);
    assert_eq!(colors(&progress(0.0, Some(100.0))), vec![token(theme.muted)]);
    assert_ne!(token(theme.muted), token(theme.separator), "React's track is `bg-muted`, a distinct token from the separator stroke it stood in for until packet W2k");
}
//#endregion 📶️ProgressPaint

//#region 🖼️UiImagePaint
// 🖼️ W1n: `Component::Image`'s default (no-`SceneHost`) path used to paint a grey placeholder box
// forever — `paint_image` was `#[cfg(test)]`-gated and nothing decoded `src` at all. These cover the
// three halves of the fix: decode, the `object-contain` rect, and the raster quad the paint step emits.

fn encode_base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = u32::from(chunk[0]);
        let b1 = chunk.get(1).copied().map_or(0, u32::from);
        let b2 = chunk.get(2).copied().map_or(0, u32::from);
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(ALPHABET[(triple >> 18) as usize & 0x3F] as char);
        out.push(ALPHABET[(triple >> 12) as usize & 0x3F] as char);
        out.push(if chunk.len() > 1 { ALPHABET[(triple >> 6) as usize & 0x3F] as char } else { '=' });
        out.push(if chunk.len() > 2 { ALPHABET[triple as usize & 0x3F] as char } else { '=' });
    }
    out
}

/// 🖼️ A `data:image/png;base64` source of `width`x`height` solid opaque red, built through the same
/// owned codec the paint registry decodes with — the third-party-free round trip AGENTS.md asks for.
fn png_data_url(width: u32, height: u32) -> String {
    let mut image = semio_framework_pixels::RasterImage::new(width, height);
    for pixel in image.pixels.chunks_mut(4) {
        pixel.copy_from_slice(&[255, 0, 0, 255]);
    }
    let encoded = semio_framework_pixels::encode_png(&image).expect("owned png encode");
    format!("data:image/png;base64,{}", encode_base64(&encoded))
}

fn image_node(id: &str, src: &str) -> UiNode {
    UiNode::Image(UiImageNode { id: id.into(), src: src.into(), alt: Some(Label::data("alt text")), presence: UiPresence::default(), menu: None })
}

#[test]
fn a_data_url_png_source_decodes_to_its_natural_size() {
    let src = png_data_url(6, 3);
    assert_eq!(admit_ui_image(&src), UiImageAdmission::Ready);
    assert_eq!(ui_image_natural_size(&src), Some((6, 3)));
    assert_eq!(admit_ui_image(&src), UiImageAdmission::Ready, "re-admitting an already-decoded source is a lookup, not a second decode");
}

#[test]
fn an_admitted_source_publishes_exactly_one_pending_upload() {
    let src = png_data_url(2, 2);
    assert_eq!(admit_ui_image(&src), UiImageAdmission::Ready);
    let upload = std::iter::from_fn(take_ui_image_upload).find(|upload| upload.key == src).expect("the decoded source is queued for the raster table");
    assert_eq!((upload.width, upload.height), (2, 2));
    assert_eq!(upload.pixels.len(), 2 * 2 * 4, "RGBA8, the exact layout PreparedRasterProducer admits");
    assert_eq!(upload.pixels[..4], [255, 0, 0, 255]);
}

#[test]
fn a_url_source_is_deferred_to_the_host_and_a_jpeg_data_url_is_unsupported() {
    assert_eq!(admit_ui_image("https://example.test/picture.png"), UiImageAdmission::Deferred, "a fetch is the host asset pipeline's job, not this crate's");
    assert_eq!(admit_ui_image("assets/picture.png"), UiImageAdmission::Deferred);
    assert_eq!(admit_ui_image("data:image/jpeg;base64,/9j/4AAQ"), UiImageAdmission::Unsupported, "no third-party JPEG codec may enter the runtime graph");
    assert_eq!(admit_ui_image(""), UiImageAdmission::Refused);
}

/// 📐️ React's `<img className="max-h-64 max-w-full object-contain">` (`🗣️Interpreter/🟦️.tsx:1974`):
/// the box clamps per axis and the natural aspect ratio is letterboxed inside it, centred.
#[test]
fn the_image_content_rect_matches_react_object_contain() {
    let bounds = Rect::new(10.0, 20.0, 100.0, 100.0);
    let wide = ui_image_content_rect(bounds, 200, 100);
    assert_eq!((wide.w, wide.h), (100.0, 50.0), "a wide picture fills the width and letterboxes the height");
    assert_eq!(wide.x, 10.0);
    assert_eq!(wide.y, 20.0 + (100.0 - 50.0) * 0.5);

    let small = ui_image_content_rect(bounds, 40, 20);
    assert_eq!((small.w, small.h), (40.0, 20.0), "a picture smaller than its box is never upscaled");

    let tall = ui_image_content_rect(Rect::new(0.0, 0.0, 400.0, 1_000.0), 400, 800);
    assert_eq!(tall.h, UI_IMAGE_MAX_BOX_HEIGHT, "max-h-64 caps the box at 16rem no matter how tall the layout rect is");
    assert_eq!(tall.w, UI_IMAGE_MAX_BOX_HEIGHT / 2.0);
}

#[test]
fn a_decoded_image_paints_a_raster_quad_instead_of_the_placeholder() {
    let src = png_data_url(4, 2);
    assert_eq!(admit_ui_image(&src), UiImageAdmission::Ready);
    let (tree, root, theme, mut atlas) = setup(&image_node("picture", &src));
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    for _ in 0..8 {
        if paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor) == RetainedNodePaintStep::Complete {
            break;
        }
    }
    let rasters: Vec<_> = draw.layers.iter().flat_map(|layer| layer.raster_instances.iter()).collect();
    assert_eq!(rasters.len(), 1, "exactly one KIND_RASTER quad, keyed by src");
    assert_eq!(rasters[0].0, src);
    let layout = tree.accepted_layout(root).expect("laid out image");
    let expected = ui_image_content_rect(Rect::new(layout.x, layout.y, layout.width, layout.height), 4, 2);
    assert_eq!(rasters[0].1.rect, [expected.x, expected.y, expected.w, expected.h]);
    assert_eq!(draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).count(), 0, "no alt-text fallback once the bitmap is real");
}

/// ⚖️ Law: the paint arm ADMITS the source itself. `admit_ui_image` had only test callers, so in
/// production the ledger stayed empty forever, `ui_image_natural_size` always answered `None`, and
/// EVERY `UiNode::Image` painted the `alt` placeholder however decodable its `src` was. No explicit
/// `admit_ui_image` here on purpose: this is exactly what a host that only paints does.
#[test]
fn the_paint_arm_admits_its_own_source_without_a_separate_admit_call() {
    let src = png_data_url(5, 5);
    assert_eq!(ui_image_natural_size(&src), None, "nothing has admitted this source yet");
    let (tree, root, theme, mut atlas) = setup(&image_node("self-admitting", &src));
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    for _ in 0..64 {
        if paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor) == RetainedNodePaintStep::Complete {
            break;
        }
    }
    assert_eq!(ui_image_natural_size(&src), Some((5, 5)), "the paint arm admitted the source");
    let rasters: Vec<_> = draw.layers.iter().flat_map(|layer| layer.raster_instances.iter()).collect();
    assert_eq!(rasters.len(), 1, "one KIND_RASTER quad, not the placeholder");
    assert_eq!(rasters[0].0, src);
    assert_eq!(draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).count(), 0, "no alt-text fallback once the bitmap is real");
    assert!(std::iter::from_fn(take_ui_image_upload).any(|upload| upload.key == src), "the decoded bitmap is queued for the host's raster-table drain");
}

#[test]
fn an_undecodable_source_still_paints_the_placeholder_and_alt_text() {
    let (tree, root, theme, mut atlas) = setup(&image_node("remote", "https://example.test/remote.png"));
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    for _ in 0..64 {
        if paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor) == RetainedNodePaintStep::Complete {
            break;
        }
    }
    assert_eq!(draw.layers.iter().flat_map(|layer| layer.raster_instances.iter()).count(), 0);
    assert!(draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01), "alt text is what React's broken <img> shows too");
}
//#endregion 🖼️UiImagePaint

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
