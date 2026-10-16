//! 🖐 `shell-pointer` staging of `theme-file-import-and-accessibility`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region 📤️FileOpenImport
impl<'ctx> PointerCtx<'ctx> {
    /// 📥️ Dispatches the oldest import's next chunks, in order and one awaited at a time (the guest's staging refuses a
    /// gap), within [`IMPORT_TRANSFER_TURN_MS`]; a finished import leaves, and a refused chunk ends its import with the
    /// `import-failed` notice. Answers the chunks it dispatched.
    pub async fn step_import_transfers(&mut self) -> usize {
        let started = chrome_now_ms();
        let mut sent = 0usize;
        while let Some(chunk) = self.opening.import_transfers.front_mut().and_then(|transfer| transfer.chunks.pop_front()) {
            sent += 1;
            match self.as_actions().dispatch_action(chunk).await {
                Ok(()) => {
                    if let Some(transfer) = self.opening.import_transfers.front_mut() {
                        transfer.delivered += 1;
                    }
                }
                Err(error) => {
                    if let Some(failed) = self.opening.import_transfers.pop_front() {
                        debug_log(&format!("[TRACE] wgpu-shell import of {} failed: {error}", failed.file));
                        let text = import_transfer_notice("import-failed", &failed.first_file, self.prefs.active_locale());
                        self.chrome.show_transient_notice(text, semio_framework::Severity::Error, Some("shell.documentTransfer.import-failed"));
                    }
                }
            }
            if self.opening.import_transfers.front().is_some_and(|transfer| transfer.chunks.is_empty()) {
                self.opening.import_transfers.pop_front();
            }
            if chrome_now_ms() - started >= IMPORT_TRANSFER_TURN_MS {
                break;
            }
        }
        sent
    }
}
//#endregion 📤️FileOpenImport
