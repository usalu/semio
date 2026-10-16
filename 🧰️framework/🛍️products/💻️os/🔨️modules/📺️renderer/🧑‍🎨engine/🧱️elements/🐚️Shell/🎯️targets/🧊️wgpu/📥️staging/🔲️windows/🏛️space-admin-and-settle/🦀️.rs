//! 🔲️ `shell-windows` staging of `space-admin-and-settle`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
//#region 🧯️BootProgramSelection
/// 🪟️ One bounded roster mutation whose first retained body is the release condition for its
/// optional React-equivalent transfer journal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowTopologyPublication {
    pub window_id: String,
    pub body_key: String,
    pub attempts: u8,
    pub journal_token: Option<u64>,
}

//#endregion 🧯️BootProgramSelection
//#endregion ShellTypes

