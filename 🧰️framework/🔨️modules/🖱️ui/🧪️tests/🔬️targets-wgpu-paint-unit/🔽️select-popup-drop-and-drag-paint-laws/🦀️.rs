// 🔽️ Select popup, drop and drag paint laws: the open select popup clamping, scrolling and hit testing, the production sync pass, the drop target flag, the activatable stack and the draggable tree item.

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
    let has_selected_highlight = open_draw
        .layers
        .iter()
        .flat_map(|layer| layer.ui_instances.iter())
        .any(|instance| (instance.color[0] - theme.row_hover.r).abs() < 0.001 && (instance.color[1] - theme.row_hover.g).abs() < 0.001 && (instance.color[2] - theme.row_hover.b).abs() < 0.001);
    assert!(has_selected_highlight, "the popup row matching the Select's current value should paint a row_hover highlight");
}

#[test]
fn retained_select_popup_is_viewport_clamped_scrolled_and_glass_foreground() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🔽️retained-select-overlay-raster/🔣️.json")).expect("retained Select/overlay raster fixture");
    assert_eq!(fixture["schema"].as_str(), Some("semio.ui.retained-select-overlay-raster.v1"));
    let select_fixture = &fixture["select"];
    let item_count = usize::try_from(select_fixture["itemCount"].as_u64().expect("item count")).expect("item count fits");
    let authored = UiNode::Select(UiSelectNode {
        id: "long-select".into(),
        value: "0".into(),
        items: (0..item_count).map(|index| UiSelectItem { value: index.to_string(), label: Label::data(index.to_string()) }).collect(),
        placeholder: None,
        on_change: action(),
        presence: UiPresence::default(),
        menu: None,
    });
    let mut tree = UiTree::new();
    tree.apply_tree(&stack(vec![authored]));
    let document = tree.root.expect("document root");
    let root = tree.node(document).and_then(|node| node.first_child).expect("select root");
    let theme = Theme::default();
    assert!((theme.control_height - select_fixture["trigger"]["height"].as_f64().expect("trigger height") as f32).abs() < 0.001);
    {
        let node = tree.node_mut(document).expect("document node");
        node.layout.x = 0.0;
        node.layout.y = 0.0;
        node.layout.width = select_fixture["viewport"]["width"].as_f64().expect("viewport width") as f32;
        node.layout.height = select_fixture["viewport"]["height"].as_f64().expect("viewport height") as f32;
    }
    {
        let node = tree.node_mut(root).expect("select node");
        node.layout.x = select_fixture["trigger"]["x"].as_f64().expect("trigger x") as f32;
        node.layout.y = select_fixture["trigger"]["y"].as_f64().expect("trigger y") as f32;
        node.layout.width = select_fixture["trigger"]["width"].as_f64().expect("trigger width") as f32;
        node.layout.height = theme.control_height;
        node.state.open = true;
    }
    let mut sync = RetainedInteractiveSyncCursor::default();
    for _ in 0..4_096 {
        if sync_interactive_state_node_step(&mut tree, root, &theme, &mut sync) == RetainedInteractiveSyncStep::Complete {
            break;
        }
    }
    let popup = tree.node(root).and_then(|node| node.state.select_popup).expect("open Select popup geometry");
    let expected = &select_fixture["expectedInitial"];
    for (actual, key) in [(popup.menu.x, "x"), (popup.menu.y, "y"), (popup.menu.w, "width"), (popup.menu.h, "height")] {
        assert!((actual - expected["menu"][key].as_f64().expect("expected menu scalar") as f32).abs() < 0.001, "menu {key}");
    }
    assert_eq!(popup.first_row, expected["firstRow"].as_u64().expect("first row") as usize);
    assert_eq!(popup.last_row, expected["lastRowExclusive"].as_u64().expect("last row") as usize);
    assert!((popup.scroll - expected["scroll"].as_f64().expect("initial scroll") as f32).abs() < 0.001);
    assert!(popup.menu.y >= SELECT_COLLISION_PADDING);
    assert!(popup.menu.y + popup.menu.h <= select_fixture["viewport"]["height"].as_f64().expect("viewport height") as f32 - SELECT_COLLISION_PADDING + 0.001, "the painted popup remains inside the viewport");
    assert!(popup.last_row - popup.first_row < item_count, "only a viewport-sized row window is retained for paint and hit testing");

    let mut atlas = FontAtlas::builtin();
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    for _ in 0..4_096 {
        match paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor) {
            RetainedNodePaintStep::Pending => {}
            RetainedNodePaintStep::Complete => break,
            RetainedNodePaintStep::Fault => panic!("retained Select paint fault"),
        }
    }
    assert_eq!(draw.glass_regions.len(), 1);
    assert!(draw.layers.iter().any(|layer| layer.foreground_of == Some(0) && !layer.ui_instances.is_empty()), "popup rows are encoded after their own glass");
    assert!(
        !draw
            .layers
            .iter()
            .filter(|layer| layer.foreground_of.is_none())
            .flat_map(|layer| layer.ui_instances.iter())
            .any(|instance| instance.rect[1] >= popup.menu.y && instance.rect[1] < popup.menu.y + popup.menu.h && (instance.params[2] - KIND_GLYPH).abs() < 0.01),
        "popup row glyphs never remain beneath the glass"
    );
    let tail = draw.layers.last().expect("popup route close layer");
    assert!(tail.scissor.is_none() && tail.foreground_of.is_none(), "completed popup paint closes its scissor and glass route");

    let mut cancelled_draw = DrawList::default();
    let mut cancelled = RetainedNodePaintCursor::default();
    for _ in 0..64 {
        let step = paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut cancelled_draw, &mut cancelled);
        assert_ne!(step, RetainedNodePaintStep::Fault);
        if cancelled.popup_route {
            break;
        }
    }
    assert!(cancelled.popup_route, "the cancellation probe reached an open popup route");
    cancelled.cancel_draw_route(&mut cancelled_draw);
    let tail = cancelled_draw.layers.last().expect("cancelled popup route close layer");
    assert!(tail.scissor.is_none() && tail.foreground_of.is_none(), "cancellation closes both retained route stacks");

    let before = popup.scroll;
    let down = popup.down.expect("a long popup has a down chevron");
    assert_eq!(semio_framework_ui_select_behavior::select_scroll_direction_at(popup, down.x + down.w * 0.5, down.y + down.h * 0.5), Some(select_fixture["downPress"]["direction"].as_f64().expect("down direction") as f32));
    assert!(arm_retained_select_scroll_at(&mut tree, root, down.x + down.w * 0.5, down.y + down.h * 0.5));
    let mut sync = RetainedInteractiveSyncCursor::default();
    for _ in 0..4_096 {
        if sync_interactive_state_node_step(&mut tree, root, &theme, &mut sync) == RetainedInteractiveSyncStep::Complete {
            break;
        }
    }
    let scrolled = tree.node(root).and_then(|node| node.state.select_popup).expect("scrolled geometry");
    let down_expected = &select_fixture["downPress"];
    assert_eq!(scrolled.first_row, down_expected["expectedFirstRow"].as_u64().expect("scrolled first row") as usize);
    assert_eq!(scrolled.last_row, down_expected["expectedLastRowExclusive"].as_u64().expect("scrolled last row") as usize);
    assert!(scrolled.scroll > before, "one down-chevron press advances the retained viewport");
    assert!((scrolled.scroll - down_expected["expectedScroll"].as_f64().expect("expected scroll") as f32).abs() < 0.001);
    assert!((scrolled.scroll - select_scroll_step(semio_framework_ui_select_behavior::select_popup_viewport_rect(scrolled).h)).abs() < 0.001, "the press uses React's shared 80%-of-viewport step");
}

/// 🔽️ A CLOSED `Select` materializes no option rows (`reconcile::children_of` gates on
/// `WidgetState::open`), so the popup's rows only exist after re-reconciling with the bit set.
#[test]
fn opening_a_selects_popup_gives_its_synthesized_item_rows_real_hit_testable_layout() {
    let fixture = select("sel", "a");
    let (mut tree, root, theme, mut atlas) = setup(&fixture);
    tree.node_mut(root).unwrap().state.open = true;
    tree.apply_tree(&fixture);
    assert!(semio_framework_ui_wgpu_mounted_layout::layout_tree_now(&mut tree, root, theme, 400.0, 400.0), "layout pass");
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

/// 🔄️ W15a item 7. `reconcile::children_of`/`apply_tree` — the only thing that ever synthesized an
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
    let mut tree = UiTree::new();
    tree.apply_tree(&stack(vec![fixture]));
    let document = tree.root.expect("document root");
    let root = tree.node(document).and_then(|node| node.first_child).expect("select root");
    let theme = Theme::default();
    let document_node = tree.node_mut(document).expect("document node");
    document_node.layout.width = 400.0;
    document_node.layout.height = 400.0;
    let select_node = tree.node_mut(root).expect("select node");
    select_node.layout.y = 100.0;
    select_node.layout.width = 400.0;
    select_node.layout.height = theme.control_height;
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
        presentation: Default::default(),
        sections: vec![UiTreeSectionNode { header_toolbar: None, window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }],
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
