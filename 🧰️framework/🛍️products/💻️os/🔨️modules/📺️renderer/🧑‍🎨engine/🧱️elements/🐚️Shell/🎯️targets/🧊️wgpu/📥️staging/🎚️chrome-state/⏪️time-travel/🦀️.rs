//! 🎚️ `shell-chrome-state` staging of `time-travel`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ⏪️TimeTravelControls
/// ✍️ Whether a time-travel chord yields to the focused retained control: a text or number field keeps its keys,
/// because hotkeys never fire while the user is typing (React's `isEditableEventTarget`).
pub fn time_travel_chord_yields_to(focus: Option<RetainedNodeFocusKind>) -> bool {
    focus == Some(RetainedNodeFocusKind::Input)
}

//#endregion ⏪️TimeTravelControls
impl ChromeState {
    /// ✋️ Whether moving focus now would take it from someone typing elsewhere — React's `timeTravelFocusIsHeldV1`: a
    /// chrome text field outside the finalize prompt, or a retained input outside the History panel, keeps its focus.
    pub fn time_travel_focus_is_held(&self, input: &InputState<ActionDescriptor>) -> bool {
        let chrome = input.focused_id.is_some() && !self.chrome_build.dialog_open();
        let retained = self.chrome_build.focused_retained_surface.as_deref().is_some_and(|surface| surface != FRAMEWORK_PANEL_TAB_HISTORY_ID && self.chrome_build.content_focus.get(surface).and_then(Option::as_ref).is_some_and(|focus| focus.kind == RetainedNodeFocusKind::Input));
        chrome || retained
    }

    /// ✍️ Whether a retained text or number field holds keyboard focus.
    pub fn retained_text_field_has_focus(&self) -> bool {
        time_travel_chord_yields_to(self.chrome_build.focused_retained_surface.as_ref().and_then(|surface| self.chrome_build.content_focus.get(surface)).and_then(Option::as_ref).map(|focus| focus.kind))
    }
}

