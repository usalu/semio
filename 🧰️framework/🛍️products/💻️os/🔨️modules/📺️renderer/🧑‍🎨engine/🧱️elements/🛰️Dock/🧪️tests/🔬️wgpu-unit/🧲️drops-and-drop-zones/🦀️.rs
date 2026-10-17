/// 🧲️ Drops and drop zones: tab and stack drops across corners, stacks and axes, abandoned drags, the drop zone and the split side.

#[test]
fn apply_drop_tab_moves_window_to_target_corner() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "x"], "a"), 0.5), (stack_tabs(&["b"], "b"), 0.5)]);
    let payload = tab_payload("a", vec![0], 0);
    let zone = DockDropZone::Tab { stack_path: vec![1], corner: WindowStackCorner::BottomRight, index: 0 };
    assert!(dock.apply_drop(&payload, &zone));
    let tabs = dock.stack_tabs_at_path(&vec![1]).expect("target stack");
    assert!(tabs.iter().any(|tab| tab.window_id == "a" && tab.corner == WindowStackCorner::BottomRight));
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
}

/// 🪟️ ONE merged top span, not one per tab: React's tab bar is `flex items-stretch justify-start`
/// with no gap utility (`🎨️Canvas/🟦️.tsx:1108`), so adjacent chips form a single silhouette edge
/// and the content clip is body + that one span.
#[test]
fn dock_stack_content_fills_full_bounds_through_one_silhouette_clip() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = stack_tabs(&["a", "b"], "a");
    let bounds = Rect::new(10.0, 20.0, 600.0, 400.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let labels = HashMap::from([("a".into(), "A".into()), ("b".into(), "B".into())]);
    let icon_ids = HashMap::new();
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids, control_names: None };
    dock.paint_chrome(&mut ctx, bounds, true);
    let fill = draw.layers.iter().find(|layer| layer.clip.is_some() && layer.ui_instances.iter().any(|instance| instance.rect == [bounds.x, bounds.y, bounds.w, bounds.h])).expect("full silhouette content fill");
    assert_eq!(fill.clip.as_ref().map(|clip| clip.scissors.len()), Some(2));
    assert!(!fill.clip.as_ref().is_some_and(|clip| clip.scissors.iter().any(|rect| rect.x <= 300 && 300 < rect.x + rect.w && rect.y <= 30 && 30 < rect.y + rect.h)));
}

/// 🏁️ `register_hit` only STAGES; `hit_at` resolves the PUBLISHED registry, so the frame build has
/// to be promoted before the pointer authority can see either target.
#[test]
fn resize_hits_win_over_later_scroll_region() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = even_layout(&["a".into(), "b".into()]);
    let canvas = Rect::new(0.0, 0.0, 400.0, 300.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let labels = HashMap::from([("a".into(), "A".into()), ("b".into(), "B".into())]);
    input.register_hit(HitTarget { rect: canvas, event: None, control_id: Some("content.scroll".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &IconAtlas::default(), input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &HashMap::new(), control_names: None };
    dock.register_resize_hits(&mut ctx, canvas);
    input.publish_hits();
    let hit = input.hit_at(200.0, 150.0).expect("split hit");
    assert_eq!(hit.kind, HitKind::DockSplit);
    assert_eq!(hit.drag_axis, Some(DragAxis::Horizontal));
    assert!(hit.rect.w >= 20.0);
}

fn stack_with(id: &str) -> DockNode {
    DockNode::Stack { windows: vec![tab(id)], active: id.into() }
}

fn tab_payload(window_id: &str, source_path: DockPath, tab_index: usize) -> DockDragPayload {
    DockDragPayload { kind: DockDragKind::Tab, window_id: window_id.into(), window_kind_id: window_id.into(), template_id: None, source_path, tab_index, ghost_label: window_id.into() }
}

fn stack_payload(window_id: &str, source_path: DockPath, tab_index: usize) -> DockDragPayload {
    DockDragPayload { kind: DockDragKind::Stack, window_id: window_id.into(), window_kind_id: window_id.into(), template_id: None, source_path, tab_index, ghost_label: window_id.into() }
}

/// 🎯️ A cross-stack tab drop lands where the DERIVED tree said it would: the drag leaves the
/// committed tree alone and paints `render_view`'s docked-out derivation, so the `stack_path` the
/// pointer resolved there is the very path `apply_drop`'s own re-derivation addresses.
///
/// 🩸️ The lane used to remove the window from the committed tree at drag promotion and then insert
/// into the mutated tree; a drop that refused left the user's layout mutilated, and `apply_drop`'s
/// own (second) removal always failed, which silently no-opped every cross-stack drop.
///
/// 🪟️ Three stacks so lifting `a` (the sole occupant of stack `[0]`) prunes that slot without
/// also emptying the drop target — `b` shifts from `[1]` down to `[0]`, and `c` (the actual
/// cross-stack drop target) shifts from `[2]` to `[1]`.
#[test]
fn apply_drop_tab_moves_window_across_stacks() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_with("a"), 0.34), (stack_with("b"), 0.33), (stack_with("c"), 0.33)]);
    let payload = tab_payload("a", vec![0], 0);
    let floating = dock.render_view(Some(&payload));
    assert_eq!(node_at(&floating.root, &vec![1]), Some(&stack_with("c")), "c shifted to [1] in the DERIVED tree the drag paints");
    assert_eq!(node_at(&dock.root, &vec![0]), Some(&stack_with("a")), "the committed tree still holds a — a drag is not an edit");
    let zone = DockDropZone::Tab { stack_path: vec![1], corner: WindowStackCorner::TopLeft, index: 0 };
    assert!(dock.apply_drop(&payload, &zone), "cross-stack tab drop must actually land");
    assert_eq!(node_at(&dock.root, &vec![1]), Some(&stack_tabs(&["a", "c"], "a")));
    assert!(find_stack_path(&dock.root, "b", &mut vec![]).is_some(), "b (untouched by this drop) still resolvable");
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
}

/// 🎯️ An abandoned drag costs nothing: the committed tree is byte-identical before and after, which
/// is what retires the `WindowLayout` snapshot the old eager-removal lane had to keep.
///
/// 🪟️ Lifting `a` leaves a one-child axis, which React's `collapseLayout` HOISTS — so the derived
/// tree the pointer hit-tests is the bare stack at the ROOT path, and that is the path the drop
/// carries. (The old drag lane collapsed on a prune-only rule and left the axis standing at `[0]`.)
#[test]
fn an_abandoned_tab_drag_leaves_the_committed_tree_untouched() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
    let before = dock.root.clone();
    let payload = tab_payload("a", vec![0], 0);
    let floating = dock.render_view(Some(&payload));
    assert_eq!(floating.root, stack_with("b"), "the derivation hoists b out of the one-child axis");
    assert_eq!(dock.root, before);
    assert!(!dock.apply_drop(&payload, &DockDropZone::Tab { stack_path: vec![7], corner: WindowStackCorner::TopLeft, index: 0 }), "a zone that resolves to nothing refuses");
    assert_eq!(dock.root, before, "a refused drop is not an edit either");
}

#[test]
fn apply_drop_tab_reinserts_into_originating_stack_at_new_index() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "b", "c"], "a"), 1.0)]);
    let payload = tab_payload("a", vec![0], 0);
    let floating = dock.render_view(Some(&payload));
    assert_eq!(floating.root, stack_tabs(&["b", "c"], "b"));
    let zone = DockDropZone::Tab { stack_path: vec![], corner: WindowStackCorner::TopLeft, index: 2 };
    assert!(dock.apply_drop(&payload, &zone));
    assert_eq!(dock.root, stack_tabs(&["b", "c", "a"], "a"));
}

/// 🪟️ Lifting the sole occupant of stack `a` prunes its slot and hoists `b` to the ROOT of the
/// derived tree — so that is where the split lands, and the axis is rebuilt from scratch.
#[test]
fn apply_drop_tab_split_targets_the_post_removal_stack() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
    let payload = tab_payload("a", vec![0], 0);
    let zone = DockDropZone::Split { stack_path: vec![], side: DockSide::Right };
    assert!(dock.apply_drop(&payload, &zone));
    assert_eq!(dock.root, DockNode::Row(vec![(stack_with("b"), 0.5), (stack_with("a"), 0.5)]));
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
}

#[test]
fn apply_drop_tab_root_split_builds_axis_pair() {
    let mut dock = DockState::default();
    dock.root = stack_tabs(&["a", "b"], "a");
    let payload = tab_payload("a", vec![], 0);
    let zone = DockDropZone::RootSplit { side: DockSide::Left };
    assert!(dock.apply_drop(&payload, &zone));
    assert_eq!(dock.root, DockNode::Row(vec![(stack_with("a"), 0.5), (stack_tabs(&["b"], "b"), 0.5)]));
}

/// 🪟️ A whole-stack drag lifts the stack NODE, tab order and all — `extractStackFromLayout`, not
/// a window-by-window removal that had to be stitched back together around `tab_index`.
#[test]
fn apply_drop_stack_moves_whole_group_preserving_order_and_target_key() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "b", "c"], "b"), 0.5), (stack_with("d"), 0.5)]);
    let payload = stack_payload("b", vec![0], 1);
    let floating = dock.render_view(Some(&payload));
    assert_eq!(floating.root, stack_with("d"), "the one remaining sibling hoists to the root");
    let zone = DockDropZone::Tab { stack_path: vec![], corner: WindowStackCorner::TopLeft, index: 0 };
    assert!(dock.apply_drop(&payload, &zone), "whole-stack tab-join must land");
    assert_eq!(dock.root, stack_tabs(&["a", "b", "c", "d"], "b"), "the group keeps its order and joins d's corner");
    assert_eq!(dock.active_window_id.as_deref(), Some("b"));
}

/// 🎯️ A whole-stack split needs no key re-anchoring at all under the committed-tree model: the zone
/// the pointer resolved against the docked-out tree IS a path into the tree `apply_drop` re-derives.
///
/// 🩸️ The old lane removed the drag's active window at promotion and extracted the REST of the source
/// stack inside `apply_drop`, so ancestors collapsed *between* hit-testing and committing; it had to
/// remember the target stack's active window and look its path up again afterwards.
#[test]
fn apply_drop_stack_split_lands_on_the_derived_path_without_reanchoring() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "x"], "a"), 0.5), (stack_with("b"), 0.5)]);
    let payload = stack_payload("a", vec![0], 0);
    let floating = dock.render_view(Some(&payload));
    assert_eq!(floating.root, stack_with("b"), "lifting the whole stack hoists b to the root");
    let zone = DockDropZone::Split { stack_path: vec![], side: DockSide::Bottom };
    assert!(dock.apply_drop(&payload, &zone), "the derived root path is exactly where the drop lands");
    assert_eq!(dock.root, DockNode::Column(vec![(stack_with("b"), 0.5), (stack_tabs(&["a", "x"], "a"), 0.5)]), "a travelled with its sibling x, as one stack node");
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
}

#[test]
fn apply_drop_stack_same_source_is_noop() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "b"], "a"), 0.5), (stack_with("c"), 0.5)]);
    let before = dock.root.clone();
    let payload = stack_payload("a", vec![0], 0);
    let zone = DockDropZone::Tab { stack_path: vec![0], corner: WindowStackCorner::TopLeft, index: 0 };
    assert!(!dock.apply_drop(&payload, &zone), "dropping a stack back onto itself is a no-operation");
    assert_eq!(dock.root, before);
}

/// 🪂️ Inside the tab bar rect → `Tab` zone wins even though it's also inside the body's column span.
///
/// Inside the body but below the tab bar → `Split` zone.
///
/// Outside every registered stack but inside the canvas → `RootSplit`.
#[test]
fn compute_dock_drop_zone_prefers_tab_bar_over_body_over_root() {
    let tab_bars = vec![(vec![0], WindowStackCorner::TopLeft, Rect::new(0.0, 0.0, 200.0, 24.0), vec![80.0, 80.0])];
    let bodies = vec![(vec![0], Rect::new(0.0, 24.0, 200.0, 200.0), "a".to_string())];
    let canvas = Rect::new(0.0, 0.0, 200.0, 224.0);
    assert_eq!(compute_dock_drop_zone(10.0, 10.0, &tab_bars, &bodies, canvas), Some(DockDropZone::Tab { stack_path: vec![0], corner: WindowStackCorner::TopLeft, index: 0 }));
    assert!(matches!(compute_dock_drop_zone(100.0, 100.0, &tab_bars, &bodies, canvas), Some(DockDropZone::Split { .. })));
    assert!(matches!(compute_dock_drop_zone(190.0, 300.0, &tab_bars, &bodies, canvas), None));
    let wide_canvas = Rect::new(0.0, 0.0, 400.0, 400.0);
    assert!(matches!(compute_dock_drop_zone(390.0, 390.0, &tab_bars, &bodies, wide_canvas), Some(DockDropZone::RootSplit { .. })));
}

/// 🪓️ Wide-and-short body: a small vertical offset from center stays dominated by the x-axis.
///
/// Tall-and-narrow body: a small horizontal offset stays dominated by the y-axis.
#[test]
fn resolve_split_side_uses_dominant_axis_from_center() {
    assert_eq!(resolve_split_side(10.0, 60.0, 400.0, 120.0), DockSide::Left);
    assert_eq!(resolve_split_side(390.0, 60.0, 400.0, 120.0), DockSide::Right);
    assert_eq!(resolve_split_side(60.0, 10.0, 120.0, 400.0), DockSide::Top);
    assert_eq!(resolve_split_side(60.0, 390.0, 120.0, 400.0), DockSide::Bottom);
}
