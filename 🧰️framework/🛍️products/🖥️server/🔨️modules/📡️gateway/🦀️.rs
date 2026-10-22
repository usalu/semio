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
use semio_framework_async_slots::ShardedMap;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, oneshot, watch, Mutex, Notify};
use tokio::time::{Instant, MissedTickBehavior};

use crate::authority::{report, AdmissionHook, AuthorityDirectory, AuthorityError, CommandBus, Committed, Decider, PolicyHook, Saga, SagaRunner, SharedStore, DIRECTORY_CAPACITY, SNAPSHOT_INTERVAL, UNAVAILABLE};
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

#[path = "❌️error/🦀️.rs"]
mod error;
pub use error::*;

#[path = "🧩️instance/🦀️.rs"]
mod instance;
pub use instance::*;

#[macro_use]
#[path = "📄️document-port/🦀️.rs"]
mod document_port;
pub use document_port::*;

#[path = "📣️fanout/🦀️.rs"]
mod fanout;
pub use fanout::*;

pub use semio_framework_server_presence::*;

#[path = "🛂️edge/🦀️.rs"]
mod edge;
pub use edge::*;

#[path = "🧠️state/🦀️.rs"]
mod state;
pub use state::*;

#[path = "🗂️static-apps/🦀️.rs"]
mod static_apps;
pub use static_apps::*;

#[path = "🔁️streams/🦀️.rs"]
mod streams;
pub use streams::*;

#[path = "🛰️presence-stream/🦀️.rs"]
mod presence_stream;
pub use presence_stream::*;

#[path = "🖥️server/🦀️.rs"]
mod server;
pub use server::*;

#[cfg(test)]
#[path = "🧪️tests/🔒️private/🦀️.rs"]
mod private_tests;
