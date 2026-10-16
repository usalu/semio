//! 📒️ `shell-directory` staging of `shell-actions`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl DirectoryState {
    //#region 🔖️NativeBackboneSync
    /// 🎭️ ticket §1 — contract §C0's `user:{userId}#{sessionId}` once identity is minted/restored,
    /// else the pre-identity local default this shell always used (`shell_actor`'s pure decision).
    pub fn current_shell_actor(&self, instance_id: u32) -> String {
        shell_actor(self.identity.as_ref(), &self.shell_session_id, instance_id)
    }
    //#endregion 🔖️NativeBackboneSync
}

//#endregion ShellActions

