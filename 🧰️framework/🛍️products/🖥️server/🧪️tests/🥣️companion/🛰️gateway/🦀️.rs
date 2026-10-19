//! 🛰️ The gateway's unit laws, run from the companion against the gateway's public vocabulary.
use axum::Router;
use axum::body::Bytes;
use axum::extract::{ConnectInfo, Path, Request, State};
use axum::extract::ws::Message;
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, header};
use axum::routing::get;
use axum::response::IntoResponse;
use futures::StreamExt;
use crate::authority::Committed;
use crate::contract::{ActorKey, CommandEnvelope, CommandOutcome, CommandReceipt, EventRecord, HybridLogicalClock, OpaqueJson, PolicyPoint, PolicyTemplate, PresenceEntry, PresenceFrame, Principal, QueryEnvelope, Rejection, Revision, Scope, ServerInstanceDefinition, TenantId};
use crate::policy::{PolicyRequest, Resolved};
use crate::storage::{AuthorityStore, BlobStore, ProjectionStore, SessionStore, StorageError, StorageProfile, content_hash};
use crate::throttle::{ClientKey, Limits, Refusal, RequestClass};
use std::collections::BTreeSet;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::oneshot;
use tokio::time::Instant;

pub use server::gateway::*;

#[path = "../../../🔨️modules/📡️gateway/🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
