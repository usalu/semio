//! 🔃️ `shell-sync` staging of `icon-export`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Phase { Next, Load, Prepare, Device, Render, Vector, Retire, Save, Complete, Cancelled, Failed }

