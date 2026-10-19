//! 🎯️ Dock state render and drop: the render view, the window activation, the drop, the tab reorder and insertion methods of `DockState`. See [`super`].

use super::*;

//#region DockLayout
impl DockState {
    /// 🖼️ The tree this frame actually RENDERS, derived from the committed `root` and never written
    /// back — React's two render-only derivations in one place: `mobileFlatStack`
    /// (`🧱️elements/🎨️Canvas/🟦️.tsx:1884-1891`), which collapses every window of a narrow viewport
    /// into ONE flat tab stack, and `modeDockOutLayout` (`:1869`, `:841-847`), which lifts the
    /// dragged tab or stack out while it floats on the cursor.
    ///
    /// 🔑️ Because the pointer resolves its drop zones against THIS tree and [`Self::apply_drop`]
    /// re-derives the very same removal from the committed tree, a zone's `stack_path` is valid in
    /// the tree the drop is applied to — which is what retires the old eager-removal lane's
    /// re-anchoring and its layout snapshot.
    pub fn render_view(&self, drag: Option<&DockDragPayload>) -> DockState {
        let mut view = self.clone();
        if let Some(payload) = drag {
            view.root = dock_out_layout(&self.root, payload);
        }
        if self.mobile {
            view.root = mobile_flat_stack(&view.root, view.active_window_id.as_deref());
            view.maximized_stack = None;
            view.active_stack = view.active_window_id.as_ref().map(|_| empty_path());
            return view;
        }
        view.maximized_stack = view.maximized_stack.filter(|path| matches!(node_at(&view.root, path), Some(DockNode::Stack { .. })));
        view.active_stack = view.active_window_id.as_deref().and_then(|id| find_stack_path(&view.root, id, &mut vec![]));
        view
    }

    /// 🪟️ Focuses one window wherever it sits — React's `activateWindow(windowId)`
    /// (`🧱️elements/🎨️Canvas/🟦️.tsx:1446-1452`), which is addressed by ID, not by stack path. The
    /// mobile flat stack renders every tab at the root path, so a path-addressed focus would aim at
    /// a node the committed tree does not have there.
    pub fn activate_window(&mut self, window_id: &str) -> bool {
        let Some(path) = find_stack_path(&self.root, window_id, &mut vec![]) else {
            return false;
        };
        self.set_stack_active(&path, window_id);
        true
    }

    /// 🌫️ Clears the focused window — React's `deactivateActiveWindow`
    /// (`🧱️elements/🎨️Canvas/🟦️.tsx:1455-1459`), reached by pressing the canvas background or a
    /// resize gutter.
    pub fn deactivate_active_window(&mut self) -> bool {
        if self.active_window_id.is_none() && self.active_stack.is_none() {
            return false;
        }
        self.active_window_id = None;
        self.active_stack = None;
        true
    }

    /// 🎯️ Commits a completed dock drag against the COMMITTED tree — React's `applyModeDrop`
    /// (`🧱️elements/🎨️Canvas/🟦️.tsx:816-838`). Every branch re-derives the same removal
    /// [`Self::render_view`] showed the user, then inserts the travelling window (or the whole
    /// travelling stack) at the resolved zone.
    ///
    /// 🩸️ The lane used to remove the dragged window EAGERLY at drag promotion and mutate `root`
    /// there, so an abandoned drag was a destructive edit only a `WindowLayout` snapshot could undo,
    /// the drag lane collapsed on the weaker prune-but-never-hoist rule the close lane had already
    /// left behind, and a whole-stack drop had to re-anchor its target by key because extraction
    /// shifted the paths under a zone the pointer had already resolved.
    pub fn apply_drop(&mut self, drag: &DockDragPayload, zone: &DockDropZone) -> bool {
        let mut window_kinds: HashMap<String, String> = self.window_instances().into_iter().collect();
        window_kinds.insert(drag.window_id.clone(), drag.window_kind_id.clone());
        let maximized_key = self.maximized_stack.as_ref().and_then(|path| node_at(&self.root, path)).and_then(|node| match node {
            DockNode::Stack { active, .. } => Some(active.clone()),
            _ => None,
        });
        if drag.kind == DockDragKind::NewWindow && self.collect_window_ids().contains(&drag.window_id) {
            return false;
        }
        let (mut next, incoming) = match drag.kind {
            DockDragKind::Tab => {
                if !self.collect_window_ids().iter().any(|id| *id == drag.window_id) {
                    return false;
                }
                let tab = DockStackTab::instance_template(drag.window_id.clone(), drag.window_kind_id.clone(), drag.template_id.clone(), WindowStackCorner::TopLeft);
                (remove_window_from_layout(&self.root, &drag.window_id).unwrap_or_else(empty_stack), DockNode::Stack { windows: vec![tab], active: drag.window_id.clone() })
            }
            DockDragKind::Stack => {
                let (base, Some(stack)) = extract_stack_from_layout(&self.root, &drag.source_path) else {
                    return false;
                };
                (base.unwrap_or_else(empty_stack), stack)
            }
            DockDragKind::NewWindow => {
                let tab = DockStackTab::instance_template(drag.window_id.clone(), drag.window_kind_id.clone(), drag.template_id.clone(), WindowStackCorner::TopLeft);
                (self.root.clone(), DockNode::Stack { windows: vec![tab], active: drag.window_id.clone() })
            }
        };
        let DockNode::Stack { windows: group, .. } = &incoming else {
            return false;
        };
        let group_ids = dock_tab_ids(group);
        let group_templates: Vec<(String, Option<String>)> = group.iter().map(|tab| (tab.window_id.clone(), tab.template_id.clone())).collect();
        let landed = match zone {
            DockDropZone::Tab { stack_path, corner, index } => insert_tabs_at_corner_in_node(&mut next, stack_path, &group_ids, *corner, Some(*index), &drag.window_id, &window_kinds),
            DockDropZone::Split { stack_path, side } => split_node_at_path(&mut next, stack_path, incoming.clone(), *side),
            DockDropZone::RootSplit { side } => {
                next = split_root_node(next, incoming.clone(), *side);
                true
            }
        };
        if !landed {
            return false;
        }
        for (window_id, template_id) in group_templates {
            set_window_template(&mut next, &window_id, template_id);
        }
        self.root = next;
        self.active_window_id = Some(drag.window_id.clone());
        self.active_stack = find_stack_path(&self.root, &drag.window_id, &mut vec![]);
        self.maximized_stack = maximized_key.and_then(|id| find_stack_path(&self.root, &id, &mut vec![])).filter(|_| self.can_maximize());
        self.split_resize_origin.clear();
        true
    }

    pub fn reorder_tab(&mut self, path: &DockPath, from: usize, to: usize) -> bool {
        let Some(stack) = node_at_mut(&mut self.root, path) else {
            return false;
        };
        let DockNode::Stack { windows, .. } = stack else {
            return false;
        };
        if from >= windows.len() || to > windows.len() || from == to {
            return false;
        }
        let window_id = windows.remove(from);
        windows.insert(to.min(windows.len()), window_id);
        true
    }

    pub fn insert_tab(&mut self, path: &DockPath, window_id: &str, index: Option<usize>) -> bool {
        self.insert_tab_at_corner(path, window_id, WindowStackCorner::TopLeft, index)
    }

    /// 🧭️ Inserts a tab into a specific chrome corner at a corner-local index.
    pub fn insert_tab_at_corner(&mut self, path: &DockPath, window_id: &str, corner: WindowStackCorner, index: Option<usize>) -> bool {
        self.insert_tabs_at_corner(path, &[window_id.to_string()], corner, index, window_id)
    }

    /// 🗄️ Inserts every window in `🪟️windows` (in order) starting at `index`, skipping any already
    /// present at the target (mirrors `insert_tab`'s dedupe guard), then focuses `active_id` — the
    /// multi-window counterpart `apply_drop` uses for whole-stack tab-join drops.
    pub fn insert_tabs(&mut self, path: &DockPath, windows: &[String], index: Option<usize>, active_id: &str) -> bool {
        self.insert_tabs_at_corner(path, windows, WindowStackCorner::TopLeft, index, active_id)
    }

    /// 🧭️ Corner-aware multi-tab insert used by tab-join drops.
    pub fn insert_tabs_at_corner(&mut self, path: &DockPath, windows: &[String], corner: WindowStackCorner, index: Option<usize>, active_id: &str) -> bool {
        self.insert_tabs_at_corner_with_kinds(path, windows, corner, index, active_id, &HashMap::new())
    }

    fn insert_tabs_at_corner_with_kinds(&mut self, path: &DockPath, windows: &[String], corner: WindowStackCorner, index: Option<usize>, active_id: &str, window_kinds: &HashMap<String, String>) -> bool {
        if !insert_tabs_at_corner_in_node(&mut self.root, path, windows, corner, index, active_id, window_kinds) {
            return false;
        }
        self.active_window_id = Some(active_id.to_string());
        self.active_stack = Some(path.to_vec());
        true
    }

    pub fn split_stack_with_window(&mut self, path: &DockPath, window_id: &str, side: DockSide) -> bool {
        self.split_stack_with_stack(path, vec![window_id.to_string()], window_id.to_string(), side)
    }

    /// 🪟️ Splits the stack at `path` with a NEW INSTANCE of an existing window kind — what "Open in New
    /// Window" (`ui.window.newWindow`, `mod+shift+n`) asks for. [`split_stack_with_window`] above cannot
    /// express it: it derives the tab's kind from its id, so a `{kind}-2` instance id would mint a tab of
    /// a window kind the app never declared and render blank.
    pub fn split_stack_with_instance(&mut self, path: &DockPath, window_id: &str, window_kind_id: &str, side: DockSide) -> bool {
        let kinds = HashMap::from([(window_id.to_string(), window_kind_id.to_string())]);
        self.split_stack_with_stack_and_kinds(path, vec![window_id.to_string()], window_id.to_string(), side, &kinds)
    }

    /// 🪟️ Splits the stack at `path` with an already-assembled multi-window stack — the whole-stack
    /// counterpart `apply_drop` uses; single-window splits go through `split_stack_with_window` above.
    pub fn split_stack_with_stack(&mut self, path: &DockPath, windows: Vec<String>, active_id: String, side: DockSide) -> bool {
        self.split_stack_with_stack_and_kinds(path, windows, active_id, side, &HashMap::new())
    }

    fn split_stack_with_stack_and_kinds(&mut self, path: &DockPath, windows: Vec<String>, active_id: String, side: DockSide, window_kinds: &HashMap<String, String>) -> bool {
        if !split_node_at_path(&mut self.root, path, stack_of(&windows, &active_id, window_kinds), side) {
            return false;
        }
        self.active_window_id = Some(active_id.clone());
        self.active_stack = find_stack_path(&self.root, &active_id, &mut vec![]);
        true
    }

    pub fn split_root_with_window(&mut self, window_id: &str, side: DockSide) -> bool {
        self.split_root_with_stack(vec![window_id.to_string()], window_id.to_string(), side)
    }

    /// 🪟️ Splits the mode root with an already-assembled multi-window stack — see
    /// `split_stack_with_stack`; single-window splits go through `split_root_with_window` above.
    pub fn split_root_with_stack(&mut self, windows: Vec<String>, active_id: String, side: DockSide) -> bool {
        self.split_root_with_stack_and_kinds(windows, active_id, side, &HashMap::new())
    }

    fn split_root_with_stack_and_kinds(&mut self, windows: Vec<String>, active_id: String, side: DockSide, window_kinds: &HashMap<String, String>) -> bool {
        let current = std::mem::replace(&mut self.root, empty_stack());
        self.root = split_root_node(current, stack_of(&windows, &active_id, window_kinds), side);
        self.active_window_id = Some(active_id.clone());
        self.active_stack = find_stack_path(&self.root, &active_id, &mut vec![]);
        true
    }

    /// 🔑️ Applies an incoming `WindowLayout` via keyed diff instead of a full teardown-and-rebuild:
    /// unchanged stacks/axes are reused wholesale (`diff_dock_node`), a stack that's merely reordered
    /// keeps the user's *current* tab focused instead of reverting to whatever a persisted snapshot's
    /// `active` field says, and `active_stack`/`maximized_stack` are re-resolved by window-id key
    /// against the new tree rather than reused as stale positional `DockPath`s a structural change
    /// could silently misdirect (or leave pointing at a now-out-of-bounds index). Any in-flight resize
    /// gesture is abandoned (`split_resize_origin` cleared) since a swapped layout invalidates its
    /// origin indices. See `ShellState::sync_dock`'s `self.dock.root = dock_from_window_layout(...)`
    /// teardown for the call site this is meant to replace — wiring that swap is a follow-up step for
    /// whoever next owns the `shell` region (out of this `dock` claim's bounds).
    pub fn apply_layout_diff(&mut self, layout: &WindowLayout) {
        let next_root = dock_from_window_layout(&layout.root);
        let maximized_key = self.maximized_stack.as_ref().and_then(|path| node_at(&self.root, path)).and_then(|node| match node {
            DockNode::Stack { active, .. } => Some(active.clone()),
            _ => None,
        });
        let active_key = self.active_window_id.clone();
        self.root = diff_dock_node(&self.root, next_root);
        self.active_window_id = active_key.filter(|id| find_stack_path(&self.root, id, &mut vec![]).is_some()).or_else(|| first_window_id(&self.root));
        self.active_stack = self.active_window_id.as_deref().and_then(|id| find_stack_path(&self.root, id, &mut vec![]));
        self.maximized_stack = maximized_key.and_then(|id| find_stack_path(&self.root, &id, &mut vec![]));
        self.split_resize_origin.clear();
    }

    pub fn stack_windows_at_path(&self, path: &DockPath) -> Option<Vec<String>> {
        let DockNode::Stack { windows, .. } = node_at(&self.root, path)? else {
            return None;
        };
        Some(dock_tab_ids(windows))
    }

    /// 🧭️ Returns the full tab records (ids + corners) for a stack path.
    pub fn stack_tabs_at_path(&self, path: &DockPath) -> Option<Vec<DockStackTab>> {
        let DockNode::Stack { windows, .. } = node_at(&self.root, path)? else {
            return None;
        };
        Some(windows.clone())
    }

    pub fn window_instances(&self) -> Vec<(String, String)> {
        fn collect(node: &DockNode, out: &mut Vec<(String, String)>) {
            match node {
                DockNode::Stack { windows, .. } => out.extend(windows.iter().map(|tab| (tab.window_id.clone(), tab.window_kind_id.clone()))),
                DockNode::Row(children) | DockNode::Column(children) => children.iter().for_each(|(child, _)| collect(child, out)),
            }
        }
        let mut instances = Vec::new();
        collect(&self.root, &mut instances);
        instances
    }

    pub fn window_kind_id(&self, window_id: &str) -> Option<&str> {
        fn find<'a>(node: &'a DockNode, window_id: &str) -> Option<&'a str> {
            match node {
                DockNode::Stack { windows, .. } => windows.iter().find(|tab| tab.window_id == window_id).map(|tab| tab.window_kind_id.as_str()),
                DockNode::Row(children) | DockNode::Column(children) => children.iter().find_map(|(child, _)| find(child, window_id)),
            }
        }
        find(&self.root, window_id)
    }

    pub fn window_template_id(&self, window_id: &str) -> Option<&str> {
        fn find<'a>(node: &'a DockNode, window_id: &str) -> Option<&'a str> {
            match node {
                DockNode::Stack { windows, .. } => windows.iter().find(|tab| tab.window_id == window_id).and_then(|tab| tab.template_id.as_deref()),
                DockNode::Row(children) | DockNode::Column(children) => children.iter().find_map(|(child, _)| find(child, window_id)),
            }
        }
        find(&self.root, window_id)
    }

    pub fn to_window_layout(&self) -> WindowLayout {
        WindowLayout { root: dock_node_to_layout_root(&self.root) }
    }

    pub fn register_hits(&self, ctx: &mut DockRenderContext<'_>, bounds: Rect) {
        if let Some(path) = &self.maximized_stack {
            if let Some(node) = node_at(&self.root, path) {
                let rect = bounds;
                if let DockNode::Stack { .. } = node {
                    render_stack(self, ctx, path, node, rect, true, true, &mut |_, _| {});
                    return;
                }
            }
        }
        render_node(self, ctx, &self.root, bounds, &empty_path(), true, &mut |_, _| {}, None);
    }

    pub fn paint_chrome(&self, ctx: &mut DockRenderContext<'_>, bounds: Rect, body_fill: bool) {
        if let Some(path) = &self.maximized_stack {
            if let Some(node) = node_at(&self.root, path) {
                if let DockNode::Stack { .. } = node {
                    render_stack(self, ctx, path, node, bounds, true, body_fill, &mut |_, _| {});
                    return;
                }
            }
        }
        render_node(self, ctx, &self.root, bounds, &empty_path(), body_fill, &mut |_, _| {}, None);
    }

    pub fn register_resize_hits(&self, ctx: &mut DockRenderContext<'_>, bounds: Rect) {
        if self.maximized_stack.is_some() {
            return;
        }
        walk_resize_hits(self, ctx, &self.root, bounds, &empty_path(), None);
    }

    pub fn split_axis_extent(&self, path: &DockPath, canvas: Rect, separator: f32) -> Option<f32> {
        let bounds = solve_node_bounds(&self.root, canvas, path, &empty_path(), separator)?;
        match node_at(&self.root, path)? {
            DockNode::Row(children) => Some(axis_distributable_extent(bounds.w, children.len(), separator).max(1.0)),
            DockNode::Column(children) => Some(axis_distributable_extent(bounds.h, children.len(), separator).max(1.0)),
            DockNode::Stack { .. } => None,
        }
    }
}
//#endregion DockLayout
