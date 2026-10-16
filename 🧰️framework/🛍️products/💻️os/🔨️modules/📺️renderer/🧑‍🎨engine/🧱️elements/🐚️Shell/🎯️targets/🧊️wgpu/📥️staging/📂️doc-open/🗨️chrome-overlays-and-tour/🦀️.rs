//! 📂️ `shell-doc-open` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays
impl OpeningState {
    /// 🛰️ One bounded offer-scope turn on the directory pump: follows sign-in and the space the shell is in, re-reads this
    /// human's own live delegations when due (one `GET /auth/agent-delegations`, [`AgentBridgeOfferScopeOwnerV1`]) and
    /// publishes the scope for the page realm.
    pub(crate) async fn pump_agent_bridge_offer_scope(&mut self, directory: &mut DirectoryState) {
        let target = directory.identity.as_ref().zip(self.open_space_id.as_ref()).filter(|(_, space_id)| !space_id.is_empty()).map(|(identity, space_id)| (identity.hub_base_url.trim_end_matches('/').to_string(), space_id.clone()));
        if let Some(due) = directory.agent_bridge_offer_scope.observe(target, chrome_now_ms()) {
            let principals = match directory.directory_client.clone() {
                Some(client) => client.live_agent_delegation_principals(&directory.directory_command_ctx(), &due.1, i64::try_from(directory_now_ms()).unwrap_or(i64::MAX)).await.ok(),
                None => None,
            };
            directory.agent_bridge_offer_scope.settle(&due, principals, chrome_now_ms());
        }
        publish_agent_bridge_offer_scope(directory.agent_bridge_offer_scope.page_json());
    }
}

//#endregion 🧯️TransientNoticeAndAgentOverlays
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

