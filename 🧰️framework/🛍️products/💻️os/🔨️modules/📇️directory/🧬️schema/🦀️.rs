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

#[path = "🪪️session-authority-v1/🦀️.rs"]
pub mod session_authority;
pub use session_authority::{DIRECTORY_SESSION_AUTHORITY_MAX_BYTES, DirectorySessionAuthorityV1, DirectorySessionKindV1};

#[path = "🌱️space-artifact-creation-v1/🦀️.rs"]
pub mod space_artifact_creation;

#[path = "📇️document-index-v1/🦀️.rs"]
pub mod document_index;
pub use document_index::{DirectoryIndexedDocumentViewV1, DocumentIndexEntryV1};

#[path = "📌️document-check-in-v1/🦀️.rs"]
pub mod document_check_in;
pub use document_check_in::{
    DocumentCheckInPhaseV1, DocumentCheckInProgressV1, DocumentCheckInReadyV1, DocumentCheckInRefusalV1, DocumentCheckInStatusV1, DocumentCheckInV1, DOCUMENT_CHECK_IN_MAX_BYTES, DOCUMENT_CHECK_IN_SCHEMA_V1, DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1,
};

#[path = "🪢️canonical-checkpoint-pair-v1/🦀️.rs"]
pub mod canonical_checkpoint_pair;
pub use canonical_checkpoint_pair::{
    AdmittedCheckpointSelectionV1, CanonicalCheckpointPairRefusalV1, CanonicalCheckpointPairV1,
};

#[path = "🌐️browser-actor/🦀️.rs"]
pub mod browser_actor;
pub use browser_actor::{
    DOCUMENT_BROWSER_ACTOR_INTERFACES, DOCUMENT_BROWSER_ACTOR_MAX_BYTES, DocumentBrowserActorByteLengthV1, DocumentBrowserActorErrorV1, DocumentBrowserActorSourceV1, DocumentExecutionTargetBrowserActorV1, DocumentOpenBrowserActorV1,
};

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

impl crate::ToValue for ArtifactHash {
    fn to_value(&self) -> crate::DslValue {
        crate::DslValue::Array(self.0.iter().map(crate::ToValue::to_value).collect())
    }
}

impl crate::FromValue for ArtifactHash {
    fn from_value(value: crate::DslValue) -> Result<Self, crate::ValueError> {
        let crate::DslValue::Array(items) = value else {
            return Err(crate::ValueError::new(protocol::value::ValueRefusalKind::InvalidValue,format!("expected an array for ArtifactHash, found {value:?}")));
        };
        if items.len() != 32 {
            return Err(crate::ValueError::new(protocol::value::ValueRefusalKind::InvalidValue,format!("expected exactly 32 bytes for ArtifactHash, found {}", items.len())));
        }
        let mut bytes = [0u8; 32];
        for (index, item) in items.into_iter().enumerate() {
            bytes[index] = item.as_u64().and_then(|value| u8::try_from(value).ok()).ok_or_else(|| crate::ValueError::new(protocol::value::ValueRefusalKind::InvalidValue,format!("expected an integer byte at ArtifactHash.{index}")))?;
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

pub(crate) fn directory_event_page_has_control(value: &crate::DslValue) -> bool {
    match value {
        crate::DslValue::String(value) => value.chars().any(char::is_control),
        crate::DslValue::Bytes(_) => true,
        crate::DslValue::Array(values) => values.iter().any(directory_event_page_has_control),
        crate::DslValue::Object(fields) => fields.iter().any(|(key, value)| key.chars().any(char::is_control) || directory_event_page_has_control(value)),
        crate::DslValue::Null | crate::DslValue::Bool(_) | crate::DslValue::Number(_) => false,
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
    crate::os_identity::time_ordered_id().chars().filter(|character| *character != '-').collect()
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

/// 🌐️ Discoverable space metadata. Account identity, caller role, and live activity are
/// structurally absent rather than redacted from [`SpaceView`].
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublicSpaceViewV1 {
    pub id: String,
    pub name: String,
    pub kind: DirectorySpaceKind,
    pub visibility: DirectorySpaceVisibility,
    pub member_count: u32,
    pub document_count: u32,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
}

/// 🔐️ Membership-qualified space metadata. Its required role makes accidental use for
/// anonymous discovery a type error.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberSpaceViewV1 {
    pub id: String,
    pub name: String,
    pub kind: DirectorySpaceKind,
    pub visibility: DirectorySpaceVisibility,
    pub owner_user_id: String,
    pub role: DirectorySpaceRole,
    pub member_count: u32,
    pub document_count: u32,
    pub active_connections: u32,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
}

/// 📖️ Public document catalog identity. Replication frontier/currentness and bootstrap
/// checkpoint state remain private to the authenticated D1 open authority.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublicDocumentCatalogEntryV1 {
    pub document_id: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub owner: DocumentOwner,
    pub pack_schema_hash: String,
}

/// 🔎️ One list entry with an explicit public/member/author authority discriminator.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(tag = "access", rename_all = "lowercase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum DirectorySpaceListEntryV1 {
    Public { space: PublicSpaceViewV1 },
    Member { space: MemberSpaceViewV1 },
    Author { space: MemberSpaceViewV1 },
}

impl DirectorySpaceListEntryV1 {
    /// 🧭️ Checks discriminator-to-role/visibility correlation after wire decoding.
    pub fn validate(&self) -> bool {
        match self {
            Self::Public { space } => space.visibility == DirectorySpaceVisibility::Public,
            Self::Member { space } => space.role == DirectorySpaceRole::Spectator,
            Self::Author { space } => space.role == DirectorySpaceRole::Author,
        }
    }
}

/// 📏️ Maximum rows one administration-page window may carry.
pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS: usize = 64;
/// 📦️ Maximum canonical response bytes of one administration page.
pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES: usize = 48 * 1024;
/// 🔑️ Maximum UTF-8 bytes of one opaque administration cursor.
pub const DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES: usize = 1024;
/// 🏷️ Canonical schema identifier of the bounded space administration page.
pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA: &str = "semio.directory.space-administration-page.v1";

/// 🗂️ The one independently paged window a cursor may advance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectorySpaceAdministrationSectionV1 {
    Members,
    Invites,
    Documents,
}

impl DirectorySpaceAdministrationSectionV1 {
    /// 🔤️ Wire spelling shared by cursor payloads and client requests.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Members => "members",
            Self::Invites => "invites",
            Self::Documents => "documents",
        }
    }

    /// 🔍️ Parses exactly the three closed section names.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "members" => Some(Self::Members),
            "invites" => Some(Self::Invites),
            "documents" => Some(Self::Documents),
            _ => None,
        }
    }
}

/// 🧑️ One administration-page member row; never carries a credential, session, or provider column.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationMemberRowV1 {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub role: DirectorySpaceRole,
    pub owner: bool,
}

/// 🎟️ One administration-page invite row; never carries the selector, secret digest, or capability.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationInviteRowV1 {
    pub invite_id: String,
    pub role: DirectorySpaceRole,
    pub created_at_ms: i64,
    pub expires_at_ms: i64,
    pub revoked: bool,
    pub accepted: bool,
}

/// 🪟️ One bounded member window; `next_cursor` is present exactly when more rows remain.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationMemberWindowV1 {
    pub rows: Vec<DirectorySpaceAdministrationMemberRowV1>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// 🪟️ One bounded invite window; author-only by construction.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationInviteWindowV1 {
    pub rows: Vec<DirectorySpaceAdministrationInviteRowV1>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// 🪟️ One bounded membership-qualified document window.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationDocumentWindowV1 {
    pub rows: Vec<DocumentView>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// 🪟️ One bounded public document-catalog window.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationPublicDocumentWindowV1 {
    pub rows: Vec<PublicDocumentCatalogEntryV1>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// 🛂️ Server-decided administration affordances; the only authority a renderer may consult.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationCapabilitiesV1 {
    pub rename_space: bool,
    pub set_visibility: bool,
    pub delete_space: bool,
    pub upsert_member: bool,
    pub remove_member: bool,
    pub create_invite: bool,
    pub revoke_invite: bool,
}



/// 🏛️ One authenticated, receipt-bound bounded administration projection of exactly one space.
/// Only the `author` shape carries invites and capability flags; `member`/`public` omit them
/// structurally rather than sending empty placeholders.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "access", rename_all = "lowercase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum DirectorySpaceAdministrationPageV1 {
    Public {
        schema: String,
        session_binding_sha256: String,
        authorization_generation: u64,
        space_id: String,
        space: PublicSpaceViewV1,
        documents: DirectorySpaceAdministrationPublicDocumentWindowV1,
        receipt_sha256: String,
    },
    Member {
        schema: String,
        session_binding_sha256: String,
        authorization_generation: u64,
        space_id: String,
        space: MemberSpaceViewV1,
        members: DirectorySpaceAdministrationMemberWindowV1,
        documents: DirectorySpaceAdministrationDocumentWindowV1,
        receipt_sha256: String,
    },
    Author {
        schema: String,
        session_binding_sha256: String,
        authorization_generation: u64,
        space_id: String,
        space: MemberSpaceViewV1,
        members: DirectorySpaceAdministrationMemberWindowV1,
        documents: DirectorySpaceAdministrationDocumentWindowV1,
        invites: DirectorySpaceAdministrationInviteWindowV1,
        capabilities: DirectorySpaceAdministrationCapabilitiesV1,
        receipt_sha256: String,
    },
}

/// 🚫️ Stable bounded administration-page denial classes shared by hub and clients.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectorySpaceAdministrationPageErrorV1 {
    Invalid,
    TooLarge,
    ReceiptMismatch,
}

pub(crate) fn directory_space_administration_cursor_valid(cursor: &Option<String>) -> bool {
    match cursor {
        None => true,
        Some(cursor) => !cursor.is_empty() && cursor.len() <= DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES && cursor.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.')),
    }
}

pub(crate) fn directory_space_administration_text_valid(value: &str) -> bool {
    !value.is_empty() && value.len() <= DOCUMENT_OPEN_ID_MAX_BYTES && !value.chars().any(char::is_control)
}

fn directory_space_administration_time_valid(value: i64) -> bool {
    value >= 0 && (value as u64) <= DOCUMENT_OPEN_MAX_SAFE_INTEGER
}

impl DirectorySpaceAdministrationPageV1 {
    

    

    /// 🧾️ The receipt digest of whichever access shape this page carries.
    pub fn receipt_sha256(&self) -> &str {
        match self {
            Self::Public { receipt_sha256, .. } | Self::Member { receipt_sha256, .. } | Self::Author { receipt_sha256, .. } => receipt_sha256,
        }
    }

    /// 🆔️ The exact space this page projects.
    pub fn space_id(&self) -> &str {
        match self {
            Self::Public { space_id, .. } | Self::Member { space_id, .. } | Self::Author { space_id, .. } => space_id,
        }
    }

    /// 🛂️ Author capabilities, absent for every non-author shape.
    pub fn capabilities(&self) -> Option<DirectorySpaceAdministrationCapabilitiesV1> {
        match self {
            Self::Author { capabilities, .. } => Some(*capabilities),
            _ => None,
        }
    }

    

    
}

pub(crate) fn directory_space_administration_members_valid(window: &DirectorySpaceAdministrationMemberWindowV1) -> bool {
    if window.rows.len() > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS || !directory_space_administration_cursor_valid(&window.next_cursor) {
        return false;
    }
    let mut previous: Option<&str> = None;
    for row in &window.rows {
        if !directory_space_administration_text_valid(&row.user_id)
            || row.email.chars().any(char::is_control)
            || row.display_name.chars().any(char::is_control)
            || row.email.len() > DOCUMENT_OPEN_ID_MAX_BYTES
            || row.display_name.len() > DOCUMENT_OPEN_ID_MAX_BYTES
            || previous.is_some_and(|previous| previous >= row.user_id.as_str())
        {
            return false;
        }
        previous = Some(row.user_id.as_str());
    }
    true
}

pub(crate) fn directory_space_administration_invites_valid(window: &DirectorySpaceAdministrationInviteWindowV1) -> bool {
    if window.rows.len() > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS || !directory_space_administration_cursor_valid(&window.next_cursor) {
        return false;
    }
    let mut previous: Option<(i64, &str)> = None;
    for row in &window.rows {
        if !directory_space_administration_text_valid(&row.invite_id)
            || !directory_space_administration_time_valid(row.created_at_ms)
            || !directory_space_administration_time_valid(row.expires_at_ms)
            || previous.is_some_and(|previous| previous <= (row.created_at_ms, row.invite_id.as_str()))
        {
            return false;
        }
        previous = Some((row.created_at_ms, row.invite_id.as_str()));
    }
    true
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
#[value(rename_all = "camelCase", deny_unknown_fields)]
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

impl DocumentOpenRendererTargetV1 {
    /// 🪞️ Canonical renderer identity for actor validation and transport projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::React => "react",
            Self::Wgpu => "wgpu",
            Self::Wasm => "wasm",
        }
    }
}

/// 👁️ Server-selected document surface authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DocumentOpenSurfaceRoleV1 {
    Viewer,
    Editor,
}

/// 📡️ Exact application-channel ABI claimed by the compiled package descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentExecutionProtocolV1 {
    pub app_channel_version: u32,
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
    pub execution_protocol: DocumentExecutionProtocolV1,
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

/// 🧭️ Complete parent dialect selected from the verified application declaration.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenParentDialectV1 {
    pub artifact_kind: String,
    pub standard: String,
    pub subset: String,
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
    pub parent_dialect: DocumentOpenParentDialectV1,
    pub surface: DocumentOpenSurfaceV1,
    pub browser_actor: DocumentOpenBrowserActorV1,
    pub grant: DocumentOpenGrantV1,
    pub checkpoint: DocumentOpenCheckpointV1,
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

pub(crate) fn valid_document_open_hash(value: &str) -> bool {
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
        secret.len() == 43 && secret.bytes().all(|byte| base64_value(byte).is_some()) && secret.as_bytes().last().and_then(|byte| base64_value(*byte)).is_some_and(|tail| tail & 0b11 == 0)
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
    ///
    /// 🪢 `artifact.kind` and `parent_dialect.artifact_kind` are two DIFFERENT id spaces and are
    /// bounded separately, never against each other: `artifact.kind` is a manifest
    /// `ArtifactKindSpec::id` (`2d.note`, `text.document`, `stdio.json`) and `parent_dialect` is the
    /// owning app's `Dialect` (`s.note.note`, `s.writer.writer`, `s.stdio.json`). Only `gis` spells
    /// them alike, so an equality here is a plan that no plugin but `gis` can ever be issued. Both
    /// are pinned to the trusted catalog's own selection by `validate_descriptor_open_target`, which
    /// requires a manifest kind with exactly `artifact.kind` and an app whose dialect is exactly
    /// `parent_dialect`, so the binding is declared there and not re-derivable from two strings.
    pub fn validate(&self, now_ms: u64) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        self.browser_actor
            .validate(DocumentBrowserActorSourceV1 { component_sha256: &self.package.component_sha256, descriptor_byte_sha256: &self.package.descriptor_byte_sha256 }, self.surface.renderer_target.as_str())
            .map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?;
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
            || [&self.parent_dialect.artifact_kind, &self.parent_dialect.standard, &self.parent_dialect.subset].into_iter().any(|value| !valid_document_open_text(value, DOCUMENT_OPEN_ID_MAX_BYTES) || value.trim() != value.as_str())
            || !valid_document_open_hash(&self.descriptor_digest_v1)
            || !valid_document_open_hash(&self.catalog.generation_id)
            || !valid_document_open_hash(&self.package.component_sha256)
            || !valid_document_open_hash(&self.package.component_blake3)
            || !valid_document_open_hash(&self.package.descriptor_byte_sha256)
            || self.package.execution_protocol.app_channel_version != crate::os_spr::CHANNEL_VERSION
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
        let checkpoint = &self.checkpoint;
        if !checkpoint.baseline_frontier.is_genesis_for(&self.scope)
            && (!valid_document_open_text(&checkpoint.baseline_frontier.head_edit_id, DOCUMENT_OPEN_ID_MAX_BYTES)
                || checkpoint.baseline_frontier.head_edit_ordinal > DOCUMENT_OPEN_MAX_SAFE_INTEGER
                || checkpoint.baseline_frontier.last_commit_seq > DOCUMENT_OPEN_MAX_SAFE_INTEGER)
        {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        if !valid_document_open_hash(&checkpoint.checkpoint_id)
            || checkpoint.descriptor_digest_v1 != self.descriptor_digest_v1
            || !valid_document_open_hash(&checkpoint.aggregate_sha256)
            || checkpoint.baseline_frontier.document_id != self.scope.document_id
            || checkpoint.baseline_frontier.head_edit_ordinal < checkpoint.baseline_frontier.last_commit_seq
            || !(checkpoint.baseline_frontier.is_genesis_for(&self.scope) || checkpoint.baseline_frontier.is_edited_for(&self.scope))
        {
            return Err(DocumentOpenPlanErrorCodeV1::Stale);
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

//#region 🪪️ExecutionTargetLease
/// 🧯️ Exact maximum accepted bytes for one verified execution-target component.
pub const DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES: u64 = 64 * 1024 * 1024;
/// 🧯️ Exact maximum accepted bytes for one verified raw package descriptor.
pub const DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES: u64 = 4 * 1024 * 1024;

/// 🧱️ Exact byte identity of one verified component, bound to the package projection.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentExecutionTargetComponentV1 {
    pub sha256: String,
    pub blake3: String,
    pub byte_length: u64,
}

/// 📜️ Exact byte identity of one verified raw package descriptor.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentExecutionTargetDescriptorV1 {
    pub sha256: String,
    pub byte_length: u64,
}

/// 🪪️ Receipt-free public fields of one document execution-target lease. It never carries a plan
/// receipt, socket grant, session token, hub origin, local path or module URL.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentExecutionTargetLeaseFieldsV1 {
    pub schema: String,
    pub version: u32,
    pub scope: DocumentScope,
    pub descriptor_digest_v1: String,
    pub catalog: DocumentOpenCatalogV1,
    pub package: DocumentOpenPackageV1,
    pub component: DocumentExecutionTargetComponentV1,
    pub descriptor: DocumentExecutionTargetDescriptorV1,
    pub browser_actor: DocumentExecutionTargetBrowserActorV1,
    pub artifact: DocumentOpenArtifactV1,
    pub parent_dialect: DocumentOpenParentDialectV1,
    pub surface: DocumentOpenSurfaceV1,
    pub grant: DocumentOpenGrantV1,
    pub checkpoint: DocumentOpenCheckpointV1,
    pub revalidation: DocumentOpenRevalidationV1,
}

impl DocumentExecutionTargetLeaseFieldsV1 {
    /// ✅ Validates every identity, byte and grant invariant of one receipt-free lease projection.
    ///
    /// 🪢 `artifact.kind` (manifest taxonomy space) and `parent_dialect.artifact_kind` (plugin
    /// dialect space) are bounded separately for the reason `DocumentOpenPlanV1::validate` records;
    /// the lease is projected from a plan that already carries both from one catalog selection.
    pub fn validate(&self) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        self.browser_actor
            .validate(DocumentBrowserActorSourceV1 { component_sha256: &self.package.component_sha256, descriptor_byte_sha256: &self.package.descriptor_byte_sha256 }, self.surface.renderer_target.as_str())
            .map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?;
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
        if self.schema != "semio.os.document-execution-target-lease/v1"
            || self.version != 1
            || ids.iter().any(|value| !valid_document_open_text(value, DOCUMENT_OPEN_ID_MAX_BYTES))
            || [&self.parent_dialect.artifact_kind, &self.parent_dialect.standard, &self.parent_dialect.subset].into_iter().any(|value| !valid_document_open_text(value, DOCUMENT_OPEN_ID_MAX_BYTES) || value.trim() != value.as_str())
            || !valid_document_open_hash(&self.descriptor_digest_v1)
            || !valid_document_open_hash(&self.catalog.generation_id)
            || !valid_document_open_hash(&self.package.component_sha256)
            || !valid_document_open_hash(&self.package.component_blake3)
            || !valid_document_open_hash(&self.package.descriptor_byte_sha256)
            || self.package.execution_protocol.app_channel_version != crate::os_spr::CHANNEL_VERSION
            || !valid_document_open_hash(&self.artifact.pack_schema_hash)
            || !valid_document_open_hash(&self.component.sha256)
            || !valid_document_open_hash(&self.component.blake3)
            || !valid_document_open_hash(&self.descriptor.sha256)
            || self.component.sha256 != self.package.component_sha256
            || self.component.blake3 != self.package.component_blake3
            || self.descriptor.sha256 != self.package.descriptor_byte_sha256
            || self.component.byte_length == 0
            || self.component.byte_length > DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES
            || self.descriptor.byte_length == 0
            || self.descriptor.byte_length > DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES
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
        let checkpoint = &self.checkpoint;
        if !valid_document_open_hash(&checkpoint.checkpoint_id)
            || checkpoint.descriptor_digest_v1 != self.descriptor_digest_v1
            || !valid_document_open_hash(&checkpoint.aggregate_sha256)
            || checkpoint.baseline_frontier.document_id != self.scope.document_id
            || checkpoint.baseline_frontier.head_edit_ordinal < checkpoint.baseline_frontier.last_commit_seq
            || !(checkpoint.baseline_frontier.is_genesis_for(&self.scope) || checkpoint.baseline_frontier.is_edited_for(&self.scope))
        {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        Ok(())
    }
}

/// 🧾 Projects one plan into receipt-free lease fields. The plan constrains every identity but no
/// byte length, so both lengths come from the installation under comparison and are independently
/// enforced against the exact verified bytes before a lease exists.
pub fn lease_fields_from_plan_v1(plan: &DocumentOpenPlanV1, component_byte_length: u64, descriptor_byte_length: u64, browser_actor_byte_length: Option<u64>) -> Result<DocumentExecutionTargetLeaseFieldsV1, DocumentOpenPlanErrorCodeV1> {
    let fields = DocumentExecutionTargetLeaseFieldsV1 {
        schema: "semio.os.document-execution-target-lease/v1".to_string(),
        version: 1,
        scope: plan.scope.clone(),
        descriptor_digest_v1: plan.descriptor_digest_v1.clone(),
        catalog: plan.catalog.clone(),
        package: plan.package.clone(),
        component: DocumentExecutionTargetComponentV1 { sha256: plan.package.component_sha256.clone(), blake3: plan.package.component_blake3.clone(), byte_length: component_byte_length },
        descriptor: DocumentExecutionTargetDescriptorV1 { sha256: plan.package.descriptor_byte_sha256.clone(), byte_length: descriptor_byte_length },
        browser_actor: plan
            .browser_actor
            .to_lease(DocumentBrowserActorSourceV1 { component_sha256: &plan.package.component_sha256, descriptor_byte_sha256: &plan.package.descriptor_byte_sha256 }, plan.surface.renderer_target.as_str(), browser_actor_byte_length)
            .map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?,
        artifact: plan.artifact.clone(),
        parent_dialect: plan.parent_dialect.clone(),
        surface: plan.surface.clone(),
        grant: plan.grant,
        checkpoint: plan.checkpoint.clone(),
        revalidation: plan.revalidation,
    };
    fields.validate()?;
    Ok(fields)
}

/// ⚖️ The one shared full-field lease relation. No transport is permitted a subset comparison.
pub fn same_lease_fields_v1(left: &DocumentExecutionTargetLeaseFieldsV1, right: &DocumentExecutionTargetLeaseFieldsV1) -> bool {
    left == right
}

/// 🌐 Complete localized execution-target status vocabulary, free of origin, path, receipt, grant,
/// digest and user identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum DocumentExecutionTargetStatusCodeV1 {
    Verifying,
    Retrying,
    CatchingUp,
    IntegrityFailed,
    Stale,
    Cancelled,
    RendererUnavailable,
    LinkExpired,
    AccessRevoked,
}

impl DocumentExecutionTargetStatusCodeV1 {
    /// 🗣️ Explicit English and German text; there is no default language.
    pub const fn text(self, locale: DocumentExecutionTargetLocaleV1) -> &'static str {
        match (self, locale) {
            (Self::Verifying, DocumentExecutionTargetLocaleV1::En) => "Verifying document component…",
            (Self::Verifying, DocumentExecutionTargetLocaleV1::De) => "Dokumentkomponente wird überprüft…",
            (Self::Retrying, DocumentExecutionTargetLocaleV1::En) => "The hub is busy. Asking again for the document component…",
            (Self::Retrying, DocumentExecutionTargetLocaleV1::De) => "Der Hub ist ausgelastet. Die Dokumentkomponente wird erneut angefragt…",
            (Self::CatchingUp, DocumentExecutionTargetLocaleV1::En) => "Catching up with the hub…",
            (Self::CatchingUp, DocumentExecutionTargetLocaleV1::De) => "Gleiche mit dem Hub ab…",
            (Self::IntegrityFailed, DocumentExecutionTargetLocaleV1::En) => "The document component could not be verified. Reopen the document.",
            (Self::IntegrityFailed, DocumentExecutionTargetLocaleV1::De) => "Die Dokumentkomponente konnte nicht verifiziert werden. Öffnen Sie das Dokument erneut.",
            (Self::Stale, DocumentExecutionTargetLocaleV1::En) => "The document target changed. Reopen the document.",
            (Self::Stale, DocumentExecutionTargetLocaleV1::De) => "Das Dokumentziel wurde geändert. Öffnen Sie das Dokument erneut.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::En) => "Opening the document was cancelled.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::De) => "Das Öffnen des Dokuments wurde abgebrochen.",
            (Self::RendererUnavailable, DocumentExecutionTargetLocaleV1::En) => "The verified document component is ready, but this renderer is unavailable.",
            (Self::RendererUnavailable, DocumentExecutionTargetLocaleV1::De) => "Die überprüfte Dokumentkomponente ist bereit, aber dieser Renderer ist nicht verfügbar.",
            (Self::LinkExpired, DocumentExecutionTargetLocaleV1::En) => "The connection was lost for too long. Reconnect to keep editing this document.",
            (Self::LinkExpired, DocumentExecutionTargetLocaleV1::De) => "Die Verbindung war zu lange unterbrochen. Verbinden Sie sich erneut, um dieses Dokument weiter zu bearbeiten.",
            (Self::AccessRevoked, DocumentExecutionTargetLocaleV1::En) => "Your access to this document was removed.",
            (Self::AccessRevoked, DocumentExecutionTargetLocaleV1::De) => "Ihr Zugriff auf dieses Dokument wurde entfernt.",
        }
    }

    /// 🔊 Progress announces; every terminal outcome asserts.
    pub const fn aria_role(self) -> &'static str {
        match self {
            Self::Verifying | Self::Retrying | Self::CatchingUp => "status",
            _ => "alert",
        }
    }
}

/// 🌍 Explicit UI language for one execution-target status; callers must choose one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DocumentExecutionTargetLocaleV1 {
    En,
    De,
}
//#endregion 🪪️ExecutionTargetLease



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

/// 🪪️ Validates immutable descriptor authority metadata without encoding or hashing.
pub fn validate_document_descriptor_v1(descriptor: &DocumentDescriptor) -> Result<(), DescriptorDigestError> {
    if descriptor.bootstrap_version == 0 {
        return Err(DescriptorDigestError::InvalidBootstrapVersion);
    }
    if descriptor.bootstrap_frontier.commit_seq > descriptor.bootstrap_frontier.head_seq {
        return Err(DescriptorDigestError::InvalidFrontier);
    }
    for (field, value) in [
        ("space_id", descriptor.space_id.as_str()),
        ("document_id", descriptor.document_id.as_str()),
        ("artifact_kind", descriptor.artifact_kind.as_str()),
        ("artifact_schema", descriptor.artifact_schema.as_str()),
        ("owner.plugin_id", descriptor.owner.plugin_id.as_str()),
        ("owner.package_id", descriptor.owner.package_id.as_str()),
        ("owner.version", descriptor.owner.version.as_str()),
    ] {
        if value.is_empty() { return Err(DescriptorDigestError::EmptyField(field)); }
    }
    for (field, value) in [
        ("owner.package_hash", descriptor.owner.package_hash.as_str()),
        ("pack_schema_hash", descriptor.pack_schema_hash.as_str()),
        ("bootstrap_snapshot_hash", descriptor.bootstrap_snapshot_hash.as_str()),
    ] {
        if value.len() != 64 || !value.chars().all(|digit| matches!(digit, '0'..='9' | 'a'..='f')) || value.chars().all(|digit| digit == '0') {
            return Err(DescriptorDigestError::InvalidHash(field));
        }
    }
    Ok(())
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

impl ArtifactFrontier {
    /// 🌱️ Empty history is exact, scope-bound, and never represented by an invented edit.
    pub fn is_genesis_for(&self, scope: &DocumentScope) -> bool {
        valid_document_open_text(&scope.space_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && valid_document_open_text(&scope.document_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && self.document_id == scope.document_id
            && self.head_edit_ordinal == 0
            && self.head_edit_id.is_empty()
            && self.last_commit_seq == 0
            && self.chain_hash.0 == [0; 32]
    }

    /// 🌿️ An edited frontier has positive counters and a real authenticated history head.
    pub fn is_edited_for(&self, scope: &DocumentScope) -> bool {
        valid_document_open_text(&scope.space_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && self.document_id == scope.document_id
            && valid_document_open_text(&self.document_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && valid_document_open_text(&self.head_edit_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && self.head_edit_ordinal > 0
            && self.head_edit_ordinal <= DOCUMENT_OPEN_MAX_SAFE_INTEGER
            && self.last_commit_seq > 0
            && self.last_commit_seq <= DOCUMENT_OPEN_MAX_SAFE_INTEGER
            && self.chain_hash.0 != [0; 32]
    }
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

/// 🔑️ Which way one reader's own access to a space moved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DirectoryAccessChange {
    Granted,
    Revoked,
}

/// 📡️ One `/directory/socket/v1` text frame (contract C1/C2) — subscribe, then gap-free replay.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "lowercase", rename_all_fields = "camelCase")]
pub enum DirectoryStreamMessage {
    Event {
        event: Box<DirectoryEvent>,
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
    /// 🔑️ The reader's OWN access to one space moved (a membership granted, an invite redeemed, a membership revoked):
    /// the directory events it may read changed retroactively — a space's earlier events became visible or invisible —
    /// so its projection re-reads the directory from the origin. Derived per reader by the hub's directory socket from
    /// the committed membership event; never published on the shared bus.
    #[value(rename = "access-changed")]
    AccessChanged {
        space_id: String,
        change: DirectoryAccessChange,
    },
}
//#endregion 🔖️Stream

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
