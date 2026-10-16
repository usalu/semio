//! 🗄️ Trusted host-side backbone ports for local studio storage — reads/writes the raw persisted
//! json directly, bypassing the duplex `Backbone` channel since there is no other process here.
// #region backbone

use crate::host::OsBackbonePort;
#[cfg(not(target_arch = "wasm32"))]
use crate::host::OsBackbonePorts;
#[cfg(not(target_arch = "wasm32"))]
use crate::space;
#[cfg(not(target_arch = "wasm32"))]
use crate::store_sync::{FolderEventLogStorage, FolderTextStorage};
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
    /// 🗃️ A single document's pack blob addressed by an arbitrary `file://` path —
    /// `<folder>/<document_id>.<extension>.pack` (authoritative) + `.ops` + a DSL mirror, via
    /// `FolderTextStorage::write_pack`/`read_pack` and the typed `store::os_store::text_format::parse_document_pack`/
    /// `print_document_pack::<OsSnapshot, OsMutation>` (this crate is fully typed, no
    /// `store::os_store::serialization::ArtifactCodec` indirection needed).
    #[cfg(not(target_arch = "wasm32"))]
    File { uri: String, storage: FolderTextStorage, document_id: String, extension: String },
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
        let path = std::path::Path::new(file_path);
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("txt").to_string();
        let document_id = path.file_stem().and_then(|stem| stem.to_str()).unwrap_or("document").to_string();
        let folder = path.parent().map_or_else(|| std::path::PathBuf::from("."), std::path::Path::to_path_buf);
        Ok(Self {
            kind: Some(SpacePortKind::File { uri, storage: crate::registry::resolve_kernel_future(FolderTextStorage::new(folder)), document_id, extension }),
            memory: crate::registry::resolve_kernel_future(MemoryBackbonePort::new()),
        })
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
/// bare JSON) — that half of the payload is dropped on write and reconstituted empty on read,
/// same loss the previous JSON-through-`OsEnvelope` bridge already had (`OsEnvelope` itself has no
/// `name` field), not a regression.
impl OsBackbonePort for SpaceBackbonePort {
    fn read(&self, uri: &str) -> Result<Vec<u8>, VcsError> {
        if let Some(kind) = &self.kind {
            match kind {
                #[cfg(not(target_arch = "wasm32"))]
                SpacePortKind::File { uri: file_uri, storage, document_id, extension } if uri == file_uri => {
                    let (pack, spr) = if let Some(pack_files) = crate::registry::resolve_kernel_future(storage.read_pack(document_id, extension))? {
                        (pack_files.pack, pack_files.spr)
                    } else {
                        match crate::registry::resolve_kernel_future(storage.read(document_id, extension))? {
                            Some(text_files) => {
                                let snapshot = <space::SpaceSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text_files.dsl).map_err(|error| VcsError::Deserialize(error.message))?;
                                let document: crate::host::OsSpaceDocument = crate::host::create_backbone_document(space::S_SPACE_SCHEMA, document_id, "", snapshot);
                                let pack_files = crate::host::export_backbone_pack(&document)?;
                                (pack_files.pack, pack_files.spr)
                            }
                            None => return Err(VcsError::Backbone(format!("missing backbone file {uri}"))),
                        }
                    };
                    let inner = crate::registry::resolve_kernel_future(store::os_store::serialization::encode_document_pack_bytes(&pack, &spr));
                    return Ok(crate::registry::resolve_kernel_future(store::os_store::serialization::encode_document_pack_bytes(&[], &inner)));
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
                SpacePortKind::File { uri: file_uri, storage, document_id, extension } if uri == file_uri => {
                    let (pack, spr) = decode_os_space_pack_payload(payload)?;
                    let parsed: store::os_store::text_format::ParsedDocumentText<space::SpaceSnapshot, space::SpaceMutation> = crate::registry::resolve_kernel_future(store::os_store::text_format::parse_document_pack(&pack, &spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                    let dsl_mirror = pack::snapshot::ArtifactDsl::print_dsl(&parsed.into_snapshot());
                    let pack_files = store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() };
                    return crate::registry::resolve_kernel_future(storage.write_pack(document_id, extension, &pack_files, &dsl_mirror));
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
    Ok(Arc::new(OsBackbonePorts::Space(SpaceBackbonePort::folder(folder_path)?)))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn open_file_space_backbone(file_path: &str) -> Result<Arc<OsBackbonePorts>, VcsError> {
    Ok(Arc::new(OsBackbonePorts::Space(SpaceBackbonePort::file(file_path)?)))
}
// #endregion backbone
