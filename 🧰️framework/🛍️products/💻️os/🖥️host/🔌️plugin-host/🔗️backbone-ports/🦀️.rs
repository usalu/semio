//! 🔗️ Backbone ports of the plugin host: the backbone port trait, its implementation, the port union and the document sync. See [`super`].

use super::*;

// #region host
//#region 🔖️Backbone
/// 🔌️ Byte-oriented studio persistence port — `read`/`write` carry `encode_backbone_payload`'s
/// binary pack+spr blob, never JSON. Every implementor today (`MemoryBackbonePort`,
/// `LocalStorageBackbonePort`, and the `file://`/`folder://` host ports opened by
/// `open_file_space_backbone`/`open_folder_space_backbone`) is bridged from the underlying
/// string-typed `store::os_store::backbone::BackbonePort` via the blanket impl below — see its doc for why.
pub trait OsBackbonePort: Send + Sync {
    fn read(&self, uri: &str) -> Result<Vec<u8>, VcsError>;
    fn write(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError>;
}

/// 🌉️ `store::os_store::backbone::BackbonePort` is the shared string-typed transport for every document kind
/// across the whole kernel (localStorage, in-memory, host file/folder ports) — changing its own
/// signature to bytes is out of scope here. Base64 is the bridge: an empty payload maps to an
/// empty string both ways (preserving `delete_os_space`'s tombstone-write semantics), and every
/// non-empty payload round-trips byte-for-byte through the encoding.
impl<T: store::os_store::backbone::BackbonePort + Send + Sync> OsBackbonePort for T {
    fn read(&self, uri: &str) -> Result<Vec<u8>, VcsError> {
        let text = resolve_kernel_future(store::os_store::backbone::BackbonePort::read(self, uri))?;
        if text.is_empty() {
            return Ok(Vec::new());
        }
        base64_codec::base64_standard_decode(text).map_err(|error| VcsError::Deserialize(error.to_string()))
    }

    fn write(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError> {
        if payload.is_empty() {
            return resolve_kernel_future(store::os_store::backbone::BackbonePort::write(self, uri, ""));
        }
        resolve_kernel_future(store::os_store::backbone::BackbonePort::write(self, uri, &base64_codec::base64_standard_encode(payload)))
    }
}

/// 🧬️ Enum dispatch over every `OsBackbonePort` implementor (O1 — dyn dispatch dropped in
/// favor of enum/match dispatch, mirroring `🏪️store`'s `BackbonePorts` shape exactly —
/// `📓️terra-store-dedyn-report.md`). `Store` covers every `store::os_store::backbone::BackbonePort`-shaped transport
/// (in-memory, localStorage) via the blanket bridge above; `Space` covers the host-only
/// file/folder-backed `backbone::SpaceBackbonePort`. These are the only two implementors in the
/// tree (verified: one blanket impl over `store::os_store::backbone::BackbonePort`, one direct impl on
/// `backbone::SpaceBackbonePort` — `📓️terra-os-backbone-report.md`).
pub enum OsBackbonePorts {
    Store(store::os_store::backbone::BackbonePorts),
    Space(crate::backbone::SpaceBackbonePort),
}

impl OsBackbonePort for OsBackbonePorts {
    fn read(&self, uri: &str) -> Result<Vec<u8>, VcsError> {
        match self {
            Self::Store(port) => OsBackbonePort::read(port, uri),
            Self::Space(port) => OsBackbonePort::read(port, uri),
        }
    }

    fn write(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError> {
        match self {
            Self::Store(port) => OsBackbonePort::write(port, uri, payload),
            Self::Space(port) => OsBackbonePort::write(port, uri, payload),
        }
    }
}

/// 🌉️ Writes any `BackboneDocument<P, Op>` to `uri`, stamping its own `backbone` ref first —
/// shared by every catalog write path below (space manifests, collections).
pub(super) fn sync_backbone_document<P, Op>(document: &BackboneDocument<P, Op>, backbone_uri: &str, port: &Arc<OsBackbonePorts>) -> Result<(), VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    Op: Clone + ToValue + FromValue + Mutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    let mut synced = document.clone();
    synced.backbone = Some(resolve_kernel_future(document_backbone_ref(backbone_uri)));
    let result = encode_backbone_payload(&synced).and_then(|payload| port.write(backbone_uri, &payload));
    close_backbone_envelope(take_backbone_envelope(synced).1)?;
    result
}
//#endregion 🔖️Backbone
// #endregion host
