//! 🪪️ `session_agent` — session, credential and agent delegation routes.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use directory::os_directory;
use directory::os_directory::{DirectorySessionAuthorityV1, DirectorySessionKindV1};
use futures::StreamExt;
use semio_framework_trace::record::TraceOutcome;
use semio_hub::artifact_authority::creation::ArtifactCreationCommitAuthorityV1;
use semio_hub::auth::rate_limit::{RateLimitClassV1, RateLimitDecisionV1, RateLimitSubjectV1};
use semio_hub::auth::password::PasswordCredentialV1;
use directory::os_directory::access_policy::DirectoryAccessActionV1;
use semio_hub::auth::agent::{decide_agent_session, AgentDelegationListV1, AgentDelegationReceiptV1, AgentDelegationSummaryV1, AgentErrorCodeV1, AgentErrorV1, AgentSessionDecisionV1, AgentSessionMintResponseV1, AgentSessionRequestV1, CreateAgentDelegationRequestV1};
use semio_hub::auth::{decide_credential_change, decide_credential_sign_in, AuthErrorCodeV1, AuthErrorV1, CredentialChangeDecisionV1, CredentialChangeRequestV1, CredentialSignInDecisionV1, CredentialSignInRequestV1, CredentialSubjectV1, SessionMintResponseV1};
use semio_hub::directory::model::{AgentDelegationRow, AuthSessionIssue, AuthSessionKind, AuthSessionRecord, CredentialAuditFactV1};
use semio_hub::directory::{HubDirectory, identity_subject_digest, AgentDelegationCapability, IdentityAssertionVerifier, SessionCapability};
use semio_hub::inference::runtime::GisMapApprovalIngressAuthorityV1;
use serde::Deserialize;
use crate::support::{now_ms, bearer, directory_error_status};
use crate::state::HubState;
use crate::socket_grants::{SocketBindingKeyV1, SocketBindingModeV1};
use crate::guard::{session_access_roles, access_permits_in_space};
use crate::access_grants::socket_text_bounded;
use crate::document_socket::DOCUMENT_SOCKET_FRAME_DEADLINE;
use crate::directory_commands::AuthedUser;
use crate::directory_events::directory_event_page_session_binding_v1;
use db::db_storage::PayloadStorage as _;

//#region 🔖️Directory
pub(crate) async fn get_session_me(headers: HeaderMap, State(state): State<HubState>) -> Result<Json<DirectorySessionAuthorityV1>, StatusCode> {
    let span = state.span("server.auth.session.read");
    let Some(token) = bearer(&headers) else {
        span.refused("no-bearer");
        return Err(StatusCode::UNAUTHORIZED);
    };
    let capability = match SessionCapability::parse(&token) {
        Ok(capability) => capability,
        Err(error) => {
            span.refused("malformed-capability");
            return Err(directory_error_status(error));
        }
    };
    let session = match state.directory.authenticate_session(&capability).await {
        Ok(Some(session)) => session,
        Ok(None) => {
            span.refused("unauthorized");
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(_) => {
            span.failed("directory-unavailable");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let span = span.principal(format!("user:{}", session.user_id));
    let user = match state.directory.get_user(&session.user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            span.refused("unknown-user");
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(_) => {
            span.failed("directory-unavailable");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let caller = AuthedUser { user_id: session.user_id, session_id: session.id, expires_at: session.expires_at, authorization_generation: session.authorization_generation, capability, session_kind: session.session_kind, device_instance_id: session.device_instance_id };
    let response = DirectorySessionAuthorityV1 {
        schema: "semio.directory.session-authority.v1".into(),
        session_binding_sha256: os_directory::io::binary::artifact_hash::hex_lower(&directory_event_page_session_binding_v1(&caller)?),
        authorization_generation: caller.authorization_generation,
        user_id: user.id,
        email: user.email,
        display_name: user.display_name,
        expires_at: caller.expires_at,
        session_kind: match session.session_kind {
            AuthSessionKind::External => DirectorySessionKindV1::External,
            AuthSessionKind::DevelopmentLocal => DirectorySessionKindV1::DevelopmentLocal,
            AuthSessionKind::Agent => DirectorySessionKindV1::Agent,
        },
    };
    if !response.validate() {
        span.failed("invalid-response");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    span.ok();
    Ok(Json(response))
}

/// 🚪️ `POST /auth/sessions/me/sign-out` — the self sign-out command. Revocation is a
/// directory fact, and the same revocation is applied to the server instance's live session set: a
/// key removed from one and left in the other is a key that still opens a door somewhere, so the two
/// move together or the caller learns the sign-out did not complete.
pub(crate) async fn post_session_sign_out(headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> StatusCode {
    let span = state.span("server.auth.session.revoke");
    if !body.is_empty() {
        span.refused("malformed-request");
        return StatusCode::BAD_REQUEST;
    }
    let Some(token) = bearer(&headers) else {
        span.refused("no-bearer");
        return StatusCode::UNAUTHORIZED;
    };
    let Ok(capability) = SessionCapability::parse(&token) else {
        span.refused("malformed-capability");
        return StatusCode::UNAUTHORIZED;
    };
    let Ok(Ok(Some(session))) = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&capability)).await else {
        span.refused("unauthorized");
        return StatusCode::UNAUTHORIZED;
    };
    let span = span.principal(format!("user:{}", session.user_id));
    let binding = SocketBindingKeyV1::Session(session.id.clone());
    let gate = state.socket_binding_gates.gate(binding.clone());
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.socket_session_revoke_attempted.add_permits(1);
    }
    let _admission = match tokio::time::timeout(std::time::Duration::from_secs(2), gate.write_owned()).await {
        Ok(admission) => admission,
        Err(_) => {
            span.failed("binding-gate-timeout");
            return StatusCode::SERVICE_UNAVAILABLE;
        }
    };
    match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.revoke_auth_session(&session.id, "self-revoked", Some(&session.user_id), &directory::os_identity::time_ordered_id())).await {
        Ok(Ok(Some(revoked))) => {
            debug_assert_eq!(revoked.id, session.id);
            if let Err(error) = state.forget_instance_session(&session.id).await {
                span.failed(&format!("instance-session-forget-{error:?}"));
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            state.socket_grants.invalidate_binding(binding.clone());
            state.document_open_plans.invalidate_binding(&binding);
            span.ok();
            StatusCode::NO_CONTENT
        }
        Ok(Ok(None)) => {
            span.refused("already-revoked");
            StatusCode::UNAUTHORIZED
        }
        Ok(Err(_)) => {
            span.failed("directory-unavailable");
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Err(_) => {
            span.failed("directory-timeout");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}

/// 🎫️ `POST /auth/sessions` — the hub's credential sign-in and session mint. The whole law
/// lives in `semio_hub::auth`: the body is decoded and bounds-checked
/// ([`CredentialSignInRequestV1::verify`]), the claimed identity gets its own rate-limit bucket on
/// top of the remote-address bucket [`rate_limit_middleware`] already charged, and
/// [`decide_credential_sign_in`] collapses "no such account", "no credential", "unreadable
/// credential" and "wrong password" into one `invalid-credentials`. Every attempt — admitted or
/// refused — appends a `credential-sign-in` fact before the response is written, so the
/// authentication log is the record of what happened. The minted session itself is
/// `issue_auth_session`'s own `session-issued` fact; nothing here writes a session row.
pub(crate) async fn post_auth_session(State(state): State<HubState>, body: Bytes) -> Response {
    let span = state.span("server.auth.session.mint");
    let Ok(request) = serde_json::from_slice::<CredentialSignInRequestV1>(&body) else {
        span.refused("malformed-request");
        return auth_error_response(AuthErrorCodeV1::MalformedRequest, None);
    };
    let verified = match request.verify() {
        Ok(verified) => verified,
        Err(code) => {
            span.refused(code.reason_code());
            return auth_error_response(code, None);
        }
    };
    if let RateLimitDecisionV1::Refused { retry_after_ms } = state.rate_limits.admit(RateLimitClassV1::Auth, &[RateLimitSubjectV1::claimed_identity(verified.email())]) {
        state.note("server.rate-limit", TraceOutcome::Refused, "auth-claimed-identity");
        span.refused("rate-limited");
        return auth_error_response(AuthErrorCodeV1::RateLimited, Some(retry_after_ms));
    }
    let correlation_id = directory::os_identity::time_ordered_id();
    let peer_class = verified.client_class().as_str();
    let subject = match state.directory.get_user_by_email(verified.email()).await {
        Ok(user) => user.map(|user| CredentialSubjectV1 { user_id: user.id, password_hash: user.password_hash }),
        Err(_) => {
            span.failed("directory-unavailable");
            return auth_error_response(AuthErrorCodeV1::DirectoryUnavailable, None);
        }
    };
    let span = match subject.as_ref() {
        Some(subject) => span.principal(format!("user:{}", subject.user_id)),
        None => span,
    };
    let policy = state.credential_sign_in;
    let decision = {
        let (verified, subject) = (verified.clone(), subject.clone());
        tokio::task::spawn_blocking(move || decide_credential_sign_in(policy, &verified, subject.as_ref())).await
    };
    let Ok(decision) = decision else {
        span.failed("credential-verification-interrupted");
        return auth_error_response(AuthErrorCodeV1::DirectoryUnavailable, None);
    };
    match decision {
        CredentialSignInDecisionV1::Refuse(code) => {
            journal_credential_sign_in(&state, subject.as_ref().map(|subject| subject.user_id.as_str()), "failure", Some(code.reason_code()), &correlation_id, peer_class).await;
            span.refused(code.reason_code());
            auth_error_response(code, None)
        }
        CredentialSignInDecisionV1::Mint { user_id, ttl_secs } => {
            let Ok(identity_subject_digest) = identity_subject_digest(semio_hub::auth::CREDENTIAL_IDENTITY_PROVIDER, verified.email()) else {
                span.refused("malformed-request");
                return auth_error_response(AuthErrorCodeV1::MalformedRequest, None);
            };
            let issue = AuthSessionIssue {
                user_id,
                identity_provider: semio_hub::auth::CREDENTIAL_IDENTITY_PROVIDER.into(),
                identity_subject_digest,
                ttl_secs,
                device_instance_id: verified.device_instance_id().to_string(),
                session_kind: AuthSessionKind::External,
                correlation_id: correlation_id.clone(),
                peer_class: peer_class.to_string(),
            };
            match state.directory.issue_auth_session(&issue).await {
                Ok(session) => {
                    if let Err(error) = state.record_instance_session(&session.record.id, &session.record.user_id, Some(&session.record.device_instance_id)).await {
                        journal_credential_sign_in(&state, Some(&session.record.user_id), "failure", Some(AuthErrorCodeV1::DirectoryUnavailable.reason_code()), &correlation_id, peer_class).await;
                        let _ = state.directory.revoke_auth_session(&session.record.id, "instance-session-store-unavailable", Some(&session.record.user_id), &correlation_id).await;
                        span.failed(&format!("instance-session-create-{error:?}"));
                        return auth_error_response(AuthErrorCodeV1::DirectoryUnavailable, None);
                    }
                    journal_credential_sign_in(&state, Some(&session.record.user_id), "success", None, &correlation_id, peer_class).await;
                    let minted = SessionMintResponseV1 { token: session.capability.expose_once(), user_id: session.record.user_id.clone() };
                    let mut response = Json(minted).into_response();
                    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
                    span.ok();
                    response
                }
                Err(_) => {
                    journal_credential_sign_in(&state, subject.as_ref().map(|subject| subject.user_id.as_str()), "failure", Some(AuthErrorCodeV1::DirectoryUnavailable.reason_code()), &correlation_id, peer_class).await;
                    span.failed("directory-unavailable");
                    auth_error_response(AuthErrorCodeV1::DirectoryUnavailable, None)
                }
            }
        }
    }
}

/// 🔁️ `POST /auth/credentials` — the principal's own password change. Two independent
/// proofs are required: a live bearer (which says *who*) and the current password (which says the
/// human is present, so a stolen capability alone can never take an account over). On success the
/// new credential and its `credential-changed` fact are written in one transaction and **every**
/// session of that principal is revoked — including this one, so the caller must sign in again
/// with the new password and any session the old password leaked is gone.
///
/// A principal that has no credential at all is refused exactly like a wrong password: the first
/// credential for a principal is issued by the operator verb `os-hub credential set`, which needs
/// filesystem access to `OS_HUB_DATA` and is therefore not reachable over the network at all.
pub(crate) async fn post_auth_credential(headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    let span = state.span("server.auth.credential.change");
    let Some(token) = bearer(&headers) else {
        span.refused("no-bearer");
        return auth_error_response(AuthErrorCodeV1::InvalidCredentials, None);
    };
    let Ok(capability) = SessionCapability::parse(&token) else {
        span.refused("malformed-capability");
        return auth_error_response(AuthErrorCodeV1::InvalidCredentials, None);
    };
    let Ok(request) = serde_json::from_slice::<CredentialChangeRequestV1>(&body) else {
        span.refused("malformed-request");
        return auth_error_response(AuthErrorCodeV1::MalformedRequest, None);
    };
    let verified = match request.verify() {
        Ok(verified) => verified,
        Err(code) => {
            span.refused(code.reason_code());
            return auth_error_response(code, None);
        }
    };
    let session = match state.directory.authenticate_session(&capability).await {
        Ok(Some(session)) => session,
        Ok(None) => {
            span.refused("invalid-credentials");
            return auth_error_response(AuthErrorCodeV1::InvalidCredentials, None);
        }
        Err(_) => {
            span.failed("directory-unavailable");
            return auth_error_response(AuthErrorCodeV1::DirectoryUnavailable, None);
        }
    };
    let span = span.principal(format!("user:{}", session.user_id));
    let correlation_id = directory::os_identity::time_ordered_id();
    let subject = match state.directory.get_user(&session.user_id).await {
        Ok(Some(user)) => CredentialSubjectV1 { user_id: user.id, password_hash: user.password_hash },
        Ok(None) => {
            span.refused("invalid-credentials");
            return auth_error_response(AuthErrorCodeV1::InvalidCredentials, None);
        }
        Err(_) => {
            span.failed("directory-unavailable");
            return auth_error_response(AuthErrorCodeV1::DirectoryUnavailable, None);
        }
    };
    let policy = state.credential_sign_in;
    let decision = {
        let (verified, subject) = (verified.clone(), subject.clone());
        tokio::task::spawn_blocking(move || match decide_credential_change(policy, &verified, &subject) {
            CredentialChangeDecisionV1::Apply { user_id, mint_iterations } => (CredentialChangeDecisionV1::Apply { user_id, mint_iterations }, Some(PasswordCredentialV1::mint(verified.new_password(), mint_iterations))),
            refused => (refused, None),
        })
        .await
    };
    let Ok((decision, minted)) = decision else {
        span.failed("credential-verification-interrupted");
        return auth_error_response(AuthErrorCodeV1::DirectoryUnavailable, None);
    };
    match decision {
        CredentialChangeDecisionV1::Refuse(code) => {
            journal_credential_change_refusal(&state, &subject.user_id, code.reason_code(), &correlation_id).await;
            span.refused(code.reason_code());
            auth_error_response(code, None)
        }
        CredentialChangeDecisionV1::Apply { user_id, .. } => {
            let Some(Ok(credential)) = minted else {
                span.refused("malformed-request");
                return auth_error_response(AuthErrorCodeV1::MalformedRequest, None);
            };
            if state.directory.set_password_credential(&user_id, &credential.encode(), Some(&user_id), &correlation_id).await.is_err() {
                span.failed("directory-unavailable");
                return auth_error_response(AuthErrorCodeV1::DirectoryUnavailable, None);
            }
            let revoked = match state.directory.revoke_auth_sessions_for_user(&user_id, "credential-changed", Some(&user_id), &correlation_id).await {
                Ok(revoked) => revoked,
                Err(_) => {
                    span.failed("directory-unavailable");
                    return auth_error_response(AuthErrorCodeV1::DirectoryUnavailable, None);
                }
            };
            if let Err(error) = state.revoke_instance_principal(&user_id).await {
                span.failed(&format!("instance-session-revoke-{error:?}"));
                return auth_error_response(AuthErrorCodeV1::DirectoryUnavailable, None);
            }
            for session in &revoked {
                let binding = SocketBindingKeyV1::Session(session.id.clone());
                state.socket_grants.invalidate_binding(binding.clone());
                state.document_open_plans.invalidate_binding(&binding);
            }
            let mut response = StatusCode::NO_CONTENT.into_response();
            response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
            span.finish(TraceOutcome::Ok, Some(format!("revoked-{}", revoked.len())));
            response
        }
    }
}

/// 🧾️ Appends one refused `credential-changed` attempt. A successful change journals its own
/// fact inside `set_password_credential`'s transaction, so only refusals are recorded here.
async fn journal_credential_change_refusal(state: &HubState, user_id: &str, reason_code: &str, correlation_id: &str) {
    let fact = CredentialAuditFactV1 {
        event_kind: semio_hub::auth::CREDENTIAL_CHANGED_EVENT.into(),
        target_user_id: Some(user_id.to_string()),
        actor_user_id: Some(user_id.to_string()),
        outcome_code: "failure".into(),
        reason_code: Some(reason_code.to_string()),
        correlation_id: correlation_id.to_string(),
        peer_class: "browser".into(),
    };
    let _ = state.directory.append_credential_audit(&fact).await;
}

/// 🧾️ Appends one `credential-sign-in` fact. A backend that cannot journal never turns a
/// refusal into an admission — the response was already decided before this is called.
async fn journal_credential_sign_in(state: &HubState, target_user_id: Option<&str>, outcome_code: &str, reason_code: Option<&str>, correlation_id: &str, peer_class: &str) {
    let fact = CredentialAuditFactV1 {
        event_kind: semio_hub::auth::CREDENTIAL_SIGN_IN_EVENT.into(),
        target_user_id: target_user_id.map(str::to_string),
        actor_user_id: None,
        outcome_code: outcome_code.to_string(),
        reason_code: reason_code.map(str::to_string),
        correlation_id: correlation_id.to_string(),
        peer_class: peer_class.to_string(),
    };
    let _ = state.directory.append_credential_audit(&fact).await;
}

/// 🛑️ The one shape every credential sign-in refusal is served in.
/// 🤖️ Every agent-delegation refusal is served exactly like a credential refusal: the code's
/// own status, `no-store`, and a `retry-after` header whenever the limiter is the reason.
fn agent_error_response(error: AgentErrorCodeV1, retry_after_ms: Option<u64>) -> Response {
    let status = StatusCode::from_u16(error.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let mut body = AgentErrorV1::new(error);
    body.retry_after_ms = retry_after_ms;
    let mut response = (status, Json(body)).into_response();
    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
    if let Some(retry_after_ms) = retry_after_ms {
        if let Ok(value) = axum::http::HeaderValue::from_str(&RateLimitDecisionV1::Refused { retry_after_ms }.retry_after_secs().to_string()) {
            response.headers_mut().insert(axum::http::header::RETRY_AFTER, value);
        }
    }
    response
}

/// 🤖️ Resolves the signed-in human behind a delegation-administration call: a live session
/// bearer that is NOT itself an agent session. An agent can never delegate onward, so the one
/// check that matters here is `session_kind`.
async fn delegating_principal(state: &HubState, headers: &HeaderMap) -> Result<AuthSessionRecord, AgentErrorCodeV1> {
    let token = bearer(headers).ok_or(AgentErrorCodeV1::InvalidDelegation)?;
    let capability = SessionCapability::parse(&token).map_err(|_| AgentErrorCodeV1::InvalidDelegation)?;
    let session = match state.directory.authenticate_session(&capability).await {
        Ok(Some(session)) => session,
        Ok(None) => return Err(AgentErrorCodeV1::InvalidDelegation),
        Err(_) => return Err(AgentErrorCodeV1::DirectoryUnavailable),
    };
    if session.session_kind.is_agent() {
        return Err(AgentErrorCodeV1::Forbidden);
    }
    Ok(session)
}

/// 🤖️ The delegation an agent session was minted from, resolved ONCE at socket admission so
/// the roster and the connection log can name the agent instead of the human whose authority it
/// borrows. `device_instance_id` carries the delegation id (`post_agent_session`), and a delegation
/// always belongs to the space and the delegating human its session names, so that human's own
/// listing for this space is the smallest read that answers it — no new directory method and no
/// second authentication. A delegation revoked between the grant and the socket simply has no row,
/// which the caller reads as an unnamed agent rather than as a person.
pub(crate) async fn agent_delegation_for_session(state: &HubState, space_id: &str, delegating_user_id: &str, delegation_id: &str) -> Option<AgentDelegationRow> {
    let rows = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.list_agent_delegations(space_id, delegating_user_id, semio_hub::directory::AGENT_DELEGATION_PAGE_MAX)).await.ok()?.ok()?;
    rows.into_iter().find(|row| row.delegation_id == delegation_id)
}

/// 🧾️ One agent-delegation fact, written through the same append the credential log uses.
async fn journal_agent_fact(state: &HubState, event_kind: &str, user_id: &str, outcome_code: &str, reason_code: Option<&str>, correlation_id: &str, peer_class: &str) {
    let fact = CredentialAuditFactV1 {
        event_kind: event_kind.to_string(),
        target_user_id: Some(user_id.to_string()),
        actor_user_id: Some(user_id.to_string()),
        outcome_code: outcome_code.to_string(),
        reason_code: reason_code.map(str::to_string),
        correlation_id: correlation_id.to_string(),
        peer_class: peer_class.to_string(),
    };
    let _ = state.directory.append_credential_audit(&fact).await;
}

/// 🤖️ `POST /auth/agent-delegations` — a signed-in human delegates a scoped, revocable
/// credential to an AI agent. The caller must be an **author** of the target space: a spectator
/// cannot manufacture an editing principal out of a read-only membership. The response carries the
/// token exactly once; nothing stores it, and no later route can read it back.
pub(crate) async fn post_agent_delegation(headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    let span = state.span("server.auth.agent.delegate");
    let session = match delegating_principal(&state, &headers).await {
        Ok(session) => session,
        Err(code) => {
            span.refused(code.as_str());
            return agent_error_response(code, None);
        }
    };
    let Ok(request) = serde_json::from_slice::<CreateAgentDelegationRequestV1>(&body) else {
        span.refused("malformed-request");
        return agent_error_response(AgentErrorCodeV1::MalformedRequest, None);
    };
    let verified = match request.verify() {
        Ok(verified) => verified,
        Err(code) => {
            span.refused(code.as_str());
            return agent_error_response(code, None);
        }
    };
    if let RateLimitDecisionV1::Refused { retry_after_ms } = state.rate_limits.admit(RateLimitClassV1::Auth, &[RateLimitSubjectV1::claimed_identity(&session.user_id)]) {
        span.refused("rate-limited");
        return agent_error_response(AgentErrorCodeV1::RateLimited, Some(retry_after_ms));
    }
    match state.directory.principal_role(verified.space_id(), session.principal()).await {
        Ok(role) if access_permits_in_space(&state, &session_access_roles(session.session_kind, role), DirectoryAccessActionV1::AgentDelegate, verified.space_id()).await => {}
        Ok(_) => {
            span.refused("forbidden");
            return agent_error_response(AgentErrorCodeV1::Forbidden, None);
        }
        Err(_) => {
            span.failed("directory-unavailable");
            return agent_error_response(AgentErrorCodeV1::DirectoryUnavailable, None);
        }
    }
    let correlation_id = directory::os_identity::time_ordered_id();
    let Ok(issued) = semio_hub::directory::prepare_agent_delegation(verified.space_id(), &session.user_id, verified.agent_label(), verified.audience(), verified.ttl_secs(), now_ms()) else {
        span.refused("malformed-request");
        return agent_error_response(AgentErrorCodeV1::MalformedRequest, None);
    };
    if state.directory.create_agent_delegation(&issued, &correlation_id, "server").await.is_err() {
        journal_agent_fact(&state, semio_hub::directory::AGENT_DELEGATED_EVENT, &session.user_id, "failure", Some(AgentErrorCodeV1::DirectoryUnavailable.as_str()), &correlation_id, "server").await;
        span.failed("directory-unavailable");
        return agent_error_response(AgentErrorCodeV1::DirectoryUnavailable, None);
    }
    let receipt = AgentDelegationReceiptV1 {
        schema: semio_hub::auth::agent::AGENT_DELEGATION_RECEIPT_SCHEMA.into(),
        agent_principal_id: semio_hub::auth::agent::agent_principal_id(&issued.record.id),
        delegation_id: issued.record.id.clone(),
        agent_label: issued.record.agent_label.clone(),
        space_id: issued.record.space_id.clone(),
        audience: issued.record.audience,
        expires_at_ms: issued.record.expires_at,
        token: issued.capability.expose_once(),
    };
    let mut response = (StatusCode::CREATED, Json(receipt)).into_response();
    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
    span.ok();
    response
}

/// 🤖️ `GET /auth/agent-delegations?space=<id>` — every delegation this human created in this
/// space, revoked ones included so the UI can show what was withdrawn. Never a token, never a
/// selector, and never another human's delegations.
pub(crate) async fn get_agent_delegations(headers: HeaderMap, Query(query): Query<AgentDelegationQueryV1>, State(state): State<HubState>) -> Response {
    let span = state.span("server.auth.agent.list");
    let session = match delegating_principal(&state, &headers).await {
        Ok(session) => session,
        Err(code) => {
            span.refused(code.as_str());
            return agent_error_response(code, None);
        }
    };
    if !socket_text_bounded(&query.space) {
        span.refused("malformed-request");
        return agent_error_response(AgentErrorCodeV1::MalformedRequest, None);
    }
    match state.directory.list_agent_delegations(&query.space, &session.user_id, semio_hub::directory::AGENT_DELEGATION_PAGE_MAX).await {
        Ok(rows) => {
            let body = AgentDelegationListV1::new(rows.into_iter().map(AgentDelegationSummaryV1::from_row).collect());
            let mut response = Json(body).into_response();
            response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
            span.ok();
            response
        }
        Err(_) => {
            span.failed("directory-unavailable");
            agent_error_response(AgentErrorCodeV1::DirectoryUnavailable, None)
        }
    }
}

/// 🤖️ `POST /auth/agent-delegations/{id}/revoke` — the withdrawal command. The delegation
/// row and its `agent-delegation-revoked` fact are written in one transaction, and every live agent session
/// minted from it is revoked with the same `authorization_generation` bump an ordinary sign-out
/// performs — so open socket grants and document plans die with it and the agent's next frame is
/// closed `4401`. A delegation that is not this human's is `403`, never `404`.
///
/// The withdrawal is linearized against the agent's own work: once the revocation is durable, the
/// route holds every revoked session's binding exclusively before it answers. A frame the agent had
/// already admitted (holding those bindings shared) finishes first — its edit precedes the
/// withdrawal —, and every frame after it re-reads the revoked session and is refused. So `204` means
/// no further edit of that agent can be accepted and its sockets are closing (ticket 26/09/23 G10 S4:
/// on 7800 a connected agent was refused in one run and still edited in the other).
pub(crate) async fn post_agent_delegation_revoke(Path(delegation_id): Path<String>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    let span = state.span("server.auth.agent.revoke");
    if !body.is_empty() {
        span.refused("malformed-request");
        return agent_error_response(AgentErrorCodeV1::MalformedRequest, None);
    }
    let session = match delegating_principal(&state, &headers).await {
        Ok(session) => session,
        Err(code) => {
            span.refused(code.as_str());
            return agent_error_response(code, None);
        }
    };
    if !socket_text_bounded(&delegation_id) {
        span.refused("malformed-request");
        return agent_error_response(AgentErrorCodeV1::MalformedRequest, None);
    }
    let correlation_id = directory::os_identity::time_ordered_id();
    match state.directory.revoke_agent_delegation(&delegation_id, &session.user_id, "delegation-revoked-by-owner", &correlation_id, now_ms()).await {
        Ok(Some(revoked)) => {
            let bindings: Vec<SocketBindingKeyV1> = revoked.iter().map(|entry| SocketBindingKeyV1::Session(entry.id.clone())).collect();
            let fence = tokio::time::timeout(DOCUMENT_SOCKET_FRAME_DEADLINE, state.socket_binding_gates.acquire(bindings.iter().cloned().map(|binding| (binding, SocketBindingModeV1::Exclusive)).collect())).await;
            let mut forgotten = Ok(());
            for (entry, binding) in revoked.iter().zip(bindings) {
                state.socket_grants.invalidate_binding(binding.clone());
                state.document_open_plans.invalidate_binding(&binding);
                if let Err(error) = state.forget_instance_session(&entry.id).await {
                    forgotten = Err(error);
                }
            }
            drop(fence);
            if let Err(error) = forgotten {
                span.failed(&format!("instance-session-forget-{error:?}"));
                return agent_error_response(AgentErrorCodeV1::DirectoryUnavailable, None);
            }
            span.ok();
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(None) => {
            span.refused("forbidden");
            agent_error_response(AgentErrorCodeV1::Forbidden, None)
        }
        Err(_) => {
            span.failed("directory-unavailable");
            agent_error_response(AgentErrorCodeV1::DirectoryUnavailable, None)
        }
    }
}

/// 🤖️ `POST /auth/agent-sessions` — the agent exchanges its delegation for a session. The
/// bearer is the delegation capability, not a password: nothing here ever sees a human secret.
///
/// The minted session is an ordinary `AuthSessionRecord` so every existing hub route authenticates
/// the agent with no special case; what differs is `session_kind = agent`, which is what makes the
/// agent its own principal at the presence layer, in the actor string on every edit, and in
/// per-actor undo. `device_instance_id` carries the delegation id, which is how revoking the
/// delegation finds and kills exactly the sessions it minted.
pub(crate) async fn post_agent_session(headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    let span = state.span("server.auth.agent.session");
    let Some(token) = bearer(&headers) else {
        span.refused("invalid-delegation");
        return agent_error_response(AgentErrorCodeV1::InvalidDelegation, None);
    };
    let Ok(capability) = AgentDelegationCapability::parse(&token) else {
        span.refused("invalid-delegation");
        return agent_error_response(AgentErrorCodeV1::InvalidDelegation, None);
    };
    let Ok(request) = serde_json::from_slice::<AgentSessionRequestV1>(&body) else {
        span.refused("malformed-request");
        return agent_error_response(AgentErrorCodeV1::MalformedRequest, None);
    };
    if let Err(code) = request.verify() {
        span.refused(code.as_str());
        return agent_error_response(code, None);
    }
    if let RateLimitDecisionV1::Refused { retry_after_ms } = state.rate_limits.admit(RateLimitClassV1::Auth, &[RateLimitSubjectV1::claimed_identity(capability.selector())]) {
        span.refused("rate-limited");
        return agent_error_response(AgentErrorCodeV1::RateLimited, Some(retry_after_ms));
    }
    let record = match state.directory.load_agent_delegation(capability.selector()).await {
        Ok(record) => record,
        Err(_) => {
            span.failed("directory-unavailable");
            return agent_error_response(AgentErrorCodeV1::DirectoryUnavailable, None);
        }
    };
    let preflight = semio_hub::directory::agent_session_preflight(record.as_ref(), &capability, request.audience, now_ms());
    let ttl_secs = match decide_agent_session(preflight) {
        AgentSessionDecisionV1::Mint { ttl_secs } => ttl_secs,
        AgentSessionDecisionV1::Refuse(code) => {
            span.refused(code.as_str());
            return agent_error_response(code, None);
        }
    };
    let Some(record) = record else {
        span.refused("invalid-delegation");
        return agent_error_response(AgentErrorCodeV1::InvalidDelegation, None);
    };
    let correlation_id = directory::os_identity::time_ordered_id();
    let Ok(identity_subject_digest) = identity_subject_digest(semio_hub::directory::AGENT_IDENTITY_PROVIDER, &record.id) else {
        span.refused("malformed-request");
        return agent_error_response(AgentErrorCodeV1::MalformedRequest, None);
    };
    let issue = AuthSessionIssue {
        user_id: record.delegating_user_id.clone(),
        identity_provider: semio_hub::directory::AGENT_IDENTITY_PROVIDER.into(),
        identity_subject_digest,
        ttl_secs,
        device_instance_id: record.id.clone(),
        session_kind: AuthSessionKind::Agent,
        correlation_id: correlation_id.clone(),
        peer_class: "agent".into(),
    };
    match state.directory.issue_auth_session(&issue).await {
        Ok(session) => {
            journal_agent_fact(&state, semio_hub::directory::AGENT_SESSION_ISSUED_EVENT, &record.delegating_user_id, "success", Some(record.audience.as_str()), &correlation_id, "agent").await;
            let minted = AgentSessionMintResponseV1 {
                schema: semio_hub::auth::agent::AGENT_SESSION_REQUEST_SCHEMA.into(),
                token: session.capability.expose_once(),
                agent_principal_id: semio_hub::auth::agent::agent_principal_id(&record.id),
                agent_label: record.agent_label.clone(),
                space_id: record.space_id.clone(),
                audience: record.audience,
                expires_at_ms: session.record.expires_at,
            };
            let mut response = Json(minted).into_response();
            response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
            span.ok();
            response
        }
        Err(_) => {
            journal_agent_fact(&state, semio_hub::directory::AGENT_SESSION_ISSUED_EVENT, &record.delegating_user_id, "failure", Some(AgentErrorCodeV1::DirectoryUnavailable.as_str()), &correlation_id, "agent").await;
            span.failed("directory-unavailable");
            agent_error_response(AgentErrorCodeV1::DirectoryUnavailable, None)
        }
    }
}

/// 🔎️ `?space=<id>` — the one query parameter the delegation listing takes.
#[derive(Deserialize)]
pub(crate) struct AgentDelegationQueryV1 {
    pub(crate) space: String,
}

pub(crate) fn auth_error_response(error: AuthErrorCodeV1, retry_after_ms: Option<u64>) -> Response {
    let status = StatusCode::from_u16(error.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let body = match retry_after_ms {
        Some(retry_after_ms) => AuthErrorV1::rate_limited(retry_after_ms),
        None => AuthErrorV1::new(error),
    };
    let mut response = (status, Json(body)).into_response();
    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
    if let Some(retry_after_ms) = retry_after_ms {
        if let Ok(value) = axum::http::HeaderValue::from_str(&RateLimitDecisionV1::Refused { retry_after_ms }.retry_after_secs().to_string()) {
            response.headers_mut().insert(axum::http::header::RETRY_AFTER, value);
        }
    }
    response
}
//#endregion 🔖️Directory
