/// 🔁️ Layout diffs and even layouts: layout diffs that keep focus and maximization, subtree reuse, even and default layouts, closing and reordering tabs and the split drop preview.

/// 🗄️ A persisted `WindowLayout` snapshot whose `active_window_kind_id` predates the user's
/// later in-session tab switch to `b` — a naive `self.dock.root = dock_from_window_layout(...)`
/// teardown would silently revert focus back to `a`.
#[test]
fn apply_layout_diff_keeps_current_tab_focused_over_stale_persisted_active() {
    let mut dock = DockState::default();
    dock.root = stack_tabs(&["a", "b"], "b");
    dock.active_window_id = Some("b".into());
    let stale_layout = WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: Some("a".into()),
            children: vec![
                WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "a".into(), title: None, instance_id: None, template_id: None, corner: None },
                WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "b".into(), title: None, instance_id: None, template_id: None, corner: None },
            ],
        }),
    };
    dock.apply_layout_diff(&stale_layout);
    assert_eq!(dock.root, stack_tabs(&["a", "b"], "b"), "same membership, reordered-or-not — the user's current tab stays focused");
}

/// 🔑️ The incoming layout reverses the two stacks' order — `b`'s *positional* path moves from
/// `[1]` to `[0]`. A stale-path reuse would now silently misdirect `active_stack`/
/// `maximized_stack` at `a` instead of following `b` by key.
#[test]
fn apply_layout_diff_reanchors_active_and_maximized_stack_by_key() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
    dock.active_window_id = Some("b".into());
    dock.active_stack = Some(vec![1]);
    dock.maximized_stack = Some(vec![1]);
    dock.split_resize_origin = vec![0.5, 0.5];
    let reversed = WindowLayout {
        root: WindowLayoutRoot::Axis(semio_framework_ui_layout::layout::WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    kind: "stack".into(),
                    size: Some(0.5),
                    active_window_kind_id: Some("b".into()),
                    children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "b".into(), title: None, instance_id: None, template_id: None, corner: None }],
                }),
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    kind: "stack".into(),
                    size: Some(0.5),
                    active_window_kind_id: Some("a".into()),
                    children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "a".into(), title: None, instance_id: None, template_id: None, corner: None }],
                }),
            ],
        }),
    };
    dock.apply_layout_diff(&reversed);
    assert_eq!(dock.active_window_id.as_deref(), Some("b"));
    assert_eq!(dock.active_stack, Some(vec![0]));
    assert_eq!(dock.maximized_stack, Some(vec![0]));
    assert!(dock.split_resize_origin.is_empty(), "an in-flight resize gesture's stale indices must not survive a layout swap");
}

#[test]
fn apply_layout_diff_clears_maximized_stack_when_its_window_is_gone() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
    dock.active_window_id = Some("a".into());
    dock.maximized_stack = Some(vec![0]);
    let without_a = WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: Some("b".into()),
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "b".into(), title: None, instance_id: None, template_id: None, corner: None }],
        }),
    };
    dock.apply_layout_diff(&without_a);
    assert_eq!(dock.maximized_stack, None);
    assert_eq!(dock.active_window_id.as_deref(), Some("b"));
}

/// 🌳️ Identical row → the whole node is byte-for-byte the same value (full reuse).
///
/// A structural kind change (Stack -> Row) at index 1 has no shared identity — adopt `next` as-is.
#[test]
fn diff_dock_node_reuses_unchanged_subtree_and_adopts_new_shape_where_changed() {
    let old = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
    let unchanged = diff_dock_node(&old, DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]));
    assert_eq!(unchanged, old);
    let next = DockNode::Row(vec![(stack_with("a"), 0.5), (DockNode::Row(vec![(stack_with("b"), 1.0)]), 0.5)]);
    let diffed = diff_dock_node(&old, next.clone());
    assert_eq!(diffed, next);
}

#[test]
fn even_layout_single_window() {
    let node = even_layout(&["main".into()]);
    assert!(matches!(node, DockNode::Stack { .. }));
    let dock = DockState::from_app(&sample_app(&["main"], None), None);
    assert_eq!(dock.active_window_id.as_deref(), Some("main"));
}

#[test]
fn even_layout_multiple_windows() {
    let node = even_layout(&["a".into(), "b".into(), "c".into()]);
    assert!(matches!(node, DockNode::Row(_)));
    if let DockNode::Row(children) = node {
        assert_eq!(children.len(), 3);
        for (child, size) in children {
            assert!(matches!(child, DockNode::Stack { .. }));
            assert!((size - 1.0 / 3.0).abs() < 0.001);
        }
    }
}

#[test]
fn parses_default_layout_row() {
    let layout = create_default_layout(&["a".into(), "b".into()], "row", None, None);
    let app = sample_app(&["a", "b"], Some(layout));
    let dock = DockState::from_app(&app, None);
    assert!(matches!(dock.root, DockNode::Row(_)));
}

#[test]
fn close_window_collapses_stack_tabs() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = stack_tabs(&["a", "b"], "a");
    let path = vec![];
    dock.close_active_in_stack(&path);
    if let DockNode::Stack { windows, active } = &dock.root {
        assert_eq!(dock_tab_ids(windows), vec!["b".to_string()]);
        assert_eq!(active, "b");
    } else {
        panic!("expected stack");
    }
}

#[test]
fn reorder_tab_within_stack() {
    let mut dock = DockState::default();
    dock.root = stack_tabs(&["a", "b", "c"], "a");
    assert!(dock.reorder_tab(&vec![], 0, 2));
    if let DockNode::Stack { windows, .. } = &dock.root {
        assert_eq!(dock_tab_ids(windows), vec!["b".to_string(), "c".to_string(), "a".to_string()]);
    } else {
        panic!("expected stack");
    }
}

#[test]
fn maximized_stack_uses_full_canvas_bounds() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b", "c"], None), Some("a"));
    dock.root = even_layout(&["a".into(), "b".into(), "c".into()]);
    dock.toggle_maximize(&vec![1]);
    let canvas = Rect::new(0.0, 0.0, 900.0, 600.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let bodies = dock.stack_body_rects(canvas, &theme, &HashMap::new(), &mut atlas);
    assert_eq!(bodies.len(), 1);
    let (_, body, _) = &bodies[0];
    assert!((body.w - (canvas.w - theme.padding_standard * 2.0)).abs() < 1.0);
    assert!((body.h - (canvas.h - dock_cap_depth(&theme))).abs() < 2.0);
}

#[test]
fn split_drop_preview_covers_half_panel() {
    let body = Rect::new(10.0, 20.0, 400.0, 300.0);
    let left = split_drop_preview_in_body(body, DockSide::Left);
    assert_eq!(left.x, 10.0);
    assert_eq!(left.w, 200.0);
    assert_eq!(left.h, 300.0);
    let right = split_drop_preview_in_body(body, DockSide::Right);
    assert_eq!(right.x, 210.0);
    assert_eq!(right.w, 200.0);
}
