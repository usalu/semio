//! 🔀️ `shell-actions` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

#[cfg(not(target_arch = "wasm32"))]
pub async fn pick_folder() -> Option<String> {
    ui_host::select_native_paths(ui_host::NativeFileDialogRequest::folder()).await.into_iter().next().map(|path| path.display().to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn pick_file_path() -> Option<String> {
    ui_host::select_native_paths(ui_host::NativeFileDialogRequest::open(["json"], false)).await.into_iter().next().map(|path| path.display().to_string())
}

#[cfg(target_arch = "wasm32")]
pub async fn pick_file_path() -> Option<String> {
    pick_native_path_from_host(serde_json::json!({ "op": "request-native-file-path", "accept": ".json,application/json" })).await
}

#[cfg(target_arch = "wasm32")]
pub async fn pick_folder() -> Option<String> {
    pick_native_path_from_host(serde_json::json!({ "op": "request-native-folder-path" })).await
}

//#region ⚖️MergePolicyAndThemeCommits

impl<'ctx> ActionsCtx<'ctx> {
    /// ⬆️ The parked theme-import picker, opened inside the gesture that asked for it — the same
    /// user-activation rule `drain_gesture_bound_work` already honours for `pending_file_opens`.
    #[cfg(target_arch = "wasm32")]
    pub async fn run_theme_import_request(&mut self) {
        let opened = request_file_open(SHELL_THEME_IMPORT_ACCEPT, Some("text"), false).await;
        let Some(file) = opened.into_iter().next() else { return };
        let action = ActionDescriptor { controller_id: "framework".into(), action: "applyImportedTheme".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": file.contents }) };
        if let Err(error) = self.dispatch_action(action).await {
            debug_log(&format!("[TRACE] wgpu-shell theme import failed: {error}"));
        }
    }
}

//#endregion ⚖️MergePolicyAndThemeCommits

