//! 🐙️ `shell-state` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ⚖️MergePolicyAndThemeCommits
/// ⚖️ `protocol::MergePolicy::default()` — "the least-surprising choice for a fresh authority that
/// has never been configured", and React's `DEFAULT_MERGE_POLICY`.
pub(crate) const SHELL_DEFAULT_MERGE_POLICY: &str = "Normal";

//#endregion ⚖️MergePolicyAndThemeCommits

