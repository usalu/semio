//! 🖱️ Accessibility and pointer dispatch: the cursor queries, the accessibility dispatch and the pointer dispatch. See [`super`].

use super::*;

// #region events
//#region 🔖️UiCommand
impl EventRouter {
    //#region 🔖️CursorApi
    #[allow(dead_code, reason = "cursor-state accessor, not yet called; likely wired by a later events-integration milestone")]
    pub fn hovered(&self) -> Option<NodeId> {
        self.hovered
    }

    /// 🖱️ What this window's retained content currently holds pointer capture on — read by
    /// `engine::Ui::window_with_pointer_capture` so a host keeps feeding a live drag its moves.
    pub fn capture(&self) -> Option<(NodeId, CaptureKind)> {
        self.capture.target.map(|(_, node, kind)| (node, kind))
    }

    pub fn capture_pointer_id(&self) -> Option<u64> {
        self.capture.target.map(|(pointer_id, _, _)| pointer_id)
    }

    /// 🎯️ Read-only: whether this window's retained content currently holds keyboard focus — see
    /// `engine::Ui::window_has_focus` (its only caller), added for the `w2-input-wiring` host-side
    /// focus arbitration (content vs. chrome routing, `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`).
    pub fn is_focused(&self) -> bool {
        self.focus.focused.is_some()
    }
    //#endregion 🔖️CursorApi

    /// 🧹️ Drops registry entries (`drag_payloads`/`drop_accept`/`scroll_thumbs`) keyed by a `NodeId`
    /// `reconcile` has since removed from `tree` — generation-tagged `NodeId`s (see `arena`'s own doc
    /// comment) make stale entries harmless to *use* (they simply never match a live node again), but
    /// this keeps the maps from growing unboundedly across a long session's worth of churn.
    fn prune_dead_registrations(&mut self, tree: &UiTree) {
        self.drag_payloads.retain(|id, _| tree.contains(*id));
        self.drop_accept.retain(|id, _| tree.contains(*id));
        self.scroll_thumbs.retain(|thumb, (scrollable, _)| tree.contains(*thumb) && tree.contains(*scrollable));
        if self.stepper_repeat.is_some_and(|repeat| !tree.contains(repeat.node)) {
            self.stepper_repeat = None;
        }
    }

    /// 🚦️ Resolves the event's target (capture target if captured, else `hit_test`), updates
    /// interaction flags, and returns any `UiCommand`s the event produced.
    ///
    /// 🔽️ A focused `Select` answers navigation/typeahead/commit keys before the generic
    /// control routing below, exactly as React's `SelectTrigger`/`SelectContent` handlers
    /// claim them ahead of the browser's own default (`🧱️elements/🔽️Select/🟦️.tsx`).
    /// `Escape` stays with the overlay stack and `Tab` still moves focus after the popup
    /// this closes, so neither of those two is ever reported as consumed.
    pub fn dispatch_accessibility(&mut self, tree: &mut UiTree, target: NodeId, event: &AccessibilityUiEvent) -> Vec<UiCommand> {
        self.prune_dead_registrations(tree);
        let mut commands = Vec::new();
        let enabled = tree.node(target).is_some_and(|node| node.spec.0.presence().state != UiState::Disabled);
        if !enabled {
            return commands;
        }
        if !matches!(event, AccessibilityUiEvent::Blur) && node_is_focusable(tree, target) {
            if let Some((blurred, fired)) = self.focus.set_focus(tree, Some(target), true) {
                self.push_app_command(tree, blurred, fired, &mut commands);
            }
            commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: Some(target) });
            self.focus_visible = true;
        }
        match event {
            AccessibilityUiEvent::Focus => {}
            AccessibilityUiEvent::Blur => {
                if tree.node(target).is_some_and(|node| matches!(node.spec.0, UiNode::Slider(_) | UiNode::Ring(_))) {
                    if let Some(fired) = cancelled_press(tree, target, "blur") {
                        self.push_app_command(tree, target, fired, &mut commands);
                    }
                }
                if self.focus.focused == Some(target) {
                    if let Some((blurred, fired)) = self.focus.clear_focus(tree) {
                        self.push_app_command(tree, blurred, fired, &mut commands);
                    }
                    commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: None });
                }
            }
            AccessibilityUiEvent::Activate => {
                let kind = tree.node(target).map(|node| node.spec.0.clone());
                match kind {
                    _ if self.toggle_disclosure(tree, target) => {}
                    Some(UiNode::Select(_)) => commands.extend(self.toggle_select_popup(tree, target)),
                    Some(UiNode::Button(button)) => {
                        if let Some(fired) = bare_action(&button.action, Trigger::Activate) {
                            self.push_app_command(tree, target, fired, &mut commands);
                        }
                    }
                    Some(UiNode::Stack(stack)) => {
                        if let Some(fired) = stack.activate.as_ref().and_then(|action| bare_action(action, Trigger::Activate)) {
                            self.push_app_command(tree, target, fired, &mut commands);
                        }
                    }
                    Some(UiNode::Toggle(toggle)) => {
                        if let Some(fired) = fired_action(&toggle.on_change, Trigger::Change, DslValue::Bool(!toggle.presence.selected)) {
                            self.push_app_command(tree, target, fired, &mut commands);
                        }
                    }
                    _ => {}
                }
            }
            AccessibilityUiEvent::Value(value) => {
                let mirrored = match tree.node(target).map(|node| &node.spec.0) {
                    Some(UiNode::Input(_) | UiNode::IconSelect(_) | UiNode::NumberStepper(_)) => MirroredValue::Buffer,
                    Some(UiNode::Slider(_)) => MirroredValue::Readout,
                    Some(UiNode::Select(select)) => MirroredValue::Fired(fired_action(&select.on_change, Trigger::Change, DslValue::String(value.clone()))),
                    Some(UiNode::Toggle(toggle)) => MirroredValue::Fired(value.parse::<bool>().ok().and_then(|value| fired_action(&toggle.on_change, Trigger::Change, DslValue::Bool(value)))),
                    Some(UiNode::Ring(ring)) => MirroredValue::Fired(value.parse::<f64>().ok().map(|value| value.clamp(0.0, 1.0)).and_then(|value| fired_action(&ring.on_change, Trigger::Change, DslValue::float(value)))),
                    _ => MirroredValue::Fired(None),
                };
                match mirrored {
                    MirroredValue::Buffer => {
                        self.write_edit_buffer(tree, target, value);
                        self.push_buffer_change(tree, &mut commands);
                    }
                    MirroredValue::Readout => {
                        self.write_edit_buffer(tree, target, value);
                        if let Some(fired) = self.finish_slider_readout_edit(tree, target) {
                            self.push_app_command(tree, target, fired, &mut commands);
                        }
                    }
                    MirroredValue::Fired(Some(fired)) => self.push_app_command(tree, target, fired, &mut commands),
                    MirroredValue::Fired(None) => {}
                }
            }
        }
        commands
    }

    pub fn dispatch_accessibility_select_option(&mut self, tree: &mut UiTree, target: NodeId, value: &str, event: &AccessibilityUiEvent) -> Vec<UiCommand> {
        let Some(node) = tree.node(target) else { return Vec::new() };
        let UiNode::Select(select) = &node.spec.0 else { return Vec::new() };
        if select.presence.state == UiState::Disabled || (!node.state.open && !select_is_segmented(tree, target)) || !select.items.iter().any(|item| item.value == value) || !matches!(event, AccessibilityUiEvent::Activate) {
            return Vec::new();
        }
        let on_change = select.on_change.clone();
        let mut commands = Vec::new();
        if let Some(fired) = fired_action(&on_change, Trigger::Change, DslValue::String(value.to_string())) {
            self.push_app_command(tree, target, fired, &mut commands);
        }
        commands.extend(self.close_overlay(tree, target));
        commands
    }

    /// ♿️ The accessibility mirror's activation of a row's `index`-th action (`accessibility::row_accessibility_action`), fired
    /// exactly as a pointer on its trailing icon fires it.
    /// A disabled action's reason hint shows on its focus and on its activation (which dispatches nothing) and hides on its blur.
    pub fn dispatch_accessibility_row_action(&mut self, tree: &mut UiTree, target: NodeId, index: usize, event: &AccessibilityUiEvent) -> Vec<UiCommand> {
        match event {
            AccessibilityUiEvent::Focus | AccessibilityUiEvent::Activate if disabled_row_action_reason(tree, target, index).is_some() => {
                let _ = self.set_row_reason(tree, Self::row_reason_hint(tree, target, index, false));
                return Vec::new();
            }
            AccessibilityUiEvent::Blur if self.revealed_row_reason() == Some((target, index)) => {
                let _ = self.set_row_reason(tree, None);
                return Vec::new();
            }
            _ => {}
        }
        let enabled = tree.node(target).is_some_and(|node| node.spec.0.presence().state != UiState::Disabled);
        if !enabled || !matches!(event, AccessibilityUiEvent::Activate) {
            return Vec::new();
        }
        self.row_action_command(tree, target, index).into_iter().collect()
    }

    /// 💬️ The absolute rect of `row`'s `index`-th action icon — the slot `paint` draws it in and [`Self::pointer_row_action`] hits —
    /// where its reason hint anchors.
    pub fn row_action_icon_rect(&self, tree: &UiTree, row: NodeId, index: usize) -> Option<Rect> {
        let item = tree.authored_tree_item(row)?;
        let slot = item.actions.as_deref()?.len().checked_sub(index)?;
        let metrics = semio_framework_ui_wgpu_mounted_layout::retained_tree_row_metrics(tree, row, &self.tree_drag_metrics);
        let band = tree_section_header_band(tree.absolute_rect(row)?, metrics.row_height, self.flow.block.is_reversed());
        let trailing = if self.tree_drag_driver == UiDriverDrag::Handle && tree_drag_role(item).is_some() { tree_drag_handle_reservation(&metrics) } else { 0.0 };
        let icon = semio_framework_ui_wgpu_layout::layout::tree_row_action_rect(band.w, band.h, slot, trailing, &metrics);
        Some(Rect::new(band.x + icon.x, band.y + icon.y, icon.w, icon.h))
    }

    /// 🎛️ The mirror's `<key>::editor` spinbutton of an enabled slider — its typed readout, which a canvas user opens by a
    /// double-click: a focus opens the readout draft on the shown value, a value replaces the draft as typed (never clamped,
    /// never committed), a blur drops an unrefused draft; `Enter` commits or refuses and `Escape` reverts through the same key
    /// route the canvas takes ([`Self::finish_slider_readout_edit`]).
    pub fn dispatch_accessibility_slider_editor(&mut self, tree: &mut UiTree, target: NodeId, event: &AccessibilityUiEvent) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        let readout = tree.node(target).and_then(|node| match &node.spec.0 {
            UiNode::Slider(slider) if node.spec.0.presence().state != UiState::Disabled => Some(slider.readout(slider_live_value(node, slider))),
            _ => None,
        });
        let Some(readout) = readout else { return commands };
        if matches!(event, AccessibilityUiEvent::Focus | AccessibilityUiEvent::Value(_)) {
            if let Some((blurred, fired)) = self.focus.set_focus(tree, Some(target), true) {
                self.push_app_command(tree, blurred, fired, &mut commands);
            }
            commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: Some(target) });
        }
        match event {
            AccessibilityUiEvent::Focus => {
                if tree.node(target).is_some_and(|node| node.state.edit.is_none()) {
                    self.write_edit_buffer(tree, target, &readout);
                }
            }
            AccessibilityUiEvent::Blur => {
                if self.focus.focused == Some(target) {
                    let _ = self.focus.clear_focus(tree);
                    commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: None });
                }
            }
            AccessibilityUiEvent::Value(value) => self.write_edit_buffer(tree, target, value),
            AccessibilityUiEvent::Activate => {}
        }
        commands
    }

    pub fn dispatch(&mut self, tree: &mut UiTree, root: NodeId, event: &UiEvent) -> Vec<UiCommand> {
        self.dispatch_pointer(tree, root, 1, event)
    }

    pub fn dispatch_pointer(&mut self, tree: &mut UiTree, root: NodeId, pointer_id: u64, event: &UiEvent) -> Vec<UiCommand> {
        if matches!(event, UiEvent::PointerCancel | UiEvent::PointerDown { .. } | UiEvent::PointerUp { .. } | UiEvent::PointerMove { .. }) && self.capture.target.is_some_and(|(owner, _, _)| owner != pointer_id) {
            return Vec::new();
        }
        self.prune_dead_registrations(tree);
        let mut commands = Vec::new();
        match event {
            UiEvent::PointerCancel => {
                self.stepper_repeat = None;
                if let Some((id, _)) = self.capture.release(pointer_id) {
                    if let Some(node) = tree.node_mut(id) {
                        node.flags.set(NodeFlags::ACTIVE, false);
                    }
                    tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
                    if let Some(fired) = cancelled_press(tree, id, "captureLost") {
                        self.push_app_command(tree, id, fired, &mut commands);
                    }
                }
                if let Some(drag) = self.drag.take() {
                    commands.push(UiCommand::DropCancelled { window_id: self.window_id.clone(), source: drag.source });
                }
                self.press_origin = None;
                self.thumb_start = None;
                self.tree_drag_handle_press = None;
                self.update_stepper_hover_segment(tree, None, 0.0, 0.0);
                commands.extend(self.update_hover(tree, None));
                let _ = self.set_row_reason(tree, None);
            }
            UiEvent::PointerMove { x, y, .. } => {
                if self.stepper_repeat.is_some_and(|repeat| absolute_rect(tree, repeat.node).is_none_or(|bounds| number_stepper_sign_at(bounds, *x, *y, self.flow.inline, self.control_border) != Some(repeat.sign))) {
                    self.stepper_repeat = None;
                }
                self.maybe_promote_to_drag(*x, *y);
                match self.capture.target {
                    Some((_, _, CaptureKind::Drag)) => self.update_drag(tree, root, *x, *y),
                    Some((_, scrollable, CaptureKind::ScrollThumb(axis))) => self.update_scroll_thumb(tree, scrollable, axis, *x, *y),
                    // 🎚️ A captured `Slider`/`Ring` reports EVERY intermediate value, not only the
                    // release — Radix's own behaviour, which React's `SliderView`/`RingView` delegate to.
                    Some((_, pressed, CaptureKind::Press)) => {
                        if tree.node(pressed).is_some_and(|node| commits_while_dragging(&node.spec.0)) {
                            commands.extend(self.pointer_commit(tree, pressed, *x, *y));
                        }
                    }
                    _ => {}
                }
                let target = self.resolve_target(tree, root, *x, *y);
                if let Some(id) = target {
                    if let Some(cmd) = self.scene_command(tree, id, event) {
                        commands.push(cmd);
                    }
                }
                self.update_stepper_hover_segment(tree, target, *x, *y);
                commands.extend(self.update_hover(tree, target));
                self.hover_row_reason(tree, target, *x, *y);
                commands.extend(self.maybe_dismiss_tooltip_on_hover_out(tree, *x, *y));
            }
            UiEvent::PointerDown { x, y, button, .. } => {
                self.focus_visible = false;
                self.tree_drag_handle_press = None;
                if let Some(dismissed) = self.dismiss_topmost_if_outside_press(tree, *x, *y) {
                    return dismissed;
                }
                self.press_origin = Some((*x, *y));
                let scroll_target = self
                    .overlays
                    .topmost()
                    .filter(|overlay| overlay.kind == OverlayKind::SelectPopup)
                    .and_then(|overlay| tree.node(overlay.root).and_then(|node| node.state.select_popup).and_then(|popup| select::select_scroll_direction_at(popup, *x, *y)).map(|_| overlay.root));
                let target = scroll_target.or_else(|| self.overlays.topmost().and_then(|overlay| self.hit_test_subtree(tree, overlay.root, *x, *y))).or_else(|| self.hit_test(tree, root, *x, *y));
                self.update_stepper_hover_segment(tree, target, *x, *y);
                commands.extend(self.update_hover(tree, target));
                if let Some(hint) = target.and_then(|row| self.pointer_row_action(tree, row, *x, *y).and_then(|index| Self::row_reason_hint(tree, row, index, true))) {
                    let _ = self.set_row_reason(tree, Some(hint));
                }
                if let Some(id) = target {
                    if let Some(cmd) = self.scene_command(tree, id, event) {
                        commands.push(cmd);
                    }
                    let disabled_stepper_side = tree.node(id).is_some_and(|node| {
                        let UiNode::NumberStepper(stepper) = &node.spec.0 else { return false };
                        absolute_rect(tree, id).and_then(|bounds| number_stepper_sign_at(bounds, *x, *y, self.flow.inline, self.control_border)).is_some_and(|sign| !number_stepper_can_step(node, stepper, sign))
                    });
                    if disabled_stepper_side {
                        return commands;
                    }
                    if let Some(&(scrollable, axis)) = self.scroll_thumbs.get(&id) {
                        let offset = tree.node(scrollable).map(|node| node.state.scroll_offset).unwrap_or_default();
                        self.capture.target = Some((pointer_id, scrollable, CaptureKind::ScrollThumb(axis)));
                        self.thumb_start = Some((*x, *y, offset.0, offset.1));
                    } else {
                        if let Some(node) = tree.node_mut(id) {
                            node.flags.set(NodeFlags::ACTIVE, true);
                        }
                        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
                        self.capture.target = Some((pointer_id, id, CaptureKind::Press));
                        let focusable = node_is_focusable(tree, id);
                        if focusable {
                            if let Some((blurred, fired)) = self.focus.set_focus(tree, Some(id), false) {
                                self.push_app_command(tree, blurred, fired, &mut commands);
                            }
                            commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: Some(id) });
                        } else if self.focus.focused.is_some() {
                            if let Some((blurred, fired)) = self.focus.clear_focus(tree) {
                                self.push_app_command(tree, blurred, fired, &mut commands);
                            }
                            commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: None });
                        }
                        // 🫳️ W2 wiring: a `Tree` row's `draggable`/`drag_data` (re-derived by key —
                        // see `find_tree_item_spec`) registers this press as a promotable drag
                        // source, exactly like a widget spec would call `set_drag_payload` itself if
                        // `UiStackNode` had room for the field (it doesn't — see that fn's own doc).
                        if let Some(item) = find_tree_item_spec(tree, id) {
                            let arms = self.tree_drag_press_arms(tree, id, *x, *y);
                            let payload = arms.then(|| item.drag_data.clone().unwrap_or_default());
                            if let Some(payload) = payload {
                                self.set_drag_payload(id, payload);
                                if self.tree_drag_driver == UiDriverDrag::Handle {
                                    self.tree_drag_handle_press = Some(id);
                                }
                            } else {
                                self.drag_payloads.remove(&id);
                            }
                        }
                        if tree.node(id).is_some_and(|node| (commits_while_dragging(&node.spec.0) || (*button == PointerButton::Primary && matches!(node.spec.0, UiNode::NumberStepper(_)))) && node.spec.0.presence().state != UiState::Disabled) {
                            self.arm_stepper_repeat(tree, id, *x, *y);
                            commands.extend(self.pointer_commit(tree, id, *x, *y));
                        }
                    }
                } else {
                    if let Some((blurred, fired)) = self.focus.clear_focus(tree) {
                        self.push_app_command(tree, blurred, fired, &mut commands);
                    }
                    commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: None });
                }
            }
            UiEvent::PointerUp { x, y, button, .. } => {
                self.stepper_repeat = None;
                let captured_scene = self.capture.target.and_then(|(_, id, _)| self.scene_command(tree, id, event).map(|command| (id, command)));
                if let Some((active_id, kind)) = self.capture.release(pointer_id) {
                    match kind {
                        CaptureKind::Press => {
                            let released = tree.node(active_id).and_then(|node| match &node.spec.0 {
                                UiNode::Slider(slider) if node.state.scrub_gesture.is_some() => fired_action(&slider.on_change, Trigger::Change, DslValue::float(slider_live_value(node, slider))),
                                UiNode::Ring(ring) if node.state.scrub_gesture.is_some() => absolute_rect(tree, active_id).and_then(|bounds| fired_action(&ring.on_change, Trigger::Change, DslValue::float(ring_t_at(bounds, *x, *y)))),
                                _ => None,
                            });
                            if let Some(fired) = released {
                                let fired = pressed(tree, &self.window_id, active_id, fired, true);
                                self.push_app_command(tree, active_id, fired, &mut commands);
                            }
                            let suppress_tree_handle_click = self.tree_drag_handle_press.take() == Some(active_id);
                            if let Some(node) = tree.node_mut(active_id) {
                                node.flags.set(NodeFlags::ACTIVE, false);
                            }
                            tree.mark_dirty(active_id, NodeFlags::DIRTY_PAINT);
                            let select_scroll_release =
                                tree.node(active_id).filter(|node| matches!(node.spec.0, UiNode::Select(_))).and_then(|node| node.state.select_popup).and_then(|popup| select::select_scroll_direction_at(popup, *x, *y)).is_some();
                            if !suppress_tree_handle_click && (select_scroll_release || self.resolve_target(tree, root, *x, *y) == Some(active_id)) && tree.node(active_id).is_some_and(|node| node.spec.0.presence().state != UiState::Disabled) {
                                let slider_readout = *button == PointerButton::Primary && self.note_slider_readout_click(tree, active_id, *x, *y);
                                // 🔽️🎴️ W2 wiring: `Select` toggles its popup (`toggle_select_popup`);
                                // a `Button` (this covers `Select`'s own synthesized item rows too —
                                // see `reconcile::children_of`'s `Select` arm — since they're plain
                                // `UiNode::Button`s) fires its action, additionally closing an open
                                // `SelectPopup` if this button *is* one of that popup's rows (picking
                                // an item closes the popup, per `toggle_select_popup`'s doc comment);
                                // a `Stack` with `activate` set fires that action (see
                                // `paint::paint_stack_frame`'s matching visual for the same field).
                                let is_select = tree.node(active_id).is_some_and(|node| matches!(node.spec.0, UiNode::Select(_)));
                                let row_action = self.pointer_row_action(tree, active_id, *x, *y);
                                let disclosed = row_action.is_none() && self.pointer_toggle_disclosure(tree, active_id, *x, *y);
                                let row_also_activates = disclosed && tree.authored_tree_item(active_id).is_some() && tree.node(active_id).is_some_and(|node| matches!(&node.spec.0, UiNode::Stack(stack) if stack.activate.is_some()));
                                if slider_readout {
                                } else if let Some(index) = row_action {
                                    commands.extend(self.row_action_command(tree, active_id, index));
                                } else if disclosed && !row_also_activates {
                                } else if is_select && select_scroll_release {
                                    let _ = select::arm_retained_select_scroll_at(tree, active_id, *x, *y);
                                } else if is_select {
                                    commands.extend(self.toggle_select_popup(tree, active_id));
                                } else {
                                    let fired = tree.node(active_id).and_then(|node| match &node.spec.0 {
                                        UiNode::Button(button) => bare_action(&button.action, Trigger::Activate).map(|fired| (fired, node.parent)),
                                        UiNode::Stack(stack) => stack.activate.as_ref().and_then(|action| bare_action(action, Trigger::Activate)).map(|fired| (fired, None)),
                                        _ => None,
                                    });
                                    if let Some((fired, parent)) = fired {
                                        self.push_app_command(tree, active_id, fired, &mut commands);
                                        if let Some(parent) = parent {
                                            let picked_from_open_select = self.overlays.topmost().is_some_and(|overlay| overlay.kind == OverlayKind::SelectPopup && overlay.root == parent);
                                            if picked_from_open_select {
                                                commands.extend(self.close_topmost_overlay(tree));
                                            }
                                        }
                                    } else if !tree.node(active_id).is_some_and(|node| matches!(node.spec.0, UiNode::NumberStepper(_) | UiNode::Slider(_) | UiNode::Ring(_))) {
                                        // 🎬️ Every other value-carrying kind — `Toggle` — commits its own gesture here,
                                        // through the same one authority (see 🔖️Commit); a `Slider` or `Ring` released its
                                        // press above, wherever the pointer let go.
                                        commands.extend(self.pointer_commit(tree, active_id, *x, *y));
                                    }
                                }
                            }
                        }
                        CaptureKind::Drag => {
                            self.tree_drag_handle_press = None;
                            if let Some(node) = tree.node_mut(active_id) {
                                node.flags.set(NodeFlags::ACTIVE, false);
                            }
                            tree.mark_dirty(active_id, NodeFlags::DIRTY_PAINT);
                            if let Some(drag) = self.drag.take() {
                                commands.push(match drag.drop_target {
                                    Some(target) => UiCommand::DropCommitted { window_id: self.window_id.clone(), source: drag.source, target, payload: drag.payload },
                                    None => UiCommand::DropCancelled { window_id: self.window_id.clone(), source: drag.source },
                                });
                            }
                        }
                        CaptureKind::ScrollThumb(_) => {
                            self.thumb_start = None;
                        }
                    }
                }
                let target = self.resolve_target(tree, root, *x, *y);
                if let Some(id) = target {
                    if let Some(cmd) = self.scene_command(tree, id, event) {
                        commands.push(cmd);
                    }
                }
                if let Some((captured_id, command)) = captured_scene {
                    if target != Some(captured_id) {
                        commands.push(command);
                    }
                }
                self.update_stepper_hover_segment(tree, target, *x, *y);
                commands.extend(self.update_hover(tree, target));
            }
            UiEvent::KeyDown { key, modifiers } => {
                self.focus_visible = true;
                let select_consumed = key != "Escape" && self.route_select_key(tree, key, *modifiers, &mut commands);
                if key == "Escape" && self.set_row_reason(tree, None) {
                } else if key == "Escape" {
                    // ⎋️ React's `Search` closes its possibles popover first and only aborts the
                    // engagement when there is none open (`onKeyDown`'s Escape arm, `🖱️ui/🎯️targets/
                    // ⚛️react/🟦️.tsx:10869`), so an overlay swallows this key exactly as it does there.
                    let over_overlay = self.overlays.topmost().is_some();
                    commands.extend(self.close_topmost_overlay(tree));
                    if !over_overlay {
                        let numeric_editor_focused = self.focus.focused.is_some_and(|id| tree.node(id).is_some_and(|node| matches!(node.spec.0, UiNode::NumberStepper(_)) || matches!(node.spec.0, UiNode::Slider(_)) && node.state.edit.is_some()));
                        if numeric_editor_focused {
                            if let Some((id, fired)) = self.focus.focused.and_then(|id| cancelled_press(tree, id, "blur").map(|fired| (id, fired))) {
                                self.push_app_command(tree, id, fired, &mut commands);
                            }
                            let _ = self.focus.clear_focus(tree);
                            self.stepper_repeat = None;
                            commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: None });
                        } else {
                            let aborted = self.focus.focused.and_then(|id| tree.node(id).and_then(|node| search_line_action(node, Trigger::Abort, "")).map(|fired| (id, fired)));
                            if let Some((id, fired)) = aborted {
                                self.push_app_command(tree, id, fired, &mut commands);
                            } else if let Some(id) = self.focus.focused {
                                self.revert_text_draft(tree, id, *modifiers);
                            }
                        }
                    }
                } else if select_consumed {
                } else if key == "Tab" {
                    let scope = self.overlays.topmost_focus_trap_root().unwrap_or(root);
                    let committed = if modifiers.shift { self.focus.focus_prev(tree, scope) } else { self.focus.focus_next(tree, scope) };
                    if let Some((blurred, fired)) = committed {
                        self.push_app_command(tree, blurred, fired, &mut commands);
                    }
                    commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: self.focus.focused });
                } else if self.focused_disclosure_activation(tree, key) {
                } else if let Some((activated, fired)) = self.focused_button_activation(tree, key) {
                    self.push_app_command(tree, activated, fired, &mut commands);
                } else if let Some((changed, fired)) = self.focused_value_key_activation(tree, self.mirrored_inline_key(key), *modifiers) {
                    self.push_app_command(tree, changed, fired, &mut commands);
                } else {
                    commands.extend(self.route_edit_key(tree, key, *modifiers));
                }
            }
            UiEvent::KeyUp { .. } => {}
            UiEvent::TextInput { text } => commands.extend(self.route_text_insert(tree, text)),
            UiEvent::Paste { text } => commands.extend(self.route_text_insert(tree, text)),
            UiEvent::Ime(ime_event) => commands.extend(self.route_ime(tree, ime_event)),
            UiEvent::Scroll { x, y, delta_x, delta_y, .. } => {
                if self.overlays.topmost().filter(|overlay| overlay.kind == OverlayKind::SelectPopup).is_some_and(|overlay| select::scroll_retained_select_at(tree, overlay.root, *x, *y, *delta_y)) {
                    return commands;
                }
                if let Some(id) = self.hit_test(tree, root, *x, *y) {
                    if let Some(cmd) = self.scene_command(tree, id, event) {
                        commands.push(cmd);
                    }
                }
                self.route_scroll(tree, root, *x, *y, *delta_x * self.flow.inline.inline_sign(), *delta_y);
            }
        }
        commands
    }

}
//#endregion 🔖️UiCommand
// #endregion events
