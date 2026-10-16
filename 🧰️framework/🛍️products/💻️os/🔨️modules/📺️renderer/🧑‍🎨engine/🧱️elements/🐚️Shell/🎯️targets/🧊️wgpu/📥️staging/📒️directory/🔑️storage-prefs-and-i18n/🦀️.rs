//! 📒️ `shell-directory` staging of `storage-prefs-and-i18n`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🛰️AgentBridgeOfferScope
/// 🎯️ Who this shell is for the agent-bridge supervisor's scoped offer (ticket 26/09/23, G12 × WG11 session 14c): the hub it
/// signed in to, the space it is in and the agents ITS human delegated there — ShellHost's `offerScope`. Only these three
/// values cross into the page realm (`dumpAgentBridgeOfferScope`); the session capability that lists them never does.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentBridgeOfferScopeV1 {
    pub hub_origin: String,
    pub space_id: String,
    pub agent_principal_ids: Vec<String>,
}

/// 🔁️ Keeps [`AgentBridgeOfferScopeV1`] true to the shell: a sign-out or a space switch retires the scope AT ONCE (one space's
/// agents are never offered to another), a listing answered for a target the shell has since left is dropped, and the
/// listing is re-read every [`AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS`] while the target stands.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AgentBridgeOfferScopeOwnerV1 {
    pub target: Option<(String, String)>,
    pub scope: Option<AgentBridgeOfferScopeV1>,
    next_listing_at_ms: f64,
    listing: bool,
}

impl AgentBridgeOfferScopeOwnerV1 {
    /// 🎯️ Follows the shell's current `(hubOrigin, spaceId)`; answers the target whose listing is due now, if any.
    pub fn observe(&mut self, target: Option<(String, String)>, now_ms: f64) -> Option<(String, String)> {
        if self.target != target {
            self.target = target;
            self.scope = None;
            self.listing = false;
            self.next_listing_at_ms = now_ms;
        }
        if self.listing || now_ms < self.next_listing_at_ms {
            return None;
        }
        let due = self.target.clone()?;
        self.listing = true;
        Some(due)
    }

    /// 📥️ Settles the listing asked for `target`: its live principals become the scope, a failed listing leaves no scope
    /// (local offers only, as ShellHost's rejected `offerScope`), and an answer for a target the shell has left is dropped.
    pub fn settle(&mut self, target: &(String, String), principals: Option<Vec<String>>, now_ms: f64) {
        if self.target.as_ref() != Some(target) {
            return;
        }
        self.listing = false;
        self.next_listing_at_ms = now_ms + AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS;
        self.scope = principals.map(|agent_principal_ids| AgentBridgeOfferScopeV1 { hub_origin: target.0.clone(), space_id: target.1.clone(), agent_principal_ids });
    }

    /// 🎯️ The standing scope, if the shell is signed in and in a space whose listing answered.
    pub fn scope(&self) -> Option<&AgentBridgeOfferScopeV1> {
        self.scope.as_ref()
    }

    /// 📜️ The scope as the page realm reads it: the three values, or `null`.
    pub fn page_json(&self) -> String {
        self.scope.as_ref().and_then(|scope| serde_json::to_string(scope).ok()).unwrap_or_else(|| "null".to_string())
    }
}

//#endregion 🛰️AgentBridgeOfferScope
//#endregion ShellChrome

//#region ShellChrome
//#region 🛰️AgentBridgeOfferScope
/// ⏱️ How often a standing scope re-reads its delegations — the offer watcher's own maximum discovery interval
/// (`BRIDGE_DISCOVERY_MAX_INTERVAL_MS`), so a revoked or expired delegation leaves the scope within one poll of it.
pub const AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS: f64 = 30_000.0;
//#endregion 🛰️AgentBridgeOfferScope
//#endregion ShellChrome
