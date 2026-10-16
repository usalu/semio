//! 📂️ `shell-doc-open` staging of `storage-prefs-and-i18n`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🛰️AgentBridgeOfferScope
/// 📤️ Publishes the shell's scope for the page realm's offer watcher.
pub fn publish_agent_bridge_offer_scope(json: String) {
    PUBLISHED_AGENT_BRIDGE_OFFER_SCOPE.with(|slot| *slot.borrow_mut() = json);
}

//#endregion 🛰️AgentBridgeOfferScope
//#endregion ShellChrome

//#region ShellChrome
//#region 🛰️AgentBridgeOfferScope
thread_local! {
    pub static PUBLISHED_AGENT_BRIDGE_OFFER_SCOPE: std::cell::RefCell<String> = std::cell::RefCell::new("null".to_string());
}
//#endregion 🛰️AgentBridgeOfferScope
//#endregion ShellChrome
