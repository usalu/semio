//! 📇️ Directory event log wire contract (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-
//! STUDIOS, contract C1): `DirectoryEvent`/`DirectoryEventBody` (persisted, backend-assigned dense
//! `seq`), `DirectoryCommand` (client intent posted to `/directory/commands`), `DirectoryStreamMessage`
//! (the `/directory/ws` wire envelope), and the read DTOs (`SpaceView`/`MemberView`/`UserView`/
//! `ConnectionView`/`DocumentView`/`InviteView`) the hub's REST surface returns. Pure data, no fold
//! logic — see the module root `../🦀️.rs`'s `DirectoryReadModel`/`fold`. `DirectorySpaceKind`/
//! `DirectorySpaceVisibility`/`DirectorySpaceRole` mirror `🪐️space`'s `SpaceKind`/`SpaceVisibility`/
//! `SpaceRole` vocabulary (atelier/studio/archive, private/public, author/spectator) string-identically;
//! this wasm-safe kernel crate does not mount that module (`🦀️.rs`'s header note: unwired pending
//! dep-DAG cleanup), so the enums are re-declared here, same convention `🌎️hub/📇️directory`'s
//! `SpaceRole` already uses for the same reason.
//!
//! 🧭️ `space.created`'s and `create-space`'s space-kind fields are named `space_kind`
//! (`spaceKind` on the wire), not contract-freeze.md's bare `kind` — both bodies are internally
//! tagged (`#[value(tag = "kind")]`), so a same-named payload field would collide with the
//! discriminator. Flagged as a `sharedFileRequest` in lane 0-A's report.
//!
//! 🌉️ `ToValue`/`FromValue` (`#[derive(ToValue, FromValue)]`, not a `serde_json`-backed bridge):
//! unblocked by `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-
//! AND-ARTIFACTS/🔍️research/📓️dslvalue-integer-fidelity.md` — `DslValue::Number` now carries
//! `UInt`/`Int`/`Float` (not a lone `f64`), so `CreateInvite.ttl_secs: u64` etc. round-trip as bare
//! integers (`3600`, never `3600.0`) the way this contract's real external hub (`🌎️hub`'s sibling
//! Rust/serde types, strict — no `arbitrary_precision`) requires on the wire. An earlier pass
//! (`📓️directory-spr-serde-removal.md`) declined this conversion for exactly that reason, before the
//! fix landed. `#[value(...)]` mirrors every `#[serde(...)]` shape this file used: `tag` +
//! `rename_all_fields`, per-variant `rename`, and mixed `rename_all` casings across sibling enums —
//! all supported by `semio_framework_value_derive` today (see its own header docs).

use semio_framework_value_derive::{FromValue, ToValue};

/// 🔐️ Domain prefix for the one canonical descriptor digest encoding.
pub const DESCRIPTOR_DIGEST_V1_DOMAIN: &[u8] = b"semio.document-descriptor.digest.v1\0";

//#region 🔖️Vocabulary
/// 🏛️ Mirrors `🪐️space::SpaceKind` string-identically (see this file's header).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DirectorySpaceKind {
    Atelier,
    Studio,
    Archive,
}

/// 👁️ Mirrors `🪐️space::SpaceVisibility` string-identically.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DirectorySpaceVisibility {
    Private,
    Public,
}

/// 🧑️‍🤝️‍🧑️ Mirrors `🪐️space::SpaceRole` string-identically.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DirectorySpaceRole {
    Author,
    Spectator,
}

/// 🎯️ Structural tenant-qualified document identity shared by directory and artifact authority.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocumentScope {
    pub space_id: String,
    pub document_id: String,
}

impl DocumentScope {
    /// 🆕️ Creates one structural document scope without flattening either identifier.
    pub fn new(space_id: impl Into<String>, document_id: impl Into<String>) -> Self {
        Self { space_id: space_id.into(), document_id: document_id.into() }
    }
}

/// #️⃣ One exactly 32-byte artifact authority hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArtifactHash(pub [u8; 32]);

impl ArtifactHash {
    /// 🧱️ Wraps an already-sized hash.
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// 🔑️ Borrows the fixed-width bytes.
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl crate::ToValue for ArtifactHash {
    fn to_value(&self) -> crate::DslValue {
        crate::DslValue::Array(self.0.iter().map(crate::ToValue::to_value).collect())
    }
}

impl crate::FromValue for ArtifactHash {
    fn from_value(value: crate::DslValue) -> Result<Self, crate::ValueError> {
        let crate::DslValue::Array(items) = value else {
            return Err(crate::ValueError::new(format!("expected an array for ArtifactHash, found {value:?}")));
        };
        if items.len() != 32 {
            return Err(crate::ValueError::new(format!("expected exactly 32 bytes for ArtifactHash, found {}", items.len())));
        }
        let mut bytes = [0u8; 32];
        for (index, item) in items.into_iter().enumerate() {
            bytes[index] = item.as_u64().and_then(|value| u8::try_from(value).ok()).ok_or_else(|| crate::ValueError::new(format!("expected an integer byte at ArtifactHash.{index}")))?;
        }
        Ok(Self(bytes))
    }
}

/// 🧾️ Canonical checkpoint identity.
pub type CheckpointId = ArtifactHash;
//#endregion 🔖️Vocabulary

//#region 🔖️Actor
/// 🎭️ Who issued a directory command / recorded a directory event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DirectoryActorKind {
    User,
    Admin,
    System,
}

/// 🎭️ `{ kind, id }` — the actor id grammar is `user:{user_id}#{shell_session_id}` for `User`
/// (contract-freeze.md §C0), opaque for `Admin`/`System`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DirectoryActor {
    pub kind: DirectoryActorKind,
    pub id: String,
}

/// 🕰️ Hybrid logical clock stamp: physical wall time plus a same-millisecond tiebreak counter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Hlc {
    pub physical_ms: i64,
    pub logical: u32,
}
//#endregion 🔖️Actor

//#region 🔖️Event
/// ⚡️ One directory event body. Every variant's `kind` tag is the contract's own dotted string
/// (e.g. `"space.created"`) — not a `rename_all` casing of the variant name — so every variant
/// carries an explicit `#[value(rename = "…")]`. `rename_all_fields = "camelCase"` casings each
/// variant's own fields independently of the tag.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all_fields = "camelCase")]
pub enum DirectoryEventBody {
    #[value(rename = "user.created")]
    UserCreated { user_id: String, email: String, display_name: String },
    #[value(rename = "space.created")]
    SpaceCreated { space_id: String, name: String, space_kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility, owner_user_id: String },
    #[value(rename = "space.renamed")]
    SpaceRenamed { space_id: String, name: String },
    #[value(rename = "space.visibility-changed")]
    SpaceVisibilityChanged { space_id: String, visibility: DirectorySpaceVisibility },
    #[value(rename = "space.archived")]
    SpaceArchived { space_id: String },
    #[value(rename = "space.deleted")]
    SpaceDeleted { space_id: String },
    #[value(rename = "member.upserted")]
    MemberUpserted { space_id: String, user_id: String, role: DirectorySpaceRole },
    #[value(rename = "member.removed")]
    MemberRemoved { space_id: String, user_id: String },
    #[value(rename = "invite.redeemed")]
    InviteRedeemed { space_id: String, user_id: String, invite_id: String, role: DirectorySpaceRole },
    #[value(rename = "document.announced")]
    DocumentAnnounced { descriptor: DocumentDescriptor },
    #[value(rename = "artifact.checkpoint-published")]
    ArtifactCheckpointPublished { checkpoint: PublishedArtifactCheckpoint },
    #[value(rename = "artifact.retention-advanced")]
    ArtifactRetentionAdvanced { retention: ArtifactRetention },
}

/// 📜️ One persisted, backend-assigned directory event. `seq` is dense and 1-based; `space_id`/
/// `user_id` are denormalized indexing hints (redundant with `body`'s own fields) for cheap
/// `?since=`/visibility filtering without decoding `body`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DirectoryEvent {
    pub seq: u64,
    pub id: String,
    pub hlc: Hlc,
    pub actor: DirectoryActor,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub body: DirectoryEventBody,
    pub recorded_at_ms: i64,
}
//#endregion 🔖️Event

//#region 🔖️Command
/// 🎮️ One client-issued directory command, posted to `POST /directory/commands` (contract C2).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase")]
pub enum DirectoryCommand {
    CreateSpace { name: String, space_kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility },
    RenameSpace { space_id: String, name: String },
    SetVisibility { space_id: String, visibility: DirectorySpaceVisibility },
    ArchiveSpace { space_id: String },
    DeleteSpace { space_id: String },
    UpsertMember { space_id: String, email: String, role: DirectorySpaceRole },
    RemoveMember { space_id: String, user_id: String },
    CreateInvite { space_id: String, role: DirectorySpaceRole, ttl_secs: u64 },
    RevokeInvite { space_id: String, invite_id: String },
    AnnounceDocument { descriptor: DocumentDescriptor },
}
//#endregion 🔖️Command

//#region 🔖️Views
/// 🏠️ One space, as the hub's REST/read surface renders it. `role` is the CALLING user's
/// membership role (server-filled per request), never derived by the pure fold.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct SpaceView {
    pub id: String,
    pub name: String,
    pub kind: DirectorySpaceKind,
    pub visibility: DirectorySpaceVisibility,
    pub owner_user_id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<DirectorySpaceRole>,
    pub member_count: u32,
    pub document_count: u32,
    pub active_connections: u32,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
}

/// 🧑️ One space member, display-ready (`email`/`display_name` joined from the user directory).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct MemberView {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub role: DirectorySpaceRole,
}

/// 🙋️ One platform user.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct UserView {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub created_at_ms: i64,
}

/// 🔴️ One realtime document connection (admin overview / presence roster).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ConnectionView {
    pub sync_session_id: String,
    pub space_id: String,
    pub document_id: String,
    pub surface: String,
    pub actor: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub role: DirectorySpaceRole,
    pub connected_at_ms: i64,
    pub presence_known: bool,
}

/// 📦️ Immutable identity of the plugin package that owns a document codec.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocumentOwner {
    pub plugin_id: String,
    pub package_id: String,
    pub version: String,
    pub package_hash: String,
}

/// 🏁️ One authoritative replication frontier bound to a canonical bootstrap snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocumentFrontier {
    pub head_seq: u64,
    pub commit_seq: u64,
    pub epoch: u64,
}

/// 🧬️ Durable, space-qualified codec and initial-bootstrap identity for one document.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocumentDescriptor {
    pub space_id: String,
    pub document_id: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub owner: DocumentOwner,
    pub pack_schema_hash: String,
    pub bootstrap_version: u32,
    pub bootstrap_frontier: DocumentFrontier,
    pub bootstrap_snapshot_hash: String,
}

/// 🚨️ Descriptor values that cannot participate in canonical authority hashing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DescriptorDigestError {
    EmptyField(&'static str),
    InvalidHash(&'static str),
    InvalidFrontier,
    InvalidBootstrapVersion,
    LengthOverflow(&'static str),
}

impl std::fmt::Display for DescriptorDigestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyField(field) => write!(formatter, "descriptor field `{field}` is empty"),
            Self::InvalidHash(field) => write!(formatter, "descriptor field `{field}` is not a nonzero lowercase SHA-256"),
            Self::InvalidFrontier => formatter.write_str("descriptor bootstrap commit exceeds head"),
            Self::InvalidBootstrapVersion => formatter.write_str("descriptor bootstrap version must be positive"),
            Self::LengthOverflow(field) => write!(formatter, "descriptor field `{field}` exceeds the u64 byte-length encoding"),
        }
    }
}

impl std::error::Error for DescriptorDigestError {}

fn decode_descriptor_hash(field: &'static str, value: &str) -> Result<[u8; 32], DescriptorDigestError> {
    if value.len() != 64 || value.as_bytes().iter().any(|byte| !matches!(byte, b'0'..=b'9' | b'a'..=b'f')) {
        return Err(DescriptorDigestError::InvalidHash(field));
    }
    let mut output = [0u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let digit = |byte| match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            _ => unreachable!(),
        };
        output[index] = digit(pair[0]) << 4 | digit(pair[1]);
    }
    if output == [0; 32] {
        return Err(DescriptorDigestError::InvalidHash(field));
    }
    Ok(output)
}

fn append_descriptor_field(output: &mut Vec<u8>, field: &'static str, bytes: &[u8]) -> Result<(), DescriptorDigestError> {
    let length = u64::try_from(bytes.len()).map_err(|_| DescriptorDigestError::LengthOverflow(field))?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}

fn append_descriptor_text(output: &mut Vec<u8>, field: &'static str, value: &str) -> Result<(), DescriptorDigestError> {
    if value.is_empty() {
        return Err(DescriptorDigestError::EmptyField(field));
    }
    append_descriptor_field(output, field, value.as_bytes())
}

/// 🧬️ Encodes every immutable descriptor leaf after `DESCRIPTOR_DIGEST_V1_DOMAIN`, in declaration
/// order, as `u64_be(payload byte length) || payload`. Text is UTF-8, unsigned integers are fixed-
/// width big-endian payloads, and the three SHA-256 strings are decoded to their 32 bytes. Owner
/// leaves remain nested-order `plugin_id, package_id, version, package_hash`; frontier leaves remain
/// `head_seq, commit_seq, epoch`. JSON serialization never participates.
pub fn descriptor_digest_encoding_v1(descriptor: &DocumentDescriptor) -> Result<Vec<u8>, DescriptorDigestError> {
    if descriptor.bootstrap_version == 0 {
        return Err(DescriptorDigestError::InvalidBootstrapVersion);
    }
    if descriptor.bootstrap_frontier.commit_seq > descriptor.bootstrap_frontier.head_seq {
        return Err(DescriptorDigestError::InvalidFrontier);
    }
    let mut output = Vec::with_capacity(DESCRIPTOR_DIGEST_V1_DOMAIN.len() + 384);
    output.extend_from_slice(DESCRIPTOR_DIGEST_V1_DOMAIN);
    append_descriptor_text(&mut output, "space_id", &descriptor.space_id)?;
    append_descriptor_text(&mut output, "document_id", &descriptor.document_id)?;
    append_descriptor_text(&mut output, "artifact_kind", &descriptor.artifact_kind)?;
    append_descriptor_text(&mut output, "artifact_schema", &descriptor.artifact_schema)?;
    append_descriptor_text(&mut output, "owner.plugin_id", &descriptor.owner.plugin_id)?;
    append_descriptor_text(&mut output, "owner.package_id", &descriptor.owner.package_id)?;
    append_descriptor_text(&mut output, "owner.version", &descriptor.owner.version)?;
    append_descriptor_field(&mut output, "owner.package_hash", &decode_descriptor_hash("owner.package_hash", &descriptor.owner.package_hash)?)?;
    append_descriptor_field(&mut output, "pack_schema_hash", &decode_descriptor_hash("pack_schema_hash", &descriptor.pack_schema_hash)?)?;
    append_descriptor_field(&mut output, "bootstrap_version", &descriptor.bootstrap_version.to_be_bytes())?;
    append_descriptor_field(&mut output, "bootstrap_frontier.head_seq", &descriptor.bootstrap_frontier.head_seq.to_be_bytes())?;
    append_descriptor_field(&mut output, "bootstrap_frontier.commit_seq", &descriptor.bootstrap_frontier.commit_seq.to_be_bytes())?;
    append_descriptor_field(&mut output, "bootstrap_frontier.epoch", &descriptor.bootstrap_frontier.epoch.to_be_bytes())?;
    append_descriptor_field(&mut output, "bootstrap_snapshot_hash", &decode_descriptor_hash("bootstrap_snapshot_hash", &descriptor.bootstrap_snapshot_hash)?)?;
    Ok(output)
}

/// 🔐️ SHA-256 of [`descriptor_digest_encoding_v1`] through the repository-owned hash primitive.
pub fn descriptor_digest_v1(descriptor: &DocumentDescriptor) -> Result<ArtifactHash, DescriptorDigestError> {
    Ok(ArtifactHash(semio_framework_hash::Sha256::digest(&descriptor_digest_encoding_v1(descriptor)?)))
}

/// 🔡️ Renders canonical lowercase hexadecimal bytes for fixtures and private storage keys.
pub fn hex_lower(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    output
}

/// 🏔️ Exact public checkpoint frontier, structurally identical to the replication wire frontier.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactFrontier {
    pub document_id: String,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub last_commit_seq: u64,
    pub chain_hash: ArtifactHash,
}

/// 🫧️ Integrity and private storage identity for one staged immutable artifact blob.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactBlobRef {
    pub sha256: ArtifactHash,
    pub byte_length: u64,
    pub storage_key: String,
}

/// 🪞️ Public integrity metadata for one staged blob; private storage keys never enter events.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct PublishedArtifactBlob {
    pub sha256: ArtifactHash,
    pub byte_length: u64,
}

/// 📡️ Storage-key-free checkpoint metadata published through the append-only directory log.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct PublishedArtifactCheckpoint {
    pub scope: DocumentScope,
    pub checkpoint_id: CheckpointId,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub parent_checkpoint_id: Option<CheckpointId>,
    pub descriptor_digest_v1: ArtifactHash,
    pub baseline_frontier: ArtifactFrontier,
    pub pack: PublishedArtifactBlob,
    pub spr: PublishedArtifactBlob,
    pub aggregate_sha256: ArtifactHash,
    pub published_at_ms: u64,
}

/// 📍️ One server-derived checkpoint including backend-private immutable blob locators.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactCheckpoint {
    pub scope: DocumentScope,
    pub checkpoint_id: CheckpointId,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub parent_checkpoint_id: Option<CheckpointId>,
    pub descriptor_digest_v1: ArtifactHash,
    pub baseline_frontier: ArtifactFrontier,
    pub pack: ArtifactBlobRef,
    pub spr: ArtifactBlobRef,
    pub aggregate_sha256: ArtifactHash,
    pub published_at_ms: u64,
}

/// 🧹️ Public retention selection vocabulary; advancement is P2-B and pruning remains P2-D.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactRetention {
    pub scope: DocumentScope,
    pub retained_checkpoint_id: CheckpointId,
    pub retained_floor: ArtifactFrontier,
    pub checkpoint_lineage_head: CheckpointId,
}

/// 🧾️ One document inside a space's durable artifact index plus live sync bookkeeping.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocumentView {
    pub descriptor: DocumentDescriptor,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub epoch: u64,
}

/// 🔗️ One outstanding (or revoked) space invite. Not event-sourced itself (secret token lives
/// outside the log) — only its `invite.redeemed` outcome is a `DirectoryEvent`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct InviteView {
    pub id: String,
    pub space_id: String,
    pub role: DirectorySpaceRole,
    pub created_at_ms: i64,
    pub expires_at_ms: i64,
    pub revoked: bool,
}
//#endregion 🔖️Views

//#region 🔖️Stream
/// 🔌️ `connection` stream message phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DirectoryConnectionPhase {
    Opened,
    Closed,
}

/// 👥️ One live presence actor in a document's roster (Amendment 3 to C1) — the hub knows all four
/// fields without ever decoding the actor's opaque `PresencePeer` bytes: `surface`/`color` are
/// stamped at hub-handshake time (`?surface=`, `HubState.session_colors`), `user_id` from auth.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DirectoryPresenceActor {
    pub actor: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub surface: String,
    pub color: u8,
}

/// 🛟️ Public checkpoint identity that makes a lagged client discard its discontinuous live state.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RebootstrapRequired {
    pub scope: DocumentScope,
    pub checkpoint_id: CheckpointId,
    pub descriptor_digest_v1: ArtifactHash,
    pub baseline_frontier: ArtifactFrontier,
}

/// 📡️ One `/directory/ws` text frame (contract C1/C2) — subscribe, then gap-free replay.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "lowercase", rename_all_fields = "camelCase")]
pub enum DirectoryStreamMessage {
    Event {
        event: DirectoryEvent,
    },
    Connection {
        phase: DirectoryConnectionPhase,
        connection: ConnectionView,
    },
    /// 👥️ Amendment 3 to C1: the document-wide roster, published on every roster change.
    Presence {
        space_id: String,
        document_id: String,
        actors: Vec<DirectoryPresenceActor>,
    },
    Heartbeat {
        head_seq: u64,
    },
    #[value(rename = "rebootstrap-required")]
    RebootstrapRequired {
        control: RebootstrapRequired,
    },
}
//#endregion 🔖️Stream

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn event_body_kind_is_the_dotted_wire_string() {
        let body = DirectoryEventBody::SpaceCreated { space_id: "sp-1".into(), name: "Studio".into(), space_kind: DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private, owner_user_id: "u-1".into() };
        let json = crate::os_pack::json::to_json_string(&body);
        assert!(json.contains("\"kind\":\"space.created\""), "got {json}");
        assert!(json.contains("\"spaceKind\":\"studio\""), "got {json}");
        assert!(json.contains("\"visibility\":\"private\""), "got {json}");
        let round: DirectoryEventBody = crate::os_pack::json::from_json_str(&json).expect("deserialize");
        assert_eq!(round, body);
    }

    #[semio_framework_async_macros::async_test]
    async fn command_kind_is_kebab_case() {
        let command = DirectoryCommand::CreateSpace { name: "Atelier".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Private };
        let json = crate::os_pack::json::to_json_string(&command);
        assert!(json.contains("\"kind\":\"create-space\""), "got {json}");
        assert!(json.contains("\"spaceKind\":\"atelier\""), "got {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn stream_message_kinds_round_trip() {
        let heartbeat = DirectoryStreamMessage::Heartbeat { head_seq: 42 };
        let json = crate::os_pack::json::to_json_string(&heartbeat);
        assert!(json.contains("\"kind\":\"heartbeat\""), "got {json}");
        assert!(json.contains("\"headSeq\":42"), "got {json} (must be a bare integer, not 42.0)");
        let round: DirectoryStreamMessage = crate::os_pack::json::from_json_str(&json).expect("deserialize");
        assert_eq!(round, heartbeat);
    }

    /// 🔢️ The exact scenario `📓️directory-spr-serde-removal.md` declined on: a `u64` field must
    /// round-trip as a bare wire integer, never `.0`-suffixed — `DslValue::Number` no longer
    /// erases the UInt/Float distinction (`📓️dslvalue-integer-fidelity.md`).
    #[semio_framework_async_macros::async_test]
    async fn create_invite_ttl_secs_is_a_bare_integer_on_the_wire() {
        let command = DirectoryCommand::CreateInvite { space_id: "sp-1".into(), role: DirectorySpaceRole::Author, ttl_secs: 3600 };
        let json = crate::os_pack::json::to_json_string(&command);
        assert!(json.contains("\"ttlSecs\":3600"), "got {json}");
        assert!(!json.contains("3600.0"), "got {json} — ttl_secs must not collapse to a float");
        let round: DirectoryCommand = crate::os_pack::json::from_json_str(&json).expect("deserialize");
        assert_eq!(round, command);
    }

    #[derive(FromValue)]
    #[value(rename_all = "camelCase")]
    struct DescriptorFixture {
        valid: DocumentDescriptor,
        canonical: String,
    }

    #[semio_framework_async_macros::async_test]
    async fn document_descriptor_matches_the_language_neutral_fixture() {
        let fixture: DescriptorFixture = crate::os_pack::json::from_json_str(include_str!("../../../🧫️fixtures/📇️directory/📄️document-descriptor.json")).expect("descriptor fixture decodes");
        assert_eq!(crate::os_pack::json::to_json_string(&fixture.valid), fixture.canonical);
    }

    #[derive(FromValue)]
    #[value(rename_all = "camelCase")]
    struct ArtifactAuthorityFixture {
        descriptor: DocumentDescriptor,
        descriptor_encoding_hex: String,
        descriptor_digest_v1: ArtifactHash,
    }

    #[semio_framework_async_macros::async_test]
    async fn document_descriptor_digest_v1_matches_the_language_neutral_binary_vector() {
        let fixture: ArtifactAuthorityFixture = crate::os_pack::json::from_json_str(include_str!("../../../🧫️fixtures/📇️directory/📄️artifact-authority.json")).expect("artifact authority fixture decodes");
        assert_eq!(hex_lower(&descriptor_digest_encoding_v1(&fixture.descriptor).expect("descriptor encodes")), fixture.descriptor_encoding_hex);
        assert_eq!(descriptor_digest_v1(&fixture.descriptor).expect("descriptor hashes"), fixture.descriptor_digest_v1);
    }
}
//#endregion 🧪️Tests
