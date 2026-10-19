//! 🗄️ Persistence binding, actor config and messages: the persistence data class and binding, the actor config, the document key and the actor message set. See [`super`].

use super::*;

//#region 🔖️Protocol
/// 🗃️ A durable place a document synchronizes with. A document may bind to several at once
/// (folder-only, semio_hub-only, or both); the actor treats each as an independent peer.
/// 🗃️ Durability × sharing class for persistence bindings and replication lanes.
/// Preview/presence lanes are always `EphemeralShared` and must never be mistaken for durable WAL state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub enum PersistenceDataClass {
    PersistedLocalOnly,
    PersistedShared,
    EphemeralLocalOnly,
    EphemeralShared,
}

impl PersistenceDataClass {
    pub const fn is_durable(self) -> bool {
        matches!(self, Self::PersistedLocalOnly | Self::PersistedShared)
    }

    pub const fn is_shared(self) -> bool {
        matches!(self, Self::PersistedShared | Self::EphemeralShared)
    }

    pub const fn allows_share(self) -> bool {
        matches!(self, Self::PersistedShared)
    }

    pub const fn allows_collaboration(self) -> bool {
        matches!(self, Self::PersistedShared)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersistedLocalOnly => "persistedLocalOnly",
            Self::PersistedShared => "persistedShared",
            Self::EphemeralLocalOnly => "ephemeralLocalOnly",
            Self::EphemeralShared => "ephemeralShared",
        }
    }
}

/// 🛤️ Classifies a wire lane. Preview and presence are broadcast-only (never WAL).
pub fn wire_lane_data_class(lane: &str) -> PersistenceDataClass {
    match lane {
        "preview" | "presence" => PersistenceDataClass::EphemeralShared,
        "command" => PersistenceDataClass::PersistedShared,
        _ => PersistenceDataClass::EphemeralLocalOnly,
    }
}

/// 🗃️ Resolves every durable peer, independently of order; empty bindings are ephemeral local-only.
pub fn bindings_data_class(bindings: &[PersistenceBinding]) -> PersistenceDataClass {
    if bindings.iter().any(|binding| binding.data_class().is_shared()) {
        PersistenceDataClass::PersistedShared
    } else if bindings.is_empty() {
        PersistenceDataClass::EphemeralLocalOnly
    } else {
        PersistenceDataClass::PersistedLocalOnly
    }
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum PersistenceBinding {
    /// 📁️ Local canonical store. A directory uses the multi-document `folder://` event log;
    /// a `*.json` path uses the single-blob `file://` export format.
    Folder { path: std::path::PathBuf },
    /// ☁️ A semio_hub node reachable over WebSocket
    /// (`remote://host:port` → `ws://host:port/scopes/{space_id}%2F{id}/document/ws`).
    Hub {
        base_url: String,
        space_id: String,
        /// 🎭️ Out-of-band presence scope (ticket 26/08/16/HUB-SPACES-…, contract §C0): rides
        /// as `?surface=` on the WS URL rather than a wire field — `PresencePeer`'s flag byte is
        /// already full. `<kind>@<standard>/<subset>#<role>`, e.g. `s.space.home@1/*#editor`.
        #[value(default, skip_serializing_if = "Option::is_none")]
        surface: Option<String>,
    },
}

impl PersistenceBinding {
    pub const fn data_class(&self) -> PersistenceDataClass {
        match self {
            Self::Folder { .. } => PersistenceDataClass::PersistedLocalOnly,
            Self::Hub { .. } => PersistenceDataClass::PersistedShared,
        }
    }
}

/// 🧾️ Everything {@link ArtifactHost::open} needs to spawn one document's actor.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactActorConfig {
    pub document_id: String,
    pub schema: String,
    pub bindings: Vec<PersistenceBinding>,
    /// 👁️ Watch the folder binding for external edits (other processes writing the file).
    #[value(default)]
    pub watch_external: bool,
    /// 🖋️ The authoring actor id used for semio_hub `Hello`/presence and operation origin filtering.
    pub actor: String,
    pub actor_identity_grant: semio_framework_value::RetainedCloneGrant,
}

/// 🗝️ Exact process-local identity for one open artifact actor. Hub documents are keyed by
/// their complete authority scope; local-only documents occupy a separate namespace.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArtifactDocumentKey {
    Local { document_id: String },
    Hub { space_id: String, document_id: String },
}

impl ArtifactDocumentKey {
    pub fn local(document_id: impl Into<String>) -> Self {
        Self::Local { document_id: document_id.into() }
    }

    pub fn hub(space_id: impl Into<String>, document_id: impl Into<String>) -> Self {
        Self::Hub { space_id: space_id.into(), document_id: document_id.into() }
    }

    pub fn for_config(config: &ArtifactActorConfig) -> Self {
        let mut hub_spaces = config.bindings.iter().filter_map(|binding| match binding {
            PersistenceBinding::Hub { space_id, .. } => Some(space_id.as_str()),
            PersistenceBinding::Folder { .. } => None,
        });
        let Some(space_id) = hub_spaces.next() else { return Self::local(config.document_id.clone()) };
        assert!(hub_spaces.all(|candidate| candidate == space_id), "one artifact actor cannot span hub spaces");
        Self::hub(space_id, config.document_id.clone())
    }

    pub fn document_id(&self) -> &str {
        match self {
            Self::Local { document_id } | Self::Hub { document_id, .. } => document_id,
        }
    }
}

/// 📨️ Caller → actor control messages, sent on the {@link ArtifactChannels} command channel.
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum ArtifactActorMsg {
    /// ⬆️ Wakes the actor to drain the store's outbound operations promptly. `envelopes` is a
    /// direct-injection fallback used only when no store is attached to the channel (empty = pure wake).
    LocalMutations {
        #[value(serialize_with = "envelope_serde::to_value", deserialize_with = "envelope_serde::from_value")]
        envelopes: Vec<MutationEnvelope>,
    },
    /// 🪢️ One canonical Store `BackboneMessage::Mutations` retained byte-for-byte by a
    /// mounted document port until its authoritative Hub command acknowledgment.
    DocumentBackbone { message: Vec<u8> },
    /// 🗃️ One exact versioned root plus recursive-owned-member archive for durable persistence.
    LocalDocumentArchive { archive: Vec<u8> },
    /// 📡️ Broadcasts this peer's presence/selection to the semio_hub.
    PresenceHeartbeat { peer: Box<PresencePeer> },
    /// 👻️ Publishes an ephemeral, best-effort UI-state blob on the semio_hub's uncredited preview
    /// lane (`crate::os_spr::wire::ClientFrame::PreviewPublish`) — e.g. a drag ghost or live cursor;
    /// `seq` is a per-`key` monotone counter so a receiver can drop stale-arriving previews.
    PublishPreview { key: String, seq: u64, payload: Vec<u8> },
    /// 🛟️ The host re-seeded a rebootstrapping hub document ([`ArtifactEvent::RebootstrapRequired`]): the canonical
    /// checkpoint pair it verified against the control and already loaded into the guest. The actor adopts it as its baseline,
    /// hands its unacked local operations back to the guest and dials at once, saying Hello at the pair's baseline.
    Reseed { pack: Vec<u8>, spr: Vec<u8>, baseline: semio_framework_os_directory::ArtifactFrontier },
    /// 🔄️ Forces an immediate re-read + diff of the folder binding (test/manual poke hook).
    ExternalChanged,
    /// ✂️ Flushes any pending outbound operations, then stops the actor.
    Detach,
}
//#endregion 🔖️Protocol
