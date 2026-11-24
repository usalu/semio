//! 🪟 Owns the native original-image dialog, file handle, raw progress and settlement.
use super::*;
use semio_framework::kernel::Effect;
use semio_renderer_wgpu_program_bridge::NativeOriginalMediaRequest;
use std::{fs::File, path::Path, sync::{Arc, atomic::{AtomicU64, Ordering}}};

pub struct NativeOriginalMediaTask {
    id: Option<u64>,
    cancel: CancelToken,
    controller: String,
    plugin_id: String,
    request: Arc<NativeOriginalMediaRequest>,
    package: Option<String>,
    component: Option<String>,
    progress: Arc<(AtomicU64, AtomicU64)>,
    observed: (u64, u64),
    effects: tokio::sync::mpsc::Receiver<Vec<Effect>>,
    answer: ShellDetached<Result<bool, String>>,
}

impl Drop for NativeOriginalMediaTask {
    fn drop(&mut self) { self.cancel.cancel_now(); }
}

impl NativeOriginalMediaTask {
    pub fn progress(&self) -> (u64, u64) {
        (self.progress.0.load(Ordering::Acquire), self.progress.1.load(Ordering::Acquire))
    }
}

pub struct NativeOriginalMediaPoll {
    pub controller: String,
    pub effects: Vec<Effect>,
    pub result: Option<Result<bool, String>>,
}

impl OpeningState {
    /// 📂 Starts native selection or settles the issued loan if a picker already owns the dialog.
    pub fn queue_original_media(
        &mut self, program: ProgramBridgeEntry, controller: String, request: NativeOriginalMediaRequest,
    ) {
        if self.native_original_media.iter().any(|task| task.controller == controller
            && task.plugin_id == program.plugin_id && task.package == program.package_id
            && task.component == program.component_sha256 && task.request.same_loan(&request)) { return; }
        let id = self.active_original_media().is_none().then(|| {
            let id = self.import_transfer_next;
            self.import_transfer_next = id.wrapping_add(1).max(1);
            id
        });
        let (plugin_id, package, component) =
            (program.plugin_id.clone(), program.package_id.clone(), program.component_sha256.clone());
        let request = Arc::new(request);
        let worker_request = Arc::clone(&request);
        let cancel = CancelToken::root_now();
        let worker_cancel = cancel.clone();
        let progress = Arc::new((AtomicU64::new(0), AtomicU64::new(0)));
        let worker_progress = Arc::clone(&progress);
        let (sender, effects) = tokio::sync::mpsc::channel(1);
        let answer = ShellDetached::spawn(async move {
            let source = if id.is_some() {
                select_original_media(&program, &worker_request, &worker_cancel).await
                    .map(|source| source.map(|(file, name, len)| (file, name, "image/png", len)))
            } else { Ok(None) };
            program.deliver_original_media(&worker_request, source, move |effects| {
                let sender = sender.clone();
                async move { if !effects.is_empty() { let _ = sender.send(effects).await; } }
            }, || !worker_cancel.is_cancelled_now(), |completed, total| {
                worker_progress.1.store(total, Ordering::Release);
                worker_progress.0.store(completed, Ordering::Release);
            }).await
        });
        self.native_original_media.push(NativeOriginalMediaTask { id, cancel, controller, plugin_id, request,
            package, component, progress, observed: (0, 0), effects, answer });
    }

    /// ⏱️ Releases every ready turn's host effects under its original controller.
    pub fn poll_original_media(&mut self, docs: &DocumentState, plugins: &PluginState) -> Vec<NativeOriginalMediaPoll> {
        let mut receipts = Vec::new();
        self.native_original_media.retain_mut(|task| {
            let mounted = docs.session.iter().chain(docs.spawned_session.iter()).any(|session|
                session.plugin_id == task.plugin_id && session.app.controller_id == task.controller
                    && session.instance_id == task.request.instance_id());
            let unchanged = plugins.plugins.iter().any(|plugin| plugin.plugin_id == task.plugin_id
                && plugin.package_id == task.package && plugin.component_sha256 == task.component);
            if !mounted || !unchanged { task.cancel.cancel_now(); }
            let result = task.answer.take();
            let effects = task.effects.try_recv().unwrap_or_default();
            let now = task.progress();
            let settled = result.is_some();
            if settled || task.observed != now || !effects.is_empty() {
                receipts.push(NativeOriginalMediaPoll { controller: task.controller.clone(),
                    effects: if mounted && unchanged { effects } else { Vec::new() }, result });
            }
            task.observed = now;
            !settled
        });
        receipts
    }

    pub(super) fn active_original_media(&self) -> Option<(u64, &NativeOriginalMediaTask)> {
        self.native_original_media.iter().find_map(|task| task.id.map(|id| (id, task)))
    }

    pub fn cancel_original_media(&self, id: u64) -> bool {
        let Some((_, task)) = self.active_original_media().filter(|(active, _)| *active == id) else { return false; };
        task.cancel.cancel_now();
        true
    }
}

async fn select_original_media(
    program: &ProgramBridgeEntry, request: &NativeOriginalMediaRequest, cancel: &CancelToken,
) -> Result<Option<(File, String, u64)>, String> {
    if cancel.is_cancelled_now() { return Ok(None); }
    let owner = program.read_app_document_identity(request.instance_id()).await.map_err(|error| error.to_string())?;
    if !request.matches_document(owner.app_instance_id, owner.parent_document_id.as_deref()) {
        return Err("original-media.stale-picker-owner".into());
    }
    if cancel.is_cancelled_now() { return Ok(None); }
    let selected = ui_host::select_native_paths(ui_host::NativeFileDialogRequest::open(["png"], false)).await;
    if selected.len() > 1 { return Err("original-media.selection-count".into()); }
    let Some(path) = selected.into_iter().next() else { return Ok(None); };
    if cancel.is_cancelled_now() { return Ok(None); }
    open_original_media(&path).map(Some)
}

fn open_original_media(path: &Path) -> Result<(File, String, u64), String> {
    let name = path.file_name().and_then(std::ffi::OsStr::to_str).ok_or("original-media.filename")?.to_owned();
    let file = File::open(path).map_err(|error| error.to_string())?;
    let meta = file.metadata().map_err(|error| error.to_string())?;
    if !meta.is_file() { return Err("original-media.file-kind".into()); }
    Ok((file, name, meta.len()))
}

/// 🧾 Projects accepted bytes and the existing transfer cancellation action.
pub(super) fn task_nodes(id: u64, task: &NativeOriginalMediaTask, german: bool) -> [UiNode; 3] {
    let (completed, total) = task.progress();
    let label = shell_chrome_string("taskManager.transfer.importOriginal", german);
    let value = if total == 0 { label.to_owned() } else { format!("{label} · {completed}/{total}") };
    let key = format!("documentTransfer:{id}");
    [
        settings_text_row(label),
        UiNode::Progress(semio_ui_declaration::ui::UiProgressNode {
            id: format!("os.task-manager.progress.{key}"), completed: completed as f64,
            total: (total != 0).then_some(total as f64), value_text: Label::data(value),
            presence: UiPresence::default(), menu: None,
        }),
        UiNode::Button(UiButtonNode {
            id: Some(format!("os.task-manager.cancel.{key}")), icon_id: IconName::Square,
            label: Label::data(shell_chrome_string("taskManager.transfer.cancelImport", german)),
            action: ActionDescriptor { controller_id: "framework".into(), action: IMPORT_TRANSFER_CANCEL_ACTION.into(),
                args: semio_renderer_wgpu_host_base::action_args_json!({ "transfer": id }) },
            style: None, presence: UiPresence::default(), menu: None,
        }),
    ]
}
