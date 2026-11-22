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
        Some(UiCommand::Scene { window_id: self.window_id.clone(), node: id, surface_id: scene.surface_id.clone(), kind: scene.component_kind, rect, event: event.clone(), click_count: self.click_count() })
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

    /// 📊️ A focused document-table row answers React's `TableView` keys through the shared table model: Up/Down/PageUp/PageDown/Home/End move focus over the table's rows, Enter and Space activate the row (`Trigger::Activate`); Escape and every other key stay with the generic routes.
    pub(super) fn focused_table_row_key(&mut self, tree: &mut UiTree, key: &str, modifiers: EventModifiers, commands: &mut Vec<UiCommand>) -> bool {
        if modifiers.ctrl || modifiers.meta || modifiers.alt {
            return false;
        }
        let Some(row) = self.focus.focused else { return false };
        if semio_ui_wgpu_mounted_layout::document_table_row(tree, row).is_none() {
            return false;
        }
        let Some(table) = tree.node(row).and_then(|node| node.parent) else { return false };
        let rows: Vec<NodeId> = tree.children(table).filter(|id| semio_ui_wgpu_mounted_layout::document_table_row(tree, *id).is_some()).collect();
        let Some(current) = rows.iter().position(|id| *id == row) else { return false };
        let row_height = tree.accepted_layout(row).map_or(0.0, |layout| layout.height).max(1.0);
        let page = tree.accepted_layout(table).map_or(1, |layout| ((layout.height / row_height) as usize).saturating_sub(1).max(1));
        let key = if key == "NumpadEnter" { "Enter" } else { key };
        let outcome = semio_ui_wgpu_layout::table::table_key(key, modifiers.shift, false, current, rows.len(), page);
        if let Some(next) = outcome.focus.and_then(|index| rows.get(index).copied()) {
            if let Some((blurred, fired)) = self.focus.set_focus(tree, Some(next), true) {
                self.push_app_command(tree, blurred, fired, commands);
            }
            commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: self.focus.focused });
            return true;
        }
        if outcome.activate {
            commands.extend(self.dispatch_accessibility(tree, row, &AccessibilityUiEvent::Activate));
            return true;
        }
        false
    }

    pub(super) fn focused_disclosure_activation(&mut self, tree: &mut UiTree, key: &str) -> bool {
        if !matches!(key, "Enter" | "NumpadEnter" | " ") {
            return false;
        }
        self.focus.focused.is_some_and(|id| self.toggle_disclosure(tree, id))
    }

    /// 🔘️ Picks the segment of the segmented select `id` under pointer `x` — a click on the chosen segment changes nothing, as a radio cannot press off.
    pub(super) fn segmented_select_click(&mut self, tree: &mut UiTree, id: NodeId, x: f32, out: &mut Vec<UiCommand>) {
        let (Some(bounds), Some(UiNode::Select(select))) = (absolute_rect(tree, id), tree.node(id).map(|node| &node.spec.0)) else { return };
        let Some(item) = select::select_segment_at(bounds, select.items.len(), x, self.flow.inline.is_rtl()).map(|index| &select.items[index]).filter(|item| item.value != select.value) else { return };
        if let Some(fired) = fired_action(&select.on_change, Trigger::Change, DslValue::String(item.value.clone())) { self.push_app_command(tree, id, fired, out); }
    }

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
            let Some(next) = select::select_segment_key(select.items.iter().position(|item| item.value == select.value), select.items.len(), key, self.flow.inline.is_rtl(), |_| false) else { return matches!(key, "ArrowLeft" | "ArrowRight" | "Home" | "End") };
            if let Some(fired) = fired_action(&select.on_change, Trigger::Change, DslValue::String(select.items[next].value.clone())) { self.push_app_command(tree, id, fired, out); }
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
                let committed = semio_ui_retained_tree::reconcile::select_commit_value(tree, id, value);
                if let Some(fired) = fired_action(&on_change, Trigger::Change, DslValue::String(committed)) {
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

    /// 🗓️ The release of a press over a popup picker's own parts: a date or colour field toggles its popup (an opening date popup centres on the field's day), the previous and next buttons step the shown month without closing, a colour swatch commits its colour and closes. `false` for every other node, which the generic activation then handles.
    pub(super) fn picker_release(&mut self, tree: &mut UiTree, id: NodeId, out: &mut Vec<UiCommand>) -> bool {
        use semio_ui_wgpu_layout::date_picker::{is_picker_kind, CivilDate};
        let Some(node) = tree.node(id) else { return false };
        match &node.spec.0 {
            UiNode::Input(input) if is_picker_kind(&input.input_kind) => {
                if input.presence.state == UiState::Disabled {
                    return true;
                }
                out.extend(self.toggle_select_popup(tree, id));
                true
            }
            UiNode::Input(input) if input.input_kind == "colorSwatch" => {
                let (change, parent) = (input.on_change.clone(), node.parent);
                if let Some(fired) = fired_action(&change, Trigger::Change, DslValue::String(input.value.clone())) {
                    self.push_app_command(tree, id, fired, out);
                }
                if let Some(parent) = parent {
                    out.extend(self.close_overlay(tree, parent));
                }
                true
            }
            UiNode::Button(button) => {
                let step = match button.id.as_deref() {
                    Some("picker.1") => -1,
                    Some("picker.2") => 1,
                    _ => return false,
                };
                let Some(parent) = node.parent else { return false };
                let Some(parent_node) = tree.node(parent) else { return false };
                let UiNode::Input(input) = &parent_node.spec.0 else { return false };
                if input.input_kind != "date" {
                    return false;
                }
                let moved: Option<CivilDate> = semio_ui_wgpu_layout::date_picker::focus_date(&input.value, parent_node.state.picker_day).and_then(|shown| shown.plus_months(step));
                if let (Some(moved), Some(parent_node)) = (moved, tree.node_mut(parent)) {
                    parent_node.state.picker_day = Some(moved.epoch_day());
                }
                tree.mark_dirty(parent, NodeFlags::DIRTY_PAINT);
                true
            }
            _ => false,
        }
    }

    /// ⌨️ The keys of a focused `date` or `color` field, as the browser's own pickers answer them. Closed: Enter, Space and ArrowDown open the popup. A date popup: arrows one day or week, Page one month (Shift: one year), Home and End the week's ends move the focused day (the shown month follows), Enter and Space pick it and close. A colour popup: arrows walk the swatches, Home and End jump, Enter and Space pick the highlighted one. Escape stays with the generic overlay close; `Tab` closes and moves on.
    pub(super) fn route_picker_key(&mut self, tree: &mut UiTree, key: &str, modifiers: EventModifiers, out: &mut Vec<UiCommand>) -> bool {
        use semio_ui_wgpu_layout::colour_picker::{colour_value, swatch_key, COLOUR_SWATCHES};
        use semio_ui_wgpu_layout::date_picker::{date_key, focus_date, is_picker_kind, DateKey};
        let Some(id) = self.focus.focused else { return false };
        let Some(node) = tree.node(id) else { return false };
        let UiNode::Input(input) = &node.spec.0 else { return false };
        if !is_picker_kind(&input.input_kind) || input.presence.state == UiState::Disabled || modifiers.ctrl || modifiers.meta {
            return false;
        }
        let key = if key == "NumpadEnter" { "Enter" } else { key };
        let (open, kind, change, value, day, monday, highlighted) = (node.state.open, input.input_kind.clone(), input.on_change.clone(), input.value.clone(), node.state.picker_day, node.state.picker_week_monday, node.state.highlighted);
        if !open {
            if matches!(key, "Enter" | " " | "ArrowDown") {
                out.extend(self.toggle_select_popup(tree, id));
                return true;
            }
            return false;
        }
        if key == "Tab" {
            out.extend(self.close_overlay(tree, id));
            return false;
        }
        let pick = |router: &mut Self, tree: &mut UiTree, picked: String, out: &mut Vec<UiCommand>| {
            if let Some(fired) = fired_action(&change, Trigger::Change, DslValue::String(picked)) {
                router.push_app_command(tree, id, fired, out);
            }
            out.extend(router.close_overlay(tree, id));
        };
        if kind == "date" {
            match focus_date(&value, day).and_then(|focus| date_key(key, modifiers.shift, focus, monday)) {
                Some(DateKey::Focus(moved)) => {
                    if let Some(node) = tree.node_mut(id) {
                        node.state.picker_day = Some(moved.epoch_day());
                    }
                    tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
                    true
                }
                Some(DateKey::Select(picked)) => {
                    pick(self, tree, picked.iso(), out);
                    true
                }
                Some(DateKey::Close) | None => false,
            }
        } else {
            let current = highlighted.or_else(|| colour_value(&value).and_then(|hex| COLOUR_SWATCHES.iter().position(|swatch| *swatch == hex))).unwrap_or(0);
            if matches!(key, "Enter" | " ") {
                if let Some(hex) = highlighted.and_then(|slot| COLOUR_SWATCHES.get(slot)) {
                    pick(self, tree, (*hex).to_string(), out);
                    return true;
                }
                return false;
            }
            let Some(next) = swatch_key(key, current) else { return false };
            if let Some(node) = tree.node_mut(id) {
                node.state.highlighted = Some(next);
            }
            tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
            true
        }
    }

    /// 🔤️ Appends `char` to the live typeahead query, restarting it when the previous keystroke is
    /// older than `select::SELECT_TYPEAHEAD_RESET_SECONDS`.
    pub(super) fn extend_select_typeahead(&mut self, char: char) -> String {
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

    /// 👆️ The `UiCommand::App` a press/drag at `(x, y)` over `id` commits, for the pointer-valued
    /// control kinds — see `pointer_commit_action`. The rect it measures the gesture against is the
    /// node's own absolute painted rect, the same one `paint` drew the knob/segments at.
    pub(super) fn pointer_commit(&mut self, tree: &mut UiTree, id: NodeId, x: f32, y: f32, press: bool) -> Option<UiCommand> {
        let bounds = absolute_rect(tree, id)?;
        let inline_suffix_width = tree.accepted_inline_suffix_width(id).unwrap_or(0.0);
        let node = tree.node(id)?;
        if press && matches!(node.spec.0, UiNode::Ring(_)) {
            return None;
        }
        let (local_delta, local_slider) = match &node.spec.0 {
            UiNode::Slider(slider) => {
                let unit_width = slider.unit_label().map(|_| inline_suffix_width);
                let track = slider_control_presentation(bounds, slider.axis_position(slider_live_value(node, slider)), 0.0, 1.0, unit_width, self.control_gap, self.flow.inline).slider.track_cell;
                (None, track.contains(x, y).then(|| semio_ui_wgpu_controls::slider::slider_node_pointer_value(slider, track, x, y)))
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
