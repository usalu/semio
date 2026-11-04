//! 🗄️ Trusted host-side backbone ports for local studio storage — reads/writes the raw persisted
//! json directly, bypassing the duplex `Backbone` channel since there is no other process here.
// #region backbone

use crate::host::OsBackbonePort;
#[cfg(not(target_arch = "wasm32"))]
use crate::host::OsBackbonePorts;
#[cfg(not(target_arch = "wasm32"))]
use crate::space;
#[cfg(not(target_arch = "wasm32"))]
use crate::store_sync::FolderEventLogStorage;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;
use store::os_store::backbone::MemoryBackbonePort;
use vcs::VcsError;

/// 🗂️ Conventional single-document id used inside a folder-backed studio backbone — a studio
/// folder holds exactly one os document at its root (app documents get their own document ids once
/// {@link OsArtifactRef} routes them through `framework/sync`'s multi-document `ArtifactHost`).
#[cfg(not(target_arch = "wasm32"))]
const SPACE_FOLDER_DOCUMENT_ID: &str = "studio";

enum SpacePortKind {
    /// 🗃️ One document stored as exactly the chosen `file://` path: the whole backbone payload (name, pack and spr) in one blob,
    /// replaced atomically through a sibling temporary file, never as `<stem>.<ext>.*.semio` companions.
    #[cfg(not(target_arch = "wasm32"))]
    File { uri: String, path: std::path::PathBuf },
    #[cfg(not(target_arch = "wasm32"))]
    Folder(String, FolderEventLogStorage),
}

pub struct SpaceBackbonePort {
    kind: Option<SpacePortKind>,
    memory: MemoryBackbonePort,
}

impl SpaceBackbonePort {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn file(file_path: &str) -> Result<Self, VcsError> {
        let uri = format!("file://{file_path}");
        Ok(Self { kind: Some(SpacePortKind::File { uri, path: std::path::PathBuf::from(file_path) }), memory: crate::registry::resolve_kernel_future(MemoryBackbonePort::new()) })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn folder(folder_path: &str) -> Result<Self, VcsError> {
        let uri = format!("folder://{folder_path}");
        Ok(Self {
            kind: Some(SpacePortKind::Folder(uri, FolderEventLogStorage::new(std::path::PathBuf::from(folder_path)))),
            memory: crate::registry::resolve_kernel_future(MemoryBackbonePort::new()),
        })
    }
}

/// 🌉️ `read`/`write`'s `payload` is the same `encode_os_space_payload` blob every
/// `OsBackbonePort` implementor carries: a `name` byte blob wrapping a nested `pack`+`spr` pair
/// (`store::os_store::serialization::encode_document_pack_bytes`, twice). The file/folder storage backends below have no
/// slot for `name` (they only ever persisted `pack`+`spr`, or before this crate's pack/dsl rollout
/// bare JSON) — that half of the payload is dropped on write and reconstituted empty on read. A file space keeps the
/// whole payload, name included, in the chosen file itself.
impl OsBackbonePort for SpaceBackbonePort {
    fn read(&self, uri: &str) -> Result<Vec<u8>, VcsError> {
        if let Some(kind) = &self.kind {
            match kind {
                #[cfg(not(target_arch = "wasm32"))]
                SpacePortKind::File { uri: file_uri, path } if uri == file_uri => {
                    return std::fs::read(path).map_err(|error| if error.kind() == std::io::ErrorKind::NotFound { VcsError::Backbone(format!("missing backbone file {uri}")) } else { VcsError::Backbone(format!("backbone file {uri}: {error}")) });
                }
                #[cfg(not(target_arch = "wasm32"))]
                SpacePortKind::Folder(folder_uri, storage) if uri == folder_uri => {
                    let (pack, spr) = crate::registry::resolve_kernel_future(storage.read(SPACE_FOLDER_DOCUMENT_ID))?.ok_or_else(|| VcsError::Backbone(format!("missing backbone file {uri}")))?;
                    let inner = crate::registry::resolve_kernel_future(store::os_store::serialization::encode_document_pack_bytes(&pack, &spr));
                    return Ok(crate::registry::resolve_kernel_future(store::os_store::serialization::encode_document_pack_bytes(&[], &inner)));
                }
                _ => {}
            }
        }
        self.read_via_memory(uri)
    }

    fn write(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError> {
        if let Some(kind) = &self.kind {
            match kind {
                #[cfg(not(target_arch = "wasm32"))]
                SpacePortKind::File { uri: file_uri, path } if uri == file_uri => {
                    if payload.is_empty() {
                        return Err(VcsError::Backbone(format!("backbone file {uri} is never emptied")));
                    }
                    decode_os_space_pack_payload(payload)?;
                    let directory = path.parent().filter(|parent| !parent.as_os_str().is_empty()).unwrap_or_else(|| std::path::Path::new("."));
                    let temporary = directory.join(format!(".{}.{}.tmp", path.file_name().and_then(|name| name.to_str()).unwrap_or("document"), std::process::id()));
                    let written = std::fs::write(&temporary, payload).and_then(|()| std::fs::rename(&temporary, path));
                    if written.is_err() {
                        let _ = std::fs::remove_file(&temporary);
                    }
                    return written.map_err(|error| VcsError::Backbone(format!("backbone file {uri}: {error}")));
                }
                #[cfg(not(target_arch = "wasm32"))]
                SpacePortKind::Folder(folder_uri, storage) if uri == folder_uri => {
                    let (pack, spr) = decode_os_space_pack_payload(payload)?;
                    return crate::registry::resolve_kernel_future(storage.write(SPACE_FOLDER_DOCUMENT_ID, space::S_SPACE_SCHEMA, &pack, &spr));
                }
                _ => {}
            }
        }
        self.write_via_memory(uri, payload)
    }
}

/// 🔓️ Strips `encode_os_space_payload`'s `name` wrapper, returning the inner `pack`+`spr`
/// pair — the half of the payload `SpaceBackbonePort`'s file/folder storage actually persists.
#[cfg(not(target_arch = "wasm32"))]
fn decode_os_space_pack_payload(payload: &[u8]) -> Result<(Vec<u8>, Vec<u8>), VcsError> {
    let (_name, inner) = crate::registry::resolve_kernel_future(store::os_store::serialization::decode_document_pack_bytes(payload))?;
    crate::registry::resolve_kernel_future(store::os_store::serialization::decode_document_pack_bytes(&inner))
}

impl SpaceBackbonePort {
    /// 🌉️ `self.memory` is a plain `store::os_store::backbone::BackbonePort` (string-typed) fallback for any uri
    /// that isn't this port's own configured file/folder uri (e.g. the space catalog uri) — bridge
    /// bytes↔string via base64, same as the blanket `impl<T: store::os_store::backbone::BackbonePort> OsBackbonePort`.
    fn read_via_memory(&self, uri: &str) -> Result<Vec<u8>, VcsError> {
        let text = crate::registry::resolve_kernel_future(store::os_store::backbone::BackbonePort::read(&self.memory, uri))?;
        if text.is_empty() {
            return Ok(Vec::new());
        }
        base64_codec::base64_standard_decode(text).map_err(|error| VcsError::Deserialize(error.to_string()))
    }

    fn write_via_memory(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError> {
        if payload.is_empty() {
            return crate::registry::resolve_kernel_future(store::os_store::backbone::BackbonePort::write(&self.memory, uri, ""));
        }
        crate::registry::resolve_kernel_future(store::os_store::backbone::BackbonePort::write(&self.memory, uri, &base64_codec::base64_standard_encode(payload)))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn open_folder_space_backbone(folder_path: &str) -> Result<Arc<OsBackbonePorts>, VcsError> {
    Ok(Arc::new(OsBackbonePorts::space(SpaceBackbonePort::folder(folder_path)?)))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn open_file_space_backbone(file_path: &str) -> Result<Arc<OsBackbonePorts>, VcsError> {
    Ok(Arc::new(OsBackbonePorts::space(SpaceBackbonePort::file(file_path)?)))
}
// #endregion backbone
