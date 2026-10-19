//! 📂️ `shell-doc-open` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 📤️FileOpenImport
/// 📥️ Turns one picker answer into the actions a shell dispatches — one per
/// [`semio_framework::kernel::import_payload_chunks`] chunk of each file, in order, with the exact
/// argument envelope the effect contract owns.
///
/// ⚖️ The chunking and the envelope are the kernel's, never this shell's: this target used to send
/// `{json, payload}` in ONE unchunked invocation while React sent `{payload, name, chunk,
/// chunkCount}` per chunk, and a plugin could satisfy only one of them. An unchunked import also asks
/// the fixed guest heap for a contiguous block the size of the whole file
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn file_open_import_actions(controller_id: &str, import_action: &str, opened: Vec<OpenedFile>, multiple: bool, retained_args: Option<&semio_framework::DslValue>) -> Vec<ActionDescriptor> {
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

/// 🛑️ The shell verb a Task Manager row's Cancel sends (`{transfer}` on the `framework` controller).
pub const IMPORT_TRANSFER_CANCEL_ACTION: &str = "cancelImportTransfer";

/// 🎞️ What one transfer moves into the guest: a picked document's chunks, or the frames one decoded video yields
/// (`RequestMediaFrames`: one `frameAction` per sampled frame, then its `doneAction` — or its one `fallbackAction`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferLane {
    DocumentImport,
    VideoFrames,
}

impl TransferLane {
    /// 🏷️ The lane a Task Manager row names.
    pub fn label(self, de: bool) -> &'static str {
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
pub struct ImportTransfer {
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

impl OpeningState {
    /// 📥️ Starts a picked-file import as an [`ImportTransfer`] the drain steps; an empty pick starts nothing. `true` when one
    /// started.
    pub fn begin_import_transfer(&mut self, picked: PickedImport, docs: &DocumentState) -> bool {
        let PickedImport { controller_id, import_action, opened, args, multiple } = picked;
        let Some(first_file) = opened.first().map(|file| file.name.clone()) else { return false };
        let file = opened.iter().map(|file| file.name.as_str()).collect::<Vec<_>>().join(", ");
        let chunks: VecDeque<ActionDescriptor> = file_open_import_actions(&controller_id, &import_action, opened, multiple, args.as_ref()).into();
        debug_log(&format!("[TRACE] wgpu-shell import {import_action} file={file} chunks={}", chunks.len()));
        self.queue_transfer(TransferLane::DocumentImport, file, first_file, controller_id, chunks, docs)
    }

    /// 🎞️ Starts the frame dispatches of one decoded video as an [`ImportTransfer`] on the [`TransferLane::VideoFrames`]
    /// lane — the host cancel `RequestMediaFrames` lacked: listed with its progress and a Cancel, stepped within the same
    /// turn budget as an import, and a cancelled stream that already handed the guest a frame is freed by the app's
    /// `importAbort`, so its open import transaction (design §15) leaves zero trace. An empty answer starts nothing.
    pub fn begin_media_frames_transfer(&mut self, controller_id: String, frames: Vec<ActionDescriptor>, docs: &DocumentState) -> bool {
        let Some(file) = frames.first().map(|frame| frame.args.as_ref().and_then(|args| args.get("name")).and_then(semio_framework::DslValue::as_str).unwrap_or("video").to_string()) else { return false };
        self.queue_transfer(TransferLane::VideoFrames, file.clone(), file, controller_id, frames.into(), docs)
    }

    /// 🧾️ Queues one transfer behind the running ones under a fresh id, owned by the session's plugin.
    pub fn queue_transfer(&mut self, lane: TransferLane, file: String, first_file: String, controller_id: String, chunks: VecDeque<ActionDescriptor>, docs: &DocumentState) -> bool {
        let owner = docs.session.as_ref().map(|session| session.plugin_id.clone()).unwrap_or_default();
        let id = self.import_transfer_next;
        self.import_transfer_next = self.import_transfer_next.wrapping_add(1).max(1);
        self.import_transfers.push_back(ImportTransfer { id, file, first_file, owner, controller_id, lane, total: chunks.len(), chunks, delivered: 0 });
        true
    }

    /// 🧵️ The Task Manager's running-tasks section — React's `TaskManagerTasksPanel` over `documentTransferTasksV1`: per import
    /// its title, lane, owner and state, a progress bar of the chunks delivered, and "Cancel <file>"; "No task is running."
    /// while none runs.
    pub fn import_task_nodes(&self, prefs: &PrefsState) -> UiNode {
        let is_de = prefs.locale_id == "de";
        let pick = |en: &str, de: &str| if is_de { de.to_string() } else { en.to_string() };
        let mut children = vec![settings_text_row(shell_chrome_string("taskManager.runningTasks", is_de))];
        if self.import_transfers.is_empty() {
            children.push(settings_text_row(&pick("No task is running.", "Es läuft keine Aufgabe.")));
        }
        for transfer in &self.import_transfers {
            let task = format!("documentTransfer:{}", transfer.id);
            let progress = format!("{}/{}", transfer.delivered, transfer.total);
            children.push(settings_text_row(&format!("{} · {} · {} · {}", transfer.file, transfer.lane.label(is_de), transfer.owner, pick("Running", "Läuft"))));
            children.push(UiNode::Progress(semio_framework_ui_declaration::ui::UiProgressNode {
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
                action: ActionDescriptor { controller_id: "framework".into(), action: IMPORT_TRANSFER_CANCEL_ACTION.into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "transfer": transfer.id }) },
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
pub async fn request_file_open(accept: &str, read_as: Option<&str>, multiple: bool) -> Vec<OpenedFile> {
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

/// 📤️ Opens the browser file picker through the page-owned door; one entry per selected file, in
/// selection order, EMPTY on cancel or on a refusal (which is logged, never swallowed).
#[cfg(target_arch = "wasm32")]
pub async fn request_file_open(accept: &str, read_as: Option<&str>, multiple: bool) -> Vec<OpenedFile> {
    let request = serde_json::json!({ "op": "request-file-open", "accept": accept, "readAs": read_as, "multiple": multiple }).to_string();
    let answer = match host_io_call(&request, None).await {
        Ok(answer) => answer,
        Err(error) => {
            debug_log(&format!("[TRACE] wgpu-shell file picker refused: {error}"));
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
            debug_log(&format!("[TRACE] wgpu-shell file picker answer unreadable: {error}"));
            Vec::new()
        }
    }
}

//#endregion 📤️FileOpenImport

#[cfg(target_arch = "wasm32")]
pub async fn present_media_export_bytes_cancellable(filename: String, mime_type: String, bytes: Vec<u8>, cancel: CancelToken) -> Result<bool, String> {
    if cancel.is_cancelled_now() { return Ok(false); }
    let request = serde_json::json!({ "op": "download-media-export", "filename": filename, "mimeType": mime_type }).to_string();
    host_io_call(&request, Some(&bytes)).await.map(|_| true)
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn present_media_export_bytes_cancellable(filename: String, mime_type: String, bytes: Vec<u8>, cancel: CancelToken) -> Result<bool, String> {
    if cancel.is_cancelled_now() { return Ok(false); }
    let extension = std::path::Path::new(&filename).extension().and_then(|value| value.to_str()).unwrap_or_else(|| mime_type.rsplit_once('/').map(|(_, value)| value).unwrap_or("dat"));
    let Some(path) = ui_host::select_native_paths(ui_host::NativeFileDialogRequest::save(&filename, [extension])).await.into_iter().next() else { return Ok(false) };
    if cancel.is_cancelled_now() { return Ok(false); }
    std::fs::write(path, bytes).map_err(|error| error.to_string())?;
    Ok(true)
}
