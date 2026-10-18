//! 🧩️ Memory executor types: the in-memory wal segment with its credit, the owners, the executor, its cursor and default and the fixed none box. See [`super`].

use super::*;

//#region 🔖️Memory
/// 🧠️ One in-process, non-durable segment of a document's WAL — `bytes` plus whether
/// `seal` has been called on it. Its chunk table is heap backing of the live segment only: a
/// segment charges [`memory_wal_segment_credit`] to its backend when it is created and returns
/// it when its retirement empties the slot, so an idle memory backend does not pre-pay the chunk
/// tables of all 64 segment slots.
pub(super) struct MemWalSegment {
    pub chunks: Box<[Option<DbIoPages>]>,
    pub len: u64,
    pub(super) sealed: bool,
}

impl MemWalSegment {
    pub fn new() -> Self {
        Self { chunks: memory_fixed_none_box(DB_IO_OPERATION_ITEMS), len: 0, sealed: false }
    }
}

/// 🧾️ Exact heap credit of one live memory WAL segment's chunk table.
pub(super) const fn memory_wal_segment_credit() -> DbIoCredit {
    DbIoCredit { pages: 0, bytes: (DB_IO_OPERATION_ITEMS * size_of::<Option<DbIoPages>>()) as u64, items: 1, controls: 0 }
}

/// 🧠️ A pure in-memory `DbStorage`: every store is a `Mutex`-guarded map, nothing ever
/// touches a filesystem. Not durable (`capabilities().durable == false`) — the backend for unit
/// tests and `db_fault_testing`'s deterministic simulation runtime, never for a real deployment. Every
/// trait method body below is synchronous (no real I/O to await), so it is simply wrapped in an
/// already-`Ready` `{ .. }` per the module doc's "Async-first" section.
pub(super) const DB_IO_MEMORY_OWNERS: usize = DB_IO_OPERATION_ITEMS;

pub(super) struct MemoryWalOwner {
    pub document: DbIoText,
    pub index: u64,
    pub(super) segment: MemWalSegment,
}

pub(super) struct MemoryPageOwner {
    pub document: DbIoText,
    pub ordinal: u64,
    pub pages: DbIoPages,
}

pub(super) struct MemoryPayloadOwner {
    pub hash: ContentHash,
    pub pages: DbIoPages,
}

pub(super) struct MemoryLeaseOwner {
    pub resource: DbIoText,
    pub holder: DbIoText,
    pub fence: EpochFence,
    pub expires_at_ms: u64,
}

pub(super) struct MemoryDbIoExecutor {
    pub writers: std::sync::Mutex<Option<Box<db_io_driver::WalWriterTable<()>>>>,
    pub wal: std::sync::Mutex<Box<[Option<MemoryWalOwner>]>>,
    pub(super) snapshots: std::sync::Mutex<Box<[Option<MemoryPageOwner>]>>,
    pub(super) payloads: std::sync::Mutex<Box<[Option<MemoryPayloadOwner>]>>,
    pub catalog: std::sync::Mutex<Option<(DbIoPages, EpochFence)>>,
    pub(super) index_runs: std::sync::Mutex<Box<[Option<MemoryPageOwner>]>>,
    pub leases: std::sync::Mutex<Box<[Option<MemoryLeaseOwner>]>>,
    pub(super) operations: std::sync::Mutex<Box<[Option<MemoryDbIoCursor>]>>,
    pub(super) retired_pages: std::sync::Mutex<Box<[Option<DbIoPages>]>>,
    pub(super) retired_wal: std::sync::Mutex<Box<[Option<MemWalSegment>]>>,
    pub(super) close_phase: std::sync::atomic::AtomicU8,
    pub(super) backing_operation: std::sync::atomic::AtomicU64,
}

pub(super) enum MemoryDbIoCursor {
    WalRead { operation: u64, chunk: u8, page: u8, offset: usize, skip: usize, remaining: usize },
    PageCopy { operation: u64, page: u8 },
    List { operation: u64, after: Option<u64> },
    PayloadHash { operation: u64, page: u8, hasher: semio_framework_hash::Hasher },
    LeaseRelease { operation: u64, index: u16 },
}

impl MemoryDbIoCursor {
    pub fn operation(&self) -> u64 {
        match self {
            Self::WalRead { operation, .. } | Self::PageCopy { operation, .. } | Self::List { operation, .. } | Self::PayloadHash { operation, .. } | Self::LeaseRelease { operation, .. } => *operation,
        }
    }
}

impl Default for MemoryDbIoExecutor {
    fn default() -> Self {
        Self {
            writers: std::sync::Mutex::new(Some(Box::new(db_io_driver::WalWriterTable::unbound()))),
            wal: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_MEMORY_OWNERS)),
            snapshots: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_MEMORY_OWNERS)),
            payloads: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_MEMORY_OWNERS)),
            catalog: std::sync::Mutex::new(None),
            index_runs: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_MEMORY_OWNERS)),
            leases: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_MEMORY_OWNERS)),
            operations: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_OPERATION_ITEMS)),
            retired_pages: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_OPERATION_ITEMS)),
            retired_wal: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_OPERATION_ITEMS)),
            close_phase: std::sync::atomic::AtomicU8::new(0),
            backing_operation: std::sync::atomic::AtomicU64::new(0),
        }
    }
}

fn memory_fixed_none_box<T>(items: usize) -> Box<[Option<T>]> {
    std::iter::repeat_with(|| None).take(items).collect()
}
//#endregion 🔖️Memory
