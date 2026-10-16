//! 🫧️ `shell-surfaces` staging of `session-and-topology`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl SettleState {
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(any(test, feature = "testkit"))]
    pub fn submit_shell_io(&mut self, operation: impl FnOnce() -> ShellIoCompletion + Send + 'static) {
        const MAX_PENDING_SHELL_IO: usize = 64;
        if self.shell_io_pending.len() >= MAX_PENDING_SHELL_IO {
            return;
        }
        let (sender, receiver) = std::sync::mpsc::channel();
        semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool().submit(
            Lane::Io,
            Box::new(move || {
                let _ = sender.send(operation());
            }),
        );
        self.shell_io_pending.push_back(PendingShellIo { receiver, task: None });
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn submit_shell_io_future(&mut self, operation: impl std::future::Future<Output = ShellIoCompletion> + Send + 'static) {
        const MAX_PENDING_SHELL_IO: usize = 64;
        if self.shell_io_pending.len() >= MAX_PENDING_SHELL_IO {
            return;
        }
        let (sender, receiver) = std::sync::mpsc::channel();
        let task = ShellPoolFuture::spawn(semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool(), Lane::Io, async move {
            let _ = sender.send(operation.await);
        });
        self.shell_io_pending.push_back(PendingShellIo { receiver, task: Some(task) });
    }

    /// 🐢️ Declares dirt without taking a refresh pass — the ONE way a producer (a dispatched action's
    /// `InvocationResult::ui_scope`, a command's, an extension answer's, or a host-effect fold) says
    /// what it invalidated. The next [`Self::refresh_ui`] drains the union.
    pub fn owe_refresh(&mut self, scope: UiDirtyScope) {
        self.owed_refresh_scope = core::mem::replace(&mut self.owed_refresh_scope, UiDirtyScope::None).merged_with(scope);
    }
}

//#endregion ShellLifecycle

