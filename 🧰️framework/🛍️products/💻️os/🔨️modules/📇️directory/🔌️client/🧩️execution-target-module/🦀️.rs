//! 🧩️ The component a hub document runs, resolved by the SERVING catalog generation (binding decision of
//! ticket 26/09/23, session 11 13:1x): the hub's execution-target lease names the exact component and
//! descriptor bytes for one document scope; a local copy is used only when its content hash equals the
//! lease's, a content-addressed store copy only when its bytes verify, and anything else is fetched from the
//! hub's own `execution-target/{component, descriptor}` routes, verified against the lease and stored before
//! a shell mounts it. React resolves the same generation through the hub's plugin-module routes (the
//! browser module derived from the same component); a native shell mounts the component itself.

use super::super::schema::{DocumentExecutionTargetLeaseFieldsV1, DocumentOpenIntentV1};
use super::{DirectoryClient, DirectoryClientError, DirectoryTransport};
use semio_framework_async::OperationContext;

/// 🧩️ Where the bytes a document will run came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionTargetModuleSource {
    /// 🏠️ The component this process already mounts has the lease's exact content hash.
    Local,
    /// 🗃️ A content-addressed store entry whose bytes verified against the lease (React's plugin-module `store`).
    Store,
    /// 🌐️ Fetched from the hub's execution-target routes, verified and then stored.
    Hub,
}

/// ⏳️ The step a resolution is on, for a shell's progress band.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionTargetModuleStep {
    Lease,
    Component,
    Descriptor,
    Verified,
}

/// 🧩️ One resolved execution target: the hub's lease, where its bytes came from and, unless the local
/// component already is the lease's, the verified store files a shell mounts.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedExecutionTargetModule {
    pub lease: DocumentExecutionTargetLeaseFieldsV1,
    pub source: ExecutionTargetModuleSource,
    pub files: Option<ExecutionTargetModuleFiles>,
}

/// 🗃️ The verified component and descriptor files of one execution target, named by their content hashes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionTargetModuleFiles {
    pub component: std::path::PathBuf,
    pub descriptor: std::path::PathBuf,
}

/// 🔁️ How often one execution-target request is asked while the hub answers `503`: its own selection deadline
/// (8 s) passed under load — measured `deadline-exceeded` on hub 7800 B2 at load ~40 (two-user gate run 22) —
/// which is a shortage to ride out, not a refusal. Every other answer stands at once.
pub const EXECUTION_TARGET_UNAVAILABLE_ATTEMPTS: usize = 3;

/// 🔁️ Asks `request` again while the hub answers `503` and the context is not cancelled, at most
/// [`EXECUTION_TARGET_UNAVAILABLE_ATTEMPTS`] times.
async fn asked_through_shortage<R, F: std::future::Future<Output = Result<R, DirectoryClientError>>>(ctx: &OperationContext, mut request: impl FnMut() -> F) -> Result<R, DirectoryClientError> {
    let mut attempt = 1;
    loop {
        match request().await {
            Err(DirectoryClientError::Http { status: 503, .. }) if attempt < EXECUTION_TARGET_UNAVAILABLE_ATTEMPTS && !ctx.cancel.is_cancelled().await => attempt += 1,
            answer => return answer,
        }
    }
}

/// ✅️ Holds `bytes` to the byte length and SHA-256 the lease declared for them; `label` names the file in the refusal.
pub fn verify_execution_target_bytes(label: &str, expected_sha256: &str, expected_byte_length: u64, bytes: &[u8]) -> Result<(), DirectoryClientError> {
    if bytes.len() as u64 != expected_byte_length {
        return Err(DirectoryClientError::Decode(format!("document execution-target {label} is {} bytes, the lease declares {expected_byte_length}", bytes.len())));
    }
    if semio_framework_hash::sha256_hex(bytes) != expected_sha256 {
        return Err(DirectoryClientError::Decode(format!("document execution-target {label} does not hash to the lease's SHA-256")));
    }
    Ok(())
}

/// 🗄️ How many component bytes the store keeps before it evicts the least recently used: every plugin of a full hub
/// catalog (≈ 800 MB over 33 plugins, hub 7800 p33) with room for a republished generation of each.
pub const EXECUTION_TARGET_STORE_COMPONENT_CAPACITY_BYTES: u64 = 2 * 1024 * 1024 * 1024;

/// ⌛️ How old a staged component write must be before the store takes it for one an interrupted process abandoned.
pub const EXECUTION_TARGET_STORE_STAGE_ABANDONED_SECS: u64 = 3_600;

/// 🗃️ The content-addressed on-disk store of execution-target files: `components/<sha256>.wasm` and
/// `descriptors/<sha256>.pack` (the canonical pack the hub serves) under one root. An entry is written whole through a rename and read back only
/// when it still verifies, so a torn or tampered file is fetched again rather than mounted. Components are bounded by
/// `component_capacity_bytes`: every verified read marks an entry used (its modification time), and admitting one evicts the
/// least recently used others until the rest fit.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionTargetModuleStore {
    root: std::path::PathBuf,
    component_capacity_bytes: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl ExecutionTargetModuleStore {
    /// 🗃️ A store rooted at `root`, bounded by [`EXECUTION_TARGET_STORE_COMPONENT_CAPACITY_BYTES`].
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self::with_component_capacity(root, EXECUTION_TARGET_STORE_COMPONENT_CAPACITY_BYTES)
    }

    /// 🗃️ A store rooted at `root` that keeps at most `component_capacity_bytes` of components.
    pub fn with_component_capacity(root: impl Into<std::path::PathBuf>, component_capacity_bytes: u64) -> Self {
        Self { root: root.into(), component_capacity_bytes }
    }

    /// 🗃️ The per-user store: `SEMIO_EXECUTION_TARGET_STORE_DIR`, else the platform's user cache directory
    /// (`XDG_CACHE_HOME`, `LOCALAPPDATA`, `~/Library/Caches` on macOS, `~/.cache`) under `semio/execution-targets`.
    pub fn for_user() -> Self {
        if let Some(root) = std::env::var_os("SEMIO_EXECUTION_TARGET_STORE_DIR").filter(|value| !value.is_empty()) {
            return Self::new(root);
        }
        let home = || std::env::var_os("HOME").map(std::path::PathBuf::from);
        let base = std::env::var_os("XDG_CACHE_HOME")
            .map(std::path::PathBuf::from)
            .or_else(|| std::env::var_os("LOCALAPPDATA").map(std::path::PathBuf::from))
            .or_else(|| if cfg!(target_os = "macos") { home().map(|home| home.join("Library").join("Caches")) } else { home().map(|home| home.join(".cache")) })
            .unwrap_or_else(std::env::temp_dir);
        Self::new(base.join("semio").join("execution-targets"))
    }

    /// 🧩️ Where a component with this SHA-256 lives.
    pub fn component_path(&self, sha256: &str) -> std::path::PathBuf {
        self.root.join("components").join(format!("{sha256}.wasm"))
    }

    /// 📜️ Where a descriptor with this SHA-256 lives.
    pub fn descriptor_path(&self, sha256: &str) -> std::path::PathBuf {
        self.root.join("descriptors").join(format!("{sha256}.pack"))
    }

    /// 🧩️ The component with exactly `expected`'s length and SHA-256, read and verified now and marked used — or `None`
    /// when the store holds no such bytes; an entry that no longer verifies is removed, so the next admission replaces it.
    pub fn component(&self, expected: &super::super::schema::DocumentExecutionTargetComponentV1) -> Option<Vec<u8>> {
        if !content_address_valid(&expected.sha256) {
            return None;
        }
        let path = self.component_path(&expected.sha256);
        let metadata = std::fs::metadata(&path).ok()?;
        let bytes = (metadata.len() == expected.byte_length).then(|| std::fs::read(&path).ok()).flatten();
        match bytes.filter(|bytes| verify_execution_target_bytes("store entry", &expected.sha256, expected.byte_length, bytes).is_ok()) {
            Some(bytes) => {
                let _ = std::fs::File::options().write(true).open(&path).and_then(|file| file.set_modified(std::time::SystemTime::now()));
                Some(bytes)
            }
            None => {
                let _ = std::fs::remove_file(&path);
                None
            }
        }
    }

    /// 💾️ Admits component `bytes` under their content address `sha256` and evicts the least recently used other components
    /// beyond the store's capacity. Entries are verified when they are read ([`Self::component`]), never trusted because
    /// they were written.
    pub fn admit_component(&self, sha256: &str, bytes: &[u8]) -> Result<(), DirectoryClientError> {
        if !content_address_valid(sha256) {
            return Err(DirectoryClientError::Decode("execution-target store address is not a SHA-256".into()));
        }
        let path = self.component_path(sha256);
        Self::store(&path, bytes)?;
        self.evict_components(&path);
        Ok(())
    }

    /// 🧹️ Removes the least recently used components other than `keep` until the rest fit the capacity, and every staged
    /// write an interrupted process left behind for longer than [`EXECUTION_TARGET_STORE_STAGE_ABANDONED_SECS`].
    fn evict_components(&self, keep: &std::path::Path) {
        let Ok(entries) = std::fs::read_dir(self.root.join("components")) else { return };
        let now = std::time::SystemTime::now();
        let mut components: Vec<(std::time::SystemTime, std::path::PathBuf, u64)> = Vec::new();
        for entry in entries.filter_map(Result::ok) {
            let (Some(name), Ok(metadata)) = (entry.file_name().to_str().map(str::to_string), entry.metadata()) else { continue };
            let modified = metadata.modified().unwrap_or(std::time::UNIX_EPOCH);
            if name.strip_suffix(".wasm").is_some_and(content_address_valid) {
                components.push((modified, entry.path(), metadata.len()));
            } else if name.split_once(".stage-").is_some_and(|(address, _)| content_address_valid(address)) && now.duration_since(modified).is_ok_and(|age| age.as_secs() > EXECUTION_TARGET_STORE_STAGE_ABANDONED_SECS) {
                let _ = std::fs::remove_file(entry.path());
            }
        }
        let mut total: u64 = components.iter().map(|(_, _, length)| *length).sum();
        components.sort();
        for (_, path, length) in components {
            if total <= self.component_capacity_bytes {
                break;
            }
            if path != keep && std::fs::remove_file(&path).is_ok() {
                total = total.saturating_sub(length);
            }
        }
    }

    /// 🔎️ Whether `path` holds exactly the declared bytes.
    fn holds(path: &std::path::Path, sha256: &str, byte_length: u64) -> bool {
        std::fs::metadata(path).is_ok_and(|metadata| metadata.len() == byte_length) && std::fs::read(path).is_ok_and(|bytes| verify_execution_target_bytes("store entry", sha256, byte_length, &bytes).is_ok())
    }

    /// 💾️ Writes verified bytes to `path` whole: a staged sibling renamed into place.
    fn store(path: &std::path::Path, bytes: &[u8]) -> Result<(), DirectoryClientError> {
        let io = |error: std::io::Error| DirectoryClientError::Decode(format!("execution-target store {}: {error}", path.display()));
        std::fs::create_dir_all(path.parent().ok_or_else(|| DirectoryClientError::Decode("execution-target store path has no parent".into()))?).map_err(io)?;
        let staged = path.with_extension(format!("stage-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos())));
        std::fs::write(&staged, bytes).map_err(io)?;
        std::fs::rename(&staged, path).map_err(|error| {
            let _ = std::fs::remove_file(&staged);
            io(error)
        })
    }
}

impl<T: DirectoryTransport> DirectoryClient<T> {
    /// 🧩️ Resolves the component `intent`'s document runs, by the serving catalog generation: the lease first,
    /// then — unless `local_component_sha256` already is the lease's component — a verified store entry, else
    /// the hub's own component and descriptor bytes, each verified against the lease and stored. `step` hears
    /// every step; `ctx.cancel` stops the resolution between requests. Nothing unverified is ever returned.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn resolve_execution_target_module(&self, ctx: &OperationContext, intent: &DocumentOpenIntentV1, local_component_sha256: Option<&str>, store: &ExecutionTargetModuleStore, mut step: impl FnMut(ExecutionTargetModuleStep)) -> Result<ResolvedExecutionTargetModule, DirectoryClientError> {
        step(ExecutionTargetModuleStep::Lease);
        let lease = self.document_execution_target_lease(ctx, intent).await?;
        self.resolve_execution_target_files(ctx, intent, lease, local_component_sha256, store, step).await
    }

    /// 🪪️ The document's execution-target lease, asked through a hub shortage ([`EXECUTION_TARGET_UNAVAILABLE_ATTEMPTS`]).
    pub async fn document_execution_target_lease(&self, ctx: &OperationContext, intent: &DocumentOpenIntentV1) -> Result<DocumentExecutionTargetLeaseFieldsV1, DirectoryClientError> {
        asked_through_shortage(ctx, || self.document_execution_target_manifest(ctx, intent)).await
    }

    /// 🧩️ The second half of [`Self::resolve_execution_target_module`] for a lease the caller already holds (a
    /// shell reads the plugin and app off it first): the local program when its content hash is the lease's,
    /// else a verified store entry, else the hub's own bytes, verified against `lease` and stored.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn resolve_execution_target_files(&self, ctx: &OperationContext, intent: &DocumentOpenIntentV1, lease: DocumentExecutionTargetLeaseFieldsV1, local_component_sha256: Option<&str>, store: &ExecutionTargetModuleStore, mut step: impl FnMut(ExecutionTargetModuleStep)) -> Result<ResolvedExecutionTargetModule, DirectoryClientError> {
        if local_component_sha256 == Some(lease.component.sha256.as_str()) {
            step(ExecutionTargetModuleStep::Verified);
            return Ok(ResolvedExecutionTargetModule { lease, source: ExecutionTargetModuleSource::Local, files: None });
        }
        let files = ExecutionTargetModuleFiles { component: store.component_path(&lease.component.sha256), descriptor: store.descriptor_path(&lease.descriptor.sha256) };
        let component_stored = store.component(&lease.component).is_some();
        let descriptor_stored = ExecutionTargetModuleStore::holds(&files.descriptor, &lease.descriptor.sha256, lease.descriptor.byte_length);
        if !component_stored {
            step(ExecutionTargetModuleStep::Component);
            let component = asked_through_shortage(ctx, || self.document_execution_target_component(ctx, intent)).await?;
            verify_execution_target_bytes("component", &lease.component.sha256, lease.component.byte_length, &component)?;
            store.admit_component(&lease.component.sha256, &component)?;
        }
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        if !descriptor_stored {
            step(ExecutionTargetModuleStep::Descriptor);
            let descriptor = asked_through_shortage(ctx, || self.document_execution_target_descriptor(ctx, intent)).await?;
            verify_execution_target_bytes("descriptor", &lease.descriptor.sha256, lease.descriptor.byte_length, &descriptor)?;
            ExecutionTargetModuleStore::store(&files.descriptor, &descriptor)?;
        }
        step(ExecutionTargetModuleStep::Verified);
        let source = if component_stored && descriptor_stored { ExecutionTargetModuleSource::Store } else { ExecutionTargetModuleSource::Hub };
        Ok(ResolvedExecutionTargetModule { lease, source, files: Some(files) })
    }
}

/// 🔡️ Whether `value` is a lowercase SHA-256 hex digest — the only name a store entry is ever given, so no address a hub
/// answer carries can name a path outside the store.
#[cfg(not(target_arch = "wasm32"))]
fn content_address_valid(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
