//! 🚶️ `shell-session` staging of `local-folders`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

impl<'ctx> SessionCtx<'ctx> {
    /// 🔁️ Keeps a host-route document's archive current: once the history moved past the cursor last written, the
    /// archive is written again unless it is byte-identical — never while a history edit is open, whose preview is not
    /// the document. A refused write is told once and retried at the next change. `true` when it wrote or was refused.
    #[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
    pub async fn flush_host_route_folder(&mut self) -> bool {
        if self.history.history_time_travel.is_some() || !self.history.host_route_folder.as_ref().is_some_and(|folder| folder.written_cursor != self.history.history_cursor) {
            return false;
        }
        let Some(folder) = self.history.host_route_folder.clone() else { return false };
        let cursor = self.history.history_cursor;
        let written = match self.docs.write_host_route_archive(&folder.uri, &folder.document_id, &folder.schema, &self.plugins).await {
            Ok(digest) => digest,
            Err(error) => {
                self.prefs.note_dispatch_fault(&error, &mut self.chrome);
                folder.written
            }
        };
        if let Some(bound) = self.history.host_route_folder.as_mut().filter(|bound| bound.uri == folder.uri) {
            bound.written = written;
            bound.written_cursor = cursor;
        }
        true
    }

    /// ✂️ Detaches a host-route document: its last change is written, the binding dropped and a folder forgotten.
    /// `false` when no document was bound through the host route.
    #[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
    pub async fn detach_host_route_folder(&mut self) -> bool {
        self.flush_host_route_folder().await;
        let Some(folder) = self.history.host_route_folder.take() else { return false };
        self.sync.sync_backbone_uri = None;
        self.sync.sync_card_kind = None;
        if folder.uri.starts_with("folder://") {
            self.history.ensure_local_folder_bindings();
            self.history.forget_local_folder(&folder.document_id);
        }
        true
    }
}

