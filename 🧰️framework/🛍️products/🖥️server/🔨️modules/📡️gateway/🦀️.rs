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
//! [`document_lane`]) precisely so no future refactor can quietly start replaying a cursor position
//! or dropping a committed event.
//!
//! **Presence is latest-state, read by cursor, and bounded per socket.** A presence room
//! ([`PresenceRooms`]) keeps each session's newest opaque state only, under a clock every change
//! advances. Nothing is queued for anybody: every socket reads the room through its own
//! [`RoomReader`] — a few numbers, whatever the room's size — once per tick while the room changes,
//! and is sent what changed since its last read as one frame of at most
//! [`PresenceSettings::max_frame_bytes`]. A roster or a burst larger than a frame continues in the
//! next frames, each reader going round the room at its own pace, so a slow reader costs the
//! server nothing and receives the newest states when it catches up. Joining is
//! [`PolicyPoint::Subscription`] `join` on [`PRESENCE_RESOURCE`], sharing is `publish`, and every
//! module may refuse a state through [`ServerModule::presence_admission`]. A socket may also
//! *watch* up to [`PresenceSettings::max_watch_scopes`] other rooms read-only (`watch` per scope):
//! one more reader per watched room, sent at the watcher's own interval under one frame's allowance
//! for all of them.
//!
//! **The edge is bounded, and the instance sizes it.** [`throttle_middleware`] spends a token of the
//! caller's address per request, takes the request's body — capped ([`Limits::body_bytes`]) and
//! given a time to arrive ([`Limits::body_patience`]) — and only then counts the request in flight
//! ([`throttle`](crate::throttle)); a refused request is answered `429` or `503` with
//! `Retry-After` before anything behind the edge runs, and its body is read all the same so that
//! its connection lives on. A command a module classes under a named allowance
//! ([`ServerModule::command_allowance`]) — the commands that make the instance keep something —
//! spends a token of that allowance as well, and gets it back when it kept nothing
//! ([`post_command`]). Every socket is counted against its address and the instance and reads no
//! message past its protocol's bound ([`bounded_socket`]), what the presence rooms send is bounded
//! per frame and in total and dealt fairly among the addresses that read ([`PresenceSettings`]),
//! and an instance mounts only the route groups it uses ([`RouteGroups`]). An instance that sets
//! no [`Limits`] bounds nothing at the throttle.
//!
//! **Every extension point is a port, and [`ServerInstance`] is the one place they are all named.**
//! An instance — hub, zentrale, this crate's own test profile — is a type implementing that trait,
//! and its ten associated types say which module set, query set, document engine, decider set, saga
//! set, resolver ladder and four storage backends this deployment is made of. [`ServerBuilder`],
//! [`ServerState`], [`Server`] and every handler below are generic over it, so the set of
//! implementations is closed in the instance's own crate and never here. The server product
//! deliberately depends on no document engine: not on the os product, not on `db`, not on any
//! concrete CRDT — and with the sets closed downstream it does not have to name one to be usable.

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::hash::{BuildHasher, Hasher};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path as FsPath, PathBuf};
use semio_framework_dispatch_macros::dyn_enum;
use std::future::Future;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex as StdMutex, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::body::Bytes;
use axum::extract::rejection::JsonRejection;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{ConnectInfo, DefaultBodyLimit, FromRequest, Path, Query, Request, State};
use axum::http::{header, HeaderMap, HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::serve::ListenerExt;
use axum::{Json, Router};
use futures::{Sink, SinkExt, Stream, StreamExt};
use semio_framework_async::ShardedMap;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, oneshot, watch, Mutex, Notify};
use tokio::time::{Instant, MissedTickBehavior};

use crate::authority::{AdmissionHook, AuthorityDirectory, AuthorityError, CommandBus, Committed, Decider, PolicyHook, Saga, SagaRunner, SharedStore, DIRECTORY_CAPACITY, SNAPSHOT_INTERVAL, UNAVAILABLE};
use crate::contract::{
    ActorKey, CommandEnvelope, CommandOutcome, CommandReceipt, EphemeralFrame, EventRecord, HybridLogicalClock, ModuleManifest, OpaqueJson, PolicyDecision, PolicyPoint, PolicyTemplate, PresenceEntry, PresenceFrame, Principal, QueryEnvelope,
    QueryResult, Rejection, Revision, Scope, ServerInstanceDefinition, TenantId,
};
use crate::policy::{AdminGate, Credential, PolicyEngine, PolicyRequest, PrincipalResolver, Resolved, ResolverChain};
use crate::storage::{content_hash, AuthorityStore, BlobStore, ProjectionStore, SessionStore, StorageError, StorageProfile};
use crate::throttle::{forwarded_address, ClientKey, Limits, Refusal, RequestClass, SocketPermit, Throttle};

//#region 🔖️Reexport
/// 🚪️ The router type a [`ServerModule`] contributes routes to. Reexported so an instance never has
/// to name the transport library itself.
pub use axum::Router as GatewayRouter;

/// 📦️ The JSON body wrapper handlers use, reexported for the same reason as [`GatewayRouter`].
pub use axum::Json as GatewayJson;
//#endregion 🔖️Reexport

//#region 🔖️Error
/// 💥️ The one error every transport surface answers with. An instance maps its own domain errors
/// into these ten shapes; the status code is derived here so no handler ever picks one by hand.
///
/// **What the caller is told.** The detail a variant carries is the server's own account of the
/// fault and is written for its operator ([`Display`](std::fmt::Display)). The body a caller
/// receives ([`message`](Self::message)) repeats it only where it describes the caller's own
/// request; a policy denial answers `forbidden` and an internal fault `internal error`, and the
/// detail of the latter goes to the operator's log instead.
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
    /// 📦️ The request body is larger than this instance accepts — 413.
    PayloadTooLarge,
    /// 🐌️ The request body did not arrive in the time this instance waits for one — 408.
    Stalled,
    /// 🐇️ The caller's address spent an allowance — 429, worth retrying after the wait: its
    /// allowance of requests, or the named one ([`Allowance`](crate::throttle::Allowance)) the
    /// refused command is counted against.
    Throttled { wait: Duration, allowance: Option<&'static str> },
    /// 🌊️ The instance is at one of its caps — 503, worth retrying after the wait.
    Overloaded(Duration),
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
            Self::PayloadTooLarge => formatter.write_str("payload too large"),
            Self::Stalled => formatter.write_str("the request body did not arrive in time"),
            Self::Throttled { wait, allowance: None } => write!(formatter, "too many requests: retry in {} ms", wait.as_millis()),
            Self::Throttled { wait, allowance: Some(allowance) } => write!(formatter, "the {allowance} allowance of this address is spent: retry in {} ms", wait.as_millis()),
            Self::Overloaded(wait) => write!(formatter, "overloaded: retry in {} ms", wait.as_millis()),
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
            Self::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Self::Stalled => StatusCode::REQUEST_TIMEOUT,
            Self::Throttled { .. } => StatusCode::TOO_MANY_REQUESTS,
            Self::Overloaded(_) => StatusCode::SERVICE_UNAVAILABLE,
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
            Self::PayloadTooLarge => "payloadTooLarge",
            Self::Stalled => "stalled",
            Self::Throttled { .. } => "throttled",
            Self::Overloaded(_) => "overloaded",
        }
    }

    /// 💬️ What the caller is told: the detail of its own request, never the server's.
    pub fn message(&self) -> String {
        match self {
            Self::Forbidden(_) => "forbidden".to_string(),
            Self::Internal(_) => "internal error".to_string(),
            Self::Throttled { allowance: None, .. } => "too many requests".to_string(),
            Self::Throttled { allowance: Some(allowance), .. } => format!("the {allowance} allowance of this address is spent"),
            Self::Overloaded(_) => "overloaded".to_string(),
            Self::Unauthorized(_) | Self::NotFound(_) | Self::Conflict(_) | Self::BadRequest(_) | Self::PayloadTooLarge | Self::Stalled => self.to_string(),
        }
    }

    /// ⏳️ The wait after which the request is worth repeating, for the two errors that pass.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::Throttled { wait, .. } | Self::Overloaded(wait) => Some(*wait),
            _ => None,
        }
    }

    /// 💰️ The named allowance whose exhaustion this error is, when it is one.
    pub fn allowance(&self) -> Option<&'static str> {
        match self {
            Self::Throttled { allowance, .. } => *allowance,
            _ => None,
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
    /// ⏳️ The wait in milliseconds after which the request is worth repeating, when it is. Carried
    /// in the body as well as in `Retry-After`, which a cross-origin page cannot read unless the
    /// response exposes it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_after_ms: Option<u64>,
    /// 🧧️ The named allowance of the caller's address that is spent, when the refusal is one of a
    /// class of commands the instance counts by what they make it keep rather than of requests as
    /// such: a client tells its user what cannot be done for now instead of only slowing down.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowance: Option<String>,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        if let Self::Internal(detail) = &self {
            crate::report("gateway", detail);
        }
        let wait = self.retry_after();
        let body = ErrorBody { kind: self.kind().to_string(), message: self.message(), retry_after_ms: wait.map(|wait| u64::try_from(wait.as_millis()).unwrap_or(u64::MAX).max(1)), allowance: self.allowance().map(str::to_string) };
        let mut response = (self.status(), Json(body)).into_response();
        if let Some(wait) = wait {
            response.headers_mut().insert(header::RETRY_AFTER, HeaderValue::from(wait.as_secs() + u64::from(wait.subsec_nanos() > 0 || wait.is_zero())));
        }
        response
    }
}

impl From<StorageError> for ServerError {
    /// 🗄️ A store's refusal as the caller may see it: its wording is the backend's and stays in the
    /// operator's log.
    fn from(error: StorageError) -> Self {
        match error {
            StorageError::NotFound => Self::NotFound("storage entry not found".to_string()),
            StorageError::Conflict(_) | StorageError::SequenceGap { .. } | StorageError::LeaseLost => {
                crate::report("gateway storage conflict", &error);
                Self::Conflict("the write contradicts what is stored".to_string())
            }
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

impl From<Refusal> for ServerError {
    fn from(refusal: Refusal) -> Self {
        match refusal {
            Refusal::Throttled { retry_after } => Self::Throttled { wait: retry_after, allowance: None },
            Refusal::Overloaded { retry_after } => Self::Overloaded(retry_after),
        }
    }
}

/// 📥️ A JSON request body as every route of this gateway takes one: the transport's own extractor,
/// answering a body it cannot take with the gateway's own [`ErrorBody`] — `payloadTooLarge` past the
/// instance's body limit, `badRequest` for anything that is not the expected JSON document sent as
/// `application/json`.
pub struct WireJson<T>(pub T);

impl<S: Send + Sync, T: serde::de::DeserializeOwned> FromRequest<S> for WireJson<T> {
    type Rejection = ServerError;

    async fn from_request(request: Request, state: &S) -> Result<Self, ServerError> {
        match Json::<T>::from_request(request, state).await {
            Ok(Json(value)) => Ok(Self(value)),
            Err(rejection) if rejection.status() == StatusCode::PAYLOAD_TOO_LARGE => Err(ServerError::PayloadTooLarge),
            Err(JsonRejection::MissingJsonContentType(_)) => Err(ServerError::BadRequest("the body must be sent as application/json".to_string())),
            Err(rejection) => Err(ServerError::BadRequest(rejection.body_text())),
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

    /// 🛃️ Admit or refuse the shape of one command envelope before the bus reads or places anything
    /// for it: an id of a form this module's actors never have, a key or a kind out of bounds. Every
    /// module is asked in registration order and the first refusal is the command's rejection; the
    /// default admits. Synchronous and pure for the same reason as
    /// [`presence_admission`](Self::presence_admission): it runs inside every turn, ahead of the
    /// store, and judges nothing but the envelope.
    fn command_admission(&self, _envelope: &CommandEnvelope) -> Result<(), Rejection> {
        Ok(())
    }

    /// 💳️ The named allowance ([`Limits::allowances`]) one command posted by a client is counted
    /// against, besides the write allowance of its request: the class of commands that make the
    /// instance keep something of a finite store — a registration, an upload. Every module is asked
    /// in registration order and the first name is the command's class; the default names none. A
    /// command the instance issues itself (a saga's follow-up) is never asked about. Synchronous and
    /// pure like [`command_admission`](Self::command_admission), and asked before it: it judges by
    /// the envelope's kind, which the admission then holds the payload to.
    fn command_allowance(&self, _envelope: &CommandEnvelope) -> Option<&'static str> {
        None
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
            let mut taken = [false; 256];
            for lease in colours.by_actor.values() {
                taken[usize::from(lease.index)] = true;
            }
            let index = taken.iter().position(|taken| !taken).unwrap_or(colours.by_actor.len() % 256) as u8;
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
    /// 🕰️ How often, at most, a member is sent what changed in its room: one `batch` per tick.
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
    /// 🖼️ The most bytes of entries and departures one server frame carries: what a room sends one
    /// member per [`tick`](Self::tick), and what a socket is sent per watch interval over everything
    /// it watches. A roster or a burst of changes larger than that continues in the next frames. An
    /// entry is never split, so a frame holding one larger entry exceeds it.
    pub max_frame_bytes: usize,
    /// 📡️ The most bytes of entries and departures all rooms of the instance together send per
    /// second, a quarter of a second's worth at once; `None` bounds nothing. The budget is dealt
    /// fairly among the client addresses whose sockets read: an address is never held back while it
    /// takes less than an even split, and one that asks for more than its fair share — through
    /// however many sockets — waits by itself, each of its sockets longer for its next frame, which
    /// then carries the newest states. Only the `welcome` of a joining session is sent at once (and
    /// counted): the rate sessions join at is bounded by the instance's edge ([`Limits::upgrades`]).
    pub max_bytes_per_second: Option<u64>,
    /// 🪦️ How many departures a room remembers. A reader that missed more is sent the roster anew.
    pub departures_kept: usize,
}

impl PresenceSettings {
    /// 📏️ The most the presence socket's protocol layer takes of one inbound message: twice the
    /// largest state, so a frame somewhat over the limit is still read and answered
    /// [`REFUSED_TOO_LARGE`], while anything larger fails the socket before it is buffered.
    pub fn inbound_bytes(&self) -> usize {
        self.max_state_bytes.saturating_mul(2)
    }
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
            max_frame_bytes: 4096,
            max_bytes_per_second: Some(16 * 1024 * 1024),
            departures_kept: 256,
        }
    }
}

/// 🌍️ The instance's answer to "may a browser page served from this origin open a socket here".
/// Checked for every presence socket that presents an `Origin`; an instance that sets none admits
/// every origin, which is right only where nothing but loopback can reach the process.
pub type OriginAdmission = Arc<dyn Fn(&str) -> bool + Send + Sync>;

/// 🧍️ One member of a presence room: its session and its entry as every reader is sent it.
#[derive(Debug)]
struct Member {
    session: String,
    wire: Box<str>,
}

/// 🏠️ One room: its members on numbered seats, the clock every join, state and departure
/// advances, the departures it still remembers (and the clock of the newest one it forgot), and
/// how many sockets read it.
///
/// A member keeps its seat while it is in the room, and a vacated seat is the next one taken.
/// `changed` is a tournament over the seats — the leaves hold the clock each seat last changed at
/// (`0` while vacant), every node above the newer of its two halves — so the next seat that
/// changed after a given clock is found in a number of steps logarithmic in the room's size
/// ([`Room::next_changed`]), which is what lets every reader go round a room of any size at the
/// cost of what it is sent.
#[derive(Debug)]
struct Room {
    seats: Vec<Option<Member>>,
    changed: Vec<u64>,
    vacant: Vec<usize>,
    seated: HashMap<String, usize>,
    departed: VecDeque<(u64, String)>,
    forgotten: u64,
    clock: u64,
    readers: usize,
    changes: watch::Sender<u64>,
}

impl Default for Room {
    fn default() -> Self {
        Self { seats: Vec::new(), changed: Vec::new(), vacant: Vec::new(), seated: HashMap::new(), departed: VecDeque::new(), forgotten: 0, clock: 0, readers: 0, changes: watch::channel(0).0 }
    }
}

impl Room {
    /// ⏭️ Advance the clock and wake every reader waiting for a change.
    fn advance(&mut self) -> u64 {
        self.clock += 1;
        self.changes.send_replace(self.clock);
        self.clock
    }

    /// 🫙️ Whether nobody is in the room and nobody reads it.
    fn deserted(&self) -> bool {
        self.seated.is_empty() && self.readers == 0
    }

    /// 🪑️ Seat `member` (or replace the entry on the seat its session holds) at the next clock.
    fn seat(&mut self, member: Member) {
        let seat = match self.seated.get(&member.session) {
            Some(seat) => *seat,
            None => {
                if self.vacant.is_empty() {
                    self.grow();
                }
                let seat = self.vacant.pop().unwrap_or_default();
                self.seated.insert(member.session.clone(), seat);
                seat
            }
        };
        self.seats[seat] = Some(member);
        let clock = self.advance();
        self.mark(seat, clock);
    }

    /// 🧳️ Vacate `session`'s seat at the next clock and remember the departure, forgetting the
    /// oldest ones past `kept`; `false` when the session holds no seat.
    fn vacate(&mut self, session: &str, kept: usize) -> bool {
        let Some(seat) = self.seated.remove(session) else { return false };
        self.seats[seat] = None;
        self.vacant.push(seat);
        self.mark(seat, 0);
        let clock = self.advance();
        self.departed.push_back((clock, session.to_string()));
        while self.departed.len() > kept {
            if let Some((clock, _)) = self.departed.pop_front() {
                self.forgotten = clock;
            }
        }
        true
    }

    /// 📈️ Double the seats (one, in an empty room) and rebuild the tournament over them.
    fn grow(&mut self) {
        let seats = (self.seats.len() * 2).max(1);
        let mut changed = vec![0; seats * 2];
        changed[seats..seats + self.seats.len()].copy_from_slice(&self.changed[self.seats.len()..]);
        for node in (1..seats).rev() {
            changed[node] = changed[node * 2].max(changed[node * 2 + 1]);
        }
        self.vacant.extend((self.seats.len()..seats).rev());
        self.seats.resize_with(seats, || None);
        self.changed = changed;
    }

    /// 📍️ Record that `seat` changed at `clock` (`0`: it is vacant).
    fn mark(&mut self, seat: usize, clock: u64) {
        let mut node = self.seats.len() + seat;
        self.changed[node] = clock;
        while node > 1 {
            node /= 2;
            self.changed[node] = self.changed[node * 2].max(self.changed[node * 2 + 1]);
        }
    }

    /// 🔎️ The first seat at or after `from` that changed after `clock`.
    fn next_changed(&self, from: usize, clock: u64) -> Option<usize> {
        let seats = self.seats.len();
        if from >= seats {
            return None;
        }
        let mut node = seats + from;
        loop {
            if self.changed[node] > clock {
                while node < seats {
                    node = if self.changed[node * 2] > clock { node * 2 } else { node * 2 + 1 };
                }
                return Some(node - seats);
            }
            while node % 2 == 1 {
                node /= 2;
            }
            if node == 0 {
                return None;
            }
            node += 1;
        }
    }
}

/// ⏲️ How often the rooms' budget is dealt anew among the client addresses that read: long enough
/// to measure what an address takes, short enough that a flood is met within a second.
pub const SHARE_EPOCH: Duration = Duration::from_millis(250);

/// 🍰️ What one client address may still take of the rooms' budget: its equal part and its fair
/// part — two levels of bytes like the whole's ([`Outflow`]) —, how many of its readers are alive,
/// and what the next deal goes by: the bytes it drew since the last deal, the rate measured up to
/// that deal, and whether a page was refused it since the last deal or during the one before.
#[derive(Debug)]
struct Share {
    readers: usize,
    equal: i128,
    fair: i128,
    filled: Instant,
    drawn: u64,
    rate: u64,
    held: bool,
    was_held: bool,
}

/// 🚰️ What all rooms together may still send, and what each client address may take of it.
///
/// The whole is a level of bytes — counted in billionths, so that a nanosecond of flow is a whole
/// number — that fills at the instance's presence budget up to a quarter of a second's worth and
/// that every page draws what it carries from. Every address that reads holds two such levels of
/// its own ([`Share`]). Its **equal part** fills at the budget divided by the number of addresses
/// reading, and a page is never refused while it is above nothing — whatever the other addresses
/// take. Its **fair part** fills at the rate the budget was last dealt at, and past its equal part
/// an address reads only while both its fair part and the whole are above nothing.
///
/// The budget is dealt max-min fairly every [`SHARE_EPOCH`]: an address that was refused no page
/// is given what it took (the mean of its last two measurements), and what those leave is split
/// evenly among the addresses that were refused one. An address that takes less than an even split
/// is never refused, so what is measured of it is what it asked for; an address that asks for more
/// than its split waits by itself, however many sockets it reads through. Over any stretch the
/// rooms send the budget, half a second's worth and at most one page per reader more.
#[derive(Debug)]
struct Outflow {
    per_second: u64,
    level: i128,
    filled: Instant,
    shares: HashMap<ClientKey, Share>,
    fair: u64,
    dealt: Instant,
}

impl Outflow {
    const BYTE: i128 = 1_000_000_000;

    /// 🏦️ The whole budget, undrawn, read by nobody, as of `now`.
    fn new(per_second: u64, now: Instant) -> Self {
        Self { per_second, level: Self::brim(per_second), filled: now, shares: HashMap::new(), fair: per_second, dealt: now }
    }

    /// 🥛️ The most a level filling at `per_second` holds: a quarter of a second's worth.
    fn brim(per_second: u64) -> i128 {
        i128::from(per_second) * Self::BYTE / 4
    }

    /// 🟰️ The rate every address's equal part fills at.
    fn even(&self) -> u64 {
        self.per_second / (self.shares.len() as u64).max(1)
    }

    /// 🫴️ Count one more reader of `client`, whose share starts full.
    fn attend(&mut self, client: ClientKey) {
        if let Some(share) = self.shares.get_mut(&client) {
            share.readers += 1;
            return;
        }
        let even = self.per_second / (self.shares.len() as u64 + 1);
        self.shares.insert(client, Share { readers: 1, equal: Self::brim(even), fair: Self::brim(self.fair), filled: self.filled, drawn: 0, rate: 0, held: false, was_held: false });
    }

    /// 🫳️ Count one reader of `client` less, forgetting the share with its last.
    fn depart(&mut self, client: ClientKey) {
        let Some(share) = self.shares.get_mut(&client) else { return };
        share.readers = share.readers.saturating_sub(1);
        if share.readers == 0 {
            self.shares.remove(&client);
        }
    }

    /// 🃏️ Deal the budget anew once an epoch has passed since the last deal.
    fn deal(&mut self, now: Instant) {
        let span = now.saturating_duration_since(self.dealt);
        if span < SHARE_EPOCH {
            return;
        }
        self.dealt = now;
        let mut modest = Vec::new();
        let mut open = 0u64;
        for share in self.shares.values_mut() {
            let rate = u64::try_from(u128::from(share.drawn) * 1_000_000_000 / span.as_nanos().max(1)).unwrap_or(u64::MAX);
            open += 1;
            if !(share.held || share.was_held) {
                modest.push(rate / 2 + share.rate / 2);
            }
            (share.rate, share.drawn, share.was_held, share.held) = (rate, 0, share.held, false);
        }
        modest.sort_unstable();
        let (mut left, mut largest) = (self.per_second, 0);
        for demand in modest {
            if u128::from(demand) * u128::from(open) > u128::from(left) {
                break;
            }
            (left, open, largest) = (left - demand, open - 1, demand);
        }
        self.fair = left.checked_div(open).unwrap_or(left + largest);
    }

    /// 🔓️ Fill up to `now` and say whether `client` may read a page; a refusal is remembered for
    /// the next deal.
    fn open(&mut self, client: ClientKey, now: Instant) -> bool {
        self.deal(now);
        let flowed = now.saturating_duration_since(self.filled).as_nanos() as i128 * i128::from(self.per_second);
        self.level = (self.level + flowed).min(Self::brim(self.per_second));
        self.filled = self.filled.max(now);
        let (even, fair, whole) = (self.even(), self.fair, self.level > 0);
        let Some(share) = self.shares.get_mut(&client) else { return whole };
        let elapsed = now.saturating_duration_since(share.filled).as_nanos() as i128;
        share.equal = (share.equal + elapsed * i128::from(even)).min(Self::brim(even));
        share.fair = (share.fair + elapsed * i128::from(fair)).min(Self::brim(fair));
        share.filled = share.filled.max(now);
        let open = share.equal > 0 || (share.fair > 0 && whole);
        share.held |= !open;
        open
    }

    /// 📉️ Draw the bytes a page carried to `client`.
    fn draw(&mut self, client: ClientKey, bytes: usize) {
        self.level -= bytes as i128 * Self::BYTE;
        if let Some(share) = self.shares.get_mut(&client) {
            share.equal -= bytes as i128 * Self::BYTE;
            share.fair -= bytes as i128 * Self::BYTE;
            share.drawn += bytes as u64;
        }
    }
}

/// 👥️ Every presence room of the instance, keyed by presence lane. Latest-state: a room keeps each
/// session's newest entry only and queues nothing for anybody — a socket reads it through its own
/// [`RoomReader`]. A room lives while it has a member or a reader.
pub struct PresenceRooms {
    rooms: ShardedMap<String, Room>,
    departures_kept: usize,
    outflow: Option<StdMutex<Outflow>>,
}

impl Default for PresenceRooms {
    fn default() -> Self {
        Self::new(&PresenceSettings::default(), Instant::now())
    }
}

impl PresenceRooms {
    /// 🌱️ No rooms, as of `now`: each room will remember [`PresenceSettings::departures_kept`]
    /// departures, and all of them together send [`PresenceSettings::max_bytes_per_second`].
    pub fn new(settings: &PresenceSettings, now: Instant) -> Self {
        let outflow = settings.max_bytes_per_second.map(|per_second| StdMutex::new(Outflow::new(per_second, now)));
        Self { rooms: ShardedMap::new(), departures_kept: settings.departures_kept, outflow }
    }

    /// 🪣️ The rooms' budget, when the instance states one.
    fn outflow(&self) -> Option<std::sync::MutexGuard<'_, Outflow>> {
        self.outflow.as_ref().map(|outflow| outflow.lock().unwrap_or_else(|poisoned| poisoned.into_inner()))
    }

    /// 🟢️ Whether the rooms' budget and `client`'s share of it allow reading another page at `now`.
    fn may_send(&self, client: ClientKey, now: Instant) -> bool {
        self.outflow().is_none_or(|mut outflow| outflow.open(client, now))
    }

    /// ➖️ Count the bytes a page carried to `client` against the rooms' budget and its share.
    fn sent(&self, client: ClientKey, bytes: usize) {
        if let Some(mut outflow) = self.outflow() {
            outflow.draw(client, bytes);
        }
    }

    /// 📖️ Start reading a room from nothing on behalf of `client`, the address whose share of the
    /// rooms' budget the pages are drawn from: the first page replaces whatever the reader's client
    /// held and is led by `lead`'s entry, when it names a member. Dropping the reader ends the read.
    pub fn read(self: &Arc<Self>, lane: &str, lead: Option<&str>, client: ClientKey) -> RoomReader {
        let changes = self.rooms.mutate_or_default(lane.to_string(), |room| {
            room.readers += 1;
            room.changes.subscribe()
        });
        if let Some(mut outflow) = self.outflow() {
            outflow.attend(client);
        }
        RoomReader { rooms: Arc::clone(self), lane: lane.to_string(), lead: lead.map(str::to_string), client, changes, seen: 0, target: 0, resume: 0, departures: 0, fresh: true, behind: true }
    }

    /// 🚪️ Add `entry`'s session to a room.
    pub fn join(&self, lane: &str, entry: &PresenceEntry) {
        let Some(wire) = wire(entry) else { return };
        self.rooms.mutate_or_default(lane.to_string(), |room| room.seat(Member { session: entry.session.clone(), wire }));
    }

    /// ✏️ Replace a member's entry; a session that is not in the room changes nothing.
    pub fn update(&self, lane: &str, entry: &PresenceEntry) {
        let Some(wire) = wire(entry) else { return };
        self.rooms.with_mut(lane, |room| {
            if let Some(room) = room.filter(|room| room.seated.contains_key(&entry.session)) {
                room.seat(Member { session: entry.session.clone(), wire });
            }
        });
    }

    /// 🚶️ Remove a session and remember its departure.
    pub fn leave(&self, lane: &str, session: &str) {
        self.rooms.with_mut(lane, |room| room.is_some_and(|room| room.vacate(session, self.departures_kept)));
        self.rooms.remove_if(lane, Room::deserted);
    }

    /// 📋️ Every member of a room, ordered by session.
    pub fn roster(&self, lane: &str) -> Vec<PresenceEntry> {
        let mut roster: Vec<PresenceEntry> = self.rooms.with(lane, |room| room.map(|room| room.seats.iter().flatten().filter_map(|member| serde_json::from_str(&member.wire).ok()).collect()).unwrap_or_default());
        roster.sort_by(|left, right| left.session.cmp(&right.session));
        roster
    }

    /// 🔢️ How many rooms have a member or a reader.
    pub fn rooms(&self) -> usize {
        self.rooms.len()
    }

    fn release(&self, lane: &str, client: ClientKey) {
        self.rooms.with_mut(lane, |room| {
            if let Some(room) = room {
                room.readers = room.readers.saturating_sub(1);
            }
        });
        self.rooms.remove_if(lane, Room::deserted);
        if let Some(mut outflow) = self.outflow() {
            outflow.depart(client);
        }
    }
}

/// 🧾️ `entry` as every reader is sent it.
fn wire(entry: &PresenceEntry) -> Option<Box<str>> {
    serde_json::to_string(entry).ok().map(String::into_boxed_str)
}

/// 🔤️ `text` as a JSON string.
fn quoted(text: &str) -> String {
    serde_json::Value::from(text).to_string()
}

/// 🔗️ Append one JSON value to a comma-joined list.
fn join(list: &mut String, value: &str) {
    if !list.is_empty() {
        list.push(',');
    }
    list.push_str(value);
}

/// 🔖️ Where one socket stands in one room: a few numbers, whatever the room's size. A reader goes
/// round the room's seats and is sent every entry that changed since its last round began — the
/// newest state, once per round however often it changed — so when a round does not fit one page,
/// every changed session still gets its turn before any gets a second. A page costs what it
/// carries, not what the room holds.
///
/// A room's departures are remembered up to [`PresenceSettings::departures_kept`]; a reader that
/// missed one that was forgotten starts over with a page that replaces what its client held.
pub struct RoomReader {
    rooms: Arc<PresenceRooms>,
    lane: String,
    lead: Option<String>,
    client: ClientKey,
    changes: watch::Receiver<u64>,
    seen: u64,
    target: u64,
    resume: usize,
    departures: u64,
    fresh: bool,
    behind: bool,
}

impl RoomReader {
    /// 📄️ Read what to send next at `now`, at most `budget` bytes of it — or one entry or
    /// departure, when the first alone is larger — and nothing while the rooms are at their budget
    /// or the reader's address is past its share of it ([`PresenceSettings::max_bytes_per_second`]):
    /// what is unsent then stays unsent. The first page of a member (a reader with a `lead`) does
    /// not wait for the budget, which it is counted against like any other: a session learns at
    /// once that it joined, and how many sessions join is for the edge to bound.
    pub fn page(&mut self, budget: usize, now: Instant) -> Page {
        let Self { rooms, lane, lead, client, changes, seen, target, resume, departures, fresh, behind } = self;
        if !(*fresh && lead.is_some()) && !rooms.may_send(*client, now) {
            *behind = true;
            return Page::default();
        }
        changes.borrow_and_update();
        let page = rooms.rooms.with(lane.as_str(), |room| {
            let mut page = Page::default();
            let Some(room) = room else {
                *behind = false;
                return page;
            };
            let leading = lead.as_deref().and_then(|session| room.seated.get(session)).copied();
            if *fresh || *departures < room.forgotten {
                (*seen, *target, *resume, *departures, *fresh) = (0, 0, 0, room.clock, false);
                page.replace = true;
                if let Some(member) = leading.and_then(|seat| room.seats[seat].as_ref()) {
                    page.entries.push_str(&member.wire);
                }
            }
            let unsent = room.departed.partition_point(|(clock, _)| *clock <= *departures);
            for (clock, session) in room.departed.range(unsent..) {
                let session = quoted(session);
                if !page.fits(session.len(), budget) {
                    break;
                }
                join(&mut page.left, &session);
                *departures = *clock;
            }
            let mut round_done = true;
            if *seen < room.clock {
                if *target <= *seen {
                    (*target, *resume) = (room.clock, 0);
                }
                while let Some(seat) = room.next_changed(*resume, *seen) {
                    if let Some(member) = room.seats[seat].as_ref().filter(|_| *seen > 0 || leading != Some(seat)) {
                        if !page.fits(member.wire.len(), budget) {
                            round_done = false;
                            break;
                        }
                        join(&mut page.entries, &member.wire);
                    }
                    *resume = seat + 1;
                }
                if round_done {
                    (*seen, *resume) = (*target, 0);
                }
            }
            *behind = !round_done || *seen < room.clock || room.departed.back().is_some_and(|(clock, _)| *clock > *departures);
            page
        });
        rooms.sent(*client, page.bytes());
        page
    }

    /// 🔔️ Wait until the room has something this reader was not sent: at once while the last page
    /// left something behind, else when the room next changes.
    pub async fn unsent(&mut self) {
        if self.behind {
            return;
        }
        if self.changes.changed().await.is_err() {
            std::future::pending::<()>().await;
        }
        self.behind = true;
    }

    /// 👁️ Whether the room has something this reader was not sent, without waiting.
    pub fn has_unsent(&self) -> bool {
        self.behind || self.changes.has_changed().unwrap_or(false)
    }

    /// 🆕️ Whether this reader has not read its first page yet.
    pub fn fresh(&self) -> bool {
        self.fresh
    }

    /// 🏃️ Whether this reader is still on the round that sends it the whole roster.
    pub fn arriving(&self) -> bool {
        self.behind && self.seen == 0
    }
}

impl Drop for RoomReader {
    fn drop(&mut self) {
        self.rooms.release(&self.lane, self.client);
    }
}

/// 📃️ What one reader is sent next of one room: entries and departed sessions, already JSON.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Page {
    /// 📇️ The entries, as JSON objects joined by commas.
    pub entries: String,
    /// 🚷️ The sessions that left, as JSON strings joined by commas.
    pub left: String,
    /// 🔁️ Whether this page replaces everything the reader was sent of the room before.
    pub replace: bool,
}

impl Page {
    /// 🕳️ Whether there is nothing to send.
    pub fn is_empty(&self) -> bool {
        !self.replace && self.bytes() == 0
    }

    /// ⚖️ The bytes of entries and departures this page carries.
    pub fn bytes(&self) -> usize {
        self.entries.len() + self.left.len()
    }

    /// 🧮️ Whether `more` bytes and their separator still fit `budget` — always, in an empty page.
    fn fits(&self, more: usize, budget: usize) -> bool {
        self.bytes() == 0 || self.bytes() + 1 + more <= budget
    }

    /// 👋️ This page as the `welcome` frame of `session`.
    pub fn welcome(&self, session: &str, colour: u8) -> String {
        format!(r#"{{"type":"welcome","session":{},"colour":{colour},"roster":[{}]}}"#, quoted(session), self.entries)
    }

    /// 📦️ This page as a `batch` frame.
    pub fn batch(&self) -> String {
        format!(r#"{{"type":"batch","entries":[{}],"left":[{}]}}"#, self.entries, self.left)
    }

    /// 🔭️ This page as the `watched` frame of `scope`, a `snapshot` when it replaces.
    pub fn watched(&self, scope: &str) -> String {
        format!(r#"{{"type":"watched","scope":{},"entries":[{}],"left":[{}]{}}}"#, quoted(scope), self.entries, self.left, if self.replace { r#","snapshot":true"# } else { "" })
    }
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
/// Hand-rolled instead of pulled from a middleware library on purpose: the whole behaviour is five
/// headers — a dependency to produce them would be a dependency to audit them. The grant never
/// allows credentials: a caller identifies itself with a bearer it sets explicitly, never with
/// anything a browser attaches by itself, so no page gains another page's ambient authority here.
pub async fn cors_middleware(request: Request, next: axum::middleware::Next) -> Response {
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

/// 🪞️ Write the CORS grant onto a response: the caller's own origin (never `*`, never with
/// credentials), and the verbs and headers this control plane actually uses.
pub fn apply_cors_headers(headers: &mut HeaderMap, origin: Option<&HeaderValue>) {
    if let Some(origin) = origin {
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
        headers.insert(header::VARY, HeaderValue::from_static("origin"));
    }
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET, POST, PUT, HEAD, DELETE, OPTIONS"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("authorization, content-type, x-semio-capability"));
}
//#endregion 🔖️Cors

//#region 🔖️Edge
/// 🧭️ Where the address a client is throttled as comes from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ClientAddressing {
    /// 🔌️ The peer of the connection: right wherever clients connect to the instance themselves.
    #[default]
    Peer,
    /// 🛡️ The address a TLS-terminating proxy — the only peer this instance then has — wrote last
    /// into [`FORWARDED_FOR`]; the peer's own address when the header is absent or names none. Right
    /// only where nothing but that proxy can reach the instance: any other peer could name an
    /// address of its choice.
    Forwarded,
}

/// 🏷️ The header a terminating proxy names the client's address in.
pub const FORWARDED_FOR: &str = "x-forwarded-for";

/// 🛣️ The path of the one `POST` that reads.
pub const QUERIES_PATH: &str = "/queries";

/// 🏷️ The address `headers` and `peer` identify a client as, under `addressing`.
pub fn client_key(addressing: ClientAddressing, headers: &HeaderMap, peer: Option<SocketAddr>) -> ClientKey {
    let forwarded = match addressing {
        ClientAddressing::Peer => None,
        ClientAddressing::Forwarded => headers.get_all(FORWARDED_FOR).iter().next_back().and_then(|value| value.to_str().ok()).and_then(forwarded_address),
    };
    ClientKey::of(forwarded.or(peer.map(|peer| peer.ip())).unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)))
}

/// 🗂️ The bucket a request spends from: a websocket upgrade, a read (`GET`, `HEAD`, `OPTIONS` and
/// the query route) or a write (everything else).
pub fn request_class(method: &Method, path: &str, headers: &HeaderMap) -> RequestClass {
    if headers.get(header::UPGRADE).and_then(|value| value.to_str().ok()).is_some_and(|value| value.eq_ignore_ascii_case("websocket")) {
        return RequestClass::Upgrade;
    }
    if method == Method::GET || method == Method::HEAD || method == Method::OPTIONS || path == QUERIES_PATH {
        return RequestClass::Read;
    }
    RequestClass::Write
}

/// 🚧️ The edge of one instance as its throttle middleware needs it: the bookkeeping and where
/// client addresses come from. A bundle of handles, cheap to clone.
#[derive(Clone)]
pub struct Edge {
    pub throttle: Arc<Throttle>,
    pub addressing: ClientAddressing,
}

/// ⏱️ How long a refused request is given to deliver the rest of its body, so that the connection
/// it came on can carry the next request.
pub const REFUSED_BODY_PATIENCE: Duration = Duration::from_millis(500);

/// 🚦️ Spend the caller's token for this request, take its body, and count it in flight while it
/// is served; a refused request is answered `429` (the address's allowance) or `503` (an instance
/// cap) with `Retry-After` before anything behind this layer runs.
///
/// **The body is taken here**, whole and at most [`Limits::body_bytes`] of it, on an instance
/// that states that limit. Before the request counts in flight: a body that trickles in holds one
/// connection for at most [`Limits::body_patience`] (`408`) and never one of the places the
/// instance serves requests in. And of a refused request too, for at most [`REFUSED_BODY_PATIENCE`]: a
/// response sent while the body is still on its way makes the transport close the connection, so a
/// flood of refused requests would open connections as fast as it is refused — and behind a proxy
/// on the same host those are the proxy's connections to this instance, whose ports then run out
/// for every client. A body past the limit is not taken (`413`), and that connection closes.
///
/// It is a layer whoever composes the final router applies ([`Server::throttled`]), outside every
/// layer that does work per request, so a refused request costs a hash lookup and reading what it
/// sent.
pub async fn throttle_middleware(State(edge): State<Edge>, request: Request, next: axum::middleware::Next) -> Response {
    let peer = request.extensions().get::<ConnectInfo<SocketAddr>>().map(|info| info.0);
    let client = client_key(edge.addressing, request.headers(), peer);
    let class = request_class(request.method(), request.uri().path(), request.headers());
    let limit = edge.throttle.limits().body_bytes.filter(|_| class != RequestClass::Upgrade);
    if let Err(refusal) = edge.throttle.admit(client, class, std::time::Instant::now()) {
        if let Some(limit) = limit {
            let _ = body_taken(request, limit, REFUSED_BODY_PATIENCE).await;
        }
        return ServerError::from(refusal).into_response();
    }
    let request = match limit {
        Some(limit) => match body_taken(request, limit, edge.throttle.limits().body_patience).await {
            Ok(request) => request,
            Err(error) => return error.into_response(),
        },
        None => request,
    };
    match edge.throttle.enter() {
        Ok(_serving) => next.run(request).await,
        Err(refusal) => ServerError::from(refusal).into_response(),
    }
}

/// 📥️ `request` with its whole body in hand: at most `limit` bytes, delivered within `patience`.
/// A body that declares or delivers more is [`ServerError::PayloadTooLarge`] and is not read past
/// the limit; one that takes longer is [`ServerError::Stalled`].
pub async fn body_taken(request: Request, limit: usize, patience: Duration) -> Result<Request, ServerError> {
    let declared = request.headers().get(header::CONTENT_LENGTH).and_then(|value| value.to_str().ok()).and_then(|value| value.parse::<usize>().ok());
    if declared.is_some_and(|declared| declared > limit) {
        return Err(ServerError::PayloadTooLarge);
    }
    let (parts, body) = request.into_parts();
    let taken = async {
        let mut chunks = body.into_data_stream();
        let mut bytes = Vec::with_capacity(declared.unwrap_or_default());
        while let Some(chunk) = chunks.next().await {
            let chunk = chunk.map_err(|error| ServerError::BadRequest(error.to_string()))?;
            if bytes.len() + chunk.len() > limit {
                return Err(ServerError::PayloadTooLarge);
            }
            bytes.extend_from_slice(&chunk);
        }
        Ok(bytes)
    };
    match tokio::time::timeout(patience, taken).await {
        Ok(bytes) => Ok(Request::from_parts(parts, axum::body::Body::from(bytes?))),
        Err(_) => Err(ServerError::Stalled),
    }
}

/// 📏️ Bound what the protocol layer of a socket takes of one inbound message: a frame or message
/// past `inbound` bytes fails the socket while it is being read, so nothing larger is ever
/// buffered, and the read buffer itself is no larger.
pub fn bounded_socket(upgrade: WebSocketUpgrade, inbound: usize) -> WebSocketUpgrade {
    upgrade.max_message_size(inbound).max_frame_size(inbound).read_buffer_size(inbound)
}

/// 📏️ The most the durable-lane socket takes of one inbound message: its client only ever closes.
pub const EVENT_STREAM_INBOUND_BYTES: usize = 1024;

/// 🚀️ Make one accepted connection send each write at once instead of holding a small one back
/// for the peer's acknowledgement (Nagle's algorithm): the answers and presence frames of this
/// gateway are small and complete, and a held one is latency its reader sees. Meant for
/// `listener.tap_io(undelayed)`.
pub fn undelayed(connection: &mut tokio::net::TcpStream) {
    if let Err(error) = connection.set_nodelay(true) {
        crate::report("a connection keeps delaying small writes", &error);
    }
}
//#endregion 🔖️Edge

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
    /// 🗄️ The authority store the bus commits into, shared with it: history is read and the outbox
    /// drained through this handle, so neither ever waits for the turns queued on the bus.
    pub store: SharedStore<I::AuthorityStore>,
    /// 🧵️ The cross-actor workflows this instance reacts with, over the outbox the bus commits into.
    /// Held here rather than by whoever calls [`ServerBuilder::build`] because the outbox and the
    /// workflows that drain it are two halves of one exactly-once guarantee: a runner living
    /// somewhere else is a runner that can be forgotten, and a forgotten runner is a queue that
    /// grows forever while every event looks delivered.
    pub sagas: Arc<Mutex<ServerSagas<I>>>,
    /// 🔭️ The rebuildable read models every query answers from, shared: queries read them side by
    /// side, and only whoever folds events into them takes them alone.
    pub projections: SharedStore<I::ProjectionStore>,
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
    /// 🚦️ The bookkeeping of this instance's edge limits: rates per client address, sockets,
    /// requests in flight.
    pub throttle: Arc<Throttle>,
    /// 🧭️ Where a client's address comes from.
    pub addressing: ClientAddressing,
    /// 🛤️ The command lane: what is queued for the bus, and whether a runner is on it.
    lane: Arc<CommandLane>,
    /// 🕰️ The instance's hybrid logical clock, advanced once per stamped command.
    clock: Arc<StdMutex<HybridLogicalClock>>,
}

impl<I: ServerInstance> Clone for ServerState<I> {
    /// 🧬️ Handle-by-handle, never field-by-field through `derive(Clone)`: the derive would demand
    /// `I: Clone` of the instance MARKER type, which is not a value anybody clones.
    fn clone(&self) -> Self {
        Self {
            authority: Arc::clone(&self.authority),
            store: Arc::clone(&self.store),
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
            throttle: Arc::clone(&self.throttle),
            addressing: self.addressing,
            lane: Arc::clone(&self.lane),
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

    /// ⚖️ Evaluate one policy question, turning a denial into [`ServerError::Forbidden`]. The
    /// engine's reason travels in the error for whoever handles it in process; the caller of a route
    /// is told `forbidden` and nothing of what was missing.
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

    /// 🏷️ The address this caller is throttled as.
    pub fn client(&self, headers: &HeaderMap, peer: Option<SocketAddr>) -> ClientKey {
        client_key(self.addressing, headers, peer)
    }

    /// 🔌️ Count one more open socket of this caller until the permit is dropped, or refuse it at the
    /// caller's or the instance's socket cap.
    pub fn open_socket(&self, headers: &HeaderMap, peer: Option<SocketAddr>) -> Result<SocketPermit, ServerError> {
        Ok(self.throttle.open_socket(self.client(headers, peer), std::time::Instant::now())?)
    }

    /// 🚧️ What [`throttle_middleware`] needs of this instance.
    pub fn edge(&self) -> Edge {
        Edge { throttle: Arc::clone(&self.throttle), addressing: self.addressing }
    }

    /// 📜️ Every durable event of `actor` after `since`, read from the shared store: the bus and the
    /// turns queued on it are never taken.
    pub async fn replay_events(&self, actor: &ActorKey, since: u64) -> Result<Vec<EventRecord>, ServerError> {
        Ok(self.store.read().await.events_since(actor, since).await?)
    }

    /// 🚰️ Hand up to `limit` committed outbox rows to this instance's workflows and run every
    /// follow-up command as its own turn, returning what each turn answered.
    ///
    /// The two halves are deliberately one call. A drain that only *returned* commands would leave
    /// the rows acknowledged while their consequences sat in a `Vec` the caller might drop, which is
    /// precisely the "state changed but the world was never told" the transactional outbox exists to
    /// rule out. The rows are read and acknowledged through the shared store, never through the bus,
    /// and both locks are released before the first follow-up is submitted, because a turn takes the
    /// store itself.
    ///
    /// Exactly-once is a property of the *queue*, not of this call: a row leaves `pending` only once
    /// it is marked delivered, so a restart between two drains re-delivers nothing, and a crash
    /// between mapping and marking re-delivers a row whose follow-up command carries an
    /// [`IdempotencyKey`](crate::contract::IdempotencyKey) and is deduplicated by the bus.
    pub async fn drain_sagas(&self, limit: usize) -> Vec<CommandOutcome> {
        let commands = {
            let mut sagas = self.sagas.lock().await;
            let mut store = self.store.write().await;
            sagas.drain_outbox(&mut *store, limit).await
        };
        self.submit_all(commands).await
    }

    /// 🧮️ How many events and outbox rows the command lane has committed since this instance was
    /// built. A command answered through the lane is counted before it is answered, so whoever
    /// relays the outbox or folds read models on behalf of a request compares these with what it
    /// last relayed and folded and asks the store only when they moved.
    pub fn committed(&self) -> Committed {
        Committed { events: self.lane.events.load(Ordering::Acquire), outbox: self.lane.outbox.load(Ordering::Acquire) }
    }

    /// 📨️ Run one command through the command lane and answer its outcome.
    pub async fn submit(&self, envelope: CommandEnvelope) -> CommandOutcome {
        let lost = unanswered(&envelope);
        self.submit_all(vec![envelope]).await.pop().unwrap_or(lost)
    }

    /// 📦️ Run commands through the command lane, in order, and answer each one's outcome.
    ///
    /// The lane is how every command of this instance reaches the bus: commands are queued, and
    /// one runner at a time takes whatever is queued — up to [`COMMAND_BATCH`] — stamps each with
    /// the instance clock, runs them as one batch of turns ([`CommandBus::submit_batch`]) and
    /// publishes the accepted events on their durable lanes. A burst of commands therefore becomes
    /// a few batches and a few durable writes, while a lone command is a batch of one and waits for
    /// nobody. The runner is a task of its own: a caller that stops waiting (a client that hung up)
    /// cancels its wait, never a turn or a commit half-way.
    pub async fn submit_all(&self, envelopes: Vec<CommandEnvelope>) -> Vec<CommandOutcome> {
        let mut waiting = Vec::with_capacity(envelopes.len());
        {
            let mut queue = self.lane.queue.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            for envelope in envelopes {
                let (answer, answered) = oneshot::channel();
                waiting.push((answered, unanswered(&envelope)));
                queue.push_back((envelope, answer));
            }
        }
        self.run_lane();
        let mut outcomes = Vec::with_capacity(waiting.len());
        for (answered, lost) in waiting {
            outcomes.push(answered.await.unwrap_or(lost));
        }
        outcomes
    }

    /// 🏃️ Start the lane's runner unless one is running; it ends once the queue is empty.
    fn run_lane(&self) {
        if self.lane.running.swap(true, Ordering::AcqRel) {
            return;
        }
        let state = self.clone();
        tokio::spawn(async move {
            loop {
                let batch: Vec<Queued> = {
                    let mut queue = state.lane.queue.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                    let take = queue.len().min(COMMAND_BATCH);
                    queue.drain(..take).collect()
                };
                if batch.is_empty() {
                    state.lane.running.store(false, Ordering::Release);
                    let queued = !state.lane.queue.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).is_empty();
                    if queued && !state.lane.running.swap(true, Ordering::AcqRel) {
                        continue;
                    }
                    return;
                }
                let (envelopes, answers): (Vec<CommandEnvelope>, Vec<oneshot::Sender<CommandOutcome>>) = batch.into_iter().unzip();
                let stamped = envelopes.into_iter().map(|envelope| (envelope, state.now())).collect();
                let (outcomes, committed) = {
                    let mut bus = state.authority.lock().await;
                    (bus.submit_batch(stamped).await, bus.committed())
                };
                state.lane.events.store(committed.events, Ordering::Release);
                state.lane.outbox.store(committed.outbox, Ordering::Release);
                for (outcome, answer) in outcomes.into_iter().zip(answers) {
                    if let CommandOutcome::Accepted { events, .. } = &outcome {
                        for event in events {
                            if let Ok(bytes) = serde_json::to_vec(event) {
                                state.fanout.publish(&stream_lane(&event.stream), bytes);
                            }
                        }
                    }
                    let _ = answer.send(outcome);
                }
            }
        });
    }
}

/// 📮️ The most queued commands one batch of the command lane takes.
pub const COMMAND_BATCH: usize = 64;

/// 📨️ One queued command and where its outcome is answered.
type Queued = (CommandEnvelope, oneshot::Sender<CommandOutcome>);

/// 🛤️ The command lane's queue, whether a runner is draining it, and how many events and outbox
/// rows the bus had committed when the runner last answered.
#[derive(Default)]
struct CommandLane {
    queue: StdMutex<VecDeque<Queued>>,
    running: AtomicBool,
    events: AtomicU64,
    outbox: AtomicU64,
}

/// 🕳️ The outcome of a command whose runner went away before answering it: retryable, under the
/// same idempotency key.
fn unanswered(envelope: &CommandEnvelope) -> CommandOutcome {
    let receipt = CommandReceipt { command_id: envelope.command_id.clone(), actor: envelope.target.clone(), revision: Revision(0), accepted_at: HybridLogicalClock::default() };
    CommandOutcome::Rejected { receipt, reason: Rejection::ActorUnavailable { detail: UNAVAILABLE.to_string() }, notices: Vec::new() }
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
/// turn runs — a client may address a command, it may never assert who is sending it. The command
/// lane ([`ServerState::submit`]) publishes accepted events onto the actor's durable lane, so live
/// subscribers see them without polling.
///
/// A command a module classes under a named allowance ([`ServerModule::command_allowance`]) spends
/// a token of its caller's address first — `429` naming the allowance when there is none — and is
/// handed the token back once its outcome shows that it kept nothing ([`kept`]). The token is spent
/// before the turn and held by nobody: a caller that hangs up while its command commits has paid.
pub async fn post_command<I: ServerInstance>(headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>, WireJson(envelope): WireJson<CommandEnvelope>) -> Result<Json<CommandOutcome>, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    let mut envelope = envelope;
    envelope.principal = resolved.principal;
    envelope.session = resolved.session;
    envelope.device = resolved.device;
    let Some(allowance) = state.modules.iter().find_map(|module| module.command_allowance(&envelope)) else {
        return Ok(Json(state.submit(envelope).await));
    };
    let client = state.client(&headers, Some(peer));
    if let Err(Refusal::Throttled { retry_after } | Refusal::Overloaded { retry_after }) = state.throttle.spend(client, allowance, std::time::Instant::now()) {
        return Err(ServerError::Throttled { wait: retry_after, allowance: Some(allowance) });
    }
    let outcome = state.submit(envelope).await;
    if !kept(&outcome) {
        state.throttle.refund(client, allowance, std::time::Instant::now());
    }
    Ok(Json(outcome))
}

/// 🧺️ Whether a command made its instance keep something: it committed an event, or was deferred
/// to a process that may. A rejection and the replay of a command already answered kept nothing.
pub fn kept(outcome: &CommandOutcome) -> bool {
    match outcome {
        CommandOutcome::Accepted { events, .. } | CommandOutcome::Transformed { canonical_events: events, .. } => !events.is_empty(),
        CommandOutcome::Pending { .. } => true,
        CommandOutcome::Rejected { .. } => false,
    }
}

/// ❓️ Answer one query from the projections, after checking the caller may read it.
pub async fn post_query<I: ServerInstance>(headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>, WireJson(envelope): WireJson<QueryEnvelope>) -> Result<Json<QueryResult>, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    let mut envelope = envelope;
    envelope.principal = resolved.principal;
    state.authorize(&PolicyRequest { point: PolicyPoint::QueryAccess, principal: envelope.principal.clone(), scope: Some(envelope.scope.clone()), resource: envelope.kind.clone(), action: "read".to_string() })?;
    let handler = state.queries.get_cloned(&envelope.kind).ok_or_else(|| ServerError::NotFound(format!("no handler for query kind '{}'", envelope.kind)))?;
    let projections = state.projections.read().await;
    Ok(Json(handler.handle(&envelope, &*projections).await?))
}

/// 💨️ Publish one ephemeral frame onto its scope's lossy lane. Nothing is persisted and nothing is
/// replayed — a subscriber that was not listening simply missed it.
pub async fn post_ephemeral<I: ServerInstance>(headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>, WireJson(frame): WireJson<EphemeralFrame>) -> Result<Json<usize>, ServerError> {
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
    let permit = state.open_socket(&headers, Some(peer))?;
    Ok(bounded_socket(ws, EVENT_STREAM_INBOUND_BYTES).on_upgrade(move |socket| handle_event_stream(socket, actor, query.since, state, permit)))
}

async fn handle_event_stream<I: ServerInstance>(socket: WebSocket, actor: ActorKey, since: u64, state: ServerState<I>, _permit: SocketPermit) {
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
/// whether it may publish states in the room it joined, and the client address whose share of the
/// rooms' budget its frames are drawn from.
#[derive(Clone, Debug, PartialEq)]
pub struct PresenceGrant {
    pub principal: Principal,
    pub may_publish: bool,
    pub client: ClientKey,
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
    Ok(PresenceGrant { principal, may_publish, client: state.client(headers, peer) })
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
/// then at most one bounded `batch` per tick while the room has something the socket was not sent.
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
    let permit = state.open_socket(&headers, Some(peer))?;
    Ok(bounded_socket(ws, state.presence_settings.inbound_bytes()).protocols([PRESENCE_PROTOCOL_V1]).on_upgrade(move |socket| async move {
        let _permit = permit;
        let (mut sender, mut receiver) = socket.split();
        run_presence(&state, &scope, &surface, &grant, &mut sender, &mut receiver).await;
    }))
}

/// 🔁️ One presence session over any message sink and stream — the socket in production, channels
/// in a test. Joins the room (leasing a colour from [`Presence`]), sends the `welcome`, then serves
/// until the peer closes, stays idle past [`PresenceSettings::idle`], takes longer than that to be
/// sent one frame, or is kicked; it always leaves the room and stops watching on the way out.
///
/// The session reads its room through a [`RoomReader`]: while the room has something it was not
/// sent, one frame of at most [`PresenceSettings::max_frame_bytes`] per tick — and, while it is
/// still being sent the roster it joined, one frame after the other as fast as it takes them.
pub async fn run_presence<I, S, R, E>(state: &ServerState<I>, scope: &Scope, surface: &str, grant: &PresenceGrant, sink: &mut S, stream: &mut R)
where
    I: ServerInstance,
    S: Sink<Message> + Unpin,
    R: Stream<Item = Result<Message, E>> + Unpin,
{
    let settings = *state.presence_settings;
    let lane = presence_lane(scope);
    let session = presence_session_id();
    let mut entry = PresenceEntry { colour: state.presence.acquire_colour(&lane, &session), session, surface: surface.to_string(), state: OpaqueJson::Null };
    let mut room = state.presence_rooms.read(&lane, Some(&entry.session), grant.client);
    state.presence_rooms.join(&lane, &entry);
    let seat = Seat { scope, lane: &lane, may_publish: grant.may_publish, patience: settings.idle };
    let mut watcher = Watcher::default();
    let mut budget = StateBudget::new(settings.max_states_per_second);
    let mut last_seen = Instant::now();
    let mut keepalive = tokio::time::interval_at(Instant::now() + settings.keepalive, settings.keepalive);
    let mut due = Instant::now();
    let mut open = feed(&mut room, &entry, &settings, sink).await.is_some();
    while open {
        tokio::select! {
            incoming = stream.next() => {
                last_seen = Instant::now();
                open = match incoming {
                    Some(Ok(Message::Text(text))) => match admit_frame(text.as_str(), &settings, &mut budget, Instant::now()) {
                        Ok(Some(PresenceFrame::State { state: shared })) => share_state(state, &seat, &mut entry, shared, sink).await,
                        Ok(Some(PresenceFrame::Watch { scopes, interval_ms })) => match admit_watch(state, &grant.principal, &scopes) {
                            Ok(scopes) => watcher.watch(state, &scopes, grant.client, watch_interval(interval_ms, &settings), &settings, sink).await,
                            Err(reason) => refuse(sink, reason, settings.idle).await,
                        },
                        Ok(_) => true,
                        Err(reason) => refuse(sink, reason, settings.idle).await,
                    },
                    Some(Ok(Message::Binary(_))) => refuse(sink, REFUSED_INVALID.to_string(), settings.idle).await,
                    Some(Ok(Message::Ping(payload))) => send(sink, Message::Pong(payload), settings.idle).await,
                    Some(Ok(Message::Pong(_))) => true,
                    Some(Ok(Message::Close(_))) | Some(Err(_)) | None => false,
                };
            }
            () = turn(&mut room, due) => {
                let fed = feed(&mut room, &entry, &settings, sink).await;
                open = fed.is_some();
                due = if fed == Some(true) && room.arriving() { Instant::now() } else { Instant::now() + settings.tick };
            }
            _ = keepalive.tick() => open = send(sink, Message::Ping(Vec::new().into()), settings.idle).await,
            _ = tokio::time::sleep_until(last_seen + settings.idle) => {
                send(sink, Message::Close(None), settings.idle).await;
                open = false;
            }
            _ = state.kicks.kicked(&entry.session) => open = false,
            () = next_flush(&mut watcher.ticks) => open = watcher.send(&settings, sink, false).await,
        }
    }
    drop(watcher);
    state.presence_rooms.leave(&lane, &entry.session);
    drop(room);
    state.presence.release_colour(&lane, &entry.session);
    state.kicks.forget(&entry.session);
}

/// 💺️ Where one presence session sits: its scope and lane, whether it may publish, and how long
/// it is given to take one frame.
struct Seat<'a> {
    scope: &'a Scope,
    lane: &'a str,
    may_publish: bool,
    patience: Duration,
}

/// ⏳️ Resolves once `room` has something its reader was not sent and the reader's next page is due.
async fn turn(room: &mut RoomReader, due: Instant) {
    room.unsent().await;
    tokio::time::sleep_until(due).await;
}

/// 📤️ Send a member the next page of its room: a `welcome` when the page replaces what the member
/// holds, a `batch` otherwise, nothing when there is nothing to send. Answers whether a frame went
/// out, or `None` when the peer no longer takes one.
async fn feed<S: Sink<Message> + Unpin>(room: &mut RoomReader, entry: &PresenceEntry, settings: &PresenceSettings, sink: &mut S) -> Option<bool> {
    let page = room.page(settings.max_frame_bytes, Instant::now());
    let frame = if page.replace {
        page.welcome(&entry.session, entry.colour)
    } else if page.is_empty() {
        return Some(false);
    } else {
        page.batch()
    };
    send(sink, Message::Text(frame.into()), settings.idle).await.then_some(true)
}

/// 📣️ Share one admitted state in the joined room — when the principal may publish and every
/// module's [`ServerModule::presence_admission`] accepts it — or answer why not.
async fn share_state<I: ServerInstance, S: Sink<Message> + Unpin>(state: &ServerState<I>, seat: &Seat<'_>, entry: &mut PresenceEntry, shared: OpaqueJson, sink: &mut S) -> bool {
    let verdict = if seat.may_publish { state.modules.iter().try_for_each(|module| module.presence_admission(seat.scope, &shared)) } else { Err(REFUSED_FORBIDDEN.to_string()) };
    match verdict {
        Ok(()) => {
            entry.state = shared;
            state.presence_rooms.update(seat.lane, entry);
            true
        }
        Err(reason) => refuse(sink, reason, seat.patience).await,
    }
}

/// 👀️ The watching half of one presence socket: one [`RoomReader`] per watched scope, in scope
/// order, the interval their changes are sent at, and the scope that goes first next time.
#[derive(Default)]
struct Watcher {
    scopes: Vec<(String, RoomReader)>,
    interval: Duration,
    ticks: Option<tokio::time::Interval>,
    next: usize,
}

impl Watcher {
    /// 🔁️ Replace the watch set: stop reading dropped scopes, start reading new ones on behalf of
    /// `client`, (re)start the interval when it changed, and send the new scopes their `snapshot`.
    async fn watch<I: ServerInstance, S: Sink<Message> + Unpin>(&mut self, state: &ServerState<I>, scopes: &BTreeSet<String>, client: ClientKey, interval: Duration, settings: &PresenceSettings, sink: &mut S) -> bool {
        self.scopes.retain(|(scope, _)| scopes.contains(scope));
        for scope in scopes {
            if !self.scopes.iter().any(|(watched, _)| watched == scope) {
                self.scopes.push((scope.clone(), state.presence_rooms.read(&presence_lane(&Scope(scope.clone())), None, client)));
            }
        }
        self.scopes.sort_by(|(left, _), (right, _)| left.cmp(right));
        if self.scopes.is_empty() {
            self.ticks = None;
        } else if self.ticks.is_none() || self.interval != interval {
            let mut ticks = tokio::time::interval_at(Instant::now() + interval, interval);
            ticks.set_missed_tick_behavior(MissedTickBehavior::Delay);
            self.ticks = Some(ticks);
        }
        self.interval = interval;
        self.send(settings, sink, true).await
    }

    /// 🕰️ Send one `watched` frame per scope that has something this socket was not sent — only
    /// the scopes that were sent nothing yet when `arrivals` — until one frame's allowance
    /// ([`PresenceSettings::max_frame_bytes`]) is spent over all of them. The scope the allowance
    /// ran out at goes first next time, so every watched scope gets its turn.
    async fn send<S: Sink<Message> + Unpin>(&mut self, settings: &PresenceSettings, sink: &mut S, arrivals: bool) -> bool {
        let count = self.scopes.len();
        let mut spent = 0;
        for offset in 0..count {
            let index = (self.next + offset) % count;
            let (scope, reader) = &mut self.scopes[index];
            if !(if arrivals { reader.fresh() } else { reader.has_unsent() }) {
                continue;
            }
            if spent >= settings.max_frame_bytes {
                self.next = index;
                break;
            }
            let page = reader.page(settings.max_frame_bytes - spent, Instant::now());
            spent += page.bytes();
            if !page.is_empty() && !send(sink, Message::Text(page.watched(scope).into()), settings.idle).await {
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

/// 📮️ Send one message, giving the peer `patience` to take it: a peer that does not is gone.
async fn send<S: Sink<Message> + Unpin>(sink: &mut S, message: Message, patience: Duration) -> bool {
    matches!(tokio::time::timeout(patience, sink.send(message)).await, Ok(Ok(())))
}

/// 🚫️ Answer a frame the server dropped with why.
async fn refuse<S: Sink<Message> + Unpin>(sink: &mut S, reason: String, patience: Duration) -> bool {
    match serde_json::to_string(&PresenceFrame::Refused { reason }) {
        Ok(text) => send(sink, Message::Text(text.into()), patience).await,
        Err(_) => true,
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
/// 🛣️ Which of the gateway's own route groups an instance mounts. The core — the instance document,
/// commands, queries, the presence socket and the durable lane — is always mounted; a group an
/// instance does not use is not a route it answers `403` on but a path that does not exist, so it
/// reads no body, asks no policy and is no surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteGroups {
    /// 💨️ `POST /scopes/{scope}/ephemeral`.
    pub ephemeral: bool,
    /// 🧱️ `GET`, `HEAD` and `PUT /blobs/{hash}`.
    pub blobs: bool,
    /// 🧩️ `GET /apps` and everything under it.
    pub apps: bool,
}

impl RouteGroups {
    /// 🌐️ Every group: the router of an instance that hosts blobs, static apps and ephemeral frames.
    pub const ALL: Self = Self { ephemeral: true, blobs: true, apps: true };
    /// 🎯️ The core alone: the router of an instance that is commands, queries and sockets.
    pub const CORE: Self = Self { ephemeral: false, blobs: false, apps: false };
}

/// 🪟️ How much of its definition an instance hands to whoever asks `GET /instance`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InstanceDisclosure {
    /// 📖️ The whole definition, policy templates included: right for an instance only its own
    /// operators reach.
    #[default]
    Whole,
    /// 🪟️ [`ServerInstanceDefinition::public`]: what a client needs to address the instance and
    /// nothing of how it authorizes.
    Public,
}

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
    limits: Limits,
    addressing: ClientAddressing,
    routes: RouteGroups,
    disclosure: InstanceDisclosure,
    activations: usize,
    snapshot_interval: u64,
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

    /// 🚦️ Bound the edge: rates per client address, sockets, requests in flight and the request
    /// body. An instance that sets none bounds nothing ([`Limits::open`]) — the shape of one that
    /// sits behind an edge of its own; a public instance passes its sizing, [`Limits::default`]
    /// being the one for a class of three hundred behind one address.
    pub fn limits(mut self, limits: Limits) -> Self {
        self.limits = limits;
        self
    }

    /// 🧭️ Say where a client's address comes from; the peer of the connection unless stated.
    pub fn addressing(mut self, addressing: ClientAddressing) -> Self {
        self.addressing = addressing;
        self
    }

    /// 🛣️ Mount only these of the gateway's own route groups; every group unless stated.
    pub fn routes(mut self, groups: RouteGroups) -> Self {
        self.routes = groups;
        self
    }

    /// 🪟️ Choose what `GET /instance` hands out; the whole definition unless stated.
    pub fn disclosure(mut self, disclosure: InstanceDisclosure) -> Self {
        self.disclosure = disclosure;
        self
    }

    /// 🧮️ Keep at most `capacity` actors placed at once ([`DIRECTORY_CAPACITY`] unless stated).
    pub fn activations(mut self, capacity: usize) -> Self {
        self.activations = capacity;
        self
    }

    /// 📸️ Snapshot an actor every `interval` committed events ([`SNAPSHOT_INTERVAL`] unless stated;
    /// `0` writes none).
    pub fn snapshots(mut self, interval: u64) -> Self {
        self.snapshot_interval = interval;
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
        let modules = Arc::new(self.modules);
        for module in modules.iter() {
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
        let admission: AdmissionHook = {
            let modules = Arc::clone(&modules);
            Box::new(move |envelope: &CommandEnvelope| modules.iter().try_for_each(|module| module.command_admission(envelope)))
        };

        let stores = I::open(&self.profile).await?;
        let mut bus = CommandBus::new(AuthorityDirectory::bounded(self.activations), stores.authority, hook).admitting(admission).snapshotting(self.snapshot_interval);
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

        let state = ServerState {
            store: Arc::clone(bus.store()),
            authority: Arc::new(Mutex::new(bus)),
            sagas: Arc::new(Mutex::new(runner)),
            projections: Arc::new(tokio::sync::RwLock::new(stores.projections)),
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
            presence_rooms: Arc::new(PresenceRooms::new(&self.presence, Instant::now())),
            presence_settings: Arc::new(self.presence),
            origin_admission: self.origin_admission,
            throttle: Throttle::new(self.limits.clone(), std::time::Instant::now()),
            addressing: self.addressing,
            lane: Arc::new(CommandLane::default()),
            clock: Arc::new(StdMutex::new(HybridLogicalClock::default())),
        };

        let disclosed = match self.disclosure {
            InstanceDisclosure::Whole => definition.clone(),
            InstanceDisclosure::Public => definition.public(),
        };
        let mut router = base_router(disclosed, self.routes, self.documents.is_some());
        for module in modules.iter() {
            router = module.routes(router).await;
        }
        if let Some(bytes) = self.limits.body_bytes {
            router = router.layer(DefaultBodyLimit::max(bytes));
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
/// backends behind it is chosen downstream. The core is always mounted; each further group only for
/// an instance that asked for it ([`RouteGroups`]), and the document socket only for an instance
/// that registered a [`DocumentAuthority`], leaving the path to an instance that owns it itself.
fn base_router<I: ServerInstance>(definition: ServerInstanceDefinition, groups: RouteGroups, hosts_documents: bool) -> Router<ServerState<I>> {
    let mut router = Router::new()
        .route("/instance", get(move || instance_body(definition.clone())))
        .route("/commands", post(post_command::<I>))
        .route("/queries", post(post_query::<I>))
        .route("/scopes/{scope}/presence/ws", get(get_presence_ws::<I>))
        .route("/actors/{tenant}/{kind}/{id}/events", get(get_events::<I>))
        .route("/actors/{tenant}/{kind}/{id}/events/ws", get(get_event_stream_ws::<I>));
    if groups.ephemeral {
        router = router.route("/scopes/{scope}/ephemeral", post(post_ephemeral::<I>));
    }
    if groups.blobs {
        router = router.route("/blobs/{hash}", get(get_blob::<I>).head(head_blob::<I>).put(put_blob::<I>));
    }
    if groups.apps {
        router = router
            .route("/apps", get(get_apps::<I>))
            .route("/apps/{app}/installs", get(get_app_installs::<I>))
            .route("/apps/{app}", get(get_app_root::<I>))
            .route("/apps/{app}/{*rest}", get(get_app_asset::<I>));
    }
    if hosts_documents {
        router = router.route("/scopes/{scope}/document/ws", get(get_document_ws::<I>));
    }
    router
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
            limits: Limits::open(),
            addressing: ClientAddressing::default(),
            routes: RouteGroups::ALL,
            disclosure: InstanceDisclosure::default(),
            activations: DIRECTORY_CAPACITY,
            snapshot_interval: SNAPSHOT_INTERVAL,
            id: "server".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// 🛣️ The wired router, ready to be mounted or wrapped: the routes, the body limit and the
    /// cross-origin grant. The throttle is not on it — whoever composes the final router puts
    /// [`throttled`](Self::throttled) around everything that does work per request.
    pub fn router(&self) -> Router {
        self.router.clone()
    }

    /// 🚦️ `router` behind this instance's throttle ([`throttle_middleware`]): the layer a composer
    /// applies outside its own per-request layers and inside only what must answer even a refused
    /// request (a transport gate, the cross-origin grant of an edge that owns it).
    pub fn throttled(&self, router: Router) -> Router {
        router.layer(axum::middleware::from_fn_with_state(self.state.edge(), throttle_middleware))
    }

    /// 🧠️ The shared state every handler runs against.
    pub fn state(&self) -> &ServerState<I> {
        &self.state
    }

    /// 🏛️ What this instance declares itself to be.
    pub fn definition(&self) -> &ServerInstanceDefinition {
        &self.definition
    }

    /// ▶️ Bind `addr` and serve the throttled router until the process is stopped. Connection info
    /// is carried into every handler so the loopback fact behind [`AdminGate`] and the peer address
    /// behind the throttle stay transport facts, and every connection sends without delay
    /// ([`undelayed`]).
    pub async fn run(self, addr: SocketAddr) -> Result<(), ServerError> {
        let listener = tokio::net::TcpListener::bind(addr).await.map_err(|error| ServerError::Internal(error.to_string()))?;
        let router = self.throttled(self.router.clone());
        axum::serve(listener.tap_io(undelayed), router.into_make_service_with_connect_info::<SocketAddr>()).await.map_err(|error| ServerError::Internal(error.to_string()))
    }
}
//#endregion 🔖️Server

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
