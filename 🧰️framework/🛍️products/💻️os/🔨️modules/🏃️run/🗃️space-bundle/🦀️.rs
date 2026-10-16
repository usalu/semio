//! 🗃️ Space bundle: the studio bundle: workflow, packages, documents, configs and the node lookups a run reads.
//!
//! Holds the `🔖️SpaceBundle` region of the module; the root re-exports its items.

use super::*;

//#region 🔖️SpaceBundle
/// 📁️ The canonical on-disk shape of a space (`## Design rulings` -> `On-disk space layout` in
/// `.claude/plans/the-final-goal-for-jolly-spindle.md`, rewritten here for W4's storage-layout task —
/// superseding the pre-W4 flat `space.os.pack`/`<artifact_ref>.pack` layout, whose filenames were
/// stale leftovers from the dissolved `OsDocument`):
/// - root: `space.space.pack`+`space.space.spr` — the space manifest slot (the `OsDocument` VCS
///   envelope's binary pack+dsl form — see `semio_framework_os::encode_os_space_payload`; today this
///   still carries a `workflow::WorkflowSnapshot`/`WorkflowMutation` pair per the os-core dissolve's
///   `## The inversion`, not yet a real `space::SpaceSnapshot` manifest — wiring the ROOT slot's
///   actual decoded type to `SpaceSnapshot` is later-wave work, this rewrite is the path/filename
///   convention only, see `read_space_document`/`write_space_document`),
/// - `collections/<collection id>.collection.pack|.spr` — one `space::CollectionSnapshot` per
///   collection (`collection_pack_path`/`collection_spr_path`/`read_collection`/`write_collection` —
///   not yet called by anything in this crate; collections aren't wired into `SpaceRunner` until a
///   later wave, this is the reserved path convention those callers will use),
/// - `artifacts/<artifact id>.pack|.spr` — one pair per document artifact, reusing the exact same
///   "id IS the address" convention `space::artifact_backbone_uri(space_id, artifact_id)` uses one
///   level up (a bundle has no separate `space_id` of its own — the bundle root already IS one space,
///   so there is nothing to reuse `artifact_backbone_uri` itself for beyond this shared convention;
///   see `artifact_pack_path`/`artifact_spr_path`). A workflow node's `artifact_ref`/`config_ref`
///   (`workflow::workflow_node_for_app`, e.g. `"artifacts/<node id>"`/`"config/<node id>"`) is passed
///   straight through as `artifact_id` — two distinct artifact ids per node, so they never collide
///   under `artifacts/`, and nothing on the `workflow` crate side of this addressing scheme needs to
///   change to fit it (`artifact_pack_path`/`config_pack_path` below are thin `artifact_pack_path`
///   aliases kept for callers' existing names),
/// - `blobs/` — content-addressed, space-level (backing a `MediaPayload::Binary` value's bytes — see
///   `FileBlobStore`; unchanged from the pre-W4 layout, already canonical),
/// - `cache/media/` — cross-run shared media-fingerprint cache (renamed from `run/media/`),
/// - `runs/<run id>.run.pack|.spr` — the `semio_framework_artifact_workflow_run::RunArtifact` VCS envelope itself (W5 Lane A);
///   `runs/<run id>/nodes/<node id>.document.pack|.spr`/`.config.pack|.spr` — that run's OWN mirrored
///   copy of each node's post-import document/config bytes (`run_node_artifact_pack_path`/
///   `run_node_config_pack_path` below) — deliberately NOT under `artifacts/`: a run never writes
///   through the same path a node's SOURCE document/config was read from (see this crate's module doc,
///   "non-destructive rework" — this replaces the old `run/state.json`-backed `RunState`, deleted by
///   this rework). This wave uses a single canonical `run id` (`"default"`, minted by `bin.rs`) — one
///   active run slot per bundle; a real multi-run history/listing is later-wave `space::DraftCatalog`
///   integration work.
///
/// Ids only — no paths inside the space document itself — so the bundle stays relocatable and syncs
/// the same way over `file://` or a semio_hub backbone.
pub struct SpaceBundle {
    pub(super) root: PathBuf,
}

impl SpaceBundle {
    pub fn open(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn space_artifact_pack_path(&self) -> PathBuf {
        self.root.join("space.space.pack")
    }

    pub fn space_artifact_spr_path(&self) -> PathBuf {
        self.root.join("space.space.spr")
    }

    /// 🗂️ Canonical collection path: `collections/<collection id>.collection.pack`.
    pub fn collection_pack_path(&self, collection_id: &str) -> PathBuf {
        self.root.join("collections").join(format!("{collection_id}.collection.pack"))
    }

    pub fn collection_spr_path(&self, collection_id: &str) -> PathBuf {
        self.root.join("collections").join(format!("{collection_id}.collection.spr"))
    }

    /// 🗂️ Canonical artifact path: `artifacts/<artifact_id>.pack` — see this region's own doc comment
    /// for why `artifact_id` is passed straight through unmodified (it may itself contain `/`, e.g. a
    /// workflow node's `artifact_ref`/`config_ref`) rather than requiring a flat basename.
    pub fn artifact_pack_path(&self, artifact_id: &str) -> PathBuf {
        self.root.join("artifacts").join(format!("{artifact_id}.pack"))
    }

    pub fn artifact_spr_path(&self, artifact_id: &str) -> PathBuf {
        self.root.join("artifacts").join(format!("{artifact_id}.spr"))
    }

    /// 🔖️ Config counterpart of `artifact_pack_path` — same alias-over-`artifact_pack_path`
    /// convention (a node's `config_ref` is already e.g. `config/<node id>`).
    pub fn config_pack_path(&self, config_ref: &str) -> PathBuf {
        self.artifact_pack_path(config_ref)
    }

    pub fn config_spr_path(&self, config_ref: &str) -> PathBuf {
        self.artifact_spr_path(config_ref)
    }

    /// 🗂️ Canonical run-document path: `runs/<run id>.run.pack`.
    pub fn run_artifact_pack_path(&self, run_id: &str) -> PathBuf {
        self.root.join("runs").join(format!("{run_id}.run.pack"))
    }

    pub fn run_artifact_spr_path(&self, run_id: &str) -> PathBuf {
        self.root.join("runs").join(format!("{run_id}.run.spr"))
    }

    /// 🗂️ Canonical run-owned node-document path: `runs/<run id>/nodes/<node id>.document.pack` — see
    /// this region's own doc comment for why this is a distinct tree from `artifacts/`.
    pub fn run_node_artifact_pack_path(&self, run_id: &str, node_id: &str) -> PathBuf {
        self.root.join("runs").join(run_id).join("nodes").join(format!("{node_id}.document.pack"))
    }

    pub fn run_node_artifact_spr_path(&self, run_id: &str, node_id: &str) -> PathBuf {
        self.root.join("runs").join(run_id).join("nodes").join(format!("{node_id}.document.spr"))
    }

    pub fn run_node_config_pack_path(&self, run_id: &str, node_id: &str) -> PathBuf {
        self.root.join("runs").join(run_id).join("nodes").join(format!("{node_id}.config.pack"))
    }

    pub fn run_node_config_spr_path(&self, run_id: &str, node_id: &str) -> PathBuf {
        self.root.join("runs").join(run_id).join("nodes").join(format!("{node_id}.config.spr"))
    }

    pub fn media_cache_dir(&self) -> PathBuf {
        self.root.join("cache").join("media")
    }

    pub fn blobs_dir(&self) -> PathBuf {
        self.root.join("blobs")
    }

    /// 📦️ Reads the studio's pack+spr bytes, matching `read_artifact`/`read_config`'s "empty
    /// spr if never persisted" convention (a bare pack with no history is a valid fresh studio).
    pub fn read_space_document(&self) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.space_artifact_pack_path();
        let pack = std::fs::read(&pack_path).map_err(|source| RunError::Io { path: pack_path, source })?;
        let spr_path = self.space_artifact_spr_path();
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    pub fn write_space_document(&self, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.space_artifact_pack_path();
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.space_artifact_spr_path(), spr).map_err(|source| RunError::Io { path: self.space_artifact_spr_path(), source })
    }

    /// 📦️ Reads one node's document pack+spr bytes, `(Vec::new(), Vec::new())` if never persisted.
    pub fn read_artifact(&self, artifact_ref: &str) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.artifact_pack_path(artifact_ref);
        let pack = match std::fs::read(&pack_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok((Vec::new(), Vec::new())),
            Err(source) => return Err(RunError::Io { path: pack_path, source }),
        };
        let spr_path = self.artifact_spr_path(artifact_ref);
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    pub fn write_artifact(&self, artifact_ref: &str, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.artifact_pack_path(artifact_ref);
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.artifact_spr_path(artifact_ref), spr).map_err(|source| RunError::Io { path: self.artifact_spr_path(artifact_ref), source })
    }

    /// 📦️ Reads one node's config pack+spr bytes — mirrors `read_artifact` exactly (same
    /// "never persisted" fallback, same error shape).
    pub fn read_config(&self, config_ref: &str) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.config_pack_path(config_ref);
        let pack = match std::fs::read(&pack_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok((Vec::new(), Vec::new())),
            Err(source) => return Err(RunError::Io { path: pack_path, source }),
        };
        let spr_path = self.config_spr_path(config_ref);
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    /// 📦️ Writes one node's config pack+spr bytes — mirrors `write_artifact` exactly
    /// (directory-created-if-missing, same error shape).
    pub fn write_config(&self, config_ref: &str, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.config_pack_path(config_ref);
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.config_spr_path(config_ref), spr).map_err(|source| RunError::Io { path: self.config_spr_path(config_ref), source })
    }

    /// 📦️ Reads one collection's pack+spr bytes — mirrors `read_artifact`'s "never persisted"
    /// fallback exactly. 🚧️ Not yet called from this crate's own runner/CLI flow (collections aren't
    /// wired into `SpaceRunner` until a later wave) — this is the reserved canonical path a future
    /// caller writes/reads through, kept symmetric with `read_artifact`/`write_artifact` on purpose.
    pub fn read_collection(&self, collection_id: &str) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.collection_pack_path(collection_id);
        let pack = match std::fs::read(&pack_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok((Vec::new(), Vec::new())),
            Err(source) => return Err(RunError::Io { path: pack_path, source }),
        };
        let spr_path = self.collection_spr_path(collection_id);
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    pub fn write_collection(&self, collection_id: &str, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.collection_pack_path(collection_id);
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.collection_spr_path(collection_id), spr).map_err(|source| RunError::Io { path: self.collection_spr_path(collection_id), source })
    }

    /// 📦️ Reads a run's own `semio_framework_artifact_workflow_run::RunArtifact` pack+spr bytes, `(Vec::new(), Vec::new())`
    /// if never persisted (no prior run of this id) — mirrors `read_artifact`'s fallback exactly.
    pub fn read_run_document(&self, run_id: &str) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.run_artifact_pack_path(run_id);
        let pack = match std::fs::read(&pack_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok((Vec::new(), Vec::new())),
            Err(source) => return Err(RunError::Io { path: pack_path, source }),
        };
        let spr_path = self.run_artifact_spr_path(run_id);
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    pub fn write_run_document(&self, run_id: &str, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.run_artifact_pack_path(run_id);
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.run_artifact_spr_path(run_id), spr).map_err(|source| RunError::Io { path: self.run_artifact_spr_path(run_id), source })
    }

    /// 📦️ Persists every `RunSink::node_artifacts`/`node_configs` entry from one completed run
    /// under `runs/<run id>/nodes/` — the only place this crate ever writes a node's post-import
    /// document/config bytes to disk (never back over `artifacts/<artifact_ref>`/`artifacts/<config_ref>`).
    pub fn write_run_nodes(&self, run_id: &str, sink: &RunSink) -> Result<(), RunError> {
        for (node_id, (pack, spr)) in &sink.node_artifacts {
            let pack_path = self.run_node_artifact_pack_path(run_id, node_id);
            if let Some(parent) = pack_path.parent() {
                std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
            }
            std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
            std::fs::write(self.run_node_artifact_spr_path(run_id, node_id), spr).map_err(|source| RunError::Io { path: self.run_node_artifact_spr_path(run_id, node_id), source })?;
        }
        for (node_id, (pack, spr)) in &sink.node_configs {
            let pack_path = self.run_node_config_pack_path(run_id, node_id);
            if let Some(parent) = pack_path.parent() {
                std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
            }
            std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
            std::fs::write(self.run_node_config_spr_path(run_id, node_id), spr).map_err(|source| RunError::Io { path: self.run_node_config_spr_path(run_id, node_id), source })?;
        }
        Ok(())
    }

    pub fn blob_store(&self) -> FileBlobStore {
        FileBlobStore::new(self.blobs_dir())
    }
}
//#endregion 🔖️SpaceBundle
