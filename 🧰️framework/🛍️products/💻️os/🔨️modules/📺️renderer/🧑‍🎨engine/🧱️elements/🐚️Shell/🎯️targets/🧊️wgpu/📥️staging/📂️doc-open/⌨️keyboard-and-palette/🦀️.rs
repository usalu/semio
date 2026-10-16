//! 📂️ `shell-doc-open` staging of `keyboard-and-palette`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl OpeningState {
    /// 📤️ One file-open round trip: opens a REAL browser picker, then hands the picked files to an [`ImportTransfer`] whose
    /// chunks the drain dispatches in order through the normal action path — so each chunk's own effects, mutations and
    /// history patch are folded in exactly as a user-pressed action's are, and a person can cancel it between chunks.
    /// A cancelled picker dispatches nothing and is not an error.
    #[cfg(target_arch = "wasm32")]
    pub(super) async fn run_file_open_request(&mut self, request: PendingFileOpen, docs: &DocumentState) {
        let PendingFileOpen { controller_id, accept, read_as, import_action, args, multiple } = request;
        let opened = request_file_open(&accept, read_as.as_deref(), multiple).await;
        debug_log(&format!("[TRACE] wgpu-shell file open accept={accept} action={import_action} files={} bytes={}", opened.len(), opened.iter().map(|file| file.contents.len()).sum::<usize>()));
        self.begin_import_transfer(PickedImport { controller_id, import_action, opened, args, multiple }, docs);
    }

    pub(crate) fn push_uri(&mut self, uri: String) {
        self.uri_history.truncate(self.uri_index + 1);
        self.uri_history.push(uri);
        self.uri_index = self.uri_history.len().saturating_sub(1);
    }
}

//#endregion ShellInput

