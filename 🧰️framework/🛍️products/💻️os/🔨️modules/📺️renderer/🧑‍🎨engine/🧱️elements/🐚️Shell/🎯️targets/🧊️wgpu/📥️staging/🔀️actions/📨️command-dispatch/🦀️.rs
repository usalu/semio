//! 🔀️ `shell-actions` staging of `command-dispatch`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

#[path = "📨️dispatch-app-switch/🦀️.rs"]
mod dispatch_and_app_switch;

#[path = "📇️directory-and-hub/🦀️.rs"]
mod directory_and_hub;
