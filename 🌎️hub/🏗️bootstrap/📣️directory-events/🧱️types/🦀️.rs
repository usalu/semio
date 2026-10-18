//! 🧱️ The `directory_events` vocabulary that carries no `HubState`: types, ledgers and helpers the state and the routes build on.

use std::sync::Arc;
use crate::directory_commands::AuthedUser;
use std::collections::BTreeSet;
use directory::os_directory::{DIRECTORY_EVENT_PAGE_MAX_BYTES, DOCUMENT_OPEN_MAX_SAFE_INTEGER, DirectoryEvent, DirectoryEventPageErrorV1, DirectoryEventPageV1, DirectoryStreamMessage, DocumentScope};
use semio_hub_directory_contract::{DIRECTORY_EVENT_READ_MAX, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS};
use serde::Deserialize;
use semio_hub_directory_contract::error::DirectoryError;
use semio_hub_directory_dispatch::HubDirectories;
use semio_hub_directory::EventLog;
use axum::extract::ws::{Message, WebSocket};
use semio_framework_hash::Sha256;
use semio_hub_ledgers::socket_grants::{SocketAudienceV1, SocketBindingKeyV1, SocketGrantRecordV1, SocketSubjectV1};
use futures::stream::SplitSink;
use axum::http::StatusCode;
use directory::os_directory;

//#region 🔖️Directory
pub trait DirectoryEventPageSource {
    async fn directory_event_head(&self) -> Result<u64, DirectoryError>;
    async fn directory_event_page(&self, since: u64, limit: usize) -> Result<Vec<DirectoryEvent>, DirectoryError>;
}

/// 📖️ Reads a bounded complete suffix in fixed pages, cursoring by observed sequence.
pub async fn load_all_directory_events<S: DirectoryEventPageSource + ?Sized>(directory: &S, since: u64) -> Result<Vec<DirectoryEvent>, DirectoryError> {
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
pub struct EventsQuery {
    pub since: Option<u64>,
    pub limit: Option<usize>,
}

pub fn directory_event_page_request_admission(uri: &axum::http::Uri) -> Result<u64, StatusCode> {
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

pub fn directory_event_page_session_binding_v1(caller: &AuthedUser) -> Result<[u8; 32], StatusCode> {
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

pub const DIRECTORY_EVENT_PAGE_DEADLINE_MS: u64 = 5_000;

pub struct DirectoryEventPageHttpControl {
    pub cancelled: std::sync::atomic::AtomicBool,
    pub active: std::sync::atomic::AtomicBool,
}

impl DirectoryEventPageHttpControl {
    pub fn new() -> Self {
        Self { cancelled: std::sync::atomic::AtomicBool::new(false), active: std::sync::atomic::AtomicBool::new(true) }
    }

    pub fn checkpoint(&self) -> Result<(), StatusCode> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            Err(StatusCode::SERVICE_UNAVAILABLE)
        } else {
            Ok(())
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn release(&self) {
        self.active.store(false, std::sync::atomic::Ordering::Release);
    }
}

pub struct DirectoryEventPageHttpRequest {
    pub control: Arc<DirectoryEventPageHttpControl>,
    response_owned: bool,
}

impl DirectoryEventPageHttpRequest {
    pub fn new(control: Arc<DirectoryEventPageHttpControl>) -> Self {
        Self { control, response_owned: false }
    }

    pub fn finish_response_owned(&mut self) {
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

/// 🌐️ Which lane a directory event page serves: the directory (every event the caller may see, never a preference) or the
/// caller's own preferences (`user.preference-recorded` of the caller, nothing else). Both share one seq, one receipt and
/// one page machinery; an event of the other lane is skipped like an invisible one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectoryEventLaneV1 {
    Directory,
    Preferences,
}

/// 🌐️ The page lane an event belongs to: a `user.preference-recorded` event rides only the preference lane.
pub fn directory_event_lane_v1(event: &DirectoryEvent) -> DirectoryEventLaneV1 {
    match event.body {
        os_directory::DirectoryEventBody::UserPreferenceRecorded { .. } => DirectoryEventLaneV1::Preferences,
        _ => DirectoryEventLaneV1::Directory,
    }
}

pub fn directory_event_page_event_visible(member_spaces: &BTreeSet<String>, event: &DirectoryEvent, caller: &AuthedUser) -> bool {
    match event.space_id.as_deref() {
        Some(space_id) => member_spaces.contains(space_id),
        None => event.user_id.as_deref() == Some(caller.user_id.as_str()),
    }
}

/// 📏️ An upper bound of the sealed page's JSON length: `envelope_bytes` is the page with no events at its
/// widest (`throughSeqInclusive` = `u64::MAX`, `hasMore` = `false`), plus each event's own JSON and one comma between
/// two. A page within this bound fits without sealing it; only a page near the limit is sealed exactly, so building a
/// page is linear in its rows (every row used to re-seal the whole page so far: quadratic).
pub fn directory_event_page_bytes_bound(envelope_bytes: usize, event_bytes: usize, events: usize) -> usize {
    envelope_bytes.saturating_add(event_bytes).saturating_add(events.saturating_sub(1))
}

pub fn seal_directory_event_page_v1(binding: [u8; 32], generation: u64, after: u64, through: u64, has_more: bool, events: Vec<DirectoryEvent>) -> Result<DirectoryEventPageV1, DirectoryEventPageErrorV1> {
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

/// 🌐️ Outbound global telemetry borrows a space authority without indexing the global lease by it.
pub fn directory_stream_message_space(message: &DirectoryStreamMessage) -> Option<&str> {
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
pub fn directory_reader_access_change(record: &SocketGrantRecordV1, created_on_socket: &BTreeSet<String>, message: &DirectoryStreamMessage) -> Option<DirectoryStreamMessage> {
    let (SocketAudienceV1::Directory { .. }, SocketSubjectV1::Session { user_id, .. }) = (&record.audience, &record.subject) else { return None };
    directory_access_change_for_reader(user_id, created_on_socket, message)
}

/// 🔑️ A grant or redemption naming `reader_user_id` for a space whose `space.created` the socket never delivered to it
/// (the space's earlier events became readable behind the reader's cursor), or a removal naming it (its events became
/// unreadable) — the reader's visible directory moved, so it re-reads from the origin. Law: fixture `🔑️access-changed-v1`.
pub fn directory_access_change_for_reader(reader_user_id: &str, created_on_socket: &BTreeSet<String>, message: &DirectoryStreamMessage) -> Option<DirectoryStreamMessage> {
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
pub fn note_directory_space_created(created_on_socket: &mut BTreeSet<String>, message: &DirectoryStreamMessage) {
    if let DirectoryStreamMessage::Event { event } = message {
        if let os_directory::DirectoryEventBody::SpaceCreated { space_id, .. } = &event.body {
            created_on_socket.insert(space_id.clone());
        }
    }
}

pub fn directory_space_message_bindings(record: &SocketGrantRecordV1, space_id: Option<&str>) -> Vec<SocketBindingKeyV1> {
    let mut bindings = record.bindings();
    if let (SocketAudienceV1::Directory { .. }, SocketSubjectV1::Session { user_id, .. }, Some(space_id)) = (&record.audience, &record.subject, space_id) {
        bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.to_owned() });
        bindings.push(SocketBindingKeyV1::Membership { user_id: user_id.clone(), space_id: space_id.to_owned() });
    }
    bindings.sort();
    bindings.dedup();
    bindings
}

pub fn directory_message_bindings(record: &SocketGrantRecordV1, message: &DirectoryStreamMessage) -> Vec<SocketBindingKeyV1> {
    directory_space_message_bindings(record, directory_stream_message_space(message))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopedDirectoryFrameDecisionV1 {
    Deliver,
    SkipUnrelated,
    CloseUnauthorized,
    CloseUnavailable,
}

pub fn directory_message_matches_scope(scope: &DocumentScope, message: &DirectoryStreamMessage) -> bool {
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
pub struct DirectoryWsV1Query {
    #[serde(default)]
    pub since: u64,
    pub space_id: Option<String>,
    pub document_id: Option<String>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectoryScopedWsV1Query {
    #[serde(default)]
    pub since: u64,
}

pub async fn send_directory_message(sender: &mut SplitSink<WebSocket, Message>, message: &DirectoryStreamMessage) -> bool {
    let text = semio_framework_pack_json::to_json_string(message);
    sender.send(Message::Text(text.into())).await.is_ok()
}
//#endregion 🔖️Directory

//#region 🔖️Directory
impl DirectoryEventPageSource for HubDirectories {
    async fn directory_event_head(&self) -> Result<u64, DirectoryError> {
        EventLog::head_seq(self).await
    }

    async fn directory_event_page(&self, since: u64, limit: usize) -> Result<Vec<DirectoryEvent>, DirectoryError> {
        EventLog::events_since(self, since, limit).await
    }
}
//#endregion 🔖️Directory
