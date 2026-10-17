//! 📖️ Events, commands and administration of the directory schema: the space kinds, visibility and roles, the artifact hash, the actor and clock, the event log records and pages, the directory command with its request, receipt and error codes and the administration intents, progress, connection and audit records. See [`super`].

use super::*;

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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ToValue, FromValue)]
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

impl semio_framework_value::ToValue for ArtifactHash {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::Array(self.0.iter().map(semio_framework_value::ToValue::to_value).collect())
    }
}

impl semio_framework_value::FromValue for ArtifactHash {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Array(items) = value else {
            return Err(semio_framework_value::ValueError::new(protocol::value::ValueRefusalKind::InvalidValue,format!("expected an array for ArtifactHash, found {value:?}")));
        };
        if items.len() != 32 {
            return Err(semio_framework_value::ValueError::new(protocol::value::ValueRefusalKind::InvalidValue,format!("expected exactly 32 bytes for ArtifactHash, found {}", items.len())));
        }
        let mut bytes = [0u8; 32];
        for (index, item) in items.into_iter().enumerate() {
            bytes[index] = item.as_u64().and_then(|value| u8::try_from(value).ok()).ok_or_else(|| semio_framework_value::ValueError::new(protocol::value::ValueRefusalKind::InvalidValue,format!("expected an integer byte at ArtifactHash.{index}")))?;
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
    /// 🧊️ Atomically freezes the space and demotes every current Author membership to Spectator.
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
    #[value(rename = "document.indexed")]
    DocumentIndexed { scope: DocumentScope, descriptor_digest_v1: ArtifactHash, entry: DocumentIndexEntryV1 },
    #[value(rename = "artifact.checkpoint-published")]
    ArtifactCheckpointPublished { checkpoint: PublishedArtifactCheckpoint },
    #[value(rename = "artifact.retention-advanced")]
    ArtifactRetentionAdvanced { retention: ArtifactRetention },
    /// 🎚️ One preference change of ONE user (`schema` names the vocabulary, `mutation` is its JSON object text — the hub
    /// never reads it). Only that user sees it, and only on the preference lane (`DIRECTORY_PREFERENCE_PAGE_PATH_V1`): the
    /// default page and every guest fold never carry it (ticket 26/09/23 U5).
    #[value(rename = "user.preference-recorded")]
    UserPreferenceRecorded { user_id: String, schema: String, mutation: String },
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

/// 📏️ Maximum raw rows represented by one event-page scan.
pub const DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS: usize = 128;

/// 📦️ Maximum canonical response bytes retained by one page owner.
pub const DIRECTORY_EVENT_PAGE_MAX_BYTES: usize = 64 * 1024;

/// ⚡️ Maximum canonical bytes of one persisted event.
pub const DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES: usize = 48 * 1024;

/// 🎚️ The preference lane's page route: the principal's own `user.preference-recorded` events on the page machinery of
/// `/directory/event-page/v1`, nothing else.
pub const DIRECTORY_PREFERENCE_PAGE_PATH_V1: &str = "/directory/preference-page/v1";

pub const USER_PREFERENCE_SCHEMA_ID_MAX_BYTES: usize = 128;

pub const USER_PREFERENCE_MUTATION_MAX_BYTES: usize = 4096;

/// 📄️ One authenticated, receipt-bound bounded scan of the durable directory log.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectoryEventPageV1 {
    pub schema: String,
    pub session_binding_sha256: String,
    pub authorization_generation: u64,
    pub after_seq_exclusive: u64,
    pub through_seq_inclusive: u64,
    pub has_more: bool,
    pub events: Vec<DirectoryEvent>,
    pub receipt_sha256: String,
}

/// 🚫️ Stable bounded-page denial classes shared by hub and clients.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectoryEventPageErrorV1 {
    Invalid,
    TooLarge,
    ReceiptMismatch,
}

#[doc(hidden)] pub fn directory_event_page_has_control(value: &semio_framework_value::DslValue) -> bool {
    match value {
        semio_framework_value::DslValue::String(value) => value.chars().any(char::is_control),
        semio_framework_value::DslValue::Bytes(_) => true,
        semio_framework_value::DslValue::Array(values) => values.iter().any(directory_event_page_has_control),
        semio_framework_value::DslValue::Object(fields) => fields.iter().any(|(key, value)| key.chars().any(char::is_control) || directory_event_page_has_control(value)),
        semio_framework_value::DslValue::Null | semio_framework_value::DslValue::Bool(_) | semio_framework_value::DslValue::Number(_) => false,
    }
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
    AnnounceDocument { descriptor: Box<DocumentDescriptor> },
    RecordUserPreference { schema: String, mutation: String },
}
//#endregion 🔖️Command

//#region 🌊️EditedArtifactFrontier
/// 🌊️ One edited, committed point of a document's ledger in its cross-runtime wire grammar: the
/// counters and content chain the hub's document authority published for it and the edit id at its
/// tip. Genesis has no edited point, so a zero ordinal or commit never validates.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct EditedArtifactFrontierV1 {
    pub document_id: String,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub last_commit_seq: u64,
    pub chain_sha256: String,
}

impl EditedArtifactFrontierV1 {
    /// 🛡️ Checks the cross-runtime integer, text, and hash boundary.
    pub fn validate(&self) -> bool {
        valid_document_open_text(&self.document_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && self.head_edit_ordinal > 0
            && self.head_edit_ordinal <= DOCUMENT_OPEN_MAX_SAFE_INTEGER
            && valid_document_open_text(&self.head_edit_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && self.last_commit_seq > 0
            && self.last_commit_seq <= DOCUMENT_OPEN_MAX_SAFE_INTEGER
            && valid_document_open_hash(&self.chain_sha256)
    }


}
//#endregion 🌊️EditedArtifactFrontier

//#region 🔖️CommandReceipt
/// 📦️ Exact posted-command request ceiling; matches the hub's public administrator request ceiling.
pub const DIRECTORY_COMMAND_REQUEST_MAX_BYTES: usize = 8 * 1024;

/// 📦️ Exact returned-receipt ceiling; matches the administrator response and event-page ceilings.
pub const DIRECTORY_COMMAND_RECEIPT_MAX_BYTES: usize = 64 * 1024;

/// 🔢️ Maximum durable events one directory command may append (`upsert-member` emits at most two).
pub const DIRECTORY_COMMAND_RECEIPT_MAX_EVENTS: usize = 4;

/// 🎟️ Maximum bytes of the one-shot invite capability a receipt may carry.
pub const DIRECTORY_COMMAND_INVITE_TOKEN_MAX_BYTES: usize = 256;

/// 🆔️ Exact hex length of one command-request idempotency correlation.
pub const DIRECTORY_COMMAND_REQUEST_ID_LEN: usize = 32;

/// 🆔️ One sealed, idempotency-correlated directory command posted to `POST /directory/commands`.
/// `request_id` is a correlation, never a capability: knowing it grants nothing, and the hub
/// re-runs authentication and authorization before returning any stored completion.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectoryCommandRequestV1 {
    pub schema: String,
    pub request_id: String,
    pub command: DirectoryCommand,
}

/// 🧾️ Closed disposition of one durable command request. `secret-undeliverable` proves no duplicate
/// was executed while stating honestly that a one-shot capability cannot be re-delivered.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum DirectoryCommandOutcomeV1 {
    Accepted,
    PreviouslyAccepted,
    SecretUndeliverable,
}

/// 🎁️ Closed command-result grammar. The invite capability lives only in the live operation's
/// receipt: it is never appended, broadcast, folded, logged, or persisted by any store.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum DirectoryCommandResultV1 {
    None,
    Invite { invite_token: String },
}

/// 🧾️ One authoritative, receipt-bound completion of exactly one command request.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectoryCommandReceiptV1 {
    pub schema: String,
    pub request_id: String,
    pub command_sha256: String,
    pub outcome: DirectoryCommandOutcomeV1,
    pub events: Vec<DirectoryEvent>,
    pub result: DirectoryCommandResultV1,
    pub receipt_sha256: String,
}

/// 🚫️ Closed command-transport denial classes shared by the hub route and both clients. The first
/// six are the only codes the hub ever puts on the wire; the rest are client-owned terminal or
/// transient transport classes that never carry raw server text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum DirectoryCommandErrorCodeV1 {
    Unauthorized,
    Forbidden,
    StaleSession,
    RequestConflict,
    Invalid,
    Overloaded,
    TooLarge,
    Capacity,
    Closed,
    Cancelled,
    Transport,
}

impl DirectoryCommandErrorCodeV1 {
    /// 🏷️ The exact kebab-case wire spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unauthorized => "unauthorized",
            Self::Forbidden => "forbidden",
            Self::StaleSession => "stale-session",
            Self::RequestConflict => "request-conflict",
            Self::Invalid => "invalid",
            Self::Overloaded => "overloaded",
            Self::TooLarge => "too-large",
            Self::Capacity => "capacity",
            Self::Closed => "closed",
            Self::Cancelled => "cancelled",
            Self::Transport => "transport",
        }
    }

    /// 🌐️ Maps one non-2xx status to its closed hub code without preserving any response body.
    pub fn from_status(status: u16) -> Self {
        match status {
            401 => Self::Unauthorized,
            403 => Self::Forbidden,
            409 => Self::RequestConflict,
            410 => Self::StaleSession,
            413 => Self::TooLarge,
            503 => Self::Overloaded,
            _ => Self::Invalid,
        }
    }

    /// 🔁️ Only transient faults may retry the byte-identical sealed request.
    pub fn is_transient(self) -> bool {
        matches!(self, Self::Overloaded | Self::Transport)
    }
}

pub(crate) fn valid_directory_command_request_id(value: &str) -> bool {
    value.len() == DIRECTORY_COMMAND_REQUEST_ID_LEN && !value.as_bytes().iter().all(|byte| *byte == b'0') && value.as_bytes().iter().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

/// 🆕️ Mints one fresh 32-hex nonzero idempotency correlation from the platform identity boundary.
/// It is a correlation, never a capability: knowing one grants nothing.
pub fn mint_directory_command_request_id() -> String {
    semio_framework_os_identity::time_ordered_id().chars().filter(|character| *character != '-').collect()
}

impl DirectoryCommandRequestV1 {
    /// 🆕️ Seals one request around an already-minted correlation id.
    pub fn new(request_id: impl Into<String>, command: DirectoryCommand) -> Self {
        Self { schema: "semio.directory.command-request.v1".into(), request_id: request_id.into(), command }
    }

    

    

    
}
//#endregion 🔖️CommandReceipt

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
    Indeterminate,
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
#[value(rename_all = "camelCase", retire_with = "std::mem::drop")]
pub struct AdminIntentResultV1 {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub invite_token: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub share_token: Option<String>,
}

impl Drop for AdminIntentResultV1 {
    fn drop(&mut self) {
        for value in [&mut self.invite_token, &mut self.share_token].into_iter().flatten() {
            for byte in unsafe { value.as_bytes_mut() } {
                unsafe { std::ptr::write_volatile(byte, 0) };
            }
        }
    }
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
