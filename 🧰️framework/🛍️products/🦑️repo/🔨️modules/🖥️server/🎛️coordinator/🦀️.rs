//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The repo coordinator in Rust: the owned append-only event store and its crash-recovery protocol,
// the event-sourced repository with replayed projections, and a hand-rolled HTTP/1.1 surface.
// Twin of `📦️packages/🐹️go/🐹️.go`; both are judged by the cases under `🧪️tests`.

//#endregion 🧲️Header

use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub use semio_framework_repo_events::{digest, Sha256};
pub use serde_json;

#[path = "⏱️config/🦀️.rs"]
mod config;
pub use config::*;

#[path = "🔢️event-encoding/🦀️.rs"]
mod event_encoding;
pub use event_encoding::*;

#[path = "🛡️durability/🦀️.rs"]
mod durability;
pub use durability::*;

#[path = "🗄️event-store/🦀️.rs"]
mod event_store;
pub use event_store::*;

#[path = "🔐️store-append/🦀️.rs"]
mod store_append;

#[path = "♻️store-recovery/🦀️.rs"]
mod store_recovery;
use store_recovery::*;

#[path = "🧾️projection-records/🦀️.rs"]
mod projection_records;
pub use projection_records::*;

#[path = "🧠️projection/🦀️.rs"]
mod projection;
pub use projection::*;

#[path = "📚️repository/🦀️.rs"]
mod repository;
pub use repository::*;

#[path = "✍️repository-records/🦀️.rs"]
mod repository_records;

#[path = "🔭️repository-projections/🦀️.rs"]
mod repository_projections;
use repository_projections::*;

#[path = "📨️http-messages/🦀️.rs"]
mod http_messages;
pub use http_messages::*;

#[path = "🌐️http-server-core/🦀️.rs"]
mod http_server_core;
pub use http_server_core::*;

#[path = "🎫️ticket-routes/🦀️.rs"]
mod ticket_routes;

#[path = "📥️diff-ingest/🦀️.rs"]
mod diff_ingest;
pub use diff_ingest::*;

#[path = "📡️event-routes/🦀️.rs"]
mod event_routes;

#[path = "🧵️service-loop/🦀️.rs"]
mod service_loop;
pub use service_loop::*;

#[path = "🪝️webhooks/🦀️.rs"]
mod webhooks;
pub use webhooks::*;

#[path = "🚀️clock-ids-and-main/🦀️.rs"]
mod clock_ids_and_main;
pub use clock_ids_and_main::*;

//#region 🧪️Tests

#[cfg(test)]
#[path = "./🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
