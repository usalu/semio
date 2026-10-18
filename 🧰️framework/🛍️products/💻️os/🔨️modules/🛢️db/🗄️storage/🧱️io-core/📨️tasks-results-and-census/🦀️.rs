//! 📨️ Tasks, results and census: the backend control, the wal segment state, the task and result unions, the lease result, the task phase and kind census and the task and result operations. See [`super`].

use super::*;

//#region 🔖️Limits
/// 🧭 Repository-owned backend identity; external driver values never cross this boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoBackendControl {
    Memory { slot: u16, generation: u64 },
    Filesystem { slot: u16, generation: u64 },
    Sqlite { slot: u16, generation: u64 },
    Postgres { slot: u16, generation: u64 },
    Neo4j { slot: u16, generation: u64 },
}

/// 🚦️ Persisted lifecycle state of one WAL segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalSegmentState {
    Active,
    Sealed,
}

/// 🗂️ Schema-first database I/O task owner.
pub enum DbIoTask {
    BackendOpen { backend: DbIoBackendControl, path: DbIoText },
    WalWriterAcquire { backend: DbIoBackendControl, document: DbIoText },
    WalCreate { backend: DbIoBackendControl, document: DbIoText, writer: WalWriterKey, index: u64 },
    WalAppend { backend: DbIoBackendControl, document: DbIoText, writer: WalWriterKey, index: u64, input: DbIoPages },
    WalSync { backend: DbIoBackendControl, document: DbIoText, writer: WalWriterKey, index: u64, class: DurabilityClass },
    WalSeal { backend: DbIoBackendControl, document: DbIoText, writer: WalWriterKey, index: u64 },
    WalRead { backend: DbIoBackendControl, document: DbIoText, index: u64, range: ByteRange, output: DbIoPageWriter },
    WalLength { backend: DbIoBackendControl, document: DbIoText, index: u64 },
    WalState { backend: DbIoBackendControl, document: DbIoText, index: u64 },
    WalList { backend: DbIoBackendControl, document: DbIoText, output: DbIoU64List },
    WalTruncate { backend: DbIoBackendControl, document: DbIoText, writer: WalWriterKey, index: u64, new_len: u64 },
    WalDelete { backend: DbIoBackendControl, document: DbIoText, writer: WalWriterKey, index: u64 },
    SnapshotWrite { backend: DbIoBackendControl, document: DbIoText, generation: u64, input: DbIoPages },
    SnapshotRead { backend: DbIoBackendControl, document: DbIoText, generation: u64, output: DbIoPageWriter },
    SnapshotLatest { backend: DbIoBackendControl, document: DbIoText, output: DbIoU64List },
    SnapshotList { backend: DbIoBackendControl, document: DbIoText, output: DbIoU64List },
    SnapshotDelete { backend: DbIoBackendControl, document: DbIoText, generation: u64 },
    PayloadPut { backend: DbIoBackendControl, input: DbIoPages },
    PayloadGet { backend: DbIoBackendControl, hash: ContentHash, output: DbIoPageWriter },
    PayloadExists { backend: DbIoBackendControl, hash: ContentHash },
    PayloadLength { backend: DbIoBackendControl, hash: ContentHash },
    PayloadDelete { backend: DbIoBackendControl, hash: ContentHash },
    CatalogRead { backend: DbIoBackendControl, output: DbIoPageWriter },
    CatalogCas { backend: DbIoBackendControl, expected: EpochFence, input: DbIoPages },
    IndexWrite { backend: DbIoBackendControl, document: DbIoText, run_id: u64, input: DbIoPages },
    IndexRead { backend: DbIoBackendControl, document: DbIoText, run_id: u64, output: DbIoPageWriter },
    IndexList { backend: DbIoBackendControl, document: DbIoText, output: DbIoU64List },
    IndexDelete { backend: DbIoBackendControl, document: DbIoText, run_id: u64 },
    LeaseAcquire { backend: DbIoBackendControl, document: DbIoText, holder: DbIoText, now_ms: u64, ttl_ms: u64 },
    LeaseRenew { backend: DbIoBackendControl, document: DbIoText, holder: DbIoText, fence: EpochFence, now_ms: u64, ttl_ms: u64 },
    LeaseRelease { backend: DbIoBackendControl, document: DbIoText, holder: DbIoText, fence: EpochFence },
    LeaseGet { backend: DbIoBackendControl, document: DbIoText, now_ms: u64 },
    BackendClose { backend: DbIoBackendControl },
}

/// 📬 Exact typed database I/O terminal result.
pub enum DbIoResult {
    Unit,
    WalWriter(WalWriterPermit),
    Length(u64),
    WalSegmentState(WalSegmentState),
    OptionalLength(Option<u64>),
    Exists(bool),
    Hash(ContentHash),
    Fence(EpochFence),
    Pages(DbIoPages),
    OptionalCatalog(Option<(DbIoPages, EpochFence)>),
    List(DbIoU64List),
    Lease(DbIoLeaseResult),
    OptionalLease(Option<DbIoLeaseResult>),
}

pub struct DbIoLeaseResult {
    pub resource: DbIoText,
    pub holder: DbIoText,
    pub fence: EpochFence,
    pub expires_at_ms: u64,
    pub result_handback: Option<DbIoResultHandback>,
}

impl DbIoLeaseResult {
    pub fn new(resource: DbIoText, holder: DbIoText, fence: EpochFence, expires_at_ms: u64) -> Self {
        Self { resource, holder, fence, expires_at_ms, result_handback: None }
    }

    pub fn handback_step(&mut self) -> bool {
        let Some(handback) = self.result_handback else { return false };
        if db_io_result_handback(handback).is_err() {
            return false;
        }
        self.result_handback = None;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.resource.terminal_is_empty() && self.holder.terminal_is_empty() && self.result_handback.is_none()
    }

    pub fn close_step(&mut self) -> bool {
        self.holder.close_step() || self.resource.close_step() || self.handback_step()
    }
}

impl std::fmt::Debug for DbIoLeaseResult {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DbIoLeaseResult").field("resource", &self.resource).field("holder", &self.holder).field("fence", &self.fence).field("expires_at_ms", &self.expires_at_ms).finish()
    }
}

impl PartialEq for DbIoLeaseResult {
    fn eq(&self, other: &Self) -> bool {
        self.resource == other.resource && self.holder == other.holder && self.fence == other.fence && self.expires_at_ms == other.expires_at_ms
    }
}

impl Eq for DbIoLeaseResult {}

pub type LeaseInfo = DbIoLeaseResult;

impl Drop for DbIoLeaseResult {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let owner =
            Self { resource: std::mem::replace(&mut self.resource, DbIoText::new()), holder: std::mem::replace(&mut self.holder, DbIoText::new()), fence: self.fence, expires_at_ms: self.expires_at_ms, result_handback: self.result_handback.take() };
        if let Err(DbIoLostOwner::Lease(owner)) = db_io_park_lost_owner(DbIoLostOwner::Lease(owner)) {
            *self = owner;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoTaskPhase {
    Admitted,
    Queued,
    Executing,
    Completed,
    Cancelled,
    Faulted,
    Closing,
}

//#region 🔖️DbIoCensus
/// 🏷️ One [`DbIoTask`] variant, as the process-wide [`db_io_census`] counts it: the cost model
/// of every storage call in typed tasks and in executor steps (one worker-pool turn each).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DbIoTaskKind {
    BackendOpen,
    WalWriterAcquire,
    WalCreate,
    WalAppend,
    WalSync,
    WalSeal,
    WalRead,
    WalLength,
    WalState,
    WalList,
    WalTruncate,
    WalDelete,
    SnapshotWrite,
    SnapshotRead,
    SnapshotLatest,
    SnapshotList,
    SnapshotDelete,
    PayloadPut,
    PayloadGet,
    PayloadExists,
    PayloadLength,
    PayloadDelete,
    CatalogRead,
    CatalogCas,
    IndexWrite,
    IndexRead,
    IndexList,
    IndexDelete,
    LeaseAcquire,
    LeaseRenew,
    LeaseRelease,
    LeaseGet,
    BackendClose,
}

impl DbIoTaskKind {
    pub const ALL: [DbIoTaskKind; 33] = [
        Self::BackendOpen,
        Self::WalWriterAcquire,
        Self::WalCreate,
        Self::WalAppend,
        Self::WalSync,
        Self::WalSeal,
        Self::WalRead,
        Self::WalLength,
        Self::WalState,
        Self::WalList,
        Self::WalTruncate,
        Self::WalDelete,
        Self::SnapshotWrite,
        Self::SnapshotRead,
        Self::SnapshotLatest,
        Self::SnapshotList,
        Self::SnapshotDelete,
        Self::PayloadPut,
        Self::PayloadGet,
        Self::PayloadExists,
        Self::PayloadLength,
        Self::PayloadDelete,
        Self::CatalogRead,
        Self::CatalogCas,
        Self::IndexWrite,
        Self::IndexRead,
        Self::IndexList,
        Self::IndexDelete,
        Self::LeaseAcquire,
        Self::LeaseRenew,
        Self::LeaseRelease,
        Self::LeaseGet,
        Self::BackendClose,
    ];
}
//#endregion 🔖️DbIoCensus

impl DbIoTask {
    pub fn kind(&self) -> DbIoTaskKind {
        match self {
            Self::BackendOpen { .. } => DbIoTaskKind::BackendOpen,
            Self::WalWriterAcquire { .. } => DbIoTaskKind::WalWriterAcquire,
            Self::WalCreate { .. } => DbIoTaskKind::WalCreate,
            Self::WalAppend { .. } => DbIoTaskKind::WalAppend,
            Self::WalSync { .. } => DbIoTaskKind::WalSync,
            Self::WalSeal { .. } => DbIoTaskKind::WalSeal,
            Self::WalRead { .. } => DbIoTaskKind::WalRead,
            Self::WalLength { .. } => DbIoTaskKind::WalLength,
            Self::WalState { .. } => DbIoTaskKind::WalState,
            Self::WalList { .. } => DbIoTaskKind::WalList,
            Self::WalTruncate { .. } => DbIoTaskKind::WalTruncate,
            Self::WalDelete { .. } => DbIoTaskKind::WalDelete,
            Self::SnapshotWrite { .. } => DbIoTaskKind::SnapshotWrite,
            Self::SnapshotRead { .. } => DbIoTaskKind::SnapshotRead,
            Self::SnapshotLatest { .. } => DbIoTaskKind::SnapshotLatest,
            Self::SnapshotList { .. } => DbIoTaskKind::SnapshotList,
            Self::SnapshotDelete { .. } => DbIoTaskKind::SnapshotDelete,
            Self::PayloadPut { .. } => DbIoTaskKind::PayloadPut,
            Self::PayloadGet { .. } => DbIoTaskKind::PayloadGet,
            Self::PayloadExists { .. } => DbIoTaskKind::PayloadExists,
            Self::PayloadLength { .. } => DbIoTaskKind::PayloadLength,
            Self::PayloadDelete { .. } => DbIoTaskKind::PayloadDelete,
            Self::CatalogRead { .. } => DbIoTaskKind::CatalogRead,
            Self::CatalogCas { .. } => DbIoTaskKind::CatalogCas,
            Self::IndexWrite { .. } => DbIoTaskKind::IndexWrite,
            Self::IndexRead { .. } => DbIoTaskKind::IndexRead,
            Self::IndexList { .. } => DbIoTaskKind::IndexList,
            Self::IndexDelete { .. } => DbIoTaskKind::IndexDelete,
            Self::LeaseAcquire { .. } => DbIoTaskKind::LeaseAcquire,
            Self::LeaseRenew { .. } => DbIoTaskKind::LeaseRenew,
            Self::LeaseRelease { .. } => DbIoTaskKind::LeaseRelease,
            Self::LeaseGet { .. } => DbIoTaskKind::LeaseGet,
            Self::BackendClose { .. } => DbIoTaskKind::BackendClose,
        }
    }


    pub fn writer_stamp(&self) -> Option<(WalWriterKey, DbIoBackendControl, &DbIoText)> {
        match self {
            Self::WalCreate { writer, backend, document, .. }
            | Self::WalAppend { writer, backend, document, .. }
            | Self::WalSync { writer, backend, document, .. }
            | Self::WalSeal { writer, backend, document, .. }
            | Self::WalTruncate { writer, backend, document, .. }
            | Self::WalDelete { writer, backend, document, .. } => Some((*writer, *backend, document)),
            _ => None,
        }
    }

    pub fn backend(&self) -> DbIoBackendControl {
        match self {
            Self::BackendOpen { backend, .. }
            | Self::WalWriterAcquire { backend, .. }
            | Self::WalCreate { backend, .. }
            | Self::WalAppend { backend, .. }
            | Self::WalSync { backend, .. }
            | Self::WalSeal { backend, .. }
            | Self::WalRead { backend, .. }
            | Self::WalLength { backend, .. }
            | Self::WalState { backend, .. }
            | Self::WalList { backend, .. }
            | Self::WalTruncate { backend, .. }
            | Self::WalDelete { backend, .. }
            | Self::SnapshotWrite { backend, .. }
            | Self::SnapshotRead { backend, .. }
            | Self::SnapshotLatest { backend, .. }
            | Self::SnapshotList { backend, .. }
            | Self::SnapshotDelete { backend, .. }
            | Self::PayloadPut { backend, .. }
            | Self::PayloadGet { backend, .. }
            | Self::PayloadExists { backend, .. }
            | Self::PayloadLength { backend, .. }
            | Self::PayloadDelete { backend, .. }
            | Self::CatalogRead { backend, .. }
            | Self::CatalogCas { backend, .. }
            | Self::IndexWrite { backend, .. }
            | Self::IndexRead { backend, .. }
            | Self::IndexList { backend, .. }
            | Self::IndexDelete { backend, .. }
            | Self::LeaseAcquire { backend, .. }
            | Self::LeaseRenew { backend, .. }
            | Self::LeaseRelease { backend, .. }
            | Self::LeaseGet { backend, .. }
            | Self::BackendClose { backend } => *backend,
        }
    }

    pub fn operation(&self) -> Option<u64> {
        match self {
            Self::WalAppend { input, .. } | Self::SnapshotWrite { input, .. } | Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } | Self::IndexWrite { input, .. } => Some(input.operation()),
            Self::WalRead { output, .. } | Self::SnapshotRead { output, .. } | Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } | Self::IndexRead { output, .. } => Some(output.operation()),
            _ => None,
        }
    }

    pub fn aggregate_credit(&self) -> DbIoCredit {
        let (bytes, items) = match self {
            Self::WalList { .. } | Self::SnapshotLatest { .. } | Self::SnapshotList { .. } | Self::IndexList { .. } => (DB_IO_TASK_SLOT_BYTES + DB_IO_LIST_TRANSIENT_BYTES, DB_IO_LIST_ITEMS * 2 + 1),
            _ => (DB_IO_TASK_SLOT_BYTES, 1),
        };
        DbIoCredit { pages: 0, bytes, items, controls: 1 }
    }

    pub fn admit_list_backing(&mut self) -> Result<(), DbError> {
        match self {
            Self::WalList { output, .. } | Self::SnapshotLatest { output, .. } | Self::SnapshotList { output, .. } | Self::IndexList { output, .. } if output.is_empty() => output.ensure_backing(),
            Self::WalList { .. } | Self::SnapshotLatest { .. } | Self::SnapshotList { .. } | Self::IndexList { .. } => Err(DbError::Internal("DB I/O list output was not empty at admission".to_string())),
            _ => Ok(()),
        }
    }

    pub fn release_unstarted_list_backing(&mut self) {
        match self {
            Self::WalList { output, .. } | Self::SnapshotLatest { output, .. } | Self::SnapshotList { output, .. } | Self::IndexList { output, .. } if output.is_empty() => output.values = None,
            _ => {}
        }
    }

    pub fn admit_pages(&self) -> Result<(), DbError> {
        match self {
            Self::WalAppend { input, .. } | Self::SnapshotWrite { input, .. } | Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } | Self::IndexWrite { input, .. } => input.admit(),
            Self::WalRead { output, .. } | Self::SnapshotRead { output, .. } | Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } | Self::IndexRead { output, .. } => {
                output.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued)
            }
            _ => Ok(()),
        }
    }

    pub fn transition_pages(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        match self {
            Self::WalAppend { input, .. } | Self::SnapshotWrite { input, .. } | Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } | Self::IndexWrite { input, .. } => input.transition(expected, next),
            Self::WalRead { output, .. } | Self::SnapshotRead { output, .. } | Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } | Self::IndexRead { output, .. } => output.transition(expected, next),
            _ => Ok(()),
        }
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        match self {
            Self::WalAppend { document, input, .. } | Self::SnapshotWrite { document, input, .. } | Self::IndexWrite { document, input, .. } => {
                if let Some(bytes) = input.close_step()? {
                    return Ok(Some(bytes));
                }
                if document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } => {
                if let Some(bytes) = input.close_step()? {
                    return Ok(Some(bytes));
                }
            }
            Self::WalRead { document, output, .. } | Self::SnapshotRead { document, output, .. } | Self::IndexRead { document, output, .. } => {
                if let Some(bytes) = output.close_step()? {
                    return Ok(Some(bytes));
                }
                if document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } => {
                if let Some(bytes) = output.close_step()? {
                    return Ok(Some(bytes));
                }
            }
            Self::BackendOpen { path, .. } => {
                if path.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::WalList { document, output, .. } | Self::SnapshotLatest { document, output, .. } | Self::SnapshotList { document, output, .. } | Self::IndexList { document, output, .. } => {
                if output.close_step() || document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::WalWriterAcquire { document, .. }
            | Self::WalCreate { document, .. }
            | Self::WalSync { document, .. }
            | Self::WalSeal { document, .. }
            | Self::WalLength { document, .. }
            | Self::WalState { document, .. }
            | Self::WalTruncate { document, .. }
            | Self::WalDelete { document, .. }
            | Self::SnapshotDelete { document, .. }
            | Self::IndexDelete { document, .. } => {
                if document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::LeaseAcquire { document, holder, .. } | Self::LeaseRenew { document, holder, .. } | Self::LeaseRelease { document, holder, .. } => {
                if holder.close_step() || document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::LeaseGet { document, .. } => {
                if document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::PayloadExists { .. } | Self::PayloadLength { .. } | Self::PayloadDelete { .. } | Self::BackendClose { .. } => {}
        }
        Ok(None)
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            Self::WalAppend { document, input, .. } | Self::SnapshotWrite { document, input, .. } | Self::IndexWrite { document, input, .. } => document.terminal_is_empty() && input.terminal_is_empty(),
            Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } => input.terminal_is_empty(),
            Self::WalRead { document, output, .. } | Self::SnapshotRead { document, output, .. } | Self::IndexRead { document, output, .. } => document.terminal_is_empty() && output.terminal_is_empty(),
            Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } => output.terminal_is_empty(),
            Self::BackendOpen { path, .. } => path.terminal_is_empty(),
            Self::WalList { document, output, .. } | Self::SnapshotLatest { document, output, .. } | Self::SnapshotList { document, output, .. } | Self::IndexList { document, output, .. } => document.terminal_is_empty() && output.terminal_is_empty(),
            Self::WalWriterAcquire { document, .. }
            | Self::WalCreate { document, .. }
            | Self::WalSync { document, .. }
            | Self::WalSeal { document, .. }
            | Self::WalLength { document, .. }
            | Self::WalState { document, .. }
            | Self::WalTruncate { document, .. }
            | Self::WalDelete { document, .. }
            | Self::SnapshotDelete { document, .. }
            | Self::IndexDelete { document, .. }
            | Self::LeaseGet { document, .. } => document.terminal_is_empty(),
            Self::LeaseAcquire { document, holder, .. } | Self::LeaseRenew { document, holder, .. } | Self::LeaseRelease { document, holder, .. } => document.terminal_is_empty() && holder.terminal_is_empty(),
            Self::PayloadExists { .. } | Self::PayloadLength { .. } | Self::PayloadDelete { .. } | Self::BackendClose { .. } => true,
        }
    }
}

impl DbIoResult {
    pub fn retained_credit(&self) -> DbIoCredit {
        match self {
            Self::List(list) if list.values.is_some() => DbIoCredit { pages: 0, bytes: DB_IO_LIST_BACKING_BYTES, items: DB_IO_LIST_ITEMS, controls: 0 },
            _ => DbIoCredit::default(),
        }
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        match self {
            Self::WalWriter(_) => {
                drop(std::mem::replace(self, Self::Unit));
                Ok(Some(0))
            }
            Self::Pages(pages) => pages.close_step(),
            Self::OptionalCatalog(Some((pages, _))) => pages.close_step(),
            Self::List(list) => Ok(list.close_step().then_some(0)),
            Self::Lease(lease) | Self::OptionalLease(Some(lease)) => {
                if lease.holder.close_step() || lease.resource.close_step() || lease.handback_step() {
                    return Ok(Some(0));
                }
                Ok(None)
            }
            Self::Unit | Self::Length(_) | Self::WalSegmentState(_) | Self::OptionalLength(_) | Self::Exists(_) | Self::Hash(_) | Self::Fence(_) | Self::OptionalCatalog(None) | Self::OptionalLease(None) => Ok(None),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            Self::WalWriter(_) => false,
            Self::Pages(pages) => pages.terminal_is_empty(),
            Self::OptionalCatalog(Some((pages, _))) => pages.terminal_is_empty(),
            Self::List(list) => list.terminal_is_empty(),
            Self::Lease(lease) | Self::OptionalLease(Some(lease)) => lease.terminal_is_empty(),
            Self::Unit | Self::Length(_) | Self::WalSegmentState(_) | Self::OptionalLength(_) | Self::Exists(_) | Self::Hash(_) | Self::Fence(_) | Self::OptionalCatalog(None) | Self::OptionalLease(None) => true,
        }
    }

    pub(super) fn attach_result_handback(&mut self, handback: DbIoResultHandback) -> bool {
        match self {
            Self::WalWriter(_) => false,
            Self::Pages(pages) => {
                pages.result_handback = Some(handback);
                true
            }
            Self::OptionalCatalog(Some((pages, _))) => {
                pages.result_handback = Some(handback);
                true
            }
            Self::List(list) => {
                list.result_handback = Some(handback);
                true
            }
            Self::Lease(lease) | Self::OptionalLease(Some(lease)) => {
                lease.result_handback = Some(handback);
                true
            }
            Self::Unit | Self::Length(_) | Self::WalSegmentState(_) | Self::OptionalLength(_) | Self::Exists(_) | Self::Hash(_) | Self::Fence(_) | Self::OptionalCatalog(None) | Self::OptionalLease(None) => false,
        }
    }
}
//#endregion 🔖️Limits
