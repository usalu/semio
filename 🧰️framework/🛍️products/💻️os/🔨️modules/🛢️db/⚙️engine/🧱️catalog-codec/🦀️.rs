//! 🧱️ Db engine shared types, catalog codec and document mount registry.
#![allow(async_fn_in_trait)]

use db_durability::{DurabilityClass, EpochFence};
use db_ids::{ArtifactId, DbError};
use db_io_core::CatalogStorage as _;
use pack::ContentHash;
use semio_framework_async::{Lane, WorkerPool};
use std::{collections::HashMap, future::Future, sync::Arc, sync::Mutex};

//#region 🔖️Ids
/// 🌉️ `protocol::ArtifactId` → `ArtifactId`, the lossless single-`String` bridge
/// `db_core`'s module doc promises — see `db_artifact`'s identical helper for the rationale (this
/// crate is the other place in the family that depends on both `db_core` and `protocol`).
pub async fn to_core_document_id(id: &protocol::ArtifactId) -> ArtifactId {
    ArtifactId(id.0.clone())
}

pub async fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
}

/// 🧵️ Every law owns its pool: laws shut their pool down and count its retained uses, so a
/// process-shared pool would let concurrent laws observe or stop each other's pool.
#[cfg(any(test, feature = "testkit"))]
pub fn test_worker_pool() -> Arc<WorkerPool> {
    Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 4)))
}
//#endregion 🔖️Ids
//#region 🔖️Frontier
/// 🧭️ The facade-level frontier: identical shape to `Frontier` except keyed by
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

/// 📍️ One document-actor generation's exact checkpoint-publication observation.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckpointPublicationSnapshot {
    pub authority_generation: u64,
    pub frontier: Frontier,
    pub head_edit_id: Option<protocol::MutationId>,
}

impl Frontier {
    /// 🏔️ True iff `self` has observed everything `other` has — mirrors
    /// `Frontier::dominates`, re-derived here since this type's `document` field has a
    /// different type than `Frontier`'s.
    pub fn dominates(&self, other: &Frontier) -> Result<bool, DbError> {
        if self.document != other.document {
            return Err(DbError::InvalidArgument(format!("frontier document mismatch: {} vs {}", self.document.0, other.document.0)));
        }
        Ok(self.head_seq >= other.head_seq && self.commit_seq >= other.commit_seq && self.epoch >= other.epoch)
    }
}

pub fn to_engine_frontier(core: &db_durability::Frontier, document: protocol::ArtifactId) -> Frontier {
    Frontier { document, head_seq: core.head_seq, commit_seq: core.commit_seq, chain_hash: core.chain_hash, epoch: core.epoch }
}
//#endregion 🔖️Frontier
//#region 🔖️Receipt
/// 🧾️ The frozen `CommandReceipt` shape: `ArtifactHandle::submit`'s resolved output.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandReceipt {
    pub command_id: protocol::MutationId,
    pub frontier: Frontier,
    pub durability: DurabilityClass,
    pub conflicts: Vec<db_artifact_contract::ConflictRecord>,
    pub state_hash: Option<ContentHash>,
    pub messages: Vec<protocol::MutationMessage>,
}

pub fn to_engine_receipt(receipt: db_artifact_contract::CommandReceipt, document: protocol::ArtifactId) -> CommandReceipt {
    CommandReceipt { command_id: receipt.command_id, frontier: to_engine_frontier(&receipt.frontier, document), durability: receipt.durability, conflicts: receipt.conflicts, state_hash: receipt.state_hash, messages: receipt.messages }
}
//#endregion 🔖️Receipt
//#region 🔖️Consistency
/// 🎚️ The frozen `Consistency` enum: which frontier/view `ArtifactHandle::query` must
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
/// 🔎️ What `ArtifactHandle::query` can ask for — this crate's own choice (the contract fixes
/// `query`'s signature, not `Query`'s shape): single or multi-path point lookups against the
/// document's schema-erased path/value convention (see `db_artifact`'s module doc), matching what
/// `ArtifactAuthority`'s mailbox actually exposes (`ArtifactMessage::Query { path, .. }`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Query {
    Get { path: String },
    GetMany { paths: Vec<String> },
}

/// 📬️ One resolved `query`: every requested path paired with its current value bytes (`None`
/// if unset/tombstoned).
#[derive(Debug)]
pub struct QueryResultEntry {
    pub path: db_io_core::DbIoText,
    pub value: Option<db_query::QueryBytes>,
}

impl QueryResultEntry {
    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    pub fn value(&self) -> Option<&db_query::QueryBytes> {
        self.value.as_ref()
    }

    pub fn into_parts(self) -> (db_io_core::DbIoText, Option<db_query::QueryBytes>) {
        (self.path, self.value)
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if let Some(value) = self.value.as_mut() {
            if value.close_step()?.is_some() {
                return Ok(true);
            }
        }
        self.value = None;
        Ok(self.path.close_step())
    }
}

pub struct QueryStream {
    pub results: [Option<QueryResultEntry>; 64],
    pub len: u8,
    pub retirement: Option<EngineQueryRetirementReservation>,
}

const ENGINE_RETIRED_QUERY_STREAMS: usize = 64;

static ENGINE_QUERY_RETIREMENT: Mutex<[Option<QueryStream>; ENGINE_RETIRED_QUERY_STREAMS]> = Mutex::new([const { None }; ENGINE_RETIRED_QUERY_STREAMS]);

static ENGINE_QUERY_RETIREMENT_OVERFLOW: Mutex<[Option<QueryStream>; ENGINE_RETIRED_QUERY_STREAMS]> = Mutex::new([const { None }; ENGINE_RETIRED_QUERY_STREAMS]);

static ENGINE_QUERY_RETIREMENT_QUARANTINE: Mutex<[Option<QueryStream>; ENGINE_RETIRED_QUERY_STREAMS]> = Mutex::new([const { None }; ENGINE_RETIRED_QUERY_STREAMS]);

static ENGINE_QUERY_RETIREMENT_RESERVATIONS: [std::sync::atomic::AtomicU64; 3] = [const { std::sync::atomic::AtomicU64::new(0) }; 3];

static ENGINE_QUERY_RETIREMENT_PRESSURE_FAULT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Clone, Copy)]
struct EngineQueryRetirementReservation {
    tier: u8,
    pub index: u8,
}

pub fn reserve_engine_query_retirement() -> Option<EngineQueryRetirementReservation> {
    for tier in 0..3u8 {
        for index in 0..ENGINE_RETIRED_QUERY_STREAMS as u8 {
            let bit = 1u64 << index;
            if ENGINE_QUERY_RETIREMENT_RESERVATIONS[tier as usize].fetch_or(bit, std::sync::atomic::Ordering::AcqRel) & bit != 0 {
                continue;
            }
            let vacant = match tier {
                0 => ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                1 => ENGINE_QUERY_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                _ => ENGINE_QUERY_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
            };
            if vacant {
                if tier != 0 {
                    ENGINE_QUERY_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
                }
                return Some(EngineQueryRetirementReservation { tier, index });
            }
            ENGINE_QUERY_RETIREMENT_RESERVATIONS[tier as usize].fetch_and(!bit, std::sync::atomic::Ordering::AcqRel);
        }
    }
    ENGINE_QUERY_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
    None
}

fn release_engine_query_retirement(reservation: &mut Option<EngineQueryRetirementReservation>) {
    if let Some(reservation) = reservation.take() {
        ENGINE_QUERY_RETIREMENT_RESERVATIONS[reservation.tier as usize].fetch_and(!(1u64 << reservation.index), std::sync::atomic::Ordering::AcqRel);
    }
}

fn engine_query_vacant_retirement_slot(tier: usize, slots: &[Option<QueryStream>]) -> Option<usize> {
    let reserved = ENGINE_QUERY_RETIREMENT_RESERVATIONS[tier].load(std::sync::atomic::Ordering::Acquire);
    slots.iter().enumerate().position(|(index, slot)| slot.is_none() && reserved & (1u64 << index) == 0)
}

fn install_reserved_engine_query_stream(owner: QueryStream) {
    let reservation = owner.retirement.unwrap_or(EngineQueryRetirementReservation { tier: 0, index: 0 });
    match reservation.tier {
        0 => ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        1 => ENGINE_QUERY_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        _ => ENGINE_QUERY_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
    }
}

impl QueryStream {
    pub fn new() -> Self {
        Self { results: std::array::from_fn(|_| None), len: 0, retirement: None }
    }

    pub fn push(&mut self, entry: QueryResultEntry) -> Result<(), QueryResultEntry> {
        if self.len() == self.results.len() {
            return Err(entry);
        }
        if self.len == 0 && self.retirement.is_none() {
            let Some(retirement) = reserve_engine_query_retirement() else { return Err(entry) };
            self.retirement = Some(retirement);
        }
        let index = self.len();
        self.results[index] = Some(entry);
        self.len += 1;
        Ok(())
    }

    pub fn push_preflighted(&mut self, entry: QueryResultEntry) {
        let index = self.len();
        self.results[index] = Some(entry);
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &QueryResultEntry> {
        self.results[..self.len()].iter().flatten()
    }

    pub fn get(&self, index: usize) -> Option<&QueryResultEntry> {
        self.results.get(index).and_then(Option::as_ref)
    }

    pub fn take(&mut self) -> Option<QueryResultEntry> {
        if self.len == 0 {
            return None;
        }
        let entry = self.results[0].take();
        for index in 1..self.len() {
            self.results[index - 1] = self.results[index].take();
        }
        self.len -= 1;
        if self.len == 0 {
            release_engine_query_retirement(&mut self.retirement);
        }
        entry
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.len == 0 {
            return Ok(false);
        }
        let index = self.len() - 1;
        let entry = self.results[index].as_mut().ok_or_else(|| DbError::Internal("engine query close lost entry".to_string()))?;
        if entry.close_step()? {
            return Ok(true);
        }
        self.results[index] = None;
        self.len -= 1;
        if self.len == 0 {
            release_engine_query_retirement(&mut self.retirement);
        }
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.results.iter().all(Option::is_none)
    }
}

#[cfg(test)]
fn retire_engine_query_stream(owner: QueryStream) -> Result<(), QueryStream> {
    if owner.retirement.is_some() {
        install_reserved_engine_query_stream(owner);
        return Ok(());
    }
    let mut retired = ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(index) = engine_query_vacant_retirement_slot(0, &retired[..]) {
        retired[index] = Some(owner);
        Ok(())
    } else {
        drop(retired);
        ENGINE_QUERY_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        let mut overflow = ENGINE_QUERY_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(index) = engine_query_vacant_retirement_slot(1, &overflow[..]) {
            overflow[index] = Some(owner);
            return Ok(());
        }
        drop(overflow);
        let mut quarantine = ENGINE_QUERY_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = engine_query_vacant_retirement_slot(2, &quarantine[..]) else { return Err(owner) };
        quarantine[index] = Some(owner);
        Ok(())
    }
}

pub fn engine_query_maintenance_step() -> Result<bool, DbError> {
    let mut retired = ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = retired.iter_mut().find(|slot| slot.is_some()) else {
        drop(retired);
        let mut overflow = ENGINE_QUERY_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = overflow.iter().position(Option::is_some) else {
            drop(overflow);
            let mut quarantine = ENGINE_QUERY_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(index) = quarantine.iter().position(Option::is_some) else { return Ok(false) };
            let mut retired = ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(target) = engine_query_vacant_retirement_slot(0, &retired[..]) else {
                drop(retired);
                let owner = quarantine[index].as_mut().ok_or_else(|| DbError::Internal("engine query quarantine changed stream owner".to_string()))?;
                if !owner.close_step()? {
                    quarantine[index] = None;
                }
                return Ok(true);
            };
            retired[target] = quarantine[index].take();
            return Ok(true);
        };
        let mut retired = ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(target) = engine_query_vacant_retirement_slot(0, &retired[..]) else {
            drop(retired);
            let owner = overflow[index].as_mut().ok_or_else(|| DbError::Internal("engine query overflow retirement changed stream owner".to_string()))?;
            if !owner.close_step()? {
                overflow[index] = None;
            }
            return Ok(true);
        };
        retired[target] = overflow[index].take();
        return Ok(true);
    };
    let owner = slot.as_mut().ok_or_else(|| DbError::Internal("engine query retirement changed owner".to_string()))?;
    if owner.close_step()? {
        return Ok(true);
    }
    *slot = None;
    Ok(true)
}

impl Drop for QueryStream {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            release_engine_query_retirement(&mut self.retirement);
            return;
        }
        install_reserved_engine_query_stream(std::mem::replace(self, Self::new()));
    }
}
//#endregion 🔖️Query
//#region 🔖️LiveQuery + Preview
/// 📡️ What `ArtifactHandle::subscribe` would filter on — defined for API-shape completeness
/// even though every construction path currently returns `DbError::Unimplemented` (see module doc).
#[derive(Clone, Debug, PartialEq)]
pub struct LiveQuerySpec {
    pub since: Option<Frontier>,
}

/// 📡️ A live subscription handle — see `LiveQuerySpec`'s doc on why this is currently
/// unreachable except through the documented `Unimplemented` error.
#[derive(Clone, Debug, PartialEq)]
pub struct LiveQuery {
    pub id: String,
}

/// 🌫️ An ephemeral preview overlay handle — see `LiveQuerySpec`'s doc; same deferral reason.
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewHandle {
    pub id: String,
    pub base: Frontier,
}
//#endregion 🔖️LiveQuery + Preview
//#region 🔖️Snapshot
/// 📸️ What kind of snapshot `ArtifactHandle::snapshot_now` was asked to build — defined for
/// API-shape completeness (see module doc: this crate does not yet build real pack snapshots).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotKind {
    Full,
    Incremental,
}

/// 📸️ What a successful `snapshot_now` would resolve to — currently unreachable, see above.
#[derive(Clone, Debug, PartialEq)]
pub struct SnapshotReceipt {
    pub generation: u64,
    pub frontier: Frontier,
}

pub type SnapshotFuture = db_actor::ReplyReceiver<Result<SnapshotReceipt, DbError>>;
//#endregion 🔖️Snapshot
//#region 🔖️Observe
/// 📡️ The default observability wiring `Database::open`/`open_at` build when the caller
/// doesn't supply their own: an in-memory `db_observe::StructuredSink` (real JSON-lines encoding,
/// just not flushed anywhere durable by default — a caller wanting file/pipe output constructs
/// `db_observe::WriterSink` themselves and passes it via `Database::open_with_emit`).
// 🔀️ dedyn-emit-runtime, O1/R1: concrete return type (`Database`'s `E` default matches it exactly),
// not `Arc<dyn Emit>` — every caller (`open`/`open_at`) infers `E` from this value.
pub async fn default_emit() -> Arc<db_observe::StructuredSink<db_observe::MemorySink>> {
    Arc::new(db_observe::StructuredSink::new(db_observe::MemorySink::new()))
}
//#endregion 🔖️Observe
//#region 🔖️Catalog
/// 📇️ One document known to this `Database`'s catalog.
#[derive(Clone, Debug, PartialEq)]
pub struct CatalogEntry {
    pub document: protocol::ArtifactId,
    pub created_at_ms: u64,
}

/// 📇️ A point-in-time read of every document this `Database` knows about.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CatalogView {
    pub artifacts: Vec<CatalogEntry>,
}

/// 💾️ The catalog root's on-disk shape — a plain JSON array, deliberately NOT reusing
/// `CatalogEntry` directly (keeps the public type free of a `serde` bound it doesn't otherwise need).
#[derive(serde::Serialize, serde::Deserialize)]
struct CatalogRootEntry {
    pub document: String,
    pub created_at_ms: u64,
}

fn json_string_len(value: &str) -> usize {
    value
        .as_bytes()
        .iter()
        .map(|byte| match byte {
            b'"' | b'\\' | 0x08 | 0x0c | b'\n' | b'\r' | b'\t' => 2,
            0x00..=0x1f => 6,
            _ => 1,
        })
        .sum::<usize>()
        + 2
}

pub fn decimal_u64<'a>(value: u64, buffer: &'a mut [u8; 20]) -> &'a [u8] {
    let mut cursor = buffer.len();
    let mut remaining = value;
    loop {
        cursor -= 1;
        buffer[cursor] = b'0' + (remaining % 10) as u8;
        remaining /= 10;
        if remaining == 0 {
            return &buffer[cursor..];
        }
    }
}

async fn catalog_write(writer: &mut db_io_core::DbIoPageWriter, bytes: &[u8]) -> Result<(), DbError> {
    let mut cursor = 0;
    while cursor < bytes.len() {
        cursor += writer.write_fragment(&bytes[cursor..])?;
        semio_framework_async::yield_once().await;
    }
    Ok(())
}

async fn catalog_write_json_string(writer: &mut db_io_core::DbIoPageWriter, value: &str) -> Result<(), DbError> {
    catalog_write(writer, b"\"").await?;
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in value.as_bytes() {
        let mut escape = [0u8; 6];
        let encoded: &[u8] = match *byte {
            b'"' => b"\\\"",
            b'\\' => b"\\\\",
            0x08 => b"\\b",
            0x0c => b"\\f",
            b'\n' => b"\\n",
            b'\r' => b"\\r",
            b'\t' => b"\\t",
            value @ 0x00..=0x1f => {
                escape.copy_from_slice(&[b'\\', b'u', b'0', b'0', HEX[(value >> 4) as usize], HEX[(value & 0x0f) as usize]]);
                &escape
            }
            _ => std::slice::from_ref(byte),
        };
        catalog_write(writer, encoded).await?;
    }
    catalog_write(writer, b"\"").await
}

pub async fn encode_catalog_pages(entries: &[CatalogEntry]) -> Result<db_io_core::DbIoPages, DbError> {
    let mut encoded_len = 2usize;
    for (index, entry) in entries.iter().enumerate() {
        let mut decimal = [0u8; 20];
        encoded_len = encoded_len
            .checked_add(usize::from(index != 0))
            .and_then(|length| length.checked_add(b"{\"document\":".len()))
            .and_then(|length| length.checked_add(json_string_len(&entry.document.0)))
            .and_then(|length| length.checked_add(b",\"created_at_ms\":".len()))
            .and_then(|length| length.checked_add(decimal_u64(entry.created_at_ms, &mut decimal).len() + 1))
            .ok_or(DbError::LimitExceeded("catalog encoded bytes"))?;
        semio_framework_async::yield_once().await;
    }
    let mut writer = db_io_core::DbIoPageWriter::try_reserve(encoded_len.div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
    catalog_write(&mut writer, b"[").await?;
    for (index, entry) in entries.iter().enumerate() {
        if index != 0 {
            catalog_write(&mut writer, b",").await?;
        }
        catalog_write(&mut writer, b"{\"document\":").await?;
        catalog_write_json_string(&mut writer, &entry.document.0).await?;
        catalog_write(&mut writer, b",\"created_at_ms\":").await?;
        let mut decimal = [0u8; 20];
        catalog_write(&mut writer, decimal_u64(entry.created_at_ms, &mut decimal)).await?;
        catalog_write(&mut writer, b"}").await?;
    }
    catalog_write(&mut writer, b"]").await?;
    writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)
}

pub async fn decode_catalog(bytes: &[u8]) -> Result<Vec<CatalogEntry>, DbError> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let raw: Vec<CatalogRootEntry> = serde_json::from_slice(bytes).map_err(|err| DbError::Corrupt(format!("catalog decode: {err}")))?;
    Ok(raw.into_iter().map(|entry| CatalogEntry { document: protocol::ArtifactId(entry.document), created_at_ms: entry.created_at_ms }).collect())
}

pub struct CatalogState {
    pub epoch: EpochFence,
    pub revision: u64,
    pub entries: Arc<Vec<CatalogEntry>>,
    pub pending: Option<DatabaseCreateCatalogToken>,
}
//#endregion 🔖️Catalog
//#region 🔖️CreateDocumentCatalogCas
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DatabaseCreateCatalogToken {
    pub slot: usize,
    pub generation: u64,
    pub revision: u64,
}
//#endregion 🔖️CreateDocumentCatalogCas

pub const DATABASE_DOCUMENT_MOUNT_WAITERS: usize = 32;

#[cfg(any(test, feature = "testkit"))]
pub type DatabaseDocumentMountPublishedHook = Arc<dyn Fn(&protocol::ArtifactId) + Send + Sync>;

#[cfg(any(test, feature = "testkit"))]
pub type DatabaseDocumentMountCleanupFaultHook = Arc<dyn Fn() + Send + Sync>;

#[cfg(any(test, feature = "testkit"))]
pub type DatabaseDocumentMountParkedHook = Arc<dyn Fn() + Send + Sync>;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DatabaseDocumentMountPolicy {
    Ensure,
    Create,
    Open,
}

#[derive(Clone)]
pub struct DatabaseDocumentMountReply {
    pub authority: Arc<db_artifact::ArtifactAuthority>,
    pub handles: Arc<std::sync::atomic::AtomicUsize>,
}

pub struct DatabaseDocumentMountWaiter {
    pub policy: DatabaseDocumentMountPolicy,
    pub create_claim: bool,
    pub reply: db_actor::ReplySender<Result<DatabaseDocumentMountReply, DbError>>,
}

pub struct DatabaseDocumentMountWait {
    pub receiver: db_actor::ReplyReceiver<Result<DatabaseDocumentMountReply, DbError>>,
    pub registry: std::sync::Weak<Mutex<DatabaseDocumentMountRegistry>>,
    pub document: String,
    pub generation: u64,
    pub slot: usize,
    pub resolved: bool,
}

impl Future for DatabaseDocumentMountWait {
    type Output = Result<Result<DatabaseDocumentMountReply, DbError>, DbError>;

    fn poll(self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        match std::pin::Pin::new(&mut this.receiver).poll(context) {
            std::task::Poll::Ready(result) => {
                this.resolved = true;
                std::task::Poll::Ready(result)
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

impl Drop for DatabaseDocumentMountWait {
    fn drop(&mut self) {
        if self.resolved {
            return;
        }
        let Some(registry) = self.registry.upgrade() else { return };
        let mut registry = registry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(DatabaseDocumentMountSlot::Opening { generation, waiters, .. }) = registry.slots.get_mut(&self.document) else { return };
        if *generation == self.generation {
            waiters.get_mut(self.slot).and_then(Option::take);
        }
    }
}

pub enum DatabaseDocumentMountFailure {
    Terminal(DbError),
    Retained { rejected: db_artifact_engine::ArtifactEngineOpenRejected, resume: Option<DatabaseDocumentMountFuture> },
}

impl From<DbError> for DatabaseDocumentMountFailure {
    fn from(error: DbError) -> Self {
        Self::Terminal(error)
    }
}

pub type DatabaseDocumentMountFuture = std::pin::Pin<Box<dyn Future<Output = Result<DatabaseDocumentMountReply, DatabaseDocumentMountFailure>> + Send + 'static>>;

type DatabaseDocumentMountCleanupFuture = std::pin::Pin<Box<dyn Future<Output = Result<DbError, db_artifact_engine::ArtifactEngineOpenRejected>> + Send + 'static>>;

pub enum DatabaseDocumentMountWork {
    Mount(DatabaseDocumentMountFuture),
    Cleanup { future: DatabaseDocumentMountCleanupFuture, resume: Option<DatabaseDocumentMountFuture> },
    Parked { rejected: db_artifact_engine::ArtifactEngineOpenRejected, resume: Option<DatabaseDocumentMountFuture> },
    Empty,
}

enum DatabaseDocumentMountPoll {
    Pending,
    Continue,
    Complete(Result<DatabaseDocumentMountReply, DbError>),
}

pub enum DatabaseDocumentMountSlot {
    Opening { generation: u64, owner: Arc<DatabaseDocumentMountOwner>, waiters: [Option<DatabaseDocumentMountWaiter>; DATABASE_DOCUMENT_MOUNT_WAITERS] },
    Ready(DatabaseDocumentMountReply),
    Closing { terminal: db_actor::ReplyReceiver<()> },
}

pub struct DatabaseDocumentMountRegistry {
    pub slots: HashMap<String, DatabaseDocumentMountSlot>,
    pub next_generation: u64,
}

impl DatabaseDocumentMountRegistry {
    pub fn new() -> Self {
        Self { slots: HashMap::new(), next_generation: 1 }
    }

    pub fn take_generation(&mut self) -> Result<u64, DbError> {
        let generation = self.next_generation;
        self.next_generation = generation.checked_add(1).filter(|next| *next != 0).ok_or(DbError::LimitExceeded("database document-mount generation"))?;
        Ok(generation)
    }

    pub fn ready_count(&self) -> usize {
        self.slots.values().filter(|slot| matches!(slot, DatabaseDocumentMountSlot::Ready(_))).count()
    }

    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DatabaseDocumentMountDriver {
    Idle,
    Queued,
    Polling,
    NonRunnable,
    Terminal,
}

pub struct DatabaseDocumentMountOwner {
    pub pool: Arc<WorkerPool>,
    pub registry: std::sync::Weak<Mutex<DatabaseDocumentMountRegistry>>,
    pub document: String,
    pub generation: u64,
    pub work: Mutex<DatabaseDocumentMountWork>,
    pub retry_job: Mutex<Option<semio_framework_async::Job>>,
    pub scheduler_fault: Mutex<Option<semio_framework_async::WorkerSubmitErrorKind>>,
    pub retry_armed: std::sync::atomic::AtomicBool,
    pub driver: std::sync::atomic::AtomicU8,
    pub wake_requested: std::sync::atomic::AtomicBool,
    pub resume_requested: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    cleanup_fault_hook: Arc<Mutex<Option<DatabaseDocumentMountCleanupFaultHook>>>,
    #[cfg(test)]
    parked_hook: Arc<Mutex<Option<DatabaseDocumentMountParkedHook>>>,
    pub terminal: std::sync::atomic::AtomicBool,
}

static DATABASE_DOCUMENT_MOUNT_WORK_LIVE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🔬️ How many document-mount owners are still alive holding their boxed mount work. Each one
/// carries the `WorkerPoolUse` clone `mount_document` handed its `run_document_mount` future, and a
/// retained-open retry carries a SECOND clone in its `resume` future — and neither is in any slot
/// table, so a mount owner that outlives its registry entry pins the database's pool use invisibly.
pub fn database_document_mount_work_live_slots() -> usize {
    DATABASE_DOCUMENT_MOUNT_WORK_LIVE.load(std::sync::atomic::Ordering::Acquire)
}

impl Drop for DatabaseDocumentMountOwner {
    fn drop(&mut self) {
        DATABASE_DOCUMENT_MOUNT_WORK_LIVE.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}

struct DatabaseDocumentMountWake {
    pub owner: std::sync::Weak<DatabaseDocumentMountOwner>,
    pub generation: u64,
}

impl std::task::Wake for DatabaseDocumentMountWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(owner) = self.owner.upgrade() {
            if owner.generation == self.generation {
                owner.request_drive(false);
            }
        }
    }
}

impl DatabaseDocumentMountOwner {
    pub fn new(
        pool: Arc<WorkerPool>,
        registry: std::sync::Weak<Mutex<DatabaseDocumentMountRegistry>>,
        document: String,
        generation: u64,
        future: DatabaseDocumentMountFuture,
        #[cfg(test)] cleanup_fault_hook: Arc<Mutex<Option<DatabaseDocumentMountCleanupFaultHook>>>,
        #[cfg(test)] parked_hook: Arc<Mutex<Option<DatabaseDocumentMountParkedHook>>>,
    ) -> Arc<Self> {
        DATABASE_DOCUMENT_MOUNT_WORK_LIVE.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Arc::new(Self {
            pool,
            registry,
            document,
            generation,
            work: Mutex::new(DatabaseDocumentMountWork::Mount(future)),
            retry_job: Mutex::new(None),
            scheduler_fault: Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            driver: std::sync::atomic::AtomicU8::new(DatabaseDocumentMountDriver::Idle as u8),
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            resume_requested: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            cleanup_fault_hook,
            #[cfg(test)]
            parked_hook,
            terminal: std::sync::atomic::AtomicBool::new(false),
        })
    }

    pub fn request_drive(self: &Arc<Self>, resume: bool) {
        use std::sync::atomic::Ordering;
        if self.terminal.load(Ordering::Acquire) || self.driver.load(Ordering::Acquire) == DatabaseDocumentMountDriver::NonRunnable as u8 {
            return;
        }
        if resume {
            self.resume_requested.store(true, Ordering::Release);
        }
        self.wake_requested.store(true, Ordering::Release);
        self.queue_if_idle();
    }

    pub fn drive(self: &Arc<Self>) {
        self.request_drive(true);
    }

    fn queue_if_idle(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.driver.compare_exchange(DatabaseDocumentMountDriver::Idle as u8, DatabaseDocumentMountDriver::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let owner = Arc::downgrade(self);
        let generation = self.generation;
        self.submit_exact(Box::new(move || {
            let Some(owner) = owner.upgrade() else { return };
            if owner.generation == generation {
                owner.poll_once();
            }
        }));
    }

    fn resume_parked(work: &mut DatabaseDocumentMountWork) {
        let current = std::mem::replace(work, DatabaseDocumentMountWork::Empty);
        *work = match current {
            DatabaseDocumentMountWork::Parked { rejected, resume } => DatabaseDocumentMountWork::Cleanup { future: Box::pin(rejected.retry_close()), resume },
            current => current,
        };
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job) {
        match self.pool.try_submit(Lane::UserVisible, job) {
            Ok(()) => {}
            Err(error) => match error.kind() {
                semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(error.into_job());
                    self.arm_retry();
                }
                kind => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(error.into_job());
                    *self.scheduler_fault.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(kind);
                    self.driver.store(DatabaseDocumentMountDriver::NonRunnable as u8, std::sync::atomic::Ordering::Release);
                }
            },
        }
    }

    pub fn scheduler_fault(&self) -> Option<semio_framework_async::WorkerSubmitErrorKind> {
        *self.scheduler_fault.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let owner = Arc::downgrade(self);
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || {
            let Some(owner) = owner.upgrade() else { return };
            owner.retry_armed.store(false, Ordering::Release);
            let job = owner.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some(job) = job {
                owner.submit_exact(job);
            }
        });
    }

    fn poll_once(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.driver.compare_exchange(DatabaseDocumentMountDriver::Queued as u8, DatabaseDocumentMountDriver::Polling as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        self.wake_requested.store(false, Ordering::Release);
        let waker = std::task::Waker::from(Arc::new(DatabaseDocumentMountWake { owner: Arc::downgrade(&self), generation: self.generation }));
        let mut context = std::task::Context::from_waker(&waker);
        loop {
            #[cfg(test)]
            let mut parked_hook = None;
            let outcome = {
                let mut work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if matches!(&*work, DatabaseDocumentMountWork::Parked { .. }) && self.resume_requested.swap(false, Ordering::AcqRel) {
                    Self::resume_parked(&mut work);
                }
                match std::mem::replace(&mut *work, DatabaseDocumentMountWork::Empty) {
                    DatabaseDocumentMountWork::Mount(mut future) => match future.as_mut().poll(&mut context) {
                        std::task::Poll::Pending => {
                            *work = DatabaseDocumentMountWork::Mount(future);
                            DatabaseDocumentMountPoll::Pending
                        }
                        std::task::Poll::Ready(Ok(reply)) => DatabaseDocumentMountPoll::Complete(Ok(reply)),
                        std::task::Poll::Ready(Err(DatabaseDocumentMountFailure::Terminal(error))) => DatabaseDocumentMountPoll::Complete(Err(error)),
                        std::task::Poll::Ready(Err(DatabaseDocumentMountFailure::Retained { rejected, resume })) => {
                            *work = DatabaseDocumentMountWork::Cleanup { future: Box::pin(rejected.retry_close()), resume };
                            DatabaseDocumentMountPoll::Continue
                        }
                    },
                    DatabaseDocumentMountWork::Cleanup { mut future, resume } => match future.as_mut().poll(&mut context) {
                        std::task::Poll::Pending => {
                            *work = DatabaseDocumentMountWork::Cleanup { future, resume };
                            DatabaseDocumentMountPoll::Pending
                        }
                        std::task::Poll::Ready(Ok(error)) => match resume {
                            Some(future) => {
                                *work = DatabaseDocumentMountWork::Mount(future);
                                DatabaseDocumentMountPoll::Continue
                            }
                            None => DatabaseDocumentMountPoll::Complete(Err(error)),
                        },
                        std::task::Poll::Ready(Err(rejected)) => {
                            #[cfg(test)]
                            if let Some(hook) = self.cleanup_fault_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                                hook();
                            }
                            *work = DatabaseDocumentMountWork::Parked { rejected, resume };
                            #[cfg(test)]
                            {
                                parked_hook = self.parked_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                            }
                            DatabaseDocumentMountPoll::Pending
                        }
                    },
                    parked @ DatabaseDocumentMountWork::Parked { .. } => {
                        *work = parked;
                        DatabaseDocumentMountPoll::Pending
                    }
                    DatabaseDocumentMountWork::Empty => DatabaseDocumentMountPoll::Pending,
                }
            };
            #[cfg(test)]
            if let Some(hook) = parked_hook {
                hook();
            }
            match outcome {
                DatabaseDocumentMountPoll::Pending => {
                    if self.wake_requested.swap(false, Ordering::AcqRel) {
                        continue;
                    }
                    if self.driver.compare_exchange(DatabaseDocumentMountDriver::Polling as u8, DatabaseDocumentMountDriver::Idle as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
                        return;
                    }
                    if self.wake_requested.swap(false, Ordering::AcqRel) {
                        self.queue_if_idle();
                    }
                    return;
                }
                DatabaseDocumentMountPoll::Continue => {}
                DatabaseDocumentMountPoll::Complete(result) => {
                    self.driver.store(DatabaseDocumentMountDriver::Terminal as u8, Ordering::Release);
                    self.complete(result);
                    return;
                }
            }
        }
    }

    pub fn complete(&self, result: Result<DatabaseDocumentMountReply, DbError>) {
        use std::sync::atomic::Ordering;
        let Some(registry) = self.registry.upgrade() else {
            return;
        };
        let mut fanout: [Option<(db_actor::ReplySender<Result<DatabaseDocumentMountReply, DbError>>, Result<DatabaseDocumentMountReply, DbError>)>; DATABASE_DOCUMENT_MOUNT_WAITERS] = std::array::from_fn(|_| None);
        {
            let mut registry = registry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(DatabaseDocumentMountSlot::Opening { generation, waiters, .. }) = registry.slots.get_mut(&self.document) else {
                return;
            };
            if *generation != self.generation {
                return;
            }
            let waiters = std::mem::replace(waiters, std::array::from_fn(|_| None));
            let authority = result.as_ref().ok().cloned();
            if let Some(completion) = authority.as_ref() {
                registry.slots.insert(self.document.clone(), DatabaseDocumentMountSlot::Ready(completion.clone()));
            } else {
                registry.slots.remove(&self.document);
            }
            let error = result.err();
            for (slot, waiter) in waiters.into_iter().enumerate() {
                let Some(waiter) = waiter else { continue };
                let outcome = match (&authority, &error) {
                    (Some(completion), _) if waiter.policy != DatabaseDocumentMountPolicy::Create || waiter.create_claim => Ok(completion.clone()),
                    (Some(_), _) => Err(DbError::AlreadyExists(format!("document {} already exists", self.document))),
                    (_, Some(error)) => Err(error.clone()),
                    _ => Err(DbError::Internal("database document-mount terminal result missing".to_string())),
                };
                fanout[slot] = Some((waiter.reply, outcome));
            }
            drop(authority);
            drop(error);
            self.terminal.store(true, Ordering::Release);
        }
        for (reply, outcome) in fanout.into_iter().flatten() {
            reply.send(outcome);
        }
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
