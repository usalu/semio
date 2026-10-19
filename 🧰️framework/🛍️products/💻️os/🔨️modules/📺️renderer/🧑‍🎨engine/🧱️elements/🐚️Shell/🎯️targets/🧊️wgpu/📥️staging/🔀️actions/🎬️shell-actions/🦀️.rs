//! 🔀️ `shell-actions` staging of `shell-actions`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

#[path = "🔄️backbone-and-check-in/🦀️.rs"]
mod backbone_sync_and_check_in;
#[cfg(not(target_arch="wasm32"))]
pub use backbone_sync_and_check_in::{shell_turn, ShellTurn};
#[cfg(target_arch="wasm32")]
pub use backbone_sync_and_check_in::{shell_turn, ShellTurn};

#[path = "📎️sync-attach-and-open/🦀️.rs"]
mod sync_attach_and_document_open;

#[path = "🎯️dispatch-action/🦀️.rs"]
mod dispatch_action_child;
