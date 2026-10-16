//! 🛣️ The `guard` handlers that act over `HubState`.

use semio_hub_ledgers::guard::{AdminPrincipalV1, AuthOutcome, admin_provider_digest, exact_admin_session_bearer};
use directory::os_directory::access_policy::{DirectoryAccessActionV1, DirectoryAccessRoleV1};
use directory::os_directory::DocumentScope;
use axum::http::{HeaderMap, StatusCode};
use semio_hub_directory_contract::HubCapability;
use semio_hub_state::HubState;
use std::net::SocketAddr;
use directory::os_directory::io::text::access_policy::directory_access_permits;
use semio_hub_runtime::support::now_ms;
use semio_hub_directory::{AuthSessionDirectory, PrincipalAuthority, ShareTokenDirectory, SpaceDirectory};

//#region 🔖️Auth
/// 🔐️ Tries the bearer as an `AuthSessionRecord` (session id -> user -> space role) first;
/// falls back to an active exact-space/document share grant when session resolution fails.
async fn resolve_auth(state: &HubState, space_id: &str, document_id: &str, token: Option<&str>) -> AuthOutcome {
    let capability = token.and_then(|value| HubCapability::parse(value).ok());
    if let Some(HubCapability::Session(capability)) = &capability {
        if let Ok(Some(session)) = state.directory.authenticate_session(capability).await {
            if let Ok(Some(role)) = state.directory.principal_role(space_id, session.principal()).await {
                return AuthOutcome::Session { user_id: session.user_id, role, session_id: session.id, authorization_generation: session.authorization_generation, session_kind: session.session_kind };
            }
        }
    }
    let scope = DocumentScope::new(space_id, document_id);
    if let Some(HubCapability::Share(capability)) = &capability {
        if let Ok(true) = state.directory.authenticate_share(&scope, capability).await {
            return AuthOutcome::ShareToken;
        }
    }
    AuthOutcome::Denied
}

/// 🛡️ The declared policy's decision for `roles` doing `action` inside one space. The space's kind is
/// read here because kind-limited grants (an archive denies writes) must see it; a space that cannot
/// be read is a refusal, never an unconstrained allow.
pub async fn access_permits_in_space(state: &HubState, roles: &[DirectoryAccessRoleV1], action: DirectoryAccessActionV1, space_id: &str) -> bool {
    match state.directory.get_space(space_id).await {
        Ok(Some(space)) => directory_access_permits(roles, action, Some(space.kind.as_str())),
        Ok(None) | Err(_) => false,
    }
}

pub async fn authorized(state: &HubState, space_id: &str, document_id: &str, token: Option<&str>) -> bool {
    access_permits_in_space(state, &resolve_auth(state, space_id, document_id, token).await.access_roles(), DirectoryAccessActionV1::DocumentRead, space_id).await
}

/// 📦️ A space-scoped blob read or write under the declared policy: a share never reaches the
/// space's content-addressed store, and only a writer may add to it.
pub async fn authorized_for_blob(state: &HubState, space_id: &str, hash: &str, token: Option<&str>, action: DirectoryAccessActionV1) -> bool {
    access_permits_in_space(state, &resolve_auth(state, space_id, hash, token).await.access_roles(), action, space_id).await
}

pub async fn authorized_for_canonical_pair(state: &HubState, scope: &DocumentScope, token: &str) -> bool {
    #[cfg(test)]
    if state.canonical_pair_authorization_gate.as_ref().is_some_and(|gate| !gate()) {
        return false;
    }
    authorized(state, &scope.space_id, &scope.document_id, Some(token)).await
}
//#endregion 🔖️Auth

//#region 🔖️AdminAuth
/// 🛡️ Resolves one request-owned verified administrator principal from a live durable session.
pub async fn authenticate_admin_principal(state: &HubState, headers: &HeaderMap, _peer: Option<SocketAddr>) -> Result<AdminPrincipalV1, StatusCode> {
    let capability = exact_admin_session_bearer(headers)?;
    let session =
        tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&capability)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let provider_digest = admin_provider_digest(&session.identity_provider);
    if session.authorization_generation == 0
        || session.expires_at <= now_ms()
        || session.session_kind.is_agent()
        || !state
            .admin_subjects
            .iter()
            .any(|subject| semio_hub_directory_contract::constant_time_digest_eq(&subject.provider_digest, &provider_digest) && semio_hub_directory_contract::constant_time_digest_eq(&subject.subject_digest, &session.identity_subject_digest))
    {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(AdminPrincipalV1 {
        user_id: session.user_id,
        auth_session_id: session.id,
        authorization_generation: session.authorization_generation,
        identity_provider: session.identity_provider,
        identity_subject_digest: session.identity_subject_digest,
        expires_at_ms: session.expires_at,
        correlation_id: directory::os_identity::time_ordered_id(),
        peer_class: "admin-rest",
        session_kind: session.session_kind,
        device_instance_id: session.device_instance_id,
    })
}

pub(crate) async fn is_admin(state: &HubState, headers: &HeaderMap, peer: Option<SocketAddr>) -> bool {
    authenticate_admin_principal(state, headers, peer).await.is_ok()
}
//#endregion 🔖️AdminAuth
