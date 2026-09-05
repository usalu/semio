//! 🗄️ Backend-agnostic os-hub identity/tenancy directory. `HubDirectory` is the single seam every
//! persistence backend (sqlite/postgres/neo4j — see the sibling `🪶️sqlite`/`🐘️postgres`/`🌐️neo4j`
//! component folders, each `#[cfg(feature = "…")]`-gated) implements; `bin.rs` never sees a driver
//! type (sqlx/neo4rs/rusqlite), only this trait and the DTOs in `model` — satisfies the "external
//! libraries stay behind an interface" rule for a trait three backends must share.
//!
//! 🎯️ Design choice (split from the pre-CW6 `HubStorage`): document persistence (snapshots,
//! operations) and content-addressed blobs are no longer this crate's concern — `db::Database`
//! (server-side document authority) and `db`'s own `PayloadStorage` own that now (see `bin.rs`).
//! This module keeps exactly the identity/tenancy surface that has no `db` counterpart: users,
//! spaces, memberships, auth sessions, share tokens, and realtime sync sessions. The former VFS
//! tree (`NodeRecord`/`list_nodes`/`create_node`) was deleted in the space/collection/artifact
//! unification wave — the collection document now replaces the hub-side tree (see
//! `.claude/plans/the-final-goal-for-jolly-spindle.md`'s "Roles/kinds/visibility" design ruling).

//#region 🔖️Error
pub mod error {
    /// @emoji 🧯️ Opaque directory error — never wraps a backend driver's error type, so no `sqlx`/
    /// `neo4rs`/`rusqlite` type ever crosses this crate's public API.
    #[derive(Debug)]
    pub enum DirectoryError {
        NotFound(String),
        Conflict(String),
        Unauthorized,
        Backend(String),
    }

    impl std::fmt::Display for DirectoryError {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::NotFound(detail) => write!(formatter, "not found: {detail}"),
                Self::Conflict(detail) => write!(formatter, "conflict: {detail}"),
                Self::Unauthorized => formatter.write_str("unauthorized"),
                Self::Backend(detail) => write!(formatter, "backend error: {detail}"),
            }
        }
    }

    impl std::error::Error for DirectoryError {}

    pub type DirectoryResult<T> = Result<T, DirectoryError>;
}
//#endregion 🔖️Error

//#region 🔖️Model
pub mod model {
    pub use ::directory::os_directory::DocumentScope;
    use serde::{Deserialize, Serialize};

    /// @emoji 🔗️ A revocable, expiring, anonymous read grant for exactly one space/document.
    /// Only its public selector and fixed digest are durable; the raw capability is returned once.
    #[derive(Clone, Debug, PartialEq)]
    pub struct ShareTokenRecord {
        pub id: String,
        pub selector: String,
        pub secret_digest: [u8; 32],
        pub scope: DocumentScope,
        pub created_at: i64,
        pub expires_at: i64,
        pub revoked_at: Option<i64>,
        pub revoked_reason: Option<String>,
    }

    /// @emoji 🙋️ A platform user — local password login and/or one linked SSO identity. Also the
    /// projection `user.created` folds into (see the module root's `//#region 🔖️Projections` on
    /// each backend).
    #[derive(Clone, Debug, PartialEq)]
    pub struct UserRecord {
        pub id: String,
        pub email: String,
        pub display_name: String,
        pub password_hash: Option<String>,
        pub sso_subject: Option<String>,
        pub sso_provider: Option<String>,
        pub created_at: i64,
    }

    /// @emoji 🏛️ A space: the tenant/workspace unit that owns documents and memberships. `kind`
    /// (`"atelier"|"studio"|"archive"`) and `visibility` (`"private"|"public"`) mirror the
    /// wasm-facing `space` crate's `SpaceKind`/`SpaceVisibility` string-identically — this crate
    /// cannot depend on that crate (server-side binary vs wasm-facing kernel), so the two are kept
    /// in lockstep by hand, same as `SpaceRole` below. Also the projection `space.created`/
    /// `space.renamed`/`space.visibility-changed`/`space.archived`/`space.deleted` fold into.
    #[derive(Clone, Debug, PartialEq)]
    pub struct SpaceRecord {
        pub id: String,
        pub name: String,
        pub owner_user_id: String,
        pub created_at: i64,
        pub kind: String,
        pub visibility: String,
    }

    /// @emoji 🧑️‍🤝️‍🧑️ A space member's permission level, string-identical to the `space` crate's
    /// `SpaceRole { Author, Spectator }` (`"author"`/`"spectator"`) — see `SpaceRecord`'s doc for
    /// why this crate re-declares rather than depends. Distinct from the wire-facing
    /// `directory::os_directory::DirectorySpaceRole` events/commands carry (see this module root's
    /// `//#region 🔖️Wire` for the conversion between the two).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum SpaceRole {
        Author,
        Spectator,
    }

    impl SpaceRole {
        pub fn as_str(&self) -> &'static str {
            match self {
                SpaceRole::Author => "author",
                SpaceRole::Spectator => "spectator",
            }
        }

        pub fn parse(value: &str) -> Option<Self> {
            match value {
                "author" => Some(SpaceRole::Author),
                "spectator" => Some(SpaceRole::Spectator),
                _ => None,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct SpaceMembershipRecord {
        pub space_id: String,
        pub user_id: String,
        pub role: SpaceRole,
        pub created_at: i64,
    }

    /// @emoji 🧭️ How a trusted identity issuer created a durable session.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum AuthSessionKind {
        External,
        DevelopmentLocal,
    }

    impl AuthSessionKind {
        pub fn as_str(self) -> &'static str {
            match self {
                Self::External => "external",
                Self::DevelopmentLocal => "development-local",
            }
        }

        pub fn parse(value: &str) -> Option<Self> {
            match value {
                "external" => Some(Self::External),
                "development-local" => Some(Self::DevelopmentLocal),
                _ => None,
            }
        }
    }

    /// @emoji 🍪️ A digest-only browser login session, distinct from a realtime connection.
    #[derive(Clone, Debug, PartialEq)]
    pub struct AuthSessionRecord {
        pub id: String,
        pub selector: String,
        pub secret_digest: [u8; 32],
        pub user_id: String,
        pub identity_provider: String,
        pub identity_subject_digest: [u8; 32],
        pub issued_at: i64,
        pub expires_at: i64,
        pub revoked_at: Option<i64>,
        pub revoked_reason: Option<String>,
        pub authorization_generation: u64,
        pub device_instance_id: String,
        pub session_kind: AuthSessionKind,
    }

    /// @emoji 🎁️ A newly issued session plus its one-time plaintext capability.
    pub struct IssuedAuthSession {
        pub record: AuthSessionRecord,
        pub capability: super::SessionCapability,
    }

    /// @emoji 🧹️ Durable revocation identity returned before connection kicks are attempted.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RevokedAuthSession {
        pub id: String,
        pub authorization_generation: u64,
        pub revoked_at: i64,
    }

    /// @emoji 🏭️ Validated input for digest-only session issuance.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct AuthSessionIssue {
        pub user_id: String,
        pub identity_provider: String,
        pub identity_subject_digest: [u8; 32],
        pub ttl_secs: i64,
        pub device_instance_id: String,
        pub session_kind: AuthSessionKind,
        pub correlation_id: String,
        pub peer_class: String,
    }

    /// @emoji 🔴️ A realtime document connection — the "session as live-features backend" record;
    /// written by `bin.rs`'s wire-v2 WS handler on Hello/disconnect, not per-operation. Not
    /// event-sourced (contract's decider laws) — `space_id`/`surface` widen this record so the
    /// admin overview and presence roster can key/filter by them without joining back to `db`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct SyncSessionRecord {
        pub id: String,
        pub auth_session_id: Option<String>,
        pub authorization_generation: u64,
        pub actor_id: String,
        pub space_id: String,
        pub document_id: String,
        pub surface: String,
        pub user_id: Option<String>,
        pub authenticated_email: Option<String>,
        pub space_role: Option<SpaceRole>,
        pub client_label: String,
        pub connected_at: i64,
        pub disconnected_at: Option<i64>,
    }

    /// @emoji 🎟️ An outstanding (or revoked) space invite. Not event-sourced itself — only its
    /// `invite.redeemed` outcome is (contract's decider laws). Raw capability bytes are never
    /// retained in this read model or the event log.
    #[derive(Clone, Debug, PartialEq)]
    pub struct InviteRecord {
        pub id: String,
        pub selector: String,
        pub secret_digest: [u8; 32],
        pub space_id: String,
        pub role: SpaceRole,
        pub created_at: i64,
        pub expires_at: i64,
        pub revoked_at: Option<i64>,
        pub revoked_reason: Option<String>,
        pub accepted_at: Option<i64>,
        pub accepted_event_id: Option<String>,
    }

    /// 🧑️ One backend-projected administration member row. Display columns only: no password
    /// hash, SSO subject/provider, or session column ever crosses the backend boundary here.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SpaceAdministrationMemberRow {
        pub user_id: String,
        pub email: String,
        pub display_name: String,
        pub role: SpaceRole,
    }

    /// 🎟️ One backend-projected administration invite row. Metadata only: the selector, secret
    /// digest, revoke reason, and accepted event id are structurally absent, not redacted later.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SpaceAdministrationInviteRow {
        pub invite_id: String,
        pub role: SpaceRole,
        pub created_at_ms: i64,
        pub expires_at_ms: i64,
        pub revoked: bool,
        pub accepted: bool,
    }

    /// 🎟️ Closed durable result of one backend-owned invitation claim.
    #[derive(Clone, Debug)]
    pub enum InviteRedemptionCommit {
        NewlyCommitted { event: ::directory::os_directory::DirectoryEvent },
        AlreadyCommitted { event: ::directory::os_directory::DirectoryEvent },
    }

    /// @emoji 🎁️ A newly issued document share plus its one-time plaintext capability.
    pub struct IssuedShareToken {
        pub record: ShareTokenRecord,
        pub capability: super::ShareCapability,
    }

    /// @emoji 🪪️ Current durable status for an id-bound session socket subject.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SocketSessionBindingStatus {
        Active { role: Option<SpaceRole>, expires_at_ms: i64 },
        Revoked,
        Expired,
        MembershipLost,
        Unavailable,
    }

    /// @emoji 🔗️ Current durable status for an id-and-selector-bound share socket subject.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SocketShareBindingStatus {
        Active { expires_at_ms: i64 },
        Revoked,
        Expired,
        Unavailable,
    }

    /// @emoji 🎁️ A newly issued invite plus its one-time plaintext capability.
    pub struct IssuedInvite {
        pub record: InviteRecord,
        pub capability: super::InviteCapability,
    }

    /// @emoji 🧾️ Privacy-minimized append-only authentication audit entry.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct AuthAuditRecord {
        pub id: String,
        pub occurred_at: i64,
        pub event_kind: String,
        pub auth_session_id: Option<String>,
        pub target_user_id: Option<String>,
        pub actor_user_id: Option<String>,
        pub provider: Option<String>,
        pub outcome_code: String,
        pub reason_code: Option<String>,
        pub correlation_id: String,
        pub peer_class: String,
    }

    /// @emoji 🧾️ One append-only administrator operation fact before backend sequence assignment.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NewAdminOperationAuditRecord {
        pub request_id: String,
        pub intent_digest: String,
        pub operation_id: String,
        pub occurred_at: i64,
        pub phase: String,
        pub intent_kind: String,
        pub target_kind: String,
        pub target_id: String,
        pub principal_user_id: String,
        pub principal_session_id: String,
        pub principal_generation: u64,
        pub correlation_id: String,
        pub event_seq_first: Option<u64>,
        pub event_seq_last: Option<u64>,
        pub outcome_code: String,
        pub reason_code: Option<String>,
    }

    /// @emoji 📜️ One durable, backend-ordered administrator operation audit fact.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct AdminOperationAuditRecord {
        pub sequence: u64,
        pub fact: NewAdminOperationAuditRecord,
    }

    /// 🎁️ Closed durable result class of one directory command; a capability plaintext is never stored.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DirectoryCommandResultKindV1 {
        None,
        Invite,
    }

    /// 🧾️ Closed durable lifecycle of one idempotency key. `Pending` is the crash/in-flight window
    /// between the claim and the durable completion; it never re-executes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DirectoryCommandDispositionV1 {
        Pending,
        Completed,
    }

    /// 🆕️ One `(actor, request id)` idempotency claim before the backend records it.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NewDirectoryCommandReceipt {
        pub actor_user_id: String,
        pub request_id: String,
        pub command_sha256: String,
        pub result_kind: DirectoryCommandResultKindV1,
        pub claimed_at: i64,
    }

    /// 🧾️ The durable completion written before any event publication.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct DirectoryCommandReceiptCompletion {
        pub actor_user_id: String,
        pub request_id: String,
        pub event_seq_first: Option<u64>,
        pub event_seq_last: Option<u64>,
        pub receipt_sha256: String,
        pub completed_at: i64,
    }

    /// 🧾️ One durable per-actor command idempotency row. It carries the command digest, result
    /// class, event range, disposition, and the canonical redacted-replay receipt digest — never a
    /// capability plaintext.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct DirectoryCommandReceiptRecord {
        pub actor_user_id: String,
        pub request_id: String,
        pub command_sha256: String,
        pub result_kind: DirectoryCommandResultKindV1,
        pub disposition: DirectoryCommandDispositionV1,
        pub event_seq_first: Option<u64>,
        pub event_seq_last: Option<u64>,
        pub receipt_sha256: Option<String>,
        pub claimed_at: i64,
        pub completed_at: Option<i64>,
    }

    /// 🔐️ Closed outcome of one atomic claim-or-read against the durable receipt store.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum DirectoryCommandClaimV1 {
        Claimed(DirectoryCommandReceiptRecord),
        Existing(DirectoryCommandReceiptRecord),
        Conflict,
    }

    /// 🔢️ Constant-space administrator overview projection owned by the backend.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct AdminDirectoryOverviewCounts {
        pub spaces: u64,
        pub users: u64,
        pub connections: u64,
    }

    /// 🏠️ One storage-folded administrator space-list row.
    #[derive(Clone, Debug, PartialEq)]
    pub struct AdminSpaceSummaryRecord {
        pub space: SpaceRecord,
        pub member_count: u64,
        pub document_count: u64,
        pub active_connections: u64,
        pub updated_at: i64,
    }
}
//#endregion 🔖️Model

use crate::artifact_authority::chunk_cas::{ArtifactCasDeleteFence, ArtifactCasDeleteOutcome, ArtifactCasObjectKey, ArtifactCasOwnershipPlanV1, ArtifactCasReservation, ArtifactChunkCasStorage};
use directory::os_directory::{
    descriptor_digest_v1, ArtifactBlobRef, ArtifactCheckpoint, ArtifactFrontier, ArtifactHash, ArtifactRetention, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryCommandOutcomeV1, DirectoryCommandReceiptV1, DirectoryCommandResultV1, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole,
    DirectorySpaceVisibility, DirectoryStreamMessage, DocumentDescriptor, Hlc, PublishedArtifactBlob, PublishedArtifactCheckpoint,
};
use directory::os_identity::time_ordered_id;
use error::{DirectoryError, DirectoryResult};
use model::*;
use semio_framework_hash::Sha256;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// 🧯️ Immutable per-document checkpoint-lineage ceiling shared by every backend.
pub const ARTIFACT_CHECKPOINT_LINEAGE_MAX: u64 = 16_384;
/// 🧯️ Immutable full-directory replay ceiling; exceeding it requires an operator repair.
pub const DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS: u64 = 1_000_000;
/// 📖️ Immutable maximum number of public directory events returned by one read.
pub const DIRECTORY_EVENT_READ_MAX: usize = 10_000;
pub const ACTIVE_SYNC_SESSION_READ_MAX: usize = 4_096;
/// 🛡️ Exact public administrator request-body ceiling.
pub const ADMIN_INTENT_REQUEST_MAX_BYTES: usize = 8 * 1024;
/// 📄 Exact administrator query page ceiling.
pub const ADMIN_PAGE_MAX: usize = 100;
/// 📄️ One public page plus one private continuation probe.
pub const ADMIN_PAGE_FETCH_MAX: usize = ADMIN_PAGE_MAX + 1;
/// 🛡️ Exact serialized administrator response ceiling.
pub const ADMIN_RESPONSE_MAX_BYTES: usize = 64 * 1024;
/// 🏛️ Exact rows one space-administration page window returns.
pub const SPACE_ADMINISTRATION_PAGE_MAX: usize = 64;
/// 🏛️ One public window plus one private continuation probe.
pub const SPACE_ADMINISTRATION_PAGE_FETCH_MAX: usize = SPACE_ADMINISTRATION_PAGE_MAX + 1;
/// 🌐️ Largest exact integer shared by the Rust, JSON, and TypeScript contracts.
pub const DIRECTORY_WIRE_INTEGER_MAX: u64 = 9_007_199_254_740_991;
/// 🔑️ Fixed UTF-8 byte ceiling for one backend-private immutable blob locator.
pub const ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES: usize = 4_096;
/// 🧯️ Maximum private ownership journal rows returned by one sweep read.
pub const ARTIFACT_CAS_SWEEP_PAGE_MAX: usize = 16;
/// 🧯️ Maximum physical objects considered by one sweep request.
pub const ARTIFACT_CAS_SWEEP_OBJECT_MAX: usize = 4_096;
/// ⏳️ Maximum wall-clock lifetime of one private pre-write reservation.
pub const ARTIFACT_CAS_RESERVATION_MAX_TTL_MS: u64 = 300_000;
/// 🛡️ Crash-recoverable lifetime of one durable per-space physical-deletion lease.
pub const ARTIFACT_CAS_DELETE_LEASE_TTL_MS: u64 = 5_000;
const ARTIFACT_CAS_SWEEP_CONTINUATION_DOMAIN_V1: &[u8] = b"semio.hub.artifact-cas.sweep-continuation.v1\0";
const ARTIFACT_CAS_DELETE_LEASE_DOMAIN_V1: &[u8] = b"semio.hub.artifact-cas.delete-lease.v1\0";
const ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES: usize = 21;
const ARTIFACT_CAS_SWEEP_CONTINUATION_BYTES: usize = ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES + 32;

/// 🗂️ One bounded page of private append-only reachability inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactCasSweepCandidatePage {
    pub observed_generation: u64,
    pub next_generation: u64,
    pub objects: Vec<ArtifactCasObjectKey>,
}

/// 🎫️ Server-instance-bound opaque position within one immutable ledger generation.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ArtifactCasSweepContinuation([u8; ARTIFACT_CAS_SWEEP_CONTINUATION_BYTES]);

impl fmt::Debug for ArtifactCasSweepContinuation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ArtifactCasSweepContinuation(<opaque>)")
    }
}

/// 🧹️ Host intent for one bounded sweep; dry-run is the only default.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactCasSweepRequest {
    pub execute: bool,
    pub max_objects: usize,
    pub continuation: Option<ArtifactCasSweepContinuation>,
}

impl Default for ArtifactCasSweepRequest {
    fn default() -> Self {
        Self { execute: false, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }
    }
}

/// 📊️ Locator-free bounded sweep result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactCasSweepResult {
    pub observed_generation: u64,
    pub final_generation: u64,
    pub examined_objects: u64,
    pub protected_objects: u64,
    pub eligible_objects: u64,
    pub deleted_objects: u64,
    pub missing_objects: u64,
    pub result_digest: ArtifactHash,
    pub continuation: Option<ArtifactCasSweepContinuation>,
}

/// 📈️ Bounded projection-rebuild progress emitted after every replayed event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProjectionRebuildProgress {
    pub completed_events: u64,
    pub total_events: u64,
}

/// 🎛️ Host-owned cancellation/progress seam for a potentially expensive full-log replay.
pub trait ProjectionRebuildControl: Send + Sync {
    fn is_cancelled(&self) -> bool;
    fn report(&self, progress: ProjectionRebuildProgress);
}

pub(crate) struct UncontrolledProjectionRebuild;

impl ProjectionRebuildControl for UncontrolledProjectionRebuild {
    fn is_cancelled(&self) -> bool {
        false
    }

    fn report(&self, _progress: ProjectionRebuildProgress) {}
}

pub(crate) static UNCONTROLLED_PROJECTION_REBUILD: UncontrolledProjectionRebuild = UncontrolledProjectionRebuild;

pub(crate) fn checkpoint_projection_rebuild(control: &dyn ProjectionRebuildControl, completed_events: u64, total_events: u64) -> DirectoryResult<()> {
    if total_events > DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS {
        return Err(DirectoryError::Conflict(format!("directory projection rebuild exceeds fixed maximum {DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS}")));
    }
    control.report(ProjectionRebuildProgress { completed_events, total_events });
    if control.is_cancelled() {
        return Err(DirectoryError::Conflict("directory projection rebuild cancelled".into()));
    }
    Ok(())
}

pub(crate) fn bounded_event_read(since_seq: u64, limit: usize) -> DirectoryResult<(i64, i64)> {
    if since_seq > DIRECTORY_WIRE_INTEGER_MAX || limit == 0 || limit > DIRECTORY_EVENT_READ_MAX {
        return Err(DirectoryError::Conflict(format!("directory event read requires since <= {DIRECTORY_WIRE_INTEGER_MAX} and limit 1..={DIRECTORY_EVENT_READ_MAX}")));
    }
    Ok((i64::try_from(since_seq).map_err(|error| DirectoryError::Conflict(error.to_string()))?, i64::try_from(limit).map_err(|error| DirectoryError::Conflict(error.to_string()))?))
}

//#region 🔖️Capabilities
pub const CAPABILITY_SELECTOR_BYTES: usize = 16;
pub const CAPABILITY_SECRET_BYTES: usize = 32;
pub const CAPABILITY_MAX_TTL_SECS: i64 = 31_536_000;
pub const DEVICE_INSTANCE_MAX_BYTES: usize = 128;
pub const AUTH_ASSERTION_MAX_BYTES: usize = 16 * 1024;
pub const AUTH_TEXT_MAX_BYTES: usize = 256;

/// @emoji 🔐️ Encodes capability bytes without a runtime dependency.
pub fn encode_capability_bytes(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn decode_capability_bytes<const N: usize>(encoded: &str) -> DirectoryResult<[u8; N]> {
    if encoded.len() != N * 2 || !encoded.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
        return Err(DirectoryError::Unauthorized);
    }
    let mut decoded = [0u8; N];
    for (index, value) in decoded.iter_mut().enumerate() {
        let digit = |byte: u8| if byte.is_ascii_digit() { byte - b'0' } else { byte - b'a' + 10 };
        *value = (digit(encoded.as_bytes()[index * 2]) << 4) | digit(encoded.as_bytes()[index * 2 + 1]);
    }
    Ok(decoded)
}

pub(crate) fn decode_auth_digest_hex(encoded: &str) -> DirectoryResult<[u8; 32]> {
    decode_capability_bytes(encoded)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapabilityKind {
    Session,
    Share,
    Invite,
    Socket,
}

impl CapabilityKind {
    fn prefix(self) -> &'static str {
        match self {
            Self::Session => "session.v1",
            Self::Share => "share.v1",
            Self::Invite => "invite.v1",
            Self::Socket => "socket.v1",
        }
    }

    fn digest_domain(self) -> &'static [u8] {
        match self {
            Self::Session => b"semio/hub/session/v1\0",
            Self::Share => b"semio/hub/share/v1\0",
            Self::Invite => b"semio/hub/invite/v1\0",
            Self::Socket => b"semio/hub/socket/v1\0",
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct CapabilityParts {
    selector: String,
    secret: [u8; CAPABILITY_SECRET_BYTES],
}

fn parse_capability_parts(encoded: &str, kind: CapabilityKind) -> DirectoryResult<CapabilityParts> {
    let mut components = encoded.split('.');
    let expected_type = match kind {
        CapabilityKind::Session => "session",
        CapabilityKind::Share => "share",
        CapabilityKind::Invite => "invite",
        CapabilityKind::Socket => "socket",
    };
    let Some(actual_type) = components.next() else { return Err(DirectoryError::Unauthorized) };
    if actual_type != expected_type || components.next() != Some("v1") {
        return Err(DirectoryError::Unauthorized);
    }
    let selector = components.next().ok_or(DirectoryError::Unauthorized)?;
    let secret = components.next().ok_or(DirectoryError::Unauthorized)?;
    if components.next().is_some() {
        return Err(DirectoryError::Unauthorized);
    }
    decode_capability_bytes::<CAPABILITY_SELECTOR_BYTES>(selector)?;
    Ok(CapabilityParts { selector: selector.to_string(), secret: decode_capability_bytes(secret)? })
}

fn mint_capability_parts() -> DirectoryResult<CapabilityParts> {
    let mut entropy = [0u8; CAPABILITY_SELECTOR_BYTES + CAPABILITY_SECRET_BYTES];
    directory::os_identity::fill_entropy(&mut entropy).map_err(|_| DirectoryError::Backend("operating-system credential entropy unavailable".into()))?;
    let selector = encode_capability_bytes(&entropy[..CAPABILITY_SELECTOR_BYTES]);
    let mut secret = [0u8; CAPABILITY_SECRET_BYTES];
    secret.copy_from_slice(&entropy[CAPABILITY_SELECTOR_BYTES..]);
    entropy.fill(0);
    Ok(CapabilityParts { selector, secret })
}

fn capability_digest(kind: CapabilityKind, secret: &[u8; CAPABILITY_SECRET_BYTES]) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(kind.digest_domain());
    hash.update(secret);
    hash.finalize()
}

pub fn constant_time_digest_eq(left: &[u8; 32], right: &[u8; 32]) -> bool {
    let mut difference = 0u8;
    for index in 0..32 {
        difference |= left[index] ^ right[index];
    }
    difference == 0
}

macro_rules! capability_type {
    ($name:ident, $kind:expr) => {
        #[derive(Clone, PartialEq, Eq)]
        pub struct $name(CapabilityParts);

        impl $name {
            pub fn parse(encoded: &str) -> DirectoryResult<Self> {
                parse_capability_parts(encoded, $kind).map(Self)
            }

            pub fn mint() -> DirectoryResult<Self> {
                mint_capability_parts().map(Self)
            }

            pub fn selector(&self) -> &str {
                &self.0.selector
            }

            pub fn secret_digest(&self) -> [u8; 32] {
                capability_digest($kind, &self.0.secret)
            }

            pub fn expose_once(&self) -> String {
                format!("{}.{}.{}", $kind.prefix(), self.0.selector, encode_capability_bytes(&self.0.secret))
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.debug_struct(stringify!($name)).field("selector", &self.0.selector).field("secret", &"[REDACTED]").finish()
            }
        }
    };
}

capability_type!(SessionCapability, CapabilityKind::Session);
capability_type!(ShareCapability, CapabilityKind::Share);
capability_type!(InviteCapability, CapabilityKind::Invite);
capability_type!(SocketGrantCapability, CapabilityKind::Socket);

pub enum HubCapability {
    Session(SessionCapability),
    Share(ShareCapability),
    Invite(InviteCapability),
}

impl HubCapability {
    pub fn parse(encoded: &str) -> DirectoryResult<Self> {
        if encoded.starts_with("session.") {
            SessionCapability::parse(encoded).map(Self::Session)
        } else if encoded.starts_with("share.") {
            ShareCapability::parse(encoded).map(Self::Share)
        } else if encoded.starts_with("invite.") {
            InviteCapability::parse(encoded).map(Self::Invite)
        } else {
            Err(DirectoryError::Unauthorized)
        }
    }
}

/// @emoji ⏳️ Validates a bounded positive TTL and returns its overflow-safe millisecond window.
pub fn capability_window(now: i64, ttl_secs: i64) -> DirectoryResult<(i64, i64)> {
    if !(1..=CAPABILITY_MAX_TTL_SECS).contains(&ttl_secs) {
        return Err(DirectoryError::Conflict(format!("capability ttl must be 1..={CAPABILITY_MAX_TTL_SECS}")));
    }
    let ttl_ms = ttl_secs.checked_mul(1_000).ok_or_else(|| DirectoryError::Conflict("capability ttl overflow".into()))?;
    let expires_at = now.checked_add(ttl_ms).ok_or_else(|| DirectoryError::Conflict("capability expiry overflow".into()))?;
    Ok((now, expires_at))
}

pub fn validate_bounded_auth_text(value: &str, field: &str, maximum: usize) -> DirectoryResult<()> {
    if value.is_empty() || value.len() > maximum {
        return Err(DirectoryError::Conflict(format!("{field} must be 1..={maximum} UTF-8 bytes")));
    }
    Ok(())
}

pub fn identity_subject_digest(provider: &str, subject: &str) -> DirectoryResult<[u8; 32]> {
    validate_bounded_auth_text(provider, "identity provider", AUTH_TEXT_MAX_BYTES)?;
    validate_bounded_auth_text(subject, "identity subject", AUTH_TEXT_MAX_BYTES)?;
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/identity-subject/v1\0");
    hash.update(&(provider.len() as u32).to_be_bytes());
    hash.update(provider.as_bytes());
    hash.update(&(subject.len() as u32).to_be_bytes());
    hash.update(subject.as_bytes());
    Ok(hash.finalize())
}

#[derive(Clone)]
pub struct IdentityAssertion(Box<[u8]>);

impl IdentityAssertion {
    pub fn new(bytes: impl Into<Box<[u8]>>) -> DirectoryResult<Self> {
        let bytes = bytes.into();
        if bytes.is_empty() || bytes.len() > AUTH_ASSERTION_MAX_BYTES {
            return Err(DirectoryError::Conflict(format!("identity assertion must be 1..={AUTH_ASSERTION_MAX_BYTES} bytes")));
        }
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentityAssurance {
    ExternalVerified,
    DevelopmentLocal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedIdentity {
    pub provider: String,
    pub subject: String,
    pub verified_email: Option<String>,
    pub display_name: Option<String>,
    pub issued_at: i64,
    pub expires_at: i64,
    pub assurance: IdentityAssurance,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IdentityVerificationProgress {
    pub completed_units: u8,
    pub total_units: u8,
}

pub trait IdentityVerificationControl: Send + Sync {
    fn now_ms(&self) -> i64;
    fn is_cancelled(&self) -> bool;
    fn report(&self, progress: IdentityVerificationProgress);
}

pub struct IdentityVerificationContext<'a> {
    pub deadline_ms: i64,
    pub control: &'a dyn IdentityVerificationControl,
}

impl IdentityVerificationContext<'_> {
    pub fn checkpoint(&self, completed_units: u8, total_units: u8) -> DirectoryResult<()> {
        self.control.report(IdentityVerificationProgress { completed_units, total_units });
        if self.control.is_cancelled() {
            return Err(DirectoryError::Conflict("identity assertion verification cancelled".into()));
        }
        if self.control.now_ms() > self.deadline_ms {
            return Err(DirectoryError::Conflict("identity assertion verification deadline exceeded".into()));
        }
        Ok(())
    }
}

pub type IdentityVerificationFuture<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = DirectoryResult<VerifiedIdentity>> + Send + 'a>>;

pub trait IdentityAssertionVerifier: Send + Sync + 'static {
    fn verify<'a>(&'a self, assertion: &'a IdentityAssertion, context: &'a IdentityVerificationContext<'a>) -> IdentityVerificationFuture<'a>;
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalBootstrapClientClass {
    Native,
    Mcp,
    ReactRelay,
    AdminRelay,
}

impl LocalBootstrapClientClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Mcp => "mcp",
            Self::ReactRelay => "react-relay",
            Self::AdminRelay => "admin-relay",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedLocalBootstrapRequest {
    pub request_id: String,
    pub run_id: String,
    pub profile_id: String,
    pub identity_provider: String,
    pub identity_subject: String,
    pub display_name: String,
    pub device_instance_id: String,
    pub client_class: LocalBootstrapClientClass,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalBootstrapRejectCode {
    Cancelled,
    Denied,
    Expired,
    ResourceLimit,
    Unavailable,
}

impl LocalBootstrapRejectCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cancelled => "cancelled",
            Self::Denied => "denied",
            Self::Expired => "expired",
            Self::ResourceLimit => "resource-limit",
            Self::Unavailable => "unavailable",
        }
    }
}

pub type LocalBootstrapAcceptFuture<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = DirectoryResult<Option<VerifiedLocalBootstrapRequest>>> + Send + 'a>>;
pub type LocalBootstrapIssueFuture<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = DirectoryResult<()>> + Send + 'a>>;
pub type LocalBootstrapTerminalFuture<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = DirectoryResult<()>> + Send + 'a>>;

pub trait LocalBootstrapTransport: Send + Sync + 'static {
    fn run_id(&self) -> &str;
    fn is_ready(&self) -> bool;
    fn request_cancelled(&self, request_id: &str) -> bool;
    fn accept<'a>(&'a self, control: &'a dyn IdentityVerificationControl) -> LocalBootstrapAcceptFuture<'a>;
    fn issue<'a>(&'a self, request: &'a VerifiedLocalBootstrapRequest, session: &'a IssuedAuthSession, context: &'a IdentityVerificationContext<'a>) -> LocalBootstrapIssueFuture<'a>;
    fn reject<'a>(&'a self, request_id: &'a str, code: LocalBootstrapRejectCode, context: &'a IdentityVerificationContext<'a>) -> LocalBootstrapTerminalFuture<'a>;
    fn cancel<'a>(&'a self, request_id: &'a str) -> LocalBootstrapTerminalFuture<'a>;
    fn shutdown<'a>(&'a self) -> LocalBootstrapTerminalFuture<'a>;
}

pub trait NativeCredentialEnvelopeDelivery: Send + Sync + 'static {
    fn deliver_native<'a>(&'a self, request_id: &'a str, capability: &'a SessionCapability, context: &'a IdentityVerificationContext<'a>) -> LocalBootstrapTerminalFuture<'a>;
}

pub trait McpCredentialEnvelopeDelivery: Send + Sync + 'static {
    fn deliver_mcp<'a>(&'a self, request_id: &'a str, capability: &'a SessionCapability, context: &'a IdentityVerificationContext<'a>) -> LocalBootstrapTerminalFuture<'a>;
}

pub trait BrowserCredentialRelay: Send + Sync + 'static {
    fn deliver_to_relay<'a>(&'a self, request_id: &'a str, client_class: LocalBootstrapClientClass, capability: &'a SessionCapability, context: &'a IdentityVerificationContext<'a>) -> LocalBootstrapTerminalFuture<'a>;
}

pub const AUTH_AUDIT_PAGE_MAX: usize = 1_000;

pub(crate) fn prepare_auth_session(issue: &AuthSessionIssue, now: i64) -> DirectoryResult<IssuedAuthSession> {
    validate_bounded_auth_text(&issue.user_id, "session user", AUTH_TEXT_MAX_BYTES)?;
    validate_bounded_auth_text(&issue.identity_provider, "identity provider", AUTH_TEXT_MAX_BYTES)?;
    validate_bounded_auth_text(&issue.device_instance_id, "device instance", DEVICE_INSTANCE_MAX_BYTES)?;
    validate_bounded_auth_text(&issue.correlation_id, "correlation id", AUTH_TEXT_MAX_BYTES)?;
    validate_bounded_auth_text(&issue.peer_class, "peer class", AUTH_TEXT_MAX_BYTES)?;
    if issue.identity_subject_digest == [0; 32] {
        return Err(DirectoryError::Conflict("identity subject digest must not be zero".into()));
    }
    let (issued_at, expires_at) = capability_window(now, issue.ttl_secs)?;
    let capability = SessionCapability::mint()?;
    let record = AuthSessionRecord {
        id: time_ordered_id(),
        selector: capability.selector().to_string(),
        secret_digest: capability.secret_digest(),
        user_id: issue.user_id.clone(),
        identity_provider: issue.identity_provider.clone(),
        identity_subject_digest: issue.identity_subject_digest,
        issued_at,
        expires_at,
        revoked_at: None,
        revoked_reason: None,
        authorization_generation: 1,
        device_instance_id: issue.device_instance_id.clone(),
        session_kind: issue.session_kind,
    };
    Ok(IssuedAuthSession { record, capability })
}

pub(crate) fn prepare_share_token(scope: &DocumentScope, ttl_secs: i64, now: i64) -> DirectoryResult<IssuedShareToken> {
    let (created_at, expires_at) = capability_window(now, ttl_secs)?;
    let capability = ShareCapability::mint()?;
    let record = ShareTokenRecord { id: time_ordered_id(), selector: capability.selector().to_string(), secret_digest: capability.secret_digest(), scope: scope.clone(), created_at, expires_at, revoked_at: None, revoked_reason: None };
    Ok(IssuedShareToken { record, capability })
}

pub(crate) fn prepare_invite(space_id: &str, role: SpaceRole, ttl_secs: i64, now: i64) -> DirectoryResult<IssuedInvite> {
    validate_bounded_auth_text(space_id, "invite space", AUTH_TEXT_MAX_BYTES)?;
    let (created_at, expires_at) = capability_window(now, ttl_secs)?;
    let capability = InviteCapability::mint()?;
    let record = InviteRecord {
        id: time_ordered_id(),
        selector: capability.selector().to_string(),
        secret_digest: capability.secret_digest(),
        space_id: space_id.to_string(),
        role,
        created_at,
        expires_at,
        revoked_at: None,
        revoked_reason: None,
        accepted_at: None,
        accepted_event_id: None,
    };
    Ok(IssuedInvite { record, capability })
}

pub(crate) fn active_capability(selector: &str, stored_digest: &[u8; 32], expires_at: i64, revoked_at: Option<i64>, capability_selector: &str, candidate_digest: &[u8; 32], now: i64) -> bool {
    selector == capability_selector && revoked_at.is_none() && expires_at > now && constant_time_digest_eq(stored_digest, candidate_digest)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InviteRedemptionPreflight {
    Claim,
    AlreadyCommitted,
    Revoked,
    Expired,
    Denied,
    Corrupt,
}

/// 🎟️ Applies the backend-independent invitation claim decision without accepting client scope or role authority.
pub(crate) fn invite_redemption_preflight(
    record: Option<&InviteRecord>,
    capability: &InviteCapability,
    actor: &DirectoryActor,
    user_id: &str,
    user_exists: bool,
    space_exists: bool,
    now_ms: i64,
) -> InviteRedemptionPreflight {
    let actor_user_id = (actor.kind == DirectoryActorKind::User)
        .then(|| actor.id.strip_prefix("user:").and_then(|rest| rest.split('#').next()))
        .flatten();
    let Some(record) = record else { return InviteRedemptionPreflight::Denied };
    if actor_user_id != Some(user_id) || record.selector != capability.selector() || !constant_time_digest_eq(&record.secret_digest, &capability.secret_digest()) {
        return InviteRedemptionPreflight::Denied;
    }
    if record.accepted_at.is_some() != record.accepted_event_id.is_some() {
        InviteRedemptionPreflight::Corrupt
    } else if record.accepted_at.is_some() {
        InviteRedemptionPreflight::AlreadyCommitted
    } else if !user_exists || !space_exists {
        InviteRedemptionPreflight::Denied
    } else if record.revoked_at.is_some() {
        InviteRedemptionPreflight::Revoked
    } else if record.expires_at <= now_ms {
        InviteRedemptionPreflight::Expired
    } else {
        InviteRedemptionPreflight::Claim
    }
}

/// 🎟️ Verifies the immutable event linked from an already-claimed invite before idempotent return.
pub(crate) fn verify_invite_redemption_event(record: &InviteRecord, event: Option<DirectoryEvent>, authenticated_user_id: &str) -> DirectoryResult<DirectoryEvent> {
    let accepted_at = record.accepted_at.ok_or_else(|| DirectoryError::Backend("invite acceptance marker is incomplete".into()))?;
    let accepted_event_id = record.accepted_event_id.as_deref().ok_or_else(|| DirectoryError::Backend("invite acceptance event marker is incomplete".into()))?;
    let event = event.ok_or_else(|| DirectoryError::Backend("invite acceptance event is missing".into()))?;
    let valid_body = matches!(
        &event.body,
        DirectoryEventBody::InviteRedeemed { space_id, user_id, invite_id, role }
            if space_id == &record.space_id
                && invite_id == &record.id
                && *role == role_to_wire(record.role)
                && event.space_id.as_deref() == Some(space_id)
                && event.user_id.as_deref() == Some(user_id)
                && actor_user_id(&event.actor).ok() == Some(user_id.as_str())
    );
    if event.id != accepted_event_id || event.recorded_at_ms != accepted_at || !valid_body {
        return Err(DirectoryError::Backend("invite acceptance marker and event differ".into()));
    }
    if event.user_id.as_deref() != Some(authenticated_user_id) {
        return Err(DirectoryError::Conflict("invite already accepted".into()));
    }
    Ok(event)
}

pub(crate) fn auth_audit(
    occurred_at: i64,
    event_kind: &str,
    auth_session_id: Option<&str>,
    target_user_id: Option<&str>,
    actor_user_id: Option<&str>,
    provider: Option<&str>,
    outcome_code: &str,
    reason_code: Option<&str>,
    correlation_id: &str,
    peer_class: &str,
) -> DirectoryResult<AuthAuditRecord> {
    for (value, field) in [(event_kind, "audit event kind"), (outcome_code, "audit outcome"), (correlation_id, "audit correlation"), (peer_class, "audit peer class")] {
        validate_bounded_auth_text(value, field, AUTH_TEXT_MAX_BYTES)?;
    }
    if let Some(reason) = reason_code {
        validate_bounded_auth_text(reason, "audit reason", AUTH_TEXT_MAX_BYTES)?;
    }
    Ok(AuthAuditRecord {
        id: time_ordered_id(),
        occurred_at,
        event_kind: event_kind.to_string(),
        auth_session_id: auth_session_id.map(str::to_string),
        target_user_id: target_user_id.map(str::to_string),
        actor_user_id: actor_user_id.map(str::to_string),
        provider: provider.map(str::to_string),
        outcome_code: outcome_code.to_string(),
        reason_code: reason_code.map(str::to_string),
        correlation_id: correlation_id.to_string(),
        peer_class: peer_class.to_string(),
    })
}

/// 🛡️ Admits one durable command idempotency claim: bounded actor, 32-hex nonzero correlation, and
/// a 64-hex lowercase canonical command digest. Backends call this before touching storage.
pub(crate) fn validate_directory_command_claim(claim: &NewDirectoryCommandReceipt) -> DirectoryResult<()> {
    validate_bounded_auth_text(&claim.actor_user_id, "command actor", AUTH_TEXT_MAX_BYTES)?;
    if claim.request_id.len() != 32 || claim.request_id.bytes().all(|byte| byte == b'0') || !claim.request_id.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
        return Err(DirectoryError::Conflict("command request id must be 32 lowercase nonzero hex digits".into()));
    }
    if claim.command_sha256.len() != 64 || !claim.command_sha256.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
        return Err(DirectoryError::Conflict("command digest must be 64 lowercase hex digits".into()));
    }
    Ok(())
}

/// 🏷️ The exact stored spelling of one durable command result class.
pub(crate) fn directory_command_result_kind_str(kind: DirectoryCommandResultKindV1) -> &'static str {
    match kind {
        DirectoryCommandResultKindV1::None => "none",
        DirectoryCommandResultKindV1::Invite => "invite",
    }
}

/// 🏷️ Reads back one stored command result class.
pub(crate) fn directory_command_result_kind_from_str(value: &str) -> DirectoryResult<DirectoryCommandResultKindV1> {
    match value {
        "none" => Ok(DirectoryCommandResultKindV1::None),
        "invite" => Ok(DirectoryCommandResultKindV1::Invite),
        other => Err(DirectoryError::Backend(format!("unknown command result kind '{other}'"))),
    }
}

pub(crate) fn validate_admin_operation_audit(fact: &NewAdminOperationAuditRecord) -> DirectoryResult<()> {
    for (value, field) in [
        (&fact.request_id, "admin request id"),
        (&fact.intent_digest, "admin intent digest"),
        (&fact.operation_id, "admin operation id"),
        (&fact.intent_kind, "admin intent kind"),
        (&fact.target_kind, "admin target kind"),
        (&fact.target_id, "admin target id"),
        (&fact.principal_user_id, "admin principal user"),
        (&fact.principal_session_id, "admin principal session"),
        (&fact.correlation_id, "admin correlation"),
        (&fact.outcome_code, "admin outcome"),
    ] {
        validate_bounded_auth_text(value, field, AUTH_TEXT_MAX_BYTES)?;
    }
    if !matches!(fact.phase.as_str(), "accepted" | "succeeded" | "failed" | "cancelled") {
        return Err(DirectoryError::Conflict("admin operation phase is invalid".into()));
    }
    if fact.intent_digest.len() != 64 || !fact.intent_digest.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
        return Err(DirectoryError::Conflict("admin intent digest must be 64 lowercase hex digits".into()));
    }
    if fact.principal_generation == 0 {
        return Err(DirectoryError::Conflict("admin principal generation must be nonzero".into()));
    }
    if fact.event_seq_first.is_some() != fact.event_seq_last.is_some() || fact.event_seq_first.zip(fact.event_seq_last).is_some_and(|(first, last)| first == 0 || first > last) {
        return Err(DirectoryError::Conflict("admin operation event range is invalid".into()));
    }
    if let Some(reason) = fact.reason_code.as_deref() {
        validate_bounded_auth_text(reason, "admin reason", AUTH_TEXT_MAX_BYTES)?;
    }
    Ok(())
}

pub(crate) fn same_admin_operation_request(existing: &NewAdminOperationAuditRecord, candidate: &NewAdminOperationAuditRecord) -> bool {
    existing.request_id == candidate.request_id
        && existing.intent_digest == candidate.intent_digest
        && existing.intent_kind == candidate.intent_kind
        && existing.target_kind == candidate.target_kind
        && existing.target_id == candidate.target_id
        && existing.principal_user_id == candidate.principal_user_id
        && existing.principal_session_id == candidate.principal_session_id
        && existing.principal_generation == candidate.principal_generation
}
//#endregion 🔖️Capabilities

//#region 🔖️Wire
// 🔗️ This crate's storage-row vocabulary (`SpaceRole`, plain `kind`/`visibility` strings — see
// `model`'s doc comments for why they are hand-kept-in-lockstep rather than depended on) versus the
// wire vocabulary the schema crate owns (`DirectorySpaceRole`/`DirectorySpaceKind`/
// `DirectorySpaceVisibility`, imported above, never redeclared). Free functions, not inherent impls
// on the wire types — they are foreign here, so the orphan rule blocks `impl DirectorySpaceRole {}`.

/// 🔁️ Local storage role -> wire role (used when an event body needs the wire enum but the value
/// in hand came from a projection read, e.g. an `InviteRecord`). `pub(crate)` — every backend's
/// `//#region 🔖️Projections` uses the reverse direction (`role_from_wire(role).as_str()`) to get
/// the exact `"author"`/`"spectator"` string its `CHECK` constraint speaks.
pub(crate) fn role_to_wire(role: SpaceRole) -> DirectorySpaceRole {
    match role {
        SpaceRole::Author => DirectorySpaceRole::Author,
        SpaceRole::Spectator => DirectorySpaceRole::Spectator,
    }
}

/// 🔁️ Wire role -> local storage role (used when a `HubDirectory` method that still speaks the
/// storage vocabulary — e.g. `create_invite` — needs a role that arrived as a `DirectoryCommand` field).
pub(crate) fn role_from_wire(role: DirectorySpaceRole) -> SpaceRole {
    match role {
        DirectorySpaceRole::Author => SpaceRole::Author,
        DirectorySpaceRole::Spectator => SpaceRole::Spectator,
    }
}

/// 🔡️ Wire space-kind -> the exact lowercase string every backend's `CHECK (kind IN (...))`
/// constraint speaks; used only by each backend's `//#region 🔖️Projections`.
pub(crate) fn kind_to_str(kind: DirectorySpaceKind) -> &'static str {
    match kind {
        DirectorySpaceKind::Atelier => "atelier",
        DirectorySpaceKind::Studio => "studio",
        DirectorySpaceKind::Archive => "archive",
    }
}

/// 🔡️ Wire visibility -> the exact lowercase string every backend's `CHECK` constraint speaks.
pub(crate) fn visibility_to_str(visibility: DirectorySpaceVisibility) -> &'static str {
    match visibility {
        DirectorySpaceVisibility::Private => "private",
        DirectorySpaceVisibility::Public => "public",
    }
}

/// @emoji 🧬️ Rejects descriptors that cannot safely select and verify a cold-open codec.
pub fn validate_document_descriptor(descriptor: &DocumentDescriptor) -> DirectoryResult<()> {
    fn present(value: &str, field: &str) -> DirectoryResult<()> {
        if value.trim().is_empty() {
            Err(DirectoryError::Conflict(format!("document descriptor {field} must not be empty")))
        } else {
            Ok(())
        }
    }
    fn hash(value: &str, field: &str) -> DirectoryResult<()> {
        if value.len() != 64 || value == "0".repeat(64) || !value.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
            return Err(DirectoryError::Conflict(format!("document descriptor {field} must be a non-zero lowercase sha-256")));
        }
        Ok(())
    }
    present(&descriptor.space_id, "spaceId")?;
    present(&descriptor.document_id, "documentId")?;
    present(&descriptor.artifact_kind, "artifactKind")?;
    present(&descriptor.artifact_schema, "artifactSchema")?;
    present(&descriptor.owner.plugin_id, "owner.pluginId")?;
    present(&descriptor.owner.package_id, "owner.packageId")?;
    present(&descriptor.owner.version, "owner.version")?;
    hash(&descriptor.owner.package_hash, "owner.packageHash")?;
    hash(&descriptor.pack_schema_hash, "packSchemaHash")?;
    hash(&descriptor.bootstrap_snapshot_hash, "bootstrapSnapshotHash")?;
    if descriptor.bootstrap_version == 0 {
        return Err(DirectoryError::Conflict("document descriptor bootstrapVersion must be positive".into()));
    }
    if descriptor.bootstrap_frontier.commit_seq > descriptor.bootstrap_frontier.head_seq {
        return Err(DirectoryError::Conflict("document descriptor bootstrap frontier commitSeq exceeds headSeq".into()));
    }
    Ok(())
}

/// 📦️ Removes the authority store's opaque locator before checkpoint metadata enters the event log.
pub fn published_artifact_checkpoint(checkpoint: &ArtifactCheckpoint) -> PublishedArtifactCheckpoint {
    PublishedArtifactCheckpoint {
        scope: checkpoint.scope.clone(),
        checkpoint_id: checkpoint.checkpoint_id,
        parent_checkpoint_id: checkpoint.parent_checkpoint_id,
        descriptor_digest_v1: checkpoint.descriptor_digest_v1,
        baseline_frontier: checkpoint.baseline_frontier.clone(),
        pack: PublishedArtifactBlob { sha256: checkpoint.pack.sha256, byte_length: checkpoint.pack.byte_length },
        spr: PublishedArtifactBlob { sha256: checkpoint.spr.sha256, byte_length: checkpoint.spr.byte_length },
        aggregate_sha256: checkpoint.aggregate_sha256,
        published_at_ms: checkpoint.published_at_ms,
    }
}

fn checkpoint_identity_input(checkpoint: &PublishedArtifactCheckpoint) -> ArtifactCheckpoint {
    ArtifactCheckpoint {
        scope: checkpoint.scope.clone(),
        checkpoint_id: checkpoint.checkpoint_id,
        parent_checkpoint_id: checkpoint.parent_checkpoint_id,
        descriptor_digest_v1: checkpoint.descriptor_digest_v1,
        baseline_frontier: checkpoint.baseline_frontier.clone(),
        pack: ArtifactBlobRef { sha256: checkpoint.pack.sha256, byte_length: checkpoint.pack.byte_length, storage_key: String::new() },
        spr: ArtifactBlobRef { sha256: checkpoint.spr.sha256, byte_length: checkpoint.spr.byte_length, storage_key: String::new() },
        aggregate_sha256: checkpoint.aggregate_sha256,
        published_at_ms: checkpoint.published_at_ms,
    }
}

fn valid_hash(hash: ArtifactHash) -> bool {
    hash.0 != [0; 32]
}

fn validate_checkpoint_shape(checkpoint: &PublishedArtifactCheckpoint) -> DirectoryResult<()> {
    if checkpoint.scope.space_id.is_empty()
        || checkpoint.scope.document_id.is_empty()
        || checkpoint.baseline_frontier.document_id != checkpoint.scope.document_id
        || checkpoint.baseline_frontier.head_edit_id.is_empty()
        || checkpoint.pack.byte_length == 0
        || checkpoint.spr.byte_length == 0
        || checkpoint.baseline_frontier.head_edit_ordinal > DIRECTORY_WIRE_INTEGER_MAX
        || checkpoint.baseline_frontier.last_commit_seq > DIRECTORY_WIRE_INTEGER_MAX
        || checkpoint.pack.byte_length > DIRECTORY_WIRE_INTEGER_MAX
        || checkpoint.spr.byte_length > DIRECTORY_WIRE_INTEGER_MAX
        || checkpoint.published_at_ms > DIRECTORY_WIRE_INTEGER_MAX
        || !valid_hash(checkpoint.checkpoint_id)
        || checkpoint.parent_checkpoint_id.is_some_and(|id| !valid_hash(id))
        || !valid_hash(checkpoint.descriptor_digest_v1)
        || !valid_hash(checkpoint.baseline_frontier.chain_hash)
        || !valid_hash(checkpoint.pack.sha256)
        || !valid_hash(checkpoint.spr.sha256)
        || !valid_hash(checkpoint.aggregate_sha256)
    {
        return Err(DirectoryError::Conflict("artifact checkpoint metadata is invalid".into()));
    }
    let identity = checkpoint_identity_input(checkpoint);
    let encoded = crate::artifact_authority::checkpoint_id_encoding_v1(&identity).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
    if (ArtifactHash(Sha256::digest(&encoded))) != checkpoint.checkpoint_id {
        return Err(DirectoryError::Conflict("artifact checkpoint identity does not match its canonical metadata".into()));
    }
    Ok(())
}

fn validate_retention_shape(retention: &ArtifactRetention) -> DirectoryResult<()> {
    if retention.scope.space_id.is_empty()
        || retention.scope.document_id.is_empty()
        || retention.retained_floor.document_id != retention.scope.document_id
        || retention.retained_floor.head_edit_id.is_empty()
        || retention.retained_floor.head_edit_ordinal > DIRECTORY_WIRE_INTEGER_MAX
        || retention.retained_floor.last_commit_seq > DIRECTORY_WIRE_INTEGER_MAX
        || !valid_hash(retention.retained_checkpoint_id)
        || !valid_hash(retention.retained_floor.chain_hash)
        || !valid_hash(retention.checkpoint_lineage_head)
    {
        return Err(DirectoryError::Conflict("artifact retention metadata is invalid".into()));
    }
    Ok(())
}

/// 🔐️ Defends the backend-only atomic append seam even when called outside the service.
pub(crate) fn validate_verified_checkpoint_append(event: &NewDirectoryEvent, checkpoint: &ArtifactCheckpoint) -> DirectoryResult<PublishedArtifactCheckpoint> {
    if event.actor.kind != DirectoryActorKind::System {
        return Err(DirectoryError::Unauthorized);
    }
    if event.space_id.as_deref() != Some(checkpoint.scope.space_id.as_str()) || event.user_id.is_some() {
        return Err(DirectoryError::Conflict("verified artifact checkpoint event scope is invalid".into()));
    }
    if checkpoint.pack.storage_key.trim().is_empty() || checkpoint.spr.storage_key.trim().is_empty() || checkpoint.pack.storage_key.len() > ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES || checkpoint.spr.storage_key.len() > ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES {
        return Err(DirectoryError::Conflict(format!("verified artifact checkpoint locator must contain 1..={ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES} UTF-8 bytes")));
    }
    let published = published_artifact_checkpoint(checkpoint);
    validate_checkpoint_shape(&published)?;
    match &event.body {
        DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: body } if body == &published => Ok(published),
        DirectoryEventBody::ArtifactCheckpointPublished { .. } => Err(DirectoryError::Conflict("verified artifact checkpoint differs from its public event".into())),
        _ => Err(DirectoryError::Conflict("verified artifact append requires one checkpoint-published event".into())),
    }
}

fn frontier_strictly_advances(previous: &ArtifactFrontier, next: &ArtifactFrontier) -> bool {
    previous.document_id == next.document_id && next.head_edit_ordinal > previous.head_edit_ordinal && next.last_commit_seq > previous.last_commit_seq
}

/// 🧠️ Dependency-free in-memory artifact projection used by embedded hosts and backend parity laws.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MemoryArtifactProjection {
    descriptors: HashMap<DocumentScope, DocumentDescriptor>,
    checkpoints: HashMap<DocumentScope, Vec<PublishedArtifactCheckpoint>>,
    retention: HashMap<DocumentScope, ArtifactRetention>,
}

impl MemoryArtifactProjection {
    pub fn active_checkpoint(&self, scope: &DocumentScope) -> Option<&PublishedArtifactCheckpoint> {
        self.checkpoints.get(scope).and_then(|lineage| lineage.last())
    }

    pub fn checkpoint_lineage(&self, scope: &DocumentScope) -> &[PublishedArtifactCheckpoint] {
        self.checkpoints.get(scope).map_or(&[], Vec::as_slice)
    }

    pub fn retention(&self, scope: &DocumentScope) -> Option<&ArtifactRetention> {
        self.retention.get(scope)
    }

    /// ⚛️ Folds all events into a clone and transfers it only after every invariant succeeds.
    pub fn fold_atomically(&mut self, events: &[DirectoryEvent]) -> DirectoryResult<()> {
        let mut next = self.clone();
        for event in events {
            next.fold(event)?;
        }
        *self = next;
        Ok(())
    }

    fn fold(&mut self, event: &DirectoryEvent) -> DirectoryResult<()> {
        match &event.body {
            DirectoryEventBody::DocumentAnnounced { descriptor } => {
                self.descriptors.insert(DocumentScope::new(&descriptor.space_id, &descriptor.document_id), descriptor.clone());
            }
            DirectoryEventBody::SpaceDeleted { space_id } => {
                self.descriptors.retain(|scope, _| &scope.space_id != space_id);
                self.checkpoints.retain(|scope, _| &scope.space_id != space_id);
                self.retention.retain(|scope, _| &scope.space_id != space_id);
            }
            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {
                validate_checkpoint_shape(checkpoint)?;
                let descriptor = self.descriptors.get(&checkpoint.scope).ok_or_else(|| DirectoryError::NotFound("memory artifact descriptor".into()))?;
                if descriptor_digest_v1(descriptor).map_err(|error| DirectoryError::Conflict(error.to_string()))? != checkpoint.descriptor_digest_v1 {
                    return Err(DirectoryError::Conflict("memory artifact descriptor digest mismatch".into()));
                }
                let lineage = self.checkpoints.entry(checkpoint.scope.clone()).or_default();
                if let Some(existing) = lineage.iter().find(|existing| existing.checkpoint_id == checkpoint.checkpoint_id) {
                    return if existing == checkpoint { Ok(()) } else { Err(DirectoryError::Conflict("memory artifact checkpoint identity conflict".into())) };
                }
                if lineage.len() as u64 >= ARTIFACT_CHECKPOINT_LINEAGE_MAX {
                    return Err(DirectoryError::Conflict(format!("artifact checkpoint lineage exceeds fixed maximum {ARTIFACT_CHECKPOINT_LINEAGE_MAX}")));
                }
                match lineage.last() {
                    None if checkpoint.parent_checkpoint_id.is_some() => return Err(DirectoryError::Conflict("memory genesis checkpoint parent".into())),
                    Some(active) if checkpoint.parent_checkpoint_id != Some(active.checkpoint_id) => return Err(DirectoryError::Conflict("memory artifact checkpoint parent".into())),
                    Some(active) if !frontier_strictly_advances(&active.baseline_frontier, &checkpoint.baseline_frontier) => return Err(DirectoryError::Conflict("memory artifact checkpoint frontier".into())),
                    _ => {}
                }
                lineage.push(checkpoint.clone());
            }
            DirectoryEventBody::ArtifactRetentionAdvanced { retention } => {
                let lineage = self.checkpoints.get(&retention.scope).ok_or_else(|| DirectoryError::NotFound("memory artifact lineage".into()))?;
                let active = lineage.last().ok_or_else(|| DirectoryError::NotFound("memory active artifact checkpoint".into()))?;
                if active.checkpoint_id != retention.checkpoint_lineage_head {
                    return Err(DirectoryError::Conflict("memory artifact retention head".into()));
                }
                let retained_index = lineage.iter().position(|checkpoint| checkpoint.checkpoint_id == retention.retained_checkpoint_id).ok_or_else(|| DirectoryError::NotFound("memory retained artifact checkpoint".into()))?;
                if lineage[retained_index].baseline_frontier != retention.retained_floor {
                    return Err(DirectoryError::Conflict("memory artifact retention floor".into()));
                }
                if let Some(previous) = self.retention.get(&retention.scope) {
                    if previous == retention {
                        return Ok(());
                    }
                    let previous_index = lineage.iter().position(|checkpoint| checkpoint.checkpoint_id == previous.retained_checkpoint_id).ok_or_else(|| DirectoryError::Conflict("memory prior retention lineage".into()))?;
                    if retained_index < previous_index {
                        return Err(DirectoryError::Conflict("memory artifact retention moved backward".into()));
                    }
                }
                self.retention.insert(retention.scope.clone(), retention.clone());
            }
            _ => {}
        }
        Ok(())
    }
}
//#endregion 🔖️Wire

//#region 🔖️Events
fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

/// @emoji ⏱️ A hybrid logical clock: wall-clock milliseconds plus a same-millisecond tiebreak
/// counter, monotone across `tick()` calls regardless of how the OS clock jitters. One instance
/// lives behind `DirectoryService`'s write lock (see `//#region 🔖️Service`) — the lock is what
/// makes a `HubClock`'s stream of `tick()`s a total order across every command this hub instance
/// executes, which `append_events` then turns into a dense backend-assigned `seq`.
#[derive(Clone, Copy, Debug, Default)]
pub struct HubClock {
    physical_ms: i64,
    logical: u32,
}

impl HubClock {
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji ⏭️ Advances the clock by one tick and returns its new `Hlc`. Same millisecond as the
    /// last tick ⇒ the logical counter increments; a later millisecond ⇒ it resets to 0. Time never
    /// runs backward even if the OS clock does (the physical component only ever holds or advances).
    pub fn tick(&mut self) -> Hlc {
        let observed = now_ms();
        if observed > self.physical_ms {
            self.physical_ms = observed;
            self.logical = 0;
        } else {
            self.logical += 1;
        }
        Hlc { physical_ms: self.physical_ms, logical: self.logical }
    }
}

/// @emoji ✉️ One decided-but-not-yet-persisted directory event: everything `decide` (see
/// `//#region 🔖️Decider`) can determine on its own. `append_events` (the `HubDirectory` trait,
/// implemented once per backend) turns this into a full `DirectoryEvent` by assigning the
/// backend-dense `seq`, minting a uuid v7 `id`, and stamping `recorded_at_ms`.
#[derive(Clone, Debug)]
pub struct NewDirectoryEvent {
    pub hlc: Hlc,
    pub actor: DirectoryActor,
    pub space_id: Option<String>,
    pub user_id: Option<String>,
    pub body: DirectoryEventBody,
}
//#endregion 🔖️Events

//#region 🔖️Decider
/// 🎁️ The extra payload a `decide`d command can carry alongside its events — today only
/// `create-invite` produces one (contract C1's `-> { inviteToken }`); the secret token itself is
/// never an event field (invites are not event-sourced, only their `invite.redeemed` outcome is),
/// so it has to travel back to the caller some other way than the event stream.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandResult {
    pub invite_token: Option<String>,
}

/// 🧾️ What `decide` computed for one `DirectoryCommand`: the events to persist (empty for the two
/// not-event-sourced invite commands, see `decide`'s own doc) plus an optional `CommandResult`.
#[derive(Clone, Debug)]
pub struct Decision {
    pub events: Vec<NewDirectoryEvent>,
    pub result: Option<CommandResult>,
}

fn new_event(clock: &mut HubClock, actor: &DirectoryActor, space_id: Option<String>, user_id: Option<String>, body: DirectoryEventBody) -> NewDirectoryEvent {
    NewDirectoryEvent { hlc: clock.tick(), actor: actor.clone(), space_id, user_id, body }
}

fn single(clock: &mut HubClock, actor: &DirectoryActor, space_id: Option<String>, user_id: Option<String>, body: DirectoryEventBody) -> Decision {
    Decision { events: vec![new_event(clock, actor, space_id, user_id, body)], result: None }
}

async fn require_space(dir: &HubDirectories, space_id: &str) -> DirectoryResult<SpaceRecord> {
    dir.get_space(space_id).await?.ok_or_else(|| DirectoryError::NotFound(format!("space '{space_id}' not found")))
}

/// 🎭️ Extracts the plain user id out of a `User`-kind actor's `user:{user_id}#{shell_session_id}`
/// grammar (contract-freeze.md §C0) — directory events denormalize the plain id, never the
/// per-tab/process actor string. `Admin`/`System` actors have no owning user, so `create-space`
/// (the only command that needs one) rejects them.
fn actor_user_id(actor: &DirectoryActor) -> DirectoryResult<&str> {
    match actor.kind {
        DirectoryActorKind::User => actor.id.strip_prefix("user:").and_then(|rest| rest.split('#').next()).ok_or_else(|| DirectoryError::Backend(format!("malformed user actor id '{}'", actor.id))),
        _ => Err(DirectoryError::Backend("operation requires a user actor".into())),
    }
}

/// @emoji 🧠️ Decides what a `DirectoryCommand` means: reads projections through `dir` (`get_space`,
/// `list_members`, `get_user_by_email`) and returns the events that would record it — it never
/// writes to `dir`, with one deliberate exception: `create-invite`/`revoke-invite` are NOT
/// event-sourced (contract's decider laws — only an invite's `invite.redeemed` outcome is an
/// event), so for those two variants this function performs the (non-event) write itself, under
/// the same write-lock serialization `DirectoryService::execute` already holds while calling it.
///
/// Authorization is **not** this function's job — `bin.rs` (lane 1-B) checks whether `actor` may
/// issue `command` against the named space *before* calling this; `decide` trusts `actor` as given
/// and only enforces the contract's structural laws:
/// - `create-space`/`archive-space`/`upsert-member` derive/emit the atelier ⇒ ≤1-author and
///   archive ⇒ nobody-writes laws (`archive-space` emits `member.upserted{spectator}` for every
///   current author, then `space.archived`, one event per projection step).
/// - `remove-member` naming the space's own owner ⇒ `DirectoryError::Conflict` (never removable).
/// - Any command naming a missing/deleted space ⇒ `DirectoryError::NotFound`.
/// - `upsert-member` with an email that has no `UserRecord` yet emits `user.created` first, using
///   a freshly minted user id the following `member.upserted` also uses.
pub async fn decide(dir: &HubDirectories, actor: &DirectoryActor, command: DirectoryCommand, clock: &mut HubClock) -> DirectoryResult<Decision> {
    match command {
        DirectoryCommand::CreateSpace { name, space_kind, visibility } => {
            let space_id = time_ordered_id();
            decide_create_space(actor, space_id, name, space_kind, visibility, clock)
        }
        DirectoryCommand::RenameSpace { space_id, name } => {
            require_space(dir, &space_id).await?;
            Ok(single(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceRenamed { space_id, name }))
        }
        DirectoryCommand::SetVisibility { space_id, visibility } => {
            require_space(dir, &space_id).await?;
            Ok(single(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceVisibilityChanged { space_id, visibility }))
        }
        DirectoryCommand::ArchiveSpace { space_id } => {
            require_space(dir, &space_id).await?;
            let members = dir.list_members(&space_id).await?;
            let mut events: Vec<NewDirectoryEvent> = members
                .into_iter()
                .filter(|(_, role)| *role == SpaceRole::Author)
                .map(|(user, _)| new_event(clock, actor, Some(space_id.clone()), Some(user.id.clone()), DirectoryEventBody::MemberUpserted { space_id: space_id.clone(), user_id: user.id, role: DirectorySpaceRole::Spectator }))
                .collect();
            events.push(new_event(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceArchived { space_id }));
            Ok(Decision { events, result: None })
        }
        DirectoryCommand::DeleteSpace { space_id } => {
            require_space(dir, &space_id).await?;
            Ok(single(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceDeleted { space_id }))
        }
        DirectoryCommand::UpsertMember { space_id, email, role } => {
            let space = require_space(dir, &space_id).await?;
            let mut events = Vec::new();
            let user_id = match dir.get_user_by_email(&email).await? {
                Some(existing) => existing.id,
                None => {
                    let user_id = time_ordered_id();
                    let display_name = email.split('@').next().unwrap_or(&email).to_string();
                    events.push(new_event(clock, actor, None, Some(user_id.clone()), DirectoryEventBody::UserCreated { user_id: user_id.clone(), email: email.clone(), display_name }));
                    user_id
                }
            };
            if role == DirectorySpaceRole::Author {
                if space.kind == "archive" {
                    return Err(DirectoryError::Conflict(format!("space '{space_id}' is an archive; no author memberships are allowed")));
                }
                if space.kind == "atelier" {
                    let has_other_author = dir.list_members(&space_id).await?.into_iter().any(|(user, existing_role)| existing_role == SpaceRole::Author && user.id != user_id);
                    if has_other_author {
                        return Err(DirectoryError::Conflict(format!("space '{space_id}' is an atelier; it already has a distinct author")));
                    }
                }
            }
            events.push(new_event(clock, actor, Some(space_id.clone()), Some(user_id.clone()), DirectoryEventBody::MemberUpserted { space_id, user_id, role }));
            Ok(Decision { events, result: None })
        }
        DirectoryCommand::RemoveMember { space_id, user_id } => {
            let space = require_space(dir, &space_id).await?;
            if space.owner_user_id == user_id {
                return Err(DirectoryError::Conflict(format!("space '{space_id}' owner membership cannot be removed")));
            }
            Ok(single(clock, actor, Some(space_id.clone()), Some(user_id.clone()), DirectoryEventBody::MemberRemoved { space_id, user_id }))
        }
        DirectoryCommand::CreateInvite { space_id, role, ttl_secs } => {
            require_space(dir, &space_id).await?;
            let ttl_secs = i64::try_from(ttl_secs).map_err(|_| DirectoryError::Conflict("invite ttl exceeds the signed storage boundary".into()))?;
            let issued = dir.issue_invite_as(&space_id, role_from_wire(role), ttl_secs, Some(actor_user_id(actor)?), &time_ordered_id()).await?;
            Ok(Decision { events: Vec::new(), result: Some(CommandResult { invite_token: Some(issued.capability.expose_once()) }) })
        }
        DirectoryCommand::RevokeInvite { space_id, invite_id } => {
            require_space(dir, &space_id).await?;
            dir.revoke_invite_as(&invite_id, "directory-command", Some(actor_user_id(actor)?), &time_ordered_id()).await?;
            Ok(Decision { events: Vec::new(), result: None })
        }
        DirectoryCommand::AnnounceDocument { descriptor } => {
            validate_document_descriptor(&descriptor)?;
            require_space(dir, &descriptor.space_id).await?;
            let scope = DocumentScope::new(&descriptor.space_id, &descriptor.document_id);
            match dir.get_document_descriptor(&scope).await? {
                Some(existing) if existing == descriptor => Ok(Decision { events: Vec::new(), result: None }),
                Some(_) => Err(DirectoryError::Conflict(format!("document descriptor for '{}/{}' is immutable", descriptor.space_id, descriptor.document_id))),
                None => Ok(single(clock, actor, Some(descriptor.space_id.clone()), None, DirectoryEventBody::DocumentAnnounced { descriptor })),
            }
        }
    }
}

fn decide_create_space(actor: &DirectoryActor, space_id: String, name: String, space_kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility, clock: &mut HubClock) -> DirectoryResult<Decision> {
    validate_bounded_auth_text(&space_id, "space id", AUTH_TEXT_MAX_BYTES)?;
    let owner_user_id = actor_user_id(actor)?.to_string();
    let owner_role = if space_kind == DirectorySpaceKind::Archive { DirectorySpaceRole::Spectator } else { DirectorySpaceRole::Author };
    let events = vec![
        new_event(clock, actor, Some(space_id.clone()), Some(owner_user_id.clone()), DirectoryEventBody::SpaceCreated { space_id: space_id.clone(), name, space_kind, visibility, owner_user_id: owner_user_id.clone() }),
        new_event(clock, actor, Some(space_id.clone()), Some(owner_user_id.clone()), DirectoryEventBody::MemberUpserted { space_id, user_id: owner_user_id, role: owner_role }),
    ];
    Ok(Decision { events, result: None })
}

/// 🛡️ Server-only retention policy intent; it is deliberately absent from `DirectoryCommand`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactDirectoryCommand {
    AdvanceRetention { retention: ArtifactRetention },
}

async fn decide_artifact_authority(dir: &HubDirectories, actor: &DirectoryActor, command: ArtifactDirectoryCommand, clock: &mut HubClock) -> DirectoryResult<Decision> {
    if !matches!(actor.kind, DirectoryActorKind::System | DirectoryActorKind::Admin) {
        return Err(DirectoryError::Unauthorized);
    }
    match command {
        ArtifactDirectoryCommand::AdvanceRetention { retention } => {
            validate_retention_shape(&retention)?;
            let active = dir.get_active_artifact_checkpoint(&retention.scope).await?.ok_or_else(|| DirectoryError::NotFound("active artifact checkpoint".into()))?;
            if retention.checkpoint_lineage_head != active.checkpoint_id {
                return Err(DirectoryError::Conflict("artifact retention lineage head is not the active checkpoint".into()));
            }
            let retained = dir.get_artifact_checkpoint(&retention.scope, retention.retained_checkpoint_id).await?.ok_or_else(|| DirectoryError::NotFound("retained artifact checkpoint".into()))?;
            if retained.baseline_frontier != retention.retained_floor {
                return Err(DirectoryError::Conflict("artifact retention floor is not the retained checkpoint baseline".into()));
            }
            let lineage = dir.list_artifact_checkpoint_lineage(&retention.scope, ARTIFACT_CHECKPOINT_LINEAGE_MAX as usize).await?;
            let retained_index = lineage.iter().position(|checkpoint| checkpoint.checkpoint_id == retention.retained_checkpoint_id).ok_or_else(|| DirectoryError::Conflict("retained artifact checkpoint is outside the active lineage".into()))?;
            let active_index = lineage.iter().position(|checkpoint| checkpoint.checkpoint_id == active.checkpoint_id).ok_or_else(|| DirectoryError::Conflict("active artifact checkpoint is outside its lineage".into()))?;
            if retained_index > active_index {
                return Err(DirectoryError::Conflict("artifact retention floor is ahead of the active baseline".into()));
            }
            if let Some(previous) = dir.get_artifact_retention(&retention.scope).await? {
                if previous == retention {
                    return Ok(Decision { events: Vec::new(), result: None });
                }
                let previous_index =
                    lineage.iter().position(|checkpoint| checkpoint.checkpoint_id == previous.retained_checkpoint_id).ok_or_else(|| DirectoryError::Conflict("existing artifact retention floor is outside the active lineage".into()))?;
                if retained_index < previous_index {
                    return Err(DirectoryError::Conflict("artifact retention floor cannot move backward".into()));
                }
            }
            Ok(single(clock, actor, Some(retention.scope.space_id.clone()), None, DirectoryEventBody::ArtifactRetentionAdvanced { retention }))
        }
    }
}

async fn decide_verified_checkpoint(dir: &HubDirectories, actor: &DirectoryActor, checkpoint: &ArtifactCheckpoint, clock: &mut HubClock) -> DirectoryResult<Decision> {
    if actor.kind != DirectoryActorKind::System {
        return Err(DirectoryError::Unauthorized);
    }
    let published = published_artifact_checkpoint(checkpoint);
    validate_checkpoint_shape(&published)?;
    if checkpoint.pack.storage_key.trim().is_empty() || checkpoint.spr.storage_key.trim().is_empty() || checkpoint.pack.storage_key.len() > ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES || checkpoint.spr.storage_key.len() > ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES {
        return Err(DirectoryError::Conflict(format!("verified artifact checkpoint locator must contain 1..={ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES} UTF-8 bytes")));
    }
    let descriptor = dir.get_document_descriptor(&published.scope).await?.ok_or_else(|| DirectoryError::NotFound(format!("document descriptor for '{}/{}'", published.scope.space_id, published.scope.document_id)))?;
    let digest = descriptor_digest_v1(&descriptor).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
    if digest != published.descriptor_digest_v1 {
        return Err(DirectoryError::Conflict("artifact checkpoint descriptor digest differs from the durable descriptor".into()));
    }
    if let Some(existing) = dir.get_verified_artifact_checkpoint(&published.scope, published.checkpoint_id).await? {
        return if &existing == checkpoint { Ok(Decision { events: Vec::new(), result: None }) } else { Err(DirectoryError::Conflict("artifact checkpoint id already names different public or private metadata".into())) };
    }
    if dir.get_artifact_checkpoint(&published.scope, published.checkpoint_id).await?.is_some() {
        return Err(DirectoryError::Conflict("artifact checkpoint public projection has no matching private authority record".into()));
    }
    if dir.artifact_checkpoint_count(&published.scope).await? >= ARTIFACT_CHECKPOINT_LINEAGE_MAX {
        return Err(DirectoryError::Conflict(format!("artifact checkpoint lineage exceeds fixed maximum {ARTIFACT_CHECKPOINT_LINEAGE_MAX}")));
    }
    match dir.get_active_artifact_checkpoint(&published.scope).await? {
        None if published.parent_checkpoint_id.is_some() => return Err(DirectoryError::Conflict("genesis artifact checkpoint must not name a parent".into())),
        Some(ref current) if published.parent_checkpoint_id != Some(current.checkpoint_id) => return Err(DirectoryError::Conflict("artifact checkpoint parent is not the active lineage head".into())),
        Some(ref current) if !frontier_strictly_advances(&current.baseline_frontier, &published.baseline_frontier) => {
            return Err(DirectoryError::Conflict("artifact checkpoint frontier does not strictly advance the active baseline".into()));
        }
        _ => {}
    }
    Ok(single(clock, actor, Some(published.scope.space_id.clone()), None, DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: published }))
}
//#endregion 🔖️Decider

//#region 🔖️Service
#[cfg(test)]
struct DirectoryPublicationTestFence {
    claimed: std::sync::atomic::AtomicBool,
    reached: tokio::sync::Notify,
    release: tokio::sync::Notify,
}

#[cfg(test)]
impl DirectoryPublicationTestFence {
    /// 🧪️ Creates a one-shot pause immediately after durable append.
    fn new() -> Self {
        Self { claimed: std::sync::atomic::AtomicBool::new(false), reached: tokio::sync::Notify::new(), release: tokio::sync::Notify::new() }
    }
}

/// 🧾️ Closed outcome of one idempotent directory-command execution.
#[derive(Clone, Debug, PartialEq)]
pub enum DirectoryCommandExecutionV1 {
    Receipt(DirectoryCommandReceiptV1),
    Conflict,
}

/// 🎁️ The durable result class one command will produce, known before it executes.
pub fn directory_command_result_kind(command: &DirectoryCommand) -> DirectoryCommandResultKindV1 {
    match command {
        DirectoryCommand::CreateInvite { .. } => DirectoryCommandResultKindV1::Invite,
        _ => DirectoryCommandResultKindV1::None,
    }
}

/// 🔒️ Seals the redacted receipt any later resolution of one durable key returns. A completed
/// secret-bearing command resolves as `secret-undeliverable` (no duplicate was minted, and the
/// one-shot capability is honestly unrecoverable); a still-pending key is equally undeliverable.
pub fn replay_directory_command_receipt(record: &DirectoryCommandReceiptRecord) -> DirectoryCommandReceiptV1 {
    let outcome = match (record.disposition, record.result_kind) {
        (DirectoryCommandDispositionV1::Completed, DirectoryCommandResultKindV1::None) => DirectoryCommandOutcomeV1::PreviouslyAccepted,
        _ => DirectoryCommandOutcomeV1::SecretUndeliverable,
    };
    DirectoryCommandReceiptV1::seal(record.request_id.clone(), record.command_sha256.clone(), outcome, Vec::new(), DirectoryCommandResultV1::None)
}

/// @emoji 🏭️ The hub's single directory writer. Every command is serialized behind one
/// `tokio::sync::Mutex<HubClock>` (dense, gap-free `seq` — two concurrent commands can never
/// interleave their `append_events` calls) and every persisted event (plus connection/presence
/// messages the caller publishes directly) fans out on one `broadcast` channel every
/// `/directory/socket/v1` connection subscribes to (contract C2).
pub struct DirectoryService {
    dir: Arc<HubDirectories>,
    write: tokio::sync::Mutex<HubClock>,
    tx: tokio::sync::broadcast::Sender<DirectoryStreamMessage>,
    artifact_cas_sweep_secret: [u8; 32],
    #[cfg(test)]
    publication_test_fence: std::sync::Mutex<Option<Arc<DirectoryPublicationTestFence>>>,
}

#[derive(Clone, Copy)]
struct ArtifactCasSweepPosition {
    execute: bool,
    observed_generation: u64,
    after_generation: u64,
    object_offset: usize,
}

impl DirectoryService {
    /// @emoji 🏗️ `channel_capacity` sizes the broadcast buffer; a subscriber that falls more than
    /// this many messages behind sees `RecvError::Lagged` and must resync via `events_since`
    /// (`?since=` replay, contract C2) — handled by `bin.rs`'s WS handler, not here.
    pub fn new(dir: Arc<HubDirectories>, channel_capacity: usize) -> Self {
        let (tx, _rx) = tokio::sync::broadcast::channel(channel_capacity);
        let mut sweep_secret = Sha256::new();
        sweep_secret.update(ARTIFACT_CAS_SWEEP_CONTINUATION_DOMAIN_V1);
        sweep_secret.update(time_ordered_id().as_bytes());
        Self {
            dir,
            write: tokio::sync::Mutex::new(HubClock::new()),
            tx,
            artifact_cas_sweep_secret: sweep_secret.finalize(),
            #[cfg(test)]
            publication_test_fence: std::sync::Mutex::new(None),
        }
    }

    #[cfg(test)]
    /// 🧪️ Arms a one-shot post-append fence for the writer-order concurrency law.
    fn arm_publication_test_fence(&self) -> Arc<DirectoryPublicationTestFence> {
        let fence = Arc::new(DirectoryPublicationTestFence::new());
        *self.publication_test_fence.lock().expect("publication test fence lock") = Some(fence.clone());
        fence
    }

    #[cfg(test)]
    /// ⏸️ Pauses the first committed page so a competing writer can prove it remains excluded.
    async fn pause_publication_test_fence_once(&self, persisted: &[DirectoryEvent]) {
        if persisted.is_empty() {
            return;
        }
        let fence = self.publication_test_fence.lock().expect("publication test fence lock").clone();
        if let Some(fence) = fence {
            if !fence.claimed.swap(true, std::sync::atomic::Ordering::SeqCst) {
                fence.reached.notify_one();
                fence.release.notified().await;
            }
        }
    }

    /// 🪝️ Publishes a committed event page while the caller still owns the single writer guard.
    fn publish_persisted_locked(&self, _clock: &tokio::sync::MutexGuard<'_, HubClock>, persisted: Vec<DirectoryEvent>) -> Vec<DirectoryEvent> {
        for event in &persisted {
            let _ = self.tx.send(DirectoryStreamMessage::Event { event: event.clone() });
        }
        persisted
    }

    /// 🔗️ Keeps durable append and synchronous fanout in the same writer-guard lifetime.
    async fn append_and_publish_locked(&self, clock: &tokio::sync::MutexGuard<'_, HubClock>, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>> {
        let persisted = if events.is_empty() { Vec::new() } else { self.dir.append_events(events).await? };
        #[cfg(test)]
        self.pause_publication_test_fence_once(&persisted).await;
        Ok(self.publish_persisted_locked(clock, persisted))
    }

    fn artifact_cas_sweep_continuation(&self, position: ArtifactCasSweepPosition) -> ArtifactCasSweepContinuation {
        let mut token = [0u8; ARTIFACT_CAS_SWEEP_CONTINUATION_BYTES];
        token[0] = u8::from(position.execute);
        token[1..9].copy_from_slice(&position.observed_generation.to_be_bytes());
        token[9..17].copy_from_slice(&position.after_generation.to_be_bytes());
        token[17..21].copy_from_slice(&u32::try_from(position.object_offset).unwrap_or(u32::MAX).to_be_bytes());
        let mut mac = Sha256::new();
        mac.update(ARTIFACT_CAS_SWEEP_CONTINUATION_DOMAIN_V1);
        mac.update(&self.artifact_cas_sweep_secret);
        mac.update(&token[..ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES]);
        token[ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES..].copy_from_slice(&mac.finalize());
        ArtifactCasSweepContinuation(token)
    }

    fn artifact_cas_delete_lease_token(&self, key: &ArtifactCasObjectKey, observed_generation: u64) -> [u8; 32] {
        let mut token = Sha256::new();
        token.update(ARTIFACT_CAS_DELETE_LEASE_DOMAIN_V1);
        token.update(&self.artifact_cas_sweep_secret);
        token.update(time_ordered_id().as_bytes());
        token.update(&observed_generation.to_be_bytes());
        token.update(&(key.space_id.len() as u64).to_be_bytes());
        token.update(key.space_id.as_bytes());
        token.update(key.kind.name().as_bytes());
        token.update(&key.digest.0);
        token.finalize()
    }

    fn artifact_cas_sweep_position(&self, token: ArtifactCasSweepContinuation, execute: bool) -> Result<ArtifactCasSweepPosition, crate::artifact_authority::AuthorityError> {
        let mut mac = Sha256::new();
        mac.update(ARTIFACT_CAS_SWEEP_CONTINUATION_DOMAIN_V1);
        mac.update(&self.artifact_cas_sweep_secret);
        mac.update(&token.0[..ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES]);
        let expected = mac.finalize();
        let valid_mac = token.0[ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES..].iter().zip(expected).fold(0u8, |difference, (actual, expected)| difference | (*actual ^ expected)) == 0;
        let token_execute = token.0[0] == 1;
        let observed_generation = u64::from_be_bytes(token.0[1..9].try_into().unwrap_or([0; 8]));
        let after_generation = u64::from_be_bytes(token.0[9..17].try_into().unwrap_or([0; 8]));
        let object_offset = usize::try_from(u32::from_be_bytes(token.0[17..21].try_into().unwrap_or([0; 4]))).unwrap_or(usize::MAX);
        if !valid_mac
            || token.0[0] > 1
            || token_execute != execute
            || observed_generation == 0
            || after_generation > observed_generation
            || object_offset > ARTIFACT_CAS_SWEEP_PAGE_MAX * crate::artifact_authority::chunk_cas::ARTIFACT_CAS_OWNERSHIP_MAX_OBJECTS
        {
            return Err(crate::artifact_authority::AuthorityError::Store("artifact CAS sweep continuation is invalid".into()));
        }
        Ok(ArtifactCasSweepPosition { execute, observed_generation, after_generation, object_offset })
    }

    /// @emoji ⚙️ The command pipeline: take the write lock → `decide` → `dir.append_events` →
    /// publish each persisted event on `tx` → release the lock. Authorization already happened in
    /// the caller (`bin.rs`); this trusts `actor` as given.
    pub async fn execute(&self, actor: DirectoryActor, command: DirectoryCommand) -> DirectoryResult<(Vec<DirectoryEvent>, Option<CommandResult>)> {
        let mut clock = self.write.lock().await;
        let decision = decide(self.dir.as_ref(), &actor, command, &mut clock).await?;
        let persisted = self.append_and_publish_locked(&clock, &decision.events).await?;
        Ok((persisted, decision.result))
    }

    /// 🆔️ The idempotent command pipeline: take the write lock → atomically claim or read the
    /// durable `(actor, request id)` key → `decide` → `append_events` → record the durable
    /// completion → only then publish. A duplicate key never re-executes, so a lost reply can never
    /// mint a second invitation; the one-shot capability travels back on this call alone.
    pub async fn execute_idempotent(&self, actor: DirectoryActor, claim: NewDirectoryCommandReceipt, command: DirectoryCommand) -> DirectoryResult<DirectoryCommandExecutionV1> {
        let mut clock = self.write.lock().await;
        match self.dir.claim_or_read_directory_command_receipt(&claim).await? {
            DirectoryCommandClaimV1::Conflict => return Ok(DirectoryCommandExecutionV1::Conflict),
            DirectoryCommandClaimV1::Existing(record) => {
                let receipt = replay_directory_command_receipt(&record);
                if record.receipt_sha256.as_deref().is_some_and(|digest| digest != receipt.receipt_sha256) {
                    return Err(DirectoryError::Backend("durable command receipt digest does not match its canonical replay".into()));
                }
                return Ok(DirectoryCommandExecutionV1::Receipt(receipt));
            }
            DirectoryCommandClaimV1::Claimed(_) => {}
        }
        let decision = match decide(self.dir.as_ref(), &actor, command, &mut clock).await {
            Ok(decision) => decision,
            Err(error) => {
                self.dir.release_directory_command_receipt(&claim.actor_user_id, &claim.request_id).await?;
                return Err(error);
            }
        };
        let persisted = if decision.events.is_empty() { Vec::new() } else { self.dir.append_events(&decision.events).await? };
        let replay_receipt_sha256 = replay_directory_command_receipt(&DirectoryCommandReceiptRecord {
            actor_user_id: claim.actor_user_id.clone(),
            request_id: claim.request_id.clone(),
            command_sha256: claim.command_sha256.clone(),
            result_kind: claim.result_kind,
            disposition: DirectoryCommandDispositionV1::Completed,
            event_seq_first: persisted.first().map(|event| event.seq),
            event_seq_last: persisted.last().map(|event| event.seq),
            receipt_sha256: None,
            claimed_at: claim.claimed_at,
            completed_at: None,
        })
        .receipt_sha256;
        self.dir
            .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
                actor_user_id: claim.actor_user_id.clone(),
                request_id: claim.request_id.clone(),
                event_seq_first: persisted.first().map(|event| event.seq),
                event_seq_last: persisted.last().map(|event| event.seq),
                receipt_sha256: replay_receipt_sha256,
                completed_at: now_ms(),
            })
            .await?;
        #[cfg(test)]
        self.pause_publication_test_fence_once(&persisted).await;
        let published = self.publish_persisted_locked(&clock, persisted);
        let result = match decision.result.and_then(|result| result.invite_token) {
            Some(invite_token) => DirectoryCommandResultV1::Invite { invite_token },
            None => DirectoryCommandResultV1::None,
        };
        Ok(DirectoryCommandExecutionV1::Receipt(DirectoryCommandReceiptV1::seal(claim.request_id, claim.command_sha256, DirectoryCommandOutcomeV1::Accepted, published, result)))
    }

    /// 🆔️ Appends an administrator-created space under its pre-audited stable resource id.
    pub async fn execute_create_space_with_id(&self, actor: DirectoryActor, space_id: String, name: String, space_kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility) -> DirectoryResult<Vec<DirectoryEvent>> {
        let mut clock = self.write.lock().await;
        if self.dir.get_space(&space_id).await?.is_some() {
            return Err(DirectoryError::Conflict("administrator space id already exists".into()));
        }
        let decision = decide_create_space(&actor, space_id, name, space_kind, visibility, &mut clock)?;
        self.append_and_publish_locked(&clock, &decision.events).await
    }

    /// 🏛️ Serializes a trusted server authority decision with its atomic event/projection append.
    pub async fn execute_artifact_authority(&self, actor: DirectoryActor, command: ArtifactDirectoryCommand) -> DirectoryResult<Vec<DirectoryEvent>> {
        let mut clock = self.write.lock().await;
        let decision = decide_artifact_authority(self.dir.as_ref(), &actor, command, &mut clock).await?;
        self.append_and_publish_locked(&clock, &decision.events).await
    }

    /// 🎫️ Commits one exact server-owned reachability reservation before CAS writes.
    pub async fn reserve_artifact_cas(&self, actor: DirectoryActor, plan: ArtifactCasOwnershipPlanV1, expires_at_ms: u64, now_ms: u64) -> DirectoryResult<ArtifactCasReservation> {
        if actor.kind != DirectoryActorKind::System {
            return Err(DirectoryError::Unauthorized);
        }
        let _write = self.write.lock().await;
        self.dir.reserve_artifact_cas(&plan, expires_at_ms, now_ms).await
    }

    /// 📣️ Consumes one live reservation with private locators, public event, and projections atomically.
    pub async fn publish_reserved_artifact_checkpoint(&self, actor: DirectoryActor, checkpoint: ArtifactCheckpoint, reservation: ArtifactCasReservation, now_ms: u64) -> DirectoryResult<Vec<DirectoryEvent>> {
        let mut clock = self.write.lock().await;
        let decision = decide_verified_checkpoint(self.dir.as_ref(), &actor, &checkpoint, &mut clock).await?;
        let persisted = match decision.events.as_slice() {
            [] => self.dir.append_reserved_artifact_checkpoint(None, &checkpoint, &reservation, now_ms).await?,
            [event] => self.dir.append_reserved_artifact_checkpoint(Some(event), &checkpoint, &reservation, now_ms).await?,
            _ => return Err(DirectoryError::Backend("verified checkpoint decision emitted more than one event".into())),
        };
        Ok(self.publish_persisted_locked(&clock, persisted))
    }

    /// 🎟️ Atomically claims one invite with its event and membership, then publishes before releasing the writer.
    pub async fn redeem_invite(&self, actor: DirectoryActor, capability: &InviteCapability, user_id: &str) -> DirectoryResult<Vec<DirectoryEvent>> {
        let mut clock = self.write.lock().await;
        if actor_user_id(&actor)? != user_id {
            return Err(DirectoryError::Unauthorized);
        }
        let hlc = clock.tick();
        match self.dir.redeem_invite_atomic(capability, &actor, user_id, hlc).await? {
            InviteRedemptionCommit::NewlyCommitted { event } => {
                let persisted = vec![event];
                #[cfg(test)]
                self.pause_publication_test_fence_once(&persisted).await;
                Ok(self.publish_persisted_locked(&clock, persisted))
            }
            InviteRedemptionCommit::AlreadyCommitted { event } => Ok(vec![event]),
        }
    }

    /// @emoji 📡️ A fresh receiver over every future published `DirectoryStreamMessage` (events,
    /// connection phases, presence, heartbeats) — `bin.rs`'s `/directory/socket/v1` handler subscribes
    /// once per connection, then replays `events_since(?since=)` before switching to live receive
    /// (contract C2's "subscribe, then replay, gap-free").
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<DirectoryStreamMessage> {
        self.tx.subscribe()
    }

    /// @emoji 📣️ Publishes a non-event stream message (connection open/close, presence roster,
    /// heartbeat) — emitted by the connection/presence layer (`bin.rs`, lane 1-B), not by this
    /// crate's own event pipeline.
    pub fn publish(&self, message: DirectoryStreamMessage) {
        let _ = self.tx.send(message);
    }

    /// 🧹️ Sweeps only historical dedicated-CAS candidates after an immediate ledger recheck.
    pub async fn sweep_artifact_cas<S: ArtifactChunkCasStorage>(
        &self,
        storage: &S,
        request: ArtifactCasSweepRequest,
        context: &crate::artifact_authority::OperationContext<'_>,
    ) -> Result<ArtifactCasSweepResult, crate::artifact_authority::AuthorityError> {
        if request.max_objects == 0 || request.max_objects > ARTIFACT_CAS_SWEEP_OBJECT_MAX {
            return Err(crate::artifact_authority::AuthorityError::ResourceLimit("artifact CAS sweep object"));
        }
        context.checkpoint()?;
        if request.execute {
            let coordinator_id = self.dir.artifact_cas_coordinator_id().await.map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
            storage.configure_coordinator(coordinator_id, context).await?;
        }
        let position = match request.continuation {
            Some(continuation) => self.artifact_cas_sweep_position(continuation, request.execute)?,
            None => {
                let _write = self.write.lock().await;
                let observed_generation = self.dir.artifact_cas_ledger_generation().await.map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
                ArtifactCasSweepPosition { execute: request.execute, observed_generation, after_generation: 0, object_offset: 0 }
            }
        };
        if request.continuation.is_some() {
            let _write = self.write.lock().await;
            let current_generation = self.dir.artifact_cas_ledger_generation().await.map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
            if current_generation != position.observed_generation {
                return Err(crate::artifact_authority::AuthorityError::Store("artifact CAS sweep continuation generation changed".into()));
            }
        }
        let mut cursor = position.after_generation;
        let mut object_offset = position.object_offset;
        let mut continuation_position = None;
        let mut examined = 0u64;
        let mut protected = 0u64;
        let mut eligible = 0u64;
        let mut deleted = 0u64;
        let mut missing = 0u64;
        let mut digest = Sha256::new();
        digest.update(b"semio.hub.artifact-cas.sweep-result.v1\0");
        while examined < request.max_objects as u64 {
            context.checkpoint()?;
            let page = self
                .dir
                .artifact_cas_sweep_candidates(cursor, position.observed_generation, ARTIFACT_CAS_SWEEP_PAGE_MAX)
                .await
                .map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
            if page.observed_generation != position.observed_generation || object_offset > page.objects.len() {
                return Err(crate::artifact_authority::AuthorityError::Store("artifact CAS sweep continuation position is invalid".into()));
            }
            let page_object_count = page.objects.len();
            for key in page.objects.into_iter().skip(object_offset) {
                if examined >= request.max_objects as u64 {
                    break;
                }
                context.checkpoint()?;
                examined += 1;
                object_offset += 1;
                digest.update(&(key.space_id.len() as u64).to_be_bytes());
                digest.update(key.space_id.as_bytes());
                digest.update(key.kind.name().as_bytes());
                digest.update(&key.digest.0);
                if !request.execute {
                    let is_protected = self
                        .dir
                        .artifact_cas_delete_preview_protected(&key, position.observed_generation, context.now_ms())
                        .await
                        .map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
                    if is_protected {
                        protected += 1;
                        digest.update(&[0]);
                    } else {
                        eligible += 1;
                        digest.update(&[3]);
                    }
                    context.report_committed(crate::artifact_authority::AuthorityProgress { stage: crate::artifact_authority::AuthorityProgressStage::CasSweep, completed_units: examined, total_units: request.max_objects as u64 });
                    semio_framework_async::yield_once().await;
                    continue;
                }
                let _write = self.write.lock().await;
                let lease_now_ms = context.now_ms();
                let lease_expires_at_ms = lease_now_ms.saturating_add(ARTIFACT_CAS_DELETE_LEASE_TTL_MS);
                let lease_token = self.artifact_cas_delete_lease_token(&key, position.observed_generation);
                let fence = self
                    .dir
                    .acquire_artifact_cas_delete_fence(&key, position.observed_generation, lease_token, lease_now_ms, lease_expires_at_ms)
                    .await
                    .map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
                match fence {
                    None => {
                        protected += 1;
                        digest.update(&[0]);
                    }
                    Some(fence) => {
                        if let Err(error) = storage.advance_physical_epoch(*fence.coordinator_id(), &key.space_id, fence.physical_epoch(), context).await {
                            let _ = self.dir.release_artifact_cas_delete_fence(fence).await;
                            return Err(error);
                        }
                        let renewal_now_ms = context.now_ms();
                        let renewal_expires_at_ms = renewal_now_ms.saturating_add(ARTIFACT_CAS_DELETE_LEASE_TTL_MS).min(context.deadline_ms());
                        let renewed = renewal_expires_at_ms > renewal_now_ms && self.dir.renew_artifact_cas_delete_fence(&fence, renewal_now_ms, renewal_expires_at_ms).await.is_ok();
                        let still_unreferenced = if renewed {
                            match self.dir.validate_artifact_cas_delete_fence(&fence, context.now_ms()).await {
                                Ok(value) => value,
                                Err(error) => {
                                    let _ = self.dir.release_artifact_cas_delete_fence(fence).await;
                                    return Err(crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)));
                                }
                            }
                        } else {
                            false
                        };
                        if still_unreferenced {
                            eligible += 1;
                            let deletion = storage.delete_if_unreferenced(&key, &fence, context).await;
                            let release = self.dir.release_artifact_cas_delete_fence(fence).await;
                            let outcome = match deletion {
                                Ok(outcome) => outcome,
                                Err(error) => {
                                    let _ = release;
                                    return Err(error);
                                }
                            };
                            release.map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
                            match outcome {
                                ArtifactCasDeleteOutcome::Deleted => {
                                    deleted += 1;
                                    digest.update(&[1]);
                                }
                                ArtifactCasDeleteOutcome::Missing => {
                                    missing += 1;
                                    digest.update(&[2]);
                                }
                            }
                        } else {
                            protected += 1;
                            let _ = self.dir.release_artifact_cas_delete_fence(fence).await;
                            digest.update(&[0]);
                        }
                    }
                }
                drop(_write);
                context.report_committed(crate::artifact_authority::AuthorityProgress { stage: crate::artifact_authority::AuthorityProgressStage::CasSweep, completed_units: examined, total_units: request.max_objects as u64 });
                semio_framework_async::yield_once().await;
            }
            if object_offset < page_object_count {
                continuation_position = Some(ArtifactCasSweepPosition { after_generation: cursor, object_offset, ..position });
                break;
            }
            if page.next_generation <= cursor || page.next_generation >= position.observed_generation {
                break;
            }
            cursor = page.next_generation;
            object_offset = 0;
            if examined >= request.max_objects as u64 {
                continuation_position = Some(ArtifactCasSweepPosition { after_generation: cursor, object_offset, ..position });
            }
        }
        let final_generation = {
            let _write = self.write.lock().await;
            let generation = self.dir.artifact_cas_ledger_generation().await.map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
            if generation < position.observed_generation {
                return Err(crate::artifact_authority::AuthorityError::Store("artifact CAS ledger generation moved backward".into()));
            }
            generation
        };
        Ok(ArtifactCasSweepResult {
            observed_generation: position.observed_generation,
            final_generation,
            examined_objects: examined,
            protected_objects: protected,
            eligible_objects: eligible,
            deleted_objects: deleted,
            missing_objects: missing,
            result_digest: ArtifactHash(digest.finalize()),
            continuation: continuation_position.map(|position| self.artifact_cas_sweep_continuation(position)),
        })
    }
}

/// 🌉️ Concrete authority-to-directory publication adapter used after exact blob readback.
pub struct HubVerifiedCheckpointPublisher<S> {
    service: Arc<DirectoryService>,
    storage: Arc<S>,
    actor_id: String,
}

impl<S> HubVerifiedCheckpointPublisher<S> {
    /// 🏗️ Binds verified publications to one system authority identity.
    pub fn new(service: Arc<DirectoryService>, storage: Arc<S>, actor_id: impl Into<String>) -> Self {
        Self { service, storage, actor_id: actor_id.into() }
    }
}

impl<S: ArtifactChunkCasStorage> crate::artifact_authority::VerifiedCheckpointPublisher for HubVerifiedCheckpointPublisher<S> {
    async fn reserve(&self, plan: &ArtifactCasOwnershipPlanV1, context: &crate::artifact_authority::OperationContext<'_>) -> Result<ArtifactCasReservation, crate::artifact_authority::AuthorityError> {
        context.checkpoint()?;
        let now_ms = context.now_ms();
        let expires_at_ms = context.deadline_ms().saturating_add(crate::artifact_authority::chunk_cas::ARTIFACT_CAS_RESERVATION_GRACE_MS).min(now_ms.saturating_add(ARTIFACT_CAS_RESERVATION_MAX_TTL_MS));
        let reservation = self
            .service
            .reserve_artifact_cas(DirectoryActor { kind: DirectoryActorKind::System, id: self.actor_id.clone() }, plan.clone(), expires_at_ms, now_ms)
            .await
            .map_err(|error| crate::artifact_authority::AuthorityError::Publication(crate::artifact_authority::adapters::bounded_message(error)))?;
        let coordinator_id = *reservation.coordinator_id();
        if coordinator_id == [0; 32] || reservation.physical_epoch() == 0 {
            return Err(crate::artifact_authority::AuthorityError::Publication("artifact CAS reservation has no physical fence permit".into()));
        }
        self.storage.configure_coordinator(coordinator_id, context).await?;
        self.storage.advance_physical_epoch(coordinator_id, &plan.scope.space_id, reservation.physical_epoch(), context).await?;
        Ok(reservation)
    }

    async fn publish_reserved(&self, checkpoint: &ArtifactCheckpoint, reservation: &ArtifactCasReservation, context: &crate::artifact_authority::OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
        context.checkpoint()?;
        self.service
            .publish_reserved_artifact_checkpoint(DirectoryActor { kind: DirectoryActorKind::System, id: self.actor_id.clone() }, checkpoint.clone(), reservation.clone(), context.now_ms())
            .await
            .map_err(|error| crate::artifact_authority::AuthorityError::Publication(crate::artifact_authority::adapters::bounded_message(error)))?;
        Ok(())
    }
}
//#endregion 🔖️Service

//#region 🔖️Trait
/// @emoji 🗄️ Backend-agnostic os-hub identity/tenancy directory. Implemented once per backend
/// (sqlite/postgres/neo4j); `HubState` holds an `Arc<HubDirectories>` (see `//#region 🔖️Dispatch`
/// below) so the directory backend is a deploy-time choice, not a compile-time one — independent of
/// `db::Database`'s own storage backend choice (see `bin.rs`, `OS_HUB_DIRECTORY_BACKEND` vs
/// `OS_HUB_STORAGE_BACKEND`).
pub trait HubDirectory: Send + Sync + 'static {
    //#region ShareTokens
    async fn issue_share_token_as(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedShareToken>;
    async fn issue_share_token(&self, scope: &DocumentScope, ttl_secs: i64, correlation_id: &str) -> DirectoryResult<IssuedShareToken> {
        self.issue_share_token_as(scope, ttl_secs, None, correlation_id).await
    }
    async fn revoke_share_token_as(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()>;
    async fn revoke_share_token(&self, scope: &DocumentScope, share_id: &str, reason: &str, correlation_id: &str) -> DirectoryResult<()> {
        self.revoke_share_token_as(scope, share_id, reason, None, correlation_id).await
    }
    async fn authenticate_share(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<bool>;
    async fn authenticate_share_binding(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<Option<ShareTokenRecord>>;
    async fn socket_share_binding(&self, share_id: &str, selector: &str, scope: &DocumentScope, now_ms: i64) -> DirectoryResult<SocketShareBindingStatus>;
    //#endregion

    //#region Users
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord>;
    /// @emoji 🔎️ Single-user lookup by id — the `member.upserted`/`invite.redeemed` projections
    /// resolve `MemberView.email`/`display_name` through this, not through `get_user_by_email`.
    async fn get_user(&self, user_id: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>>;
    async fn admin_overview_counts(&self) -> DirectoryResult<AdminDirectoryOverviewCounts>;
    //#endregion

    //#region Spaces
    /// @emoji 🔎️ Single-space lookup by id — used by the hub handler to read `kind`/`visibility`
    /// (grant compilation, public-visibility fallback) without listing every space. Also `decide`'s
    /// (`//#region 🔖️Decider`) own "does this space exist" read.
    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>>;
    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>>;
    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>>;
    async fn list_admin_space_summaries_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<AdminSpaceSummaryRecord>>;
    async fn list_admin_space_members_page(&self, space_id: &str, offset: usize, limit: usize) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>>;
    /// @emoji 🧑️‍🤝️‍🧑️ The current member roster — `decide` reads this to enforce the atelier/
    /// archive laws and to compute `archive-space`'s demote-every-author events.
    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>>;
    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>>;
    /// 🏛️ One keyset-ordered bounded administration member window (`user_id ASC`), projected to
    /// display columns inside the backend so no credential-bearing record is ever constructed.
    async fn list_space_administration_members_page(&self, space_id: &str, after_user_id: Option<&str>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationMemberRow>>;
    //#endregion

    //#region Documents
    async fn get_document_descriptor(&self, scope: &DocumentScope) -> DirectoryResult<Option<DocumentDescriptor>>;
    async fn list_document_descriptors(&self, space_id: &str) -> DirectoryResult<Vec<DocumentDescriptor>>;
    async fn list_document_descriptors_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<DocumentDescriptor>>;
    async fn get_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<PublishedArtifactCheckpoint>>;
    /// 🔐️ Internal authority read model; never serialized through directory wire DTOs.
    async fn get_verified_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<ArtifactCheckpoint>>;
    async fn get_active_artifact_checkpoint(&self, scope: &DocumentScope) -> DirectoryResult<Option<PublishedArtifactCheckpoint>>;
    async fn get_artifact_retention(&self, scope: &DocumentScope) -> DirectoryResult<Option<ArtifactRetention>>;
    async fn artifact_checkpoint_count(&self, scope: &DocumentScope) -> DirectoryResult<u64>;
    /// 🧵️ Oldest-to-newest lineage with a fixed caller bound; zero or max+1 is rejected.
    async fn list_artifact_checkpoint_lineage(&self, scope: &DocumentScope, limit: usize) -> DirectoryResult<Vec<PublishedArtifactCheckpoint>>;
    /// 🎫️ Appends one exact expiring private reachability reservation before CAS writes.
    async fn reserve_artifact_cas(&self, _plan: &ArtifactCasOwnershipPlanV1, _expires_at_ms: u64, _now_ms: u64) -> DirectoryResult<ArtifactCasReservation> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// ⚛️ Consumes one live exact reservation with the public/private checkpoint commit.
    async fn append_reserved_artifact_checkpoint(&self, _event: Option<&NewDirectoryEvent>, _checkpoint: &ArtifactCheckpoint, _reservation: &ArtifactCasReservation, _now_ms: u64) -> DirectoryResult<Vec<DirectoryEvent>> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 🧹️ Reads a bounded page of private historical object candidates for sweeping.
    async fn artifact_cas_ledger_generation(&self) -> DirectoryResult<u64> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 🪪 Reads the durable private identity that binds directory epochs to the selected CAS.
    async fn artifact_cas_coordinator_id(&self) -> DirectoryResult<[u8; 32]> {
        Err(DirectoryError::Backend("artifact CAS barrier identity is unavailable for this backend".into()))
    }
    /// 🧹️ Reads a bounded historical page through one immutable sweep generation.
    async fn artifact_cas_sweep_candidates(&self, _after_generation: u64, _through_generation: u64, _limit: usize) -> DirectoryResult<ArtifactCasSweepCandidatePage> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 🔭 Reads exact current reachability for a mutation-free dry-run preview.
    async fn artifact_cas_delete_preview_protected(&self, _key: &ArtifactCasObjectKey, _observed_generation: u64, _now_ms: u64) -> DirectoryResult<bool> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 🛡️ Atomically acquires a durable per-space deletion lease and rechecks reachability.
    async fn acquire_artifact_cas_delete_fence(&self, _key: &ArtifactCasObjectKey, _observed_generation: u64, _lease_token: [u8; 32], _now_ms: u64, _expires_at_ms: u64) -> DirectoryResult<Option<ArtifactCasDeleteFence>> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 🛡️ Rechecks the exact live lease, epoch, and reachability after CAS epoch activation.
    async fn validate_artifact_cas_delete_fence(&self, _fence: &ArtifactCasDeleteFence, _now_ms: u64) -> DirectoryResult<bool> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 💓 Renews a deletion lease while the physical store operation remains in flight.
    async fn renew_artifact_cas_delete_fence(&self, _fence: &ArtifactCasDeleteFence, _now_ms: u64, _expires_at_ms: u64) -> DirectoryResult<()> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 🔓 Releases exactly the opaque deletion lease owned by this fence.
    async fn release_artifact_cas_delete_fence(&self, _fence: ArtifactCasDeleteFence) -> DirectoryResult<()> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    //#endregion

    //#region AuthSessions
    async fn issue_auth_session(&self, issue: &AuthSessionIssue) -> DirectoryResult<IssuedAuthSession>;
    async fn authenticate_session(&self, capability: &SessionCapability) -> DirectoryResult<Option<AuthSessionRecord>>;
    async fn socket_session_binding(&self, session_id: &str, user_id: &str, authorization_generation: u64, space_id: Option<&str>, now_ms: i64) -> DirectoryResult<SocketSessionBindingStatus>;
    async fn revoke_auth_session(&self, id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Option<RevokedAuthSession>>;
    async fn revoke_auth_sessions_for_user(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>>;
    async fn revoke_auth_sessions_for_identity(&self, provider: &str, subject_digest: [u8; 32], reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>>;
    async fn list_auth_audit(&self, limit: usize, offset: usize) -> DirectoryResult<Vec<AuthAuditRecord>>;
    //#endregion

    //#region AdminOperations
    /// 🧾 Appends exactly one accepted or terminal administrator operation fact.
    async fn append_admin_operation_audit(&self, fact: &NewAdminOperationAuditRecord) -> DirectoryResult<AdminOperationAuditRecord>;
    /// 🔎 Reads the at-most-two facts for one bounded idempotency request key.
    async fn admin_operation_audit_for_request(&self, request_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>>;
    /// 🎯 Reads the at-most-two facts for one server-issued operation identifier.
    async fn admin_operation_audit_for_operation(&self, operation_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>>;
    /// 📄 Reads one backend-ordered bounded operation-audit page.
    async fn list_admin_operation_audit(&self, after_sequence: u64, limit: usize) -> DirectoryResult<Vec<AdminOperationAuditRecord>>;
    //#endregion

    //#region CommandReceipts
    /// 🔐️ Atomically claims one `(authenticated user id, request id)` idempotency key or reads the
    /// row already under it. An equal key with an unequal command digest is `Conflict` and never
    /// executes. This is the single serialization point the command writer holds across its whole
    /// decide/append sequence, so a duplicate request can never mint a second invitation.
    async fn claim_or_read_directory_command_receipt(&self, _claim: &NewDirectoryCommandReceipt) -> DirectoryResult<DirectoryCommandClaimV1> {
        Err(DirectoryError::Backend("directory command receipts are unavailable for this backend".into()))
    }
    /// 🧾️ Records the durable completion of one claimed key. The caller publishes only after this
    /// returns, so a delivered event is always covered by a durable receipt.
    async fn complete_directory_command_receipt(&self, _completion: &DirectoryCommandReceiptCompletion) -> DirectoryResult<DirectoryCommandReceiptRecord> {
        Err(DirectoryError::Backend("directory command receipts are unavailable for this backend".into()))
    }
    /// 🧹️ Releases one claimed key whose command failed before any durable event was appended.
    async fn release_directory_command_receipt(&self, _actor_user_id: &str, _request_id: &str) -> DirectoryResult<()> {
        Err(DirectoryError::Backend("directory command receipts are unavailable for this backend".into()))
    }
    //#endregion

    //#region Invites
    // 🎟️ Not event-sourced (contract's decider laws) — only redemption is (`invite.redeemed`, see
    // `DirectoryService::redeem_invite`). `create_invite`/`revoke_invite` are called directly by
    // `decide` as its one documented write exception (`//#region 🔖️Decider`).
    async fn issue_invite_as(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedInvite>;
    async fn issue_invite(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, correlation_id: &str) -> DirectoryResult<IssuedInvite> {
        self.issue_invite_as(space_id, role, ttl_secs, None, correlation_id).await
    }
    /// 🎟️ Claims `accepted_at`, appends the derived event and applies membership in one backend transaction.
    async fn redeem_invite_atomic(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str, hlc: Hlc) -> DirectoryResult<InviteRedemptionCommit>;
    async fn revoke_invite_as(&self, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()>;
    async fn revoke_invite(&self, invite_id: &str, reason: &str, correlation_id: &str) -> DirectoryResult<()> {
        self.revoke_invite_as(invite_id, reason, None, correlation_id).await
    }
    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>>;
    /// 🏛️ One keyset-ordered bounded administration invite window (`created_at DESC, id DESC`),
    /// projected to metadata inside the backend so no selector or secret digest is ever read.
    async fn list_space_administration_invites_page(&self, space_id: &str, after: Option<(i64, &str)>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationInviteRow>>;
    //#endregion

    //#region SyncSessions
    /// @emoji 🔴️ Widened over the pre-ticket signature with `space_id`/`surface` (contract's
    /// presence scope is `(space_id, document_id, surface)`).
    async fn record_sync_session_open(
        &self,
        auth_session_id: Option<&str>,
        authorization_generation: u64,
        actor_id: &str,
        space_id: &str,
        document_id: &str,
        surface: &str,
        user_id: Option<&str>,
        authenticated_email: Option<&str>,
        space_role: Option<SpaceRole>,
        client_label: &str,
    ) -> DirectoryResult<SyncSessionRecord>;
    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()>;
    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>>;
    /// @emoji 🟢️ Every still-open session, optionally scoped to one space — the admin connections
    /// view and the per-space presence roster both read this instead of iterating documents.
    async fn list_active_sync_sessions_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<SyncSessionRecord>>;
    async fn list_active_sync_sessions(&self, space_id: Option<&str>, limit: usize) -> DirectoryResult<Vec<SyncSessionRecord>> {
        self.list_active_sync_sessions_page(space_id, 0, limit).await
    }
    /// @emoji 🧹️ Marks every still-open session closed — called once at hub boot, before any real
    /// connection lands, to clear crash residue from the previous process (a session that never got
    /// its `disconnected_at` because the hub was killed mid-connection).
    async fn close_all_sync_sessions(&self) -> DirectoryResult<()>;
    //#endregion

    //#region EventLog
    /// @emoji ➕️ Persists `events` in one backend transaction: each gets a dense backend-assigned
    /// `seq` (contiguous with the current head, no gaps even under concurrent callers — callers are
    /// expected to already be serialized by `DirectoryService`'s write lock, but a backend MUST NOT
    /// rely on that alone for `seq` density since `rebuild_projections`/tests may call this
    /// directly), a minted uuid v7 `id`, and `recorded_at_ms`; then applies each event's projection
    /// (`//#region 🔖️Projections`) in the same transaction before committing.
    async fn append_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>>;
    /// @emoji 📜️ Every event with `seq > since_seq`, ascending, capped at `limit` — backs both
    /// `GET /directory/events?since=` and `/directory/socket/v1`'s post-subscribe replay (contract C2).
    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>>;
    /// @emoji 🔝️ The current log length (0 when empty) — `DirectoryStreamMessage::Heartbeat`'s
    /// `head_seq` and the admin overview's `headSeq` both read this.
    async fn head_seq(&self) -> DirectoryResult<u64>;
    /// @emoji 🔁️ Truncates every projection table and replays the whole log from `seq` 1 through
    /// each event's projection (`//#region 🔖️Projections`) — returns the number of events replayed
    /// (which must equal `head_seq()` afterward). `POST /admin/api/directory/rebuild` (contract C2).
    async fn rebuild_projections(&self) -> DirectoryResult<u64>;
    /// 🔁️ Bounded replay with monotonic progress and success-only transaction commit.
    async fn rebuild_projections_controlled(&self, control: &dyn ProjectionRebuildControl) -> DirectoryResult<u64>;
    //#endregion
}
//#endregion 🔖️Trait

//#region 🔖️Backends
// 🧭️ Top-level `mod` declarations in a real (file-backed, non-inline) module resolve `#[path]`
// relative to THIS file's own directory (🌎️hub/📇️directory/) — no cumulative/leaf-prefixed math
// needed here, that convention is for paths declared inside an entry file's inline nested `mod`
// blocks (see `rustEntryPathRules` in 🔣️taxonomy.json and `bin.rs`'s own `mod directory` line).
#[cfg(feature = "sqlite")]
#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

#[cfg(feature = "postgres")]
#[path = "🐘️postgres/🦀️.rs"]
pub mod postgres;

#[cfg(feature = "neo4j")]
#[path = "🌐️neo4j/🦀️.rs"]
pub mod neo4j;
//#endregion 🔖️Backends

//#region 🔖️Dispatch
/// 🗄️ Closed-set dispatch enum over every `HubDirectory` backend this crate can compile in.
/// Hand-written, not `dyn_enum_close!`-generated: the macro's DSL has no per-variant `#[cfg]`
/// support (verified against its parser — `DynEnumVariant::parse` never calls
/// `Attribute::parse_outer`), and each variant here is gated on its own Cargo feature — see
/// `📓️terra-dedyn-fw-hub-repo-report.md`. The shape (one `From<Backend>` impl per variant plus a
/// match-delegating `impl HubDirectory`) is otherwise identical to what the macro emits for every
/// other closed-set family in this program (R11). `bin.rs`'s `connect_directory` builds exactly
/// one variant at process startup, chosen by `OS_HUB_DIRECTORY_BACKEND`.
pub enum HubDirectories {
    #[cfg(feature = "sqlite")]
    Sqlite(sqlite::SqliteDirectory),
    #[cfg(feature = "postgres")]
    Postgres(postgres::PostgresDirectory),
    #[cfg(feature = "neo4j")]
    Neo4j(neo4j::Neo4jDirectory),
}

#[cfg(feature = "sqlite")]
impl ::core::convert::From<sqlite::SqliteDirectory> for HubDirectories {
    fn from(value: sqlite::SqliteDirectory) -> Self {
        Self::Sqlite(value)
    }
}

#[cfg(feature = "postgres")]
impl ::core::convert::From<postgres::PostgresDirectory> for HubDirectories {
    fn from(value: postgres::PostgresDirectory) -> Self {
        Self::Postgres(value)
    }
}

#[cfg(feature = "neo4j")]
impl ::core::convert::From<neo4j::Neo4jDirectory> for HubDirectories {
    fn from(value: neo4j::Neo4jDirectory) -> Self {
        Self::Neo4j(value)
    }
}

impl HubDirectory for HubDirectories {
    async fn issue_share_token_as(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedShareToken> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.issue_share_token_as(scope, ttl_secs, actor_user_id, correlation_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.issue_share_token_as(scope, ttl_secs, actor_user_id, correlation_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.issue_share_token_as(scope, ttl_secs, actor_user_id, correlation_id).await,
        }
    }

    async fn revoke_share_token_as(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.revoke_share_token_as(scope, share_id, reason, actor_user_id, correlation_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.revoke_share_token_as(scope, share_id, reason, actor_user_id, correlation_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.revoke_share_token_as(scope, share_id, reason, actor_user_id, correlation_id).await,
        }
    }

    async fn authenticate_share(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<bool> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.authenticate_share(scope, capability).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.authenticate_share(scope, capability).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.authenticate_share(scope, capability).await,
        }
    }

    async fn authenticate_share_binding(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<Option<ShareTokenRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.authenticate_share_binding(scope, capability).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.authenticate_share_binding(scope, capability).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.authenticate_share_binding(scope, capability).await,
        }
    }

    async fn socket_share_binding(&self, share_id: &str, selector: &str, scope: &DocumentScope, now_ms: i64) -> DirectoryResult<SocketShareBindingStatus> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.socket_share_binding(share_id, selector, scope, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.socket_share_binding(share_id, selector, scope, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.socket_share_binding(share_id, selector, scope, now_ms).await,
        }
    }

    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.create_user(email, display_name, password_hash, sso_subject, sso_provider).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.create_user(email, display_name, password_hash, sso_subject, sso_provider).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.create_user(email, display_name, password_hash, sso_subject, sso_provider).await,
        }
    }

    async fn get_user(&self, user_id: &str) -> DirectoryResult<Option<UserRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_user(user_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_user(user_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_user(user_id).await,
        }
    }

    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_user_by_email(email).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_user_by_email(email).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_user_by_email(email).await,
        }
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_user_by_sso_subject(provider, subject).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_user_by_sso_subject(provider, subject).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_user_by_sso_subject(provider, subject).await,
        }
    }

    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_users(limit, offset).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_users(limit, offset).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_users(limit, offset).await,
        }
    }

    async fn admin_overview_counts(&self) -> DirectoryResult<AdminDirectoryOverviewCounts> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.admin_overview_counts().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.admin_overview_counts().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.admin_overview_counts().await,
        }
    }

    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_space(space_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_space(space_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_space(space_id).await,
        }
    }

    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_spaces_for_user(user_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_spaces_for_user(user_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_spaces_for_user(user_id).await,
        }
    }

    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_spaces(limit, offset).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_spaces(limit, offset).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_spaces(limit, offset).await,
        }
    }

    async fn list_admin_space_summaries_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<AdminSpaceSummaryRecord>> {
        if limit == 0 || limit > ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator space page limit must be 1..={ADMIN_PAGE_FETCH_MAX}")));
        }
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_admin_space_summaries_page(space_id, offset, limit).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_admin_space_summaries_page(space_id, offset, limit).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_admin_space_summaries_page(space_id, offset, limit).await,
        }
    }

    async fn list_admin_space_members_page(&self, space_id: &str, offset: usize, limit: usize) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        if limit == 0 || limit > ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator member page limit must be 1..={ADMIN_PAGE_FETCH_MAX}")));
        }
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_admin_space_members_page(space_id, offset, limit).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_admin_space_members_page(space_id, offset, limit).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_admin_space_members_page(space_id, offset, limit).await,
        }
    }

    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_members(space_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_members(space_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_members(space_id).await,
        }
    }

    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_role(space_id, user_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_role(space_id, user_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_role(space_id, user_id).await,
        }
    }

    async fn list_space_administration_members_page(&self, space_id: &str, after_user_id: Option<&str>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationMemberRow>> {
        if limit == 0 || limit > SPACE_ADMINISTRATION_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("space administration member page limit must be 1..={SPACE_ADMINISTRATION_PAGE_FETCH_MAX}")));
        }
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_space_administration_members_page(space_id, after_user_id, limit).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_space_administration_members_page(space_id, after_user_id, limit).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_space_administration_members_page(space_id, after_user_id, limit).await,
        }
    }

    async fn get_document_descriptor(&self, scope: &DocumentScope) -> DirectoryResult<Option<DocumentDescriptor>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_document_descriptor(scope).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_document_descriptor(scope).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_document_descriptor(scope).await,
        }
    }

    async fn list_document_descriptors(&self, space_id: &str) -> DirectoryResult<Vec<DocumentDescriptor>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_document_descriptors(space_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_document_descriptors(space_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_document_descriptors(space_id).await,
        }
    }

    async fn list_document_descriptors_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<DocumentDescriptor>> {
        if limit == 0 || limit > ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator document page limit must be 1..={ADMIN_PAGE_FETCH_MAX}")));
        }
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_document_descriptors_page(space_id, offset, limit).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_document_descriptors_page(space_id, offset, limit).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_document_descriptors_page(space_id, offset, limit).await,
        }
    }

    async fn get_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_artifact_checkpoint(scope, checkpoint_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_artifact_checkpoint(scope, checkpoint_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_artifact_checkpoint(scope, checkpoint_id).await,
        }
    }

    async fn get_verified_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<ArtifactCheckpoint>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_verified_artifact_checkpoint(scope, checkpoint_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_verified_artifact_checkpoint(scope, checkpoint_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_verified_artifact_checkpoint(scope, checkpoint_id).await,
        }
    }

    async fn get_active_artifact_checkpoint(&self, scope: &DocumentScope) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_active_artifact_checkpoint(scope).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_active_artifact_checkpoint(scope).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_active_artifact_checkpoint(scope).await,
        }
    }

    async fn get_artifact_retention(&self, scope: &DocumentScope) -> DirectoryResult<Option<ArtifactRetention>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_artifact_retention(scope).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_artifact_retention(scope).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_artifact_retention(scope).await,
        }
    }

    async fn artifact_checkpoint_count(&self, scope: &DocumentScope) -> DirectoryResult<u64> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.artifact_checkpoint_count(scope).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.artifact_checkpoint_count(scope).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.artifact_checkpoint_count(scope).await,
        }
    }

    async fn list_artifact_checkpoint_lineage(&self, scope: &DocumentScope, limit: usize) -> DirectoryResult<Vec<PublishedArtifactCheckpoint>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_artifact_checkpoint_lineage(scope, limit).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_artifact_checkpoint_lineage(scope, limit).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_artifact_checkpoint_lineage(scope, limit).await,
        }
    }

    async fn reserve_artifact_cas(&self, plan: &ArtifactCasOwnershipPlanV1, expires_at_ms: u64, now_ms: u64) -> DirectoryResult<ArtifactCasReservation> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.reserve_artifact_cas(plan, expires_at_ms, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.reserve_artifact_cas(plan, expires_at_ms, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.reserve_artifact_cas(plan, expires_at_ms, now_ms).await,
        }
    }

    async fn append_reserved_artifact_checkpoint(&self, event: Option<&NewDirectoryEvent>, checkpoint: &ArtifactCheckpoint, reservation: &ArtifactCasReservation, now_ms: u64) -> DirectoryResult<Vec<DirectoryEvent>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.append_reserved_artifact_checkpoint(event, checkpoint, reservation, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.append_reserved_artifact_checkpoint(event, checkpoint, reservation, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.append_reserved_artifact_checkpoint(event, checkpoint, reservation, now_ms).await,
        }
    }

    async fn artifact_cas_ledger_generation(&self) -> DirectoryResult<u64> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.artifact_cas_ledger_generation().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.artifact_cas_ledger_generation().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.artifact_cas_ledger_generation().await,
        }
    }

    async fn artifact_cas_coordinator_id(&self) -> DirectoryResult<[u8; 32]> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.artifact_cas_coordinator_id().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.artifact_cas_coordinator_id().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.artifact_cas_coordinator_id().await,
        }
    }

    async fn artifact_cas_sweep_candidates(&self, after_generation: u64, through_generation: u64, limit: usize) -> DirectoryResult<ArtifactCasSweepCandidatePage> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.artifact_cas_sweep_candidates(after_generation, through_generation, limit).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.artifact_cas_sweep_candidates(after_generation, through_generation, limit).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.artifact_cas_sweep_candidates(after_generation, through_generation, limit).await,
        }
    }

    async fn artifact_cas_delete_preview_protected(&self, key: &ArtifactCasObjectKey, observed_generation: u64, now_ms: u64) -> DirectoryResult<bool> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.artifact_cas_delete_preview_protected(key, observed_generation, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.artifact_cas_delete_preview_protected(key, observed_generation, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.artifact_cas_delete_preview_protected(key, observed_generation, now_ms).await,
        }
    }

    async fn acquire_artifact_cas_delete_fence(&self, key: &ArtifactCasObjectKey, observed_generation: u64, lease_token: [u8; 32], now_ms: u64, expires_at_ms: u64) -> DirectoryResult<Option<ArtifactCasDeleteFence>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.acquire_artifact_cas_delete_fence(key, observed_generation, lease_token, now_ms, expires_at_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.acquire_artifact_cas_delete_fence(key, observed_generation, lease_token, now_ms, expires_at_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.acquire_artifact_cas_delete_fence(key, observed_generation, lease_token, now_ms, expires_at_ms).await,
        }
    }

    async fn validate_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64) -> DirectoryResult<bool> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.validate_artifact_cas_delete_fence(fence, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.validate_artifact_cas_delete_fence(fence, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.validate_artifact_cas_delete_fence(fence, now_ms).await,
        }
    }

    async fn renew_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64, expires_at_ms: u64) -> DirectoryResult<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.renew_artifact_cas_delete_fence(fence, now_ms, expires_at_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.renew_artifact_cas_delete_fence(fence, now_ms, expires_at_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.renew_artifact_cas_delete_fence(fence, now_ms, expires_at_ms).await,
        }
    }

    async fn release_artifact_cas_delete_fence(&self, fence: ArtifactCasDeleteFence) -> DirectoryResult<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.release_artifact_cas_delete_fence(fence).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.release_artifact_cas_delete_fence(fence).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.release_artifact_cas_delete_fence(fence).await,
        }
    }

    async fn issue_auth_session(&self, issue: &AuthSessionIssue) -> DirectoryResult<IssuedAuthSession> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.issue_auth_session(issue).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.issue_auth_session(issue).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.issue_auth_session(issue).await,
        }
    }

    async fn authenticate_session(&self, capability: &SessionCapability) -> DirectoryResult<Option<AuthSessionRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.authenticate_session(capability).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.authenticate_session(capability).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.authenticate_session(capability).await,
        }
    }

    async fn socket_session_binding(&self, session_id: &str, user_id: &str, authorization_generation: u64, space_id: Option<&str>, now_ms: i64) -> DirectoryResult<SocketSessionBindingStatus> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.socket_session_binding(session_id, user_id, authorization_generation, space_id, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.socket_session_binding(session_id, user_id, authorization_generation, space_id, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.socket_session_binding(session_id, user_id, authorization_generation, space_id, now_ms).await,
        }
    }

    async fn revoke_auth_session(&self, id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Option<RevokedAuthSession>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.revoke_auth_session(id, reason, actor_user_id, correlation_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.revoke_auth_session(id, reason, actor_user_id, correlation_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.revoke_auth_session(id, reason, actor_user_id, correlation_id).await,
        }
    }

    async fn revoke_auth_sessions_for_user(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.revoke_auth_sessions_for_user(user_id, reason, actor_user_id, correlation_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.revoke_auth_sessions_for_user(user_id, reason, actor_user_id, correlation_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.revoke_auth_sessions_for_user(user_id, reason, actor_user_id, correlation_id).await,
        }
    }

    async fn revoke_auth_sessions_for_identity(&self, provider: &str, subject_digest: [u8; 32], reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.revoke_auth_sessions_for_identity(provider, subject_digest, reason, actor_user_id, correlation_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.revoke_auth_sessions_for_identity(provider, subject_digest, reason, actor_user_id, correlation_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.revoke_auth_sessions_for_identity(provider, subject_digest, reason, actor_user_id, correlation_id).await,
        }
    }

    async fn list_auth_audit(&self, limit: usize, offset: usize) -> DirectoryResult<Vec<AuthAuditRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_auth_audit(limit, offset).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_auth_audit(limit, offset).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_auth_audit(limit, offset).await,
        }
    }

    async fn append_admin_operation_audit(&self, fact: &NewAdminOperationAuditRecord) -> DirectoryResult<AdminOperationAuditRecord> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.append_admin_operation_audit(fact).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.append_admin_operation_audit(fact).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.append_admin_operation_audit(fact).await,
        }
    }

    async fn admin_operation_audit_for_request(&self, request_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.admin_operation_audit_for_request(request_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.admin_operation_audit_for_request(request_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.admin_operation_audit_for_request(request_id).await,
        }
    }

    async fn admin_operation_audit_for_operation(&self, operation_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.admin_operation_audit_for_operation(operation_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.admin_operation_audit_for_operation(operation_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.admin_operation_audit_for_operation(operation_id).await,
        }
    }

    async fn list_admin_operation_audit(&self, after_sequence: u64, limit: usize) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_admin_operation_audit(after_sequence, limit).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_admin_operation_audit(after_sequence, limit).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_admin_operation_audit(after_sequence, limit).await,
        }
    }

    async fn issue_invite_as(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedInvite> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.issue_invite_as(space_id, role, ttl_secs, actor_user_id, correlation_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.issue_invite_as(space_id, role, ttl_secs, actor_user_id, correlation_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.issue_invite_as(space_id, role, ttl_secs, actor_user_id, correlation_id).await,
        }
    }

    async fn redeem_invite_atomic(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str, hlc: Hlc) -> DirectoryResult<InviteRedemptionCommit> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.redeem_invite_atomic(capability, actor, user_id, hlc).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.redeem_invite_atomic(capability, actor, user_id, hlc).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.redeem_invite_atomic(capability, actor, user_id, hlc).await,
        }
    }

    async fn revoke_invite_as(&self, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.revoke_invite_as(invite_id, reason, actor_user_id, correlation_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.revoke_invite_as(invite_id, reason, actor_user_id, correlation_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.revoke_invite_as(invite_id, reason, actor_user_id, correlation_id).await,
        }
    }

    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_invites(space_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_invites(space_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_invites(space_id).await,
        }
    }

    async fn list_space_administration_invites_page(&self, space_id: &str, after: Option<(i64, &str)>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationInviteRow>> {
        if limit == 0 || limit > SPACE_ADMINISTRATION_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("space administration invite page limit must be 1..={SPACE_ADMINISTRATION_PAGE_FETCH_MAX}")));
        }
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_space_administration_invites_page(space_id, after, limit).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_space_administration_invites_page(space_id, after, limit).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_space_administration_invites_page(space_id, after, limit).await,
        }
    }

    async fn record_sync_session_open(
        &self,
        auth_session_id: Option<&str>,
        authorization_generation: u64,
        actor_id: &str,
        space_id: &str,
        document_id: &str,
        surface: &str,
        user_id: Option<&str>,
        authenticated_email: Option<&str>,
        space_role: Option<SpaceRole>,
        client_label: &str,
    ) -> DirectoryResult<SyncSessionRecord> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.record_sync_session_open(auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.record_sync_session_open(auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.record_sync_session_open(auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label).await,
        }
    }

    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.record_sync_session_close(sync_session_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.record_sync_session_close(sync_session_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.record_sync_session_close(sync_session_id).await,
        }
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_sync_sessions_for_document(document_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_sync_sessions_for_document(document_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_sync_sessions_for_document(document_id).await,
        }
    }

    async fn list_active_sync_sessions_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<SyncSessionRecord>> {
        if limit == 0 || limit > ACTIVE_SYNC_SESSION_READ_MAX {
            return Err(DirectoryError::Conflict(format!("active sync-session limit must be 1..={ACTIVE_SYNC_SESSION_READ_MAX}")));
        }
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_active_sync_sessions_page(space_id, offset, limit).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_active_sync_sessions_page(space_id, offset, limit).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_active_sync_sessions_page(space_id, offset, limit).await,
        }
    }

    async fn close_all_sync_sessions(&self) -> DirectoryResult<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.close_all_sync_sessions().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.close_all_sync_sessions().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.close_all_sync_sessions().await,
        }
    }

    async fn append_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.append_events(events).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.append_events(events).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.append_events(events).await,
        }
    }

    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.events_since(since_seq, limit).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.events_since(since_seq, limit).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.events_since(since_seq, limit).await,
        }
    }

    async fn head_seq(&self) -> DirectoryResult<u64> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.head_seq().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.head_seq().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.head_seq().await,
        }
    }

    async fn rebuild_projections(&self) -> DirectoryResult<u64> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.rebuild_projections().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.rebuild_projections().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.rebuild_projections().await,
        }
    }

    async fn rebuild_projections_controlled(&self, control: &dyn ProjectionRebuildControl) -> DirectoryResult<u64> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.rebuild_projections_controlled(control).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.rebuild_projections_controlled(control).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.rebuild_projections_controlled(control).await,
        }
    }
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
// 🔬️ Exercises `decide`/`DirectoryService` against the sqlite backend (the only one the default
// feature set — Amendment 2 — actually compiles/runs here); postgres/neo4j get the same coverage
// via their own `#[cfg(test)]` modules once `🛢️db`'s optional-dependency gap is fixed (not this
// lane's to fix, see the lane report).
#[cfg(all(test, feature = "sqlite"))]
mod tests {
    use super::sqlite::SqliteDirectory;
    use super::*;
    use crate::artifact_authority::chunk_cas::{
        artifact_cas_manifest_locator_v1, prepare_artifact_cas_manifest_v1, prepare_artifact_cas_ownership_v1, ArtifactCasObjectKind, ArtifactChunkBlobStore, FsArtifactChunkCasStorage, MemoryArtifactChunkCasStorage,
    };
    use crate::artifact_authority::{ArtifactBlobIntegrity, ArtifactPair, AuthorityLimits, AuthorityOperationControl, AuthorityProgress, AuthorityProgressStage, ImmutableArtifactBlobStore, OperationContext, StagedArtifactBlob};
    use db::db_storage::{DbIoPageWriter, MemoryStorage as GenericMemoryStorage, PayloadStorage, DB_IO_PAGE_BYTES};
    use directory::{DslValue, FromValue, ToValue};
    use std::collections::BTreeSet;
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
    use std::sync::Arc;

    struct IdentityProbe {
        now: i64,
        cancelled: bool,
        progress: std::sync::Mutex<Vec<IdentityVerificationProgress>>,
    }

    impl IdentityVerificationControl for IdentityProbe {
        fn now_ms(&self) -> i64 {
            self.now
        }

        fn is_cancelled(&self) -> bool {
            self.cancelled
        }

        fn report(&self, progress: IdentityVerificationProgress) {
            self.progress.lock().expect("identity progress lock").push(progress);
        }
    }

    struct TestIdentityVerifier {
        fail: bool,
    }

    impl IdentityAssertionVerifier for TestIdentityVerifier {
        fn verify<'a>(&'a self, assertion: &'a IdentityAssertion, context: &'a IdentityVerificationContext<'a>) -> IdentityVerificationFuture<'a> {
            Box::pin(async move {
                context.checkpoint(0, 2)?;
                semio_framework_async::yield_once().await;
                context.checkpoint(1, 2)?;
                if self.fail || assertion.as_bytes() != b"signed-test-assertion" {
                    return Err(DirectoryError::Unauthorized);
                }
                context.checkpoint(2, 2)?;
                Ok(VerifiedIdentity {
                    provider: "test-verifier".into(),
                    subject: "test-subject".into(),
                    verified_email: Some("verified@example.com".into()),
                    display_name: Some("Verified".into()),
                    issued_at: 1,
                    expires_at: 2,
                    assurance: IdentityAssurance::ExternalVerified,
                })
            })
        }
    }

    #[test]
    fn typed_capabilities_match_neutral_sha256_vectors_and_fixed_boundaries() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🔐️auth/🧪️fixtures/🔑️capability-v1/🔣️.json")).expect("auth capability fixture");
        let session = SessionCapability::parse(fixture["session"]["capability"].as_str().expect("session capability")).expect("session parser");
        let share = ShareCapability::parse(fixture["share"]["capability"].as_str().expect("share capability")).expect("share parser");
        let invite = InviteCapability::parse(fixture["invite"]["capability"].as_str().expect("invite capability")).expect("invite parser");
        let socket = SocketGrantCapability::parse(fixture["socket"]["capability"].as_str().expect("socket capability")).expect("socket parser");
        assert_eq!(session.selector(), fixture["session"]["selector"].as_str().expect("session selector"));
        assert_eq!(encode_capability_bytes(&session.secret_digest()), fixture["session"]["digestHex"].as_str().expect("session digest"));
        assert_eq!(encode_capability_bytes(&share.secret_digest()), fixture["share"]["digestHex"].as_str().expect("share digest"));
        assert_eq!(encode_capability_bytes(&invite.secret_digest()), fixture["invite"]["digestHex"].as_str().expect("invite digest"));
        assert_eq!(socket.selector(), fixture["socket"]["selector"].as_str().expect("socket selector"));
        assert_eq!(encode_capability_bytes(&socket.secret_digest()), fixture["socket"]["digestHex"].as_str().expect("socket digest"));
        assert_eq!(socket.expose_once().len(), 107);
        assert!(HubCapability::parse(&socket.expose_once()).is_err(), "socket grants are never HTTP hub capabilities");
        let socket_rejections = fixture["socket"]["rejectedCapabilities"].as_array().expect("socket rejection vectors");
        for rejected in &socket_rejections[..4] {
            assert!(SocketGrantCapability::parse(rejected.as_str().expect("socket rejection")).is_err());
        }
        let wrong_secret = SocketGrantCapability::parse(socket_rejections[4].as_str().expect("wrong-secret socket capability")).expect("wrong-secret grammar is valid");
        assert!(!constant_time_digest_eq(&wrong_secret.secret_digest(), &socket.secret_digest()));
        assert!(SessionCapability::parse(fixture["share"]["capability"].as_str().expect("share capability")).is_err());
        assert!(ShareCapability::parse(&share.expose_once().to_uppercase()).is_err());
        assert_eq!(capability_window(1, CAPABILITY_MAX_TTL_SECS).expect("maximum ttl").1, 1 + CAPABILITY_MAX_TTL_SECS * 1_000);
        assert!(capability_window(1, CAPABILITY_MAX_TTL_SECS + 1).is_err());
        assert!(IdentityAssertion::new(vec![0; AUTH_ASSERTION_MAX_BYTES].into_boxed_slice()).is_ok());
        assert!(IdentityAssertion::new(vec![0; AUTH_ASSERTION_MAX_BYTES + 1].into_boxed_slice()).is_err());
        let mut issue = AuthSessionIssue {
            user_id: "fixture-user".into(),
            identity_provider: "oidc.example".into(),
            identity_subject_digest: [7; 32],
            ttl_secs: 60,
            device_instance_id: "d".repeat(DEVICE_INSTANCE_MAX_BYTES),
            session_kind: AuthSessionKind::External,
            correlation_id: "fixture-correlation".into(),
            peer_class: "fixture".into(),
        };
        assert!(prepare_auth_session(&issue, 1).is_ok());
        issue.device_instance_id.push('d');
        assert!(prepare_auth_session(&issue, 1).is_err());
        assert!(constant_time_digest_eq(&[9; 32], &[9; 32]));
        assert!(!constant_time_digest_eq(&[9; 32], &[8; 32]));
        assert_eq!(
            encode_capability_bytes(&identity_subject_digest(fixture["identity"]["provider"].as_str().expect("provider"), fixture["identity"]["subject"].as_str().expect("subject")).expect("identity digest")),
            fixture["identity"]["digestHex"].as_str().expect("identity digest vector"),
        );
    }

    #[tokio::test]
    async fn identity_verifier_port_honors_progress_cancel_deadline_and_provider_error() {
        let assertion = IdentityAssertion::new(b"signed-test-assertion".to_vec().into_boxed_slice()).expect("bounded assertion");
        let success = IdentityProbe { now: 10, cancelled: false, progress: std::sync::Mutex::new(Vec::new()) };
        let verified = TestIdentityVerifier { fail: false }.verify(&assertion, &IdentityVerificationContext { deadline_ms: 10, control: &success }).await.expect("verified identity");
        assert_eq!(verified.subject, "test-subject");
        assert_eq!(
            success.progress.into_inner().expect("success progress"),
            vec![IdentityVerificationProgress { completed_units: 0, total_units: 2 }, IdentityVerificationProgress { completed_units: 1, total_units: 2 }, IdentityVerificationProgress { completed_units: 2, total_units: 2 }]
        );

        let cancelled = IdentityProbe { now: 10, cancelled: true, progress: std::sync::Mutex::new(Vec::new()) };
        assert!(matches!(TestIdentityVerifier { fail: false }.verify(&assertion, &IdentityVerificationContext { deadline_ms: 10, control: &cancelled }).await, Err(DirectoryError::Conflict(_))));
        let expired = IdentityProbe { now: 11, cancelled: false, progress: std::sync::Mutex::new(Vec::new()) };
        assert!(matches!(TestIdentityVerifier { fail: false }.verify(&assertion, &IdentityVerificationContext { deadline_ms: 10, control: &expired }).await, Err(DirectoryError::Conflict(_))));
        let provider_error = IdentityProbe { now: 10, cancelled: false, progress: std::sync::Mutex::new(Vec::new()) };
        assert!(matches!(TestIdentityVerifier { fail: true }.verify(&assertion, &IdentityVerificationContext { deadline_ms: 10, control: &provider_error }).await, Err(DirectoryError::Unauthorized)));
    }

    fn user_actor(user_id: &str) -> DirectoryActor {
        DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{user_id}#s1") }
    }

    fn descriptor(space_id: &str, document_id: &str) -> DocumentDescriptor {
        DocumentDescriptor {
            space_id: space_id.into(),
            document_id: document_id.into(),
            artifact_kind: "s.gis.gismap".into(),
            artifact_schema: "s.gis.gismap@1/*".into(),
            owner: directory::os_directory::DocumentOwner { plugin_id: "s.gis".into(), package_id: "s.gis.gismap".into(), version: "1.0.0".into(), package_hash: "22".repeat(32) },
            pack_schema_hash: "11".repeat(32),
            bootstrap_version: 1,
            bootstrap_frontier: directory::os_directory::DocumentFrontier { head_seq: 7, commit_seq: 7, epoch: 2 },
            bootstrap_snapshot_hash: "33".repeat(32),
        }
    }

    fn artifact_projection_fixture() -> (DocumentDescriptor, PublishedArtifactCheckpoint, PublishedArtifactCheckpoint, ArtifactRetention, u64) {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/📸️artifact-checkpoint-projection.json")).expect("checkpoint projection fixture");
        let decode = |field: &str| DslValue::from(fixture.get(field).expect("fixture field").clone());
        (
            DocumentDescriptor::from_value(decode("descriptor")).expect("fixture descriptor"),
            PublishedArtifactCheckpoint::from_value(decode("checkpoint1")).expect("fixture checkpoint 1"),
            PublishedArtifactCheckpoint::from_value(decode("checkpoint2")).expect("fixture checkpoint 2"),
            ArtifactRetention::from_value(decode("retention")).expect("fixture retention"),
            fixture["lineageMaximum"].as_u64().expect("fixture maximum"),
        )
    }

    fn verified_checkpoint(checkpoint: &PublishedArtifactCheckpoint, locator_suffix: &str) -> ArtifactCheckpoint {
        let mut verified = checkpoint_identity_input(checkpoint);
        verified.pack.storage_key = format!("semio.artifact-cas.manifest/v1/{}", semio_framework_hash::hex_lower(&Sha256::digest(format!("pack:{locator_suffix}").as_bytes())));
        verified.spr.storage_key = format!("semio.artifact-cas.manifest/v1/{}", semio_framework_hash::hex_lower(&Sha256::digest(format!("spr:{locator_suffix}").as_bytes())));
        verified
    }

    fn ownership_plan(checkpoint: &ArtifactCheckpoint) -> ArtifactCasOwnershipPlanV1 {
        let pack_manifest_id = crate::artifact_authority::chunk_cas::decode_artifact_cas_manifest_locator_v1(&checkpoint.pack.storage_key).expect("pack manifest locator");
        let spr_manifest_id = crate::artifact_authority::chunk_cas::decode_artifact_cas_manifest_locator_v1(&checkpoint.spr.storage_key).expect("SPR manifest locator");
        let mut objects = vec![
            ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Manifest, digest: pack_manifest_id },
            ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Manifest, digest: spr_manifest_id },
        ];
        objects.sort_by_key(|object| (object.kind, object.digest.0));
        objects.dedup();
        ArtifactCasOwnershipPlanV1 { scope: checkpoint.scope.clone(), checkpoint_id: checkpoint.checkpoint_id, pack_manifest_id, spr_manifest_id, objects }
    }

    async fn publish_reserved(service: &DirectoryService, actor: DirectoryActor, checkpoint: ArtifactCheckpoint) -> DirectoryResult<Vec<DirectoryEvent>> {
        let reservation = service.reserve_artifact_cas(actor.clone(), ownership_plan(&checkpoint), 1_000, 100).await?;
        service.publish_reserved_artifact_checkpoint(actor, checkpoint, reservation, 100).await
    }

    struct ArtifactCasProbe {
        now_ms: AtomicU64,
        cancel_after_sweep: Option<u64>,
        cancelled: AtomicBool,
        progress: std::sync::Mutex<Vec<AuthorityProgress>>,
    }

    impl ArtifactCasProbe {
        fn new(now_ms: u64, cancel_after_sweep: Option<u64>) -> Self {
            Self { now_ms: AtomicU64::new(now_ms), cancel_after_sweep, cancelled: AtomicBool::new(false), progress: std::sync::Mutex::new(Vec::new()) }
        }
    }

    impl AuthorityOperationControl for ArtifactCasProbe {
        fn now_ms(&self) -> u64 {
            self.now_ms.load(Ordering::SeqCst)
        }

        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::SeqCst)
        }

        fn report(&self, progress: AuthorityProgress) {
            self.progress.lock().expect("artifact CAS progress").push(progress);
            if progress.stage == AuthorityProgressStage::CasSweep && self.cancel_after_sweep.is_some_and(|after| progress.completed_units >= after) {
                self.cancelled.store(true, Ordering::SeqCst);
            }
        }
    }

    struct BlockingDeleteArtifactCas {
        inner: Arc<MemoryArtifactChunkCasStorage>,
        block_next: AtomicBool,
        entered: tokio::sync::Notify,
        release: tokio::sync::Notify,
    }

    impl BlockingDeleteArtifactCas {
        fn new() -> Self {
            Self { inner: Arc::new(MemoryArtifactChunkCasStorage::default()), block_next: AtomicBool::new(true), entered: tokio::sync::Notify::new(), release: tokio::sync::Notify::new() }
        }
    }

    impl ArtifactChunkCasStorage for BlockingDeleteArtifactCas {
        async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
            self.inner.configure_coordinator(coordinator_id, context).await
        }

        async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
            self.inner.advance_physical_epoch(coordinator_id, space_id, epoch, context).await
        }

        async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<crate::artifact_authority::chunk_cas::ArtifactCasPutOutcome, crate::artifact_authority::AuthorityError> {
            self.inner.put_if_absent(key, bytes, context).await
        }

        async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, crate::artifact_authority::AuthorityError> {
            self.inner.get(key, context).await
        }

        async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, crate::artifact_authority::AuthorityError> {
            if self.block_next.swap(false, Ordering::SeqCst) {
                self.entered.notify_one();
                self.release.notified().await;
            }
            self.inner.delete_if_unreferenced(key, fence, context).await
        }
    }

    struct ProcessBlockingDeleteArtifactCas {
        inner: FsArtifactChunkCasStorage,
        entered: std::path::PathBuf,
        release: std::path::PathBuf,
    }

    impl ArtifactChunkCasStorage for ProcessBlockingDeleteArtifactCas {
        async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
            self.inner.configure_coordinator(coordinator_id, context).await
        }

        async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
            self.inner.advance_physical_epoch(coordinator_id, space_id, epoch, context).await
        }

        async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<crate::artifact_authority::chunk_cas::ArtifactCasPutOutcome, crate::artifact_authority::AuthorityError> {
            self.inner.put_if_absent(key, bytes, context).await
        }

        async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, crate::artifact_authority::AuthorityError> {
            self.inner.get(key, context).await
        }

        async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, crate::artifact_authority::AuthorityError> {
            std::fs::write(&self.entered, []).map_err(|_| crate::artifact_authority::AuthorityError::Store("artifact CAS process race readiness write failed".into()))?;
            let wait_deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(30);
            while !self.release.exists() {
                context.checkpoint()?;
                if tokio::time::Instant::now() >= wait_deadline {
                    return Err(crate::artifact_authority::AuthorityError::Store("artifact CAS process race release timed out".into()));
                }
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            }
            self.inner.delete_if_unreferenced(key, fence, context).await
        }
    }

    struct FailingAdvanceArtifactCas(MemoryArtifactChunkCasStorage);

    impl ArtifactChunkCasStorage for FailingAdvanceArtifactCas {
        async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
            self.0.configure_coordinator(coordinator_id, context).await
        }

        async fn advance_physical_epoch(&self, _: [u8; 32], _: &str, _: u64, _: &OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
            Err(crate::artifact_authority::AuthorityError::Cancelled)
        }

        async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<crate::artifact_authority::chunk_cas::ArtifactCasPutOutcome, crate::artifact_authority::AuthorityError> {
            self.0.put_if_absent(key, bytes, context).await
        }

        async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, crate::artifact_authority::AuthorityError> {
            self.0.get(key, context).await
        }

        async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, crate::artifact_authority::AuthorityError> {
            self.0.delete_if_unreferenced(key, fence, context).await
        }
    }

    fn materialized_checkpoint(mut public: PublishedArtifactCheckpoint, pair: &ArtifactPair) -> ArtifactCheckpoint {
        public.pack = PublishedArtifactBlob { sha256: ArtifactHash(Sha256::digest(&pair.pack)), byte_length: pair.pack.len() as u64 };
        public.spr = PublishedArtifactBlob { sha256: ArtifactHash(Sha256::digest(&pair.spr)), byte_length: pair.spr.len() as u64 };
        let mut aggregate = Sha256::new();
        aggregate.update(&pair.pack);
        aggregate.update(&pair.spr);
        public.aggregate_sha256 = ArtifactHash(aggregate.finalize());
        public.checkpoint_id = ArtifactHash(Sha256::digest(&crate::artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&public)).expect("checkpoint identity")));
        let mut checkpoint = checkpoint_identity_input(&public);
        checkpoint.pack.storage_key = artifact_cas_manifest_locator_v1(prepare_artifact_cas_manifest_v1(&checkpoint.scope.space_id, &pair.pack).expect("pack plan").manifest_id);
        checkpoint.spr.storage_key = artifact_cas_manifest_locator_v1(prepare_artifact_cas_manifest_v1(&checkpoint.scope.space_id, &pair.spr).expect("SPR plan").manifest_id);
        checkpoint
    }

    fn scoped_materialized_checkpoint(mut public: PublishedArtifactCheckpoint, descriptor: &DocumentDescriptor, pair: &ArtifactPair, parent_checkpoint_id: Option<ArtifactHash>, ordinal: u64) -> ArtifactCheckpoint {
        public.scope = DocumentScope::new(descriptor.space_id.clone(), descriptor.document_id.clone());
        public.parent_checkpoint_id = parent_checkpoint_id;
        public.descriptor_digest_v1 = descriptor_digest_v1(descriptor).expect("descriptor digest");
        public.baseline_frontier.document_id = descriptor.document_id.clone();
        public.baseline_frontier.head_edit_ordinal = ordinal;
        public.baseline_frontier.head_edit_id = format!("edit-{ordinal}");
        public.baseline_frontier.last_commit_seq = ordinal;
        public.baseline_frontier.chain_hash = ArtifactHash(Sha256::digest(&ordinal.to_be_bytes()));
        public.published_at_ms = ordinal;
        materialized_checkpoint(public, pair)
    }

    fn generic_payload_pages(bytes: &[u8]) -> db::db_storage::DbIoPages {
        let mut writer = DbIoPageWriter::try_reserve(bytes.len().div_ceil(DB_IO_PAGE_BYTES)).expect("generic payload pages");
        for fragment in bytes.chunks(DB_IO_PAGE_BYTES) {
            assert_eq!(writer.write_fragment(fragment).expect("generic payload fragment"), fragment.len());
        }
        loop {
            if let Some(pages) = writer.seal_retained_step().expect("generic payload pages seal") {
                return pages;
            }
        }
    }

    async fn stage_reserved_checkpoint<S: ArtifactChunkCasStorage>(
        service: &DirectoryService,
        storage: Arc<S>,
        actor: DirectoryActor,
        checkpoint: &ArtifactCheckpoint,
        pair: &ArtifactPair,
        expires_at_ms: u64,
        now_ms: u64,
        context: &OperationContext<'_>,
    ) -> DirectoryResult<ArtifactCasReservation> {
        let plan = prepare_artifact_cas_ownership_v1(checkpoint, pair).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let reservation = service.reserve_artifact_cas(actor, plan, expires_at_ms, now_ms).await?;
        let coordinator_id = *reservation.coordinator_id();
        storage.configure_coordinator(coordinator_id, context).await.map_err(|error| DirectoryError::Backend(error.to_string()))?;
        storage.advance_physical_epoch(coordinator_id, &checkpoint.scope.space_id, reservation.physical_epoch(), context).await.map_err(|error| DirectoryError::Backend(error.to_string()))?;
        let blobs = ArtifactChunkBlobStore::new(storage);
        let pack = blobs.stage(&checkpoint.scope.space_id, ArtifactBlobIntegrity { sha256: checkpoint.pack.sha256, byte_length: checkpoint.pack.byte_length }, &pair.pack, context).await.map_err(|error| DirectoryError::Backend(error.to_string()))?;
        let spr = blobs.stage(&checkpoint.scope.space_id, ArtifactBlobIntegrity { sha256: checkpoint.spr.sha256, byte_length: checkpoint.spr.byte_length }, &pair.spr, context).await.map_err(|error| DirectoryError::Backend(error.to_string()))?;
        if pack.storage_key != checkpoint.pack.storage_key || spr.storage_key != checkpoint.spr.storage_key {
            return Err(DirectoryError::Conflict("artifact CAS pre-write plan changed while staging".into()));
        }
        Ok(reservation)
    }

    fn sweep_convergence_plan(index: usize, object_count: usize) -> ArtifactCasOwnershipPlanV1 {
        let digest = |kind: &str, object: usize| ArtifactHash(Sha256::digest(format!("sweep-continuation:{index}:{kind}:{object}").as_bytes()));
        let pack_manifest_id = digest("pack-manifest", 0);
        let spr_manifest_id = digest("spr-manifest", 0);
        let mut objects: Vec<_> = (0..object_count.saturating_sub(2)).map(|object| ArtifactCasObjectKey { space_id: "sweep-space".into(), kind: ArtifactCasObjectKind::Chunk, digest: digest("chunk", object) }).collect();
        objects.push(ArtifactCasObjectKey { space_id: "sweep-space".into(), kind: ArtifactCasObjectKind::Manifest, digest: pack_manifest_id });
        objects.push(ArtifactCasObjectKey { space_id: "sweep-space".into(), kind: ArtifactCasObjectKind::Manifest, digest: spr_manifest_id });
        objects.sort_by_key(|object| (object.kind, object.digest.0));
        ArtifactCasOwnershipPlanV1 { scope: DocumentScope::new("sweep-space", format!("sweep-document-{index}")), checkpoint_id: digest("checkpoint", 0), pack_manifest_id, spr_manifest_id, objects }
    }

    fn artifact_event(seq: u64, body: DirectoryEventBody) -> DirectoryEvent {
        DirectoryEvent {
            seq,
            id: format!("event-{seq}"),
            hlc: Hlc { physical_ms: seq as i64, logical: 0 },
            actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() },
            space_id: Some("raum:ä".into()),
            user_id: None,
            body,
            recorded_at_ms: seq as i64,
        }
    }

    struct RebuildProbe {
        cancelled: AtomicBool,
        cancel_after_first: bool,
        progress: std::sync::Mutex<Vec<ProjectionRebuildProgress>>,
    }

    impl ProjectionRebuildControl for RebuildProbe {
        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::SeqCst)
        }

        fn report(&self, progress: ProjectionRebuildProgress) {
            self.progress.lock().expect("progress").push(progress);
            if self.cancel_after_first && progress.completed_events == 1 {
                self.cancelled.store(true, Ordering::SeqCst);
            }
        }
    }

    // 🌱️ Every test in this module creates a space owned by `user_actor("u-owner")`; `decide`'s
    // `CreateSpace` arm (correctly) never mints the owner's `hub_user` row itself — it only has an
    // actor id, no email, so it cannot self-heal a missing user the way `UpsertMember` can. In
    // production the owner's `hub_user` row must predate `create-space`; the trusted identity
    // completion boundary provisions it before issuing a session. This fixture reproduces that precondition by
    // appending a bare `user.created` event for `"u-owner"` under a `System` actor, mirroring
    // `SqliteDirectory::seed`'s own pattern one file over.
    async fn fresh_dir() -> Arc<HubDirectories> {
        let dir = SqliteDirectory::connect(":memory:").await.expect("connect");
        let seed_actor = DirectoryActor { kind: DirectoryActorKind::System, id: "system:test-seed".into() };
        let mut clock = HubClock::new();
        let events = vec![new_event(&mut clock, &seed_actor, None, Some("u-owner".into()), DirectoryEventBody::UserCreated { user_id: "u-owner".into(), email: "u-owner@example.com".into(), display_name: "Owner".into() })];
        dir.append_events(&events).await.expect("seed owner user");
        Arc::new(HubDirectories::from(dir))
    }

    fn admin_audit_fact(request_id: &str, digest_byte: &str, phase: &str, outcome: &str) -> NewAdminOperationAuditRecord {
        NewAdminOperationAuditRecord {
            request_id: request_id.into(),
            intent_digest: digest_byte.repeat(64),
            operation_id: format!("operation:{request_id}"),
            occurred_at: 1,
            phase: phase.into(),
            intent_kind: "delete-space".into(),
            target_kind: "space".into(),
            target_id: "space:one".into(),
            principal_user_id: "user:admin".into(),
            principal_session_id: "session:admin".into(),
            principal_generation: 7,
            correlation_id: "correlation:admin".into(),
            event_seq_first: None,
            event_seq_last: None,
            outcome_code: outcome.into(),
            reason_code: None,
        }
    }

    #[tokio::test]
    async fn admin_operation_audit_concurrent_first_writer_is_idempotent_and_first_terminal_wins() {
        let directory = fresh_dir().await;
        let accepted = admin_audit_fact("request:race", "1", "accepted", "accepted");
        let barrier = Arc::new(tokio::sync::Barrier::new(33));
        let mut writers = Vec::new();
        for _ in 0..32 {
            let directory = directory.clone();
            let accepted = accepted.clone();
            let barrier = barrier.clone();
            writers.push(tokio::spawn(async move {
                barrier.wait().await;
                directory.append_admin_operation_audit(&accepted).await
            }));
        }
        barrier.wait().await;
        let mut sequences = BTreeSet::new();
        for writer in writers {
            sequences.insert(writer.await.expect("race writer").expect("idempotent append").sequence);
        }
        assert_eq!(sequences.len(), 1);
        assert_eq!(directory.admin_operation_audit_for_request("request:race").await.expect("race audit").len(), 1);
        let collision = admin_audit_fact("request:race", "2", "accepted", "accepted");
        assert!(matches!(directory.append_admin_operation_audit(&collision).await, Err(DirectoryError::Conflict(_))));

        let cancelled = admin_audit_fact("request:race", "1", "cancelled", "operator-cancelled");
        let failed = admin_audit_fact("request:race", "1", "failed", "late-failure");
        assert_eq!(directory.append_admin_operation_audit(&cancelled).await.expect("cancel terminal").fact.phase, "cancelled");
        assert!(matches!(directory.append_admin_operation_audit(&failed).await, Err(DirectoryError::Conflict(_))), "a different later terminal cannot masquerade as the winning cancellation");
        assert_eq!(directory.append_admin_operation_audit(&cancelled).await.expect("idempotent winning terminal").fact.phase, "cancelled");
        let operation_rows = directory.admin_operation_audit_for_operation("operation:request:race").await.expect("operation-id status lookup");
        assert_eq!(operation_rows.len(), 2);
        assert!(operation_rows.iter().all(|row| row.fact.request_id == "request:race"));
        let rows = directory.list_admin_operation_audit(0, ADMIN_PAGE_MAX).await.expect("bounded audit page");
        assert_eq!(rows.len(), 2);
        assert!(directory.list_admin_operation_audit(0, ADMIN_PAGE_MAX + 1).await.is_err());
    }

    #[tokio::test]
    async fn admin_bounded_overview_space_and_document_projections_enforce_exact_page_boundary() {
        let directory = fresh_dir().await;
        let service = DirectoryService::new(directory.clone(), 64);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        for index in 0..=ADMIN_PAGE_MAX {
            service.execute(owner.clone(), DirectoryCommand::AnnounceDocument { descriptor: descriptor(&space_id, &format!("document:{index:03}")) }).await.expect("announce bounded-page document");
        }

        assert_eq!(directory.admin_overview_counts().await.expect("constant-space overview counts"), AdminDirectoryOverviewCounts { spaces: 1, users: 1, connections: 0 },);
        let spaces = directory.list_admin_space_summaries_page(None, 0, ADMIN_PAGE_FETCH_MAX).await.expect("bounded space summary page");
        assert_eq!(spaces.len(), 1);
        assert_eq!(spaces[0].member_count, 1);
        assert_eq!(spaces[0].document_count, u64::try_from(ADMIN_PAGE_FETCH_MAX).expect("page maximum fits u64"));
        assert_eq!(spaces[0].active_connections, 0);
        assert_eq!(directory.list_admin_space_summaries_page(Some(&space_id), 0, 1).await.expect("exact space summary").len(), 1);
        assert_eq!(directory.list_admin_space_members_page(&space_id, 0, ADMIN_PAGE_FETCH_MAX).await.expect("bounded member page").len(), 1);
        assert!(directory.list_admin_space_summaries_page(None, 0, 0).await.is_err());
        assert!(directory.list_admin_space_summaries_page(None, 0, ADMIN_PAGE_FETCH_MAX + 1).await.is_err());
        assert!(directory.list_admin_space_members_page(&space_id, 0, 0).await.is_err());
        assert!(directory.list_admin_space_members_page(&space_id, 0, ADMIN_PAGE_FETCH_MAX + 1).await.is_err());
        let first = directory.list_document_descriptors_page(None, 0, ADMIN_PAGE_FETCH_MAX).await.expect("page plus continuation probe");
        assert_eq!(first.len(), ADMIN_PAGE_FETCH_MAX);
        assert_eq!(first.first().expect("first descriptor").document_id, "document:000");
        assert_eq!(first.last().expect("continuation descriptor").document_id, "document:100");
        let continuation = directory.list_document_descriptors_page(Some(&space_id), ADMIN_PAGE_MAX, ADMIN_PAGE_FETCH_MAX).await.expect("scoped continuation page");
        assert_eq!(continuation.len(), 1);
        assert_eq!(continuation[0].document_id, "document:100");
        assert!(directory.list_document_descriptors_page(None, 0, 0).await.is_err());
        assert!(directory.list_document_descriptors_page(None, 0, ADMIN_PAGE_FETCH_MAX + 1).await.is_err());
    }

    async fn create_space(service: &DirectoryService, owner: &DirectoryActor, kind: DirectorySpaceKind) -> String {
        let (events, _) = service.execute(owner.clone(), DirectoryCommand::CreateSpace { name: "Space".into(), space_kind: kind, visibility: DirectorySpaceVisibility::Private }).await.expect("create-space");
        events[0].space_id.clone().expect("space id on space.created")
    }

    /// 📣️ A committed page cannot be overtaken on the live channel while it still owns the writer guard.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn directory_append_and_live_broadcast_share_one_writer_guard_and_projection_order() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/📣️ordered-append-broadcast-v1/🔣️.json")).expect("language-neutral ordered publication fixture");
        assert_eq!(fixture["schema"], "semio.hub.directory.ordered-append-broadcast/v1");
        assert_eq!(fixture["cases"].as_array().expect("ordered publication cases").len(), 4);

        let dir = fresh_dir().await;
        let service = Arc::new(DirectoryService::new(dir.clone(), 16));
        let owner = user_actor("u-owner");
        let space_id = create_space(service.as_ref(), &owner, DirectorySpaceKind::Studio).await;
        service
            .execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "ordered@example.com".into(), role: DirectorySpaceRole::Spectator })
            .await
            .expect("seed ordered member");
        let member_id = dir
            .list_members(&space_id)
            .await
            .expect("seeded member projection")
            .into_iter()
            .find(|(user, _)| user.email == "ordered@example.com")
            .expect("ordered member")
            .0
            .id;
        let since = dir.head_seq().await.expect("head before concurrent publication");
        let mut receiver = service.subscribe();
        let fence = service.arm_publication_test_fence();

        let first_service = service.clone();
        let first_owner = owner.clone();
        let first_space = space_id.clone();
        let first = tokio::spawn(async move {
            first_service
                .execute(first_owner, DirectoryCommand::UpsertMember { space_id: first_space, email: "ordered@example.com".into(), role: DirectorySpaceRole::Author })
                .await
                .expect("first concurrent member update")
                .0
        });
        tokio::time::timeout(std::time::Duration::from_secs(1), fence.reached.notified()).await.expect("first committed page reached publication fence");

        let second_service = service.clone();
        let second_owner = owner.clone();
        let second_space = space_id.clone();
        let second = tokio::spawn(async move {
            second_service
                .execute(second_owner, DirectoryCommand::UpsertMember { space_id: second_space, email: "ordered@example.com".into(), role: DirectorySpaceRole::Spectator })
                .await
                .expect("second concurrent member update")
                .0
        });
        assert!(tokio::time::timeout(std::time::Duration::from_millis(50), receiver.recv()).await.is_err(), "a later writer cannot append or broadcast while the committed first page is fenced");
        fence.release.notify_one();

        let first_events = first.await.expect("first writer task");
        let second_events = second.await.expect("second writer task");
        assert_eq!(first_events.len(), 1);
        assert_eq!(second_events.len(), 1);
        assert_eq!([first_events[0].seq, second_events[0].seq], [since + 1, since + 2]);
        let mut observed = Vec::new();
        for _ in 0..2 {
            match tokio::time::timeout(std::time::Duration::from_secs(1), receiver.recv()).await.expect("ordered live event deadline").expect("ordered live event") {
                DirectoryStreamMessage::Event { event } => observed.push(event.seq),
                _ => panic!("directory writer emitted a non-event message"),
            }
        }
        assert_eq!(observed, vec![since + 1, since + 2]);
        assert_eq!(dir.events_since(since, 2).await.expect("durable concurrent event page").iter().map(|event| event.seq).collect::<Vec<_>>(), observed);
        assert_eq!(
            dir.list_members(&space_id)
                .await
                .expect("final member projection")
                .into_iter()
                .find(|(user, _)| user.id == member_id)
                .expect("final ordered member")
                .1,
            SpaceRole::Spectator,
        );
    }

    // 🔬️ Replaying the whole log through `rebuild_projections` reproduces the exact same
    // projections a live command stream built, and `events_since(0)` is dense `1..=head_seq()`.
    #[tokio::test]
    async fn event_log_replay_matches_projections() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir.clone(), 64);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        service.execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "member@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("upsert-member");
        service.execute(owner.clone(), DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "Renamed".into() }).await.expect("rename-space");

        let head = dir.head_seq().await.expect("head seq");
        let seqs: Vec<u64> = dir.events_since(0, 100).await.expect("events since").iter().map(|event| event.seq).collect();
        assert_eq!(seqs, (1..=head).collect::<Vec<_>>(), "seq is dense 1..n");

        let spaces_before = dir.list_spaces(100, 0).await.expect("list spaces");
        let members_before = dir.list_members(&space_id).await.expect("list members");
        let users_before = dir.list_users(100, 0).await.expect("list users");

        let replayed = dir.rebuild_projections().await.expect("rebuild");
        assert_eq!(replayed, head);
        assert_eq!(dir.list_spaces(100, 0).await.expect("list spaces"), spaces_before);
        assert_eq!(dir.list_members(&space_id).await.expect("list members"), members_before);
        assert_eq!(dir.list_users(100, 0).await.expect("list users"), users_before);
    }

    #[tokio::test]
    async fn artifact_chunk_cas_retention_space_delete_and_dry_run_preserve_live_bytes() {
        let (template_descriptor, first_public, second_public, _, _) = artifact_projection_fixture();
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir.clone(), 64);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        let mut descriptor = template_descriptor;
        descriptor.space_id = space_id.clone();
        descriptor.document_id = "artifact-cas-retention".into();
        service.execute(owner.clone(), DirectoryCommand::AnnounceDocument { descriptor: descriptor.clone() }).await.expect("announce CAS document");
        let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
        let first_pair = ArtifactPair { pack: b"shared-pack".to_vec(), spr: b"old-spr".to_vec() };
        let first = scoped_materialized_checkpoint(first_public, &descriptor, &first_pair, None, 1);
        let second_pair = ArtifactPair { pack: first_pair.pack.clone(), spr: b"new-spr".to_vec() };
        let second = scoped_materialized_checkpoint(second_public, &descriptor, &second_pair, Some(first.checkpoint_id), 2);
        let generic_pool = Arc::new(db::semio_framework_async::process_worker_pool(db::semio_framework_async::WorkerPoolConfig::new(db::semio_framework_async::ProcessKind::HeadlessBatch, 2)));
        let generic = GenericMemoryStorage::new(generic_pool).await.expect("open generic payload storage");
        let generic_hash = generic.put(generic_payload_pages(&first_pair.spr)).await.expect("seed identical generic payload bytes");
        let storage = Arc::new(MemoryArtifactChunkCasStorage::default());
        let control = ArtifactCasProbe::new(100, None);
        let context = OperationContext::new(10_000, AuthorityLimits::maximum(), &control);

        let first_reservation = stage_reserved_checkpoint(&service, storage.clone(), system.clone(), &first, &first_pair, 1_000, 100, &context).await.expect("reserve and stage first");
        service.publish_reserved_artifact_checkpoint(system.clone(), first.clone(), first_reservation, 100).await.expect("publish first");
        let second_reservation = stage_reserved_checkpoint(&service, storage.clone(), system.clone(), &second, &second_pair, 1_001, 101, &context).await.expect("reserve and stage second");
        service.publish_reserved_artifact_checkpoint(system.clone(), second.clone(), second_reservation, 101).await.expect("publish second");
        let retention = ArtifactRetention { scope: second.scope.clone(), retained_checkpoint_id: second.checkpoint_id, retained_floor: second.baseline_frontier.clone(), checkpoint_lineage_head: second.checkpoint_id };
        service.execute_artifact_authority(system.clone(), ArtifactDirectoryCommand::AdvanceRetention { retention }).await.expect("advance retention");
        assert_eq!(dir.get_verified_artifact_checkpoint(&first.scope, first.checkpoint_id).await.expect("released private checkpoint"), None);
        assert_eq!(dir.get_verified_artifact_checkpoint(&second.scope, second.checkpoint_id).await.expect("live private checkpoint"), Some(second.clone()));
        assert!(matches!(service.reserve_artifact_cas(system.clone(), prepare_artifact_cas_ownership_v1(&first, &first_pair).expect("first ownership"), 2_000, 200).await, Err(DirectoryError::Conflict(_))));

        let old_spr = StagedArtifactBlob { storage_key: first.spr.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: first.spr.sha256, byte_length: first.spr.byte_length } };
        let live_spr = StagedArtifactBlob { storage_key: second.spr.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: second.spr.sha256, byte_length: second.spr.byte_length } };
        let blobs = ArtifactChunkBlobStore::new(storage.clone());
        let dry = service.sweep_artifact_cas(storage.as_ref(), ArtifactCasSweepRequest { execute: false, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &context).await.expect("dry sweep");
        assert_eq!(dry.observed_generation, dry.final_generation);
        assert!(dry.eligible_objects >= 2);
        assert!(dry.protected_objects >= 2);
        assert_eq!(dry.deleted_objects, 0);
        assert_eq!(blobs.read(&space_id, &old_spr, &context).await.expect("dry run preserves released bytes"), first_pair.spr);

        let swept = service.sweep_artifact_cas(storage.as_ref(), ArtifactCasSweepRequest { execute: true, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &context).await.expect("retention sweep");
        assert_eq!(swept.deleted_objects, swept.eligible_objects);
        assert!(blobs.read(&space_id, &old_spr, &context).await.is_err());
        assert_eq!(blobs.read(&space_id, &live_spr, &context).await.expect("retained checkpoint survives"), second_pair.spr);

        service.execute(owner, DirectoryCommand::DeleteSpace { space_id: space_id.clone() }).await.expect("delete space");
        assert_eq!(dir.get_active_artifact_checkpoint(&second.scope).await.expect("deleted active checkpoint"), None);
        let deleted = service.sweep_artifact_cas(storage.as_ref(), ArtifactCasSweepRequest { execute: true, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &context).await.expect("space deletion sweep");
        assert!(deleted.deleted_objects >= 2);
        assert!(deleted.missing_objects >= 2);
        assert!(blobs.read(&space_id, &live_spr, &context).await.is_err());
        assert!(generic.contains(&generic_hash).await.expect("generic payload survives dedicated CAS sweep"));
    }

    #[tokio::test]
    async fn artifact_chunk_cas_expiry_supersedes_tokens_and_sweep_cancellation_commits_one_delete() {
        let (template_descriptor, public, _, _, _) = artifact_projection_fixture();
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir, 64);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        let mut descriptor = template_descriptor;
        descriptor.space_id = space_id;
        descriptor.document_id = "artifact-cas-expiry".into();
        service.execute(owner, DirectoryCommand::AnnounceDocument { descriptor: descriptor.clone() }).await.expect("announce expiry document");
        let pair = ArtifactPair { pack: b"expired-pack".to_vec(), spr: b"expired-spr".to_vec() };
        let checkpoint = scoped_materialized_checkpoint(public, &descriptor, &pair, None, 1);
        let plan = prepare_artifact_cas_ownership_v1(&checkpoint, &pair).expect("ownership");
        let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
        let expired = service.reserve_artifact_cas(system.clone(), plan.clone(), 200, 100).await.expect("initial reservation");
        let preview_storage = MemoryArtifactChunkCasStorage::default();
        let preview_control = ArtifactCasProbe::new(201, None);
        let preview_context = OperationContext::new(10_000, AuthorityLimits::maximum(), &preview_control);
        let preview = service.sweep_artifact_cas(&preview_storage, ArtifactCasSweepRequest { execute: false, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &preview_context).await.expect("mutation-free expiry preview");
        assert!(preview.eligible_objects > 0);
        let replacement = service.reserve_artifact_cas(system.clone(), plan.clone(), 400, 201).await.expect("replacement reservation");
        assert!(replacement.write_epoch > expired.write_epoch);
        assert_eq!(replacement.physical_epoch(), expired.physical_epoch() + 1, "dry-run does not consume a fence epoch");
        assert!(matches!(service.publish_reserved_artifact_checkpoint(system.clone(), checkpoint.clone(), expired, 201).await, Err(DirectoryError::Conflict(_))));
        let storage = Arc::new(MemoryArtifactChunkCasStorage::default());
        let stage_control = ArtifactCasProbe::new(201, None);
        let stage_context = OperationContext::new(10_000, AuthorityLimits::maximum(), &stage_control);
        let blobs = ArtifactChunkBlobStore::new(storage.clone());
        blobs.stage(&checkpoint.scope.space_id, ArtifactBlobIntegrity { sha256: checkpoint.pack.sha256, byte_length: checkpoint.pack.byte_length }, &pair.pack, &stage_context).await.expect("stage expired pack");
        blobs.stage(&checkpoint.scope.space_id, ArtifactBlobIntegrity { sha256: checkpoint.spr.sha256, byte_length: checkpoint.spr.byte_length }, &pair.spr, &stage_context).await.expect("stage expired SPR");
        let protected = service.sweep_artifact_cas(storage.as_ref(), ArtifactCasSweepRequest { execute: false, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &stage_context).await.expect("replacement protects bytes");
        assert_eq!(protected.eligible_objects, 0);
        assert_eq!(protected.protected_objects, plan.objects.len() as u64);
        assert!(matches!(service.publish_reserved_artifact_checkpoint(system, checkpoint, replacement, 400).await, Err(DirectoryError::Conflict(_))));

        stage_control.now_ms.store(401, Ordering::SeqCst);
        let cancel = ArtifactCasProbe::new(401, Some(1));
        let cancel_context = OperationContext::new(10_000, AuthorityLimits::maximum(), &cancel);
        assert!(matches!(
            service.sweep_artifact_cas(storage.as_ref(), ArtifactCasSweepRequest { execute: true, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &cancel_context).await,
            Err(crate::artifact_authority::AuthorityError::Cancelled)
        ));
        let progress = cancel.progress.lock().expect("cancel progress");
        let swept: Vec<_> = progress.iter().filter(|item| item.stage == AuthorityProgressStage::CasSweep).copied().collect();
        assert_eq!(swept.len(), 1);
        assert_eq!(swept[0].completed_units, 1);
        drop(progress);
        let mut live_objects = 0usize;
        for key in &plan.objects {
            live_objects += usize::from(storage.get(key, &stage_context).await.is_ok());
        }
        assert_eq!(live_objects + 1, plan.objects.len());
    }

    #[tokio::test]
    async fn artifact_chunk_cas_opaque_continuation_converges_after_page_overflow_cancel_and_resume() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🗿️artifact-authority/🧪️fixtures/🧱️artifact-chunk-cas/🔣️.json")).expect("artifact CAS fixture");
        let law = &fixture["sweepContinuation"];
        let object_counts = law["planObjectCounts"].as_array().expect("plan object counts");
        let total_objects = law["totalObjects"].as_u64().expect("total objects");
        let maximum = usize::try_from(law["requestMaximumObjects"].as_u64().expect("request maximum")).expect("bounded request maximum");
        assert_eq!(law["tokenPayloadBytes"].as_u64(), Some(ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES as u64));
        assert_eq!(law["tokenAuthenticationBytes"].as_u64(), Some((ARTIFACT_CAS_SWEEP_CONTINUATION_BYTES - ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES) as u64));
        assert_eq!(law["cursorExposesObjectIdentity"].as_bool(), Some(false));
        assert_eq!(law["invalidAfterGenerationChange"].as_bool(), Some(true));
        assert_eq!(law["invalidAfterRestart"].as_bool(), Some(true));
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir.clone(), 64);
        let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
        for (index, count) in object_counts.iter().enumerate() {
            let count = usize::try_from(count.as_u64().expect("plan object count")).expect("bounded plan object count");
            service.reserve_artifact_cas(system.clone(), sweep_convergence_plan(index, count), 200, 100).await.expect("reserve convergence plan");
        }
        assert_eq!(dir.artifact_cas_ledger_generation().await.expect("sweep generation"), law["ledgerGeneration"].as_u64().expect("fixture generation"));
        let storage = MemoryArtifactChunkCasStorage::default();
        let probe = ArtifactCasProbe::new(201, None);
        let context = OperationContext::new(10_000, AuthorityLimits::maximum(), &probe);
        assert!(matches!(
            service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum + 1, continuation: None }, &context).await,
            Err(crate::artifact_authority::AuthorityError::ResourceLimit("artifact CAS sweep object"))
        ));
        let first = service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: None }, &context).await.expect("first bounded sweep");
        let continuation = first.continuation.expect("first sweep continuation");
        assert_eq!(first.examined_objects, law["expectedExaminedPerRequest"][0].as_u64().expect("first examined"));
        assert_eq!(format!("{continuation:?}"), "ArtifactCasSweepContinuation(<opaque>)");
        let first_position = service.artifact_cas_sweep_position(continuation, true).expect("decode owned continuation");
        assert_eq!(first_position.observed_generation, law["expectedFirstCursor"]["observedGeneration"].as_u64().expect("cursor generation"));
        assert_eq!(first_position.after_generation, law["expectedFirstCursor"]["afterGeneration"].as_u64().expect("cursor page"));
        assert_eq!(first_position.object_offset as u64, law["expectedFirstCursor"]["objectOffset"].as_u64().expect("cursor offset"));
        assert!(matches!(service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: false, max_objects: maximum, continuation: Some(continuation) }, &context).await, Err(crate::artifact_authority::AuthorityError::Store(_))));

        let restarted = DirectoryService::new(dir.clone(), 64);
        assert!(matches!(restarted.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: Some(continuation) }, &context).await, Err(crate::artifact_authority::AuthorityError::Store(_))));
        let cancel_probe = ArtifactCasProbe::new(201, Some(1));
        let cancel_context = OperationContext::new(10_000, AuthorityLimits::maximum(), &cancel_probe);
        assert!(matches!(service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: Some(continuation) }, &cancel_context).await, Err(crate::artifact_authority::AuthorityError::Cancelled)));
        assert_eq!(cancel_probe.progress.lock().expect("cancel progress").iter().filter(|progress| progress.stage == AuthorityProgressStage::CasSweep).count(), 1);
        let second = service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: Some(continuation) }, &context).await.expect("resume bounded sweep");
        assert_eq!(second.examined_objects, law["expectedExaminedPerRequest"][1].as_u64().expect("second examined"));
        assert!(second.continuation.is_none());
        assert_eq!(first.examined_objects + second.examined_objects, total_objects);

        let changed = service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: 1, continuation: None }, &context).await.expect("generation-bound continuation").continuation.expect("generation-bound token");
        service.reserve_artifact_cas(system, sweep_convergence_plan(object_counts.len(), 2), 500, 300).await.expect("advance sweep generation");
        assert!(matches!(service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: Some(changed) }, &context).await, Err(crate::artifact_authority::AuthorityError::Store(_))));
    }

    #[tokio::test]
    async fn artifact_chunk_cas_failed_epoch_advance_releases_directory_lease() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir, 64);
        let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
        let plan = sweep_convergence_plan(9_999, 2);
        service.reserve_artifact_cas(system.clone(), plan.clone(), 200, 100).await.expect("expired cleanup reservation");
        let storage = FailingAdvanceArtifactCas(MemoryArtifactChunkCasStorage::default());
        let probe = ArtifactCasProbe::new(201, None);
        let context = OperationContext::new(10_000, AuthorityLimits::maximum(), &probe);
        assert!(matches!(service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: 1, continuation: None }, &context).await, Err(crate::artifact_authority::AuthorityError::Cancelled)));
        service.reserve_artifact_cas(system, plan, 400, 201).await.expect("failure cleanup releases live lease immediately");
    }

    #[tokio::test]
    async fn artifact_chunk_cas_two_service_sweep_and_reservation_race_is_serialized_before_rewrite() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🗿️artifact-authority/🧪️fixtures/🧱️artifact-chunk-cas/🔣️.json")).expect("artifact CAS fixture");
        let barrier = &fixture["deleteBarrier"];
        assert_eq!(barrier["leaseMaximumMs"].as_u64(), Some(ARTIFACT_CAS_DELETE_LEASE_TTL_MS));
        assert_eq!(barrier["dryRunAdvancesEpoch"].as_bool(), Some(false));
        assert_eq!(barrier["orders"][1]["oldDeleteOutcome"].as_str(), Some("stale-fence-rejected"));
        assert_eq!(barrier["orders"][1]["publishedReadOutcome"].as_str(), Some("exact"));
        let (mut orphan_descriptor, orphan_public, live_public, _, _) = artifact_projection_fixture();
        let dir = fresh_dir().await;
        let service = Arc::new(DirectoryService::new(dir.clone(), 64));
        let owner = user_actor("u-owner");
        let space_id = create_space(service.as_ref(), &owner, DirectorySpaceKind::Studio).await;
        orphan_descriptor.space_id = space_id.clone();
        orphan_descriptor.document_id = "artifact-cas-race-orphan".into();
        let mut live_descriptor = orphan_descriptor.clone();
        live_descriptor.document_id = "artifact-cas-race-live".into();
        service.execute(owner.clone(), DirectoryCommand::AnnounceDocument { descriptor: orphan_descriptor.clone() }).await.expect("announce orphan document");
        service.execute(owner, DirectoryCommand::AnnounceDocument { descriptor: live_descriptor.clone() }).await.expect("announce live document");
        let pair = ArtifactPair { pack: b"race-pack".to_vec(), spr: b"race-spr".to_vec() };
        let orphan = scoped_materialized_checkpoint(orphan_public, &orphan_descriptor, &pair, None, 1);
        let live = scoped_materialized_checkpoint(live_public, &live_descriptor, &pair, None, 1);
        let storage = Arc::new(BlockingDeleteArtifactCas::new());
        let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
        let stage_control = ArtifactCasProbe::new(100, None);
        let stage_context = OperationContext::new(10_000, AuthorityLimits::maximum(), &stage_control);
        stage_reserved_checkpoint(service.as_ref(), storage.clone(), system.clone(), &orphan, &pair, 200, 100, &stage_context).await.expect("stage orphan");
        let live_plan = prepare_artifact_cas_ownership_v1(&live, &pair).expect("live ownership");

        let sweep_service = Arc::new(DirectoryService::new(dir.clone(), 64));
        let sweep_storage = storage.clone();
        let sweep_task = tokio::spawn(async move {
            let control = ArtifactCasProbe::new(300, None);
            let context = OperationContext::new(10_000, AuthorityLimits::maximum(), &control);
            sweep_service.sweep_artifact_cas(sweep_storage.as_ref(), ArtifactCasSweepRequest { execute: true, max_objects: 1, continuation: None }, &context).await
        });
        storage.entered.notified().await;
        let reserve_service = DirectoryService::new(dir, 64);
        assert!(matches!(reserve_service.reserve_artifact_cas(system.clone(), live_plan.clone(), 1_000, 300).await, Err(DirectoryError::Conflict(_))));
        let reservation = reserve_service.reserve_artifact_cas(system.clone(), live_plan, 10_000, 5_301).await.expect("reserve after deletion lease expiry");
        let rewrite_control = ArtifactCasProbe::new(5_301, None);
        let rewrite_context = OperationContext::new(20_000, AuthorityLimits::maximum(), &rewrite_control);
        storage.advance_physical_epoch(*reservation.coordinator_id(), &space_id, reservation.physical_epoch(), &rewrite_context).await.expect("advance raced reservation epoch before stage");
        let blobs = ArtifactChunkBlobStore::new(storage.clone());
        let pack = blobs.stage(&space_id, ArtifactBlobIntegrity { sha256: live.pack.sha256, byte_length: live.pack.byte_length }, &pair.pack, &rewrite_context).await.expect("stage raced pack");
        let spr = blobs.stage(&space_id, ArtifactBlobIntegrity { sha256: live.spr.sha256, byte_length: live.spr.byte_length }, &pair.spr, &rewrite_context).await.expect("stage raced SPR");
        reserve_service.publish_reserved_artifact_checkpoint(system, live.clone(), reservation, 5_301).await.expect("publish raced checkpoint");
        storage.release.notify_one();
        assert!(sweep_task.await.expect("join sweep").is_err());
        assert_eq!(pack.storage_key, live.pack.storage_key);
        assert_eq!(spr.storage_key, live.spr.storage_key);
        assert_eq!(blobs.read(&space_id, &pack, &rewrite_context).await.expect("read raced pack"), pair.pack);
        assert_eq!(blobs.read(&space_id, &spr, &rewrite_context).await.expect("read raced SPR"), pair.spr);
    }

    #[tokio::test]
    async fn artifact_chunk_cas_filesystem_process_sweep_and_publication_race_preserves_exact_bytes() {
        const ROOT_ENV: &str = "SEMIO_ARTIFACT_CAS_DIRECTORY_PROCESS_RACE_ROOT";
        const MODE_ENV: &str = "SEMIO_ARTIFACT_CAS_DIRECTORY_PROCESS_RACE_MODE";
        if let (Ok(root), Ok(mode)) = (std::env::var(ROOT_ENV), std::env::var(MODE_ENV)) {
            let root = std::path::PathBuf::from(root);
            let path_text = root.join("directory.sqlite3").to_str().expect("UTF-8 process race path").to_string();
            let directory = SqliteDirectory::connect(&path_text).await.expect("child opens shared directory");
            let service = DirectoryService::new(Arc::new(HubDirectories::from(directory)), 16);
            let storage = FsArtifactChunkCasStorage::open(&root.join("artifact-cas").join("v1")).await.expect("child opens shared filesystem CAS");
            let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
            let (mut descriptor, _, live_public, _, _) = artifact_projection_fixture();
            descriptor.space_id = "default".into();
            descriptor.document_id = "artifact-cas-process-race-live".into();
            let pair = ArtifactPair { pack: b"process-race-pack".to_vec(), spr: b"process-race-spr".to_vec() };
            let live = scoped_materialized_checkpoint(live_public, &descriptor, &pair, None, 1);
            if mode == "old-sweep" {
                let storage = ProcessBlockingDeleteArtifactCas { inner: storage, entered: root.join("old-delete-entered"), release: root.join("old-delete-release") };
                let control = ArtifactCasProbe::new(300, None);
                let context = OperationContext::new(20_000, AuthorityLimits::maximum(), &control);
                let error = service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: 1, continuation: None }, &context).await.expect_err("old process deletion fence is stale");
                assert!(matches!(&error, crate::artifact_authority::AuthorityError::Store(message) if message == "artifact CAS deletion fence is stale"), "unexpected old process sweep error: {error:?}");
            } else {
                assert_eq!(mode, "successor-publication");
                let storage = Arc::new(storage);
                let control = ArtifactCasProbe::new(5_301, None);
                let context = OperationContext::new(20_000, AuthorityLimits::maximum(), &control);
                let reservation = stage_reserved_checkpoint(&service, storage.clone(), system.clone(), &live, &pair, 10_000, 5_301, &context).await.expect("successor reserves, advances, and stages");
                service.publish_reserved_artifact_checkpoint(system, live.clone(), reservation, 5_301).await.expect("successor publishes");
                let blobs = ArtifactChunkBlobStore::new(storage);
                let pack = StagedArtifactBlob { storage_key: live.pack.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: live.pack.sha256, byte_length: live.pack.byte_length } };
                let spr = StagedArtifactBlob { storage_key: live.spr.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: live.spr.sha256, byte_length: live.spr.byte_length } };
                assert_eq!(blobs.read("default", &pack, &context).await.expect("successor pack read"), pair.pack);
                assert_eq!(blobs.read("default", &spr, &context).await.expect("successor SPR read"), pair.spr);
            }
            return;
        }

        let root = std::env::temp_dir().join(format!("semio-artifact-cas-directory-process-race-{}", directory::os_identity::time_ordered_id()));
        std::fs::create_dir_all(&root).expect("create process race root");
        let path_text = root.join("directory.sqlite3").to_str().expect("UTF-8 process race path").to_string();
        let cas_root = root.join("artifact-cas").join("v1");
        let (mut orphan_descriptor, orphan_public, _, _, _) = artifact_projection_fixture();
        orphan_descriptor.space_id = "default".into();
        orphan_descriptor.document_id = "artifact-cas-process-race-orphan".into();
        let mut live_descriptor = orphan_descriptor.clone();
        live_descriptor.document_id = "artifact-cas-process-race-live".into();
        let pair = ArtifactPair { pack: b"process-race-pack".to_vec(), spr: b"process-race-spr".to_vec() };
        let orphan = scoped_materialized_checkpoint(orphan_public, &orphan_descriptor, &pair, None, 1);
        {
            let directory = SqliteDirectory::connect(&path_text).await.expect("open process race directory");
            directory.seed().await.expect("seed process race directory");
            let service = DirectoryService::new(Arc::new(HubDirectories::from(directory)), 16);
            service.execute(user_actor("seed"), DirectoryCommand::AnnounceDocument { descriptor: orphan_descriptor }).await.expect("announce process race orphan");
            service.execute(user_actor("seed"), DirectoryCommand::AnnounceDocument { descriptor: live_descriptor }).await.expect("announce process race live");
            let storage = Arc::new(FsArtifactChunkCasStorage::open(&cas_root).await.expect("open process race filesystem CAS"));
            let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
            let control = ArtifactCasProbe::new(100, None);
            let context = OperationContext::new(20_000, AuthorityLimits::maximum(), &control);
            stage_reserved_checkpoint(&service, storage, system, &orphan, &pair, 200, 100, &context).await.expect("stage expired process race orphan");
        }

        let executable = std::env::current_exe().expect("process race test executable");
        let spawn = |mode: &str| {
            std::process::Command::new(&executable)
                .arg("artifact_chunk_cas_filesystem_process_sweep_and_publication_race_preserves_exact_bytes")
                .arg("--test-threads=1")
                .env(ROOT_ENV, &root)
                .env(MODE_ENV, mode)
                .spawn()
                .expect("spawn process race child")
        };
        let mut old = spawn("old-sweep");
        let entered = root.join("old-delete-entered");
        for _ in 0..4_000 {
            if entered.exists() {
                break;
            }
            assert!(old.try_wait().expect("poll old sweep child").is_none(), "old sweep child exited before conditional deletion");
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
        assert!(entered.exists(), "old sweep reached conditional filesystem deletion");
        let mut successor = spawn("successor-publication");
        assert!(successor.wait().expect("wait successor publication child").success());
        std::fs::write(root.join("old-delete-release"), []).expect("release old process delete");
        assert!(old.wait().expect("wait old sweep child").success());

        let directory = SqliteDirectory::connect(&path_text).await.expect("reopen process race directory");
        let service = DirectoryService::new(Arc::new(HubDirectories::from(directory)), 16);
        let (mut live_descriptor, _, live_public, _, _) = artifact_projection_fixture();
        live_descriptor.space_id = "default".into();
        live_descriptor.document_id = "artifact-cas-process-race-live".into();
        let live = scoped_materialized_checkpoint(live_public, &live_descriptor, &pair, None, 1);
        assert_eq!(service.dir.get_verified_artifact_checkpoint(&live.scope, live.checkpoint_id).await.expect("published process race reference"), Some(live.clone()));
        let storage = Arc::new(FsArtifactChunkCasStorage::open(&cas_root).await.expect("reopen process race filesystem CAS"));
        let blobs = ArtifactChunkBlobStore::new(storage);
        let control = ArtifactCasProbe::new(5_302, None);
        let context = OperationContext::new(20_000, AuthorityLimits::maximum(), &control);
        let pack = StagedArtifactBlob { storage_key: live.pack.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: live.pack.sha256, byte_length: live.pack.byte_length } };
        let spr = StagedArtifactBlob { storage_key: live.spr.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: live.spr.sha256, byte_length: live.spr.byte_length } };
        assert_eq!(blobs.read("default", &pack, &context).await.expect("parent exact pack read"), pair.pack);
        assert_eq!(blobs.read("default", &spr, &context).await.expect("parent exact SPR read"), pair.spr);
        drop(service);
        std::fs::remove_dir_all(root).expect("remove process race root");
    }

    #[tokio::test]
    async fn artifact_checkpoint_publication_is_atomic_bounded_idempotent_and_replayable() {
        let (descriptor, first, second, first_retention, fixture_maximum) = artifact_projection_fixture();
        assert_eq!(fixture_maximum, ARTIFACT_CHECKPOINT_LINEAGE_MAX);
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir.clone(), 64);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        assert_ne!(space_id, descriptor.space_id);
        service.execute(owner.clone(), DirectoryCommand::CreateSpace { name: "Fixture".into(), space_kind: DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private }).await.expect("second space");
        let fixture_space = dir.list_spaces(100, 0).await.expect("spaces").into_iter().find(|space| space.name == "Fixture").expect("fixture space");
        let mut announced = descriptor.clone();
        announced.space_id = fixture_space.id.clone();
        let mut first = first;
        let mut second = second;
        let mut first_retention = first_retention;
        first.scope.space_id = fixture_space.id.clone();
        second.scope.space_id = fixture_space.id.clone();
        first_retention.scope.space_id = fixture_space.id.clone();
        let digest = descriptor_digest_v1(&announced).expect("digest");
        first.descriptor_digest_v1 = digest;
        second.descriptor_digest_v1 = digest;
        first.checkpoint_id = ArtifactHash(Sha256::digest(&crate::artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&first)).expect("first identity")));
        second.parent_checkpoint_id = Some(first.checkpoint_id);
        second.checkpoint_id = ArtifactHash(Sha256::digest(&crate::artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&second)).expect("second identity")));
        first_retention.retained_checkpoint_id = first.checkpoint_id;
        first_retention.checkpoint_lineage_head = second.checkpoint_id;
        service.execute(owner, DirectoryCommand::AnnounceDocument { descriptor: announced }).await.expect("announce");
        let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };

        let first_verified = verified_checkpoint(&first, "one");
        let second_verified = verified_checkpoint(&second, "two");
        let admin = DirectoryActor { kind: DirectoryActorKind::Admin, id: "admin:not-authority".into() };
        assert!(matches!(publish_reserved(&service, admin, first_verified.clone()).await, Err(DirectoryError::Unauthorized)));
        let first_events = publish_reserved(&service, system.clone(), first_verified.clone()).await.expect("publish first");
        assert_eq!(first_events.len(), 1);
        let public_json = serde_json::Value::from(&first_events[0].body.to_value()).to_string();
        assert!(!public_json.contains("storageKey"));
        assert!(!public_json.contains(&first_verified.pack.storage_key));
        assert!(!public_json.contains(&first_verified.spr.storage_key));
        let head = dir.head_seq().await.expect("head");
        let mut failed_publication_stream = service.subscribe();
        assert!(publish_reserved(&service, system.clone(), first_verified.clone()).await.expect("idempotent first").is_empty());
        assert_eq!(dir.head_seq().await.expect("head unchanged"), head);
        let mut private_conflict = first_verified.clone();
        private_conflict.pack.storage_key.push_str("-altered");
        assert!(matches!(publish_reserved(&service, system.clone(), private_conflict).await, Err(DirectoryError::Conflict(_))));
        let mut public_conflict = first_verified.clone();
        public_conflict.published_at_ms += 1;
        assert!(matches!(publish_reserved(&service, system.clone(), public_conflict).await, Err(DirectoryError::Conflict(_))));
        let forged_public =
            NewDirectoryEvent { hlc: Hlc { physical_ms: 1, logical: 0 }, actor: system.clone(), space_id: Some(first.scope.space_id.clone()), user_id: None, body: DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: first.clone() } };
        assert!(matches!(dir.append_events(&[forged_public]).await, Err(DirectoryError::Conflict(_))));
        assert_eq!(dir.head_seq().await.expect("append failure leaves head"), head);
        assert!(matches!(failed_publication_stream.try_recv(), Err(tokio::sync::broadcast::error::TryRecvError::Empty)));

        publish_reserved(&service, system.clone(), second_verified.clone()).await.expect("publish second");
        service.execute_artifact_authority(system.clone(), ArtifactDirectoryCommand::AdvanceRetention { retention: first_retention.clone() }).await.expect("retain first");
        assert!(service.execute_artifact_authority(system.clone(), ArtifactDirectoryCommand::AdvanceRetention { retention: first_retention.clone() }).await.expect("idempotent retention").is_empty());
        let second_retention = ArtifactRetention { scope: second.scope.clone(), retained_checkpoint_id: second.checkpoint_id, retained_floor: second.baseline_frontier.clone(), checkpoint_lineage_head: second.checkpoint_id };
        service.execute_artifact_authority(system.clone(), ArtifactDirectoryCommand::AdvanceRetention { retention: second_retention.clone() }).await.expect("retain second");
        assert!(matches!(service.execute_artifact_authority(system, ArtifactDirectoryCommand::AdvanceRetention { retention: first_retention }).await, Err(DirectoryError::Conflict(_))));

        let scope = second.scope.clone();
        assert_eq!(dir.get_active_artifact_checkpoint(&scope).await.expect("active"), Some(second.clone()));
        assert_eq!(dir.get_verified_artifact_checkpoint(&scope, first.checkpoint_id).await.expect("released private"), None);
        assert_eq!(dir.get_verified_artifact_checkpoint(&scope, second.checkpoint_id).await.expect("private active"), Some(second_verified.clone()));
        assert_eq!(dir.list_artifact_checkpoint_lineage(&scope, ARTIFACT_CHECKPOINT_LINEAGE_MAX as usize).await.expect("lineage"), vec![first.clone(), second.clone()]);
        assert_eq!(dir.get_artifact_retention(&scope).await.expect("retention"), Some(second_retention.clone()));
        assert!(matches!(dir.list_artifact_checkpoint_lineage(&scope, ARTIFACT_CHECKPOINT_LINEAGE_MAX as usize + 1).await, Err(DirectoryError::Conflict(_))));

        let before = (dir.get_active_artifact_checkpoint(&scope).await.expect("before active"), dir.get_artifact_retention(&scope).await.expect("before retention"));
        let cancel = RebuildProbe { cancelled: AtomicBool::new(false), cancel_after_first: true, progress: std::sync::Mutex::new(Vec::new()) };
        assert!(matches!(dir.rebuild_projections_controlled(&cancel).await, Err(DirectoryError::Conflict(_))));
        assert_eq!((dir.get_active_artifact_checkpoint(&scope).await.expect("rollback active"), dir.get_artifact_retention(&scope).await.expect("rollback retention")), before);
        let complete = RebuildProbe { cancelled: AtomicBool::new(false), cancel_after_first: false, progress: std::sync::Mutex::new(Vec::new()) };
        let replayed = dir.rebuild_projections_controlled(&complete).await.expect("controlled rebuild");
        let progress = complete.progress.lock().expect("progress");
        assert_eq!(progress.first().expect("initial").completed_events, 0);
        assert_eq!(progress.last().expect("final"), &ProjectionRebuildProgress { completed_events: replayed, total_events: replayed });
        assert_eq!(dir.get_active_artifact_checkpoint(&scope).await.expect("rebuilt active"), Some(second));
        assert_eq!(dir.get_verified_artifact_checkpoint(&scope, first.checkpoint_id).await.expect("rebuilt released private"), None);
        assert_eq!(dir.get_verified_artifact_checkpoint(&scope, second_verified.checkpoint_id).await.expect("rebuilt private"), Some(second_verified));
        assert_eq!(dir.get_artifact_retention(&scope).await.expect("rebuilt retention"), Some(second_retention));
    }

    #[test]
    fn memory_projection_is_atomic_and_fixed_caps_reject_max_plus_one() {
        let (descriptor, first, second, retention, fixture_maximum) = artifact_projection_fixture();
        let events = vec![
            artifact_event(1, DirectoryEventBody::DocumentAnnounced { descriptor }),
            artifact_event(2, DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: first.clone() }),
            artifact_event(3, DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: second.clone() }),
            artifact_event(4, DirectoryEventBody::ArtifactRetentionAdvanced { retention: retention.clone() }),
        ];
        let mut projection = MemoryArtifactProjection::default();
        projection.fold_atomically(&events).expect("memory fold");
        assert_eq!(projection.active_checkpoint(&second.scope), Some(&second));
        assert_eq!(projection.retention(&second.scope), Some(&retention));
        let before = projection.clone();
        let mut backward = retention;
        backward.checkpoint_lineage_head = first.checkpoint_id;
        assert!(projection.fold_atomically(&[artifact_event(5, DirectoryEventBody::ArtifactRetentionAdvanced { retention: backward })]).is_err());
        assert_eq!(projection, before);

        projection.checkpoints.insert(first.scope.clone(), vec![first.clone(); fixture_maximum as usize]);
        assert!(projection.fold_atomically(&[artifact_event(6, DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: second })]).is_err());
        let probe = RebuildProbe { cancelled: AtomicBool::new(false), cancel_after_first: false, progress: std::sync::Mutex::new(Vec::new()) };
        checkpoint_projection_rebuild(&probe, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS).expect("rebuild exact maximum");
        assert!(checkpoint_projection_rebuild(&probe, 0, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS + 1).is_err());
    }

    #[test]
    fn artifact_public_scalars_and_private_locators_obey_exact_max_plus_one_laws() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/📸️artifact-checkpoint-projection.json")).expect("checkpoint projection fixture");
        assert_eq!(fixture["wireIntegerMaximum"].as_u64(), Some(DIRECTORY_WIRE_INTEGER_MAX));
        assert_eq!(fixture["privateLocatorMaximumBytes"].as_u64(), Some(ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES as u64));
        assert_eq!(fixture["eventReadMaximum"].as_u64(), Some(DIRECTORY_EVENT_READ_MAX as u64));
        bounded_event_read(DIRECTORY_WIRE_INTEGER_MAX, DIRECTORY_EVENT_READ_MAX).expect("event read exact maxima");
        assert!(bounded_event_read(DIRECTORY_WIRE_INTEGER_MAX + 1, DIRECTORY_EVENT_READ_MAX).is_err());
        assert!(bounded_event_read(0, DIRECTORY_EVENT_READ_MAX + 1).is_err());
        let (_, first, _, retention, _) = artifact_projection_fixture();
        let mut public_max = first.clone();
        public_max.published_at_ms = DIRECTORY_WIRE_INTEGER_MAX;
        validate_checkpoint_shape(&public_max).expect("wire integer exact maximum");
        public_max.published_at_ms = DIRECTORY_WIRE_INTEGER_MAX + 1;
        assert!(validate_checkpoint_shape(&public_max).is_err());

        let mut frontier_max = first.clone();
        frontier_max.baseline_frontier.head_edit_ordinal = DIRECTORY_WIRE_INTEGER_MAX;
        frontier_max.baseline_frontier.last_commit_seq = DIRECTORY_WIRE_INTEGER_MAX;
        frontier_max.pack.byte_length = DIRECTORY_WIRE_INTEGER_MAX;
        frontier_max.spr.byte_length = DIRECTORY_WIRE_INTEGER_MAX;
        frontier_max.checkpoint_id = ArtifactHash(Sha256::digest(&crate::artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&frontier_max)).expect("max identity")));
        validate_checkpoint_shape(&frontier_max).expect("all exact wire maxima");
        frontier_max.baseline_frontier.head_edit_ordinal += 1;
        assert!(validate_checkpoint_shape(&frontier_max).is_err());

        let mut retention_max = retention;
        retention_max.retained_floor.head_edit_ordinal = DIRECTORY_WIRE_INTEGER_MAX;
        retention_max.retained_floor.last_commit_seq = DIRECTORY_WIRE_INTEGER_MAX;
        validate_retention_shape(&retention_max).expect("retention exact maximum");
        retention_max.retained_floor.last_commit_seq += 1;
        assert!(validate_retention_shape(&retention_max).is_err());

        let mut private = verified_checkpoint(&first, "limits");
        private.pack.storage_key = "p".repeat(ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES);
        private.spr.storage_key = "s".repeat(ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES);
        let event = NewDirectoryEvent {
            hlc: Hlc { physical_ms: 1, logical: 0 },
            actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() },
            space_id: Some(private.scope.space_id.clone()),
            user_id: None,
            body: DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: published_artifact_checkpoint(&private) },
        };
        validate_verified_checkpoint_append(&event, &private).expect("locator exact maximum");
        private.pack.storage_key.push('x');
        assert!(validate_verified_checkpoint_append(&event, &private).is_err());
    }

    #[tokio::test]
    async fn artifact_chunk_cas_sqlite_and_filesystem_restart_rebuild_restore_exact_authority() {
        let (mut descriptor, public, _, _, _) = artifact_projection_fixture();
        descriptor.space_id = "default".into();
        descriptor.document_id = "artifact-cas-restart".into();
        let pair = ArtifactPair { pack: b"restart-pack".to_vec(), spr: b"restart-spr".to_vec() };
        let verified = scoped_materialized_checkpoint(public, &descriptor, &pair, None, 1);
        let mut root = std::env::temp_dir();
        root.push(format!("semio-artifact-checkpoint-{}", directory::os_identity::time_ordered_id()));
        std::fs::create_dir_all(&root).expect("create test directory");
        let path = root.join("directory.sqlite3");
        let cas_root = root.join("artifact-cas").join("v1");
        let path_text = path.to_str().expect("utf8 test path").to_string();
        let control = ArtifactCasProbe::new(100, None);
        let context = OperationContext::new(10_000, AuthorityLimits::maximum(), &control);
        {
            let directory = SqliteDirectory::connect(&path_text).await.expect("connect");
            directory.seed().await.expect("seed");
            let directories = Arc::new(HubDirectories::from(directory));
            let service = DirectoryService::new(directories, 16);
            service.execute(user_actor("seed"), DirectoryCommand::AnnounceDocument { descriptor: descriptor.clone() }).await.expect("announce descriptor");
            let storage = Arc::new(FsArtifactChunkCasStorage::open(&cas_root).await.expect("open filesystem CAS"));
            let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
            let reservation = stage_reserved_checkpoint(&service, storage, system.clone(), &verified, &pair, 1_000, 100, &context).await.expect("reserve and stage restart fixture");
            service.publish_reserved_artifact_checkpoint(system, verified.clone(), reservation, 100).await.expect("publish verified checkpoint");
        }
        let reopened = SqliteDirectory::connect(&path_text).await.expect("reopen");
        assert_eq!(reopened.get_verified_artifact_checkpoint(&verified.scope, verified.checkpoint_id).await.expect("restart private"), Some(verified.clone()));
        let generation = reopened.artifact_cas_ledger_generation().await.expect("restart ledger generation");
        let storage = Arc::new(FsArtifactChunkCasStorage::open(&cas_root).await.expect("reopen filesystem CAS"));
        let blobs = ArtifactChunkBlobStore::new(storage);
        let pack = StagedArtifactBlob { storage_key: verified.pack.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: verified.pack.sha256, byte_length: verified.pack.byte_length } };
        assert_eq!(blobs.read("default", &pack, &context).await.expect("restart CAS read"), pair.pack);
        reopened.rebuild_projections().await.expect("rebuild");
        assert_eq!(reopened.artifact_cas_ledger_generation().await.expect("rebuilt ledger generation"), generation);
        assert_eq!(reopened.get_verified_artifact_checkpoint(&verified.scope, verified.checkpoint_id).await.expect("rebuilt private"), Some(verified.clone()));
        assert_eq!(blobs.read("default", &pack, &context).await.expect("rebuilt CAS read"), pair.pack);
        drop(reopened);
        std::fs::remove_dir_all(&root).expect("remove exact restart test directory");
    }

    #[tokio::test]
    async fn document_descriptor_is_immutable_space_scoped_and_survives_restart() {
        let mut root = std::env::temp_dir();
        root.push(format!("semio-document-descriptor-{}", directory::os_identity::time_ordered_id()));
        std::fs::create_dir_all(&root).expect("create test directory");
        let path = root.join("directory.sqlite3");
        let path_text = path.to_str().expect("utf8 test path").to_string();

        let persisted = descriptor("default", "shared-document");
        let mut zero_identity = persisted.clone();
        zero_identity.pack_schema_hash = "0".repeat(64);
        assert!(matches!(validate_document_descriptor(&zero_identity), Err(DirectoryError::Conflict(_))));
        {
            let directory = SqliteDirectory::connect(&path_text).await.expect("connect");
            directory.seed().await.expect("seed");
            let directories = Arc::new(HubDirectories::from(directory));
            let service = DirectoryService::new(directories.clone(), 16);
            let (events, _) = service.execute(user_actor("seed"), DirectoryCommand::AnnounceDocument { descriptor: persisted.clone() }).await.expect("announce");
            assert!(matches!(&events[0].body, DirectoryEventBody::DocumentAnnounced { descriptor } if descriptor == &persisted));

            let mut conflict = persisted.clone();
            conflict.pack_schema_hash = "44".repeat(32);
            assert!(matches!(service.execute(user_actor("seed"), DirectoryCommand::AnnounceDocument { descriptor: conflict }).await, Err(DirectoryError::Conflict(_))));

            let other = descriptor("other-space", "shared-document");
            let actor = user_actor("seed");
            let (created, _) = service.execute(actor.clone(), DirectoryCommand::CreateSpace { name: "Other".into(), space_kind: DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private }).await.expect("create other space");
            let other_space = created[0].space_id.clone().expect("space id");
            let mut other = other;
            other.space_id = other_space.clone();
            service.execute(actor, DirectoryCommand::AnnounceDocument { descriptor: other.clone() }).await.expect("announce same document id in other space");
            assert_eq!(directories.list_document_descriptors(&other_space).await.expect("other descriptors"), vec![other]);
        }

        let reopened = SqliteDirectory::connect(&path_text).await.expect("reopen");
        assert_eq!(reopened.get_document_descriptor(&DocumentScope::new("default", "shared-document")).await.expect("read after restart"), Some(persisted.clone()));
        let before = reopened.list_document_descriptors("default").await.expect("before rebuild");
        reopened.rebuild_projections().await.expect("rebuild");
        assert_eq!(reopened.list_document_descriptors("default").await.expect("after rebuild"), before);

        drop(reopened);
        for target in [&path, &root.join("directory.sqlite3-wal"), &root.join("directory.sqlite3-shm")] {
            if target.exists() {
                std::fs::remove_file(target).expect("remove exact sqlite test file");
            }
        }
        std::fs::remove_dir(&root).expect("remove empty test directory");
    }

    // 🔬️ Decider law: an atelier rejects a second, distinct author (re-upserting the sole existing
    // author is not exercised here — that is the sqlite backend's own membership round-trip test).
    #[tokio::test]
    async fn atelier_rejects_a_second_distinct_author() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir, 16);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Atelier).await;
        let err = service.execute(owner, DirectoryCommand::UpsertMember { space_id, email: "other@example.com".into(), role: DirectorySpaceRole::Author }).await.unwrap_err();
        assert!(matches!(err, DirectoryError::Conflict(_)));
    }

    // 🔬️ Decider law: `archive-space` first demotes every current author to spectator, then
    // archives — nobody is left an author of a frozen space.
    #[tokio::test]
    async fn archive_space_demotes_every_author_to_spectator() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir.clone(), 16);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        service.execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "second@example.com".into(), role: DirectorySpaceRole::Author }).await.expect("second author");
        service.execute(owner.clone(), DirectoryCommand::ArchiveSpace { space_id: space_id.clone() }).await.expect("archive-space");
        let members = dir.list_members(&space_id).await.expect("list members");
        assert_eq!(members.len(), 2);
        assert!(members.iter().all(|(_, role)| *role == SpaceRole::Spectator));
        assert_eq!(dir.get_space(&space_id).await.expect("get space").expect("space exists").kind, "archive");
    }

    // 🔬️ Decider law: the owner's membership can never be removed via `remove-member`.
    #[tokio::test]
    async fn owner_membership_can_never_be_removed() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir, 16);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        let err = service.execute(owner, DirectoryCommand::RemoveMember { space_id, user_id: "u-owner".into() }).await.unwrap_err();
        assert!(matches!(err, DirectoryError::Conflict(_)));
    }

    // 🔬️ Decider law: any command naming a deleted (or otherwise missing) space is `NotFound`.
    #[tokio::test]
    async fn command_naming_a_deleted_space_is_not_found() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir, 16);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        service.execute(owner.clone(), DirectoryCommand::DeleteSpace { space_id: space_id.clone() }).await.expect("delete-space");
        let err = service.execute(owner, DirectoryCommand::RenameSpace { space_id, name: "Nope".into() }).await.unwrap_err();
        assert!(matches!(err, DirectoryError::NotFound(_)));
    }

    // 🔬️ Decider law: `upsert-member` with an email that has no `UserRecord` yet emits
    // `user.created` before `member.upserted`, both under one decision.
    #[tokio::test]
    async fn upsert_member_with_unknown_email_creates_the_user_first() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir, 16);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        let (events, _) = service.execute(owner, DirectoryCommand::UpsertMember { space_id, email: "new@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("upsert-member");
        assert_eq!(events.len(), 2);
        assert!(matches!(events[0].body, DirectoryEventBody::UserCreated { .. }));
        assert!(matches!(events[1].body, DirectoryEventBody::MemberUpserted { .. }));
    }

    // 🔬️ Invite create -> redeem -> revoke round-trip: redemption grants membership and emits
    // `invite.redeemed`; a revoked invite still round-trips its `revoked_at`.
    #[tokio::test]
    async fn invite_create_redeem_revoke_round_trip() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir.clone(), 16);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;

        let (_, result) = service.execute(owner.clone(), DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 3600 }).await.expect("create-invite");
        let token = result.expect("command result").invite_token.expect("invite token");
        let capability = InviteCapability::parse(&token).expect("typed invite capability");
        let invited = dir.create_user("invited@example.com", "Invited", None, None, None).await.expect("create invited user");

        let redeemed = service.redeem_invite(user_actor(&invited.id), &capability, &invited.id).await.expect("redeem");
        assert!(matches!(redeemed.last().expect("at least one event").body, DirectoryEventBody::InviteRedeemed { .. }));
        let members = dir.list_members(&space_id).await.expect("list members");
        assert!(members.iter().any(|(user, role)| user.email == "invited@example.com" && *role == SpaceRole::Spectator));

        let invites = dir.list_invites(&space_id).await.expect("list invites");
        assert_eq!(invites.len(), 1);
        assert_eq!(invites[0].accepted_at, Some(redeemed[0].recorded_at_ms));
        assert_eq!(invites[0].accepted_event_id.as_deref(), Some(redeemed[0].id.as_str()));
        let retried = service.redeem_invite(user_actor(&invited.id), &capability, &invited.id).await.expect("idempotent same-user retry");
        assert_eq!(retried, redeemed);
        assert!(matches!(dir.revoke_invite(&invites[0].id, "test-revoke", "invite-round-trip").await, Err(DirectoryError::Conflict(message)) if message == "invite already accepted"));
    }

    /// 🎟️ Two independent service writers still produce one durable invitation claim across restart and rebuild.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn invite_redemption_sqlite_claim_is_exactly_once_across_concurrency_restart_and_rebuild() {
        let root = std::env::temp_dir().join(format!("semio-invite-claim-{}", time_ordered_id()));
        std::fs::create_dir(&root).expect("create invite test directory");
        let path = root.join("directory.sqlite3");
        let path_text = path.to_string_lossy().into_owned();
        let primary_backend = SqliteDirectory::connect(&path_text).await.expect("connect primary");
        primary_backend.seed().await.expect("seed");
        let primary = Arc::new(HubDirectories::from(primary_backend));
        let invited = primary.create_user("invite-race@example.com", "Invite Race", None, None, None).await.expect("create invited user");
        let issued = primary.issue_invite("default", SpaceRole::Spectator, 3600, "invite-race").await.expect("issue invite");
        let secondary = Arc::new(HubDirectories::from(SqliteDirectory::connect(&path_text).await.expect("connect secondary")));
        let services = [Arc::new(DirectoryService::new(primary.clone(), 16)), Arc::new(DirectoryService::new(secondary.clone(), 16))];
        let barrier = Arc::new(tokio::sync::Barrier::new(3));
        let mut claims = Vec::new();
        for service in services {
            let barrier = barrier.clone();
            let capability = issued.capability.clone();
            let user_id = invited.id.clone();
            claims.push(tokio::spawn(async move {
                barrier.wait().await;
                service.redeem_invite(user_actor(&user_id), &capability, &user_id).await
            }));
        }
        barrier.wait().await;
        let results = [claims.remove(0).await.expect("first claim task"), claims.remove(0).await.expect("second claim task")];
        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 2);
        let returned_ids: BTreeSet<_> = results.iter().map(|result| result.as_ref().expect("same-user claim result")[0].id.as_str()).collect();
        assert_eq!(returned_ids.len(), 1, "the second service returns the original immutable event");
        let events = primary.events_since(0, DIRECTORY_EVENT_PAGE_MAX).await.expect("durable events");
        let redeemed: Vec<_> = events.iter().filter(|event| matches!(event.body, DirectoryEventBody::InviteRedeemed { .. })).collect();
        assert_eq!(redeemed.len(), 1);
        assert_eq!(primary.list_invites("default").await.expect("claimed invite")[0].accepted_at, Some(redeemed[0].recorded_at_ms));
        assert_eq!(primary.list_members("default").await.expect("projected membership").iter().filter(|(user, _)| user.id == invited.id).count(), 1);

        let other = primary.create_user("invite-race-other@example.com", "Invite Race Other", None, None, None).await.expect("create competing user");
        let contested = primary.issue_invite("default", SpaceRole::Spectator, 3600, "invite-race-different-users").await.expect("issue contested invite");
        let barrier = Arc::new(tokio::sync::Barrier::new(3));
        let contenders = [(primary.clone(), invited.id.clone()), (secondary.clone(), other.id.clone())];
        let mut claims = Vec::new();
        for (directory, user_id) in contenders {
            let barrier = barrier.clone();
            let capability = contested.capability.clone();
            claims.push(tokio::spawn(async move {
                barrier.wait().await;
                DirectoryService::new(directory, 16).redeem_invite(user_actor(&user_id), &capability, &user_id).await
            }));
        }
        barrier.wait().await;
        let contested_results = [claims.remove(0).await.expect("first contested claim"), claims.remove(0).await.expect("second contested claim")];
        assert_eq!(contested_results.iter().filter(|result| result.is_ok()).count(), 1);
        assert_eq!(contested_results.iter().filter(|result| matches!(result, Err(DirectoryError::Conflict(message)) if message == "invite already accepted")).count(), 1);
        let contested_event = primary
            .events_since(0, DIRECTORY_EVENT_PAGE_MAX)
            .await
            .expect("contested durable events")
            .into_iter()
            .find(|event| matches!(&event.body, DirectoryEventBody::InviteRedeemed { invite_id, .. } if invite_id == &contested.record.id))
            .expect("one contested event");
        let contested_user = contested_event.user_id.clone().expect("contested event user");
        let contested_retry = DirectoryService::new(primary.clone(), 16)
            .redeem_invite(user_actor(&contested_user), &contested.capability, &contested_user)
            .await
            .expect("winning user idempotent retry");
        assert_eq!(contested_retry[0].id, contested_event.id);
        drop(primary);
        drop(secondary);

        let reopened_backend = SqliteDirectory::connect(&path_text).await.expect("reopen");
        let reopened = Arc::new(HubDirectories::from(reopened_backend));
        let reopened_service = DirectoryService::new(reopened.clone(), 16);
        let reopened_retry = reopened_service.redeem_invite(user_actor(&invited.id), &issued.capability, &invited.id).await.expect("restart same-user retry");
        assert_eq!(reopened_retry[0].id, redeemed[0].id);
        let reopened_contested = reopened_service.redeem_invite(user_actor(&contested_user), &contested.capability, &contested_user).await.expect("restart contested winner retry");
        assert_eq!(reopened_contested[0].id, contested_event.id);
        let before = reopened.events_since(0, DIRECTORY_EVENT_PAGE_MAX).await.expect("events before rebuild");
        reopened.rebuild_projections().await.expect("rebuild projections");
        let after = reopened.events_since(0, DIRECTORY_EVENT_PAGE_MAX).await.expect("events after rebuild");
        assert_eq!(before, after);
        assert_eq!(after.iter().filter(|event| matches!(event.body, DirectoryEventBody::InviteRedeemed { .. })).count(), 2);
        assert_eq!(reopened.list_members("default").await.expect("rebuilt membership").iter().filter(|(user, _)| user.id == invited.id).count(), 1);
        drop(reopened_service);
        drop(reopened);
        for target in [&path, &root.join("directory.sqlite3-wal"), &root.join("directory.sqlite3-shm")] {
            if target.exists() {
                std::fs::remove_file(target).expect("remove exact invite test file");
            }
        }
        std::fs::remove_dir(&root).expect("remove invite test directory");
    }

    /// 🎟️ A projection failure rolls back the accepted marker and event so the exact capability remains retryable.
    #[tokio::test]
    async fn invite_redemption_projection_failure_rolls_back_claim_event_and_membership() {
        let backend = SqliteDirectory::connect(":memory:").await.expect("connect");
        backend.seed().await.expect("seed");
        let mut clock = HubClock::new();
        backend
            .append_events(&[new_event(
                &mut clock,
                &DirectoryActor { kind: DirectoryActorKind::System, id: "system:invite-failure".into() },
                None,
                Some("u-invite-failure".into()),
                DirectoryEventBody::UserCreated { user_id: "u-invite-failure".into(), email: "invite-failure@example.com".into(), display_name: "Invite Failure".into() },
            )])
            .await
            .expect("seed failure user");
        let issued = backend.issue_invite("default", SpaceRole::Spectator, 3600, "invite-failure").await.expect("issue invite");
        let head_before_forgery = backend.head_seq().await.expect("head before forged redemption");
        let forged = new_event(
            &mut clock,
            &user_actor("u-invite-failure"),
            Some("default".into()),
            Some("u-invite-failure".into()),
            DirectoryEventBody::InviteRedeemed { space_id: "default".into(), user_id: "u-invite-failure".into(), invite_id: issued.record.id.clone(), role: DirectorySpaceRole::Spectator },
        );
        assert!(matches!(backend.append_events(&[forged]).await, Err(DirectoryError::Conflict(_))));
        assert_eq!(backend.head_seq().await.expect("head after forged redemption"), head_before_forgery);
        backend.install_invite_projection_failure().expect("install projection failure");
        let directory = Arc::new(HubDirectories::from(backend));
        let service = DirectoryService::new(directory.clone(), 16);
        assert!(matches!(service.redeem_invite(user_actor("u-invite-failure"), &issued.capability, "u-invite-failure").await, Err(DirectoryError::Backend(_))));
        assert_eq!(directory.list_invites("default").await.expect("invite after rollback")[0].accepted_at, None);
        assert_eq!(directory.events_since(0, DIRECTORY_EVENT_PAGE_MAX).await.expect("events after rollback").iter().filter(|event| matches!(event.body, DirectoryEventBody::InviteRedeemed { .. })).count(), 0);
        assert_eq!(directory.get_role("default", "u-invite-failure").await.expect("membership after rollback"), None);
        let HubDirectories::Sqlite(backend) = directory.as_ref() else { panic!("invite rollback law requires SQLite") };
        backend.clear_invite_projection_failure().expect("clear projection failure");
        let redeemed = service.redeem_invite(user_actor("u-invite-failure"), &issued.capability, "u-invite-failure").await.expect("retry exact invite");
        assert_eq!(redeemed.len(), 1);
        assert_eq!(directory.get_role("default", "u-invite-failure").await.expect("membership after retry"), Some(SpaceRole::Spectator));
        backend
            .lock()
            .expect("sqlite corruption fixture lock")
            .execute("UPDATE hub_space_invite SET accepted_event_id = 'missing-event' WHERE id = ?1", [&issued.record.id])
            .expect("install corrupt acceptance marker");
        assert!(matches!(service.redeem_invite(user_actor("u-invite-failure"), &issued.capability, "u-invite-failure").await, Err(DirectoryError::Backend(_))));
    }

    /// 📣️ A committed redemption is broadcast before a later command can enter the shared writer.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn invite_redemption_commit_and_publication_precede_the_next_directory_command() {
        let directory = fresh_dir().await;
        let service = Arc::new(DirectoryService::new(directory.clone(), 16));
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        let invited = directory.create_user("invite-order@example.com", "Invite Order", None, None, None).await.expect("create invited user");
        let issued = directory.issue_invite(&space_id, SpaceRole::Spectator, 3600, "invite-order").await.expect("issue invite");
        let mut receiver = service.subscribe();
        let fence = service.arm_publication_test_fence();
        let redeem_service = service.clone();
        let redeem_user = invited.id.clone();
        let capability = issued.capability.clone();
        let redeem = tokio::spawn(async move { redeem_service.redeem_invite(user_actor(&redeem_user), &capability, &redeem_user).await });
        fence.reached.notified().await;
        let rename_service = service.clone();
        let rename_actor = owner.clone();
        let rename_space = space_id.clone();
        let rename = tokio::spawn(async move { rename_service.execute(rename_actor, DirectoryCommand::RenameSpace { space_id: rename_space, name: "After Invite".into() }).await });
        tokio::task::yield_now().await;
        assert!(!rename.is_finished(), "later command remains excluded until redemption publication");
        fence.release.notify_one();
        let redeemed = redeem.await.expect("redeem task").expect("redeem");
        let renamed = rename.await.expect("rename task").expect("rename").0;
        assert!(redeemed[0].seq < renamed[0].seq);
        let first = receiver.recv().await.expect("redeem publication");
        let second = receiver.recv().await.expect("rename publication");
        assert!(matches!(first, DirectoryStreamMessage::Event { event } if event.seq == redeemed[0].seq));
        assert!(matches!(second, DirectoryStreamMessage::Event { event } if event.seq == renamed[0].seq));
        let retried = service.redeem_invite(user_actor(&invited.id), &issued.capability, &invited.id).await.expect("same-user retry");
        assert_eq!(retried, redeemed);
        assert!(tokio::time::timeout(std::time::Duration::from_millis(25), receiver.recv()).await.is_err(), "idempotent retry never republishes the original event");
    }
}
//#endregion 🧪️Tests
