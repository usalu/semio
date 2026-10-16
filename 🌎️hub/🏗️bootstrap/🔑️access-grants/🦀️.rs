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
use semio_hub_artifact_creation::ArtifactCreationCommitAuthorityV1;
use semio_hub_trusted_catalog_descriptor::{TrustedCatalogAsset, VerifiedExecutionTargetAssets};
use semio_hub_auth::rate_limit::{RateLimitSubjectV1, StreamLimitClassV1};
use directory::os_directory::DocumentScope;
use semio_hub_directory_contract::model::SocketSessionBindingStatus;
use semio_hub_directory::{AuthSessionDirectory, DocumentDirectory, EventLog, PrincipalAuthority, ShareTokenDirectory};
use semio_hub_directory_contract::{HubCapability, LocalBootstrapTransport, SessionCapability, SocketGrantCapability};
use semio_hub_inference_runtime_contract::GisMapApprovalIngressAuthorityV1;
use serde::Serialize;
use semio_hub_runtime::startup::DocumentOpenCatalogAuthorityV1;
use semio_hub_runtime::support::{DirectoryJson, now_ms, directory_error_status};
use semio_hub_state::HubState;
use semio_hub_ledgers::socket_grants::{SOCKET_GRANT_TTL_MS, SOCKET_PROTOCOL_V1, SocketAudienceV1, SocketSubjectV1, SocketBindingValidityV1, SocketGrantLedgerErrorV1};
#[cfg(any(test, feature = "testkit"))]
use semio_hub_ledgers::socket_grants::{SESSION_PROTOCOL_V1, SocketGrantRecordV1};
use semio_hub_ledgers::open_plan::DocumentOpenPlanAuthorityV1;
use semio_hub_runtime::transport::rate_limit_refusal_response;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_route_models::directory_commands::FencedDirectoryCommandErrorV1;
#[cfg(any(test, feature = "testkit"))]
use crate::directory_commands::{acquire_directory_command_fence, invalidate_directory_event_authority};
use db::storage::PayloadStorage as _;

//#region 🔖️Rest

pub async fn socket_binding_validity(state: &HubState, subject: &SocketSubjectV1, audience: &SocketAudienceV1) -> SocketBindingValidityV1 {
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
pub async fn authenticate_document_credential(state: &HubState, scope: &DocumentScope, bearer: &str) -> Result<(SocketSubjectV1, String), DocumentOpenPlanErrorCodeV1> {
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

pub async fn issue_document_open_plan_inner(space_id: String, document_id: String, headers: HeaderMap, state: HubState, body: Bytes) -> Result<DirectoryJson<DocumentOpenPlanV1>, DocumentOpenPlanRouteError> {
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
    #[cfg(any(test, feature = "testkit"))]
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

pub async fn issue_document_open_plan(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<DirectoryJson<DocumentOpenPlanV1>, DocumentOpenPlanRouteError> {
    if uri.query().is_some() {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    #[cfg(any(test, feature = "testkit"))]
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
pub async fn issue_document_plan_socket_grant_inner(space_id: String, document_id: String, headers: HeaderMap, state: HubState, body: Bytes) -> Result<Json<DocumentSocketGrantReceiptV1>, DocumentOpenPlanRouteError> {
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

/// 🛡️ One protected document-scoped execution-target read. Every call re-authenticates the exact
/// session or share binding, reloads the durable descriptor, and resolves the current trusted
/// selection through the catalog's own exact-selection accessor. It accepts only the bounded
/// `DocumentOpenIntentV1`; a package id, digest, catalog generation, local path or plan receipt is
/// never a selector. The private browser verifier rejects independently selected reads that differ
/// from its open plan; this route rejects authorization or directory changes during one selection.
pub async fn document_execution_target_selection(space_id: String, document_id: String, headers: HeaderMap, state: HubState, body: Bytes) -> Result<(DocumentExecutionTargetLeaseFieldsV1, VerifiedExecutionTargetAssets, RateLimitSubjectV1), DocumentOpenPlanRouteError> {
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
    #[cfg(any(test, feature = "testkit"))]
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

pub async fn issue_document_execution_target_manifest(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<Response, DocumentOpenPlanRouteError> {
    issue_document_execution_target(DocumentExecutionTargetAssetV1::Manifest, uri, space_id, document_id, state, request).await
}

pub async fn issue_document_execution_target_component(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<Response, DocumentOpenPlanRouteError> {
    issue_document_execution_target(DocumentExecutionTargetAssetV1::Component, uri, space_id, document_id, state, request).await
}

pub async fn issue_document_execution_target_descriptor(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<Response, DocumentOpenPlanRouteError> {
    issue_document_execution_target(DocumentExecutionTargetAssetV1::Descriptor, uri, space_id, document_id, state, request).await
}

pub async fn issue_document_execution_target_browser_actor(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<Response, DocumentOpenPlanRouteError> {
    issue_document_execution_target(DocumentExecutionTargetAssetV1::BrowserActor, uri, space_id, document_id, state, request).await
}
//#endregion 🪪️ExecutionTargetLease

pub async fn issue_document_plan_socket_grant(
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

#[cfg(any(test, feature = "testkit"))]
use semio_hub_directory_service::CommandResult;
use semio_hub_ledgers::access_grants::{DOCUMENT_EXECUTION_TARGET_DEADLINE_MS, DOCUMENT_EXECUTION_TARGET_REQUEST_MAX_BYTES, DOCUMENT_OPEN_PLAN_DEADLINE_MS, DOCUMENT_OPEN_PLAN_EXCHANGE_REQUEST_MAX_BYTES, DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES, DOCUMENT_SOCKET_GRANT_SCHEMA_V1, DocumentExecutionTargetAssetV1, DocumentOpenPlanRouteError, DocumentSocketGrantReceiptV1, SocketGrantReceiptV1, document_execution_target_bytes, document_execution_target_stream, document_open_checkpoint, document_open_plan_exchange_error, document_open_plan_route_error, socket_actor_id, socket_issue_bearer, socket_text_bounded};

/// 🧪️ A document socket grant without the open-plan route: the credential's subject and actor, a
/// descriptor-only plan-free admission for socket laws that are not about plan revalidation.
#[cfg(any(test, feature = "testkit"))]
pub async fn issue_document_socket_grant_fixture(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<DocumentSocketGrantReceiptV1>, StatusCode> {
    if !socket_text_bounded(&space_id) || !socket_text_bounded(&document_id) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let scope = DocumentScope::new(space_id, document_id);
    let (subject, actor_id) = authenticate_document_socket_subject(&state, &scope, &headers).await.map_err(|error| match error {
        DocumentOpenPlanErrorCodeV1::DeadlineExceeded => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::UNAUTHORIZED,
    })?;
    let descriptor = state.directory.get_document_descriptor(&scope).await.map_err(directory_error_status)?;
    if descriptor.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    let audience = SocketAudienceV1::Document(scope);
    let _admission = state.socket_binding_gates.share_record(&subject, &audience).await;
    if socket_binding_validity(&state, &subject, &audience).await != SocketBindingValidityV1::Active {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let now = now_ms();
    let binding_expiry = match &subject {
        SocketSubjectV1::Session { expires_at_ms, .. } | SocketSubjectV1::Share { expires_at_ms, .. } => *expires_at_ms,
    };
    let expires_at_ms = now.checked_add(SOCKET_GRANT_TTL_MS).ok_or(StatusCode::SERVICE_UNAVAILABLE)?.min(binding_expiry);
    state.socket_grants.admit_document_without_plan_for_test(audience, actor_id.clone(), subject, now, expires_at_ms).map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(DocumentSocketGrantReceiptV1 { schema: DOCUMENT_SOCKET_GRANT_SCHEMA_V1, protocol: SESSION_PROTOCOL_V1, actor_id, expires_at_ms }))
}

#[cfg(any(test, feature = "testkit"))]
pub async fn pause_directory_command_authority(state: &HubState, user_id: &str, fenced: bool) {
    if let Some(gate) = &state.live_gate {
        if !fenced {
            gate.directory_command_attempted.add_permits(1);
        }
        let pause = gate.directory_command_pause_user.lock().unwrap().as_ref().is_some_and(|(user, phase)| user == user_id && *phase == fenced);
        if pause {
            gate.directory_command_admitted.add_permits(1);
            gate.directory_command_release.acquire().await.expect("directory command test release").forget();
        }
    }
}

#[cfg(any(test, feature = "testkit"))]
pub async fn pause_directory_command_membership_fence(state: &HubState) {
    if let Some(test_gate) = &state.live_gate {
        if test_gate.socket_membership_remove_enabled.load(std::sync::atomic::Ordering::Acquire) {
            test_gate.socket_membership_remove_admitted.add_permits(1);
            let _ = test_gate.socket_membership_remove_release.acquire().await;
        }
    }
}

#[cfg(any(test, feature = "testkit"))]
pub async fn pause_admin_effect_started(state: &HubState) {
    if let Some(gate) = &state.live_gate {
        if gate.admin_effect_pause_enabled.load(std::sync::atomic::Ordering::Acquire) {
            gate.admin_effect_admitted.add_permits(1);
            gate.admin_effect_release.acquire().await.expect("administrator effect test release").forget();
        }
    }
}

#[cfg(any(test, feature = "testkit"))]
pub async fn execute_directory_command_fenced(state: &HubState, actor: DirectoryActor, command: DirectoryCommand) -> Result<(Vec<DirectoryEvent>, Option<CommandResult>), FencedDirectoryCommandErrorV1> {
    let _authority = acquire_directory_command_fence(state, Vec::new(), &command).await?;
    if matches!(&command, DirectoryCommand::RemoveMember { .. }) {
        pause_directory_command_membership_fence(state).await;
    }
    let result = state.directory_service.execute(actor, command).await.map_err(FencedDirectoryCommandErrorV1::Directory)?;
    invalidate_directory_event_authority(state, &result.0);
    Ok(result)
}

#[cfg(any(test, feature = "testkit"))]
pub async fn pause_global_directory_send_for_test(state: &HubState, record: &SocketGrantRecordV1, mode: u8) {
    let Some(gate) = &state.live_gate else { return };
    let SocketSubjectV1::Session { user_id, .. } = &record.subject else { return };
    let pause = gate.socket_global_send_pause.lock().expect("global send pause").clone();
    if matches!(&record.audience, SocketAudienceV1::Directory { .. }) && pause.as_ref().is_some_and(|(recipient, point)| recipient == user_id && *point == mode) {
        gate.socket_global_send_admitted.add_permits(1);
        gate.socket_global_send_release.acquire().await.expect("global send release").forget();
    }
}

pub async fn issue_directory_socket_grant(headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Result<Json<SocketGrantReceiptV1>, StatusCode> {
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

pub async fn issue_scoped_directory_socket_grant(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Result<Json<SocketGrantReceiptV1>, StatusCode> {
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
