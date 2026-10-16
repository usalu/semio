//! 📄️ `shell-docs` staging of `time-travel`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::time_travel::*;
use semio_framework::kernel::{HistoryReprojection, HistoryReprojectionStatus, HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage};

//#region ⏪️TimeTravelControls
/// ✅️ Whether `verb` is offered and enabled in the session's stage — the one gate the band and the chords share.
pub(crate) fn time_travel_verb_enabled(status: &HistoryTimeTravel, verb: TimeTravelVerb) -> bool {
    time_travel_band_controls(status).into_iter().any(|control| control.verb == verb && control.disabled_by.is_none())
}

//#endregion ⏪️TimeTravelControls
impl DocumentState {
    /// ⌨️ The dispatch a `ui.timeTravel.*` chord asks for, or `None` when it must fall through: no session, a verb the
    /// stage does not offer or has disabled, or a focused text field (hotkeys never fire while the user is typing).
    pub(crate) fn time_travel_shortcut_action(&self, verb: TimeTravelVerb, chrome: &ChromeState, history: &HistoryState) -> Option<ActionDescriptor> {
        let status = history.history_time_travel.as_ref()?;
        if !time_travel_verb_enabled(status, verb) || chrome.retained_text_field_has_focus() {
            return None;
        }
        Some(verb.action(&self.session.as_ref()?.app.controller_id, status))
    }
}

