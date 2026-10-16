//! 🗨️ `shell-view-overlays` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
#[cfg(any(test, feature = "testkit"))]
pub fn context_menu_paths_equal(a: &[usize], b: &[usize]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(left, right)| left == right)
}

#[cfg(any(test, feature = "testkit"))]
pub fn context_menu_submenu_open(active: &[usize], row_path: &[usize], is_active: bool, has_children: bool) -> bool {
    if !has_children {
        return false;
    }
    if is_active && active.len() == row_path.len() {
        return true;
    }
    active.len() > row_path.len() && row_path.iter().enumerate().all(|(index, value)| active.get(index) == Some(value))
}

//#endregion ShellTypes

