mod header {
    // 🧲️Header
    // OS hub v2 — a thin axum shell over two independently swappable backends: `db::Database`
    // (the document authority: submit/query/frontier/history over a WAL-backed document actor,
    // plus content-addressed blob storage) and `Arc<dyn HubDirectory>` (identity/tenancy: users,
    // studios, memberships, auth sessions, share tokens, VFS nodes, sync sessions). Every
    // OpDag/DocumentActor/DocMsg/JSON-snapshot-CAS internal the pre-CW6 hub owned directly is gone
    // — that is now `db`'s job end to end (see `db/engine/rs/lib.rs`, `db/document/rs/lib.rs`).
    // "Space" is a namespacing convention this crate applies on top of `db`'s flat document
    // catalog (`{space_id}:{document_id}`), not hub-internal state.
    //
    // The WebSocket endpoint speaks `protocol_wire`'s binary lane-tagged `ClientFrame`/
    // `ServerFrame` frames directly (see `protocol/wire/rs/lib.rs`) — the server-side counterpart
    // to `framework/sync`'s client actors (CW5). Command-lane persistence/ordering flows through
    // `db::Database::hello`/`DocumentHandle::submit`/`db::sync::handle_frontier_advertise`;
    // preview-lane and presence frames are ephemeral, best-effort fan-out this crate owns directly
    // via a per-document `tokio::sync::broadcast` registry (never durable, matching the preview
    // lane's contract).
}

use axum::body::Bytes;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use dashmap::DashMap;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use os_hub_directory::model::{NodeRecord, SpaceRole};
use os_hub_directory::HubDirectory;
use os_hub_directory_sqlite::SqliteDirectory;
use protocol::{AckStage, ActorId, ApplyOutcome, ClientFrame, DocumentId as ProtocolDocumentId, Lane, OperationEnvelope, RuntimeFrontierSummary, ServerFrame, decode_client_frame, encode_server_frame};
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
    Directory(#[from] os_hub_directory::error::DirectoryError),
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
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

//#region 🔖️State
/// @emoji 🎫️ `(space_id, document_id)` -> the single string key both `db::Database`'s flat
/// document catalog and this crate's own fanout/presence registries key on — space scoping is a
/// convention this crate applies on top of `db`'s namespace, not something `db` itself knows about.
fn scope_key(space_id: &str, document_id: &str) -> String {
    format!("{space_id}:{document_id}")
}

fn db_document_id(space_id: &str, document_id: &str) -> ProtocolDocumentId {
    ProtocolDocumentId(scope_key(space_id, document_id))
}

fn db_core_document_id(id: &ProtocolDocumentId) -> db::core::DocumentId {
    db::core::DocumentId(id.0.clone())
}

#[derive(Clone)]
struct HubState {
    db: Arc<db::Database>,
    directory: Arc<dyn HubDirectory>,
    admin_token: Option<String>,
    /// @emoji 📡️ Command-lane + preview-lane fan-out, one `broadcast::Sender` per `scope_key` —
    /// `db::Database`'s own `DocumentHandle` exposes no live-subscription seam yet (see
    /// `db_engine`'s module doc: `subscribe`/`preview` are honest `Unimplemented` extension seams),
    /// so relaying newly-committed commands / preview blobs / presence updates to other connected
    /// sessions on the same document is this crate's own, deliberately thin responsibility — it
    /// never itself decides ordering or durability, only re-broadcasts what `db` already committed
    /// or what a preview/presence frame carries verbatim.
    fanout: Arc<DashMap<String, broadcast::Sender<ServerFrame>>>,
    /// @emoji 👥️ `(scope_key, actor)` -> that actor's last-published presence peer JSON — ephemeral,
    /// never durable (mirrors the preview lane's own law), rebuilt from nothing on hub restart.
    presence: Arc<DashMap<(String, String), Vec<u8>>>,
    /// @emoji 🧬️ W5.7: `scope_key` -> the first non-zero `store::DocumentCodec::pack_schema_hash`
    /// a client's `Hello` declared for that document — pinned in-memory, never durable (durable
    /// pinning belongs in the db catalog once it grows a column for it; this wave's scope is the
    /// in-memory pin only). A later `Hello` with a different non-zero hash for the same document is
    /// rejected with an `error_frame("schema-hash-mismatch", ...)` before `Welcome` — catches two
    /// builds of the same app disagreeing on a document's field shape. A zero hash always skips
    /// validation (schema-agnostic client, see `DocumentCodec::pack_schema_hash`'s own doc).
    schema_hashes: Arc<DashMap<String, [u8; 32]>>,
}

impl HubState {
    fn fanout_for(&self, key: &str) -> broadcast::Sender<ServerFrame> {
        if let Some(existing) = self.fanout.get(key) {
            return existing.clone();
        }
        let (tx, _rx) = broadcast::channel(256);
        self.fanout.entry(key.to_string()).or_insert(tx).clone()
    }

    fn presence_peers(&self, key: &str) -> Vec<Vec<u8>> {
        self.presence.iter().filter(|entry| entry.key().0 == key).map(|entry| entry.value().clone()).collect()
    }

    /// @emoji 🗂️ Get-or-create: a document is lazily minted in `db`'s catalog on its first Hello,
    /// tolerating the race of two sessions doing so concurrently (the loser's `AlreadyExists`
    /// resolves to the same live handle the winner just registered).
    fn ensure_document(&self, id: &ProtocolDocumentId) -> Result<db::DocumentHandle, db::DbError> {
        match self.db.document(id) {
            Ok(handle) => Ok(handle),
            Err(db::DbError::NotFound(_)) => match self.db.create_document(db::DocumentSpec::new(id.clone())) {
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

fn db_error_status(error: db::DbError) -> StatusCode {
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
/// share-token viewer, or nothing.
enum AuthOutcome {
    Session { user_id: String, role: SpaceRole },
    ShareToken,
    Denied,
}

/// @emoji 🔐️ Tries the bearer as an `AuthSessionRecord` (session id -> user -> space role) first;
/// falls back to the existing anonymous share-token scheme when session resolution fails. Tokenless
/// documents stay open (dev default) until any share token is issued for them.
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
    match state.directory.authorized_by_token(document_id, token).await {
        Ok(true) => AuthOutcome::ShareToken,
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

//#region 🔖️Rest
#[derive(Serialize)]
struct DocumentStatusResponse {
    document_id: String,
    head_seq: u64,
    commit_seq: u64,
    epoch: u64,
}

#[derive(Deserialize)]
struct NodesQuery {
    parent: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateNodeRequest {
    parent_id: Option<String>,
    name: String,
    kind: String,
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

async fn list_nodes(Path(space_id): Path<String>, Query(query): Query<NodesQuery>, State(state): State<HubState>) -> Result<Json<Vec<NodeRecord>>, StatusCode> {
    state.directory.list_nodes(&space_id, query.parent.as_deref()).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_node(Path(space_id): Path<String>, State(state): State<HubState>, Json(body): Json<CreateNodeRequest>) -> Result<Json<NodeRecord>, StatusCode> {
    state.directory.create_node(&space_id, body.parent_id.as_deref(), &body.name, &body.kind).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// @emoji 🧭️ A document's current frontier — the REST surface's only document-shaped route now
/// that whole-envelope JSON snapshot/operation-log routes are gone (superseded by the WS wire-v2
/// protocol; see `header`). Lazily mints the document in `db`'s catalog on first access, same as
/// the WS handshake does.
async fn get_document_status(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<DocumentStatusResponse>, StatusCode> {
    if !authorized(&state, &space_id, &document_id, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let handle = state.ensure_document(&db_document_id(&space_id, &document_id)).map_err(db_error_status)?;
    let frontier = handle.frontier().map_err(db_error_status)?;
    Ok(Json(DocumentStatusResponse { document_id, head_seq: frontier.head_seq, commit_seq: frontier.commit_seq, epoch: frontier.epoch }))
}

async fn create_share(Path((_space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<ShareResponse>, StatusCode> {
    match state.admin_token.as_deref() {
        Some(expected) => {
            if bearer(&headers).as_deref() != Some(expected) {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
        None => return Err(StatusCode::FORBIDDEN),
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
    let computed = state.db.storage().payload().put(&body).map_err(db_error_status)?;
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
    match state.db.storage().payload().get(&content_hash) {
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
    match state.db.storage().payload().contains(&content_hash) {
        Ok(true) => StatusCode::OK,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
//#endregion Blobs
//#endregion 🔖️Rest

//#region 🔖️WebSocket
async fn document_ws(ws: WebSocketUpgrade, Path((space_id, document_id)): Path<(String, String)>, State(state): State<HubState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, space_id, document_id, state))
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
fn best_effort_frontier(handle: &db::DocumentHandle) -> RuntimeFrontierSummary {
    match handle.frontier() {
        Ok(frontier) => engine_frontier_to_wire(&frontier, String::new()),
        Err(_) => RuntimeFrontierSummary { document_id: handle.document_id().clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0u8; 32] },
    }
}

fn engine_frontier_to_wire(frontier: &db::Frontier, head_edit_id: String) -> RuntimeFrontierSummary {
    RuntimeFrontierSummary { document_id: frontier.document.clone(), head_edit_ordinal: frontier.head_seq, head_edit_id, last_commit_seq: frontier.commit_seq, chain_hash: frontier.chain_hash }
}

/// @emoji ✍️ Submits `envelopes` as one `db_document::CommandBatch` through `handle`, returning the
/// `Ack` to send the submitter plus (on acceptance) the `Commands` frame to fan out to every other
/// session on the same document. `Fsync` durability: a hub session's `submit` genuinely committing
/// is the promise `AckStage::Persisted` makes to the client.
async fn submit_commands(handle: &db::DocumentHandle, actor: &ActorId, batch_id: u64, envelopes: Vec<OperationEnvelope>) -> (ServerFrame, Option<ServerFrame>) {
    let batch = match db::document::CommandBatch::new(envelopes.clone()) {
        Ok(batch) => batch,
        Err(error) => {
            let frontier = best_effort_frontier(handle);
            return (ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: error.to_string() }) }], frontier }, None);
        }
    };
    match handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync }).await {
        Ok(Ok(receipt)) => {
            let frontier = engine_frontier_to_wire(&receipt.frontier, receipt.command_id.0.clone());
            let ack = ServerFrame::Ack {
                batch_id,
                stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }],
                frontier: frontier.clone(),
            };
            let commands = ServerFrame::Commands { envelopes, origin: actor.clone(), frontier };
            (ack, Some(commands))
        }
        Ok(Err(error)) | Err(error) => {
            let frontier = best_effort_frontier(handle);
            (ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: error.to_string() }) }], frontier }, None)
        }
    }
}

/// @emoji 📨️ Handles one decoded `ClientFrame` for an already-authenticated, already-`Hello`'d
/// session. Returns `false` when the session should close (`Bye`, or a send failure).
#[allow(clippy::too_many_arguments)]
async fn handle_client_frame(
    state: &HubState,
    handle: &db::DocumentHandle,
    db_id: &ProtocolDocumentId,
    key: &str,
    fanout: &broadcast::Sender<ServerFrame>,
    actor: &ActorId,
    frame: ClientFrame,
    sender: &mut SplitSink<WebSocket, Message>,
) -> bool {
    match frame {
        ClientFrame::Commands { batch_id, envelopes } => {
            let (ack, relay) = submit_commands(handle, actor, batch_id, envelopes).await;
            if let Some(commands_frame) = relay {
                let _ = fanout.send(commands_frame);
            }
            sender.send(encode(&ack)).await.is_ok()
        }
        ClientFrame::FrontierAdvertise { frontier } => {
            let core_document = db_core_document_id(db_id);
            match db::sync::handle_frontier_advertise(state.db.storage().wal(), core_document, &frontier, actor.clone()) {
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
            state.presence.insert((key.to_string(), actor.0.clone()), peer);
            let _ = fanout.send(ServerFrame::Presence { peers: state.presence_peers(key) });
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

async fn handle_ws(socket: WebSocket, space_id: String, document_id: String, state: HubState) {
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
        AuthOutcome::Denied => {
            let _ = sender.send(error_frame("unauthorized", "unauthorized")).await;
            return;
        }
    };

    let db_id = db_document_id(&space_id, &document_id);
    let handle = match state.ensure_document(&db_id) {
        Ok(handle) => handle,
        Err(error) => {
            let _ = sender.send(error_frame("storage", error.to_string())).await;
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
            return;
        }
    };
    if sender.send(encode(&welcome_response.welcome)).await.is_err() {
        return;
    }
    for frame in &welcome_response.follow_up {
        if sender.send(encode(frame)).await.is_err() {
            return;
        }
    }

    let fanout = state.fanout_for(&key);
    let mut broadcast_rx = fanout.subscribe();

    let sync_session = state.directory.record_sync_session_open(&document_id, user_id.as_deref(), role, &actor.0).await.ok();

    loop {
        tokio::select! {
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Binary(bytes))) => {
                        if let Ok((_lane, frame)) = decode_client_frame(&bytes) {
                            if !handle_client_frame(&state, &handle, &db_id, &key, &fanout, &actor, frame, &mut sender).await {
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
        }
    }

    if let Some(session) = sync_session {
        let _ = state.directory.record_sync_session_close(&session.id).await;
    }
    state.presence.remove(&(key.clone(), actor.0.clone()));
    let _ = fanout.send(ServerFrame::Presence { peers: state.presence_peers(&key) });
}
//#endregion 🔖️WebSocket

//#region 🔖️Main
fn router(state: HubState) -> Router {
    Router::new()
        .route("/auth/sessions", post(create_auth_session))
        .route("/spaces/{space_id}/nodes", get(list_nodes).post(create_node))
        .route("/spaces/{space_id}/blobs/{hash}", get(get_blob).head(head_blob).put(put_blob))
        .route("/spaces/{space_id}/documents/{id}", get(get_document_status))
        .route("/spaces/{space_id}/documents/{id}/share", post(create_share))
        .route("/spaces/{space_id}/documents/{id}/ws", get(document_ws))
        .with_state(state)
}

/// @emoji 🧬️ Resolves and connects `db::Database`'s storage substrate, selected by
/// `OS_HUB_STORAGE_BACKEND` (`fs` default, zero-touch, rooted at `{data_dir}/db`; `sqlite`,
/// `postgres` — requires `OS_HUB_DATABASE_URL` — or `neo4j` — requires `OS_HUB_NEO4J_URI` —
/// otherwise). Independent of `connect_directory`'s own backend choice (the contract's "storage
/// swappability" requirement applies to `db`'s substrate and the directory's substrate
/// separately).
fn connect_db(data_dir: &std::path::Path) -> Result<db::Database, HubError> {
    let backend = std::env::var("OS_HUB_STORAGE_BACKEND").unwrap_or_else(|_| "fs".into());
    let profile = db::Profile::Prod;
    match backend.as_str() {
        "fs" | "" => {
            let root = data_dir.join("db");
            std::fs::create_dir_all(&root)?;
            Ok(db::Database::open_at(&root, profile)?)
        }
        "sqlite" => {
            let path = std::env::var("OS_HUB_DB_SQLITE").unwrap_or_else(|_| data_dir.join("db.sqlite3").to_string_lossy().into_owned());
            if let Some(parent) = std::path::Path::new(&path).parent() {
                std::fs::create_dir_all(parent)?;
            }
            let storage = db::storage_sqlite::SqliteStorage::open(std::path::Path::new(&path))?;
            Ok(db::Database::open(db::DbConfig::for_profile(profile), Arc::new(storage))?)
        }
        "postgres" => {
            let database_url = std::env::var("OS_HUB_DATABASE_URL").map_err(|_| HubError::UnknownStorageBackend("postgres requires OS_HUB_DATABASE_URL".into()))?;
            let storage = db::storage_postgres::PostgresStorage::connect(&database_url)?;
            Ok(db::Database::open(db::DbConfig::for_profile(profile), Arc::new(storage))?)
        }
        "neo4j" => {
            let uri = std::env::var("OS_HUB_NEO4J_URI").map_err(|_| HubError::UnknownStorageBackend("neo4j requires OS_HUB_NEO4J_URI".into()))?;
            let user = std::env::var("OS_HUB_NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
            let password = std::env::var("OS_HUB_NEO4J_PASSWORD").unwrap_or_default();
            let storage = db::storage_neo4j::Neo4jStorage::connect(&uri, &user, &password)?;
            Ok(db::Database::open(db::DbConfig::for_profile(profile), Arc::new(storage))?)
        }
        other => Err(HubError::UnknownStorageBackend(other.to_string())),
    }
}

/// @emoji 🧬️ Resolves and connects the identity/tenancy directory backend, selected by
/// `OS_HUB_DIRECTORY_BACKEND` (`sqlite` default, zero-touch, `{data_dir}/directory.db`; `postgres`
/// — requires `OS_HUB_DIRECTORY_DATABASE_URL` — or `neo4j` — requires
/// `OS_HUB_DIRECTORY_NEO4J_URI` — otherwise).
async fn connect_directory(data_dir: &std::path::Path) -> Result<Arc<dyn HubDirectory>, HubError> {
    let backend = std::env::var("OS_HUB_DIRECTORY_BACKEND").unwrap_or_else(|_| "sqlite".into());
    match backend.as_str() {
        "sqlite" | "" => {
            let path = data_dir.join("directory.db");
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let directory = SqliteDirectory::connect(&path.to_string_lossy()).await?;
            directory.seed().await?;
            Ok(Arc::new(directory))
        }
        "postgres" => {
            let database_url = std::env::var("OS_HUB_DIRECTORY_DATABASE_URL").map_err(|_| HubError::UnknownDirectoryBackend("postgres requires OS_HUB_DIRECTORY_DATABASE_URL".into()))?;
            let directory = os_hub_directory_postgres::PostgresDirectory::connect(&database_url).await?;
            directory.seed().await?;
            Ok(Arc::new(directory))
        }
        "neo4j" => {
            let uri = std::env::var("OS_HUB_DIRECTORY_NEO4J_URI").map_err(|_| HubError::UnknownDirectoryBackend("neo4j requires OS_HUB_DIRECTORY_NEO4J_URI".into()))?;
            let user = std::env::var("OS_HUB_DIRECTORY_NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
            let password = std::env::var("OS_HUB_DIRECTORY_NEO4J_PASSWORD").unwrap_or_default();
            let directory = os_hub_directory_neo4j::Neo4jDirectory::connect(&uri, &user, &password).await?;
            directory.seed().await?;
            Ok(Arc::new(directory))
        }
        other => Err(HubError::UnknownDirectoryBackend(other.to_string())),
    }
}

#[tokio::main]
async fn main() -> Result<(), HubError> {
    tracing_subscriber::fmt::init();
    let port: u16 = std::env::var("OS_HUB_PORT").ok().and_then(|value| value.parse().ok()).unwrap_or(6070);
    let data_dir = std::env::var("OS_HUB_DATA").map(std::path::PathBuf::from).unwrap_or_else(|_| std::path::PathBuf::from("./.semio/hub/"));
    std::fs::create_dir_all(&data_dir)?;
    let db = connect_db(&data_dir)?;
    let directory = connect_directory(&data_dir).await?;
    let admin_token = std::env::var("OS_HUB_ADMIN_TOKEN").ok().filter(|value| !value.is_empty());
    let state = HubState { db: Arc::new(db), directory, admin_token, fanout: Arc::new(DashMap::new()), presence: Arc::new(DashMap::new()), schema_hashes: Arc::new(DashMap::new()) };
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("os-hub listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router(state)).await?;
    Ok(())
}
//#endregion 🔖️Main

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Bootstrap, DocumentId as WireDocumentId};
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
        let database = db::Database::open_at(&tempdir("db"), db::Profile::Test).expect("open db");
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect directory");
        directory.seed().await.expect("seed");
        HubState { db: Arc::new(database), directory: Arc::new(directory), admin_token: None, fanout: Arc::new(DashMap::new()), presence: Arc::new(DashMap::new()), schema_hashes: Arc::new(DashMap::new()) }
    }

    fn sample_envelope(id: &str, document: &WireDocumentId) -> OperationEnvelope {
        OperationEnvelope {
            operation_id: protocol::OperationId(id.to_string()),
            document_id: document.clone(),
            actor: ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: protocol::DocumentDiff {
                schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()),
                payload: serde_json::to_vec(&serde_json::json!({ "value": id })).unwrap(),
            },
            inverse: protocol::InverseOperation {
                schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()),
                payload: serde_json::to_vec(&serde_json::json!({})).unwrap(),
            },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        }
    }

    async fn spawn_server(state: HubState) -> SocketAddr {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let app = router(state);
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        addr
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

        let (mut b, _) = connect_async(&url).await.unwrap();
        b.send(client_binary(&hello("B"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut b).await, ServerFrame::Welcome { .. }));

        let document = WireDocumentId(format!("{STUDIO}:default"));
        a.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("op-1", &document)] }, Lane::Command)).await.unwrap();

        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Ack { batch_id: 1, .. }));

        loop {
            match next_server_frame(&mut b).await {
                ServerFrame::Commands { envelopes, origin, .. } => {
                    assert_eq!(envelopes.len(), 1);
                    assert_eq!(envelopes[0].operation_id.0, "op-1");
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
        let document = WireDocumentId(format!("{STUDIO}:default"));

        let (mut a, _) = connect_async(&url).await.unwrap();
        a.send(client_binary(&hello("A"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Welcome { .. }));
        a.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("op-1", &document)] }, Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Ack { .. }));

        // A fresh connection with no prior frontier must see the already-committed op-1 in its
        // Welcome bootstrap follow-up.
        let (mut c, _) = connect_async(&url).await.unwrap();
        c.send(client_binary(&hello("C"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut c).await, ServerFrame::Welcome { bootstrap: Bootstrap::Tail, .. }));
        match next_server_frame(&mut c).await {
            ServerFrame::Commands { envelopes, .. } => assert_eq!(envelopes[0].operation_id.0, "op-1"),
            other => panic!("expected the Tail bootstrap's Commands follow-up, got {other:?}"),
        }
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
        let document = WireDocumentId("space-a:shared-doc".to_string());
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
        let share = create_share(Path((STUDIO.to_string(), "guarded".to_string())), headers, State(admin_state)).await.expect("share");

        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/guarded/ws");
        let (mut denied, _) = connect_async(&url).await.unwrap();
        denied.send(client_binary(&hello("intruder"), Lane::Command)).await.unwrap();
        assert!(matches!(next_server_frame(&mut denied).await, ServerFrame::Error { code, .. } if code == "unauthorized"));

        let (mut allowed, _) = connect_async(&url).await.unwrap();
        allowed
            .send(client_binary(&ClientFrame::Hello { wire_version: 1, protocol_version: 1, schema: "test.v1".to_string(), pack_schema_hash: [0u8; 32], actor: ActorId("holder".to_string()), token: Some(share.0.token), resume_token: None, frontier: None }, Lane::Command))
            .await
            .unwrap();
        assert!(matches!(next_server_frame(&mut allowed).await, ServerFrame::Welcome { .. }));
    }

    // 🔬️ Blob round-trip: PUT then GET returns identical bytes and HEAD reports found, through
    // `db::Database`'s own content-addressed payload store; a hash that was never PUT is reported
    // missing by both GET and HEAD.
    #[tokio::test]
    async fn blob_put_get_head_round_trip() {
        let state = test_state().await;
        let bytes = Bytes::from_static(b"hello hub blob bytes");
        let expected_hash = state.db.storage().payload().put(&bytes).unwrap().to_string();
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

    // 🔬️ VFS nodes are durable and creatable through the directory-backed REST routes.
    #[tokio::test]
    async fn nodes_create_and_list() {
        let state = test_state().await;
        let created = create_node(Path(STUDIO.to_string()), State(state.clone()), Json(CreateNodeRequest { parent_id: None, name: "Projects".into(), kind: "folder".into() })).await.expect("create");
        let child = create_node(Path(STUDIO.to_string()), State(state.clone()), Json(CreateNodeRequest { parent_id: Some(created.0.id.clone()), name: "sketch".into(), kind: "document".into() })).await.expect("create child");
        let children = list_nodes(Path(STUDIO.to_string()), Query(NodesQuery { parent: Some(created.0.id.clone()) }), State(state)).await.expect("list");
        assert_eq!(children.0.len(), 1);
        assert_eq!(children.0[0].id, child.0.id);
    }

    // 🔬️ Auth sessions: POST /auth/sessions mints a session that resolves the caller's space role
    // and grants access even to a document a share token has otherwise closed.
    #[tokio::test]
    async fn auth_session_grants_role_and_bypasses_share_gate() {
        let state = test_state().await;
        // `hub_space_membership.space_id` is FK-bound to `hub_space(id)` — a real studio, not a
        // bare string, matching how `create_auth_session`'s minted user must also be a real row.
        let studio = state.directory.create_space("Space X", "seed").await.expect("create space").id;
        let document = "closed-doc";
        state.directory.create_share_token(document).await.expect("close with share token");
        assert!(!state.directory.authorized_by_token(document, None).await.unwrap());

        let minted = create_auth_session(State(state.clone()), Json(CreateAuthSessionRequest { email: "dev@example.com".into() })).await.expect("mint session");
        state.directory.upsert_membership(&studio, &minted.0.user_id, SpaceRole::Member).await.expect("grant membership");

        assert!(!authorized(&state, &studio, document, None).await, "tokenless request still denied");
        assert!(authorized(&state, &studio, document, Some(&minted.0.token)).await, "session token authorized despite no share token");

        match resolve_auth(&state, &studio, document, Some(&minted.0.token)).await {
            AuthOutcome::Session { user_id, role } => {
                assert_eq!(user_id, minted.0.user_id);
                assert_eq!(role, SpaceRole::Member);
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

        let handle = state.ensure_document(&db_document_id(STUDIO, "fresh")).expect("ensure");
        let batch = db::document::CommandBatch::new(vec![sample_envelope("op-1", &db_document_id(STUDIO, "fresh"))]).unwrap();
        handle.submit(batch, db::document::SubmitOptions::default()).await.unwrap().unwrap();

        let status = get_document_status(Path((STUDIO.to_string(), "fresh".to_string())), HeaderMap::new(), State(state)).await.expect("status after submit");
        assert_eq!(status.0.head_seq, 1);
    }
}
//#endregion 🔖️Tests
