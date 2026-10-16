//! 🔲️ `shell-windows` staging of `window-actions-and-commands`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region 🎬️WindowActionsAndSearchPanes
/// 🔀️ The retained Tree surface owned by one World window instance's Projection pane.
pub(crate) fn window_projection_surface_id(window_id: &str) -> String {
    format!("{window_id}/{WINDOW_PROJECTION_BODY_KEY}")
}

impl WindowState {
    /// 🎬️ Whether this window carries React's `engagement` PROP — `windowEngagementToSpec`'s own
    /// `hasContent`: at least one option, a `control`, a `controls` row or a status line
    /// (`🛠️ShellHelpers/🟦️.tsx:1852`). An engagement carrying only an INPUT is a search spec and no
    /// engagement spec at all, which is why the two panes mount independently.
    pub(crate) fn window_has_engagement_spec(&self, window_id: &str) -> bool {
        self.window_engagements.get(window_id).is_some_and(|engagement| {
            engagement.options.as_ref().is_some_and(|options| !options.is_empty())
                || engagement.control.is_some()
                || engagement.controls.as_ref().is_some_and(|controls| !controls.is_empty())
                || engagement.status.as_ref().is_some_and(|status| !status.is_empty())
        })
    }

    /// 🔎️ Whether this window carries React's `search` PROP — `windowEngagementToSearchSpec`'s own
    /// `hasContent`: a typed input OR at least one autocomplete possible (`🛠️ShellHelpers/🟦️.tsx:1874`).
    /// The BODY still needs the input (React's `Search` returns `null` without one); the PANE mounts
    /// either way, which is why this is a separate question from [`Self::build_window_search_ui`].
    pub(crate) fn window_has_search_spec(&self, window_id: &str) -> bool {
        self.window_engagements.get(window_id).is_some_and(|engagement| engagement.input.is_some() || engagement.possible_engagements.as_ref().is_some_and(|possibles| !possibles.is_empty()))
    }
}

//#endregion 🎬️WindowActionsAndSearchPanes
impl WindowState {
    // #endregion
    // #region active-utility
    /// 🧰️ Applies a user-driven `setActiveUtility`: re-selecting the active utility deactivates it, otherwise
    /// it becomes the active utility for that window kind (Architecture Decision 4).
    ///
    /// 🎓️ Advance-by-doing: only the activation branch counts as "the utility was activated" —
    /// see `chrome_tour_note_utility_performed`.
    pub(crate) fn toggle_active_utility(&mut self, window_id: &str, utility_id: &str, chrome: &mut ChromeState, docs: &mut DocumentState) {
        let next = (!utility_id.is_empty() && self.active_utility_for_window(window_id) != Some(utility_id)).then_some(utility_id);
        self.set_active_utility(window_id, next, chrome, docs);
    }

    /// 🎯️ An explicit guest arm is idempotent and only a changed activation advances its tutorial.
    pub(crate) fn set_active_utility(&mut self, window_id: &str, utility_id: Option<&str>, chrome: &mut ChromeState, docs: &mut DocumentState) {
        if self.active_utility_for_window(window_id) == utility_id { return; }
        match utility_id {
            Some(utility_id) => {
                self.active_utility_by_window.insert(window_id.into(), utility_id.into());
                docs.chrome_tour_note_utility_performed(utility_id, chrome);
            }
            None => { self.active_utility_by_window.remove(window_id); }
        }
    }

    /// 🧰️ The active utility id for a window kind, if any.
    pub(crate) fn active_utility_for_window(&self, window_id: &str) -> Option<&str> {
        self.active_utility_by_window.get(window_id).map(String::as_str)
    }

    /// 🚦️ Whether window-scoped actions stay enabled: `true` when no utility is active or the active utility
    /// declares `allows_actions_while_active` (P5 — replaces the old `UTILITY_ID_PREFIXES` whitelist).
    pub(crate) fn actions_enabled_for_window(&self, app: &AppDefinition, window_id: &str) -> bool {
        match self.active_utility_for_window(window_id) {
            None => true,
            Some(utility_id) => app.utilities.iter().find(|utility| utility.id == utility_id).map(|utility| utility.allows_actions_while_active).unwrap_or(true),
        }
    }

    //#region ShellCommandHistory
    /// 📨️ Arms `hostEvent{windowId, kind: blur}` for the pane that lost the activation — React's pane `onBlur` twin — so
    /// the program ends an open gesture there. The first observation of a session seeds the witness without a blur.
    pub(crate) fn arm_window_blur_host_event(&mut self, settle: &mut SettleState, plugins: &PluginState, docs: &DocumentState) {
        let Some(instance_id) = docs.session.as_ref().map(|session| session.instance_id) else {
            return;
        };
        let Some(active) = self.active_window_id.clone().filter(|id| !id.is_empty()) else {
            return;
        };
        let blurred = self.host_event_active_window.replace((instance_id, active.clone())).filter(|(seen, window)| *seen == instance_id && *window != active).map(|(_, window)| window);
        if let (Some(blurred), Some(controller_id)) = (blurred, docs.shell_command_controller_id(plugins)) {
            settle.deferred_actions.push(window_host_event_action(&controller_id, &blurred, semio_framework::HOST_EVENT_KIND_BLUR));
        }
    }
    //#endregion ShellCommandHistory
}

//#endregion ActionPanelAndUtilities

