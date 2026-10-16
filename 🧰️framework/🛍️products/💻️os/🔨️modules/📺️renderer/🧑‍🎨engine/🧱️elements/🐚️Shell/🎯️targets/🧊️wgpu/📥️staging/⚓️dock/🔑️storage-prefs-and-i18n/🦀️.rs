//! ⚓️ `shell-dock` staging of `storage-prefs-and-i18n`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🌉️AgentBridgeConfigDoor
/// 🌉️ Takes whatever the hook left, exactly once.
#[cfg(target_arch = "wasm32")]
pub(crate) fn take_pending_agent_bridge_config() -> Option<Option<crate::agent_bridge::AgentBridgeConfig>> {
    PENDING_AGENT_BRIDGE_CONFIG.with(|slot| slot.borrow_mut().take())
}

/// 🖥️ Native has no equivalent hook and deliberately no environment discovery either: a native
/// embedder holds the shell directly and calls `ShellState::set_agent_bridge_config`.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn take_pending_agent_bridge_config() -> Option<Option<crate::agent_bridge::AgentBridgeConfig>> {
    None
}

//#endregion 🌉️AgentBridgeConfigDoor
//#endregion ShellChrome

