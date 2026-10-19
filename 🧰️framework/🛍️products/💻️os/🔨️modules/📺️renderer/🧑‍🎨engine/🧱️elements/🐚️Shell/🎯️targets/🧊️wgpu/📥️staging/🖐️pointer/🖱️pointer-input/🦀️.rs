//! 🖐️ `shell-pointer` staging of `pointer-input`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

#[path = "🔑️key-and-move/🦀️.rs"]
mod key_and_move;
pub use key_and_move::{host_window_blur_pending, tree_row_has_chevron, ui_event_from_key_action};

#[path = "🎯️shell-hit/🦀️.rs"]
mod shell_hit;

#[path = "♿️access-and-routing/🦀️.rs"]
mod accessibility_and_routing;
pub use accessibility_and_routing::{
    drag_captures_pointer, pointer_hit_owner, pointer_press_belongs_to_shell_chrome,
};
