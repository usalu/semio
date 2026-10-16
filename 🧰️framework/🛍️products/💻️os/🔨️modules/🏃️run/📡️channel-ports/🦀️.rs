//! 📡️ Channel ports: the app channel host and media cache ports plus the file and in-memory blob stores.
//!
//! Holds the `🔖️AppChannelHost`, `🔖️MediaCache`, `🔖️BlobStore` regions of the module; the root re-exports its items.

use super::*;

//#region 🔖️AppChannelHost
/// 🔌️ The one seam `SpaceRunner` calls through — every concrete plugin host (native wasmtime,
/// browser worker, or an in-process fake for tests) implements this the same way, driving a node
/// through exactly the binary `AppCommand`/`AppFrame` channel a live UI speaks (see
/// `protocol_channel`) — a headless run is never a separate UI-mock API. `open` mints an opaque
/// handle the runner threads back on every later `exchange` call; `exchange` is a single batched
/// duplex round trip (`WasmPluginRuntime::exchange`'s native counterpart) that the caller now
/// `.await`s instead of blocking on (async-first rewrite, ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet terra-directory-and-run) — the guest
/// contract is still exactly one turn at a time per instance, so `SpaceRunner` never issues a second
/// `exchange` for the same `node` handle before the first one's future resolves (see `run`'s
/// strictly sequential per-node loop and `compute_node`'s single `.await` point); a caller that
/// wants concurrency must open distinct instances, never overlap two turns on one. `ctx` carries
/// `SpaceRunner`'s own `OperationContext` (`actor` set to `node`) so a real implementation can
/// honor cancellation/deadline/trace INSIDE one exchange too, not just between them — `compute_node`
/// itself only checks `ctx.cancel` between exchanges (see `RunError::Cancelled`'s own doc).
// 🔇️ `async fn` in a trait normally warns because it hides a `Send` bound a `dyn` caller might
// need — irrelevant here: `AppChannelHost` is used only as a generic bound (`SpaceRunner<H:
// AppChannelHost>`), never as `dyn AppChannelHost` (verified: `grep -rn "dyn AppChannelHost"`
// across the repo has zero hits), so no caller can ever demand a `Send` future regardless of what
// `open`/`exchange` do inside — R3's rule (Send obtained structurally, never a bound) applies to
// generic call sites too, not only `dyn` ones. MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet
// `run-kernel-wiring`): `WasmtimeNodeHost`'s own `open`/`exchange` now genuinely cross real OS
// threads inside one `.await` (`run_turn`'s `Kernel`-dispatched `ShardExecutor` turn) — the "single
// thread, one `block_on`" claim this comment used to make described the OLD stub bodies, not the
// real ones; the reasoning above (no `dyn`, so no forced `Send`) is what actually keeps this legal,
// and it always was.
#[allow(async_fn_in_trait)]
pub trait AppChannelHost {
    /// 🗺️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS (W2-A): `artifact_ref`
    /// (the node's own `WorkflowNode.artifact_ref`) is threaded through so a real host can populate
    /// its `InstanceDirectory` at instantiate-app time — a fake/test host is free to ignore it.
    async fn open(&mut self, plugin_id: &str, app_id: &str, artifact_ref: &str) -> Result<u32, RunError>;
    async fn exchange(&mut self, ctx: &OperationContext, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError>;
}
//#endregion 🔖️AppChannelHost

//#region 🔖️MediaCache
/// 📦️ Content-addressed cache of exported `Media` values, keyed by `MediaFingerprint`. Lets a
/// downstream dirty node import a clean upstream node's last output without re-instantiating that
/// upstream node at all — the whole point of fingerprint-based incrementality.
pub trait MediaCache {
    fn get(&self, fingerprint: &MediaFingerprint) -> Option<Media>;
    fn put(&mut self, fingerprint: &MediaFingerprint, media: &Media);
}

//#endregion 🔖️MediaCache

//#region 🔖️BlobStore
/// 💾️ Disk-backed `store::BlobStore` under `<studio>/blobs/<hash>` — backs both a `WasmPluginRuntime`'s
/// guest-side `write-blob`/`read-blob` host calls (via `WasmtimeNodeHost` registering it on every
/// runtime it loads) and `media_to_artifact`/`media_from_document`'s own resolution of a
/// `MediaPayload::Binary` value's bytes on the way on/off the wire.
pub struct FileBlobStore {
    pub(super) root: PathBuf,
}

impl FileBlobStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn blob_path(&self, hash: &str) -> PathBuf {
        self.root.join(hash)
    }
}

impl BlobStore for FileBlobStore {
    async fn put(&self, bytes: &[u8], media_type: &str) -> Result<store::BlobRef, store::VcsError> {
        let hash = framework_hash::hash_bytes(bytes);
        std::fs::create_dir_all(&self.root).map_err(|error| store::VcsError::Backbone(error.to_string()))?;
        std::fs::write(self.blob_path(&hash), bytes).map_err(|error| store::VcsError::Backbone(error.to_string()))?;
        Ok(store::BlobRef { hash, size: bytes.len() as u64, media_type: media_type.to_string() })
    }

    async fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, store::VcsError> {
        match std::fs::read(self.blob_path(hash)) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(store::VcsError::Backbone(error.to_string())),
        }
    }

    async fn has(&self, hash: &str) -> Result<bool, store::VcsError> {
        Ok(self.blob_path(hash).exists())
    }

    async fn delete(&self, hash: &str) -> Result<(), store::VcsError> {
        match std::fs::remove_file(self.blob_path(hash)) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(store::VcsError::Backbone(error.to_string())),
        }
    }
}

/// 🧠️ Process-local `store::BlobStore` — the test-only map cache counterpart for blob bytes, used
/// wherever a full `SpaceBundle` (and its `blobs/` dir) isn't in play, chiefly `SpaceRunner`'s own
/// `FakeHost`-based unit tests.
#[derive(Default)]
pub struct InMemoryBlobStore {
    pub(super) entries: Mutex<HashMap<String, (Vec<u8>, String)>>,
}

impl BlobStore for InMemoryBlobStore {
    async fn put(&self, bytes: &[u8], media_type: &str) -> Result<store::BlobRef, store::VcsError> {
        let hash = framework_hash::hash_bytes(bytes);
        let mut entries = self.entries.lock().map_err(|_| store::VcsError::Backbone("blob store lock poisoned".into()))?;
        entries.insert(hash.clone(), (bytes.to_vec(), media_type.to_string()));
        Ok(store::BlobRef { hash, size: bytes.len() as u64, media_type: media_type.to_string() })
    }

    async fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, store::VcsError> {
        let entries = self.entries.lock().map_err(|_| store::VcsError::Backbone("blob store lock poisoned".into()))?;
        Ok(entries.get(hash).map(|(bytes, _)| bytes.clone()))
    }

    async fn has(&self, hash: &str) -> Result<bool, store::VcsError> {
        let entries = self.entries.lock().map_err(|_| store::VcsError::Backbone("blob store lock poisoned".into()))?;
        Ok(entries.contains_key(hash))
    }

    async fn delete(&self, hash: &str) -> Result<(), store::VcsError> {
        let mut entries = self.entries.lock().map_err(|_| store::VcsError::Backbone("blob store lock poisoned".into()))?;
        entries.remove(hash);
        Ok(())
    }
}
//#endregion 🔖️BlobStore
