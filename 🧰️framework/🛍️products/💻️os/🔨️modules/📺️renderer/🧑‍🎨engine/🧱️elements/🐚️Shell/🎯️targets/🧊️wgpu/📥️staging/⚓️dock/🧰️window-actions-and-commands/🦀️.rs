//! ⚓️ `shell-dock` staging of `window-actions-and-commands`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region 🎬️WindowActionsAndSearchPanes
impl ShellDockState {
    /// 🪟️ The window KIND one live instance renders, which is what `resolve_window_actions` scopes an
    /// Actions pane by — `puzzle3d-main-top` and `puzzle3d-main-perspective` are both `puzzle3d-main`.
    pub fn window_kind_of<'a>(&self, session: &'a ActiveSession, window_id: &str) -> Option<&'a semio_framework::WindowKindDefinition> {
        let kind_id = self.live_window_kind_id(session, window_id).unwrap_or(window_id).to_string();
        session.app.window_kinds.iter().find(|kind| kind.id == kind_id)
    }

    /// 🧰️ Whether this window carries React's `actionPane` PROP — `windowActionPaneNode` answers
    /// `undefined` for a kind whose `resolveWindowActions(...).filter(inPalette)` is empty, "so the
    /// rail chip never renders" (`🛠️ShellHelpers/🟦️.tsx:4260`).
    pub fn window_has_action_pane(&self, window_id: &str, docs: &DocumentState) -> bool {
        let Some(session) = docs.session.as_ref() else { return false };
        let Some(kind) = self.window_kind_of(session, window_id) else { return false };
        semio_framework::resolve_window_actions(&session.app, kind).into_iter().any(|action| action.in_palette)
    }
}

//#endregion 🎬️WindowActionsAndSearchPanes
impl ShellDockState {
    // #region utility-derivation
    /// 🧰️ The window kind whose utilities/actions the shell chrome currently scopes to (the focused window,
    /// else the view-state's active kind, else the app's first kind).
    pub fn active_utility_bar_window_kind<'a>(&self, session: &'a ActiveSession, windows: &WindowState) -> &'a semio_framework::WindowKindDefinition {
        let active_id = windows.active_window_id.as_deref().or(session.view_state.window_id.as_deref());
        active_id.and_then(|id| self.live_window_kind_id(session, id)).and_then(|kind_id| session.app.window_kinds.iter().find(|kind| kind.id == kind_id)).unwrap_or_else(|| session.app.window_kinds.first())
    }

    /// 🧰️ Derives the ACTIVE window's utility `UtilityNode`s — the roster the shell's own hit router and
    /// keybinding lane address by collection path. The chrome that PAINTS them is each pane's own
    /// `Utilities` group, so this answers `active_utility_bar_window_kind`'s window, never the footer.
    pub fn derive_utility_nodes(&self, session: &ActiveSession, prefs: &PrefsState, windows: &WindowState) -> Vec<UtilityNode> {
        self.derive_window_utility_nodes(session, self.active_utility_bar_window_kind(session, windows).id.as_str(), prefs, windows)
    }

    /// 🧰️ Derives ONE window instance's utility bar from the app's declared utilities scoped to that
    /// window's kind, marking the host-owned active utility pressed (Architecture Decision 5) — React's
    /// `resolveUtilityNodes(session.app, kind, activeUtilityByWindowId[instance.id], instance.id, …)`
    /// (`🏛️ShellHost/🟦️.tsx:10249`), which is derived PER PANE, never once for the shell.
    pub fn derive_window_utility_nodes(&self, session: &ActiveSession, window_id: &str, prefs: &PrefsState, windows: &WindowState) -> Vec<UtilityNode> {
        if session.app.utilities.is_empty() {
            return Vec::new();
        }
        let kind_id = self.live_window_kind_id(session, window_id).unwrap_or(window_id).to_string();
        let window_kind = session.app.window_kinds.iter().find(|kind| kind.id == kind_id).unwrap_or_else(|| session.app.window_kinds.first());
        let resolved = resolve_window_utilities(&session.app, window_kind);
        if resolved.is_empty() {
            return Vec::new();
        }
        let specs: Vec<semio_framework_ui_declaration::utilities::DerivedUtilitySpec> = resolved
            .iter()
            .map(|utility| semio_framework_ui_declaration::utilities::DerivedUtilitySpec {
                id: utility.id.clone(),
                label: utility.label.resolve(prefs.active_terminology(), prefs.active_locale()).to_string(),
                icon_id: utility.icon_id.clone(),
                group: utility.group.clone(),
                category: utility.category,
            })
            .collect();
        let active = windows.active_utility_by_window.get(window_id).map(String::as_str);
        semio_framework_ui_declaration::utilities::derive_utility_nodes(&session.app.controller_id, &specs, active)
    }
}

//#endregion ActionPanelAndUtilities

