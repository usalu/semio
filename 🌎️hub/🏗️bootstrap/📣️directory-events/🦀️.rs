//! 📣️ `directory_events` — directory event pages, event stream and directory WebSockets.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use directory::os_directory::io::text::access_policy::directory_access_permits;
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
use semio_hub::artifact_authority::creation::ArtifactCreationCommitAuthorityV1;
use directory::os_directory::access_policy::DirectoryAccessActionV1;
use semio_hub::directory::error::DirectoryError;
use semio_hub::directory::model::{DirectoryPrincipalV1, DocumentScope};
use semio_hub::directory::{HubDirectories, HubDirectory, DIRECTORY_EVENT_READ_MAX, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS, LocalBootstrapTransport, AUTH_TEXT_MAX_BYTES};
use semio_hub::inference::runtime::GisMapApprovalIngressAuthorityV1;
use serde::Deserialize;
use std::collections::BTreeSet;
use std::sync::Arc;
use tokio::sync::broadcast;
use crate::support::{DirectoryJson, now_ms, bearer, directory_error_status};
use crate::state::HubState;
use crate::socket_grants::{SOCKET_PROTOCOL_V1, SocketBindingKeyV1, SocketAudienceV1, SocketSubjectV1, SocketBindingValidityV1, SocketGrantRecordV1, SocketGrantAdmissionV1, SocketLiveLeaseV1, socket_live_authority_with_bindings};
use crate::guard::session_access_roles;
use crate::access_grants::{socket_text_bounded, socket_binding_validity};
#[cfg(test)]
use crate::access_grants::pause_global_directory_send_for_test;
use crate::document_socket::{consume_scoped_directory_socket_grant, consume_directory_socket_grant, verified_rebootstrap_control};
use crate::directory_commands::{AuthedUser, resolve_bearer_user, resolve_optional_bearer_user, CredentialOptionalRefusalV1, role_wire, DirectorySpaceAccessDecisionV1, directory_space_access_decision};
use crate::lifecycle::hub_shutdown_close_frame;
use db::db_storage::PayloadStorage as _;

//#region 🔖️Directory
pub(crate) trait DirectoryEventPageSource {
    async fn directory_event_head(&self) -> Result<u64, DirectoryError>;
    async fn directory_event_page(&self, since: u64, limit: usize) -> Result<Vec<DirectoryEvent>, DirectoryError>;
}

impl DirectoryEventPageSource for HubDirectories {
    async fn directory_event_head(&self) -> Result<u64, DirectoryError> {
        HubDirectory::head_seq(self).await
    }

    async fn directory_event_page(&self, since: u64, limit: usize) -> Result<Vec<DirectoryEvent>, DirectoryError> {
        HubDirectory::events_since(self, since, limit).await
    }
}

/// 📖️ Reads a bounded complete suffix in fixed pages, cursoring by observed sequence.
pub(crate) async fn load_all_directory_events<S: DirectoryEventPageSource + ?Sized>(directory: &S, since: u64) -> Result<Vec<DirectoryEvent>, DirectoryError> {
    let head = directory.directory_event_head().await?;
    let expected = head.saturating_sub(since);
    if expected > DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS {
        return Err(DirectoryError::Conflict(format!("directory event suffix exceeds fixed maximum {DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS}")));
    }
    let mut cursor = since;
    let capacity = usize::try_from(expected).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
    let mut events = Vec::with_capacity(capacity);
    loop {
        let page = directory.directory_event_page(cursor, DIRECTORY_EVENT_READ_MAX).await?;
        let page_len = page.len();
        if page_len == 0 {
            break;
        }
        for event in &page {
            if event.seq <= cursor {
                return Err(DirectoryError::Backend("directory event page did not strictly advance its cursor".into()));
            }
            cursor = event.seq;
        }
        events.extend(page);
        if u64::try_from(events.len()).map_err(|error| DirectoryError::Conflict(error.to_string()))? > DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS {
            return Err(DirectoryError::Conflict(format!("directory event suffix exceeds fixed maximum {DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS}")));
        }
        if page_len < DIRECTORY_EVENT_READ_MAX {
            break;
        }
    }
    Ok(events)
}

#[derive(Deserialize)]
pub(crate) struct EventsQuery {
    pub(crate) since: Option<u64>,
    pub(crate) limit: Option<usize>,
}

fn directory_event_page_request_admission(uri: &axum::http::Uri) -> Result<u64, StatusCode> {
    let query = uri.query().ok_or(StatusCode::BAD_REQUEST)?;
    if query.contains('&') || query.contains('%') || query.contains('+') {
        return Err(StatusCode::BAD_REQUEST);
    }
    let (name, value) = query.split_once('=').ok_or(StatusCode::BAD_REQUEST)?;
    if name != "after" || value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) || (value.len() > 1 && value.starts_with('0')) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let after = value.parse::<u64>().map_err(|_| StatusCode::BAD_REQUEST)?;
    if after > DOCUMENT_OPEN_MAX_SAFE_INTEGER {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(after)
}

pub(crate) fn directory_event_page_session_binding_v1(caller: &AuthedUser) -> Result<[u8; 32], StatusCode> {
    let session_len = u32::try_from(caller.session_id.len()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let user_len = u32::try_from(caller.user_id.len()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/directory-event-page/session-binding/v1\0");
    hash.update(&session_len.to_be_bytes());
    hash.update(caller.session_id.as_bytes());
    hash.update(&user_len.to_be_bytes());
    hash.update(caller.user_id.as_bytes());
    hash.update(&caller.authorization_generation.to_be_bytes());
    hash.update(&caller.expires_at.to_be_bytes());
    Ok(hash.finalize().into())
}

const DIRECTORY_EVENT_PAGE_DEADLINE_MS: u64 = 5_000;

pub(crate) struct DirectoryEventPageHttpControl {
    pub(crate) cancelled: std::sync::atomic::AtomicBool,
    pub(crate) active: std::sync::atomic::AtomicBool,
}

impl DirectoryEventPageHttpControl {
    pub(crate) fn new() -> Self {
        Self { cancelled: std::sync::atomic::AtomicBool::new(false), active: std::sync::atomic::AtomicBool::new(true) }
    }

    pub(crate) fn checkpoint(&self) -> Result<(), StatusCode> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            Err(StatusCode::SERVICE_UNAVAILABLE)
        } else {
            Ok(())
        }
    }

    pub(crate) fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    pub(crate) fn release(&self) {
        self.active.store(false, std::sync::atomic::Ordering::Release);
    }
}

struct DirectoryEventPageHttpRequest {
    control: Arc<DirectoryEventPageHttpControl>,
    response_owned: bool,
}

impl DirectoryEventPageHttpRequest {
    pub(crate) fn new(control: Arc<DirectoryEventPageHttpControl>) -> Self {
        Self { control, response_owned: false }
    }

    pub(crate) fn finish_response_owned(&mut self) {
        self.response_owned = true;
        self.control.release();
    }
}

impl Drop for DirectoryEventPageHttpRequest {
    fn drop(&mut self) {
        if !self.response_owned {
            self.control.cancel();
            self.control.release();
        }
    }
}

async fn revalidate_directory_event_page_caller(state: &HubState, caller: &AuthedUser, binding: [u8; 32]) -> Result<AuthedUser, StatusCode> {
    let session = state.directory.authenticate_session(&caller.capability).await.map_err(|_| StatusCode::UNAUTHORIZED)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let current = AuthedUser { user_id: session.user_id, session_id: session.id, expires_at: session.expires_at, authorization_generation: session.authorization_generation, capability: caller.capability.clone(), session_kind: session.session_kind, device_instance_id: session.device_instance_id };
    if current.session_id != caller.session_id || current.user_id != caller.user_id || current.authorization_generation != caller.authorization_generation || directory_event_page_session_binding_v1(&current)? != binding {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(current)
}

/// 🧑‍🤝‍🧑 The spaces `principal` holds a role in ([`HubDirectory::principal_ceiling`]), read ONCE per event page:
/// raw directory events are member-only ([`event_visible`]), so this set decides every row of the page. The page used to
/// read the space and the caller's role per event — two directory round trips per row, 256 for a full page (ticket
/// 26/09/23 WG8).
async fn directory_member_space_ids(state: &HubState, principal: DirectoryPrincipalV1<'_>) -> Result<BTreeSet<String>, StatusCode> {
    let ceiling = state.directory.principal_ceiling(principal).await.map_err(directory_error_status)?;
    Ok(state.directory.list_spaces_for_user(principal.user_id).await.map_err(directory_error_status)?.into_iter().filter(|(space, role)| ceiling.cap(&space.id, Some(*role)).is_some()).map(|(space, _)| space.id).collect())
}

/// 🌐️ Which lane a directory event page serves: the directory (every event the caller may see, never a preference) or the
/// caller's own preferences (`user.preference-recorded` of the caller, nothing else). Both share one seq, one receipt and
/// one page machinery; an event of the other lane is skipped like an invisible one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DirectoryEventLaneV1 {
    Directory,
    Preferences,
}

/// 🌐️ The page lane an event belongs to: a `user.preference-recorded` event rides only the preference lane.
fn directory_event_lane_v1(event: &DirectoryEvent) -> DirectoryEventLaneV1 {
    match event.body {
        os_directory::DirectoryEventBody::UserPreferenceRecorded { .. } => DirectoryEventLaneV1::Preferences,
        _ => DirectoryEventLaneV1::Directory,
    }
}

fn directory_event_page_event_visible(member_spaces: &BTreeSet<String>, event: &DirectoryEvent, caller: &AuthedUser) -> bool {
    match event.space_id.as_deref() {
        Some(space_id) => member_spaces.contains(space_id),
        None => event.user_id.as_deref() == Some(caller.user_id.as_str()),
    }
}

/// 📏️ An upper bound of the sealed page's JSON length: `envelope_bytes` is the page with no events at its
/// widest (`throughSeqInclusive` = `u64::MAX`, `hasMore` = `false`), plus each event's own JSON and one comma between
/// two. A page within this bound fits without sealing it; only a page near the limit is sealed exactly, so building a
/// page is linear in its rows (every row used to re-seal the whole page so far: quadratic).
fn directory_event_page_bytes_bound(envelope_bytes: usize, event_bytes: usize, events: usize) -> usize {
    envelope_bytes.saturating_add(event_bytes).saturating_add(events.saturating_sub(1))
}

fn seal_directory_event_page_v1(binding: [u8; 32], generation: u64, after: u64, through: u64, has_more: bool, events: Vec<DirectoryEvent>) -> Result<DirectoryEventPageV1, DirectoryEventPageErrorV1> {
    let mut page = DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: os_directory::io::binary::artifact_hash::hex_lower(&binding),
        authorization_generation: generation,
        after_seq_exclusive: after,
        through_seq_inclusive: through,
        has_more,
        events,
        receipt_sha256: String::new(),
    };
    page.receipt_sha256 = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(page.canonical_unsigned_json().as_bytes()));
    if semio_framework_pack_json::to_json_string(&page).len() > DIRECTORY_EVENT_PAGE_MAX_BYTES {
        return Err(DirectoryEventPageErrorV1::TooLarge);
    }
    page.validate()?;
    Ok(page)
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

pub(crate) async fn get_directory_event_page_v1(OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<DirectoryEventPageV1>, StatusCode> {
    serve_directory_event_page_v1(uri, headers, state, DirectoryEventLaneV1::Directory).await
}

/// 🌐️ `GET /directory/preference-page/v1?after=` — the caller's own preference lane on the directory page machinery.
pub(crate) async fn get_directory_preference_page_v1(OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<DirectoryEventPageV1>, StatusCode> {
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
pub(crate) async fn directory_message_visible(state: &HubState, message: &DirectoryStreamMessage, caller: Option<&AuthedUser>) -> bool {
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

pub(crate) async fn get_directory_events(Query(query): Query<EventsQuery>, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<Vec<DirectoryEvent>>, CredentialOptionalRefusalV1> {
    let caller = resolve_optional_bearer_user(&state, &headers).await?;
    let events = state.directory.events_since(query.since.unwrap_or(0), query.limit.unwrap_or(500)).await.map_err(directory_error_status)?;
    Ok(DirectoryJson(visibility_filter_events(&state, events, caller.as_ref()).await?))
}

/// 🌐️ Outbound global telemetry borrows a space authority without indexing the global lease by it.
pub(crate) fn directory_stream_message_space(message: &DirectoryStreamMessage) -> Option<&str> {
    match message {
        DirectoryStreamMessage::Event { event } => event.space_id.as_deref(),
        DirectoryStreamMessage::Connection { connection, .. } => Some(&connection.space_id),
        DirectoryStreamMessage::Presence { space_id, .. } => Some(space_id),
        DirectoryStreamMessage::RebootstrapRequired { control } => Some(&control.scope.space_id),
        DirectoryStreamMessage::Heartbeat { .. } | DirectoryStreamMessage::AccessChanged { .. } => None,
    }
}

/// 🔑️ The `access-changed` frame one delivered-or-skipped directory event owes a GLOBAL directory socket's own reader
/// ([`directory_access_change_for_reader`]); scoped sockets and non-session subjects owe none.
fn directory_reader_access_change(record: &SocketGrantRecordV1, created_on_socket: &BTreeSet<String>, message: &DirectoryStreamMessage) -> Option<DirectoryStreamMessage> {
    let (SocketAudienceV1::Directory { .. }, SocketSubjectV1::Session { user_id, .. }) = (&record.audience, &record.subject) else { return None };
    directory_access_change_for_reader(user_id, created_on_socket, message)
}

/// 🔑️ A grant or redemption naming `reader_user_id` for a space whose `space.created` the socket never delivered to it
/// (the space's earlier events became readable behind the reader's cursor), or a removal naming it (its events became
/// unreadable) — the reader's visible directory moved, so it re-reads from the origin. Law: fixture `🔑️access-changed-v1`.
pub(crate) fn directory_access_change_for_reader(reader_user_id: &str, created_on_socket: &BTreeSet<String>, message: &DirectoryStreamMessage) -> Option<DirectoryStreamMessage> {
    let DirectoryStreamMessage::Event { event } = message else { return None };
    let (space_id, change) = match &event.body {
        os_directory::DirectoryEventBody::MemberUpserted { space_id, user_id: member, .. } | os_directory::DirectoryEventBody::InviteRedeemed { space_id, user_id: member, .. } if member == reader_user_id && !created_on_socket.contains(space_id) => (space_id, os_directory::DirectoryAccessChange::Granted),
        os_directory::DirectoryEventBody::MemberRemoved { space_id, user_id: member } if member == reader_user_id => (space_id, os_directory::DirectoryAccessChange::Revoked),
        _ => return None,
    };
    Some(DirectoryStreamMessage::AccessChanged { space_id: space_id.clone(), change })
}

/// 🏗️ Remembers every space whose `space.created` a global directory socket delivered, so a membership in it is not an
/// access change for that reader (it already read the space from its first event).
fn note_directory_space_created(created_on_socket: &mut BTreeSet<String>, message: &DirectoryStreamMessage) {
    if let DirectoryStreamMessage::Event { event } = message {
        if let os_directory::DirectoryEventBody::SpaceCreated { space_id, .. } = &event.body {
            created_on_socket.insert(space_id.clone());
        }
    }
}

fn directory_space_message_bindings(record: &SocketGrantRecordV1, space_id: Option<&str>) -> Vec<SocketBindingKeyV1> {
    let mut bindings = record.bindings();
    if let (SocketAudienceV1::Directory { .. }, SocketSubjectV1::Session { user_id, .. }, Some(space_id)) = (&record.audience, &record.subject, space_id) {
        bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.to_owned() });
        bindings.push(SocketBindingKeyV1::Membership { user_id: user_id.clone(), space_id: space_id.to_owned() });
    }
    bindings.sort();
    bindings.dedup();
    bindings
}

pub(crate) fn directory_message_bindings(record: &SocketGrantRecordV1, message: &DirectoryStreamMessage) -> Vec<SocketBindingKeyV1> {
    directory_space_message_bindings(record, directory_stream_message_space(message))
}

pub(crate) async fn socket_directory_membership_visibility(state: &HubState, record: &SocketGrantRecordV1, message: &DirectoryStreamMessage) -> SocketBindingValidityV1 {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScopedDirectoryFrameDecisionV1 {
    Deliver,
    SkipUnrelated,
    CloseUnauthorized,
    CloseUnavailable,
}

pub(crate) fn directory_message_matches_scope(scope: &DocumentScope, message: &DirectoryStreamMessage) -> bool {
    match message {
        DirectoryStreamMessage::Event { event } => match &event.body {
            os_directory::DirectoryEventBody::DocumentAnnounced { descriptor } => descriptor.space_id == scope.space_id && descriptor.document_id == scope.document_id,
            os_directory::DirectoryEventBody::DocumentIndexed { scope: indexed, .. } => indexed == scope,
            os_directory::DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => checkpoint.scope == *scope,
            os_directory::DirectoryEventBody::ArtifactRetentionAdvanced { retention } => retention.scope == *scope,
            os_directory::DirectoryEventBody::UserCreated { .. }
            | os_directory::DirectoryEventBody::SpaceCreated { .. }
            | os_directory::DirectoryEventBody::SpaceRenamed { .. }
            | os_directory::DirectoryEventBody::SpaceVisibilityChanged { .. }
            | os_directory::DirectoryEventBody::SpaceArchived { .. }
            | os_directory::DirectoryEventBody::SpaceDeleted { .. }
            | os_directory::DirectoryEventBody::MemberUpserted { .. }
            | os_directory::DirectoryEventBody::MemberRemoved { .. }
            | os_directory::DirectoryEventBody::InviteRedeemed { .. }
            | os_directory::DirectoryEventBody::UserPreferenceRecorded { .. } => false,
        },
        DirectoryStreamMessage::Connection { connection, .. } => connection.space_id == scope.space_id && connection.document_id == scope.document_id,
        DirectoryStreamMessage::Presence { space_id, document_id, .. } => space_id == &scope.space_id && document_id == &scope.document_id,
        DirectoryStreamMessage::Heartbeat { .. } | DirectoryStreamMessage::AccessChanged { .. } => false,
        DirectoryStreamMessage::RebootstrapRequired { control } => control.scope == *scope,
    }
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DirectoryWsV1Query {
    #[serde(default)]
    pub(crate) since: u64,
    pub(crate) space_id: Option<String>,
    pub(crate) document_id: Option<String>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DirectoryScopedWsV1Query {
    #[serde(default)]
    pub(crate) since: u64,
}

/// 📝️ The directory lane's admission span (`server.directory.socket`), the same shape the
/// document lane reports: one record for the upgrade decision and one for the session's lifetime.
pub(crate) async fn directory_ws_v1(ws: WebSocketUpgrade, Query(query): Query<DirectoryWsV1Query>, headers: HeaderMap, State(state): State<HubState>) -> Response {
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

pub(crate) async fn directory_scoped_ws_v1(ws: WebSocketUpgrade, Path((space_id, document_id)): Path<(String, String)>, Query(query): Query<DirectoryScopedWsV1Query>, headers: HeaderMap, State(state): State<HubState>) -> Response {
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

async fn send_directory_message(sender: &mut SplitSink<WebSocket, Message>, message: &DirectoryStreamMessage) -> bool {
    let text = semio_framework_pack_json::to_json_string(message);
    sender.send(Message::Text(text.into())).await.is_ok()
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
