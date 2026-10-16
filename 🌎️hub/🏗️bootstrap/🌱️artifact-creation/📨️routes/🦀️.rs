//! 🛣️ The `artifact_creation` handlers that act over `HubState`.

use std::sync::Arc;
use semio_hub_artifact_creation::ArtifactCreationActorV1;
use semio_hub_route_models::artifact_creation::{ArtifactCreationHttpAdmissionV1, ArtifactCreationHttpControlV1, artifact_creation_error_status, artifact_creation_request_id_v1, artifact_creation_space_id_v1, artifact_creation_status_response, artifact_creation_task_key_v1, await_artifact_creation_http_admission_v1};
use semio_hub_artifact_authority::{AuthorityLimits, OperationContext};
use axum::body::Bytes;
use directory::os_directory::access_policy::DirectoryAccessActionV1;
use semio_hub_runtime::support::{DirectoryJson, bearer, now_ms};
use axum::http::{HeaderMap, StatusCode};
use semio_hub_state::HubState;
use axum::extract::{OriginalUri, Path, State};
use axum::response::Response;
use directory::os_directory::schema::space_artifact_creation::{SPACE_ARTIFACT_CREATION_MAX_BYTES, SpaceArtifactCreateV1, SpaceArtifactCreationPhaseV1};
use semio_hub_ledgers::socket_grants::{SocketBindingGuardV1, SocketBindingKeyV1};
use semio_hub_directory_contract::model::SocketSessionBindingStatus;
use semio_framework_trace::record::{TraceOutcome, TraceRecord};
use directory::os_directory::io::text::access_policy::directory_access_permits;
use semio_hub_session_routes::directory_commands::resolve_bearer_user;
use semio_hub_ledgers::guard::session_access_roles;

//#region 🔖️Directory
async fn acquire_artifact_creation_actor(state: &HubState, space_id: &str, token: Option<&str>) -> Result<(ArtifactCreationActorV1, Vec<SocketBindingGuardV1>), StatusCode> {
    if !artifact_creation_space_id_v1(space_id) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let caller = resolve_bearer_user(state, token).await.ok_or(StatusCode::UNAUTHORIZED)?;
    let guards = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        state.socket_binding_gates.share_bindings(vec![
            SocketBindingKeyV1::User(caller.user_id.clone()),
            SocketBindingKeyV1::Session(caller.session_id.clone()),
            SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.to_string() },
            SocketBindingKeyV1::Membership { user_id: caller.user_id.clone(), space_id: space_id.to_string() },
        ]),
    )
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let binding = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.socket_session_binding(&caller.session_id, &caller.user_id, caller.authorization_generation, Some(space_id), now_ms())).await;
    let space_kind = state.directory.get_space(space_id).await.ok().flatten().map(|space| space.kind);
    match binding {
        Ok(Ok(SocketSessionBindingStatus::Active { role: Some(role), session_kind, .. })) if space_kind.as_deref().is_some_and(|kind| directory_access_permits(&session_access_roles(session_kind, Some(role)), DirectoryAccessActionV1::ArtifactCreate, Some(kind))) => {
            Ok((ArtifactCreationActorV1 { user_id: caller.user_id, session_id: caller.session_id, authorization_generation: caller.authorization_generation }, guards))
        }
        Ok(Ok(SocketSessionBindingStatus::Active { .. } | SocketSessionBindingStatus::MembershipLost)) => Err(StatusCode::FORBIDDEN),
        Ok(Ok(SocketSessionBindingStatus::Revoked | SocketSessionBindingStatus::Expired)) => Err(StatusCode::UNAUTHORIZED),
        Ok(Ok(SocketSessionBindingStatus::Unavailable)) | Ok(Err(_)) | Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

pub async fn get_space_artifact_creation_catalog(Path(space_id): Path<String>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Response {
    if uri.query().is_some() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let Some(service) = state.artifact_creation.as_ref() else { return StatusCode::SERVICE_UNAVAILABLE.into_response() };
    let (actor, _guards) = match acquire_artifact_creation_actor(&state, &space_id, bearer(&headers).as_deref()).await {
        Ok(actor) => actor,
        Err(status) => return status.into_response(),
    };
    match service.creation_catalog(&actor, &space_id, u64::try_from(now_ms()).unwrap_or_default()).await {
        Ok(catalog) => {
            let mut response = DirectoryJson(catalog).into_response();
            response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
            response
        }
        Err(error) => artifact_creation_error_status(error).into_response(),
    }
}

/// 🧗️ No socket is waiting on what follows: the route has already answered 202 and the
/// client polls the durable status. Its bound is therefore the stall bound, fed by the
/// guest codec's own fuel progress, never the calendar.
pub async fn post_space_artifact_creation(Path(space_id): Path<String>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    if uri.query().is_some()
        || body.is_empty()
        || body.len() > SPACE_ARTIFACT_CREATION_MAX_BYTES
        || headers.get_all(axum::http::header::CONTENT_TYPE).iter().count() != 1
        || headers.get(axum::http::header::CONTENT_TYPE).and_then(|value| value.to_str().ok()) != Some("application/json")
    {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let request = match std::str::from_utf8(&body).ok().and_then(SpaceArtifactCreateV1::parse_canonical_json) {
        Some(request) => request,
        None => return StatusCode::BAD_REQUEST.into_response(),
    };
    let Some(service) = state.artifact_creation.as_ref().cloned() else { return StatusCode::SERVICE_UNAVAILABLE.into_response() };
    let (actor, guards) = match acquire_artifact_creation_actor(&state, &space_id, bearer(&headers).as_deref()).await {
        Ok(actor) => actor,
        Err(status) => return status.into_response(),
    };
    let key = artifact_creation_task_key_v1(&actor.user_id, &space_id, &request.request_id);
    let control = Arc::new(ArtifactCreationHttpControlV1::new());
    let mut predecessor_failures = 0u8;
    let reservation = loop {
        match state.artifact_creation_tasks.reserve(key.clone(), control.clone()) {
            ArtifactCreationHttpAdmissionV1::Owner(reservation) => break Some(reservation),
            ArtifactCreationHttpAdmissionV1::Join(pending) => match await_artifact_creation_http_admission_v1(pending.as_ref()).await {
                Some(1) => break None,
                Some(2) if predecessor_failures < 2 => {
                    predecessor_failures += 1;
                    continue;
                }
                _ => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
            },
            ArtifactCreationHttpAdmissionV1::Unavailable => return ([(axum::http::header::RETRY_AFTER, "1")], StatusCode::SERVICE_UNAVAILABLE).into_response(),
        }
    };
    let Ok(context) = OperationContext::stall_bounded(semio_hub_artifact_creation::ARTIFACT_CREATION_STALL_BOUND_MS, AuthorityLimits::maximum(), control.as_ref()) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let acceptance = match service.accept(&actor, &space_id, request, &context).await {
        Ok(acceptance) => acceptance,
        Err(error) => return artifact_creation_error_status(error).into_response(),
    };
    drop(guards);
    if let Some(execution) = acceptance.execution {
        let Some(reservation) = reservation else {
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        };
        let authority = state.artifact_creation_commit_authority.clone();
        let execution_tracer = state.tracer.clone();
        reservation.activate(async move {
            let context = match OperationContext::stall_bounded(semio_hub_artifact_creation::ARTIFACT_CREATION_STALL_BOUND_MS, AuthorityLimits::maximum(), control.as_ref()) {
                Ok(context) => context,
                Err(error) => {
                    let mut record = TraceRecord::new("server.artifact.creation", TraceOutcome::Failed);
                    record.detail = Some(format!("creation-retained-execution-unbounded:{error}"));
                    execution_tracer.emit(record);
                    return;
                }
            };
            if let Err(error) = service.execute(execution, authority.as_ref(), &context).await {
                let mut record = TraceRecord::new("server.artifact.maintenance", TraceOutcome::Failed);
                record.detail = Some(format!("creation-retained-execution-unavailable:{error}"));
                execution_tracer.emit(record);
            }
            if let Some(detail) = control.take_fault() {
                let mut record = TraceRecord::new("server.artifact.creation", TraceOutcome::Failed);
                record.detail = Some(detail);
                execution_tracer.emit(record);
            }
        });
    } else if let Some(reservation) = reservation {
        reservation.finish();
    }
    artifact_creation_status_response(acceptance.status)
}

pub async fn get_space_artifact_creation_status(Path((space_id, request_id)): Path<(String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Response {
    if uri.query().is_some() || !artifact_creation_request_id_v1(&request_id) {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let Some(service) = state.artifact_creation.as_ref() else { return StatusCode::SERVICE_UNAVAILABLE.into_response() };
    let (actor, _guards) = match acquire_artifact_creation_actor(&state, &space_id, bearer(&headers).as_deref()).await {
        Ok(actor) => actor,
        Err(status) => return status.into_response(),
    };
    match service.status(&actor, &space_id, &request_id, u64::try_from(now_ms()).unwrap_or_default()).await {
        Ok(mut status) => {
            if matches!(status.phase, SpaceArtifactCreationPhaseV1::Accepted | SpaceArtifactCreationPhaseV1::Preparing) {
                status.progress = state.artifact_creation_tasks.live_progress(&artifact_creation_task_key_v1(&actor.user_id, &space_id, &request_id));
            }
            artifact_creation_status_response(status)
        }
        Err(error) => artifact_creation_error_status(error).into_response(),
    }
}

pub async fn post_space_artifact_creation_cancel(Path((space_id, request_id)): Path<(String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    if uri.query().is_some() || !body.is_empty() || !artifact_creation_request_id_v1(&request_id) {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let Some(service) = state.artifact_creation.as_ref() else { return StatusCode::SERVICE_UNAVAILABLE.into_response() };
    let (actor, _guards) = match acquire_artifact_creation_actor(&state, &space_id, bearer(&headers).as_deref()).await {
        Ok(actor) => actor,
        Err(status) => return status.into_response(),
    };
    match service.cancel(&actor, &space_id, &request_id, u64::try_from(now_ms()).unwrap_or_default()).await {
        Ok(status) => {
            state.artifact_creation_tasks.cancel(&artifact_creation_task_key_v1(&actor.user_id, &space_id, &request_id));
            artifact_creation_status_response(status)
        }
        Err(error) => artifact_creation_error_status(error).into_response(),
    }
}
//#endregion 🔖️Directory
