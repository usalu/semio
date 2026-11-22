//! 🗂️ Owns one kept document's reopen until its fresh program instance can be mounted: read the archive from the document's folder lane, load it into a new instance and check its identity, React's `openKeptFile` ([`🏛️ShellHost/🗂️local-catalog/📂️files`]) as the shell's one cancellable document open.
use super::*;
use semio_renderer_wgpu_program_bridge::ProgramFault;

/// 🗂️ The kept document a reopen restores, as the local catalog lists it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeptFileRequest {
    pub document_id: String,
    pub schema: String,
    pub name: String,
    pub target: String,
}

#[cfg(target_arch = "wasm32")]
type KeptOpened = BrowserOpenedFile;
#[cfg(not(target_arch = "wasm32"))]
type KeptOpened = NativeOpenedFile;

fn kept_answer(opened: Option<KeptOpened>) -> ShellDocumentOpenAnswer {
    #[cfg(target_arch = "wasm32")]
    return ShellDocumentOpenAnswer::BrowserFile(opened);
    #[cfg(not(target_arch = "wasm32"))]
    ShellDocumentOpenAnswer::NativeFile(opened)
}

impl OpeningState {
    /// 🗂️ Starts the reopen of `request` on `app` of `program` as the shell's one document open, its band reading "Opening {name}…" with Cancel; `false` while another open runs.
    pub fn start_kept_file_open(&mut self, request: KeptFileRequest, program: ProgramBridgeEntry, app: AppDefinition, source_instance: u32) -> bool {
        if self.document_opening.as_ref().is_some_and(ShellDocumentOpening::running) { return false; }
        #[cfg(not(target_arch = "wasm32"))]
        if self.native_media_save.is_some() { return false; }
        let cancel = CancelToken::root_now();
        let worker_cancel = cancel.clone();
        let (plugin_id, app_id, label) = (program.plugin_id.clone(), app.id.clone(), request.name.clone());
        let pending = ShellDetached::spawn(async move {
            reopen_kept_file(request, program, app, source_instance, worker_cancel).await.map(kept_answer).map_err(ProgramFault::from)
        });
        let mut opening = ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Instantiating, plugin_id, app_id, None, Some(pending));
        opening.cancel = Some(cancel);
        opening.kept = true;
        self.document_opening = Some(opening);
        true
    }
}

/// 📖 The kept archive's bytes from the page's folder transport; `None` on a cancel.
#[cfg(target_arch = "wasm32")]
async fn read_kept_archive(request: &KeptFileRequest, cancel: &CancelToken) -> Result<Option<Vec<u8>>, String> {
    use semio_renderer_wgpu_shell_docs::{backbone_folder_call, backbone_folder_request, BackboneFolderVerb};
    let uri = format!("folder://{}", request.target);
    let answer = backbone_folder_call(&backbone_folder_request(BackboneFolderVerb::Read, &uri, &request.document_id, None), None).await?;
    if cancel.is_cancelled_now() { return Ok(None); }
    let answer: serde_json::Value = serde_json::from_str(&answer).map_err(|error| error.to_string())?;
    if let Some(error) = answer.get("error").and_then(serde_json::Value::as_str) { return Err(error.to_owned()); }
    match answer.get("status").and_then(serde_json::Value::as_u64) {
        Some(200) => semio_framework_io_base64::base64_standard_decode(answer.get("bodyBase64").and_then(serde_json::Value::as_str).ok_or("natural-file.kept-archive-unavailable")?).map(Some).map_err(|error| error.to_string()),
        Some(204) => Err("natural-file.kept-archive-unavailable".into()),
        Some(status) => Err(format!("backbone-folder.status-{status}")),
        None => Err("natural-file.kept-archive-unavailable".into()),
    }
}

/// 📖 The kept archive's bytes from the document's folder, read in the source's own steps (no await while it is held) so a cancel lands between them; `None` on a cancel.
#[cfg(not(target_arch = "wasm32"))]
async fn read_kept_archive(request: &KeptFileRequest, cancel: &CancelToken) -> Result<Option<Vec<u8>>, String> {
    let storage = store_sync::sync::FolderEventLogStorage::new(std::path::PathBuf::from(&request.target));
    let limit = protocol::channel::archive_reopen_route::DOCUMENT_ARCHIVE_CONTIGUOUS_REOPEN_MAXIMUM_BYTES;
    let cancelled = || cancel.is_cancelled_now();
    let Some(mut source) = storage.open_archive_source(&request.document_id, &request.schema, limit, &cancelled).map_err(|error| error.to_string())? else {
        return Err("natural-file.kept-archive-unavailable".into());
    };
    let seal = source.seal();
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(seal.extent).map_err(|error| error.to_string())?;
    bytes.resize(seal.extent, 0);
    let mut offset = 0;
    while offset < seal.extent {
        if cancel.is_cancelled_now() { return Ok(None); }
        let count = source.read_next(&mut bytes[offset..], &cancelled).map_err(|error| error.to_string())?;
        if count == 0 { return Err("folder record ended prematurely".into()); }
        offset += count;
    }
    if !storage.archive_source_is_current(&request.document_id, &request.schema, seal, limit, &cancelled).map_err(|error| error.to_string())? {
        return Err("folder archive changed during restore".into());
    }
    Ok(Some(bytes))
}

#[cfg(target_arch = "wasm32")]
fn kept_opened(program: ProgramBridgeEntry, app: AppDefinition, request: &KeptFileRequest, source_instance: u32, instance: u32) -> KeptOpened {
    let mut opened = BrowserOpenedFile::new(program, app, request.name.clone(), source_instance, instance, request.document_id.clone());
    opened.kept_folder = Some(format!("folder://{}", request.target));
    opened
}

#[cfg(not(target_arch = "wasm32"))]
fn kept_opened(program: ProgramBridgeEntry, _app: AppDefinition, request: &KeptFileRequest, source_instance: u32, instance: u32) -> KeptOpened {
    NativeOpenedFile::kept(program, request.name.clone(), source_instance, instance, request.document_id.clone(), std::path::PathBuf::from(&request.target))
}

async fn reopen_kept_file(request: KeptFileRequest, program: ProgramBridgeEntry, app: AppDefinition, source_instance: u32, cancel: CancelToken) -> Result<Option<KeptOpened>, String> {
    let Some(bytes) = read_kept_archive(&request, &cancel).await? else { return Ok(None) };
    if cancel.is_cancelled_now() { return Ok(None); }
    let archive = protocol::decode_document_archive_bytes(&bytes).await.map_err(|error| error.to_string())?;
    if cancel.is_cancelled_now() { return Ok(None); }
    let instance = program.create_app(&app.id).await.map_err(|error| error.to_string())?;
    let opened = kept_opened(program.clone(), app, &request, source_instance, instance);
    if cancel.is_cancelled_now() { return Ok(None); }
    program.load_app_document_archive(instance, &archive).await.map_err(|error| error.to_string())?;
    let identity = program.read_app_document_identity(instance).await.map_err(|error| error.to_string())?;
    if identity.parent_document_id.as_deref() != Some(request.document_id.as_str()) { return Err("natural-file.kept-archive-identity-mismatch".into()); }
    if cancel.is_cancelled_now() { return Ok(None); }
    Ok(Some(opened))
}
