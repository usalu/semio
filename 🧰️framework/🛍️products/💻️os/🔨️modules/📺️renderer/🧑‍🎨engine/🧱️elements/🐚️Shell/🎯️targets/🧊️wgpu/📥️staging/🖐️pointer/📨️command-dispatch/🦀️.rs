//! 🖐 `shell-pointer` staging of `command-dispatch`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellActions
impl<'ctx> PointerCtx<'ctx> {
    pub async fn apply_pending_shell_uri(&mut self) -> Result<(), String> {
        let uri = self.opening.shell_uri(&self.docs);
        self.as_actions().apply_shell_uri(&uri).await
    }
}
//#endregion ShellActions
