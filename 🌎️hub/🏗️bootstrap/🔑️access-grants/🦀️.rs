//! 🔑️ `access_grants` — document and directory socket grants, open plans and execution-target leases.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::body::Bytes;
use axum::extract::{OriginalUri, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use directory::os_directory;
use directory::os_directory::{DirectoryActor, DirectoryCommand, DirectoryEvent, DocumentExecutionTargetComponentV1, DocumentExecutionTargetDescriptorV1, DocumentExecutionTargetLeaseFieldsV1, DocumentOpenCatalogV1, DocumentOpenCheckpointV1, DocumentOpenIntentV1, DocumentOpenParentDialectV1, DocumentOpenPlanErrorCodeV1, DocumentOpenPlanErrorV1, DocumentOpenPlanV1, DocumentOpenRevalidationV1, DocumentPlanSocketGrantIntentV1, PublishedArtifactCheckpoint, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, DOCUMENT_OPEN_MAX_SAFE_INTEGER, DOCUMENT_OPEN_PLAN_MAX_TTL_MS};
use directory::os_directory::io::binary::descriptor_digest::descriptor_digest_v1;
use futures::StreamExt;
use semio_framework_hash::Sha256;
use semio_framework_trace::record::TraceOutcome;
use semio_hub::artifact_authority::creation::ArtifactCreationCommitAuthorityV1;
use semio_hub::artifact_authority::trusted_catalog::{TrustedCatalogAsset, VerifiedExecutionTargetAssets};
use semio_hub::auth::rate_limit::{RateLimitSubjectV1, StreamLimitClassV1, StreamPermitV1};
use semio_hub::directory::model::{DocumentScope, SocketSessionBindingStatus};
use semio_hub::directory::{HubDirectory, HubCapability, LocalBootstrapTransport, SessionCapability, SocketGrantCapability, AUTH_TEXT_MAX_BYTES};
use semio_hub::inference::runtime::GisMapApprovalIngressAuthorityV1;
use serde::Serialize;
use crate::startup::DocumentOpenCatalogAuthorityV1;
use crate::support::{DirectoryJson, now_ms, directory_error_status};
use crate::state::HubState;
use crate::socket_grants::{SOCKET_GRANT_TTL_MS, SOCKET_PROTOCOL_V1, SocketAudienceV1, SocketSubjectV1, SocketBindingValidityV1, SocketGrantLedgerErrorV1};
#[cfg(test)]
use crate::socket_grants::{SESSION_PROTOCOL_V1, SocketGrantRecordV1};
#[cfg(test)]
use crate::presence::{db_artifact_id, document_scope_key_v1};
use crate::open_plan::DocumentOpenPlanAuthorityV1;
use crate::transport::rate_limit_refusal_response;
#[cfg(test)]
use crate::directory_commands::{FencedDirectoryCommandErrorV1, acquire_directory_command_fence, invalidate_directory_event_authority};
use db::db_storage::PayloadStorage as _;

//#region 🔖️State
pub(crate) const DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES: usize = 8 * 1024;

const DOCUMENT_OPEN_PLAN_DEADLINE_MS: u64 = 10_000;

pub(crate) const DOCUMENT_OPEN_PLAN_EXCHANGE_REQUEST_MAX_BYTES: usize = 8 * 1024;
//#endregion 🔖️State

//#region 🔖️Rest
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocketGrantReceiptV1 {
    pub(crate) schema: &'static str,
    pub(crate) protocol: &'static str,
    pub(crate) grant: String,
    pub(crate) actor_id: String,
    pub(crate) expires_at_ms: i64,
}

/// 📝️ Schema of [`DocumentSocketGrantReceiptV1`] — `os.directory#/$defs/DocumentSocketGrantReceiptV1`.
pub(crate) const DOCUMENT_SOCKET_GRANT_SCHEMA_V1: &str = "semio.hub.document-socket-grant/v1";

/// 📝️ The answer to a document open-plan exchange: the actor the next `semio.session.v1`
/// upgrade of the same credential is admitted as, and until when. It carries no secret — the
/// credential itself is what the upgrade presents.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DocumentSocketGrantReceiptV1 {
    pub(crate) schema: &'static str,
    pub(crate) protocol: &'static str,
    pub(crate) actor_id: String,
    pub(crate) expires_at_ms: i64,
}

pub(crate) fn socket_issue_bearer(headers: &HeaderMap) -> Result<String, StatusCode> {
    let values = headers.get_all(axum::http::header::AUTHORIZATION);
    if values.iter().count() != 1 {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let encoded = values.iter().next().and_then(|value| value.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    if encoded.len() > AUTH_TEXT_MAX_BYTES {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let capability = encoded.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    if capability.is_empty() || capability.len() > AUTH_TEXT_MAX_BYTES {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(capability.to_string())
}

/// 🙋️ The hub actor id bound to one credential: a session or a share token hashes to the same
/// actor on every open-plan, socket grant and socket upgrade, so no caller ever names its actor.
pub(crate) fn socket_actor_id(material: &[u8; 32], stable_session: bool) -> String {
    let mut digest = Sha256::new();
    digest.update(b"semio/hub/socket/actor/v1\0");
    digest.update(if stable_session { b"session".as_slice() } else { b"share".as_slice() });
    digest.update(material);
    format!("hub.v1.{}", os_directory::io::binary::artifact_hash::hex_lower(&digest.finalize()))
}

pub(crate) fn socket_text_bounded(value: &str) -> bool {
    !value.is_empty() && value.len() <= AUTH_TEXT_MAX_BYTES
}

pub(crate) async fn socket_binding_validity(state: &HubState, subject: &SocketSubjectV1, audience: &SocketAudienceV1) -> SocketBindingValidityV1 {
    let validity = tokio::time::timeout(std::time::Duration::from_secs(2), subject.revalidate(state.directory.as_ref(), audience, now_ms())).await.unwrap_or(SocketBindingValidityV1::Unavailable);
    if validity != SocketBindingValidityV1::Active {
        return validity;
    }
    let SocketAudienceV1::DirectoryScoped(scope) = audience else { return SocketBindingValidityV1::Active };
    match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_document_descriptor(scope)).await {
        Ok(Ok(Some(_))) => SocketBindingValidityV1::Active,
        Ok(Ok(None)) => SocketBindingValidityV1::Unauthorized,
        Ok(Err(_)) | Err(_) => SocketBindingValidityV1::Unavailable,
    }
}

async fn issue_socket_grant(state: &HubState, subject: SocketSubjectV1, audience: SocketAudienceV1, actor_id: String) -> Result<Json<SocketGrantReceiptV1>, StatusCode> {
    let binding = subject.binding();
    let _admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.share_record(&subject, &audience)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let validity = socket_binding_validity(state, &subject, &audience).await;
    match validity {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => return Err(StatusCode::UNAUTHORIZED),
        SocketBindingValidityV1::Unavailable => return Err(StatusCode::SERVICE_UNAVAILABLE),
    }
    let capability = SocketGrantCapability::mint().map_err(directory_error_status)?;
    let now = now_ms();
    let binding_expiry = match &subject {
        SocketSubjectV1::Session { expires_at_ms, .. } | SocketSubjectV1::Share { expires_at_ms, .. } => *expires_at_ms,
    };
    let expires_at_ms = now.checked_add(SOCKET_GRANT_TTL_MS).ok_or(StatusCode::SERVICE_UNAVAILABLE)?.min(binding_expiry);
    if expires_at_ms <= now {
        return Err(StatusCode::UNAUTHORIZED);
    }
    state.socket_grants.issue(&capability, audience.clone(), actor_id.clone(), subject, now, expires_at_ms).map_err(|error| match error {
        SocketGrantLedgerErrorV1::Capacity => StatusCode::SERVICE_UNAVAILABLE,
        SocketGrantLedgerErrorV1::Rejected => StatusCode::UNAUTHORIZED,
    })?;
    let record = state.socket_grants.pending(&capability, &audience, now_ms()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let validity = socket_binding_validity(state, &record.subject, &record.audience).await;
    match validity {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => {
            state.socket_grants.invalidate_binding(binding.clone());
            state.document_open_plans.invalidate_binding(&binding);
            return Err(StatusCode::UNAUTHORIZED);
        }
        SocketBindingValidityV1::Unavailable => {
            state.socket_grants.reject_pending(&record.selector);
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
    }
    Ok(Json(SocketGrantReceiptV1 { schema: "semio.hub.socket-grant/v1", protocol: SOCKET_PROTOCOL_V1, grant: capability.expose_once(), actor_id, expires_at_ms }))
}

async fn authenticate_document_socket_subject(state: &HubState, scope: &DocumentScope, headers: &HeaderMap) -> Result<(SocketSubjectV1, String), DocumentOpenPlanErrorCodeV1> {
    let bearer = socket_issue_bearer(headers).map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?;
    authenticate_document_credential(state, scope, &bearer).await
}

/// 🪪️ One document credential — session or share token — to its socket subject and the actor a
/// grant issued now would carry: a session's actor is bound to the session (stable across plans, so
/// per-actor undo spans reconnects), a share holder's actor is minted per plan (two viewers of one link
/// stay two presences). The socket upgrade resolves the same subject and admits the pending grant
/// issued to its binding, so the actor always comes from the hub, never from the caller.
pub(crate) async fn authenticate_document_credential(state: &HubState, scope: &DocumentScope, bearer: &str) -> Result<(SocketSubjectV1, String), DocumentOpenPlanErrorCodeV1> {
    let capability = HubCapability::parse(bearer).map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?;
    let (subject, actor_id) = match capability {
        HubCapability::Session(capability) => {
            let session = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&capability))
                .await
                .map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?
                .map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?
                .ok_or(DocumentOpenPlanErrorCodeV1::Denied)?;
            let role = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.principal_role(&scope.space_id, session.principal()))
                .await
                .map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?
                .map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?
                .ok_or(DocumentOpenPlanErrorCodeV1::Denied)?;
            let actor_id = socket_actor_id(&session.secret_digest, true);
            let subject = SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: Some(role), expires_at_ms: session.expires_at, session_kind: session.session_kind, device_instance_id: session.device_instance_id };
            (subject, actor_id)
        }
        HubCapability::Share(capability) => {
            let share = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_share_binding(&scope, &capability))
                .await
                .map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?
                .map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?
                .ok_or(DocumentOpenPlanErrorCodeV1::Denied)?;
            let mut ephemeral_actor_material = [0u8; 32];
            directory::os_identity::fill_entropy(&mut ephemeral_actor_material).map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?;
            let actor_id = socket_actor_id(&ephemeral_actor_material, false);
            ephemeral_actor_material.fill(0);
            (SocketSubjectV1::Share { share_id: share.id, selector: share.selector, scope: scope.clone(), expires_at_ms: share.expires_at }, actor_id)
        }
        HubCapability::Invite(_) => return Err(DocumentOpenPlanErrorCodeV1::Denied),
    };
    Ok((subject, actor_id))
}

pub(crate) type DocumentOpenPlanRouteError = (StatusCode, DirectoryJson<DocumentOpenPlanErrorV1>);

fn document_open_plan_route_error(status: StatusCode, code: DocumentOpenPlanErrorCodeV1) -> DocumentOpenPlanRouteError {
    (status, DirectoryJson(DocumentOpenPlanErrorV1 { schema: "semio.hub.document-open-plan-error/v1".into(), code }))
}

fn document_open_plan_exchange_error(code: DocumentOpenPlanErrorCodeV1) -> DocumentOpenPlanRouteError {
    let status = match code {
        DocumentOpenPlanErrorCodeV1::Denied => StatusCode::UNAUTHORIZED,
        DocumentOpenPlanErrorCodeV1::NotFound => StatusCode::NOT_FOUND,
        DocumentOpenPlanErrorCodeV1::Expired => StatusCode::GONE,
        DocumentOpenPlanErrorCodeV1::Stale | DocumentOpenPlanErrorCodeV1::AlreadyConsumed => StatusCode::CONFLICT,
        DocumentOpenPlanErrorCodeV1::Cancelled => StatusCode::REQUEST_TIMEOUT,
        DocumentOpenPlanErrorCodeV1::CatalogUnavailable | DocumentOpenPlanErrorCodeV1::ComponentUnavailable | DocumentOpenPlanErrorCodeV1::DeadlineExceeded => StatusCode::SERVICE_UNAVAILABLE,
    };
    document_open_plan_route_error(status, code)
}

pub(crate) fn document_open_checkpoint(checkpoint: PublishedArtifactCheckpoint) -> DocumentOpenCheckpointV1 {
    DocumentOpenCheckpointV1 {
        checkpoint_id: os_directory::io::binary::artifact_hash::hex_lower(&checkpoint.checkpoint_id.0),
        descriptor_digest_v1: os_directory::io::binary::artifact_hash::hex_lower(&checkpoint.descriptor_digest_v1.0),
        baseline_frontier: checkpoint.baseline_frontier,
        aggregate_sha256: os_directory::io::binary::artifact_hash::hex_lower(&checkpoint.aggregate_sha256.0),
    }
}

pub(crate) async fn issue_document_open_plan_inner(space_id: String, document_id: String, headers: HeaderMap, state: HubState, body: Bytes) -> Result<DirectoryJson<DocumentOpenPlanV1>, DocumentOpenPlanRouteError> {
    let content_types = headers.get_all(axum::http::header::CONTENT_TYPE);
    if !socket_text_bounded(&space_id)
        || !socket_text_bounded(&document_id)
        || body.is_empty()
        || body.len() > DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES
        || content_types.iter().count() != 1
        || content_types.iter().next().and_then(|value| value.to_str().ok()) != Some("application/json")
    {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    let encoded = std::str::from_utf8(&body).map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    let intent: DocumentOpenIntentV1 = semio_framework_pack_json::from_json_str(encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    intent.validate().map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    let scope = DocumentScope::new(space_id, document_id);
    if intent.scope != scope {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    if !state.readiness.features.open_plan {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::CatalogUnavailable));
    }
    let (subject, server_actor_id) = authenticate_document_socket_subject(&state, &scope, &headers).await.map_err(document_open_plan_exchange_error)?;
    let audience = SocketAudienceV1::Document(scope.clone());
    let _admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.share_record(&subject, &audience)).await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    match subject.revalidate(state.directory.as_ref(), &audience, now_ms()).await {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Denied)),
        SocketBindingValidityV1::Unavailable => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)),
    }
    let descriptor =
        state.directory.get_document_descriptor(&scope).await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?.ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::NotFound))?;
    let descriptor_digest_v1 = os_directory::io::binary::artifact_hash::hex_lower(&descriptor_digest_v1(&descriptor).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale))?.0);
    let checkpoint = state
        .directory
        .get_active_artifact_checkpoint(&scope)
        .await
        .map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?
        .ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::NotFound))?;
    if checkpoint.scope != scope || os_directory::io::binary::artifact_hash::hex_lower(&checkpoint.descriptor_digest_v1.0) != descriptor_digest_v1 || !(checkpoint.baseline_frontier.is_genesis_for(&scope) || checkpoint.baseline_frontier.is_edited_for(&scope)) {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    let checkpoint = document_open_checkpoint(checkpoint);
    let catalog = state.openable_catalog.as_ref().ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::CatalogUnavailable))?;
    let writable = subject.surface_writable();
    let selected = catalog.resolve_document_open(&descriptor, intent.requested_surface_id.as_deref(), writable).ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?;
    let directory_revision = state.directory.head_seq().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    if directory_revision == 0 || directory_revision > DOCUMENT_OPEN_MAX_SAFE_INTEGER {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded));
    }
    let (session_generation, share_generation) = match &subject {
        SocketSubjectV1::Session { authorization_generation, .. } => (Some(*authorization_generation), None),
        SocketSubjectV1::Share { .. } => (None, Some(1)),
    };
    let mut client_instance_id_digest = Sha256::new();
    client_instance_id_digest.update(b"semio/hub/document-open/client-instance/v1\0");
    client_instance_id_digest.update(intent.client_instance_id.as_bytes());
    let client_instance_id_digest = client_instance_id_digest.finalize();
    let authority = DocumentOpenPlanAuthorityV1 {
        scope: scope.clone(),
        descriptor,
        descriptor_digest_v1,
        catalog: DocumentOpenCatalogV1 { generation_id: catalog.generation_id().to_string() },
        package: selected.package,
        artifact: selected.artifact,
        parent_dialect: selected.parent_dialect,
        surface: selected.surface,
        browser_actor: selected.browser_actor,
        grant: selected.grant,
        checkpoint,
        revalidation: DocumentOpenRevalidationV1 { directory_revision, membership_generation: directory_revision, session_generation, share_generation },
        subject: subject.clone(),
        server_actor_id,
        client_instance_id_digest,
    };
    authority.validate().map_err(document_open_plan_exchange_error)?;
    #[cfg(test)]
    if let Some(gate) = &state.document_open_plan_issue_gate {
        gate.admitted.add_permits(1);
        gate.release.acquire().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Cancelled))?.forget();
    }
    if state.directory.head_seq().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))? != directory_revision {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    match subject.revalidate(state.directory.as_ref(), &audience, now_ms()).await {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Denied)),
        SocketBindingValidityV1::Unavailable => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)),
    }
    let issued_at_ms = u64::try_from(now_ms()).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    let binding_expiry_ms = match &subject {
        SocketSubjectV1::Session { expires_at_ms, .. } | SocketSubjectV1::Share { expires_at_ms, .. } => u64::try_from(*expires_at_ms).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Expired))?,
    };
    let expires_at_ms = issued_at_ms.checked_add(DOCUMENT_OPEN_PLAN_MAX_TTL_MS).ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?.min(binding_expiry_ms);
    if expires_at_ms <= issued_at_ms {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Expired));
    }
    state.document_open_plans.issue(authority, issued_at_ms, expires_at_ms).map(DirectoryJson).map_err(document_open_plan_exchange_error)
}

pub(crate) async fn issue_document_open_plan(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<DirectoryJson<DocumentOpenPlanV1>, DocumentOpenPlanRouteError> {
    if uri.query().is_some() {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    #[cfg(test)]
    let deadline_ms = state.document_open_plan_deadline_ms.unwrap_or(DOCUMENT_OPEN_PLAN_DEADLINE_MS);
    #[cfg(not(test))]
    let deadline_ms = DOCUMENT_OPEN_PLAN_DEADLINE_MS;
    tokio::time::timeout(std::time::Duration::from_millis(deadline_ms), async move {
        let (parts, body) = request.into_parts();
        let body = axum::body::to_bytes(body, DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES).await.map_err(|_| document_open_plan_route_error(StatusCode::PAYLOAD_TOO_LARGE, DocumentOpenPlanErrorCodeV1::Denied))?;
        issue_document_open_plan_inner(space_id, document_id, parts.headers, state, body).await
    })
    .await
    .unwrap_or_else(|_| Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)))
}

/// 🧭️ The pinned revision is a witness of issue order, not a freeze on the whole directory:
/// demanding equality made every outstanding plan die on the next directory append anywhere on
/// the hub — another space's invite, a rename, an unrelated membership edit — so a member's live
/// socket was closed 4401 by an event that had nothing to do with it. What a plan may never do is
/// claim a revision the directory has not reached: that pin is forged and stays refused. Whether
/// this caller still holds this scope is decided by membership and session revocation on their own
/// paths, which is what closes a removed member's socket in the very same exchange.
pub(crate) async fn issue_document_plan_socket_grant_inner(space_id: String, document_id: String, headers: HeaderMap, state: HubState, body: Bytes) -> Result<Json<DocumentSocketGrantReceiptV1>, DocumentOpenPlanRouteError> {
    let content_types = headers.get_all(axum::http::header::CONTENT_TYPE);
    if !socket_text_bounded(&space_id)
        || !socket_text_bounded(&document_id)
        || body.is_empty()
        || body.len() > DOCUMENT_OPEN_PLAN_EXCHANGE_REQUEST_MAX_BYTES
        || content_types.iter().count() != 1
        || content_types.iter().next().and_then(|value| value.to_str().ok()) != Some("application/json")
    {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    let encoded = std::str::from_utf8(&body).map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    let intent: DocumentPlanSocketGrantIntentV1 = semio_framework_pack_json::from_json_str(encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    intent.validate().map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    if !state.readiness.features.open_plan_exchange {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::CatalogUnavailable));
    }
    let scope = DocumentScope::new(space_id, document_id);
    let (subject, _) = authenticate_document_socket_subject(&state, &scope, &headers).await.map_err(document_open_plan_exchange_error)?;
    let audience = SocketAudienceV1::Document(scope.clone());
    let _admission = state.socket_binding_gates.share_record(&subject, &audience).await;
    match subject.revalidate(state.directory.as_ref(), &audience, now_ms()).await {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Denied)),
        SocketBindingValidityV1::Unavailable => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)),
    }
    let exchange_at_ms = u64::try_from(now_ms()).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    let authority = state.document_open_plans.authority_for_authenticated_exchange(&intent.plan_receipt, &scope, &subject, exchange_at_ms).map_err(document_open_plan_exchange_error)?;
    let descriptor = state.directory.get_document_descriptor(&scope).await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    if descriptor.as_ref() != Some(&authority.descriptor) {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    let catalog = state.openable_catalog.as_ref().ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::CatalogUnavailable))?;
    if catalog.generation_id() != authority.catalog.generation_id
        || catalog.resolve_document_open(&authority.descriptor, Some(&authority.surface.surface_id), authority.grant.write).is_none_or(|selected| {
            selected.package != authority.package
                || selected.artifact != authority.artifact
                || selected.parent_dialect != authority.parent_dialect
                || selected.surface != authority.surface
                || selected.browser_actor != authority.browser_actor
                || selected.grant != authority.grant
        })
    {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    let directory_revision = state.directory.head_seq().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    if authority.revalidation.directory_revision > directory_revision || authority.revalidation.membership_generation > directory_revision {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    state.document_open_plans.exchange_to_socket_grant(&intent.plan_receipt, &authority, exchange_at_ms, state.socket_grants.as_ref()).map(Json).map_err(document_open_plan_exchange_error)
}

//#region 🪪️ExecutionTargetLease
const DOCUMENT_EXECUTION_TARGET_REQUEST_MAX_BYTES: usize = 8 * 1024;

const DOCUMENT_EXECUTION_TARGET_DEADLINE_MS: u64 = 8_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DocumentExecutionTargetAssetV1 {
    Manifest,
    Component,
    Descriptor,
    BrowserActor,
}

/// 🛡️ One protected document-scoped execution-target read. Every call re-authenticates the exact
/// session or share binding, reloads the durable descriptor, and resolves the current trusted
/// selection through the catalog's own exact-selection accessor. It accepts only the bounded
/// `DocumentOpenIntentV1`; a package id, digest, catalog generation, local path or plan receipt is
/// never a selector. The private browser verifier rejects independently selected reads that differ
/// from its open plan; this route rejects authorization or directory changes during one selection.
pub(crate) async fn document_execution_target_selection(space_id: String, document_id: String, headers: HeaderMap, state: HubState, body: Bytes) -> Result<(DocumentExecutionTargetLeaseFieldsV1, VerifiedExecutionTargetAssets, RateLimitSubjectV1), DocumentOpenPlanRouteError> {
    let content_types = headers.get_all(axum::http::header::CONTENT_TYPE);
    if !socket_text_bounded(&space_id)
        || !socket_text_bounded(&document_id)
        || body.is_empty()
        || body.len() > DOCUMENT_EXECUTION_TARGET_REQUEST_MAX_BYTES
        || content_types.iter().count() != 1
        || content_types.iter().next().and_then(|value| value.to_str().ok()) != Some("application/json")
    {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    let encoded = std::str::from_utf8(&body).map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    let intent: DocumentOpenIntentV1 = semio_framework_pack_json::from_json_str(encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    intent.validate().map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    let scope = DocumentScope::new(space_id, document_id);
    if intent.scope != scope {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    if !state.readiness.features.open_plan {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::CatalogUnavailable));
    }
    let (subject, _) = authenticate_document_socket_subject(&state, &scope, &headers).await.map_err(document_open_plan_exchange_error)?;
    let audience = SocketAudienceV1::Document(scope.clone());
    let _admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.share_record(&subject, &audience)).await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    match subject.revalidate(state.directory.as_ref(), &audience, now_ms()).await {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Denied)),
        SocketBindingValidityV1::Unavailable => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)),
    }
    let directory_revision = state.directory.head_seq().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    if directory_revision == 0 || directory_revision > DOCUMENT_OPEN_MAX_SAFE_INTEGER {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded));
    }
    let descriptor =
        state.directory.get_document_descriptor(&scope).await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?.ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::NotFound))?;
    let descriptor_digest_v1 = os_directory::io::binary::artifact_hash::hex_lower(&descriptor_digest_v1(&descriptor).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale))?.0);
    let checkpoint = state
        .directory
        .get_active_artifact_checkpoint(&scope)
        .await
        .map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?
        .ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::NotFound))?;
    if checkpoint.scope != scope || os_directory::io::binary::artifact_hash::hex_lower(&checkpoint.descriptor_digest_v1.0) != descriptor_digest_v1 || !(checkpoint.baseline_frontier.is_genesis_for(&scope) || checkpoint.baseline_frontier.is_edited_for(&scope)) {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    let checkpoint = document_open_checkpoint(checkpoint);
    let catalog = state.openable_catalog.as_ref().ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::CatalogUnavailable))?;
    let writable = subject.surface_writable();
    let generation_id = catalog.generation_id().to_string();
    let assets = catalog.assets_for_current_selection(&descriptor, intent.requested_surface_id.as_deref(), writable, &generation_id).ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?;
    let (session_generation, share_generation) = match &subject {
        SocketSubjectV1::Session { authorization_generation, .. } => (Some(*authorization_generation), None),
        SocketSubjectV1::Share { .. } => (None, Some(1)),
    };
    let component_byte_length = assets.component.byte_length();
    let descriptor_byte_length = u64::try_from(assets.descriptor.len()).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?;
    if component_byte_length == 0 || component_byte_length > DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES || descriptor_byte_length == 0 || descriptor_byte_length > DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable));
    }
    let browser_actor_byte_length = assets.browser_actor.as_ref().map(TrustedCatalogAsset::byte_length);
    let browser_actor = assets
        .selection
        .browser_actor
        .to_lease(
            os_directory::DocumentBrowserActorSourceV1 { component_sha256: &assets.selection.package.component_sha256, descriptor_byte_sha256: &assets.selection.package.descriptor_byte_sha256 },
            assets.selection.surface.renderer_target.as_str(),
            browser_actor_byte_length,
        )
        .map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?;
    let fields = DocumentExecutionTargetLeaseFieldsV1 {
        schema: "semio.os.document-execution-target-lease/v1".into(),
        version: 1,
        scope,
        descriptor_digest_v1,
        catalog: DocumentOpenCatalogV1 { generation_id: generation_id.clone() },
        package: assets.selection.package.clone(),
        component: DocumentExecutionTargetComponentV1 { sha256: assets.selection.package.component_sha256.clone(), blake3: assets.selection.package.component_blake3.clone(), byte_length: component_byte_length },
        descriptor: DocumentExecutionTargetDescriptorV1 { sha256: assets.selection.package.descriptor_byte_sha256.clone(), byte_length: descriptor_byte_length },
        browser_actor,
        artifact: assets.selection.artifact.clone(),
        parent_dialect: DocumentOpenParentDialectV1 { artifact_kind: assets.selection.parent_dialect.artifact_kind.clone(), standard: assets.selection.parent_dialect.standard.clone(), subset: assets.selection.parent_dialect.subset.clone() },
        surface: assets.selection.surface.clone(),
        grant: assets.selection.grant,
        checkpoint,
        revalidation: DocumentOpenRevalidationV1 { directory_revision, membership_generation: directory_revision, session_generation, share_generation },
    };
    fields.validate().map_err(document_open_plan_exchange_error)?;
    #[cfg(test)]
    if let Some(gate) = &state.document_open_plan_issue_gate {
        gate.admitted.add_permits(1);
        gate.release.acquire().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Cancelled))?.forget();
    }
    match subject.revalidate(state.directory.as_ref(), &audience, now_ms()).await {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Denied)),
        SocketBindingValidityV1::Unavailable => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)),
    }
    if state.directory.head_seq().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))? != directory_revision {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    let principal = match &subject {
        SocketSubjectV1::Session { session_id, .. } => RateLimitSubjectV1::principal(session_id),
        SocketSubjectV1::Share { share_id, .. } => RateLimitSubjectV1::principal(share_id),
    };
    Ok((fields, assets, principal))
}

async fn issue_document_execution_target(
    asset: DocumentExecutionTargetAssetV1,
    uri: axum::http::Uri,
    space_id: String,
    document_id: String,
    state: HubState,
    request: axum::extract::Request,
) -> Result<Response, DocumentOpenPlanRouteError> {
    if uri.query().is_some() {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    let hub = state.clone();
    let (fields, assets, principal) = tokio::time::timeout(std::time::Duration::from_millis(DOCUMENT_EXECUTION_TARGET_DEADLINE_MS), async move {
        let (parts, body) = request.into_parts();
        let body = axum::body::to_bytes(body, DOCUMENT_EXECUTION_TARGET_REQUEST_MAX_BYTES).await.map_err(|_| document_open_plan_route_error(StatusCode::PAYLOAD_TOO_LARGE, DocumentOpenPlanErrorCodeV1::Denied))?;
        document_execution_target_selection(space_id, document_id, parts.headers, state, body).await
    })
    .await
    .unwrap_or_else(|_| Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)))?;
    let streamed = match asset {
        DocumentExecutionTargetAssetV1::Manifest => return Ok(DirectoryJson(fields).into_response()),
        DocumentExecutionTargetAssetV1::Descriptor => return Ok(document_execution_target_bytes(&assets.descriptor)),
        DocumentExecutionTargetAssetV1::Component => &assets.component,
        DocumentExecutionTargetAssetV1::BrowserActor => assets.browser_actor.as_ref().ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?,
    };
    match hub.asset_streams.admit(StreamLimitClassV1::ExecutionTargetAsset, principal).await {
        Ok(permit) => document_execution_target_stream(streamed, permit),
        Err(refusal) => {
            hub.note("server.rate-limit", TraceOutcome::Refused, refusal.class);
            Ok(rate_limit_refusal_response(refusal))
        }
    }
}

/// 🌊️ Streams one catalog-verified execution-target asset (the document's component or browser actor, tens of MB) from
/// the generation it was verified in. The route's deadline bounds the selection that authorizes it, never the transfer:
/// reading and hashing the whole asset under that deadline answered 503 for every large component once the hub was busy
/// (ticket 26/09/23 S15: catalog B2 puzzle3d, `execution-target/component` 503 after a slow creation; the plugin-module
/// twin of this defect was S12-1b). The stream withholds its last chunk until length, SHA-256 and BLAKE3 match, so a
/// tampered asset ends the body short instead of completing it. It holds `permit`, its slot of the asset stream rule, until
/// the body ends or the client drops it.
fn document_execution_target_stream(asset: &TrustedCatalogAsset, permit: StreamPermitV1) -> Result<Response, DocumentOpenPlanRouteError> {
    let stream = asset.stream().map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?;
    let length = stream.byte_length();
    let body = axum::body::Body::from_stream(futures::stream::unfold((stream, permit), |(mut stream, permit)| async move {
        stream.next_chunk().await.map(|chunk| (chunk.map_err(|error| std::io::Error::other(error.to_string())), (stream, permit)))
    }));
    Ok((StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/octet-stream".to_string()), (axum::http::header::CONTENT_LENGTH, length.to_string()), (axum::http::header::CACHE_CONTROL, "no-store".to_string())], body).into_response())
}

fn document_execution_target_bytes(bytes: &[u8]) -> Response {
    (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/octet-stream".to_string()), (axum::http::header::CONTENT_LENGTH, bytes.len().to_string()), (axum::http::header::CACHE_CONTROL, "no-store".to_string())], bytes.to_vec())
        .into_response()
}

pub(crate) async fn issue_document_execution_target_manifest(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<Response, DocumentOpenPlanRouteError> {
    issue_document_execution_target(DocumentExecutionTargetAssetV1::Manifest, uri, space_id, document_id, state, request).await
}

pub(crate) async fn issue_document_execution_target_component(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<Response, DocumentOpenPlanRouteError> {
    issue_document_execution_target(DocumentExecutionTargetAssetV1::Component, uri, space_id, document_id, state, request).await
}

pub(crate) async fn issue_document_execution_target_descriptor(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<Response, DocumentOpenPlanRouteError> {
    issue_document_execution_target(DocumentExecutionTargetAssetV1::Descriptor, uri, space_id, document_id, state, request).await
}

pub(crate) async fn issue_document_execution_target_browser_actor(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<Response, DocumentOpenPlanRouteError> {
    issue_document_execution_target(DocumentExecutionTargetAssetV1::BrowserActor, uri, space_id, document_id, state, request).await
}
//#endregion 🪪️ExecutionTargetLease

pub(crate) async fn issue_document_plan_socket_grant(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<Json<DocumentSocketGrantReceiptV1>, DocumentOpenPlanRouteError> {
    if uri.query().is_some() {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    tokio::time::timeout(std::time::Duration::from_secs(2), async move {
        let (parts, body) = request.into_parts();
        let body = axum::body::to_bytes(body, DOCUMENT_OPEN_PLAN_EXCHANGE_REQUEST_MAX_BYTES).await.map_err(|_| document_open_plan_route_error(StatusCode::PAYLOAD_TOO_LARGE, DocumentOpenPlanErrorCodeV1::Denied))?;
        issue_document_plan_socket_grant_inner(space_id, document_id, parts.headers, state, body).await
    })
    .await
    .unwrap_or_else(|_| Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)))
}

#[cfg(test)]
include!("../../🧪️tests/🔬️standalone/🦀️.rs");

pub(crate) async fn issue_directory_socket_grant(headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Result<Json<SocketGrantReceiptV1>, StatusCode> {
    if !body.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let capability = SessionCapability::parse(&socket_issue_bearer(&headers)?).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let session =
        tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&capability)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let actor_id = socket_actor_id(&session.secret_digest, true);
    let audience = SocketAudienceV1::Directory { auth_session_id: session.id.clone(), authorization_generation: session.authorization_generation };
    let subject = SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: None, expires_at_ms: session.expires_at, session_kind: session.session_kind, device_instance_id: session.device_instance_id };
    issue_socket_grant(&state, subject, audience, actor_id).await
}

pub(crate) async fn issue_scoped_directory_socket_grant(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Result<Json<SocketGrantReceiptV1>, StatusCode> {
    if !socket_text_bounded(&space_id) || !socket_text_bounded(&document_id) || !body.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let capability = SessionCapability::parse(&socket_issue_bearer(&headers)?).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let session =
        tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&capability)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let scope = DocumentScope::new(space_id, document_id);
    let role = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.socket_session_binding(&session.id, &session.user_id, session.authorization_generation, Some(&scope.space_id), now_ms())).await {
        Ok(Ok(SocketSessionBindingStatus::Active { role: Some(role), expires_at_ms, .. })) if expires_at_ms == session.expires_at => role,
        Ok(Ok(SocketSessionBindingStatus::Unavailable)) | Ok(Err(_)) | Err(_) => return Err(StatusCode::SERVICE_UNAVAILABLE),
        _ => return Err(StatusCode::UNAUTHORIZED),
    };
    match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_document_descriptor(&scope)).await {
        Ok(Ok(Some(_))) => {}
        Ok(Ok(None)) => return Err(StatusCode::NOT_FOUND),
        Ok(Err(_)) | Err(_) => return Err(StatusCode::SERVICE_UNAVAILABLE),
    }
    let actor_id = socket_actor_id(&session.secret_digest, true);
    let subject = SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: Some(role), expires_at_ms: session.expires_at, session_kind: session.session_kind, device_instance_id: session.device_instance_id };
    issue_socket_grant(&state, subject, SocketAudienceV1::DirectoryScoped(scope), actor_id).await
}
//#endregion 🔖️Rest
