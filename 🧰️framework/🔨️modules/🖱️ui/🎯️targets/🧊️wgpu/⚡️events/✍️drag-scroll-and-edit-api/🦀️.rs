//! ✍️ Drag, scroll and edit API: the drag payload and promotion, the scroll thumbs, the stepper and slider edit helpers and the text, IME and edit key routing. See [`super`].

use super::*;

// #region events
//#region 🔖️UiCommand
impl EventRouter {
    //#region 🔖️DragDropApi
    pub fn set_drag_payload(&mut self, node: NodeId, payload: DragPayload) {
        self.drag_payloads.insert(node, payload);
    }

    #[allow(dead_code, reason = "drag-drop registry accessor, not yet called; likely wired by a later events-integration milestone")]
    pub fn clear_drag_payload(&mut self, node: NodeId) {
        self.drag_payloads.remove(&node);
    }

    #[allow(dead_code, reason = "drag-drop registry accessor, not yet called; likely wired by a later events-integration milestone")]
    pub fn set_drop_accept(&mut self, node: NodeId, predicate: impl Fn(&DragPayload) -> bool + Send + Sync + 'static) {
        self.drop_accept.insert(node, Box::new(predicate));
    }

    pub fn drag_session(&self) -> Option<&DragSession> {
        self.drag.as_ref()
    }

    /// 🫳️ Promotes a `Press` capture on a `drag_payloads`-registered node to `CaptureKind::Drag` once
    /// the pointer has moved past `DRAG_PROMOTE_THRESHOLD_SQ` from `press_origin`.
    pub(super) fn maybe_promote_to_drag(&mut self, x: f32, y: f32) {
        let Some((pointer_id, id, CaptureKind::Press)) = self.capture.target else { return };
        let Some(payload) = self.drag_payloads.get(&id).cloned() else { return };
        let Some((origin_x, origin_y)) = self.press_origin else { return };
        if (x - origin_x).powi(2) + (y - origin_y).powi(2) < DRAG_PROMOTE_THRESHOLD_SQ {
            return;
        }
        self.capture.target = Some((pointer_id, id, CaptureKind::Drag));
        self.drag = Some(DragSession { source: id, payload, ghost: None, pointer_x: x, pointer_y: y, drop_target: None });
    }

    /// 🔄️ Live-updates the active `DragSession`'s pointer position and re-evaluates the drop target
    /// under it.
    pub(super) fn update_drag(&mut self, tree: &UiTree, root: NodeId, x: f32, y: f32) {
        if let Some(drag) = self.drag.as_mut() {
            drag.pointer_x = x;
            drag.pointer_y = y;
        }
        let target = self.hit_test(tree, root, x, y).and_then(|hit| self.nearest_accepting_drop_target(tree, hit));
        if let Some(drag) = self.drag.as_mut() {
            drag.drop_target = target;
        }
    }

    /// 🎯️ Walks `from`'s bubble chain for the nearest `NodeFlags::DROP_TARGET` node whose
    /// `drop_accept` predicate (if any) accepts the active `DragSession`'s payload.
    fn nearest_accepting_drop_target(&self, tree: &UiTree, from: NodeId) -> Option<NodeId> {
        let mut found = None;
        bubble(tree, from, |id| {
            if !tree.node(id).is_some_and(|node| node.flags.contains(NodeFlags::DROP_TARGET)) {
                return false;
            }
            let accepts = match self.drop_accept.get(&id) {
                Some(predicate) => self.drag.as_ref().is_some_and(|drag| predicate(&drag.payload)),
                None => true,
            };
            if accepts {
                found = Some(id);
                true
            } else {
                false
            }
        });
        found
    }
    //#endregion 🔖️DragDropApi

    //#region 🔖️ScrollApi
    #[allow(dead_code, reason = "scroll-thumb registry accessor, not yet called; likely wired by a later events-integration milestone")]
    pub fn register_scroll_thumb(&mut self, thumb: NodeId, scrollable: NodeId, axis: ScrollAxis) {
        self.scroll_thumbs.insert(thumb, (scrollable, axis));
    }

    pub(super) fn route_scroll(&mut self, tree: &mut UiTree, root: NodeId, x: f32, y: f32, delta_x: f32, delta_y: f32) {
        let Some(hit) = self.hit_test(tree, root, x, y) else { return };
        let Some(scrollable) = nearest_scrollable_ancestor(tree, hit) else { return };
        let Some(viewport) = tree.accepted_layout(scrollable) else { return };
        let (content_width, content_height) = tree.children(scrollable).filter_map(|child| tree.accepted_layout(child)).fold((0.0_f32, 0.0_f32), |(width, height), child| (width.max(child.x + child.width), height.max(child.y + child.height)));
        let max_x = (content_width - viewport.width).max(0.0);
        let max_y = (content_height - viewport.height).max(0.0);
        if let Some(node) = tree.node_mut(scrollable) {
            let (offset_x, offset_y) = node.state.scroll_offset;
            node.state.scroll_offset = ((offset_x + delta_x).clamp(0.0, max_x), (offset_y + delta_y).clamp(0.0, max_y));
        }
        tree.mark_dirty(scrollable, NodeFlags::DIRTY_PAINT);
    }

    pub(super) fn update_scroll_thumb(&mut self, tree: &mut UiTree, scrollable: NodeId, axis: ScrollAxis, x: f32, y: f32) {
        let Some((origin_x, origin_y, start_x, start_y)) = self.thumb_start else { return };
        let (delta_x, delta_y) = (x - origin_x, y - origin_y);
        let Some(node) = tree.node_mut(scrollable) else { return };
        node.state.scroll_offset = match axis {
            ScrollAxis::Horizontal => ((start_x + delta_x).max(0.0), start_y),
            ScrollAxis::Vertical => (start_x, (start_y + delta_y).max(0.0)),
        };
        tree.mark_dirty(scrollable, NodeFlags::DIRTY_PAINT);
    }
    //#endregion 🔖️ScrollApi

    //#region 🔖️EditApi
    pub(super) fn apply_stepper_local_delta(&mut self, tree: &mut UiTree, id: NodeId, delta: f64) {
        let Some(node) = tree.node_mut(id) else { return };
        let UiNode::NumberStepper(stepper) = &node.spec.0 else { return };
        let value = constrain_stepper_value(number_stepper_live_value(node, stepper) + delta, stepper);
        let text = semio_framework_ui_wgpu_controls::stepper::stepper_value_text(value, stepper.precision, stepper.display_factor);
        let caret = text.len();
        node.state.edit = Some(EditState { text, caret, anchor: caret, composition: None, scroll_x: 0.0 });
        node.state.number_refusal = None;
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
    }

    /// 🧷️ Shows a stepper's law-resolved `value` in its edit buffer, the caret at its end and any earlier refusal gone.
    pub(super) fn apply_stepper_local_value(&mut self, tree: &mut UiTree, id: NodeId, value: f64) {
        let Some(node) = tree.node_mut(id) else { return };
        let UiNode::NumberStepper(stepper) = &node.spec.0 else { return };
        let text = semio_framework_ui_wgpu_controls::stepper::stepper_value_text(value, stepper.precision, stepper.display_factor);
        let caret = text.len();
        node.state.edit = Some(EditState { text, caret, anchor: caret, composition: None, scroll_x: 0.0 });
        node.state.number_refusal = None;
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
    }

    /// ⏮️ Escape on a text field that holds a draft (the shared text keyboard law's `Revert`): the draft is dropped and the
    /// field shows its published value again, the caret at its end; nothing is dispatched and the field keeps focus.
    pub(super) fn revert_text_draft(&mut self, tree: &mut UiTree, id: NodeId, modifiers: EventModifiers) {
        let Some(node) = tree.node_mut(id) else { return };
        let UiNode::Input(input) = &node.spec.0 else { return };
        if !commits_on_blur(&node.spec.0) || text_field_key(&input.input_kind, "Escape", modifiers) != Some(ui_contract::TextInputKeyAction::Revert) {
            return;
        }
        let Some(text) = editable_value(&node.spec.0) else { return };
        let caret = text.len();
        node.state.edit = Some(EditState { text, caret, anchor: caret, composition: None, scroll_x: 0.0 });
        node.state.number_refusal = None;
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
    }

    /// 🖋️ Replaces `id`'s edit buffer with `text` exactly as typed (never clamped), the caret at its end; a stepper earns
    /// its refusal at once, every other control at its commit.
    pub(super) fn write_edit_buffer(&mut self, tree: &mut UiTree, id: NodeId, text: &str) {
        if let Some(node) = tree.node_mut(id) {
            let caret = text.len();
            node.state.edit = Some(EditState { text: text.to_string(), caret, anchor: caret, composition: None, scroll_x: 0.0 });
        }
        self.normalize_stepper_edit(tree, id);
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
    }

    /// 🪁️ After an edit-buffer mutation: the typed draft is kept as typed (never clamped), a stepper — which dispatches every
    /// admitted keystroke — shows the refusal its draft earns at once, and every other numeric control re-earns its refusal at
    /// its commit (Enter, blur).
    fn normalize_stepper_edit(&mut self, tree: &mut UiTree, id: NodeId) {
        let Some(node) = tree.node_mut(id) else { return };
        node.state.number_refusal = if matches!(node.spec.0, UiNode::NumberStepper(_)) { edit_refusal(node) } else { None };
    }

    pub(super) fn arm_stepper_repeat(&mut self, tree: &UiTree, id: NodeId, x: f32, y: f32) {
        let Some(node) = tree.node(id) else { return };
        let UiNode::NumberStepper(stepper) = &node.spec.0 else { return };
        let Some(bounds) = absolute_rect(tree, id) else { return };
        let Some(sign) = number_stepper_sign_at(bounds, x, y, self.flow.inline, self.control_border) else { return };
        if number_stepper_can_step(node, stepper, sign) {
            self.stepper_repeat = Some(StepperRepeat { node: id, sign, next_at: self.clock_seconds + STEPPER_REPEAT_DELAY_SECONDS + STEPPER_REPEAT_INTERVAL_SECONDS });
        }
    }

    pub(super) fn advance_stepper_repeat(&mut self, tree: &mut UiTree, seconds: f64) -> bool {
        let Some(mut repeat) = self.stepper_repeat else { return false };
        if seconds + STEPPER_REPEAT_CLOCK_EPSILON_SECONDS < repeat.next_at {
            return false;
        }
        if !tree.node(repeat.node).is_some_and(|node| matches!(node.spec.0, UiNode::NumberStepper(_)) && node.spec.0.presence().state != UiState::Disabled) {
            self.stepper_repeat = None;
            return true;
        }
        let ticks = ((seconds + STEPPER_REPEAT_CLOCK_EPSILON_SECONDS - repeat.next_at) / STEPPER_REPEAT_INTERVAL_SECONDS).floor().max(0.0) as u32 + 1;
        let step = tree.node(repeat.node).and_then(|node| match &node.spec.0 {
            UiNode::NumberStepper(stepper) => Some(stepper.step),
            _ => None,
        });
        if let Some(step) = step {
            self.apply_stepper_local_delta(tree, repeat.node, repeat.sign * step * f64::from(ticks));
        }
        repeat.next_at += STEPPER_REPEAT_INTERVAL_SECONDS * f64::from(ticks);
        self.stepper_repeat = Some(repeat);
        true
    }

    /// 🎬️ The focused editable node's buffer as a dispatchable action, for a node that commits on
    /// every keystroke (`Trigger::Change` — React's `InputView` with no `commit: "blur"`, and its
    /// `IconSelector` editor, both of which fire per change). `None` for a blur-committing node,
    /// whose one dispatch moment is `FocusState::set_focus`.
    fn changed_buffer_action(&self, tree: &UiTree) -> Option<(NodeId, FiredAction)> {
        let id = self.focus.focused?;
        let node = tree.node(id)?;
        if commits_on_blur(&node.spec.0) || matches!(node.spec.0, UiNode::Slider(_)) {
            return None;
        }
        edit_commit_action(node, &node.state.edit.as_ref()?.text).map(|fired| (id, fired))
    }

    /// 📋️ Wraps `changed_buffer_action` for a caller that already owns the command list.
    pub(super) fn push_buffer_change(&mut self, tree: &mut UiTree, out: &mut Vec<UiCommand>) {
        if let Some((id, fired)) = self.changed_buffer_action(tree) {
            let fired = if tree.node(id).is_some_and(|node| is_continuous_control(&node.spec.0)) { pressed(tree, &self.window_id, id, fired, false) } else { fired };
            self.push_app_command(tree, id, fired, out);
        }
    }

    pub(super) fn apply_slider_local_value(&mut self, tree: &mut UiTree, id: NodeId, value: f64) {
        let Some(node) = tree.node_mut(id) else { return };
        let UiNode::Slider(slider) = &node.spec.0 else { return };
        node.state.slider_draft_value = Some(constrain_slider_value(value, slider));
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
    }

    pub(super) fn note_slider_readout_click(&mut self, tree: &mut UiTree, id: NodeId, x: f32, y: f32) -> bool {
        let Some(bounds) = absolute_rect(tree, id) else { return false };
        let Some(node) = tree.node(id) else { return false };
        let UiNode::Slider(slider) = &node.spec.0 else { return false };
        let value = slider_live_value(node, slider);
        let unit_width = slider.unit_label().map(|_| tree.accepted_inline_suffix_width(id).unwrap_or(0.0));
        if !slider_control_presentation(bounds, slider.axis_position(value), 0.0, 1.0, unit_width, self.control_gap, self.flow.inline).slider.value_cell.contains(x, y) {
            if let Some(node) = tree.node_mut(id) {
                node.state.slider_readout_click_at = None;
            }
            return false;
        }
        let double_click = node.state.slider_readout_click_at.is_some_and(|at| self.clock_seconds - at <= SLIDER_DOUBLE_CLICK_SECONDS);
        let text = double_click.then(|| slider.readout(value));
        let Some(node) = tree.node_mut(id) else { return false };
        node.state.slider_readout_click_at = (!double_click).then_some(self.clock_seconds);
        if let Some(text) = text {
            let caret = text.len();
            node.state.edit = Some(EditState { text, caret, anchor: caret, composition: None, scroll_x: 0.0 });
            tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        }
        true
    }

    pub(super) fn finish_slider_readout_edit(&mut self, tree: &mut UiTree, id: NodeId) -> Option<FiredAction> {
        let node = tree.node(id)?;
        let UiNode::Slider(slider) = &node.spec.0 else { return None };
        let live = slider_live_value(node, slider);
        let typed = typed_number(&node.state.edit.as_ref()?.text, slider.display_factor, slider.precision, std::iter::once(live).chain(slider.snaps.iter().copied()), Some(slider.min), Some(slider.max), slider.limits.as_ref());
        let slider = slider.clone();
        let slider = &slider;
        let candidate = match typed {
            Ok(value) if value >= slider.min && value <= slider.max => constrain_slider_value(value, slider),
            Ok(value) => value,
            Err(refusal) => {
                if let Some(node) = tree.node_mut(id) {
                    node.state.number_refusal = Some(refusal);
                }
                tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
                return None;
            }
        };
        let action = slider.on_change.clone();
        let epsilon = if slider.step > 0.0 { slider.step * 0.25 } else { 1e-9 };
        let changed = Some(candidate).filter(|value| slider.snaps.contains(value) || (*value - live).abs() > epsilon);
        if let Some(node) = tree.node_mut(id) {
            node.state.edit = None;
            node.state.number_refusal = None;
            if let Some(value) = changed {
                node.state.slider_draft_value = Some(value);
            }
        }
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        changed.and_then(|value| fired_action(&action, Trigger::Change, DslValue::float(value))).map(|fired| pressed(tree, &self.window_id, id, fired, true))
    }

    pub(super) fn route_text_insert(&mut self, tree: &mut UiTree, text: &str) -> Vec<UiCommand> {
        let mut out = Vec::new();
        let Some(id) = self.focus.focused else { return out };
        if text == " " {
            let confirmed =
                tree.node(id).filter(|node| node.state.edit.as_ref().is_none_or(|edit| edit.text.trim().is_empty())).and_then(|node| search_line_action(node, Trigger::RepeatLast, "").or_else(|| search_line_action(node, Trigger::Submit, "")));
            if let Some(fired) = confirmed {
                self.push_app_command(tree, id, fired, &mut out);
                return out;
            }
        }
        let Some(node) = tree.node_mut(id) else { return out };
        let Some(edit) = node.state.edit.as_mut() else { return out };
        insert_at_caret(edit, text);
        self.normalize_stepper_edit(tree, id);
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        self.push_buffer_change(tree, &mut out);
        out
    }

    pub(super) fn route_ime(&mut self, tree: &mut UiTree, event: &ImeEvent) -> Vec<UiCommand> {
        let mut out = Vec::new();
        let Some(id) = self.focus.focused else { return out };
        let Some(node) = tree.node_mut(id) else { return out };
        let Some(edit) = node.state.edit.as_mut() else { return out };
        let mutated = match event {
            ImeEvent::Start => {
                edit.composition = Some(String::new());
                false
            }
            ImeEvent::Update { text, .. } => {
                edit.composition = Some(text.clone());
                false
            }
            ImeEvent::Commit { text } => {
                edit.composition = None;
                insert_at_caret(edit, text);
                true
            }
            ImeEvent::Cancel => {
                edit.composition = None;
                false
            }
        };
        if mutated {
            self.normalize_stepper_edit(tree, id);
        }
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        if mutated {
            self.push_buffer_change(tree, &mut out);
        }
        out
    }

    /// ⌨️ Caret motion (with `Shift` extending the selection), `Home`/`End`, `Backspace`/`Delete`,
    /// and clipboard shortcuts for the focused node's `EditState`. A no-operation if nothing is focused or
    /// the focused node has no `EditState` (isn't a `UiNode::Input`, or hasn't been focused since
    /// `FocusState::set_focus` seeded one).
    pub(super) fn route_edit_key(&mut self, tree: &mut UiTree, key: &str, modifiers: EventModifiers) -> Vec<UiCommand> {
        let mut out = Vec::new();
        let Some(id) = self.focus.focused else { return out };
        if matches!(key, "Enter" | "NumpadEnter") {
            if tree.node(id).is_some_and(|node| matches!(node.spec.0, UiNode::Slider(_)) && node.state.edit.is_some()) {
                let fired = self.finish_slider_readout_edit(tree, id);
                if tree.node(id).is_some_and(|node| node.state.number_refusal.is_some()) {
                    return out;
                }
                let _ = self.focus.clear_focus(tree);
                if let Some(fired) = fired {
                    self.push_app_command(tree, id, fired, &mut out);
                }
                out.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: None });
                return out;
            }
            if tree.node(id).is_some_and(|node| matches!(node.spec.0, UiNode::NumberStepper(_))) {
                if let Some((blurred, fired)) = self.focus.clear_focus(tree) {
                    self.push_app_command(tree, blurred, fired, &mut out);
                }
                self.stepper_repeat = None;
                out.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: None });
                return out;
            }
            let submitted = tree.node(id).and_then(|node| {
                let fallback = editable_value(&node.spec.0).unwrap_or_default();
                search_line_action(node, Trigger::Submit, node.state.edit.as_ref().map_or(fallback.as_str(), |edit| edit.text.as_str()))
            });
            if let Some(fired) = submitted {
                self.push_app_command(tree, id, fired, &mut out);
                return out;
            }
            let law = tree.node(id).and_then(|node| match &node.spec.0 {
                UiNode::Input(input) => Some(text_field_key(&input.input_kind, key, modifiers)),
                _ => None,
            });
            match law {
                Some(Some(ui_contract::TextInputKeyAction::Newline)) => {
                    if let Some(edit) = tree.node_mut(id).and_then(|node| node.state.edit.as_mut()) {
                        insert_at_caret(edit, "\n");
                    }
                    tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
                    self.push_buffer_change(tree, &mut out);
                    return out;
                }
                Some(None) => return out,
                _ => {}
            }
            let fired = tree.node(id).filter(|node| commits_on_blur(&node.spec.0)).and_then(|node| node.state.edit.as_ref().and_then(|edit| edit_commit_action(node, &edit.text)));
            if let Some(node) = tree.node_mut(id) {
                let refusal = edit_refusal(node);
                node.state.number_refusal = refusal;
            }
            tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
            if let Some(fired) = fired {
                self.push_app_command(tree, id, fired, &mut out);
            }
            return out;
        }
        let Some(node) = tree.node_mut(id) else { return out };
        let Some(edit) = node.state.edit.as_mut() else { return out };
        let has_selection = edit.anchor != edit.caret;
        let mut mutated = false;
        let law = match &node.spec.0 {
            UiNode::Input(input) if input.input_kind == "number" => number_field_key(key, modifiers.shift, input.min, input.max).map(|(law_key, large)| (input, law_key, large)),
            _ => None,
        };
        match key {
            _ if law.is_some() => {
                let Some((input, law_key, large)) = law else { return out };
                let Some(current) = edit.text.trim().parse::<f64>().ok().filter(|value| value.is_finite()) else { return out };
                let current = typed_number(&edit.text, input.display_factor, input.precision, current_stored(input), input.min, input.max, input.limits.as_ref()).unwrap_or(current);
                let next = ui_contract::ui_number_key_value(current, input.min, input.max, input.step.unwrap_or(0.0), input.precision, input.display_factor, input.snaps.iter().copied(), law_key, large);
                edit.text = number_field_text(input, next);
                edit.caret = edit.text.len();
                edit.anchor = edit.caret;
                mutated = true;
            }
            "ArrowLeft" => {
                edit.caret = if has_selection && !modifiers.shift { selection_bounds(edit.anchor, edit.caret).0 } else { prev_char_boundary(&edit.text, edit.caret) };
                if !modifiers.shift {
                    edit.anchor = edit.caret;
                }
            }
            "ArrowRight" => {
                edit.caret = if has_selection && !modifiers.shift { selection_bounds(edit.anchor, edit.caret).1 } else { next_char_boundary(&edit.text, edit.caret) };
                if !modifiers.shift {
                    edit.anchor = edit.caret;
                }
            }
            "Home" => {
                edit.caret = 0;
                if !modifiers.shift {
                    edit.anchor = 0;
                }
            }
            "End" => {
                edit.caret = edit.text.len();
                if !modifiers.shift {
                    edit.anchor = edit.text.len();
                }
            }
            "Backspace" => {
                if has_selection {
                    let (start, end) = selection_bounds(edit.anchor, edit.caret);
                    edit.text.replace_range(start..end, "");
                    edit.caret = start;
                    edit.anchor = start;
                    mutated = true;
                } else if edit.caret > 0 {
                    let start = prev_char_boundary(&edit.text, edit.caret);
                    edit.text.replace_range(start..edit.caret, "");
                    edit.caret = start;
                    edit.anchor = start;
                    mutated = true;
                }
            }
            "Delete" => {
                if has_selection {
                    let (start, end) = selection_bounds(edit.anchor, edit.caret);
                    edit.text.replace_range(start..end, "");
                    edit.caret = start;
                    edit.anchor = start;
                    mutated = true;
                } else if edit.caret < edit.text.len() {
                    let end = next_char_boundary(&edit.text, edit.caret);
                    edit.text.replace_range(edit.caret..end, "");
                    mutated = true;
                }
            }
            "a" | "A" if modifiers.ctrl || modifiers.meta => {
                edit.anchor = 0;
                edit.caret = edit.text.len();
            }
            "c" | "C" if modifiers.ctrl || modifiers.meta => {
                if has_selection {
                    let (start, end) = selection_bounds(edit.anchor, edit.caret);
                    out.push(UiCommand::ClipboardCopy { window_id: self.window_id.clone(), text: edit.text[start..end].to_string() });
                }
                return out;
            }
            "x" | "X" if modifiers.ctrl || modifiers.meta => {
                if has_selection {
                    let (start, end) = selection_bounds(edit.anchor, edit.caret);
                    out.push(UiCommand::ClipboardCut { window_id: self.window_id.clone(), text: edit.text[start..end].to_string() });
                    edit.text.replace_range(start..end, "");
                    edit.caret = start;
                    edit.anchor = start;
                    mutated = true;
                } else {
                    return out;
                }
            }
            "v" | "V" if modifiers.ctrl || modifiers.meta => {
                out.push(UiCommand::ClipboardPasteRequested { window_id: self.window_id.clone() });
                return out;
            }
            _ => return out,
        }
        if mutated {
            self.normalize_stepper_edit(tree, id);
        }
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        if mutated {
            self.push_buffer_change(tree, &mut out);
        }
        out
    }
    //#endregion 🔖️EditApi

}
//#endregion 🔖️UiCommand
// #endregion events
