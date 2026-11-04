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

//#region 🎞️MediaActionInbox
/// 📏️ Most media actions the page may leave for one frame turn; a full inbox refuses instead of growing.
pub const MEDIA_ACTION_INBOX_CAPACITY: usize = 16;

thread_local! {
    /// 🎞️ Media-window actions the page host handed over (the trim "Cut selection" button), waiting for the shell's next turn.
    pub static MEDIA_ACTION_INBOX: std::cell::RefCell<std::collections::VecDeque<ActionDescriptor>> = const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

/// 🎞️ Parses `{windowId, controllerId, action, args}` into the one `ActionDescriptor` the UI intents use; `windowId` rides in `args` exactly as the window's own controls send it, so the normal dispatch scopes the action to that window.
pub fn parse_media_action(json: &str) -> Result<ActionDescriptor, &'static str> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(|_| "media-action.invalid-json")?;
    let record = value.as_object().ok_or("media-action.invalid-shape")?;
    let text = |key: &str| record.get(key).and_then(serde_json::Value::as_str).filter(|text| !text.is_empty()).ok_or("media-action.missing-field");
    let (window_id, controller_id, action) = (text("windowId")?, text("controllerId")?, text("action")?);
    let mut args = match record.get("args") {
        None | Some(serde_json::Value::Null) => serde_json::Map::new(),
        Some(serde_json::Value::Object(args)) => args.clone(),
        Some(_) => return Err("media-action.invalid-args"),
    };
    args.insert("windowId".into(), serde_json::Value::String(window_id.to_string()));
    Ok(ActionDescriptor { controller_id: controller_id.to_string(), action: action.to_string(), args: Some(semio_framework_value::DslValue::from(serde_json::Value::Object(args))) })
}

/// 🎞️ Leaves one media action for the shell's next turn; refuses a malformed one or a full inbox.
pub fn push_media_action(json: &str) -> Result<(), &'static str> {
    let action = parse_media_action(json)?;
    MEDIA_ACTION_INBOX.with(|inbox| {
        let mut inbox = inbox.borrow_mut();
        if inbox.len() >= MEDIA_ACTION_INBOX_CAPACITY {
            return Err("media-action.inbox-full");
        }
        inbox.push_back(action);
        Ok(())
    })
}

/// 🎞️ Takes every waiting media action, oldest first, exactly once.
pub fn take_media_actions() -> Vec<ActionDescriptor> {
    MEDIA_ACTION_INBOX.with(|inbox| inbox.borrow_mut().drain(..).collect())
}

impl ShellDockState {
    /// 🎞️ The media actions the page left for this turn, oldest first.
    pub fn drain_media_actions(&self) -> Vec<ActionDescriptor> {
        take_media_actions()
    }
}
//#endregion 🎞️MediaActionInbox
