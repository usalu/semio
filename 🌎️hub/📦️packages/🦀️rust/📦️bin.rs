//! OS hub backend — thin axum shell over `db::Database` (document authority + content-addressed
//! blobs) and `HubDirectory` (identity/tenancy), speaking `protocol_wire`'s binary frames over
//! WebSocket.
//!
//! The WebSocket endpoint speaks `protocol_wire`'s binary lane-tagged `ClientFrame`/`ServerFrame`
//! frames directly (see `protocol/wire/rs/lib.rs`) — the server-side counterpart to
//! `framework/sync`'s client actors (CW5). Command-lane persistence/ordering flows through
//! `db::Database::hello`/`ArtifactHandle::submit`/`db::sync::handle_frontier_advertise`;
//! preview-lane and presence frames are ephemeral, best-effort fan-out this crate owns directly via
//! a per-document `tokio::sync::broadcast` registry (never durable, matching the preview lane's
//! contract). "Space" is a namespacing convention this crate applies on top of `db`'s flat document
//! catalog (`{space_id}:{document_id}`), not hub-internal state.

use axum::body::Bytes;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use dashmap::DashMap;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use directory::os_directory::{
    self, ConnectionView, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryConnectionPhase, DirectoryEvent, DirectoryPresenceActor, DirectoryReadModel, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentView, InviteView, MemberView, SpaceView,
};
use protocol::{decode_client_frame, encode_server_frame, AckStage, ActorId, ApplyOutcome, ClientFrame, ArtifactId as ProtocolArtifactId, Lane, MutationEnvelope, RuntimeFrontierSummary, ServerFrame};
#[cfg(feature = "sqlite")]
use semio_hub::directory::sqlite::SqliteDirectory;
use semio_hub::directory::error::DirectoryError;
use semio_hub::directory::model::{InviteRecord, SpaceRole, SyncSessionRecord};
use semio_hub::directory::{CommandResult, DirectoryService, HubDirectory};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast;

//#region ⚠️ Errors
/// @emoji 🧯️ Top-level startup error — the only fallible paths outside a document/WS session are
/// opening `db::Database`'s storage backend, connecting the directory backend, and binding the
/// HTTP listener.
#[derive(Debug, thiserror::Error)]
enum HubError {
    #[error(transparent)]
    Directory(#[from] DirectoryError),
    #[error(transparent)]
    Db(#[from] db::DbError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("unknown OS_HUB_STORAGE_BACKEND: {0}")]
    UnknownStorageBackend(String),
    #[error("unknown OS_HUB_DIRECTORY_BACKEND: {0}")]
    UnknownDirectoryBackend(String),
}
//#endregion ⚠️ Errors

fn now_ms() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

//#region 🔖️State
/// @emoji 🎫️ `(space_id, document_id)` -> the single string key both `db::Database`'s flat
/// document catalog and this crate's own fanout/presence registries key on — space scoping is a
/// convention this crate applies on top of `db`'s namespace, not something `db` itself knows about.
fn scope_key(space_id: &str, document_id: &str) -> String {
    format!("{space_id}:{document_id}")
}

fn db_artifact_id(space_id: &str, document_id: &str) -> ProtocolArtifactId {
    ProtocolArtifactId(scope_key(space_id, document_id))
}

fn db_core_document_id(id: &ProtocolArtifactId) -> db::ArtifactId {
    db::ArtifactId(id.0.clone())
}

/// @emoji 👤️ One connected actor's presence in a document (contract §C7.3) — inserted with `peer:
/// None` right after the hub sends `ServerFrame::Session`, updated to `peer: Some(bytes)` on
/// `ClientFrame::Presence`, removed at handler exit. `surface`/`color` are fixed for the life of the
/// connection (stamped at handshake time); the hub never decodes `peer` — it stores and forwards it
/// opaquely.
struct PresenceSession {
    surface: String,
    user_id: Option<String>,
    color: u8,
    peer: Option<Vec<u8>>,
}

/// @emoji 🎨️ One actor's held palette index within a space, ref-counted across that actor's
/// concurrently open document sockets in the same space (contract §C7.3: "A's second document socket
/// keeps 0").
struct ColorLease {
    index: u8,
    refs: u32,
}

/// @emoji 🌈️ One space's live session-color leases (contract §C7.3) — never persisted, rebuilt from
/// nothing on hub restart, mirroring `presence`'s own ephemeral law.
#[derive(Default)]
struct SpaceColors {
    by_actor: std::collections::BTreeMap<String, ColorLease>,
}

#[derive(Clone)]
struct HubState {
    db: Arc<db::Database>,
    directory: Arc<dyn HubDirectory>,
    /// @emoji 🏭️ Wave 1.B: the single serialized directory writer (contract §C1's decider laws +
    /// dense event `seq`) built once over `directory` at startup — see `semio_hub::directory::
    /// DirectoryService`'s own doc. `/directory/commands` and `/directory/invites/{token}/redeem`
    /// go through this; every other `/directory/*` route reads `directory` directly.
    directory_service: Arc<DirectoryService>,
    admin_token: Option<String>,
    /// @emoji 🛡️ Contract §C0 `OS_HUB_ADMIN_DIR`: the admin SPA's static asset root. Lane 2-E owns
    /// the actual `/admin` file-serving handler (and its 503-if-missing stub) — this lane only
    /// carries the resolved path through `HubState` so that handler has something to read.
    // 🌵️ Unread until 2-E's handler lands and calls `state.admin_dir` — not dead code, just not
    // wired to a route yet (explicitly out of this lane's scope, see the doc above).
    #[allow(dead_code)]
    admin_dir: std::path::PathBuf,
    /// @emoji 📡️ Command-lane + preview-lane fan-out, one `broadcast::Sender` per `scope_key` —
    /// `db::Database`'s own `ArtifactHandle` exposes no live-subscription seam yet (see
    /// `db_engine`'s module doc: `subscribe`/`preview` are honest `Unimplemented` extension seams),
    /// so relaying newly-committed commands / preview blobs / presence updates to other connected
    /// sessions on the same document is this crate's own, deliberately thin responsibility — it
    /// never itself decides ordering or durability, only re-broadcasts what `db` already committed
    /// or what a preview/presence frame carries verbatim.
    fanout: Arc<DashMap<String, broadcast::Sender<ServerFrame>>>,
    /// @emoji 👥️ `(scope_key, actor)` -> that actor's presence session (contract §C7.3) — ephemeral,
    /// never durable (mirrors the preview lane's own law), rebuilt from nothing on hub restart. The
    /// roster is document-wide now (contract §C7.0): `ServerFrame::Presence` fans out on `fanout`, not
    /// a surface-scoped channel; a peer's `surface` travels INSIDE its `PresencePeer` bytes, stamped
    /// by the client actor, never decoded by this hub.
    presence: Arc<DashMap<(String, String), PresenceSession>>,
    /// @emoji 🎨️ Contract §C7.3 session colors: `space_id` -> that space's live `(actor -> palette
    /// index)` leases. `acquire_color`/`release_color` below are the only mutators. Never persisted.
    session_colors: Arc<DashMap<String, SpaceColors>>,
    /// @emoji 🦵️ Wave 1.B admin kick: `syncSessionId` (the `SyncSessionRecord.id`/`ConnectionView.
    /// syncSessionId` the directory hands out on connect) -> a `Notify` the WS loop `select!`s on
    /// alongside its socket/broadcast reads. `POST /admin/api/connections/{syncSessionId}/close`
    /// fires it; the loop observes the wake-up and closes the connection on its own next tick —
    /// this map never itself closes a socket, only signals the session that owns it to.
    session_kicks: Arc<DashMap<String, Arc<tokio::sync::Notify>>>,
    /// @emoji 🧬️ W5.7: `scope_key` -> the first non-zero `store::ArtifactCodec::pack_schema_hash`
    /// a client's `Hello` declared for that document — pinned in-memory, never durable (durable
    /// pinning belongs in the db catalog once it grows a column for it; this wave's scope is the
    /// in-memory pin only). A later `Hello` with a different non-zero hash for the same document is
    /// rejected with an `error_frame("schema-hash-mismatch", ...)` before `Welcome` — catches two
    /// builds of the same app disagreeing on a document's field shape. A zero hash always skips
    /// validation (schema-agnostic client, see `ArtifactCodec::pack_schema_hash`'s own doc).
    schema_hashes: Arc<DashMap<String, [u8; 32]>>,
    /// @emoji 🧩️ Wave 1.B: installed runtime extensions mirrored from dev `/extensions` static dir —
    /// populated by hub deploy copy / sideload; `GET /extensions` lists `install.json` rows.
    extensions_root: std::path::PathBuf,
    /// @emoji ⚖️ Contract §C9 ("hub `📦️bin.rs`: policy from config → `SubmitOptions.policy`"): the
    /// authority-local `protocol::MergePolicy` every `submit_commands` call on this hub instance
    /// judges a batch's worst graded conflict/message level against — read once at startup from
    /// `OS_HUB_MERGE_POLICY` (see `merge_policy_from_env`'s doc), never per-connection/per-space,
    /// matching `protocol::MergePolicy`'s own "local/authority state, never on the wire" law.
    merge_policy: protocol::MergePolicy,
}

impl HubState {
    fn fanout_for(&self, key: &str) -> broadcast::Sender<ServerFrame> {
        if let Some(existing) = self.fanout.get(key) {
            return existing.clone();
        }
        let (tx, _rx) = broadcast::channel(256);
        self.fanout.entry(key.to_string()).or_insert(tx).clone()
    }

    /// @emoji 👥️ The document-wide roster's raw peer bytes (contract §C7.3) — entries whose `peer` is
    /// still `None` (handshake-only, no `ClientFrame::Presence` published yet) are excluded.
    fn presence_peers(&self, key: &str) -> Vec<Vec<u8>> {
        self.presence.iter().filter(|entry| entry.key().0 == key).filter_map(|entry| entry.value().peer.clone()).collect()
    }

    /// @emoji 📡️ Amendment 3 to C1: the SAME roster as `presence_peers`, shaped as
    /// `DirectoryPresenceActor`s the hub already knows without ever decoding a peer's bytes.
    fn directory_presence_actors(&self, key: &str) -> Vec<DirectoryPresenceActor> {
        self.presence
            .iter()
            .filter(|entry| entry.key().0 == key && entry.value().peer.is_some())
            .map(|entry| DirectoryPresenceActor { actor: entry.key().1.clone(), user_id: entry.value().user_id.clone(), surface: entry.value().surface.clone(), color: entry.value().color })
            .collect()
    }

    /// @emoji 🎨️ Contract §C7.3: an existing lease for `actor` in `space` is ref-counted and its
    /// index reused; otherwise the lowest index in `0..=255` not currently held by any live actor of
    /// `space`, wrapping `n % 256` once all 256 are taken.
    fn acquire_color(&self, space: &str, actor: &str) -> u8 {
        let mut colors = self.session_colors.entry(space.to_string()).or_default();
        if let Some(lease) = colors.by_actor.get_mut(actor) {
            lease.refs += 1;
            return lease.index;
        }
        let used: std::collections::BTreeSet<u8> = colors.by_actor.values().map(|lease| lease.index).collect();
        let index = (0..=255u8).find(|candidate| !used.contains(candidate)).unwrap_or((colors.by_actor.len() as u32 % 256) as u8);
        colors.by_actor.insert(actor.to_string(), ColorLease { index, refs: 1 });
        index
    }

    /// @emoji 🎨️ `refs -= 1`, dropping the lease at 0 — freed on the last disconnect of that actor's
    /// shell session across all of its document sockets in `space`.
    fn release_color(&self, space: &str, actor: &str) {
        let Some(mut colors) = self.session_colors.get_mut(space) else { return };
        let drop_lease = match colors.by_actor.get_mut(actor) {
            Some(lease) => {
                lease.refs = lease.refs.saturating_sub(1);
                lease.refs == 0
            }
            None => false,
        };
        if drop_lease {
            colors.by_actor.remove(actor);
        }
    }

    /// @emoji 🗂️ Get-or-create: a document is lazily minted in `db`'s catalog on its first Hello,
    /// tolerating the race of two sessions doing so concurrently (the loser's `AlreadyExists`
    /// resolves to the same live handle the winner just registered).
    fn ensure_document(&self, id: &ProtocolArtifactId) -> Result<db::ArtifactHandle, db::DbError> {
        match self.db.document(id) {
            Ok(handle) => Ok(handle),
            Err(db::DbError::NotFound(_)) => match self.db.create_document(db::ArtifactSpec::new(id.clone())) {
                Ok(handle) => Ok(handle),
                Err(db::DbError::AlreadyExists(_)) => self.db.document(id),
                Err(other) => Err(other),
            },
            Err(other) => Err(other),
        }
    }
}

fn bearer(headers: &HeaderMap) -> Option<String> {
    headers.get(axum::http::header::AUTHORIZATION).and_then(|value| value.to_str().ok()).and_then(|value| value.strip_prefix("Bearer ")).map(|value| value.to_string())
}

fn db_error_status(error: &db::DbError) -> StatusCode {
    match error {
        db::DbError::NotFound(_) => StatusCode::NOT_FOUND,
        db::DbError::AlreadyExists(_) | db::DbError::Conflict(_) => StatusCode::CONFLICT,
        db::DbError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        db::DbError::InvalidArgument(_) | db::DbError::LimitExceeded(_) => StatusCode::BAD_REQUEST,
        db::DbError::Unavailable(_) | db::DbError::Timeout(_) => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// @emoji #⃣ Decodes a 64-hex-char blob URL path segment into a `db::ContentHash` — the inverse of
/// `ContentHash`'s `Display` (see `pack_core::ContentHash`), never trusted as-is (a malformed path
/// is `BAD_REQUEST`, not a panic).
fn parse_content_hash(hex: &str) -> Option<db::ContentHash> {
    if hex.len() != 64 {
        return None;
    }
    let mut bytes = [0u8; 32];
    let raw = hex.as_bytes();
    for (index, slot) in bytes.iter_mut().enumerate() {
        let byte_str = std::str::from_utf8(&raw[index * 2..index * 2 + 2]).ok()?;
        *slot = u8::from_str_radix(byte_str, 16).ok()?;
    }
    Some(db::ContentHash(bytes))
}
//#endregion 🔖️State

//#region 🔖️Auth
/// @emoji 🔎️ What a bearer token resolved to: an authenticated space member, an anonymous
/// share-token viewer, an anonymous spectator admitted only because the space is `visibility ==
/// "public"`, or nothing.
enum AuthOutcome {
    Session { user_id: String, role: SpaceRole },
    ShareToken,
    /// @emoji 👁️ Public-visibility fallback — an implicit anonymous `SpaceRole::Spectator`, never
    /// persisted as a membership row. Granted only when no session/share-token access resolved AND
    /// the space's own `visibility` is `"public"` (design ruling: "an implicit anonymous spectator
    /// role granted to unauthenticated requests" — deliberately a HANDLER-level fallback, not a
    /// policy-engine concept, since `db_security`'s `RoleBasedPolicy` stays purely mechanical).
    Public,
    Denied,
}

/// @emoji 🔐️ Tries the bearer as an `AuthSessionRecord` (session id -> user -> space role) first;
/// falls back to the existing anonymous share-token scheme when session resolution fails; and
/// finally falls back to `AuthOutcome::Public` when the space itself is `visibility == "public"`.
/// Tokenless documents in a non-public space stay open (dev default) until any share token is
/// issued for them, same as before this wave.
async fn resolve_auth(state: &HubState, space_id: &str, document_id: &str, token: Option<&str>) -> AuthOutcome {
    if let Some(session_id) = token {
        if let Ok(Some(session)) = state.directory.get_auth_session(session_id).await {
            if session.expires_at > now_ms() {
                if let Ok(Some(role)) = state.directory.get_role(space_id, &session.user_id).await {
                    return AuthOutcome::Session { user_id: session.user_id, role };
                }
            }
        }
    }
    if let Ok(true) = state.directory.authorized_by_token(document_id, token).await {
        return AuthOutcome::ShareToken;
    }
    match state.directory.get_space(space_id).await {
        Ok(Some(space)) if space.visibility == "public" => AuthOutcome::Public,
        _ => AuthOutcome::Denied,
    }
}

async fn authorized(state: &HubState, space_id: &str, document_id: &str, token: Option<&str>) -> bool {
    !matches!(resolve_auth(state, space_id, document_id, token).await, AuthOutcome::Denied)
}

/// @emoji 📦️ Space-scoped blobs have no owning document, so this borrows `resolve_auth`'s
/// session -> role branch as-is (space role lookup never touches `document_id`) by passing the
/// blob hash in the document-id slot; the share-token branch then degrades to `Denied` unless a
/// document happens to share the blob's hash as its id, which content hashes never do in
/// practice. A session with any space role is required — a document's share token intentionally
/// does not widen into read access over the whole space's content-addressed blob store.
async fn authorized_for_blob(state: &HubState, space_id: &str, hash: &str, token: Option<&str>) -> bool {
    !matches!(resolve_auth(state, space_id, hash, token).await, AuthOutcome::Denied)
}
//#endregion 🔖️Auth

//#region 🔖️AdminAuth
/// @emoji 🛡️ Contract §C2: bearer `OS_HUB_ADMIN_TOKEN` when the hub was started with one configured;
/// otherwise (dev default) a loopback peer address is implicitly admin — `main()` logs this fallback
/// loudly once at startup so it is never a silent surprise. Shared by every `/admin/api/*` route and
/// `create_share` alike (this supersedes `create_share`'s previous "no token configured ⇒ 403"
/// behaviour, which predates the loopback-admin dev default).
fn is_admin(state: &HubState, headers: &HeaderMap, peer: Option<SocketAddr>) -> bool {
    match state.admin_token.as_deref() {
        Some(expected) => bearer(headers).as_deref() == Some(expected),
        None => peer.is_some_and(|addr| addr.ip().is_loopback()),
    }
}
//#endregion 🔖️AdminAuth

//#region 🔖️Rest
#[derive(Serialize)]
struct DocumentStatusResponse {
    document_id: String,
    head_seq: u64,
    commit_seq: u64,
    epoch: u64,
}

#[derive(Serialize)]
struct ShareResponse {
    token: String,
}

#[derive(Deserialize)]
struct CreateAuthSessionRequest {
    email: String,
}

#[derive(Serialize)]
struct CreateAuthSessionResponse {
    token: String,
    user_id: String,
}

#[derive(Serialize)]
struct BlobRecord {
    hash: String,
    media_type: String,
    size: i64,
}

/// @emoji 🧭️ A document's current frontier — the REST surface's only document-shaped route now
/// that whole-envelope JSON snapshot/operation-log routes are gone (superseded by the WS wire-v2
/// protocol; see this module's doc). Lazily mints the document in `db`'s catalog on first access,
/// same as the WS handshake does.
async fn get_document_status(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<DocumentStatusResponse>, StatusCode> {
    if !authorized(&state, &space_id, &document_id, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let handle = state.ensure_document(&db_artifact_id(&space_id, &document_id)).map_err(|e| db_error_status(&e))?;
    let frontier = handle.frontier().map_err(|e| db_error_status(&e))?;
    Ok(Json(DocumentStatusResponse { document_id, head_seq: frontier.head_seq, commit_seq: frontier.commit_seq, epoch: frontier.epoch }))
}

async fn create_share(Path((_space_id, document_id)): Path<(String, String)>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<ShareResponse>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let token = state.directory.create_share_token(&document_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ShareResponse { token }))
}

/// @emoji 🧪️ Dev-mode session mint: trades a bare email for a bearer session token, upserting the
/// user if it doesn't exist yet. No password/SSO check — real SSO/OAuth is explicitly future scope;
/// this exists only so `AuthSessionRecord`-backed routes have a caller until that lands.
async fn create_auth_session(State(state): State<HubState>, Json(body): Json<CreateAuthSessionRequest>) -> Result<Json<CreateAuthSessionResponse>, StatusCode> {
    let user = match state.directory.get_user_by_email(&body.email).await {
        Ok(Some(user)) => user,
        Ok(None) => state.directory.create_user(&body.email, &body.email, None, None, None).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let session = state.directory.create_auth_session(&user.id, 60 * 60 * 24 * 30, None).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(CreateAuthSessionResponse { token: session.id, user_id: user.id }))
}

//#region Blobs
async fn put_blob(Path((space_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Result<Json<BlobRecord>, StatusCode> {
    if !authorized_for_blob(&state, &space_id, &hash, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let media_type = headers.get(axum::http::header::CONTENT_TYPE).and_then(|value| value.to_str().ok()).unwrap_or("application/octet-stream").to_string();
    let computed = state.db.storage().payload().put(&body).await.map_err(|e| db_error_status(&e))?;
    let computed_hex = computed.to_string();
    // The path hash is client-supplied (content-addressed URL); a mismatch against the
    // storage-computed hash means the client sent the wrong bytes for that address — a bad
    // request, distinct from a document CAS conflict.
    if computed_hex != hash {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Json(BlobRecord { hash: computed_hex, media_type, size: body.len() as i64 }))
}

async fn get_blob(Path((space_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<impl IntoResponse, StatusCode> {
    if !authorized_for_blob(&state, &space_id, &hash, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let content_hash = parse_content_hash(&hash).ok_or(StatusCode::BAD_REQUEST)?;
    match state.db.storage().payload().get(&content_hash).await {
        Ok(bytes) => Ok(([(axum::http::header::CONTENT_TYPE, "application/octet-stream")], bytes)),
        Err(db::DbError::NotFound(_)) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn head_blob(Path((space_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> StatusCode {
    if !authorized_for_blob(&state, &space_id, &hash, bearer(&headers).as_deref()).await {
        return StatusCode::UNAUTHORIZED;
    }
    let Some(content_hash) = parse_content_hash(&hash) else { return StatusCode::BAD_REQUEST };
    match state.db.storage().payload().contains(&content_hash).await {
        Ok(true) => StatusCode::OK,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
//#endregion Blobs
//#endregion 🔖️Rest

//#region 🔖️WebSocket
/// @emoji 🎯️ Contract §C0: presence scope is `(space_id, document_id, surface)`, `surface` travels
/// out of band as `?surface=` on the document WS URL — missing ⇒ `""` (a document with no surface
/// concept still gets one scope, just the empty-string one).
#[derive(Deserialize)]
struct DocumentWsQuery {
    #[serde(default)]
    surface: Option<String>,
}

async fn document_ws(ws: WebSocketUpgrade, Path((space_id, document_id)): Path<(String, String)>, axum::extract::Query(query): axum::extract::Query<DocumentWsQuery>, State(state): State<HubState>) -> impl IntoResponse {
    let surface = query.surface.unwrap_or_default();
    ws.on_upgrade(move |socket| handle_ws(socket, space_id, document_id, surface, state))
}

fn encode(frame: &ServerFrame) -> Message {
    Message::Binary(encode_server_frame(frame, Lane::Command).into())
}

fn error_frame(code: &str, message: impl Into<String>) -> Message {
    encode(&ServerFrame::Error { code: code.to_string(), message: message.into() })
}

/// @emoji 🧭️ Best-effort `RuntimeFrontierSummary` for an `Ack` when the triggering `submit` itself
/// failed — re-reads the document's current (unaffected) frontier so the client still learns
/// "where the server actually is", falling back to an all-zero genesis summary only if even that
/// read fails (a document wedged badly enough that this happens has bigger problems than one Ack).
fn best_effort_frontier(handle: &db::ArtifactHandle) -> RuntimeFrontierSummary {
    match handle.frontier() {
        Ok(frontier) => engine_frontier_to_wire(&frontier, String::new()),
        Err(_) => RuntimeFrontierSummary { document_id: handle.document_id().clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0u8; 32] },
    }
}

fn engine_frontier_to_wire(frontier: &db::db_engine::Frontier, head_edit_id: String) -> RuntimeFrontierSummary {
    RuntimeFrontierSummary { document_id: frontier.document.clone(), head_edit_ordinal: frontier.head_seq, head_edit_id, last_commit_seq: frontier.commit_seq, chain_hash: frontier.chain_hash }
}

/// @emoji ⚖️ `OS_HUB_MERGE_POLICY=laissez-faire|normal|vigilant` (default `normal`) — read once at
/// startup into `HubState.merge_policy` (see its own doc). An unrecognized value is a non-fatal
/// misconfiguration (logged, falls back to the default) rather than refusing to boot, matching this
/// crate's generally-forgiving stance on env parsing elsewhere in `main`.
fn merge_policy_from_env() -> protocol::MergePolicy {
    match std::env::var("OS_HUB_MERGE_POLICY").ok().as_deref() {
        None => protocol::MergePolicy::default(),
        Some("laissez-faire") => protocol::MergePolicy::LaissezFaire,
        Some("normal") => protocol::MergePolicy::Normal,
        Some("vigilant") => protocol::MergePolicy::Vigilant,
        Some(other) => {
            tracing::warn!("unknown OS_HUB_MERGE_POLICY '{other}' (expected laissez-faire|normal|vigilant), defaulting to normal");
            protocol::MergePolicy::default()
        }
    }
}

/// @emoji 🧾️ `ApplyOutcome::Rejected.messages`'s wire payload: a packed `Vec<protocol::
/// MutationMessage>` blob (contract §C9/§C8). `📡️wire`'s own doc names `pack::encode_record_body`
/// as the intended codec, but that needs a `RecordSpec` no `MutationMessage`-adjacent type in this
/// tree defines yet (verified: none of `📡️spr/🎮️command`/`📡️spr/⚔️conflict` register one) — adding
/// one is a `📡️spr` schema change outside this lease. `MutationMessage` already derives `serde::
/// Serialize`/`Deserialize` (used verbatim for every other opaque wire/WAL payload in this same
/// crate, e.g. `sample_envelope`'s diff/inverse bytes below), so this crate uses that instead: a
/// real, round-trippable encoding of the actual messages, not a placeholder. Flagged in this
/// ticket's report for the coordinator/lane 1-C to reconcile against `pack::encode_record_body` if
/// a `MutationMessage` `RecordSpec` lands later.
fn encode_messages(messages: &[protocol::MutationMessage]) -> Vec<u8> {
    serde_json::to_vec(messages).unwrap_or_default()
}

/// @emoji 🧾️ Every `protocol::MutationMessage` `error` carries, if any — non-empty only for
/// `db::DbError::Rejected` (the outcome-step gate `db_artifact::ArtifactEngine::submit` returns per
/// contract §C9); every other `DbError` variant has nothing to add here.
fn messages_for_error(error: &db::DbError) -> Vec<u8> {
    match error {
        db::DbError::Rejected { messages, .. } => encode_messages(messages),
        _ => Vec::new(),
    }
}

/// @emoji ✍️ Submits `envelopes` as one `db_artifact::CommandBatch` through `handle`, returning the
/// `Ack` to send the submitter plus (on acceptance) the `Commands` frame to fan out to every other
/// session on the same document. `Fsync` durability: a hub session's `submit` genuinely committing
/// is the promise `AckStage::Persisted` makes to the client. `policy` is `HubState.merge_policy`
/// (contract §C9) — the outcome-step gate `handle.submit` runs before any WAL append.
///
/// 🎯️ Design choice (accepted-but-degraded messages have no relay carrier yet): when `policy`
/// admits a batch whose worst graded level is still `Warning`-or-above (a "degraded merge", contract
/// §C5), `receipt.messages` is non-empty but neither `ApplyOutcome::Accepted` nor
/// `ServerFrame::Commands` (both fieldless/message-less in the CURRENTLY LANDED `📡️wire` shape —
/// verified against `📡️spr/📡️wire/🦀️component.rs`) has anywhere to carry them to the submitter's
/// peers. `📡️wire` is lane 1-C's lease, already landed `ApplyOutcome::Rejected{reason, messages}`
/// for this exact contract clause's rejected half; widening `Accepted`/`Commands` further is a wire
/// change this lane is not authorized to make unilaterally (per the worker brief's "if you must
/// touch a file outside your lease, STOP and report instead"), so `receipt.messages` is deliberately
/// dropped here rather than silently faked onto a field that doesn't exist — see this ticket's
/// report for the gap.
async fn submit_commands(handle: &db::ArtifactHandle, actor: &ActorId, batch_id: u64, envelopes: Vec<MutationEnvelope>, policy: protocol::MergePolicy) -> (ServerFrame, Option<ServerFrame>) {
    let batch = match db::document::CommandBatch::new(envelopes.clone()) {
        Ok(batch) => batch,
        Err(error) => {
            let frontier = best_effort_frontier(handle);
            return (ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: error.to_string(), messages: Vec::new() }) }], frontier }, None);
        }
    };
    match handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync, policy }).await {
        Ok(Ok(receipt)) => {
            let frontier = engine_frontier_to_wire(&receipt.frontier, receipt.command_id.0.clone());
            let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: frontier.clone() };
            let commands = ServerFrame::Commands { envelopes, origin: actor.clone(), frontier };
            (ack, Some(commands))
        }
        Ok(Err(error)) | Err(error) => {
            let frontier = best_effort_frontier(handle);
            let messages = messages_for_error(&error);
            (ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: error.to_string(), messages }) }], frontier }, None)
        }
    }
}

/// @emoji 🚪️ Runs `envelopes` through `gate.admit_command` one at a time (tenant isolation, then
/// `Action::Write` authz on `AuthzScope::CommandKind`, DoS budget, replay dedupe — see
/// `SecurityGate::admit_command`'s own doc) before any of them reach `db::ArtifactHandle::submit`.
/// Returns the first rejection reason, or `None` once every envelope is admitted. `kind` is a
/// constant ("write") rather than a per-envelope command-kind string: this crate sits above
/// `db_artifact`'s pipeline and never interprets an operation's schema/diff semantics (matches
/// `db_security`'s own module doc — payload interpretation stays out of this layer), so command-kind
/// granularity inside one document is not this wave's concern.
fn admit_writes(gate: &db::security::SecurityGate, principal: &db::security::Principal, tenant: &db::security::TenantId, document: &ProtocolArtifactId, envelopes: &[MutationEnvelope], physical_ms: u64) -> Option<String> {
    envelopes.iter().find_map(|envelope| gate.admit_command(principal, tenant, document, "write", &envelope.actor, &envelope.mutation_id, physical_ms).err().map(|error| error.to_string()))
}

/// @emoji 📨️ Handles one decoded `ClientFrame` for an already-authenticated, already-`Hello`'d
/// session. Returns `false` when the session should close (`Bye`, or a send failure).
#[allow(clippy::too_many_arguments)]
async fn handle_client_frame(
    state: &HubState,
    handle: &db::ArtifactHandle,
    db_id: &ProtocolArtifactId,
    key: &str,
    space_id: &str,
    document_id: &str,
    fanout: &broadcast::Sender<ServerFrame>,
    actor: &ActorId,
    gate: &db::security::SecurityGate,
    principal: &db::security::Principal,
    tenant: &db::security::TenantId,
    frame: ClientFrame,
    sender: &mut SplitSink<WebSocket, Message>,
) -> bool {
    match frame {
        ClientFrame::Commands { batch_id, envelopes } => {
            if let Some(reason) = admit_writes(gate, principal, tenant, db_id, &envelopes, now_ms().max(0) as u64) {
                let frontier = best_effort_frontier(handle);
                let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason, messages: Vec::new() }) }], frontier };
                return sender.send(encode(&ack)).await.is_ok();
            }
            let (ack, relay) = submit_commands(handle, actor, batch_id, envelopes, state.merge_policy).await;
            if let Some(commands_frame) = relay {
                let _ = fanout.send(commands_frame);
            }
            sender.send(encode(&ack)).await.is_ok()
        }
        ClientFrame::FrontierAdvertise { frontier } => {
            let core_document = db_core_document_id(db_id);
            match db::sync::handle_frontier_advertise(state.db.storage().wal(), core_document, &frontier, actor.clone()).await {
                Ok(Some(catch_up)) => sender.send(encode(&catch_up)).await.is_ok(),
                Ok(None) => true,
                Err(_) => true,
            }
        }
        ClientFrame::PreviewPublish { key: preview_key, seq, payload } => {
            let _ = fanout.send(ServerFrame::Preview { actor: actor.clone(), key: preview_key, seq, payload });
            true
        }
        ClientFrame::Presence { peer } => {
            if let Some(mut entry) = state.presence.get_mut(&(key.to_string(), actor.0.clone())) {
                entry.peer = Some(peer);
            }
            let _ = fanout.send(ServerFrame::Presence { peers: state.presence_peers(key) });
            state.directory_service.publish(DirectoryStreamMessage::Presence { space_id: space_id.to_string(), document_id: document_id.to_string(), actors: state.directory_presence_actors(key) });
            true
        }
        // 🪙️ Command-lane credit-based flow control: no server-side congestion control implemented
        // this wave (matches `framework/sync`'s client, which also accepts and ignores this frame).
        ClientFrame::CreditGrant { .. } => true,
        ClientFrame::Bye => false,
        // A second `Hello` mid-session has nothing to negotiate beyond the first — ignored rather
        // than torn down, matching this crate's generally forgiving-of-redundant-frames stance.
        ClientFrame::Hello { .. } => true,
    }
}

async fn handle_ws(socket: WebSocket, space_id: String, document_id: String, surface: String, state: HubState) {
    let (mut sender, mut receiver) = socket.split();

    let hello = match receiver.next().await {
        Some(Ok(Message::Binary(bytes))) => decode_client_frame(&bytes).ok().map(|(_lane, frame)| frame),
        _ => None,
    };
    let Some(ClientFrame::Hello { pack_schema_hash, actor, token, frontier, .. }) = hello else {
        let _ = sender.send(error_frame("protocol", "expected hello frame")).await;
        return;
    };

    let key = scope_key(&space_id, &document_id);
    if pack_schema_hash != [0u8; 32] {
        let pinned = *state.schema_hashes.entry(key.clone()).or_insert(pack_schema_hash);
        if pinned != pack_schema_hash {
            let _ = sender.send(error_frame("schema-hash-mismatch", "pack schema hash does not match the hash already pinned for this document")).await;
            return;
        }
    }

    let auth = resolve_auth(&state, &space_id, &document_id, token.as_deref()).await;
    let (user_id, role) = match &auth {
        AuthOutcome::Session { user_id, role } => (Some(user_id.clone()), Some(*role)),
        AuthOutcome::ShareToken => (None, None),
        // 👁️ Public-visibility fallback: an implicit anonymous spectator, never a persisted
        // membership row (see `AuthOutcome::Public`'s doc).
        AuthOutcome::Public => (None, Some(SpaceRole::Spectator)),
        AuthOutcome::Denied => {
            let _ = sender.send(error_frame("unauthorized", "unauthorized")).await;
            return;
        }
    };
    // 🎨️ Contract §C7.3: acquired after successful Hello/auth and before `Welcome`, released at
    // handler exit (every early-return path below releases it explicitly; the loop-exit cleanup
    // releases it on a clean disconnect).
    let color = state.acquire_color(&space_id, &actor.0);

    // 🔒️ Per-connection `SecurityGate`: `space_grants` compiles this space's `kind` into
    // author=rw/spectator=ro grants (archive additionally deny-overrides author writes), a fresh
    // `RoleBasedPolicy` from them, and a `Principal` carrying the caller's resolved role. A share-
    // token/public-visibility caller (no session role) is admitted as `"spectator"` — read-only,
    // the least-privilege default for a connection this crate cannot attribute to a real member.
    // `TenantId` reuses the space id: this crate has no separate tenant concept yet, and every
    // scope this gate ever evaluates already belongs to exactly this one space/document connection.
    let space_kind = state.directory.get_space(&space_id).await.ok().flatten().map_or_else(|| "studio".to_string(), |space| space.kind);
    let policy = db::security::space_grants(&space_id, &space_kind).into_iter().fold(db::security::RoleBasedPolicy::new(), db::security::RoleBasedPolicy::with_grant);
    let gate = db::security::SecurityGate::new(policy, db::security::ReplayGuard::new(60_000, 256), db::security::BudgetRegistry::new(240, 60), Arc::new(db::NullEmit));
    let tenant = db::security::TenantId::from(space_id.clone());
    // 🎯️ Role mapping: a resolved membership uses its own `SpaceRole`; `AuthOutcome::Public` (the
    // NEW implicit-anonymous-spectator fallback for `visibility == "public"`, per the design
    // ruling) is deliberately read-only. `AuthOutcome::ShareToken` predates the role system
    // entirely — it already granted unconditional (read+write) per-document access before this
    // wave — so it maps to `"author"` here to preserve that existing contract rather than silently
    // regressing every share-token holder to read-only.
    let role_str = match &auth {
        AuthOutcome::Session { role, .. } => role.as_str().to_string(),
        AuthOutcome::ShareToken => "author".to_string(),
        AuthOutcome::Public => "spectator".to_string(),
        AuthOutcome::Denied => unreachable!("Denied already returned above"),
    };
    let principal = db::security::Principal::new(actor.clone(), tenant.clone(), vec![role_str]);

    let db_id = db_artifact_id(&space_id, &document_id);
    let handle = match state.ensure_document(&db_id) {
        Ok(handle) => handle,
        Err(error) => {
            let _ = sender.send(error_frame("storage", error.to_string())).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };

    let session_id = uuid::Uuid::now_v7().to_string();
    // 🔖️ 64KiB inline-snapshot threshold: this crate's own choice (`db_sync::build_welcome`'s
    // `snapshot_chunk_bytes` fixes the threshold, not a value) — generous enough that a fresh
    // replica's typical backlog never needs a follow-up `SnapshotChunk` round trip, small enough
    // to never balloon a single WS frame unreasonably.
    let welcome_response = match state.db.hello(&db_id, frontier.as_ref(), session_id, &actor, 64 * 1024) {
        Ok(response) => response,
        Err(error) => {
            let _ = sender.send(error_frame("storage", error.to_string())).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };
    if sender.send(encode(&welcome_response.welcome)).await.is_err() {
        state.release_color(&space_id, &actor.0);
        return;
    }
    for frame in &welcome_response.follow_up {
        if sender.send(encode(frame)).await.is_err() {
            state.release_color(&space_id, &actor.0);
            return;
        }
    }
    // 🎨️ Contract §C7.3: sent exactly once per connection, after `Welcome` (and its follow-up
    // bootstrap frames) and before any `Presence` frame.
    if sender.send(encode(&ServerFrame::Session { actor: actor.0.clone(), color })).await.is_err() {
        state.release_color(&space_id, &actor.0);
        return;
    }
    state.presence.insert((key.clone(), actor.0.clone()), PresenceSession { surface: surface.clone(), user_id: user_id.clone(), color, peer: None });

    let fanout = state.fanout_for(&key);
    let mut broadcast_rx = fanout.subscribe();

    let sync_session = state.directory.record_sync_session_open(&space_id, &document_id, &surface, user_id.as_deref(), role, &actor.0).await.ok();
    if let Some(session) = &sync_session {
        let view = connection_view(&state, session).await;
        state.directory_service.publish(DirectoryStreamMessage::Connection { phase: DirectoryConnectionPhase::Opened, connection: view });
    }
    // 🦵️ Admin kick: only a session the directory actually recorded gets a live `Notify` registered
    // under its `syncSessionId` (see `session_kicks`' own doc) — a session that failed to record
    // (e.g. directory hiccup) falls back to a `Notify` nobody can ever reach, i.e. un-kickable, which
    // matches this crate's generally forgiving stance on directory-write failures elsewhere in this
    // handler.
    let kick = match &sync_session {
        Some(session) => {
            let notify = Arc::new(tokio::sync::Notify::new());
            state.session_kicks.insert(session.id.clone(), notify.clone());
            notify
        }
        None => Arc::new(tokio::sync::Notify::new()),
    };

    loop {
        tokio::select! {
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Binary(bytes))) => {
                        if let Ok((_lane, frame)) = decode_client_frame(&bytes) {
                            if !handle_client_frame(&state, &handle, &db_id, &key, &space_id, &document_id, &fanout, &actor, &gate, &principal, &tenant, frame, &mut sender).await {
                                break;
                            }
                        }
                    }
                    Some(Ok(Message::Ping(payload))) => {
                        if sender.send(Message::Pong(payload)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
            event = broadcast_rx.recv() => {
                match event {
                    Ok(frame) => {
                        if sender.send(encode(&frame)).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {}
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            _ = kick.notified() => break,
        }
    }

    if let Some(session) = sync_session {
        let view = connection_view(&state, &session).await;
        let _ = state.directory.record_sync_session_close(&session.id).await;
        state.session_kicks.remove(&session.id);
        state.directory_service.publish(DirectoryStreamMessage::Connection { phase: DirectoryConnectionPhase::Closed, connection: view });
    }
    state.presence.remove(&(key.clone(), actor.0.clone()));
    let _ = fanout.send(ServerFrame::Presence { peers: state.presence_peers(&key) });
    state.release_color(&space_id, &actor.0);
    state.directory_service.publish(DirectoryStreamMessage::Presence { space_id: space_id.clone(), document_id: document_id.clone(), actors: state.directory_presence_actors(&key) });
}
//#endregion 🔖️WebSocket

//#region 🔖️Directory
/// @emoji 🙋️ A bearer token resolved to a live, unexpired `AuthSessionRecord`'s user — every
/// `/directory/*`/`/auth/sessions/me` route that needs a caller identity resolves through this
/// (distinct from `AuthOutcome`, which is the document-WS auth-lite scheme with its share-token/
/// public-visibility fallbacks; the directory control plane has no such fallbacks — a command with
/// no valid session is simply unauthenticated).
struct AuthedUser {
    user_id: String,
}

async fn resolve_bearer_user(state: &HubState, token: Option<&str>) -> Option<AuthedUser> {
    let session = state.directory.get_auth_session(token?).await.ok().flatten()?;
    if session.expires_at <= now_ms() {
        return None;
    }
    Some(AuthedUser { user_id: session.user_id })
}

fn directory_error_status(error: DirectoryError) -> StatusCode {
    match error {
        DirectoryError::NotFound(_) => StatusCode::NOT_FOUND,
        DirectoryError::Conflict(_) => StatusCode::CONFLICT,
        DirectoryError::Unauthorized => StatusCode::UNAUTHORIZED,
        DirectoryError::Backend(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// @emoji 📇️ Rebuilds `DirectoryReadModel` by folding the WHOLE event log on every call — simple
/// and always-correct (a maintained/cached projection updated incrementally off `DirectoryService::
/// subscribe` would be the natural follow-up optimization once a real log grows large enough to
/// matter; this wave's event logs are dev/test-scale, so correctness-first wins over that added
/// moving part).
async fn load_read_model(state: &HubState) -> Result<DirectoryReadModel, StatusCode> {
    let events = state.directory.events_since(0, usize::MAX).await.map_err(directory_error_status)?;
    Ok(os_directory::fold_all(DirectoryReadModel::default(), &events))
}

fn role_wire(role: SpaceRole) -> DirectorySpaceRole {
    match role {
        SpaceRole::Author => DirectorySpaceRole::Author,
        SpaceRole::Spectator => DirectorySpaceRole::Spectator,
    }
}

fn invite_view(record: InviteRecord) -> InviteView {
    InviteView { id: record.id, space_id: record.space_id, role: role_wire(record.role), created_at_ms: record.created_at, expires_at_ms: record.expires_at, revoked: record.revoked_at.is_some() }
}

/// @emoji 🔴️ `ConnectionView` for one live `SyncSessionRecord` — `presenceKnown` cross-references
/// `state.presence` (contract: "connections = `list_active_sync_sessions()` joined with the
/// in-memory presence map").
async fn connection_view(state: &HubState, session: &SyncSessionRecord) -> ConnectionView {
    let email = match &session.user_id {
        Some(id) => state.directory.get_user(id).await.ok().flatten().map(|user| user.email),
        None => None,
    };
    let scope = scope_key(&session.space_id, &session.document_id);
    let presence_known = state.presence.get(&(scope, session.client_label.clone())).is_some_and(|entry| entry.peer.is_some());
    ConnectionView {
        sync_session_id: session.id.clone(),
        space_id: session.space_id.clone(),
        document_id: session.document_id.clone(),
        surface: session.surface.clone(),
        actor: session.client_label.clone(),
        user_id: session.user_id.clone(),
        email,
        role: session.space_role.map(role_wire).unwrap_or(DirectorySpaceRole::Spectator),
        connected_at_ms: session.connected_at,
        presence_known,
    }
}

/// @emoji 📄️ `db.catalog()` filtered by the `{space_id}:` prefix, each surviving handle's own
/// `frontier()` (contract: space detail's `documents` + admin's `documents?space=`).
async fn documents_for_space(state: &HubState, space_id: &str) -> Vec<DocumentView> {
    let prefix = format!("{space_id}:");
    let mut views = Vec::new();
    for entry in state.db.catalog().artifacts {
        let Some(document_id) = entry.document.0.strip_prefix(prefix.as_str()) else { continue };
        let Ok(handle) = state.db.document(&entry.document) else { continue };
        let Ok(frontier) = handle.frontier() else { continue };
        views.push(DocumentView { id: document_id.to_string(), head_seq: frontier.head_seq, commit_seq: frontier.commit_seq, epoch: frontier.epoch });
    }
    views
}

/// @emoji 🏠️ Fills a folded `DirectorySpace`'s `SpaceView` with the two fields the pure fold cannot
/// know: the CALLING user's own `role` (server-filled per request, never derived by `fold`) and the
/// live `document_count`/`active_connections` (owned by `db`'s catalog and the directory's sync
/// sessions respectively, neither of which the directory event log itself tracks).
async fn space_view(state: &HubState, space: &os_directory::DirectorySpace, caller: Option<&AuthedUser>) -> SpaceView {
    let mut view = space.view.clone();
    view.role = caller.and_then(|user| space.members.iter().find(|member| member.user_id == user.user_id).map(|member| member.role));
    view.document_count = documents_for_space(state, &view.id).await.len() as u32;
    view.active_connections = state.directory.list_active_sync_sessions(Some(&view.id)).await.map(|sessions| sessions.len() as u32).unwrap_or(0);
    view
}

/// @emoji ⚖️ Contract §C2's command authorization matrix: `create-space` any session; `delete-space`/
/// `archive-space` owner or admin; everything else any AUTHOR of the named space or admin. `decide`
/// itself performs zero authorization (its own doc) — this is that check, run before `execute`.
async fn authorize_directory_command(state: &HubState, actor_user_id: &str, admin: bool, command: &DirectoryCommand) -> Result<(), StatusCode> {
    if admin {
        return Ok(());
    }
    match command {
        DirectoryCommand::CreateSpace { .. } => Ok(()),
        DirectoryCommand::DeleteSpace { space_id } | DirectoryCommand::ArchiveSpace { space_id } => {
            let space = state.directory.get_space(space_id).await.map_err(directory_error_status)?.ok_or(StatusCode::NOT_FOUND)?;
            if space.owner_user_id == actor_user_id {
                Ok(())
            } else {
                Err(StatusCode::FORBIDDEN)
            }
        }
        DirectoryCommand::RenameSpace { space_id, .. }
        | DirectoryCommand::SetVisibility { space_id, .. }
        | DirectoryCommand::UpsertMember { space_id, .. }
        | DirectoryCommand::RemoveMember { space_id, .. }
        | DirectoryCommand::CreateInvite { space_id, .. }
        | DirectoryCommand::RevokeInvite { space_id, .. } => match state.directory.get_role(space_id, actor_user_id).await {
            Ok(Some(SpaceRole::Author)) => Ok(()),
            Ok(_) => Err(StatusCode::FORBIDDEN),
            Err(error) => Err(directory_error_status(error)),
        },
    }
}

#[derive(Serialize)]
struct DirectoryCommandResponse {
    events: Vec<DirectoryEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
}

fn command_result_json(result: Option<CommandResult>) -> Option<serde_json::Value> {
    result.and_then(|value| value.invite_token).map(|token| serde_json::json!({ "inviteToken": token }))
}

async fn post_directory_commands(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>, Json(command): Json<DirectoryCommand>) -> Result<(StatusCode, Json<DirectoryCommandResponse>), StatusCode> {
    let user = resolve_bearer_user(&state, bearer(&headers).as_deref()).await.ok_or(StatusCode::UNAUTHORIZED)?;
    let admin = is_admin(&state, &headers, Some(peer));
    authorize_directory_command(&state, &user.user_id, admin, &command).await?;
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hub-rest", user.user_id) };
    let (events, result) = state.directory_service.execute(actor, command).await.map_err(directory_error_status)?;
    Ok((StatusCode::ACCEPTED, Json(DirectoryCommandResponse { events, result: command_result_json(result) })))
}

async fn get_directory_spaces(headers: HeaderMap, State(state): State<HubState>) -> Result<Json<Vec<SpaceView>>, StatusCode> {
    let caller = resolve_bearer_user(&state, bearer(&headers).as_deref()).await;
    let model = load_read_model(&state).await?;
    let mut views = Vec::new();
    for space in model.spaces.values() {
        let visible = space.view.visibility == DirectorySpaceVisibility::Public || caller.as_ref().is_some_and(|user| space.members.iter().any(|member| member.user_id == user.user_id));
        if visible {
            views.push(space_view(&state, space, caller.as_ref()).await);
        }
    }
    views.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(Json(views))
}

#[derive(Serialize)]
struct SpaceDetailResponse {
    #[serde(flatten)]
    view: SpaceView,
    members: Vec<MemberView>,
    documents: Vec<DocumentView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invites: Option<Vec<InviteView>>,
}

async fn get_directory_space(Path(space_id): Path<String>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<SpaceDetailResponse>, StatusCode> {
    let caller = resolve_bearer_user(&state, bearer(&headers).as_deref()).await;
    let model = load_read_model(&state).await?;
    let space = model.spaces.get(&space_id).ok_or(StatusCode::NOT_FOUND)?;
    let membership = caller.as_ref().and_then(|user| space.members.iter().find(|member| member.user_id == user.user_id));
    if space.view.visibility != DirectorySpaceVisibility::Public && membership.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    let is_author = matches!(membership, Some(member) if member.role == DirectorySpaceRole::Author);
    let documents = documents_for_space(&state, &space_id).await;
    let invites = if is_author { state.directory.list_invites(&space_id).await.ok().map(|records| records.into_iter().map(invite_view).collect()) } else { None };
    let view = space_view(&state, space, caller.as_ref()).await;
    Ok(Json(SpaceDetailResponse { view, members: space.members.clone(), documents, invites }))
}

async fn post_redeem_invite(Path(token): Path<String>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<Vec<DirectoryEvent>>, StatusCode> {
    let user = resolve_bearer_user(&state, bearer(&headers).as_deref()).await.ok_or(StatusCode::UNAUTHORIZED)?;
    let record = state.directory.get_user(&user.user_id).await.map_err(directory_error_status)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hub-rest", user.user_id) };
    let events = state.directory_service.redeem_invite(actor, &token, &record.email, &record.display_name).await.map_err(directory_error_status)?;
    Ok(Json(events))
}

#[derive(Deserialize)]
struct EventsQuery {
    since: Option<u64>,
    limit: Option<usize>,
}

/// @emoji 👁️ One event's visibility for `caller`: events with no `space_id` (`user.created`) are
/// always visible (member display-name resolution needs them, and a platform user's email/display
/// name carries no more exposure than an existing member listing already does); a space-scoped event
/// is visible iff the space is public or `caller` holds ANY role in it.
async fn event_visible(state: &HubState, event: &DirectoryEvent, caller: Option<&AuthedUser>) -> bool {
    let Some(space_id) = &event.space_id else { return true };
    match state.directory.get_space(space_id).await {
        Ok(Some(space)) if space.visibility == "public" => true,
        Ok(Some(_)) => match caller {
            Some(user) => matches!(state.directory.get_role(space_id, &user.user_id).await, Ok(Some(_))),
            None => false,
        },
        _ => false,
    }
}

async fn visibility_filter_events(state: &HubState, events: Vec<DirectoryEvent>, caller: Option<&AuthedUser>) -> Vec<DirectoryEvent> {
    let mut visible = Vec::with_capacity(events.len());
    for event in events {
        if event_visible(state, &event, caller).await {
            visible.push(event);
        }
    }
    visible
}

async fn get_directory_events(axum::extract::Query(query): axum::extract::Query<EventsQuery>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<Vec<DirectoryEvent>>, StatusCode> {
    let caller = resolve_bearer_user(&state, bearer(&headers).as_deref()).await;
    let events = state.directory.events_since(query.since.unwrap_or(0), query.limit.unwrap_or(500)).await.map_err(directory_error_status)?;
    Ok(Json(visibility_filter_events(&state, events, caller.as_ref()).await))
}

#[derive(Deserialize)]
struct DirectoryWsQuery {
    token: Option<String>,
    #[serde(default)]
    since: u64,
}

async fn directory_ws(ws: WebSocketUpgrade, axum::extract::Query(query): axum::extract::Query<DirectoryWsQuery>, State(state): State<HubState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_directory_ws(socket, query.token, query.since, state))
}

async fn send_directory_message(sender: &mut SplitSink<WebSocket, Message>, message: &DirectoryStreamMessage) -> bool {
    let text = serde_json::to_string(message).unwrap_or_default();
    sender.send(Message::Text(text.into())).await.is_ok()
}

/// @emoji 📡️ Contract §C2's "subscribe, then replay, gap-free": subscribes to `DirectoryService`'s
/// live broadcast FIRST (so nothing published between "read events_since" and "start listening" is
/// ever missed), THEN replays `events_since(since)`, THEN forwards live messages — dropping any
/// already-replayed `Event` (`seq <= last_replayed`) so a reconnecting client's stream is both
/// gap-free and duplicate-free.
async fn handle_directory_ws(socket: WebSocket, token: Option<String>, since: u64, state: HubState) {
    let (mut sender, mut receiver) = socket.split();
    let caller = resolve_bearer_user(&state, token.as_deref()).await;
    let mut live = state.directory_service.subscribe();

    let replay = match state.directory.events_since(since, usize::MAX).await {
        Ok(events) => visibility_filter_events(&state, events, caller.as_ref()).await,
        Err(_) => Vec::new(),
    };
    let mut last_replayed = since;
    for event in replay {
        last_replayed = last_replayed.max(event.seq);
        if !send_directory_message(&mut sender, &DirectoryStreamMessage::Event { event }).await {
            return;
        }
    }

    loop {
        tokio::select! {
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(payload))) => {
                        if sender.send(Message::Pong(payload)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
            event = live.recv() => {
                match event {
                    Ok(DirectoryStreamMessage::Event { event }) => {
                        if event.seq <= last_replayed || !event_visible(&state, &event, caller.as_ref()).await {
                            continue;
                        }
                        last_replayed = last_replayed.max(event.seq);
                        if !send_directory_message(&mut sender, &DirectoryStreamMessage::Event { event }).await {
                            break;
                        }
                    }
                    Ok(message) => {
                        if !send_directory_message(&mut sender, &message).await {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {}
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SessionMeResponse {
    user_id: String,
    email: String,
    display_name: String,
    expires_at: i64,
}

async fn get_session_me(headers: HeaderMap, State(state): State<HubState>) -> Result<Json<SessionMeResponse>, StatusCode> {
    let token = bearer(&headers).ok_or(StatusCode::UNAUTHORIZED)?;
    let session = state.directory.get_auth_session(&token).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::UNAUTHORIZED)?;
    if session.expires_at <= now_ms() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let user = state.directory.get_user(&session.user_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(Json(SessionMeResponse { user_id: user.id, email: user.email, display_name: user.display_name, expires_at: session.expires_at }))
}

async fn delete_session_me(headers: HeaderMap, State(state): State<HubState>) -> StatusCode {
    let Some(token) = bearer(&headers) else { return StatusCode::UNAUTHORIZED };
    match state.directory.revoke_auth_session(&token).await {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// @emoji 🌐️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS w4-h — root cause of
/// STEP 1's "space created but never replicated" symptom: contract §C0 puts the hub and every `s`
/// shell on DIFFERENT origins (hub `8787`; shell dev/preview `6072`/`6073`/the collab-e2e `7400-7498`
/// pool), so every fetch the shell issues — starting with `POST /auth/sessions` during the identity
/// bootstrap (§C3), which every later directory command depends on — is cross-origin. With no CORS
/// grant, the browser blocks the preflight before the request ever reaches this router; reproduced
/// live via the collab-e2e harness's new console capture: `Access to fetch at
/// 'http://127.0.0.1:<hub>/auth/sessions' from origin 'http://127.0.0.1:<shell>' has been blocked by
/// CORS policy`, which surfaces downstream as `[os-shell] identity bootstrap: hub unreachable` and
/// then `replayShellCommand: directory command dropped, no signed-in identity` for every
/// `os.directory.*` command — the create-space POST is silently never sent, so the hub-side broadcast
/// this ticket's directory-lane/fold plumbing was built to deliver never had an event to carry. No
/// `tower-http` dependency (outside this lease's `Cargo.toml`): a bare `axum::middleware::from_fn`
/// mirrors the caller's own `Origin` back (never a bare `*`, so this stays compatible with a future
/// cookie/credentialed scheme) and answers every `OPTIONS` preflight with 204 before it reaches route
/// dispatch. Applied to the WHOLE router (`router()`, `🔖️Main`) rather than only `/directory/*` —
/// `/auth/sessions` (`🔖️Rest`) is the very first cross-origin call in the boot sequence and must be
/// reachable before any directory command can even be attempted; a narrower per-route layer would
/// leave the actual observed failure unfixed.
async fn cors_middleware(request: axum::extract::Request, next: axum::middleware::Next) -> axum::response::Response {
    let origin = request.headers().get(axum::http::header::ORIGIN).cloned();
    if request.method() == axum::http::Method::OPTIONS {
        let mut response = axum::response::Response::builder().status(StatusCode::NO_CONTENT).body(axum::body::Body::empty()).unwrap_or_default();
        apply_cors_headers(response.headers_mut(), origin.as_ref());
        return response;
    }
    let mut response = next.run(request).await;
    apply_cors_headers(response.headers_mut(), origin.as_ref());
    response
}

/// @emoji 🌐️ See {@link cors_middleware}. Reflects the request's own `Origin` (never `*`) plus the
/// bearer/JSON headers and verbs this control plane actually uses.
fn apply_cors_headers(headers: &mut HeaderMap, origin: Option<&axum::http::HeaderValue>) {
    if let Some(origin) = origin {
        headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
        headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, axum::http::HeaderValue::from_static("true"));
        headers.insert(axum::http::header::VARY, axum::http::HeaderValue::from_static("origin"));
    }
    headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_METHODS, axum::http::HeaderValue::from_static("GET, POST, PUT, HEAD, DELETE, OPTIONS"));
    headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, axum::http::HeaderValue::from_static("authorization, content-type"));
}
//#endregion 🔖️Directory

//#region 🔖️Admin
/// @emoji 🧮️ Best-effort recursive directory size in bytes — used only for the admin overview's
/// `dataDirBytes`; any unreadable entry is silently skipped rather than failing the whole overview.
fn dir_size(path: &std::path::Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                total += if metadata.is_dir() { dir_size(&entry.path()) } else { metadata.len() };
            }
        }
    }
    total
}

async fn admin_overview(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<serde_json::Value>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let model = load_read_model(&state).await?;
    let head_seq = state.directory.head_seq().await.map_err(directory_error_status)?;
    let connections = state.directory.list_active_sync_sessions(None).await.map_err(directory_error_status)?.len();
    let users = state.directory.list_users(i64::MAX, 0).await.map_err(directory_error_status)?.len();
    // 🌵️ `extensions_root` is `{data_dir}/extension-modules` (see `main`'s own construction) — its
    // parent is `data_dir` itself, the nearest thing `HubState` carries to `OS_HUB_DATA`'s root.
    let data_dir_bytes = state.extensions_root.parent().map(dir_size).unwrap_or(0);
    Ok(Json(serde_json::json!({
        "counts": { "spaces": model.spaces.len(), "users": users, "connections": connections },
        "backends": { "sqlite": cfg!(feature = "sqlite"), "postgres": cfg!(feature = "postgres"), "neo4j": cfg!(feature = "neo4j") },
        "dataDirBytes": data_dir_bytes,
        "headSeq": head_seq,
        "openArtifacts": state.db.catalog().artifacts.len(),
    })))
}

async fn admin_spaces(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<Vec<SpaceView>>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let model = load_read_model(&state).await?;
    let mut views = Vec::new();
    for space in model.spaces.values() {
        views.push(space_view(&state, space, None).await);
    }
    views.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(Json(views))
}

async fn admin_space(Path(space_id): Path<String>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<SpaceDetailResponse>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let model = load_read_model(&state).await?;
    let space = model.spaces.get(&space_id).ok_or(StatusCode::NOT_FOUND)?;
    let documents = documents_for_space(&state, &space_id).await;
    let invites = state.directory.list_invites(&space_id).await.map_err(directory_error_status)?.into_iter().map(invite_view).collect();
    let view = space_view(&state, space, None).await;
    Ok(Json(SpaceDetailResponse { view, members: space.members.clone(), documents, invites: Some(invites) }))
}

async fn admin_users(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<Vec<os_directory::UserView>>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let users = state.directory.list_users(i64::MAX, 0).await.map_err(directory_error_status)?;
    Ok(Json(users.into_iter().map(|user| os_directory::UserView { id: user.id, email: user.email, display_name: user.display_name, created_at_ms: user.created_at }).collect()))
}

async fn admin_connections(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<Vec<ConnectionView>>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let sessions = state.directory.list_active_sync_sessions(None).await.map_err(directory_error_status)?;
    let mut views = Vec::with_capacity(sessions.len());
    for session in &sessions {
        views.push(connection_view(&state, session).await);
    }
    Ok(Json(views))
}

#[derive(Deserialize)]
struct DocumentsQuery {
    space: Option<String>,
}

async fn admin_documents(axum::extract::Query(query): axum::extract::Query<DocumentsQuery>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<Vec<DocumentView>>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    match query.space {
        Some(space_id) => Ok(Json(documents_for_space(&state, &space_id).await)),
        None => {
            let mut views = Vec::new();
            for entry in state.db.catalog().artifacts {
                let Ok(handle) = state.db.document(&entry.document) else { continue };
                let Ok(frontier) = handle.frontier() else { continue };
                views.push(DocumentView { id: entry.document.0.clone(), head_seq: frontier.head_seq, commit_seq: frontier.commit_seq, epoch: frontier.epoch });
            }
            Ok(Json(views))
        }
    }
}

async fn admin_events(axum::extract::Query(query): axum::extract::Query<EventsQuery>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<Vec<DirectoryEvent>>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let events = state.directory.events_since(query.since.unwrap_or(0), query.limit.unwrap_or(500)).await.map_err(directory_error_status)?;
    Ok(Json(events))
}

/// @emoji 🛡️ Contract §C2: actor kind `admin`, bypasses `authorize_directory_command` entirely
/// (unlike `POST /directory/commands`, this route never resolves a bearer user — `is_admin` alone
/// gates it). `create-space` still rejects an `Admin`-kind actor (`decide`'s own "create-space
/// requires a user actor" law) — an admin operator creating a space needs a real user session and
/// belongs on `/directory/commands` instead; this route is for acting ON existing spaces/members.
async fn admin_commands(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>, Json(command): Json<DirectoryCommand>) -> Result<(StatusCode, Json<DirectoryCommandResponse>), StatusCode> {
    if !is_admin(&state, &headers, Some(peer)) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let actor = DirectoryActor { kind: DirectoryActorKind::Admin, id: "admin".to_string() };
    let (events, result) = state.directory_service.execute(actor, command).await.map_err(directory_error_status)?;
    Ok((StatusCode::ACCEPTED, Json(DirectoryCommandResponse { events, result: command_result_json(result) })))
}

async fn admin_rebuild(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<serde_json::Value>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let replayed = state.directory.rebuild_projections().await.map_err(directory_error_status)?;
    Ok(Json(serde_json::json!({ "eventsReplayed": replayed })))
}

async fn admin_close_connection(Path(sync_session_id): Path<String>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> StatusCode {
    if !is_admin(&state, &headers, Some(peer)) {
        return StatusCode::UNAUTHORIZED;
    }
    match state.session_kicks.get(&sync_session_id) {
        Some(notify) => {
            notify.notify_one();
            StatusCode::NO_CONTENT
        }
        None => StatusCode::NOT_FOUND,
    }
}

/// @emoji 🦵️ Kicks every live connection for `user_id` (join of `list_active_sync_sessions` against
/// `session_kicks`, same as `admin_close_connection`). `HubDirectory` has no "enumerate a user's
/// browser `AuthSessionRecord`s" read, only revoke-by-id, so a login session this hub never saw
/// connect over WS cannot be enumerated here — flagged in this ticket's report as a trait gap for
/// the coordinator/1-A to consider; this route's contract name is still honored for every LIVE
/// connection, which is the only realtime-relevant half of "revoke this user's sessions" anyway.
async fn admin_revoke_user_sessions(Path(user_id): Path<String>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> StatusCode {
    if !is_admin(&state, &headers, Some(peer)) {
        return StatusCode::UNAUTHORIZED;
    }
    let Ok(sessions) = state.directory.list_active_sync_sessions(None).await else { return StatusCode::INTERNAL_SERVER_ERROR };
    for session in sessions.iter().filter(|session| session.user_id.as_deref() == Some(user_id.as_str())) {
        if let Some(notify) = state.session_kicks.get(&session.id) {
            notify.notify_one();
        }
    }
    StatusCode::NO_CONTENT
}
//#endregion 🔖️Admin

//#region 🔖️Extensions
/// @emoji 🧩️ Hub mirror of dev `staticDirVitePlugin` `/extensions` — lists installed extension metadata.
#[derive(Serialize)]
struct ExtensionListResponse {
    extensions: Vec<serde_json::Value>,
}

fn extension_asset_content_type(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|value| value.to_str()) {
        Some("js") | Some("mjs") => "text/javascript",
        Some("wasm") => "application/wasm",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

fn extension_asset_path(root: &std::path::Path, extension_id: &str, rest: &str) -> Option<std::path::PathBuf> {
    if extension_id.is_empty() || extension_id.contains('/') || extension_id.contains('\\') || extension_id.contains("..") {
        return None;
    }
    if rest.is_empty() || rest.contains("..") {
        return None;
    }
    let base = root.join(extension_id);
    let path = base.join(rest);
    if !path.starts_with(&base) {
        return None;
    }
    Some(path)
}

async fn list_extensions(State(state): State<HubState>) -> Result<Json<ExtensionListResponse>, StatusCode> {
    let mut extensions = Vec::new();
    let read_dir = tokio::fs::read_dir(&state.extensions_root).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut entries = read_dir;
    while let Ok(Some(entry)) = entries.next_entry().await {
        if !entry.file_type().await.map(|kind| kind.is_dir()).unwrap_or(false) {
            continue;
        }
        let meta_path = entry.path().join("install.json");
        let bytes = match tokio::fs::read(&meta_path).await {
            Ok(value) => value,
            Err(_) => continue,
        };
        let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        extensions.push(value);
    }
    extensions.sort_by(|left, right| {
        left.get("extensionId")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .cmp(right.get("extensionId").and_then(|value| value.as_str()).unwrap_or_default())
    });
    Ok(Json(ExtensionListResponse { extensions }))
}

async fn get_extension_asset(Path((extension_id, rest)): Path<(String, String)>, State(state): State<HubState>) -> Result<impl IntoResponse, StatusCode> {
    let path = extension_asset_path(&state.extensions_root, &extension_id, &rest).ok_or(StatusCode::BAD_REQUEST)?;
    let bytes = tokio::fs::read(&path).await.map_err(|error| if error.kind() == std::io::ErrorKind::NotFound { StatusCode::NOT_FOUND } else { StatusCode::INTERNAL_SERVER_ERROR })?;
    let content_type = extension_asset_content_type(&path);
    Ok(([(axum::http::header::CONTENT_TYPE, content_type)], bytes))
}
//#endregion 🔖️Extensions

//#region 🔖️AdminPage
/// @emoji 🛡️ Static SPA serving for `/admin` (contract §C2) — reads from `HubState.admin_dir`
/// (lane 1-B's field, `OS_HUB_ADMIN_DIR` else the compile-time default pointing at lane 2-E's own
/// `bun nx run os-hub-admin:build` output), mirroring `🔖️Extensions`'s `extension_asset_path`/
/// `get_extension_asset` pair exactly: plain `tokio::fs::read`, no `tower-http`, a traversal guard
/// before ever joining onto `root`, then a second `starts_with` check on the joined path as
/// defense-in-depth. SPA fallback: any requested path whose exact file is missing (a client-side
/// route, e.g. `/admin/spaces/sp-1`) falls back to `index.html`; a genuinely missing admin build
/// (nobody ran the vite build yet) is a 503 with a build hint, never a confusing 404 loop.
fn admin_asset_content_type(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|value| value.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") | Some("mjs") => "text/javascript",
        Some("css") => "text/css",
        Some("svg") => "image/svg+xml",
        Some("woff2") => "font/woff2",
        Some("json") => "application/json",
        Some("wasm") => "application/wasm",
        _ => "application/octet-stream",
    }
}

/// @emoji 🚧️ Rejects `..`, a backslash (a Windows separator smuggled into a URL path segment), and
/// strips every leading `/` before joining onto `root` — `PathBuf::join` treats an absolute second
/// argument as a full replacement of the base, which would otherwise let `rest = "/etc/passwd"` read
/// clean outside `root` entirely (the one way this guard differs from `extension_asset_path`, which
/// never sees a leading-slash `rest` in the first place).
fn admin_asset_path(root: &std::path::Path, rest: &str) -> Option<std::path::PathBuf> {
    if rest.contains("..") || rest.contains('\\') {
        return None;
    }
    let path = root.join(rest.trim_start_matches('/'));
    if !path.starts_with(root) {
        return None;
    }
    Some(path)
}

async fn admin_page(state: &HubState, rest: &str) -> axum::response::Response {
    let root = &state.admin_dir;
    if !root.is_dir() {
        return (StatusCode::SERVICE_UNAVAILABLE, "admin SPA not built — run: bun nx run os-hub-admin:build").into_response();
    }
    let Some(requested) = admin_asset_path(root, rest) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let path = if requested.is_file() { requested } else { root.join("index.html") };
    match tokio::fs::read(&path).await {
        Ok(bytes) => ([(axum::http::header::CONTENT_TYPE, admin_asset_content_type(&path))], bytes).into_response(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn get_admin_root(State(state): State<HubState>) -> impl IntoResponse {
    admin_page(&state, "index.html").await
}

async fn get_admin_asset(Path(rest): Path<String>, State(state): State<HubState>) -> impl IntoResponse {
    admin_page(&state, &rest).await
}
//#endregion 🔖️AdminPage

//#region 🔖️Main
fn router(state: HubState) -> Router {
    Router::new()
        .route("/auth/sessions", post(create_auth_session))
        .route("/auth/sessions/me", get(get_session_me).delete(delete_session_me))
        .route("/directory/commands", post(post_directory_commands))
        .route("/directory/spaces", get(get_directory_spaces))
        .route("/directory/spaces/{id}", get(get_directory_space))
        .route("/directory/invites/{token}/redeem", post(post_redeem_invite))
        .route("/directory/events", get(get_directory_events))
        .route("/directory/ws", get(directory_ws))
        .route("/admin/api/overview", get(admin_overview))
        .route("/admin/api/spaces", get(admin_spaces))
        .route("/admin/api/spaces/{id}", get(admin_space))
        .route("/admin/api/users", get(admin_users))
        .route("/admin/api/connections", get(admin_connections))
        .route("/admin/api/documents", get(admin_documents))
        .route("/admin/api/events", get(admin_events))
        .route("/admin/api/commands", post(admin_commands))
        .route("/admin/api/directory/rebuild", post(admin_rebuild))
        .route("/admin/api/connections/{sync_session_id}/close", post(admin_close_connection))
        .route("/admin/api/users/{id}/sessions/revoke", post(admin_revoke_user_sessions))
        .route("/extensions", get(list_extensions))
        .route("/extensions/{extension_id}/{*rest}", get(get_extension_asset))
        .route("/admin", get(get_admin_root))
        .route("/admin/{*path}", get(get_admin_asset))
        .route("/spaces/{space_id}/blobs/{hash}", get(get_blob).head(head_blob).put(put_blob))
        .route("/spaces/{space_id}/documents/{id}", get(get_document_status))
        .route("/spaces/{space_id}/documents/{id}/share", post(create_share))
        .route("/spaces/{space_id}/documents/{id}/ws", get(document_ws))
        // 🐙️ w4-h: router-wide CORS grant — see `cors_middleware`'s doc comment (`🔖️Directory` region)
        // for why this must cover the whole router, not just `/directory/*`.
        .layer(axum::middleware::from_fn(cors_middleware))
        .with_state(state)
}

/// @emoji 🧬️ Resolves and connects `db::Database`'s storage substrate, selected by
/// `OS_HUB_STORAGE_BACKEND` (`fs` default, zero-touch, rooted at `{data_dir}/db`; `sqlite`,
/// `postgres` — requires `OS_HUB_DATABASE_URL` — or `neo4j` — requires `OS_HUB_NEO4J_URI` —
/// otherwise, each match arm compiled only when this crate's same-named feature enables `db`'s own
/// matching storage feature). Independent of `connect_directory`'s own backend choice (the
/// contract's "storage swappability" requirement applies to `db`'s substrate and the directory's
/// substrate separately, even though both now share the same three feature names).
/// @emoji 🌉️ Hub's own `db::semio_framework_async::HostAsyncRuntime` — hub is a real concurrent
/// server under `#[tokio::main]` (unlike `db_cli`'s single-shot, inline-`run_blocking` runtime), so
/// `run_blocking` genuinely dispatches onto tokio's blocking-thread pool (`spawn_blocking`) rather
/// than ever running on (and stalling) the calling task's own worker thread. `db`'s own crates name
/// no `tokio` themselves (see `db_storage`'s module doc) — this is the one place in this binary
/// that bridges `db_storage::DbFuture`'s async-first boundary onto hub's already-tokio process,
/// mirroring `🛎️services::TokioHostRuntime`'s shape at a fraction of its scope (hub needs exactly
/// one capability off this trait: `run_blocking`).
struct HubDbRuntime;

impl db::semio_framework_async::HostAsyncRuntime for HubDbRuntime {
    fn open_scope(&self, owner: db::semio_framework_async::ScopeOwner, parent: Option<&db::semio_framework_async::ScopeHandle>) -> db::semio_framework_async::ScopeHandle {
        let cancel = match parent {
            Some(parent) => parent.cancel.child(),
            None => db::semio_framework_async::CancelToken::root(),
        };
        db::semio_framework_async::ScopeHandle { id: db::semio_framework_async::ScopeId(0), owner, cancel }
    }

    fn spawn_scoped(&self, _scope: &db::semio_framework_async::ScopeHandle, _ctx: db::semio_framework_async::OperationContext, fut: db::semio_framework_async::HostFuture<()>) {
        tokio::spawn(fut);
    }

    fn run_blocking(&self, _scope: &db::semio_framework_async::ScopeHandle, _ctx: db::semio_framework_async::OperationContext, work: Box<dyn FnOnce() + Send>) {
        tokio::task::spawn_blocking(work);
    }

    async fn sleep_until(&self, deadline_ms: u64) {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64);
        let delay = std::time::Duration::from_millis(deadline_ms.saturating_sub(now));
        tokio::time::sleep(delay).await;
    }

    async fn cancel_scope(&self, _owner: &db::semio_framework_async::ScopeOwner, _grace_ms: u64) -> db::semio_framework_async::ScopeDrainReport {
        // 🎯️ Hub never opens a cancellable scope of its own on this runtime today (only
        // `SqliteStorage::open`'s fixed internal scope, which it never cancels) — a real
        // drain-and-report implementation is `TokioHostRuntime`'s job (packet R2), not this
        // single-purpose bridge's.
        db::semio_framework_async::ScopeDrainReport::default()
    }

    fn now_ms(&self) -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
    }
}

async fn connect_db(data_dir: &std::path::Path) -> Result<db::Database, HubError> {
    let backend = std::env::var("OS_HUB_STORAGE_BACKEND").unwrap_or_else(|_| "fs".into());
    let profile = db::Profile::Prod;
    match backend.as_str() {
        "fs" | "" => {
            let root = data_dir.join("db");
            std::fs::create_dir_all(&root)?;
            Ok(db::Database::open_at(&root, profile)?)
        }
        #[cfg(feature = "sqlite")]
        "sqlite" => {
            let path = std::env::var("OS_HUB_DB_SQLITE").unwrap_or_else(|_| data_dir.join("db.sqlite3").to_string_lossy().into_owned());
            if let Some(parent) = std::path::Path::new(&path).parent() {
                std::fs::create_dir_all(parent)?;
            }
            use db::semio_framework_async::HostAsyncRuntime as _;
            let runtime: std::sync::Arc<dyn db::semio_framework_async::HostAsyncRuntime> = std::sync::Arc::new(HubDbRuntime);
            let scope = runtime.open_scope(db::semio_framework_async::ScopeOwner::Service("hub_db_sqlite"), None);
            let storage = db::storage_sqlite::SqliteStorage::open(runtime, scope, std::path::Path::new(&path))?;
            Ok(db::Database::open(db::DbConfig::for_profile(profile), Arc::new(storage))?)
        }
        #[cfg(feature = "postgres")]
        "postgres" => {
            let database_url = std::env::var("OS_HUB_DATABASE_URL").map_err(|_| HubError::UnknownStorageBackend("postgres requires OS_HUB_DATABASE_URL".into()))?;
            let storage = db::storage_postgres::PostgresStorage::connect(&database_url).await?;
            Ok(db::Database::open(db::DbConfig::for_profile(profile), Arc::new(storage))?)
        }
        #[cfg(feature = "neo4j")]
        "neo4j" => {
            let uri = std::env::var("OS_HUB_NEO4J_URI").map_err(|_| HubError::UnknownStorageBackend("neo4j requires OS_HUB_NEO4J_URI".into()))?;
            let user = std::env::var("OS_HUB_NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
            let password = std::env::var("OS_HUB_NEO4J_PASSWORD").unwrap_or_default();
            let storage = db::storage_neo4j::Neo4jStorage::connect(&uri, &user, &password).await?;
            Ok(db::Database::open(db::DbConfig::for_profile(profile), Arc::new(storage))?)
        }
        other => Err(HubError::UnknownStorageBackend(other.to_string())),
    }
}

/// @emoji 🧬️ Resolves and connects the identity/tenancy directory backend, selected by
/// `OS_HUB_DIRECTORY_BACKEND` (`sqlite` default, zero-touch, `{data_dir}/directory.db`; `postgres`
/// — requires `OS_HUB_DIRECTORY_DATABASE_URL` — or `neo4j` — requires
/// `OS_HUB_DIRECTORY_NEO4J_URI` — otherwise, each match arm compiled only when this crate's
/// same-named feature is enabled).
// 🌵️ `data_dir` is only read by the `sqlite` arm below (`postgres`/`neo4j` connect via env-provided
// URIs/connection strings instead, never a local path) — whenever the `sqlite` feature is off it
// goes genuinely unused (whether or not another backend feature is on), so the allow is scoped to
// exactly that condition rather than blanket-silencing the lint for every feature set.
#[cfg_attr(not(feature = "sqlite"), allow(unused_variables))]
async fn connect_directory(data_dir: &std::path::Path) -> Result<Arc<dyn HubDirectory>, HubError> {
    let backend = std::env::var("OS_HUB_DIRECTORY_BACKEND").unwrap_or_else(|_| "sqlite".into());
    match backend.as_str() {
        #[cfg(feature = "sqlite")]
        "sqlite" | "" => {
            let path = data_dir.join("directory.db");
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let directory = SqliteDirectory::connect(&path.to_string_lossy()).await?;
            directory.seed().await?;
            Ok(Arc::new(directory))
        }
        #[cfg(feature = "postgres")]
        "postgres" => {
            let database_url = std::env::var("OS_HUB_DIRECTORY_DATABASE_URL").map_err(|_| HubError::UnknownDirectoryBackend("postgres requires OS_HUB_DIRECTORY_DATABASE_URL".into()))?;
            let directory = semio_hub::directory::postgres::PostgresDirectory::connect(&database_url).await?;
            directory.seed().await?;
            Ok(Arc::new(directory))
        }
        #[cfg(feature = "neo4j")]
        "neo4j" => {
            let uri = std::env::var("OS_HUB_DIRECTORY_NEO4J_URI").map_err(|_| HubError::UnknownDirectoryBackend("neo4j requires OS_HUB_DIRECTORY_NEO4J_URI".into()))?;
            let user = std::env::var("OS_HUB_DIRECTORY_NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
            let password = std::env::var("OS_HUB_DIRECTORY_NEO4J_PASSWORD").unwrap_or_default();
            let directory = semio_hub::directory::neo4j::Neo4jDirectory::connect(&uri, &user, &password).await?;
            directory.seed().await?;
            Ok(Arc::new(directory))
        }
        other => Err(HubError::UnknownDirectoryBackend(other.to_string())),
    }
}

#[tokio::main]
async fn main() -> Result<(), HubError> {
    tracing_subscriber::fmt::init();
    let port: u16 = std::env::var("OS_HUB_PORT").ok().and_then(|value| value.parse().ok()).unwrap_or(8787);
    let data_dir = std::env::var("OS_HUB_DATA").map_or_else(|_| std::path::PathBuf::from("./.🧬semio/🌐hub/"), std::path::PathBuf::from);
    std::fs::create_dir_all(&data_dir)?;
    let db = connect_db(&data_dir).await?;
    let directory = connect_directory(&data_dir).await?;
    // 🧹️ Contract §C0: clear crash residue before any real connection lands — a session that never
    // got its `disconnected_at` because a previous process was killed mid-connection.
    directory.close_all_sync_sessions().await?;
    let directory_service = Arc::new(DirectoryService::new(directory.clone(), 1024));
    let admin_token = std::env::var("OS_HUB_ADMIN_TOKEN").ok().filter(|value| !value.is_empty());
    if admin_token.is_none() {
        // 🛡️ Contract §C0/§C2: no configured token ⇒ a loopback peer is implicitly admin (dev
        // default) — logged loudly once so this is never a silent surprise in a deployment that
        // forgot to set `OS_HUB_ADMIN_TOKEN`.
        tracing::warn!("OS_HUB_ADMIN_TOKEN is not set — /admin/api/* and document sharing fall back to loopback-peer-is-admin (dev default); set OS_HUB_ADMIN_TOKEN to require a bearer token instead");
    }
    let admin_dir = std::env::var("OS_HUB_ADMIN_DIR").map(std::path::PathBuf::from).unwrap_or_else(|_| std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔨️modules/🛡️admin/📦️packages/🟦️typescript/📤️dist")));
    let extensions_root = std::env::var("OS_HUB_EXTENSIONS_DIR").map(std::path::PathBuf::from).unwrap_or_else(|_| data_dir.join("extension-modules"));
    std::fs::create_dir_all(&extensions_root)?;
    let state = HubState {
        db: Arc::new(db),
        directory,
        directory_service,
        admin_token,
        admin_dir,
        fanout: Arc::new(DashMap::new()),
        presence: Arc::new(DashMap::new()),
        session_colors: Arc::new(DashMap::new()),
        session_kicks: Arc::new(DashMap::new()),
        schema_hashes: Arc::new(DashMap::new()),
        extensions_root,
        merge_policy: merge_policy_from_env(),
    };
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("os-hub listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router(state).into_make_service_with_connect_info::<SocketAddr>()).await?;
    Ok(())
}
//#endregion 🔖️Main

//#region 🔖️Tests
// 🪶️ Gated on the `sqlite` feature (not just `test`): every test below constructs a `HubState`
// through `SqliteDirectory` — the zero-external-dependency backend — so the full bin test suite
// naturally lives behind the same feature a plain `cargo test` already enables by default (see
// `Cargo.toml`'s `default = ["sqlite"]`). `postgres`/`neo4j` each carry their own backend-only
// tests in `📇️directory/{🐘️postgres,🌐️neo4j}/🦀️component.rs` instead of duplicating this suite.
#[cfg(all(test, feature = "sqlite"))]
mod tests {
    use super::*;
    use protocol::{Bootstrap, ArtifactId as WireArtifactId};
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Message as WsMessage;

    /// @emoji 🏛️ The seeded space id every test routes against (see `SqliteDirectory::seed`).
    const STUDIO: &str = "default";

    /// @emoji 📁️ A fresh, never-reused temp directory per call — `uuid::Uuid::now_v7` rather than
    /// `now_ms()` alone, since `cargo test` runs this whole module's `#[tokio::test]`s
    /// concurrently within one process: two tests calling `test_state()` in the same millisecond
    /// would otherwise collide on the identical `os-hub-test-db-<pid>-<ms>` path and open the SAME
    /// `db::Database` storage root, corrupting each other's catalog/WAL state.
    fn tempdir(name: &str) -> std::path::PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!("os-hub-test-{name}-{}", uuid::Uuid::now_v7()));
        dir
    }

    async fn test_state() -> HubState {
        let dir = tempdir("db");
        let database = db::Database::open_at(&dir, db::Profile::Test).expect("open db");
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect directory");
        directory.seed().await.expect("seed");
        let directory: Arc<dyn HubDirectory> = Arc::new(directory);
        let directory_service = Arc::new(DirectoryService::new(directory.clone(), 1024));
        HubState {
            db: Arc::new(database),
            directory,
            directory_service,
            admin_token: None,
            admin_dir: dir.join("admin-dist"),
            fanout: Arc::new(DashMap::new()),
            presence: Arc::new(DashMap::new()),
            session_colors: Arc::new(DashMap::new()),
            session_kicks: Arc::new(DashMap::new()),
            schema_hashes: Arc::new(DashMap::new()),
            extensions_root: dir.join("extension-modules"),
            merge_policy: protocol::MergePolicy::default(),
        }
    }

    /// @emoji 🏗️ Test-only `create-space` through `DirectoryService::execute` (the trait's own
    /// `create_space` write method is gone — see `📓️w1-b-report.md`) — returns the minted space id.
    /// `decide` performs zero authorization of its own, so `owner_user_id` need not be a real,
    /// already-existing user for these low-level fixture setups.
    async fn create_space_for_test(state: &HubState, owner_user_id: &str, name: &str, space_kind: os_directory::DirectorySpaceKind, visibility: DirectorySpaceVisibility) -> String {
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{owner_user_id}#test") };
        let (events, _) = state.directory_service.execute(actor, DirectoryCommand::CreateSpace { name: name.to_string(), space_kind, visibility }).await.expect("create space");
        events
            .into_iter()
            .find_map(|event| match event.body {
                os_directory::DirectoryEventBody::SpaceCreated { space_id, .. } => Some(space_id),
                _ => None,
            })
            .expect("space.created event")
    }

    /// @emoji 🏗️ Test-only `upsert-member` through `DirectoryService::execute` — `email` must match
    /// an already-minted `AuthSessionRecord`'s user for the member to land on that SAME user rather
    /// than a freshly-created one (`decide`'s `UpsertMember` resolves-or-creates by email).
    async fn upsert_member_for_test(state: &HubState, space_id: &str, email: &str, role: DirectorySpaceRole) {
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#test".to_string() };
        state.directory_service.execute(actor, DirectoryCommand::UpsertMember { space_id: space_id.to_string(), email: email.to_string(), role }).await.expect("upsert member");
    }

    fn sample_envelope(id: &str, document: &WireArtifactId) -> MutationEnvelope {
        MutationEnvelope {
            mutation_id: protocol::MutationId(id.to_string()),
            document_id: document.clone(),
            actor: ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({ "value": id })).unwrap() },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({})).unwrap() },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        }
    }

    async fn spawn_server(state: HubState) -> SocketAddr {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let app = router(state).into_make_service_with_connect_info::<SocketAddr>();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        addr
    }

    /// @emoji 🧪️ A loopback `ConnectInfo` for handlers under test called directly (not through a
    /// real socket accept) — every test-suite caller of `create_share` used to rely on the admin
    /// bearer check alone; `is_admin` now also accepts loopback when no token is configured, so
    /// tests that DO configure a token still exercise the bearer branch unaffected.
    fn loopback_peer() -> axum::extract::ConnectInfo<SocketAddr> {
        axum::extract::ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 0)))
    }

    async fn next_server_frame<S>(ws: &mut S) -> ServerFrame
    where
        S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
    {
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
        loop {
            match tokio::time::timeout_at(deadline, ws.next()).await {
                Ok(Some(Ok(WsMessage::Binary(bytes)))) => return protocol::decode_server_frame(&bytes).expect("server frame").1,
                Ok(Some(Ok(_))) => continue,
                Ok(Some(other)) => panic!("expected binary frame, got {other:?}"),
                Ok(None) => panic!("stream ended before server frame"),
                Err(_) => panic!("no server frame before 5s deadline"),
            }
        }
    }

    fn client_binary(frame: &ClientFrame, lane: Lane) -> WsMessage {
        WsMessage::Binary(protocol::encode_client_frame(frame, lane).into())
    }

    fn hello(actor: &str) -> ClientFrame {
        ClientFrame::Hello { wire_version: 1, protocol_version: 1, schema: "test.v1".to_string(), pack_schema_hash: [0u8; 32], actor: ActorId(actor.to_string()), token: None, resume_token: None, frontier: None }
    }

    // 🔬️ WS duplex fan-out over the real wire-v2 protocol: A's committed command reaches B on its
    // own socket as a `ServerFrame::Commands`, and B's Ack for A's own submit never round-trips
    // back to A as a duplicate Commands frame (origin filtering is the caller's job — this test
    // only asserts B observes it, matching `framework/sync`'s own origin check).
    #[tokio::test]
    async fn ws_duplex_fan_out() {
        let addr = spawn_server(test_state().await).await;
        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/default/ws");

        let (mut a, _) = connect_async(&url).await.unwrap();
        a.send(client_binary(&hello("A"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Session { .. }));

        let (mut b, _) = connect_async(&url).await.unwrap();
        b.send(client_binary(&hello("B"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut b).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut b).await, ServerFrame::Session { .. }));

        let document = WireArtifactId(format!("{STUDIO}:default"));
        a.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("op-1", &document)] }, Lane::Command)).await.unwrap();

        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Ack { batch_id: 1, .. }));

        loop {
            match next_server_frame(&mut b).await {
                ServerFrame::Commands { envelopes, origin, .. } => {
                    assert_eq!(envelopes.len(), 1);
                    assert_eq!(envelopes[0].mutation_id.0, "op-1");
                    assert_eq!(origin, ActorId("A".to_string()));
                    break;
                }
                ServerFrame::Presence { .. } => continue,
                other => panic!("unexpected frame on B: {other:?}"),
            }
        }
    }

    // 🔬️ A reconnecting client whose `Hello.frontier` is stale gets the missing commands replayed
    // via `Welcome`'s `Bootstrap::Tail` follow-up — the `db::Database::hello` integration.
    #[tokio::test]
    async fn reconnect_replays_missing_commands_via_bootstrap_tail() {
        let addr = spawn_server(test_state().await).await;
        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/default/ws");
        let document = WireArtifactId(format!("{STUDIO}:default"));

        let (mut a, _) = connect_async(&url).await.unwrap();
        a.send(client_binary(&hello("A"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Session { .. }));
        a.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("op-1", &document)] }, Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Ack { .. }));

        // A fresh connection with no prior frontier must see the already-committed op-1 in its
        // Welcome bootstrap follow-up, sent BEFORE the connection's own `Session` frame (contract
        // §C7.3: Session is sent after Welcome AND its follow-up bootstrap frames).
        let (mut c, _) = connect_async(&url).await.unwrap();
        c.send(client_binary(&hello("C"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut c).await, ServerFrame::Welcome { bootstrap: Bootstrap::Tail, .. }));
        match next_server_frame(&mut c).await {
            ServerFrame::Commands { envelopes, .. } => assert_eq!(envelopes[0].mutation_id.0, "op-1"),
            other => panic!("expected the Tail bootstrap's Commands follow-up, got {other:?}"),
        }
        assert!(matches!(next_server_frame(&mut c).await, ServerFrame::Session { .. }));
    }

    // 🔬️ Space-scoped documents: the same document id in two different studios lands in two
    // independent `db` documents (the `{space_id}:{document_id}` scope key) — a peer on
    // space-b's `shared-doc` never observes space-a's commands.
    #[tokio::test]
    async fn space_scoped_documents_are_isolated() {
        let state = test_state().await;
        let addr = spawn_server(state).await;

        let url_a = format!("ws://{addr}/spaces/space-a/documents/shared-doc/ws");
        let (mut a, _) = connect_async(&url_a).await.unwrap();
        a.send(client_binary(&hello("A"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Session { .. }));
        let document = WireArtifactId("space-a:shared-doc".to_string());
        a.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("only-in-a", &document)] }, Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Ack { .. }));

        let url_b = format!("ws://{addr}/spaces/space-b/documents/shared-doc/ws");
        let (mut b, _) = connect_async(&url_b).await.unwrap();
        b.send(client_binary(&hello("B"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut b).await, ServerFrame::Welcome { bootstrap: Bootstrap::None, .. }), "space-b's document must not see space-a's committed op");
    }

    // 🔬️ Auth-lite: issuing a share token closes an otherwise-open document to a tokenless Hello.
    #[tokio::test]
    async fn share_token_gates_ws_access() {
        let state = test_state().await;
        let admin_state = HubState { admin_token: Some("admin-secret".to_string()), ..state };
        let addr = spawn_server(admin_state.clone()).await;

        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::AUTHORIZATION, "Bearer admin-secret".parse().unwrap());
        let share = create_share(Path((STUDIO.to_string(), "guarded".to_string())), headers, loopback_peer(), State(admin_state)).await.expect("share");

        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/guarded/ws");
        let (mut denied, _) = connect_async(&url).await.unwrap();
        denied.send(client_binary(&hello("intruder"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut denied).await, ServerFrame::Error { code, .. } if code == "unauthorized"));

        let (mut allowed, _) = connect_async(&url).await.unwrap();
        allowed
            .send(client_binary(
                &ClientFrame::Hello { wire_version: 1, protocol_version: 1, schema: "test.v1".to_string(), pack_schema_hash: [0u8; 32], actor: ActorId("holder".to_string()), token: Some(share.0.token), resume_token: None, frontier: None },
                Lane::Command,
            ))
            .await
            .unwrap();
        assert!(matches!(next_server_frame(&mut allowed).await, ServerFrame::Welcome { .. }));
    }

    // 🔬️ `SecurityGate` wiring: a `Spectator` session may `Hello` into a document (reads are
    // unaffected) but a `Commands` submission is rejected — `admit_writes` catches it via
    // `space_grants`'s role-scoped `RoleBasedPolicy` before `db::ArtifactHandle::submit` ever
    // runs — while an `Author` session on the same space succeeds, proving the gate is wired into
    // the real write path rather than merely unit-tested in isolation.
    #[tokio::test]
    async fn security_gate_rejects_spectator_writes_and_allows_author_writes() {
        let state = test_state().await;
        let space = create_space_for_test(&state, "seed", "Gated Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;

        let spectator_session = create_auth_session(State(state.clone()), Json(CreateAuthSessionRequest { email: "spectator@example.com".into() })).await.expect("mint spectator session");
        upsert_member_for_test(&state, &space, "spectator@example.com", DirectorySpaceRole::Spectator).await;

        let author_session = create_auth_session(State(state.clone()), Json(CreateAuthSessionRequest { email: "author@example.com".into() })).await.expect("mint author session");
        upsert_member_for_test(&state, &space, "author@example.com", DirectorySpaceRole::Author).await;

        let addr = spawn_server(state).await;
        let url = format!("ws://{addr}/spaces/{space}/documents/gated-doc/ws");
        let document = WireArtifactId(format!("{space}:gated-doc"));
        let hello_with_token = |actor: &str, token: String| ClientFrame::Hello { wire_version: 1, protocol_version: 1, schema: "test.v1".to_string(), pack_schema_hash: [0u8; 32], actor: ActorId(actor.to_string()), token: Some(token), resume_token: None, frontier: None };

        let (mut spectator, _) = connect_async(&url).await.unwrap();
        spectator.send(client_binary(&hello_with_token("spectator", spectator_session.0.token), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut spectator).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut spectator).await, ServerFrame::Session { .. }));
        spectator.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("op-spectator", &document)] }, Lane::Command)).await.unwrap();
        match next_server_frame(&mut spectator).await {
            ServerFrame::Ack { stages, .. } => {
                assert_eq!(stages.len(), 1);
                match &stages[0] {
                    AckStage::Applied { outcome } => assert!(matches!(outcome.as_ref(), ApplyOutcome::Rejected { .. }), "a spectator's write must be rejected by the security gate"),
                    _ => panic!("expected a single Applied stage"),
                }
            }
            _ => panic!("expected an ack frame"),
        }

        let (mut author, _) = connect_async(&url).await.unwrap();
        author.send(client_binary(&hello_with_token("author", author_session.0.token), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut author).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut author).await, ServerFrame::Session { .. }));
        author.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("op-author", &document)] }, Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut author).await, ServerFrame::Ack { batch_id: 1, .. }));
    }

    // 🔬️ Blob round-trip: PUT then GET returns identical bytes and HEAD reports found, through
    // `db::Database`'s own content-addressed payload store; a hash that was never PUT is reported
    // missing by both GET and HEAD.
    #[tokio::test]
    async fn blob_put_get_head_round_trip() {
        let state = test_state().await;
        let bytes = Bytes::from_static(b"hello hub blob bytes");
        let expected_hash = state.db.storage().payload().put(&bytes).await.unwrap().to_string();
        // A re-put through the route with the correct address must be idempotent and agree.
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::CONTENT_TYPE, "text/plain".parse().unwrap());
        let put = put_blob(Path((STUDIO.to_string(), expected_hash.clone())), headers, State(state.clone()), bytes.clone()).await.expect("put blob");
        assert_eq!(put.0.hash, expected_hash);
        assert_eq!(put.0.size, bytes.len() as i64);

        let response = get_blob(Path((STUDIO.to_string(), expected_hash.clone())), HeaderMap::new(), State(state.clone())).await.expect("get blob").into_response();
        let got = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("read body");
        assert_eq!(got.as_ref(), bytes.as_ref());

        assert_eq!(head_blob(Path((STUDIO.to_string(), expected_hash.clone())), HeaderMap::new(), State(state.clone())).await, StatusCode::OK);

        let missing = "0".repeat(64);
        assert_eq!(head_blob(Path((STUDIO.to_string(), missing.clone())), HeaderMap::new(), State(state.clone())).await, StatusCode::NOT_FOUND);
        assert_eq!(get_blob(Path((STUDIO.to_string(), missing)), HeaderMap::new(), State(state)).await.err(), Some(StatusCode::NOT_FOUND));
    }

    // 🔬️ A client-provided hash that doesn't match the computed content hash is a bad request.
    #[tokio::test]
    async fn blob_put_rejects_hash_mismatch() {
        let state = test_state().await;
        let bytes = Bytes::from_static(b"mismatched content");
        let result = put_blob(Path((STUDIO.to_string(), "0".repeat(64))), HeaderMap::new(), State(state), bytes).await;
        assert_eq!(result.err(), Some(StatusCode::BAD_REQUEST));
    }

    // 🔬️ A `visibility == "public"` space grants an anonymous caller an implicit
    // `AuthOutcome::Public` — the hub-handler-level fallback, never a policy-engine concept (see
    // `AuthOutcome::Public`'s doc) — once the tokenless-open-by-default share-token scheme no
    // longer resolves the request itself (a share token has been issued for the document, closing
    // it); a private space with the same shape stays denied.
    #[tokio::test]
    async fn public_visibility_grants_anonymous_spectator_fallback() {
        let state = test_state().await;
        let public_space = create_space_for_test(&state, "seed", "Public Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        let private_space = create_space_for_test(&state, "seed", "Private Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        state.directory.create_share_token("guarded-doc").await.expect("close document with a share token");

        assert!(matches!(resolve_auth(&state, &public_space, "guarded-doc", None).await, AuthOutcome::Public));
        assert!(matches!(resolve_auth(&state, &private_space, "guarded-doc", None).await, AuthOutcome::Denied));
    }

    // 🔬️ Auth sessions: POST /auth/sessions mints a session that resolves the caller's space role
    // and grants access even to a document a share token has otherwise closed.
    #[tokio::test]
    async fn auth_session_grants_role_and_bypasses_share_gate() {
        let state = test_state().await;
        // `hub_space_membership.space_id` is FK-bound to `hub_space(id)` — a real studio, not a
        // bare string, matching how `create_auth_session`'s minted user must also be a real row.
        let studio = create_space_for_test(&state, "seed", "Space X", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let document = "closed-doc";
        state.directory.create_share_token(document).await.expect("close with share token");
        assert!(!state.directory.authorized_by_token(document, None).await.unwrap());

        let minted = create_auth_session(State(state.clone()), Json(CreateAuthSessionRequest { email: "dev@example.com".into() })).await.expect("mint session");
        upsert_member_for_test(&state, &studio, "dev@example.com", DirectorySpaceRole::Spectator).await;

        assert!(!authorized(&state, &studio, document, None).await, "tokenless request still denied");
        assert!(authorized(&state, &studio, document, Some(&minted.0.token)).await, "session token authorized despite no share token");

        match resolve_auth(&state, &studio, document, Some(&minted.0.token)).await {
            AuthOutcome::Session { user_id, role } => {
                assert_eq!(user_id, minted.0.user_id);
                assert_eq!(role, SpaceRole::Spectator);
            }
            _ => panic!("expected a resolved session"),
        }
    }

    // 🔬️ GET .../documents/{id} reports the document's current frontier, lazily minting it in
    // `db`'s catalog on first access.
    #[tokio::test]
    async fn document_status_reports_frontier_and_lazily_mints() {
        let state = test_state().await;
        let status = get_document_status(Path((STUDIO.to_string(), "fresh".to_string())), HeaderMap::new(), State(state.clone())).await.expect("status");
        assert_eq!(status.0.head_seq, 0);

        let handle = state.ensure_document(&db_artifact_id(STUDIO, "fresh")).expect("ensure");
        let batch = db::document::CommandBatch::new(vec![sample_envelope("op-1", &db_artifact_id(STUDIO, "fresh"))]).unwrap();
        handle.submit(batch, db::document::SubmitOptions::default()).await.unwrap().unwrap();

        let status = get_document_status(Path((STUDIO.to_string(), "fresh".to_string())), HeaderMap::new(), State(state)).await.expect("status after submit");
        assert_eq!(status.0.head_seq, 1);
    }

    async fn next_directory_message<S>(ws: &mut S) -> DirectoryStreamMessage
    where
        S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
    {
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
        loop {
            match tokio::time::timeout_at(deadline, ws.next()).await {
                Ok(Some(Ok(WsMessage::Text(text)))) => return serde_json::from_str(&text).expect("directory stream message decodes"),
                Ok(Some(Ok(_))) => continue,
                Ok(Some(other)) => panic!("expected a text frame, got {other:?}"),
                Ok(None) => panic!("stream ended before a directory message"),
                Err(_) => panic!("no directory message before the 5s deadline"),
            }
        }
    }

    // 🔬️ `POST /directory/commands` -> `DirectoryService::execute` -> `HubDirectory::append_events`
    // -> `GET /directory/spaces` re-folds the log and projects the caller's own role/member count.
    #[tokio::test]
    async fn directory_commands_append_events_and_project() {
        let state = test_state().await;
        let space_id = create_space_for_test(&state, "seed", "Atelier Alpha", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;

        let events = state.directory.events_since(0, usize::MAX).await.expect("events");
        assert!(events.iter().any(|event| matches!(&event.body, os_directory::DirectoryEventBody::SpaceCreated { space_id: id, .. } if id == &space_id)));
        assert!(events.iter().any(|event| matches!(&event.body, os_directory::DirectoryEventBody::MemberUpserted { space_id: id, user_id, .. } if id == &space_id && user_id == "seed")));

        let owner_session = create_auth_session(State(state.clone()), Json(CreateAuthSessionRequest { email: "owner-user@example.com".into() })).await.expect("mint owner session");
        upsert_member_for_test(&state, &space_id, "owner-user@example.com", DirectorySpaceRole::Author).await;

        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", owner_session.0.token).parse().unwrap());
        let spaces = get_directory_spaces(headers, State(state)).await.expect("list spaces");
        let projected = spaces.0.iter().find(|space| space.id == space_id).expect("projected space");
        assert_eq!(projected.name, "Atelier Alpha");
        assert_eq!(projected.role, Some(DirectorySpaceRole::Author));
        assert_eq!(projected.member_count, 2, "the synthetic owner actor plus the newly granted author");
    }

    // 🔬️ `/directory/ws?since=0`: subscribe-then-replay is visibility-filtered exactly like `GET
    // /directory/events` — B only ever sees events for spaces B belongs to, both in the replay
    // (events already committed before B connects) and in the live tail (events committed after).
    #[tokio::test]
    async fn directory_ws_replays_then_streams_live() {
        let state = test_state().await;
        let space_mine = create_space_for_test(&state, "seed", "B's Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let space_other = create_space_for_test(&state, "seed", "Other Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let b_session = create_auth_session(State(state.clone()), Json(CreateAuthSessionRequest { email: "b@example.com".into() })).await.expect("mint B session");
        upsert_member_for_test(&state, &space_other, "someone-else@example.com", DirectorySpaceRole::Spectator).await;
        upsert_member_for_test(&state, &space_mine, "b@example.com", DirectorySpaceRole::Author).await;

        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/directory/ws?token={}&since=0", b_session.0.token);
        let (mut ws, _) = connect_async(&url).await.unwrap();

        // Replay: the seeded `default` studio's own `user.created` (space-less, always visible), the
        // fresh `user.created` for "someone-else" (`upsert_member_for_test`'s unknown-email path,
        // also space-less/always-visible), plus the 3 events of `space_mine` — `space_other`'s
        // `space.created`/`member.upserted` never arrive. 5 visible events total.
        let mut seen_spaces = std::collections::HashSet::new();
        let mut saw_own_membership = false;
        for _ in 0..5u32 {
            match next_directory_message(&mut ws).await {
                DirectoryStreamMessage::Event { event } => {
                    if let Some(space_id) = &event.space_id {
                        seen_spaces.insert(space_id.clone());
                    }
                    if matches!(&event.body, os_directory::DirectoryEventBody::MemberUpserted { user_id, .. } if user_id == &b_session.0.user_id) {
                        saw_own_membership = true;
                    }
                }
                other => panic!("expected an Event during replay, got {other:?}"),
            }
        }
        assert!(saw_own_membership, "B must see the replayed member.upserted naming them");
        assert_eq!(seen_spaces, std::collections::HashSet::from([space_mine.clone()]), "B must never see space_other's events");

        // Live: the same filter holds for events committed AFTER B is already connected. Both
        // `upsert_member_for_test` calls below mint a brand-new user first (space-less `user.created`,
        // always visible) before their `member.upserted` — skip those and check the first
        // space-scoped event is `space_mine`'s, never `space_other`'s.
        upsert_member_for_test(&state, &space_other, "yet-another@example.com", DirectorySpaceRole::Spectator).await;
        upsert_member_for_test(&state, &space_mine, "second-member@example.com", DirectorySpaceRole::Spectator).await;
        loop {
            match next_directory_message(&mut ws).await {
                DirectoryStreamMessage::Event { event } if event.space_id.is_none() => continue,
                DirectoryStreamMessage::Event { event } => {
                    assert_eq!(event.space_id.as_deref(), Some(space_mine.as_str()), "space_other's live event must be dropped, not delivered");
                    break;
                }
                other => panic!("expected an Event message, got {other:?}"),
            }
        }
    }

    // 🔬️ Contract §C7.0/§C7.3: the roster is document-wide now — A (`surface=editor`) and C
    // (`surface=viewer`) on the SAME document see each other's presence bytes; `surface` no longer
    // scopes any broadcast channel (`surface_fanout` is deleted, `ServerFrame::Presence` fans out on
    // the document-wide `fanout` alongside `Commands`) — it travels only INSIDE each peer's opaque
    // `PresencePeer` bytes, which this hub stores and forwards without ever decoding.
    #[tokio::test]
    async fn presence_roster_is_document_wide_and_frames_carry_surface_only_inside_peer() {
        let addr = spawn_server(test_state().await).await;
        let url_editor = format!("ws://{addr}/spaces/{STUDIO}/documents/shared/ws?surface=editor");
        let url_viewer = format!("ws://{addr}/spaces/{STUDIO}/documents/shared/ws?surface=viewer");

        let (mut a, _) = connect_async(&url_editor).await.unwrap();
        a.send(client_binary(&hello("A"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Session { .. }));

        let (mut c, _) = connect_async(&url_viewer).await.unwrap();
        c.send(client_binary(&hello("C"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut c).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut c).await, ServerFrame::Session { .. }));

        a.send(client_binary(&ClientFrame::Presence { peer: b"A-presence".to_vec() }, Lane::Command)).await.unwrap();
        loop {
            match next_server_frame(&mut c).await {
                ServerFrame::Presence { peers } => {
                    assert!(peers.contains(&b"A-presence".to_vec()), "C (different surface, SAME document) must see A's presence — the roster is document-wide, not surface-scoped");
                    break;
                }
                other => panic!("unexpected frame on C: {other:?}"),
            }
        }

        // A also subscribes to the SAME document-wide `fanout` it just published on, so its own
        // presence publish loops back to it too — proving there is only one channel now, not a
        // second surface-scoped one A alone would be on.
        match next_server_frame(&mut a).await {
            ServerFrame::Presence { peers } => assert!(peers.contains(&b"A-presence".to_vec()), "A observes its own publish via the document-wide fanout"),
            other => panic!("unexpected frame on A: {other:?}"),
        }
    }

    // 🔬️ Contract §C7.3 session colors: the lowest free index in `0..=255` per SPACE (not per
    // document) — A gets 0, B gets 1; A's second document socket in the SAME space reuses A's
    // existing lease (still 0, ref-counted) rather than minting a new one; once BOTH of A's sockets
    // close, color 0 is freed and a brand-new actor C is assigned it (B, still connected, keeps 1).
    #[tokio::test]
    async fn session_frame_assigns_lowest_free_color_per_space_and_releases_on_last_disconnect() {
        let addr = spawn_server(test_state().await).await;
        let url = |document: &str| format!("ws://{addr}/spaces/{STUDIO}/documents/{document}/ws");

        async fn welcome_and_session<S>(ws: &mut S) -> (String, u8)
        where
            S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
        {
            assert!(matches!(next_server_frame(ws).await, ServerFrame::Welcome { .. }));
            match next_server_frame(ws).await {
                ServerFrame::Session { actor, color } => (actor, color),
                other => panic!("expected Session, got {other:?}"),
            }
        }

        let (mut a1, _) = connect_async(&url("doc1")).await.unwrap();
        a1.send(client_binary(&hello("A"), Lane::Command)).await.unwrap();
        assert_eq!(welcome_and_session(&mut a1).await, ("A".to_string(), 0));

        let (mut b, _) = connect_async(&url("doc1")).await.unwrap();
        b.send(client_binary(&hello("B"), Lane::Command)).await.unwrap();
        assert_eq!(welcome_and_session(&mut b).await, ("B".to_string(), 1));

        // A's second document socket, same space: the existing lease is reused (still 0), not a new
        // lowest-free index (which would otherwise be 2).
        let (mut a2, _) = connect_async(&url("doc2")).await.unwrap();
        a2.send(client_binary(&hello("A"), Lane::Command)).await.unwrap();
        assert_eq!(welcome_and_session(&mut a2).await, ("A".to_string(), 0));

        drop(a1);
        drop(a2);
        // Let both of A's handler tasks observe the socket close and release their color lease's ref.
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        let (mut c, _) = connect_async(&url("doc1")).await.unwrap();
        c.send(client_binary(&hello("C"), Lane::Command)).await.unwrap();
        assert_eq!(welcome_and_session(&mut c).await, ("C".to_string(), 0), "color 0 is freed once BOTH of A's document sockets disconnect, and is the lowest free index (B still holds 1)");
    }

    // 🔬️ Amendment 3 to C1: `DirectoryStreamMessage::Presence` is actually published (it used to be
    // defined but never sent) — `spaceId`/`documentId` name the roster, and each
    // `DirectoryPresenceActor` carries the `surface`/`color` this hub knows without ever decoding the
    // actor's opaque `PresencePeer` bytes.
    #[tokio::test]
    async fn directory_ws_publishes_presence_roster_with_surface_and_color() {
        let state = test_state().await;
        let addr = spawn_server(state.clone()).await;

        let observer_session = create_auth_session(State(state.clone()), Json(CreateAuthSessionRequest { email: "presence-observer@example.com".into() })).await.expect("mint observer session");
        let dir_url = format!("ws://{addr}/directory/ws?token={}&since=0", observer_session.0.token);
        let (mut observer, _) = connect_async(&dir_url).await.unwrap();
        // `since=0` replays only the seeded `default` studio's own space-less `user.created` (see
        // `connection_events_reach_admin_stream`'s doc) — draining it also proves the observer's live
        // loop is already running before the document connection below publishes anything.
        match next_directory_message(&mut observer).await {
            DirectoryStreamMessage::Event { .. } => {}
            other => panic!("expected the seeded replay, got {other:?}"),
        }

        let doc_url = format!("ws://{addr}/spaces/{STUDIO}/documents/watched-presence/ws?surface=editor");
        let (mut doc, _) = connect_async(&doc_url).await.unwrap();
        doc.send(client_binary(&hello("presence-actor"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut doc).await, ServerFrame::Welcome { .. }));
        let color = match next_server_frame(&mut doc).await {
            ServerFrame::Session { color, .. } => color,
            other => panic!("expected Session, got {other:?}"),
        };

        doc.send(client_binary(&ClientFrame::Presence { peer: b"presence-actor-bytes".to_vec() }, Lane::Command)).await.unwrap();

        loop {
            match next_directory_message(&mut observer).await {
                DirectoryStreamMessage::Connection { .. } => continue,
                DirectoryStreamMessage::Presence { space_id, document_id, actors } => {
                    assert_eq!(space_id, STUDIO);
                    assert_eq!(document_id, "watched-presence");
                    let actor = actors.iter().find(|actor| actor.actor == "presence-actor").expect("presence-actor in the published roster");
                    assert_eq!(actor.surface, "editor");
                    assert_eq!(actor.color, color);
                    break;
                }
                other => panic!("expected Presence, got {other:?}"),
            }
        }
    }

    // 🔬️ `record_sync_session_open`/`_close` publish `DirectoryStreamMessage::Connection{phase}` —
    // any `/directory/ws` observer sees a document WS session's open and close in real time.
    #[tokio::test]
    async fn connection_events_reach_admin_stream() {
        let state = test_state().await;
        let observer_session = create_auth_session(State(state.clone()), Json(CreateAuthSessionRequest { email: "observer@example.com".into() })).await.expect("mint observer session");

        let addr = spawn_server(state).await;
        let dir_url = format!("ws://{addr}/directory/ws?token={}&since=0", observer_session.0.token);
        let (mut observer, _) = connect_async(&dir_url).await.unwrap();
        // `since=0` also replays the seeded `default` studio's own `user.created` (visible to
        // everyone, per `event_visible`'s doc) — draining it also proves the observer's live loop is
        // already running by the time the document connection below publishes, with no other
        // synchronization point needed.
        match next_directory_message(&mut observer).await {
            DirectoryStreamMessage::Event { .. } => {}
            other => panic!("expected the seeded user.created replay, got {other:?}"),
        }

        let doc_url = format!("ws://{addr}/spaces/{STUDIO}/documents/watched/ws");
        let (mut doc, _) = connect_async(&doc_url).await.unwrap();
        doc.send(client_binary(&hello("watched-actor"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut doc).await, ServerFrame::Welcome { .. }));

        match next_directory_message(&mut observer).await {
            DirectoryStreamMessage::Connection { phase, connection } => {
                assert_eq!(phase, DirectoryConnectionPhase::Opened);
                assert_eq!(connection.document_id, "watched");
                assert_eq!(connection.actor, "watched-actor");
            }
            other => panic!("expected Connection(Opened), got {other:?}"),
        }

        drop(doc);
        match next_directory_message(&mut observer).await {
            DirectoryStreamMessage::Connection { phase, .. } => assert_eq!(phase, DirectoryConnectionPhase::Closed),
            other => panic!("expected Connection(Closed), got {other:?}"),
        }
    }

    // 🔬️ Admin API round trip: spaces/users/connections list what setup created, `presenceKnown`
    // tracks whether that connection has published a `ClientFrame::Presence` yet, and closing a
    // listed connection's `syncSessionId` actually kicks the live document WS session.
    #[tokio::test]
    async fn admin_api_lists_spaces_users_connections_and_kicks() {
        let state = test_state().await;
        let space_id = create_space_for_test(&state, "seed", "Admin Visible Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let _ = create_auth_session(State(state.clone()), Json(CreateAuthSessionRequest { email: "someone@example.com".into() })).await.expect("mint session");

        let addr = spawn_server(state.clone()).await;
        let doc_url = format!("ws://{addr}/spaces/{STUDIO}/documents/kickable/ws");
        let (mut doc, _) = connect_async(&doc_url).await.unwrap();
        doc.send(client_binary(&hello("kick-me"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut doc).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut doc).await, ServerFrame::Session { .. }));
        // Let the server side finish recording the sync session before the admin reads it back.
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let spaces = admin_spaces(HeaderMap::new(), loopback_peer(), State(state.clone())).await.expect("admin spaces");
        assert!(spaces.0.iter().any(|space| space.id == space_id));

        let users = admin_users(HeaderMap::new(), loopback_peer(), State(state.clone())).await.expect("admin users");
        assert!(users.0.iter().any(|user| user.email == "someone@example.com"));

        let connections = admin_connections(HeaderMap::new(), loopback_peer(), State(state.clone())).await.expect("admin connections");
        let connection = connections.0.iter().find(|connection| connection.actor == "kick-me").expect("kickable connection listed");
        assert!(!connection.presence_known, "no ClientFrame::Presence published yet");

        doc.send(client_binary(&ClientFrame::Presence { peer: b"kick-me-presence".to_vec() }, Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut doc).await, ServerFrame::Presence { .. }));
        let connections = admin_connections(HeaderMap::new(), loopback_peer(), State(state.clone())).await.expect("admin connections after presence");
        let connection = connections.0.iter().find(|connection| connection.actor == "kick-me").expect("kickable connection still listed");
        assert!(connection.presence_known, "presenceKnown flips true once the actor's PresenceSession carries a peer");

        assert_eq!(admin_close_connection(Path(connection.sync_session_id.clone()), HeaderMap::new(), loopback_peer(), State(state.clone())).await, StatusCode::NO_CONTENT);
        let closed = tokio::time::timeout(std::time::Duration::from_secs(5), doc.next()).await.expect("connection closes before the 5s deadline");
        // The kicked session's task just `break`s its select loop and drops the socket — no clean WS
        // close handshake is sent, so the client observes either a `Close` frame, a stream end, or
        // (most commonly, since the TCP connection drops mid-handshake) a protocol error. Any of the
        // three means the connection is gone, which is what the kick promises.
        assert!(matches!(closed, Some(Ok(WsMessage::Close(_))) | None | Some(Err(_))), "the kicked connection must close, got {closed:?}");
    }

    // 🔬️ `is_admin`: loopback is the dev-default admin when `OS_HUB_ADMIN_TOKEN` is unset (and a
    // non-loopback peer never is); once a token IS configured, loopback alone no longer suffices —
    // only the matching bearer token does, from any peer address.
    #[tokio::test]
    async fn admin_loopback_default_and_bearer_when_configured() {
        let state = test_state().await;
        assert!(is_admin(&state, &HeaderMap::new(), Some(SocketAddr::from(([127, 0, 0, 1], 0)))), "loopback is admin when no token is configured");
        assert!(!is_admin(&state, &HeaderMap::new(), Some(SocketAddr::from(([10, 0, 0, 5], 0)))), "a non-loopback peer is never admin when no token is configured");

        let configured = HubState { admin_token: Some("s3cret".to_string()), ..state };
        assert!(!is_admin(&configured, &HeaderMap::new(), Some(SocketAddr::from(([127, 0, 0, 1], 0)))), "loopback no longer suffices once a token is configured");
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::AUTHORIZATION, "Bearer s3cret".parse().unwrap());
        assert!(is_admin(&configured, &headers, Some(SocketAddr::from(([10, 0, 0, 5], 0)))), "the correct bearer token is admin regardless of peer address");
    }

    // 🔬️ `space.deleted` closes the space's document WS to a later Hello — `get_space` returns
    // `None` post-deletion, so `resolve_auth` falls straight past even its public-visibility
    // fallback into `AuthOutcome::Denied` (the space was deliberately created PUBLIC, so this proves
    // it is the deletion, not privacy, doing the denying).
    #[tokio::test]
    async fn deleted_space_denies_ws_hello() {
        let state = test_state().await;
        let space_id = create_space_for_test(&state, "seed", "Doomed Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#test".to_string() };
        state.directory_service.execute(actor, DirectoryCommand::DeleteSpace { space_id: space_id.clone() }).await.expect("delete space");
        // Close the pre-existing tokenless-open-by-default fallback (see `share_token_gates_ws_access`)
        // so the Hello below is forced through the space-visibility path this test targets, rather
        // than being trivially admitted before that path is ever consulted.
        state.directory.create_share_token("gone").await.expect("close document with a share token");

        let addr = spawn_server(state).await;
        let url = format!("ws://{addr}/spaces/{space_id}/documents/gone/ws");
        let (mut ws, _) = connect_async(&url).await.unwrap();
        ws.send(client_binary(&hello("late-comer"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut ws).await, ServerFrame::Error { code, .. } if code == "unauthorized"));
    }

    // 🔬️ `GET`/`DELETE /auth/sessions/me`: a live session resolves the caller's identity; revoking
    // it makes the SAME token unauthorized on a subsequent call.
    #[tokio::test]
    async fn auth_sessions_me_roundtrip() {
        let state = test_state().await;
        let session = create_auth_session(State(state.clone()), Json(CreateAuthSessionRequest { email: "me@example.com".into() })).await.expect("mint session");
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", session.0.token).parse().unwrap());

        let me = get_session_me(headers.clone(), State(state.clone())).await.expect("session me");
        assert_eq!(me.0.user_id, session.0.user_id);
        assert_eq!(me.0.email, "me@example.com");

        assert_eq!(delete_session_me(headers.clone(), State(state.clone())).await, StatusCode::NO_CONTENT);
        assert_eq!(get_session_me(headers, State(state)).await.err(), Some(StatusCode::UNAUTHORIZED));
    }
}
//#endregion 🔖️Tests
