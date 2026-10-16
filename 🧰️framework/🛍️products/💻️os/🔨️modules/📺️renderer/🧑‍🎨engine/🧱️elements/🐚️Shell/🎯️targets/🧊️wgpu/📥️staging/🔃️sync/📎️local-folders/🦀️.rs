//! 🔃️ `shell-sync` staging of `local-folders`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::local_folders::*;

impl SyncState {
    /// 🗂️ The document a folder or file binding holds now: the document actor's on the native build, the host-route
    /// mirror's on the browser build.
    pub(super) fn attached_document_id(&self, history: &HistoryState) -> Option<&str> {
        #[cfg(any(target_arch = "wasm32", test))]
        if let Some(folder) = history.host_route_folder.as_ref() {
            return Some(folder.document_id.as_str());
        }
        self.sync_channel.as_ref().map(|channel| channel.document_id.as_str())
    }
}

