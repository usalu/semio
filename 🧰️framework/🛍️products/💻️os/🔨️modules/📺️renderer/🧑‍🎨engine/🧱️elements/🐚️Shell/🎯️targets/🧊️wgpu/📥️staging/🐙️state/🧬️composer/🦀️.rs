//! 🐙️ `shell-state` composer: the `ShellState` struct of the fifteen component structs and its `Drop`, cut out of the `shell` parent (RENDERER-INTEGRATION-2).

use super::*;

//#region ShellTypes

/// 🐙️ The shell state: exactly the fifteen component structs, which own the fields and the methods of the former god object.
pub struct ShellState {
    pub chrome: ChromeState,
    pub surfaces: SurfaceState,
    pub tutorial: TutorialState,
    pub settle: SettleState,
    pub plugins: PluginState,
    pub history: HistoryState,
    pub prefs: PrefsState,
    pub directory: DirectoryState,
    pub sync: SyncState,
    pub docs: DocumentState,
    pub opening: OpeningState,
    pub windows: WindowState,
    pub dock: ShellDockState,
    pub input: ShellInputState,
    pub commands: CommandState,
}
//#endregion ShellTypes

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ShellState {
    fn drop(&mut self) {
        self.directory.directory_cancel.cancel_now();
        if let Some(task) = self.directory.identity_bootstrap_task.take() {
            task.cancel();
        }
        if let Some(mut home) = self.directory.directory_home.take() {
            if let Some((plugin_id, instance_id)) = home.take_destroy_authority() {
                if let Some(program) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == plugin_id) {
                    program.destroy_app(instance_id);
                }
            }
        }
        for pending in &self.settle.shell_io_pending {
            if let Some(task) = &pending.task {
                task.cancel();
            }
        }
    }
}
