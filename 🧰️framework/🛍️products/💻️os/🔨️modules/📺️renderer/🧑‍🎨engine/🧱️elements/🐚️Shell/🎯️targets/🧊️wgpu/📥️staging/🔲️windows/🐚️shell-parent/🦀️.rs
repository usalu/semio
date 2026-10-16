//! 🔲️ `shell-windows` staging of `shell-parent`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowIconOverride {
    pub plugin_id: String,
    pub app_id: String,
    pub app_instance_id: u32,
    pub icon_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowTitleOverride {
    pub plugin_id: String,
    pub app_id: String,
    pub app_instance_id: u32,
    pub title: String,
}

//#endregion ShellTypes

