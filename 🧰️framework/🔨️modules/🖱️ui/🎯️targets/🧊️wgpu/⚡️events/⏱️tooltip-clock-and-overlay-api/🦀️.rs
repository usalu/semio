//! ⏰️ Tooltip clock and overlay API: the retained caret, the clock deadlines and advance, the row reason hints and the overlay open, close and dismiss methods. See [`super`].

use super::*;

// #region events
//#region 🔖️UiCommand
impl EventRouter {
    //#region 🔖️TooltipClock
    fn set_retained_caret_visible(tree: &mut UiTree, node: NodeId, visible: bool) -> bool {
        let Some(record) = tree.node_mut(node) else { return false };
        if record.state.caret_visible == visible {
            return false;
        }
        record.state.caret_visible = visible;
        tree.mark_dirty(node, NodeFlags::DIRTY_PAINT);
        true
    }

    fn focused_editable(&self, tree: &UiTree) -> Option<NodeId> {
        self.focus.focused.filter(|node| tree.node(*node).is_some_and(|node| node.state.edit.is_some()))
    }

    pub fn synchronize_retained_caret(&mut self, tree: &mut UiTree, reset: bool) -> bool {
        let focused = self.focused_editable(tree);
        if focused.is_none() && self.caret.is_some_and(|caret| caret.source == CaretSource::Scene) {
            return false;
        }
        let same = self.caret.is_some_and(|caret| caret.source == CaretSource::Retained && Some(caret.node) == focused);
        if same && !reset {
            return false;
        }
        let mut changed = false;
        if let Some(previous) = self.caret.take() {
            if previous.source == CaretSource::Retained {
                changed |= Self::set_retained_caret_visible(tree, previous.node, false);
            }
        }
        let Some(node) = focused else { return changed };
        Self::set_retained_caret_visible(tree, node, true);
        self.caret = Some(CaretClock { node, source: CaretSource::Retained, visible: true, next_at: self.clock_seconds + CARET_BLINK_SECONDS });
        true
    }

    pub fn arm_scene_caret(&mut self, tree: &mut UiTree, node: NodeId, seconds: f64) -> Option<CaretStep> {
        if !seconds.is_finite() || seconds < self.clock_seconds || !tree.node(node).is_some_and(|node| matches!(node.spec.0, UiNode::ComponentScene(_))) {
            return None;
        }
        self.clock_seconds = seconds;
        if let Some(previous) = self.caret.take() {
            if previous.source == CaretSource::Retained {
                Self::set_retained_caret_visible(tree, previous.node, false);
            }
        }
        self.caret = Some(CaretClock { node, source: CaretSource::Scene, visible: true, next_at: seconds + CARET_BLINK_SECONDS });
        Some(CaretStep { node, source: CaretSource::Scene, visible: true })
    }

    pub fn clear_scene_caret(&mut self, node: NodeId) -> Option<CaretStep> {
        self.caret.filter(|caret| caret.source == CaretSource::Scene && caret.node == node)?;
        self.caret = None;
        Some(CaretStep { node, source: CaretSource::Scene, visible: false })
    }

    fn advance_caret(&mut self, tree: &mut UiTree, seconds: f64) -> (Option<CaretStep>, bool) {
        let Some(mut caret) = self.caret else { return (None, false) };
        if seconds < caret.next_at {
            return (None, false);
        }
        caret.visible = !caret.visible;
        caret.next_at = seconds + CARET_BLINK_SECONDS;
        self.caret = Some(caret);
        let state_changed = caret.source != CaretSource::Retained || Self::set_retained_caret_visible(tree, caret.node, caret.visible);
        (Some(CaretStep { node: caret.node, source: caret.source, visible: caret.visible }), state_changed)
    }

    /// ⏱️ Feeds one frame's monotonic seconds in and answers what hover reveal owes this frame —
    /// the immediate-mode stand-in for React's `setTimeout`/`clearTimeout` pair in
    /// `ChromeControlHint` (`🧱️elements/💡️ChromeControlHint/🟦️.tsx:44-57`). A non-monotonic value is
    /// ignored rather than rewinding every armed deadline.
    pub fn next_clock_deadline(&self) -> Option<f64> {
        let mut deadline = self.stepper_repeat.map(|repeat| repeat.next_at);
        if let Some(caret) = self.caret {
            deadline = Some(deadline.map_or(caret.next_at, |current| current.min(caret.next_at)));
        }
        if let Some(dismiss_at) = self.tooltip_dismiss_at {
            deadline = Some(deadline.map_or(dismiss_at, |current| current.min(dismiss_at)));
        }
        if !self.overlays.topmost().is_some_and(|overlay| overlay.kind == OverlayKind::Tooltip) {
            if let Some((_, entered)) = self.hover_since.filter(|(leaf, _)| self.hover_revealed != Some(*leaf)) {
                let reveal_at = entered + f64::from(TOOLTIP_DWELL_SECONDS);
                deadline = Some(deadline.map_or(reveal_at, |current| current.min(reveal_at)));
            }
        }
        deadline
    }

    pub fn suspend_clock(&mut self, tree: &mut UiTree) -> bool {
        let repeat = self.stepper_repeat.take().is_some();
        let hover = self.hover_since.take().is_some();
        let revealed = self.hover_revealed.take().is_some();
        let dismiss = self.tooltip_dismiss_at.take().is_some();
        let caret = self.caret.take();
        let caret_changed = caret.is_some_and(|caret| caret.source != CaretSource::Retained || Self::set_retained_caret_visible(tree, caret.node, false));
        repeat | hover | revealed | dismiss | caret_changed
    }

    pub fn revealed_tooltip_node(&self) -> Option<NodeId> {
        self.hover_revealed
    }

    /// 💬️ The row and action index whose reason hint is shown, if any.
    pub fn revealed_row_reason(&self) -> Option<(NodeId, usize)> {
        self.row_reason.map(|hint| (hint.row, hint.index))
    }

    /// 💡️ Shows `next`'s reason hint (or none), repainting the rows it leaves and enters; `true` when it changed.
    pub(super) fn set_row_reason(&mut self, tree: &mut UiTree, next: Option<RowReasonHint>) -> bool {
        let changed = self.revealed_row_reason() != next.map(|hint| (hint.row, hint.index));
        if changed {
            for row in [self.row_reason.map(|hint| hint.row), next.map(|hint| hint.row)].into_iter().flatten() {
                tree.mark_dirty(row, NodeFlags::DIRTY_PAINT);
            }
        }
        self.row_reason = next;
        changed
    }

    /// 🧠️ The hint of `row`'s `index`-th action when it is a disabled action naming its reason, else `None`.
    pub(super) fn row_reason_hint(tree: &UiTree, row: NodeId, index: usize, hovered: bool) -> Option<RowReasonHint> {
        disabled_row_action_reason(tree, row, index).map(|_| RowReasonHint { row, index, hovered })
    }

    /// 🏹️ The pointer at `(x, y)` over `target`: entering a disabled action's icon shows its reason, leaving the icon hides a hint
    /// the pointer revealed (a hint focus or a press revealed stays until blur or Escape).
    pub(super) fn hover_row_reason(&mut self, tree: &mut UiTree, target: Option<NodeId>, x: f32, y: f32) {
        let over = target.and_then(|row| self.pointer_row_action(tree, row, x, y).and_then(|index| Self::row_reason_hint(tree, row, index, true)));
        match over {
            Some(hint) => {
                let _ = self.set_row_reason(tree, Some(hint));
            }
            None if self.row_reason.is_some_and(|hint| hint.hovered) => {
                let _ = self.set_row_reason(tree, None);
            }
            None => {}
        }
    }

    pub fn advance_clock(&mut self, tree: &mut UiTree, seconds: f64) -> RouterClockStep {
        if !seconds.is_finite() || seconds < self.clock_seconds {
            return RouterClockStep { tooltip: TooltipStep::Idle, caret: None, commands: Vec::new(), changed: false, next_deadline: self.next_clock_deadline() };
        }
        self.clock_seconds = seconds;
        let (caret, caret_changed) = self.advance_caret(tree, seconds);
        let mut changed = caret_changed | self.advance_stepper_repeat(tree, seconds);
        if let Some(deadline) = self.tooltip_dismiss_at {
            if seconds >= deadline {
                self.tooltip_dismiss_at = None;
                changed = true;
                if self.overlays.topmost().is_some_and(|overlay| overlay.kind == OverlayKind::Tooltip) {
                    let commands = self.close_topmost_overlay(tree);
                    return RouterClockStep { tooltip: TooltipStep::Dismissed, caret, commands, changed, next_deadline: self.next_clock_deadline() };
                }
            }
            return RouterClockStep { tooltip: TooltipStep::Idle, caret, commands: Vec::new(), changed, next_deadline: self.next_clock_deadline() };
        }
        if self.overlays.topmost().is_some_and(|overlay| overlay.kind == OverlayKind::Tooltip) {
            return RouterClockStep { tooltip: TooltipStep::Idle, caret, commands: Vec::new(), changed, next_deadline: self.next_clock_deadline() };
        }
        let Some((leaf, entered)) = self.hover_since else {
            return RouterClockStep { tooltip: TooltipStep::Idle, caret, commands: Vec::new(), changed, next_deadline: self.next_clock_deadline() };
        };
        if self.hover_revealed == Some(leaf) || seconds < entered + TOOLTIP_DWELL_SECONDS {
            return RouterClockStep { tooltip: TooltipStep::Idle, caret, commands: Vec::new(), changed, next_deadline: self.next_clock_deadline() };
        }
        self.hover_revealed = Some(leaf);
        RouterClockStep { tooltip: TooltipStep::Reveal(leaf), caret, commands: Vec::new(), changed: true, next_deadline: self.next_clock_deadline() }
    }

    //#endregion 🔖️TooltipClock

    //#region 🔖️OverlayApi
    /// 🪟️ Opens an overlay: flags `root` `NodeFlags::OVERLAY` (hit-test priority — see 🔖️HitTest) and
    /// pushes it onto the z-ordered stack with `kind`'s default placement/dismissal policy.
    /// `Dialog`/`CommandPalette` become focus-trap scopes automatically.
    pub fn open_overlay(&mut self, tree: &mut UiTree, root: NodeId, kind: OverlayKind, anchor: OverlayAnchor) {
        if let Some(node) = tree.node_mut(root) {
            node.flags.set(NodeFlags::OVERLAY, true);
        }
        tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
        let focus_trap = matches!(kind, OverlayKind::Dialog | OverlayKind::CommandPalette);
        self.overlays.open(OpenOverlay { root, kind, anchor, placement: kind.default_placement(), dismiss: kind.dismiss_policy(), focus_trap });
    }

    pub fn close_overlay(&mut self, tree: &mut UiTree, root: NodeId) -> Vec<UiCommand> {
        match self.overlays.close_root(root) {
            Some(overlay) => self.finish_close(tree, overlay),
            None => Vec::new(),
        }
    }

    pub fn close_topmost_overlay(&mut self, tree: &mut UiTree) -> Vec<UiCommand> {
        match self.overlays.close_topmost() {
            Some(overlay) => self.finish_close(tree, overlay),
            None => Vec::new(),
        }
    }

    #[allow(dead_code, reason = "overlay-stack accessor, not yet called; likely wired by a later events-integration milestone")]
    pub fn topmost_overlay(&self) -> Option<&OpenOverlay> {
        self.overlays.topmost()
    }

    /// 🥞️ Every open overlay bottom-to-top — what `engine::Ui::open_overlays` publishes so a paint
    /// pass can place and draw them above the document.
    pub fn open_overlays(&self) -> &[OpenOverlay] {
        self.overlays.as_slice()
    }

    /// 🔽️ W2 wiring: the consumer-side effect of a `Select` click — flips `tree::WidgetState::open`
    /// via `open_overlay`/`close_overlay` (root *and* anchor are the `Select` node itself: its own
    /// synthesized item rows, see `reconcile::children_of`'s `Select` arm, are already its retained
    /// children, and marking the `Select` node `NodeFlags::OVERLAY` gives the whole popup hit-test
    /// priority over its own later-painted siblings). All dismissal paths (outside-press, `Escape`,
    /// an explicit `close_overlay`, or picking an item — see `dispatch`'s `PointerUp` handling) funnel
    /// through `finish_close`, which clears `open` back to `false` uniformly.
    pub fn toggle_select_popup(&mut self, tree: &mut UiTree, select_id: NodeId) -> Vec<UiCommand> {
        let already_open = tree.node(select_id).is_some_and(|node| node.state.open);
        if already_open {
            self.close_overlay(tree, select_id)
        } else {
            self.open_overlay(tree, select_id, OverlayKind::SelectPopup, OverlayAnchor::Node(select_id));
            if let Some(node) = tree.node_mut(select_id) {
                node.state.open = true;
            }
            Vec::new()
        }
    }

    /// 🧹️ Clears `NodeFlags::OVERLAY`, and clears focus too if it was inside the closed overlay's
    /// subtree (dangling focus into a now-hidden subtree would otherwise route key events nowhere
    /// useful). `SelectPopup`'s `tree::WidgetState::open` is the popup's own show/hide bit
    /// (`paint::paint_select` reads it) — cleared here too, so every dismissal path (see
    /// `toggle_select_popup`'s doc comment) stays in sync with the overlay lifecycle uniformly.
    fn finish_close(&mut self, tree: &mut UiTree, overlay: OpenOverlay) -> Vec<UiCommand> {
        if let Some(node) = tree.node_mut(overlay.root) {
            node.flags.set(NodeFlags::OVERLAY, false);
            if overlay.kind == OverlayKind::SelectPopup {
                node.state.open = false;
                node.state.highlighted = None;
                node.state.scroll_offset = (0.0, 0.0);
                node.state.select_popup = None;
            }
        }
        if overlay.kind == OverlayKind::Tooltip {
            self.tooltip_dismiss_at = None;
        }
        tree.mark_dirty(overlay.root, NodeFlags::DIRTY_PAINT);
        let mut out = vec![UiCommand::OverlayClosed { window_id: self.window_id.clone(), root: overlay.root, kind: overlay.kind }];
        if let Some(focused) = self.focus.focused {
            if is_descendant(tree, focused, overlay.root) {
                if overlay.kind == OverlayKind::SelectPopup {
                    if focused != overlay.root {
                        if let Some((blurred, fired)) = self.focus.set_focus(tree, Some(overlay.root), true) {
                            self.push_app_command(tree, blurred, fired, &mut out);
                        }
                        out.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: Some(overlay.root) });
                    }
                } else {
                    if let Some((blurred, fired)) = self.focus.clear_focus(tree) {
                        self.push_app_command(tree, blurred, fired, &mut out);
                    }
                    out.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: None });
                }
            }
        }
        out
    }

    /// 👆️ If the topmost overlay dismisses on outside-press and `(x, y)` lands outside its subtree,
    /// closes it and returns the resulting commands — the caller must swallow the press (not route it
    /// any further) when this returns `Some`.
    pub(super) fn dismiss_topmost_if_outside_press(&mut self, tree: &mut UiTree, x: f32, y: f32) -> Option<Vec<UiCommand>> {
        let top = self.overlays.topmost()?;
        if !top.dismiss.outside_press_swallow {
            return None;
        }
        let overlay_root = top.root;
        if self.hit_test_subtree(tree, overlay_root, x, y).is_some() {
            return None;
        }
        Some(self.close_topmost_overlay(tree))
    }

    /// 🖱️ `Tooltip`-only: arms (or disarms) the hover-out countdown as the pointer leaves or re-enters
    /// the anchor and the overlay's own bounds. `advance_clock` is what actually closes it once
    /// `DismissPolicy::hover_out_delay_seconds` has elapsed, matching React's `onPointerLeave` →
    /// `clearTimeout`/close pair rather than the immediate close this used to do.
    pub(super) fn maybe_dismiss_tooltip_on_hover_out(&mut self, tree: &mut UiTree, x: f32, y: f32) -> Vec<UiCommand> {
        let Some(top) = self.overlays.topmost() else { return Vec::new() };
        if top.kind != OverlayKind::Tooltip {
            return Vec::new();
        }
        let overlay_root = top.root;
        let anchor = top.anchor;
        let delay = top.dismiss.hover_out_delay_seconds;
        let inside_overlay = node_abs_rect(tree, overlay_root).is_some_and(|rect| rect.contains(x, y));
        let inside_anchor = match anchor {
            OverlayAnchor::Node(id) => node_abs_rect(tree, id).is_some_and(|rect| rect.contains(x, y)),
            OverlayAnchor::Point { .. } => false,
        };
        if inside_overlay || inside_anchor {
            self.tooltip_dismiss_at = None;
            return Vec::new();
        }
        match delay {
            Some(delay) => {
                self.tooltip_dismiss_at.get_or_insert(self.clock_seconds + f64::from(delay));
                Vec::new()
            }
            None => self.close_topmost_overlay(tree),
        }
    }
    //#endregion 🔖️OverlayApi

}
//#endregion 🔖️UiCommand
// #endregion events
