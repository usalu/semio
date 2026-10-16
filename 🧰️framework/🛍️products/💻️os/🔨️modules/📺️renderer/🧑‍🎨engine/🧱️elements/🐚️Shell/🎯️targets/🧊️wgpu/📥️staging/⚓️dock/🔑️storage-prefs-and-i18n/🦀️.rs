//! ⚓️ `shell-dock` staging of `storage-prefs-and-i18n`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🌉️AgentBridgeConfigDoor
/// 🌉️ Takes whatever the hook left, exactly once.
#[cfg(target_arch = "wasm32")]
pub fn take_pending_agent_bridge_config() -> Option<Option<semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentBridgeConfig>> {
    PENDING_AGENT_BRIDGE_CONFIG.with(|slot| slot.borrow_mut().take())
}

/// 🖥️ Native has no equivalent hook and deliberately no environment discovery either: a native
/// embedder holds the shell directly and calls `ShellState::set_agent_bridge_config`.
#[cfg(not(target_arch = "wasm32"))]
pub fn take_pending_agent_bridge_config() -> Option<Option<semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentBridgeConfig>> {
    None
}

//#endregion 🌉️AgentBridgeConfigDoor
//#endregion ShellChrome

//#region ShellChrome
//#region 🌉️AgentBridgeConfigDoor
/// 🔗️ The one config slot a supervisor may fill before (or after) the shell exists — packet W15e.
///
/// ⚖️ Why a slot and not a boot-descriptor axis: bridge admission is a PROTECTED in-memory value the
/// local supervisor hands over, never a url/`<meta>`/env carrier — React makes exactly that call in
/// `discoverAgentBridgeConfig`'s own docstring and returns `null` unconditionally rather than reading
/// `VITE_SEMIO_BRIDGE_*`. Putting it on the boot descriptor would put it in the address bar and in
/// every referrer; putting it here keeps it where React keeps it.
#[cfg(target_arch = "wasm32")]
thread_local! {
    pub static PENDING_AGENT_BRIDGE_CONFIG: std::cell::RefCell<Option<Option<semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentBridgeConfig>>> = const { std::cell::RefCell::new(None) };
}
//#endregion 🌉️AgentBridgeConfigDoor
//#endregion ShellChrome
