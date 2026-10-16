//! 🧲️ `shell-plugins` staging of `panel-records`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
/// 👁️✏️ Byte-identical to React's `DEFAULT_APP_NONE_VALUE` (`📌️ChromePanels/🟦️.tsx`) — the
/// default-apps `Select` value that clears a pin rather than naming an app.
pub(crate) const DEFAULT_APP_NONE_VALUE: &str = "none";

//#endregion 🧾️PanelRecordsAssembler
//#endregion ShellLifecycle

