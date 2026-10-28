//! 🪪️ Wire entry records: the document identity, media-export handle and state, child pack entries and window-config pack entry.

use super::malformed;

/// 🪪️ A bounded document root identity read independently from authored renderer props.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppDocumentIdentity {
    pub app_instance_id: u32,
    #[serde(with = "identity_revision")]
    pub base_revision: u64,
    #[serde(deserialize_with = "deserialize_identity_document_id")]
    pub parent_document_id: Option<String>,
}

mod identity_revision {
    pub fn serialize<S:serde::Serializer>(revision:&u64,serializer:S)->Result<S::Ok,S::Error>{serializer.serialize_str(&revision.to_string())}
    pub fn deserialize<'de,D:serde::Deserializer<'de>>(deserializer:D)->Result<u64,D::Error>{let value=<String as serde::Deserialize>::deserialize(deserializer)?;let revision=value.parse::<u64>().map_err(serde::de::Error::custom)?;if revision.to_string()!=value{return Err(serde::de::Error::custom("document revision must be canonical u64 decimal"))}Ok(revision)}
}

fn deserialize_identity_document_id<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    serde::Deserialize::deserialize(deserializer)
}
//#region 🔖️MediaExportWire
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MediaExportHandleWire {
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub operation_id: u64,
    pub base_revision: u64,
    pub generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaExportStateWire {
    Running,
    Complete,
    Cancelled,
    Failed,
}

impl MediaExportStateWire {
    pub(super) fn wire(self) -> u8 {
        match self {
            Self::Running => 0,
            Self::Complete => 1,
            Self::Cancelled => 2,
            Self::Failed => 3,
        }
    }

    pub(super) fn from_wire(value: u8) -> Result<Self, protocol::ProtocolError> {
        match value {
            0 => Ok(Self::Running),
            1 => Ok(Self::Complete),
            2 => Ok(Self::Cancelled),
            3 => Ok(Self::Failed),
            _ => Err(malformed("media export state", 0, "unknown state")),
        }
    }
}
//#endregion 🔖️MediaExportWire
//#region 🔖️ChildPackEntry
/// 🧸️ One owned child's whole persisted envelope, as it travels between host and guest.
/// Composed children are their OWN envelopes with their own `ArtifactVcs` history, so a composing
/// document's own pack+spr (`AppFrame::Document`) is not sufficient to save or restore it — its children
/// would exist only until the process ended. `AppCommand::LoadChildren`/`AppFrame::Children` carry
/// exactly these, keyed the way the parent's `ArtifactChild` handles name them.
#[derive(Clone, Debug, PartialEq)]
pub struct ChildPackEntry {
    pub slot: String,
    pub child_id: String,
    /// 🎯️ `ArtifactDialect` as its `<kind>@<standard>/<subset>` wire string — the guest needs it to
    /// pick the right `ChildStoreFactory`, and it is not recoverable from the pack bytes alone.
    pub dialect: String,
    /// 📦️ The child's full envelope pack (`encode_document_pack_bytes` framing: pack + spr).
    pub envelope_pack: Vec<u8>,
    /// 🧭️ The owner path of the member that owns this entry (`slot/childId[/slot/childId]*`, `%25` / `%2F` escaped); empty for the document itself. CHANNEL_VERSION 23 wire addition.
    pub owner: String,
}

/// 🪆️ One owned child's CURRENT content as its head snapshot pack (`ArtifactPack::encode_pack`), nested members included — what a
/// reader that composes on read takes beside the parent (design §20.15), e.g. an inference request's `child:<slot>/<childId>` dependency.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildHeadPackEntry {
    pub slot: String,
    pub child_id: String,
    /// 🎯️ `ArtifactDialect` as its `<kind>@<standard>/<subset>` wire string.
    pub dialect: String,
    /// 📦️ The child's head snapshot pack.
    pub head_pack: Vec<u8>,
    /// 🧭️ The owner path of the member that owns this entry, as [`ChildPackEntry::owner`]. CHANNEL_VERSION 23 wire addition.
    pub owner: String,
}
//#endregion 🔖️ChildPackEntry
//#region 🔖️WindowConfigPackEntry
/// 🪟️ One concrete window's persisted-local configuration envelope.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowConfigPackEntry {
    pub window_id: String,
    pub window_kind_id: String,
    pub envelope_pack: Vec<u8>,
}
//#endregion 🔖️WindowConfigPackEntry
