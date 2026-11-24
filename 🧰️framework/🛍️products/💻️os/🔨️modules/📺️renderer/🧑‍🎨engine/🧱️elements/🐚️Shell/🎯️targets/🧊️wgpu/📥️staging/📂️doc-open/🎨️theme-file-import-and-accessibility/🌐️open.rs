//! 🌐 Owns one picked or dropped natural file in the browser until its fresh program instance can be mounted: pick, route, bounded read and typed import, the wgpu twin of React's `openNaturalFile` ([`🏛️ShellHost/📤️document-transfer-tabs`]).
use super::*;
use semio_renderer_wgpu_program_bridge::ProgramFault;
use std::sync::{atomic::{AtomicU8, Ordering}, Arc};

/// 📏 How many bytes one host-io range read moves (React's `NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1`).
const BROWSER_FILE_READ_CHUNK_BYTES: u64 = 256 * 1024;

/// 📏 The widest range the page answers in one read (`WGPU_PICKED_FILE_RANGE_MAX_BYTES`).
const BROWSER_FILE_RANGE_MAX_BYTES: u64 = 1024 * 1024;

/// 🔁 How many times a route may ask for more of the file before the content check is given up as unsettled.
const BROWSER_SNIFF_ROUNDS: usize = 64;

/// 📂 A fresh instance holding the imported file, destroyed unless it was published.
pub struct BrowserOpenedFile {
    pub program: ProgramBridgeEntry,
    pub app: AppDefinition,
    pub name: String,
    pub source_instance: u32,
    pub instance: u32,
    pub document_id: String,
    /// 🗂️ The `folder://` lane of a document the device already keeps (a reopen): it is published without being admitted again.
    pub kept_folder: Option<String>,
    published: bool,
}

impl BrowserOpenedFile {
    /// 📂 A fresh instance of `app` holding the document `document_id`, to be published or destroyed.
    pub fn new(program: ProgramBridgeEntry, app: AppDefinition, name: String, source_instance: u32, instance: u32, document_id: String) -> Self {
        Self { program, app, name, source_instance, instance, document_id, kept_folder: None, published: false }
    }

    pub fn publish(mut self) { self.published = true; }
}

impl Drop for BrowserOpenedFile {
    fn drop(&mut self) {
        if !self.published { self.program.destroy_app(self.instance); }
    }
}

/// 🧭 The Open file catalog of this shell: the product's claims, revision sniffs and package claims as the page answers them, else the formats the mounted editors declare.
pub async fn browser_catalog(plugins: &[ProgramBridgeEntry]) -> BrowserCatalog {
    let claims = host_io_call(&serde_json::json!({ "op": "format-claims" }).to_string(), None).await.ok();
    claims.as_deref().and_then(BrowserCatalog::from_claims_json).unwrap_or_else(|| BrowserCatalog::from_mounted(plugins))
}

/// 💾 The folder a kept document lives in, React's `localArtifactAdmissionV1` target: `<dataDir>/os/local-documents/<sha256 of the document id>`, as the `folder://` address the browser's folder transport serves.
pub async fn local_document_folder_uri(data_dir: Option<&str>, document_id: &str) -> Result<String, String> {
    let data_dir = data_dir.filter(|dir| !dir.is_empty()).ok_or("natural-file.no-data-folder")?;
    let digest = host_io_call(&serde_json::json!({ "op": "sha256-hex", "text": document_id }).to_string(), None).await?;
    let target = format!("{}/os/local-documents/{digest}", data_dir.trim_end_matches(['/', '\\']));
    if target.len() + "/.semio/.genesis-4294967295-ffffffffffffffff.stage".len() > 256 { return Err("natural-file.invalid-request".into()); }
    Ok(format!("folder://{target}"))
}

/// 🗂 Admits one kept document to the device's local catalog through the page's facet client (the `os.config.local-catalog` store facet React writes); `true` once the store's read-back confirmed it.
pub async fn admit_local_catalog_document(document_id: &str, schema: &str, name: &str, target: &str, plugin_id: &str, app_id: &str, admitted_at_ms: u64) -> bool {
    let document = serde_json::json!({ "documentId": document_id, "schema": schema, "name": name, "storage": "folder", "target": target, "admittedAtMs": admitted_at_ms, "surface": { "pluginId": plugin_id, "appId": app_id } });
    host_io_call(&serde_json::json!({ "op": "local-catalog-admit", "document": document }).to_string(), None).await.is_ok_and(|answer| answer == "true")
}

/// 🗂 Retires one document from the device's local catalog (React's `unlist`); `true` once confirmed.
pub async fn retire_local_catalog_document(document_id: &str) -> bool {
    host_io_call(&serde_json::json!({ "op": "local-catalog-retire", "documentId": document_id }).to_string(), None).await.is_ok_and(|answer| answer == "true")
}

/// 📖 Reads `[start, end)` of a picked file through the page's range door, in answers no wider than the door admits.
async fn read_picked_range(token: u32, start: u64, end: u64) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::with_capacity((end - start) as usize);
    while (bytes.len() as u64) < end - start {
        let from = start + bytes.len() as u64;
        let to = (from + BROWSER_FILE_RANGE_MAX_BYTES).min(end);
        let answer = host_io_call(&serde_json::json!({ "op": "read-picked-file", "token": token, "start": from, "end": to }).to_string(), None).await?;
        let chunk = semio_framework_io_base64::base64_standard_decode(&answer).map_err(|_| "open-file.read-not-base64".to_string())?;
        if chunk.len() as u64 != to - from { return Err("open-file.size-changed".into()); }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

/// 🗑 Lets the page drop its hold on a picked file.
pub async fn release_browser_picked_file(token: u32) {
    let _ = host_io_call(&serde_json::json!({ "op": "release-picked-file", "token": token }).to_string(), None).await;
}

impl OpeningState {
    /// 📂 Starts the pick and route step of one Open File as the shell's one document open; `false` while another runs.
    pub fn start_browser_file_open(&mut self, label: String, source: Option<BrowserPickedFile>, catalog: BrowserCatalog, plugin_id: String, app_id: String) -> bool {
        if self.document_opening.as_ref().is_some_and(ShellDocumentOpening::running) { return false; }
        let cancel = CancelToken::root_now();
        let worker_cancel = cancel.clone();
        let pending = ShellDetached::spawn(async move {
            pick_and_route(source, catalog, worker_cancel).await.map(ShellDocumentOpenAnswer::BrowserRouting).map_err(ProgramFault::from)
        });
        let mut opening = ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Instantiating, plugin_id, app_id, None, Some(pending));
        opening.cancel = Some(cancel);
        self.document_opening = Some(opening);
        true
    }

    /// 📥 Continues a routed open with the bounded read and typed import into a fresh instance of the owner editor.
    pub fn start_browser_file_import(&mut self, mut opening: ShellDocumentOpening, program: ProgramBridgeEntry, app: AppDefinition, format_kind: String, file: BrowserPickedFile, source_instance: u32) {
        let cancel = opening.cancel.clone().unwrap_or_else(CancelToken::root_now);
        let progress = Arc::new(AtomicU8::new(0));
        opening.plugin_id = program.plugin_id.clone();
        opening.app_id = app.id.clone();
        opening.native_transfer_progress = progress.clone();
        opening.phase = ShellDocumentOpenPhase::Instantiating;
        opening.pending = Some(ShellDetached::spawn(async move {
            import_browser_file(program, app, format_kind, file, source_instance, cancel, progress).await.map(ShellDocumentOpenAnswer::BrowserFile).map_err(ProgramFault::from)
        }));
        self.document_opening = Some(opening);
    }
}

async fn pick_and_route(source: Option<BrowserPickedFile>, catalog: BrowserCatalog, cancel: CancelToken) -> Result<BrowserOpenRouting, String> {
    let file = match source {
        Some(file) => file,
        None => {
            let answer = host_io_call(&serde_json::json!({ "op": "request-picked-file", "accept": catalog.accept() }).to_string(), None).await?;
            match serde_json::from_str::<Option<BrowserPickedFile>>(&answer).map_err(|error| error.to_string())? {
                Some(file) => file,
                None => return Ok(BrowserOpenRouting::Cancelled(None)),
            }
        }
    };
    let watched = cancel.clone();
    let cache = RangeCache::new(file.size, move || watched.is_cancelled_now());
    for _ in 0..BROWSER_SNIFF_ROUNDS {
        let routing = route_browser_file(file.clone(), &catalog, cancel.is_cancelled_now(), Some(&cache), &|bytes| cache.head(bytes));
        let misses = cache.take_misses();
        if misses.is_empty() {
            if !matches!(routing, BrowserOpenRouting::Routed { .. }) {
                if let Some(token) = routing.token() { release_browser_picked_file(token).await; }
            }
            return Ok(routing);
        }
        for (start, end) in misses {
            cache.insert(start, end, read_picked_range(file.token, start, end).await?);
        }
    }
    release_browser_picked_file(file.token).await;
    Err("open-file.content-check-unsettled".into())
}

async fn import_browser_file(program: ProgramBridgeEntry, app: AppDefinition, format_kind: String, file: BrowserPickedFile, source_instance: u32, cancel: CancelToken, progress: Arc<AtomicU8>) -> Result<Option<BrowserOpenedFile>, String> {
    let outcome = import_picked_bytes(&program, &app, &format_kind, &file, source_instance, &cancel, &progress).await;
    release_browser_picked_file(file.token).await;
    outcome
}

async fn import_picked_bytes(program: &ProgramBridgeEntry, app: &AppDefinition, format_kind: &str, file: &BrowserPickedFile, source_instance: u32, cancel: &CancelToken, progress: &AtomicU8) -> Result<Option<BrowserOpenedFile>, String> {
    let maximum = program.natural_file_maximum_bytes(format_kind).await? as u64;
    if file.size == 0 || file.size > maximum { return Err("open-file.inadmissible-extent".into()); }
    let mut bytes = Vec::with_capacity(file.size as usize);
    while (bytes.len() as u64) < file.size {
        if cancel.is_cancelled_now() { return Ok(None); }
        let start = bytes.len() as u64;
        let end = (start + BROWSER_FILE_READ_CHUNK_BYTES).min(file.size);
        bytes.extend_from_slice(&read_picked_range(file.token, start, end).await?);
        progress.store((bytes.len() as u64 * 100 / file.size) as u8 + 1, Ordering::Release);
        semio_framework_async::yield_once().await;
    }
    if cancel.is_cancelled_now() { return Ok(None); }
    let instance = program.create_app(&app.id).await.map_err(|error| error.to_string())?;
    let mut opened = BrowserOpenedFile { program: program.clone(), app: app.clone(), name: file.name.clone(), source_instance, instance, document_id: String::new(), kept_folder: None, published: false };
    if cancel.is_cancelled_now() { return Ok(None); }
    program.import_natural_file(instance, format_kind, &bytes).await?;
    if cancel.is_cancelled_now() { return Ok(None); }
    let owner = program.read_app_document_identity(instance).await.map_err(|error| error.to_string())?;
    opened.document_id = owner.parent_document_id.ok_or("open-file.no-document-identity")?;
    Ok(Some(opened))
}
