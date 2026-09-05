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

/// 📏️ Maximum raw rows represented by one event-page scan.
pub const DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS: usize = 128;
/// 📦️ Maximum canonical response bytes retained by one page owner.
pub const DIRECTORY_EVENT_PAGE_MAX_BYTES: usize = 64 * 1024;
/// ⚡️ Maximum canonical bytes of one persisted event.
pub const DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES: usize = 48 * 1024;

#[derive(Clone, Debug, PartialEq, ToValue)]
#[value(rename_all = "camelCase")]
struct DirectoryEventPageReceiptV1 {
    schema: String,
    session_binding_sha256: String,
    authorization_generation: u64,
    after_seq_exclusive: u64,
    through_seq_inclusive: u64,
    has_more: bool,
    events: Vec<DirectoryEvent>,
}

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

fn directory_event_page_has_control(value: &crate::DslValue) -> bool {
    match value {
        crate::DslValue::String(value) => value.chars().any(char::is_control),
        crate::DslValue::Array(values) => values.iter().any(directory_event_page_has_control),
        crate::DslValue::Object(fields) => fields.iter().any(|(key, value)| key.chars().any(char::is_control) || directory_event_page_has_control(value)),
        crate::DslValue::Null | crate::DslValue::Bool(_) | crate::DslValue::Number(_) => false,
    }
}

/// 🛡️ Admits one fully assigned event into the durable directory log and bounded page protocol.
pub fn validate_directory_event_page_event(event: &DirectoryEvent) -> Result<(), DirectoryEventPageErrorV1> {
    let encoded = crate::os_pack::json::to_json_string(event);
    if event.seq == 0
        || event.seq > DOCUMENT_OPEN_MAX_SAFE_INTEGER
        || encoded.len() > DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES
        || directory_event_page_has_control(&crate::ToValue::to_value(event))
    {
        Err(DirectoryEventPageErrorV1::Invalid)
    } else {
        Ok(())
    }
}

impl DirectoryEventPageV1 {
    /// 🧾️ Returns the canonical UTF-8 JSON covered by `receiptSha256`.
    pub fn canonical_unsigned_json(&self) -> String {
        crate::os_pack::json::to_json_string(&DirectoryEventPageReceiptV1 {
            schema: self.schema.clone(),
            session_binding_sha256: self.session_binding_sha256.clone(),
            authorization_generation: self.authorization_generation,
            after_seq_exclusive: self.after_seq_exclusive,
            through_seq_inclusive: self.through_seq_inclusive,
            has_more: self.has_more,
            events: self.events.clone(),
        })
    }

    /// 🔐️ Verifies the lowercase SHA-256 receipt over the declaration-ordered unsigned page.
    pub fn receipt_matches(&self) -> bool {
        self.receipt_sha256 == semio_framework_hash::sha256_hex(self.canonical_unsigned_json().as_bytes())
    }

    /// ✅️ Checks bounded range, canonical digest, event ordering, and cross-runtime integer laws.
    pub fn validate(&self) -> Result<(), DirectoryEventPageErrorV1> {
        if self.schema != "semio.directory.event-page.v1"
            || !valid_document_open_hash(&self.session_binding_sha256)
            || self.authorization_generation == 0
            || self.authorization_generation > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || self.after_seq_exclusive > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || self.through_seq_inclusive > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || self.after_seq_exclusive > self.through_seq_inclusive
            || self.events.len() > DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS
            || self.receipt_sha256.len() != 64
            || !self.receipt_sha256.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        {
            return Err(DirectoryEventPageErrorV1::Invalid);
        }
        let mut previous = self.after_seq_exclusive;
        for event in &self.events {
            if event.seq <= previous
                || event.seq > self.through_seq_inclusive
                || validate_directory_event_page_event(event).is_err()
            {
                return Err(DirectoryEventPageErrorV1::Invalid);
            }
            previous = event.seq;
        }
        if !self.receipt_matches() {
            return Err(DirectoryEventPageErrorV1::ReceiptMismatch);
        }
        if crate::os_pack::json::to_json_string(self).len() > DIRECTORY_EVENT_PAGE_MAX_BYTES {
            return Err(DirectoryEventPageErrorV1::TooLarge);
        }
        Ok(())
    }

    /// 📥️ Parses exactly one canonical page, rejecting whitespace, trailing bytes, and duplicate or unknown fields.
    pub fn parse_canonical_json(json: &str) -> Result<Self, DirectoryEventPageErrorV1> {
        if json.len() > DIRECTORY_EVENT_PAGE_MAX_BYTES {
            return Err(DirectoryEventPageErrorV1::TooLarge);
        }
        let page: Self = crate::os_pack::json::from_json_str(json).map_err(|_| DirectoryEventPageErrorV1::Invalid)?;
        if crate::os_pack::json::to_json_string(&page) != json {
            return Err(DirectoryEventPageErrorV1::Invalid);
        }
        page.validate()?;
        Ok(page)
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
    AnnounceDocument { descriptor: DocumentDescriptor },
}
//#endregion 🔖️Command

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

#[derive(Clone, Debug, PartialEq, ToValue)]
#[value(rename_all = "camelCase")]
struct DirectoryCommandReceiptUnsignedV1 {
    schema: String,
    request_id: String,
    command_sha256: String,
    outcome: DirectoryCommandOutcomeV1,
    events: Vec<DirectoryEvent>,
    result: DirectoryCommandResultV1,
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

fn valid_directory_command_request_id(value: &str) -> bool {
    value.len() == DIRECTORY_COMMAND_REQUEST_ID_LEN
        && !value.as_bytes().iter().all(|byte| *byte == b'0')
        && value.as_bytes().iter().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

/// 🆕️ Mints one fresh 32-hex nonzero idempotency correlation from the platform identity boundary.
/// It is a correlation, never a capability: knowing one grants nothing.
pub fn mint_directory_command_request_id() -> String {
    crate::os_identity::time_ordered_id().chars().filter(|character| *character != '-').collect()
}

/// 🔐️ The one canonical command digest both the hub and every client derive independently.
pub fn directory_command_sha256(command: &DirectoryCommand) -> String {
    semio_framework_hash::sha256_hex(crate::os_pack::json::to_json_string(command).as_bytes())
}

impl DirectoryCommandRequestV1 {
    /// 🆕️ Seals one request around an already-minted correlation id.
    pub fn new(request_id: impl Into<String>, command: DirectoryCommand) -> Self {
        Self { schema: "semio.directory.command-request.v1".into(), request_id: request_id.into(), command }
    }

    /// 🧾️ Returns the canonical UTF-8 JSON that both peers hash and count bytes over.
    pub fn canonical_json(&self) -> String {
        crate::os_pack::json::to_json_string(self)
    }

    /// ✅️ Checks the closed envelope, the correlation grammar, and the request byte ceiling.
    pub fn validate(&self) -> Result<(), DirectoryCommandErrorCodeV1> {
        if self.schema != "semio.directory.command-request.v1" || !valid_directory_command_request_id(&self.request_id) {
            return Err(DirectoryCommandErrorCodeV1::Invalid);
        }
        if self.canonical_json().len() > DIRECTORY_COMMAND_REQUEST_MAX_BYTES {
            return Err(DirectoryCommandErrorCodeV1::TooLarge);
        }
        Ok(())
    }

    /// 📥️ Parses exactly one canonical request, rejecting padding, unknown fields, and oversize bodies.
    pub fn parse_canonical_json(json: &str) -> Result<Self, DirectoryCommandErrorCodeV1> {
        if json.len() > DIRECTORY_COMMAND_REQUEST_MAX_BYTES {
            return Err(DirectoryCommandErrorCodeV1::TooLarge);
        }
        let request: Self = crate::os_pack::json::from_json_str(json).map_err(|_| DirectoryCommandErrorCodeV1::Invalid)?;
        if request.canonical_json() != json {
            return Err(DirectoryCommandErrorCodeV1::Invalid);
        }
        request.validate()?;
        Ok(request)
    }
}

impl DirectoryCommandReceiptV1 {
    /// 🧾️ Returns the canonical UTF-8 JSON covered by `receiptSha256`.
    pub fn canonical_unsigned_json(&self) -> String {
        crate::os_pack::json::to_json_string(&DirectoryCommandReceiptUnsignedV1 {
            schema: self.schema.clone(),
            request_id: self.request_id.clone(),
            command_sha256: self.command_sha256.clone(),
            outcome: self.outcome,
            events: self.events.clone(),
            result: self.result.clone(),
        })
    }

    /// 🔐️ Seals one completion by hashing its declaration-ordered unsigned canonical JSON.
    pub fn seal(request_id: impl Into<String>, command_sha256: impl Into<String>, outcome: DirectoryCommandOutcomeV1, events: Vec<DirectoryEvent>, result: DirectoryCommandResultV1) -> Self {
        let mut receipt = Self {
            schema: "semio.directory.command-receipt.v1".into(),
            request_id: request_id.into(),
            command_sha256: command_sha256.into(),
            outcome,
            events,
            result,
            receipt_sha256: String::new(),
        };
        receipt.receipt_sha256 = semio_framework_hash::sha256_hex(receipt.canonical_unsigned_json().as_bytes());
        receipt
    }

    /// 🔐️ Verifies the lowercase SHA-256 receipt over the declaration-ordered unsigned completion.
    pub fn receipt_matches(&self) -> bool {
        self.receipt_sha256 == semio_framework_hash::sha256_hex(self.canonical_unsigned_json().as_bytes())
    }

    /// ✅️ Checks the closed envelope, the secret-result rule, event laws, digest, and byte ceiling.
    pub fn validate(&self) -> Result<(), DirectoryCommandErrorCodeV1> {
        let token = match &self.result {
            DirectoryCommandResultV1::None => None,
            DirectoryCommandResultV1::Invite { invite_token } => Some(invite_token.as_str()),
        };
        if self.schema != "semio.directory.command-receipt.v1"
            || !valid_directory_command_request_id(&self.request_id)
            || !valid_document_open_hash(&self.command_sha256)
            || !valid_document_open_hash(&self.receipt_sha256)
            || self.events.len() > DIRECTORY_COMMAND_RECEIPT_MAX_EVENTS
            || token.is_some_and(|token| token.is_empty() || token.len() > DIRECTORY_COMMAND_INVITE_TOKEN_MAX_BYTES || token.chars().any(char::is_control))
            || (self.outcome != DirectoryCommandOutcomeV1::Accepted && token.is_some())
        {
            return Err(DirectoryCommandErrorCodeV1::Invalid);
        }
        let mut previous = 0;
        for event in &self.events {
            if event.seq <= previous || validate_directory_event_page_event(event).is_err() {
                return Err(DirectoryCommandErrorCodeV1::Invalid);
            }
            previous = event.seq;
        }
        if self.outcome != DirectoryCommandOutcomeV1::Accepted && !self.events.is_empty() {
            return Err(DirectoryCommandErrorCodeV1::Invalid);
        }
        if !self.receipt_matches() {
            return Err(DirectoryCommandErrorCodeV1::Invalid);
        }
        if crate::os_pack::json::to_json_string(self).len() > DIRECTORY_COMMAND_RECEIPT_MAX_BYTES {
            return Err(DirectoryCommandErrorCodeV1::TooLarge);
        }
        Ok(())
    }

    /// 📥️ Parses exactly one canonical receipt bound to the request that asked for it.
    pub fn parse_canonical_json(json: &str, request: &DirectoryCommandRequestV1) -> Result<Self, DirectoryCommandErrorCodeV1> {
        if json.len() > DIRECTORY_COMMAND_RECEIPT_MAX_BYTES {
            return Err(DirectoryCommandErrorCodeV1::TooLarge);
        }
        let receipt: Self = crate::os_pack::json::from_json_str(json).map_err(|_| DirectoryCommandErrorCodeV1::Invalid)?;
        if crate::os_pack::json::to_json_string(&receipt) != json
            || receipt.request_id != request.request_id
            || receipt.command_sha256 != directory_command_sha256(&request.command)
        {
            return Err(DirectoryCommandErrorCodeV1::Invalid);
        }
        receipt.validate()?;
        Ok(receipt)
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

#[derive(Clone, Debug, PartialEq, ToValue)]
#[value(tag = "access", rename_all = "lowercase", rename_all_fields = "camelCase")]
enum DirectorySpaceAdministrationReceiptV1 {
    Public {
        schema: String,
        session_binding_sha256: String,
        authorization_generation: u64,
        space_id: String,
        space: PublicSpaceViewV1,
        documents: DirectorySpaceAdministrationPublicDocumentWindowV1,
    },
    Member {
        schema: String,
        session_binding_sha256: String,
        authorization_generation: u64,
        space_id: String,
        space: MemberSpaceViewV1,
        members: DirectorySpaceAdministrationMemberWindowV1,
        documents: DirectorySpaceAdministrationDocumentWindowV1,
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
    },
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

fn directory_space_administration_cursor_valid(cursor: &Option<String>) -> bool {
    match cursor {
        None => true,
        Some(cursor) => {
            !cursor.is_empty()
                && cursor.len() <= DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES
                && cursor.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
        }
    }
}

fn directory_space_administration_text_valid(value: &str) -> bool {
    !value.is_empty() && value.len() <= DOCUMENT_OPEN_ID_MAX_BYTES && !value.chars().any(char::is_control)
}

fn directory_space_administration_time_valid(value: i64) -> bool {
    value >= 0 && (value as u64) <= DOCUMENT_OPEN_MAX_SAFE_INTEGER
}

impl DirectorySpaceAdministrationPageV1 {
    /// 🧾️ Returns the canonical UTF-8 JSON covered by `receiptSha256`.
    pub fn canonical_unsigned_json(&self) -> String {
        let receipt = match self {
            Self::Public { schema, session_binding_sha256, authorization_generation, space_id, space, documents, .. } => DirectorySpaceAdministrationReceiptV1::Public {
                schema: schema.clone(),
                session_binding_sha256: session_binding_sha256.clone(),
                authorization_generation: *authorization_generation,
                space_id: space_id.clone(),
                space: space.clone(),
                documents: documents.clone(),
            },
            Self::Member { schema, session_binding_sha256, authorization_generation, space_id, space, members, documents, .. } => DirectorySpaceAdministrationReceiptV1::Member {
                schema: schema.clone(),
                session_binding_sha256: session_binding_sha256.clone(),
                authorization_generation: *authorization_generation,
                space_id: space_id.clone(),
                space: space.clone(),
                members: members.clone(),
                documents: documents.clone(),
            },
            Self::Author { schema, session_binding_sha256, authorization_generation, space_id, space, members, documents, invites, capabilities, .. } => DirectorySpaceAdministrationReceiptV1::Author {
                schema: schema.clone(),
                session_binding_sha256: session_binding_sha256.clone(),
                authorization_generation: *authorization_generation,
                space_id: space_id.clone(),
                space: space.clone(),
                members: members.clone(),
                documents: documents.clone(),
                invites: invites.clone(),
                capabilities: *capabilities,
            },
        };
        crate::os_pack::json::to_json_string(&receipt)
    }

    /// 🔐️ Verifies the lowercase SHA-256 receipt over the declaration-ordered unsigned page.
    pub fn receipt_matches(&self) -> bool {
        self.receipt_sha256() == semio_framework_hash::sha256_hex(self.canonical_unsigned_json().as_bytes())
    }

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

    /// ✅️ Checks schema, binding, window bounds, ordering, canonical digest, and byte ceiling.
    pub fn validate(&self) -> Result<(), DirectorySpaceAdministrationPageErrorV1> {
        let (schema, binding, generation, space_id) = match self {
            Self::Public { schema, session_binding_sha256, authorization_generation, space_id, .. }
            | Self::Member { schema, session_binding_sha256, authorization_generation, space_id, .. }
            | Self::Author { schema, session_binding_sha256, authorization_generation, space_id, .. } => (schema, session_binding_sha256, *authorization_generation, space_id),
        };
        let anonymous = generation == 0 && binding.bytes().all(|byte| byte == b'0');
        let bound = generation >= 1 && generation <= DOCUMENT_OPEN_MAX_SAFE_INTEGER && !binding.bytes().all(|byte| byte == b'0');
        if schema != DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA
            || !valid_document_open_hash(binding)
            || !valid_document_open_hash(self.receipt_sha256())
            || !directory_space_administration_text_valid(space_id)
            || !(anonymous || bound)
            || (!matches!(self, Self::Public { .. }) && !bound)
        {
            return Err(DirectorySpaceAdministrationPageErrorV1::Invalid);
        }
        let ok = match self {
            Self::Public { space, documents, .. } => {
                space.id == *space_id
                    && space.visibility == DirectorySpaceVisibility::Public
                    && documents.rows.len() <= DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS
                    && directory_space_administration_cursor_valid(&documents.next_cursor)
            }
            Self::Member { space, members, documents, .. } => {
                space.id == *space_id
                    && space.role == DirectorySpaceRole::Spectator
                    && directory_space_administration_members_valid(members)
                    && documents.rows.len() <= DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS
                    && directory_space_administration_cursor_valid(&documents.next_cursor)
            }
            Self::Author { space, members, documents, invites, .. } => {
                space.id == *space_id
                    && space.role == DirectorySpaceRole::Author
                    && directory_space_administration_members_valid(members)
                    && documents.rows.len() <= DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS
                    && directory_space_administration_cursor_valid(&documents.next_cursor)
                    && directory_space_administration_invites_valid(invites)
            }
        };
        if !ok {
            return Err(DirectorySpaceAdministrationPageErrorV1::Invalid);
        }
        if !self.receipt_matches() {
            return Err(DirectorySpaceAdministrationPageErrorV1::ReceiptMismatch);
        }
        if crate::os_pack::json::to_json_string(self).len() > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES {
            return Err(DirectorySpaceAdministrationPageErrorV1::TooLarge);
        }
        Ok(())
    }

    /// 📥️ Parses exactly one canonical page, rejecting whitespace, trailing bytes, and unknown fields.
    pub fn parse_canonical_json(json: &str) -> Result<Self, DirectorySpaceAdministrationPageErrorV1> {
        if json.len() > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES {
            return Err(DirectorySpaceAdministrationPageErrorV1::TooLarge);
        }
        let page: Self = crate::os_pack::json::from_json_str(json).map_err(|_| DirectorySpaceAdministrationPageErrorV1::Invalid)?;
        if crate::os_pack::json::to_json_string(&page) != json {
            return Err(DirectorySpaceAdministrationPageErrorV1::Invalid);
        }
        page.validate()?;
        Ok(page)
    }
}

fn directory_space_administration_members_valid(window: &DirectorySpaceAdministrationMemberWindowV1) -> bool {
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

fn directory_space_administration_invites_valid(window: &DirectorySpaceAdministrationInviteWindowV1) -> bool {
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
            || self.parent_dialect.artifact_kind != self.artifact.kind
            || [&self.parent_dialect.artifact_kind, &self.parent_dialect.standard, &self.parent_dialect.subset]
                .into_iter()
                .any(|value| !valid_document_open_text(value, DOCUMENT_OPEN_ID_MAX_BYTES) || value.trim() != value.as_str())
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
    pub artifact: DocumentOpenArtifactV1,
    pub parent_dialect: DocumentOpenParentDialectV1,
    pub surface: DocumentOpenSurfaceV1,
    pub grant: DocumentOpenGrantV1,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<DocumentOpenCheckpointV1>,
    pub revalidation: DocumentOpenRevalidationV1,
}

impl DocumentExecutionTargetLeaseFieldsV1 {
    /// ✅ Validates every identity, byte and grant invariant of one receipt-free lease projection.
    pub fn validate(&self) -> Result<(), DocumentOpenPlanErrorCodeV1> {
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
            || self.parent_dialect.artifact_kind != self.artifact.kind
            || [&self.parent_dialect.artifact_kind, &self.parent_dialect.standard, &self.parent_dialect.subset]
                .into_iter()
                .any(|value| !valid_document_open_text(value, DOCUMENT_OPEN_ID_MAX_BYTES) || value.trim() != value.as_str())
            || !valid_document_open_hash(&self.descriptor_digest_v1)
            || !valid_document_open_hash(&self.catalog.generation_id)
            || !valid_document_open_hash(&self.package.component_sha256)
            || !valid_document_open_hash(&self.package.component_blake3)
            || !valid_document_open_hash(&self.package.descriptor_byte_sha256)
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
        if let Some(checkpoint) = &self.checkpoint {
            if !valid_document_open_hash(&checkpoint.checkpoint_id)
                || checkpoint.descriptor_digest_v1 != self.descriptor_digest_v1
                || !valid_document_open_hash(&checkpoint.aggregate_sha256)
                || checkpoint.baseline_frontier.document_id != self.scope.document_id
                || checkpoint.baseline_frontier.head_edit_ordinal < checkpoint.baseline_frontier.last_commit_seq
                || checkpoint.baseline_frontier.chain_hash.0 == [0; 32]
            {
                return Err(DocumentOpenPlanErrorCodeV1::Denied);
            }
        }
        Ok(())
    }
}

/// 🧾 Projects one plan into receipt-free lease fields. The plan constrains every identity but no
/// byte length, so both lengths come from the installation under comparison and are independently
/// enforced against the exact verified bytes before a lease exists.
pub fn lease_fields_from_plan_v1(plan: &DocumentOpenPlanV1, component_byte_length: u64, descriptor_byte_length: u64) -> DocumentExecutionTargetLeaseFieldsV1 {
    DocumentExecutionTargetLeaseFieldsV1 {
        schema: "semio.os.document-execution-target-lease/v1".to_string(),
        version: 1,
        scope: plan.scope.clone(),
        descriptor_digest_v1: plan.descriptor_digest_v1.clone(),
        catalog: plan.catalog.clone(),
        package: plan.package.clone(),
        component: DocumentExecutionTargetComponentV1 { sha256: plan.package.component_sha256.clone(), blake3: plan.package.component_blake3.clone(), byte_length: component_byte_length },
        descriptor: DocumentExecutionTargetDescriptorV1 { sha256: plan.package.descriptor_byte_sha256.clone(), byte_length: descriptor_byte_length },
        artifact: plan.artifact.clone(),
        parent_dialect: plan.parent_dialect.clone(),
        surface: plan.surface.clone(),
        grant: plan.grant,
        checkpoint: plan.checkpoint.clone(),
        revalidation: plan.revalidation,
    }
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
    IntegrityFailed,
    Stale,
    Cancelled,
    RendererUnavailable,
}

impl DocumentExecutionTargetStatusCodeV1 {
    /// 🗣️ Explicit English and German text; there is no default language.
    pub const fn text(self, locale: DocumentExecutionTargetLocaleV1) -> &'static str {
        match (self, locale) {
            (Self::Verifying, DocumentExecutionTargetLocaleV1::En) => "Verifying document component…",
            (Self::Verifying, DocumentExecutionTargetLocaleV1::De) => "Dokumentkomponente wird überprüft…",
            (Self::IntegrityFailed, DocumentExecutionTargetLocaleV1::En) => "The document component could not be verified. Reopen the document.",
            (Self::IntegrityFailed, DocumentExecutionTargetLocaleV1::De) => "Die Dokumentkomponente konnte nicht verifiziert werden. Öffnen Sie das Dokument erneut.",
            (Self::Stale, DocumentExecutionTargetLocaleV1::En) => "The document target changed. Reopen the document.",
            (Self::Stale, DocumentExecutionTargetLocaleV1::De) => "Das Dokumentziel wurde geändert. Öffnen Sie das Dokument erneut.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::En) => "Opening the document was cancelled.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::De) => "Das Öffnen des Dokuments wurde abgebrochen.",
            (Self::RendererUnavailable, DocumentExecutionTargetLocaleV1::En) => "The verified document component is ready, but this renderer is unavailable.",
            (Self::RendererUnavailable, DocumentExecutionTargetLocaleV1::De) => "Die überprüfte Dokumentkomponente ist bereit, aber dieser Renderer ist nicht verfügbar.",
        }
    }

    /// 🔊 Progress announces; every terminal outcome asserts.
    pub const fn aria_role(self) -> &'static str {
        match self {
            Self::Verifying => "status",
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

//#region 💡️InferencePort
/// 🧯 Exact maximum accepted bytes for one inference request or approval body.
pub const GIS_MAP_INFERENCE_REQUEST_MAX_BYTES: usize = 1024;
/// 🧯 Exact maximum accepted bytes for one bounded owner-private response body.
pub const GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES: usize = 16 * 1024;
/// 📈 Highest progress cursor the hub's append-only bounded progress table admits.
pub const GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR: u64 = 16;
/// 📃 Highest number of lifecycle events one owner-private page may carry.
pub const GIS_MAP_INFERENCE_EVENT_PAGE_MAX_ITEMS: usize = 8;
/// ⏳ Highest job lifetime the hub admits for one submitted job.
pub const GIS_MAP_INFERENCE_JOB_MAX_LIFETIME_MS: u64 = 120_000;
/// 🔖 The one inference service the GIS Map port may name.
pub const GIS_MAP_INFERENCE_SERVICE_ID: &str = "s.gis.gismap.inference";

/// 📤 The closed client intent one submit carries — a service and a lifetime and nothing else.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceJobRequestV1 {
    pub schema: String,
    pub version: u32,
    pub request_id: String,
    pub service_id: String,
    pub policy_version: u32,
    pub lifetime_ms: u64,
}

/// ✅ The closed body one approval carries; the hash is echoed, never computed by a client.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceApprovalRequestV1 {
    pub schema: String,
    pub version: u32,
    pub job_id: String,
    pub proposal_hash: String,
}

/// 🖥 The hub's own job lifecycle vocabulary, mirrored exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum GisMapInferenceJobStateV1 {
    Accepted,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

/// 🖥 The hub's own proposal lifecycle vocabulary, mirrored exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum GisMapInferenceProposalStateV1 {
    None,
    Offered,
    Approved,
    Stale,
    Cancelled,
}

/// 🧾 The closed receipt one accepted submit returns.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceJobReceiptV1 {
    pub schema: String,
    pub job_id: String,
    pub state: GisMapInferenceJobStateV1,
    pub proposal_state: GisMapInferenceProposalStateV1,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub proposal_hash: Option<String>,
    pub cursor: u64,
    pub expires_at_ms: u64,
}

/// 📈 One owner-private progress row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceProgressV1 {
    pub cursor: u64,
    pub run_epoch: u64,
    pub completed: u64,
    pub total: u64,
    pub at_ms: u64,
}

/// 🗓 One owner-private lifecycle event.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceEventV1 {
    pub ordinal: u64,
    pub kind: String,
    pub at_ms: u64,
}

/// 📃 The owner-private bounded page one events, cancel or poll read returns.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceEventPageV1 {
    pub schema: String,
    pub job_id: String,
    pub state: GisMapInferenceJobStateV1,
    pub proposal_state: GisMapInferenceProposalStateV1,
    pub cancel_requested: bool,
    pub stale: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub proposal_hash: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<GisMapInferencePreviewV1>,
    pub events: Vec<GisMapInferenceEventV1>,
    pub progress: Vec<GisMapInferenceProgressV1>,
    pub next_cursor: u64,
}

/// 🗺 The bounded Hub-validated geometry an owner may inspect before approving a proposal.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferencePreviewV1 {
    pub schema: String,
    pub job_id: String,
    pub proposal_hash: String,
    pub region_id: String,
    pub ring: [[f64; 2]; 5],
}

/// ✅ The closed approval outcome; `applied` is true only after a real committed-WAL witness.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceApprovalReceiptV1 {
    pub schema: String,
    pub job_id: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub proposal_hash: String,
    pub applied: bool,
}

/// 🚦 The complete published failure vocabulary the four authenticated routes may answer with, plus
/// the two a client itself may reach: an indeterminate call and a port refused before any request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum GisMapInferencePortCodeV1 {
    Unavailable,
    Denied,
    NotFound,
    Invalid,
    Bounds,
    Conflict,
    Capacity,
    Expired,
    Cancelled,
    CommitUnavailable,
    Storage,
    Transport,
    LeaseUnverified,
}

impl GisMapInferencePortCodeV1 {
    /// 🏷 The exact published wire code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Unavailable => "inference.unavailable",
            Self::Denied => "inference.denied",
            Self::NotFound => "inference.not-found",
            Self::Invalid => "inference.invalid",
            Self::Bounds => "inference.bounds",
            Self::Conflict => "inference.conflict",
            Self::Capacity => "inference.capacity",
            Self::Expired => "inference.expired",
            Self::Cancelled => "inference.cancelled",
            Self::CommitUnavailable => "approval.commit-unavailable",
            Self::Storage => "inference.storage",
            Self::Transport => "inference.transport",
            Self::LeaseUnverified => "inference.lease-unverified",
        }
    }

    /// 🚦 Maps one exact HTTP status onto the vocabulary; an unmapped status is indeterminate.
    pub const fn from_status(status: u16) -> Self {
        match status {
            400 => Self::Invalid,
            403 => Self::Denied,
            404 => Self::NotFound,
            409 => Self::Conflict,
            410 => Self::Expired,
            413 => Self::Bounds,
            429 => Self::Capacity,
            503 => Self::Unavailable,
            _ => Self::Transport,
        }
    }

    /// 🗣 Explicit English and German text; there is no default language.
    pub const fn text(self, locale: DocumentExecutionTargetLocaleV1) -> &'static str {
        match (self, locale) {
            (Self::Unavailable, DocumentExecutionTargetLocaleV1::En) => "Proposals are unavailable for this document.",
            (Self::Unavailable, DocumentExecutionTargetLocaleV1::De) => "Für dieses Dokument sind keine Vorschläge verfügbar.",
            (Self::Denied, DocumentExecutionTargetLocaleV1::En) => "You may not request proposals for this document.",
            (Self::Denied, DocumentExecutionTargetLocaleV1::De) => "Sie dürfen für dieses Dokument keine Vorschläge anfordern.",
            (Self::NotFound, DocumentExecutionTargetLocaleV1::En) => "This proposal no longer exists.",
            (Self::NotFound, DocumentExecutionTargetLocaleV1::De) => "Dieser Vorschlag existiert nicht mehr.",
            (Self::Invalid, DocumentExecutionTargetLocaleV1::En) => "The request was rejected as malformed.",
            (Self::Invalid, DocumentExecutionTargetLocaleV1::De) => "Die Anfrage wurde als fehlerhaft abgelehnt.",
            (Self::Bounds, DocumentExecutionTargetLocaleV1::En) => "The request exceeded its accepted size.",
            (Self::Bounds, DocumentExecutionTargetLocaleV1::De) => "Die Anfrage hat die zulässige Größe überschritten.",
            (Self::Conflict, DocumentExecutionTargetLocaleV1::En) => "The document changed; request a new proposal.",
            (Self::Conflict, DocumentExecutionTargetLocaleV1::De) => "Das Dokument hat sich geändert; fordern Sie einen neuen Vorschlag an.",
            (Self::Capacity, DocumentExecutionTargetLocaleV1::En) => "Too many proposals are running. Try again shortly.",
            (Self::Capacity, DocumentExecutionTargetLocaleV1::De) => "Es laufen zu viele Vorschläge. Versuchen Sie es in Kürze erneut.",
            (Self::Expired, DocumentExecutionTargetLocaleV1::En) => "This proposal expired before it was approved.",
            (Self::Expired, DocumentExecutionTargetLocaleV1::De) => "Dieser Vorschlag ist vor der Freigabe abgelaufen.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::En) => "The proposal was cancelled.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::De) => "Der Vorschlag wurde abgebrochen.",
            (Self::CommitUnavailable, DocumentExecutionTargetLocaleV1::En) => "The approved proposal could not be committed and was not applied.",
            (Self::CommitUnavailable, DocumentExecutionTargetLocaleV1::De) => "Der freigegebene Vorschlag konnte nicht übernommen werden und wurde nicht angewendet.",
            (Self::Storage, DocumentExecutionTargetLocaleV1::En) => "The proposal service is temporarily unavailable.",
            (Self::Storage, DocumentExecutionTargetLocaleV1::De) => "Der Vorschlagsdienst ist vorübergehend nicht verfügbar.",
            (Self::Transport, DocumentExecutionTargetLocaleV1::En) => "The outcome is unknown. Reopen the document before retrying.",
            (Self::Transport, DocumentExecutionTargetLocaleV1::De) => "Das Ergebnis ist unbekannt. Öffnen Sie das Dokument erneut, bevor Sie es wiederholen.",
            (Self::LeaseUnverified, DocumentExecutionTargetLocaleV1::En) => "This document has no verified execution target, so no proposal can start.",
            (Self::LeaseUnverified, DocumentExecutionTargetLocaleV1::De) => "Dieses Dokument hat kein verifiziertes Ausführungsziel, daher kann kein Vorschlag starten.",
        }
    }
}

/// 💡 The complete rendered lifecycle of one host-owned ephemeral inference port. `Idle` and
/// `Submitting` have no server counterpart, `Approving` is the server's `approval-prepared`, and
/// the four terminals are exactly `Applied | Cancelled | Stale | Failed`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum GisMapInferencePortPhaseV1 {
    Idle,
    Submitting,
    Running,
    Offered,
    Approving,
    Applied,
    Cancelled,
    Stale,
    Failed,
}

impl GisMapInferencePortPhaseV1 {
    /// 🏁 A terminal phase accepts no further server answer — only an explicit clear.
    pub const fn terminal(self) -> bool {
        matches!(self, Self::Applied | Self::Cancelled | Self::Stale | Self::Failed)
    }

    /// 🔊 Work in flight announces politely; every terminal asserts.
    pub const fn aria_role(self) -> &'static str {
        if self.terminal() {
            "alert"
        } else {
            "status"
        }
    }

    /// 🗣 Explicit English and German text; there is no default language.
    pub const fn text(self, locale: DocumentExecutionTargetLocaleV1) -> &'static str {
        match (self, locale) {
            (Self::Idle, DocumentExecutionTargetLocaleV1::En) => "No proposal requested.",
            (Self::Idle, DocumentExecutionTargetLocaleV1::De) => "Kein Vorschlag angefordert.",
            (Self::Submitting, DocumentExecutionTargetLocaleV1::En) => "Requesting a bounds proposal…",
            (Self::Submitting, DocumentExecutionTargetLocaleV1::De) => "Begrenzungsvorschlag wird angefordert…",
            (Self::Running, DocumentExecutionTargetLocaleV1::En) => "Computing the bounds proposal…",
            (Self::Running, DocumentExecutionTargetLocaleV1::De) => "Begrenzungsvorschlag wird berechnet…",
            (Self::Offered, DocumentExecutionTargetLocaleV1::En) => "A bounds proposal is ready for review.",
            (Self::Offered, DocumentExecutionTargetLocaleV1::De) => "Ein Begrenzungsvorschlag liegt zur Prüfung bereit.",
            (Self::Approving, DocumentExecutionTargetLocaleV1::En) => "Waiting for the server to commit the approved proposal…",
            (Self::Approving, DocumentExecutionTargetLocaleV1::De) => "Warten auf die Freigabe des Vorschlags durch den Server…",
            (Self::Applied, DocumentExecutionTargetLocaleV1::En) => "The approved proposal was committed to the document.",
            (Self::Applied, DocumentExecutionTargetLocaleV1::De) => "Der freigegebene Vorschlag wurde im Dokument übernommen.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::En) => "The proposal was cancelled.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::De) => "Der Vorschlag wurde abgebrochen.",
            (Self::Stale, DocumentExecutionTargetLocaleV1::En) => "The document changed while the proposal ran. Request a new one.",
            (Self::Stale, DocumentExecutionTargetLocaleV1::De) => "Das Dokument hat sich während des Vorschlags geändert. Fordern Sie einen neuen an.",
            (Self::Failed, DocumentExecutionTargetLocaleV1::En) => "The proposal did not complete.",
            (Self::Failed, DocumentExecutionTargetLocaleV1::De) => "Der Vorschlag wurde nicht abgeschlossen.",
        }
    }
}

/// 🎛 The complete localized control and region labels; EN and DE are both explicit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum GisMapInferencePortControlV1 {
    Heading,
    Cancel,
    Approve,
    Close,
    Progress,
}

impl GisMapInferencePortControlV1 {
    /// 🗣 Explicit English and German text; there is no default language.
    pub const fn text(self, locale: DocumentExecutionTargetLocaleV1) -> &'static str {
        match (self, locale) {
            (Self::Heading, DocumentExecutionTargetLocaleV1::En) => "Bounds proposal",
            (Self::Heading, DocumentExecutionTargetLocaleV1::De) => "Begrenzungsvorschlag",
            (Self::Cancel, DocumentExecutionTargetLocaleV1::En) => "Cancel proposal",
            (Self::Cancel, DocumentExecutionTargetLocaleV1::De) => "Vorschlag abbrechen",
            (Self::Approve, DocumentExecutionTargetLocaleV1::En) => "Approve proposal",
            (Self::Approve, DocumentExecutionTargetLocaleV1::De) => "Vorschlag freigeben",
            (Self::Close, DocumentExecutionTargetLocaleV1::En) => "Close proposal",
            (Self::Close, DocumentExecutionTargetLocaleV1::De) => "Vorschlag schließen",
            (Self::Progress, DocumentExecutionTargetLocaleV1::En) => "Proposal progress",
            (Self::Progress, DocumentExecutionTargetLocaleV1::De) => "Fortschritt des Vorschlags",
        }
    }
}

/// 💡 Complete renderer-visible state of one document's port — never a receipt, bearer, origin,
/// path, base pack, proposal body or user identity, and never anything persisted into a document.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferencePortStatusV1 {
    pub phase: GisMapInferencePortPhaseV1,
    pub job_id: Option<String>,
    pub cursor: u64,
    pub completed: u64,
    pub total: u64,
    pub proposal_hash: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<GisMapInferencePreviewV1>,
    pub cancel_requested: bool,
    pub code: Option<GisMapInferencePortCodeV1>,
}

impl Default for GisMapInferencePortStatusV1 {
    /// 💤 The one starting value; a document with no port has exactly this.
    fn default() -> Self {
        Self { phase: GisMapInferencePortPhaseV1::Idle, job_id: None, cursor: 0, completed: 0, total: 0, proposal_hash: None, preview: None, cancel_requested: false, code: None }
    }
}

/// 🎬 Every input the port's state machine accepts.
#[derive(Clone, Debug, PartialEq)]
pub enum GisMapInferencePortEventV1 {
    Start,
    LeaseUnverified,
    Receipt(GisMapInferenceJobReceiptV1),
    Page(GisMapInferenceEventPageV1),
    Approve,
    Approval(GisMapInferenceApprovalReceiptV1),
    Cancel,
    Failed(GisMapInferencePortCodeV1),
    Clear,
}

/// 🗺 Projects one exact server page onto a rendered phase. Staleness outranks everything, then the
/// job's own terminal states, then the proposal's.
fn gis_map_inference_server_phase_v1(state: GisMapInferenceJobStateV1, proposal_state: GisMapInferenceProposalStateV1, stale: bool) -> GisMapInferencePortPhaseV1 {
    if stale || proposal_state == GisMapInferenceProposalStateV1::Stale {
        GisMapInferencePortPhaseV1::Stale
    } else if state == GisMapInferenceJobStateV1::Cancelled || proposal_state == GisMapInferenceProposalStateV1::Cancelled {
        GisMapInferencePortPhaseV1::Cancelled
    } else if state == GisMapInferenceJobStateV1::Failed {
        GisMapInferencePortPhaseV1::Failed
    } else if proposal_state == GisMapInferenceProposalStateV1::Approved {
        GisMapInferencePortPhaseV1::Applied
    } else if proposal_state == GisMapInferenceProposalStateV1::Offered || state == GisMapInferenceJobStateV1::Succeeded {
        GisMapInferencePortPhaseV1::Offered
    } else {
        GisMapInferencePortPhaseV1::Running
    }
}

/// 🧮 Total, pure transition — the Rust twin of `reduceGisMapInferencePortV1`. It never fabricates a
/// phase the server has not reported: `Submitting` is only left on an exact receipt, `Cancelled`
/// only on an exact server answer (a Cancel click is recorded as `cancel_requested`, never as an
/// optimistic terminal), `Approving` is only reachable from `Offered`, and an answer for a different
/// job id or after a terminal is ignored outright.
pub fn reduce_gis_map_inference_port_v1(current: &GisMapInferencePortStatusV1, event: &GisMapInferencePortEventV1) -> GisMapInferencePortStatusV1 {
    if matches!(event, GisMapInferencePortEventV1::Clear) {
        return GisMapInferencePortStatusV1::default();
    }
    if current.phase.terminal() {
        return current.clone();
    }
    let mut next = current.clone();
    match event {
        GisMapInferencePortEventV1::Clear => unreachable!(),
        GisMapInferencePortEventV1::Start => {
            if current.phase == GisMapInferencePortPhaseV1::Idle {
                next = GisMapInferencePortStatusV1 { phase: GisMapInferencePortPhaseV1::Submitting, ..GisMapInferencePortStatusV1::default() };
            }
        }
        GisMapInferencePortEventV1::LeaseUnverified => {
            if matches!(current.phase, GisMapInferencePortPhaseV1::Idle | GisMapInferencePortPhaseV1::Submitting) {
                next.phase = GisMapInferencePortPhaseV1::Failed;
                next.preview = None;
                next.code = Some(GisMapInferencePortCodeV1::LeaseUnverified);
            }
        }
        GisMapInferencePortEventV1::Receipt(receipt) => {
            if current.phase == GisMapInferencePortPhaseV1::Submitting {
                next.phase = gis_map_inference_server_phase_v1(receipt.state, receipt.proposal_state, false);
                next.job_id = Some(receipt.job_id.clone());
                next.cursor = receipt.cursor;
                next.proposal_hash = receipt.proposal_hash.clone();
                next.preview = None;
            }
        }
        GisMapInferencePortEventV1::Page(page) => {
            if current.job_id.as_deref() == Some(page.job_id.as_str()) {
                let server = gis_map_inference_server_phase_v1(page.state, page.proposal_state, page.stale);
                let phase = if current.phase == GisMapInferencePortPhaseV1::Approving && !server.terminal() { GisMapInferencePortPhaseV1::Approving } else { server };
                if let Some(latest) = page.progress.last() {
                    next.completed = latest.completed;
                    next.total = latest.total;
                }
                next.phase = phase;
                next.cursor = current.cursor.max(page.next_cursor);
                next.proposal_hash = page.proposal_hash.clone();
                next.preview = if matches!(phase, GisMapInferencePortPhaseV1::Offered | GisMapInferencePortPhaseV1::Approving) { page.preview.clone() } else { None };
                next.cancel_requested = current.cancel_requested || page.cancel_requested;
                if phase == GisMapInferencePortPhaseV1::Failed && next.code.is_none() {
                    next.code = Some(GisMapInferencePortCodeV1::Storage);
                }
            }
        }
        GisMapInferencePortEventV1::Approve => {
            let preview_matches = current.preview.as_ref().zip(current.job_id.as_ref()).zip(current.proposal_hash.as_ref()).is_some_and(|((preview, job_id), proposal_hash)| preview.job_id == *job_id && preview.proposal_hash == *proposal_hash);
            if current.phase == GisMapInferencePortPhaseV1::Offered && preview_matches && !current.cancel_requested {
                next.phase = GisMapInferencePortPhaseV1::Approving;
            }
        }
        GisMapInferencePortEventV1::Approval(receipt) => {
            if current.phase == GisMapInferencePortPhaseV1::Approving && current.job_id.as_deref() == Some(receipt.job_id.as_str()) && current.proposal_hash.as_deref() == Some(receipt.proposal_hash.as_str()) {
                if receipt.applied {
                    next.phase = GisMapInferencePortPhaseV1::Applied;
                } else {
                    next.phase = GisMapInferencePortPhaseV1::Failed;
                    next.code = Some(GisMapInferencePortCodeV1::CommitUnavailable);
                }
                next.preview = None;
            }
        }
        GisMapInferencePortEventV1::Cancel => {
            if current.phase != GisMapInferencePortPhaseV1::Idle {
                next.cancel_requested = true;
            }
        }
        GisMapInferencePortEventV1::Failed(code) => {
            next.phase = if *code == GisMapInferencePortCodeV1::Cancelled { GisMapInferencePortPhaseV1::Cancelled } else { GisMapInferencePortPhaseV1::Failed };
            next.preview = None;
            next.code = Some(*code);
        }
    }
    next
}
//#endregion 💡️InferencePort


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

    #[derive(FromValue)]
    #[value(rename_all = "camelCase")]
    struct DirectoryEventPageFixture {
        valid: DirectoryEventPageV1,
        canonical_unsigned: String,
        expected_receipt_sha256: String,
    }

    #[derive(FromValue)]
    #[value(rename_all = "camelCase")]
    struct DirectoryCommandRequestVector {
        name: String,
        request_id: String,
        command: DirectoryCommand,
        canonical: String,
        command_sha256: String,
    }

    #[derive(FromValue)]
    #[value(rename_all = "camelCase")]
    struct DirectoryCommandReceiptVector {
        name: String,
        request_name: String,
        outcome: DirectoryCommandOutcomeV1,
        canonical: String,
        receipt_sha256: String,
    }

    #[derive(FromValue)]
    #[value(rename_all = "camelCase")]
    struct DirectoryCommandRejectedRequestVector {
        name: String,
        source: String,
        code: String,
    }

    #[derive(FromValue)]
    #[value(rename_all = "camelCase")]
    struct DirectoryCommandRejectedReceiptVector {
        name: String,
        request_name: String,
        source: String,
        code: String,
    }

    #[derive(FromValue)]
    #[value(rename_all = "camelCase")]
    struct DirectoryCommandReceiptFixture {
        requests: Vec<DirectoryCommandRequestVector>,
        receipts: Vec<DirectoryCommandReceiptVector>,
        rejected_requests: Vec<DirectoryCommandRejectedRequestVector>,
        rejected_receipts: Vec<DirectoryCommandRejectedReceiptVector>,
    }

    #[semio_framework_async_macros::async_test]
    async fn directory_command_receipt_v1_matches_language_neutral_vectors_and_rejects_hostiles() {
        let fixture: DirectoryCommandReceiptFixture = crate::os_pack::json::from_json_str(include_str!("../../../🧫️fixtures/📇️directory/🧾️command-receipt-v1.json")).expect("command-receipt fixture decodes");
        let request_of = |name: &str| -> DirectoryCommandRequestV1 {
            let vector = fixture.requests.iter().find(|request| request.name == name).expect("request vector");
            DirectoryCommandRequestV1::new(vector.request_id.clone(), vector.command.clone())
        };
        for vector in &fixture.requests {
            let request = DirectoryCommandRequestV1::new(vector.request_id.clone(), vector.command.clone());
            assert_eq!(request.canonical_json(), vector.canonical, "{} canonical request", vector.name);
            assert_eq!(directory_command_sha256(&vector.command), vector.command_sha256, "{} command digest", vector.name);
            assert_eq!(DirectoryCommandRequestV1::parse_canonical_json(&vector.canonical), Ok(request), "{} round trip", vector.name);
        }
        let mut delivered = 0usize;
        for vector in &fixture.receipts {
            let request = request_of(&vector.request_name);
            let receipt = DirectoryCommandReceiptV1::parse_canonical_json(&vector.canonical, &request).unwrap_or_else(|error| panic!("{} canonical receipt: {error:?}", vector.name));
            assert_eq!(receipt.outcome, vector.outcome, "{} outcome", vector.name);
            assert_eq!(receipt.receipt_sha256, vector.receipt_sha256, "{} receipt digest", vector.name);
            assert_eq!(DirectoryCommandReceiptV1::seal(receipt.request_id.clone(), receipt.command_sha256.clone(), receipt.outcome, receipt.events.clone(), receipt.result.clone()), receipt, "{} seal", vector.name);
            if let DirectoryCommandResultV1::Invite { invite_token } = &receipt.result {
                assert_eq!(receipt.outcome, DirectoryCommandOutcomeV1::Accepted, "{} only a live acceptance carries a capability", vector.name);
                assert!(invite_token.len() <= DIRECTORY_COMMAND_INVITE_TOKEN_MAX_BYTES);
                delivered += 1;
            } else if receipt.outcome != DirectoryCommandOutcomeV1::Accepted {
                assert!(receipt.events.is_empty(), "{} a redacted outcome delivers no events", vector.name);
            }
        }
        assert_eq!(delivered, 1, "exactly one neutral vector proves live one-shot capability delivery");
        for vector in &fixture.rejected_requests {
            let expected = if vector.code == "too-large" { DirectoryCommandErrorCodeV1::TooLarge } else { DirectoryCommandErrorCodeV1::Invalid };
            assert_eq!(DirectoryCommandRequestV1::parse_canonical_json(&vector.source), Err(expected), "{} rejected request", vector.name);
        }
        for vector in &fixture.rejected_receipts {
            let expected = if vector.code == "too-large" { DirectoryCommandErrorCodeV1::TooLarge } else { DirectoryCommandErrorCodeV1::Invalid };
            assert_eq!(DirectoryCommandReceiptV1::parse_canonical_json(&vector.source, &request_of(&vector.request_name)), Err(expected), "{} rejected receipt", vector.name);
        }
        let minted = mint_directory_command_request_id();
        assert!(minted.len() == DIRECTORY_COMMAND_REQUEST_ID_LEN && minted.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')) && minted != "0".repeat(DIRECTORY_COMMAND_REQUEST_ID_LEN));
        assert_ne!(minted, mint_directory_command_request_id());
        for (status, code) in [(401u16, DirectoryCommandErrorCodeV1::Unauthorized), (403, DirectoryCommandErrorCodeV1::Forbidden), (409, DirectoryCommandErrorCodeV1::RequestConflict), (410, DirectoryCommandErrorCodeV1::StaleSession), (413, DirectoryCommandErrorCodeV1::TooLarge), (503, DirectoryCommandErrorCodeV1::Overloaded), (500, DirectoryCommandErrorCodeV1::Invalid)] {
            assert_eq!(DirectoryCommandErrorCodeV1::from_status(status), code);
            assert_eq!(code.is_transient(), matches!(code, DirectoryCommandErrorCodeV1::Overloaded));
        }
        assert!(DirectoryCommandErrorCodeV1::Transport.is_transient() && !DirectoryCommandErrorCodeV1::Cancelled.is_transient() && !DirectoryCommandErrorCodeV1::Capacity.is_transient() && !DirectoryCommandErrorCodeV1::Closed.is_transient());
    }

    #[semio_framework_async_macros::async_test]
    async fn directory_event_page_v1_matches_language_neutral_receipt_and_rejects_hostiles() {
        let fixture: DirectoryEventPageFixture = crate::os_pack::json::from_json_str(include_str!("../../../🧫️fixtures/📇️directory/📃️event-page-v1.json")).expect("event-page fixture decodes");
        assert_eq!(fixture.valid.canonical_unsigned_json(), fixture.canonical_unsigned);
        assert_eq!(semio_framework_hash::sha256_hex(fixture.canonical_unsigned.as_bytes()), fixture.expected_receipt_sha256);
        assert_eq!(fixture.valid.validate(), Ok(()));
        let canonical = crate::os_pack::json::to_json_string(&fixture.valid);
        assert_eq!(DirectoryEventPageV1::parse_canonical_json(&canonical), Ok(fixture.valid.clone()));

        let mut hostile = fixture.valid.clone();
        hostile.session_binding_sha256.make_ascii_uppercase();
        assert_eq!(hostile.validate(), Err(DirectoryEventPageErrorV1::Invalid));
        hostile = fixture.valid.clone();
        hostile.authorization_generation = DOCUMENT_OPEN_MAX_SAFE_INTEGER + 1;
        assert_eq!(hostile.validate(), Err(DirectoryEventPageErrorV1::Invalid));
        hostile = fixture.valid.clone();
        hostile.after_seq_exclusive = hostile.events[0].seq;
        assert_eq!(hostile.validate(), Err(DirectoryEventPageErrorV1::Invalid));
        hostile = fixture.valid.clone();
        hostile.events[0].seq = hostile.after_seq_exclusive;
        assert_eq!(hostile.validate(), Err(DirectoryEventPageErrorV1::Invalid));
        hostile = fixture.valid.clone();
        hostile.receipt_sha256 = "b".repeat(64);
        assert_eq!(hostile.validate(), Err(DirectoryEventPageErrorV1::ReceiptMismatch));
        hostile = fixture.valid.clone();
        if let DirectoryEventBody::SpaceRenamed { name, .. } = &mut hostile.events[0].body {
            name.push('\u{1}');
        }
        assert_eq!(hostile.validate(), Err(DirectoryEventPageErrorV1::Invalid));

        let mut boundary = fixture.valid.events[0].clone();
        if let DirectoryEventBody::SpaceRenamed { name, .. } = &mut boundary.body {
            name.clear();
        }
        let base = crate::os_pack::json::to_json_string(&boundary).len();
        if let DirectoryEventBody::SpaceRenamed { name, .. } = &mut boundary.body {
            *name = "x".repeat(DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES - base);
        }
        assert_eq!(crate::os_pack::json::to_json_string(&boundary).len(), DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES);
        assert_eq!(validate_directory_event_page_event(&boundary), Ok(()));
        if let DirectoryEventBody::SpaceRenamed { name, .. } = &mut boundary.body {
            name.push('x');
        }
        assert_eq!(validate_directory_event_page_event(&boundary), Err(DirectoryEventPageErrorV1::Invalid));

        assert_eq!(DirectoryEventPageV1::parse_canonical_json(&format!("{canonical} ")), Err(DirectoryEventPageErrorV1::Invalid));
        assert_eq!(DirectoryEventPageV1::parse_canonical_json(&canonical.replacen("{\"schema\":", "{\"schema\":\"duplicate\",\"schema\":", 1)), Err(DirectoryEventPageErrorV1::Invalid));
        assert_eq!(DirectoryEventPageV1::parse_canonical_json(&canonical.replacen("{\"schema\":", "{\"unexpected\":true,\"schema\":", 1)), Err(DirectoryEventPageErrorV1::Invalid));
    }

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
        let fixture: DescriptorFixture = crate::os_pack::json::from_json_str(include_str!("../../../🧫️fixtures/📇️directory/🪪️document-descriptor.json")).expect("descriptor fixture decodes");
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
        let fixture: ArtifactAuthorityFixture = crate::os_pack::json::from_json_str(include_str!("../../../🧫️fixtures/📇️directory/🛡️artifact-authority.json")).expect("artifact authority fixture decodes");
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
        let fixture: DocumentOpenPlanFixture = crate::os_pack::json::from_json_str(include_str!("../../../🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("document open plan fixture decodes");
        assert_eq!(hex_lower(&descriptor_digest_v1(&fixture.descriptor).expect("descriptor hashes").0), fixture.descriptor_digest_v1);
        assert_eq!(fixture.intent.validate(), Ok(()));
        assert_eq!(fixture.valid_plan.validate(fixture.now_ms), Ok(()));
        assert_eq!(fixture.valid_plan.parent_dialect.artifact_kind, fixture.valid_plan.artifact.kind);
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
        let mut parent_kind = fixture.valid_plan.clone();
        parent_kind.parent_dialect.artifact_kind = "s.foreign.document".into();
        assert_eq!(parent_kind.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
        let mut parent_control = fixture.valid_plan.clone();
        parent_control.parent_dialect.standard.push('\u{85}');
        assert_eq!(parent_control.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
        let mut parent_trim = fixture.valid_plan.clone();
        parent_trim.parent_dialect.subset = " * ".into();
        assert_eq!(parent_trim.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
        let mut parent_kind_trim = fixture.valid_plan.clone();
        parent_kind_trim.artifact.kind = " s.gis:gismap ".into();
        parent_kind_trim.parent_dialect.artifact_kind = parent_kind_trim.artifact.kind.clone();
        assert_eq!(parent_kind_trim.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
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
