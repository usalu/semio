//! 🗄️ Db facade — Shape V2 #[path] glue.

extern crate semio_framework_os_kernel as pack;
pub use semio_framework_os_kernel::os_pack::testkit as pack_testkit;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as vcs;

pub use crate as db_core;
pub use crate as db;

#[path = "../../🦀️component.rs"]
mod db_facade;
pub use db_facade::*;

#[path = "../../👁️preview/🦀️component.rs"]
pub mod db_preview;

#[path = "../../🔢️index/🦀️component.rs"]
pub mod db_index;

#[path = "../../🗄️storage/🦀️component.rs"]
pub mod db_storage;

#[cfg(feature = "postgres")]
#[path = "../../🗄️storage/🐘️postgres/🦀️component.rs"]
pub mod db_storage_postgres;

#[cfg(feature = "sqlite")]
#[path = "../../🗄️storage/🪶️sqlite/🦀️component.rs"]
pub mod db_storage_sqlite;

#[cfg(feature = "neo4j")]
#[path = "../../🗄️storage/🌐️neo4j/🦀️component.rs"]
pub mod db_storage_neo4j;

#[path = "../../🆔️ids/🦀️component.rs"]
pub mod db_ids;

#[path = "../../💾️durability/🦀️component.rs"]
pub mod db_durability;

#[path = "../../🎚️policy/🦀️component.rs"]
pub mod db_policy;

#[path = "../../🕸️version-graph/🦀️component.rs"]
pub mod db_version_graph;

pub use db_durability::Frontier;
pub use db_ids::{check_len, ActorId, ArtifactId, DbError, DbLimits, GenerationId};
pub use db_policy::{DbCapabilities, DbConfig, Priority, Profile};
pub use db_durability::*;
pub use db_policy::*;
pub use db_version_graph::*;

#[path = "../../🎭️actor/🦀️component.rs"]
pub mod db_actor;

#[path = "../../🔒️security/🦀️component.rs"]
pub mod db_security;

#[path = "../../👁️observe/🦀️component.rs"]
pub mod db_observe;

#[path = "../../🧪️testkit/🦀️component.rs"]
pub mod db_testkit;

#[path = "../../📝️wal/🦀️component.rs"]
pub mod db_wal;

#[path = "../../📸️snapshot/🦀️component.rs"]
pub mod db_snapshot;

#[path = "../../🔘️state/🦀️component.rs"]
pub mod db_state;

#[path = "../../📽️projection/🦀️component.rs"]
pub mod db_projection;

#[path = "../../🗜️compact/🦀️component.rs"]
pub mod db_compact;

#[path = "../../⚔️conflict/🦀️component.rs"]
pub mod db_conflict;

#[path = "../../🌐️cluster/🦀️component.rs"]
pub mod db_cluster;

#[path = "../../🔄️sync/🦀️component.rs"]
pub mod db_sync;

#[path = "../../📄️artifact/🦀️component.rs"]
pub mod db_artifact;

#[path = "../../🔍️query/🦀️component.rs"]
pub mod db_query;

#[path = "../../⌨️cli/🦀️component.rs"]
pub mod db_cli;

#[path = "../../⚙️engine/🦀️component.rs"]
pub mod db_engine;

