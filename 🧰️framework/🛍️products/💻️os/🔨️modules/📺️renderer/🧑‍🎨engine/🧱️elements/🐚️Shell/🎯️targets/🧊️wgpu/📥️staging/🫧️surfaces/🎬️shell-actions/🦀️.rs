//! 🫧️ `shell-surfaces` staging of `shell-actions`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl SurfaceState {
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn schedule_sync_path_pick(&mut self, pick: impl std::future::Future<Output = Option<String>> + Send + 'static, settle: &mut SettleState) {
        settle.submit_shell_io_future(async move {
            let Some(path) = pick.await else { return ShellIoCompletion::Finished };
            ShellIoCompletion::Actions(vec![ActionDescriptor { controller_id: "framework.sync".into(), action: "setSyncDraft".into(), args: Some(semio_framework::dsl_value!({ "path": path })) }])
        });
    }
}

//#endregion ShellActions

