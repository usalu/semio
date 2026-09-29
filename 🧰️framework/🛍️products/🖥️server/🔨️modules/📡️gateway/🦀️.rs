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
//! exactly once. The ephemeral lane carries [`EphemeralFrame`]s and
//! document frames: lossy, never persisted, never replayed, dropped the moment a socket closes.
//! They travel on different [`Fanout`] lanes ([`stream_lane`], [`ephemeral_lane`],
//! [`document_lane`], [`presence_lane`]) precisely so no future refactor can quietly start replaying
//! a cursor position or dropping a committed event.
//!
//! **Presence is latest-state, coalesced per tick.** A presence room ([`PresenceRooms`]) keeps each
//! session's newest opaque state only; one ticker per room publishes the sessions that changed and
//! those that left as one `batch` on [`presence_lane`], so a room costs O(changed sessions) per tick
//! however often its members move. Joining is [`PolicyPoint::Subscription`] `join` on
//! [`PRESENCE_RESOURCE`], sharing is `publish`, and every module may refuse a state through
//! [`ServerModule::presence_admission`]. A socket may also *watch* up to
//! [`PresenceSettings::max_watch_scopes`] other rooms read-only (`watch` per scope): each watched
//! room is relayed by a forwarder and coalesced per the watcher's own interval ([`Watching`]).
//!
//! **Every extension point is a port, and [`ServerInstance`] is the one place they are all named.**
//! An instance — hub, zentrale, this crate's own test profile — is a type implementing that trait,
//! and its ten associated types say which module set, query set, document engine, decider set, saga
//! set, resolver ladder and four storage backends this deployment is made of. [`ServerBuilder`],
//! [`ServerState`], [`Server`] and every handler below are generic over it, so the set of
//! implementations is closed in the instance's own crate and never here. The server product
//! deliberately depends on no document engine: not on the os product, not on `db`, not on any
//! concrete CRDT — and with the sets closed downstream it does not have to name one to be usable.

use std::collections::{BTreeMap, BTreeSet};
use std::hash::{BuildHasher, Hasher};
use std::net::SocketAddr;
use std::path::{Path as FsPath, PathBuf};
use semio_framework_dispatch_macros::dyn_enum;
use std::future::Future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex as StdMutex, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::body::Bytes;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{ConnectInfo, Path, Query, State};
use axum::http::{header, HeaderMap, HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures::{Sink, SinkExt, Stream, StreamExt};
use semio_framework_async::ShardedMap;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc, Mutex, Notify};
use tokio::time::{Instant, MissedTickBehavior};

use crate::authority::{AuthorityDirectory, AuthorityError, CommandBus, Decider, PolicyHook, Saga, SagaRunner};
use crate::contract::{
    ActorKey, CommandEnvelope, CommandOutcome, EphemeralFrame, EventRecord, HybridLogicalClock, ModuleManifest, OpaqueJson, PolicyDecision, PolicyPoint, PolicyTemplate, PresenceEntry, PresenceFrame, Principal, QueryEnvelope, QueryResult,
    Scope, ServerInstanceDefinition, TenantId,
};
use crate::policy::{AdminGate, Credential, PolicyEngine, PolicyRequest, PrincipalResolver, Resolved, ResolverChain};
use crate::storage::{content_hash, AuthorityStore, BlobStore, ProjectionStore, SessionStore, StorageError, StorageProfile};

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
#[derive(Clone, Debug)]
pub enum ServerError {
    /// 🔒️ The caller could not be authenticated — 401.
    Unauthorized(String),
    /// ⛔️ The caller is known and policy still says no — 403.
    Forbidden(String),
    /// 🕳️ Nothing is addressed by this path — 404.
    NotFound(String),
    /// ⚔️ The write contradicts what is already stored — 409.
    Conflict(String),
    /// 🚧️ The request itself is malformed — 400.
    BadRequest(String),
    /// 🔥️ The server failed to answer at all — 500.
    Internal(String),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthorized(detail) => write!(formatter, "unauthorized: {detail}"),
            Self::Forbidden(detail) => write!(formatter, "forbidden: {detail}"),
            Self::NotFound(detail) => write!(formatter, "not found: {detail}"),
            Self::Conflict(detail) => write!(formatter, "conflict: {detail}"),
            Self::BadRequest(detail) => write!(formatter, "bad request: {detail}"),
            Self::Internal(detail) => write!(formatter, "internal error: {detail}"),
        }
    }
}

impl std::error::Error for ServerError {}

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

//#region 🔖️Instance
/// 🪪️ One deployment of this product — hub, zentrale, a test profile — expressed as a **type**.
///
/// Every extension point of the server is an associated type here, and every one of them is chosen
/// by the deployment rather than by the framework. That is the whole design: a set of
/// implementations must be closed at the site that owns the implementations, so a downstream crate
/// names its own module set, its own deciders, its own storage backends, and — when it really has
/// several of one kind — closes them into one enum in its own crate with `dyn_enum_close!`, which
/// writes that enum for every port of this product, whether it is declared with plain `async fn` or
/// with `impl Future<..> + Send` (the macro delegates the latter as an `async fn` over the future's
/// `Output`, since one `match` cannot return two different opaque futures).
/// Closing those sets inside this crate instead would force this crate to name hub's types, which
/// inverts the dependency the product exists to avoid — and it is what made the document port
/// unusable before: an empty enum cannot be implemented from outside, so the document websocket
/// answered 404 unconditionally and the presence code behind it was unreachable.
///
/// Nothing in this crate implements `ServerInstance`. The reference profile that exercises it lives
/// with the tests (`🧪️tests/🧩️instance/🦀️.rs`), because a demo decider and an in-memory store are
/// fixtures, not product surface.
pub trait ServerInstance: Sized + Send + Sync + 'static {
    /// 🧩️ The module set this deployment is assembled from.
    type Modules: ServerModule<Instance = Self> + 'static;
    /// ❓️ The read handlers it answers queries with; [`NoQueryHandler`] when it has none yet.
    type Queries: QueryHandler + 'static;
    /// 📄️ The replication engine behind its document websocket; [`NoDocumentAuthority`] when it
    /// hosts none.
    type Documents: DocumentAuthority + 'static;
    /// ⚖️ The deterministic cores its actors are decided by.
    type Deciders: Decider + 'static;
    /// 🧵️ The cross-actor workflows it runs off the outbox.
    type Sagas: Saga + 'static;
    /// 🪜️ The rungs of its authentication ladder.
    type Resolvers: PrincipalResolver + 'static;
    /// 🏛️ The backend holding its authoritative history.
    type AuthorityStore: AuthorityStore + 'static;
    /// 🔭️ The backend holding its rebuildable read models.
    type ProjectionStore: ProjectionStore + 'static;
    /// 🧱️ The backend holding its content-addressed bytes.
    type BlobStore: BlobStore + 'static;
    /// 🎫️ The backend holding its live authentication state.
    type SessionStore: SessionStore + 'static;

    /// 🗄️ Open the four durable roles for this deployment shape. The instance decides what
    /// [`StorageProfile::Embedded`]'s `data_dir` means — a directory to open, or nothing at all for
    /// a profile that keeps everything in memory — because the framework has no backend to impose.
    async fn open(profile: &StorageProfile) -> Result<InstanceStores<Self>, StorageError>;
}

/// 🗄️ The four durable roles of one instance, opened together so a half-open server is not a state
/// [`ServerBuilder::build`] has to handle.
pub struct InstanceStores<I: ServerInstance> {
    pub authority: I::AuthorityStore,
    pub projections: I::ProjectionStore,
    pub blobs: I::BlobStore,
    pub sessions: I::SessionStore,
}

/// 🧵️ The saga runner of one instance, over the workflows that instance declared.
pub type ServerSagas<I> = SagaRunner<<I as ServerInstance>::Sagas>;
//#endregion 🔖️Instance

//#region 🔖️Module
/// 🧩️ The runtime half of a server module.
///
/// The declarative half is [`ModuleManifest`], which is pure data and lives in the contract so an
/// instance definition can be inspected, diffed and served without ever constructing a server. This
/// trait is the half that cannot be data: [`routes`](Self::routes) hands out a live
/// `Router<`[`ServerState`]`>` and therefore names the transport library, which is exactly why it
/// is declared here in layer 3 and not next to the manifest in layer 1. Keeping the two halves
/// apart is what lets a client, a CLI or a documentation generator read a manifest without axum.
///
/// A module belongs to exactly one [`ServerInstance`], named by [`Instance`](Self::Instance): its
/// routes take that instance's [`ServerState`], its deciders and rungs are that instance's decider
/// and resolver types. That binding is also why this trait is not `#[dyn_enum]`'d — the macro
/// refuses an associated type, because an enum has no single type to give it — so an instance with
/// several modules writes the delegating enum by hand, or gives each module its own server.
pub trait ServerModule: Send + Sync {
    /// 🪪️ The deployment this module is part of.
    type Instance: ServerInstance;

    /// 📇️ What this module declares to the instance registering it.
    async fn manifest(&self) -> ModuleManifest;

    /// ⚖️ The deciders this module registers on the command bus, one per actor kind it serves.
    async fn deciders(&self) -> Vec<<Self::Instance as ServerInstance>::Deciders> {
        Vec::new()
    }

    /// 🧵️ The cross-actor workflows this module reacts with, appended to the instance's one saga
    /// runner in module registration order. A module owns its workflows for the same reason it owns
    /// its deciders: the subsystem that emits an event is the subsystem that knows what must follow.
    async fn sagas(&self) -> Vec<<Self::Instance as ServerInstance>::Sagas> {
        Vec::new()
    }

    /// 🛣️ The routes this module mounts. Called once at build time with the router under
    /// construction; a module that serves no HTTP surface returns it untouched.
    async fn routes(&self, router: Router<ServerState<Self::Instance>>) -> Router<ServerState<Self::Instance>> {
        router
    }

    /// 🪜️ The authentication rungs this module contributes, appended to the shared ladder in
    /// module registration order.
    async fn resolvers(&self) -> Vec<<Self::Instance as ServerInstance>::Resolvers> {
        Vec::new()
    }

    /// 🎓️ The role definitions this module registers into the shared policy engine.
    async fn templates(&self) -> Vec<PolicyTemplate> {
        Vec::new()
    }

    /// 🧍️ Admit or refuse one state a presence session of `scope` shares. Every module is asked in
    /// registration order and the first refusal is sent back as a `refused` frame; the default
    /// admits, so an instance without presence rules shares any state. Synchronous on purpose: it
    /// runs inside every socket's receive loop and must neither suspend nor reach anything but the
    /// state it judges.
    fn presence_admission(&self, _scope: &Scope, _state: &OpaqueJson) -> Result<(), String> {
        Ok(())
    }
}

//#endregion 🔖️Module

//#region 🔖️DocumentPort
/// 📄️ The port a replication engine plugs into, and the only thing this product knows about one.
///
/// The server product deliberately depends on no document engine: not on the os product, not on
/// `db`, not on any concrete CRDT or OT implementation. The gateway can therefore bridge a document
/// websocket — handshake, submit, relay — while naming nothing but opaque byte frames. An instance
/// may supply an implementation, or none at all, in which case the gateway does not mount the
/// document route and the instance is free to own `/scopes/{scope}/document/ws` itself (hub does,
/// because its socket carries grant admission, presence leases and live revocation).
/// **Send futures, declared not inferred.** Every method of this port returns
/// `impl Future<..> + Send` instead of being written `async fn`, and that is structural, not a
/// style choice: [`ServerState`] reaches this port behind an
/// instance's associated type, so the concrete future is opaque at the call site and axum's
/// handler and socket tasks — which are `Send` by construction — cannot otherwise prove it may
/// cross a thread. An `async fn` here compiles and then fails at every route that uses it. The
/// implementations stay ordinary `async fn`, which Rust accepts against this signature, and so does
/// the delegate `dyn_enum_close!` generates for a set of them: the macro emits `async fn .. -> T`
/// over the future's `Output`, because two match arms cannot unify two distinct opaque futures.
/// 🤝️ When the gateway runs [`DocumentAuthority::welcome`] relative to the first client frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DocumentHandshake {
    /// 👋️ Send welcome before reading any client frame.
    #[default]
    ServerFirst,
    /// 👂️ Wait for one client binary frame, then call welcome with that frame as `hello`.
    ClientFirst,
}

/// 🎓️ Socket identity derived server-side from the authenticated principal — never from the URL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentSocketIdentity {
    /// 🙋️ Hub-issued document actor id envelopes must be authored as.
    pub actor: String,
    /// 🎫️ Presence / kick session id for this socket.
    pub session: String,
}

/// 🧭️ Bind a document socket to the principal's hub-issued actor. Callers never supply actor ids.
pub fn document_socket_identity(resolved: &Resolved) -> Result<DocumentSocketIdentity, ServerError> {
    let actor = match &resolved.actor {
        Some(actor) if !actor.is_empty() => actor.clone(),
        _ => match &resolved.principal {
            Principal::User { id } | Principal::ServiceAccount { id } | Principal::Device { id } if !id.is_empty() => id.clone(),
            Principal::Anonymous | Principal::User { .. } | Principal::ServiceAccount { .. } | Principal::Device { .. } => {
                return Err(ServerError::Unauthorized("document socket requires an authenticated principal with an actor grant".into()));
            }
        },
    };
    let session = resolved.session.as_ref().map(|session| session.0.clone()).filter(|session| !session.is_empty()).unwrap_or_else(|| actor.clone());
    Ok(DocumentSocketIdentity { actor, session })
}

#[dyn_enum]
pub trait DocumentAuthority: Send + Sync {
    /// 🤝️ Whether this engine greets first or waits for a client hello frame.
    fn handshake(&self) -> impl Future<Output = DocumentHandshake> + Send;

    /// 🧭️ Resolve the socket actor for this principal on `scope`, or refuse when the principal holds
    /// no grant. The default binds from [`Resolved::actor`] / the principal id; hub overrides to
    /// require a hub-issued actor grant.
    fn bind_socket(&self, scope: &Scope, resolved: &Resolved) -> impl Future<Output = Result<DocumentSocketIdentity, ServerError>> + Send;

    /// 👋️ The handshake frames for a joining actor. `resume` is the engine's own resumption token;
    /// `hello` is the first client binary frame when [`DocumentHandshake::ClientFirst`], else `None`.
    /// Returning several frames lets an engine stream welcome plus bootstrap chunks as distinct WS
    /// messages — the gateway never concatenates them.
    fn welcome(&self, scope: &Scope, actor: &str, resume: Option<&str>, hello: Option<&[u8]>) -> impl Future<Output = Result<Vec<Vec<u8>>, ServerError>> + Send;

    /// 📨️ Apply one client frame and return the frames for the submitter and for peer relay.
    /// `actor` is the socket's server-bound hub actor id (from [`DocumentAuthority::bind_socket`]),
    /// not a caller-supplied query value — envelopes are authored as that socket actor.
    fn submit_frame(&self, scope: &Scope, actor: &str, principal: &Principal, frame: &[u8]) -> impl Future<Output = Result<DocumentFrames, ServerError>> + Send;
}

/// 📤️ Frames produced by one [`DocumentAuthority::submit_frame`] call.
///
/// Split on purpose: an ack belongs only to the submitter, while the accepted command batch must
/// reach every other session on the document. Engines that have nothing to hide from the submitter
/// put the same bytes in both fields.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocumentFrames {
    /// 🪞 Frames returned only to the socket that submitted.
    pub echo: Vec<Vec<u8>>,
    /// 📣 Frames relayed to every other session on the document lane.
    pub relay: Vec<Vec<u8>>,
}

impl DocumentFrames {
    /// 🪞📣 The same frames for the submitter and for every peer.
    pub fn mirrored(frames: Vec<Vec<u8>>) -> Self {
        Self { echo: frames.clone(), relay: frames }
    }
}

/// 🕳️ The document authority of an instance that hosts none.
///
/// Uninhabited on purpose, and the only honest way to say "no replication engine here": naming it
/// as [`ServerInstance::Documents`] makes `Option<Arc<Self>>` permanently `None`, so the gateway
/// never mounts `/scopes/{scope}/document/ws`. An instance that *does* have an engine names that
/// engine instead and registers it with [`ServerBuilder::document_authority`], which mounts the route.
pub enum NoDocumentAuthority {}

impl DocumentAuthority for NoDocumentAuthority {
    async fn handshake(&self) -> DocumentHandshake {
        match *self {}
    }

    async fn bind_socket(&self, _scope: &Scope, _resolved: &Resolved) -> Result<DocumentSocketIdentity, ServerError> {
        match *self {}
    }

    async fn welcome(&self, _scope: &Scope, _actor: &str, _resume: Option<&str>, _hello: Option<&[u8]>) -> Result<Vec<Vec<u8>>, ServerError> {
        match *self {}
    }

    async fn submit_frame(&self, _scope: &Scope, _actor: &str, _principal: &Principal, _frame: &[u8]) -> Result<DocumentFrames, ServerError> {
        match *self {}
    }
}
//#endregion 🔖️DocumentPort

//#region 🔖️Query
/// ❓️ One registered read. A query never touches an actor's private state — it answers from a
/// [`ProjectionStore`], which is why the handler is handed nothing else.
/// The projection store is taken as a type parameter rather than as one instance's concrete
/// backend, so a handler keeps the property that makes it a query: it reads through
/// [`ProjectionStore`] and can reach nothing else.
/// **Send futures, declared not inferred.** Every method of this port returns
/// `impl Future<..> + Send` instead of being written `async fn`, and that is structural, not a
/// style choice: [`ServerState`] reaches this port behind an
/// instance's associated type, so the concrete future is opaque at the call site and axum's
/// handler and socket tasks — which are `Send` by construction — cannot otherwise prove it may
/// cross a thread. An `async fn` here compiles and then fails at every route that uses it. The
/// implementations stay ordinary `async fn`, which Rust accepts against this signature, and so does
/// the delegate `dyn_enum_close!` generates for a set of them: the macro emits `async fn .. -> T`
/// over the future's `Output`, because two match arms cannot unify two distinct opaque futures.
#[dyn_enum]
pub trait QueryHandler: Send + Sync {
    /// 🏷️ The [`QueryEnvelope::kind`] this handler answers.
    fn kind(&self) -> impl Future<Output = &str> + Send;

    /// 📤️ Answer one query against the read models.
    fn handle<P: ProjectionStore>(&self, envelope: &QueryEnvelope, projections: &P) -> impl Future<Output = Result<QueryResult, ServerError>> + Send;
}

/// 🕳️ The query set of an instance that registers no read handler. Uninhabited, so `queries` stays
/// empty and `POST /queries` answers [`ServerError::NotFound`] for every kind.
pub enum NoQueryHandler {}

impl QueryHandler for NoQueryHandler {
    async fn kind(&self) -> &str {
        match *self {}
    }

    async fn handle<P: ProjectionStore>(&self, _envelope: &QueryEnvelope, _projections: &P) -> Result<QueryResult, ServerError> {
        match *self {}
    }
}
//#endregion 🔖️Query

//#region 🔖️Fanout
/// 📻️ How many frames a lane buffers before a slow subscriber is lagged out of them.
const FANOUT_CAPACITY: usize = 256;

/// 📡️ Per-lane broadcast fan-out. One [`broadcast`] sender per lane key, created on the first
/// subscribe and dropped when the last subscriber goes, so an idle instance holds no lanes at all.
#[derive(Clone, Default)]
pub struct Fanout {
    channels: Arc<ShardedMap<String, broadcast::Sender<Vec<u8>>>>,
}

impl Fanout {
    /// 🌱️ A registry holding no lanes.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🔔️ Subscribe to `lane`, creating it if nobody holds it yet. The returned [`Subscription`] is
    /// the lane's lifetime: dropping the last one removes the sender.
    pub fn subscribe(&self, lane: &str) -> Subscription {
        let sender = self.channels.get_or_insert_with_cloned(lane.to_string(), || broadcast::channel(FANOUT_CAPACITY).0);
        Subscription { lane: lane.to_string(), channels: Arc::clone(&self.channels), receiver: sender.subscribe() }
    }

    /// 📢️ Publish `bytes` on `lane` and report how many subscribers received it. Publishing to a
    /// lane nobody holds is a no-op — a fan-out never creates a lane, only a subscriber does.
    pub fn publish(&self, lane: &str, bytes: Vec<u8>) -> usize {
        match self.channels.get_cloned(lane) {
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
    channels: Arc<ShardedMap<String, broadcast::Sender<Vec<u8>>>>,
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

    /// 📬️ The next frame, or the fact that frames were missed, or the end of the lane — for a
    /// subscriber whose protocol is a delta stream and must resynchronize rather than silently skip.
    pub async fn next(&mut self) -> Delivery {
        match self.receiver.recv().await {
            Ok(bytes) => Delivery::Frame(bytes),
            Err(broadcast::error::RecvError::Lagged(_)) => Delivery::Lagged,
            Err(broadcast::error::RecvError::Closed) => Delivery::Closed,
        }
    }

    /// 🏷️ The lane this subscription listens on.
    pub fn lane(&self) -> &str {
        &self.lane
    }
}

/// 📬️ What [`Subscription::next`] delivers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Delivery {
    /// 📦️ One published frame.
    Frame(Vec<u8>),
    /// ⏭️ The subscriber fell behind and frames were dropped for it.
    Lagged,
    /// 🔚️ The lane has no sender any more.
    Closed,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        self.channels.remove_if(&self.lane, |sender| sender.receiver_count() <= 1);
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

/// 🧍️ The presence lane key of one scope: its room, its colour namespace and its batch lane.
pub fn presence_lane(scope: &Scope) -> String {
    format!("presence:{}", scope.0)
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
    colours: ShardedMap<String, ScopeColours>,
    sessions: ShardedMap<(String, String), PresenceSession>,
}

impl Presence {
    /// 🐣️ An empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🎨️ The lowest palette slot not currently held in `scope`, or the actor's existing slot with
    /// one more reference on it. Wraps once all 256 slots are taken.
    pub fn acquire_colour(&self, scope: &str, actor: &str) -> u8 {
        self.colours.mutate_or_default(scope.to_string(), |colours| {
            if let Some(lease) = colours.by_actor.get_mut(actor) {
                lease.refs += 1;
                return lease.index;
            }
            let taken: Vec<u8> = colours.by_actor.values().map(|lease| lease.index).collect();
            let index = (0..=u8::MAX).find(|candidate| !taken.contains(candidate)).unwrap_or((colours.by_actor.len() % 256) as u8);
            colours.by_actor.insert(actor.to_string(), ColourLease { index, refs: 1 });
            index
        })
    }

    /// 🧹️ Drop one reference on the actor's slot, freeing it on the last disconnect.
    pub fn release_colour(&self, scope: &str, actor: &str) {
        self.colours.with_mut(scope, |colours| {
            let Some(colours) = colours else { return };
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
        });
    }

    /// 🔎️ The slot this actor currently holds in `scope`, if any.
    pub fn colour_of(&self, scope: &str, actor: &str) -> Option<u8> {
        self.colours.with(scope, |colours| colours.and_then(|colours| colours.by_actor.get(actor).map(|lease| lease.index)))
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
        self.sessions.with_mut(&(scope.to_string(), actor.to_string()), |session| {
            if let Some(session) = session {
                session.peer = Some(peer);
            }
        });
    }

    /// 🚶️ Remove the session and release its colour.
    pub fn leave(&self, scope: &str, actor: &str) {
        self.sessions.remove(&(scope.to_string(), actor.to_string()));
        self.release_colour(scope, actor);
    }

    /// 📋️ Everyone currently present in `scope`, ordered by actor.
    pub fn roster(&self, scope: &str) -> Vec<PresenceSession> {
        let mut roster = Vec::new();
        self.sessions.for_each(|(entry_scope, _), session| {
            if entry_scope == scope {
                roster.push(session.clone());
            }
        });
        roster.sort_by(|left, right| left.actor.cmp(&right.actor));
        roster
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceRoom
/// 🧍️ The resource presence admission is evaluated on, at [`PolicyPoint::Subscription`] with the
/// actions [`PRESENCE_JOIN`] and [`PRESENCE_PUBLISH`].
pub const PRESENCE_RESOURCE: &str = "presence";

/// 🚪️ The action admitting a principal into a presence room.
pub const PRESENCE_JOIN: &str = "join";

/// 📣️ The action admitting a principal to share a state in a presence room.
pub const PRESENCE_PUBLISH: &str = "publish";

/// 👀️ The action admitting a principal to watch a presence room it has not joined, read-only.
pub const PRESENCE_WATCH: &str = "watch";

/// 🔌️ The websocket subprotocol of the presence socket.
pub const PRESENCE_PROTOCOL_V1: &str = "semio.presence.v1";

/// 🚫️ The `refused` reason of a client frame larger than [`PresenceSettings::max_state_bytes`].
pub const REFUSED_TOO_LARGE: &str = "state-too-large";

/// 🚫️ The `refused` reason of a frame that is neither a `state` nor a `watch` frame.
pub const REFUSED_INVALID: &str = "frame-invalid";

/// 🚫️ The `refused` reason of a state from a principal policy does not let publish, and — followed
/// by the scope — of a watch naming a scope policy does not let it watch.
pub const REFUSED_FORBIDDEN: &str = "forbidden";

/// 🚫️ The `refused` reason of a watch naming more than [`PresenceSettings::max_watch_scopes`] scopes.
pub const REFUSED_WATCH_TOO_MANY: &str = "watch-too-many";

/// ⏱️ The knobs of the presence socket. The defaults are the protocol's (design §15).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PresenceSettings {
    /// 🕰️ How often a room publishes its coalesced `batch`.
    pub tick: Duration,
    /// 💤️ How long a socket may stay silent (no frame, no pong) before it is closed.
    pub idle: Duration,
    /// 🏓️ How often the server pings every socket.
    pub keepalive: Duration,
    /// 📏️ The largest client frame (`state` or `watch`) accepted, in bytes.
    pub max_state_bytes: usize,
    /// 🚦️ How many client frames per second a session may send; the excess is dropped.
    pub max_states_per_second: u32,
    /// 🏷️ The longest `surface` a session may join with, in characters.
    pub max_surface_chars: usize,
    /// 👀️ The most scopes one `watch` may name.
    pub max_watch_scopes: usize,
    /// 🐢️ The slowest interval a watcher may ask for; the fastest is [`tick`](Self::tick).
    pub max_watch_interval: Duration,
}

impl Default for PresenceSettings {
    fn default() -> Self {
        Self {
            tick: Duration::from_millis(100),
            idle: Duration::from_secs(60),
            keepalive: Duration::from_secs(20),
            max_state_bytes: 2048,
            max_states_per_second: 30,
            max_surface_chars: 64,
            max_watch_scopes: 16,
            max_watch_interval: Duration::from_secs(60),
        }
    }
}

/// 🌍️ The instance's answer to "may a browser page served from this origin open a socket here".
/// Checked for every presence socket that presents an `Origin`; an instance that sets none admits
/// every origin, which is right only where nothing but loopback can reach the process.
pub type OriginAdmission = Arc<dyn Fn(&str) -> bool + Send + Sync>;

/// 🧍️ One member of a presence room.
#[derive(Clone, Debug, PartialEq)]
struct Member {
    colour: u8,
    surface: String,
    state: OpaqueJson,
}

/// 🏠️ One room: its members, what changed since the last tick and who left.
#[derive(Debug, Default)]
struct Room {
    members: BTreeMap<String, Member>,
    changed: BTreeSet<String>,
    left: BTreeSet<String>,
    ticking: bool,
}

/// 🕰️ What one tick of a room produced.
#[derive(Clone, Debug, PartialEq)]
pub enum Drained {
    /// 📦️ The coalesced changes since the last tick.
    Batch(PresenceFrame),
    /// 🤫️ Nothing changed; the room keeps ticking.
    Quiet,
    /// 🔚️ The room is empty and has nothing left to announce; its ticker ends.
    Finished,
}

/// 👥️ Every presence room of the instance, keyed by presence lane. Latest-state: a room keeps
/// each session's newest state only, so however often a session sends, a tick carries it once.
#[derive(Default)]
pub struct PresenceRooms {
    rooms: ShardedMap<String, Room>,
}

impl PresenceRooms {
    /// 🌱️ No rooms.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🚪️ Add a session to a room and answer the roster (the session included) and whether the
    /// room needs a ticker started — true exactly when none is running.
    pub fn join(&self, lane: &str, session: &str, colour: u8, surface: &str) -> (Vec<PresenceEntry>, bool) {
        self.rooms.mutate_or_default(lane.to_string(), |room| {
            room.members.insert(session.to_string(), Member { colour, surface: surface.to_string(), state: OpaqueJson::Null });
            room.changed.insert(session.to_string());
            room.left.remove(session);
            let start = !room.ticking;
            room.ticking = true;
            (entries(room.members.iter()), start)
        })
    }

    /// ✏️ Replace a member's state; it is published with the next tick.
    pub fn update(&self, lane: &str, session: &str, state: OpaqueJson) {
        self.rooms.with_mut(lane, |room| {
            if let Some(room) = room {
                if let Some(member) = room.members.get_mut(session) {
                    member.state = state;
                    room.changed.insert(session.to_string());
                }
            }
        });
    }

    /// 🚶️ Remove a session; its departure is published with the next tick.
    pub fn leave(&self, lane: &str, session: &str) {
        self.rooms.with_mut(lane, |room| {
            if let Some(room) = room {
                if room.members.remove(session).is_some() {
                    room.changed.remove(session);
                    room.left.insert(session.to_string());
                }
            }
        });
    }

    /// 📋️ Every member of a room, ordered by session.
    pub fn roster(&self, lane: &str) -> Vec<PresenceEntry> {
        self.rooms.with(lane, |room| room.map(|room| entries(room.members.iter())).unwrap_or_default())
    }

    /// 🕰️ Take what changed since the last tick. An empty room with nothing to announce stops
    /// ticking and is dropped, atomically with respect to a concurrent join (which then starts a new
    /// ticker).
    pub fn drain(&self, lane: &str) -> Drained {
        let drained = self.rooms.with_mut(lane, |room| {
            let Some(room) = room else { return Drained::Finished };
            if room.changed.is_empty() && room.left.is_empty() {
                if room.members.is_empty() {
                    room.ticking = false;
                    return Drained::Finished;
                }
                return Drained::Quiet;
            }
            let changed = std::mem::take(&mut room.changed);
            let left = std::mem::take(&mut room.left).into_iter().collect();
            Drained::Batch(PresenceFrame::Batch { entries: entries(room.members.iter().filter(|(session, _)| changed.contains(*session))), left })
        });
        if drained == Drained::Finished {
            self.rooms.remove_if(lane, |room| !room.ticking && room.members.is_empty());
        }
        drained
    }
}

fn entries<'a>(members: impl Iterator<Item = (&'a String, &'a Member)>) -> Vec<PresenceEntry> {
    members.map(|(session, member)| PresenceEntry { session: session.clone(), colour: member.colour, surface: member.surface.clone(), state: member.state.clone() }).collect()
}

/// 🚦️ A session's state-frame allowance: at most `limit` frames per one-second window.
#[derive(Clone, Copy, Debug)]
pub struct StateBudget {
    window: Instant,
    spent: u32,
    limit: u32,
}

impl StateBudget {
    /// 🌱️ A fresh allowance of `limit` frames per second.
    pub fn new(limit: u32) -> Self {
        Self { window: Instant::now(), spent: 0, limit }
    }

    /// ✅️ Spend one frame at `now`; `false` when the window's allowance is exhausted.
    pub fn spend(&mut self, now: Instant) -> bool {
        if now.duration_since(self.window) >= Duration::from_secs(1) {
            self.window = now;
            self.spent = 0;
        }
        self.spent += 1;
        self.spent <= self.limit
    }
}

/// 🛃️ Judge one client text frame: `Ok(Some(frame))` — a `state` or a `watch` — to apply,
/// `Ok(None)` to drop silently (over the rate), `Err(reason)` to answer with a `refused` frame.
pub fn admit_frame(text: &str, settings: &PresenceSettings, budget: &mut StateBudget, now: Instant) -> Result<Option<PresenceFrame>, String> {
    if text.len() > settings.max_state_bytes {
        return Err(REFUSED_TOO_LARGE.to_string());
    }
    if !budget.spend(now) {
        return Ok(None);
    }
    match serde_json::from_str::<PresenceFrame>(text) {
        Ok(frame @ (PresenceFrame::State { .. } | PresenceFrame::Watch { .. })) => Ok(Some(frame)),
        _ => Err(REFUSED_INVALID.to_string()),
    }
}

/// 🐢️ The interval a watcher asked for, clamped between [`PresenceSettings::tick`] (a room never
/// changes faster) and [`PresenceSettings::max_watch_interval`].
pub fn watch_interval(interval_ms: u64, settings: &PresenceSettings) -> Duration {
    Duration::from_millis(interval_ms).min(settings.max_watch_interval).max(settings.tick)
}

/// 🗒️ What changed in one watched scope since the watcher's last interval.
#[derive(Debug, Default)]
struct Pending {
    entries: BTreeMap<String, PresenceEntry>,
    left: BTreeSet<String>,
}

/// 👀️ One socket's watch set: per watched scope, what its room announced since the watcher's last
/// interval — the latest entry per session and the sessions that left — until the interval flushes
/// it as one `watched` frame per changed scope. However often a watched room ticks, a watcher
/// receives each scope at most once per interval.
#[derive(Debug, Default)]
pub struct Watching {
    scopes: BTreeMap<String, Pending>,
}

impl Watching {
    /// 🔁️ Replace the watch set with `scopes` and answer the scopes it newly contains, in order. A
    /// scope that stays watched keeps its pending changes; a dropped scope loses them.
    pub fn watch(&mut self, scopes: &BTreeSet<String>) -> Vec<String> {
        self.scopes.retain(|scope, _| scopes.contains(scope));
        let added: Vec<String> = scopes.iter().filter(|scope| !self.scopes.contains_key(*scope)).cloned().collect();
        for scope in &added {
            self.scopes.insert(scope.clone(), Pending::default());
        }
        added
    }

    /// 👁️ Whether `scope` is watched.
    pub fn watches(&self, scope: &str) -> bool {
        self.scopes.contains_key(scope)
    }

    /// 🫙️ Whether no scope is watched.
    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }

    /// 🧺️ Fold one room batch into a watched scope's pending changes; a scope no longer watched
    /// ignores it.
    pub fn absorb(&mut self, scope: &str, entries: Vec<PresenceEntry>, left: Vec<String>) {
        let Some(pending) = self.scopes.get_mut(scope) else { return };
        for entry in entries {
            pending.left.remove(&entry.session);
            pending.entries.insert(entry.session.clone(), entry);
        }
        for session in left {
            pending.entries.remove(&session);
            pending.left.insert(session);
        }
    }

    /// 📸️ The `snapshot` of a watched scope from its room's roster, discarding the scope's pending
    /// changes, which the roster already holds; `None` for a scope no longer watched.
    pub fn snapshot(&mut self, scope: &str, roster: Vec<PresenceEntry>) -> Option<PresenceFrame> {
        let pending = self.scopes.get_mut(scope)?;
        *pending = Pending::default();
        Some(PresenceFrame::Watched { scope: Scope(scope.to_string()), entries: roster, left: Vec::new(), snapshot: true })
    }

    /// 🕰️ One `watched` frame per scope that changed since the last flush, in scope order.
    pub fn flush(&mut self) -> Vec<PresenceFrame> {
        self.scopes
            .iter_mut()
            .filter(|(_, pending)| !pending.entries.is_empty() || !pending.left.is_empty())
            .map(|(scope, pending)| {
                let Pending { entries, left } = std::mem::take(pending);
                PresenceFrame::Watched { scope: Scope(scope.clone()), entries: entries.into_values().collect(), left: left.into_iter().collect(), snapshot: false }
            })
            .collect()
    }
}

/// 🎲️ A fresh presence session id: 128 bits from the process's randomly keyed hasher over a
/// counter and the clock, as 32 lowercase hex characters. Public, never a credential.
pub fn presence_session_id() -> String {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let keys = std::collections::hash_map::RandomState::new();
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    let count = NEXT.fetch_add(1, Ordering::Relaxed);
    let half = |salt: u64| {
        let mut hasher = keys.build_hasher();
        hasher.write_u64(salt);
        hasher.write_u64(count);
        hasher.write_u128(nanos);
        hasher.finish()
    };
    format!("{:016x}{:016x}", half(0x7072_6573_656e_6365), half(0x7365_7373_696f_6e73))
}
//#endregion 🔖️PresenceRoom

//#region 🔖️Kick
/// 🦵️ Per-session close signals. The administration plane fires one; the socket loop owning that
/// session observes it and closes itself. Nothing here ever touches a socket.
#[derive(Default)]
pub struct KickMap {
    signals: ShardedMap<String, Arc<Notify>>,
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
        self.signals.get_or_insert_with_cloned(session.to_string(), || Arc::new(Notify::new()))
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

/// 🛂️ WebSocket subprotocol that carries a session bearer after this marker.
pub const SESSION_PROTOCOL_V1: &str = "semio.session.v1";


/// 🎟️ The bearer token of an `Authorization: Bearer …` header, if there is one.
pub fn bearer(headers: &HeaderMap) -> Option<String> {
    headers.get(header::AUTHORIZATION).and_then(|value| value.to_str().ok()).and_then(|value| value.strip_prefix("Bearer ")).map(|value| value.to_string())
}

/// 🪪️ Normalize what a caller presented into a transport-free [`Credential`]. `loopback` comes from
/// the peer address and never from a header — it is a fact only the transport can establish, and a
/// 🎟️ Session bearer offered as `Sec-WebSocket-Protocol: semio.session.v1, <token>` when the
/// browser cannot set an `Authorization` header on the upgrade.
pub fn session_protocol_bearer(headers: &HeaderMap) -> Option<String> {
    if headers.contains_key(header::AUTHORIZATION) {
        return None;
    }
    let values = headers.get_all(header::SEC_WEBSOCKET_PROTOCOL);
    if values.iter().count() != 1 {
        return None;
    }
    let offered = values.iter().next().and_then(|value| value.to_str().ok())?;
    let (protocol, token) = offered.split_once(", ")?;
    if protocol != SESSION_PROTOCOL_V1 || token.is_empty() || token.contains(',') {
        return None;
    }
    Some(token.to_string())
}


/// header claiming it would be a header granting itself the administration plane.
pub fn credential(headers: &HeaderMap, peer: Option<SocketAddr>) -> Credential {
    Credential { bearer: bearer(headers).or_else(|| session_protocol_bearer(headers)), capability: headers.get(CAPABILITY_HEADER).and_then(|value| value.to_str().ok()).map(|value| crate::contract::CapabilityProof(value.to_string())), loopback: peer.is_some_and(|peer| peer.ip().is_loopback()) }
}
//#endregion 🔖️Credential

//#region 🔖️Store
/// 🏛️ The bus shape this gateway serializes every command through: the instance's own authority
/// store and its own decider set, both known to the compiler at every call site — no boxed trait
/// object, and no enum this crate had to invent on the instance's behalf.
pub type ServerAuthority<I> = CommandBus<<I as ServerInstance>::AuthorityStore, <I as ServerInstance>::Deciders>;
//#endregion 🔖️Store

//#region 🔖️State
/// 🧠️ Everything a handler may reach, for one [`ServerInstance`]. Cloneable and cheap: every field
/// is an [`Arc`], so the whole value is a bundle of handles rather than a bundle of data.
pub struct ServerState<I: ServerInstance> {
    /// 🏛️ The command bus, serialized behind a mutex because a turn is by definition one at a time.
    pub authority: Arc<Mutex<ServerAuthority<I>>>,
    /// 🧵️ The cross-actor workflows this instance reacts with, over the outbox the bus commits into.
    /// Held here rather than by whoever calls [`ServerBuilder::build`] because the outbox and the
    /// workflows that drain it are two halves of one exactly-once guarantee: a runner living
    /// somewhere else is a runner that can be forgotten, and a forgotten runner is a queue that
    /// grows forever while every event looks delivered.
    pub sagas: Arc<Mutex<ServerSagas<I>>>,
    /// 🔭️ The rebuildable read models every query answers from.
    pub projections: Arc<Mutex<I::ProjectionStore>>,
    /// 🧱️ Content-addressed bytes.
    pub blobs: Arc<Mutex<I::BlobStore>>,
    /// 🎫️ Live authentication state.
    pub sessions: Arc<Mutex<I::SessionStore>>,
    /// ⚖️ Roles as data. A standard lock rather than an async one because the command bus's own
    /// admission hook is synchronous and must consult it inside a turn.
    pub policy: Arc<RwLock<PolicyEngine>>,
    /// 🪜️ The authentication ladder, fixed at build time.
    pub resolvers: Arc<ResolverChain<I::Resolvers>>,
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
    pub queries: Arc<ShardedMap<String, Arc<I::Queries>>>,
    /// 📄️ The replication engine, when the instance supplied one.
    pub documents: Option<Arc<I::Documents>>,
    /// 🏗️ The deployment shape this instance's storage was opened in — the profile itself rather
    /// than a path, because [`StorageProfile::Ephemeral`] owns no directory and a handler that
    /// asked for one would have been handed a fabricated empty path.
    pub profile: Arc<StorageProfile>,
    /// 🧩️ The modules this instance was built from, asked at runtime by the hooks that need them
    /// (presence admission).
    pub modules: Arc<Vec<I::Modules>>,
    /// 👥️ The presence rooms of the presence socket.
    pub presence_rooms: Arc<PresenceRooms>,
    /// ⏱️ The presence socket's knobs.
    pub presence_settings: Arc<PresenceSettings>,
    /// 🌍️ Which browser origins may open a presence socket; `None` admits every origin.
    pub origin_admission: Option<OriginAdmission>,
    /// 🕰️ The instance's hybrid logical clock, advanced once per stamped command.
    clock: Arc<StdMutex<HybridLogicalClock>>,
}

impl<I: ServerInstance> Clone for ServerState<I> {
    /// 🧬️ Handle-by-handle, never field-by-field through `derive(Clone)`: the derive would demand
    /// `I: Clone` of the instance MARKER type, which is not a value anybody clones.
    fn clone(&self) -> Self {
        Self {
            authority: Arc::clone(&self.authority),
            sagas: Arc::clone(&self.sagas),
            projections: Arc::clone(&self.projections),
            blobs: Arc::clone(&self.blobs),
            sessions: Arc::clone(&self.sessions),
            policy: Arc::clone(&self.policy),
            resolvers: Arc::clone(&self.resolvers),
            admin: Arc::clone(&self.admin),
            fanout: Arc::clone(&self.fanout),
            presence: Arc::clone(&self.presence),
            kicks: Arc::clone(&self.kicks),
            apps: Arc::clone(&self.apps),
            queries: Arc::clone(&self.queries),
            documents: self.documents.clone(),
            profile: Arc::clone(&self.profile),
            modules: Arc::clone(&self.modules),
            presence_rooms: Arc::clone(&self.presence_rooms),
            presence_settings: Arc::clone(&self.presence_settings),
            origin_admission: self.origin_admission.clone(),
            clock: Arc::clone(&self.clock),
        }
    }
}

impl<I: ServerInstance> ServerState<I> {
    /// 🕰️ The next clock reading: wall-clock milliseconds, with the counter breaking ties so two
    /// commands stamped inside the same millisecond still order.
    pub fn now(&self) -> HybridLogicalClock {
        let millis = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_millis() as u64);
        let mut clock = self.clock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        *clock = if millis > clock.millis { HybridLogicalClock { millis, counter: 0 } } else { HybridLogicalClock { millis: clock.millis, counter: clock.counter.saturating_add(1) } };
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
    pub async fn identify(&self, headers: &HeaderMap, peer: Option<SocketAddr>) -> Resolved {
        self.resolvers.resolve(&credential(headers, peer)).await
    }

    /// 📜️ Every durable event of `actor` after `since`.
    pub async fn replay_events(&self, actor: &ActorKey, since: u64) -> Result<Vec<EventRecord>, ServerError> {
        let authority = self.authority.lock().await;
        Ok(authority.store().events_since(actor, since).await?)
    }

    /// 🚰️ Hand up to `limit` committed outbox rows to this instance's workflows and run every
    /// follow-up command as its own turn, returning what each turn answered.
    ///
    /// The two halves are deliberately one call. A drain that only *returned* commands would leave
    /// the rows acknowledged while their consequences sat in a `Vec` the caller might drop, which is
    /// precisely the "state changed but the world was never told" the transactional outbox exists to
    /// rule out. Both locks are released before the first follow-up is submitted, because a turn
    /// takes the bus lock itself and holding it across the drain would deadlock the server on its
    /// own workflow.
    ///
    /// Exactly-once is a property of the *queue*, not of this call: a row leaves `pending` only once
    /// it is marked delivered, so a restart between two drains re-delivers nothing, and a crash
    /// between mapping and marking re-delivers a row whose follow-up command carries an
    /// [`IdempotencyKey`](crate::contract::IdempotencyKey) and is deduplicated by the bus.
    pub async fn drain_sagas(&self, limit: usize) -> Vec<CommandOutcome> {
        let commands = {
            let mut sagas = self.sagas.lock().await;
            let mut authority = self.authority.lock().await;
            sagas.drain_outbox(authority.store_mut(), limit).await
        };
        let mut outcomes = Vec::with_capacity(commands.len());
        for command in commands {
            let now = self.now();
            outcomes.push(self.authority.lock().await.submit(command, now).await);
        }
        outcomes
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
pub async fn put_blob<I: ServerInstance>(Path(hash): Path<String>, headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>, body: Bytes) -> Result<Json<BlobReceipt>, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest { point: PolicyPoint::BlobWrite, principal: resolved.principal, scope: None, resource: hash.clone(), action: "write".to_string() })?;
    let addressed = parse_content_hash(&hash).ok_or_else(|| ServerError::BadRequest("blob address is not 64 hex characters".to_string()))?;
    let computed = content_hash(&body);
    if computed != addressed {
        return Err(ServerError::Conflict(format!("blob address {hash} does not match the content hash {computed} of the uploaded bytes")));
    }
    state.blobs.lock().await.put(computed, &body).await?;
    Ok(Json(BlobReceipt { hash, size: body.len() }))
}

/// 📦️ Read bytes back by address.
pub async fn get_blob<I: ServerInstance>(Path(hash): Path<String>, headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>) -> Result<Response, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest { point: PolicyPoint::BlobRead, principal: resolved.principal, scope: None, resource: hash.clone(), action: "read".to_string() })?;
    let addressed = parse_content_hash(&hash).ok_or_else(|| ServerError::BadRequest("blob address is not 64 hex characters".to_string()))?;
    let bytes = state.blobs.lock().await.get(&addressed).await.ok_or_else(|| ServerError::NotFound(format!("no blob at {hash}")))?;
    Ok(([(header::CONTENT_TYPE, "application/octet-stream")], bytes).into_response())
}

/// ❓️ The cheap half of an upload negotiation: does this address already hold bytes.
pub async fn head_blob<I: ServerInstance>(Path(hash): Path<String>, headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>) -> StatusCode {
    let resolved = state.identify(&headers, Some(peer)).await;
    let request = PolicyRequest { point: PolicyPoint::BlobRead, principal: resolved.principal, scope: None, resource: hash.clone(), action: "read".to_string() };
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
    /// strips every leading `/` before joining — [`Path::join`](std::path::Path::join) treats an absolute argument as a
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
    apps: ShardedMap<String, StaticAppHost>,
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
        self.apps.get_cloned(name)
    }

    /// 📋️ Every registered app name, ordered.
    pub fn names(&self) -> Vec<String> {
        let mut names = Vec::new();
        self.apps.for_each(|name, _| names.push(name.clone()));
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
pub async fn post_command<I: ServerInstance>(headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>, Json(envelope): Json<CommandEnvelope>) -> Result<Json<CommandOutcome>, ServerError> {
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
pub async fn post_query<I: ServerInstance>(headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>, Json(envelope): Json<QueryEnvelope>) -> Result<Json<QueryResult>, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    let mut envelope = envelope;
    envelope.principal = resolved.principal;
    state.authorize(&PolicyRequest { point: PolicyPoint::QueryAccess, principal: envelope.principal.clone(), scope: Some(envelope.scope.clone()), resource: envelope.kind.clone(), action: "read".to_string() })?;
    let handler = state.queries.get_cloned(&envelope.kind).ok_or_else(|| ServerError::NotFound(format!("no handler for query kind '{}'", envelope.kind)))?;
    let projections = state.projections.lock().await;
    Ok(Json(handler.handle(&envelope, &*projections).await?))
}

/// 💨️ Publish one ephemeral frame onto its scope's lossy lane. Nothing is persisted and nothing is
/// replayed — a subscriber that was not listening simply missed it.
pub async fn post_ephemeral<I: ServerInstance>(headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>, Json(frame): Json<EphemeralFrame>) -> Result<Json<usize>, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    let mut frame = frame;
    frame.principal = resolved.principal;
    state.authorize(&PolicyRequest { point: PolicyPoint::Subscription, principal: frame.principal.clone(), scope: Some(frame.scope.clone()), resource: frame.kind.clone(), action: "publish".to_string() })?;
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
pub async fn pump_events<I: ServerInstance, S>(state: &ServerState<I>, actor: &ActorKey, since: u64, live: &mut Subscription, sink: &mut S) -> Result<(), ServerError>
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
pub async fn get_events<I: ServerInstance>(
    Path((tenant, kind, id)): Path<(String, String, String)>,
    Query(query): Query<EventStreamQuery>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState<I>>,
) -> Result<Json<Vec<EventRecord>>, ServerError> {
    let actor = ActorKey { tenant: TenantId(tenant), kind, id };
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest { point: PolicyPoint::EventDelivery, principal: resolved.principal, scope: None, resource: stream_lane(&actor), action: "read".to_string() })?;
    Ok(Json(state.replay_events(&actor, query.since).await?))
}

/// 📡️ The durable lane as a websocket: replay then live, gap-free and duplicate-free.
pub async fn get_event_stream_ws<I: ServerInstance>(
    ws: WebSocketUpgrade,
    Path((tenant, kind, id)): Path<(String, String, String)>,
    Query(query): Query<EventStreamQuery>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState<I>>,
) -> Result<Response, ServerError> {
    let actor = ActorKey { tenant: TenantId(tenant), kind, id };
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest { point: PolicyPoint::Subscription, principal: resolved.principal, scope: None, resource: stream_lane(&actor), action: "subscribe".to_string() })?;
    Ok(ws.on_upgrade(move |socket| handle_event_stream(socket, actor, query.since, state)))
}

async fn handle_event_stream<I: ServerInstance>(socket: WebSocket, actor: ActorKey, since: u64, state: ServerState<I>) {
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
/// 📄️ Non-identity join hints for a document socket. Actor and session are never query parameters —
/// the gateway binds them from the authenticated principal.
#[derive(Clone, Debug, Deserialize)]
pub struct DocumentStreamQuery {
    /// 🪟️ Which surface the actor joined from.
    pub surface: Option<String>,
    /// ⏮️ The engine's own resumption token, passed through verbatim.
    pub resume: Option<String>,
}

/// 🔗️ Bridge one document socket onto the instance's [`DocumentAuthority`].
pub async fn get_document_ws<I: ServerInstance>(
    ws: WebSocketUpgrade,
    Path(scope): Path<String>,
    Query(query): Query<DocumentStreamQuery>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState<I>>,
) -> Result<Response, ServerError> {
    let scope = Scope(scope);
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest { point: PolicyPoint::Subscription, principal: resolved.principal.clone(), scope: Some(scope.clone()), resource: document_lane(&scope), action: "subscribe".to_string() })?;
    let documents = state.documents.as_ref().ok_or_else(|| ServerError::NotFound("this instance hosts no document authority".to_string()))?;
    let identity = documents.bind_socket(&scope, &resolved).await?;
    Ok(ws.protocols([SESSION_PROTOCOL_V1]).on_upgrade(move |socket| handle_document(socket, scope, query, resolved.principal, identity, state)))
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

async fn handle_document<I: ServerInstance>(socket: WebSocket, scope: Scope, query: DocumentStreamQuery, principal: Principal, identity: DocumentSocketIdentity, state: ServerState<I>) {
    let Some(documents) = state.documents.clone() else { return };
    let (mut sender, mut receiver) = socket.split();
    let session = identity.session.clone();
    let actor = identity.actor.clone();
    let lane = document_lane(&scope);
    let mut live = state.fanout.subscribe(&lane);
    state.presence.join(&scope.0, &actor, query.surface.as_deref().unwrap_or("unknown"));

    let handshake = documents.handshake().await;
    let hello = match handshake {
        DocumentHandshake::ServerFirst => None,
        DocumentHandshake::ClientFirst => {
            match receiver.next().await {
                Some(Ok(Message::Binary(payload))) => Some(payload.to_vec()),
                Some(Ok(Message::Ping(payload))) => {
                    let _ = sender.send(Message::Pong(payload)).await;
                    match receiver.next().await {
                        Some(Ok(Message::Binary(payload))) => Some(payload.to_vec()),
                        _ => {
                            state.presence.leave(&scope.0, &actor);
                            return;
                        }
                    }
                }
                _ => {
                    state.presence.leave(&scope.0, &actor);
                    return;
                }
            }
        }
    };

    match documents.welcome(&scope, &actor, query.resume.as_deref(), hello.as_deref()).await {
        Ok(frames) => {
            for frame in frames {
                if sender.send(Message::Binary(frame.into())).await.is_err() {
                    state.presence.leave(&scope.0, &actor);
                    return;
                }
            }
        }
        Err(error) => {
            let _ = sender.send(Message::Text(error.to_string().into())).await;
            state.presence.leave(&scope.0, &actor);
            return;
        }
    }

    loop {
        tokio::select! {
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Binary(payload))) => {
                        match documents.submit_frame(&scope, &actor, &principal, &payload).await {
                            Ok(frames) => {
                                for frame in &frames.echo {
                                    if sender.send(Message::Binary(frame.clone().into())).await.is_err() {
                                        break;
                                    }
                                }
                                for frame in &frames.relay {
                                    state.fanout.publish(&lane, wrap_relay(&session, frame));
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

    state.presence.leave(&scope.0, &actor);
    state.kicks.forget(&session);
}

//#endregion 🔖️DocumentStream

//#region 🔖️PresenceStream
/// 🧍️ Where a presence socket joins from — a hint every member sees, never an identity.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct PresenceStreamQuery {
    /// 🪟️ The surface the session joins from (≤ [`PresenceSettings::max_surface_chars`] chars).
    pub surface: Option<String>,
}

/// 🎫️ What a presence socket was admitted as: the principal every later `watch` is authorized for,
/// and whether it may publish states in the room it joined.
#[derive(Clone, Debug, PartialEq)]
pub struct PresenceGrant {
    pub principal: Principal,
    pub may_publish: bool,
}

/// 🛂️ Admit one presence socket before it is upgraded: the origin (when the instance names an
/// allowlist and the caller presents an `Origin`), the surface length, and
/// [`PolicyPoint::Subscription`] `join` on [`PRESENCE_RESOURCE`]. A joiner whose principal may not
/// also `publish` is refused every state but still sees the room.
pub async fn admit_presence<I: ServerInstance>(state: &ServerState<I>, headers: &HeaderMap, peer: Option<SocketAddr>, scope: &Scope, surface: &str) -> Result<PresenceGrant, ServerError> {
    if let (Some(admits), Some(origin)) = (&state.origin_admission, headers.get(header::ORIGIN)) {
        if !origin.to_str().is_ok_and(|origin| admits(origin)) {
            return Err(ServerError::Forbidden(format!("origin {origin:?} may not join presence")));
        }
    }
    if surface.chars().count() > state.presence_settings.max_surface_chars {
        return Err(ServerError::BadRequest(format!("surface exceeds {} characters", state.presence_settings.max_surface_chars)));
    }
    let principal = state.identify(headers, peer).await.principal;
    state.authorize(&presence_request(&principal, scope, PRESENCE_JOIN))?;
    let may_publish = state.authorize(&presence_request(&principal, scope, PRESENCE_PUBLISH)).is_ok();
    Ok(PresenceGrant { principal, may_publish })
}

/// 👀️ Admit one `watch`: at most [`PresenceSettings::max_watch_scopes`] scopes, each one granted
/// [`PolicyPoint::Subscription`] [`PRESENCE_WATCH`] on [`PRESENCE_RESOURCE`] for the socket's
/// principal — the scoped policy that admits joins decides watches too. Answers the distinct scopes,
/// or the `refused` reason naming the first scope refused; a refused watch changes nothing.
pub fn admit_watch<I: ServerInstance>(state: &ServerState<I>, principal: &Principal, scopes: &[Scope]) -> Result<BTreeSet<String>, String> {
    if scopes.len() > state.presence_settings.max_watch_scopes {
        return Err(REFUSED_WATCH_TOO_MANY.to_string());
    }
    for scope in scopes {
        if state.authorize(&presence_request(principal, scope, PRESENCE_WATCH)).is_err() {
            return Err(format!("{REFUSED_FORBIDDEN} {}", scope.0));
        }
    }
    Ok(scopes.iter().map(|scope| scope.0.clone()).collect())
}

fn presence_request(principal: &Principal, scope: &Scope, action: &str) -> PolicyRequest {
    PolicyRequest { point: PolicyPoint::Subscription, principal: principal.clone(), scope: Some(scope.clone()), resource: PRESENCE_RESOURCE.to_string(), action: action.to_string() }
}

/// 👥️ `GET /scopes/{scope}/presence/ws` — the presence socket (`semio.presence.v1`): a `welcome`,
/// then one coalesced `batch` per tick while the room changes.
pub async fn get_presence_ws<I: ServerInstance>(
    ws: WebSocketUpgrade,
    Path(scope): Path<String>,
    Query(query): Query<PresenceStreamQuery>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState<I>>,
) -> Result<Response, ServerError> {
    let scope = Scope(scope);
    let surface = query.surface.unwrap_or_default();
    let grant = admit_presence(&state, &headers, Some(peer), &scope, &surface).await?;
    Ok(ws.protocols([PRESENCE_PROTOCOL_V1]).on_upgrade(move |socket| async move {
        let (mut sender, mut receiver) = socket.split();
        run_presence(&state, &scope, &surface, &grant, &mut sender, &mut receiver).await;
    }))
}

/// 🔁️ One presence session over any message sink and stream — the socket in production, channels
/// in a test. Subscribes to the room's batches first, joins (leasing a colour from [`Presence`]),
/// sends the `welcome`, then serves until the peer closes, stays idle past
/// [`PresenceSettings::idle`], or is kicked; it always leaves the room and stops watching on the way
/// out.
pub async fn run_presence<I, S, R, E>(state: &ServerState<I>, scope: &Scope, surface: &str, grant: &PresenceGrant, sink: &mut S, stream: &mut R)
where
    I: ServerInstance,
    S: Sink<Message> + Unpin,
    R: Stream<Item = Result<Message, E>> + Unpin,
{
    let settings = *state.presence_settings;
    let lane = presence_lane(scope);
    let session = presence_session_id();
    let mut live = state.fanout.subscribe(&lane);
    let colour = state.presence.join(&lane, &session, surface);
    let (roster, start) = state.presence_rooms.join(&lane, &session, colour, surface);
    if start {
        tokio::spawn(presence_ticker(state.clone(), lane.clone()));
    }
    let seat = Seat { scope, lane: &lane, session: &session, may_publish: grant.may_publish };
    let mut watcher = Watcher::new();
    let mut budget = StateBudget::new(settings.max_states_per_second);
    let mut last_seen = Instant::now();
    let mut keepalive = tokio::time::interval_at(Instant::now() + settings.keepalive, settings.keepalive);
    let mut open = send_presence(sink, &PresenceFrame::Welcome { session: session.clone(), colour, roster }).await;
    while open {
        tokio::select! {
            incoming = stream.next() => {
                last_seen = Instant::now();
                open = match incoming {
                    Some(Ok(Message::Text(text))) => match admit_frame(text.as_str(), &settings, &mut budget, Instant::now()) {
                        Ok(Some(PresenceFrame::State { state: shared })) => share_state(state, &seat, shared, sink).await,
                        Ok(Some(PresenceFrame::Watch { scopes, interval_ms })) => match admit_watch(state, &grant.principal, &scopes) {
                            Ok(scopes) => watcher.watch(state, &scopes, watch_interval(interval_ms, &settings), sink).await,
                            Err(reason) => send_presence(sink, &PresenceFrame::Refused { reason }).await,
                        },
                        Ok(_) => true,
                        Err(reason) => send_presence(sink, &PresenceFrame::Refused { reason }).await,
                    },
                    Some(Ok(Message::Binary(_))) => send_presence(sink, &PresenceFrame::Refused { reason: REFUSED_INVALID.to_string() }).await,
                    Some(Ok(Message::Ping(payload))) => sink.send(Message::Pong(payload)).await.is_ok(),
                    Some(Ok(Message::Pong(_))) => true,
                    Some(Ok(Message::Close(_))) | Some(Err(_)) | None => false,
                };
            }
            delivery = live.next() => {
                open = match delivery {
                    Delivery::Frame(bytes) => match String::from_utf8(bytes) {
                        Ok(text) => sink.send(Message::Text(text.into())).await.is_ok(),
                        Err(_) => true,
                    },
                    Delivery::Lagged => send_presence(sink, &PresenceFrame::Welcome { session: session.clone(), colour, roster: state.presence_rooms.roster(&lane) }).await,
                    Delivery::Closed => false,
                };
            }
            _ = keepalive.tick() => open = sink.send(Message::Ping(Vec::new().into())).await.is_ok(),
            _ = tokio::time::sleep_until(last_seen + settings.idle) => {
                let _ = sink.send(Message::Close(None)).await;
                open = false;
            }
            _ = state.kicks.kicked(&session) => open = false,
            Some((watched, delivery)) = watcher.deliveries.recv() => open = watcher.deliver(state, &watched, delivery, sink).await,
            () = next_flush(&mut watcher.ticks) => open = watcher.flush(sink).await,
        }
    }
    drop(watcher);
    state.presence_rooms.leave(&lane, &session);
    state.presence.leave(&lane, &session);
    state.kicks.forget(&session);
}

/// 💺️ Who one presence session is: its scope, lane and session id, and whether it may publish.
struct Seat<'a> {
    scope: &'a Scope,
    lane: &'a str,
    session: &'a str,
    may_publish: bool,
}

/// 📣️ Share one admitted state in the joined room — when the principal may publish and every
/// module's [`ServerModule::presence_admission`] accepts it — or answer why not.
async fn share_state<I: ServerInstance, S: Sink<Message> + Unpin>(state: &ServerState<I>, seat: &Seat<'_>, shared: OpaqueJson, sink: &mut S) -> bool {
    let verdict = if seat.may_publish { state.modules.iter().try_for_each(|module| module.presence_admission(seat.scope, &shared)) } else { Err(REFUSED_FORBIDDEN.to_string()) };
    match verdict {
        Ok(()) => {
            if let Ok(bytes) = serde_json::to_vec(&shared) {
                state.presence.publish_peer(seat.lane, seat.session, bytes);
            }
            state.presence_rooms.update(seat.lane, seat.session, shared);
            true
        }
        Err(reason) => send_presence(sink, &PresenceFrame::Refused { reason }).await,
    }
}

/// 📦️ How many relayed batches a watching socket may have queued before its forwarders wait (and,
/// waiting, fall behind their rooms and resynchronize with a snapshot).
const WATCH_BACKLOG: usize = 64;

/// 📨️ What a watched room's forwarder relays: one batch, or the fact that it fell behind.
enum WatchDelivery {
    Batch { entries: Vec<PresenceEntry>, left: Vec<String> },
    Lagged,
}

/// 🧵️ One forwarder task, aborted when dropped — unwatching a scope or closing the socket stops it.
struct Forwarder(tokio::task::JoinHandle<()>);

impl Drop for Forwarder {
    fn drop(&mut self) {
        self.0.abort();
    }
}

/// 👀️ The watching half of one presence socket: its [`Watching`] set, one forwarder per watched
/// room relaying that room's batches (parsed on the forwarder, not on the socket's task), and the
/// interval that flushes the coalesced changes.
struct Watcher {
    watching: Watching,
    forwarders: BTreeMap<String, Forwarder>,
    sender: mpsc::Sender<(String, WatchDelivery)>,
    deliveries: mpsc::Receiver<(String, WatchDelivery)>,
    interval: Duration,
    ticks: Option<tokio::time::Interval>,
}

impl Watcher {
    fn new() -> Self {
        let (sender, deliveries) = mpsc::channel(WATCH_BACKLOG);
        Self { watching: Watching::default(), forwarders: BTreeMap::new(), sender, deliveries, interval: Duration::ZERO, ticks: None }
    }

    /// 🔁️ Replace the watch set: stop forwarding dropped scopes, subscribe to each new scope's room
    /// before sending its `snapshot`, and (re)start the interval when it changed.
    async fn watch<I: ServerInstance, S: Sink<Message> + Unpin>(&mut self, state: &ServerState<I>, scopes: &BTreeSet<String>, interval: Duration, sink: &mut S) -> bool {
        let added = self.watching.watch(scopes);
        self.forwarders.retain(|scope, _| scopes.contains(scope));
        if self.watching.is_empty() {
            self.ticks = None;
        } else if self.ticks.is_none() || self.interval != interval {
            let mut ticks = tokio::time::interval_at(Instant::now() + interval, interval);
            ticks.set_missed_tick_behavior(MissedTickBehavior::Delay);
            self.ticks = Some(ticks);
        }
        self.interval = interval;
        for scope in added {
            let lane = presence_lane(&Scope(scope.clone()));
            let live = state.fanout.subscribe(&lane);
            self.forwarders.insert(scope.clone(), Forwarder(tokio::spawn(forward_watched(scope.clone(), live, self.sender.clone()))));
            if let Some(snapshot) = self.watching.snapshot(&scope, state.presence_rooms.roster(&lane)) {
                if !send_presence(sink, &snapshot).await {
                    return false;
                }
            }
        }
        true
    }

    /// 🧺️ Take one relayed delivery: fold a batch into its scope, or resynchronize a scope whose
    /// forwarder fell behind with a fresh `snapshot`.
    async fn deliver<I: ServerInstance, S: Sink<Message> + Unpin>(&mut self, state: &ServerState<I>, scope: &str, delivery: WatchDelivery, sink: &mut S) -> bool {
        match delivery {
            WatchDelivery::Batch { entries, left } => {
                self.watching.absorb(scope, entries, left);
                true
            }
            WatchDelivery::Lagged => match self.watching.snapshot(scope, state.presence_rooms.roster(&presence_lane(&Scope(scope.to_string())))) {
                Some(snapshot) => send_presence(sink, &snapshot).await,
                None => true,
            },
        }
    }

    /// 🕰️ Send one `watched` frame per scope that changed since the last interval.
    async fn flush<S: Sink<Message> + Unpin>(&mut self, sink: &mut S) -> bool {
        for frame in self.watching.flush() {
            if !send_presence(sink, &frame).await {
                return false;
            }
        }
        true
    }
}

/// ⏰️ The next flush of a watcher's interval — never, while nothing is watched.
async fn next_flush(ticks: &mut Option<tokio::time::Interval>) {
    match ticks {
        Some(ticks) => {
            ticks.tick().await;
        }
        None => std::future::pending().await,
    }
}

/// 📡️ Relay one watched room's batches to the watching socket until it stops watching.
async fn forward_watched(scope: String, mut live: Subscription, into: mpsc::Sender<(String, WatchDelivery)>) {
    loop {
        let delivery = match live.next().await {
            Delivery::Frame(bytes) => match serde_json::from_slice(&bytes) {
                Ok(PresenceFrame::Batch { entries, left }) => WatchDelivery::Batch { entries, left },
                _ => continue,
            },
            Delivery::Lagged => WatchDelivery::Lagged,
            Delivery::Closed => return,
        };
        if into.send((scope.clone(), delivery)).await.is_err() {
            return;
        }
    }
}

async fn send_presence<S: Sink<Message> + Unpin>(sink: &mut S, frame: &PresenceFrame) -> bool {
    match serde_json::to_string(frame) {
        Ok(text) => sink.send(Message::Text(text.into())).await.is_ok(),
        Err(_) => true,
    }
}

/// 🕰️ One room's ticker: every [`PresenceSettings::tick`] it publishes the coalesced `batch` on
/// the room's lane, and it ends once the room is empty and has announced every departure.
async fn presence_ticker<I: ServerInstance>(state: ServerState<I>, lane: String) {
    let mut ticks = tokio::time::interval_at(Instant::now() + state.presence_settings.tick, state.presence_settings.tick);
    ticks.set_missed_tick_behavior(MissedTickBehavior::Delay);
    loop {
        ticks.tick().await;
        match state.presence_rooms.drain(&lane) {
            Drained::Batch(frame) => {
                if let Ok(bytes) = serde_json::to_vec(&frame) {
                    state.fanout.publish(&lane, bytes);
                }
            }
            Drained::Quiet => {}
            Drained::Finished => return,
        }
    }
}
//#endregion 🔖️PresenceStream

//#region 🔖️AppRoutes
/// 📋️ The names of every app this instance hosts.
pub async fn get_apps<I: ServerInstance>(State(state): State<ServerState<I>>) -> Json<Vec<String>> {
    Json(state.apps.names())
}

/// 🧩️ The `install.json` entries under one app's root.
pub async fn get_app_installs<I: ServerInstance>(Path(app): Path<String>, State(state): State<ServerState<I>>) -> Result<Json<Vec<AppInstall>>, ServerError> {
    let host = state.apps.host(&app).ok_or_else(|| ServerError::NotFound(format!("no app '{app}'")))?;
    Ok(Json(scan_installs(host.root())))
}

/// 🏠️ One app's entry document.
pub async fn get_app_root<I: ServerInstance>(Path(app): Path<String>, State(state): State<ServerState<I>>) -> Response {
    serve_app(&state, &app, "index.html")
}

/// 📎️ One asset inside an app.
pub async fn get_app_asset<I: ServerInstance>(Path((app, rest)): Path<(String, String)>, State(state): State<ServerState<I>>) -> Response {
    serve_app(&state, &app, &rest)
}

fn serve_app<I: ServerInstance>(state: &ServerState<I>, app: &str, rest: &str) -> Response {
    match state.apps.host(app) {
        Some(host) => host.serve(rest),
        None => ServerError::NotFound(format!("no app '{app}'")).into_response(),
    }
}
//#endregion 🔖️AppRoutes

//#region 🔖️Server
/// 🏗️ Assembles one server out of a storage profile and one [`ServerInstance`]'s modules.
pub struct ServerBuilder<I: ServerInstance> {
    profile: StorageProfile,
    modules: Vec<I::Modules>,
    queries: Vec<I::Queries>,
    sagas: Vec<I::Sagas>,
    apps: Vec<(String, PathBuf)>,
    documents: Option<Arc<I::Documents>>,
    admin_token: Option<String>,
    presence: PresenceSettings,
    origin_admission: Option<OriginAdmission>,
    id: String,
    version: String,
}

impl<I: ServerInstance> ServerBuilder<I> {
    /// ⏱️ Tune the presence socket (tick, idle timeout, keepalive, limits).
    pub fn presence(mut self, settings: PresenceSettings) -> Self {
        self.presence = settings;
        self
    }

    /// 🌍️ Name the browser origins that may open a presence socket.
    pub fn origin_admission(mut self, admits: OriginAdmission) -> Self {
        self.origin_admission = Some(admits);
        self
    }

    /// 🧩️ Register one module. Its deciders, templates, resolvers and routes are collected at
    /// [`build`](Self::build) time, in registration order.
    pub fn module(mut self, module: I::Modules) -> Self {
        self.modules.push(module);
        self
    }

    /// ❓️ Register one query handler under the kind it declares.
    pub fn query(mut self, handler: I::Queries) -> Self {
        self.queries.push(handler);
        self
    }

    /// 🧵️ Register one cross-actor workflow on this instance's saga runner, for a workflow that
    /// belongs to the deployment rather than to one of its modules.
    pub fn saga(mut self, saga: I::Sagas) -> Self {
        self.sagas.push(saga);
        self
    }

    /// 📄️ Supply the replication engine backing the document websocket.
    pub fn document_authority(mut self, documents: Arc<I::Documents>) -> Self {
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

    /// 🔨️ Collect every module's contribution into one shared engine, bus, ladder and router, over
    /// the storage the instance opens for this profile.
    pub async fn build(self) -> Result<Server<I>, ServerError> {
        let policy = Arc::new(RwLock::new(PolicyEngine::new()));
        let mut chain = ResolverChain::<I::Resolvers>::new();
        let mut definition = ServerInstanceDefinition { id: self.id.clone(), version: self.version.clone(), modules: Vec::new() };
        let mut deciders: Vec<I::Deciders> = Vec::new();
        let mut workflows: Vec<I::Sagas> = Vec::new();
        for module in &self.modules {
            let manifest = module.manifest().await;
            let templates: Vec<PolicyTemplate> = manifest.policies.iter().cloned().chain(module.templates().await).collect();
            if let Ok(mut engine) = policy.write() {
                for template in templates {
                    if template.auto_apply {
                        engine.set_authenticated_template(template.name.clone());
                    }
                    engine.register_template(template);
                }
            }
            definition.modules.push(manifest);
            deciders.extend(module.deciders().await);
            workflows.extend(module.sagas().await);
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

        let stores = I::open(&self.profile).await?;
        let mut bus = CommandBus::new(AuthorityDirectory::new(), stores.authority, hook);
        for decider in deciders {
            bus.register(decider).await;
        }
        let mut runner = ServerSagas::<I>::new();
        for saga in workflows.into_iter().chain(self.sagas) {
            runner.register(saga);
        }

        let apps = AppRegistry::new();
        for (name, dir) in &self.apps {
            apps.register(name, dir.clone());
        }
        let queries: ShardedMap<String, Arc<I::Queries>> = ShardedMap::new();
        for handler in self.queries {
            queries.insert(handler.kind().await.to_string(), Arc::new(handler));
        }

        let modules = Arc::new(self.modules);
        let state = ServerState {
            authority: Arc::new(Mutex::new(bus)),
            sagas: Arc::new(Mutex::new(runner)),
            projections: Arc::new(Mutex::new(stores.projections)),
            blobs: Arc::new(Mutex::new(stores.blobs)),
            sessions: Arc::new(Mutex::new(stores.sessions)),
            policy,
            resolvers: Arc::new(chain),
            admin: Arc::new(AdminGate::new(self.admin_token.clone())),
            fanout: Arc::new(Fanout::new()),
            presence: Arc::new(Presence::new()),
            kicks: Arc::new(KickMap::new()),
            apps: Arc::new(apps),
            queries: Arc::new(queries),
            documents: self.documents.clone(),
            profile: Arc::new(self.profile.clone()),
            modules: Arc::clone(&modules),
            presence_rooms: Arc::new(PresenceRooms::new()),
            presence_settings: Arc::new(self.presence),
            origin_admission: self.origin_admission,
            clock: Arc::new(StdMutex::new(HybridLogicalClock::default())),
        };

        let mut router = base_router(definition.clone(), self.documents.is_some());
        for module in modules.iter() {
            router = module.routes(router).await;
        }
        let router = router.layer(axum::middleware::from_fn(cors_middleware)).with_state(state.clone());
        Ok(Server { state, router, definition })
    }
}

/// 🚦️ The admission question one command envelope asks of the policy engine.
fn admission_request(envelope: &CommandEnvelope) -> PolicyRequest {
    PolicyRequest { point: PolicyPoint::CommandAdmission, principal: envelope.principal.clone(), scope: Some(envelope.scope.clone()), resource: format!("{}/{}", envelope.target.kind, envelope.target.id), action: envelope.kind.clone() }
}

/// 🛣️ Every route the framework itself owns, before any module adds its own. Each handler is
/// instantiated at the instance being built, so a route is monomorphic even though the set of
/// backends behind it is chosen downstream. The document socket is mounted only for an instance
/// that registered a [`DocumentAuthority`], leaving the path to an instance that owns it itself.
fn base_router<I: ServerInstance>(definition: ServerInstanceDefinition, hosts_documents: bool) -> Router<ServerState<I>> {
    let router = Router::new()
        .route("/instance", get(move || instance_body(definition.clone())))
        .route("/commands", post(post_command::<I>))
        .route("/queries", post(post_query::<I>))
        .route("/scopes/{scope}/ephemeral", post(post_ephemeral::<I>))
        .route("/scopes/{scope}/presence/ws", get(get_presence_ws::<I>))
        .route("/actors/{tenant}/{kind}/{id}/events", get(get_events::<I>))
        .route("/actors/{tenant}/{kind}/{id}/events/ws", get(get_event_stream_ws::<I>))
        .route("/blobs/{hash}", get(get_blob::<I>).head(head_blob::<I>).put(put_blob::<I>))
        .route("/apps", get(get_apps::<I>))
        .route("/apps/{app}/installs", get(get_app_installs::<I>))
        .route("/apps/{app}", get(get_app_root::<I>))
        .route("/apps/{app}/{*rest}", get(get_app_asset::<I>));
    if hosts_documents {
        router.route("/scopes/{scope}/document/ws", get(get_document_ws::<I>))
    } else {
        router
    }
}

async fn instance_body(definition: ServerInstanceDefinition) -> Json<ServerInstanceDefinition> {
    Json(definition)
}

/// 🖥️ One built server: its shared state, its router and the definition it reports.
pub struct Server<I: ServerInstance> {
    state: ServerState<I>,
    router: Router,
    definition: ServerInstanceDefinition,
}

impl<I: ServerInstance> Server<I> {
    /// 🏗️ Start assembling a server over one storage profile.
    pub fn builder(profile: StorageProfile) -> ServerBuilder<I> {
        ServerBuilder {
            profile,
            modules: Vec::new(),
            queries: Vec::new(),
            sagas: Vec::new(),
            apps: Vec::new(),
            documents: None,
            admin_token: None,
            presence: PresenceSettings::default(),
            origin_admission: None,
            id: "server".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// 🛣️ The fully wired router, ready to be served or mounted.
    pub fn router(&self) -> Router {
        self.router.clone()
    }

    /// 🧠️ The shared state every handler runs against.
    pub fn state(&self) -> &ServerState<I> {
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
        axum::serve(listener, self.router.into_make_service_with_connect_info::<SocketAddr>()).await.map_err(|error| ServerError::Internal(error.to_string()))
    }
}
//#endregion 🔖️Server

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
