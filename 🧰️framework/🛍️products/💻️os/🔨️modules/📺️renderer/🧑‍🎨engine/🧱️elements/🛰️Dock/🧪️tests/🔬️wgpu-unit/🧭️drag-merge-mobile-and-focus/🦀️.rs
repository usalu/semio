/// 🧭️ Drag, merge, mobile and focus: drag to split and merge, the corner tab bar widths, the mobile flat stack, window activation, the drag lane, the focus border, the insert index and the themed axis geometry.

/// 🎯️ A drag-to-split always previews and commits an exact 50 % split, on both the stack and the
/// root path — React's `splitWithWindow`/`splitRootWithWindow` build a two-child axis with no ratio
/// of their own (`🎨️Canvas/🟦️.tsx:403-421`), and the preview covers exactly half the body (`:778-796`).
#[test]
fn drag_to_split_commits_an_even_two_child_axis() {
    let mut dock = dock_with(stack_tabs(&["a", "b"], "a"), "a");
    assert!(dock.apply_drop(&tab_payload("a", vec![], 0), &DockDropZone::Split { stack_path: vec![], side: DockSide::Bottom }));
    assert_eq!(dock.root, DockNode::Column(vec![(stack_with("b"), 0.5), (stack_with("a"), 0.5)]));
    let mut dock = dock_with(stack_tabs(&["a", "b"], "a"), "a");
    assert!(dock.apply_drop(&tab_payload("a", vec![], 0), &DockDropZone::RootSplit { side: DockSide::Left }));
    assert_eq!(dock.root, DockNode::Row(vec![(stack_with("a"), 0.5), (stack_tabs(&["b"], "b"), 0.5)]));
}

/// 🎯️ A drag-to-merge lands the tab in the target stack's own corner group and focuses it — the
/// corner-local index maps onto the flat child list exactly like `flatIndexForCornerInsert`
/// (`🎨️Canvas/🟦️.tsx:335-347`).
#[test]
fn drag_to_merge_joins_the_target_corner_group_and_focuses_the_tab() {
    let mut dock = dock_with(DockNode::Row(vec![(stack_tabs(&["a", "x"], "a"), 0.5), (stack_with("b"), 0.5)]), "a");
    assert!(dock.apply_drop(&tab_payload("a", vec![0], 0), &DockDropZone::Tab { stack_path: vec![1], corner: WindowStackCorner::TopLeft, index: 0 }));
    let tabs = dock.stack_tabs_at_path(&vec![1]).expect("target stack");
    assert_eq!(dock_tab_ids(&tabs), vec!["a".to_string(), "b".to_string()]);
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
    assert_eq!(dock.active_stack, Some(vec![1]));
}

/// 🪟️ Tab-bar geometry stays in step with what `render_stack` paints: the per-corner drop widths a
/// drag hit-tests against are the SAME capped chip widths, so a drop index can never point between
/// two painted tabs.
#[test]
fn corner_tab_bar_widths_match_the_painted_chip_widths() {
    let dock = dock_with(stack_tabs(&["a", "b"], "a"), "a");
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let labels = HashMap::from([("a".to_string(), "A Window Whose Title Is Far Too Long To Fit Twelve Rem Of Tab".to_string()), ("b".to_string(), "B".to_string())]);
    let bars = dock.stack_corner_tab_bar_rects(Rect::new(0.0, 0.0, 600.0, 400.0), &theme, &mut atlas, &labels);
    let (_, _, _, widths) = bars.iter().find(|(_, corner, _, _)| *corner == WindowStackCorner::TopLeft).expect("top-left bar");
    let expected: Vec<f32> = ["a", "b"].iter().map(|id| dock_tab_chip(&mut atlas, &theme, labels.get(*id).unwrap(), dock.tab_action_count()).1).collect();
    assert_eq!(widths.len(), 2);
    for (got, want) in widths.iter().zip(expected.iter()) {
        assert!((got - want).abs() < 0.001, "{got} vs {want}");
    }
    assert!((widths[0] - MODE_DOCK_TAB_MAX_WIDTH_PX).abs() < 0.001, "the long title is capped, not grown");
}

/// 📱️ Below the breakpoint React renders ONE flat tab stack for the whole mode, through the same
/// `ModeDockStack` chrome as desktop — no split tree, no maximize, every window a tab
/// (`🎨️Canvas/🟦️.tsx:1884-1901`).
#[test]
fn mobile_collapses_every_window_into_one_flat_tab_stack() {
    let mut dock = dock_with(DockNode::Row(vec![(stack_tabs(&["a", "x"], "a"), 0.5), (DockNode::Column(vec![(stack_with("b"), 0.5), (stack_with("c"), 0.5)]), 0.5)]), "b");
    dock.toggle_maximize(&vec![0]);
    let desktop = dock.render_view(None);
    assert_eq!(desktop.root, dock.root, "above the breakpoint the render tree IS the committed tree");
    dock.mobile = true;
    let view = dock.render_view(None);
    assert_eq!(view.root, stack_tabs(&["a", "x", "b", "c"], "b"), "layout order, active window kept, one stack");
    assert_eq!(view.maximized_stack, None, "a mobile window already fills the canvas");
    assert_eq!(view.active_stack, Some(Vec::new()), "the flat stack IS the root");
    assert_eq!(dock.root, DockNode::Row(vec![(stack_tabs(&["a", "x"], "a"), 0.5), (DockNode::Column(vec![(stack_with("b"), 0.5), (stack_with("c"), 0.5)]), 0.5)]), "the committed tree is never flattened");
    let bodies = view.stack_body_rects(Rect::new(0.0, 0.0, 600.0, 900.0), &Theme::default(), &HashMap::new(), &mut FontAtlas::builtin());
    assert_eq!(bodies.len(), 1, "one pane, not four");
}

/// 📱️ A flat-stack tab is painted at the ROOT path, so focus has to travel by window ID — React's
/// `activateWindow(windowId)` (`🎨️Canvas/🟦️.tsx:1446-1452`), never by the path the chip sits at.
#[test]
fn activate_window_focuses_by_id_across_the_committed_tree() {
    let mut dock = dock_with(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_tabs(&["b", "c"], "b"), 0.5)]), "a");
    assert!(dock.activate_window("c"));
    assert_eq!(dock.active_window_id.as_deref(), Some("c"));
    assert_eq!(dock.active_stack, Some(vec![1]), "the id resolved to its own stack, not to the root the chip was painted at");
    assert_eq!(dock.stack_tabs_at_path(&vec![1]).map(|tabs| dock_tab_ids(&tabs)), Some(vec!["b".to_string(), "c".to_string()]));
    assert!(!dock.activate_window("nothing-here"));
}

/// 🌫️ Pressing the canvas background or a gutter clears the active window and keeps the layout —
/// React's `deactivateActiveWindow` (`🎨️Canvas/🟦️.tsx:1455-1472`).
#[test]
fn deactivate_clears_focus_without_touching_the_layout() {
    let mut dock = dock_with(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), "a");
    let before = dock.root.clone();
    assert!(dock.deactivate_active_window());
    assert_eq!(dock.active_window_id, None);
    assert_eq!(dock.active_stack, None);
    assert_eq!(dock.root, before);
    assert!(!dock.deactivate_active_window(), "an already-unfocused mode has nothing to clear");
}

/// 🪟️ The drag lane now collapses on React's `collapseLayout`: an emptied slot is pruned AND a
/// single-child axis is hoisted, keeping the child's own ratio (`🎨️Canvas/🟦️.tsx:225-236`).
///
/// 🩸️ It used to run a prune-only rule, so the drag lane and the close lane disagreed about the tree
/// a removal leaves behind — the docked-out preview kept a one-child axis React had already hoisted,
/// and every drop-zone path inside it was one segment deeper than React's.
#[test]
fn the_drag_lane_hoists_a_single_child_axis_like_the_close_lane() {
    let mut dock = dock_with(DockNode::Row(vec![(DockNode::Column(vec![(stack_with("a"), 0.25), (stack_with("b"), 0.75)]), 0.4), (stack_with("c"), 0.6)]), "a");
    let payload = tab_payload("a", vec![0, 0], 0);
    let floating = dock.render_view(Some(&payload));
    assert_eq!(floating.root, DockNode::Row(vec![(stack_with("b"), 0.75), (stack_with("c"), 0.6)]), "b hoists into the column's slot with ITS OWN 0.75 ratio");
    let mut closed = dock.clone();
    assert!(closed.close_window("a"));
    assert_eq!(closed.root, floating.root, "one collapse rule for the drag lane and the close lane");
}

/// 🪟️ The active stack's outline is painted, not commented out: a hairline along the whole
/// silhouette in `--active-base` when the stack holds the active window, `--border-normal-color`
/// otherwise (`⚛️react/🟦️.tsx:7494-7509`).
#[test]
fn the_active_stack_paints_a_silhouette_focus_border() {
    let theme = Theme::default();
    let strokes = |dock: &DockState| {
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut draw = DrawList::default();
        let labels = HashMap::from([("a".to_string(), "A".to_string()), ("b".to_string(), "B".to_string())]);
        let icon_ids = HashMap::new();
        let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids, control_names: None };
        dock.paint_chrome(&mut ctx, Rect::new(0.0, 0.0, 600.0, 400.0), false);
        draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| instance.rect[2] <= theme.stroke_hairline || instance.rect[3] <= theme.stroke_hairline).count()
    };
    let focused = dock_with(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), "a");
    assert!(strokes(&focused) > 0, "both stacks are outlined — the active one just wears a different colour");
    let mut unfocused = focused.clone();
    assert!(unfocused.deactivate_active_window());
    assert_eq!(strokes(&unfocused), strokes(&focused), "the outline is the window frame, not a focus-only decoration");
}

/// 📥️ Tab insert midpoints are measured on the painted (gapless) chip run — React hit-tests real tab
/// rects (`computeModeDropZone`, `🎨️Canvas/🟦️.tsx:797`), so a phantom gap must not shift the index.
#[test]
fn tab_insert_index_follows_the_painted_gapless_chip_run() {
    let bar = Rect::new(0.0, 0.0, 200.0, 24.0);
    let widths = vec![80.0, 80.0];
    assert_eq!(compute_tab_insert_index(10.0, bar, &widths, 0.0), 0);
    assert_eq!(compute_tab_insert_index(45.0, bar, &widths, 0.0), 1);
    assert_eq!(compute_tab_insert_index(125.0, bar, &widths, 0.0), 2, "past the last tab's midpoint the drop appends");
    let tab_bars = vec![(vec![0], WindowStackCorner::TopLeft, bar, widths)];
    let bodies = vec![(vec![0], Rect::new(0.0, 24.0, 200.0, 200.0), "a".to_string())];
    assert_eq!(compute_dock_drop_zone(125.0, 10.0, &tab_bars, &bodies, Rect::new(0.0, 0.0, 200.0, 224.0)), Some(DockDropZone::Tab { stack_path: vec![0], corner: WindowStackCorner::TopLeft, index: 2 }));
}

/// 📐️ The shared React geometry fixture is the dock's physical axis law: themed separators consume
/// extent before weights, nested axes use the same solver, bodies add horizontal WindowChrome
/// padding, and resize targets stay centred on the reserved separator while clipped to the canvas.
#[test]
fn themed_axis_geometry_matches_the_shared_react_fixture() {
    let repo = fixture_path!("../../../../../../../../../../..");
    let fixture_path = repo.join("🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/📐️dock-axis-geometry/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&fixture_path).unwrap_or_else(|error| panic!("read {}: {error}", fixture_path.display()))).expect("dock axis fixture");
    let token = fixture["oracleSample"]["tokenPixels"].as_f64().expect("token pixels") as f32;
    let width = fixture["viewport"]["width"].as_f64().expect("viewport width") as f32;
    let height = fixture["viewport"]["height"].as_f64().expect("viewport height") as f32;
    let canvas = Rect::new(token, token, width - token * 2.0, height - token * 2.0);
    let stack = |id: &str| DockNode::Stack { windows: vec![DockStackTab::new(id)], active: id.to_string() };
    let dock = DockState {
        root: DockNode::Row(vec![(stack("left"), 35.0), (DockNode::Column(vec![(stack("right-top"), 70.0), (stack("right-bottom"), 30.0)]), 65.0)]),
        active_window_id: Some("left".into()),
        active_stack: Some(vec![0]),
        ..DockState::default()
    };
    let frames = dock.stack_frame_rects_with_separator(canvas, token);
    assert_eq!(frames.len(), 3);
    for (_, rect, id) in &frames {
        let expected = &fixture["oracleSample"]["stacks"][id];
        for (actual, key) in [(rect.x, "x"), (rect.y, "y"), (rect.w, "width"), (rect.h, "height")] {
            let wanted = expected[key].as_f64().unwrap_or_else(|| panic!("{id}.{key}")) as f32;
            assert!((actual - wanted).abs() < 0.001, "📐️ {id}.{key}: {actual} != {wanted}");
        }
    }
    assert!((dock.split_axis_extent(&Vec::new(), canvas, token).expect("root row") - (canvas.w - token)).abs() < 0.001);
    assert!((dock.split_axis_extent(&vec![1], canvas, token).expect("nested column") - (canvas.h - token)).abs() < 0.001);

    let theme = Theme::light();
    assert!((theme.gap_standard - token).abs() < 0.001, "📐️ the neutral sample is the default live spacing token");
    let mut atlas = FontAtlas::builtin();
    let labels = HashMap::new();
    let bodies = dock.stack_body_rects(canvas, &theme, &labels, &mut atlas);
    for (_, body, id) in &bodies {
        let frame = frames.iter().find(|(_, _, frame_id)| frame_id == id).expect("matching frame").1;
        assert!((body.x - frame.x - token).abs() < 0.001, "📐️ {id} begins after one horizontal body inset");
        assert!((body.w - (frame.w - token * 2.0)).abs() < 0.001, "📐️ {id} removes both horizontal body insets");
    }

    let mut draw = DrawList::default();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let icon_ids = HashMap::new();
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids, control_names: None };
    dock.register_resize_hits(&mut ctx, canvas);
    let split_hits = input.staged_hits().iter().filter(|hit| hit.kind == HitKind::DockSplit).collect::<Vec<_>>();
    assert_eq!(split_hits.len(), 2, "📐️ root and nested axes each publish one resize target");
    for hit in split_hits {
        assert!(
            hit.rect.x >= canvas.x && hit.rect.y >= canvas.y && hit.rect.x + hit.rect.w <= canvas.x + canvas.w + 0.001 && hit.rect.y + hit.rect.h <= canvas.y + canvas.h + 0.001,
            "📐️ physical hit rect is clipped to its solved canvas: {:?}",
            hit.rect
        );
    }

    let single = DockState { root: stack("single"), ..DockState::default() };
    assert_eq!(single.stack_frame_rects_with_separator(canvas, token)[0].1, canvas, "📐️ a single stack reserves no separator");
}
