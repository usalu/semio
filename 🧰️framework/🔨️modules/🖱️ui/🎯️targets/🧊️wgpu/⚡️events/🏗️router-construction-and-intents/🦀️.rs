//! 🏗️ Router construction and intents: the `EventRouter` constructor, the interaction transfer, the close step, the flow and tree drag settings, the intent API and the hover updates. See [`super`].

use super::*;

// #region events
//#region 🔖️UiCommand
impl EventRouter {
    pub fn new(window_id: impl Into<String>) -> Self {
        Self {
            window_id: window_id.into(),
            capture: CaptureState::default(),
            focus: FocusState::new(),
            hovered: None,
            hover_chain: Vec::new(),
            press_origin: None,
            overlays: OverlayStack::new(),
            drag: None,
            tree_drag_driver: UiDriverDrag::Handle,
            tree_drag_metrics: TreeRowMetrics::from_theme(&semio_framework_ui_theme::Theme::default()),
            control_border: semio_framework_ui_theme::Theme::default().stroke_hairline,
            control_gap: semio_framework_ui_theme::Theme::default().gap_standard,
            tree_drag_handle_press: None,
            drag_payloads: BTreeMap::new(),
            drop_accept: BTreeMap::new(),
            scroll_thumbs: BTreeMap::new(),
            retiring_payload: None,
            thumb_start: None,
            clock_seconds: 0.0,
            hover_since: None,
            hover_revealed: None,
            row_reason: None,
            tooltip_dismiss_at: None,
            select_typeahead: None,
            stepper_repeat: None,
            caret: None,
            intents: UiIntentSequencer::default(),
            flow: UiFlow::DEFAULT,
            focus_visible: false,
        }
    }

    /// 🎞️ Rebinds presented interaction owners into a separately reconciled candidate tree. Only
    /// exact document identity, key, and widget-kind matches transfer; candidate registrations stay
    /// owned by the candidate paint.
    pub fn transfer_interaction_from(&mut self, presented: &EventRouter, presented_tree: &UiTree, candidate_tree: &UiTree) {
        let remap = |source: NodeId| candidate_tree.interaction_successor_from(presented_tree, source);
        self.capture.target = presented.capture.target.and_then(|(pointer, node, kind)| remap(node).map(|node| (pointer, node, kind)));
        self.focus.focused = presented.focus.focused.and_then(remap);
        self.focus.tab_order.clear();
        self.focus.tab_order.extend(presented.focus.tab_order.iter().filter_map(|node| remap(*node)));
        self.hovered = presented.hovered.and_then(remap);
        self.hover_chain.clear();
        self.hover_chain.extend(presented.hover_chain.iter().filter_map(|node| remap(*node)));
        self.press_origin = self.capture.target.and(presented.press_origin);
        self.overlays.open.clear();
        self.overlays.open.extend(presented.overlays.open.iter().filter_map(|overlay| {
            let root = remap(overlay.root)?;
            let anchor = match overlay.anchor {
                OverlayAnchor::Node(node) => OverlayAnchor::Node(remap(node)?),
                point => point,
            };
            Some(OpenOverlay { root, anchor, ..*overlay })
        }));
        self.drag = presented
            .drag
            .as_ref()
            .and_then(|drag| Some(DragSession { source: remap(drag.source)?, payload: drag.payload.clone(), ghost: drag.ghost.clone(), pointer_x: drag.pointer_x, pointer_y: drag.pointer_y, drop_target: drag.drop_target.and_then(remap) }));
        self.tree_drag_driver = presented.tree_drag_driver;
        self.tree_drag_metrics = presented.tree_drag_metrics;
        self.control_border = presented.control_border;
        self.control_gap = presented.control_gap;
        self.tree_drag_handle_press = presented.tree_drag_handle_press.and_then(remap);
        self.thumb_start = presented.thumb_start;
        self.clock_seconds = presented.clock_seconds;
        self.hover_since = presented.hover_since.and_then(|(node, at)| remap(node).map(|node| (node, at)));
        self.hover_revealed = presented.hover_revealed.and_then(remap);
        self.row_reason = presented.row_reason.and_then(|hint| remap(hint.row).map(|row| RowReasonHint { row, ..hint }));
        self.tooltip_dismiss_at = presented.tooltip_dismiss_at;
        self.select_typeahead = presented.select_typeahead.clone();
        self.stepper_repeat = presented.stepper_repeat.and_then(|repeat| remap(repeat.node).map(|node| StepperRepeat { node, ..repeat }));
        self.caret = presented.caret.and_then(|caret| remap(caret.node).map(|node| CaretClock { node, ..caret }));
        self.intents = presented.intents.clone();
        self.flow = presented.flow;
        self.focus_visible = presented.focus_visible;
    }

    /// 🧹️ Silently retires one input owner or storage scalar during surface unmount.
    pub fn close_step(&mut self) -> bool {
        if self.capture.release_any().is_some()
            || self.focus.focused.take().is_some()
            || self.hovered.take().is_some()
            || self.press_origin.take().is_some()
            || self.tree_drag_handle_press.take().is_some()
            || self.thumb_start.take().is_some()
            || self.hover_since.take().is_some()
            || self.hover_revealed.take().is_some()
            || self.row_reason.take().is_some()
            || self.tooltip_dismiss_at.take().is_some()
            || self.stepper_repeat.take().is_some()
            || self.caret.take().is_some()
        {
            return false;
        }
        if self.focus.tab_order.pop().is_some() || self.hover_chain.pop().is_some() || self.overlays.open.pop().is_some() {
            return false;
        }
        if self.focus.tab_order.capacity() > 0 {
            self.focus.tab_order = Vec::new();
            return false;
        }
        if self.hover_chain.capacity() > 0 {
            self.hover_chain = Vec::new();
            return false;
        }
        if self.overlays.open.capacity() > 0 {
            self.overlays.open = Vec::new();
            return false;
        }
        if let Some(payload) = self.retiring_payload.as_mut() {
            if payload.close_step() {
                self.retiring_payload = None;
            }
            return false;
        }
        if let Some(drag) = self.drag.as_mut() {
            if !drag.payload.is_empty() || drag.payload.capacity() > 0 {
                self.retiring_payload = Some(RetiringDragPayload::new(std::mem::take(&mut drag.payload)));
                return false;
            }
            if let Some(ghost) = drag.ghost.as_mut() {
                if ghost.label.pop().is_some() {
                    return false;
                }
                if ghost.label.capacity() > 0 {
                    ghost.label = String::new();
                    return false;
                }
                drag.ghost = None;
                return false;
            }
            self.drag = None;
            return false;
        }
        if let Some((_, payload)) = self.drag_payloads.pop_first() {
            self.retiring_payload = Some(RetiringDragPayload::new(payload));
            return false;
        }
        if self.drop_accept.pop_first().is_some() || self.scroll_thumbs.pop_first().is_some() {
            return false;
        }
        if let Some((query, _)) = self.select_typeahead.as_mut() {
            if query.pop().is_some() {
                return false;
            }
            if query.capacity() > 0 {
                *query = String::new();
                return false;
            }
            self.select_typeahead = None;
            return false;
        }
        if !self.intents.close_step() {
            return false;
        }
        if self.window_id.pop().is_some() {
            return false;
        }
        if self.window_id.capacity() > 0 {
            self.window_id = String::new();
            return false;
        }
        true
    }

    pub(super) fn toggle_disclosure(&mut self, tree: &mut UiTree, id: NodeId) -> bool {
        tree.toggle_disclosure(id).is_some()
    }

    /// 🖱️ A press anywhere on an expandable row folds it. The chevron paints that state; it is the
    /// same control, not a separate gate. `toggle_disclosure` already refuses a row with no children.
    pub(super) fn pointer_toggle_disclosure(&mut self, tree: &mut UiTree, id: NodeId, _x: f32, _y: f32) -> bool {
        self.toggle_disclosure(tree, id)
    }

    /// 🧭️ This window's logical flow.
    pub fn flow(&self) -> UiFlow {
        self.flow
    }

    /// 🧭️ This window's inline direction — the `rtl` argument every mirrored formula takes.
    pub fn flow_inline(&self) -> FlowInline {
        self.flow.inline
    }

    /// 🧭️ Replaces this window's flow. Answers whether it changed, so the caller can invalidate the
    /// layout generation the way `Ui::set_viewport` does for metrics.
    pub fn set_flow(&mut self, flow: UiFlow) -> bool {
        if self.flow == flow {
            return false;
        }
        self.flow = flow;
        self.tree_drag_metrics = self.tree_drag_metrics.with_inline(flow.inline);
        true
    }

    /// ⌨️ Whether a focus ring should paint — React's `focus-visible`. True only after a KEY moved or
    /// activated focus; any pointer press clears it.
    pub fn focus_visible(&self) -> bool {
        self.focus_visible
    }

    pub fn set_control_border(&mut self, border: f32) {
        self.control_border = border.max(0.0);
    }

    pub fn set_control_gap(&mut self, gap: f32) {
        self.control_gap = gap.max(0.0);
    }

    pub fn set_tree_drag_policy(&mut self, tree: &mut UiTree, driver: UiDriverDrag, metrics: TreeRowMetrics) -> Vec<UiCommand> {
        self.tree_drag_metrics = metrics;
        if self.tree_drag_driver == driver {
            return Vec::new();
        }
        self.tree_drag_driver = driver;
        self.cancel_tree_drag(tree).into_iter().collect()
    }

    fn cancel_tree_drag(&mut self, tree: &mut UiTree) -> Option<UiCommand> {
        let (_, source, kind) = self.capture.target?;
        if find_tree_item_spec(tree, source).and_then(tree_drag_role).is_none() {
            return None;
        }
        let _ = self.capture.release_any();
        self.press_origin = None;
        self.tree_drag_handle_press = None;
        self.drag_payloads.remove(&source);
        if let Some(node) = tree.node_mut(source) {
            node.flags.set(NodeFlags::ACTIVE, false);
        }
        tree.mark_dirty(source, NodeFlags::DIRTY_PAINT);
        if kind != CaptureKind::Drag {
            self.drag = None;
            return None;
        }
        let drag = self.drag.take()?;
        Some(UiCommand::DropCancelled { window_id: self.window_id.clone(), source: drag.source })
    }

    pub(super) fn tree_drag_press_arms(&self, tree: &UiTree, row: NodeId, x: f32, y: f32) -> bool {
        let Some(item) = find_tree_item_spec(tree, row) else { return false };
        if item.presence.state == UiState::Disabled || tree_drag_role(item).is_none() {
            return false;
        }
        if self.tree_drag_driver == UiDriverDrag::Surface {
            return true;
        }
        let Some(row_rect) = node_abs_rect(tree, row) else { return false };
        let metrics = semio_framework_ui_wgpu_mounted_layout::retained_tree_row_metrics(tree, row, &self.tree_drag_metrics);
        let relative = tree_drag_handle_rect(row_rect.w, &metrics);
        Rect::new(row_rect.x + relative.x, row_rect.y + relative.y, relative.w, relative.h).contains(x, y)
    }

    //#region 🎬️IntentApi
    /// 🎬️ Turns one fired gesture into an addressed [`UiIntentCommand`] — the wgpu twin of React's
    /// `dispatchTrigger` → `emitIntent` → `UiDocumentStore::buildIntent` chain.
    ///
    /// Three things happen here and nowhere else: the node's stamped address supplies
    /// `surface`/`revision`/`node`/`node_key`; a gesture whose recorded revision is already more than
    /// one behind the document it is being resolved against is DROPPED (`intent_is_stale` — React's
    /// own `is_stale`, so a click on geometry the user never saw cannot misapply); and the surface's
    /// `seq` is minted. A node the document never published (the chrome's immediate-mode widgets, the
    /// testkit's `apply_tree`) has no address and no revision to be stale against — it still fires,
    /// with a default address, which is exactly the unowned-element channel React keeps `onAction` for.
    fn build_intent(&mut self, tree: &UiTree, node: NodeId, fired: FiredAction) -> Option<UiIntentCommand> {
        let bindings = tree.node(node).and_then(|node| node.intent.clone());
        let current_revision = tree.document().map_or(0, |document| document.revision().0);
        let (address, action) = match bindings {
            Some(bindings) => {
                if intent_is_stale(bindings.address.revision, current_revision) {
                    return None;
                }
                let action = match bindings.action_for(fired.trigger) {
                    Some(action) => action.clone(),
                    None => descriptor_action_id(&fired.action)?,
                };
                (bindings.address, action)
            }
            None => (UiIntentAddress::default(), descriptor_action_id(&fired.action)?),
        };
        let seq = self.intents.next(&address.surface);
        Some(UiIntentCommand { address, trigger: fired.trigger, action, args: fired.action.args, input: fired.input, seq })
    }

    /// 🎬️ [`Self::build_intent`] wrapped as the command the host drains. `None` when the gesture was
    /// dropped as stale or could not be addressed.
    pub(super) fn app_command(&mut self, tree: &UiTree, node: NodeId, fired: FiredAction) -> Option<UiCommand> {
        Some(UiCommand::App { window_id: self.window_id.clone(), intent: self.build_intent(tree, node, fired)? })
    }

    /// 🎬️ [`Self::app_command`] for a caller that already owns the command list.
    pub(super) fn push_app_command(&mut self, tree: &UiTree, node: NodeId, fired: FiredAction, out: &mut Vec<UiCommand>) {
        if let Some(command) = self.app_command(tree, node, fired) {
            out.push(command);
        }
    }

    /// 🎬️ A row action's intent: the row's own address with the ACTION's versioned id — its verb on the row's ONE target
    /// (`mounted_layout::document_row_action_id`) — `build_intent` would answer the row's activation, its primary action, for
    /// the same trigger.
    pub(super) fn row_action_command(&mut self, tree: &UiTree, row: NodeId, index: usize) -> Option<UiCommand> {
        let action = semio_framework_ui_wgpu_mounted_layout::document_row_action_id(tree, row, index)?.ok()?;
        let args = tree.authored_tree_item(row)?.actions.as_deref()?.get(index)?.action.args.clone();
        let current_revision = tree.document().map_or(0, |document| document.revision().0);
        let address = match tree.node(row)?.intent.as_ref() {
            Some(bindings) if intent_is_stale(bindings.address.revision, current_revision) => return None,
            Some(bindings) => bindings.address.clone(),
            None => UiIntentAddress::default(),
        };
        let seq = self.intents.next(&address.surface);
        Some(UiCommand::App { window_id: self.window_id.clone(), intent: UiIntentCommand { address, trigger: Trigger::Activate, action, args, input: None, seq } })
    }

    /// 🎬️ The index of the Row-placed action whose trailing slot (`layout::tree_row_action_at`, the slots `paint` draws) holds the
    /// pointer on a tree or table row — a click there fires that action, never the row's activation or disclosure.
    pub(super) fn pointer_row_action(&self, tree: &UiTree, id: NodeId, x: f32, y: f32) -> Option<usize> {
        let item = tree.authored_tree_item(id)?;
        let metrics = semio_framework_ui_wgpu_mounted_layout::retained_tree_row_metrics(tree, id, &self.tree_drag_metrics);
        let band = tree_section_header_band(tree.absolute_rect(id)?, metrics.row_height, self.flow.block.is_reversed());
        let trailing = if self.tree_drag_driver == UiDriverDrag::Handle && tree_drag_role(item).is_some() { tree_drag_handle_reservation(&metrics) } else { 0.0 };
        semio_framework_ui_wgpu_layout::layout::tree_row_action_at(item, band.w, band.h, trailing, &metrics, x - band.x, y - band.y)
    }
    //#endregion 🎬️IntentApi

    pub(super) fn resolve_target(&self, tree: &UiTree, root: NodeId, x: f32, y: f32) -> Option<NodeId> {
        match self.capture.target {
            Some((_, id, _)) => Some(id),
            None => self.overlays.topmost().and_then(|overlay| self.hit_test_subtree(tree, overlay.root, x, y)).or_else(|| self.hit_test(tree, root, x, y)),
        }
    }

    pub fn hit_test(&self, tree: &UiTree, root: NodeId, x: f32, y: f32) -> Option<NodeId> {
        hit_test_node(tree, root, 0.0, 0.0, x, y, self.flow.block.is_reversed(), Some(&self.tree_drag_metrics))
    }

    pub(super) fn hit_test_subtree(&self, tree: &UiTree, subtree_root: NodeId, x: f32, y: f32) -> Option<NodeId> {
        let rect = tree.absolute_rect(subtree_root)?;
        let layout = tree.accepted_layout(subtree_root)?;
        hit_test_node(tree, subtree_root, rect.x - layout.x, rect.y - layout.y, x, y, self.flow.block.is_reversed(), Some(&self.tree_drag_metrics))
    }

    /// 👆️ Flips `NodeFlags::HOVERED` off every node in the old hover bubble chain that isn't in the
    /// new one, and on for every new node that wasn't in the old one — see `hover_chain`'s own doc
    /// comment for why the whole chain (not just the leaf) carries the flag. ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: the per-row `hover_action`/
    /// `unhover_action` dispatch this used to fire is deleted — a `Tree`'s hover now flows through its
    /// `UiTreeNode.interaction_domain` binding (`interactionHover`) instead of an ad hoc per-item action.
    pub(super) fn update_hover(&mut self, tree: &mut UiTree, target: Option<NodeId>) -> Vec<UiCommand> {
        let commands = Vec::new();
        if self.hovered == target {
            return commands;
        }
        let mut new_chain = Vec::new();
        if let Some(leaf) = target {
            bubble(tree, leaf, |id| {
                new_chain.push(id);
                false
            });
        }
        for &previous in &self.hover_chain {
            if !new_chain.contains(&previous) {
                if let Some(node) = tree.node_mut(previous) {
                    node.flags.set(NodeFlags::HOVERED, false);
                }
                tree.mark_dirty(previous, NodeFlags::DIRTY_PAINT);
            }
        }
        for &next in &new_chain {
            if !self.hover_chain.contains(&next) {
                if let Some(node) = tree.node_mut(next) {
                    node.flags.set(NodeFlags::HOVERED, true);
                }
                tree.mark_dirty(next, NodeFlags::DIRTY_PAINT);
            }
        }
        self.hover_chain = new_chain;
        self.hovered = target;
        self.hover_since = target.map(|leaf| (leaf, self.clock_seconds));
        self.hover_revealed = None;
        commands
    }

    pub(super) fn update_stepper_hover_segment(&mut self, tree: &mut UiTree, target: Option<NodeId>, x: f32, y: f32) {
        if let Some(previous) = self.hovered {
            if let Some(node) = tree.node_mut(previous) {
                if matches!(node.spec.0, UiNode::NumberStepper(_)) {
                    node.state.stepper_hovered_segment = None;
                    tree.mark_dirty(previous, NodeFlags::DIRTY_PAINT);
                }
            }
        }
        let Some(target) = target else { return };
        let Some(bounds) = absolute_rect(tree, target) else { return };
        let Some(node) = tree.node_mut(target) else { return };
        if !matches!(node.spec.0, UiNode::NumberStepper(_)) || node.spec.0.presence().state == UiState::Disabled {
            return;
        }
        node.state.stepper_hovered_segment = Some(number_stepper_sign_at(bounds, x, y, self.flow.inline, self.control_border).map_or(0, |sign| if sign < 0.0 { -1 } else { 1 }));
        tree.mark_dirty(target, NodeFlags::DIRTY_PAINT);
    }

}
//#endregion 🔖️UiCommand
// #endregion events
