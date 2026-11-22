//! ✍️ Drag, scroll and edit API: the drag payload and promotion, the scroll thumbs, the stepper and slider edit helpers and the text, IME and edit key routing. See [`super`].

use super::*;

// #region events
//#region 🔖️UiCommand
impl EventRouter {
    /// 📜️ A press on a scrollbar of `from`'s nearest scroll host: on a thumb it starts a thumb drag, beside it the host pages one viewport towards the press. `false` when the press is not on a scrollbar.
    pub(super) fn press_scrollbar(&mut self, tree: &mut UiTree, from: NodeId, x: f32, y: f32, pointer_id: u64) -> bool {
        let Some(scrollable) = nearest_scrollable_ancestor(tree, from) else { return false };
        let rtl = self.flow.inline == FlowInline::Rtl;
        let Some(bar) = [true, false].into_iter().filter_map(|vertical| scrollbar_for(tree, scrollable, vertical, rtl)).find(|bar| bar.track.contains(x, y)) else { return false };
        let offset = tree.node(scrollable).map(|node| node.state.scroll_offset).unwrap_or_default();
        if bar.thumb.contains(x, y) {
            self.capture.target = Some((pointer_id, scrollable, CaptureKind::ScrollThumb(if bar.vertical { ScrollAxis::Vertical } else { ScrollAxis::Horizontal })));
            self.thumb_start = Some((x, y, offset.0, offset.1));
        } else if let Some(node) = tree.node_mut(scrollable) {
            node.state.scroll_offset = if bar.vertical { (offset.0, bar.offset_for_track_press(offset.1, y)) } else { (bar.offset_for_track_press(offset.0, x), offset.1) };
            tree.mark_dirty(scrollable, NodeFlags::DIRTY_PAINT);
        }
        true
    }

    pub(super) fn route_scroll(&mut self, tree: &mut UiTree, root: NodeId, x: f32, y: f32, delta_x: f32, delta_y: f32) {
        let Some(hit) = self.hit_test(tree, root, x, y) else { return };
        if self.scroll_text_area(tree, hit, delta_y) {
            return;
        }
        let mut from = Some(hit);
        while let Some(start) = from {
            let Some(scrollable) = nearest_scrollable_ancestor(tree, start) else { return };
            let Some(viewport) = tree.accepted_layout(scrollable) else { return };
            let (content_width, content_height) = tree.children(scrollable).filter_map(|child| tree.accepted_layout(child)).fold((0.0_f32, 0.0_f32), |(width, height), child| (width.max(child.x + child.width), height.max(child.y + child.height)));
            let max_x = (content_width - viewport.width).max(0.0);
            let max_y = (content_height - viewport.height).max(0.0);
            let (offset_x, offset_y) = tree.node(scrollable).map_or((0.0, 0.0), |node| node.state.scroll_offset);
            let moves_x = delta_x != 0.0 && (max_x > 0.0 || offset_x > 0.0);
            let moves_y = delta_y != 0.0 && (max_y > 0.0 || offset_y > 0.0);
            if !moves_x && !moves_y {
                from = tree.node(scrollable).and_then(|node| node.parent);
                continue;
            }
            if let Some(node) = tree.node_mut(scrollable) {
                node.state.scroll_offset = ((offset_x + delta_x).clamp(0.0, max_x), (offset_y + delta_y).clamp(0.0, max_y));
            }
            tree.mark_dirty(scrollable, NodeFlags::DIRTY_PAINT);
            return;
        }
    }

    pub(super) fn update_scroll_thumb(&mut self, tree: &mut UiTree, scrollable: NodeId, axis: ScrollAxis, x: f32, y: f32) {
        let Some((origin_x, origin_y, start_x, start_y)) = self.thumb_start else { return };
        let vertical = axis == ScrollAxis::Vertical;
        let Some(bar) = scrollbar_for(tree, scrollable, vertical, self.flow.inline == FlowInline::Rtl) else { return };
        let Some(node) = tree.node_mut(scrollable) else { return };
        node.state.scroll_offset = if vertical { (start_x, bar.offset_for_thumb_drag(start_y, y - origin_y)) } else { (bar.offset_for_thumb_drag(start_x, x - origin_x), start_y) };
        tree.mark_dirty(scrollable, NodeFlags::DIRTY_PAINT);
    }

    /// 🛞️ A wheel turn over a textarea that holds a laid-out text, focused or not, scrolls it while it has room that way.
    pub(super) fn scroll_text_area(&mut self, tree: &mut UiTree, id: NodeId, delta_y: f32) -> bool {
        let Some(node) = tree.node_mut(id) else { return false };
        let live = match (&node.state.edit, &node.spec.0) {
            (Some(edit), _) => Some(edit.text.as_str()),
            (None, UiNode::Input(input)) => Some(input.value.as_str()),
            _ => None,
        };
        let Some(laid) = node.state.edit_layout.as_ref().filter(|laid| laid.multi && live == Some(laid.text.as_str())) else { return false };
        let max = laid.max_scroll().1;
        let offset = node.state.scroll_offset;
        let next = (offset.1 + delta_y).clamp(0.0, max);
        if delta_y == 0.0 || next == offset.1 {
            return false;
        }
        node.state.scroll_offset = (offset.0, next);
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        true
    }

    /// 👆️ The caret position under a pointer at `(x, y)` in a field whose laid-out draft is current, `None` while it is not or mid-composition.
    fn text_byte_at(tree: &UiTree, id: NodeId, x: f32, y: f32) -> Option<usize> {
        let node = tree.node(id)?;
        let edit = node.state.edit.as_ref()?;
        let laid = node.state.edit_layout.as_ref().filter(|laid| laid.text == edit.text)?;
        if edit.composition.as_deref().is_some_and(|composition| !composition.is_empty()) {
            return None;
        }
        let bounds = absolute_rect(tree, id)?;
        let (scroll_x, scroll_y) = if laid.multi { (0.0, node.state.scroll_offset.1) } else { (node.state.scroll_offset.0, 0.0) };
        Some(laid.byte_at(x - bounds.x - laid.inner[0] + scroll_x, y - bounds.y - laid.inner[1] + scroll_y))
    }

    /// 🖱️ Counts a press into the click chain — DOM `MouseEvent.detail` — and returns its count: a press by the same button within [`DOUBLE_CLICK_SECONDS`] and [`DOUBLE_CLICK_SLOP_PX`] of the one before continues the chain, any other starts it.
    pub(super) fn note_press(&mut self, x: f32, y: f32, button: PointerButton) -> u32 {
        let seconds = self.clock_seconds;
        let count = self.click_chain.filter(|chain| chain.button == button && seconds - chain.seconds <= DOUBLE_CLICK_SECONDS && (chain.x - x).abs() <= DOUBLE_CLICK_SLOP_PX && (chain.y - y).abs() <= DOUBLE_CLICK_SLOP_PX).map_or(1, |chain| chain.count.saturating_add(1));
        self.click_chain = Some(ClickChain { seconds, x, y, button, count });
        count
    }

    /// 🔢️ The click count of the latest press: 1 single, 2 double, 3 triple.
    pub(super) fn click_count(&self) -> u32 {
        self.click_chain.map_or(1, |chain| chain.count)
    }

    /// 🖱️ A primary press on a text field: one click places the caret at the pointer (Shift extends the selection), a second selects the word, a third the line; the held press then drags the selection by character, word or line as the click began. A field with no laid-out text yet takes the caret at its first layout ([`Self::settle_text_press`]).
    pub(super) fn begin_text_pointer(&mut self, tree: &mut UiTree, id: NodeId, x: f32, y: f32, modifiers: EventModifiers) {
        if !tree.node(id).is_some_and(|node| matches!(node.spec.0, UiNode::Input(_))) {
            return;
        }
        let press = PendingTextPress { node: id, x, y, extend: modifiers.shift, unit: ((self.click_count() - 1) % 3 + 1) as u8 };
        self.text_pending = None;
        if !self.place_text_press(tree, press) && tree.node(id).is_some_and(|node| node.state.edit.as_ref().is_some_and(|edit| node.state.edit_layout.as_ref().is_none_or(|laid| laid.text != edit.text))) {
            self.text_pending = Some(press);
        }
    }

    /// ⌛️ Places the caret of a text press that waited for its field's first layout; called by the frame step right after it parks one.
    pub fn settle_text_press(&mut self, tree: &mut UiTree, id: NodeId) {
        if let Some(press) = self.text_pending.filter(|press| press.node == id) {
            if self.place_text_press(tree, press) {
                self.text_pending = None;
            }
        }
    }

    fn place_text_press(&mut self, tree: &mut UiTree, press: PendingTextPress) -> bool {
        let Some(byte) = Self::text_byte_at(tree, press.node, press.x, press.y) else { return false };
        let Some(edit) = tree.node_mut(press.node).and_then(|node| node.state.edit.as_mut()) else { return false };
        let origin = match press.unit {
            2 => word_range_at(&edit.text, byte),
            3 => (line_start(&edit.text, byte), line_end(&edit.text, byte)),
            _ => {
                if !press.extend {
                    edit.anchor = byte;
                }
                (edit.anchor, byte)
            }
        };
        (edit.anchor, edit.caret) = origin;
        self.text_drag = Some(TextDrag { node: press.node, unit: press.unit, origin });
        tree.mark_dirty(press.node, NodeFlags::DIRTY_PAINT);
        true
    }

    /// ✋️ A held press moving over its text field drags the caret end of the selection to the pointer, by the unit its click began with: by character, or by word or line with the clicked span kept selected.
    pub(super) fn drag_text_pointer(&mut self, tree: &mut UiTree, drag: TextDrag, x: f32, y: f32) {
        let Some(byte) = Self::text_byte_at(tree, drag.node, x, y) else { return };
        let Some(edit) = tree.node_mut(drag.node).and_then(|node| node.state.edit.as_mut()) else { return };
        let (anchor, caret) = match drag.unit {
            1 => (edit.anchor, byte),
            unit => {
                let (start, end) = if unit == 2 { word_range_at(&edit.text, byte) } else { (line_start(&edit.text, byte), line_end(&edit.text, byte)) };
                if start < drag.origin.0 { (drag.origin.1, start) } else { (drag.origin.0, end.max(drag.origin.1)) }
            }
        };
        if (edit.anchor, edit.caret) == (anchor, caret) {
            return;
        }
        (edit.anchor, edit.caret) = (anchor, caret);
        tree.mark_dirty(drag.node, NodeFlags::DIRTY_PAINT);
    }

    pub(super) fn apply_stepper_local_delta(&mut self, tree: &mut UiTree, id: NodeId, delta: f64) {
        let Some(node) = tree.node_mut(id) else { return };
        let UiNode::NumberStepper(stepper) = &node.spec.0 else { return };
        let value = constrain_stepper_value(number_stepper_live_value(node, stepper) + delta, stepper);
        let text = semio_ui_wgpu_controls::stepper::stepper_value_text(value, stepper.precision, stepper.display_factor);
        let caret = text.len();
        node.state.edit = Some(EditState { text, caret, anchor: caret, composition: None, scroll_x: 0.0 });
        node.state.number_refusal = None;
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
    }

    /// 🧷️ Shows a stepper's law-resolved `value` in its edit buffer, the caret at its end and any earlier refusal gone.
    pub(super) fn apply_stepper_local_value(&mut self, tree: &mut UiTree, id: NodeId, value: f64) {
        let Some(node) = tree.node_mut(id) else { return };
        let UiNode::NumberStepper(stepper) = &node.spec.0 else { return };
        let text = semio_ui_wgpu_controls::stepper::stepper_value_text(value, stepper.precision, stepper.display_factor);
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
        let multi_line = matches!(&node.spec.0, UiNode::Input(input) if input.input_kind == "longText");
        let Some(edit) = node.state.edit.as_mut() else { return out };
        if multi_line || !text.contains(['\n', '\r']) {
            insert_at_caret(edit, text);
        } else {
            insert_at_caret(edit, &text.replace("\r\n", " ").replace(['\n', '\r'], " "));
        }
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
                node.state.composition_cursor = None;
                false
            }
            ImeEvent::Update { text, cursor } => {
                let mut at = (*cursor).min(text.len());
                while !text.is_char_boundary(at) {
                    at -= 1;
                }
                edit.composition = Some(text.clone());
                node.state.composition_cursor = Some(at);
                false
            }
            ImeEvent::Commit { text } => {
                edit.composition = None;
                node.state.composition_cursor = None;
                insert_at_caret(edit, text);
                true
            }
            ImeEvent::Cancel => {
                edit.composition = None;
                node.state.composition_cursor = None;
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
    /// 📋️ Runs one text-selection menu verb on the text field `id`: focuses it, then answers the verb's own chord (`a`, `c`, `x`, `v` with the command modifier) through the field's edit key route, so a menu row and the typed chord share one body and the field's Copy, Cut and Paste commands.
    pub fn run_text_selection_verb(&mut self, tree: &mut UiTree, id: NodeId, key: &str) -> Vec<UiCommand> {
        let mut out = Vec::new();
        if let Some((blurred, fired)) = self.focus.set_focus(tree, Some(id), true) {
            self.push_app_command(tree, blurred, fired, &mut out);
        }
        out.extend(self.route_edit_key(tree, key, EventModifiers { shift: false, ctrl: true, alt: false, meta: false }));
        out
    }

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
        if !matches!(key, "ArrowUp" | "ArrowDown") {
            node.state.caret_goal_x = None;
        }
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
            "ArrowLeft" | "ArrowRight" => {
                let forward = key == "ArrowRight";
                edit.caret = if modifiers.meta {
                    if forward { line_end(&edit.text, edit.caret) } else { line_start(&edit.text, edit.caret) }
                } else if modifiers.alt {
                    if forward { word_end_after(&edit.text, edit.caret) } else { word_start_before(&edit.text, edit.caret) }
                } else if modifiers.ctrl {
                    if forward { word_start_after(&edit.text, edit.caret) } else { word_start_before(&edit.text, edit.caret) }
                } else if has_selection && !modifiers.shift {
                    let (start, end) = selection_bounds(edit.anchor, edit.caret);
                    if forward { end } else { start }
                } else if forward {
                    next_char_boundary(&edit.text, edit.caret)
                } else {
                    prev_char_boundary(&edit.text, edit.caret)
                };
                if !modifiers.shift {
                    edit.anchor = edit.caret;
                }
            }
            "ArrowUp" | "ArrowDown" => {
                let down = key == "ArrowDown";
                let laid = node.state.edit_layout.as_ref().filter(|laid| laid.multi && laid.text == edit.text);
                let target = if modifiers.meta {
                    Some(if down { edit.text.len() } else { 0 })
                } else if let Some(laid) = laid {
                    let moved = laid.vertical(edit.caret, node.state.caret_goal_x, down);
                    if let Some((_, goal)) = moved {
                        node.state.caret_goal_x = Some(goal);
                    }
                    moved.map(|(byte, _)| byte)
                } else {
                    vertical_target(&edit.text, edit.caret, down)
                };
                let Some(target) = target else { return out };
                edit.caret = target;
                if !modifiers.shift {
                    edit.anchor = target;
                }
            }
            "Home" | "End" => {
                let end = key == "End";
                edit.caret = match (modifiers.ctrl, end) {
                    (true, false) => 0,
                    (true, true) => edit.text.len(),
                    (false, false) => line_start(&edit.text, edit.caret),
                    (false, true) => line_end(&edit.text, edit.caret),
                };
                if !modifiers.shift {
                    edit.anchor = edit.caret;
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
                    let start = if modifiers.meta {
                        line_start(&edit.text, edit.caret).min(prev_char_boundary(&edit.text, edit.caret))
                    } else if modifiers.ctrl || modifiers.alt {
                        word_start_before(&edit.text, edit.caret)
                    } else {
                        prev_char_boundary(&edit.text, edit.caret)
                    };
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
                    let end = if modifiers.ctrl || modifiers.alt { word_end_after(&edit.text, edit.caret).max(next_char_boundary(&edit.text, edit.caret)) } else { next_char_boundary(&edit.text, edit.caret) };
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

}
//#endregion 🔖️UiCommand
// #endregion events
