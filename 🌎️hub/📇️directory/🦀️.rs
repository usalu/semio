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
    use ::directory::os_directory::ArtifactHash;
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

    impl InviteRedemptionCommit {
        pub fn event(&self) -> &::directory::os_directory::DirectoryEvent {
            match self {
                Self::NewlyCommitted { event } | Self::AlreadyCommitted { event } => event,
            }
        }

        pub fn into_event(self) -> ::directory::os_directory::DirectoryEvent {
            match self {
                Self::NewlyCommitted { event } | Self::AlreadyCommitted { event } => event,
            }
        }
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

    /// 🧾️ Private identity atomically committed by a short administrator writer.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NewAdminOperationEffectReceiptV1 {
        pub operation_id: String,
        pub intent_digest: String,
        pub committed_at: i64,
        pub outcome_code: String,
    }

    /// 🧷️ Private factual completion proof; capability plaintext is never retained here.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct AdminOperationEffectReceiptV1 {
        pub operation_id: String,
        pub intent_digest: String,
        pub committed_at: i64,
        pub outcome_code: String,
        pub event_seq_first: Option<u64>,
        pub event_seq_last: Option<u64>,
    }

    /// 🧷️ Closed factual outcome of one backend-owned administrator effect transaction.
    #[derive(Debug)]
    pub enum AdminEffectCommitV1<T> {
        Applied(T),
        RejectedBeforeCommit,
        Indeterminate,
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

    /// 🧾️ Durable state of one authenticated checkpoint-publication correlation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CheckpointPublicationDispositionV1 {
        Pending,
        Completed,
    }

    /// 🆕️ One author-scoped exact-command claim before expensive materialization begins.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NewCheckpointPublicationClaimV1 {
        pub actor_user_id: String,
        pub correlation_id: String,
        pub command_sha256: String,
        pub claimed_at: i64,
    }

    /// 📣️ Exact checkpoint identity completed atomically with the public checkpoint event.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CheckpointPublicationCompletionV1 {
        pub actor_user_id: String,
        pub correlation_id: String,
        pub command_sha256: String,
        pub checkpoint_id: ArtifactHash,
        pub completed_at: i64,
    }

    /// 🧾️ Durable author/correlation/digest identity and its exact successful result.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CheckpointPublicationReceiptRecordV1 {
        pub actor_user_id: String,
        pub correlation_id: String,
        pub command_sha256: String,
        pub disposition: CheckpointPublicationDispositionV1,
        pub checkpoint_id: Option<ArtifactHash>,
        pub claimed_at: i64,
        pub completed_at: Option<i64>,
    }

    /// 🔐️ Closed atomic claim outcome; unequal digests never share a correlation.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum CheckpointPublicationClaimV1 {
        Claimed(CheckpointPublicationReceiptRecordV1),
        Existing(CheckpointPublicationReceiptRecordV1),
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
    descriptor_digest_v1, ArtifactBlobRef, ArtifactCheckpoint, ArtifactFrontier, ArtifactHash, ArtifactRetention, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryCommandOutcomeV1, DirectoryCommandReceiptV1, DirectoryCommandResultV1,
    DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentDescriptor, Hlc, PublishedArtifactBlob, PublishedArtifactCheckpoint,
};
use directory::os_identity::time_ordered_id;
use error::{DirectoryError, DirectoryResult};
pub use crate::artifact_authority::creation::{ArtifactCreationIntentV1, ArtifactCreationClaimV1, ArtifactCreationFactV1, ArtifactCreationFactAppendV1, ArtifactCreationOperationV1, DocumentGenesisAppendV1, DocumentGenesisCommitV1};
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

/// 🧭️ A verified stored scope selects serialization keys; it grants no redemption authority.
pub struct InviteRedemptionScopeHintV1 {
    space_id: String,
}

impl InviteRedemptionScopeHintV1 {
    pub fn space_id(&self) -> &str {
        &self.space_id
    }
}

/// 🔐️ Reveals a stored scope only for the exact capability secret and authenticated actor.
pub(crate) fn verify_invite_redemption_scope_hint(record: Option<&InviteRecord>, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str) -> DirectoryResult<InviteRedemptionScopeHintV1> {
    let record = record.ok_or(DirectoryError::Unauthorized)?;
    if actor_user_id(actor).ok() != Some(user_id) || record.selector != capability.selector() || !constant_time_digest_eq(&record.secret_digest, &capability.secret_digest()) {
        return Err(DirectoryError::Unauthorized);
    }
    Ok(InviteRedemptionScopeHintV1 { space_id: record.space_id.clone() })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InviteRedemptionSpaceStateV1 {
    Missing,
    Writable,
    Archived,
}

impl InviteRedemptionSpaceStateV1 {
    fn from_kind(kind: Option<&str>) -> DirectoryResult<Self> {
        match kind {
            None => Ok(Self::Missing),
            Some("studio" | "atelier") => Ok(Self::Writable),
            Some("archive") => Ok(Self::Archived),
            Some(_) => Err(DirectoryError::Backend("invite space kind is invalid".into())),
        }
    }
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
pub(crate) fn invite_redemption_preflight(record: Option<&InviteRecord>, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str, user_exists: bool, space_state: InviteRedemptionSpaceStateV1, now_ms: i64) -> InviteRedemptionPreflight {
    let actor_user_id = (actor.kind == DirectoryActorKind::User).then(|| actor.id.strip_prefix("user:").and_then(|rest| rest.split('#').next())).flatten();
    let Some(record) = record else { return InviteRedemptionPreflight::Denied };
    if actor_user_id != Some(user_id) || record.selector != capability.selector() || !constant_time_digest_eq(&record.secret_digest, &capability.secret_digest()) || space_state == InviteRedemptionSpaceStateV1::Missing {
        return InviteRedemptionPreflight::Denied;
    }
    if record.accepted_at.is_some() != record.accepted_event_id.is_some() {
        InviteRedemptionPreflight::Corrupt
    } else if record.accepted_at.is_some() {
        InviteRedemptionPreflight::AlreadyCommitted
    } else if !user_exists {
        InviteRedemptionPreflight::Denied
    } else if record.revoked_at.is_some() {
        InviteRedemptionPreflight::Revoked
    } else if record.expires_at <= now_ms {
        InviteRedemptionPreflight::Expired
    } else if space_state == InviteRedemptionSpaceStateV1::Archived && record.role == SpaceRole::Author {
        InviteRedemptionPreflight::Denied
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

/// 🛡️ Admits one durable author/correlation/exact-command checkpoint publication identity.
pub(crate) fn validate_checkpoint_publication_claim(claim: &NewCheckpointPublicationClaimV1) -> DirectoryResult<()> {
    validate_bounded_auth_text(&claim.actor_user_id, "checkpoint publication actor", AUTH_TEXT_MAX_BYTES)?;
    if claim.correlation_id.len() != 32 || claim.correlation_id.bytes().all(|byte| byte == b'0') || !claim.correlation_id.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
        return Err(DirectoryError::Conflict("checkpoint publication correlation must be 32 lowercase nonzero hex digits".into()));
    }
    if claim.command_sha256.len() != 64 || claim.command_sha256.bytes().all(|byte| byte == b'0') || !claim.command_sha256.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
        return Err(DirectoryError::Conflict("checkpoint publication command digest must be 64 lowercase nonzero hex digits".into()));
    }
    Ok(())
}

pub(crate) fn validate_checkpoint_publication_completion(completion: &CheckpointPublicationCompletionV1) -> DirectoryResult<()> {
    validate_checkpoint_publication_claim(&NewCheckpointPublicationClaimV1 {
        actor_user_id: completion.actor_user_id.clone(),
        correlation_id: completion.correlation_id.clone(),
        command_sha256: completion.command_sha256.clone(),
        claimed_at: completion.completed_at,
    })?;
    if completion.checkpoint_id.0 == [0; 32] {
        return Err(DirectoryError::Conflict("checkpoint publication completion has an empty checkpoint id".into()));
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

pub(crate) fn validate_admin_operation_effect_receipt(receipt: &AdminOperationEffectReceiptV1) -> DirectoryResult<()> {
    validate_bounded_auth_text(&receipt.operation_id, "admin effect operation id", AUTH_TEXT_MAX_BYTES)?;
    validate_bounded_auth_text(&receipt.outcome_code, "admin effect outcome", AUTH_TEXT_MAX_BYTES)?;
    if receipt.intent_digest.len() != 64 || !receipt.intent_digest.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
        return Err(DirectoryError::Conflict("admin effect intent digest must be 64 lowercase hex digits".into()));
    }
    if receipt.committed_at <= 0 {
        return Err(DirectoryError::Conflict("admin effect commit time must be positive".into()));
    }
    if receipt.event_seq_first.is_some() != receipt.event_seq_last.is_some() || receipt.event_seq_first.zip(receipt.event_seq_last).is_some_and(|(first, last)| first == 0 || first > last) {
        return Err(DirectoryError::Conflict("admin effect event range is invalid".into()));
    }
    Ok(())
}

pub(crate) fn admin_operation_effect_receipt_v1(claim: &NewAdminOperationEffectReceiptV1, events: &[DirectoryEvent]) -> DirectoryResult<AdminOperationEffectReceiptV1> {
    let receipt = AdminOperationEffectReceiptV1 {
        operation_id: claim.operation_id.clone(),
        intent_digest: claim.intent_digest.clone(),
        committed_at: claim.committed_at,
        outcome_code: claim.outcome_code.clone(),
        event_seq_first: events.first().map(|event| event.seq),
        event_seq_last: events.last().map(|event| event.seq),
    };
    validate_admin_operation_effect_receipt(&receipt)?;
    Ok(receipt)
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
        || !(checkpoint.baseline_frontier.is_genesis_for(&checkpoint.scope) || checkpoint.baseline_frontier.is_edited_for(&checkpoint.scope))
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
        || checkpoint.baseline_frontier.head_edit_ordinal < checkpoint.baseline_frontier.last_commit_seq
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
        || !(retention.retained_floor.is_genesis_for(&retention.scope) || retention.retained_floor.is_edited_for(&retention.scope))
        || retention.retained_floor.head_edit_ordinal > DIRECTORY_WIRE_INTEGER_MAX
        || retention.retained_floor.last_commit_seq > DIRECTORY_WIRE_INTEGER_MAX
        || !valid_hash(retention.retained_checkpoint_id)
        || retention.retained_floor.head_edit_ordinal < retention.retained_floor.last_commit_seq
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

/// 🧱️ Transaction and replay projections validate the durable parent before changing the active head.
pub(crate) fn validate_published_checkpoint_lineage(descriptor: &DocumentDescriptor, active: Option<&PublishedArtifactCheckpoint>, count: u64, candidate: &PublishedArtifactCheckpoint) -> DirectoryResult<()> {
    validate_checkpoint_shape(candidate)?;
    if descriptor.space_id != candidate.scope.space_id || descriptor.document_id != candidate.scope.document_id || descriptor_digest_v1(descriptor).ok() != Some(candidate.descriptor_digest_v1) || count >= ARTIFACT_CHECKPOINT_LINEAGE_MAX {
        return Err(DirectoryError::Conflict("artifact checkpoint durable descriptor or lineage bound differs".into()));
    }
    match active {
        None if count == 0 && candidate.parent_checkpoint_id.is_none() && candidate.baseline_frontier.is_genesis_for(&candidate.scope)
            && descriptor.bootstrap_frontier == (::directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 }) && descriptor.bootstrap_snapshot_hash == candidate.pack.sha256.hex() => Ok(()),
        Some(previous) if previous.scope == candidate.scope && candidate.parent_checkpoint_id == Some(previous.checkpoint_id)
            && candidate.baseline_frontier.is_edited_for(&candidate.scope) && frontier_strictly_advances(&previous.baseline_frontier, &candidate.baseline_frontier) => Ok(()),
        _ => Err(DirectoryError::Conflict("artifact checkpoint durable parent or frontier differs".into())),
    }
}

/// 🌱️ Validates the exact prepared creation and its fixed author/author/system event packet.
pub(crate) fn validate_document_genesis_append_v1(operation: &ArtifactCreationOperationV1, append: &DocumentGenesisAppendV1) -> DirectoryResult<()> {
    let Some(prepared) = operation.prepared.as_ref() else { return Err(DirectoryError::Conflict("genesis creation has no durable prepared pair".into())); };
    prepared.validate(&operation.intent)?;
    if operation.intent != append.intent || operation.phase != ::directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1::Preparing || operation.revision != 2 || append.now_ms >= append.intent.deadline_ms || append.now_ms < prepared.checkpoint.published_at_ms { return Err(DirectoryError::Conflict("genesis creation identity, phase or deadline changed".into())); }
    let mut canonical = append.checkpoint.clone();
    canonical.pack.storage_key = prepared.checkpoint.pack.storage_key.clone(); canonical.spr.storage_key = prepared.checkpoint.spr.storage_key.clone();
    if canonical != prepared.checkpoint { return Err(DirectoryError::Conflict("genesis checkpoint differs from prepared bytes".into())); }
    crate::artifact_authority::chunk_cas::validate_artifact_cas_publication_v1(&append.reservation.plan, &append.checkpoint).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
    validate_published_checkpoint_lineage(&prepared.descriptor, None, 0, &published_artifact_checkpoint(&append.checkpoint))?;
    let expected = [DirectoryEventBody::DocumentAnnounced { descriptor: prepared.descriptor.clone() }, DirectoryEventBody::DocumentIndexed { scope: append.intent.scope.clone(), descriptor_digest_v1: prepared.checkpoint.descriptor_digest_v1, entry: ::directory::os_directory::DocumentIndexEntryV1 { name: append.intent.request.name.clone(), dialect: append.intent.parent_dialect.clone() } }];
    for (event, body) in append.events[..2].iter().zip(expected) {
        if event.body != body || event.actor.kind != DirectoryActorKind::User || actor_user_id(&event.actor)? != append.intent.actor.user_id || event.user_id.as_deref() != Some(&append.intent.actor.user_id) || event.space_id.as_deref() != Some(&append.intent.scope.space_id) { return Err(DirectoryError::Conflict("genesis author event packet differs".into())); }
    }
    validate_verified_checkpoint_append(&append.events[2], &append.checkpoint)?;
    Ok(())
}

/// 🧾️ Only the transaction's actual dense public triple can construct its completion fact.
pub(crate) fn document_genesis_completion_v1(operation: &ArtifactCreationOperationV1, append: &DocumentGenesisAppendV1, events: &[DirectoryEvent]) -> DirectoryResult<ArtifactCreationFactV1> {
    validate_document_genesis_append_v1(operation, append)?;
    if events.len() != 3 || events[0].seq == 0 || events[0].seq.checked_add(1) != Some(events[1].seq) || events[1].seq.checked_add(1) != Some(events[2].seq) || events.iter().zip(&append.events).any(|(event, original)| event.body != original.body || event.actor != original.actor || event.space_id != original.space_id || event.user_id != original.user_id) { return Err(DirectoryError::Conflict("genesis committed event sequence differs".into())); }
    Ok(ArtifactCreationFactV1 { actor_user_id: append.intent.actor.user_id.clone(), request_id: append.intent.request.request_id.clone(), revision: 3, recorded_at_ms: append.now_ms, body: crate::artifact_authority::creation::ArtifactCreationFactBodyV1::Committed { receipt: crate::artifact_authority::creation::ArtifactCreationReceiptV1 { ready: append.intent.ready(), checkpoint_id: append.checkpoint.checkpoint_id, descriptor_digest_v1: append.checkpoint.descriptor_digest_v1, event_seq_first: events[0].seq, event_seq_last: events[2].seq, event_ids: events.iter().map(|event| event.id.clone()).collect() } } })
}

/// 🔏️ Validates the exact immutable descriptor binding before any backend indexes a document.
pub(crate) fn document_index_projection_v1(event: &DirectoryEvent, descriptor: &DocumentDescriptor) -> DirectoryResult<::directory::os_directory::DirectoryIndexedDocumentViewV1> {
    let DirectoryEventBody::DocumentIndexed { scope, descriptor_digest_v1: digest, entry } = &event.body else { return Err(DirectoryError::Conflict("document index event required".into())); };
    ::directory::os_directory::validate_directory_event_page_event(event).map_err(|_| DirectoryError::Conflict("document index event is invalid".into()))?;
    if descriptor.space_id != scope.space_id || descriptor.document_id != scope.document_id || descriptor.artifact_kind != entry.dialect.artifact_kind || descriptor_digest_v1(descriptor).ok().as_ref() != Some(digest) {
        return Err(DirectoryError::Conflict("document index descriptor binding differs".into()));
    }
    Ok(::directory::os_directory::DirectoryIndexedDocumentViewV1 { descriptor: descriptor.clone(), descriptor_digest_v1: *digest, entry: entry.clone(), created_at_ms: event.recorded_at_ms, created_by: event.user_id.clone().ok_or(DirectoryError::Unauthorized)? })
}

/// 📇️ No active checkpoint may outlive its descriptor-bound discoverable index row.
pub(crate) fn validate_checkpoint_index_v1(index: Option<&::directory::os_directory::DirectoryIndexedDocumentViewV1>, descriptor: &DocumentDescriptor, checkpoint: &PublishedArtifactCheckpoint) -> DirectoryResult<()> {
    if index.is_some_and(|row| &row.descriptor == descriptor && row.descriptor_digest_v1 == checkpoint.descriptor_digest_v1 && row.entry.dialect.artifact_kind == descriptor.artifact_kind) { Ok(()) }
    else { Err(DirectoryError::Conflict("artifact checkpoint requires its descriptor-bound index".into())) }
}

/// 🧠️ Dependency-free in-memory artifact projection used by embedded hosts and backend parity laws.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MemoryArtifactProjection {
    descriptors: HashMap<DocumentScope, DocumentDescriptor>,
    index: HashMap<DocumentScope, ::directory::os_directory::DirectoryIndexedDocumentViewV1>,
    checkpoints: HashMap<DocumentScope, Vec<PublishedArtifactCheckpoint>>,
    retention: HashMap<DocumentScope, ArtifactRetention>,
}

impl MemoryArtifactProjection {
    /// 📇️ Returns only a descriptor-verified persisted presentation row.
    pub fn indexed_document(&self, scope: &DocumentScope) -> Option<&::directory::os_directory::DirectoryIndexedDocumentViewV1> {
        self.index.get(scope)
    }

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
                self.index.retain(|scope, _| &scope.space_id != space_id);
                self.checkpoints.retain(|scope, _| &scope.space_id != space_id);
                self.retention.retain(|scope, _| &scope.space_id != space_id);
            }
            DirectoryEventBody::DocumentIndexed { scope, .. } => {
                let descriptor = self.descriptors.get(scope).ok_or_else(|| DirectoryError::NotFound("indexed document descriptor".into()))?;
                let row = document_index_projection_v1(event, descriptor)?;
                if self.index.get(scope).is_some_and(|existing| existing != &row) {
                    return Err(DirectoryError::Conflict("document index is already bound".into()));
                }
                self.index.insert(scope.clone(), row);
            }
            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {
                validate_checkpoint_shape(checkpoint)?;
                let descriptor = self.descriptors.get(&checkpoint.scope).ok_or_else(|| DirectoryError::NotFound("memory artifact descriptor".into()))?;
                validate_checkpoint_index_v1(self.index.get(&checkpoint.scope), descriptor, checkpoint)?;
                if descriptor_digest_v1(descriptor).map_err(|error| DirectoryError::Conflict(error.to_string()))? != checkpoint.descriptor_digest_v1 {
                    return Err(DirectoryError::Conflict("memory artifact descriptor digest mismatch".into()));
                }
                let lineage = self.checkpoints.entry(checkpoint.scope.clone()).or_default();
                if let Some(existing) = lineage.iter().find(|existing| existing.checkpoint_id == checkpoint.checkpoint_id) {
                    return if existing == checkpoint { Ok(()) } else { Err(DirectoryError::Conflict("memory artifact checkpoint identity conflict".into())) };
                }
                validate_published_checkpoint_lineage(descriptor, lineage.last(), lineage.len() as u64, checkpoint)?;
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
/// - `create-space`/`upsert-member` derive the atelier ⇒ ≤1-author law.
/// - `archive-space` emits one intrinsically demoting event; backend transactions enforce
///   archive ⇒ nobody-writes even when an independent writer invalidates this decision.
/// - `remove-member` naming the space's own owner ⇒ `DirectoryError::Conflict` (never removable).
/// - Any command naming a missing/deleted space ⇒ `DirectoryError::NotFound`.
/// - `upsert-member` with an email that has no `UserRecord` yet emits `user.created` first, using
///   a freshly minted user id the following `member.upserted` also uses.
pub async fn decide(
    dir: &HubDirectories,
    actor: &DirectoryActor,
    command: DirectoryCommand,
    clock: &mut HubClock,
) -> DirectoryResult<Decision> {
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
            Ok(single(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceArchived { space_id }))
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
            let actor_user_id = Some(actor_user_id(actor)?);
            let correlation_id = time_ordered_id();
            let issued = dir.issue_invite_as(&space_id, role_from_wire(role), ttl_secs, actor_user_id, &correlation_id).await?;
            Ok(Decision { events: Vec::new(), result: Some(CommandResult { invite_token: Some(issued.capability.expose_once()) }) })
        }
        DirectoryCommand::RevokeInvite { space_id, invite_id } => {
            require_space(dir, &space_id).await?;
            let actor_user_id = Some(actor_user_id(actor)?);
            let correlation_id = time_ordered_id();
            dir.revoke_invite_as(&space_id, &invite_id, "directory-command", actor_user_id, &correlation_id).await?;
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
        None => return Err(DirectoryError::Conflict("ordinary artifact publication requires a committed genesis parent".into())),
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
struct DirectoryWriterTestFence {
    claimed: std::sync::atomic::AtomicBool,
    reached: tokio::sync::Notify,
    release: tokio::sync::Notify,
}

#[cfg(test)]
impl DirectoryWriterTestFence {
    /// 🧪️ Creates a one-shot pause at an explicitly armed writer boundary.
    fn new() -> Self {
        Self { claimed: std::sync::atomic::AtomicBool::new(false), reached: tokio::sync::Notify::new(), release: tokio::sync::Notify::new() }
    }
}

/// 🛑️ A projection refusal whose transaction can be rolled back before any commit attempt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirectoryProjectionRejectionV1 {
    MissingSpace { space_id: String },
    ArchivedAuthor { space_id: String },
    ArchivedDocument { space_id: String },
}

impl DirectoryProjectionRejectionV1 {
    /// 🧯️ Exposes the semantic refusal without claiming a retry-safe transport outcome.
    pub(crate) fn into_error(self) -> DirectoryError {
        DirectoryError::Conflict(match self {
            Self::MissingSpace { space_id } => format!("space '{space_id}' no longer exists"),
            Self::ArchivedAuthor { space_id } => format!("space '{space_id}' is an archive; no author memberships are allowed"),
            Self::ArchivedDocument { space_id } => format!("space '{space_id}' is an archive; document publication is forbidden"),
        })
    }
}

/// 🧮️ Extracts the transactional space authority needed by archive-sensitive projections.
pub(crate) fn directory_projection_space_v1(body: &DirectoryEventBody) -> Option<&str> {
    match body {
        DirectoryEventBody::SpaceArchived { space_id } | DirectoryEventBody::MemberUpserted { space_id, .. } | DirectoryEventBody::InviteRedeemed { space_id, .. } => Some(space_id),
        DirectoryEventBody::DocumentAnnounced { descriptor } => Some(&descriptor.space_id),
        DirectoryEventBody::DocumentIndexed { scope, .. } => Some(&scope.space_id),
        DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => Some(&checkpoint.scope.space_id),
        _ => None,
    }
}

/// 🏛️ Evaluates intrinsic event invariants against the space read under the backend writer lock.
pub(crate) fn directory_projection_rejection_v1(body: &DirectoryEventBody, space_kind: Option<&str>) -> Option<DirectoryProjectionRejectionV1> {
    let space_id = directory_projection_space_v1(body)?;
    if space_kind.is_none() {
        return Some(DirectoryProjectionRejectionV1::MissingSpace { space_id: space_id.into() });
    }
    if space_kind == Some("archive") && matches!(body, DirectoryEventBody::MemberUpserted { role: DirectorySpaceRole::Author, .. } | DirectoryEventBody::InviteRedeemed { role: DirectorySpaceRole::Author, .. }) {
        return Some(DirectoryProjectionRejectionV1::ArchivedAuthor { space_id: space_id.into() });
    }
    if space_kind == Some("archive") && matches!(body, DirectoryEventBody::DocumentAnnounced { .. } | DirectoryEventBody::DocumentIndexed { .. } | DirectoryEventBody::ArtifactCheckpointPublished { .. }) {
        return Some(DirectoryProjectionRejectionV1::ArchivedDocument { space_id: space_id.into() });
    }
    None
}

/// 🔏️ Only an acknowledged rollback may produce RejectedBeforeCommit; every uncertain error stays Err.
#[derive(Clone, Debug)]
pub enum DirectoryAppendOutcomeV1 {
    Appended(Vec<DirectoryEvent>),
    RejectedBeforeCommit(DirectoryProjectionRejectionV1),
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
    delivery_epochs: std::sync::Mutex<(u64, u64, std::collections::BTreeMap<String, u64>)>,
    delivery_write: tokio::sync::RwLock<()>,
    delivery_invalidations: tokio::sync::broadcast::Sender<String>,
    artifact_cas_sweep_secret: [u8; 32],
    #[cfg(test)]
    publication_test_fence: std::sync::Mutex<Option<Arc<DirectoryWriterTestFence>>>,
    #[cfg(test)]
    decision_test_fence: std::sync::Mutex<Option<Arc<DirectoryWriterTestFence>>>,
}

/// 🔐️ A socket retains this opaque read lease only through its bounded final network send.
pub struct DirectoryDeliveryLeaseV1<'a> { _guard: tokio::sync::RwLockReadGuard<'a, ()> }

const DIRECTORY_DELIVERY_SCOPE_MAX: usize = 4096;

#[derive(Clone, Copy)]
struct ArtifactCasSweepPosition {
    execute: bool,
    observed_generation: u64,
    after_generation: u64,
    object_offset: usize,
}

impl DirectoryService {
    /// 🔢️ Captures a monotone reconnect fence before a directory socket subscribes and replays.
    pub fn delivery_epoch(&self, space_id: Option<&str>) -> u64 {
        let epochs = self.delivery_epochs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        space_id.map_or(epochs.0, |space| epochs.2.get(space).copied().unwrap_or(epochs.1))
    }

    /// 📡️ A wakeup is advisory; consumers recheck the epoch before any later event delivery.
    pub fn subscribe_delivery_invalidations(&self) -> tokio::sync::broadcast::Receiver<String> { self.delivery_invalidations.subscribe() }

    /// 🛂️ The epoch check and actual send share a lease; authority and visibility reads precede it.
    pub async fn acquire_delivery_lease(&self, space_id: Option<&str>, expected_epoch: u64) -> Option<DirectoryDeliveryLeaseV1<'_>> {
        let guard = self.delivery_write.read().await;
        (self.delivery_epoch(space_id) == expected_epoch).then_some(DirectoryDeliveryLeaseV1 { _guard: guard })
    }

    async fn invalidate_delivery_locked(&self, _clock: &tokio::sync::MutexGuard<'_, HubClock>, space_id: &str) {
        let _delivery = self.delivery_write.write().await;
        let mut epochs = self.delivery_epochs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        epochs.0 = epochs.0.checked_add(1).expect("directory delivery generation exhausted");
        let next = epochs.0;
        if epochs.2.len() == DIRECTORY_DELIVERY_SCOPE_MAX && !epochs.2.contains_key(space_id) {
            epochs.2.clear();
            epochs.1 = next;
        }
        epochs.2.insert(space_id.into(), next);
        let _ = self.delivery_invalidations.send(space_id.into());
    }

    /// 🗄️ Server-owned services share the exact directory backend without exposing a driver.
    pub(crate) fn backend(&self) -> &Arc<HubDirectories> { &self.dir }

    /// 📣️ Holds one writer through the sole genesis transaction and ordered publication of its triple.
    pub async fn publish_document_genesis(&self, intent: ArtifactCreationIntentV1, prepared: &crate::artifact_authority::creation::ArtifactCreationPreparedV1, checkpoint: ArtifactCheckpoint, reservation: ArtifactCasReservation, now_ms: u64) -> DirectoryResult<ArtifactCreationOperationV1> {
        let mut clock = self.write.lock().await;
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#artifact-creation", intent.actor.user_id) };
        let bodies = [DirectoryEventBody::DocumentAnnounced { descriptor: prepared.descriptor.clone() }, DirectoryEventBody::DocumentIndexed { scope: intent.scope.clone(), descriptor_digest_v1: checkpoint.descriptor_digest_v1, entry: ::directory::os_directory::DocumentIndexEntryV1 { name: intent.request.name.clone(), dialect: intent.parent_dialect.clone() } }, DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: published_artifact_checkpoint(&checkpoint) }];
        let mut ordinal = 0;
        let events = bodies.map(|body| { let system = ordinal == 2; ordinal += 1; NewDirectoryEvent { hlc: clock.tick(), actor: if system { DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-creation".into() } } else { actor.clone() }, space_id: Some(intent.scope.space_id.clone()), user_id: if system { None } else { Some(intent.actor.user_id.clone()) }, body } });
        let append = DocumentGenesisAppendV1 { intent, events, checkpoint, reservation, now_ms };
        match self.dir.append_document_genesis(&append).await? {
            DocumentGenesisCommitV1::Committed { events, operation } => { self.publish_persisted_locked(&clock, events); Ok(operation) }
            DocumentGenesisCommitV1::Existing(operation) => Ok(operation),
            DocumentGenesisCommitV1::Indeterminate => {
                let reconciled = async {
                let facts = self.dir.read_artifact_creation(&append.intent.actor.user_id, &append.intent.request.request_id).await?;
                let operation = ArtifactCreationOperationV1::fold(&facts)?;
                if operation.intent != append.intent { return Err(DirectoryError::Conflict("genesis uncertain receipt belongs to another intent".into())); }
                let Some(receipt) = operation.receipt.as_ref() else { return Err(DirectoryError::Backend("genesis commit acknowledgement is indeterminate; no durable receipt is readable".into())); };
                let events = self.dir.events_since(receipt.event_seq_first - 1, 3).await?;
                let prepared = ArtifactCreationOperationV1::fold(&facts[..2])?;
                let completion = document_genesis_completion_v1(&prepared, &append, &events)?;
                if facts.last() != Some(&completion) { return Err(DirectoryError::Conflict("genesis uncertain public triple differs from its exact stored receipt".into())); }
                self.publish_persisted_locked(&clock, events);
                Ok(operation)
                }.await;
                if reconciled.is_err() { self.invalidate_delivery_locked(&clock, &append.intent.scope.space_id).await; }
                reconciled
            }
        }
    }

    /// @emoji 🏗️ `channel_capacity` sizes the broadcast buffer; a subscriber that falls more than
    /// this many messages behind sees `RecvError::Lagged` and must resync via `events_since`
    /// (`?since=` replay, contract C2) — handled by `bin.rs`'s WS handler, not here.
    pub fn new(dir: Arc<HubDirectories>, channel_capacity: usize) -> Self {
        let (tx, _rx) = tokio::sync::broadcast::channel(channel_capacity);
        let (delivery_invalidations, _) = tokio::sync::broadcast::channel(channel_capacity);
        let mut sweep_secret = Sha256::new();
        sweep_secret.update(ARTIFACT_CAS_SWEEP_CONTINUATION_DOMAIN_V1);
        sweep_secret.update(time_ordered_id().as_bytes());
        Self {
            dir,
            write: tokio::sync::Mutex::new(HubClock::new()),
            tx,
            delivery_epochs: std::sync::Mutex::new((0, 0, std::collections::BTreeMap::new())),
            delivery_write: tokio::sync::RwLock::new(()),
            delivery_invalidations,
            artifact_cas_sweep_secret: sweep_secret.finalize(),
            #[cfg(test)]
            publication_test_fence: std::sync::Mutex::new(None),
            #[cfg(test)]
            decision_test_fence: std::sync::Mutex::new(None),
        }
    }

    #[cfg(test)]
    fn arm_decision_test_fence(&self) -> Arc<DirectoryWriterTestFence> {
        let fence = Arc::new(DirectoryWriterTestFence::new());
        *self.decision_test_fence.lock().unwrap() = Some(fence.clone());
        fence
    }

    #[cfg(test)]
    async fn pause_decision_test_fence_once(&self) {
        let fence = self.decision_test_fence.lock().unwrap().take();
        if let Some(fence) = fence {
            fence.reached.notify_one();
            fence.release.notified().await;
        }
    }

    #[cfg(test)]
    /// 🧪️ Arms a one-shot post-append fence for the writer-order concurrency law.
    fn arm_publication_test_fence(&self) -> Arc<DirectoryWriterTestFence> {
        let fence = Arc::new(DirectoryWriterTestFence::new());
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
            let _ = self.tx.send(DirectoryStreamMessage::Event { event: Box::new(event.clone()) });
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
        #[cfg(test)]
        self.pause_decision_test_fence_once().await;
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
                self.dir.release_directory_command_receipt(&claim.actor_user_id, &claim.request_id, &claim.command_sha256).await?;
                return Err(error);
            }
        };
        #[cfg(test)]
        self.pause_decision_test_fence_once().await;
        let persisted = match self.dir.append_decided_events(&decision.events).await? {
            DirectoryAppendOutcomeV1::Appended(events) => events,
            DirectoryAppendOutcomeV1::RejectedBeforeCommit(reason) => {
                self.dir.release_directory_command_receipt(&claim.actor_user_id, &claim.request_id, &claim.command_sha256).await?;
                return Err(reason.into_error());
            }
        };
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

    /// 🧷️ Runs one pre-audited command and commits its factual effect receipt with the writer.
    pub async fn execute_with_admin_effect(
        &self,
        actor: DirectoryActor,
        command: DirectoryCommand,
        effect: &NewAdminOperationEffectReceiptV1,
    ) -> AdminEffectCommitV1<(Vec<DirectoryEvent>, Option<CommandResult>)> {
        let mut clock = self.write.lock().await;
        match &command {
            DirectoryCommand::CreateInvite { space_id, role, ttl_secs } => {
                match require_space(self.dir.as_ref(), space_id).await {
                    Ok(_) => {}
                    Err(DirectoryError::Backend(_)) => return AdminEffectCommitV1::Indeterminate,
                    Err(_) => return AdminEffectCommitV1::RejectedBeforeCommit,
                }
                let Ok(ttl_secs) = i64::try_from(*ttl_secs) else { return AdminEffectCommitV1::RejectedBeforeCommit };
                let Ok(actor_user_id) = actor_user_id(&actor) else { return AdminEffectCommitV1::RejectedBeforeCommit };
                let correlation_id = time_ordered_id();
                return match self.dir.issue_invite_as_with_admin_effect(space_id, role_from_wire(*role), ttl_secs, Some(actor_user_id), &correlation_id, effect).await {
                    AdminEffectCommitV1::Applied(issued) => AdminEffectCommitV1::Applied((Vec::new(), Some(CommandResult { invite_token: Some(issued.capability.expose_once()) }))),
                    AdminEffectCommitV1::RejectedBeforeCommit => AdminEffectCommitV1::RejectedBeforeCommit,
                    AdminEffectCommitV1::Indeterminate => AdminEffectCommitV1::Indeterminate,
                };
            }
            DirectoryCommand::RevokeInvite { space_id, invite_id } => {
                match require_space(self.dir.as_ref(), space_id).await {
                    Ok(_) => {}
                    Err(DirectoryError::Backend(_)) => return AdminEffectCommitV1::Indeterminate,
                    Err(_) => return AdminEffectCommitV1::RejectedBeforeCommit,
                }
                let Ok(actor_user_id) = actor_user_id(&actor) else { return AdminEffectCommitV1::RejectedBeforeCommit };
                let correlation_id = time_ordered_id();
                return match self.dir.revoke_invite_as_with_admin_effect(space_id, invite_id, "directory-command", Some(actor_user_id), &correlation_id, effect).await {
                    AdminEffectCommitV1::Applied(()) => AdminEffectCommitV1::Applied((Vec::new(), None)),
                    AdminEffectCommitV1::RejectedBeforeCommit => AdminEffectCommitV1::RejectedBeforeCommit,
                    AdminEffectCommitV1::Indeterminate => AdminEffectCommitV1::Indeterminate,
                };
            }
            _ => {}
        }
        let decision = match decide(self.dir.as_ref(), &actor, command, &mut clock).await {
            Ok(decision) => decision,
            Err(DirectoryError::Backend(_)) => return AdminEffectCommitV1::Indeterminate,
            Err(_) => return AdminEffectCommitV1::RejectedBeforeCommit,
        };
        #[cfg(test)]
        self.pause_decision_test_fence_once().await;
        let persisted = if decision.events.is_empty() {
            Vec::new()
        } else {
            match self.dir.append_decided_events_with_admin_effect(&decision.events, effect).await {
                AdminEffectCommitV1::Applied(events) => events,
                AdminEffectCommitV1::RejectedBeforeCommit => return AdminEffectCommitV1::RejectedBeforeCommit,
                AdminEffectCommitV1::Indeterminate => return AdminEffectCommitV1::Indeterminate,
            }
        };
        AdminEffectCommitV1::Applied((self.publish_persisted_locked(&clock, persisted), decision.result))
    }

    /// 🌱️ Creates an administrator-selected resource id with its factual receipt in one append.
    pub async fn execute_create_space_with_id_and_admin_effect(
        &self,
        actor: DirectoryActor,
        space_id: String,
        name: String,
        space_kind: DirectorySpaceKind,
        visibility: DirectorySpaceVisibility,
        effect: &NewAdminOperationEffectReceiptV1,
    ) -> AdminEffectCommitV1<Vec<DirectoryEvent>> {
        let mut clock = self.write.lock().await;
        match self.dir.get_space(&space_id).await {
            Ok(Some(_)) => return AdminEffectCommitV1::RejectedBeforeCommit,
            Ok(None) => {}
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        }
        let decision = match decide_create_space(&actor, space_id, name, space_kind, visibility, &mut clock) {
            Ok(decision) => decision,
            Err(DirectoryError::Backend(_)) => return AdminEffectCommitV1::Indeterminate,
            Err(_) => return AdminEffectCommitV1::RejectedBeforeCommit,
        };
        match self.dir.append_decided_events_with_admin_effect(&decision.events, effect).await {
            AdminEffectCommitV1::Applied(events) => AdminEffectCommitV1::Applied(self.publish_persisted_locked(&clock, events)),
            AdminEffectCommitV1::RejectedBeforeCommit => AdminEffectCommitV1::RejectedBeforeCommit,
            AdminEffectCommitV1::Indeterminate => AdminEffectCommitV1::Indeterminate,
        }
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
            [] => self.dir.append_reserved_artifact_checkpoint(None, &checkpoint, &reservation, None, now_ms).await?,
            [event] => self.dir.append_reserved_artifact_checkpoint(Some(event), &checkpoint, &reservation, None, now_ms).await?,
            _ => return Err(DirectoryError::Backend("verified checkpoint decision emitted more than one event".into())),
        };
        Ok(self.publish_persisted_locked(&clock, persisted))
    }

    /// 🆔️ Claims one durable author/correlation/exact-command publication identity.
    pub async fn claim_or_read_checkpoint_publication(&self, claim: &NewCheckpointPublicationClaimV1) -> DirectoryResult<CheckpointPublicationClaimV1> {
        self.dir.claim_or_read_checkpoint_publication(claim).await
    }

    /// 🧹️ Releases only this author's still-pending publication claim after pre-append failure.
    pub async fn release_checkpoint_publication(&self, actor_user_id: &str, correlation_id: &str, command_sha256: &str) -> DirectoryResult<()> {
        self.dir.release_checkpoint_publication(actor_user_id, correlation_id, command_sha256).await
    }

    /// 📣️ Completes the exact durable claim in the same backend transaction as checkpoint/event publication.
    pub async fn publish_reserved_artifact_checkpoint_and_complete_checkpoint_publication(
        &self,
        actor: DirectoryActor,
        checkpoint: ArtifactCheckpoint,
        reservation: ArtifactCasReservation,
        completion: CheckpointPublicationCompletionV1,
        now_ms: u64,
    ) -> DirectoryResult<Vec<DirectoryEvent>> {
        validate_checkpoint_publication_completion(&completion)?;
        if completion.checkpoint_id != checkpoint.checkpoint_id {
            return Err(DirectoryError::Conflict("checkpoint publication completion differs from the verified checkpoint".into()));
        }
        let mut clock = self.write.lock().await;
        let decision = decide_verified_checkpoint(self.dir.as_ref(), &actor, &checkpoint, &mut clock).await?;
        let persisted = match decision.events.as_slice() {
            [event] => self.dir.append_reserved_artifact_checkpoint(Some(event), &checkpoint, &reservation, Some(&completion), now_ms).await?,
            [] => return Err(DirectoryError::Conflict("checkpoint publication claim cannot complete without a new checkpoint event".into())),
            _ => return Err(DirectoryError::Backend("verified checkpoint decision emitted more than one event".into())),
        };
        Ok(self.publish_persisted_locked(&clock, persisted))
    }

    /// 🎟️ Atomically claims one invite with its event and membership, then publishes before releasing the writer.
    pub async fn redeem_invite(&self, actor: DirectoryActor, capability: &InviteCapability, user_id: &str) -> DirectoryResult<InviteRedemptionCommit> {
        let mut clock = self.write.lock().await;
        if actor_user_id(&actor)? != user_id {
            return Err(DirectoryError::Unauthorized);
        }
        let hlc = clock.tick();
        let committed = self.dir.redeem_invite_atomic(capability, &actor, user_id, hlc).await?;
        if let InviteRedemptionCommit::NewlyCommitted { event } = &committed {
            let persisted = vec![event.clone()];
            #[cfg(test)]
            self.pause_publication_test_fence_once(&persisted).await;
            self.publish_persisted_locked(&clock, persisted);
        }
        Ok(committed)
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
    async fn issue_share_token_as_with_admin_effect(
        &self,
        scope: &DocumentScope,
        ttl_secs: i64,
        actor_user_id: Option<&str>,
        correlation_id: &str,
        effect: &NewAdminOperationEffectReceiptV1,
    ) -> AdminEffectCommitV1<IssuedShareToken>;
    async fn issue_share_token(&self, scope: &DocumentScope, ttl_secs: i64, correlation_id: &str) -> DirectoryResult<IssuedShareToken> {
        self.issue_share_token_as(scope, ttl_secs, None, correlation_id).await
    }
    async fn revoke_share_token_as(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()>;
    async fn revoke_share_token_as_with_admin_effect(
        &self,
        scope: &DocumentScope,
        share_id: &str,
        reason: &str,
        actor_user_id: Option<&str>,
        correlation_id: &str,
        effect: &NewAdminOperationEffectReceiptV1,
    ) -> AdminEffectCommitV1<()>;
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
    async fn claim_artifact_creation(&self, intent: &ArtifactCreationIntentV1) -> DirectoryResult<ArtifactCreationClaimV1>;
    async fn read_artifact_creation(&self, actor_user_id: &str, request_id: &str) -> DirectoryResult<Vec<ArtifactCreationFactV1>>;
    async fn append_artifact_creation_fact(&self, append: &ArtifactCreationFactAppendV1) -> DirectoryResult<ArtifactCreationOperationV1>;
    async fn append_document_genesis(&self, append: &DocumentGenesisAppendV1) -> DirectoryResult<DocumentGenesisCommitV1>;
    async fn artifact_creation_recovery_candidates(&self, now_ms: u64, limit: usize) -> DirectoryResult<Vec<ArtifactCreationIntentV1>>;
    async fn artifact_creation_terminate_uncommitted(&self, intent: &ArtifactCreationIntentV1, now_ms: u64) -> DirectoryResult<ArtifactCreationOperationV1>;
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
    async fn append_reserved_artifact_checkpoint(
        &self,
        _event: Option<&NewDirectoryEvent>,
        _checkpoint: &ArtifactCheckpoint,
        _reservation: &ArtifactCasReservation,
        _completion: Option<&CheckpointPublicationCompletionV1>,
        _now_ms: u64,
    ) -> DirectoryResult<Vec<DirectoryEvent>> {
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
    async fn revoke_auth_sessions_for_user_with_admin_effect(
        &self,
        user_id: &str,
        reason: &str,
        actor_user_id: Option<&str>,
        correlation_id: &str,
        effect: &NewAdminOperationEffectReceiptV1,
    ) -> AdminEffectCommitV1<Vec<RevokedAuthSession>>;
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
    /// 🧷️ Reads an exact private writer receipt without exposing capability material.
    async fn admin_operation_effect_receipt(&self, operation_id: &str, intent_digest: &str) -> DirectoryResult<Option<AdminOperationEffectReceiptV1>>;
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
    async fn release_directory_command_receipt(&self, _actor_user_id: &str, _request_id: &str, _command_sha256: &str) -> DirectoryResult<()> {
        Err(DirectoryError::Backend("directory command receipts are unavailable for this backend".into()))
    }

    /// 🆔️ Atomically claims or reads one author/correlation/exact-command checkpoint publication.
    async fn claim_or_read_checkpoint_publication(&self, _claim: &NewCheckpointPublicationClaimV1) -> DirectoryResult<CheckpointPublicationClaimV1> {
        Err(DirectoryError::Backend("checkpoint publication receipts are unavailable for this backend".into()))
    }
    /// 🧹️ Releases an equal still-pending claim; a substituted digest never removes its owner.
    async fn release_checkpoint_publication(&self, _actor_user_id: &str, _correlation_id: &str, _command_sha256: &str) -> DirectoryResult<()> {
        Err(DirectoryError::Backend("checkpoint publication receipts are unavailable for this backend".into()))
    }
    //#endregion

    //#region Invites
    // 🎟️ Not event-sourced (contract's decider laws) — only redemption is (`invite.redeemed`, see
    // `DirectoryService::redeem_invite`). `create_invite`/`revoke_invite` are called directly by
    // `decide` as its one documented write exception (`//#region 🔖️Decider`).
    async fn issue_invite_as(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedInvite>;
    async fn issue_invite_as_with_admin_effect(
        &self,
        space_id: &str,
        role: SpaceRole,
        ttl_secs: i64,
        actor_user_id: Option<&str>,
        correlation_id: &str,
        effect: &NewAdminOperationEffectReceiptV1,
    ) -> AdminEffectCommitV1<IssuedInvite>;
    async fn issue_invite(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, correlation_id: &str) -> DirectoryResult<IssuedInvite> {
        self.issue_invite_as(space_id, role, ttl_secs, None, correlation_id).await
    }
    /// 🧭️ Verifies the exact stored capability before selecting the server-owned serialization scope.
    async fn invite_redemption_scope_hint(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str) -> DirectoryResult<InviteRedemptionScopeHintV1>;
    /// 🎟️ Claims `accepted_at`, appends the derived event and applies membership in one backend transaction.
    async fn redeem_invite_atomic(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str, hlc: Hlc) -> DirectoryResult<InviteRedemptionCommit>;
    async fn revoke_invite_as(&self, space_id: &str, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()>;
    async fn revoke_invite_as_with_admin_effect(
        &self,
        space_id: &str,
        invite_id: &str,
        reason: &str,
        actor_user_id: Option<&str>,
        correlation_id: &str,
        effect: &NewAdminOperationEffectReceiptV1,
    ) -> AdminEffectCommitV1<()>;
    async fn revoke_invite(&self, space_id: &str, invite_id: &str, reason: &str, correlation_id: &str) -> DirectoryResult<()> {
        self.revoke_invite_as(space_id, invite_id, reason, None, correlation_id).await
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
    async fn append_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>> {
        match self.append_decided_events(events).await? {
            DirectoryAppendOutcomeV1::Appended(events) => Ok(events),
            DirectoryAppendOutcomeV1::RejectedBeforeCommit(reason) => Err(reason.into_error()),
        }
    }
    /// 🔐️ Appends a decided batch or proves a semantic refusal by acknowledging rollback.
    /// Errors, including failed rollback or commit, never authorize release of an idempotency claim.
    async fn append_decided_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<DirectoryAppendOutcomeV1>;
    /// 🧷️ Commits the exact event page and its private administrator effect proof atomically.
    async fn append_decided_events_with_admin_effect(&self, events: &[NewDirectoryEvent], effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<DirectoryEvent>>;
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
    async fn claim_artifact_creation(&self, intent: &ArtifactCreationIntentV1) -> DirectoryResult<ArtifactCreationClaimV1> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.claim_artifact_creation(intent).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.claim_artifact_creation(intent).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.claim_artifact_creation(intent).await,
        }
    }

    async fn read_artifact_creation(&self, actor_user_id: &str, request_id: &str) -> DirectoryResult<Vec<ArtifactCreationFactV1>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.read_artifact_creation(actor_user_id, request_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.read_artifact_creation(actor_user_id, request_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.read_artifact_creation(actor_user_id, request_id).await,
        }
    }

    async fn append_artifact_creation_fact(&self, append: &ArtifactCreationFactAppendV1) -> DirectoryResult<ArtifactCreationOperationV1> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.append_artifact_creation_fact(append).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.append_artifact_creation_fact(append).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.append_artifact_creation_fact(append).await,
        }
    }

    async fn append_document_genesis(&self, append: &DocumentGenesisAppendV1) -> DirectoryResult<DocumentGenesisCommitV1> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.append_document_genesis(append).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.append_document_genesis(append).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.append_document_genesis(append).await,
        }
    }

    async fn artifact_creation_terminate_uncommitted(&self, intent: &ArtifactCreationIntentV1, now_ms: u64) -> DirectoryResult<ArtifactCreationOperationV1> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.artifact_creation_terminate_uncommitted(intent, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.artifact_creation_terminate_uncommitted(intent, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.artifact_creation_terminate_uncommitted(intent, now_ms).await,
        }
    }

    async fn artifact_creation_recovery_candidates(&self, now_ms: u64, limit: usize) -> DirectoryResult<Vec<ArtifactCreationIntentV1>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.artifact_creation_recovery_candidates(now_ms, limit).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.artifact_creation_recovery_candidates(now_ms, limit).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.artifact_creation_recovery_candidates(now_ms, limit).await,
        }
    }

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

    async fn issue_share_token_as_with_admin_effect(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<IssuedShareToken> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.issue_share_token_as_with_admin_effect(scope, ttl_secs, actor_user_id, correlation_id, effect).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.issue_share_token_as_with_admin_effect(scope, ttl_secs, actor_user_id, correlation_id, effect).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.issue_share_token_as_with_admin_effect(scope, ttl_secs, actor_user_id, correlation_id, effect).await,
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

    async fn revoke_share_token_as_with_admin_effect(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.revoke_share_token_as_with_admin_effect(scope, share_id, reason, actor_user_id, correlation_id, effect).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.revoke_share_token_as_with_admin_effect(scope, share_id, reason, actor_user_id, correlation_id, effect).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.revoke_share_token_as_with_admin_effect(scope, share_id, reason, actor_user_id, correlation_id, effect).await,
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

    async fn append_reserved_artifact_checkpoint(
        &self,
        event: Option<&NewDirectoryEvent>,
        checkpoint: &ArtifactCheckpoint,
        reservation: &ArtifactCasReservation,
        completion: Option<&CheckpointPublicationCompletionV1>,
        now_ms: u64,
    ) -> DirectoryResult<Vec<DirectoryEvent>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.append_reserved_artifact_checkpoint(event, checkpoint, reservation, completion, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.append_reserved_artifact_checkpoint(event, checkpoint, reservation, completion, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.append_reserved_artifact_checkpoint(event, checkpoint, reservation, completion, now_ms).await,
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

    async fn revoke_auth_sessions_for_user_with_admin_effect(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<RevokedAuthSession>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.revoke_auth_sessions_for_user_with_admin_effect(user_id, reason, actor_user_id, correlation_id, effect).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.revoke_auth_sessions_for_user_with_admin_effect(user_id, reason, actor_user_id, correlation_id, effect).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.revoke_auth_sessions_for_user_with_admin_effect(user_id, reason, actor_user_id, correlation_id, effect).await,
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

    async fn admin_operation_effect_receipt(&self, operation_id: &str, intent_digest: &str) -> DirectoryResult<Option<AdminOperationEffectReceiptV1>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.admin_operation_effect_receipt(operation_id, intent_digest).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.admin_operation_effect_receipt(operation_id, intent_digest).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.admin_operation_effect_receipt(operation_id, intent_digest).await,
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

    async fn claim_or_read_directory_command_receipt(&self, claim: &NewDirectoryCommandReceipt) -> DirectoryResult<DirectoryCommandClaimV1> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.claim_or_read_directory_command_receipt(claim).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.claim_or_read_directory_command_receipt(claim).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.claim_or_read_directory_command_receipt(claim).await,
        }
    }

    async fn complete_directory_command_receipt(&self, completion: &DirectoryCommandReceiptCompletion) -> DirectoryResult<DirectoryCommandReceiptRecord> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.complete_directory_command_receipt(completion).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.complete_directory_command_receipt(completion).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.complete_directory_command_receipt(completion).await,
        }
    }

    async fn release_directory_command_receipt(&self, actor_user_id: &str, request_id: &str, command_sha256: &str) -> DirectoryResult<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.release_directory_command_receipt(actor_user_id, request_id, command_sha256).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.release_directory_command_receipt(actor_user_id, request_id, command_sha256).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.release_directory_command_receipt(actor_user_id, request_id, command_sha256).await,
        }
    }

    async fn claim_or_read_checkpoint_publication(&self, claim: &NewCheckpointPublicationClaimV1) -> DirectoryResult<CheckpointPublicationClaimV1> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.claim_or_read_checkpoint_publication(claim).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.claim_or_read_checkpoint_publication(claim).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.claim_or_read_checkpoint_publication(claim).await,
        }
    }

    async fn release_checkpoint_publication(&self, actor_user_id: &str, correlation_id: &str, command_sha256: &str) -> DirectoryResult<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.release_checkpoint_publication(actor_user_id, correlation_id, command_sha256).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.release_checkpoint_publication(actor_user_id, correlation_id, command_sha256).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.release_checkpoint_publication(actor_user_id, correlation_id, command_sha256).await,
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

    async fn issue_invite_as_with_admin_effect(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<IssuedInvite> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.issue_invite_as_with_admin_effect(space_id, role, ttl_secs, actor_user_id, correlation_id, effect).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.issue_invite_as_with_admin_effect(space_id, role, ttl_secs, actor_user_id, correlation_id, effect).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.issue_invite_as_with_admin_effect(space_id, role, ttl_secs, actor_user_id, correlation_id, effect).await,
        }
    }

    async fn invite_redemption_scope_hint(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str) -> DirectoryResult<InviteRedemptionScopeHintV1> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.invite_redemption_scope_hint(capability, actor, user_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.invite_redemption_scope_hint(capability, actor, user_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.invite_redemption_scope_hint(capability, actor, user_id).await,
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

    async fn revoke_invite_as(&self, space_id: &str, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.revoke_invite_as(space_id, invite_id, reason, actor_user_id, correlation_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.revoke_invite_as(space_id, invite_id, reason, actor_user_id, correlation_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.revoke_invite_as(space_id, invite_id, reason, actor_user_id, correlation_id).await,
        }
    }

    async fn revoke_invite_as_with_admin_effect(&self, space_id: &str, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.revoke_invite_as_with_admin_effect(space_id, invite_id, reason, actor_user_id, correlation_id, effect).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.revoke_invite_as_with_admin_effect(space_id, invite_id, reason, actor_user_id, correlation_id, effect).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.revoke_invite_as_with_admin_effect(space_id, invite_id, reason, actor_user_id, correlation_id, effect).await,
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

    async fn append_decided_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<DirectoryAppendOutcomeV1> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.append_decided_events(events).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.append_decided_events(events).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.append_decided_events(events).await,
        }
    }

    async fn append_decided_events_with_admin_effect(&self, events: &[NewDirectoryEvent], effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<DirectoryEvent>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.append_decided_events_with_admin_effect(events, effect).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.append_decided_events_with_admin_effect(events, effect).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.append_decided_events_with_admin_effect(events, effect).await,
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
