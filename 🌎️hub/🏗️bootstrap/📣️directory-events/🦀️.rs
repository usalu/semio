//! 📣️ `directory_events` — directory event pages, event stream and directory WebSockets.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use directory::os_directory::access_policy::directory_access_permits;
use directory::os_directory::io::text::validate_directory_event_page_event;
use axum::extract::ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade};
use axum::extract::{OriginalUri, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use directory::os_directory;
use directory::os_directory::{DirectoryEvent, DirectoryEventPageErrorV1, DirectoryEventPageV1, DirectoryStreamMessage, DIRECTORY_EVENT_PAGE_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS, DOCUMENT_OPEN_MAX_SAFE_INTEGER};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use protocol::{decode_client_frame, ClientFrame};
use semio_framework_hash::Sha256;
use semio_framework_trace::record::TraceOutcome;
use semio_hub_artifact_creation::ArtifactCreationCommitAuthorityV1;
use directory::os_directory::access_policy::DirectoryAccessActionV1;
use semio_hub_directory_contract::model::DirectoryPrincipalV1;
use directory::os_directory::DocumentScope;
use semio_hub_directory::{AuthSessionDirectory, EventLog, PrincipalAuthority, SpaceDirectory};
use semio_hub_directory_contract::{LocalBootstrapTransport, AUTH_TEXT_MAX_BYTES};
use semio_hub_inference_runtime_contract::GisMapApprovalIngressAuthorityV1;
use serde::Deserialize;
use std::collections::BTreeSet;
use std::sync::Arc;
use tokio::sync::broadcast;
use semio_hub_runtime::support::{DirectoryJson, now_ms, bearer, directory_error_status};
use semio_hub_state::HubState;
use semio_hub_ledgers::socket_grants::{SOCKET_PROTOCOL_V1, SocketAudienceV1, SocketBindingValidityV1, SocketGrantRecordV1, SocketGrantAdmissionV1, SocketLiveLeaseV1};
use crate::socket_grants::socket_live_authority_with_bindings;
use semio_hub_ledgers::guard::session_access_roles;
use semio_hub_ledgers::access_grants::socket_text_bounded;
use semio_hub_session_routes::access_grants::socket_binding_validity;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_session_routes::access_grants::pause_global_directory_send_for_test;
use crate::document_socket::{consume_scoped_directory_socket_grant, consume_directory_socket_grant, verified_rebootstrap_control};
use semio_hub_route_models::directory_commands::{AuthedUser, CredentialOptionalRefusalV1, role_wire, DirectorySpaceAccessDecisionV1, directory_space_access_decision};
use semio_hub_session_routes::directory_commands::{resolve_bearer_user, resolve_optional_bearer_user};
use semio_hub_runtime::lifecycle::hub_shutdown_close_frame;
use db::storage::PayloadStorage as _;
use semio_hub_route_models::directory_events::{DIRECTORY_EVENT_PAGE_DEADLINE_MS, DirectoryEventLaneV1, DirectoryEventPageHttpControl, DirectoryEventPageHttpRequest, DirectoryScopedWsV1Query, DirectoryWsV1Query, EventsQuery, ScopedDirectoryFrameDecisionV1, directory_event_lane_v1, directory_event_page_bytes_bound, directory_event_page_event_visible, directory_event_page_request_admission, directory_event_page_session_binding_v1, directory_message_bindings, directory_message_matches_scope, directory_reader_access_change, directory_space_message_bindings, directory_stream_message_space, load_all_directory_events, note_directory_space_created, seal_directory_event_page_v1, send_directory_message};

//#region 🔖️Directory

async fn revalidate_directory_event_page_caller(state: &HubState, caller: &AuthedUser, binding: [u8; 32]) -> Result<AuthedUser, StatusCode> {
    let session = state.directory.authenticate_session(&caller.capability).await.map_err(|_| StatusCode::UNAUTHORIZED)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let current = AuthedUser { user_id: session.user_id, session_id: session.id, expires_at: session.expires_at, authorization_generation: session.authorization_generation, capability: caller.capability.clone(), session_kind: session.session_kind, device_instance_id: session.device_instance_id };
    if current.session_id != caller.session_id || current.user_id != caller.user_id || current.authorization_generation != caller.authorization_generation || directory_event_page_session_binding_v1(&current)? != binding {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(current)
}

/// 🧑‍🤝‍🧑 The spaces `principal` holds a role in ([`PrincipalAuthority::principal_ceiling`]), read ONCE per event page:
/// raw directory events are member-only ([`event_visible`]), so this set decides every row of the page. The page used to
/// read the space and the caller's role per event — two directory round trips per row, 256 for a full page (ticket
/// 26/09/23 WG8).
async fn directory_member_space_ids(state: &HubState, principal: DirectoryPrincipalV1<'_>) -> Result<BTreeSet<String>, StatusCode> {
    let ceiling = state.directory.principal_ceiling(principal).await.map_err(directory_error_status)?;
    Ok(state.directory.list_spaces_for_user(principal.user_id).await.map_err(directory_error_status)?.into_iter().filter(|(space, role)| ceiling.cap(&space.id, Some(*role)).is_some()).map(|(space, _)| space.id).collect())
}

async fn build_directory_event_page_v1(state: &HubState, caller: &AuthedUser, after: u64, control: &DirectoryEventPageHttpControl, lane: DirectoryEventLaneV1) -> Result<DirectoryEventPageV1, StatusCode> {
    control.checkpoint()?;
    let binding = directory_event_page_session_binding_v1(caller)?;
    let raw = state.directory.events_since(after, DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    control.checkpoint()?;
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        if gate.directory_event_page_fence_enabled.load(std::sync::atomic::Ordering::Acquire) {
            gate.directory_event_page_read_admitted.add_permits(1);
            gate.directory_event_page_read_release.acquire().await.expect("directory event page read test release").forget();
        }
    }
    control.checkpoint()?;
    let caller = revalidate_directory_event_page_caller(state, caller, binding).await?;
    control.checkpoint()?;
    let member_spaces = directory_member_space_ids(state, caller.principal()).await?;
    control.checkpoint()?;
    let envelope = DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: os_directory::io::binary::artifact_hash::hex_lower(&binding),
        authorization_generation: caller.authorization_generation,
        after_seq_exclusive: after,
        through_seq_inclusive: u64::MAX,
        has_more: false,
        events: Vec::new(),
        receipt_sha256: "0".repeat(64),
    };
    let envelope_bytes = semio_framework_pack_json::to_json_string(&envelope).len();
    let raw_len = raw.len();
    let mut through = after;
    let mut events = Vec::new();
    let mut event_bytes = 0usize;
    let mut stopped_for_bytes = false;
    for event in raw {
        control.checkpoint()?;
        if event.seq <= through || validate_directory_event_page_event(&event).is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        if directory_event_lane_v1(&event) != lane || !directory_event_page_event_visible(&member_spaces, &event, &caller) {
            if directory_event_page_bytes_bound(envelope_bytes, event_bytes, events.len()) > DIRECTORY_EVENT_PAGE_MAX_BYTES {
                match seal_directory_event_page_v1(binding, caller.authorization_generation, after, event.seq, true, events.clone()) {
                    Ok(_) => {}
                    Err(DirectoryEventPageErrorV1::TooLarge) => {
                        stopped_for_bytes = true;
                        break;
                    }
                    Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
            through = event.seq;
            continue;
        }
        let bytes = semio_framework_pack_json::to_json_string(&event).len();
        if directory_event_page_bytes_bound(envelope_bytes, event_bytes.saturating_add(bytes), events.len() + 1) > DIRECTORY_EVENT_PAGE_MAX_BYTES {
            let mut candidate = events.clone();
            candidate.push(event.clone());
            match seal_directory_event_page_v1(binding, caller.authorization_generation, after, event.seq, true, candidate) {
                Ok(_) => {}
                Err(DirectoryEventPageErrorV1::TooLarge) => {
                    stopped_for_bytes = true;
                    break;
                }
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        through = event.seq;
        event_bytes = event_bytes.saturating_add(bytes);
        events.push(event);
    }
    seal_directory_event_page_v1(binding, caller.authorization_generation, after, through, stopped_for_bytes || raw_len == DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS, events).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_directory_event_page_v1(OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<DirectoryEventPageV1>, StatusCode> {
    serve_directory_event_page_v1(uri, headers, state, DirectoryEventLaneV1::Directory).await
}

/// 🌐️ `GET /directory/preference-page/v1?after=` — the caller's own preference lane on the directory page machinery.
pub async fn get_directory_preference_page_v1(OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<DirectoryEventPageV1>, StatusCode> {
    serve_directory_event_page_v1(uri, headers, state, DirectoryEventLaneV1::Preferences).await
}

async fn serve_directory_event_page_v1(uri: axum::http::Uri, headers: HeaderMap, state: HubState, lane: DirectoryEventLaneV1) -> Result<DirectoryJson<DirectoryEventPageV1>, StatusCode> {
    let after = directory_event_page_request_admission(&uri)?;
    let control = Arc::new(DirectoryEventPageHttpControl::new());
    let mut request = DirectoryEventPageHttpRequest::new(control.clone());
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        if gate.directory_event_page_fence_enabled.load(std::sync::atomic::Ordering::Acquire) {
            *gate.directory_event_page_control.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(control.clone());
        }
    }
    let operation = async {
        let caller = resolve_bearer_user(&state, bearer(&headers).as_deref()).await.ok_or(StatusCode::UNAUTHORIZED)?;
        if lane == DirectoryEventLaneV1::Preferences && !directory_access_permits(&session_access_roles(caller.session_kind, None), DirectoryAccessActionV1::PreferenceRead, None) {
            return Err(StatusCode::FORBIDDEN);
        }
        build_directory_event_page_v1(&state, &caller, after, control.as_ref(), lane).await.map(DirectoryJson)
    };
    let response = match tokio::time::timeout(std::time::Duration::from_millis(DIRECTORY_EVENT_PAGE_DEADLINE_MS), operation).await {
        Ok(result) => result,
        Err(_) => {
            control.cancel();
            Err(StatusCode::GATEWAY_TIMEOUT)
        }
    };
    request.finish_response_owned();
    response
}

/// 🪪️ Revalidates the exact browser session behind a long-lived directory stream. Revocation and
/// expiry therefore take effect on the next outbound frame instead of leaving a previously opened
/// socket privileged indefinitely.
async fn caller_active(state: &HubState, caller: &AuthedUser) -> bool {
    if caller.expires_at <= now_ms() {
        return false;
    }
    matches!(
        state.directory.authenticate_session(&caller.capability).await,
        Ok(Some(session)) if session.id == caller.session_id && session.user_id == caller.user_id && session.authorization_generation == caller.authorization_generation
    )
}

async fn directory_space_access_for_user(state: &HubState, space_id: &str, principal: Option<DirectoryPrincipalV1<'_>>) -> DirectorySpaceAccessDecisionV1 {
    let Ok(Some(space)) = state.directory.get_space(space_id).await else { return DirectorySpaceAccessDecisionV1::Hidden };
    let role = match principal {
        Some(principal) => state.directory.principal_role(space_id, principal).await.ok().flatten().map(role_wire),
        None => None,
    };
    directory_space_access_decision(space.visibility == "public", role)
}

/// 👁️ Raw durable events are member-only. Public discovery has no event stream, so
/// no redaction of actor, HLC, sequence, or identity-bearing event bodies can be forgotten.
async fn event_visible(state: &HubState, event: &DirectoryEvent, caller: Option<&AuthedUser>) -> bool {
    let Some(space_id) = &event.space_id else {
        return match (caller, event.user_id.as_deref()) {
            (Some(caller), Some(user_id)) if caller.user_id == user_id => true,
            _ => false,
        };
    };
    directory_space_access_for_user(state, space_id, caller.map(AuthedUser::principal)).await.is_member()
}

/// 🛡️ The single privacy boundary for every directory WebSocket frame. Realtime connection
/// and presence telemetry requires current membership even for public spaces; public visibility
/// exposes directory metadata, not who is online or their account email.
pub async fn directory_message_visible(state: &HubState, message: &DirectoryStreamMessage, caller: Option<&AuthedUser>) -> bool {
    let Some(caller) = caller else { return false };
    if !caller_active(state, caller).await {
        return false;
    }
    match message {
        DirectoryStreamMessage::Event { event } => event_visible(state, event, Some(caller)).await,
        DirectoryStreamMessage::Connection { connection, .. } => directory_space_access_for_user(state, &connection.space_id, Some(caller.principal())).await.is_member(),
        DirectoryStreamMessage::Presence { space_id, .. } => directory_space_access_for_user(state, space_id, Some(caller.principal())).await.is_member(),
        DirectoryStreamMessage::Heartbeat { .. } | DirectoryStreamMessage::AccessChanged { .. } => false,
        DirectoryStreamMessage::RebootstrapRequired { control } => directory_space_access_for_user(state, &control.scope.space_id, Some(caller.principal())).await.is_member(),
    }
}

/// 👁️ Keeps the events `caller` may read — its own space-less events and every event of a space it belongs
/// to — deciding the whole page from ONE membership read (it used to read the space and the role per distinct space).
async fn visibility_filter_events(state: &HubState, events: Vec<DirectoryEvent>, caller: Option<&AuthedUser>) -> Result<Vec<DirectoryEvent>, StatusCode> {
    let Some(caller) = caller else { return Ok(Vec::new()) };
    let member_spaces = directory_member_space_ids(state, caller.principal()).await?;
    Ok(events.into_iter().filter(|event| directory_event_page_event_visible(&member_spaces, event, caller)).collect())
}

pub async fn get_directory_events(Query(query): Query<EventsQuery>, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<Vec<DirectoryEvent>>, CredentialOptionalRefusalV1> {
    let caller = resolve_optional_bearer_user(&state, &headers).await?;
    let events = state.directory.events_since(query.since.unwrap_or(0), query.limit.unwrap_or(500)).await.map_err(directory_error_status)?;
    Ok(DirectoryJson(visibility_filter_events(&state, events, caller.as_ref()).await?))
}

pub async fn socket_directory_membership_visibility(state: &HubState, record: &SocketGrantRecordV1, message: &DirectoryStreamMessage) -> SocketBindingValidityV1 {
    let Some(principal) = record.subject.principal() else { return SocketBindingValidityV1::Unauthorized };
    let Some(space_id) = directory_stream_message_space(message) else {
        return if matches!(message, DirectoryStreamMessage::AccessChanged { .. }) || matches!(message, DirectoryStreamMessage::Event { event } if event.user_id.as_deref() == Some(principal.user_id)) { SocketBindingValidityV1::Active } else { SocketBindingValidityV1::Unauthorized };
    };
    match state.directory.get_space(space_id).await {
        Ok(Some(_)) => {}
        Ok(None) => return SocketBindingValidityV1::Unauthorized,
        Err(_) => return SocketBindingValidityV1::Unavailable,
    }
    match state.directory.principal_role(space_id, principal).await {
        Ok(Some(_)) => SocketBindingValidityV1::Active,
        Ok(None) => SocketBindingValidityV1::Unauthorized,
        Err(_) => SocketBindingValidityV1::Unavailable,
    }
}

/// 📝️ The directory lane's admission span (`server.directory.socket`), the same shape the
/// document lane reports: one record for the upgrade decision and one for the session's lifetime.
pub async fn directory_ws_v1(ws: WebSocketUpgrade, Query(query): Query<DirectoryWsV1Query>, headers: HeaderMap, State(state): State<HubState>) -> Response {
    let span = state.span("server.directory.socket");
    let scope = match (query.space_id, query.document_id) {
        (Some(space_id), Some(document_id)) if socket_text_bounded(&space_id) && socket_text_bounded(&document_id) => Some(DocumentScope::new(space_id, document_id)),
        (None, None) => None,
        _ => {
            span.refused("bounds");
            return StatusCode::BAD_REQUEST.into_response();
        }
    };
    let admission = match consume_directory_socket_grant(&state, &headers).await {
        Ok(admission) => admission,
        Err(status) => {
            span.refused(&format!("socket-grant-{}", status.as_u16()));
            return (status, "socket grant rejected").into_response();
        }
    };
    let mut span = span.principal(admission.record.subject.trace_principal());
    if let Some(scope) = &scope {
        span = span.space(scope.space_id.clone()).artifact(scope.document_id.clone());
    }
    let session_span = span.fork();
    span.finish(TraceOutcome::Ok, Some("upgrade".to_string()));
    ws.protocols([SOCKET_PROTOCOL_V1])
        .on_upgrade(move |socket| async move {
            handle_directory_ws_v1(socket, query.since, scope, state, admission).await;
            session_span.cancelled("closed");
        })
        .into_response()
}

pub async fn directory_scoped_ws_v1(ws: WebSocketUpgrade, Path((space_id, document_id)): Path<(String, String)>, Query(query): Query<DirectoryScopedWsV1Query>, headers: HeaderMap, State(state): State<HubState>) -> Response {
    let span = state.span("server.directory.socket").space(space_id.clone()).artifact(document_id.clone());
    if !socket_text_bounded(&space_id) || !socket_text_bounded(&document_id) {
        span.refused("bounds");
        return StatusCode::BAD_REQUEST.into_response();
    }
    let scope = DocumentScope::new(space_id, document_id);
    let admission = match consume_scoped_directory_socket_grant(&state, &headers, scope.clone()).await {
        Ok(admission) => admission,
        Err(status) => {
            span.refused(&format!("socket-grant-{}", status.as_u16()));
            return (status, "socket grant rejected").into_response();
        }
    };
    let span = span.principal(admission.record.subject.trace_principal());
    let session_span = span.fork();
    span.finish(TraceOutcome::Ok, Some("upgrade".to_string()));
    ws.protocols([SOCKET_PROTOCOL_V1])
        .on_upgrade(move |socket| async move {
            handle_directory_ws_v1(socket, query.since, Some(scope), state, admission).await;
            session_span.cancelled("closed");
        })
        .into_response()
}

async fn send_socket_directory_message(
    sender: &mut SplitSink<WebSocket, Message>,
    state: &HubState,
    record: &SocketGrantRecordV1,
    live_id: &str,
    delivery_space_id: Option<&str>,
    delivery_epoch: u64,
    message: &DirectoryStreamMessage,
) -> ScopedDirectoryFrameDecisionV1 {
    #[cfg(test)]
    pause_global_directory_send_for_test(state, record, 1).await;
    #[cfg(test)]
    if matches!(&record.audience, SocketAudienceV1::DirectoryScoped(_)) {
        if let Some(gate) = &state.live_gate {
            if gate.socket_scoped_send_mode.load(std::sync::atomic::Ordering::Acquire) == 1 {
                gate.socket_scoped_send_admitted.add_permits(1);
                gate.socket_scoped_send_release.acquire().await.expect("scoped send test release").forget();
            }
        }
    }
    let _admission = match socket_live_authority_with_bindings(state, record, live_id, directory_message_bindings(record, message)).await {
        Ok(admission) => admission,
        Err(SocketBindingValidityV1::Unauthorized) => return ScopedDirectoryFrameDecisionV1::CloseUnauthorized,
        Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => return ScopedDirectoryFrameDecisionV1::CloseUnavailable,
    };
    #[cfg(test)]
    if matches!(&record.audience, SocketAudienceV1::DirectoryScoped(_)) {
        if let Some(gate) = &state.live_gate {
            if gate.socket_scoped_send_mode.load(std::sync::atomic::Ordering::Acquire) == 2 {
                gate.socket_scoped_send_admitted.add_permits(1);
                gate.socket_scoped_send_release.acquire().await.expect("scoped send test release").forget();
            }
        }
    }
    let decision = match &record.audience {
        SocketAudienceV1::DirectoryScoped(scope) => {
            if directory_message_matches_scope(scope, message) {
                ScopedDirectoryFrameDecisionV1::Deliver
            } else {
                ScopedDirectoryFrameDecisionV1::SkipUnrelated
            }
        }
        SocketAudienceV1::Directory { .. } => match tokio::time::timeout(std::time::Duration::from_secs(2), socket_directory_membership_visibility(state, record, message)).await.unwrap_or(SocketBindingValidityV1::Unavailable) {
            SocketBindingValidityV1::Active => ScopedDirectoryFrameDecisionV1::Deliver,
            SocketBindingValidityV1::Unauthorized => ScopedDirectoryFrameDecisionV1::SkipUnrelated,
            SocketBindingValidityV1::Unavailable => ScopedDirectoryFrameDecisionV1::CloseUnavailable,
        },
        SocketAudienceV1::Document(_) => ScopedDirectoryFrameDecisionV1::CloseUnauthorized,
    };
    if decision != ScopedDirectoryFrameDecisionV1::Deliver {
        return decision;
    }
    let _delivery = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory_service.acquire_delivery_lease(delivery_space_id, delivery_epoch)).await {
        Ok(Some(delivery)) => delivery,
        Ok(None) | Err(_) => return ScopedDirectoryFrameDecisionV1::CloseUnavailable,
    };
    #[cfg(test)]
    pause_global_directory_send_for_test(state, record, 2).await;
    match tokio::time::timeout(std::time::Duration::from_secs(2), send_directory_message(sender, message)).await {
        Ok(true) => ScopedDirectoryFrameDecisionV1::Deliver,
        _ => ScopedDirectoryFrameDecisionV1::CloseUnavailable,
    }
}

async fn send_socket_directory_rebootstrap(
    sender: &mut SplitSink<WebSocket, Message>,
    state: &HubState,
    record: &SocketGrantRecordV1,
    live_id: &str,
    delivery_space_id: Option<&str>,
    delivery_epoch: u64,
    scope: &DocumentScope,
) -> SocketBindingValidityV1 {
    let _admission = match socket_live_authority_with_bindings(state, record, live_id, directory_space_message_bindings(record, Some(&scope.space_id))).await {
        Ok(admission) => admission,
        Err(validity) => return validity,
    };
    if !matches!(&record.audience, SocketAudienceV1::DirectoryScoped(audience_scope) if audience_scope == scope) && !matches!(&record.audience, SocketAudienceV1::Directory { .. }) {
        return SocketBindingValidityV1::Unauthorized;
    }
    let Some(principal) = record.subject.principal() else { return SocketBindingValidityV1::Unauthorized };
    match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.principal_role(&scope.space_id, principal)).await {
        Ok(Ok(Some(_))) => {}
        Ok(Ok(None)) => return SocketBindingValidityV1::Unauthorized,
        Ok(Err(_)) | Err(_) => return SocketBindingValidityV1::Unavailable,
    }
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.socket_rebootstrap_read.add_permits(1);
    }
    let control = match tokio::time::timeout(std::time::Duration::from_secs(2), verified_rebootstrap_control(state, scope)).await {
        Ok(control) => control,
        Err(_) => return SocketBindingValidityV1::Unavailable,
    };
    match control {
        Some(control) => {
            let _delivery = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory_service.acquire_delivery_lease(delivery_space_id, delivery_epoch)).await {
                Ok(Some(delivery)) => delivery,
                Ok(None) | Err(_) => return SocketBindingValidityV1::Unavailable,
            };
            match tokio::time::timeout(std::time::Duration::from_secs(2), send_directory_message(sender, &DirectoryStreamMessage::RebootstrapRequired { control })).await {
                Ok(true) => SocketBindingValidityV1::Active,
                _ => SocketBindingValidityV1::Unavailable,
            }
        }
        None => SocketBindingValidityV1::Active,
    }
}

async fn handle_directory_ws_v1(socket: WebSocket, since: u64, scope: Option<DocumentScope>, state: HubState, admission: SocketGrantAdmissionV1) {
    let (mut sender, mut receiver) = socket.split();
    let Some(mut drain) = state.socket_drain.admit() else {
        let _ = tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(hub_shutdown_close_frame())).await;
        return;
    };
    let SocketGrantAdmissionV1 { record } = admission;
    let hello = match tokio::time::timeout(std::time::Duration::from_secs(2), receiver.next()).await {
        Ok(Some(Ok(Message::Binary(bytes)))) => decode_client_frame(&bytes).await.ok().map(|(_, frame)| frame),
        _ => None,
    };
    let Some(ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema, resume_token, .. }) = hello else {
        let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
        return;
    };
    if !socket_text_bounded(&schema) || resume_token.as_ref().is_some_and(|value| value.len() > AUTH_TEXT_MAX_BYTES) {
        let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
        return;
    }
    let binding_gates = match tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.share_record(&record.subject, &record.audience)).await {
        Ok(admission) => admission,
        Err(_) => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
            return;
        }
    };
    let (live_id, notify) = match state.socket_grants.register_live(&record) {
        Ok(live) => live,
        Err(_) => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
            return;
        }
    };
    let live_lease = SocketLiveLeaseV1 { ledger: state.socket_grants.clone(), record: record.clone(), id: live_id, notify };
    let validity = socket_binding_validity(&state, &record.subject, &record.audience).await;
    match validity {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
            return;
        }
        SocketBindingValidityV1::Unavailable => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
            return;
        }
    }
    drop(binding_gates);
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        if gate.socket_directory_pause_enabled.load(std::sync::atomic::Ordering::Acquire) {
            gate.socket_directory_admitted.add_permits(1);
            gate.socket_directory_release.acquire().await.expect("directory socket admission test release").forget();
        }
    }
    let mut delivery_invalidations = state.directory_service.subscribe_delivery_invalidations();
    let delivery_space_id = match &record.audience {
        SocketAudienceV1::DirectoryScoped(scope) => Some(scope.space_id.as_str()),
        SocketAudienceV1::Directory { .. } | SocketAudienceV1::Document(_) => None,
    };
    let delivery_epoch = state.directory_service.delivery_epoch(delivery_space_id);
    let mut live = state.directory_service.subscribe();
    let replay = match load_all_directory_events(state.directory.as_ref(), since).await {
        Ok(events) => events,
        Err(_) => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "directory-replay-unavailable".into() }))).await;
            return;
        }
    };
    let mut last_replayed = since;
    let mut created_on_socket = BTreeSet::new();
    for event in replay {
        let seq = event.seq;
        let message = DirectoryStreamMessage::Event { event: Box::new(event) };
        let access = directory_reader_access_change(&record, &created_on_socket, &message);
        for frame in std::iter::once(&message).chain(access.as_ref()) {
            match send_socket_directory_message(&mut sender, &state, &record, &live_lease.id, delivery_space_id, delivery_epoch, frame).await {
                ScopedDirectoryFrameDecisionV1::Deliver => {
                    if matches!(frame, DirectoryStreamMessage::Event { .. }) {
                        last_replayed = last_replayed.max(seq);
                        note_directory_space_created(&mut created_on_socket, frame);
                    }
                }
                ScopedDirectoryFrameDecisionV1::SkipUnrelated => {}
                ScopedDirectoryFrameDecisionV1::CloseUnauthorized => {
                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                    return;
                }
                ScopedDirectoryFrameDecisionV1::CloseUnavailable => {
                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                    return;
                }
            }
        }
    }
    if state.directory_service.delivery_epoch(delivery_space_id) != delivery_epoch {
        let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "directory-replay-invalidated".into() }))).await;
        return;
    }
    let mut authorization_tick = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        tokio::select! {
            () = drain.closing() => {
                let _ = tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(hub_shutdown_close_frame())).await;
                break;
            }
            invalidation = delivery_invalidations.recv() => match invalidation {
                Ok(_) => {
                    if state.directory_service.delivery_epoch(delivery_space_id) != delivery_epoch {
                        let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "directory-delivery-invalidated".into() }))).await;
                        break;
                    }
                }
                Err(broadcast::error::RecvError::Lagged(_) | broadcast::error::RecvError::Closed) => {
                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "directory-delivery-invalidated".into() }))).await;
                    break;
                }
            },
            _ = authorization_tick.tick() => match socket_live_authority(&state, &record, &live_lease.id).await {
                Ok(_) => {}
                Err(SocketBindingValidityV1::Unauthorized) => {
                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                    break;
                }
                Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                    break;
                }
            },
            incoming = receiver.next() => match incoming {
                Some(Ok(Message::Close(_))) | None => break,
                Some(Ok(Message::Ping(payload))) => if sender.send(Message::Pong(payload)).await.is_err() { break },
                Some(Ok(_)) => {}
                Some(Err(_)) => break,
            },
            message = live.recv() => match message {
                Ok(message) => {
                    let seq = match &message { DirectoryStreamMessage::Event { event } => Some(event.seq), _ => None };
                    if seq.is_some_and(|seq| seq <= last_replayed) {
                        continue;
                    }
                    let access = directory_reader_access_change(&record, &created_on_socket, &message);
                    let mut closed = false;
                    for frame in std::iter::once(&message).chain(access.as_ref()) {
                        match send_socket_directory_message(&mut sender, &state, &record, &live_lease.id, delivery_space_id, delivery_epoch, frame).await {
                            ScopedDirectoryFrameDecisionV1::Deliver => {
                                if matches!(frame, DirectoryStreamMessage::Event { .. }) {
                                    if let Some(seq) = seq { last_replayed = last_replayed.max(seq); }
                                    note_directory_space_created(&mut created_on_socket, frame);
                                }
                            }
                            ScopedDirectoryFrameDecisionV1::SkipUnrelated => {}
                            ScopedDirectoryFrameDecisionV1::CloseUnauthorized => {
                                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                                closed = true;
                                break;
                            }
                            ScopedDirectoryFrameDecisionV1::CloseUnavailable => {
                                let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                                closed = true;
                                break;
                            }
                        }
                    }
                    if closed {
                        break;
                    }
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    let validity = match scope.as_ref() {
                        Some(scope) => send_socket_directory_rebootstrap(&mut sender, &state, &record, &live_lease.id, delivery_space_id, delivery_epoch, scope).await,
                        None => match socket_live_authority(&state, &record, &live_lease.id).await {
                            Ok(_) => SocketBindingValidityV1::Active,
                            Err(validity) => validity,
                        },
                    };
                    match validity {
                        SocketBindingValidityV1::Active => {
                            let _ = tokio::time::timeout(
                                std::time::Duration::from_secs(2),
                                sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "rebootstrap-required".into() }))),
                            )
                            .await;
                        }
                        SocketBindingValidityV1::Unauthorized => { let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await; }
                        SocketBindingValidityV1::Unavailable => { let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await; }
                    }
                    break;
                }
                Err(broadcast::error::RecvError::Closed) => break,
            },
            _ = live_lease.notify.notified() => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                break;
            }
        }
    }
}
//#endregion 🔖️Directory
