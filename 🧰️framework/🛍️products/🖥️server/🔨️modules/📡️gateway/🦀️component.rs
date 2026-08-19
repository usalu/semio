//! 📡️ Gateway: layer 3 — the transport, and nothing else.
//!
//! Layer 1 is the [`contract`](crate::contract) (pure data two parties agree on), layer 2 is the
//! [`authority`](crate::authority) turn protocol plus [`policy`](crate::policy) and
//! [`storage`](crate::storage). This file owns layer 3: HTTP verbs, websockets, CORS, static assets
//! and the process that binds a socket. Nothing here decides anything — it authenticates, it
//! authorizes by asking the engine, it serializes, and it hands the result to the layer below.
//!
//! **The two lanes never merge.** The durable lane carries [`EventRecord`](crate::contract::
//! EventRecord)s: sequenced, replayable, replayed from [`AuthorityStore::events_since`] on connect
//! and deduplicated by `seq` at the replay/live seam, so a reconnecting client sees every fact
//! exactly once. The ephemeral lane carries [`EphemeralFrame`](crate::contract::EphemeralFrame)s and
//! document frames: lossy, never persisted, never replayed, dropped the moment a socket closes.
//! They travel on different [`Fanout`] lanes ([`stream_lane`], [`ephemeral_lane`],
//! [`document_lane`]) precisely so no future refactor can quietly start replaying a cursor position
//! or dropping a committed event.
//!
//! **The document engine is a port, not a dependency.** [`DocumentAuthority`] is the whole of what
//! this product knows about replication engines. The server product deliberately does not depend on
//! the os product, on `db`, or on any concrete engine; an instance supplies the implementation.

use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::path::{Path as FsPath, PathBuf};
use std::sync::{Arc, Mutex as StdMutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Bytes;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{ConnectInfo, Path, Query, State};
use axum::http::{header, HeaderMap, HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use dashmap::DashMap;
use futures::{Sink, SinkExt, StreamExt};
use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, Mutex, Notify};

use crate::authority::{AuthorityDirectory, AuthorityError, CommandBus, Deciders, PolicyHook};
use crate::contract::{
    ActorKey, CommandEnvelope, CommandOutcome, EphemeralFrame, EventRecord, HybridLogicalClock,
    ModuleManifest, PolicyDecision, PolicyGrant, PolicyPoint, PolicyTemplate, Principal,
    QueryEnvelope, QueryResult, Scope, ServerInstanceDefinition, TenantId,
};
use crate::policy::{AdminGate, Credential, PolicyEngine, PolicyRequest, PrincipalResolvers, ResolverChain};
use crate::storage::{
    content_hash, AuthorityStore, AuthorityStores, BlobStore, BlobStores, MemoryAuthorityStore,
    MemoryBlobStore, MemoryProjectionStore, MemorySessionStore, ProjectionStores, SessionStores,
    StorageError, StorageProfile,
};

//#region 🔖️Reexport
/// 🚪️ The router type a [`ServerModule`] contributes routes to. Reexported so an instance never has
/// to name the transport library itself.
pub use axum::Router as GatewayRouter;

/// 📦️ The JSON body wrapper handlers use, reexported for the same reason as [`GatewayRouter`].
pub use axum::Json as GatewayJson;
//#endregion 🔖️Reexport

//#region 🔖️Error
/// 💥️ The one error every transport surface answers with. An instance maps its own domain errors
/// into these six shapes; the status code is derived here so no handler ever picks one by hand.
#[derive(Clone, Debug, thiserror::Error)]
pub enum ServerError {
    /// 🔒️ The caller could not be authenticated — 401.
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    /// ⛔️ The caller is known and policy still says no — 403.
    #[error("forbidden: {0}")]
    Forbidden(String),
    /// 🕳️ Nothing is addressed by this path — 404.
    #[error("not found: {0}")]
    NotFound(String),
    /// ⚔️ The write contradicts what is already stored — 409.
    #[error("conflict: {0}")]
    Conflict(String),
    /// 🚧️ The request itself is malformed — 400.
    #[error("bad request: {0}")]
    BadRequest(String),
    /// 🔥️ The server failed to answer at all — 500.
    #[error("internal error: {0}")]
    Internal(String),
}

impl ServerError {
    /// 🔢️ The HTTP status this error is answered with.
    pub fn status(&self) -> StatusCode {
        match self {
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// 🏷️ The stable machine-readable tag a client branches on instead of the status code.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Unauthorized(_) => "unauthorized",
            Self::Forbidden(_) => "forbidden",
            Self::NotFound(_) => "notFound",
            Self::Conflict(_) => "conflict",
            Self::BadRequest(_) => "badRequest",
            Self::Internal(_) => "internal",
        }
    }
}

/// 🧾️ The JSON body every [`ServerError`] renders as, so a client never has to parse a status line.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBody {
    /// 🏷️ The [`ServerError::kind`] tag.
    pub kind: String,
    /// 💬️ The human-facing detail.
    pub message: String,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let body = ErrorBody { kind: self.kind().to_string(), message: self.to_string() };
        (self.status(), Json(body)).into_response()
    }
}

impl From<StorageError> for ServerError {
    fn from(error: StorageError) -> Self {
        match error {
            StorageError::NotFound => Self::NotFound("storage entry not found".to_string()),
            StorageError::Conflict(detail) => Self::Conflict(detail),
            StorageError::SequenceGap { .. } | StorageError::LeaseLost => Self::Conflict(error.to_string()),
            StorageError::Backend(detail) => Self::Internal(detail),
        }
    }
}

impl From<AuthorityError> for ServerError {
    fn from(error: AuthorityError) -> Self {
        match error {
            AuthorityError::LeaseLost => Self::Conflict(error.to_string()),
            AuthorityError::UnknownActorKind(kind) => Self::NotFound(format!("unknown actor kind: {kind}")),
            AuthorityError::Storage(detail) => Self::Internal(detail),
        }
    }
}
//#endregion 🔖️Error

//#region 🔖️Module
/// 🧩️ The runtime half of a server module.
///
/// The declarative half is [`ModuleManifest`], which is pure data and lives in the contract so an
/// instance definition can be inspected, diffed and served without ever constructing a server. This
/// trait is the half that cannot be data: [`routes`](Self::routes) hands out a live
/// [`Router<ServerState>`] and therefore names the transport library, which is exactly why it is
/// declared here in layer 3 and not next to the manifest in layer 1. Keeping the two halves apart
/// is what lets a client, a CLI or a documentation generator read a manifest without linking axum.
#[dyn_enum]
pub trait ServerModule: Send + Sync {
    /// 📇️ What this module declares to the instance registering it.
    async fn manifest(&self) -> ModuleManifest;

    /// ⚖️ The deciders this module registers on the command bus, one per actor kind it serves.
    async fn deciders(&self) -> Vec<Deciders> {
        Vec::new()
    }

    /// 🛣️ The routes this module mounts. Called once at build time with the router under
    /// construction; a module that serves no HTTP surface returns it untouched.
    async fn routes(&self, router: Router<ServerState>) -> Router<ServerState> {
        router
    }

    /// 🪜️ The authentication rungs this module contributes, appended to the shared ladder in
    /// module registration order.
    async fn resolvers(&self) -> Vec<PrincipalResolvers> {
        Vec::new()
    }

    /// 🎓️ The role definitions this module registers into the shared policy engine.
    async fn templates(&self) -> Vec<PolicyTemplate> {
        Vec::new()
    }
}

/// 🧮️ The framework's reference [`ServerModule`]: contributes one policy template and one health
/// route, nothing else. Kept in production scope (not only in tests) so [`ServerModules`] closes
/// over a real variant and this crate's own `build_collects_every_module_contribution` test
/// exercises the genuine enum-dispatch path; a product's own modules are added as further
/// `ServerModules` variants alongside it.
pub struct CountingModule;

impl ServerModule for CountingModule {
    async fn manifest(&self) -> ModuleManifest {
        ModuleManifest {
            id: "counting".into(),
            policies: vec![PolicyTemplate {
                name: "author".into(),
                grants: vec![PolicyGrant { point: PolicyPoint::CommandAdmission, resource: "*".into(), action: "*".into() }],
            }],
            ..Default::default()
        }
    }

    async fn routes(&self, router: Router<ServerState>) -> Router<ServerState> {
        router.route("/counting/health", get(|| async { "ok" }))
    }
}

dyn_enum_close! {
    pub enum ServerModules: ServerModule {
        Counting(CountingModule),
    }
}
//#endregion 🔖️Module

//#region 🔖️DocumentPort
/// 📄️ The port a replication engine plugs into, and the only thing this product knows about one.
///
/// The server product deliberately depends on no document engine: not on the os product, not on
/// `db`, not on any concrete CRDT or OT implementation. The gateway can therefore bridge a document
/// websocket — handshake, submit, relay — while naming nothing but opaque byte frames. Hub supplies
/// the implementation; another instance may supply a different one, or none at all, in which case
/// the document route answers [`ServerError::NotFound`].
#[dyn_enum]
pub trait DocumentAuthority: Send + Sync {
    /// 👋️ The handshake frame for a joining actor. `resume` carries whatever resumption token the
    /// engine minted previously; the gateway never interprets it.
    async fn welcome(&self, scope: &Scope, actor: &str, resume: Option<&str>) -> Result<Vec<u8>, ServerError>;

    /// 📨️ Apply one client frame and return the frames to send back to the submitter and relay to
    /// the other sessions on the same document.
    async fn submit_frame(&self, scope: &Scope, principal: &Principal, frame: &[u8]) -> Result<Vec<Vec<u8>>, ServerError>;
}

// 📄️ Not closed over a real reference variant (O1 de-dyn): this crate has zero implementors by
// design — the replication engine is always caller-supplied (Hub or another product this
// framework product deliberately does not depend on, per the module doc above) — so
// `DocumentAuthorities` closes over an empty, uninhabited set. `Option<Arc<DocumentAuthorities>>`
// therefore always observes `None` today, matching every existing test's expectation exactly; the
// day a real engine lands, it is added here as the first variant, never as a `Box<dyn ..>`.
dyn_enum_close! {
    pub enum DocumentAuthorities: DocumentAuthority {}
}
//#endregion 🔖️DocumentPort

//#region 🔖️Query
/// ❓️ One registered read. A query never touches an actor's private state — it answers from a
/// [`ProjectionStore`], which is why the handler is handed nothing else.
#[dyn_enum]
pub trait QueryHandler: Send + Sync {
    /// 🏷️ The [`QueryEnvelope::kind`] this handler answers.
    async fn kind(&self) -> &str;

    /// 📤️ Answer one query against the read models.
    async fn handle(&self, envelope: &QueryEnvelope, projections: &ProjectionStores) -> Result<QueryResult, ServerError>;
}

// ❓️ Not closed over a real reference variant (O1 de-dyn): this crate has zero implementors by
// design — every query kind is defined by the product built on this framework, not by the
// framework itself — so `QueryHandlers` closes over an empty, uninhabited set, matching every
// existing test's expectation that `queries` starts and stays empty. A product's first real
// handler is added here as the first variant, never as a `Box<dyn ..>`.
dyn_enum_close! {
    pub enum QueryHandlers: QueryHandler {}
}
//#endregion 🔖️Query

//#region 🔖️Fanout
/// 📻️ How many frames a lane buffers before a slow subscriber is lagged out of them.
const FANOUT_CAPACITY: usize = 256;

/// 📡️ Per-lane broadcast fan-out. One [`broadcast`] sender per lane key, created on the first
/// subscribe and dropped when the last subscriber goes, so an idle instance holds no lanes at all.
#[derive(Clone, Default)]
pub struct Fanout {
    channels: Arc<DashMap<String, broadcast::Sender<Vec<u8>>>>,
}

impl Fanout {
    /// 🌱️ A registry holding no lanes.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🔔️ Subscribe to `lane`, creating it if nobody holds it yet. The returned [`Subscription`] is
    /// the lane's lifetime: dropping the last one removes the sender.
    pub fn subscribe(&self, lane: &str) -> Subscription {
        let sender = self.channels.entry(lane.to_string()).or_insert_with(|| broadcast::channel(FANOUT_CAPACITY).0).clone();
        Subscription { lane: lane.to_string(), channels: Arc::clone(&self.channels), receiver: sender.subscribe() }
    }

    /// 📢️ Publish `bytes` on `lane` and report how many subscribers received it. Publishing to a
    /// lane nobody holds is a no-op — a fan-out never creates a lane, only a subscriber does.
    pub fn publish(&self, lane: &str, bytes: Vec<u8>) -> usize {
        match self.channels.get(lane) {
            Some(sender) => sender.send(bytes).unwrap_or(0),
            None => 0,
        }
    }

    /// 🔢️ How many lanes currently have at least one subscriber.
    pub fn lanes(&self) -> usize {
        self.channels.len()
    }
}

/// 🎧️ One live subscription. Holding it keeps its lane alive; dropping it releases the lane once no
/// other subscriber remains.
pub struct Subscription {
    lane: String,
    channels: Arc<DashMap<String, broadcast::Sender<Vec<u8>>>>,
    receiver: broadcast::Receiver<Vec<u8>>,
}

impl Subscription {
    /// 📥️ The next frame, or `None` once the lane is closed. A lagged subscriber silently skips the
    /// frames it missed rather than erroring: an ephemeral lane is lossy by construction, and the
    /// durable lane recovers by replaying from its sequence number.
    pub async fn recv(&mut self) -> Option<Vec<u8>> {
        loop {
            match self.receiver.recv().await {
                Ok(bytes) => return Some(bytes),
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    }

    /// 🏷️ The lane this subscription listens on.
    pub fn lane(&self) -> &str {
        &self.lane
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        self.channels.remove_if(&self.lane, |_, sender| sender.receiver_count() <= 1);
    }
}

/// 📚️ The durable lane key of one actor's event stream.
pub fn stream_lane(actor: &ActorKey) -> String {
    format!("stream:{}/{}/{}", actor.tenant.0, actor.kind, actor.id)
}

/// 💨️ The ephemeral lane key of one scope — cursors, selections, typing.
pub fn ephemeral_lane(scope: &Scope) -> String {
    format!("ephemeral:{}", scope.0)
}

/// 📄️ The document lane key of one scope, carrying opaque engine frames between sessions.
pub fn document_lane(scope: &Scope) -> String {
    format!("document:{}", scope.0)
}
//#endregion 🔖️Fanout

//#region 🔖️Presence
/// 🎨️ One actor's held palette slot in a scope, ref-counted across that actor's concurrently open
/// sockets so a second window never steals a third colour.
#[derive(Clone, Debug, PartialEq, Eq)]
struct ColourLease {
    index: u8,
    refs: u32,
}

/// 🌈️ One scope's live palette leases. Never persisted, rebuilt from nothing after a restart.
#[derive(Default)]
struct ScopeColours {
    by_actor: BTreeMap<String, ColourLease>,
}

/// 👤️ One connected actor's presence in a scope. `peer` stays opaque — the gateway stores and
/// forwards it, never decodes it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceSession {
    /// 🙋️ Who is present.
    pub actor: String,
    /// 🪟️ Which surface the actor joined from.
    pub surface: String,
    /// 🎨️ The palette slot leased for the life of the session.
    pub colour: u8,
    /// 📦️ The last opaque presence payload the actor published, if any.
    pub peer: Option<Vec<u8>>,
}

/// 👥️ The ephemeral presence registry: who is in which scope, and which of the 256 palette slots
/// each of them holds.
#[derive(Default)]
pub struct Presence {
    colours: DashMap<String, ScopeColours>,
    sessions: DashMap<(String, String), PresenceSession>,
}

impl Presence {
    /// 🐣️ An empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🎨️ The lowest palette slot not currently held in `scope`, or the actor's existing slot with
    /// one more reference on it. Wraps once all 256 slots are taken.
    pub fn acquire_colour(&self, scope: &str, actor: &str) -> u8 {
        let mut colours = self.colours.entry(scope.to_string()).or_default();
        if let Some(lease) = colours.by_actor.get_mut(actor) {
            lease.refs += 1;
            return lease.index;
        }
        let taken: Vec<u8> = colours.by_actor.values().map(|lease| lease.index).collect();
        let index = (0..=u8::MAX).find(|candidate| !taken.contains(candidate)).unwrap_or((colours.by_actor.len() % 256) as u8);
        colours.by_actor.insert(actor.to_string(), ColourLease { index, refs: 1 });
        index
    }

    /// 🧹️ Drop one reference on the actor's slot, freeing it on the last disconnect.
    pub fn release_colour(&self, scope: &str, actor: &str) {
        let Some(mut colours) = self.colours.get_mut(scope) else { return };
        let exhausted = match colours.by_actor.get_mut(actor) {
            Some(lease) => {
                lease.refs = lease.refs.saturating_sub(1);
                lease.refs == 0
            }
            None => false,
        };
        if exhausted {
            colours.by_actor.remove(actor);
        }
    }

    /// 🔎️ The slot this actor currently holds in `scope`, if any.
    pub fn colour_of(&self, scope: &str, actor: &str) -> Option<u8> {
        self.colours.get(scope).and_then(|colours| colours.by_actor.get(actor).map(|lease| lease.index))
    }

    /// 🚪️ Register a session and lease it a colour.
    pub fn join(&self, scope: &str, actor: &str, surface: &str) -> u8 {
        let colour = self.acquire_colour(scope, actor);
        let session = PresenceSession { actor: actor.to_string(), surface: surface.to_string(), colour, peer: None };
        self.sessions.insert((scope.to_string(), actor.to_string()), session);
        colour
    }

    /// 📤️ Record the opaque payload an actor last published.
    pub fn publish_peer(&self, scope: &str, actor: &str, peer: Vec<u8>) {
        if let Some(mut session) = self.sessions.get_mut(&(scope.to_string(), actor.to_string())) {
            session.peer = Some(peer);
        }
    }

    /// 🚶️ Remove the session and release its colour.
    pub fn leave(&self, scope: &str, actor: &str) {
        self.sessions.remove(&(scope.to_string(), actor.to_string()));
        self.release_colour(scope, actor);
    }

    /// 📋️ Everyone currently present in `scope`, ordered by actor.
    pub fn roster(&self, scope: &str) -> Vec<PresenceSession> {
        let mut roster: Vec<PresenceSession> =
            self.sessions.iter().filter(|entry| entry.key().0 == scope).map(|entry| entry.value().clone()).collect();
        roster.sort_by(|left, right| left.actor.cmp(&right.actor));
        roster
    }
}
//#endregion 🔖️Presence

//#region 🔖️Kick
/// 🦵️ Per-session close signals. The administration plane fires one; the socket loop owning that
/// session observes it and closes itself. Nothing here ever touches a socket.
#[derive(Default)]
pub struct KickMap {
    signals: DashMap<String, Arc<Notify>>,
}

impl KickMap {
    /// 🌱️ An empty map.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🔔️ Signal the session to close. The permit is stored, so a kick that arrives before the loop
    /// starts waiting is still observed.
    pub fn kick(&self, session: &str) {
        self.signal(session).notify_one();
    }

    /// ⏳️ Wait until this session is kicked.
    pub async fn kicked(&self, session: &str) {
        let signal = self.signal(session);
        signal.notified().await;
    }

    /// 🧹️ Forget a session's signal once its socket is gone.
    pub fn forget(&self, session: &str) {
        self.signals.remove(session);
    }

    /// 🔢️ How many sessions currently carry a signal.
    pub fn tracked(&self) -> usize {
        self.signals.len()
    }

    fn signal(&self, session: &str) -> Arc<Notify> {
        self.signals.entry(session.to_string()).or_default().clone()
    }
}
//#endregion 🔖️Kick

//#region 🔖️Cors
/// 🌐️ Reflect the caller's own `Origin` back rather than answering a bare `*`, and short-circuit
/// every `OPTIONS` preflight with 204 before it reaches route dispatch.
///
/// Hand-rolled instead of pulled from a middleware library on purpose: reflecting the origin keeps
/// this compatible with a credentialed scheme, and the whole behaviour is six headers — a
/// dependency to produce them would be a dependency to audit them.
pub async fn cors_middleware(request: axum::extract::Request, next: axum::middleware::Next) -> Response {
    let origin = request.headers().get(header::ORIGIN).cloned();
    if request.method() == Method::OPTIONS {
        let mut response = StatusCode::NO_CONTENT.into_response();
        apply_cors_headers(response.headers_mut(), origin.as_ref());
        return response;
    }
    let mut response = next.run(request).await;
    apply_cors_headers(response.headers_mut(), origin.as_ref());
    response
}

/// 🪞️ Write the CORS grant onto a response: the caller's own origin (never `*`), credentials, and
/// the verbs and headers this control plane actually uses.
pub fn apply_cors_headers(headers: &mut HeaderMap, origin: Option<&HeaderValue>) {
    if let Some(origin) = origin {
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
        headers.insert(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("true"));
        headers.insert(header::VARY, HeaderValue::from_static("origin"));
    }
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET, POST, PUT, HEAD, DELETE, OPTIONS"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("authorization, content-type, x-semio-capability"));
}
//#endregion 🔖️Cors

//#region 🔖️Credential
/// 🛂️ The header a caller presents a [`CapabilityProof`](crate::contract::CapabilityProof) in.
pub const CAPABILITY_HEADER: &str = "x-semio-capability";

/// 🎟️ The bearer token of an `Authorization: Bearer …` header, if there is one.
pub fn bearer(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(|value| value.to_string())
}

/// 🪪️ Normalize what a caller presented into a transport-free [`Credential`]. `loopback` comes from
/// the peer address and never from a header — it is a fact only the transport can establish, and a
/// header claiming it would be a header granting itself the administration plane.
pub fn credential(headers: &HeaderMap, peer: Option<SocketAddr>) -> Credential {
    Credential {
        bearer: bearer(headers),
        capability: headers
            .get(CAPABILITY_HEADER)
            .and_then(|value| value.to_str().ok())
            .map(|value| crate::contract::CapabilityProof(value.to_string())),
        loopback: peer.is_some_and(|peer| peer.ip().is_loopback()),
    }
}
//#endregion 🔖️Credential

//#region 🔖️Store
/// 🏛️ The bus shape this gateway serializes every command through. `AuthorityStores` (O1 de-dyn:
/// `🗄️storage`'s `#[dyn_enum]`-closed enum) replaces the former `DynAuthorityStore` boxed-trait
/// wrapper — a [`StorageProfile`] still picks a backend at runtime, but now as a concrete enum
/// variant known to the compiler at every call site rather than an erased `Box<dyn AuthorityStore>`.
pub type ServerAuthority = CommandBus<AuthorityStores>;
//#endregion 🔖️Store

//#region 🔖️State
/// 🧠️ Everything a handler may reach. Cloneable and cheap: every field is an [`Arc`], so the whole
/// value is a bundle of handles rather than a bundle of data.
#[derive(Clone)]
pub struct ServerState {
    /// 🏛️ The command bus, serialized behind a mutex because a turn is by definition one at a time.
    pub authority: Arc<Mutex<ServerAuthority>>,
    /// 🔭️ The rebuildable read models every query answers from.
    pub projections: Arc<Mutex<ProjectionStores>>,
    /// 🧱️ Content-addressed bytes.
    pub blobs: Arc<Mutex<BlobStores>>,
    /// 🎫️ Live authentication state.
    pub sessions: Arc<Mutex<SessionStores>>,
    /// ⚖️ Roles as data. A standard lock rather than an async one because the command bus's own
    /// admission hook is synchronous and must consult it inside a turn.
    pub policy: Arc<RwLock<PolicyEngine>>,
    /// 🪜️ The authentication ladder, fixed at build time.
    pub resolvers: Arc<ResolverChain>,
    /// 🚪️ The gate in front of the administration plane.
    pub admin: Arc<AdminGate>,
    /// 📡️ Lane fan-out for both the durable and the ephemeral lane.
    pub fanout: Arc<Fanout>,
    /// 👥️ Who is present where, and which colour they hold.
    pub presence: Arc<Presence>,
    /// 🦵️ Per-session close signals.
    pub kicks: Arc<KickMap>,
    /// 🧩️ The static apps this instance hosts.
    pub apps: Arc<AppRegistry>,
    /// ❓️ The registered read handlers, keyed by query kind.
    pub queries: Arc<DashMap<String, Arc<QueryHandlers>>>,
    /// 📄️ The replication engine, when the instance supplied one.
    pub documents: Option<Arc<DocumentAuthorities>>,
    /// 🏗️ Where this instance's durable state lives.
    pub data_dir: Arc<PathBuf>,
    /// 🕰️ The instance's hybrid logical clock, advanced once per stamped command.
    clock: Arc<StdMutex<HybridLogicalClock>>,
}

impl ServerState {
    /// 🕰️ The next clock reading: wall-clock milliseconds, with the counter breaking ties so two
    /// commands stamped inside the same millisecond still order.
    pub fn now(&self) -> HybridLogicalClock {
        let millis = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_millis() as u64);
        let mut clock = self.clock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        *clock = if millis > clock.millis {
            HybridLogicalClock { millis, counter: 0 }
        } else {
            HybridLogicalClock { millis: clock.millis, counter: clock.counter.saturating_add(1) }
        };
        *clock
    }

    /// ⚖️ Evaluate one policy question, turning a denial into [`ServerError::Forbidden`].
    pub fn authorize(&self, request: &PolicyRequest) -> Result<(), ServerError> {
        let engine = self.policy.read().map_err(|_| ServerError::Internal("policy engine poisoned".to_string()))?;
        match engine.evaluate(request) {
            PolicyDecision::Allow => Ok(()),
            PolicyDecision::Deny { reason } => Err(ServerError::Forbidden(reason)),
        }
    }

    /// 🙋️ Who this caller is, according to the ladder.
    pub async fn identify(&self, headers: &HeaderMap, peer: Option<SocketAddr>) -> crate::policy::Resolved {
        self.resolvers.resolve(&credential(headers, peer)).await
    }

    /// 📜️ Every durable event of `actor` after `since`.
    pub async fn replay_events(&self, actor: &ActorKey, since: u64) -> Result<Vec<EventRecord>, ServerError> {
        let authority = self.authority.lock().await;
        Ok(authority.store().events_since(actor, since).await?)
    }
}
//#endregion 🔖️State

//#region 🔖️Blob
/// #️⃣ Decode a 64-hex-character blob address, refusing anything else rather than panicking.
pub fn parse_content_hash(hex: &str) -> Option<protocol::codec::ids::ContentHash> {
    if hex.len() != 64 {
        return None;
    }
    let mut bytes = [0u8; 32];
    let raw = hex.as_bytes();
    for (index, slot) in bytes.iter_mut().enumerate() {
        let pair = std::str::from_utf8(&raw[index * 2..index * 2 + 2]).ok()?;
        *slot = u8::from_str_radix(pair, 16).ok()?;
    }
    Some(protocol::codec::ids::ContentHash(bytes))
}

/// 🧾️ What a successful upload answers with.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobReceipt {
    /// #️⃣ The verified content address.
    pub hash: String,
    /// 📏️ How many bytes are stored under it.
    pub size: usize,
}

/// 💾️ Store bytes at a client-supplied content address. The address is re-derived from the bytes
/// and a mismatch is a [`ServerError::Conflict`]: the caller asked to bind an address to content
/// that does not hash to it, which is the one thing a content-addressed store may never do.
pub async fn put_blob(
    Path(hash): Path<String>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState>,
    body: Bytes,
) -> Result<Json<BlobReceipt>, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest {
        point: PolicyPoint::BlobWrite,
        principal: resolved.principal,
        scope: None,
        resource: hash.clone(),
        action: "write".to_string(),
    })?;
    let addressed = parse_content_hash(&hash).ok_or_else(|| ServerError::BadRequest("blob address is not 64 hex characters".to_string()))?;
    let computed = content_hash(&body).await;
    if computed != addressed {
        return Err(ServerError::Conflict(format!("blob address {hash} does not match the content hash {computed} of the uploaded bytes")));
    }
    state.blobs.lock().await.put(computed, &body).await?;
    Ok(Json(BlobReceipt { hash, size: body.len() }))
}

/// 📦️ Read bytes back by address.
pub async fn get_blob(
    Path(hash): Path<String>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState>,
) -> Result<Response, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest {
        point: PolicyPoint::BlobRead,
        principal: resolved.principal,
        scope: None,
        resource: hash.clone(),
        action: "read".to_string(),
    })?;
    let addressed = parse_content_hash(&hash).ok_or_else(|| ServerError::BadRequest("blob address is not 64 hex characters".to_string()))?;
    let bytes = state.blobs.lock().await.get(&addressed).await.ok_or_else(|| ServerError::NotFound(format!("no blob at {hash}")))?;
    Ok(([(header::CONTENT_TYPE, "application/octet-stream")], bytes).into_response())
}

/// ❓️ The cheap half of an upload negotiation: does this address already hold bytes.
pub async fn head_blob(
    Path(hash): Path<String>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState>,
) -> StatusCode {
    let resolved = state.identify(&headers, Some(peer)).await;
    let request = PolicyRequest {
        point: PolicyPoint::BlobRead,
        principal: resolved.principal,
        scope: None,
        resource: hash.clone(),
        action: "read".to_string(),
    };
    if let Err(error) = state.authorize(&request) {
        return error.status();
    }
    let Some(addressed) = parse_content_hash(&hash) else { return StatusCode::BAD_REQUEST };
    if state.blobs.lock().await.has(&addressed).await {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
//#endregion 🔖️Blob

//#region 🔖️StaticApp
/// 🗂️ One directory served as a single-page app: a traversal-guarded read, a content-type table and
/// an `index.html` fallback for client-side routes.
///
/// This is the only static server in the product. Every hosted surface — an admin console, an
/// extension bundle, a plugin's assets — is an instance of this type registered in the
/// [`AppRegistry`], because a second copy of a path-traversal guard is a second copy of a
/// vulnerability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticAppHost {
    root: PathBuf,
}

impl StaticAppHost {
    /// 🏠️ Serve this directory.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// 📍️ The directory being served.
    pub fn root(&self) -> &FsPath {
        &self.root
    }

    /// 🚧️ Resolve a request path inside the root, or refuse it.
    ///
    /// Refuses any `..` segment and any backslash (a Windows separator smuggled through a URL), and
    /// strips every leading `/` before joining — [`PathBuf::join`] treats an absolute argument as a
    /// full replacement of the base, so `/etc/passwd` would otherwise escape the root entirely. The
    /// joined path is checked against the root a second time as defence in depth.
    pub fn resolve(&self, rest: &str) -> Option<PathBuf> {
        if rest.contains("..") || rest.contains('\\') {
            return None;
        }
        let path = self.root.join(rest.trim_start_matches('/'));
        if !path.starts_with(&self.root) {
            return None;
        }
        Some(path)
    }

    /// 🏷️ The content type of an asset, by extension.
    pub fn content_type(path: &FsPath) -> &'static str {
        match path.extension().and_then(|value| value.to_str()) {
            Some("html") => "text/html; charset=utf-8",
            Some("js") | Some("mjs") => "text/javascript",
            Some("css") => "text/css",
            Some("json") => "application/json",
            Some("svg") => "image/svg+xml",
            Some("png") => "image/png",
            Some("woff2") => "font/woff2",
            Some("wasm") => "application/wasm",
            _ => "application/octet-stream",
        }
    }

    /// 📤️ Serve one path. A missing file that is not itself a build output falls back to
    /// `index.html` so a client-side route reloads correctly; a root that was never built at all is
    /// a 503 with a hint, never a confusing 404 loop.
    pub fn serve(&self, rest: &str) -> Response {
        if !self.root.is_dir() {
            return (StatusCode::SERVICE_UNAVAILABLE, format!("static app not built at {}", self.root.display())).into_response();
        }
        let Some(requested) = self.resolve(rest) else {
            return ServerError::BadRequest(format!("refused path '{rest}'")).into_response();
        };
        let path = if requested.is_file() { requested } else { self.root.join("index.html") };
        match std::fs::read(&path) {
            Ok(bytes) => ([(header::CONTENT_TYPE, Self::content_type(&path))], bytes).into_response(),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => ServerError::NotFound(rest.to_string()).into_response(),
            Err(error) => ServerError::Internal(error.to_string()).into_response(),
        }
    }
}
//#endregion 🔖️StaticApp

//#region 🔖️Apps
/// 🧩️ One installed entry discovered under an app root: a directory holding an `install.json`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInstall {
    /// 🆔️ The directory name the entry was found under.
    pub id: String,
    /// 📇️ The verbatim `install.json` document; the gateway never interprets its shape.
    pub manifest: serde_json::Value,
}

/// 🔍️ Every `install.json` entry directly under `root`, ordered by id. An unreadable or malformed
/// entry is skipped rather than failing the whole scan.
pub fn scan_installs(root: &FsPath) -> Vec<AppInstall> {
    let Ok(entries) = std::fs::read_dir(root) else { return Vec::new() };
    let mut installs: Vec<AppInstall> = entries
        .flatten()
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .filter_map(|entry| {
            let bytes = std::fs::read(entry.path().join("install.json")).ok()?;
            let manifest = serde_json::from_slice(&bytes).ok()?;
            Some(AppInstall { id: entry.file_name().to_string_lossy().into_owned(), manifest })
        })
        .collect();
    installs.sort_by(|left, right| left.id.cmp(&right.id));
    installs
}

/// 🗃️ The static surfaces this instance hosts, one [`StaticAppHost`] per registered name.
#[derive(Default)]
pub struct AppRegistry {
    apps: DashMap<String, StaticAppHost>,
}

impl AppRegistry {
    /// 🌱️ A registry hosting nothing.
    pub fn new() -> Self {
        Self::default()
    }

    /// ➕️ Host `dir` under `name`, replacing any app of that name.
    pub fn register(&self, name: &str, dir: impl Into<PathBuf>) {
        self.apps.insert(name.to_string(), StaticAppHost::new(dir));
    }

    /// 🔎️ The host registered under `name`.
    pub fn host(&self, name: &str) -> Option<StaticAppHost> {
        self.apps.get(name).map(|entry| entry.value().clone())
    }

    /// 📋️ Every registered app name, ordered.
    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.apps.iter().map(|entry| entry.key().clone()).collect();
        names.sort();
        names
    }

    /// 🧩️ The `install.json` entries under one registered app's root.
    pub fn installs(&self, name: &str) -> Vec<AppInstall> {
        self.host(name).map(|host| scan_installs(host.root())).unwrap_or_default()
    }
}
//#endregion 🔖️Apps

//#region 🔖️Command
/// 📨️ Submit one command. The envelope's principal is overwritten with the resolved one before the
/// turn runs — a client may address a command, it may never assert who is sending it. Accepted
/// events are published onto the actor's durable lane so live subscribers see them without polling.
pub async fn post_command(
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState>,
    Json(envelope): Json<CommandEnvelope>,
) -> Result<Json<CommandOutcome>, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    let mut envelope = envelope;
    envelope.principal = resolved.principal;
    envelope.session = resolved.session;
    envelope.device = resolved.device;
    let now = state.now();
    let outcome = state.authority.lock().await.submit(envelope, now).await;
    if let CommandOutcome::Accepted { events, .. } = &outcome {
        for event in events {
            if let Ok(bytes) = serde_json::to_vec(event) {
                state.fanout.publish(&stream_lane(&event.stream), bytes);
            }
        }
    }
    Ok(Json(outcome))
}

/// ❓️ Answer one query from the projections, after checking the caller may read it.
pub async fn post_query(
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState>,
    Json(envelope): Json<QueryEnvelope>,
) -> Result<Json<QueryResult>, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    let mut envelope = envelope;
    envelope.principal = resolved.principal;
    state.authorize(&PolicyRequest {
        point: PolicyPoint::QueryAccess,
        principal: envelope.principal.clone(),
        scope: Some(envelope.scope.clone()),
        resource: envelope.kind.clone(),
        action: "read".to_string(),
    })?;
    let handler = state
        .queries
        .get(&envelope.kind)
        .map(|entry| Arc::clone(entry.value()))
        .ok_or_else(|| ServerError::NotFound(format!("no handler for query kind '{}'", envelope.kind)))?;
    let projections = state.projections.lock().await;
    Ok(Json(handler.handle(&envelope, &projections).await?))
}

/// 💨️ Publish one ephemeral frame onto its scope's lossy lane. Nothing is persisted and nothing is
/// replayed — a subscriber that was not listening simply missed it.
pub async fn post_ephemeral(
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState>,
    Json(frame): Json<EphemeralFrame>,
) -> Result<Json<usize>, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    let mut frame = frame;
    frame.principal = resolved.principal;
    state.authorize(&PolicyRequest {
        point: PolicyPoint::Subscription,
        principal: frame.principal.clone(),
        scope: Some(frame.scope.clone()),
        resource: frame.kind.clone(),
        action: "publish".to_string(),
    })?;
    let lane = ephemeral_lane(&frame.scope);
    let bytes = serde_json::to_vec(&frame).map_err(|error| ServerError::BadRequest(error.to_string()))?;
    Ok(Json(state.fanout.publish(&lane, bytes)))
}
//#endregion 🔖️Command

//#region 🔖️EventStream
/// 🪡️ The replay/live seam. Everything with a sequence at or below the high-water mark has already
/// been delivered, so a frame arriving twice — once from the replay, once from the live lane —
/// is admitted exactly once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventSeam {
    delivered: u64,
}

impl EventSeam {
    /// 🌱️ A seam resuming after `since`.
    pub fn new(since: u64) -> Self {
        Self { delivered: since }
    }

    /// ✅️ Whether this event still has to be delivered, advancing the high-water mark if so.
    pub fn admit(&mut self, event: &EventRecord) -> bool {
        if event.seq <= self.delivered {
            return false;
        }
        self.delivered = event.seq;
        true
    }

    /// 🚩️ The highest sequence delivered so far.
    pub fn delivered(&self) -> u64 {
        self.delivered
    }
}

/// 📼️ The generic durable-lane bridge: **subscribe first**, then replay, then forward live.
///
/// The order is the whole point. Subscribing before reading `events_since` means an event committed
/// between the read and the first `recv` is buffered rather than lost; the [`EventSeam`] then drops
/// whatever the replay already covered, so the seam has neither a gap nor a duplicate. Returns once
/// the sink refuses a frame or the lane closes.
pub async fn pump_events<S>(state: &ServerState, actor: &ActorKey, since: u64, live: &mut Subscription, sink: &mut S) -> Result<(), ServerError>
where
    S: Sink<Message> + Unpin,
{
    let mut seam = EventSeam::new(since);
    for event in state.replay_events(actor, since).await? {
        if seam.admit(&event) && !deliver_event(sink, &event).await {
            return Ok(());
        }
    }
    while let Some(bytes) = live.recv().await {
        let Ok(event) = serde_json::from_slice::<EventRecord>(&bytes) else { continue };
        if seam.admit(&event) && !deliver_event(sink, &event).await {
            break;
        }
    }
    Ok(())
}

async fn deliver_event<S>(sink: &mut S, event: &EventRecord) -> bool
where
    S: Sink<Message> + Unpin,
{
    let Ok(text) = serde_json::to_string(event) else { return true };
    sink.send(Message::Text(text.into())).await.is_ok()
}

/// 🔖️ Where a durable-lane subscriber resumes from.
#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct EventStreamQuery {
    /// 🚩️ The last sequence the client already holds; `0` replays the whole stream.
    #[serde(default)]
    pub since: u64,
}

/// 📜️ One page of durable history, for a client that would rather poll than hold a socket.
pub async fn get_events(
    Path((tenant, kind, id)): Path<(String, String, String)>,
    Query(query): Query<EventStreamQuery>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState>,
) -> Result<Json<Vec<EventRecord>>, ServerError> {
    let actor = ActorKey { tenant: TenantId(tenant), kind, id };
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest {
        point: PolicyPoint::EventDelivery,
        principal: resolved.principal,
        scope: None,
        resource: stream_lane(&actor),
        action: "read".to_string(),
    })?;
    Ok(Json(state.replay_events(&actor, query.since).await?))
}

/// 📡️ The durable lane as a websocket: replay then live, gap-free and duplicate-free.
pub async fn get_event_stream_ws(
    ws: WebSocketUpgrade,
    Path((tenant, kind, id)): Path<(String, String, String)>,
    Query(query): Query<EventStreamQuery>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState>,
) -> Result<Response, ServerError> {
    let actor = ActorKey { tenant: TenantId(tenant), kind, id };
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest {
        point: PolicyPoint::Subscription,
        principal: resolved.principal,
        scope: None,
        resource: stream_lane(&actor),
        action: "subscribe".to_string(),
    })?;
    Ok(ws.on_upgrade(move |socket| handle_event_stream(socket, actor, query.since, state)))
}

async fn handle_event_stream(socket: WebSocket, actor: ActorKey, since: u64, state: ServerState) {
    let (mut sender, mut receiver) = socket.split();
    let mut live = state.fanout.subscribe(&stream_lane(&actor));
    let pump = pump_events(&state, &actor, since, &mut live, &mut sender);
    tokio::select! {
        _ = pump => {}
        _ = drain_until_close(&mut receiver) => {}
    }
}

async fn drain_until_close(receiver: &mut futures::stream::SplitStream<WebSocket>) {
    while let Some(Ok(message)) = receiver.next().await {
        if matches!(message, Message::Close(_)) {
            return;
        }
    }
}
//#endregion 🔖️EventStream

//#region 🔖️DocumentStream
/// 📄️ How a session identifies itself when joining a document.
#[derive(Clone, Debug, Deserialize)]
pub struct DocumentStreamQuery {
    /// 🙋️ The actor joining, as the presence roster will list it.
    pub actor: String,
    /// 🎫️ The session id the administration plane may kick.
    pub session: Option<String>,
    /// 🪟️ Which surface the actor joined from.
    pub surface: Option<String>,
    /// ⏮️ The engine's own resumption token, passed through verbatim.
    pub resume: Option<String>,
}

/// 🔗️ Bridge one document socket onto the instance's [`DocumentAuthority`].
pub async fn get_document_ws(
    ws: WebSocketUpgrade,
    Path(scope): Path<String>,
    Query(query): Query<DocumentStreamQuery>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState>,
) -> Result<Response, ServerError> {
    if state.documents.is_none() {
        return Err(ServerError::NotFound("this instance hosts no document authority".to_string()));
    }
    let scope = Scope(scope);
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest {
        point: PolicyPoint::Subscription,
        principal: resolved.principal.clone(),
        scope: Some(scope.clone()),
        resource: document_lane(&scope),
        action: "subscribe".to_string(),
    })?;
    Ok(ws.on_upgrade(move |socket| handle_document(socket, scope, query, resolved.principal, state)))
}

/// 📮️ Wrap a relayed frame with the session that produced it, so a session never receives its own
/// frames back through the lane it also sends on.
fn wrap_relay(origin: &str, frame: &[u8]) -> Vec<u8> {
    let origin = origin.as_bytes();
    let mut wrapped = Vec::with_capacity(2 + origin.len() + frame.len());
    wrapped.extend_from_slice(&(origin.len() as u16).to_le_bytes());
    wrapped.extend_from_slice(origin);
    wrapped.extend_from_slice(frame);
    wrapped
}

/// 📬️ The inverse of [`wrap_relay`]; a malformed wrapper is dropped rather than trusted.
fn unwrap_relay(bytes: &[u8]) -> Option<(&str, &[u8])> {
    let length = usize::from(u16::from_le_bytes([*bytes.first()?, *bytes.get(1)?]));
    let origin = std::str::from_utf8(bytes.get(2..2 + length)?).ok()?;
    Some((origin, bytes.get(2 + length..)?))
}

async fn handle_document(socket: WebSocket, scope: Scope, query: DocumentStreamQuery, principal: Principal, state: ServerState) {
    let Some(documents) = state.documents.clone() else { return };
    let (mut sender, mut receiver) = socket.split();
    let session = query.session.clone().unwrap_or_else(|| query.actor.clone());
    let lane = document_lane(&scope);
    let mut live = state.fanout.subscribe(&lane);
    state.presence.join(&scope.0, &query.actor, query.surface.as_deref().unwrap_or("unknown"));

    match documents.welcome(&scope, &query.actor, query.resume.as_deref()).await {
        Ok(welcome) => {
            if sender.send(Message::Binary(welcome.into())).await.is_err() {
                state.presence.leave(&scope.0, &query.actor);
                return;
            }
        }
        Err(error) => {
            let _ = sender.send(Message::Text(error.to_string().into())).await;
            state.presence.leave(&scope.0, &query.actor);
            return;
        }
    }

    loop {
        tokio::select! {
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Binary(payload))) => {
                        match documents.submit_frame(&scope, &principal, &payload).await {
                            Ok(frames) => {
                                for frame in frames {
                                    if sender.send(Message::Binary(frame.clone().into())).await.is_err() {
                                        break;
                                    }
                                    state.fanout.publish(&lane, wrap_relay(&session, &frame));
                                }
                            }
                            Err(error) => {
                                if sender.send(Message::Text(error.to_string().into())).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                    Some(Ok(Message::Ping(payload))) => {
                        if sender.send(Message::Pong(payload)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Err(_)) => break,
                    Some(Ok(_)) => {}
                }
            }
            relayed = live.recv() => {
                let Some(bytes) = relayed else { break };
                let Some((origin, frame)) = unwrap_relay(&bytes) else { continue };
                if origin == session {
                    continue;
                }
                if sender.send(Message::Binary(frame.to_vec().into())).await.is_err() {
                    break;
                }
            }
            _ = state.kicks.kicked(&session) => break,
        }
    }

    state.presence.leave(&scope.0, &query.actor);
    state.kicks.forget(&session);
}
//#endregion 🔖️DocumentStream

//#region 🔖️AppRoutes
/// 📋️ The names of every app this instance hosts.
pub async fn get_apps(State(state): State<ServerState>) -> Json<Vec<String>> {
    Json(state.apps.names())
}

/// 🧩️ The `install.json` entries under one app's root.
pub async fn get_app_installs(Path(app): Path<String>, State(state): State<ServerState>) -> Result<Json<Vec<AppInstall>>, ServerError> {
    let host = state.apps.host(&app).ok_or_else(|| ServerError::NotFound(format!("no app '{app}'")))?;
    Ok(Json(scan_installs(host.root())))
}

/// 🏠️ One app's entry document.
pub async fn get_app_root(Path(app): Path<String>, State(state): State<ServerState>) -> Response {
    serve_app(&state, &app, "index.html")
}

/// 📎️ One asset inside an app.
pub async fn get_app_asset(Path((app, rest)): Path<(String, String)>, State(state): State<ServerState>) -> Response {
    serve_app(&state, &app, &rest)
}

fn serve_app(state: &ServerState, app: &str, rest: &str) -> Response {
    match state.apps.host(app) {
        Some(host) => host.serve(rest),
        None => ServerError::NotFound(format!("no app '{app}'")).into_response(),
    }
}
//#endregion 🔖️AppRoutes

//#region 🔖️Server
/// 🏗️ Assembles one server out of a storage profile and a set of modules.
pub struct ServerBuilder {
    profile: StorageProfile,
    modules: Vec<ServerModules>,
    queries: Vec<QueryHandlers>,
    apps: Vec<(String, PathBuf)>,
    documents: Option<Arc<DocumentAuthorities>>,
    admin_token: Option<String>,
    id: String,
    version: String,
}

impl ServerBuilder {
    /// 🧩️ Register one module. Its deciders, templates, resolvers and routes are collected at
    /// [`build`](Self::build) time, in registration order.
    pub fn module(mut self, module: ServerModules) -> Self {
        self.modules.push(module);
        self
    }

    /// ❓️ Register one query handler under the kind it declares.
    pub fn query(mut self, handler: QueryHandlers) -> Self {
        self.queries.push(handler);
        self
    }

    /// 📄️ Supply the replication engine backing the document websocket.
    pub fn document_authority(mut self, documents: Arc<DocumentAuthorities>) -> Self {
        self.documents = Some(documents);
        self
    }

    /// 🗂️ Host a directory as a static app under `name`.
    pub fn app(mut self, name: &str, dir: impl Into<PathBuf>) -> Self {
        self.apps.push((name.to_string(), dir.into()));
        self
    }

    /// 🚪️ Configure the administration gate. `None` leaves it in loopback-only mode.
    pub fn admin_token(mut self, token: Option<String>) -> Self {
        self.admin_token = token;
        self
    }

    /// 🏷️ Name this instance, as reported by `GET /instance`.
    pub fn identity(mut self, id: &str, version: &str) -> Self {
        self.id = id.to_string();
        self.version = version.to_string();
        self
    }

    /// 🔨️ Collect every module's contribution into one shared engine, bus, ladder and router.
    pub async fn build(self) -> Server {
        let policy = Arc::new(RwLock::new(PolicyEngine::new()));
        let mut chain = ResolverChain::new();
        let mut definition = ServerInstanceDefinition { id: self.id.clone(), version: self.version.clone(), modules: Vec::new() };
        let mut deciders: Vec<Deciders> = Vec::new();
        for module in &self.modules {
            let manifest = module.manifest().await;
            if let Ok(mut engine) = policy.write() {
                for template in manifest.policies.iter().cloned().chain(module.templates().await) {
                    engine.register_template(template);
                }
            }
            definition.modules.push(manifest);
            deciders.extend(module.deciders().await);
            for resolver in module.resolvers().await {
                chain.push(resolver);
            }
        }

        let hook: PolicyHook = {
            let policy = Arc::clone(&policy);
            Box::new(move |envelope: &CommandEnvelope| match policy.read() {
                Ok(engine) => engine.evaluate(&admission_request(envelope)),
                Err(_) => PolicyDecision::Deny { reason: "policy engine poisoned".to_string() },
            })
        };

        let StorageProfile::Embedded { data_dir } = &self.profile;
        let store = AuthorityStores::Memory(MemoryAuthorityStore::new());
        let mut bus = CommandBus::new(AuthorityDirectory::new(), store, hook);
        for decider in deciders {
            bus.register(decider).await;
        }

        let apps = AppRegistry::new();
        for (name, dir) in &self.apps {
            apps.register(name, dir.clone());
        }
        let queries: DashMap<String, Arc<QueryHandlers>> = DashMap::new();
        for handler in self.queries {
            queries.insert(handler.kind().await.to_string(), Arc::new(handler));
        }

        let state = ServerState {
            authority: Arc::new(Mutex::new(bus)),
            projections: Arc::new(Mutex::new(ProjectionStores::Memory(MemoryProjectionStore::new()))),
            blobs: Arc::new(Mutex::new(BlobStores::Memory(MemoryBlobStore::new()))),
            sessions: Arc::new(Mutex::new(SessionStores::Memory(MemorySessionStore::new()))),
            policy,
            resolvers: Arc::new(chain),
            admin: Arc::new(AdminGate::new(self.admin_token.clone())),
            fanout: Arc::new(Fanout::new()),
            presence: Arc::new(Presence::new()),
            kicks: Arc::new(KickMap::new()),
            apps: Arc::new(apps),
            queries: Arc::new(queries),
            documents: self.documents.clone(),
            data_dir: Arc::new(PathBuf::from(data_dir)),
            clock: Arc::new(StdMutex::new(HybridLogicalClock::default())),
        };

        let mut router = base_router(definition.clone());
        for module in &self.modules {
            router = module.routes(router).await;
        }
        let router = router.layer(axum::middleware::from_fn(cors_middleware)).with_state(state.clone());
        Server { state, router, definition }
    }
}

/// 🚦️ The admission question one command envelope asks of the policy engine.
fn admission_request(envelope: &CommandEnvelope) -> PolicyRequest {
    PolicyRequest {
        point: PolicyPoint::CommandAdmission,
        principal: envelope.principal.clone(),
        scope: Some(envelope.scope.clone()),
        resource: format!("{}/{}", envelope.target.kind, envelope.target.id),
        action: envelope.kind.clone(),
    }
}

/// 🛣️ Every route the framework itself owns, before any module adds its own.
fn base_router(definition: ServerInstanceDefinition) -> Router<ServerState> {
    Router::new()
        .route("/instance", get(move || instance_body(definition.clone())))
        .route("/commands", post(post_command))
        .route("/queries", post(post_query))
        .route("/scopes/{scope}/ephemeral", post(post_ephemeral))
        .route("/scopes/{scope}/document/ws", get(get_document_ws))
        .route("/actors/{tenant}/{kind}/{id}/events", get(get_events))
        .route("/actors/{tenant}/{kind}/{id}/events/ws", get(get_event_stream_ws))
        .route("/blobs/{hash}", get(get_blob).head(head_blob).put(put_blob))
        .route("/apps", get(get_apps))
        .route("/apps/{app}/installs", get(get_app_installs))
        .route("/apps/{app}", get(get_app_root))
        .route("/apps/{app}/{*rest}", get(get_app_asset))
}

async fn instance_body(definition: ServerInstanceDefinition) -> Json<ServerInstanceDefinition> {
    Json(definition)
}

/// 🖥️ One built server: its shared state, its router and the definition it reports.
pub struct Server {
    state: ServerState,
    router: Router,
    definition: ServerInstanceDefinition,
}

impl Server {
    /// 🏗️ Start assembling a server over one storage profile.
    pub fn builder(profile: StorageProfile) -> ServerBuilder {
        ServerBuilder {
            profile,
            modules: Vec::new(),
            queries: Vec::new(),
            apps: Vec::new(),
            documents: None,
            admin_token: None,
            id: "server".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// 🛣️ The fully wired router, ready to be served or mounted.
    pub fn router(&self) -> Router {
        self.router.clone()
    }

    /// 🧠️ The shared state every handler runs against.
    pub fn state(&self) -> &ServerState {
        &self.state
    }

    /// 🏛️ What this instance declares itself to be.
    pub fn definition(&self) -> &ServerInstanceDefinition {
        &self.definition
    }

    /// ▶️ Bind `addr` and serve until the process is stopped. Connection info is carried into every
    /// handler so the loopback fact behind [`AdminGate`] stays a transport fact.
    pub async fn run(self, addr: SocketAddr) -> Result<(), ServerError> {
        let listener = tokio::net::TcpListener::bind(addr).await.map_err(|error| ServerError::Internal(error.to_string()))?;
        axum::serve(listener, self.router.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .map_err(|error| ServerError::Internal(error.to_string()))
    }
}
//#endregion 🔖️Server

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{CommandId, PolicyGrant, TraceContext};
    use futures::channel::mpsc::unbounded;
    use std::time::Duration;

    async fn state() -> ServerState {
        Server::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).build().await.state().clone()
    }

    fn actor(id: &str) -> ActorKey {
        ActorKey { tenant: TenantId("t1".into()), kind: "counter".into(), id: id.into() }
    }

    fn event(stream: &ActorKey, seq: u64) -> EventRecord {
        EventRecord { stream: stream.clone(), seq, hlc: HybridLogicalClock::default(), kind: "counter.incremented".into(), payload: vec![seq as u8] }
    }

    fn grant(state: &ServerState, point: PolicyPoint, action: &str) {
        let mut engine = state.policy.write().unwrap();
        engine.register_template(PolicyTemplate {
            name: format!("{point:?}-{action}"),
            grants: vec![PolicyGrant { point, resource: "*".into(), action: action.to_string() }],
        });
        engine.assign("anonymous".to_string(), format!("{point:?}-{action}"));
    }

    fn scratch(name: &str) -> PathBuf {
        let unique = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
        let path = std::env::temp_dir().join(format!("semio-gateway-{name}-{unique}"));
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    fn loopback() -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], 4242))
    }

    //#region 🔖️Presence
    #[test]
    fn a_colour_lease_takes_the_lowest_free_slot() {
        let presence = Presence::new();
        assert_eq!(presence.acquire_colour("space-1", "alice"), 0);
        assert_eq!(presence.acquire_colour("space-1", "bob"), 1);
        assert_eq!(presence.acquire_colour("space-1", "bob"), 1);
        presence.release_colour("space-1", "alice");
        assert_eq!(presence.acquire_colour("space-1", "carol"), 0);
        assert_eq!(presence.acquire_colour("space-2", "dave"), 0);
    }

    #[test]
    fn a_colour_is_freed_only_on_the_last_disconnect() {
        let presence = Presence::new();
        assert_eq!(presence.acquire_colour("space-1", "alice"), 0);
        assert_eq!(presence.acquire_colour("space-1", "alice"), 0);
        presence.release_colour("space-1", "alice");
        assert_eq!(presence.colour_of("space-1", "alice"), Some(0));
        assert_eq!(presence.acquire_colour("space-1", "bob"), 1);
        presence.release_colour("space-1", "alice");
        assert_eq!(presence.colour_of("space-1", "alice"), None);
        assert_eq!(presence.acquire_colour("space-1", "carol"), 0);
    }

    #[test]
    fn joining_and_leaving_maintains_the_roster() {
        let presence = Presence::new();
        presence.join("space-1", "bob", "canvas");
        presence.join("space-1", "alice", "canvas");
        presence.publish_peer("space-1", "alice", vec![7]);
        let roster = presence.roster("space-1");
        assert_eq!(roster.iter().map(|session| session.actor.as_str()).collect::<Vec<_>>(), vec!["alice", "bob"]);
        assert_eq!(roster[0].peer, Some(vec![7]));
        presence.leave("space-1", "alice");
        assert_eq!(presence.roster("space-1").len(), 1);
        assert_eq!(presence.colour_of("space-1", "alice"), None);
    }
    //#endregion 🔖️Presence

    //#region 🔖️Fanout
    #[tokio::test]
    async fn a_lane_reaches_every_subscriber_and_is_dropped_when_empty() {
        let fanout = Fanout::new();
        assert_eq!(fanout.publish("stream:a", vec![1]), 0);
        let mut first = fanout.subscribe("stream:a");
        let mut second = fanout.subscribe("stream:a");
        assert_eq!(fanout.lanes(), 1);
        assert_eq!(fanout.publish("stream:a", vec![9]), 2);
        assert_eq!(first.recv().await, Some(vec![9]));
        assert_eq!(second.recv().await, Some(vec![9]));
        drop(first);
        assert_eq!(fanout.lanes(), 1);
        drop(second);
        assert_eq!(fanout.lanes(), 0);
    }

    #[test]
    fn lane_keys_keep_the_durable_and_ephemeral_lanes_apart() {
        let scope = Scope("space-1".into());
        assert_eq!(document_lane(&scope), "document:space-1");
        assert_eq!(ephemeral_lane(&scope), "ephemeral:space-1");
        assert_eq!(stream_lane(&actor("c1")), "stream:t1/counter/c1");
    }
    //#endregion 🔖️Fanout

    //#region 🔖️Kick
    #[tokio::test]
    async fn a_kick_fired_before_the_wait_is_still_observed() {
        let kicks = KickMap::new();
        kicks.kick("session-1");
        tokio::time::timeout(Duration::from_millis(200), kicks.kicked("session-1")).await.expect("kick must be observed");
        assert_eq!(kicks.tracked(), 1);
        kicks.forget("session-1");
        assert_eq!(kicks.tracked(), 0);
        assert!(tokio::time::timeout(Duration::from_millis(20), kicks.kicked("session-2")).await.is_err());
    }
    //#endregion 🔖️Kick

    //#region 🔖️StaticApp
    #[test]
    fn a_static_host_refuses_traversal() {
        let host = StaticAppHost::new("/srv/app");
        assert!(host.resolve("../secret").is_none());
        assert!(host.resolve("nested/../../secret").is_none());
        assert!(host.resolve("nested\\secret").is_none());
        assert_eq!(host.resolve("/etc/passwd"), Some(PathBuf::from("/srv/app/etc/passwd")));
        assert_eq!(host.resolve("assets/app.js"), Some(PathBuf::from("/srv/app/assets/app.js")));
    }

    #[tokio::test]
    async fn a_client_route_falls_back_to_index_html() {
        let root = scratch("spa");
        std::fs::write(root.join("index.html"), b"<!doctype html>shell").unwrap();
        std::fs::write(root.join("app.js"), b"export {}").unwrap();
        let host = StaticAppHost::new(&root);

        let asset = host.serve("app.js");
        assert_eq!(asset.status(), StatusCode::OK);
        assert_eq!(asset.headers().get(header::CONTENT_TYPE).unwrap(), "text/javascript");

        let route = host.serve("spaces/sp-1");
        assert_eq!(route.status(), StatusCode::OK);
        let body = axum::body::to_bytes(route.into_body(), usize::MAX).await.unwrap();
        assert_eq!(body.as_ref(), b"<!doctype html>shell");

        assert_eq!(host.serve("../secret").status(), StatusCode::BAD_REQUEST);
        assert_eq!(StaticAppHost::new(root.join("missing")).serve("index.html").status(), StatusCode::SERVICE_UNAVAILABLE);
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn installs_are_scanned_and_ordered() {
        let root = scratch("apps");
        for id in ["beta", "alpha"] {
            std::fs::create_dir_all(root.join(id)).unwrap();
            std::fs::write(root.join(id).join("install.json"), format!("{{\"extensionId\":\"{id}\"}}")).unwrap();
        }
        std::fs::create_dir_all(root.join("empty")).unwrap();
        let registry = AppRegistry::new();
        registry.register("extensions", &root);
        let installs = registry.installs("extensions");
        assert_eq!(installs.iter().map(|install| install.id.as_str()).collect::<Vec<_>>(), vec!["alpha", "beta"]);
        assert_eq!(installs[0].manifest["extensionId"], "alpha");
        assert_eq!(registry.names(), vec!["extensions".to_string()]);
        assert!(registry.installs("nothing").is_empty());
        std::fs::remove_dir_all(&root).unwrap();
    }
    //#endregion 🔖️StaticApp

    //#region 🔖️Cors
    #[test]
    fn cors_reflects_the_callers_own_origin() {
        let mut headers = HeaderMap::new();
        apply_cors_headers(&mut headers, Some(&HeaderValue::from_static("http://127.0.0.1:6072")));
        assert_eq!(headers.get(header::ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(), "http://127.0.0.1:6072");
        assert_eq!(headers.get(header::ACCESS_CONTROL_ALLOW_CREDENTIALS).unwrap(), "true");
        assert_eq!(headers.get(header::VARY).unwrap(), "origin");

        let mut anonymous = HeaderMap::new();
        apply_cors_headers(&mut anonymous, None);
        assert!(anonymous.get(header::ACCESS_CONTROL_ALLOW_ORIGIN).is_none());
        assert!(anonymous.get(header::ACCESS_CONTROL_ALLOW_METHODS).is_some());
    }
    //#endregion 🔖️Cors

    //#region 🔖️Credential
    #[test]
    fn a_bearer_and_the_loopback_fact_are_read_from_the_transport() {
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, HeaderValue::from_static("Bearer tok"));
        headers.insert(CAPABILITY_HEADER, HeaderValue::from_static("share-1"));
        let local = credential(&headers, Some(loopback()));
        assert_eq!(local.bearer.as_deref(), Some("tok"));
        assert_eq!(local.capability.map(|proof| proof.0), Some("share-1".to_string()));
        assert!(local.loopback);
        assert!(!credential(&headers, Some(SocketAddr::from(([10, 0, 0, 7], 80)))).loopback);
        assert!(!credential(&HeaderMap::new(), None).loopback);
        assert!(credential(&HeaderMap::new(), None).bearer.is_none());
    }
    //#endregion 🔖️Credential

    //#region 🔖️Blob
    #[tokio::test]
    async fn a_blob_address_that_does_not_match_its_bytes_is_a_conflict() {
        let state = state().await;
        grant(&state, PolicyPoint::BlobWrite, "write");
        grant(&state, PolicyPoint::BlobRead, "read");

        let honest = content_hash(b"hello").await.to_string();
        let receipt = put_blob(Path(honest.clone()), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), Bytes::from_static(b"hello"))
            .await
            .expect("an honest address is accepted");
        assert_eq!(receipt.0.size, 5);

        let lie = content_hash(b"world").await.to_string();
        let error = put_blob(Path(lie), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), Bytes::from_static(b"hello"))
            .await
            .expect_err("a mismatched address is refused");
        assert_eq!(error.status(), StatusCode::CONFLICT);

        assert_eq!(head_blob(Path(honest.clone()), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone())).await, StatusCode::OK);
        let missing = content_hash(b"world").await.to_string();
        assert_eq!(head_blob(Path(missing), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone())).await, StatusCode::NOT_FOUND);
        let short = put_blob(Path("beef".to_string()), HeaderMap::new(), ConnectInfo(loopback()), State(state), Bytes::from_static(b"hello")).await;
        assert_eq!(short.err().map(|error| error.status()), Some(StatusCode::BAD_REQUEST));
    }

    #[tokio::test]
    async fn a_blob_write_without_a_grant_is_forbidden() {
        let state = state().await;
        let hash = content_hash(b"hello").await.to_string();
        let error = put_blob(Path(hash), HeaderMap::new(), ConnectInfo(loopback()), State(state), Bytes::from_static(b"hello"))
            .await
            .expect_err("closed by default");
        assert_eq!(error.status(), StatusCode::FORBIDDEN);
    }
    //#endregion 🔖️Blob

    //#region 🔖️EventStream
    #[tokio::test]
    async fn the_replay_live_seam_has_no_gap_and_no_duplicate() {
        let state = state().await;
        let actor = actor("c1");
        {
            let mut authority = state.authority.lock().await;
            authority.store_mut().append_events(&actor, &[event(&actor, 1), event(&actor, 2), event(&actor, 3)], &[]).await.unwrap();
        }

        let mut live = state.fanout.subscribe(&stream_lane(&actor));
        state.fanout.publish(&stream_lane(&actor), serde_json::to_vec(&event(&actor, 3)).unwrap());
        state.fanout.publish(&stream_lane(&actor), serde_json::to_vec(&event(&actor, 4)).unwrap());

        let (mut sink, stream) = unbounded::<Message>();
        let pump = pump_events(&state, &actor, 0, &mut live, &mut sink);
        assert!(tokio::time::timeout(Duration::from_millis(120), pump).await.is_err());
        drop(sink);

        let delivered: Vec<u64> = stream
            .collect::<Vec<Message>>()
            .await
            .iter()
            .filter_map(|message| match message {
                Message::Text(text) => serde_json::from_str::<EventRecord>(text.as_str()).ok(),
                _ => None,
            })
            .map(|record| record.seq)
            .collect();
        assert_eq!(delivered, vec![1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn a_resuming_subscriber_skips_what_it_already_holds() {
        let state = state().await;
        let actor = actor("c1");
        {
            let mut authority = state.authority.lock().await;
            authority.store_mut().append_events(&actor, &[event(&actor, 1), event(&actor, 2)], &[]).await.unwrap();
        }
        let mut live = state.fanout.subscribe(&stream_lane(&actor));
        let (mut sink, stream) = unbounded::<Message>();
        let pump = pump_events(&state, &actor, 1, &mut live, &mut sink);
        assert!(tokio::time::timeout(Duration::from_millis(80), pump).await.is_err());
        drop(sink);
        assert_eq!(stream.collect::<Vec<Message>>().await.len(), 1);
    }

    #[test]
    fn the_seam_admits_every_sequence_once() {
        let actor = actor("c1");
        let mut seam = EventSeam::new(0);
        assert!(seam.admit(&event(&actor, 1)));
        assert!(seam.admit(&event(&actor, 2)));
        assert!(!seam.admit(&event(&actor, 2)));
        assert!(!seam.admit(&event(&actor, 1)));
        assert!(seam.admit(&event(&actor, 3)));
        assert_eq!(seam.delivered(), 3);
    }
    //#endregion 🔖️EventStream

    //#region 🔖️Relay
    #[test]
    fn a_relayed_frame_names_the_session_that_produced_it() {
        let wrapped = wrap_relay("session-7", b"frame");
        assert_eq!(unwrap_relay(&wrapped), Some(("session-7", b"frame".as_slice())));
        assert_eq!(unwrap_relay(b"\x40\x00short"), None);
        assert_eq!(unwrap_relay(&[]), None);
    }
    //#endregion 🔖️Relay

    //#region 🔖️Server
    #[tokio::test]
    async fn build_collects_every_module_contribution() {
        let server = Server::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() })
            .identity("hub", "0.1.0")
            .module(ServerModules::Counting(CountingModule))
            .app("admin", "/srv/admin")
            .admin_token(Some("secret".to_string()))
            .build()
            .await;

        assert_eq!(server.definition().modules.len(), 1);
        assert_eq!(server.definition().id, "hub");
        assert!(server.state().admin.is_configured());
        assert_eq!(server.state().apps.names(), vec!["admin".to_string()]);
        assert!(server.state().documents.is_none());

        let request = admission_request(&envelope());
        assert!(server.state().authorize(&request).is_err());
        server.state().policy.write().unwrap().assign("anonymous".to_string(), "author".to_string());
        assert!(server.state().authorize(&request).is_ok());
        let _ = server.router();
    }

    #[tokio::test]
    async fn an_unroutable_command_is_rejected_and_publishes_nothing() {
        let state = state().await;
        let mut live = state.fanout.subscribe(&stream_lane(&actor("c1")));
        let outcome = post_command(HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), Json(envelope())).await.unwrap();
        assert!(matches!(outcome.0, CommandOutcome::Rejected { .. }));
        assert!(tokio::time::timeout(Duration::from_millis(20), live.recv()).await.is_err());
    }

    #[tokio::test]
    async fn the_clock_never_goes_backwards() {
        let state = state().await;
        let first = state.now();
        let second = state.now();
        assert!(second > first);
    }

    fn envelope() -> CommandEnvelope {
        CommandEnvelope {
            command_id: CommandId("cmd-1".into()),
            kind: "counter.increment".into(),
            version: 1,
            target: actor("c1"),
            scope: Scope("space-1".into()),
            principal: Principal::Anonymous,
            session: None,
            device: None,
            payload: vec![1],
            causal_frontier: None,
            client_hlc: HybridLogicalClock::default(),
            expected_revision: None,
            idempotency_key: None,
            capability_proof: None,
            trace: TraceContext::default(),
        }
    }
    //#endregion 🔖️Server
}
