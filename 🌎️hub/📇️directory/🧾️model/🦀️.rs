//! 🧾️ Directory records: the DTOs every backend and the service exchange. Independent of the artifact authority; it reaches back only to the four capability types it embeds.

//#region 🔖️Model
use ::directory::os_directory::ArtifactHash;
pub use ::directory::os_directory::DocumentScope;
use serde::{Deserialize, Serialize};

/// 🔗️ A revocable, expiring, anonymous read grant for exactly one space/document.
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

/// 🙋️ A platform user — local password login and/or one linked SSO identity. Also the
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

/// 🏛️ A space: the tenant/workspace unit that owns documents and memberships. `kind`
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

/// 🧑️‍🤝️‍🧑️ A space member's permission level, string-identical to the `space` crate's
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

    /// 🧢️ This role capped by `ceiling`: an author under a spectator ceiling is a spectator, and no
    /// ceiling ever raises a spectator.
    pub fn within(self, ceiling: SpaceRole) -> SpaceRole {
        match (self, ceiling) {
            (SpaceRole::Author, SpaceRole::Author) => SpaceRole::Author,
            _ => SpaceRole::Spectator,
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

/// 🧭️ How a trusted identity issuer created a durable session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthSessionKind {
    External,
    DevelopmentLocal,
    /// 🤖️ An AI agent acting under a credential a signed-in human delegated to it
    /// (`AgentDelegationRecord`). The session belongs to the delegating user's account for
    /// membership and role, but the principal is the agent: its edits, its presence row and its
    /// per-actor undo history are all its own.
    Agent,
}

impl AuthSessionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::External => "external",
            Self::DevelopmentLocal => "development-local",
            Self::Agent => "agent",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "external" => Some(Self::External),
            "development-local" => Some(Self::DevelopmentLocal),
            "agent" => Some(Self::Agent),
            _ => None,
        }
    }

    /// 🤖️ Whether this session's principal is an agent rather than the human who owns
    /// the account — the one question presence, attribution and undo ownership ask.
    pub fn is_agent(self) -> bool {
        matches!(self, Self::Agent)
    }
}

/// 🍪️ A digest-only browser login session, distinct from a realtime connection.
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

impl AuthSessionRecord {
    /// 🎚️ Who this session asks a space-role question as ([`super::HubDirectory::principal_role`]).
    pub fn principal(&self) -> DirectoryPrincipalV1<'_> {
        DirectoryPrincipalV1 { user_id: &self.user_id, session_kind: self.session_kind, device_instance_id: &self.device_instance_id }
    }
}

/// 🎚️ The authenticated principal of one space-role question: the session's account, its kind and its
/// device instance — for an [`AuthSessionKind::Agent`] session, the id of the delegation it was minted from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectoryPrincipalV1<'a> {
    pub user_id: &'a str,
    pub session_kind: AuthSessionKind,
    pub device_instance_id: &'a str,
}

/// 🧢️ What one session caps its account's membership roles at ([`super::HubDirectory::principal_ceiling`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirectoryPrincipalCeilingV1 {
    /// 🙋️ A human session: its account's membership roles, uncapped.
    Account,
    /// 🤖️ A live agent delegation: its one space only, at most its audience's role.
    Delegation { space_id: String, role: SpaceRole },
    /// 🚫️ A revoked, expired, foreign or unknown delegation: no role anywhere.
    Nothing,
}

impl DirectoryPrincipalCeilingV1 {
    /// 🧢️ `membership` in `space_id` under this ceiling.
    pub fn cap(&self, space_id: &str, membership: Option<SpaceRole>) -> Option<SpaceRole> {
        match self {
            Self::Account => membership,
            Self::Delegation { space_id: scoped, role } => membership.filter(|_| scoped == space_id).map(|membership| membership.within(*role)),
            Self::Nothing => None,
        }
    }
}

/// 🎁️ A newly issued session plus its one-time plaintext capability.
pub struct IssuedAuthSession {
    pub record: AuthSessionRecord,
    pub capability: super::SessionCapability,
}

/// 🧹️ Durable revocation identity returned before connection kicks are attempted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RevokedAuthSession {
    pub id: String,
    pub authorization_generation: u64,
    pub revoked_at: i64,
}

/// 🏭️ Validated input for digest-only session issuance.
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

/// 🔴️ A realtime document connection — the "session as live-features backend" record;
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

/// 🎟️ An outstanding (or revoked) space invite. Not event-sourced itself — only its
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

/// 🤖️ An outstanding (or revoked) agent delegation: the scoped, revocable credential a
/// signed-in human hands to an AI agent so it can act inside ONE space as its own principal.
///
/// Modelled on [`InviteRecord`] on purpose — same selector/secret-digest split, same
/// expiry/revocation columns, same "raw capability bytes never land in the read model or the
/// event log" law. What it adds is the *scope*: `space_id` plus `audience`, the closed set of
/// capability audiences the agent's sessions may exercise. Issuance and revocation are written
/// in the same transaction as their `agent-delegated` / `agent-delegation-revoked` facts, so
/// the row and the log can never disagree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentDelegationRecord {
    pub id: String,
    pub selector: String,
    pub secret_digest: [u8; 32],
    pub space_id: String,
    pub delegating_user_id: String,
    pub agent_label: String,
    pub audience: AgentAudience,
    pub created_at: i64,
    pub expires_at: i64,
    pub revoked_at: Option<i64>,
    pub revoked_reason: Option<String>,
}

/// 🎯️ The closed set of capability audiences one delegation admits. `Read` is the floor
/// every delegation carries; `Edit` additionally admits document mutation. A delegation never
/// carries administration authority — an agent can never invite, revoke, or delegate onward.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentAudience {
    Read,
    Edit,
}

impl AgentAudience {
    pub const ALL: [AgentAudience; 2] = [Self::Read, Self::Edit];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Edit => "edit",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|audience| audience.as_str() == value)
    }

    /// 🎚️ The space role an agent session under this audience is admitted with. `Read`
    /// is a spectator; `Edit` is an author — and never more, whatever the delegating human's
    /// own role is. `SpaceRole` has no administrative variant, so this is a ceiling by
    /// construction: no delegation can ever hand an agent authority the role vocabulary
    /// cannot even spell.
    pub fn space_role(self) -> SpaceRole {
        match self {
            Self::Read => SpaceRole::Spectator,
            Self::Edit => SpaceRole::Author,
        }
    }
}

/// 🎁️ A newly created delegation plus its one-time plaintext capability.
pub struct IssuedAgentDelegation {
    pub record: AgentDelegationRecord,
    pub capability: super::AgentDelegationCapability,
}

/// 📋️ One backend-projected delegation row for the delegation UI. Metadata only: the
/// selector and secret digest are structurally absent, not redacted later.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentDelegationRow {
    pub delegation_id: String,
    pub space_id: String,
    pub delegating_user_id: String,
    pub agent_label: String,
    pub audience: AgentAudience,
    pub created_at_ms: i64,
    pub expires_at_ms: i64,
    pub revoked: bool,
    /// 🕰️ When this delegation last minted an agent session, or `None` if it never has.
    /// Derived, never stored: it is `MAX(issued_at)` over the agent sessions whose
    /// `device_instance_id` is this delegation's id, which is exactly the set
    /// `revoke_agent_delegation` cascades over. A delegation the human has forgotten about is
    /// the one they most need to see here, so "never used" is a state, not a missing value.
    pub last_used_at_ms: Option<i64>,
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

/// 🎁️ A newly issued document share plus its one-time plaintext capability.
pub struct IssuedShareToken {
    pub record: ShareTokenRecord,
    pub capability: super::ShareCapability,
}

/// 🪪️ Current durable status for an id-bound session socket subject: an active binding names the session's
/// kind with its (ceiling-capped) role, because the declared policy grants an agent session other roles than a human's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocketSessionBindingStatus {
    Active { role: Option<SpaceRole>, expires_at_ms: i64, session_kind: AuthSessionKind },
    Revoked,
    Expired,
    MembershipLost,
    Unavailable,
}

/// 🔗️ Current durable status for an id-and-selector-bound share socket subject.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocketShareBindingStatus {
    Active { expires_at_ms: i64 },
    Revoked,
    Expired,
    Unavailable,
}

/// 🎁️ A newly issued invite plus its one-time plaintext capability.
pub struct IssuedInvite {
    pub record: InviteRecord,
    pub capability: super::InviteCapability,
}

/// 🧾️ Privacy-minimized append-only authentication audit entry.
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

/// 🧾️ One credential-lifecycle fact before the backend stamps its identity and time —
/// the durable record of a sign-in attempt or a credential change. `outcome_code` is
/// `"success"` or `"failure"`; `reason_code` carries the refusal's public error code and never
/// a secret, an address, or which half of the credential was wrong.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CredentialAuditFactV1 {
    pub event_kind: String,
    pub target_user_id: Option<String>,
    pub actor_user_id: Option<String>,
    pub outcome_code: String,
    pub reason_code: Option<String>,
    pub correlation_id: String,
    pub peer_class: String,
}

/// 🧾️ One append-only administrator operation fact before backend sequence assignment.
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

/// 📜️ One durable, backend-ordered administrator operation audit fact.
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
//#endregion 🔖️Model
