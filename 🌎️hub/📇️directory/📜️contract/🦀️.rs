//! 📜️ Directory contract: the error vocabulary, bounded constants, events, records and capability vocabulary every directory backend, the writer and the HTTP surface share. It depends on no other hub crate.

#[path = "../🧾️model/🦀️.rs"]
pub mod model;

#[path = "../🎟️capabilities/🦀️.rs"]
mod capabilities;
use capabilities::*;

#[path = "../🕰️events/🦀️.rs"]
mod events;
use events::*;

#[path = "../🛟️backend-support/🦀️.rs"]
mod backend_support;
use directory::os_directory::{DirectoryActor, DirectoryActorKind, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility};
use crate::error::{DirectoryError, DirectoryResult};
use crate::model::SpaceRole;

//#region 🔖️Error
pub mod error {
    /// 🧯️ Opaque directory error — never wraps a backend driver's error type, so no `sqlx`/
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

//#region 🔖️Wire
/// 🔁️ Local storage role -> wire role (used when an event body needs the wire enum but the value
/// in hand came from a projection read, e.g. an `InviteRecord`). `pub(crate)` — every backend's
/// `//#region 🔖️Projections` uses the reverse direction (`role_from_wire(role).as_str()`) to get
/// the exact `"author"`/`"spectator"` string its `CHECK` constraint speaks.
pub fn role_to_wire(role: SpaceRole) -> DirectorySpaceRole {
    match role {
        SpaceRole::Author => DirectorySpaceRole::Author,
        SpaceRole::Spectator => DirectorySpaceRole::Spectator,
    }
}

/// 🔁️ Wire role -> local storage role (used when a directory capability method that still speaks the
/// storage vocabulary — e.g. `create_invite` — needs a role that arrived as a `DirectoryCommand` field).
pub fn role_from_wire(role: DirectorySpaceRole) -> SpaceRole {
    match role {
        DirectorySpaceRole::Author => SpaceRole::Author,
        DirectorySpaceRole::Spectator => SpaceRole::Spectator,
    }
}

/// 🔡️ Wire space-kind -> the exact lowercase string every backend's `CHECK (kind IN (...))`
/// constraint speaks; used only by each backend's `//#region 🔖️Projections`.
pub fn kind_to_str(kind: DirectorySpaceKind) -> &'static str {
    match kind {
        DirectorySpaceKind::Atelier => "atelier",
        DirectorySpaceKind::Studio => "studio",
        DirectorySpaceKind::Archive => "archive",
    }
}

/// 🔡️ Wire visibility -> the exact lowercase string every backend's `CHECK` constraint speaks.
pub fn visibility_to_str(visibility: DirectorySpaceVisibility) -> &'static str {
    match visibility {
        DirectorySpaceVisibility::Private => "private",
        DirectorySpaceVisibility::Public => "public",
    }
}
//#endregion 🔖️Wire

//#region 🔖️Decider
/// 🎭️ Extracts the plain user id out of a `User`-kind actor's `user:{user_id}#{shell_session_id}`
/// grammar (contract-freeze.md §C0) — directory events denormalize the plain id, never the
/// per-tab/process actor string. `Admin`/`System` actors have no owning user, so `create-space`
/// (the only command that needs one) rejects them.
pub fn actor_user_id(actor: &DirectoryActor) -> DirectoryResult<&str> {
    match actor.kind {
        DirectoryActorKind::User => actor.id.strip_prefix("user:").and_then(|rest| rest.split('#').next()).ok_or_else(|| DirectoryError::Backend(format!("malformed user actor id '{}'", actor.id))),
        _ => Err(DirectoryError::Backend("operation requires a user actor".into())),
    }
}
//#endregion 🔖️Decider

pub use capabilities::{AGENT_DELEGATED_EVENT, AGENT_DELEGATION_PAGE_MAX, AGENT_DELEGATION_REVOKED_EVENT, AGENT_IDENTITY_PROVIDER, AGENT_LABEL_MAX_BYTES, AGENT_SESSION_ISSUED_EVENT, AUTH_ASSERTION_MAX_BYTES, AUTH_AUDIT_PAGE_MAX, AUTH_TEXT_MAX_BYTES, AgentDelegationCapability, AgentSessionPreflight, CAPABILITY_MAX_TTL_SECS, DEVICE_INSTANCE_MAX_BYTES, DIRECTORY_FORMAT_SCHEMA, DIRECTORY_FORMAT_VERSION, DirectoryFormatAdmission, HubCapability, IdentityAssertion, IdentityAssertionVerifier, IdentityAssurance, IdentityVerificationContext, IdentityVerificationControl, IdentityVerificationFuture, IdentityVerificationProgress, InviteCapability, InviteRedemptionPreflight, InviteRedemptionScopeHintV1, InviteRedemptionSpaceStateV1, LocalBootstrapAcceptFuture, LocalBootstrapClientClass, LocalBootstrapIssueFuture, LocalBootstrapRejectCode, LocalBootstrapTerminalFuture, LocalBootstrapTransport, SessionCapability, ShareCapability, SocketGrantCapability, VerifiedIdentity, VerifiedLocalBootstrapRequest, active_capability, admin_operation_effect_receipt_v1, admit_directory_format, agent_session_preflight, auth_audit, capability_window, constant_time_digest_eq, decode_auth_digest_hex, directory_command_result_kind_from_str, directory_command_result_kind_str, encode_capability_bytes, identity_subject_digest, invite_redemption_preflight, prepare_agent_delegation, prepare_auth_session, prepare_invite, prepare_share_token, same_admin_operation_request, validate_admin_operation_audit, validate_admin_operation_effect_receipt, validate_bounded_auth_text, validate_checkpoint_publication_claim, validate_checkpoint_publication_completion, validate_directory_command_claim, verify_invite_redemption_event, verify_invite_redemption_scope_hint};
pub use events::{HubClock, NewDirectoryEvent, now_ms};
pub use backend_support::{AdminEffectPreflightFailure, actor_kind_from_str, actor_kind_to_str, admin_effect_preflight, backend};
