// 🧵️ Retained paint grant laws: the glyph count helper and the grant-bounded retained text, input, select and tree paint with their cancellation and exact owner witnesses.

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
    let offset = semio_framework_ui_text::faux_bold_offset(Theme::default().font_size_emphasized);
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
        header_toolbar: None,
        window: None,
        id: id.into(),
        label: Some(Label::data(id)),
        default_open: Some(true),
        presence: UiPresence::default(),
        items: vec![UiTreeItemNode::base(item, Label::data(item))],
    };
    let tree_of = |id: &str, item: &str| UiNode::Tree(UiTreeNode { presentation: Default::default(), sections: vec![section(id, item)], presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None });
    let (mut tree, root, theme, _) = setup(&tree_of("mounted", "row"));
    let Some(node) = tree.node_mut(root) else { panic!("retained tree root") };
    node.spec = semio_framework_ui_retained_tree::tree::WidgetSpec(tree_of("never-mounted", "ghost"));
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
        presentation: Default::default(),
        sections: vec![UiTreeSectionNode { header_toolbar: None, window: None, id: "section".into(), label: Some(Label::data("Section")), default_open: Some(true), presence: UiPresence::default(), items: vec![nested] }],
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
