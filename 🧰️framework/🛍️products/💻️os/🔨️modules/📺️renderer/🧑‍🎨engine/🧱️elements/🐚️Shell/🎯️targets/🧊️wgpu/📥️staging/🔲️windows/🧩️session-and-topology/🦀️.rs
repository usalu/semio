//! 🔲️ `shell-windows` staging of `session-and-topology`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl WindowState {
    pub fn window_icon_override(&self, window_id: &str, docs: &DocumentState) -> Option<&str> {
        let session = docs.session.as_ref()?;
        self.window_icon_overrides.get(window_id).filter(|icon| icon.plugin_id == session.plugin_id && icon.app_id == session.app.id && icon.app_instance_id == session.instance_id).map(|icon| icon.icon_id.as_str())
    }

    pub fn window_title_override(&self, window_id: &str, docs: &DocumentState) -> Option<&str> {
        let session = docs.session.as_ref()?;
        self.window_title_overrides.get(window_id).filter(|title| title.plugin_id == session.plugin_id && title.app_id == session.app.id && title.app_instance_id == session.instance_id).map(|title| title.title.as_str())
    }

    /// 🪟️ A changed window-instance roster invalidates every guest body because the live view carries
    /// the complete roster and focused instance. The settle lane performs the refresh on a later
    /// frame, keeping pointer and command turns bounded while guaranteeing a newly mounted instance
    /// receives its first document and removed instances enter the normal retirement lane.
    pub fn owe_window_topology_refresh(&mut self, settle: &mut SettleState) {
        self.window_topology_refresh_owed = true;
        settle.owe_refresh(UiDirtyScope::Full);
        settle.owe_settle();
    }

    pub fn window_topology_publication_has_credit(&self) -> bool {
        self.window_topology_publications.len() < semio_framework_ui_action::ACTION_QUEUE_ITEM_CAPACITY
    }

    /// 🪟️ Commits one already-admitted roster mutation and arms its required Full refresh.
    /// Physical transfers bring an action token reserved before the dock changes; direct Display
    /// opens intentionally carry no React journal.
    pub fn commit_window_topology_publication(&mut self, window_id: &str, body_key: &str, journal_token: Option<u64>, settle: &mut SettleState) {
        debug_assert!(self.window_topology_publication_has_credit());
        self.window_topology_publications.push_back(WindowTopologyPublication { window_id: window_id.to_string(), body_key: body_key.to_string(), attempts: 0, journal_token });
        self.owe_window_topology_refresh(settle);
    }

    /// 🧾️ Reserves and builds one physical transfer note before the dock tree can change.
    /// The returned token binds release to this exact owner even when later bodies publish first.
    pub fn reserve_window_topology_action(&mut self, action: ActionDescriptor) -> Result<u64, semio_framework_ui_action::BoundedActionFault> {
        match self.window_topology_actions.reserve(&action.controller_id, &action.action, semio_framework_ui_action::ACTION_ITEM_BYTE_CAPACITY) {
            Ok(mut reservation) => action.args.as_ref().map_or(Ok(()), |args| reservation.builder().value(None, args)).and_then(|()| reservation.publish()),
            Err(error) => Err(error),
        }?;
        let token = self.next_window_topology_action_token;
        self.next_window_topology_action_token = self.next_window_topology_action_token.wrapping_add(1).max(1);
        self.window_topology_action_tokens.push_back(token);
        Ok(token)
    }

    pub fn take_window_topology_action(&mut self, token: u64) -> Option<semio_framework_ui_action::BoundedAction> {
        let index = self.window_topology_action_tokens.iter().position(|candidate| *candidate == token)?;
        debug_assert_eq!(self.window_topology_action_tokens.remove(index), Some(token));
        self.window_topology_actions.remove_at(index)
    }

    pub fn discard_window_topology_journal(&mut self, publication: &WindowTopologyPublication, chrome: &mut ChromeState, plugins: &mut PluginState, prefs: &mut PrefsState) {
        if publication.journal_token.is_some_and(|token| self.take_window_topology_action(token).is_none()) {
            prefs.record_surface_fault(&publication.window_id, &publication.body_key, "window topology journal owner was absent".into(), chrome, plugins);
        }
    }
}

//#endregion ShellLifecycle

