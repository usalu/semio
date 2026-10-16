//! 🖐 `shell-pointer` staging of `session-and-topology`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellLifecycle
impl<'ctx> PointerCtx<'ctx> {
    pub async fn hot_reload_plugins(&mut self, plugins: Vec<ProgramBridgeEntry>) -> Result<(), String> {
        self.docs.prepare_hot_reload(plugins, &mut self.plugins, &mut self.directory);
        self.as_actions().boot().await
    }
}
//#endregion ShellLifecycle
