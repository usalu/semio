//! 🗄️ Db facade — Shape V2 #[path] glue.

// 🎛️ Ruling R7 (2026-08-19): `async fn` in a public trait lints "auto trait bounds cannot be
// specified" on every crate this ticket converts, because our own ruling R3 forbids the lint's own
// suggested fix (`-> impl Future<..> + Send`) — Send on a spawned future is derived STRUCTURALLY
// from the concrete `DbBackend`/`WalRef`/... enum at the call site, never from a bound on the
// trait method. Silencing the lint here is deliberate, not an oversight.
#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as pack;
pub use semio_framework_os_kernel::os_pack::testkit as pack_testkit;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate as db_core;
pub use crate as db;
pub use pack::ContentHash;

// 🌉️ Re-exported so a `db_storage::DbFuture`-driving caller outside this crate (`db_cli`'s own
// module glob covers `db_cli` itself; `🌎️hub`'s bin entry needs it directly) can name
// `HostAsyncRuntime`/`ScopeHandle`/etc. to build the runtime `FsStorage::open`/
// `db_storage_sqlite::SqliteStorage::open` require, without that caller's own `Cargo.toml` naming
// `semio-framework-async` a second time — this crate already depends on it (see `db_storage`'s
// "Async-first" module doc).
pub use semio_framework_async;

#[path = "../../🦀️.rs"]
#[cfg(not(target_arch = "wasm32"))]
mod db_facade;
#[cfg(not(target_arch = "wasm32"))]
pub use db_facade::*;

#[path = "../../🔮️preview/🦀️.rs"]
pub mod db_preview;

#[path = "../../🔢️index/🦀️.rs"]
pub mod db_index;

#[path = "../../🗄️storage/🦀️.rs"]
pub mod db_storage;

#[cfg(feature = "postgres")]
#[path = "../../🗄️storage/🐘️postgres/🦀️.rs"]
pub mod db_storage_postgres;

#[cfg(feature = "sqlite")]
#[path = "../../🗄️storage/🪶️sqlite/🦀️.rs"]
pub mod db_storage_sqlite;

#[cfg(feature = "neo4j")]
#[path = "../../🗄️storage/🌐️neo4j/🦀️.rs"]
pub mod db_storage_neo4j;

#[path = "../../🆔️ids/🦀️.rs"]
pub mod db_ids;

#[path = "../../💾️durability/🦀️.rs"]
pub mod db_durability;

#[path = "../../🎚️policy/🦀️.rs"]
pub mod db_policy;

#[path = "../../🕸️version-graph/🦀️.rs"]
pub mod db_version_graph;

pub use db_durability::Frontier;
pub use db_durability::*;
pub use db_ids::{check_len, ActorId, ArtifactId, DbError, DbLimits, GenerationId};
pub use db_policy::*;
pub use db_policy::{DbCapabilities, DbConfig, Priority, Profile};
pub use db_version_graph::*;

#[path = "../../🎭️actor/🦀️.rs"]
pub mod db_actor;

#[path = "../../🔒️security/🦀️.rs"]
pub mod db_security;

#[path = "../../👁️observe/🦀️.rs"]
pub mod db_observe;

#[path = "../../🧪️testkit/🦀️.rs"]
pub mod db_testkit;

#[path = "../../📝️wal/🦀️.rs"]
pub mod db_wal;

#[path = "../../📸️snapshot/🦀️.rs"]
pub mod db_snapshot;

#[path = "../../🔘️state/🦀️.rs"]
pub mod db_state;

#[path = "../../📽️projection/🦀️.rs"]
pub mod db_projection;

#[path = "../../🗜️compact/🦀️.rs"]
pub mod db_compact;

#[path = "../../⚔️conflict/🦀️.rs"]
pub mod db_conflict;

#[path = "../../🌐️cluster/🦀️.rs"]
pub mod db_cluster;

#[path = "../../🔄️sync/🦀️.rs"]
pub mod db_sync;

#[path = "../../🗿️artifact/🦀️.rs"]
pub mod db_artifact;

#[path = "../../🔍️query/🦀️.rs"]
pub mod db_query;

#[path = "../../⌨️cli/🦀️.rs"]
#[cfg(not(target_arch = "wasm32"))]
pub mod db_cli;

#[path = "../../⚙️engine/🦀️.rs"]
#[cfg(not(target_arch = "wasm32"))]
pub mod db_engine;
