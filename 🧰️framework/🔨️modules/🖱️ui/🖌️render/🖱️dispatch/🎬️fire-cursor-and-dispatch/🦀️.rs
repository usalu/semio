//! 🎬️ Fire, cursor and dispatch: the listener firing, the cursor resolution, the `Dispatcher::dispatch` entry, the target resolution and the hover update. See [`super`].

use super::*;

//#region 🔖️Outcome
impl Dispatcher {
    //#region 🎬️FireApi
    /// 🎬️ Fires `id`'s binding for `trigger`, if any — building the [`UiIntent`] from the node's own
    /// [`ListenerSet`] addressing. `None` if there's no such binding, or if the listener's own captured
    /// revision is already stale against the tree it's being resolved in (see [`is_stale`]).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn fire(&mut self, tree: &DispatchTree, id: FrameNodeId, trigger: Trigger, input: Option<UiValue>) -> Option<UiIntent> {
        let node = tree.node(id)?;
        let binding = node.listeners.binding_for(trigger)?;
        if is_stale(node.listeners.revision, tree.revision()) {
            return None;
        }
        let args = match binding.args.as_ref() {
            Some(args) => Some(args.credited_clone()?),
            None => None,
        };
        self.seq += 1;
        Some(UiIntent {
            surface: node.listeners.surface.clone(),
            revision: node.listeners.revision,
            node: node.listeners.node,
            node_key: node.listeners.node_key.clone(),
            trigger,
            action: binding.action.clone(),
            args,
            input,
            seq: self.seq,
        })
    }

    /// 🎬️ Fires a binding on behalf of an already-captured node, additionally rejecting it if the
    /// *capture's own* recorded revision (from when it started, possibly frames ago) is now stale
    /// against the current tree — the case [`fire`] alone cannot catch, since `id`'s own
    /// [`ListenerSet::revision`] is always in sync with the tree that produced it, never with how long
    /// ago the capture began. This is the ticket's "a stale interaction is rejected rather than
    /// misapplied" — see the `stale_captured_activate_is_rejected...` test.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn fire_captured(&mut self, tree: &DispatchTree, id: FrameNodeId, entry: CaptureEntry, trigger: Trigger, input: Option<UiValue>) -> Option<UiIntent> {
        if is_stale(entry.revision, tree.revision()) {
            return None;
        }
        self.fire(tree, id, trigger, input)
    }
    //#endregion 🎬️FireApi

    //#region 🖱️CursorApi
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn cursor_for(&self, tree: &DispatchTree, target: Option<FrameNodeId>) -> CursorRequest {
        if self.drag.is_some() {
            return CursorRequest::Grabbing;
        }
        let Some(node) = target.and_then(|id| tree.node(id)) else { return CursorRequest::Default };
        if node.flags.contains(DispatchFlags::DRAG_SOURCE) {
            return CursorRequest::Grab;
        }
        if node.flags.contains(DispatchFlags::EDITABLE) {
            return CursorRequest::Text;
        }
        if !node.listeners.bindings.is_empty() || node.flags.contains(DispatchFlags::FOCUSABLE) {
            return CursorRequest::Pointer;
        }
        CursorRequest::Default
    }
    //#endregion 🖱️CursorApi

    /// 🚦️ Ported from `events.rs::EventRouter::dispatch`: resolves the event's target (capture target
    /// if captured, else [`hit_test`]), updates interaction state, and returns the resulting
    /// [`DispatchOutcome`]. Always called against the *presented* [`DispatchTree`] — see this file's
    /// module docstring.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn dispatch(&mut self, tree: &DispatchTree, event: &DispatchEvent) -> DispatchOutcome {
        let Some(root) = tree.root() else { return DispatchOutcome::default() };
        let mut outcome = DispatchOutcome::default();

        match event {
            DispatchEvent::PointerCancel { pointer } => {
                self.press_origin.remove(&pointer.id);
                self.thumb_start.remove(&pointer.id);
                if let Some(entry) = self.capture.remove(&pointer.id) {
                    if entry.kind == CaptureKind::Drag {
                        self.drag = None;
                    }
                    outcome.handled = true;
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
                if self.update_hover(tree, None) {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
            }
            DispatchEvent::PointerMove { pointer, x, y, .. } => {
                self.maybe_promote_to_drag(pointer.id, *x, *y);
                match self.capture.get(&pointer.id).copied() {
                    Some(entry) if entry.kind == CaptureKind::Drag => self.update_drag(tree, root, *x, *y),
                    Some(entry) => {
                        if let CaptureKind::ScrollThumb(axis) = entry.kind {
                            self.update_scroll_thumb(entry.element, axis, pointer.id, *x, *y);
                            outcome.invalidation.insert(InvalidationReason::LAYOUT);
                        }
                    }
                    None => {}
                }
                let target = self.resolve_target(tree, root, pointer.id, *x, *y);
                if self.update_hover(tree, target) {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
                if self.maybe_dismiss_tooltip_on_hover_out(tree, *x, *y) {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
                outcome.handled = target.is_some();
                outcome.cursor = self.cursor_for(tree, target);
            }
            DispatchEvent::PointerDown { pointer, x, y, .. } => {
                if !self.press_origin.contains_key(&pointer.id) && self.press_origin.len() >= 16 {
                    return outcome;
                }
                if self.dismiss_topmost_if_outside_press(tree, *x, *y) {
                    outcome.handled = true;
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                    return outcome;
                }
                self.press_origin.insert(pointer.id, (*x, *y));
                let target = hit_test(tree, root, *x, *y);
                if self.update_hover(tree, target) {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
                match target.and_then(|id| tree.node(id).map(|node| (id, node))) {
                    Some((id, node)) => {
                        outcome.handled = true;
                        if let Some(&(scrollable, axis)) = self.scroll_thumbs.get(&node.element) {
                            self.capture.insert(pointer.id, CaptureEntry { element: scrollable, kind: CaptureKind::ScrollThumb(axis), revision: tree.revision() });
                            let (offset_x, offset_y) = self.scroll_offset(scrollable);
                            self.thumb_start.insert(pointer.id, (*x, *y, offset_x, offset_y));
                        } else {
                            self.capture.insert(pointer.id, CaptureEntry { element: node.element, kind: CaptureKind::Press, revision: tree.revision() });
                            if node.flags.contains(DispatchFlags::FOCUSABLE) {
                                self.apply_focus_transition(tree, Some(node.element));
                                outcome.invalidation.insert(InvalidationReason::PAINT);
                                if node.flags.contains(DispatchFlags::EDITABLE) {
                                    let cursor_bounds = node_bounds(tree, id).unwrap_or(Bounds::new(0.0, 0.0, 0.0, 0.0));
                                    outcome.ime = Some(ImeDirective::Enable { cursor_bounds });
                                }
                            }
                        }
                    }
                    None => {
                        if self.clear_focus(tree) {
                            outcome.invalidation.insert(InvalidationReason::PAINT);
                            outcome.ime = Some(ImeDirective::Disable);
                        }
                    }
                }
                outcome.cursor = self.cursor_for(tree, target);
            }
            DispatchEvent::PointerUp { pointer, x, y, .. } => {
                self.press_origin.remove(&pointer.id);
                if let Some(entry) = self.capture.remove(&pointer.id) {
                    outcome.handled = true;
                    match entry.kind {
                        CaptureKind::Press => {
                            if hit_test(tree, root, *x, *y) == tree.element_node(entry.element) {
                                if let Some(id) = tree.element_node(entry.element) {
                                    let node = tree.node(id).expect("resolved from tree.element_node");
                                    if node.flags.contains(DispatchFlags::OVERLAY_TRIGGER) {
                                        self.toggle_overlay_trigger(tree, entry.element);
                                        outcome.invalidation.insert(InvalidationReason::PAINT);
                                    } else if let Some(intent) = self.fire_captured(tree, id, entry, Trigger::Activate, None) {
                                        let parent_overlay = self.overlays.topmost().copied().filter(|overlay| overlay.kind == OverlayKind::SelectPopup && is_descendant(tree, id, tree.element_node(overlay.root).unwrap_or(id)));
                                        outcome.intents.push(intent);
                                        outcome.invalidation.insert(InvalidationReason::STRUCTURE);
                                        if let Some(overlay) = parent_overlay {
                                            self.overlays.close_root(overlay.root);
                                            self.finish_close(tree, overlay);
                                        }
                                    }
                                }
                            }
                        }
                        CaptureKind::Drag => {
                            if let Some(drag) = self.drag.take() {
                                outcome.invalidation.insert(InvalidationReason::PAINT);
                                if let Some(target) = drag.drop_target {
                                    if let Some(target_id) = tree.element_node(target) {
                                        if let Some(value) = drag_payload_to_value(&drag.payload) {
                                            if let Some(intent) = self.fire(tree, target_id, Trigger::Drop, Some(value)) {
                                                outcome.intents.push(intent);
                                                outcome.invalidation.insert(InvalidationReason::STRUCTURE);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        CaptureKind::ScrollThumb(_) => {
                            self.thumb_start.remove(&pointer.id);
                        }
                    }
                }
                let target = self.resolve_target(tree, root, pointer.id, *x, *y);
                if self.update_hover(tree, target) {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
                outcome.cursor = self.cursor_for(tree, target);
            }
            DispatchEvent::KeyDown { key, modifiers } => {
                if key == "Escape" {
                    if self.close_topmost_overlay(tree) {
                        outcome.handled = true;
                        outcome.invalidation.insert(InvalidationReason::PAINT);
                    }
                } else if key == "Tab" {
                    let scope_element = self.overlays.topmost_focus_trap_root();
                    let scope = scope_element.and_then(|element| tree.element_node(element)).unwrap_or(root);
                    let changed = if modifiers.shift { self.focus_prev(tree, scope) } else { self.focus_next(tree, scope) };
                    outcome.handled = true;
                    if changed {
                        outcome.invalidation.insert(InvalidationReason::PAINT);
                    }
                } else {
                    let (changed, intent) = self.route_edit_key(key, *modifiers);
                    outcome.handled = changed;
                    if changed {
                        outcome.invalidation.insert(InvalidationReason::PAINT);
                    }
                    if let Some(intent) = intent {
                        outcome.intents.push(intent);
                    }
                }
            }
            DispatchEvent::KeyUp { .. } => {}
            DispatchEvent::TextInput { text } => {
                outcome.handled = self.route_text_insert(text);
                if outcome.handled {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
            }
            DispatchEvent::Paste { text } => {
                outcome.handled = self.route_text_insert(text);
                if outcome.handled {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
            }
            DispatchEvent::TextEditStart { .. } | DispatchEvent::TextEditChunk { .. } | DispatchEvent::TextEditCommit { .. } | DispatchEvent::TextEditAbort { .. } => {}
            DispatchEvent::Ime(ime_event) => {
                outcome.handled = self.route_ime(ime_event);
                if outcome.handled {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
            }
            DispatchEvent::Scroll { x, y, delta_x, delta_y, .. } => {
                outcome.handled = self.route_scroll(tree, root, *x, *y, *delta_x, *delta_y);
                if outcome.handled {
                    outcome.invalidation.insert(InvalidationReason::LAYOUT);
                }
            }
            DispatchEvent::Accessibility { .. } => {}
        }

        if let Some(pointer) = event_pointer(event) {
            outcome.capture = self.capture.get(&pointer).and_then(|entry| {
                let hitbox = tree.element_node(entry.element).and_then(|id| tree.node(id)).and_then(|node| node.hitbox);
                hitbox.map(|hitbox| (hitbox, entry.kind))
            });
        }
        outcome
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn resolve_target(&self, tree: &DispatchTree, root: FrameNodeId, pointer: PointerId, x: f32, y: f32) -> Option<FrameNodeId> {
        match self.capture.get(&pointer) {
            Some(entry) => tree.element_node(entry.element),
            None => hit_test(tree, root, x, y),
        }
    }

    /// 👆️ Ported from `events.rs::EventRouter::update_hover`: flips the hover bubble chain — every
    /// node from the hit target up to the root observes [`Dispatcher::is_hovered`], not just the leaf,
    /// so an ancestor layout container still reads as hovered for a host's reveal-on-hover styling.
    /// Returns whether anything changed.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn update_hover(&mut self, tree: &DispatchTree, target: Option<FrameNodeId>) -> bool {
        let target_element = target.and_then(|id| tree.node(id)).map(|node| node.element);
        if self.hovered == target_element {
            return false;
        }
        let mut new_chain = Vec::with_capacity(64);
        if let Some(leaf) = target {
            bubble(tree, leaf, |id| {
                if new_chain.len() >= 64 {
                    return true;
                }
                if let Some(node) = tree.node(id) {
                    new_chain.push(node.element);
                }
                false
            });
        }
        self.hover_chain = new_chain;
        self.hovered = target_element;
        true
    }
}
//#endregion 🔖️Outcome
