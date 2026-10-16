//! 🐙️ `shell-state` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 🔖️IdentityPure
/// 🎭️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0 — mints this process's
/// stable per-shell session id (the `{sessionId}` half of `user:{userId}#{sessionId}`), once, at
/// `ShellState::new`. Process id + boot-time millis is deliberately simple (CLAUDE.md: concise code,
/// no cryptographic uniqueness need) — collisions would require two `semio-wgpu-native` processes
/// booting the same pid at the same millisecond, which the OS itself already rules out.
pub fn mint_shell_session_id() -> String {
    format!("wgpu-{}", semio_framework_os_kernel::os_identity::time_ordered_id())
}

/// 🔐️ The hub origin the hub workspace's local-bootstrap entry names — slice WG6's twin of AU3
/// §4.4's `hubBootstrapOriginV1`. It is the HUB's own origin, never the page's: a `POST` to the UI
/// origin reaches the SPA's index route and answers HTML, which is why a sign-in could not have
/// worked from React's shell until AU3 separated the two. A declared origin that is not a bare
/// scheme+authority is refused rather than trimmed, and an undeclared hub falls back to the request
/// base url the directory client already uses.
pub fn shell_hub_bootstrap_origin() -> String {
    let declared = shell_directory_request_base_url();
    semio_framework_os_renderer_wgpu_hub_contracts::hub_sign_in::parse_hub_origin(&declared).unwrap_or(declared)
}

/// 🌐️ The native directory client's network byte budget per minute. It carries the largest bounded answer the
/// client takes — a hub document's execution-target component (`DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES`,
/// 64 MiB) — twice over: at the old 10 MB a 17.7 MB block component was refused mid-body
/// (`os.directory-client exhausted its network_bytes_per_min budget`, gate run 23 on hub 7800 B2).
#[cfg(not(target_arch = "wasm32"))]
pub const SHELL_DIRECTORY_NETWORK_BYTES_PER_MINUTE: u64 = 2 * semio_framework_os_kernel::os_directory::DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES;

//#endregion 🔖️IdentityPure
//#region ShellTypes
/// 🪪️ React's `InputOriginV1` values this target can mint — a CONTROL press, and a pointer/camera
/// stream an engine surface derived from raw input (`🏛️ShellHost/🎯️input-ledger/🟦️.ts`).
pub const SHELL_DISPATCH_ORIGIN_USER: &str = "user";

//#endregion ShellTypes

