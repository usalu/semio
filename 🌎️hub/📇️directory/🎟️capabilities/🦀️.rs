//! 🎟️ Capabilities, identity assertions, local bootstrap transports, auth preflight and audit validators.

use crate::model::{AdminOperationEffectReceiptV1, AgentAudience, AgentDelegationRecord, AuthAuditRecord, AuthSessionIssue, AuthSessionRecord, CheckpointPublicationCompletionV1, DirectoryCommandResultKindV1, InviteRecord, IssuedAgentDelegation, IssuedAuthSession, IssuedInvite, IssuedShareToken, NewAdminOperationAuditRecord, NewAdminOperationEffectReceiptV1, NewCheckpointPublicationClaimV1, NewDirectoryCommandReceipt, ShareTokenRecord, SpaceRole};
use directory::os_directory::{DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DocumentScope};
use crate::error::{DirectoryError, DirectoryResult};
use semio_framework_hash::Sha256;
use crate::{actor_user_id, role_to_wire};
use directory::os_identity::time_ordered_id;

//#region 🔖️Capabilities
pub const CAPABILITY_SELECTOR_BYTES: usize = 16;
pub const CAPABILITY_SECRET_BYTES: usize = 32;
pub const CAPABILITY_MAX_TTL_SECS: i64 = 31_536_000;
pub const DEVICE_INSTANCE_MAX_BYTES: usize = 128;
pub const AUTH_ASSERTION_MAX_BYTES: usize = 16 * 1024;
pub const AUTH_TEXT_MAX_BYTES: usize = 256;

/// 🔐️ Encodes capability bytes without a runtime dependency.
/// 📐️ The stamp shape a directory database carries, so a future format is recognised rather than
/// guessed at. Mirrors `🗄️stores/🦀️.rs`'s `STORE_FORMAT_SCHEMA` for the four filesystem stores.
pub const DIRECTORY_FORMAT_SCHEMA: &str = "semio/hub/directory-format/v1";

/// 📐️ The directory record format this build writes and reads. Bump it in the same commit that
/// changes what a `hub_directory_event` row, a projection column or a capability digest means.
pub const DIRECTORY_FORMAT_VERSION: i64 = 1;

/// 📐️ What this build must do about the stamp it found on a directory database.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectoryFormatAdmission {
    Stamp,
    Admitted,
}

/// 📐️ Decides whether a directory database may be opened, from the stamp it carries.
///
/// The hub has **no migration framework** on purpose (`🌎️hub/README.md` § "There is no cross-version
/// upgrade path"), and `🗄️stores/🦀️.rs` already pays that decision's price for the four filesystem
/// stores by stamping each one on creation and refusing a root it did not write. The three directory
/// backends carried no stamp at all, so a build could fold over event rows and projection columns it
/// only half-understands and produce a plausible, wrong directory in silence — the exact failure the
/// filesystem law exists to prevent. Four outcomes, each a decision rather than a default:
/// - **no stamp, no events** — creation: write this build's stamp.
/// - **no stamp, events already there** — adoption at [`DIRECTORY_FORMAT_VERSION`], with a warning.
///   Exactly one directory format has ever existed, so an unstamped database is genuinely a v1 one.
///   This branch stops being reachable the moment a v2 exists, and must be deleted then.
/// - **a newer version, or a schema this build does not know** — refused; this build cannot know
///   what a later format means.
/// - **an older version** — refused; there is no migration framework to run.
pub fn admit_directory_format(observed: Option<(String, i64)>, populated: bool, backend: &'static str) -> DirectoryResult<DirectoryFormatAdmission> {
    let Some((schema, version)) = observed else {
        if populated {
            eprintln!("[WARN] adopting an unstamped {backend} hub directory as format v{DIRECTORY_FORMAT_VERSION}");
        }
        return Ok(DirectoryFormatAdmission::Stamp);
    };
    if schema != DIRECTORY_FORMAT_SCHEMA {
        return Err(DirectoryError::Backend(format!(
            "{backend} hub directory carries an unknown format stamp {schema:?}; this build writes {DIRECTORY_FORMAT_SCHEMA}. There is no migration framework: back the database up and start this build from an empty one."
        )));
    }
    if version != DIRECTORY_FORMAT_VERSION {
        return Err(DirectoryError::Backend(format!(
            "{backend} hub directory is format v{version} and this build reads v{DIRECTORY_FORMAT_VERSION}. There is no migration framework: back the database up and start this build from an empty one."
        )));
    }
    Ok(DirectoryFormatAdmission::Admitted)
}

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

pub fn decode_auth_digest_hex(encoded: &str) -> DirectoryResult<[u8; 32]> {
    decode_capability_bytes(encoded)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapabilityKind {
    Session,
    Share,
    Invite,
    Socket,
    /// 🤖️ The long-lived, scoped, revocable credential a signed-in human hands to an AI
    /// agent. It is never a session: it is exchanged for one at `POST /auth/agent-sessions`.
    AgentDelegation,
}

impl CapabilityKind {
    fn prefix(self) -> &'static str {
        match self {
            Self::Session => "session.v1",
            Self::Share => "share.v1",
            Self::Invite => "invite.v1",
            Self::Socket => "socket.v1",
            Self::AgentDelegation => "delegation.v1",
        }
    }

    fn digest_domain(self) -> &'static [u8] {
        match self {
            Self::Session => b"semio/hub/session/v1\0",
            Self::Share => b"semio/hub/share/v1\0",
            Self::Invite => b"semio/hub/invite/v1\0",
            Self::Socket => b"semio/hub/socket/v1\0",
            Self::AgentDelegation => b"semio/hub/agent-delegation/v1\0",
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct CapabilityParts {
    pub(super) selector: String,
    secret: [u8; CAPABILITY_SECRET_BYTES],
}

fn parse_capability_parts(encoded: &str, kind: CapabilityKind) -> DirectoryResult<CapabilityParts> {
    let mut components = encoded.split('.');
    let expected_type = match kind {
        CapabilityKind::Session => "session",
        CapabilityKind::Share => "share",
        CapabilityKind::Invite => "invite",
        CapabilityKind::Socket => "socket",
        CapabilityKind::AgentDelegation => "delegation",
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
capability_type!(AgentDelegationCapability, CapabilityKind::AgentDelegation);

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

/// ⏳️ Validates a bounded positive TTL and returns its overflow-safe millisecond window.
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
pub struct IdentityAssertion(pub Box<[u8]>);

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
    /// 🚪️ Resolves once the launcher's end of the pipe is gone, in every phase of the hub — also
    /// while it is still loading its catalog and nobody is accepting requests yet.
    fn closed<'a>(&'a self) -> LocalBootstrapTerminalFuture<'a>;
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

/// 🤖️ The durable fact kinds agent delegation appends to the same authentication log sessions use.
/// Nothing here is a counter or a mutable status column: `agent-delegated` is the issuance, and
/// `agent-delegation-revoked` is the withdrawal, each written in the transaction that changed the
/// row it describes.
pub const AGENT_DELEGATED_EVENT: &str = "agent-delegated";
pub const AGENT_DELEGATION_REVOKED_EVENT: &str = "agent-delegation-revoked";
pub const AGENT_SESSION_ISSUED_EVENT: &str = "agent-session-issued";

/// 🤖️ The identity provider every agent session is issued under, so a directory revocation by
/// identity can target exactly the agent sessions and leave the delegating human signed in.
pub const AGENT_IDENTITY_PROVIDER: &str = "agent.delegation.v1";

/// 🤖️ The most delegations one listing returns.
pub const AGENT_DELEGATION_PAGE_MAX: usize = 200;

pub fn prepare_auth_session(issue: &AuthSessionIssue, now: i64) -> DirectoryResult<IssuedAuthSession> {
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

pub fn prepare_share_token(scope: &DocumentScope, ttl_secs: i64, now: i64) -> DirectoryResult<IssuedShareToken> {
    let (created_at, expires_at) = capability_window(now, ttl_secs)?;
    let capability = ShareCapability::mint()?;
    let record = ShareTokenRecord { id: time_ordered_id(), selector: capability.selector().to_string(), secret_digest: capability.secret_digest(), scope: scope.clone(), created_at, expires_at, revoked_at: None, revoked_reason: None };
    Ok(IssuedShareToken { record, capability })
}

pub fn prepare_invite(space_id: &str, role: SpaceRole, ttl_secs: i64, now: i64) -> DirectoryResult<IssuedInvite> {
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

/// 🤖️ Mints one agent delegation, bounds-checked. `agent_label` is what a human will read in
/// the roster next to the robot badge, so it is bounded like every other auth text.
pub fn prepare_agent_delegation(space_id: &str, delegating_user_id: &str, agent_label: &str, audience: AgentAudience, ttl_secs: i64, now: i64) -> DirectoryResult<IssuedAgentDelegation> {
    validate_bounded_auth_text(space_id, "agent delegation space", AUTH_TEXT_MAX_BYTES)?;
    validate_bounded_auth_text(delegating_user_id, "agent delegation user", AUTH_TEXT_MAX_BYTES)?;
    validate_bounded_auth_text(agent_label, "agent delegation label", AGENT_LABEL_MAX_BYTES)?;
    let (created_at, expires_at) = capability_window(now, ttl_secs)?;
    let capability = AgentDelegationCapability::mint()?;
    let record = AgentDelegationRecord {
        id: time_ordered_id(),
        selector: capability.selector().to_string(),
        secret_digest: capability.secret_digest(),
        space_id: space_id.to_string(),
        delegating_user_id: delegating_user_id.to_string(),
        agent_label: agent_label.to_string(),
        audience,
        created_at,
        expires_at,
        revoked_at: None,
        revoked_reason: None,
    };
    Ok(IssuedAgentDelegation { record, capability })
}

/// 🏷️ The longest agent label a delegation may carry — it rides the presence roster, whose
/// per-entry budget is the binding constraint.
pub const AGENT_LABEL_MAX_BYTES: usize = 64;

/// ⚖️ Why an agent-session exchange was refused, or that it may proceed. Every refusal that a
/// caller holding a *wrong* credential could observe collapses into `Denied`, exactly like the
/// invite preflight, so a delegation id cannot be probed for existence. `Revoked` and `Expired` are
/// only ever reported to a caller that already proved the matching secret.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentSessionPreflight {
    Mint,
    Revoked,
    Expired,
    Denied,
}

/// ⚖️ The whole agent-session law, pure and clock-injected: the record must exist, the
/// presented capability must match its selector AND its secret digest in constant time, the
/// delegation must not be revoked or expired, and the requested audience must be exactly the one
/// the delegation was scoped to — an agent can never widen its own scope at exchange time.
pub fn agent_session_preflight(record: Option<&AgentDelegationRecord>, capability: &AgentDelegationCapability, requested_audience: AgentAudience, now_ms: i64) -> AgentSessionPreflight {
    let Some(record) = record else { return AgentSessionPreflight::Denied };
    if record.selector != capability.selector() || !constant_time_digest_eq(&record.secret_digest, &capability.secret_digest()) {
        return AgentSessionPreflight::Denied;
    }
    if requested_audience != record.audience {
        return AgentSessionPreflight::Denied;
    }
    if record.revoked_at.is_some() {
        AgentSessionPreflight::Revoked
    } else if record.expires_at <= now_ms {
        AgentSessionPreflight::Expired
    } else {
        AgentSessionPreflight::Mint
    }
}

pub fn active_capability(selector: &str, stored_digest: &[u8; 32], expires_at: i64, revoked_at: Option<i64>, capability_selector: &str, candidate_digest: &[u8; 32], now: i64) -> bool {
    selector == capability_selector && revoked_at.is_none() && expires_at > now && constant_time_digest_eq(stored_digest, candidate_digest)
}

/// 🧭️ A verified stored scope selects serialization keys; it grants no redemption authority.
pub struct InviteRedemptionScopeHintV1 {
    pub space_id: String,
}

impl InviteRedemptionScopeHintV1 {
    pub fn space_id(&self) -> &str {
        &self.space_id
    }
}

/// 🔐️ Reveals a stored scope only for the exact capability secret and authenticated actor.
pub fn verify_invite_redemption_scope_hint(record: Option<&InviteRecord>, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str) -> DirectoryResult<InviteRedemptionScopeHintV1> {
    let record = record.ok_or(DirectoryError::Unauthorized)?;
    if actor_user_id(actor).ok() != Some(user_id) || record.selector != capability.selector() || !constant_time_digest_eq(&record.secret_digest, &capability.secret_digest()) {
        return Err(DirectoryError::Unauthorized);
    }
    Ok(InviteRedemptionScopeHintV1 { space_id: record.space_id.clone() })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InviteRedemptionSpaceStateV1 {
    Missing,
    Writable,
    Archived,
}

impl InviteRedemptionSpaceStateV1 {
    pub fn from_kind(kind: Option<&str>) -> DirectoryResult<Self> {
        match kind {
            None => Ok(Self::Missing),
            Some("studio" | "atelier") => Ok(Self::Writable),
            Some("archive") => Ok(Self::Archived),
            Some(_) => Err(DirectoryError::Backend("invite space kind is invalid".into())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InviteRedemptionPreflight {
    Claim,
    AlreadyCommitted,
    Revoked,
    Expired,
    Denied,
    Corrupt,
}

/// 🎟️ Applies the backend-independent invitation claim decision without accepting client scope or role authority.
pub fn invite_redemption_preflight(record: Option<&InviteRecord>, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str, user_exists: bool, space_state: InviteRedemptionSpaceStateV1, now_ms: i64) -> InviteRedemptionPreflight {
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
pub fn verify_invite_redemption_event(record: &InviteRecord, event: Option<DirectoryEvent>, authenticated_user_id: &str) -> DirectoryResult<DirectoryEvent> {
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

pub fn auth_audit(
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
pub fn validate_directory_command_claim(claim: &NewDirectoryCommandReceipt) -> DirectoryResult<()> {
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
pub fn validate_checkpoint_publication_claim(claim: &NewCheckpointPublicationClaimV1) -> DirectoryResult<()> {
    validate_bounded_auth_text(&claim.actor_user_id, "checkpoint publication actor", AUTH_TEXT_MAX_BYTES)?;
    if claim.correlation_id.len() != 32 || claim.correlation_id.bytes().all(|byte| byte == b'0') || !claim.correlation_id.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
        return Err(DirectoryError::Conflict("checkpoint publication correlation must be 32 lowercase nonzero hex digits".into()));
    }
    if claim.command_sha256.len() != 64 || claim.command_sha256.bytes().all(|byte| byte == b'0') || !claim.command_sha256.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
        return Err(DirectoryError::Conflict("checkpoint publication command digest must be 64 lowercase nonzero hex digits".into()));
    }
    Ok(())
}

pub fn validate_checkpoint_publication_completion(completion: &CheckpointPublicationCompletionV1) -> DirectoryResult<()> {
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
pub fn directory_command_result_kind_str(kind: DirectoryCommandResultKindV1) -> &'static str {
    match kind {
        DirectoryCommandResultKindV1::None => "none",
        DirectoryCommandResultKindV1::Invite => "invite",
    }
}

/// 🏷️ Reads back one stored command result class.
pub fn directory_command_result_kind_from_str(value: &str) -> DirectoryResult<DirectoryCommandResultKindV1> {
    match value {
        "none" => Ok(DirectoryCommandResultKindV1::None),
        "invite" => Ok(DirectoryCommandResultKindV1::Invite),
        other => Err(DirectoryError::Backend(format!("unknown command result kind '{other}'"))),
    }
}

pub fn validate_admin_operation_audit(fact: &NewAdminOperationAuditRecord) -> DirectoryResult<()> {
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

pub fn validate_admin_operation_effect_receipt(receipt: &AdminOperationEffectReceiptV1) -> DirectoryResult<()> {
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

pub fn admin_operation_effect_receipt_v1(claim: &NewAdminOperationEffectReceiptV1, events: &[DirectoryEvent]) -> DirectoryResult<AdminOperationEffectReceiptV1> {
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

pub fn same_admin_operation_request(existing: &NewAdminOperationAuditRecord, candidate: &NewAdminOperationAuditRecord) -> bool {
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
