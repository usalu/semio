//! 📒️ `shell-directory` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

/// 💬️ Byte-identical to React's own `<ol id="framework.chat.feed">` (`💬️AgentChatPanel/🟦️.tsx`) — the
/// transcript's container, addressed as one node by a probe and by the accessibility mirror.
pub const FRAMEWORK_CHAT_FEED_ID: &str = "framework.chat.feed";

/// 📃️ How many conversation rows the panel body PAINTS. React's feed scrolls and pins itself to the
/// bottom on every new entry, so the human sees the newest window; this is that window's size on a
/// target whose panel body is a fixed-credit layout tree. The bridge keeps up to
/// `AGENT_CONVERSATION_MAX_ENTRIES` (200) behind it — nothing is discarded, only unpainted.
pub const AGENT_CHAT_VISIBLE_ENTRIES: usize = 24;

//#region 🔖️IdentityPure
/// 🌱️ Native: `S_HUB_URL`/`S_USER`/`S_DATA_DIR` straight off the process env (contract §C0 — "wgpu
/// native reads `S_*` directly"). No hub env ⇒ `None` ⇒ every identity/binding/directory code path
/// below is a no-op, preserving today's local-only behaviour byte-for-byte.
#[cfg(not(target_arch = "wasm32"))]
pub fn resolve_identity_env() -> Option<IdentityEnv> {
    IdentityEnv::from_process_env()
}

/// 🌱️ Browser: the same decision from the boot environment the TypeScript host handed over
/// (`semioWgpuSetHubEnv`). A page that was never told about a hub stays local-only, exactly as a
/// native process without `S_*` does.
#[cfg(target_arch = "wasm32")]
pub fn resolve_identity_env() -> Option<IdentityEnv> {
    BOOT_HUB_ENV.with(|cell| cell.borrow().as_ref().map(|(_, _, data_dir)| IdentityEnv { data_dir: data_dir.as_ref().map(std::path::PathBuf::from) }))
}

/// 📇️ The base the shell's `DirectoryClient` prefixes every hub path with.
///
/// ⚖️ Native talks to the hub origin directly (the credential's own `hub_origin`, filled in by
/// `DirectoryClient::authenticated`, so this is only the pre-identity default). The browser goes
/// through the SAME same-origin reverse proxy prefix the React shell's worker uses
/// (`requestBaseUrl: "/_semio/hub"`, `🏪️store/👷️worker/🟦️.ts`) — same-origin is what lets the
/// session cookie travel at all, so the two renderers cannot disagree about where the hub is.
#[cfg(not(target_arch = "wasm32"))]
pub fn shell_directory_request_base_url() -> String {
    std::env::var("S_HUB_URL").ok().filter(|value| !value.trim().is_empty()).unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
pub fn shell_directory_request_base_url() -> String {
    "/_semio/hub".to_string()
}

/// 🖥️ This shell session's device-instance id, reduced to the hub's own admission grammar
/// (`^[A-Za-z0-9._:-]+$`, 128 bytes). It is derived from the session id rather than from anything
/// about the human — never a fingerprint — and a session id carrying a character the route would
/// refuse is filtered rather than rejected, so a sign-in can never fail on a local id format.
pub fn shell_hub_device_instance_id(session_id: &str) -> String {
    let filtered: String = session_id.chars().filter(|character| character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | ':' | '-')).take(semio_framework_os_renderer_wgpu_hub_contracts::hub_sign_in::HUB_SIGN_IN_DEVICE_INSTANCE_MAX_BYTES).collect();
    if filtered.is_empty() {
        "wgpu-shell".to_string()
    } else {
        filtered
    }
}

/// 🔌️ The one transport the shell's directory client is built on for this target.
#[cfg(target_arch = "wasm32")]
pub fn shell_directory_transport() -> ShellDirectoryTransport {
    ShellDirectoryTransport::default()
}

/// 🎭️ contract §C0's actor grammar: `user:{userId}#{sessionId}` once an identity is minted/restored,
/// falling back to the pre-identity local default (`wgpu-{instanceId}`) — the same default this shell
/// used everywhere before this lane, so local-only (no hub env) behaviour is unchanged.
pub fn shell_actor(identity: Option<&Identity>, session_id: &str, instance_id: u32) -> String {
    match identity {
        Some(identity) => actor_id(identity, session_id),
        None => format!("wgpu-{instance_id}"),
    }
}

//#endregion 🔖️IdentityPure

//#region 🔖️IdentityPure
#[cfg(target_arch = "wasm32")]
thread_local! {
    /// 🌐️ Browser boot environment supplied by `semioWgpuSetHubEnv` from the TypeScript host.
    pub static BOOT_HUB_ENV: std::cell::RefCell<Option<(String, String, Option<String>)>> = std::cell::RefCell::new(None);
}
//#endregion 🔖️IdentityPure
