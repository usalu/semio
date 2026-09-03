//! 📇️ Directory event log wire contract (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-
//! STUDIOS, contract C1): `DirectoryEvent`/`DirectoryEventBody` (persisted, backend-assigned dense
//! `seq`), `DirectoryCommand` (client intent posted to `/directory/commands`), `DirectoryStreamMessage`
//! (the `/directory/socket/v1` wire envelope), and the read DTOs (`SpaceView`/`MemberView`/`UserView`/
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
#[value(rename_all = "camelCase", deny_unknown_fields)]
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

//#region 🔖️Admin
/// 🛡️ One strict administrator intent; actor and authority fields are always server-derived.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum AdminIntentV1 {
    CreateSpace { request_id: String, name: String, space_kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility },
    RenameSpace { request_id: String, space_id: String, name: String },
    SetSpaceVisibility { request_id: String, space_id: String, visibility: DirectorySpaceVisibility },
    ArchiveSpace { request_id: String, space_id: String },
    DeleteSpace { request_id: String, space_id: String },
    UpsertSpaceMember { request_id: String, space_id: String, email: String, role: DirectorySpaceRole },
    RemoveSpaceMember { request_id: String, space_id: String, user_id: String },
    CreateSpaceInvite { request_id: String, space_id: String, role: DirectorySpaceRole, ttl_secs: u32 },
    RevokeSpaceInvite { request_id: String, space_id: String, invite_id: String },
    IssueDocumentShare { request_id: String, scope: DocumentScope, ttl_secs: u32 },
    RevokeDocumentShare { request_id: String, scope: DocumentScope, share_id: String, reason_code: String },
    RevokeUserSessions { request_id: String, user_id: String, reason_code: String },
    KickConnection { request_id: String, sync_session_id: String, reason_code: String },
    RebuildDirectoryProjections { request_id: String, expected_head_seq: u64 },
}

impl AdminIntentV1 {
    /// 🪪️ Returns the caller's bounded idempotency key.
    pub fn request_id(&self) -> &str {
        match self {
            Self::CreateSpace { request_id, .. }
            | Self::RenameSpace { request_id, .. }
            | Self::SetSpaceVisibility { request_id, .. }
            | Self::ArchiveSpace { request_id, .. }
            | Self::DeleteSpace { request_id, .. }
            | Self::UpsertSpaceMember { request_id, .. }
            | Self::RemoveSpaceMember { request_id, .. }
            | Self::CreateSpaceInvite { request_id, .. }
            | Self::RevokeSpaceInvite { request_id, .. }
            | Self::IssueDocumentShare { request_id, .. }
            | Self::RevokeDocumentShare { request_id, .. }
            | Self::RevokeUserSessions { request_id, .. }
            | Self::KickConnection { request_id, .. }
            | Self::RebuildDirectoryProjections { request_id, .. } => request_id,
        }
    }
}

/// 📍 Terminal or accepted state of one administrator intent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum AdminIntentStateV1 {
    Succeeded,
    Accepted,
    Failed,
    Cancelled,
}

/// 🧾 Bounded public outcome without capability or private locator material.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct AdminIntentOutcomeV1 {
    pub code: String,
    pub durable: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kick_attempted: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kick_signalled: Option<u32>,
}

/// 🎟️ One-display-only secret result, never stored in an audit fact or query projection.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct AdminIntentResultV1 {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub invite_token: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub share_token: Option<String>,
}

/// 🧾 Receipt for exactly one accepted administrator intent.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct AdminIntentReceiptV1 {
    pub operation_id: String,
    pub correlation_id: String,
    pub state: AdminIntentStateV1,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub event_seq_first: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub event_seq_last: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<AdminIntentResultV1>,
    pub outcome: AdminIntentOutcomeV1,
}

/// ⏳ Observable bounded progress for one running administrator operation.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct AdminOperationProgressV1 {
    pub completed_events: u64,
    pub total_events: u64,
    pub cancel_requested: bool,
}

/// 🔎 Durable receipt plus optional in-process progress for an expensive operation.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct AdminOperationStatusV1 {
    pub receipt: AdminIntentReceiptV1,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<AdminOperationProgressV1>,
}

/// 📄 One bounded cursor page observed at a server wall-clock instant.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct AdminPageV1<T> {
    pub rows: Vec<T>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub observed_at_ms: i64,
}

/// 🔴️ Trusted subset of a persisted sync-session binding.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct AdminRecordedConnectionV1 {
    pub sync_session_id: String,
    pub scope: DocumentScope,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub authenticated_user_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<DirectorySpaceRole>,
    pub connected_at_ms: i64,
    pub source: String,
}

/// 📸 Exact page of recorded bindings; it makes no transport-level liveness claim.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct AdminConnectionSnapshotV1 {
    pub rows: Vec<AdminRecordedConnectionV1>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub observed_at_ms: i64,
    pub source: String,
    pub head_seq: u64,
}

/// 🧮 Append-only operation-audit phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum AdminOperationAuditPhaseV1 {
    Accepted,
    Succeeded,
    Failed,
    Cancelled,
}

/// 📜 Public redacted administrator operation audit fact.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct AdminOperationAuditV1 {
    pub sequence: u64,
    pub operation_id: String,
    pub occurred_at_ms: i64,
    pub phase: AdminOperationAuditPhaseV1,
    pub intent_kind: String,
    pub target_kind: String,
    pub target_id: String,
    pub principal_user_id: String,
    pub principal_session_id: String,
    pub principal_generation: u64,
    pub correlation_id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub event_seq_first: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub event_seq_last: Option<u64>,
    pub outcome_code: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}
//#endregion 🔖️Admin

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

/// 🧯️ Maximum UTF-8 byte length for one public document-open identity.
pub const DOCUMENT_OPEN_ID_MAX_BYTES: usize = 256;
/// 🧯️ Maximum UTF-8 byte length for one client-instance identity.
pub const DOCUMENT_OPEN_CLIENT_INSTANCE_MAX_BYTES: usize = 128;
/// ⏳ Maximum lifetime of a document-open plan.
pub const DOCUMENT_OPEN_PLAN_MAX_TTL_MS: u64 = 30_000;
/// 🔢 Largest integer that has an exact representation in every v1 implementation.
pub const DOCUMENT_OPEN_MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

/// 📨 Structural, non-authoritative preference submitted to the protected open-plan command.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenIntentV1 {
    pub schema: String,
    pub version: u32,
    pub scope: DocumentScope,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub requested_surface_id: Option<String>,
    pub client_instance_id: String,
}

/// 🖼️ Renderer implementation selected by the verified server catalog.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DocumentOpenRendererTargetV1 {
    React,
    Wgpu,
    Wasm,
}

/// 👁️ Server-selected document surface authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DocumentOpenSurfaceRoleV1 {
    Viewer,
    Editor,
}

/// 📦️ Exact verified package projection required by one open plan.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenPackageV1 {
    pub plugin_id: String,
    pub package_id: String,
    pub version: String,
    pub component_sha256: String,
    pub component_blake3: String,
    pub descriptor_byte_sha256: String,
}

/// 🗂️ Immutable verified-catalog generation selected for one plan.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenCatalogV1 {
    pub generation_id: String,
}

/// 🧬️ Exact immutable artifact projection required by one open plan.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenArtifactV1 {
    pub kind: String,
    pub schema: String,
    pub pack_schema_hash: String,
}

/// 🪟️ One server-selected declared surface.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenSurfaceV1 {
    pub surface_id: String,
    pub app_id: String,
    pub window_kind_id: String,
    pub role: DocumentOpenSurfaceRoleV1,
    pub renderer_target: DocumentOpenRendererTargetV1,
}

/// 🔐️ Effective document operations after catalog and subject policy intersection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenGrantV1 {
    pub read: bool,
    pub write: bool,
    pub observe: bool,
}

/// 🏔️ Public immutable bootstrap identity selected for this plan.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenCheckpointV1 {
    pub checkpoint_id: String,
    pub descriptor_digest_v1: String,
    pub baseline_frontier: ArtifactFrontier,
    pub aggregate_sha256: String,
}

/// 🔁️ Durable generations that must remain exact until admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenRevalidationV1 {
    pub directory_revision: u64,
    pub membership_generation: u64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub session_generation: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub share_generation: Option<u64>,
}

/// 🎫️ Short-lived server-owned open decision. The receipt is exchanged once over protected HTTP.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenPlanV1 {
    pub schema: String,
    pub version: u32,
    pub receipt: String,
    pub expires_at_unix_ms: u64,
    pub scope: DocumentScope,
    pub descriptor_digest_v1: String,
    pub catalog: DocumentOpenCatalogV1,
    pub package: DocumentOpenPackageV1,
    pub artifact: DocumentOpenArtifactV1,
    pub surface: DocumentOpenSurfaceV1,
    pub grant: DocumentOpenGrantV1,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<DocumentOpenCheckpointV1>,
    pub revalidation: DocumentOpenRevalidationV1,
}

/// 🔄️ Protected command that exchanges one plan receipt for one document socket grant.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentPlanSocketGrantIntentV1 {
    pub schema: String,
    pub version: u32,
    pub plan_receipt: String,
}

/// 🚫️ Stable redacted open-plan failure vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum DocumentOpenPlanErrorCodeV1 {
    Denied,
    NotFound,
    CatalogUnavailable,
    ComponentUnavailable,
    Stale,
    Expired,
    AlreadyConsumed,
    Cancelled,
    DeadlineExceeded,
}

/// 🚨️ Public bounded open-plan failure without authority or catalog detail.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenPlanErrorV1 {
    pub schema: String,
    pub code: DocumentOpenPlanErrorCodeV1,
}

fn valid_document_open_text(value: &str, max_bytes: usize) -> bool {
    !value.is_empty() && value.len() <= max_bytes && !value.chars().any(char::is_control)
}

fn valid_document_open_hash(value: &str) -> bool {
    value.len() == 64 && !value.as_bytes().iter().all(|byte| *byte == b'0') && value.as_bytes().iter().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn valid_document_open_receipt(value: &str) -> bool {
    value.strip_prefix("open.v1.").is_some_and(|secret| {
        let base64_value = |byte| match byte {
            b'A'..=b'Z' => Some(byte - b'A'),
            b'a'..=b'z' => Some(byte - b'a' + 26),
            b'0'..=b'9' => Some(byte - b'0' + 52),
            b'-' => Some(62),
            b'_' => Some(63),
            _ => None,
        };
        secret.len() == 43
            && secret.bytes().all(|byte| base64_value(byte).is_some())
            && secret.as_bytes().last().and_then(|byte| base64_value(*byte)).is_some_and(|tail| tail & 0b11 == 0)
    })
}

impl DocumentOpenIntentV1 {
    /// ✅ Validates the strict public intent without interpreting its fields as authority.
    pub fn validate(&self) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        if self.schema != "semio.hub.document-open-intent/v1"
            || self.version != 1
            || !valid_document_open_text(&self.scope.space_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            || !valid_document_open_text(&self.scope.document_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            || !valid_document_open_text(&self.client_instance_id, DOCUMENT_OPEN_CLIENT_INSTANCE_MAX_BYTES)
            || self.requested_surface_id.as_deref().is_some_and(|value| !valid_document_open_text(value, DOCUMENT_OPEN_ID_MAX_BYTES))
        {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        Ok(())
    }
}

impl DocumentOpenPlanV1 {
    /// ✅ Validates a complete receipt-free authority projection at a caller-supplied wall time.
    pub fn validate(&self, now_ms: u64) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        let ids = [
            self.scope.space_id.as_str(),
            self.scope.document_id.as_str(),
            self.package.plugin_id.as_str(),
            self.package.package_id.as_str(),
            self.package.version.as_str(),
            self.artifact.kind.as_str(),
            self.artifact.schema.as_str(),
            self.surface.surface_id.as_str(),
            self.surface.app_id.as_str(),
            self.surface.window_kind_id.as_str(),
        ];
        if self.schema != "semio.hub.document-open-plan/v1"
            || self.version != 1
            || !valid_document_open_receipt(&self.receipt)
            || self.expires_at_unix_ms > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || self.expires_at_unix_ms <= now_ms
            || self.expires_at_unix_ms.checked_sub(now_ms).is_none_or(|ttl| ttl > DOCUMENT_OPEN_PLAN_MAX_TTL_MS)
            || ids.iter().any(|value| !valid_document_open_text(value, DOCUMENT_OPEN_ID_MAX_BYTES))
            || !valid_document_open_hash(&self.descriptor_digest_v1)
            || !valid_document_open_hash(&self.catalog.generation_id)
            || !valid_document_open_hash(&self.package.component_sha256)
            || !valid_document_open_hash(&self.package.component_blake3)
            || !valid_document_open_hash(&self.package.descriptor_byte_sha256)
            || !valid_document_open_hash(&self.artifact.pack_schema_hash)
            || !self.grant.read
            || !self.grant.observe
            || self.grant.write != matches!(self.surface.role, DocumentOpenSurfaceRoleV1::Editor)
            || self.revalidation.directory_revision == 0
            || self.revalidation.directory_revision > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || self.revalidation.membership_generation == 0
            || self.revalidation.membership_generation > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || (self.revalidation.session_generation.is_some() == self.revalidation.share_generation.is_some())
            || self.revalidation.session_generation == Some(0)
            || self.revalidation.session_generation.is_some_and(|generation| generation > DOCUMENT_OPEN_MAX_SAFE_INTEGER)
            || self.revalidation.share_generation == Some(0)
            || self.revalidation.share_generation.is_some_and(|generation| generation > DOCUMENT_OPEN_MAX_SAFE_INTEGER)
        {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        if let Some(checkpoint) = &self.checkpoint {
            if !valid_document_open_text(&checkpoint.baseline_frontier.head_edit_id, DOCUMENT_OPEN_ID_MAX_BYTES)
                || checkpoint.baseline_frontier.head_edit_ordinal > DOCUMENT_OPEN_MAX_SAFE_INTEGER
                || checkpoint.baseline_frontier.last_commit_seq > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            {
                return Err(DocumentOpenPlanErrorCodeV1::Denied);
            }
            if !valid_document_open_hash(&checkpoint.checkpoint_id)
                || checkpoint.descriptor_digest_v1 != self.descriptor_digest_v1
                || !valid_document_open_hash(&checkpoint.aggregate_sha256)
                || checkpoint.baseline_frontier.document_id != self.scope.document_id
                || checkpoint.baseline_frontier.head_edit_ordinal < checkpoint.baseline_frontier.last_commit_seq
                || checkpoint.baseline_frontier.chain_hash.0 == [0; 32]
            {
                return Err(DocumentOpenPlanErrorCodeV1::Stale);
            }
        }
        Ok(())
    }
}

impl DocumentPlanSocketGrantIntentV1 {
    /// ✅ Validates the exact one-use receipt exchange command shape.
    pub fn validate(&self) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        if self.schema != "semio.hub.document-plan-socket-grant-intent/v1" || self.version != 1 || !valid_document_open_receipt(&self.plan_receipt) {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        Ok(())
    }
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
#[value(rename_all = "camelCase", deny_unknown_fields)]
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

/// 📡️ One `/directory/socket/v1` text frame (contract C1/C2) — subscribe, then gap-free replay.
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

    #[derive(FromValue)]
    #[value(rename_all = "camelCase")]
    struct DocumentOpenPlanFixture {
        now_ms: u64,
        descriptor: DocumentDescriptor,
        descriptor_digest_v1: String,
        intent: DocumentOpenIntentV1,
        valid_plan: DocumentOpenPlanV1,
        exchange_intent: DocumentPlanSocketGrantIntentV1,
    }

    #[semio_framework_async_macros::async_test]
    async fn document_open_plan_v1_matches_language_neutral_fixture() {
        let fixture: DocumentOpenPlanFixture = crate::os_pack::json::from_json_str(include_str!("../../../🧫️fixtures/📇️directory/📄️document-open-plan-v1.json")).expect("document open plan fixture decodes");
        assert_eq!(hex_lower(&descriptor_digest_v1(&fixture.descriptor).expect("descriptor hashes").0), fixture.descriptor_digest_v1);
        assert_eq!(fixture.intent.validate(), Ok(()));
        assert_eq!(fixture.valid_plan.validate(fixture.now_ms), Ok(()));
        assert_eq!(fixture.exchange_intent.validate(), Ok(()));

        let mut overlong = fixture.valid_plan.clone();
        overlong.expires_at_unix_ms = fixture.now_ms + DOCUMENT_OPEN_PLAN_MAX_TTL_MS + 1;
        assert_eq!(overlong.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));

        let encoded = crate::os_pack::json::to_json_string(&fixture.valid_plan);
        let forged = format!("{},\"actor\":\"caller-selected\"}}", encoded.strip_suffix('}').expect("object"));
        assert!(crate::os_pack::json::from_json_str::<DocumentOpenPlanV1>(&forged).is_err());
        let nested_scope = encoded.replace("\"documentId\":\"plan:\u{6771}\u{4eac}\"", "\"documentId\":\"plan:\u{6771}\u{4eac}\",\"actor\":\"caller-selected\"");
        assert!(crate::os_pack::json::from_json_str::<DocumentOpenPlanV1>(&nested_scope).is_err());
        let nested_frontier = encoded.replace("\"headEditOrdinal\":2", "\"headEditOrdinal\":2,\"storageKey\":\"private\"");
        assert!(crate::os_pack::json::from_json_str::<DocumentOpenPlanV1>(&nested_frontier).is_err());

        let mut unicode_control = fixture.valid_plan.clone();
        unicode_control.surface.app_id = "app.\u{85}hidden".into();
        assert_eq!(unicode_control.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
        let mut noncanonical_receipt = fixture.valid_plan.clone();
        noncanonical_receipt.receipt = "open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyB".into();
        assert_eq!(noncanonical_receipt.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
        let mut frontier_control = fixture.valid_plan.clone();
        frontier_control.checkpoint.as_mut().expect("checkpoint").baseline_frontier.head_edit_id = "edit:\u{85}".into();
        assert_eq!(frontier_control.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
        let mut frontier_overlong = fixture.valid_plan.clone();
        frontier_overlong.checkpoint.as_mut().expect("checkpoint").baseline_frontier.head_edit_id = "a".repeat(DOCUMENT_OPEN_ID_MAX_BYTES + 1);
        assert_eq!(frontier_overlong.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
        let mut unsafe_expiry = fixture.valid_plan.clone();
        unsafe_expiry.expires_at_unix_ms = DOCUMENT_OPEN_MAX_SAFE_INTEGER + 1;
        assert_eq!(unsafe_expiry.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
        let mut unsafe_frontier = fixture.valid_plan.clone();
        unsafe_frontier.checkpoint.as_mut().expect("checkpoint").baseline_frontier.head_edit_ordinal = DOCUMENT_OPEN_MAX_SAFE_INTEGER + 1;
        assert_eq!(unsafe_frontier.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
        let mut unsafe_revalidation = fixture.valid_plan;
        unsafe_revalidation.revalidation.directory_revision = DOCUMENT_OPEN_MAX_SAFE_INTEGER + 1;
        assert_eq!(unsafe_revalidation.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    }
}
//#endregion 🧪️Tests
