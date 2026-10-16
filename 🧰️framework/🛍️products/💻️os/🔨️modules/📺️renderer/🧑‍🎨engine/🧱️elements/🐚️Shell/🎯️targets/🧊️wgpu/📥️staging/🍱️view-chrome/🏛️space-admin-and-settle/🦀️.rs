//! 🍱️ `shell-view-chrome` staging of `space-admin-and-settle`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes

/// ⏱️ Spends one of a retained document's paint opportunities — only when the step was the document's OWN work
/// (`UiDocumentFrameCursor::last_step_was_own_work`: waiting on the layout worker pool or on another surface's layout costs
/// nothing) — and answers whether the document may keep stepping under `budget`.
pub fn document_opportunity_remains(cursor: &mut ShellChromeChildCursor, budget: usize) -> bool {
    if cursor.document.last_step_was_own_work() {
        cursor.scalar = cursor.scalar.saturating_add(1);
    }
    !cursor.document.terminal_is_fault() && cursor.scalar < budget
}

//#endregion ShellTypes

