//! 🎨️ Media export, file import, theme documents and chrome accessibility.
//!
//! Media export, file open/import, host IO tracing, media frame requests, the theme document model and editor,
//! theme doors, merge policy and the chrome accessible names and nodes.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

/// ⬇️ Browser half of the wgpu shell's export door. `encoding` used to be ignored here, so a binary
/// export reached the user as its own base64 TEXT under a binary file name — the same defect React's
/// shell carried. The bytes come from `kernel::media_export_bytes`, the ONE contract both renderers
/// answer to (`🎠️kernel/🧫️fixtures/⬇️media-export-encoding/🔣️.json`).
///
/// ⬇️ The bytes are decided HERE, by the kernel contract both renderers answer to; only the
/// presentation crosses to the page.
#[cfg(target_arch = "wasm32")]
pub(crate) fn download_media_export(filename: &str, mime_type: &str, data: &str, encoding: Option<&str>) {
    let bytes = match semio_framework::kernel::media_export_bytes(data, encoding) {
        Ok(bytes) => bytes,
        Err(error) => {
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!("[TRACE] wgpu-shell download {filename} refused: {error}")));
            return;
        }
    };
    let (filename, mime_type) = (filename.to_string(), mime_type.to_string());
    crate::spawn_app_task(async move {
        let length = bytes.len();
        match present_media_export_bytes(filename.clone(), mime_type, bytes).await {
            Ok(true) => ShellState::debug_log(&format!("[TRACE] wgpu-shell download presented name={filename} bytes={length}")),
            Ok(false) => {}
            Err(error) => ShellState::debug_log(&format!("[TRACE] wgpu-shell download {filename} not presented: {error}")),
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn download_media_export(filename: &str, mime_type: &str, data: &str, encoding: Option<&str>) {
    let (filename, mime_type, data, encoding) = (filename.to_string(), mime_type.to_string(), data.to_string(), encoding.map(str::to_string));
    ShellPoolFuture::spawn(crate::renderer_worker_pool(), Lane::Io, async move {
        download_media_export_worker(&filename, &mime_type, &data, encoding.as_deref()).await;
    });
}

#[cfg(not(target_arch = "wasm32"))]
async fn download_media_export_worker(filename: &str, mime_type: &str, data: &str, encoding: Option<&str>) {
    let bytes = match semio_framework::kernel::media_export_bytes(data, encoding) {
        Ok(bytes) => bytes,
        Err(error) => {
            ShellState::debug_log(&format!("[TRACE] wgpu-shell download {filename} refused: {error}"));
            return;
        }
    };
    if let Err(error) = present_media_export_bytes(filename.to_string(), mime_type.to_string(), bytes).await {
        ShellState::debug_log(&format!("[TRACE] wgpu-shell download {filename} not presented: {error}"));
    }
}

/// 📦️ Presents accepted bytes without a base64 roundtrip; false means the user cancelled.
#[cfg(target_arch = "wasm32")]
pub(crate) async fn present_media_export_bytes(filename: String, mime_type: String, bytes: Vec<u8>) -> Result<bool, String> {
    present_media_export_bytes_cancellable(filename, mime_type, bytes, CancelToken::root_now()).await
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn present_media_export_bytes_cancellable(filename: String, mime_type: String, bytes: Vec<u8>, cancel: CancelToken) -> Result<bool, String> {
    if cancel.is_cancelled_now() { return Ok(false); }
    let request = serde_json::json!({ "op": "download-media-export", "filename": filename, "mimeType": mime_type }).to_string();
    host_io_call(&request, Some(&bytes)).await.map(|_| true)
}

/// 💾️ Writes accepted bytes only after the native save dialog returns a destination.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn present_media_export_bytes(filename: String, mime_type: String, bytes: Vec<u8>) -> Result<bool, String> {
    present_media_export_bytes_cancellable(filename, mime_type, bytes, CancelToken::root_now()).await
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn present_media_export_bytes_cancellable(filename: String, mime_type: String, bytes: Vec<u8>, cancel: CancelToken) -> Result<bool, String> {
    if cancel.is_cancelled_now() { return Ok(false); }
    let extension = std::path::Path::new(&filename).extension().and_then(|value| value.to_str()).unwrap_or_else(|| mime_type.rsplit_once('/').map(|(_, value)| value).unwrap_or("dat"));
    let Some(path) = ui_host::select_native_paths(ui_host::NativeFileDialogRequest::save(&filename, [extension])).await.into_iter().next() else { return Ok(false) };
    if cancel.is_cancelled_now() { return Ok(false); }
    std::fs::write(path, bytes).map_err(|error| error.to_string())?;
    Ok(true)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn request_file_save(filename: &str) -> Option<std::path::PathBuf> {
    ui_host::select_native_paths(ui_host::NativeFileDialogRequest::save(filename, ["json"])).await.into_iter().next()
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn pick_folder() -> Option<String> {
    ui_host::select_native_paths(ui_host::NativeFileDialogRequest::folder()).await.into_iter().next().map(|path| path.display().to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn pick_file_path() -> Option<String> {
    ui_host::select_native_paths(ui_host::NativeFileDialogRequest::open(["json"], false)).await.into_iter().next().map(|path| path.display().to_string())
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn pick_file_path() -> Option<String> {
    pick_native_path_from_host(serde_json::json!({ "op": "request-native-file-path", "accept": ".json,application/json" })).await
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn pick_folder() -> Option<String> {
    pick_native_path_from_host(serde_json::json!({ "op": "request-native-folder-path" })).await
}

#[cfg(target_arch = "wasm32")]
async fn pick_native_path_from_host(request: serde_json::Value) -> Option<String> {
    let answer = host_io_call(&request.to_string(), None).await.ok()?;
    serde_json::from_str::<Option<String>>(&answer).ok().flatten()
}

//#region 📤️FileOpenImport
/// 📤️ One file a picker handed back. The NAME is half the payload, not decoration: every import leaf
/// in the repo resolves the file's format from its extension (`document_io::import_row_for_name`),
/// so a shell that hands over contents alone can only guess.
pub struct OpenedFile {
    pub name: String,
    pub contents: String,
}

/// 📥️ Turns one picker answer into the actions a shell dispatches — one per
/// [`semio_framework::kernel::import_payload_chunks`] chunk of each file, in order, with the exact
/// argument envelope the effect contract owns.
///
/// ⚖️ The chunking and the envelope are the kernel's, never this shell's: this target used to send
/// `{json, payload}` in ONE unchunked invocation while React sent `{payload, name, chunk,
/// chunkCount}` per chunk, and a plugin could satisfy only one of them. An unchunked import also asks
/// the fixed guest heap for a contiguous block the size of the whole file
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn file_open_import_actions(controller_id: &str, import_action: &str, opened: Vec<OpenedFile>, multiple: bool, retained_args: Option<&semio_framework::DslValue>) -> Vec<ActionDescriptor> {
    let total = opened.len();
    let mut actions = Vec::new();
    for (index, file) in opened.into_iter().enumerate() {
        for chunk in semio_framework::kernel::import_payload_chunks(&file.contents) {
            let mut args = semio_framework::kernel::import_chunk_arguments(&file.name, &chunk, multiple.then_some((index, total)));
            if let (semio_framework::DslValue::Object(fields), Some(semio_framework::DslValue::Object(extra))) = (&mut args, retained_args) { for (name, value) in extra { if !fields.iter().any(|(key, _)| key == name) { fields.push((name.clone(), value.clone())); } } }
            actions.push(ActionDescriptor { controller_id: controller_id.to_string(), action: import_action.to_string(), args: Some(args) });
        }
    }
    actions
}

/// 📥️ The verb an app declares to free an import a host stopped part-way (remodel's `importAbort {reason?}`) — React's
/// `IMPORT_ABORT_ACTION_ID`: sent without a reason, a person's cancel, once a cancelled import handed the guest a chunk.
pub(crate) const IMPORT_ABORT_ACTION_ID: &str = "importAbort";

/// 🛑️ The shell verb a Task Manager row's Cancel sends (`{transfer}` on the `framework` controller).
pub(crate) const IMPORT_TRANSFER_CANCEL_ACTION: &str = "cancelImportTransfer";

/// ⏱️ The wall budget one drain spends dispatching import chunks, so a Cancel lands between chunks.
const IMPORT_TRANSFER_TURN_MS: f64 = 8.0;

/// 📥️ The files one picker answered, with the verb their chunks go to.
pub(crate) struct PickedImport {
    pub controller_id: String,
    pub import_action: String,
    pub args: Option<semio_framework::DslValue>,
    pub opened: Vec<OpenedFile>,
    pub multiple: bool,
}

/// 🎞️ What one transfer moves into the guest: a picked document's chunks, or the frames one decoded video yields
/// (`RequestMediaFrames`: one `frameAction` per sampled frame, then its `doneAction` — or its one `fallbackAction`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TransferLane {
    DocumentImport,
    VideoFrames,
}

impl TransferLane {
    /// 🏷️ The lane a Task Manager row names.
    pub(crate) fn label(self, de: bool) -> &'static str {
        match self {
            Self::DocumentImport => shell_chrome_string("taskManager.documentImport", de),
            Self::VideoFrames => shell_chrome_string("taskManager.videoFrames", de),
        }
    }
}

/// 📥️ One transfer moving into the session's program, dispatch by dispatch — React's `importOpenedFilesV1` task: the
/// Task Manager lists it with its progress and a Cancel, and a cancelled transfer that already handed the guest a chunk or a
/// frame is freed by the app's `importAbort`, so its open import transaction leaves zero trace.
#[derive(Clone, Debug)]
pub(crate) struct ImportTransfer {
    pub id: u64,
    pub file: String,
    pub first_file: String,
    pub owner: String,
    pub controller_id: String,
    pub lane: TransferLane,
    pub chunks: VecDeque<ActionDescriptor>,
    pub delivered: usize,
    pub total: usize,
}

/// 🗣️ One document-transfer outcome a person is told about — React's `DOCUMENT_TRANSFER_NOTICE_LABELS_V1`, by its code
/// `shell.documentTransfer.<notice>`; `{file}` names the file.
fn import_transfer_notice(notice: &str, file: &str, locale: Locale) -> String {
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

impl ShellState {
    /// 📥️ Starts a picked-file import as an [`ImportTransfer`] the drain steps; an empty pick starts nothing. `true` when one
    /// started.
    pub(crate) fn begin_import_transfer(&mut self, picked: PickedImport) -> bool {
        let PickedImport { controller_id, import_action, opened, args, multiple } = picked;
        let Some(first_file) = opened.first().map(|file| file.name.clone()) else { return false };
        let file = opened.iter().map(|file| file.name.as_str()).collect::<Vec<_>>().join(", ");
        let chunks: VecDeque<ActionDescriptor> = file_open_import_actions(&controller_id, &import_action, opened, multiple, args.as_ref()).into();
        Self::debug_log(&format!("[TRACE] wgpu-shell import {import_action} file={file} chunks={}", chunks.len()));
        self.queue_transfer(TransferLane::DocumentImport, file, first_file, controller_id, chunks)
    }

    /// 🎞️ Starts the frame dispatches of one decoded video as an [`ImportTransfer`] on the [`TransferLane::VideoFrames`]
    /// lane — the host cancel `RequestMediaFrames` lacked: listed with its progress and a Cancel, stepped within the same
    /// turn budget as an import, and a cancelled stream that already handed the guest a frame is freed by the app's
    /// `importAbort`, so its open import transaction (design §15) leaves zero trace. An empty answer starts nothing.
    pub(crate) fn begin_media_frames_transfer(&mut self, controller_id: String, frames: Vec<ActionDescriptor>) -> bool {
        let Some(file) = frames.first().map(|frame| frame.args.as_ref().and_then(|args| args.get("name")).and_then(semio_framework::DslValue::as_str).unwrap_or("video").to_string()) else { return false };
        self.queue_transfer(TransferLane::VideoFrames, file.clone(), file, controller_id, frames.into())
    }

    /// 🧾️ Queues one transfer behind the running ones under a fresh id, owned by the session's plugin.
    fn queue_transfer(&mut self, lane: TransferLane, file: String, first_file: String, controller_id: String, chunks: VecDeque<ActionDescriptor>) -> bool {
        let owner = self.session.as_ref().map(|session| session.plugin_id.clone()).unwrap_or_default();
        let id = self.import_transfer_next;
        self.import_transfer_next = self.import_transfer_next.wrapping_add(1).max(1);
        self.import_transfers.push_back(ImportTransfer { id, file, first_file, owner, controller_id, lane, total: chunks.len(), chunks, delivered: 0 });
        true
    }

    /// 📥️ Dispatches the oldest import's next chunks, in order and one awaited at a time (the guest's staging refuses a
    /// gap), within [`IMPORT_TRANSFER_TURN_MS`]; a finished import leaves, and a refused chunk ends its import with the
    /// `import-failed` notice. Answers the chunks it dispatched.
    pub(crate) async fn step_import_transfers(&mut self) -> usize {
        let started = chrome_now_ms();
        let mut sent = 0usize;
        while let Some(chunk) = self.import_transfers.front_mut().and_then(|transfer| transfer.chunks.pop_front()) {
            sent += 1;
            match self.dispatch_action(chunk).await {
                Ok(()) => {
                    if let Some(transfer) = self.import_transfers.front_mut() {
                        transfer.delivered += 1;
                    }
                }
                Err(error) => {
                    if let Some(failed) = self.import_transfers.pop_front() {
                        Self::debug_log(&format!("[TRACE] wgpu-shell import of {} failed: {error}", failed.file));
                        let text = import_transfer_notice("import-failed", &failed.first_file, self.active_locale());
                        self.show_transient_notice(text, semio_framework::Severity::Error, Some("shell.documentTransfer.import-failed"));
                    }
                }
            }
            if self.import_transfers.front().is_some_and(|transfer| transfer.chunks.is_empty()) {
                self.import_transfers.pop_front();
            }
            if chrome_now_ms() - started >= IMPORT_TRANSFER_TURN_MS {
                break;
            }
        }
        sent
    }

    /// 🛑️ A person's Cancel of one import: its chunks are dropped, an import that already handed the guest a chunk arms the
    /// app's `importAbort` when the app declares it, and the person is told — React's `importOpenedFilesV1` cancel.
    pub(crate) fn cancel_import_transfer(&mut self, id: u64) -> bool {
        let Some(index) = self.import_transfers.iter().position(|transfer| transfer.id == id) else { return false };
        let Some(transfer) = self.import_transfers.remove(index) else { return false };
        let declares_abort = self.session.as_ref().is_some_and(|session| session.app.actions.iter().any(|action| action.id == IMPORT_ABORT_ACTION_ID));
        if transfer.delivered > 0 && declares_abort {
            self.deferred_actions.push(ActionDescriptor { controller_id: transfer.controller_id.clone(), action: IMPORT_ABORT_ACTION_ID.into(), args: crate::action_args_json!({}) });
        }
        let text = import_transfer_notice("import-cancelled", &transfer.first_file, self.active_locale());
        self.show_transient_notice(text, semio_framework::Severity::Info, Some("shell.documentTransfer.import-cancelled"));
        true
    }

    /// 🧵️ The Task Manager's running-tasks section — React's `TaskManagerTasksPanel` over `documentTransferTasksV1`: per import
    /// its title, lane, owner and state, a progress bar of the chunks delivered, and "Cancel <file>"; "No task is running."
    /// while none runs.
    pub(crate) fn import_task_nodes(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        let pick = |en: &str, de: &str| if is_de { de.to_string() } else { en.to_string() };
        let mut children = vec![settings_text_row(shell_chrome_string("taskManager.runningTasks", is_de))];
        if self.import_transfers.is_empty() {
            children.push(settings_text_row(&pick("No task is running.", "Es läuft keine Aufgabe.")));
        }
        for transfer in &self.import_transfers {
            let task = format!("documentTransfer:{}", transfer.id);
            let progress = format!("{}/{}", transfer.delivered, transfer.total);
            children.push(settings_text_row(&format!("{} · {} · {} · {}", transfer.file, transfer.lane.label(is_de), transfer.owner, pick("Running", "Läuft"))));
            children.push(UiNode::Progress(ui_wgpu::wgpu::component::ui::UiProgressNode {
                id: format!("os.task-manager.progress.{task}"),
                completed: transfer.delivered as f64,
                total: Some(transfer.total as f64),
                value_text: Label::data(if is_de { format!("Fortschritt von {}: {progress}", transfer.file) } else { format!("Progress of {}: {progress}", transfer.file) }),
                presence: UiPresence::default(),
                menu: None,
            }));
            children.push(UiNode::Button(UiButtonNode {
                id: Some(format!("os.task-manager.cancel.{task}")),
                icon_id: IconName::Square,
                label: Label::data(if is_de { format!("{} abbrechen", transfer.file) } else { format!("Cancel {}", transfer.file) }),
                action: ActionDescriptor { controller_id: "framework".into(), action: IMPORT_TRANSFER_CANCEL_ACTION.into(), args: crate::action_args_json!({ "transfer": transfer.id }) },
                style: None,
                presence: UiPresence::default(),
                menu: None,
            }));
        }
        UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some("os.task-manager.tasks".into()), children, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None })
    }
}

/// 📤️ Opens the native file picker; one entry per selected file, in selection order.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn request_file_open(accept: &str, read_as: Option<&str>, multiple: bool) -> Vec<OpenedFile> {
    use std::fs as system_fs;
    let extensions: Vec<String> = accept.split(',').filter_map(|entry| entry.trim().strip_prefix('.').map(str::to_string)).collect();
    let paths = ui_host::select_native_paths(ui_host::NativeFileDialogRequest::open(extensions.clone(), multiple)).await;
    paths
        .into_iter()
        .filter_map(|path| {
            let name = path.file_name().map(|name| name.to_string_lossy().to_string()).unwrap_or_default();
            if read_as == Some("dataUrl") {
                let bytes = system_fs::read(&path).ok()?;
                let mime = extensions.first().map(|ext| format!("application/{ext}")).unwrap_or_else(|| "application/octet-stream".into());
                return Some(OpenedFile { name, contents: format!("data:{mime};base64,{}", semio_framework_io_base64::base64_standard_encode(&bytes)) });
            }
            system_fs::read_to_string(path).ok().map(|contents| OpenedFile { name, contents })
        })
        .collect()
}

/// 🚪️ Calls the ONE file-door binding the host environment installs (`semioWgpuHostIo`,
/// `🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts`). The PAGE installs it directly; the frame Worker installs a
/// `postMessage` bridge to the same implementation running on the page.
///
/// 🐛️ Why this indirection is the fix and not an indirection: this shell runs inside a dedicated Worker
/// (the one that owns the `OffscreenCanvas`), and a Worker has NO `window` and NO `document`. Both
/// browser halves here used to start with `web_sys::window()?` and return silently — so `Export
/// Document…` produced real bytes and handed them to nobody, and `Import Document…` opened nothing at
/// all, while every other hop of both journeys was already correct
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §C).
#[cfg(target_arch = "wasm32")]
static HOST_IO_REQUEST_TRACE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

#[cfg(target_arch = "wasm32")]
struct HostIoRequestTraceGuard(u8);

#[cfg(target_arch = "wasm32")]
impl HostIoRequestTraceGuard {
    pub(crate) fn begin(request_json: &str) -> Self {
        let trace = if request_json.contains("\"op\":\"directory-http\"") {
            if request_json.contains("/auth/sessions/me") {
                1
            } else if request_json.contains("\"method\":\"POST\"") && request_json.contains("/directory/commands") {
                2
            } else if request_json.contains("/active-checkpoint/") {
                3
            } else if request_json.contains("/directory/spaces") {
                4
            } else {
                5
            }
        } else if request_json.contains("request-file-") || request_json.contains("download-media-export") {
            6
        } else if request_json.contains("extension-store-") {
            7
        } else if request_json.contains("format-temporal-values") {
            8
        } else if request_json.contains("\"op\":\"storage\"") {
            9
        } else if request_json.contains("\"op\":\"socket\"") {
            10
        } else {
            11
        };
        Self(HOST_IO_REQUEST_TRACE.swap(trace, std::sync::atomic::Ordering::AcqRel))
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for HostIoRequestTraceGuard {
    fn drop(&mut self) {
        HOST_IO_REQUEST_TRACE.store(self.0, std::sync::atomic::Ordering::Release);
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn host_io_request_trace_label() -> &'static str {
    match HOST_IO_REQUEST_TRACE.load(std::sync::atomic::Ordering::Acquire) {
        1 => "directory-get-session",
        2 => "directory-post-command",
        3 => "directory-get-checkpoint",
        4 => "directory-get-spaces",
        5 => "directory-http-other",
        6 => "file-dialog-or-download",
        7 => "extension-store",
        8 => "temporal-format",
        9 => "storage",
        10 => "socket",
        11 => "host-io-other",
        _ => "none",
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn host_io_request_trace_label() -> &'static str {
    "none"
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn host_io_call(request_json: &str, bytes: Option<&[u8]>) -> Result<String, String> {
    use wasm_bindgen::JsCast;
    let _trace = HostIoRequestTraceGuard::begin(request_json);
    let global = js_sys::global();
    let door = js_sys::Reflect::get(&global, &wasm_bindgen::JsValue::from_str("semioWgpuHostIo")).ok().and_then(|value| value.dyn_into::<js_sys::Function>().ok()).ok_or_else(|| "this isolate installs no semioWgpuHostIo door".to_string())?;
    let payload = match bytes {
        Some(bytes) => js_sys::Uint8Array::from(bytes).into(),
        None => wasm_bindgen::JsValue::NULL,
    };
    let result = door.call2(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_str(request_json), &payload).map_err(|error| format!("{error:?}"))?;
    let resolved = match result.dyn_ref::<js_sys::Promise>() {
        Some(promise) => semio_framework_async::browser::JsFuture::from(promise.clone()).await.map_err(|error| format!("{error:?}"))?,
        None => result,
    };
    resolved.as_string().ok_or_else(|| "semioWgpuHostIo answered a non-string".to_string())
}

/// 📤️ Opens the browser file picker through the page-owned door; one entry per selected file, in
/// selection order, EMPTY on cancel or on a refusal (which is logged, never swallowed).
#[cfg(target_arch = "wasm32")]
pub(crate) async fn request_file_open(accept: &str, read_as: Option<&str>, multiple: bool) -> Vec<OpenedFile> {
    let request = serde_json::json!({ "op": "request-file-open", "accept": accept, "readAs": read_as, "multiple": multiple }).to_string();
    let answer = match host_io_call(&request, None).await {
        Ok(answer) => answer,
        Err(error) => {
            ShellState::debug_log(&format!("[TRACE] wgpu-shell file picker refused: {error}"));
            return Vec::new();
        }
    };
    #[derive(serde::Deserialize)]
    struct PickedFile {
        name: String,
        contents: String,
    }
    match serde_json::from_str::<Vec<PickedFile>>(&answer) {
        Ok(files) => files.into_iter().map(|file| OpenedFile { name: file.name, contents: file.contents }).collect(),
        Err(error) => {
            ShellState::debug_log(&format!("[TRACE] wgpu-shell file picker answer unreadable: {error}"));
            Vec::new()
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/📤️wgpu-file-open-import/🦀️.rs"]
mod file_open_import_tests;
//#endregion 📤️FileOpenImport

//#region RequestMediaFrames
/// 🔓️ Decodes a `data:<mime>;base64,<data>` URL's payload; `None` for anything malformed or missing a
/// comma separator. Used by both `RequestMediaFrames.payload` (bytes already in memory) and the
/// fallback-response encoding below.
fn decode_data_url(data_url: &str) -> Option<Vec<u8>> {
    use base64::Engine;
    let comma = data_url.find(',')?;
    base64::engine::general_purpose::STANDARD.decode(&data_url[comma + 1..]).ok()
}

/// 📦️ Builds the single `fallback_action` `ActionDescriptor` — raw `bytes` re-encoded as a data URL
/// merged into `base_args`, same shape on every failure path (`ffmpeg` missing, spawn/scratch-dir I/O
/// failure, no source bytes at all) so the plugin's fallback handler only needs to handle one shape.
fn fallback_action_descriptor(controller_id: &str, fallback_action: &str, bytes: &[u8], name: &str, base_args: &Value) -> ActionDescriptor {
    use base64::Engine;
    let mut args = base_args.clone();
    if let Some(obj) = args.as_object_mut() {
        obj.insert("payload".into(), Value::String(format!("data:application/octet-stream;base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes))));
        obj.insert("name".into(), Value::String(name.to_string()));
    }
    ActionDescriptor { controller_id: controller_id.to_string(), action: fallback_action.to_string(), args: Some(semio_framework::DslValue::from(args)) }
}

/// 🧮️ Pure `ffmpeg` argument computation for D5 frame extraction (precedent: `animate/video/rs/lib.rs`'s
/// `run_ffmpeg`, which this shell doesn't depend on directly — cross-technology dep avoided per house
/// rules — but whose `Command::new("ffmpeg").args(...).status()` invocation convention this follows).
/// `sample_stride` floors at 1 (every frame); `max_frames` of 0 means "host default" (capped generously
/// rather than truly unlimited, so a pathological request can't fill disk); `max_long_edge_px` of 0
/// skips the scale filter entirely (native resolution).
#[cfg(any(not(target_arch = "wasm32"), test))]
fn ffmpeg_frame_extraction_args(sample_stride: u32, max_frames: u32, max_long_edge_px: u32, input: &std::path::Path, out_dir: &std::path::Path) -> Vec<String> {
    let stride = sample_stride.max(1);
    let filter = if max_long_edge_px > 0 { format!("select=not(mod(n\\,{stride})),scale={max_long_edge_px}:-2") } else { format!("select=not(mod(n\\,{stride}))") };
    let frames_cap = if max_frames > 0 { max_frames } else { 100_000 };
    vec!["-y".into(), "-i".into(), input.display().to_string(), "-vf".into(), filter, "-vsync".into(), "vfr".into(), "-frames:v".into(), frames_cap.to_string(), out_dir.join("%06d.jpg").display().to_string()]
}

/// ⏱️ Approximate per-extracted-frame timestamp from the requested sampling cadence — `ffmpeg`'s own
/// frame PTS aren't threaded back through the `%06d.jpg` sequence (would need an `ffprobe` pass or
/// `-frame_pts`/timebase math this ticket scopes out; documented simplification, same spirit as the D1
/// wgpu point-sprite pass note above `render_world_3d`). Good enough for frame *ordering*/spacing; exact
/// decode timestamps are future work if a downstream consumer needs true sub-frame sync.
#[cfg(any(not(target_arch = "wasm32"), test))]
fn approx_sampled_timestamp_ms(index: u32, sample_stride: u32, fps_hint: f64) -> f64 {
    let stride = sample_stride.max(1) as f64;
    let fps = if fps_hint > 0.0 { fps_hint } else { 30.0 };
    (index as f64) * stride / fps * 1000.0
}

#[cfg(not(target_arch = "wasm32"))]
fn ffmpeg_available() -> bool {
    use std::process as system_process;
    system_process::Command::new("ffmpeg").arg("-version").stdout(system_process::Stdio::null()).stderr(system_process::Stdio::null()).status().is_ok_and(|status| status.success())
}

/// 🎞️ D5 native pipeline, beside `request_file_open` above: obtains video bytes (native file
/// picker, or `payload`'s data-URL bytes when the caller already has them from a drop zone), shells out
/// to `ffmpeg` to sample frames into a scratch temp directory, base64-encodes each resulting JPEG, and
/// returns one `ActionDescriptor` per frame (`frame_action`) followed by one for `done_action` — or, on
/// any failure (no `ffmpeg` on `PATH`, no source bytes, scratch I/O failure, non-zero `ffmpeg` exit), a
/// single `fallback_action` descriptor carrying the raw bytes so the plugin's own in-process decoder
/// (MJPEG/H.264-baseline) gets a chance.
#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn request_media_frames(
    controller_id: &str,
    accept: &str,
    frame_action: &str,
    done_action: &str,
    fallback_action: &str,
    sample_stride: u32,
    max_frames: u32,
    max_long_edge_px: u32,
    fps_hint: f64,
    payload: Option<&str>,
    args: Option<Value>,
) -> Vec<ActionDescriptor> {
    use base64::Engine;
    use std::fs as system_fs;
    use std::process as system_process;
    let base_args = args.unwrap_or_else(|| serde_json::json!({}));
    let (bytes, name) = match payload {
        Some(data_url) => match decode_data_url(data_url) {
            Some(decoded) => (decoded, "video".to_string()),
            None => return Vec::new(),
        },
        None => {
            let extensions: Vec<String> = accept.split(',').filter_map(|entry| entry.trim().strip_prefix('.').map(str::to_string)).collect();
            let Some(path) = ui_host::select_native_paths(ui_host::NativeFileDialogRequest::open(extensions, false)).await.into_iter().next() else {
                return Vec::new();
            };
            let Ok(bytes) = system_fs::read(&path) else {
                return Vec::new();
            };
            let name = path.file_name().map_or_else(|| "video".to_string(), |value| value.to_string_lossy().into_owned());
            (bytes, name)
        }
    };
    if !ffmpeg_available() {
        return vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, &name, &base_args)];
    }
    let scratch_dir = std::env::temp_dir().join(format!("semio-media-frames-{}-{}", system_process::id(), name.len()));
    if system_fs::create_dir_all(&scratch_dir).is_err() {
        return vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, &name, &base_args)];
    }
    let input_path = scratch_dir.join("source.bin");
    if system_fs::write(&input_path, &bytes).is_err() {
        let _ = system_fs::remove_dir_all(&scratch_dir);
        return vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, &name, &base_args)];
    }
    let ffmpeg_args = ffmpeg_frame_extraction_args(sample_stride, max_frames, max_long_edge_px, &input_path, &scratch_dir);
    let extracted = system_process::Command::new("ffmpeg").args(&ffmpeg_args).status().is_ok_and(|status| status.success());
    if !extracted {
        let _ = system_fs::remove_dir_all(&scratch_dir);
        return vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, &name, &base_args)];
    }
    let mut frame_paths: Vec<std::path::PathBuf> =
        system_fs::read_dir(&scratch_dir).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.path())).filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("jpg")).collect()).unwrap_or_default();
    frame_paths.sort();
    let total = frame_paths.len();
    let mut actions = Vec::with_capacity(total + 1);
    for (index, frame_path) in frame_paths.iter().enumerate() {
        let Ok(jpeg_bytes) = system_fs::read(frame_path) else {
            continue;
        };
        let mut frame_args = base_args.clone();
        if let Some(obj) = frame_args.as_object_mut() {
            obj.insert("payload".into(), Value::String(format!("data:image/jpeg;base64,{}", base64::engine::general_purpose::STANDARD.encode(&jpeg_bytes))));
            obj.insert("name".into(), Value::String(name.clone()));
            obj.insert("frameIndex".into(), serde_json::json!(index));
            obj.insert("timestampMs".into(), serde_json::json!(approx_sampled_timestamp_ms(index as u32, sample_stride, fps_hint)));
            obj.insert("index".into(), serde_json::json!(index));
            obj.insert("total".into(), serde_json::json!(total));
        }
        actions.push(ActionDescriptor { controller_id: controller_id.to_string(), action: frame_action.to_string(), args: Some(semio_framework::DslValue::from(frame_args)) });
    }
    let mut done_args = base_args;
    if let Some(obj) = done_args.as_object_mut() {
        obj.insert("name".into(), Value::String(name));
        obj.insert("frameCount".into(), serde_json::json!(total));
        obj.insert("sampledCount".into(), serde_json::json!(total));
    }
    actions.push(ActionDescriptor { controller_id: controller_id.to_string(), action: done_action.to_string(), args: Some(semio_framework::DslValue::from(done_args)) });
    let _ = system_fs::remove_dir_all(&scratch_dir);
    actions
}

/// 🕸️ wasm32 mirrors `request_file_open`'s stub: no native `ffmpeg`/file-dialog surface, the browser
/// React shell handles `RequestMediaFrames` itself. If `payload` bytes are already in hand (a drop
/// zone), still honor `fallback_action` with them so an in-process program decoder gets a chance even on
/// this native/wasm shell.
#[cfg(target_arch = "wasm32")]
#[allow(clippy::too_many_arguments)]
pub(crate) fn request_media_frames(
    controller_id: &str,
    _accept: &str,
    _frame_action: &str,
    _done_action: &str,
    fallback_action: &str,
    _sample_stride: u32,
    _max_frames: u32,
    _max_long_edge_px: u32,
    _fps_hint: f64,
    payload: Option<&str>,
    args: Option<Value>,
) -> Vec<ActionDescriptor> {
    match payload.and_then(decode_data_url) {
        Some(bytes) => vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, "video", &args.unwrap_or_else(|| serde_json::json!({})))],
        None => Vec::new(),
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-media-frames/🦀️.rs"]
mod media_frames_tests;
//#endregion RequestMediaFrames

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-context-menu-keyboard/🦀️.rs"]
mod context_menu_keyboard_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/⬇️wgpu-media-export-encoding/🦀️.rs"]
mod media_export_encoding_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/📐️wgpu-dpi-logical-units/🦀️.rs"]
mod dpi_logical_units_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/💓️chrome-maintenance-pressure/🦀️.rs"]
mod chrome_maintenance_pressure_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🎬️wgpu-plugin-install/🦀️.rs"]
mod plugin_install_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🔀️wgpu-document-relay/🦀️.rs"]
mod document_relay_tests;

//#region 🎨️ThemeDocumentModel
/// 🎨️ The AUTHORED design-token document — the very `🔣️.json` the styling codegen turns into
/// React's `STYLING_SEMIO_THEME` and into this target's compile-time `ui_styling` constants. Embedded
/// so the wgpu theme EDITOR can enumerate exactly the keys React's editor enumerates
/// (`host.theme.colors`/`spacing`/`fontStacks`/`strokes`/`radii`/`opacities`/`metrics`/`appearances`),
/// rather than a hand-kept second key list that would drift on the next regeneration.
///
/// ⚖️ The generated `STYLING_SEMIO_THEME` is this document minus `version`/`fontFaces`/`levels`/
/// `presence` plus `id`/`label`, which is exactly what [`ThemeDocument`]'s `#[serde(default)]` fields
/// keep and what [`shell_theme_document_base`] stamps back on.
const SHELL_THEME_DOCUMENT_JSON: &str = include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🔣️.json");

/// 🎨️ React's `UiTheme["strokes"][k]` — `number | number[]` (a dash pattern is a list).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum ThemeNumber {
    Scalar(f64),
    List(Vec<f64>),
}

impl ToValue for ThemeNumber {
    fn to_value(&self)->DslValue{match self{Self::Scalar(value)=>value.to_value(),Self::List(values)=>values.to_value()}}
}
impl FromValue for ThemeNumber {
    fn from_value(value:DslValue)->Result<Self,semio_framework_value::ValueError>{match value{DslValue::Array(_)=>Vec::<f64>::from_value(value).map(Self::List),_=>f64::from_value(value).map(Self::Scalar)}}
}

impl ThemeNumber {
    /// 🔢️ React's `Array.isArray(value) ? value.join(", ") : String(value)` — the text
    /// `themeNumberInputRow` puts in the box.
    fn as_text(&self) -> String {
        match self {
            ThemeNumber::Scalar(value) => format_theme_scalar(*value),
            ThemeNumber::List(values) => values.iter().map(|value| format_theme_scalar(*value)).collect::<Vec<_>>().join(", "),
        }
    }

    /// 🔢️ React's `onBlur` parse: a comma anywhere makes it a list, otherwise one float; an
    /// unparseable buffer commits NOTHING (React returns without calling `onCommit`).
    pub(crate) fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        if raw.contains(',') {
            let parts: Vec<f64> = raw.split(',').filter_map(|part| part.trim().parse::<f64>().ok()).collect();
            return (!parts.is_empty()).then_some(ThemeNumber::List(parts));
        }
        raw.parse::<f64>().ok().map(ThemeNumber::Scalar)
    }

    /// 🔢️ The scalar a `Theme` field wants — a list answers its first entry, exactly as React's
    /// `setThemeRadius`/`setThemeOpacity` narrow `typeof value === "number" ? value : value[0]!`.
    pub(crate) fn scalar(&self) -> Option<f64> {
        match self {
            ThemeNumber::Scalar(value) => Some(*value),
            ThemeNumber::List(values) => values.first().copied(),
        }
    }
}

/// 🔢️ `String(n)` in JavaScript never prints a trailing `.0`, and these strings are compared against
/// React's boxes byte for byte in the parity probe.
fn format_theme_scalar(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

/// 🖌️ React's `ThemePaintRef` (`🖱️ui/🎨️styling/🌓️theme/🟦️.ts:33`) — a primitive token, a literal hex,
/// or an oklab-free channel blend of two tokens.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub(crate) struct ThemePaintRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mix: Option<(String, String, f64)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub(crate) struct ThemeIcons {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub aliases: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub variants: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub themed_aliases: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub themed_variants: BTreeMap<String, String>,
}

/// 🎨️ The wgpu twin of React's `UiTheme` (`🖱️ui/🎨️styling/🌓️theme/🟦️.ts:63`) — the document BOTH
/// renderers' theme editors mutate and BOTH persist into `os.config.ui-preferences`'
/// `customThemes[id].config`. Every map is a `BTreeMap`, so a section's rows enumerate in the same
/// ascending key order React's `Object.keys(...).sort()` produces and a saved document is byte-stable.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub(crate) struct ThemeDocument {
    #[serde(default)]
    #[value(default)]
    pub id: String,
    #[serde(default)]
    #[value(default)]
    pub label: String,
    #[serde(default)]
    #[value(default)]
    pub colors: BTreeMap<String, String>,
    #[serde(default)]
    #[value(default)]
    pub spacing: BTreeMap<String, String>,
    #[serde(default)]
    #[value(default)]
    pub font_stacks: BTreeMap<String, String>,
    #[serde(default)]
    #[value(default)]
    pub canvas_fonts: BTreeMap<String, String>,
    #[serde(default)]
    #[value(default)]
    pub strokes: BTreeMap<String, ThemeNumber>,
    #[serde(default)]
    #[value(default)]
    pub radii: BTreeMap<String, ThemeNumber>,
    #[serde(default)]
    #[value(default)]
    pub opacities: BTreeMap<String, ThemeNumber>,
    #[serde(default)]
    #[value(default)]
    pub metrics: BTreeMap<String, BTreeMap<String, ThemeNumber>>,
    #[serde(default)]
    #[value(default)]
    pub appearances: BTreeMap<String, BTreeMap<String, BTreeMap<String, ThemePaintRef>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icons: Option<ThemeIcons>,
}

#[path = "../../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🌓️theme/📐️geometry/🦀️.rs"]
mod theme_geometry;

impl ThemeDocument {
    /// 🧬️ Parses the one canonical theme schema shared by save, import, resolution and labels.
    pub(crate) fn parse(text: &str) -> Option<Self> {
        const FIELDS: &[&str] = &["id", "label", "colors", "spacing", "fontStacks", "canvasFonts", "strokes", "radii", "opacities", "metrics", "appearances"];
        const GROUPS: &[&str] = &["board", "map", "canvas", "chrome", "outcome", "diagram"];
        let value: serde_json::Value = serde_json::from_str(text).ok()?;
        let object = value.as_object()?;
        if FIELDS.iter().any(|field| !object.contains_key(*field)) || object.get("icons").is_some_and(|icons| !icons.is_object()) {
            return None;
        }
        let document: Self = serde_json::from_str(text).ok()?;
        let scalar_maps = document.radii.values().chain(document.opacities.values()).all(|value| matches!(value, ThemeNumber::Scalar(_)));
        let palettes = ["light", "dark"]
            .iter()
            .all(|appearance| document.appearances.get(*appearance).is_some_and(|groups| GROUPS.iter().all(|group| groups.get(*group).is_some_and(|paints| paints.values().all(|paint| theme_paint_ref_is_resolvable(&document.colors, paint))))));
        (scalar_maps && palettes && theme_document_geometry(&document).is_some()).then_some(document)
    }

    /// 💾️ Serializes the same validated document shape React stores and exports.
    pub(crate) fn canonical_json(&self) -> Option<String> {
        let text = serde_json::to_string_pretty(self).ok()?;
        Self::parse(&text).map(|_| text)
    }
}

pub(crate) fn custom_theme_document_from(prefs: &ChromePrefsState, custom_id: &str) -> Option<ThemeDocument> {
    ThemeDocument::parse(prefs.custom_themes.get(custom_id)?)
}

/// 🎨️ The authored base document, parsed once. `id`/`label` are stamped the way the styling codegen
/// stamps them onto `STYLING_SEMIO_THEME`.
pub(crate) fn shell_theme_document_base() -> &'static ThemeDocument {
    static BASE: std::sync::OnceLock<ThemeDocument> = std::sync::OnceLock::new();
    BASE.get_or_init(|| {
        let mut document: ThemeDocument = serde_json::from_str(SHELL_THEME_DOCUMENT_JSON).unwrap_or_else(|_| ThemeDocument {
            id: String::new(),
            label: String::new(),
            colors: BTreeMap::new(),
            spacing: BTreeMap::new(),
            font_stacks: BTreeMap::new(),
            canvas_fonts: BTreeMap::new(),
            strokes: BTreeMap::new(),
            radii: BTreeMap::new(),
            opacities: BTreeMap::new(),
            metrics: BTreeMap::new(),
            appearances: BTreeMap::new(),
            icons: None,
        });
        document.id = "semio".to_string();
        document.label = "semio".to_string();
        document
    })
}

/// 🖌️ `parseHex6` — React accepts `#rgb` and `#rrggbb` alike.
fn theme_parse_hex6(hex: &str) -> [u8; 3] {
    let raw = hex.trim().trim_start_matches('#');
    if raw.len() == 3 {
        let expand = |index: usize| u8::from_str_radix(&raw[index..index + 1].repeat(2), 16).unwrap_or(0);
        return [expand(0), expand(1), expand(2)];
    }
    let value = u32::from_str_radix(raw, 16).unwrap_or(0);
    [((value >> 16) & 0xff) as u8, ((value >> 8) & 0xff) as u8, (value & 0xff) as u8]
}

/// 🖌️ `blendHex` — a straight sRGB channel lerp, NOT an oklab mix: React's paint refs blend in the
/// authored space and only the LEVEL ramp below is oklab.
fn theme_blend_hex(a: &str, b: &str, ratio_a: f64) -> [u8; 3] {
    let left = theme_parse_hex6(a);
    let right = theme_parse_hex6(b);
    let t = ratio_a.clamp(0.0, 1.0);
    std::array::from_fn(|channel| (f64::from(left[channel]) * t + f64::from(right[channel]) * (1.0 - t)).round() as u8)
}

fn theme_paint_ref_is_resolvable(colors: &BTreeMap<String, String>, paint: &ThemePaintRef) -> bool {
    let token_exists = |token: &str| colors.get(token).is_some_and(|value| !value.is_empty());
    if let Some((left, right, _)) = paint.mix.as_ref() {
        return token_exists(left) && (right == "transparent" || token_exists(right));
    }
    paint.hex.is_some() || paint.token.as_deref().is_some_and(token_exists)
}

/// 🖌️ The Rust twin of `resolveThemePaint` (`🖱️ui/🎨️styling/🌓️theme/🟦️.ts:109`), including its
/// `"transparent"` mix special case (a mix against transparent carries `1 - ratio` as its alpha
/// unless the ref states one). An unresolvable ref answers `None` rather than inventing a colour.
fn theme_resolve_paint(colors: &BTreeMap<String, String>, paint: &ThemePaintRef) -> Option<[u8; 4]> {
    let mut alpha = paint.alpha.unwrap_or(1.0);
    let rgb = if let Some((left, right, ratio)) = paint.mix.as_ref() {
        let left_hex = colors.get(left)?;
        let right_hex = if right == "transparent" { "#000000".to_string() } else { colors.get(right)?.clone() };
        if right == "transparent" && paint.alpha.is_none() {
            alpha = 1.0 - ratio;
        }
        theme_blend_hex(left_hex, &right_hex, *ratio)
    } else if let Some(hex) = paint.hex.as_ref() {
        theme_parse_hex6(hex)
    } else {
        theme_parse_hex6(colors.get(paint.token.as_deref()?)?)
    };
    Some([rgb[0], rgb[1], rgb[2], (alpha.clamp(0.0, 1.0) * 255.0).round() as u8])
}

/// 🖌️ One resolved paint as this target's linear-float [`Rgba`], through the same
/// `ui_styling::color::rgba8_to_linear` decode every generated palette constant went through.
fn theme_paint_rgba(colors: &BTreeMap<String, String>, group: &BTreeMap<String, ThemePaintRef>, key: &str, fallback: Rgba) -> Rgba {
    group.get(key).and_then(|paint| theme_resolve_paint(colors, paint)).map(|[r, g, b, a]| Rgba::from_srgb8(r, g, b, a)).unwrap_or(fallback)
}

/// 🎨️ Re-tokenises one appearance of a `Theme` FROM a live document — the runtime twin of the ui
/// crate's compile-time `from_chrome`, field for field, so an edit in the settings panel repaints the
/// shell on the next frame exactly as React's `SET_UI_THEME_DRAFT` + `applyUiThemeToRoot` repaints
/// its DOM.
///
/// 🪜️ The six `level*` surfaces are NOT authored paints: the styling projection injects them as
/// `oklabMix(base, foreground, k · shadeStepPercent/100)` (`🖱️ui/🎨️styling/📽️projection/🟦️.ts:129`),
/// so they are recomputed here through `ui_styling::color::oklab_mix` — which is what makes editing
/// `chrome.base` move the navbar, the panels and the menu surface and not just the page background.
///
/// ⚖️ A key the document does not carry keeps the built-in value it had: this layers onto the
/// appearance's own generated `Theme`, it does not replace it, so a partial custom document can never
/// blank a surface.
///
/// 🪜️ The formula-derived ramp, recomputed off the two paints it is a function of.
pub(crate) fn theme_from_document(document: &ThemeDocument, dark: bool) -> Theme {
    let mut theme = if dark { Theme::dark() } else { Theme::light() };
    let colors = &document.colors;
    if let Some([r, g, b]) = colors.get("diff-added").map(|hex| theme_parse_hex6(hex)) {
        theme.diff_added = Rgba::from_srgb8(r, g, b, 255);
    }
    let appearance = document.appearances.get(if dark { "dark" } else { "light" });
    if let Some(chrome) = appearance.and_then(|groups| groups.get("chrome")) {
        let paint = |key: &str, fallback: Rgba| theme_paint_rgba(colors, chrome, key, fallback);
        let base = paint("base", theme.background);
        let foreground = paint("foreground", theme.text);
        theme.background = base;
        theme.input_bg = base;
        theme.canvas_clear = base;
        theme.text = foreground;
        theme.muted = paint("muted", theme.muted);
        theme.text_muted = paint("mutedForeground", theme.text_muted);
        theme.accent = paint("accent", theme.accent);
        theme.accent_hover = paint("activeHover", theme.accent_hover);
        theme.active_foreground = paint("activeForeground", theme.active_foreground);
        theme.selected = paint("activeBase", theme.selected);
        theme.button_hover = paint("hoverInteractiveFill", theme.button_hover);
        theme.row_hover = theme.button_hover;
        theme.separator = paint("borderNormal", theme.separator);
        theme.panel_border = theme.separator;
        theme.border_normal = theme.separator;
        theme.border_emphasized = paint("borderEmphasized", theme.border_emphasized);
        theme.text_element = paint("borderElement", theme.text_element);
        let focus_alpha = document.opacities.get("chromeFocusRingAlpha").and_then(ThemeNumber::scalar).unwrap_or(f64::from(theme.focus_ring.a));
        theme.focus_ring = theme.accent.with_alpha(focus_alpha as f32);
        let shade_step = ui_styling::levels::SHADE_STEP_PERCENT as f32 / 100.0;
        let base_linear = [base.r, base.g, base.b, base.a];
        let foreground_linear = [foreground.r, foreground.g, foreground.b, foreground.a];
        theme.level_bg = std::array::from_fn(|index| {
            let mixed = ui_styling::color::oklab_mix(base_linear, foreground_linear, index as f32 * shade_step);
            Rgba::new(mixed[0], mixed[1], mixed[2], mixed[3])
        });
        theme.navbar = theme.level_bg[1];
        theme.button = theme.level_bg[1];
        theme.panel = theme.level_bg[3];
        theme.temporary = theme.level_bg[5];
    }
    if let Some(outcome) = appearance.and_then(|groups| groups.get("outcome")) {
        theme.error = theme_paint_rgba(colors, outcome, "error", theme.error);
        theme.warning = theme_paint_rgba(colors, outcome, "warning", theme.warning);
        theme.success = theme_paint_rgba(colors, outcome, "success", theme.success);
        theme.progress = theme_paint_rgba(colors, outcome, "progress", theme.progress);
    }
    if let Some(diagram) = appearance.and_then(|groups| groups.get("diagram")) {
        theme.checker_light = theme_paint_rgba(colors, diagram, "checkerLight", theme.checker_light);
        theme.checker_dark = theme_paint_rgba(colors, diagram, "checkerDark", theme.checker_dark);
        theme.diagram_stroke = theme_paint_rgba(colors, diagram, "stroke", theme.diagram_stroke);
        theme.diagram_seam = theme_paint_rgba(colors, diagram, "seam", theme.diagram_seam);
        theme.diagram_accent = theme_paint_rgba(colors, diagram, "accent", theme.diagram_accent);
        theme.diagram_accent_fill = theme_paint_rgba(colors, diagram, "accentFill", theme.diagram_accent_fill);
        theme.diagram_shape_outline = theme_paint_rgba(colors, diagram, "shapeOutline", theme.diagram_shape_outline);
        theme.diagram_field_residual = theme_paint_rgba(colors, diagram, "fieldResidual", theme.diagram_field_residual);
        theme.diagram_field_reaction = theme_paint_rgba(colors, diagram, "fieldReaction", theme.diagram_field_reaction);
        theme.diagram_field_displacement = theme_paint_rgba(colors, diagram, "fieldDisplacement", theme.diagram_field_displacement);
    }
    theme.border_radius = document.radii.get("chrome").and_then(ThemeNumber::scalar).map_or(theme.border_radius, |value| value as f32);
    theme.stroke_hairline = document.strokes.get("chromeBorderHairline").and_then(ThemeNumber::scalar).map_or(theme.stroke_hairline, |value| value as f32);
    theme.stroke_focus = document.strokes.get("chromeBorderFocus").and_then(ThemeNumber::scalar).map_or(theme.stroke_focus, |value| value as f32);
    apply_theme_document_metrics(&mut theme, document);
    theme
}

/// 📐️ Resolves the shared named geometry bindings before any target scalar is cast to f32.
fn theme_document_geometry(document: &ThemeDocument) -> Option<BTreeMap<&'static str, f64>> {
    let canonical = shell_theme_document_base();
    let scalar = |section: &str, key: &str| document.metrics.get(section).and_then(|rows| rows.get(key)).or_else(|| canonical.metrics.get(section).and_then(|rows| rows.get(key))).and_then(|number| match number { ThemeNumber::Scalar(value) => Some(*value), ThemeNumber::List(_) => None });
    let compact = document.spacing.get("compact").or_else(|| canonical.spacing.get("compact"))?;
    let spacing_px = theme_geometry::resolve_spacing_px(compact, scalar("dom", "rootRemPx")?)?;
    let limit = theme_geometry::contract()["x-semio-resolution"]["maxMagnitudePx"].as_f64()?;
    let mut values = BTreeMap::new();
    for binding in theme_geometry::bindings()["bindings"].as_array()? {
        let value = scalar(binding["section"].as_str()?, binding["key"].as_str()?)?;
        if !value.is_finite() || value < 0.0 { return None; }
        let pixels = value * if binding["unit"].as_str()? == "spacing" { spacing_px } else { 1.0 };
        if !pixels.is_finite() || pixels > limit { return None; }
        values.insert(binding["themeField"].as_str()?, pixels);
    }
    Some(values)
}

/// 🎛️ Projects validated authored geometry into the typed renderer fields and retains direct duration units.
fn apply_theme_document_metrics(theme: &mut Theme, document: &ThemeDocument) {
    let Some(values) = theme_document_geometry(document) else { return };
    theme.navbar_height = values["navbar_height"] as f32;
    theme.footer_height = values["footer_height"] as f32;
    theme.panel_header_height = values["panel_header_height"] as f32;
    theme.control_height = values["control_height"] as f32;
    theme.control_height_small = values["control_height_small"] as f32;
    theme.gap_standard = values["gap_standard"] as f32;
    theme.padding_standard = values["padding_standard"] as f32;
    theme.panel_inset = values["panel_inset"] as f32;
    theme.tree_row_height = values["tree_row_height"] as f32;
    theme.tree_indent_per_level = values["tree_indent_per_level"] as f32;
    theme.tree_toggle_width = values["tree_toggle_width"] as f32;
    theme.panel_min_width = values["panel_min_width"] as f32;
    theme.panel_max_width = values["panel_max_width"] as f32;
    theme.window_measures_default_width = values["window_measures_default_width"] as f32;
    theme.window_engagement_max_width = values["window_engagement_max_width"] as f32;
    theme.font_size_body = values["font_size_body"] as f32;
    theme.font_size_small = values["font_size_small"] as f32;
    theme.font_size_emphasized = values["font_size_emphasized"] as f32;
    if let Some(value) = document.metrics.get("chrome").and_then(|rows| rows.get("celebrateBorderDurationSeconds")).and_then(ThemeNumber::scalar) {
        theme.celebrate_duration_seconds = value as f32;
    }
}
//#endregion 🎨️ThemeDocumentModel

//#region 🎨️ThemeEditorPanel
/// 🎨️ The eight editable sections React's `buildSettingsThemeTree` carries beside its selector
/// (`📌️ChromePanels/🟦️.tsx:820-827`), in React's own order, each `(section id suffix, label key)`.
const SHELL_THEME_EDITOR_SECTIONS: &[(&str, &str)] = &[
    ("colors", "settings.theme.colors"),
    ("spacing", "settings.theme.spacing"),
    ("fonts", "settings.theme.fonts"),
    ("strokes", "settings.theme.strokes"),
    ("radii", "settings.theme.radii"),
    ("opacities", "settings.theme.opacities"),
    ("metrics", "settings.theme.metrics"),
    ("appearances", "settings.theme.appearances"),
];

const SHELL_THEME_APPEARANCE_GROUPS: &[(&str, &str)] = &[
    ("board", "settings.theme.group.board"),
    ("map", "settings.theme.group.map"),
    ("canvas", "settings.theme.group.canvas"),
    ("chrome", "settings.theme.group.chrome"),
    ("outcome", "settings.theme.group.outcome"),
    ("diagram", "settings.theme.group.diagram"),
];
const SHELL_THEME_EDITOR_SCROLL_ID: &str = "tree:framework.settings.theme.select";
const SHELL_THEME_EDITOR_VIEWPORT_ROWS: usize = 28;
const SHELL_THEME_EDITOR_ROW_HEIGHT: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::dom::TREE_ROW_UI_SPACING) as f32;

fn theme_editor_input_item(id: &str, label: &str, value: &str, action: &str, args: Option<DslValue>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.to_string(),
        label: Label::data(label),
        control: Some(UiControlNode::Input(UiInputNode {
            id: id.to_string(),
            input_kind: "text".into(),
            value: value.to_string(),
            placeholder: None,
            accessibility_label: None,
            commit: Some("blur".into()),
            min: None,
            max: None,
            step: None,
            accept: None,
            precision: None,
            snaps: Vec::new(),
            on_change: ActionDescriptor { controller_id: "framework".into(), action: action.into(), args },
            on_submit: None,
            on_abort: None,
            on_repeat_last: None,
            presence: UiPresence::default(),
            menu: None,
            ..Default::default()
        })),
        ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
    }
}

fn theme_editor_control_item(id: &str, label: &str, control: UiControlNode) -> UiTreeItemNode {
    UiTreeItemNode { id: id.to_string(), label: Label::data(label), control: Some(control), ..UiTreeItemNode::base(String::new(), Label::data(String::new())) }
}

impl ShellState {
    /// 🎨️ The theme document this session is editing — the live draft, else the selected custom
    /// theme's own persisted document, else the authored base. React's
    /// `uiThemeBase = uiThemeDraft ?? activeUiTheme()`, verbatim.
    pub(crate) fn theme_document(&self) -> ThemeDocument {
        if let Some(draft) = self.theme_draft.as_ref() {
            return draft.clone();
        }
        if let Some(document) = self.chrome_build.preferences.custom_themes.get(&self.chrome_build.preferences.theme_id).and_then(|raw| ThemeDocument::parse(raw)) {
            return document;
        }
        shell_theme_document_base().clone()
    }

    /// 🎨️ The draft, seeded from the active document on first write — React's `draftThemePatch`,
    /// which `structuredClone`s `uiThemeBase` before every patch.
    fn theme_draft_patch(&mut self, patch: impl FnOnce(&mut ThemeDocument)) {
        let mut draft = self.theme_draft.take().unwrap_or_else(|| self.theme_document());
        patch(&mut draft);
        set_active_theme_document(Some(draft.clone()));
        self.theme_draft = Some(draft);
    }

    /// 🛡️ Stages a geometry edit and publishes only a fully resolvable document.
    fn theme_geometry_draft_patch(&mut self, patch: impl FnOnce(&mut ThemeDocument)) {
        let mut draft = self.theme_document();
        patch(&mut draft);
        if theme_document_geometry(&draft).is_some() {
            set_active_theme_document(Some(draft.clone()));
            self.theme_draft = Some(draft);
        }
    }

    /// 🎨️ Drops the draft and the present-side re-tokenisation with it — React's
    /// `dispatch({ type: "SET_UI_THEME_DRAFT", value: null })`.
    pub(crate) fn clear_theme_draft(&mut self) {
        self.theme_draft = None;
        set_active_theme_document(None);
    }

    fn theme_section_open(&self, id: &str, default_open: bool) -> bool {
        self.canonical_tree_open(id, default_open)
    }

    fn theme_item_open(&self, id: &str) -> bool {
        self.canonical_tree_open(id, false)
    }

    fn theme_visible_rows(&self) -> std::ops::Range<usize> {
        let offset = self.scroll_offsets.get(SHELL_THEME_EDITOR_SCROLL_ID).copied().unwrap_or_default().max(0.0);
        let start = (offset / SHELL_THEME_EDITOR_ROW_HEIGHT).floor() as usize;
        start.saturating_sub(2)..start.saturating_add(SHELL_THEME_EDITOR_VIEWPORT_ROWS)
    }

    fn theme_window_rows(&self, rows: Vec<UiTreeItemNode>, cursor: &mut usize, visible: &std::ops::Range<usize>) -> (Vec<UiTreeItemNode>, Option<UiTreeWindow>) {
        let total = rows.len();
        let first = visible.start.saturating_sub(*cursor).min(total);
        let last = visible.end.saturating_sub(*cursor).min(total).max(first);
        *cursor += total;
        let materialized: Vec<_> = rows.into_iter().skip(first).take(last - first).collect();
        let window = (materialized.len() != total).then_some(UiTreeWindow { row_extent: UiTreeWindowRowExtent::Standard, total: total as u32, offset: first as u32 });
        (materialized, window)
    }

    fn theme_leaf_section(&self, id: &str, label: &str, rows: Vec<UiTreeItemNode>, cursor: &mut usize, visible: &std::ops::Range<usize>) -> UiTreeSectionNode {
        *cursor += 1;
        let open = self.theme_section_open(id, false);
        let total = rows.len();
        let (items, window) = if open { self.theme_window_rows(rows, cursor, visible) } else { (Vec::new(), Some(UiTreeWindow { row_extent: UiTreeWindowRowExtent::Standard, total: total as u32, offset: 0 })) };
        UiTreeSectionNode { header_toolbar: None, id: id.to_string(), label: Some(Label::data(label)), default_open: Some(open), presence: UiPresence::default(), items, window }
    }

    pub(crate) fn build_settings_theme_editor_tree(&self, document: &ThemeDocument, is_de: bool) -> UiNode {
        let active_id = self.chrome_build.preferences.theme_id.clone();
        let mut themes = vec![UiSelectItem { value: "semio".into(), label: Label::data("Semio") }, UiSelectItem { value: "mono".into(), label: Label::data("Mono") }];
        for id in self.chrome_build.preferences.custom_themes.keys().cloned() {
            let label = custom_theme_document_from(&self.chrome_build.preferences, &id).map(|theme| theme.label).unwrap_or_else(|| id.clone());
            themes.push(UiSelectItem { value: id, label: Label::data(label) });
        }
        let mut selector = vec![theme_editor_control_item(
            "framework.settings.theme.select.picker",
            shell_chrome_string("settings.theme.select", is_de),
            UiControlNode::Select(UiSelectNode {
                presence: UiPresence::default(),
                id: "framework.settings.theme.select".into(),
                value: active_id.clone(),
                items: themes,
                placeholder: None,
                on_change: ActionDescriptor { controller_id: "framework".into(), action: "setThemeId".into(), args: None },
                menu: None,
            }),
        )];
        selector.push(theme_editor_input_item("framework.settings.theme.saveLabel", shell_chrome_string("common.name", is_de), &self.theme_save_label, "setThemeSaveLabel", None));
        let button = |id: &str, label: &'static str, icon_id: IconName, action: &str, args: Option<DslValue>, disabled: bool| {
            theme_editor_control_item(
                &format!("{id}.action"),
                shell_chrome_string(label, is_de),
                UiControlNode::Button(UiButtonNode {
                    id: Some(id.to_string()),
                    icon_id,
                    label: Label::data(shell_chrome_string(label, is_de)),
                    action: ActionDescriptor { controller_id: "framework".into(), action: action.into(), args },
                    style: None,
                    presence: UiPresence { state: if disabled { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
                    menu: None,
                }),
            )
        };
        selector.push(button("framework.settings.theme.save", "settings.theme.save", IconName::Save, "saveTheme", None, self.theme_save_label.trim().is_empty()));
        selector.push(button("framework.settings.theme.reset", "settings.theme.reset", IconName::RotateCcw, "resetThemeId", None, self.theme_draft.is_none() && active_id == "semio"));
        selector.push(button("framework.settings.theme.export", "settings.theme.export", IconName::Export, "exportTheme", None, false));
        selector.push(button("framework.settings.theme.import", "settings.theme.import", IconName::Import, "importTheme", None, false));
        if active_id.starts_with("custom.") {
            selector.push(button("framework.settings.theme.delete", "settings.theme.delete", IconName::Trash2, "deleteThemeId", crate::action_args_json!({ "value": active_id }), false));
        }
        let visible = self.theme_visible_rows();
        let select_open = self.theme_section_open("framework.settings.theme.select", true);
        let mut cursor = 1 + usize::from(select_open) * selector.len();
        let mut sections = vec![UiTreeSectionNode {
            header_toolbar: None,
            id: "framework.settings.theme.select".into(),
            label: Some(Label::data(shell_chrome_string("settings.theme.select", is_de))),
            default_open: Some(select_open),
            presence: UiPresence::default(),
            items: if select_open { selector } else { Vec::new() },
            window: None,
        }];
        let leaf_rows = |suffix: &str| -> Vec<UiTreeItemNode> {
            let id = format!("framework.settings.theme.{suffix}");
            match suffix {
                "colors" => document.colors.iter().map(|(key, value)| theme_editor_input_item(&format!("{id}.{key}"), key, value, "setThemeColor", crate::action_args_json!({ "key": key }))).collect(),
                "spacing" => document.spacing.iter().map(|(key, value)| {
                    let mut item = theme_editor_input_item(&format!("{id}.{key}"), key, value, "setThemeSpacing", crate::action_args_json!({ "key": key }));
                    if key == "compact" {
                        let ui = &theme_geometry::contract()["properties"]["compact"]["x-semio-ui"];
                        let locale = if is_de { "de" } else { "en" };
                        let label = ui["label"][locale].as_str().expect("localized compact label");
                        item.label = Label::data(label);
                        item.description = Some(ui["description"][locale].as_str().expect("localized compact description").into());
                        if let Some(UiControlNode::Input(input)) = item.control.as_mut() { input.accessibility_label = Some(Label::data(label)); }
                    }
                    item
                }).collect(),
                "fonts" => document.font_stacks.iter().map(|(key, value)| theme_editor_input_item(&format!("{id}.{key}"), key, value, "setThemeFontStack", crate::action_args_json!({ "key": key }))).collect(),
                "strokes" => document.strokes.iter().map(|(key, value)| theme_editor_input_item(&format!("{id}.{key}"), key, &value.as_text(), "setThemeStroke", crate::action_args_json!({ "key": key }))).collect(),
                "radii" => document.radii.iter().map(|(key, value)| theme_editor_input_item(&format!("{id}.{key}"), key, &value.as_text(), "setThemeRadius", crate::action_args_json!({ "key": key }))).collect(),
                "opacities" => document.opacities.iter().map(|(key, value)| theme_editor_input_item(&format!("{id}.{key}"), key, &value.as_text(), "setThemeOpacity", crate::action_args_json!({ "key": key }))).collect(),
                _ => Vec::new(),
            }
        };
        for (suffix, label_key) in SHELL_THEME_EDITOR_SECTIONS.iter().take(6) {
            let id = format!("framework.settings.theme.{suffix}");
            sections.push(self.theme_leaf_section(&id, shell_chrome_string(*label_key, is_de), leaf_rows(suffix), &mut cursor, &visible));
        }
        sections.push(self.theme_metrics_section(document, is_de, &mut cursor, &visible));
        sections.push(self.theme_appearances_section(document, is_de, &mut cursor, &visible));
        UiNode::Tree(UiTreeNode { presentation: Default::default(), sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
    }

    fn theme_metrics_section(&self, document: &ThemeDocument, is_de: bool, cursor: &mut usize, visible: &std::ops::Range<usize>) -> UiTreeSectionNode {
        let id = "framework.settings.theme.metrics";
        *cursor += 1;
        let open = self.theme_section_open(id, false);
        let mut items = Vec::new();
        if open {
            for (section, entries) in &document.metrics {
                let section_id = format!("{id}.{section}");
                *cursor += 1;
                let group_open = self.theme_item_open(&section_id);
                let rows = entries.iter().map(|(key, value)| {
                    let mut item = theme_editor_input_item(&format!("{section_id}.{key}"), key, &value.as_text(), "setThemeMetric", crate::action_args_json!({ "section": section, "key": key }));
                    let guidance = if section == "dom" && key == "rootRemPx" { Some("positiveRoot") }
                        else if theme_geometry::bindings()["bindings"].as_array().is_some_and(|bindings| bindings.iter().any(|binding| binding["section"].as_str() == Some(section.as_str()) && binding["key"].as_str() == Some(key.as_str()))) { Some("nonnegativeScalar") }
                        else { None };
                    if let Some(guidance) = guidance {
                        item.description = theme_geometry::contract()["x-semio-resolution"]["metricUi"][guidance][if is_de { "de" } else { "en" }].as_str().map(str::to_string);
                        if let Some(UiControlNode::Input(input)) = item.control.as_mut() { input.accessibility_label = Some(Label::data(key)); }
                    }
                    item
                }).collect();
                let total = entries.len();
                let (children, window) = if group_open { self.theme_window_rows(rows, cursor, visible) } else { (Vec::new(), Some(UiTreeWindow { row_extent: UiTreeWindowRowExtent::Standard, total: total as u32, offset: 0 })) };
                items.push(UiTreeItemNode { id: section_id, label: Label::data(section), default_open: Some(group_open), items: Some(children), window, ..UiTreeItemNode::base(String::new(), Label::data(String::new())) });
            }
        }
        UiTreeSectionNode { header_toolbar: None, id: id.into(), label: Some(Label::data(shell_chrome_string("settings.theme.metrics", is_de))), default_open: Some(open), presence: UiPresence::default(), items, window: None }
    }

    fn theme_appearances_section(&self, document: &ThemeDocument, is_de: bool, cursor: &mut usize, visible: &std::ops::Range<usize>) -> UiTreeSectionNode {
        let id = "framework.settings.theme.appearances";
        *cursor += 1;
        let open = self.theme_section_open(id, false);
        let mut appearances = Vec::new();
        if open {
            for appearance in ["light", "dark"] {
                let appearance_id = format!("{id}.{appearance}");
                *cursor += 1;
                let appearance_open = self.theme_item_open(&appearance_id);
                let mut groups = Vec::new();
                if appearance_open {
                    for (group, label_key) in SHELL_THEME_APPEARANCE_GROUPS {
                        let group_id = format!("{appearance_id}.{group}");
                        *cursor += 1;
                        let group_open = self.theme_item_open(&group_id);
                        let mut rows = Vec::new();
                        for (paint, reference) in document.appearances.get(appearance).and_then(|entries| entries.get(*group)).into_iter().flatten() {
                            let (hex, alpha) = theme_paint_editor_values(&document.colors, reference);
                            rows.push(theme_editor_input_item(&format!("{group_id}.{paint}"), paint, &hex, "setThemeAppearancePaint", crate::action_args_json!({ "appearance": appearance, "group": group, "paint": paint, "channel": "hex" })));
                            rows.push(theme_editor_input_item(
                                &format!("{group_id}.{paint}.alpha"),
                                paint,
                                &format!("{alpha:.2}"),
                                "setThemeAppearancePaint",
                                crate::action_args_json!({ "appearance": appearance, "group": group, "paint": paint, "channel": "alpha" }),
                            ));
                        }
                        let total = rows.len();
                        let (children, window) = if group_open { self.theme_window_rows(rows, cursor, visible) } else { (Vec::new(), Some(UiTreeWindow { row_extent: UiTreeWindowRowExtent::Standard, total: total as u32, offset: 0 })) };
                        groups.push(UiTreeItemNode {
                            id: group_id,
                            label: Label::data(shell_chrome_string(*label_key, is_de)),
                            default_open: Some(group_open),
                            items: Some(children),
                            window,
                            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
                        });
                    }
                }
                let label = shell_chrome_string(if appearance == "light" { "settings.theme.appearance.light" } else { "settings.theme.appearance.dark" }, is_de);
                appearances.push(UiTreeItemNode { id: appearance_id, label: Label::data(label), default_open: Some(appearance_open), items: Some(groups), ..UiTreeItemNode::base(String::new(), Label::data(String::new())) });
            }
        }
        UiTreeSectionNode { header_toolbar: None, id: id.into(), label: Some(Label::data(shell_chrome_string("settings.theme.appearances", is_de))), default_open: Some(open), presence: UiPresence::default(), items: appearances, window: None }
    }
}

/// 🖌️ What React's appearance-paint row shows: the resolved `#rrggbb` from `rgba8ToHex` and the
/// resolved alpha as `rgba[3] / 255` (`buildThemeAppearanceGroupItems`), so the boxes read the same
/// on both renderers even for a `mix`/`token` ref that carries neither literally.
fn theme_paint_editor_values(colors: &BTreeMap<String, String>, reference: &ThemePaintRef) -> (String, f64) {
    let [r, g, b, a] = theme_resolve_paint(colors, reference).unwrap_or([0, 0, 0, 255]);
    (format!("#{r:02x}{g:02x}{b:02x}"), f64::from(a) / 255.0)
}
//#endregion 🎨️ThemeEditorPanel

//#region 🎨️ThemeDocumentDoors
/// 🎨️ Publishes (or clears) the DRAFT document the present side re-tokenises from. The draft lives
/// beside `theme_id` in the same owned preference snapshot `frame()`'s `resolve_theme_for_ids` reads,
/// which is what makes an edit in the Settings panel repaint the chrome on the very next frame —
/// React's `applyUiThemeToRoot(uiThemeDraft)`.
pub(crate) fn set_active_theme_document(document: Option<ThemeDocument>) {
    with_chrome_prefs(|prefs| {
        prefs.theme_document_draft = document;
        prefs.theme_document_generation = prefs.theme_document_generation.wrapping_add(1);
        prefs.theme_document_resolved = None;
    });
}

/// 🎨️ Commits one document into the custom-theme registry under `id` and activates it — the present
/// side of React's `saveTheme`, which writes `canonicalUiTheme(saved)` into
/// `customThemes[id]` and then selects it. The stored text is the SAME canonical JSON React stores,
/// so a theme saved on either renderer opens on the other.
pub(crate) fn save_custom_theme_document(id: &str, document: &ThemeDocument) -> Option<String> {
    let raw = document.canonical_json()?;
    with_chrome_prefs(|prefs| {
        prefs.custom_themes.insert(id.to_string(), raw.clone());
        prefs.theme_id = id.to_string();
        prefs.theme_document_draft = None;
        prefs.theme_document_generation = prefs.theme_document_generation.wrapping_add(1);
        prefs.theme_document_resolved = None;
    });
    Some(raw)
}

/// 🎨️ The resolved theme for a canonical document-backed selection or live draft.
///
/// 🧊️ Cached on `(theme id, appearance, draft generation)`: `resolve_theme_for_ids` runs once per
/// frame per appearance, and re-resolving ~450 document entries there would put a JSON walk in the
/// frame loop.
pub(crate) fn document_theme_for_ids(theme_id: &str, dark: bool) -> Option<Theme> {
    with_chrome_prefs(|prefs| {
        let key = (theme_id.to_string(), dark, prefs.theme_document_generation);
        if let Some((cached, theme)) = prefs.theme_document_resolved.as_ref() {
            if cached == &key {
                return Some(*theme);
            }
        }
        let document = match prefs.theme_document_draft.clone() {
            Some(draft) => draft,
            None => custom_theme_document_from(prefs, theme_id)?,
        };
        let theme = theme_from_document(&document, dark);
        prefs.theme_document_resolved = Some((key, theme));
        Some(theme)
    })
}

/// 🆔️ React's `saveTheme` slug: lowercase, non-alphanumerics collapsed to `-`, trimmed, under
/// `custom.`.
pub(crate) fn custom_theme_id_for_label(label: &str) -> Option<String> {
    let mut slug = String::new();
    let mut pending_dash = false;
    for character in label.trim().to_lowercase().chars() {
        if character.is_ascii_alphanumeric() {
            if pending_dash && !slug.is_empty() {
                slug.push('-');
            }
            pending_dash = false;
            slug.push(character);
        } else {
            pending_dash = true;
        }
    }
    (!slug.is_empty()).then(|| format!("custom.{slug}"))
}

/// 🔀️ Merges the value a retained control committed INTO the args its node authored, instead of
/// replacing them.
///
/// 🩸️ Every retained commit on this target used to dispatch `{ value }` alone, so an authored arg was
/// silently dropped: the Default Apps leaf authored `{ row: "<dialect>/<role>" }` on its `Select` and
/// its own handler read `args.row`, which therefore never arrived, and the whole theme editor (one
/// verb, ~450 rows, the key in the args) would have been unable to say WHICH key it wrote. React's
/// handlers are closures that captured those coordinates, which is what this restores. Keys stay
/// ascending so the projected argument object is stable.
pub(crate) fn merge_committed_args(authored: Option<&DslValue>, value: DslValue) -> Option<DslValue> {
    let mut fields: Vec<(String, DslValue)> = match authored {
        Some(semio_framework_value::DslValue::Object(existing)) => existing.iter().filter(|(key, _)| key != "value").cloned().collect(),
        _ => Vec::new(),
    };
    fields.push(("value".to_string(), value));
    fields.sort_by(|left, right| left.0.cmp(&right.0));
    Some(semio_framework_value::DslValue::Object(fields))
}
//#endregion 🎨️ThemeDocumentDoors

//#region ⚖️MergePolicyAndThemeCommits
/// ⚖️ React's `MERGE_POLICY_OPTIONS` (`📌️ChromePanels/🟦️.tsx:339`) with each option's label key —
/// the variant names `protocol::MergePolicy` serializes to, so the value a user picks here is the
/// value an authority reads.
pub(crate) const SHELL_MERGE_POLICY_OPTIONS: &[(&str, &str)] = &[("LaissezFaire", "settings.mergePolicy.laissezFaire"), ("Normal", "settings.mergePolicy.normal"), ("Vigilant", "settings.mergePolicy.vigilant")];

/// ⚖️ `protocol::MergePolicy::default()` — "the least-surprising choice for a fresh authority that
/// has never been configured", and React's `DEFAULT_MERGE_POLICY`.
pub(crate) const SHELL_DEFAULT_MERGE_POLICY: &str = "Normal";

/// ⬆️ React's `requestFileOpen(".theme.dsl,.dsl,text/plain")` accept list, verbatim.
pub(crate) const SHELL_THEME_IMPORT_ACCEPT: &str = ".theme.dsl,.dsl,text/plain";

impl ShellState {
    /// 🎨️ One theme-editor commit — the eight `draftThemePatch` mutators of React's `ShellHost`
    /// (`🏛️ShellHost/🟦️.tsx:8248-8301`), dispatched by verb over the row's own coordinates. A value
    /// that does not parse commits NOTHING, exactly as React's `onBlur` returns without calling
    /// `onCommit`.
    ///
    /// 🖌️ React edits a paint through ONE call carrying both channels: the colour input sends
    /// the new hex with the row's current alpha, the alpha box sends the row's current hex with
    /// the new alpha. Either way the ref becomes a literal `{ hex, alpha }` — a token/mix ref
    /// the user has overridden is no longer that token, exactly as React's
    /// `next.appearances[a][g][k] = { hex, alpha }` replaces it outright.
    pub(crate) fn apply_theme_editor_commit(&mut self, verb: &str, args: &Value) {
        let text = args.get("value").and_then(Value::as_str).unwrap_or_default().to_string();
        let key = args.get("key").and_then(Value::as_str).unwrap_or_default().to_string();
        match verb {
            "setThemeColor" => self.theme_draft_patch(move |document| {
                document.colors.insert(key, text);
            }),
            "setThemeSpacing" => self.theme_geometry_draft_patch(move |document| {
                document.spacing.insert(key, text);
            }),
            "setThemeFontStack" => self.theme_draft_patch(move |document| {
                document.font_stacks.insert(key, text);
            }),
            "setThemeStroke" => {
                if let Some(number) = ThemeNumber::parse(&text) {
                    self.theme_draft_patch(move |document| {
                        document.strokes.insert(key, number);
                    });
                }
            }
            "setThemeRadius" => {
                if let Some(number) = ThemeNumber::parse(&text) {
                    self.theme_draft_patch(move |document| {
                        document.radii.insert(key, number);
                    });
                }
            }
            "setThemeOpacity" => {
                if let Some(number) = ThemeNumber::parse(&text) {
                    self.theme_draft_patch(move |document| {
                        document.opacities.insert(key, number);
                    });
                }
            }
            "setThemeMetric" => {
                let section = args.get("section").and_then(Value::as_str).unwrap_or_default().to_string();
                if let Some(number) = ThemeNumber::parse(&text) {
                    self.theme_geometry_draft_patch(move |document| {
                        document.metrics.entry(section).or_default().insert(key, number);
                    });
                }
            }
            "setThemeAppearancePaint" => {
                let appearance = args.get("appearance").and_then(Value::as_str).unwrap_or_default().to_string();
                let group = args.get("group").and_then(Value::as_str).unwrap_or_default().to_string();
                let paint = args.get("paint").and_then(Value::as_str).unwrap_or_default().to_string();
                let channel = args.get("channel").and_then(Value::as_str).unwrap_or("hex").to_string();
                let document = self.theme_document();
                let current = document.appearances.get(&appearance).and_then(|groups| groups.get(&group)).and_then(|paints| paints.get(&paint)).cloned().unwrap_or_default();
                let (current_hex, current_alpha) = theme_paint_editor_values(&document.colors, &current);
                let reference = if channel == "alpha" {
                    let Some(alpha) = text.trim().parse::<f64>().ok() else { return };
                    ThemePaintRef { hex: Some(current_hex), alpha: Some(alpha.clamp(0.0, 1.0)), ..ThemePaintRef::default() }
                } else {
                    ThemePaintRef { hex: Some(text), alpha: Some(current_alpha), ..ThemePaintRef::default() }
                };
                self.theme_draft_patch(move |document| {
                    document.appearances.entry(appearance).or_default().entry(group).or_default().insert(paint, reference);
                });
            }
            _ => {}
        }
    }

    /// ⬆️ The parked theme-import picker, opened inside the gesture that asked for it — the same
    /// user-activation rule `drain_gesture_bound_work` already honours for `pending_file_opens`.
    #[cfg(target_arch = "wasm32")]
    pub(crate) async fn run_theme_import_request(&mut self) {
        let opened = request_file_open(SHELL_THEME_IMPORT_ACCEPT, Some("text"), false).await;
        let Some(file) = opened.into_iter().next() else { return };
        let action = ActionDescriptor { controller_id: "framework".into(), action: "applyImportedTheme".into(), args: crate::action_args_json!({ "value": file.contents }) };
        if let Err(error) = self.dispatch_action(action).await {
            Self::debug_log(&format!("[TRACE] wgpu-shell theme import failed: {error}"));
        }
    }
}
//#endregion ⚖️MergePolicyAndThemeCommits

//#region ♿️ChromeAccessibleNames
/// ♿️ The accessible NAME each chrome control painted this walk, keyed by control id.
///
/// 🩸️ A GPU canvas has no elements, so the only accessible names this target ever published came out
/// of the retained DOCUMENT projection (`dumpAccessibility` over `UI_ENGINE`) — the scene trees. Every
/// navbar chip, footer pill, panel tab, pane chip and surface control was invisible to a reader
/// (audit W14 §C4), even though React gives each of them an `sr-only` label for free. The two central
/// chrome item painters record the label they are ABOUT to paint here, so the published name is
/// always the text on screen rather than a second, drifting table.
///
/// ⛓️ Staged per walk and promoted by `publish_retained_hit_registry` together with the hit registry,
/// so names and targets can never describe different frames — the same double-buffer discipline
/// `retained_hit_windows` follows. Bounded by [`SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY`].
const SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY: usize = 512;

/// ♿️ Semantic properties captured beside the chrome label during its paint walk.
struct ChromeControlPresentation {
    pub(crate) label: String,
    pub(crate) description: Option<String>,
    pub(crate) role: Option<&'static str>,
    pub(crate) pressed: Option<bool>,
    pub(crate) selected: Option<bool>,
    pub(crate) controls: Option<String>,
    pub(crate) value_text: Option<String>,
    pub(crate) value_range: Option<[f64; 3]>,
    pub(crate) disabled: bool,
}

/// 🪡️ Per thread on wasm and in tests — each test owns its shell, so a concurrent test's frame setup can never
/// clear the names another test is about to publish — and one worker-wide cell natively.
#[cfg(any(target_arch = "wasm32", test))]
thread_local! {
    static CHROME_CONTROL_NAMES: std::cell::RefCell<BTreeMap<String, ChromeControlPresentation>> = std::cell::RefCell::new(BTreeMap::new());
}

#[cfg(not(any(target_arch = "wasm32", test)))]
static CHROME_CONTROL_NAMES: crate::element_contracts::WorkerCell<BTreeMap<String, ChromeControlPresentation>> = crate::element_contracts::WorkerCell::new(Default::default);

pub(crate) fn with_chrome_control_names<R>(f: impl FnOnce(&mut BTreeMap<String, ChromeControlPresentation>) -> R) -> R {
    #[cfg(any(target_arch = "wasm32", test))]
    {
        CHROME_CONTROL_NAMES.with(|cell| f(&mut cell.borrow_mut()))
    }
    #[cfg(not(any(target_arch = "wasm32", test)))]
    {
        f(&mut CHROME_CONTROL_NAMES.borrow_mut())
    }
}

/// ♿️ Records one chrome control's painted label. Called from the two painters every chrome chip goes
/// through, so a new chip is announced without its author doing anything.
pub(crate) fn note_chrome_control_name(control_id: &str, label: Option<&str>) {
    let (Some(label), false) = (label, control_id.is_empty()) else { return };
    with_chrome_control_names(|names| {
        if names.len() < SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY || names.contains_key(control_id) {
            names.insert(control_id.to_string(), ChromeControlPresentation { label: label.to_string(), description: None, role: None, pressed: None, selected: None, controls: None, value_text: None, value_range: None, disabled: false });
        }
    });
}

/// ♿️ Records the consequence text a reader hears with one chrome control (`aria-describedby`).
pub(crate) fn note_chrome_control_description(control_id: &str, description: Option<&str>) {
    with_chrome_control_names(|names| {
        if let Some(presentation) = names.get_mut(control_id) {
            presentation.description = description.map(str::to_string);
        }
    });
}

/// 🎛️ Records a dialog control's role, pressed state and value (a slider's range and spoken value, a spin
/// button's text) beside its painted name.
pub(crate) fn note_chrome_control_semantics(control_id: &str, semantics: &ChromeDialogSemantics) {
    with_chrome_control_names(|names| {
        if let Some(presentation) = names.get_mut(control_id) {
            presentation.role = semantics.role.or(presentation.role);
            presentation.pressed = semantics.pressed.or(presentation.pressed);
            presentation.value_text.clone_from(&semantics.value_text);
            presentation.value_range = semantics.value_range;
        }
    });
}

/// 🚫️ Records that a painted chrome control is disabled (`aria-disabled`) — a band control a session refusal blocks.
pub(crate) fn note_chrome_control_disabled(control_id: &str, disabled: bool) {
    with_chrome_control_names(|names| {
        if let Some(presentation) = names.get_mut(control_id) {
            presentation.disabled = disabled;
        }
    });
}

/// 🔘️ Captures the painted pressed state of chrome buttons beside their accepted-frame label.
pub(crate) fn note_chrome_group_item(item: &ChromeGroupItem<'_>) {
    note_chrome_control_name(item.control_id, item.label);
    if item.kind == HitKind::PanelTab || item.control_id == "ui.fullscreen.toggle" || item.kind == HitKind::Toggle && item.control_id.starts_with("framework.utility.toggle.") {
        with_chrome_control_names(|names| {
            if let Some(presentation) = names.get_mut(item.control_id) {
                presentation.role = Some("button");
                presentation.pressed = Some(item.active);
            }
        });
    }
}

/// 🗣️ Localizes Dock actions while their title and pointer target belong to the same painted frame.
pub(crate) fn note_dock_control_name(control_id: &str, name: crate::dock::DockControlName<'_>, german: bool) {
    use crate::dock::DockControlName;
    let label = match name {
        DockControlName::Body(title) | DockControlName::Tab { title, .. } => title.to_string(),
        DockControlName::Focus => shell_chrome_string("common.focus", german).to_string(),
        DockControlName::Unfocus => shell_chrome_string("common.unfocus", german).to_string(),
        DockControlName::Close => shell_chrome_string("common.close", german).to_string(),
        DockControlName::Drag(title) => shell_chrome_string("tree.drag.sortTarget", german).replace("{{target}}", title),
    };
    note_chrome_control_name(control_id, Some(&label));
    with_chrome_control_names(|names| {
        if let Some(presentation) = names.get_mut(control_id) {
            match name {
                DockControlName::Body(_) => presentation.role = Some("tabpanel"),
                DockControlName::Tab { selected, panel, .. } => {
                    presentation.role = Some("tab");
                    presentation.selected = Some(selected);
                    presentation.controls = Some(panel.to_string());
                }
                _ => {}
            }
        }
    });
}

/// ♿️ The ARIA role a chrome hit announces — React's own element choice per control kind.
fn chrome_accessibility_role(kind: &HitKind) -> &'static str {
    match kind {
        HitKind::Toggle => "switch",
        HitKind::Select => "combobox",
        HitKind::DropdownItem => "option",
        HitKind::PanelTab => "button",
        HitKind::Slider => "slider",
        HitKind::Input => "textbox",
        HitKind::ContextMenu => "menuitem",
        _ => "button",
    }
}

impl ShellState {
    /// ⌨️ One chrome control's visible label WITH its inline hotkey badge — React's `ButtonGroupItem`
    /// paints a `ControlHotkeyBadge` beside the label of ANY bound control while the driver's
    /// `hotkeys` axis is `"inline"` (the default driver's), not only beside the two role chips
    /// (audit W14 §B14). Idempotent: a builder that already appended the badge (the role chips do,
    /// for their own measured layout) is left alone rather than double-badged.
    pub(crate) fn chrome_control_label(&self, control_id: &str, label: &str) -> String {
        match shell_control_hotkey_badge(&self.shortcut_table(), control_id) {
            Some(badge) if !label.ends_with(&badge) => format!("{label} {badge}"),
            _ => label.to_string(),
        }
    }

    fn palette_accessibility_nodes(&self, hits: &[HitTarget<ActionDescriptor>]) -> Option<Vec<ui_contract::AccessibilityProjectionNode>> {
        let kind = match self.overlay_state {
            OverlayState::Search => ShellPaletteKind::Search,
            OverlayState::Find => ShellPaletteKind::Find,
            _ => return None,
        };
        let is_de = self.locale_id == "de";
        let title = shell_chrome_string(if kind == ShellPaletteKind::Search { "search.title" } else { "find.title" }, is_de).to_string();
        let description = shell_chrome_string(if kind == ShellPaletteKind::Search { "search.description" } else { "find.description" }, is_de).to_string();
        let empty = shell_chrome_string(if kind == ShellPaletteKind::Search { "search.empty" } else { "find.empty" }, is_de).to_string();
        let query = self.palette_query(kind).to_string();
        let selected_index = match kind {
            ShellPaletteKind::Search => self.search_selected,
            ShellPaletteKind::Find => self.find_selected,
        };
        let rows: Vec<(usize, String, String, Option<String>, String)> = match kind {
            ShellPaletteKind::Search => self.filtered_search_items().into_iter().enumerate().map(|(index, item)| (index, item.id, item.label, item.description, item.group)).collect(),
            ShellPaletteKind::Find => self.filtered_find_items().into_iter().enumerate().map(|(index, item)| (index, item.id, item.label, item.description, item.category.unwrap_or_default())).collect(),
        };
        let rect_for = |id: &str| hits.iter().find(|hit| hit.control_id.as_deref() == Some(id)).map(|hit| hit.rect);
        let dialog_rect = rect_for(kind.dialog_id());
        let input_rect = rect_for(kind.input_id());
        let list_rect = dialog_rect.zip(input_rect).map(|(dialog, input)| Rect::new(dialog.x, input.y + input.h, dialog.w, (dialog.y + dialog.h - input.y - input.h).max(0.0)));
        let basic = |key: String, role: &str, depth: usize, label: Option<String>, description: Option<String>, rect: Option<Rect>| ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: 0,
            key,
            role: role.to_string(),
            depth,
            label,
            description,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).to_string(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: false,
            tabbable: false,
            actionable: false,
            focused: false,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: rect.map(|rect| [rect.x, rect.y, rect.w, rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false,
        };
        let mut nodes = Vec::with_capacity(SHELL_PALETTE_PLAN_ENTRY_CAPACITY + 4);
        nodes.push(basic(kind.dialog_id().to_string(), "dialog", 0, Some(title.clone()), Some(description.clone()), dialog_rect));
        let mut close = basic(kind.close_id().to_string(), "button", 1, Some(shell_chrome_string("common.close", is_de).to_string()), None, rect_for(kind.close_id()));
        close.focusable = true;
        close.tabbable = true;
        close.actionable = true;
        nodes.push(close);
        let mut input = basic(kind.input_id().to_string(), "combobox", 1, Some(title.clone()), Some(description), input_rect);
        input.shortcut = shell_control_hotkey_badge(&self.shortcut_table(), kind.input_id());
        input.focusable = true;
        input.tabbable = true;
        input.actionable = true;
        input.focused = self.accessibility_focused_control_id.as_deref() == Some(kind.input_id());
        input.expanded = Some(true);
        input.editable = true;
        input.controls = Some(kind.list_id().to_string());
        input.active_descendant = rows.get(selected_index).map(|(_, id, _, _, _)| id.clone());
        input.value_text = Some(query);
        nodes.push(input);
        nodes.push(basic(kind.list_id().to_string(), "listbox", 1, Some(title), None, list_rect));
        if rows.is_empty() {
            let mut status = basic(format!("{}.empty", kind.dialog_id()), "status", 2, Some(empty), None, list_rect);
            status.live = ui_contract::liveness_name(ui_contract::Liveness::Polite).to_string();
            nodes.push(status);
        } else {
            let mut groups = Vec::<String>::new();
            for (_, _, _, _, group) in &rows {
                if !groups.iter().any(|existing| existing == group) {
                    groups.push(group.clone());
                }
            }
            for (group_index, group) in groups.into_iter().enumerate() {
                nodes.push(basic(format!("{}.group.{group_index}", kind.dialog_id()), "group", 2, (!group.is_empty()).then_some(group.clone()), None, None));
                for (index, id, label, row_description, _) in rows.iter().filter(|(_, _, _, _, row_group)| row_group == &group) {
                    let mut option = basic(id.clone(), "option", 3, Some(label.clone()), row_description.clone(), rect_for(&kind.control_id(*index)));
                    option.actionable = true;
                    option.selected = Some(*index == selected_index);
                    nodes.push(option);
                }
            }
        }
        for (index, node) in nodes.iter_mut().enumerate() {
            node.node_id = index as u64 + 1;
        }
        Some(nodes)
    }

    /// 🌴️ The open declared dialog as a modal accessibility tree — the twin of React's `UIDialog` (`role="dialog"`
    /// named by its title and described by its body): the dialog node, then every control it painted this frame one
    /// level below it in paint order, named, described, valued and pressed from the painted registry, a gated submit
    /// or choice `disabled`, each focus stop tabbable, and the dialog's own keyboard focus as the focused node. While
    /// it is open nothing behind the veil is published, as React's `aria-modal` makes the page inert.
    fn dialog_accessibility_nodes(&self, hits: &[HitTarget<ActionDescriptor>]) -> Option<Vec<ui_contract::AccessibilityProjectionNode>> {
        let request = self.chrome_build.dialog_stack.last()?;
        let key = request.accessibility_key();
        let prefix = format!("{key}.");
        let stops: Vec<String> = request.stops().into_iter().map(|stop| request.control_id(stop)).collect();
        let focused = request.control_id(request.focused());
        let controls: Vec<(&str, &HitTarget<ActionDescriptor>)> = hits.iter().filter_map(|hit| hit.control_id.as_deref().filter(|id| id.starts_with(&prefix)).map(|id| (id, hit))).collect();
        let bounds = controls.iter().map(|(_, hit)| hit.rect).reduce(|lhs, rhs| {
            let (x, y) = (lhs.x.min(rhs.x), lhs.y.min(rhs.y));
            Rect::new(x, y, (lhs.x + lhs.w).max(rhs.x + rhs.w) - x, (lhs.y + lhs.h).max(rhs.y + rhs.h) - y)
        });
        let node = |node_id: u64, key: &str, role: &str, depth: usize, label: String| {
            let mut node = chrome_status_accessibility_node(node_id, key, label);
            node.role = role.to_string();
            node.depth = depth;
            node.live = ui_contract::liveness_name(ui_contract::Liveness::Off).to_string();
            node
        };
        let mut dialog = node(1, &key, "dialog", 0, request.title.clone());
        dialog.description = (!request.body.is_empty()).then(|| request.body.clone());
        dialog.rect = bounds.map(|rect| [rect.x, rect.y, rect.w, rect.h]);
        let mut nodes = vec![dialog];
        with_chrome_control_names(|names| {
            for (id, hit) in controls.into_iter().take(SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY) {
                let presentation = names.get(id);
                let pressed = presentation.and_then(|name| name.pressed);
                let role = presentation.and_then(|name| name.role).unwrap_or_else(|| if pressed.is_some() { "button" } else { chrome_accessibility_role(&hit.kind) });
                let disabled = presentation.is_some_and(|name| name.disabled);
                let draft = (hit.kind == HitKind::Input).then(|| (0..request.fields.len()).find(|index| request.control_id(ChromeDialogStop::Field(*index)) == id).map(|index| request.fields[index].draft.clone())).flatten();
                let mut control = node(nodes.len() as u64 + 1, id, role, 1, presentation.map_or_else(|| humanize_control_id(id), |name| name.label.clone()));
                control.description = presentation.and_then(|name| name.description.clone());
                control.disabled = disabled;
                control.focusable = true;
                control.tabbable = stops.iter().any(|stop| stop == id);
                control.actionable = !disabled;
                control.focused = focused == id;
                control.pressed = pressed;
                control.editable = matches!(role, "textbox" | "spinbutton");
                control.rect = Some([hit.rect.x, hit.rect.y, hit.rect.w, hit.rect.h]);
                control.value_min = presentation.and_then(|name| name.value_range).map(|range| range[0]);
                control.value_max = presentation.and_then(|name| name.value_range).map(|range| range[1]);
                control.value_now = presentation.and_then(|name| name.value_range).map(|range| range[2]);
                control.value_text = presentation.and_then(|name| name.value_text.clone()).or(draft);
                nodes.push(control);
            }
        });
        Some(nodes)
    }

    /// ♿️ This frame's chrome controls as accessibility projection nodes — the SAME wire shape the
    /// retained document publishes, so `🚀️browser-boot/🟦️.ts`'s mirror gives them elements with no
    /// new code: `aria-label` from the painted text, `aria-keyshortcuts` from this session's own
    /// remappable chord table (React's `aria-keyshortcuts` twin, audit W14 §B14), `aria-disabled`
    /// from the registry.
    pub(crate) fn chrome_accessibility_nodes(&self, hits: &[HitTarget<ActionDescriptor>]) -> Vec<ui_contract::AccessibilityProjectionNode> {
        if let Some(mut nodes) = self.dialog_accessibility_nodes(hits).or_else(|| self.palette_accessibility_nodes(hits)) {
            if let Some(notice) = self.transient_notice_accessibility_node(nodes.iter().map(|node| node.node_id).max().unwrap_or(0) + 1) {
                nodes.push(notice);
            }
            for mut band in self.band_accessibility_nodes(nodes.iter().map(|node| node.node_id).max().unwrap_or(0) + 1) {
                band.focusable = false;
                band.focused = false;
                nodes.push(band);
            }
            return nodes;
        }
        let shortcuts = self.shortcut_table();
        with_chrome_control_names(|names| {
            let mut nodes = hits
                .iter()
                .filter_map(|hit| hit.control_id.as_deref().filter(|id| !id.is_empty()).map(|id| (id, hit)))
                .filter(|(id, _)| !self.retained_hit_windows.contains_key(*id))
                .filter(|(id, hit)| match hit.kind {
                    HitKind::PanelResize | HitKind::DockSplit | HitKind::DockJoinCorner => false,
                    HitKind::ScrollRegion => names.get(*id).is_some_and(|name| name.role == Some("tabpanel")),
                    HitKind::Generic => hit.event.is_some() || names.contains_key(*id),
                    _ => true,
                })
                .take(SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY)
                .enumerate()
                .map(|(index, (id, hit))| {
                    let presentation = names.get(id);
                    let pressed = presentation.and_then(|name| name.pressed).or_else(|| self.chrome_accessibility_pressed(id, &hit.kind));
                    let role = presentation.and_then(|name| name.role).unwrap_or_else(|| if pressed.is_some() { "button" } else { chrome_accessibility_role(&hit.kind) });
                    let selected = names.get(id).and_then(|name| name.selected).or_else(|| self.chrome_accessibility_selected(id, &hit.kind));
                    let focusable = !matches!(role, "tabpanel" | "note");
                    ui_contract::AccessibilityProjectionNode {
                        value_step: None,
                        invalid: false,
                        set_size: None,
                        pos_in_set: None,
                        tone: None,
                        node_id: index as u64 + 1,
                        key: id.to_string(),
                        role: role.to_string(),
                        depth: 0,
                        label: Some(names.get(id).map(|name| name.label.clone()).unwrap_or_else(|| humanize_control_id(id))),
                        description: names.get(id).and_then(|name| name.description.clone()).or_else(|| ShellPaletteKind::from_input_id(id).map(|kind| {
                            shell_chrome_string(
                                match kind {
                                    ShellPaletteKind::Search => "search.description",
                                    ShellPaletteKind::Find => "find.description",
                                },
                                self.locale_id == "de",
                            )
                            .to_string()
                        })),
                        live: ui_contract::liveness_name(ui_contract::Liveness::Off).to_string(),
                        shortcut: shell_control_hotkey_badge(&shortcuts, id),
                        hidden: false,
                        disabled: presentation.is_some_and(|name| name.disabled),
                        focusable,
                        tabbable: focusable && (role != "tab" || selected == Some(true)),
                        actionable: names.get(id).is_none_or(|name| name.role != Some("tabpanel") && !name.disabled),
                        focused: self.accessibility_focused_control_id.as_deref() == Some(id),
                        checked: (role == "switch").then(|| self.chrome_accessibility_checked(id, &hit.kind)).flatten(),
                        pressed,
                        selected,
                        expanded: (hit.kind == HitKind::Select).then(|| self.open_selects.get(id).copied().unwrap_or(false)),
                        editable: false,
                        multiline: false,
                        controls: names.get(id).and_then(|name| name.controls.clone()),
                        active_descendant: None,
                        level: None,
                        rect: Some([hit.rect.x, hit.rect.y, hit.rect.w, hit.rect.h]),
                        value_min: presentation.and_then(|name| name.value_range).map(|range| range[0]),
                        value_max: presentation.and_then(|name| name.value_range).map(|range| range[1]),
                        value_now: presentation.and_then(|name| name.value_range).map(|range| range[2]),
                        value_text: ShellPaletteKind::from_input_id(id)
                            .map(|kind| self.palette_query(kind).to_string())
                            .or_else(|| presentation.and_then(|name| name.value_text.clone()))
                            .or_else(|| self.widget_maps.input_metas.get(id).map(|meta| meta.value.clone())),
                        busy: false,
                    }
                })
                .collect::<Vec<_>>();
            for anchor in PanelAnchor::ALL.into_iter().filter(|anchor| !self.mobile_panel_active() && self.anchor_open(*anchor)) {
                for row in self.anchor_panel_tab_rows(anchor) {
                    for tab in row {
                        if nodes.len() >= SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY || nodes.iter().any(|node| node.key == tab.id) {
                            continue;
                        }
                        nodes.push(ui_contract::AccessibilityProjectionNode {
                            value_step: None,
                            invalid: false,
                            set_size: None,
                            pos_in_set: None,
                            tone: None,
                            node_id: nodes.len() as u64 + 1,
                            key: tab.id.clone(),
                            role: "button".to_string(),
                            depth: 0,
                            label: Some(tab.label.clone()),
                            description: None,
                            live: ui_contract::liveness_name(ui_contract::Liveness::Off).to_string(),
                            shortcut: shell_control_hotkey_badge(&shortcuts, &tab.id),
                            hidden: false,
                            disabled: false,
                            focusable: true,
                            tabbable: true,
                            actionable: true,
                            focused: self.accessibility_focused_control_id.as_deref() == Some(tab.id.as_str()),
                            checked: None,
                            pressed: Some(self.anchor_state(anchor).path.iter().any(|id| id == &tab.id)),
                            selected: None,
                            expanded: None,
                            editable: false,
                            multiline: false,
                            controls: None,
                            active_descendant: None,
                            level: None,
                            rect: None,
                            value_min: None,
                            value_max: None,
                            value_now: None,
                            value_text: None,
                            busy: false,
                        });
                    }
                }
            }
            if let Some(export) = self.icon_export.as_ref() {
                let mut status = chrome_status_accessibility_node(nodes.len() as u64 + 1, "shell.icon-export.progress", export.message(self.locale_id == "de"));
                let (done, total) = export.counts();
                status.role = "progressbar".into();
                status.value_min = Some(0.0);
                status.value_max = Some(total as f64);
                status.value_now = Some(done as f64);
                status.value_text = status.label.clone();
                status.busy = export.running();
                nodes.push(status);
            }
            if let Some(notice) = self.transient_notice_accessibility_node(nodes.len() as u64 + 1).filter(|_| nodes.len() < SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY) {
                nodes.push(notice);
            }
            if let Some(folder) = self.folder_reconnect_accessibility_node(nodes.len() as u64 + 1).filter(|_| nodes.len() < SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY) {
                nodes.push(folder);
            }
            for band in self.band_accessibility_nodes(nodes.len() as u64 + 1) {
                if nodes.len() < SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY {
                    nodes.push(band);
                }
            }
            for (key, label) in self.footer_status_chips() {
                if nodes.len() < SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY && !nodes.iter().any(|node| node.key == key) {
                    nodes.push(chrome_status_accessibility_node(nodes.len() as u64 + 1, key, label));
                }
            }
            nodes
        })
    }

    /// 🧯️ The showing transient notice as the polite status React renders it (`role=status`, `aria-live=polite`,
    /// `data-notice-code`): named by its localized message and described by its machine code, so an assistive technology
    /// and the ARIA mirror hear what the banner paints and not only its close button. Projected over a modal too — a
    /// refusal answered while a dialog is open is still told. `None` once dismissed or past its 4 s deadline.
    fn transient_notice_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        let notice = self.chrome_build.transient_notice.as_ref().filter(|notice| !transient_notice_expired(notice, chrome_now_ms()))?;
        let mut node = chrome_status_accessibility_node(node_id, TRANSIENT_NOTICE_STATUS_ID, notice.message.clone());
        node.description = notice.code.clone();
        Some(node)
    }

    /// 🏷️ The footer chips this shell paints without a hit target — the presence roster (React's `#s-presence-peers`
    /// status) and the hub connection — with the text they paint, so a screen reader and the
    /// accessibility mirror read them too. The signed-out action is a separate button and gets its own hit-backed node.
    pub(crate) fn footer_status_chips(&self) -> Vec<(&'static str, String)> {
        let mut chips = Vec::new();
        if !self.mobile_panel_active() {
            chips.push(("s-presence-peers", ui_wgpu::wgpu::presence_bar_chip_accessible_text(&self.footer_presence_rows(), None, self.active_locale())));
        }
        chips.push(("s-hub-connection", self.hub_footer_label()));
        chips
    }
}

/// 🔊️ One non-interactive chrome status node: named, politely live, never focusable nor actionable.
pub(crate) fn chrome_status_accessibility_node(node_id: u64, key: &str, label: String) -> ui_contract::AccessibilityProjectionNode {
    ui_contract::AccessibilityProjectionNode {
        value_step: None,
        invalid: false,
        set_size: None,
        pos_in_set: None,
        tone: None,
        node_id,
        key: key.to_string(),
        role: "status".to_string(),
        depth: 0,
        label: Some(label),
        description: None,
        live: ui_contract::liveness_name(ui_contract::Liveness::Polite).to_string(),
        shortcut: None,
        hidden: false,
        disabled: false,
        focusable: false,
        tabbable: false,
        actionable: false,
        focused: false,
        checked: None,
        pressed: None,
        selected: None,
        expanded: None,
        editable: false,
        multiline: false,
        controls: None,
        active_descendant: None,
        level: None,
        rect: None,
        value_min: None,
        value_max: None,
        value_now: None,
        value_text: None,
        busy: false,
    }
}
//#endregion ♿️ChromeAccessibleNames

#[cfg(test)]
#[path = "../../../🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs"]
mod theme_editor_and_accessibility_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/⌨️native-keyboard-ring/🦀️.rs"]
mod native_keyboard_ring_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🎯️presented-input-authority/🦀️.rs"]
mod presented_input_authority_tests;
