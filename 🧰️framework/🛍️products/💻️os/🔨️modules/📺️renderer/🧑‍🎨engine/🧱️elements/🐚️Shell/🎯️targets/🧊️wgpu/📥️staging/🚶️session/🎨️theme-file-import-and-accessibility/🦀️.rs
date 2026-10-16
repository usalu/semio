//! 🚶️ `shell-session` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

/// ⬇️ Browser half of the wgpu shell's export door. `encoding` used to be ignored here, so a binary
/// export reached the user as its own base64 TEXT under a binary file name — the same defect React's
/// shell carried. The bytes come from `kernel::media_export_bytes`, the ONE contract both renderers
/// answer to (`🎠️kernel/🧫️fixtures/⬇️media-export-encoding/🔣️.json`).
///
/// ⬇️ The bytes are decided HERE, by the kernel contract both renderers answer to; only the
/// presentation crosses to the page.
#[cfg(target_arch = "wasm32")]
pub fn download_media_export(filename: &str, mime_type: &str, data: &str, encoding: Option<&str>) {
    let bytes = match semio_framework::kernel::media_export_bytes(data, encoding) {
        Ok(bytes) => bytes,
        Err(error) => {
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!("[TRACE] wgpu-shell download {filename} refused: {error}")));
            return;
        }
    };
    let (filename, mime_type) = (filename.to_string(), mime_type.to_string());
    semio_framework_os_renderer_wgpu_host_base::spawn_app_task(async move {
        let length = bytes.len();
        match present_media_export_bytes(filename.clone(), mime_type, bytes).await {
            Ok(true) => debug_log(&format!("[TRACE] wgpu-shell download presented name={filename} bytes={length}")),
            Ok(false) => {}
            Err(error) => debug_log(&format!("[TRACE] wgpu-shell download {filename} not presented: {error}")),
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn download_media_export(filename: &str, mime_type: &str, data: &str, encoding: Option<&str>) {
    let (filename, mime_type, data, encoding) = (filename.to_string(), mime_type.to_string(), data.to_string(), encoding.map(str::to_string));
    ShellPoolFuture::spawn(semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool(), Lane::Io, async move {
        download_media_export_worker(&filename, &mime_type, &data, encoding.as_deref()).await;
    });
}

#[cfg(not(target_arch = "wasm32"))]
async fn download_media_export_worker(filename: &str, mime_type: &str, data: &str, encoding: Option<&str>) {
    let bytes = match semio_framework::kernel::media_export_bytes(data, encoding) {
        Ok(bytes) => bytes,
        Err(error) => {
            debug_log(&format!("[TRACE] wgpu-shell download {filename} refused: {error}"));
            return;
        }
    };
    if let Err(error) = present_media_export_bytes(filename.to_string(), mime_type.to_string(), bytes).await {
        debug_log(&format!("[TRACE] wgpu-shell download {filename} not presented: {error}"));
    }
}

/// 📦️ Presents accepted bytes without a base64 roundtrip; false means the user cancelled.
#[cfg(target_arch = "wasm32")]
pub async fn present_media_export_bytes(filename: String, mime_type: String, bytes: Vec<u8>) -> Result<bool, String> {
    present_media_export_bytes_cancellable(filename, mime_type, bytes, CancelToken::root_now()).await
}

/// 💾️ Writes accepted bytes only after the native save dialog returns a destination.
#[cfg(not(target_arch = "wasm32"))]
pub async fn present_media_export_bytes(filename: String, mime_type: String, bytes: Vec<u8>) -> Result<bool, String> {
    present_media_export_bytes_cancellable(filename, mime_type, bytes, CancelToken::root_now()).await
}

//#region 📤️FileOpenImport
/// 📥️ The verb an app declares to free an import a host stopped part-way (remodel's `importAbort {reason?}`) — React's
/// `IMPORT_ABORT_ACTION_ID`: sent without a reason, a person's cancel, once a cancelled import handed the guest a chunk.
pub const IMPORT_ABORT_ACTION_ID: &str = "importAbort";

/// 🗣️ One document-transfer outcome a person is told about — React's `DOCUMENT_TRANSFER_NOTICE_LABELS_V1`, by its code
/// `shell.documentTransfer.<notice>`; `{file}` names the file.
pub fn import_transfer_notice(notice: &str, file: &str, locale: Locale) -> String {
    let (en, de) = match notice {
        "import-cancelled" => ("Import of “{file}” cancelled.", "Import von „{file}“ abgebrochen."),
        _ => ("“{file}” could not be opened as a document.", "„{file}“ konnte nicht als Dokument geöffnet werden."),
    };
    match locale {
        Locale::En => en,
        Locale::De => de,
    }
    .replace("{file}", file)
}

impl<'ctx> SessionCtx<'ctx> {
    /// 🛑️ A person's Cancel of one import: its chunks are dropped, an import that already handed the guest a chunk arms the
    /// app's `importAbort` when the app declares it, and the person is told — React's `importOpenedFilesV1` cancel.
    pub fn cancel_import_transfer(&mut self, id: u64) -> bool {
        let Some(index) = self.opening.import_transfers.iter().position(|transfer| transfer.id == id) else { return false };
        let Some(transfer) = self.opening.import_transfers.remove(index) else { return false };
        let declares_abort = self.docs.session.as_ref().is_some_and(|session| session.app.actions.iter().any(|action| action.id == IMPORT_ABORT_ACTION_ID));
        if transfer.delivered > 0 && declares_abort {
            self.settle.deferred_actions.push(ActionDescriptor { controller_id: transfer.controller_id.clone(), action: IMPORT_ABORT_ACTION_ID.into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({}) });
        }
        let text = import_transfer_notice("import-cancelled", &transfer.first_file, self.prefs.active_locale());
        self.chrome.show_transient_notice(text, semio_framework::Severity::Info, Some("shell.documentTransfer.import-cancelled"));
        true
    }
}

//#endregion 📤️FileOpenImport

