//! 🕹️ `shell-input` staging of `chrome-dialog-and-tooltips`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl ShellInputState {
    /// 👆️ Applies one click on the open dialog: a field takes focus (a toggle flips, a select advances, a
    /// slider takes the pointer's value through the shared detent law, a reference field stages the live
    /// selection), a segment picks its option, a stepper button steps, a chip removes its reference, an
    /// enabled button closes the dialog while the hit pipeline dispatches its event, a gated one does
    /// nothing, and a veil click cancels — sending the cancel action exactly as the Cancel button does.
    pub fn resolve_chrome_dialog_click(&mut self, ops: &[ChromeDialogPaintOp], dialog: Rect, x: f32, y: f32, chrome: &mut ChromeState, settle: &mut SettleState) {
        let press = ops.iter().find_map(|op| match op {
            ChromeDialogPaintOp::Hit { rect, stop, event, .. } if rect.contains(x, y) => Some(ChromeDialogPress::Stop { stop: *stop, enabled: event.is_some(), rect: *rect }),
            ChromeDialogPaintOp::Segment { rect, field, option, .. } if rect.contains(x, y) => Some(ChromeDialogPress::Segment { field: *field, option: *option }),
            ChromeDialogPaintOp::Nudge { rect, field, sign, .. } if rect.contains(x, y) => Some(ChromeDialogPress::Nudge { field: *field, sign: *sign }),
            _ => None,
        });
        let selection = self.interaction_selection.clone();
        let Some(request) = chrome.chrome_build.dialog_stack.last_mut() else { return };
        match press {
            Some(ChromeDialogPress::Stop { stop: ChromeDialogStop::Field(index), rect, .. }) => {
                request.focus_stop(ChromeDialogStop::Field(index));
                match request.fields[index].kind {
                    ChromeDialogFieldKind::Toggle => request.flip(index),
                    ChromeDialogFieldKind::Select { .. } => request.cycle(index, true),
                    ChromeDialogFieldKind::Slider { .. } => request.point_slider(index, f64::from((x - rect.x) / rect.w.max(1.0))),
                    ChromeDialogFieldKind::Reference { .. } => request.use_selection(index, &selection),
                    _ => {}
                }
            }
            Some(ChromeDialogPress::Stop { stop: ChromeDialogStop::Chip(index, chip), .. }) => request.remove_chip(index, chip),
            Some(ChromeDialogPress::Segment { field, option }) => {
                request.focus_stop(ChromeDialogStop::Field(field));
                request.choose(field, option);
            }
            Some(ChromeDialogPress::Nudge { field, sign }) => {
                request.focus_stop(ChromeDialogStop::Field(field));
                request.nudge(field, sign);
            }
            Some(ChromeDialogPress::Stop { stop: ChromeDialogStop::Cancel, .. } | ChromeDialogPress::Stop { enabled: true, .. }) => chrome.chrome_build.close_topmost_dialog(),
            Some(ChromeDialogPress::Stop { .. }) => {}
            None if !dialog.contains(x, y) => {
                let cancel = request.action(ChromeDialogStop::Cancel);
                chrome.chrome_build.close_topmost_dialog();
                settle.deferred_actions.extend(cancel);
            }
            None => {}
        }
    }

    /// ⌨️ The open dialog owns the keyboard: Escape cancels, Tab walks fields → Cancel → choices →
    /// submit, Enter (or Space) takes the focused button and submits from a field, Space flips a toggle,
    /// arrows move a select or a segmented choice, step a number or axis, and move a slider (PageUp/PageDown
    /// jump between its detents, Home/End to its ends), and typed text edits a text, number or axis field.
    /// On a reference field Enter/Space stage the current selection and Backspace/Delete drop the last
    /// reference; on a chip Enter/Space/Backspace/Delete remove it. Always consumes the key — a modal never
    /// lets one fall through to the shell behind it.
    pub fn handle_chrome_dialog_key(&mut self, action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers, chrome: &mut ChromeState, settle: &mut SettleState) -> bool {
        use ui_wgpu::wgpu::KeyAction;
        let selection = self.interaction_selection.clone();
        let Some(request) = chrome.chrome_build.dialog_stack.last_mut() else { return false };
        let stop = request.focused();
        let field = match stop {
            ChromeDialogStop::Field(index) => Some((index, request.fields[index].kind.clone(), request.fields[index].draft.clone())),
            _ => None,
        };
        let take = |request: &ChromeDialogRequest, stop: ChromeDialogStop| match stop {
            ChromeDialogStop::Cancel => Some(request.action(stop)),
            ChromeDialogStop::Field(_) => request.action(ChromeDialogStop::Confirm).map(Some),
            _ => request.action(stop).map(Some),
        };
        let taken = match (action, &field) {
            (KeyAction::Escape, _) => Some(request.action(ChromeDialogStop::Cancel)),
            (KeyAction::Tab, _) => {
                request.move_focus(modifiers.shift);
                None
            }
            (KeyAction::Enter | KeyAction::Space(_) | KeyAction::Backspace | KeyAction::Delete, None) if matches!(stop, ChromeDialogStop::Chip(..)) => {
                if let ChromeDialogStop::Chip(index, chip) = stop {
                    request.remove_chip(index, chip);
                }
                None
            }
            (KeyAction::Enter | KeyAction::Space(_), Some((index, ChromeDialogFieldKind::Reference { .. }, _))) => {
                request.use_selection(*index, &selection);
                None
            }
            (KeyAction::Backspace | KeyAction::Delete, Some((index, ChromeDialogFieldKind::Reference { .. }, _))) => {
                let last = request.reference_ids(*index).len().checked_sub(1);
                if let Some(last) = last {
                    request.remove_chip(*index, last);
                }
                None
            }
            (KeyAction::Enter, _) => take(request, stop),
            (KeyAction::Space(_), Some((index, ChromeDialogFieldKind::Toggle, _))) => {
                request.flip(*index);
                None
            }
            (KeyAction::Space(_), Some((index, ChromeDialogFieldKind::Select { .. } | ChromeDialogFieldKind::Segmented { .. }, _))) => {
                request.cycle(*index, true);
                None
            }
            (KeyAction::Space(_), Some((_, ChromeDialogFieldKind::Slider { .. }, _))) => None,
            (KeyAction::Space(_), Some((index, _, draft))) => {
                request.set_draft(*index, format!("{draft} "));
                None
            }
            (KeyAction::Space(_), None) => take(request, stop),
            (KeyAction::ArrowLeft | KeyAction::ArrowUp | KeyAction::ArrowRight | KeyAction::ArrowDown, Some((index, ChromeDialogFieldKind::Select { .. } | ChromeDialogFieldKind::Segmented { .. }, _))) => {
                request.cycle(*index, matches!(action, KeyAction::ArrowRight | KeyAction::ArrowDown));
                None
            }
            (_, Some((index, ChromeDialogFieldKind::Slider { .. }, _))) => {
                request.slide(*index, action, modifiers.shift);
                None
            }
            (KeyAction::PageUp | KeyAction::PageDown, Some((index, ChromeDialogFieldKind::Number { .. } | ChromeDialogFieldKind::Axis { .. }, _))) => {
                request.nudge_by(*index, if matches!(action, KeyAction::PageUp) { 1.0 } else { -1.0 }, true);
                None
            }
            (KeyAction::ArrowUp | KeyAction::ArrowDown, Some((index, ChromeDialogFieldKind::Number { .. } | ChromeDialogFieldKind::Axis { .. }, _))) => {
                request.nudge(*index, if matches!(action, KeyAction::ArrowUp) { 1.0 } else { -1.0 });
                None
            }
            (KeyAction::Char(text), Some((index, ChromeDialogFieldKind::Text | ChromeDialogFieldKind::Number { .. } | ChromeDialogFieldKind::Axis { .. } | ChromeDialogFieldKind::Color { .. }, draft))) => {
                request.set_draft(*index, format!("{draft}{text}"));
                None
            }
            (KeyAction::Backspace, Some((index, ChromeDialogFieldKind::Text | ChromeDialogFieldKind::Number { .. } | ChromeDialogFieldKind::Axis { .. } | ChromeDialogFieldKind::Color { .. }, draft))) => {
                let mut draft = draft.clone();
                draft.pop();
                request.set_draft(*index, draft);
                None
            }
            _ => None,
        };
        if let Some(dispatch) = taken {
            chrome.chrome_build.close_topmost_dialog();
            settle.deferred_actions.extend(dispatch);
        }
        true
    }

    /// 🦮️ One accessibility event on the open dialog's tree, answered exactly as its keyboard and pointer are: focus
    /// moves the dialog's focus stop, activation presses the control (a toggle flips, a select advances, a segment
    /// picks its option, a stepper button steps, a reference field stages the live selection, a chip removes itself,
    /// an enabled button dispatches and closes, a gated one does nothing) and a value edits a text, number or axis
    /// field or sets a slider. `false` when `key` names nothing in the open dialog.
    pub fn handle_chrome_dialog_accessibility_event(&mut self, key: &str, event: &ui_render::AccessibilityEvent, chrome: &mut ChromeState, settle: &mut SettleState) -> bool {
        let selection = self.interaction_selection.clone();
        let Some(request) = chrome.chrome_build.dialog_stack.last_mut() else { return false };
        if key == request.accessibility_key() {
            return true;
        }
        let Some((stop, option, sign)) = request.accessibility_target(key) else { return false };
        let mut taken = None;
        match (event, stop) {
            (ui_render::AccessibilityEvent::Focus, _) => request.focus_stop(stop),
            (ui_render::AccessibilityEvent::Activate, ChromeDialogStop::Field(index)) => {
                request.focus_stop(stop);
                match option {
                    Some(option) => request.choose(index, option),
                    None if sign != 0.0 => request.nudge(index, sign),
                    None => match request.fields[index].kind {
                        ChromeDialogFieldKind::Toggle => request.flip(index),
                        ChromeDialogFieldKind::Select { .. } | ChromeDialogFieldKind::Segmented { .. } => request.cycle(index, true),
                        ChromeDialogFieldKind::Reference { .. } => request.use_selection(index, &selection),
                        _ => {}
                    },
                }
            }
            (ui_render::AccessibilityEvent::Activate, ChromeDialogStop::Chip(index, chip)) => request.remove_chip(index, chip),
            (ui_render::AccessibilityEvent::Activate, _) => {
                let action = request.action(stop);
                if stop == ChromeDialogStop::Cancel || action.is_some() {
                    taken = Some(action);
                }
            }
            (ui_render::AccessibilityEvent::Value(value), ChromeDialogStop::Field(index)) => match request.fields[index].kind {
                ChromeDialogFieldKind::Slider { .. } => {
                    if let Some(value) = value.trim().parse::<f64>().ok().filter(|value| value.is_finite()) {
                        request.set_slider(index, value);
                    }
                }
                ChromeDialogFieldKind::Text | ChromeDialogFieldKind::Number { .. } | ChromeDialogFieldKind::Axis { .. } | ChromeDialogFieldKind::Color { .. } => request.set_draft(index, value.clone()),
                _ => {}
            },
            _ => {}
        }
        if let Some(dispatch) = taken {
            chrome.chrome_build.close_topmost_dialog();
            settle.deferred_actions.extend(dispatch);
        }
        true
    }
}

//#endregion ShellChrome

