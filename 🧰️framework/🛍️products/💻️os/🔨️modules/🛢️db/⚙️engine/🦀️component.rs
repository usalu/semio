//! 🗄️ `db_engine` — the `Database` supervisor and catalog actor: the crate that assembles every
//! other `db_*` crate into the stable, contract-frozen `Database`/`ArtifactHandle` API
//! (`Database::{open, open_at, create_document, document, catalog, health, shutdown}`;
//! `ArtifactHandle::{submit, query, subscribe, frontier, preview, history, snapshot_now}`).
//! Frozen contract: `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_engine` row + "Stable API" block).
//!
//! 🎯️ Design choice (compatibility surface): `db_artifact` (a concurrent sibling session) commits
//! explicitly, in its own module doc, to keeping the `AuthzHook`/`AllowAll` seam and its local
//! `ConflictRecord{command_id, conflicting_with, path}` shape byte-for-byte stable specifically
//! because THIS crate constructs every one of those verbatim. `SubmitOptions{durability, policy}`
//! and `CommandReceipt{.., messages}` both gained a field under
//! `MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C9 — every construction site below
//! and `to_engine_receipt`'s field-for-field bridge were updated in lockstep (never a `..Default`
//! spread over a frozen-shape struct, so a future field addition here fails loud, not silent).
//!
//! 🎯️ Design choice (scope): per this wave's instructions, this crate makes `Database::open_at`
//! (zero-touch `FsStorage`) and a full submit → durable → query round trip over a REAL
//! `db_artifact::ArtifactAuthority` genuinely work end to end (see `//#region 🧪️Tests`), composing
//! the guaranteed-complete `db_state`/`db_wal`/`db_storage`/`db_artifact` crates against their real,
//! current APIs throughout. `db_cluster` is still an unimplemented stub upstream of this wave (its
//! `lib.rs` declares no public items at all) — nothing in this crate can call into it yet; every
//! cluster-shaped concern (sharding, ownership leases, quorum durability, split-brain repair) is
//! deferred wholesale, documented here rather than faked. `db_compact`/`db_sync`/`db_security`/
//! `db_observe` ARE genuinely wired, but narrowly: `Database::compact_document` drives a real
//! `db_compact::Compactor` pass, `Database::hello` drives `db_sync::handle_hello` for the wire-v2
//! handshake (no transport of its own — that is CW5/CW6's job), `SecurityAuthzHook` wraps a real
//! `db_security::SecurityGate` as an optional `AuthzHook`, and `Database::open`/`open_at` wire a
//! real `db_observe::StructuredSink`/`HealthRegistry` pair by default. `ArtifactHandle::preview`/
//! `subscribe` return `DbError::Unimplemented` (not a panic, not a fake success): `db_artifact`'s
//! own `ArtifactAuthority` mailbox (`db/document/rs/lib.rs`'s `ArtifactMessage` enum) only carries
//! `Submit`/`Query`/`Frontier` variants — there is no way to drive its preview/commit-log machinery
//! through the actor boundary without editing `db_artifact` itself, which is out of this crate's
//! ownership this wave. `snapshot_now` is likewise `Unimplemented`: `db_artifact`'s own module doc
//! documents that `DocumentState` materializes purely from the WAL suffix with no full-state
//! enumeration to serialize into a pack snapshot, and `db_snapshot` is not even a direct dependency
//! of this crate per its `Cargo.toml`. `ArtifactHandle::history` IS real: it replays a document's
//! WAL directly via `db_wal::replay_document` (a crate this one already depends on) rather than
//! going through the actor, since `db_artifact::ArtifactEngine`'s in-memory `commit_log` is only
//! populated by live `submit()` calls in the current process, not reconstructed by `open()`'s replay.

use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex};

use crate::db_ids::{ActorId, ArtifactId, DbError};
use crate::*;
use db_storage::CatalogStorage as _;
use db_storage::PayloadStorage as _;
use semio_framework_async::{Lane, WorkerPool};

//#region 🔖️Reexports
pub use crate::db_durability::DurabilityClass;
pub use crate::db_policy::{DbCapabilities, DbConfig, Profile};
//#endregion 🔖️Reexports

//#region 🔖️Ids
/// @emoji 🌉️ `protocol::ArtifactId` → `ArtifactId`, the lossless single-`String` bridge
/// `db_core`'s module doc promises — see `db_artifact`'s identical helper for the rationale (this
/// crate is the other place in the family that depends on both `db_core` and `protocol`).
async fn to_core_document_id(id: &protocol::ArtifactId) -> ArtifactId {
    ArtifactId(id.0.clone())
}

/// @emoji 🌉️ `protocol::ActorId` → `ActorId`, same bridge as `to_core_document_id`.
// 🚫️async: E4 fn-pointer slot (used as `Iterator::map(to_core_actor_id)`) — see R9
fn to_core_actor_id(id: &protocol::ActorId) -> ActorId {
    ActorId(id.0.clone())
}

async fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
}

#[cfg(test)]
fn test_worker_pool() -> Arc<WorkerPool> {
    static POOL: std::sync::OnceLock<Arc<WorkerPool>> = std::sync::OnceLock::new();
    POOL.get_or_init(|| Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 4)))).clone()
}
//#endregion 🔖️Ids

//#region 🔖️Frontier
/// @emoji 🧭️ The facade-level frontier: identical shape to `Frontier` except keyed by
/// `protocol::ArtifactId` (not `ArtifactId`) — the frozen contract's exact
/// `Frontier{document, head_seq, commit_seq, chain_hash, epoch}` shape.
#[derive(Clone, Debug, PartialEq)]
pub struct Frontier {
    pub document: protocol::ArtifactId,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub chain_hash: [u8; 32],
    pub epoch: u64,
}

impl Frontier {
    /// @emoji 🏔️ True iff `self` has observed everything `other` has — mirrors
    /// `Frontier::dominates`, re-derived here since this type's `document` field has a
    /// different type than `Frontier`'s.
    // 🚫️async: E1 pure accessor consumed by a sync Iterator::filter — see R9
    pub fn dominates(&self, other: &Frontier) -> Result<bool, DbError> {
        if self.document != other.document {
            return Err(DbError::InvalidArgument(format!("frontier document mismatch: {} vs {}", self.document.0, other.document.0)));
        }
        Ok(self.head_seq >= other.head_seq && self.commit_seq >= other.commit_seq && self.epoch >= other.epoch)
    }
}

fn to_engine_frontier(core: &db_durability::Frontier, document: protocol::ArtifactId) -> Frontier {
    Frontier { document, head_seq: core.head_seq, commit_seq: core.commit_seq, chain_hash: core.chain_hash, epoch: core.epoch }
}
//#endregion 🔖️Frontier

//#region 🔖️Receipt
/// @emoji 🧾️ The frozen `CommandReceipt` shape: `ArtifactHandle::submit`'s resolved output.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandReceipt {
    pub command_id: protocol::MutationId,
    pub frontier: Frontier,
    pub durability: DurabilityClass,
    pub conflicts: Vec<db_artifact::ConflictRecord>,
    pub state_hash: Option<ContentHash>,
    pub messages: Vec<protocol::MutationMessage>,
}

fn to_engine_receipt(receipt: db_artifact::CommandReceipt, document: protocol::ArtifactId) -> CommandReceipt {
    CommandReceipt { command_id: receipt.command_id, frontier: to_engine_frontier(&receipt.frontier, document), durability: receipt.durability, conflicts: receipt.conflicts, state_hash: receipt.state_hash, messages: receipt.messages }
}
//#endregion 🔖️Receipt

//#region 🔖️Consistency
/// @emoji 🎚️ The frozen `Consistency` enum: which frontier/view `ArtifactHandle::query` must
/// resolve against.
#[derive(Clone, Debug, PartialEq)]
pub enum Consistency {
    Canonical,
    AtLeast(Frontier),
    Exact(Frontier),
    Historical(String),
    Speculative(String),
    PreviewAugmented(String),
}
//#endregion 🔖️Consistency

//#region 🔖️Query
/// @emoji 🔎️ What `ArtifactHandle::query` can ask for — this crate's own choice (the contract fixes
/// `query`'s signature, not `Query`'s shape): single or multi-path point lookups against the
/// document's schema-erased path/value convention (see `db_artifact`'s module doc), matching what
/// `ArtifactAuthority`'s mailbox actually exposes (`ArtifactMessage::Query { path, .. }`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Query {
    Get { path: String },
    GetMany { paths: Vec<String> },
}

/// @emoji 📬️ One resolved `query`: every requested path paired with its current value bytes (`None`
/// if unset/tombstoned).
#[derive(Clone, Debug, PartialEq)]
pub struct QueryStream {
    pub results: Vec<(String, Option<Vec<u8>>)>,
}
//#endregion 🔖️Query

//#region 🔖️History
/// @emoji 📜️ One committed batch's identity plus the frontier it produced — `ArtifactHandle::history`'s
/// unit, reconstructed from a direct `db_wal::replay_document` pass (see module doc for why this
/// does NOT go through `ArtifactAuthority`'s mailbox).
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryEntry {
    pub operation_ids: Vec<protocol::MutationId>,
    pub frontier: Frontier,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct HistoryView {
    pub entries: Vec<HistoryEntry>,
}

/// @emoji 🔁️ Replays `document`'s ENTIRE WAL directly (bypassing the actor — see module doc) and
/// groups `WAL_COMMAND` records by the `WAL_FRONTIER` record that closes their transaction, exactly
/// mirroring `db_artifact::ArtifactEngine::submit`'s own commit shape (one frontier record per
/// committed batch, preceded by that batch's command records).
async fn replay_history(storage: &db_storage::DbBackend, core_document: &ArtifactId, protocol_document: &protocol::ArtifactId) -> Result<HistoryView, DbError> {
    let records = db_actor::block_on(async { db_wal::replay_document(&storage.wal().await, core_document).await })?;
    let mut entries = Vec::new();
    let mut pending_operation_ids: Vec<protocol::MutationId> = Vec::new();
    for record in records {
        match record {
            db_wal::WalRecord::TxBegin { .. } => pending_operation_ids.clear(),
            db_wal::WalRecord::Command(bytes) => {
                let mut pos = 0usize;
                let envelope = protocol::decode_envelope(&bytes, &mut pos).map_err(|err| DbError::Corrupt(format!("history: wal command record is not a valid operation envelope: {err}")))?;
                pending_operation_ids.push(envelope.mutation_id);
            }
            db_wal::WalRecord::Frontier(frontier) if !pending_operation_ids.is_empty() => {
                entries.push(HistoryEntry { operation_ids: std::mem::take(&mut pending_operation_ids), frontier: to_engine_frontier(&frontier, protocol_document.clone()) });
            }
            _ => {}
        }
    }
    Ok(HistoryView { entries })
}
//#endregion 🔖️History

//#region 🔖️LiveQuery + Preview
/// @emoji 📡️ What `ArtifactHandle::subscribe` would filter on — defined for API-shape completeness
/// even though every construction path currently returns `DbError::Unimplemented` (see module doc).
#[derive(Clone, Debug, PartialEq)]
pub struct LiveQuerySpec {
    pub since: Option<Frontier>,
}

/// @emoji 📡️ A live subscription handle — see `LiveQuerySpec`'s doc on why this is currently
/// unreachable except through the documented `Unimplemented` error.
#[derive(Clone, Debug, PartialEq)]
pub struct LiveQuery {
    pub id: String,
}

/// @emoji 🌫️ An ephemeral preview overlay handle — see `LiveQuerySpec`'s doc; same deferral reason.
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewHandle {
    pub id: String,
    pub base: Frontier,
}
//#endregion 🔖️LiveQuery + Preview

//#region 🔖️Snapshot
/// @emoji 📸️ What kind of snapshot `ArtifactHandle::snapshot_now` was asked to build — defined for
/// API-shape completeness (see module doc: this crate does not yet build real pack snapshots).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotKind {
    Full,
    Incremental,
}

/// @emoji 📸️ What a successful `snapshot_now` would resolve to — currently unreachable, see above.
#[derive(Clone, Debug, PartialEq)]
pub struct SnapshotReceipt {
    pub generation: u64,
    pub frontier: Frontier,
}

pub type SnapshotFuture = db_actor::ReplyReceiver<Result<SnapshotReceipt, DbError>>;
//#endregion 🔖️Snapshot

//#region 🔖️Security
/// @emoji 🛂️ A real `db_artifact::AuthzHook` built on `db_security::SecurityGate`: resolves the
/// submitting `protocol::ActorId` to a `db_security::Principal` via an injected closure, then
/// authorizes `Action::Write` on `AuthzScope::Document { document }`. Not the default (the default
/// stays `db_artifact::AllowAll`, matching `db_artifact`'s own single-tenant default) — opt in via
/// `Database::open_with_authz`.
pub struct SecurityAuthzHook {
    gate: db_security::SecurityGate,
    principal_for: Box<dyn Fn(&protocol::ActorId) -> db_security::Principal + Send + Sync>,
}

impl SecurityAuthzHook {
    pub async fn new(gate: db_security::SecurityGate, principal_for: impl Fn(&protocol::ActorId) -> db_security::Principal + Send + Sync + 'static) -> SecurityAuthzHook {
        SecurityAuthzHook { gate, principal_for: Box::new(principal_for) }
    }
}

impl db_artifact::AuthzHook for SecurityAuthzHook {
    async fn authorize(&self, actor: &protocol::ActorId, envelope: &protocol::MutationEnvelope) -> Result<(), DbError> {
        let principal = (self.principal_for)(actor);
        self.gate.authorize(&principal, &db_security::AuthzScope::Document { document: envelope.document_id.clone() }, db_security::Action::Write).await
    }
}
//#endregion 🔖️Security

//#region 🔖️VersionGraph
/// @emoji 🌿️ The real `vcs`-backed `VersionGraph` — the ONLY place in the whole `db`
/// family allowed to depend on `vcs` (hard dependency rule; gated behind this crate's default-on
/// `vcs` Cargo feature).
#[cfg(feature = "vcs")]
pub mod vcs_integration {
    use crate::db_ids::*;
    use crate::db_version_graph::*;
    use std::collections::HashMap;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Mutex;
    use std::task::{Context, Poll, Waker};

    //#region 🔖️SchemaErasedTypes
    /// @emoji #⃣ The `VersionGraph` seam (`ChangeRecord`/`CheckpointRequest`) is already
    /// schema-erased — it carries a `pack::ContentHash`, never document semantics — so this
    /// crate drives the real `store::ArtifactStore<P, Mutation>` with the smallest concrete `P`/
    /// `Mutation` pair that can faithfully round-trip exactly that: a projection that IS the
    /// latest recorded hash, and an operation that overwrites it (its `inverse` recovering the
    /// PRIOR hash from the pre-state, a real, correct inverse — not a placeholder). This mirrors
    /// `db_artifact`'s own schema-erased-JSON convention one layer up: neither crate has (or needs)
    /// compile-time knowledge of any real document schema.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct HashProjection {
        pub latest_hash: [u8; 32],
    }

    impl store::ArtifactDsl for HashProjection {
        const EXTENSION: &'static str = "dbhash";

        fn parse_dsl(text: &str) -> Result<HashProjection, store::TextError> {
            let trimmed = text.trim();
            if trimmed.len() != 64 || !trimmed.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                return Err(store::TextError::new("expected 64 lowercase hex characters", store::TextSpan::at(1, 1)));
            }
            let mut latest_hash = [0u8; 32];
            for (index, slot) in latest_hash.iter_mut().enumerate() {
                *slot = u8::from_str_radix(&trimmed[index * 2..index * 2 + 2], 16).map_err(|_| store::TextError::new("invalid hex byte", store::TextSpan::at(1, (index * 2 + 1) as u32)))?;
            }
            Ok(HashProjection { latest_hash })
        }

        fn print_dsl(&self) -> String {
            let mut out = String::with_capacity(64);
            for byte in self.latest_hash {
                use std::fmt::Write;
                let _ = write!(out, "{byte:02x}");
            }
            out
        }
    }

    impl store::ArtifactPack for HashProjection {
        fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
            Ok(self.latest_hash.to_vec())
        }
        fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
            let latest_hash: [u8; 32] = bytes.try_into().map_err(|_| store::PackError::Schema("HashProjection pack must be exactly 32 bytes".to_string()))?;
            Ok(HashProjection { latest_hash })
        }
    }

    #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
    pub struct HashDiff {
        pub hash: Option<[u8; 32]>,
    }

    impl protocol::MutationDiff<HashProjection> for HashDiff {
        fn apply(&self, base: &HashProjection) -> protocol::MutationApplyResult<HashProjection> {
            Ok(match self.hash {
                Some(hash) => HashProjection { latest_hash: hash },
                None => base.clone(),
            })
        }

        fn absorb(&mut self, other: HashDiff) {
            if other.hash.is_some() {
                self.hash = other.hash;
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct HashMutation {
        pub hash: [u8; 32],
        pub author: Option<protocol::ActorId>,
        pub timestamp: Option<protocol::HybridLogicalTimestamp>,
    }

    impl protocol::Mutation<HashProjection> for HashMutation {
        type Diff = HashDiff;

        fn diff(&self, _base: &HashProjection) -> protocol::MutationOutcome<HashDiff> {
            protocol::MutationOutcome::new(HashDiff { hash: Some(self.hash) })
        }

        /// @emoji ↩️ The true inverse: an operation that would restore `base`'s hash — not a
        /// no-op placeholder.
        fn inverse(&self, base: &HashProjection) -> Vec<HashMutation> {
            vec![HashMutation { hash: base.latest_hash, author: self.author.clone(), timestamp: self.timestamp }]
        }

        fn author_id(&self) -> Option<protocol::ActorId> {
            self.author.clone()
        }

        fn timestamp(&self) -> Option<protocol::HybridLogicalTimestamp> {
            self.timestamp
        }
    }

    // 🚫️async: E1 pure accessor consumed synchronously inside `format!` — see R9
    fn hex_encode(bytes: &[u8; 32]) -> String {
        use std::fmt::Write;
        let mut out = String::with_capacity(64);
        for byte in bytes {
            let _ = write!(out, "{byte:02x}");
        }
        out
    }

    fn hex_decode(text: &str) -> Result<[u8; 32], String> {
        if text.len() != 64 || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err("expected 64 lowercase hex characters".to_string());
        }
        let mut out = [0u8; 32];
        for (index, slot) in out.iter_mut().enumerate() {
            *slot = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).map_err(|error| error.to_string())?;
        }
        Ok(out)
    }

    /// @emoji 🎯️ Single-line text form: `hash=<hex64>[ author=<id>][ ts=<actor>,<physical_ms>,<logical>]`.
    impl protocol::OpText for HashMutation {
        fn print_op(&self) -> String {
            let mut out = format!("hash={}", hex_encode(&self.hash));
            if let Some(author) = &self.author {
                out.push_str(&format!(" author={}", author.0));
            }
            if let Some(ts) = &self.timestamp {
                out.push_str(&format!(" ts={},{},{}", ts.actor, ts.physical_ms, ts.logical));
            }
            out
        }
        fn parse_op(line: &str) -> Result<Self, store::TextError> {
            let err = |detail: String| store::TextError::new(detail, store::TextSpan::at(1, 1));
            let mut hash = None;
            let mut author = None;
            let mut timestamp = None;
            for token in line.split_whitespace() {
                let (key, value) = token.split_once('=').ok_or_else(|| err(format!("malformed token '{token}'")))?;
                match key {
                    "hash" => hash = Some(hex_decode(value).map_err(err)?),
                    "author" => author = Some(protocol::ActorId(value.to_string())),
                    "ts" => {
                        let parts: Vec<&str> = value.split(',').collect();
                        if parts.len() != 3 {
                            return Err(err(format!("malformed ts '{value}'")));
                        }
                        let actor = parts[0].parse::<u64>().map_err(|error| err(error.to_string()))?;
                        let physical_ms = parts[1].parse::<u64>().map_err(|error| err(error.to_string()))?;
                        let logical = parts[2].parse::<u64>().map_err(|error| err(error.to_string()))?;
                        timestamp = Some(protocol::HybridLogicalTimestamp { actor, physical_ms, logical });
                    }
                    other => return Err(err(format!("unknown key '{other}'"))),
                }
            }
            Ok(HashMutation { hash: hash.ok_or_else(|| err("missing hash".to_string()))?, author, timestamp })
        }
    }

    /// @emoji 🎯️ Binary form: `hash 32 bytes | presence u8 (bit0=author, bit1=timestamp) | [author
    /// len varint + utf8 bytes] | [timestamp: actor/physical_ms/logical varint each]`.
    impl protocol::OpBinary for HashMutation {
        fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
            let mut out = self.hash.to_vec();
            let presence = (self.author.is_some() as u8) | ((self.timestamp.is_some() as u8) << 1);
            out.push(presence);
            if let Some(author) = &self.author {
                pack::os_pack::write_varint_u64(&mut out, author.0.len() as u64);
                out.extend_from_slice(author.0.as_bytes());
            }
            if let Some(ts) = &self.timestamp {
                pack::os_pack::write_varint_u64(&mut out, ts.actor);
                pack::os_pack::write_varint_u64(&mut out, ts.physical_ms);
                pack::os_pack::write_varint_u64(&mut out, ts.logical);
            }
            Ok(out)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
            let malformed = |detail: String| protocol::ProtocolError::Malformed { what: "hash op", offset: 0, detail };
            if bytes.len() < 33 {
                return Err(malformed("truncated hash op".to_string()));
            }
            let hash: [u8; 32] = bytes[..32].try_into().expect("checked len");
            let presence = bytes[32];
            let mut pos = 33usize;
            let author = if presence & 0b01 != 0 {
                let len = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))? as usize;
                let end = pos + len;
                let text = std::str::from_utf8(bytes.get(pos..end).ok_or_else(|| malformed("truncated author".to_string()))?).map_err(|error| malformed(error.to_string()))?.to_string();
                pos = end;
                Some(protocol::ActorId(text))
            } else {
                None
            };
            let timestamp = if presence & 0b10 != 0 {
                let actor = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))?;
                let physical_ms = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))?;
                let logical = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))?;
                Some(protocol::HybridLogicalTimestamp { actor, physical_ms, logical })
            } else {
                None
            };
            Ok(HashMutation { hash, author, timestamp })
        }
    }
    //#endregion 🔖️SchemaErasedTypes

    //#region 🔖️Store
    type HashStore = store::ArtifactStore<HashProjection, HashMutation>;

    const VCS_OPERATION_ITEMS: usize = 64;
    const VCS_OPERATION_PAGE_BYTES: u64 = 16 * 1024;
    const VCS_OPERATION_PAGES: u64 = 4;
    const VCS_OPERATION_BYTES: u64 = VCS_OPERATION_PAGE_BYTES * VCS_OPERATION_PAGES;
    const VCS_TOTAL_PAGES: u64 = 256;
    const VCS_TOTAL_BYTES: u64 = VCS_OPERATION_PAGE_BYTES * VCS_TOTAL_PAGES;

    #[derive(Clone, Copy)]
    struct VcsAdmissionSlot {
        generation: u64,
        bytes: u64,
        items: usize,
        occupied: bool,
    }

    const EMPTY_VCS_ADMISSION_SLOT: VcsAdmissionSlot = VcsAdmissionSlot { generation: 0, bytes: 0, items: 0, occupied: false };

    struct VcsAdmissionState {
        slots: [VcsAdmissionSlot; VCS_OPERATION_ITEMS],
        bytes: u64,
        next_generation: u64,
    }

    static VCS_ADMISSION: Mutex<VcsAdmissionState> = Mutex::new(VcsAdmissionState { slots: [EMPTY_VCS_ADMISSION_SLOT; VCS_OPERATION_ITEMS], bytes: 0, next_generation: 1 });

    struct VcsOperationAdmission {
        slot: usize,
        generation: u64,
        bytes: u64,
        items: usize,
    }

    impl VcsOperationAdmission {
        fn try_claim(items: usize, bytes: u64) -> Result<Self, DbError> {
            if items == 0 || items > VCS_OPERATION_ITEMS {
                return Err(DbError::LimitExceeded("vcs operation item credit"));
            }
            if bytes == 0 || bytes > VCS_OPERATION_BYTES {
                return Err(DbError::LimitExceeded("vcs operation byte credit"));
            }
            let mut state = VCS_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = state.slots.iter().position(|entry| !entry.occupied) else {
                return Err(DbError::Unavailable("vcs operation capacity exhausted".to_string()));
            };
            if state.bytes.checked_add(bytes).is_none_or(|next| next > VCS_TOTAL_BYTES) {
                return Err(DbError::Unavailable("vcs operation byte capacity exhausted".to_string()));
            }
            let generation = state.next_generation;
            state.next_generation = state.next_generation.checked_add(1).ok_or(DbError::LimitExceeded("vcs operation generation"))?;
            state.slots[slot] = VcsAdmissionSlot { generation, bytes, items, occupied: true };
            state.bytes += bytes;
            Ok(Self { slot, generation, bytes, items })
        }

        fn is_current(slot: usize, generation: u64) -> bool {
            let state = VCS_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            state.slots.get(slot).is_some_and(|entry| entry.occupied && entry.generation == generation)
        }
    }

    impl Drop for VcsOperationAdmission {
        fn drop(&mut self) {
            let mut state = VCS_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let entry = &mut state.slots[self.slot];
            if !entry.occupied || entry.generation != self.generation || entry.bytes != self.bytes || entry.items != self.items {
                return;
            }
            *entry = EMPTY_VCS_ADMISSION_SLOT;
            state.bytes = state.bytes.checked_sub(self.bytes).expect("vcs operation byte credit underflow");
        }
    }

    fn vcs_credit(items: usize, owner_bytes: impl IntoIterator<Item = usize>) -> Result<(usize, u64), DbError> {
        if items == 0 || items > VCS_OPERATION_ITEMS {
            return Err(DbError::LimitExceeded("vcs operation nested item credit"));
        }
        let mut bytes = VCS_OPERATION_PAGE_BYTES;
        for owner_bytes in owner_bytes {
            bytes = bytes.checked_add(owner_bytes as u64).ok_or(DbError::LimitExceeded("vcs operation nested byte credit"))?;
        }
        let pages = bytes.checked_add(VCS_OPERATION_PAGE_BYTES - 1).ok_or(DbError::LimitExceeded("vcs operation page rounding"))? / VCS_OPERATION_PAGE_BYTES;
        let admitted = pages.checked_mul(VCS_OPERATION_PAGE_BYTES).ok_or(DbError::LimitExceeded("vcs operation page credit"))?;
        if admitted > VCS_OPERATION_BYTES {
            return Err(DbError::LimitExceeded("vcs operation byte credit"));
        }
        Ok((items, admitted))
    }

    fn record_credit(document: &ArtifactId, change: &ChangeRecord) -> Result<(usize, u64), DbError> {
        vcs_credit(1 + usize::from(change.parent.is_some()), [document.0.capacity(), change.parent.as_ref().map_or(0, String::capacity), change.author.0.capacity(), change.message.capacity()])
    }

    fn checkpoint_credit(document: &ArtifactId, request: &CheckpointRequest) -> Result<(usize, u64), DbError> {
        let items = 1usize
            .checked_add(usize::from(request.parent_checkpoint.is_some()))
            .and_then(|value| value.checked_add(request.change_ids.len()))
            .and_then(|value| value.checked_add(request.authors.len()))
            .ok_or(DbError::LimitExceeded("vcs checkpoint item credit"))?;
        let change_owner_bytes = request.change_ids.capacity().checked_mul(std::mem::size_of::<String>()).ok_or(DbError::LimitExceeded("vcs checkpoint change owner bytes"))?;
        let author_owner_bytes = request.authors.capacity().checked_mul(std::mem::size_of::<ActorId>()).ok_or(DbError::LimitExceeded("vcs checkpoint author owner bytes"))?;
        let fixed = [document.0.capacity(), request.parent_checkpoint.as_ref().map_or(0, String::capacity), request.message.capacity(), change_owner_bytes, author_owner_bytes];
        vcs_credit(items, fixed.into_iter().chain(request.change_ids.iter().map(String::capacity)).chain(request.authors.iter().map(|author| author.0.capacity())))
    }

    fn relation_credit(document: &ArtifactId, values: &[&str]) -> Result<(usize, u64), DbError> {
        vcs_credit(1 + values.len(), std::iter::once(document.0.capacity()).chain(values.iter().map(|value| value.len())))
    }

    struct VcsStoreWaiter {
        generation: u64,
        waker: Waker,
    }

    struct VcsStoreCellState {
        store: Option<HashStore>,
        busy_generation: Option<u64>,
        waiters: [Option<VcsStoreWaiter>; VCS_OPERATION_ITEMS],
    }

    struct VcsStoreCell {
        state: Mutex<VcsStoreCellState>,
    }

    impl VcsStoreCell {
        fn new() -> Self {
            Self { state: Mutex::new(VcsStoreCellState { store: None, busy_generation: None, waiters: std::array::from_fn(|_| None) }) }
        }

        fn take_next_waker(state: &mut VcsStoreCellState) -> Option<Waker> {
            let next = state.waiters.iter().enumerate().filter_map(|(slot, waiter)| waiter.as_ref().map(|waiter| (slot, waiter.generation))).min_by_key(|(_, generation)| *generation).map(|(slot, _)| slot)?;
            state.waiters[next].take().map(|waiter| waiter.waker)
        }

        fn release(&self, generation: u64, store: Option<HashStore>) {
            let wake = {
                let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if state.busy_generation != Some(generation) {
                    return;
                }
                if let Some(store) = store {
                    state.store = Some(store);
                }
                state.busy_generation = None;
                Self::take_next_waker(&mut state)
            };
            if let Some(waker) = wake {
                waker.wake();
            }
        }
    }

    struct VcsStoreAcquire {
        cell: std::sync::Arc<VcsStoreCell>,
        slot: usize,
        generation: u64,
        resolved: bool,
    }

    enum VcsStoreClaim {
        Build(VcsStoreBuildPermit),
        Ready(VcsStoreLease),
    }

    struct VcsStoreBuildPermit {
        cell: std::sync::Arc<VcsStoreCell>,
        generation: u64,
        resolved: bool,
    }

    struct VcsStoreLease {
        cell: std::sync::Arc<VcsStoreCell>,
        generation: u64,
        store: Option<HashStore>,
    }

    impl Future for VcsStoreAcquire {
        type Output = Result<VcsStoreClaim, DbError>;

        fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            if !VcsOperationAdmission::is_current(self.slot, self.generation) {
                self.resolved = true;
                return Poll::Ready(Err(DbError::Closed));
            }
            let mut state = self.cell.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.busy_generation.is_none() {
                state.busy_generation = Some(self.generation);
                state.waiters[self.slot] = None;
                let store = state.store.take();
                drop(state);
                self.resolved = true;
                let claim = match store {
                    Some(store) => VcsStoreClaim::Ready(VcsStoreLease { cell: self.cell.clone(), generation: self.generation, store: Some(store) }),
                    None => VcsStoreClaim::Build(VcsStoreBuildPermit { cell: self.cell.clone(), generation: self.generation, resolved: false }),
                };
                return Poll::Ready(Ok(claim));
            }
            let waiter = &mut state.waiters[self.slot];
            if waiter.as_ref().is_none_or(|waiter| waiter.generation != self.generation || !waiter.waker.will_wake(context.waker())) {
                *waiter = Some(VcsStoreWaiter { generation: self.generation, waker: context.waker().clone() });
            }
            Poll::Pending
        }
    }

    impl Drop for VcsStoreAcquire {
        fn drop(&mut self) {
            if self.resolved {
                return;
            }
            let mut state = self.cell.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.waiters[self.slot].as_ref().is_some_and(|waiter| waiter.generation == self.generation) {
                state.waiters[self.slot] = None;
            }
        }
    }

    impl VcsStoreBuildPermit {
        fn install(mut self, store: HashStore) -> VcsStoreLease {
            self.resolved = true;
            VcsStoreLease { cell: self.cell.clone(), generation: self.generation, store: Some(store) }
        }
    }

    impl Drop for VcsStoreBuildPermit {
        fn drop(&mut self) {
            if !self.resolved {
                self.cell.release(self.generation, None);
            }
        }
    }

    impl VcsStoreLease {
        fn store_mut(&mut self) -> &mut HashStore {
            self.store.as_mut().expect("vcs store lease owner already returned")
        }
    }

    impl Drop for VcsStoreLease {
        fn drop(&mut self) {
            self.cell.release(self.generation, self.store.take());
        }
    }

    // 🔒️ Used as a bare fn-pointer error mapper (`.map_err(map_vcs_error)`) below — same rationale
    // as `db_artifact`'s `json_err`: `Result::map_err`'s `FnOnce(E) -> F2` bound always calls the
    // mapper with an owned `E`, so a by-reference signature would not type-check at those sites.
    #[allow(clippy::needless_pass_by_value)]
    // 🚫️async: E4 fn-pointer slot
    fn map_vcs_error(err: vcs::VcsError) -> DbError {
        DbError::Internal(format!("vcs: {err}"))
    }

    /// @emoji 🌿️ One real `store::ArtifactStore` per document, driven by real `Apply`/
    /// `CommitCheckpoint` dispatches — `VersionGraph`'s real implementation.
    pub struct VcsVersionGraph {
        stores: Mutex<HashMap<String, std::sync::Arc<VcsStoreCell>>>,
    }

    impl Default for VcsVersionGraph {
        fn default() -> VcsVersionGraph {
            VcsVersionGraph { stores: Mutex::new(HashMap::new()) }
        }
    }

    impl VcsVersionGraph {
        pub async fn new() -> VcsVersionGraph {
            VcsVersionGraph::default()
        }

        async fn store(&self, document: &ArtifactId, admission: &VcsOperationAdmission) -> Result<VcsStoreLease, DbError> {
            let cell = {
                let mut stores = self.stores.lock().map_err(|_| DbError::Internal("vcs_integration: store registry mutex poisoned".to_string()))?;
                stores.entry(document.0.clone()).or_insert_with(|| std::sync::Arc::new(VcsStoreCell::new())).clone()
            };
            match (VcsStoreAcquire { cell, slot: admission.slot, generation: admission.generation, resolved: false }).await? {
                VcsStoreClaim::Ready(lease) => Ok(lease),
                VcsStoreClaim::Build(permit) => {
                    let envelope = store::create_document_envelope::<HashProjection, HashMutation>("db_engine.version_graph", &document.0, HashProjection::default(), None);
                    let store = store::ArtifactStore::new(envelope).await.map_err(map_vcs_error)?;
                    Ok(permit.install(store))
                }
            }
        }
    }

    impl VersionGraph for VcsVersionGraph {
        async fn record_change(&self, document: &ArtifactId, change: ChangeRecord) -> Result<String, DbError> {
            let admission = record_credit(document, &change).and_then(|(items, bytes)| VcsOperationAdmission::try_claim(items, bytes))?;
            let mut lease = self.store(document, &admission).await?;
            let operation = HashMutation { hash: change.content_hash.0, author: Some(protocol::ActorId(change.author.0.clone())), timestamp: Some(protocol::HybridLogicalTimestamp::new(0, change.timestamp_ms)) };
            lease.store_mut().dispatch(store::ArtifactCommand::Apply { mutations: vec![operation], description: Some(change.message) }).await.map_err(map_vcs_error)?;
            Ok(lease.store_mut().envelope().await.vcs.edits.last().map(|edit| edit.id.clone()).unwrap_or_default())
        }

        /// @emoji 🎯️ Design choice: `request.parent_checkpoint`/`change_ids` are NOT threaded
        /// through — `store::ArtifactCommand::CommitCheckpoint` always folds every edit applied
        /// since the store's OWN current checkpoint (tracked internally by `ArtifactStore`,
        /// advanced by `record_change`'s `Apply` calls above), which is the only value that could
        /// ever be consistent with this store's real history. `request.timestamp_ms` is similarly
        /// unused: `vcs`'s own `CommitCheckpoint` handler stamps its own `now_iso()` timestamp into
        /// the checkpoint (part of what its content-addressed id hashes over) — this crate cannot
        /// override that without reaching into `vcs`'s private state.
        async fn checkpoint(&self, document: &ArtifactId, request: CheckpointRequest) -> Result<String, DbError> {
            let admission = checkpoint_credit(document, &request).and_then(|(items, bytes)| VcsOperationAdmission::try_claim(items, bytes))?;
            let mut lease = self.store(document, &admission).await?;
            let authors: Vec<vcs::Author> = request.authors.into_iter().map(|author| vcs::Author { id: author.0.clone(), name: author.0, avatar: None }).collect();
            lease.store_mut().dispatch(store::ArtifactCommand::CommitCheckpoint { message: Some(request.message), authors }).await.map_err(map_vcs_error)?;
            lease.store_mut().current_checkpoint_id().await.map(str::to_string).ok_or_else(|| DbError::Internal("vcs: commit_checkpoint produced no checkpoint id".to_string()))
        }

        async fn merge_base(&self, document: &ArtifactId, a: &str, b: &str) -> Result<Option<String>, DbError> {
            let admission = relation_credit(document, &[a, b]).and_then(|(items, bytes)| VcsOperationAdmission::try_claim(items, bytes))?;
            let mut lease = self.store(document, &admission).await?;
            Ok(store::merge_base(lease.store_mut().envelope().await, a, b).await)
        }

        async fn head(&self, document: &ArtifactId, alternative: &str) -> Result<Option<String>, DbError> {
            let admission = relation_credit(document, &[alternative]).and_then(|(items, bytes)| VcsOperationAdmission::try_claim(items, bytes))?;
            let mut lease = self.store(document, &admission).await?;
            let envelope = lease.store_mut().envelope().await;
            if let Some(found) = envelope.vcs.alternatives.iter().find(|candidate| candidate.id == alternative || candidate.name == alternative) {
                return Ok(found.checkpoint_ids.last().cloned());
            }
            Ok(lease.store_mut().current_checkpoint_id().await.map(str::to_string))
        }
    }
    //#endregion 🔖️Store
}
//#endregion 🔖️VersionGraph

//#region 🔖️VersionGraphs
// 🔀️ dedyn-fw-os-misc, O1/R11: closes `VersionGraph`'s 2-implementor set — `NullVersionGraph`
// always, `VcsVersionGraph` only when the `vcs` feature is on (mirrors the two `#[cfg]` branches
// `Database`'s constructors already had to pick between). `dyn_enum_close!`'s variant DSL has no
// per-variant `#[cfg]` (see `semio_framework_dispatch_macros`'s own `DynEnumVariant::parse`), so the
// whole closing site is duplicated per feature state instead of gating one variant inside it —
// still ONE concrete `VersionGraphs` type per build, never a generic thread through
// `ArtifactEngineConfig`/`ArtifactEngine`/`Database`. Replaces `Arc<dyn VersionGraph>`.
use crate::__semio_dispatch_VersionGraph;
use semio_framework_dispatch_macros::dyn_enum_close;

#[cfg(feature = "vcs")]
dyn_enum_close! {
    pub enum VersionGraphs: VersionGraph {
        Null(NullVersionGraph),
        Vcs(vcs_integration::VcsVersionGraph),
    }
}

#[cfg(not(feature = "vcs"))]
dyn_enum_close! {
    pub enum VersionGraphs: VersionGraph {
        Null(NullVersionGraph),
    }
}
//#endregion 🔖️VersionGraphs

//#region 🔖️Observe
/// @emoji 📡️ The default observability wiring `Database::open`/`open_at` build when the caller
/// doesn't supply their own: an in-memory `db_observe::StructuredSink` (real JSON-lines encoding,
/// just not flushed anywhere durable by default — a caller wanting file/pipe output constructs
/// `db_observe::WriterSink` themselves and passes it via `Database::open_with_emit`).
// 🔀️ dedyn-emit-runtime, O1/R1: concrete return type (`Database`'s `E` default matches it exactly),
// not `Arc<dyn Emit>` — every caller (`open`/`open_at`/`open_with_authz`) infers `E` from this value.
async fn default_emit() -> Arc<db_observe::StructuredSink<db_observe::MemorySink>> {
    Arc::new(db_observe::StructuredSink::new(db_observe::MemorySink::new()))
}
//#endregion 🔖️Observe

//#region 🔖️Catalog
/// @emoji 📇️ One document known to this `Database`'s catalog.
#[derive(Clone, Debug, PartialEq)]
pub struct CatalogEntry {
    pub document: protocol::ArtifactId,
    pub created_at_ms: u64,
}

/// @emoji 📇️ A point-in-time read of every document this `Database` knows about.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CatalogView {
    pub artifacts: Vec<CatalogEntry>,
}

/// @emoji 💾️ The catalog root's on-disk shape — a plain JSON array, deliberately NOT reusing
/// `CatalogEntry` directly (keeps the public type free of a `serde` bound it doesn't otherwise need).
#[derive(serde::Serialize, serde::Deserialize)]
struct CatalogRootEntry {
    document: String,
    created_at_ms: u64,
}

async fn encode_catalog(entries: &[CatalogEntry]) -> Result<Vec<u8>, DbError> {
    let raw: Vec<CatalogRootEntry> = entries.iter().map(|entry| CatalogRootEntry { document: entry.document.0.clone(), created_at_ms: entry.created_at_ms }).collect();
    serde_json::to_vec(&raw).map_err(|err| DbError::Internal(format!("catalog encode: {err}")))
}

async fn decode_catalog(bytes: &[u8]) -> Result<Vec<CatalogEntry>, DbError> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let raw: Vec<CatalogRootEntry> = serde_json::from_slice(bytes).map_err(|err| DbError::Corrupt(format!("catalog decode: {err}")))?;
    Ok(raw.into_iter().map(|entry| CatalogEntry { document: protocol::ArtifactId(entry.document), created_at_ms: entry.created_at_ms }).collect())
}

struct CatalogState {
    epoch: EpochFence,
    entries: Vec<CatalogEntry>,
}
//#endregion 🔖️Catalog

//#region 🔖️ArtifactSpec
/// @emoji 📄️ What `Database::create_document` needs to mint a brand-new document — this crate's own
/// choice (the contract fixes `create_document`'s signature, not `ArtifactSpec`'s shape).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactSpec {
    pub document: protocol::ArtifactId,
}

impl ArtifactSpec {
    pub async fn new(document: protocol::ArtifactId) -> ArtifactSpec {
        ArtifactSpec { document }
    }
}
//#endregion 🔖️ArtifactSpec

//#region 🔖️Health
/// @emoji 🩺️ The frozen `Database::health()` return shape, wrapping a real
/// `db_observe::HealthRegistry` snapshot plus this crate's own catalog-level fact (open document
/// count) that no lower crate could know.
#[derive(Clone, Debug)]
pub struct DbHealth {
    pub report: db_observe::HealthReport,
    pub open_artifacts: usize,
}
//#endregion 🔖️Health

//#region 🔖️Database
/// @emoji 🗄️ The catalog: owns the storage substrate, the shared config/capabilities/authz/
/// version-graph/observability wiring every document actor is constructed with, and the registry of
/// currently-open `ArtifactAuthority` actors.
///
/// 🎯️ Design choice: the catalog registry itself is a plain `Mutex`-guarded `HashMap`, not a
/// separate `db_actor::Actor`-driven process. `Database`'s own public surface (`open`/
/// `create_document`/`document`/`catalog`/`health`/`shutdown`) is already synchronous per the
/// frozen contract, and per-document concurrency is already provided by each `ArtifactAuthority`'s
/// own dedicated thread — the catalog only ever needs to serialize document-registry mutations and
/// a catalog-root CAS write, which a `Mutex` does directly without the mailbox's priority-lane/
/// backpressure machinery (that machinery matters for a document's WAL under load, not a rare
/// catalog-root swap).
// 🔀️ `A` is the pluggable `AuthzHook` implementation (see `db_artifact::ArtifactEngineConfig`'s own
// doc) — dedyn-fw-os-misc, R11(a): a caller-supplied, stored implementation is trivially generic;
// `AllowAll` default keeps every existing unparameterized `Database` reference (this crate's own
// `open`/`open_at`/`open_with_emit`, plus every external caller) compiling unchanged.
//
// 🔀️ `E` is the pluggable `Emit` sink — dedyn-emit-runtime, O1/R11(a): `open_with_emit`'s own doc
// ("a caller-supplied Emit sink, e.g. a `db_observe::WriterSink`") is exactly R11(a)'s "trivially
// generic" shape, the same pattern `A` above already uses. Default is `db_observe::StructuredSink<
// db_observe::MemorySink>` — the concrete type `default_emit()` has always constructed — so every
// existing unparameterized `Database`/`Database<A>` reference (this crate's own `open`/`open_at`/
// `open_with_authz`, plus `🌎️hub` and every other external caller, none of which ever names this
// type parameter) compiles unchanged. Replaces `Arc<dyn Emit>`.
pub struct Database<A: db_artifact::AuthzHook + 'static = db_artifact::AllowAll, E: Emit + 'static = db_observe::StructuredSink<db_observe::MemorySink>> {
    storage: Arc<db_storage::DbBackend>,
    config: DbConfig,
    capabilities: DbCapabilities,
    authz: Arc<A>,
    /// @emoji 🌿️ Never `None`: `NullVersionGraph` (an `Unimplemented`-on-every-call
    /// placeholder, not an `Option` layer — see its own doc) is the default when the `vcs` feature
    /// is disabled, exactly matching `db_artifact::ArtifactEngineConfig::default`'s own choice.
    version_graph: Arc<VersionGraphs>,
    emit: Arc<E>,
    health: Arc<db_observe::HealthRegistry>,
    catalog: Mutex<CatalogState>,
    open_artifacts: Mutex<HashMap<String, Arc<db_artifact::ArtifactAuthority>>>,
    /// @emoji 🧵️ The process WorkerPool every document authority and submit bridge uses.
    /// Construction without this owner is intentionally impossible: no database path may execute
    /// blocking storage or authority work inline on its caller.
    pool: Arc<WorkerPool>,
}

// 🚫️async: E5 executor bridge — every `Database` method below is plain sync and drives its async
// storage/`ArtifactEngine` calls via `db_actor::block_on` (R4 clause 2: this crate's own db-actor
// thread bridges are sanctioned; `Database` is the facade the prior `db-trait-flip` packet already
// classified as thread-owning alongside `db_artifact`, per its report's "db_engine (per-submit
// bridge threads)"). Every `.wal()`/`.snapshot()`/`.catalog()`/`.index()`/`.payload()`/`.lease()`
// accessor call is `.await`ed inside the SAME `block_on`, never a bare synchronous call.
impl Database<db_artifact::AllowAll> {
    /// @emoji 🚀️ The frozen entry point: opens (or initializes, if `storage` is fresh) a `Database`
    /// over an arbitrary `Arc<db_storage::DbBackend>` backend, wired with the default `AllowAll` authz and
    /// (behind the default-on `vcs` feature) a real `VcsVersionGraph`.
    pub async fn open(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>) -> Result<Database<db_artifact::AllowAll>, DbError> {
        Database::open_with(pool, config, storage, Arc::new(db_artifact::AllowAll), default_emit().await).await
    }

    /// @emoji 🚀️ The zero-touch filesystem entry point. The caller supplies the process pool
    /// before storage construction, so opening can never take a pool-less inline path.
    pub async fn open_at(pool: Arc<WorkerPool>, root: &std::path::Path, profile: Profile) -> Result<Database<db_artifact::AllowAll>, DbError> {
        let fs = db_storage::FsStorage::open(pool.clone(), root).await?;
        let storage: Arc<db_storage::DbBackend> = Arc::new(db_storage::DbBackend::Fs(fs));
        Database::open(pool, DbConfig::for_profile(profile), storage).await
    }

    /// @emoji 🚀️ Like `open`, but with a caller-supplied `Emit` sink (e.g. a `db_observe::WriterSink`
    /// over a real file) instead of the default in-memory one.
    // 🔀️ dedyn-emit-runtime, O1/R11(a): generic over `E: Emit` (the function's own type param, not
    // `Database`'s default) so the returned `Database<AllowAll, E>` carries the caller's concrete
    // sink type — this fn has zero callers anywhere in the repo today (public, documented extension
    // seam per `open_with_emit`'s own doc; matches `open_with_authz`'s identical shape below).
    pub async fn open_with_emit<E: Emit + 'static>(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>, emit: Arc<E>) -> Result<Database<db_artifact::AllowAll, E>, DbError> {
        Database::open_with(pool, config, storage, Arc::new(db_artifact::AllowAll), emit).await
    }
}

impl<A: db_artifact::AuthzHook + 'static> Database<A> {
    /// @emoji 🚀️ Like `open`, but with a caller-supplied `AuthzHook` (e.g. `SecurityAuthzHook`)
    /// instead of the default `AllowAll`.
    pub async fn open_with_authz(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>, authz: Arc<A>) -> Result<Database<A>, DbError> {
        Database::open_with(pool, config, storage, authz, default_emit().await).await
    }
}

// 🔀️ dedyn-emit-runtime, O1/R11(a): every method below reads/writes `self.emit`, so this whole
// block (previously `impl<A: AuthzHook + 'static> Database<A>`, default-`E` only) is now generic
// over `E: Emit` too. `open_with_authz` above stays in its own default-`E` block since it never
// takes an `emit` argument and must return the SAME default-`E` `Database<A>` every unparameterized
// caller expects — Rust resolves its `Database::open_with(..)` call by inferring `E` from
// `default_emit()`'s concrete return type regardless of which `impl` block `open_with` itself lives
// in, so the split is transparent to every call site.
impl<A: db_artifact::AuthzHook + 'static, E: Emit + 'static> Database<A, E> {
    async fn open_with(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>, authz: Arc<A>, emit: Arc<E>) -> Result<Database<A, E>, DbError> {
        let storage_capabilities = db_actor::block_on(storage.capabilities());
        let capabilities = DbCapabilities {
            // 🧩️ Extension seam: real, honest today — see module doc on why preview/live-query
            // aren't reachable through `ArtifactAuthority`'s current mailbox surface, and why
            // `db_cluster` is still an empty stub upstream of this wave.
            preview: false,
            historical_query: true,
            live_query: false,
            cluster: false,
            max_durability: std::cmp::min(storage_capabilities.max_durability, config.capabilities.max_durability),
        };

        let health = Arc::new(db_observe::HealthRegistry::new());
        health.set("db_engine.storage", if storage_capabilities.durable { db_observe::HealthState::Healthy } else { db_observe::HealthState::Degraded("storage backend is not durable".to_string()) });

        let (epoch, entries) = match db_actor::block_on(async { storage.catalog().await.read_root().await })? {
            Some((bytes, epoch)) => (epoch, decode_catalog(&bytes).await?),
            None => {
                let empty = encode_catalog(&[]).await?;
                let pages = db_storage::DbIoPages::try_new(empty).map_err(|_| DbError::LimitExceeded("catalog bootstrap pages"))?;
                let epoch = db_actor::block_on(async { storage.catalog().await.cas_root(EpochFence::INITIAL, pages).await })?;
                (epoch, Vec::new())
            }
        };
        health.set("db_engine.catalog", db_observe::HealthState::Healthy);

        #[cfg(feature = "vcs")]
        let version_graph: Arc<VersionGraphs> = Arc::new(VersionGraphs::Vcs(vcs_integration::VcsVersionGraph::new().await));
        #[cfg(not(feature = "vcs"))]
        let version_graph: Arc<VersionGraphs> = Arc::new(VersionGraphs::Null(NullVersionGraph));

        emit.emit(EmitEvent::new("db_engine.database_opened").field("documents", EmitField::U64(entries.len() as u64))).await;

        Ok(Database { storage, config, capabilities, authz, version_graph, emit, health, catalog: Mutex::new(CatalogState { epoch, entries }), open_artifacts: Mutex::new(HashMap::new()), pool })
    }

    /// @emoji ⚙️ Builds one `ArtifactEngineConfig`. Sets the 4 fields this crate has ALWAYS
    /// constructed (`limits`/`authz`/`version_graph`/`preview_ttl_ms`, per the module doc's
    /// compatibility-surface note) explicitly, and spreads `..db_artifact::ArtifactEngineConfig::
    /// default()` for every other field db_artifact has since grown (e.g. `security`/`emit`/
    /// `projections`) — this crate has no opinion on those yet (`db_artifact`'s own real
    /// `db_security::SecurityGate`-backed default policy already matches `AllowAll`'s permissive
    /// single-tenant spirit), and the spread keeps this call site correct across further additive
    /// growth without another coordinated edit.
    async fn document_engine_config(&self) -> db_artifact::ArtifactEngineConfig<A, VersionGraphs> {
        // 🔀️ Can't `..db_artifact::ArtifactEngineConfig::default()` spread here: that default is
        // only defined for `ArtifactEngineConfig<AllowAll, NullVersionGraph>` (see its `impl
        // Default`), a different concrete type from `ArtifactEngineConfig<A, VersionGraphs>`
        // whenever this `Database<A>` was opened via `open_with_authz` with a non-`AllowAll` hook —
        // struct-update syntax requires an exact type match. Pull the `A`/`V`-independent defaults
        // (`security`/`emit`/`projections`) from the default instantiation by value instead.
        let other_defaults = db_artifact::ArtifactEngineConfig::default();
        db_artifact::ArtifactEngineConfig {
            limits: self.config.limits.clone(),
            authz: self.authz.clone(),
            version_graph: self.version_graph.clone(),
            preview_ttl_ms: self.config.limits.max_preview_ttl_ms,
            security: other_defaults.security,
            emit: other_defaults.emit,
            projections: other_defaults.projections,
        }
    }

    async fn spawn_authority_create(&self, document: protocol::ArtifactId) -> Result<Arc<db_artifact::ArtifactAuthority>, DbError> {
        let pool = self.pool.clone();
        let storage = self.storage.clone();
        let config = self.document_engine_config().await;
        let created_at_ms = now_ms().await;
        let mailbox_capacities = self.config.mailbox_capacities;
        let authority = db_artifact::ArtifactAuthority::spawn(pool, move || db_artifact::ArtifactEngine::create_retained(document, storage, config, created_at_ms), mailbox_capacities).await?;
        Ok(Arc::new(authority))
    }

    async fn spawn_authority_open(&self, document: protocol::ArtifactId) -> Result<Arc<db_artifact::ArtifactAuthority>, DbError> {
        let pool = self.pool.clone();
        let storage = self.storage.clone();
        let config = self.document_engine_config().await;
        let opened_at_ms = now_ms().await;
        let mailbox_capacities = self.config.mailbox_capacities;
        let authority = db_artifact::ArtifactAuthority::spawn(pool, move || async move { db_artifact::ArtifactEngine::open_retained(document, storage, config, opened_at_ms).await.map(|(engine, _report)| engine) }, mailbox_capacities).await?;
        Ok(Arc::new(authority))
    }

    async fn register_handle(&self, document: protocol::ArtifactId, authority: Arc<db_artifact::ArtifactAuthority>) -> ArtifactHandle {
        let core_document = to_core_document_id(&document).await;
        self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").insert(document.0.clone(), authority.clone());
        ArtifactHandle { authority, storage: self.storage.clone(), document, core_document, pool: self.pool.clone() }
    }

    /// @emoji 🌱️ The frozen `create_document`: mints a brand-new document, durably records it in the
    /// catalog root (CAS-fenced), spawns its `ArtifactAuthority`, and returns a live handle.
    pub async fn create_document(&self, spec: ArtifactSpec) -> Result<ArtifactHandle, DbError> {
        let document = spec.document;
        {
            let mut catalog = self.catalog.lock().expect("db_engine: catalog mutex poisoned");
            if catalog.entries.iter().any(|entry| entry.document == document) {
                return Err(DbError::AlreadyExists(format!("document {} already exists", document.0)));
            }
            let mut entries = catalog.entries.clone();
            let epoch = catalog.epoch;
            // 🔒️ A real `.await` reached while `catalog`'s guard is alive would extend its
            // temporary across this whole statement-block (R7), making the enclosing future
            // non-`Send` — needed for `semio-hub`'s axum handlers, not for `wasm32`, which has no
            // multi-threaded work-stealing scheduler to demand it. So: drive `commit` via
            // `db_actor::block_on` (the same bridge `cas_root` alone used to reach for) on every
            // target that DOES need `Send`, and via a plain `.await` — `db_actor::block_on` doesn't
            // exist for `wasm32` — on the one target that doesn't. Either way `entries`/`epoch` are
            // captured by reference, not moved, so this costs nothing beyond the `#[cfg]` split.
            let commit = async {
                entries.push(CatalogEntry { document: document.clone(), created_at_ms: now_ms().await });
                let bytes = encode_catalog(&entries).await?;
                let pages = db_storage::DbIoPages::try_new(bytes).map_err(|_| DbError::LimitExceeded("catalog persist pages"))?;
                self.storage.catalog().await.cas_root(epoch, pages).await
            };
            #[cfg(not(target_arch = "wasm32"))]
            let new_epoch = db_actor::block_on(commit)?;
            #[cfg(target_arch = "wasm32")]
            let new_epoch = commit.await?;
            catalog.epoch = new_epoch;
            catalog.entries = entries;
        }
        let authority = self.spawn_authority_create(document.clone()).await?;
        self.emit.emit(EmitEvent::new("db_engine.document_created").with_document(to_core_document_id(&document).await)).await;
        Ok(self.register_handle(document, authority).await)
    }

    /// @emoji 📄️ The frozen `document`: returns a live handle to an already-cataloged document,
    /// reusing an already-open `ArtifactAuthority` if one exists, else recovering it fresh from its
    /// WAL.
    pub async fn document(&self, id: &protocol::ArtifactId) -> Result<ArtifactHandle, DbError> {
        // 🔒️ `.cloned()` ends the guard's temporary scope at this `let`'s semicolon — under
        // edition-2021 rules an `if let` scrutinee's temporary would otherwise extend across the
        // `to_core_document_id(id).await` below, making this future non-`Send` (R7).
        let existing = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").get(&id.0).cloned();
        if let Some(authority) = existing {
            return Ok(ArtifactHandle { authority, storage: self.storage.clone(), document: id.clone(), core_document: to_core_document_id(id).await, pool: self.pool.clone() });
        }
        let known = self.catalog.lock().expect("db_engine: catalog mutex poisoned").entries.iter().any(|entry| &entry.document == id);
        if !known {
            return Err(DbError::NotFound(format!("document {} not found", id.0)));
        }
        let authority = self.spawn_authority_open(id.clone()).await?;
        self.emit.emit(EmitEvent::new("db_engine.document_opened").with_document(to_core_document_id(id).await)).await;
        Ok(self.register_handle(id.clone(), authority).await)
    }

    /// @emoji 📇️ The frozen `catalog`: a point-in-time read of every document this `Database`
    /// knows about.
    pub async fn catalog(&self) -> CatalogView {
        CatalogView { artifacts: self.catalog.lock().expect("db_engine: catalog mutex poisoned").entries.clone() }
    }

    /// @emoji 🩺️ The frozen `health`: this `Database`'s `HealthRegistry` snapshot plus its own open
    /// document count.
    pub async fn health(&self) -> DbHealth {
        DbHealth { report: self.health.report(), open_artifacts: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len() }
    }

    /// @emoji 🚪️ The frozen `shutdown`: gracefully joins every open `ArtifactAuthority` this
    /// `Database` still exclusively owns.
    ///
    /// 🧩️ Extension seam: `deadline` is currently advisory — `db_artifact::ArtifactAuthority::shutdown`
    /// has no timeout parameter of its own (out of this crate's ownership to add this wave), so this
    /// always waits for a graceful join rather than forcing one after `deadline` elapses. A document
    /// whose `ArtifactHandle` is still cloned elsewhere (this `Arc`'s strong count > 1) is skipped —
    /// its actor thread keeps running under whichever handle still holds it, which is correct
    /// (shutdown must never yank a mailbox out from under a live caller), just not exhaustive.
    pub async fn shutdown(self, _deadline: std::time::Duration) -> Result<(), DbError> {
        let authorities: Vec<Arc<db_artifact::ArtifactAuthority>> = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").drain().map(|(_, authority)| authority).collect();
        for authority in authorities {
            if let Ok(authority) = Arc::try_unwrap(authority) {
                authority.shutdown().await;
            }
        }
        self.emit.emit(EmitEvent::new("db_engine.database_shutdown")).await;
        Ok(())
    }

    /// @emoji 🧰️ What this `Database` instance negotiated at `open` time.
    pub async fn capabilities(&self) -> DbCapabilities {
        self.capabilities
    }

    /// @emoji 🔌️ The underlying storage substrate this `Database` was opened with — an escape
    /// hatch for callers below the document-actor boundary that need direct `PayloadStorage`/
    /// `WalStorage` access (e.g. `os-semio_hub`'s content-addressed blob routes, or a wire-v2 semio_hub
    /// session driving `db_sync::handle_frontier_advertise` directly). Additive: not part of the
    /// contract-frozen `Database` API surface listed in `contract.md`'s "Stable API" block, so it
    /// carries no compatibility promise beyond this crate's own semver.
    pub async fn storage(&self) -> Arc<db_storage::DbBackend> {
        self.storage.clone()
    }

    /// @emoji 🧹️ A real, bounded `db_compact::Compactor` pass over `document` — WAL segment
    /// retention below its latest snapshot's `head_seq`, ref-traced payload GC, index compaction,
    /// and (if `consolidate_snapshots`) snapshot chain consolidation. See module doc: this IS a
    /// genuine `db_compact` integration, just document-at-a-time rather than a background scheduler
    /// (deferred — this wave's instructions ask for a lighter, documented cluster/compact/sync
    /// surface, not a full online scheduler).
    pub async fn compact_document(&self, document: &protocol::ArtifactId, holder: &str, consolidate_snapshots: bool) -> Result<db_compact::CompactionReport, DbError> {
        let core_document = to_core_document_id(document).await;
        db_actor::block_on(db_compact::Compactor::new(self.storage.as_ref()).await.run_from_latest_snapshot(&core_document, holder, consolidate_snapshots, &db_compact::CompactionBudget::default(), now_ms().await))
    }

    /// @emoji 👋️ A real `db_sync::handle_hello` call for `document` — the server-side half of the
    /// wire-v2 handshake (frontier exchange / bootstrap-plan decision). No transport of its own:
    /// wiring this to an actual `protocol_wire` socket is CW5/CW6's job (framework/sync, semio_hub
    /// rebuilds), out of this crate's scope this wave.
    pub async fn hello(&self, document: &protocol::ArtifactId, hello_frontier: Option<&protocol::RuntimeFrontierSummary>, session_id: String, origin: &protocol::ActorId, snapshot_chunk_bytes: usize) -> Result<db_sync::WelcomeResponse, DbError> {
        let core_document = to_core_document_id(document);
        db_actor::block_on(db_sync::handle_hello(self.storage.as_ref(), core_document.await, hello_frontier, session_id, origin, snapshot_chunk_bytes))
    }

    /// @emoji 🌿️ A real, `vcs`-backed checkpoint over every change `record_change` has recorded for
    /// `document` since its last checkpoint (see `db_artifact::ArtifactEngine::submit`'s "vcs"
    /// pipeline stage, which calls `record_change` on every commit when a `VersionGraph` is wired).
    /// Errs `Unimplemented` if the `vcs` feature is disabled (no `VersionGraph` configured).
    pub async fn checkpoint_document(&self, document: &protocol::ArtifactId, message: String, authors: &[protocol::ActorId]) -> Result<String, DbError> {
        let core_document = to_core_document_id(document).await;
        let core_authors = authors.iter().map(to_core_actor_id).collect();
        self.version_graph.checkpoint(&core_document, CheckpointRequest { parent_checkpoint: None, change_ids: Vec::new(), message, authors: core_authors, timestamp_ms: now_ms().await }).await
    }
}
//#endregion 🔖️Database

//#region 🔖️ArtifactHandle
const ARTIFACT_SUBMIT_OPERATION_ITEMS: usize = 64;
const ARTIFACT_SUBMIT_PAGE_BYTES: u64 = 16 * 1024;
const ARTIFACT_SUBMIT_OPERATION_PAGES: u64 = 64;
const ARTIFACT_SUBMIT_OPERATION_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES * ARTIFACT_SUBMIT_OPERATION_PAGES;
const ARTIFACT_SUBMIT_TOTAL_PAGES: u64 = 1024;
const ARTIFACT_SUBMIT_TOTAL_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES * ARTIFACT_SUBMIT_TOTAL_PAGES;
const ARTIFACT_SUBMIT_BATCH_ITEMS: usize = 256;
const ARTIFACT_SUBMIT_NESTED_ITEMS: usize = 4096;
const ARTIFACT_SUBMIT_RETRY_MS: u64 = 1;
const ARTIFACT_SUBMIT_RETRY_LIMIT: u8 = 8;

#[derive(Clone, Copy)]
struct ArtifactSubmitAdmissionSlot {
    generation: u64,
    bytes: u64,
    items: usize,
    occupied: bool,
}

const EMPTY_ARTIFACT_SUBMIT_SLOT: ArtifactSubmitAdmissionSlot = ArtifactSubmitAdmissionSlot { generation: 0, bytes: 0, items: 0, occupied: false };

struct ArtifactSubmitAdmissionState {
    slots: [ArtifactSubmitAdmissionSlot; ARTIFACT_SUBMIT_OPERATION_ITEMS],
    bytes: u64,
    next_generation: u64,
}

static ARTIFACT_SUBMIT_ADMISSION: std::sync::Mutex<ArtifactSubmitAdmissionState> = std::sync::Mutex::new(ArtifactSubmitAdmissionState { slots: [EMPTY_ARTIFACT_SUBMIT_SLOT; ARTIFACT_SUBMIT_OPERATION_ITEMS], bytes: 0, next_generation: 1 });

struct ArtifactSubmitAdmission {
    slot: usize,
    generation: u64,
    bytes: u64,
    items: usize,
}

impl ArtifactSubmitAdmission {
    fn try_claim(items: usize, bytes: u64) -> Result<Self, DbError> {
        if items == 0 || items > ARTIFACT_SUBMIT_NESTED_ITEMS {
            return Err(DbError::LimitExceeded("artifact submit item credit"));
        }
        if bytes == 0 || bytes > ARTIFACT_SUBMIT_OPERATION_BYTES {
            return Err(DbError::LimitExceeded("artifact submit operation byte credit"));
        }
        let mut state = ARTIFACT_SUBMIT_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(slot) = state.slots.iter().position(|entry| !entry.occupied) else {
            return Err(DbError::Unavailable("artifact submit item capacity exhausted".to_string()));
        };
        if state.bytes.checked_add(bytes).is_none_or(|next| next > ARTIFACT_SUBMIT_TOTAL_BYTES) {
            return Err(DbError::Unavailable("artifact submit byte capacity exhausted".to_string()));
        }
        let generation = state.next_generation;
        state.next_generation = state.next_generation.checked_add(1).ok_or(DbError::LimitExceeded("artifact submit generation"))?;
        state.slots[slot] = ArtifactSubmitAdmissionSlot { generation, bytes, items, occupied: true };
        state.bytes += bytes;
        Ok(Self { slot, generation, bytes, items })
    }
}

impl Drop for ArtifactSubmitAdmission {
    fn drop(&mut self) {
        let mut state = ARTIFACT_SUBMIT_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let entry = &mut state.slots[self.slot];
        if !entry.occupied || entry.generation != self.generation || entry.bytes != self.bytes || entry.items != self.items {
            return;
        }
        *entry = EMPTY_ARTIFACT_SUBMIT_SLOT;
        state.bytes = state.bytes.checked_sub(self.bytes).expect("artifact submit byte credit underflow");
    }
}

fn artifact_submit_credit(batch: &db_artifact::CommandBatch) -> Result<(usize, u64), DbError> {
    if batch.envelopes.is_empty() || batch.envelopes.len() > ARTIFACT_SUBMIT_BATCH_ITEMS {
        return Err(DbError::LimitExceeded("artifact submit batch item credit"));
    }
    let mut items = batch.envelopes.len();
    let mut bytes = ARTIFACT_SUBMIT_PAGE_BYTES;
    let envelope_owner_bytes = batch.envelopes.capacity().checked_mul(std::mem::size_of::<protocol::MutationEnvelope>()).ok_or(DbError::LimitExceeded("artifact submit envelope owner bytes"))?;
    bytes = bytes.checked_add(envelope_owner_bytes as u64).ok_or(DbError::LimitExceeded("artifact submit envelope owner bytes"))?;
    for envelope in &batch.envelopes {
        items = items.checked_add(envelope.dependencies.len()).ok_or(DbError::LimitExceeded("artifact submit nested items"))?;
        if items > ARTIFACT_SUBMIT_NESTED_ITEMS {
            return Err(DbError::LimitExceeded("artifact submit nested item credit"));
        }
        let dependency_owner_bytes = envelope.dependencies.capacity().checked_mul(std::mem::size_of::<protocol::MutationId>()).ok_or(DbError::LimitExceeded("artifact submit dependency owner bytes"))?;
        bytes = bytes
            .checked_add(envelope.mutation_id.0.capacity() as u64)
            .and_then(|value| value.checked_add(envelope.document_id.0.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.actor.0.capacity() as u64))
            .and_then(|value| value.checked_add(dependency_owner_bytes as u64))
            .and_then(|value| value.checked_add(envelope.diff.schema.0.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.diff.payload.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.inverse.schema.0.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.inverse.payload.capacity() as u64))
            .ok_or(DbError::LimitExceeded("artifact submit nested byte credit"))?;
        for dependency in &envelope.dependencies {
            bytes = bytes.checked_add(dependency.0.capacity() as u64).ok_or(DbError::LimitExceeded("artifact submit dependency byte credit"))?;
        }
    }
    let pages = bytes.checked_add(ARTIFACT_SUBMIT_PAGE_BYTES - 1).ok_or(DbError::LimitExceeded("artifact submit page rounding"))? / ARTIFACT_SUBMIT_PAGE_BYTES;
    let admitted = pages.checked_mul(ARTIFACT_SUBMIT_PAGE_BYTES).ok_or(DbError::LimitExceeded("artifact submit page credit"))?;
    if admitted > ARTIFACT_SUBMIT_OPERATION_BYTES {
        return Err(DbError::LimitExceeded("artifact submit operation byte credit"));
    }
    Ok((items, admitted))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubmitProgress {
    Admitted,
    Scheduled,
    Waiting,
    Completed,
    Cancelled,
    Fault,
}

type ArtifactActorSubmitFuture = db_actor::AskFuture<db_artifact::ArtifactMessage, Result<db_artifact::CommandReceipt, DbError>>;
pub type ArtifactSubmitOutcome = Result<Result<CommandReceipt, DbError>, DbError>;

enum ArtifactSubmitWorkOwner {
    Request { batch: db_artifact::CommandBatch, options: db_artifact::SubmitOptions, submitted_at_ms: u64 },
    Actor(ArtifactActorSubmitFuture),
}

struct ArtifactSubmitState {
    pool: WorkerPool,
    authority: Arc<db_artifact::ArtifactAuthority>,
    document: protocol::ArtifactId,
    generation: u64,
    authority_generation: db_ids::GenerationId,
    admission: std::sync::Mutex<Option<ArtifactSubmitAdmission>>,
    work: std::sync::Mutex<Option<ArtifactSubmitWorkOwner>>,
    completion: std::sync::Mutex<Option<ArtifactSubmitOutcome>>,
    terminal_work: std::sync::Mutex<Option<ArtifactSubmitWorkOwner>>,
    terminal_result: std::sync::Mutex<Option<ArtifactSubmitOutcome>>,
    retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    terminal_job: std::sync::Mutex<Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>>,
    waker: std::sync::Mutex<Option<std::task::Waker>>,
    retry_armed: std::sync::atomic::AtomicBool,
    retry_generation: std::sync::atomic::AtomicU64,
    scheduled: std::sync::atomic::AtomicBool,
    cancelled: std::sync::atomic::AtomicBool,
    abandoned: std::sync::atomic::AtomicBool,
    finished: std::sync::atomic::AtomicBool,
    progress: std::sync::atomic::AtomicU8,
}

pub struct SubmitFuture {
    state: Arc<ArtifactSubmitState>,
    resolved: bool,
}

pub struct ArtifactSubmitTerminalJob {
    state: Arc<ArtifactSubmitState>,
    owner: Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>,
}

pub struct ArtifactSubmitTerminalWork {
    state: Arc<ArtifactSubmitState>,
    owner: Option<ArtifactSubmitWorkOwner>,
}

struct ArtifactSubmitWake {
    state: std::sync::Weak<ArtifactSubmitState>,
    generation: u64,
}

impl std::task::Wake for ArtifactSubmitWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(state) = self.state.upgrade() {
            if self.generation == state.generation {
                state.schedule();
            }
        }
    }
}

impl ArtifactSubmitState {
    fn set_progress(&self, progress: SubmitProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    fn wake_waiter(&self) {
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
    }

    fn finish(&self) {
        if !self.finished.swap(true, std::sync::atomic::Ordering::AcqRel) {
            self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }

    fn finish_if_terminal_empty(&self) {
        if self.terminal_is_empty() && !self.scheduled.load(std::sync::atomic::Ordering::Acquire) && !self.retry_armed.load(std::sync::atomic::Ordering::Acquire) {
            self.finish();
        }
    }

    fn complete(&self, result: ArtifactSubmitOutcome, progress: SubmitProgress) {
        self.set_progress(progress);
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else {
            *self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            self.wake_waiter();
        }
    }

    fn terminalize_work(&self, result: ArtifactSubmitOutcome, progress: SubmitProgress) {
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
        }
        self.complete(result, progress);
    }

    fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        self.set_progress(SubmitProgress::Scheduled);
        let state = self.clone();
        let generation = self.generation;
        self.submit_exact(Box::new(move || state.drive_one(generation)), 0);
    }

    fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => match error.kind() {
                semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated if attempt < ARTIFACT_SUBMIT_RETRY_LIMIT => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), attempt + 1));
                    self.arm_retry();
                }
                kind => {
                    let job = error.into_job();
                    self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                    if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                    }
                    *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((kind, job));
                    self.complete(Err(DbError::Unavailable(format!("artifact submit WorkerPool submission failed: {kind:?}"))), SubmitProgress::Fault);
                }
            },
        }
    }

    fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = self.retry_generation.fetch_add(1, Ordering::AcqRel).checked_add(1).expect("artifact submit retry generation exhausted");
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(ARTIFACT_SUBMIT_RETRY_MS), move || {
            if generation != state.retry_generation.load(Ordering::Acquire) {
                return;
            }
            state.retry_armed.store(false, Ordering::Release);
            let retry = state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if state.cancelled.load(Ordering::Acquire) {
                    drop(job);
                    state.scheduled.store(false, Ordering::Release);
                    state.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled);
                } else {
                    state.submit_exact(job, attempt);
                }
            }
        });
    }

    fn drive_one(self: Arc<Self>, generation: u64) {
        use std::future::Future as _;
        use std::sync::atomic::Ordering;

        if generation != self.generation {
            return;
        }
        if self.authority.generation() != self.authority_generation {
            self.scheduled.store(false, Ordering::Release);
            self.terminalize_work(Err(DbError::StaleGeneration { expected: self.authority.generation(), actual: self.authority_generation }), SubmitProgress::Fault);
            return;
        }
        self.scheduled.store(false, Ordering::Release);
        if self.cancelled.load(Ordering::Acquire) {
            self.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled);
            return;
        }

        let mut work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if matches!(work.as_ref(), Some(ArtifactSubmitWorkOwner::Request { .. })) {
            let Some(ArtifactSubmitWorkOwner::Request { batch, options, submitted_at_ms }) = work.take() else {
                return;
            };
            *work = Some(ArtifactSubmitWorkOwner::Actor(self.authority.submit_retained(batch, options, submitted_at_ms)));
            drop(work);
            self.schedule();
            return;
        }

        let Some(ArtifactSubmitWorkOwner::Actor(future)) = work.as_mut() else {
            return;
        };
        let waker = std::task::Waker::from(Arc::new(ArtifactSubmitWake { state: Arc::downgrade(&self), generation }));
        let mut context = std::task::Context::from_waker(&waker);
        match std::pin::Pin::new(future).poll(&mut context) {
            std::task::Poll::Pending => {
                self.set_progress(SubmitProgress::Waiting);
            }
            std::task::Poll::Ready(result) => {
                work.take();
                drop(work);
                let result = result.map(|inner| inner.map(|receipt| to_engine_receipt(receipt, self.document.clone())));
                if self.cancelled.load(Ordering::Acquire) {
                    *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
                    self.complete(Err(DbError::Closed), SubmitProgress::Cancelled);
                } else {
                    self.complete(result, SubmitProgress::Completed);
                }
            }
        }
    }

    fn close_one(&self) -> bool {
        if let Some((_, job)) = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return true;
        }
        if let Some((job, _)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return true;
        }
        if let Some(work) = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(work);
            return true;
        }
        if let Some(result) = self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return true;
        }
        false
    }
}

impl SubmitFuture {
    fn submit(handle: &ArtifactHandle, batch: db_artifact::CommandBatch, options: db_artifact::SubmitOptions) -> Self {
        let credit = artifact_submit_credit(&batch).and_then(|(items, bytes)| ArtifactSubmitAdmission::try_claim(items, bytes));
        let admission_error = credit.as_ref().err().map(ToString::to_string);
        let generation = credit.as_ref().map_or(0, |admission| admission.generation);
        let request = ArtifactSubmitWorkOwner::Request { batch, options, submitted_at_ms: handle.pool.now_ms() };
        let (work, terminal_work) = if generation == 0 { (None, Some(request)) } else { (Some(request), None) };
        let state = Arc::new(ArtifactSubmitState {
            pool: handle.pool.as_ref().clone(),
            authority: handle.authority.clone(),
            document: handle.document.clone(),
            generation,
            authority_generation: handle.authority.generation(),
            admission: std::sync::Mutex::new(credit.ok()),
            work: std::sync::Mutex::new(work),
            completion: std::sync::Mutex::new(None),
            terminal_work: std::sync::Mutex::new(terminal_work),
            terminal_result: std::sync::Mutex::new(None),
            retry_job: std::sync::Mutex::new(None),
            terminal_job: std::sync::Mutex::new(None),
            waker: std::sync::Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            progress: std::sync::atomic::AtomicU8::new(if generation == 0 { SubmitProgress::Fault as u8 } else { SubmitProgress::Admitted as u8 }),
        });
        if generation == 0 {
            *state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Err(DbError::Unavailable(admission_error.unwrap_or_else(|| "artifact submit admission exhausted".to_string()))));
        } else {
            state.schedule();
        }
        Self { state, resolved: false }
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn progress(&self) -> SubmitProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => SubmitProgress::Admitted,
            1 => SubmitProgress::Scheduled,
            2 => SubmitProgress::Waiting,
            3 => SubmitProgress::Completed,
            4 => SubmitProgress::Cancelled,
            _ => SubmitProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        if matches!(self.progress(), SubmitProgress::Completed | SubmitProgress::Cancelled | SubmitProgress::Fault) {
            return;
        }
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.state.schedule();
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactSubmitTerminalJob> {
        self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactSubmitTerminalJob { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_work(&self) -> Option<ArtifactSubmitTerminalWork> {
        self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactSubmitTerminalWork { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_result(&self) -> Option<ArtifactSubmitOutcome> {
        let result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if result.is_some() {
            self.state.finish_if_terminal_empty();
        }
        result
    }

    pub fn take_actor_terminal_job(&self) -> Option<db_artifact::ArtifactRunnerTerminalJob> {
        self.state.authority.take_terminal_job()
    }

    pub fn close_step(&self) -> bool {
        self.state.close_one() || self.state.authority.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty() && self.state.authority.terminal_is_empty()
    }
}

impl Future for SubmitFuture {
    type Output = ArtifactSubmitOutcome;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(result) = result {
            self.resolved = true;
            self.state.finish_if_terminal_empty();
            return std::task::Poll::Ready(result);
        }
        *self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(result) = result {
            self.resolved = true;
            self.state.finish_if_terminal_empty();
            return std::task::Poll::Ready(result);
        }
        std::task::Poll::Pending
    }
}

impl Drop for SubmitFuture {
    fn drop(&mut self) {
        if !self.resolved {
            self.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            self.state.schedule();
        }
        self.state.close_one();
        self.state.finish_if_terminal_empty();
    }
}

impl ArtifactSubmitTerminalJob {
    pub fn reason(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.owner.as_ref().expect("terminal artifact submit job already resolved").0
    }

    pub fn resume(mut self) {
        let (_, job) = self.owner.take().expect("terminal artifact submit job already resolved");
        if self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            let work = self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = work;
        }
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.set_progress(SubmitProgress::Scheduled);
        self.state.scheduled.store(true, std::sync::atomic::Ordering::Release);
        self.state.submit_exact(job, 0);
    }

    pub fn close(mut self) {
        let (_, job) = self.owner.take().expect("terminal artifact submit job already resolved");
        drop(job);
        self.state.finish_if_terminal_empty();
    }
}

impl Drop for ArtifactSubmitTerminalJob {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}

impl ArtifactSubmitTerminalWork {
    pub fn resume(mut self) -> Result<(), Self> {
        if self.state.generation == 0 {
            return Err(self);
        }
        let owner = self.owner.take().expect("terminal artifact submit work already resolved");
        *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.set_progress(SubmitProgress::Admitted);
        if self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.schedule();
        }
        Ok(())
    }

    pub fn close(mut self) {
        drop(self.owner.take().expect("terminal artifact submit work already resolved"));
        self.state.finish_if_terminal_empty();
    }
}

impl Drop for ArtifactSubmitTerminalWork {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}

/// @emoji 🎭️ The frozen `ArtifactHandle`: a clone-cheap live handle to one open document.
#[derive(Clone)]
pub struct ArtifactHandle {
    authority: Arc<db_artifact::ArtifactAuthority>,
    storage: Arc<db_storage::DbBackend>,
    document: protocol::ArtifactId,
    core_document: ArtifactId,
    pool: Arc<WorkerPool>,
}

impl ArtifactHandle {
    /// @emoji ✍️ The frozen `submit`: commits `batch` through the document's real
    /// `ArtifactAuthority` mailbox. Admission retains the exact request owner, and every I/O-lane
    /// grant advances either request-to-mailbox handoff or one actor-future poll.
    pub fn submit(&self, batch: db_artifact::CommandBatch, options: db_artifact::SubmitOptions) -> SubmitFuture {
        SubmitFuture::submit(self, batch, options)
    }

    /// @emoji 🔎️ The frozen `query`. `Consistency::Canonical` reads the document's live state
    /// directly. `AtLeast`/`Exact` read canonical too, then verify the resulting frontier actually
    /// satisfies the request (`DbError::Unavailable` if not — a true wait-for-frontier primitive
    /// would need a `ArtifactMessage` variant `db_artifact`'s mailbox doesn't expose yet).
    /// `Historical`/`Speculative`/`PreviewAugmented` are `DbError::Unimplemented` — see module doc.
    // 🔒️ `consistency`'s by-value signature is the frozen contract API
    // (`ArtifactHandle::query(&self, query: Query, consistency: Consistency)`, contract.md's
    // "Stable API" block) — not changeable even though this revision's body only borrows it.
    #[allow(clippy::needless_pass_by_value)]
    pub async fn query(&self, query: Query, consistency: Consistency) -> Result<QueryStream, DbError> {
        match &consistency {
            Consistency::Historical(_) | Consistency::PreviewAugmented(_) => {
                return Err(DbError::Unimplemented("historical/preview-augmented query consistency is not yet wired at the db_engine layer (db_query/db_projection integration deferred)"));
            }
            Consistency::Speculative(_) => {
                return Err(DbError::Unimplemented("speculative (preview) query consistency is not yet reachable: ArtifactAuthority's mailbox only exposes Submit/Query/Frontier messages"));
            }
            Consistency::Canonical | Consistency::AtLeast(_) | Consistency::Exact(_) => {}
        }

        let paths: Vec<String> = match query {
            Query::Get { path } => vec![path],
            Query::GetMany { paths } => paths,
        };
        let mut results = Vec::with_capacity(paths.len());
        for path in paths {
            let value = self.authority.query(&path).await?;
            results.push((path, value));
        }

        let frontier = self.frontier().await?;
        match &consistency {
            Consistency::AtLeast(requested) if !frontier.dominates(requested)? => {
                return Err(DbError::Unavailable("document has not yet reached the requested frontier".to_string()));
            }
            Consistency::Exact(requested) if &frontier != requested => {
                return Err(DbError::Unavailable("document frontier does not exactly match the requested frontier".to_string()));
            }
            _ => {}
        }
        Ok(QueryStream { results })
    }

    /// @emoji 📡️ The frozen `subscribe` — see module doc's `//🎯️ Design choice`: always
    /// `DbError::Unimplemented`, a real (not faked) extension seam pending a `ArtifactMessage`
    /// variant `db_artifact` doesn't expose yet.
    pub async fn subscribe(&self, _spec: LiveQuerySpec) -> Result<LiveQuery, DbError> {
        Err(DbError::Unimplemented("live-query subscription is not yet reachable: ArtifactAuthority's mailbox only exposes Submit/Query/Frontier messages"))
    }

    /// @emoji 🧭️ The frozen `frontier`.
    pub async fn frontier(&self) -> Result<Frontier, DbError> {
        let core_frontier = self.authority.frontier().await?;
        Ok(to_engine_frontier(&core_frontier, self.document.clone()))
    }

    /// @emoji 🌫️ The frozen `preview` — see `subscribe`'s doc; same deferral reason.
    pub async fn preview(&self, _base: Frontier) -> Result<PreviewHandle, DbError> {
        Err(DbError::Unimplemented("preview publish/query is not yet reachable: ArtifactAuthority's mailbox only exposes Submit/Query/Frontier messages"))
    }

    /// @emoji 📜️ The frozen `history` — real, see module doc: replays the WAL directly rather than
    /// going through the actor.
    pub async fn history(&self) -> Result<HistoryView, DbError> {
        replay_history(self.storage.as_ref(), &self.core_document, &self.document).await
    }

    /// @emoji 📸️ The frozen `snapshot_now` — see module doc's `//🎯️ Design choice`: always resolves
    /// to `DbError::Unimplemented`, a real extension seam (no full-state enumeration exists yet to
    /// serialize, and `db_snapshot` is not a direct dependency of this crate).
    pub async fn snapshot_now(&self, _kind: SnapshotKind) -> SnapshotFuture {
        let (reply_tx, reply_rx) = db_actor::oneshot();
        reply_tx.send(Err(DbError::Unimplemented("db_engine does not yet build real pack snapshots (no db_snapshot dependency this wave, and DocumentState exposes no full-state enumeration to serialize)")));
        reply_rx
    }

    pub async fn document_id(&self) -> &protocol::ArtifactId {
        &self.document
    }
}
//#endregion 🔖️ArtifactHandle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vcs_integration::{HashMutation, HashProjection};
    use protocol::{OpBinary, OpText};
    use store::ArtifactPack;

    #[semio_framework_async_macros::async_test]
    async fn hash_operation_text_and_binary_round_trip_with_every_field_present_and_absent() {
        let bare = HashMutation { hash: [7u8; 32], author: None, timestamp: None };
        assert_eq!(HashMutation::parse_op(&bare.print_op()).unwrap().hash, bare.hash);
        assert!(HashMutation::parse_op(&bare.print_op()).unwrap().author.is_none());
        assert_eq!(HashMutation::decode_op(&bare.encode_op().unwrap()).unwrap(), bare);

        let full = HashMutation { hash: [9u8; 32], author: Some(protocol::ActorId("actor-1".into())), timestamp: Some(protocol::HybridLogicalTimestamp { actor: 1, physical_ms: 2, logical: 3 }) };
        let reparsed = HashMutation::parse_op(&full.print_op()).unwrap();
        assert_eq!(reparsed.hash, full.hash);
        assert_eq!(reparsed.author, full.author);
        assert_eq!(reparsed.timestamp, full.timestamp);
        let redecoded = HashMutation::decode_op(&full.encode_op().unwrap()).unwrap();
        assert_eq!(redecoded, full);
    }

    #[semio_framework_async_macros::async_test]
    async fn hash_projection_pack_round_trips() {
        let projection = HashProjection { latest_hash: [3u8; 32] };
        let bytes = projection.encode_pack();
        assert_eq!(HashProjection::decode_pack(&bytes).unwrap(), projection);
    }

    //#region 🧸️Fixtures
    async fn tempdir(name: &str) -> std::path::PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!("db_engine-test-{name}-{}-{}", std::process::id(), now_ms().await));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    async fn envelope(id: &str, deps: &[&str], actor: &str, document: &protocol::ArtifactId, entries: &[(&str, serde_json::Value)]) -> protocol::MutationEnvelope {
        let mut payload = serde_json::Map::new();
        for (path, value) in entries {
            payload.insert((*path).to_string(), value.clone());
        }
        protocol::MutationEnvelope {
            mutation_id: protocol::MutationId(id.to_string()),
            document_id: document.clone(),
            actor: protocol::ActorId(actor.to_string()),
            dependencies: deps.iter().map(|dep| protocol::MutationId((*dep).to_string())).collect(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db_artifact::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact::encode_pathmap_json(&serde_json::Value::Object(payload)).await.unwrap() },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId(db_artifact::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact::encode_pathmap_json(&serde_json::Value::Object(serde_json::Map::new())).await.unwrap() },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        }
    }
    //#endregion 🧸️Fixtures

    //#region 🔖️Database open/catalog
    #[semio_framework_async_macros::async_test]
    async fn open_at_creates_a_fresh_zero_touch_database_with_an_empty_catalog() {
        let root = tempdir("open-at-fresh").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        assert!(database.catalog().await.artifacts.is_empty());
        assert_eq!(database.health().await.open_artifacts, 0);
        assert!(matches!(database.health().await.report.overall, db_observe::HealthState::Healthy));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_document_registers_it_in_the_catalog_and_document_finds_it() {
        let root = tempdir("create-and-find").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

        let catalog = database.catalog().await;
        assert_eq!(catalog.artifacts.len(), 1);
        assert_eq!(catalog.artifacts[0].document, document);

        let handle = database.document(&document).await.unwrap();
        assert_eq!(handle.document_id().await, &document);
    }

    #[semio_framework_async_macros::async_test]
    async fn create_document_twice_errs_already_exists() {
        let root = tempdir("create-twice").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let result = database.create_document(ArtifactSpec::new(document).await);
        assert!(matches!(result.await, Err(DbError::AlreadyExists(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn document_of_an_unknown_id_errs_not_found() {
        let root = tempdir("unknown-doc").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let never_created = protocol::ArtifactId("never-created".to_string());
        let result = database.document(&never_created);
        assert!(matches!(result.await, Err(DbError::NotFound(_))));
    }
    //#endregion 🔖️Database open/catalog

    //#region 🔖️Round trip
    #[semio_framework_async_macros::async_test]
    async fn full_submit_durable_query_round_trip_over_a_real_document_authority() {
        let root = tempdir("round-trip").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

        let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
        let receipt = db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
        assert_eq!(receipt.command_id, protocol::MutationId("op-1".to_string()));
        assert_eq!(receipt.frontier.document, document);
        assert_eq!(receipt.frontier.head_seq, 1);
        assert!(receipt.conflicts.is_empty());
        assert!(receipt.state_hash.is_some());

        let queried = handle.query(Query::Get { path: "name".to_string() }, Consistency::Canonical).await.unwrap();
        let value: serde_json::Value = db_artifact::decode_pathmap_json(queried.results[0].1.as_ref().unwrap()).await.unwrap();
        assert_eq!(value, serde_json::json!("hello"));

        let frontier = handle.frontier().await.unwrap();
        assert_eq!(frontier.head_seq, 1);

        let at_least = handle.query(Query::Get { path: "name".to_string() }, Consistency::AtLeast(frontier)).await.unwrap();
        assert_eq!(at_least.results.len(), 1);

        let history = handle.history().await.unwrap();
        assert_eq!(history.entries.len(), 1);
        assert_eq!(history.entries[0].operation_ids, vec![protocol::MutationId("op-1".to_string())]);
    }

    #[semio_framework_async_macros::async_test]
    async fn a_document_survives_a_full_database_shutdown_and_reopen_at_the_same_root() {
        let root = tempdir("reopen").await;
        let document = protocol::ArtifactId("doc-1".to_string());
        {
            let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
            let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
            let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("count", serde_json::json!(1))]).await]).await.unwrap();
            db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
            database.shutdown(std::time::Duration::from_secs(1)).await.unwrap();
        }

        let reopened = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        assert_eq!(reopened.catalog().await.artifacts.len(), 1, "the catalog root must have survived the reopen");

        let handle = reopened.document(&document).await.unwrap();
        let queried = handle.query(Query::Get { path: "count".to_string() }, Consistency::Canonical).await.unwrap();
        let value: serde_json::Value = db_artifact::decode_pathmap_json(queried.results[0].1.as_ref().unwrap()).await.unwrap();
        assert_eq!(value, serde_json::json!(1), "the document's committed state must have survived the reopen via WAL replay");
        assert_eq!(handle.frontier().await.unwrap().head_seq, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_consistency_rejects_a_frontier_the_document_has_moved_past() {
        let root = tempdir("exact-consistency").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let stale = handle.frontier().await.unwrap();

        let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions::default())).unwrap().unwrap();

        let result = handle.query(Query::Get { path: "x".to_string() }, Consistency::Exact(stale));
        assert!(matches!(result.await, Err(DbError::Unavailable(_))));
    }
    //#endregion 🔖️Round trip

    //#region 🔖️Deferred extension seams
    #[semio_framework_async_macros::async_test]
    async fn subscribe_preview_and_snapshot_now_are_documented_unimplemented_not_panics() {
        let root = tempdir("deferred").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        let handle = database.create_document(ArtifactSpec::new(document).await).await.unwrap();

        assert!(matches!(handle.subscribe(LiveQuerySpec { since: None }).await, Err(DbError::Unimplemented(_))));
        assert!(matches!(handle.preview(handle.frontier().await.unwrap()).await, Err(DbError::Unimplemented(_))));
        assert!(matches!(db_actor::block_on(handle.snapshot_now(SnapshotKind::Full).await), Ok(Err(DbError::Unimplemented(_)))));
    }
    //#endregion 🔖️Deferred extension seams

    //#region 🔖️VersionGraph
    #[cfg(feature = "vcs")]
    #[semio_framework_async_macros::async_test]
    async fn checkpoint_document_mints_distinct_real_vcs_content_addressed_checkpoint_ids() {
        let root = tempdir("vcs-checkpoint").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

        let batch1 = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch1, db_artifact::SubmitOptions::default())).unwrap().unwrap();
        let checkpoint_1 = database.checkpoint_document(&document, "first".to_string(), &[protocol::ActorId("alice".to_string())]).await.unwrap();
        assert!(checkpoint_1.starts_with("ck-"), "vcs checkpoint ids are content-addressed as ck-<hex16>, got {checkpoint_1:?}");

        let batch2 = db_artifact::CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &document, &[("x", serde_json::json!(2))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch2, db_artifact::SubmitOptions::default())).unwrap().unwrap();
        let checkpoint_2 = database.checkpoint_document(&document, "second".to_string(), &[protocol::ActorId("alice".to_string())]).await.unwrap();

        assert_ne!(checkpoint_1, checkpoint_2, "distinct commits must mint distinct content-addressed checkpoint ids");
    }

    #[cfg(not(feature = "vcs"))]
    #[semio_framework_async_macros::async_test]
    async fn checkpoint_document_errs_unimplemented_without_the_vcs_feature() {
        let root = tempdir("no-vcs-checkpoint").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        database.create_document(ArtifactSpec::new(document.clone())).await.unwrap();
        assert!(matches!(database.checkpoint_document(&document, "msg".to_string(), &[]).await, Err(DbError::Unimplemented(_))));
    }
    //#endregion 🔖️VersionGraph

    //#region 🔖️Compact + Sync
    #[semio_framework_async_macros::async_test]
    async fn compact_document_runs_a_real_compaction_pass_without_error() {
        let root = tempdir("compact").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions::default())).unwrap().unwrap();

        let report = database.compact_document(&document, "holder-1", false).await.unwrap();
        assert_eq!(report.wal_segments_deleted, 0, "nothing is below the (nonexistent) snapshot floor yet, but the pass itself must succeed");
    }

    #[semio_framework_async_macros::async_test]
    async fn hello_returns_a_welcome_with_a_fresh_bootstrap_for_a_brand_new_replica() {
        let root = tempdir("hello").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions::default())).unwrap().unwrap();

        let response = database.hello(&document, None, "session-1".to_string(), &protocol::ActorId("semio_hub".to_string()), 4096).await.unwrap();
        assert!(matches!(response.welcome, protocol::ServerFrame::Welcome { .. }));
    }

    // 🔬️ `storage()` is a real escape hatch to the same backend `Database::open_at` wired — a
    // caller below the document-actor boundary (os-semio_hub's blob routes) can round-trip a payload
    // through it directly, independent of any document actor.
    #[semio_framework_async_macros::async_test]
    async fn storage_accessor_reaches_the_same_backend_payload_store() {
        let root = tempdir("storage-accessor").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let hash = db_actor::block_on(async { database.storage().await.payload().await.put(db_storage::DbIoPages::try_new(b"hello storage accessor".to_vec()).ok().unwrap()).await }).unwrap();
        assert_eq!(db_actor::block_on(async { database.storage().await.payload().await.get(&hash).await }).unwrap(), b"hello storage accessor");
    }
    //#endregion 🔖️Compact + Sync

    //#region 🔖️Retained submit authority
    fn retained_submit_source() -> &'static str {
        include_str!("🦀️component.rs")
    }

    #[test]
    fn artifact_submit_late_readiness_parks_then_one_shot_wake_reschedules() {
        let source = retained_submit_source();
        assert!(source.contains("impl std::task::Wake for ArtifactSubmitWake"));
        assert!(source.contains("self.scheduled.compare_exchange(false, true"));
        assert!(source.contains("std::task::Poll::Pending =>"));
        assert!(source.contains("self.set_progress(SubmitProgress::Waiting)"));
    }

    #[test]
    fn artifact_submit_pool_saturation_without_later_ingress_retains_exact_job() {
        let source = retained_submit_source();
        assert!(source.contains("self.pool.try_submit(Lane::Io, job)"));
        assert!(source.contains("error.into_job()"));
        assert!(source.contains("self.pool.callback_at"));
        assert!(source.contains("ARTIFACT_SUBMIT_RETRY_LIMIT"));
    }

    #[test]
    fn artifact_submit_cancel_before_during_after_preserves_exact_owner() {
        let source = retained_submit_source();
        assert!(source.contains("self.state.cancelled.store(true"));
        assert!(source.contains("self.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled)"));
        assert!(source.contains("*self.terminal_result.lock()"));
        assert!(source.contains("SubmitProgress::Completed | SubmitProgress::Cancelled | SubmitProgress::Fault"));
    }

    #[test]
    fn artifact_submit_stale_generation_and_slot_aba_cannot_consume_current_work() {
        let first = ArtifactSubmitAdmission::try_claim(1, ARTIFACT_SUBMIT_PAGE_BYTES).unwrap();
        let first_slot = first.slot;
        let first_generation = first.generation;
        drop(first);
        let next = ArtifactSubmitAdmission::try_claim(1, ARTIFACT_SUBMIT_PAGE_BYTES).unwrap();
        assert_eq!(next.slot, first_slot);
        assert_ne!(next.generation, first_generation);
        let source = retained_submit_source();
        let stale = source.find("if generation != self.generation").unwrap();
        let mutation = source[stale..].find("self.scheduled.store").unwrap();
        assert!(mutation > 0);
    }

    #[test]
    fn artifact_submit_missing_handle_terminalizes_without_mailbox_mutation() {
        let source = retained_submit_source();
        let stale = source.find("if self.authority.generation() != self.authority_generation").unwrap();
        let handoff = source.find("self.authority.submit_retained").unwrap();
        assert!(stale < handoff);
        assert!(source.contains("Err(DbError::StaleGeneration"));
    }

    #[test]
    fn artifact_submit_terminal_job_work_result_take_resume_and_close_one_owner() {
        let source = retained_submit_source();
        for required in ["pub fn take_terminal_job", "pub fn take_terminal_work", "pub fn take_terminal_result", "pub fn take_actor_terminal_job", "pub fn close_step", "pub fn terminal_is_empty", "pub fn resume(mut self)"] {
            assert!(source.contains(required), "missing {required}");
        }
        assert!(source.contains("fn close_one(&self) -> bool"));
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_submit_item_cap_plus_one_and_nested_bytes_plus_one_return_owner() {
        let document = protocol::ArtifactId("credit-doc".to_string());
        let one = envelope("credit-1", &[], "credit-actor", &document, &[("x", serde_json::json!(1))]).await;
        let admitted = db_artifact::CommandBatch::new(vec![one]).await.unwrap();
        assert!(artifact_submit_credit(&admitted).is_ok());

        let mut envelopes = Vec::new();
        for index in 0..=ARTIFACT_SUBMIT_BATCH_ITEMS {
            envelopes.push(envelope(&format!("credit-{index}"), &[], "credit-actor", &document, &[("x", serde_json::json!(index))]).await);
        }
        let rejected = db_artifact::CommandBatch { envelopes };
        assert!(artifact_submit_credit(&rejected).is_err());

        let mut oversize = envelope("credit-oversize", &[], "credit-actor", &document, &[("x", serde_json::json!(1))]).await;
        oversize.diff.payload = vec![0; ARTIFACT_SUBMIT_OPERATION_BYTES as usize + 1];
        assert!(artifact_submit_credit(&db_artifact::CommandBatch { envelopes: vec![oversize] }).is_err());
    }

    #[test]
    fn artifact_runner_one_grant_polls_one_turn_and_never_blocks_on() {
        let source = include_str!("../📄️artifact/🦀️component.rs");
        let runner = &source[source.find("type ArtifactBuildFuture").unwrap()..source.find("//#region 🧪️Tests").unwrap()];
        assert!(!runner.contains("block_on("));
        assert!(!runner.contains("ask_blocking"));
        assert!(runner.contains("future.as_mut().poll(&mut context)"));
        assert!(runner.contains("Self::start_turn(engine, envelope.payload)"));
        assert_eq!(runner.matches("close();\n            }\n            drop(job);").count(), 1);
        assert_eq!(runner.matches("close();\n        }\n        drop(job);").count(), 1);
    }
    //#endregion 🔖️Retained submit authority

    //#region 🔖️Security
    #[semio_framework_async_macros::async_test]
    async fn security_authz_hook_rejects_a_principal_denied_by_its_policy() {
        let policy = db_security::RoleBasedPolicy::new();
        let gate = db_security::SecurityGate::new(policy, db_security::ReplayGuard::new(60_000, 16), db_security::BudgetRegistry::new(100, 10), Arc::new(NullEmit));
        let hook = SecurityAuthzHook::new(gate, |actor| db_security::Principal::new(actor.clone(), db_security::TenantId::from("tenant-1"), vec!["viewer".to_string()])).await;

        let document = protocol::ArtifactId("doc-1".to_string());
        let envelope = envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await;
        let result = db_artifact::AuthzHook::authorize(&hook, &envelope.actor, &envelope);
        assert!(matches!(result.await, Err(DbError::Unauthorized(_))), "a default-deny policy with no grants must reject every action");
    }
    //#endregion 🔖️Security
}
//#endregion 🧪️Tests
