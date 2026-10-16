//! 📄️ `shell-docs` staging of `space-admin-and-settle`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
/// 📏️ Bounded steps one `drain_retained_document_arenas` spends returning the previous refresh's
/// retained documents and their `UiValue` pages. Finite so a registry that never settles is left
/// behind rather than spinning the shell.
pub(crate) const SHELL_DOCUMENT_RETIREMENT_STEPS: usize = 65_536;

//#endregion ShellTypes

