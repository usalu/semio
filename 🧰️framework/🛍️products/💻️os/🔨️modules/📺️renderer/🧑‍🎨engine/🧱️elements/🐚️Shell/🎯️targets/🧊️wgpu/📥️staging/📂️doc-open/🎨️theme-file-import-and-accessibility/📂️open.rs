//! 📂 Owns one picked natural file until its fresh program and archive can be mounted.
use super::*;
use semio_renderer_wgpu_program_bridge::{NativeMediaDeclaration, ProgramFault};
use std::{io::Read, path::{Path, PathBuf}, sync::{Arc, atomic::{AtomicU8, Ordering}}};

pub struct NativeOpenedFile {
    pub program: ProgramBridgeEntry,
    pub source_instance: u32,
    pub instance: u32,
    pub document_id: String,
    pub folder: PathBuf,
    published: bool,
}

impl NativeOpenedFile {
    pub fn publish(mut self) { self.published = true; }
}

impl Drop for NativeOpenedFile {
    fn drop(&mut self) {
        if !self.published { self.program.destroy_app(self.instance); }
    }
}

impl OpeningState {
    pub fn start_native_file_open(
        &mut self, program: ProgramBridgeEntry, app: String, source_instance: u32,
        declaration: NativeMediaDeclaration, folder: PathBuf, label: String,
    ) -> bool {
        if self.document_opening.as_ref().is_some_and(ShellDocumentOpening::running) || self.native_media_save.is_some() {
            return false;
        }
        let plugin_id = program.plugin_id.clone();
        let app_id = app.clone();
        let cancel = CancelToken::root_now();
        let worker_cancel = cancel.clone();
        let progress = Arc::new(AtomicU8::new(0));
        let worker_progress = progress.clone();
        let pending = ShellDetached::spawn(async move {
            open_native_file(program, app, source_instance, declaration, folder, worker_cancel, worker_progress).await
                .map(ShellDocumentOpenAnswer::NativeFile).map_err(ProgramFault::from)
        });
        let mut opening = ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Instantiating, plugin_id, app_id, None, Some(pending));
        opening.cancel = Some(cancel);
        opening.native_transfer_progress = progress;
        self.document_opening = Some(opening);
        true
    }
}

async fn open_native_file(
    program: ProgramBridgeEntry, app: String, source_instance: u32, declaration: NativeMediaDeclaration,
    folder: PathBuf, cancel: CancelToken, progress: Arc<AtomicU8>,
) -> Result<Option<NativeOpenedFile>, String> {
    if cancel.is_cancelled_now() { return Ok(None); }
    let extension = declaration.extension().ok_or("Open File has no declared extension")?;
    let Some(path) = ui_host::select_native_paths(ui_host::NativeFileDialogRequest::open([extension], false)).await
        .into_iter().next() else { return Ok(None); };
    if cancel.is_cancelled_now() { return Ok(None); }
    let mut opened = if declaration.natural_pdf_source().is_some() {
        let (file, extent) = open_selected_file(&path, u32::MAX as u64)?;
        let mut retained = None;
        let imported = program.import_native_pdf_source(
            &app, &declaration, file, extent as u32, &cancel,
            |done, total| progress.store((done * 100 / total) as u8 + 1, Ordering::Release),
            |instance| retained = Some(NativeOpenedFile {
                program: program.clone(), source_instance, instance, document_id: String::new(),
                folder: folder.clone(), published: false,
            }),
        ).await?;
        if imported.is_none() { return Ok(None); }
        retained.ok_or("Open File lost the natural source instance")?
    } else {
        let maximum_bytes = declaration.natural_file_payload_limit().await?;
        let Some(bytes) = read_selected_file(&path, maximum_bytes, || cancel.is_cancelled_now()).await? else { return Ok(None); };
        let instance = program.create_app(&app).await.map_err(|error| error.to_string())?;
        let opened = NativeOpenedFile { program, source_instance, instance, document_id: String::new(), folder, published: false };
        if cancel.is_cancelled_now() || !wait_native_file_import(&cancel, opened.program.import_native_file(instance, &declaration, bytes)).await? {
            return Ok(None);
        }
        opened
    };
    let instance = opened.instance;
    if cancel.is_cancelled_now() { return Ok(None); }
    let owner = opened.program.read_app_document_identity(instance).await.map_err(|error| error.to_string())?;
    let document = owner.parent_document_id.as_deref().ok_or("Open File did not create a document identity")?;
    let archive = opened.program.read_app_document_archive(instance).await.map_err(|error| error.to_string())?;
    let schema = opened.program.manifest.apps.iter().find(|candidate| candidate.id == app)
        .map(|candidate| candidate.io.artifact_schema.as_str()).ok_or("Open File lost its declared editor")?;
    let encoded = protocol::encode_document_archive_bytes(&archive).map_err(|error| error.to_string())?;
    if cancel.is_cancelled_now() || opened.program.read_app_document_identity(instance).await.map_err(|error| error.to_string())? != owner {
        return Ok(None);
    }
    if cancel.is_cancelled_now() { return Ok(None); }
    let storage = store_sync::sync::FolderEventLogStorage::new(opened.folder.clone());
    storage.create_archive_if_absent(document, schema, &encoded).await.map_err(|error| error.to_string())?;
    let readback = storage.archive_matches(document, &encoded, || cancel.is_cancelled_now());
    if cancel.is_cancelled_now() { return Ok(None); }
    if !readback.map_err(|error| error.to_string())? { return Err("Open File could not verify the local document archive".into()); }
    opened.document_id = document.to_owned();
    Ok(Some(opened))
}

/// 🛑 Drops a cancelled import request so the kernel can retire its abandoned command.
async fn wait_native_file_import(
    cancel: &CancelToken,
    request: impl std::future::Future<Output = Result<(), String>>,
) -> Result<bool, String> {
    match semio_framework_async::select2(cancel.cancelled(), request).await {
        semio_framework_async::Either::Left(()) => Ok(false),
        semio_framework_async::Either::Right(result) => result.map(|()| true),
    }
}

fn open_selected_file(path: &Path, maximum_bytes: u64) -> Result<(std::fs::File, u64), String> {
    let file = std::fs::File::open(path).map_err(|error| error.to_string())?;
    let metadata = file.metadata().map_err(|error| error.to_string())?;
    if !metadata.is_file() || metadata.len() == 0 || metadata.len() > maximum_bytes {
        return Err("Open File has an inadmissible input extent".into());
    }
    Ok((file, metadata.len()))
}

async fn read_selected_file(path: &Path, maximum_bytes: usize, cancelled: impl Fn() -> bool) -> Result<Option<Vec<u8>>, String> {
    let (file, extent) = open_selected_file(path, maximum_bytes as u64)?;
    let mut input = file.take(maximum_bytes as u64 + 1);
    let mut data = Vec::with_capacity(extent as usize);
    let mut page = [0; 4096];
    loop {
        if cancelled() { return Ok(None); }
        let count = input.read(&mut page).map_err(|error| error.to_string())?;
        if count == 0 { break; }
        if data.len() + count > maximum_bytes { return Err("Open File exceeded its input cap".into()); }
        data.extend_from_slice(&page[..count]);
        semio_framework_async::yield_once().await;
    }
    if data.len() as u64 != extent { return Err("Open File changed size while being read".into()); }
    Ok(Some(data))
}

pub fn native_local_documents() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("S_DATA_DIR").map(PathBuf::from).filter(|path| path.is_absolute()) {
        return Some(path.join("documents"));
    }
    #[cfg(target_os = "windows")]
    let root = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);
    #[cfg(target_os = "macos")]
    let root = std::env::var_os("HOME").map(|home| PathBuf::from(home).join("Library/Application Support"));
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let root = std::env::var_os("XDG_DATA_HOME").map(PathBuf::from).filter(|path| path.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")));
    root.filter(|path| path.is_absolute()).map(|path| path.join("semio/documents"))
}
