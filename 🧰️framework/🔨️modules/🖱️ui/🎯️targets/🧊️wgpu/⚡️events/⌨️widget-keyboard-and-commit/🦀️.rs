//! 🐙️ Widget keyboard and commit: the inline key mirror, the scene command, the focused activation, the select and value keys and the pointer commit. See [`super`].

use super::*;

// #region events
//#region 🔖️UiCommand
impl EventRouter {
    /// 🧭️ One key name as this window's INLINE direction means it. React inverts the horizontal pair
    /// per control (`🎚️Slider/🟦️.tsx:383-384`, `📑️Tabs/🟦️.tsx:164-165`, `🎛️ToggleGroup/🟦️.tsx:118-119`
    /// all read `dir === "rtl"`); doing it once here means every value control inherits the rule
    /// instead of each re-deriving it. Vertical and page keys are direction-invariant, and a caret
    /// move inside text is NOT routed through here — `route_edit_key` walks byte boundaries, which
    /// are already logical.
    pub fn mirrored_inline_key<'k>(&self, key: &'k str) -> &'k str {
        if !self.flow.inline.is_rtl() {
            return key;
        }
        match key {
            "ArrowLeft" => "ArrowRight",
            "ArrowRight" => "ArrowLeft",
            other => other,
        }
    }

    /// 🎬️ If `id` is a `ComponentScene` leaf, resolves its `SurfaceKind`/absolute rect (the same
    /// ancestor-offset accumulation `scene_slots::collect_scene_slots`/`hit_test_node`/
    /// `paint::paint_node` each do independently — not reusing `collect_scene_slots` itself, since
    /// that walks the WHOLE tree per call and this runs once per real input event) and builds the
    /// `UiCommand::Scene` the host should route into that surface's per-`SurfaceKind` input handler.
    pub(super) fn scene_command(&self, tree: &UiTree, id: NodeId, event: &UiEvent) -> Option<UiCommand> {
        let node = tree.node(id)?;
        let UiNode::ComponentScene(scene) = &node.spec.0 else { return None };
        let rect = absolute_rect(tree, id)?;
        Some(UiCommand::Scene { window_id: self.window_id.clone(), node: id, surface_id: scene.surface_id.clone(), kind: scene.component_kind, rect, event: event.clone() })
    }

    /// ⏎️ The action `Enter` or `Space` activates on the focused enabled `Button` — the keyboard half of a click, as a
    /// native `<button>` answers it ([WAI-ARIA button pattern](https://www.w3.org/WAI/ARIA/apg/patterns/button/)).
    pub(super) fn focused_button_activation(&self, tree: &UiTree, key: &str) -> Option<(NodeId, FiredAction)> {
        if !matches!(key, "Enter" | "NumpadEnter" | " ") {
            return None;
        }
        let id = self.focus.focused?;
        match &tree.node(id)?.spec.0 {
            UiNode::Button(button) if button.presence.state != UiState::Disabled => bare_action(&button.action, Trigger::Activate).map(|fired| (id, fired)),
            _ => None,
        }
    }

    pub(super) fn focused_disclosure_activation(&mut self, tree: &mut UiTree, key: &str) -> bool {
        if !matches!(key, "Enter" | "NumpadEnter" | " ") {
            return false;
        }
        self.focus.focused.is_some_and(|id| self.toggle_disclosure(tree, id))
    }

    //#region 🔖️WidgetKeyboard
    /// 🔽️ Routes one key at the focused `Select`, returning whether it was consumed. Opens the popup
    /// (highlighting the selected/last/first-matching row), moves the highlight, commits it, closes
    /// on `Tab`, or extends the typeahead query — the union of React's `SelectTrigger` and
    /// `SelectContent` `onKeyDown` handlers, whose decision table lives with the element
    /// (`select::select_key`). `Tab` closes but reports `false`, so the same keystroke still moves
    /// focus the way it does in the DOM.
    ///
    /// ↹️ `finish_close` drops focus that sat inside the closed overlay's subtree, and a
    /// `Select` IS its own popup root — so put focus back on the trigger before the `Tab`
    /// handler runs, or the tab move would restart from the top of the document instead
    /// of stepping off this control, which is what React's trigger-retained focus does.
    pub(super) fn route_select_key(&mut self, tree: &mut UiTree, key: &str, modifiers: EventModifiers, out: &mut Vec<UiCommand>) -> bool {
        let Some(id) = self.focus.focused else { return false };
        let Some(node) = tree.node(id) else { return false };
        let UiNode::Select(select) = &node.spec.0 else { return false };
        if select.presence.state == UiState::Disabled {
            return false;
        }
        if select_is_segmented(tree, id) {
            let step = match self.mirrored_inline_key(key) {
                "ArrowRight" | "ArrowDown" => 1,
                "ArrowLeft" | "ArrowUp" => select.items.len().saturating_sub(1),
                _ => return false,
            };
            if select.items.is_empty() {
                return true;
            }
            let next = select.items.iter().position(|item| item.value == select.value).map_or(0, |current| (current + step) % select.items.len());
            if let Some(fired) = fired_action(&select.on_change, Trigger::Change, DslValue::String(select.items[next].value.clone())) {
                self.push_app_command(tree, id, fired, out);
            }
            return true;
        }
        let open = node.state.open;
        let highlighted = node.state.highlighted;
        let labels: Vec<String> = select.items.iter().map(|item| item.label.as_str().to_string()).collect();
        let values: Vec<String> = select.items.iter().map(|item| item.value.clone()).collect();
        let selected = values.iter().position(|value| value == &select.value);
        let on_change = select.on_change.clone();
        match select::select_key(open, key, modifiers.alt, modifiers.ctrl, modifiers.meta) {
            select::SelectKey::Ignored => false,
            select::SelectKey::Open(intent) => {
                let index = select::select_open_index(intent, labels.iter().map(String::as_str), selected);
                self.select_typeahead = match intent {
                    select::SelectOpenIntent::Typeahead(char) => Some((char.to_string(), self.clock_seconds)),
                    _ => None,
                };
                out.extend(self.toggle_select_popup(tree, id));
                self.set_select_highlight(tree, id, index);
                true
            }
            select::SelectKey::Move(movement) => {
                let index = select::select_moved_index(highlighted, labels.len(), movement);
                self.select_typeahead = None;
                self.set_select_highlight(tree, id, index);
                true
            }
            select::SelectKey::Commit => {
                let Some(value) = highlighted.and_then(|index| values.get(index)) else { return true };
                if let Some(fired) = fired_action(&on_change, Trigger::Change, DslValue::String(value.clone())) {
                    self.push_app_command(tree, id, fired, out);
                }
                self.select_typeahead = None;
                out.extend(self.close_overlay(tree, id));
                true
            }
            select::SelectKey::Close => {
                self.select_typeahead = None;
                out.extend(self.close_overlay(tree, id));
                if self.focus.focused.is_none() {
                    if let Some((blurred, fired)) = self.focus.set_focus(tree, Some(id), true) {
                        self.push_app_command(tree, blurred, fired, out);
                    }
                }
                false
            }
            select::SelectKey::Typeahead(char) => {
                let query = self.extend_select_typeahead(char);
                let index = select::select_typeahead_index(labels.iter().map(String::as_str), &query, highlighted);
                if index.is_some() {
                    self.set_select_highlight(tree, id, index);
                }
                true
            }
        }
    }

    /// 🔤️ Appends `char` to the live typeahead query, restarting it when the previous keystroke is
    /// older than `select::SELECT_TYPEAHEAD_RESET_SECONDS`.
    fn extend_select_typeahead(&mut self, char: char) -> String {
        let fresh = self.select_typeahead.as_ref().is_some_and(|(_, at)| self.clock_seconds - at < f64::from(select::SELECT_TYPEAHEAD_RESET_SECONDS));
        let mut query = if fresh { self.select_typeahead.take().map(|(query, _)| query).unwrap_or_default() } else { String::new() };
        query.push(char);
        self.select_typeahead = Some((query.clone(), self.clock_seconds));
        query
    }

    /// 🔦️ Writes the popup's highlighted row and repaints it — `paint`'s only input for the
    /// keyboard-active row (`tree::WidgetState::highlighted`).
    fn set_select_highlight(&mut self, tree: &mut UiTree, id: NodeId, index: Option<usize>) {
        if let Some(node) = tree.node_mut(id) {
            node.state.highlighted = index;
            if let (Some(index), Some(popup)) = (index, node.state.select_popup) {
                node.state.scroll_offset.1 = select::select_revealed_scroll(popup, index);
            }
        }
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
    }

    /// ⌨️ The value-committing keyboard gestures the other focusable controls answer, which a plain
    /// `<button>`'s Enter/Space (see `focused_button_activation`) does not cover: a `Toggle` flips
    /// (React renders it as a button, so Enter/Space click it), a `Slider` steps
    /// (`slider_key_value`), and a `NumberStepper` answers the field-key law ([`number_field_key`]) through
    /// `ui_number_key_value` exactly as React's `Stepper` does: every press reports — an absolute stepper the law's value (its
    /// bound again at the bound), a relative one its step as a delta (an arrow, ten rungs with `Shift`) or the move (a page key,
    /// `Home`, `End`) — and shows the law's value with any earlier refusal gone.
    pub(super) fn focused_value_key_activation(&mut self, tree: &mut UiTree, key: &str, modifiers: EventModifiers) -> Option<(NodeId, FiredAction)> {
        let id = self.focus.focused?;
        let node = tree.node(id)?;
        if node.spec.0.presence().state == UiState::Disabled {
            return None;
        }
        let (fired, local_stepper, local_slider) = match &node.spec.0 {
            UiNode::Toggle(toggle) if matches!(key, "Enter" | "NumpadEnter" | " ") => (fired_action(&toggle.on_change, Trigger::Change, DslValue::Bool(!toggle.presence.selected)), None, None),
            UiNode::Slider(slider) if node.state.edit.is_none() => {
                let value = slider_key_value_from(slider, slider_live_value(node, slider), key, modifiers.shift)?;
                (fired_action(&slider.on_change, Trigger::Change, DslValue::float(value)), None, Some(value))
            }
            UiNode::NumberStepper(stepper) => {
                let (law_key, large) = number_field_key(key, modifiers.shift, stepper.min, stepper.max)?;
                let live = number_stepper_live_value(node, stepper);
                let next = ui_contract::ui_number_key_value(live, stepper.min, stepper.max, stepper.step, stepper.precision, stepper.display_factor, stepper.snaps.iter().copied(), law_key, large);
                let binds_delta = node.intent.as_ref().map_or_else(|| !stepper.on_delta.action.is_empty(), |intent| intent.binds(Trigger::Delta));
                let fired = if binds_delta {
                    let rungs = if large { ui_contract::SLIDER_PAGE_STEPS } else { 1.0 };
                    let delta = match law_key {
                        ui_contract::SliderKey::Increment => stepper.step * rungs,
                        ui_contract::SliderKey::Decrement => -stepper.step * rungs,
                        _ => next - live,
                    };
                    fired_action(&stepper.on_delta, Trigger::Delta, DslValue::float(delta))
                } else {
                    fired_action(&stepper.on_absolute, Trigger::Change, DslValue::float(next))
                };
                (fired, Some(next), None)
            }
            _ => (None, None, None),
        };
        if let Some(value) = local_slider {
            self.apply_slider_local_value(tree, id, value);
        }
        if let Some(value) = local_stepper {
            self.apply_stepper_local_value(tree, id, value);
        }
        let fired = fired?;
        let continuous = fired.trigger == Trigger::Change && (local_slider.is_some() || local_stepper.is_some());
        Some((id, if continuous { pressed(tree, &self.window_id, id, fired, true) } else { fired }))
    }
    //#endregion 🔖️WidgetKeyboard

    /// 👆️ The `UiCommand::App` a press/drag at `(x, y)` over `id` commits, for the pointer-valued
    /// control kinds — see `pointer_commit_action`. The rect it measures the gesture against is the
    /// node's own absolute painted rect, the same one `paint` drew the knob/segments at.
    pub(super) fn pointer_commit(&mut self, tree: &mut UiTree, id: NodeId, x: f32, y: f32) -> Option<UiCommand> {
        let bounds = absolute_rect(tree, id)?;
        let inline_suffix_width = tree.accepted_inline_suffix_width(id).unwrap_or(0.0);
        let node = tree.node(id)?;
        let (local_delta, local_slider) = match &node.spec.0 {
            UiNode::Slider(slider) => {
                let unit_width = slider.unit_label().map(|_| inline_suffix_width);
                let track = slider_control_presentation(bounds, slider.axis_position(slider_live_value(node, slider)), 0.0, 1.0, unit_width, self.control_gap, self.flow.inline).slider.track_cell;
                (None, track.contains(x, y).then(|| semio_framework_ui_wgpu_controls::slider::slider_node_pointer_value(slider, track, x, y)))
            }
            UiNode::NumberStepper(stepper) => (number_stepper_sign_at(bounds, x, y, self.flow.inline, self.control_border).filter(|sign| number_stepper_can_step(node, stepper, *sign)).map(|sign| sign * stepper.step), None),
            _ => (None, None),
        };
        let (is_dragged, is_stepper) = (matches!(node.spec.0, UiNode::Slider(_) | UiNode::Ring(_)), matches!(node.spec.0, UiNode::NumberStepper(_)));
        let fired = pointer_commit_action(node, bounds, x, y, self.flow.inline, self.control_border, inline_suffix_width, self.control_gap).map(|fired| match fired.trigger {
            Trigger::Change if is_dragged => pressed(tree, &self.window_id, id, fired, false),
            Trigger::Change if is_stepper => pressed(tree, &self.window_id, id, fired, true),
            _ => fired,
        });
        let command = fired.and_then(|fired| self.app_command(tree, id, fired));
        if let Some(value) = local_slider {
            self.apply_slider_local_value(tree, id, value);
        }
        if let Some(delta) = local_delta {
            self.apply_stepper_local_delta(tree, id, delta);
        }
        command
    }
}
//#endregion 🔖️UiCommand
// #endregion events
