/// 🧱️ Paint fixtures and loading border laws: the action, text, stack and button fixtures, the paint setup and the text, stack, button, section and tree loading and waiting border laws.

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
    assert!(semio_framework_ui_wgpu_mounted_layout::layout_tree_now(&mut tree, root, theme, 400.0, 400.0), "layout pass");
    (tree, root, theme, atlas)
}

fn paint_retained_node_for_test(tree: &UiTree, node: NodeId, theme: &Theme, atlas: &mut FontAtlas) -> DrawList {
    let layout = tree.accepted_layout(node).expect("retained node layout");
    let absolute = tree.absolute_rect(node).expect("retained node bounds");
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    for _ in 0..16_384 {
        match paint_node_step_with_driver(tree, node, absolute.x - layout.x, absolute.y - layout.y, theme, atlas, None, false, UiDriverDrag::Handle, false, ui_contract::FlowInline::Ltr, &mut draw, &mut cursor) {
            RetainedNodePaintStep::Pending => {}
            RetainedNodePaintStep::Complete => return draw,
            RetainedNodePaintStep::Fault => panic!("retained node paint fault"),
        }
    }
    panic!("retained node paint exceeded its fixed budget")
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
        presentation: Default::default(),
        sections: vec![UiTreeSectionNode { header_toolbar: None, window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", Label::data("Item"))] }],
        presence: UiPresence::status(UiStatus::Loading),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
}

fn waiting_tree() -> UiNode {
    UiNode::Tree(UiTreeNode {
        presentation: Default::default(),
        sections: vec![UiTreeSectionNode { header_toolbar: None, window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", Label::data("Item"))] }],
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
//#endregion 🔖️FidelityFixes
