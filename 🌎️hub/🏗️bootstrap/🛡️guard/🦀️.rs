//! 🛡️ `guard` — caller authentication and administrator authorization.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use directory::os_directory::io::text::access_policy::directory_access_permits;
use axum::http::{HeaderMap, StatusCode};
use directory::os_directory::{DirectoryActor, DirectoryActorKind};
use futures::StreamExt;
use semio_framework_hash::Sha256;
use directory::os_directory::access_policy::{DirectoryAccessActionV1, DirectoryAccessRoleV1};
use semio_hub_directory_contract::model::{AuthSessionKind, DirectoryPrincipalV1, SpaceRole};
use directory::os_directory::DocumentScope;
use semio_hub_directory_contract::{identity_subject_digest, IdentityAssertionVerifier, LocalBootstrapTransport, SessionCapability, AUTH_TEXT_MAX_BYTES};
use semio_hub_inference_runtime_contract::GisMapApprovalIngressAuthorityV1;
use std::net::SocketAddr;
use std::sync::Arc;
use semio_hub_runtime::startup::HubError;
use semio_hub_runtime::support::now_ms;
use semio_hub_runtime::readiness::HubMode;
use semio_hub_runtime::transport::{ForwardedTlsTrustV1, CrossOriginPolicyV1};
use db::storage::PayloadStorage as _;

//#region 🔖️Auth
/// 🔎️ What a bearer token resolved to for document authority: an authenticated space
/// member, an exact document share, or nothing. Public discovery never enters this boundary.
pub enum AuthOutcome {
    Session { user_id: String, role: SpaceRole, session_id: String, authorization_generation: u64, session_kind: AuthSessionKind },
    ShareToken,
    Denied,
}

/// 🎭️ THE declared-policy roles of one authenticated session holding `role` in a space (its ceiling-capped role,
/// [`PrincipalAuthority::principal_role`]): a human session is signed in plus its membership role; an agent session holds
/// only its agent role — `agent-editor` under an author ceiling, `agent-reader` under a spectator one — so no grant the
/// policy writes for humans (space creation, administration, delegation, preferences) ever reaches an agent.
pub fn session_access_roles(session_kind: AuthSessionKind, role: Option<SpaceRole>) -> Vec<DirectoryAccessRoleV1> {
    if session_kind.is_agent() {
        return role
            .map(|role| match role {
                SpaceRole::Author => DirectoryAccessRoleV1::AgentEditor,
                SpaceRole::Spectator => DirectoryAccessRoleV1::AgentReader,
            })
            .into_iter()
            .collect();
    }
    [DirectoryAccessRoleV1::Authenticated]
        .into_iter()
        .chain(role.map(|role| match role {
            SpaceRole::Author => DirectoryAccessRoleV1::Author,
            SpaceRole::Spectator => DirectoryAccessRoleV1::Spectator,
        }))
        .collect()
}

impl AuthOutcome {
    /// 🎭️ The roles this bearer holds for its document: a session holds [`session_access_roles`], a share token is a
    /// share, and a refused bearer holds nothing.
    pub fn access_roles(&self) -> Vec<DirectoryAccessRoleV1> {
        match self {
            Self::Session { role, session_kind, .. } => session_access_roles(*session_kind, Some(*role)),
            Self::ShareToken => vec![DirectoryAccessRoleV1::Share],
            Self::Denied => Vec::new(),
        }
    }
}

//#endregion 🔖️Auth

//#region 🔖️AdminAuth
#[derive(Clone)]
pub struct AdminSubject {
    pub provider_digest: [u8; 32],
    pub subject_digest: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdminPrincipalV1 {
    pub user_id: String,
    pub auth_session_id: String,
    pub authorization_generation: u64,
    pub identity_provider: String,
    pub identity_subject_digest: [u8; 32],
    pub expires_at_ms: i64,
    pub correlation_id: String,
    pub peer_class: &'static str,
    pub session_kind: AuthSessionKind,
    pub device_instance_id: String,
}

impl AdminPrincipalV1 {
    /// 🎚️ Who this administrator asks a space-role question as.
    pub fn principal(&self) -> DirectoryPrincipalV1<'_> {
        DirectoryPrincipalV1 { user_id: &self.user_id, session_kind: self.session_kind, device_instance_id: &self.device_instance_id }
    }

    pub fn event_actor(&self) -> DirectoryActor {
        DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#admin-session:{}", self.user_id, self.auth_session_id) }
    }

    pub fn same_authority(&self, other: &Self) -> bool {
        self.user_id == other.user_id
            && self.auth_session_id == other.auth_session_id
            && self.authorization_generation == other.authorization_generation
            && self.identity_provider == other.identity_provider
            && semio_hub_directory_contract::constant_time_digest_eq(&self.identity_subject_digest, &other.identity_subject_digest)
            && other.expires_at_ms > now_ms()
    }
}

pub fn admin_provider_digest(provider: &str) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/admin-provider/v1\0");
    hash.update(&(provider.len() as u32).to_be_bytes());
    hash.update(provider.as_bytes());
    hash.finalize()
}

pub fn configured_admin_subjects() -> Result<Arc<[AdminSubject]>, HubError> {
    const ADMIN_SUBJECTS_MAX: usize = 64;
    let Some(encoded) = std::env::var("OS_HUB_ADMIN_SUBJECTS").ok().filter(|value| !value.is_empty()) else { return Ok(Arc::from([])) };
    let mut subjects = Vec::new();
    for entry in encoded.split(',') {
        if subjects.len() == ADMIN_SUBJECTS_MAX {
            return Err(HubError::UnsafeAuthConfiguration("OS_HUB_ADMIN_SUBJECTS exceeds 64 entries".into()));
        }
        let (provider, subject) = entry.split_once(':').ok_or_else(|| HubError::UnsafeAuthConfiguration("OS_HUB_ADMIN_SUBJECTS entries must be provider:subject".into()))?;
        let subject_digest = identity_subject_digest(provider, subject).map_err(HubError::Directory)?;
        let provider_digest = admin_provider_digest(provider);
        if subjects.iter().any(|existing: &AdminSubject| semio_hub_directory_contract::constant_time_digest_eq(&existing.provider_digest, &provider_digest) && semio_hub_directory_contract::constant_time_digest_eq(&existing.subject_digest, &subject_digest)) {
            return Err(HubError::UnsafeAuthConfiguration("OS_HUB_ADMIN_SUBJECTS contains a duplicate identity".into()));
        }
        subjects.push(AdminSubject { provider_digest, subject_digest });
    }
    Ok(subjects.into())
}

/// 🛂️ Every configuration a hub refuses to boot with, decided before a single store is opened.
///
/// **Production used to be unreachable.** Its only identity requirement was an external
/// `IdentityAssertionVerifier` adapter, of which this repository has none and `main` passed a
/// hardcoded `None` — so `OS_HUB_MODE=production` always died on the first check. That was never the
/// real rule: what production needs is *an identity authority*, and the hub grew its own when AU1/AU3
/// built password sign-in (`POST /auth/sessions` → `decide_credential_sign_in` → `issue_auth_session`)
/// and the `os-hub credential set` operator verb that seeds the first user without any launcher.
/// So the gate now accepts either authority, and deliberately does **not** wrap the credential path
/// in a second `IdentityAssertionVerifier` implementation: that trait is the external-IdP seam, and
/// re-expressing `decide_credential_sign_in` behind it would be a second declaration site of the one
/// decision that matters.
///
/// **A network bind is now reachable too, and only under three explicit statements together**: the
/// mode is production, the cross-origin policy is an operator-named [`CrossOriginPolicyV1::Allowlist`]
/// (not the loopback-development default and not `closed`, which would admit no browser at all), and
/// [`ForwardedTlsTrustV1::TerminatingProxy`] says TLS is terminated in front. Any one of them missing
/// is a refusal that names it. Development is unchanged: loopback, and the inherited fd-3 transport.
pub fn validate_auth_startup(
    mode: HubMode,
    bind: std::net::IpAddr,
    verifier: Option<&Arc<dyn IdentityAssertionVerifier>>,
    local_bootstrap: Option<&Arc<dyn LocalBootstrapTransport>>,
    admin_subjects: &[AdminSubject],
    credential_sign_in_enabled: bool,
    cross_origin: &CrossOriginPolicyV1,
    forwarded_tls: ForwardedTlsTrustV1,
) -> Result<(), HubError> {
    match mode {
        HubMode::Production => {
            if verifier.is_none() && !credential_sign_in_enabled {
                return Err(HubError::UnsafeAuthConfiguration("production requires an identity authority: set OS_HUB_CREDENTIAL_SIGN_IN=true to use the hub's own credentials, or configure an IdentityAssertionVerifier adapter".into()));
            }
            if admin_subjects.is_empty() {
                return Err(HubError::UnsafeAuthConfiguration("production requires OS_HUB_ADMIN_SUBJECTS".into()));
            }
            if !bind.is_loopback() {
                if !matches!(cross_origin, CrossOriginPolicyV1::Allowlist(_)) {
                    return Err(HubError::UnsafeAuthConfiguration("a production hub on a network interface requires OS_HUB_ALLOWED_ORIGINS to name the origins its browsers are served from".into()));
                }
                if forwarded_tls != ForwardedTlsTrustV1::TerminatingProxy {
                    return Err(HubError::UnsafeAuthConfiguration("a production hub on a network interface speaks cleartext HTTP and requires OS_HUB_TRUSTED_FORWARDING=proxy, which states that a TLS-terminating reverse proxy is the only thing that can reach this socket".into()));
                }
            }
        }
        HubMode::Development => {
            if !bind.is_loopback() {
                return Err(HubError::UnsafeAuthConfiguration("development mode must bind to loopback".into()));
            }
            if local_bootstrap.is_none() {
                return Err(HubError::UnsafeAuthConfiguration("development requires a protected LocalBootstrapTransport adapter".into()));
            }
        }
    }
    Ok(())
}

pub fn exact_admin_session_bearer(headers: &HeaderMap) -> Result<SessionCapability, StatusCode> {
    let mut values = headers.get_all(axum::http::header::AUTHORIZATION).iter();
    let value = values.next().ok_or(StatusCode::UNAUTHORIZED)?;
    if values.next().is_some() || value.as_bytes().len() > "Bearer ".len() + AUTH_TEXT_MAX_BYTES {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let encoded = value.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    if encoded.is_empty() || encoded.len() > AUTH_TEXT_MAX_BYTES || encoded.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    SessionCapability::parse(encoded).map_err(|_| StatusCode::UNAUTHORIZED)
}

//#endregion 🔖️AdminAuth
