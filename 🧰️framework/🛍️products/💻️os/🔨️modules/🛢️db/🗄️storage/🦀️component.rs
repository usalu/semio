//! 🗄️ `db_storage` — the pluggable storage substrate seam for the `db` crate family: the trait
//! family (`WalStorage`, `SnapshotStorage`, `PayloadStorage`, `CatalogStorage`,
//! `IndexStorage`, `LeaseStorage`) every backend (this crate's `MemoryStorage`/`FsStorage`, plus
//! the sibling `db_storage_sqlite`/`db_storage_postgres`/`db_storage_neo4j` modules) implements
//! identically, so `db_engine`/the `db` facade select a backend via [`DbBackend`] at
//! `Database::open` rather than at compile time. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! 🎯️ Design choice: this crate has no opinion on WAL record framing (`db_wal` reuses
//! `protocol::{SprWriter, FrameCursor, recover}` on top of `WalStorage`'s raw segment bytes) or
//! snapshot pack-file structure (`db_snapshot` builds `.spk` bytes handed to `SnapshotStorage`
//! whole) — every trait here stores/retrieves opaque byte blobs keyed by document + a small
//! integer, plus the two primitives (`CatalogStorage::cas_root`, `LeaseStorage`) that need
//! fencing semantics of their own. This keeps the trait family stable while the format built on
//! top of it (owned by `db_wal`/`db_snapshot`/`db_index`) can evolve independently.
//!
//! ⏳️ **Async-first, zero-dyn (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet
//! `db-dedyn`, ruling **O1**)**: every sub-trait method is a plain `async fn` returning
//! `Result<T, DbError>` directly — never a boxed `dyn Future` in trait-method return position
//! (ruling **R1**; that shape, `DbFuture<'a, T> = Pin<Box<dyn Future<..> + 'a>>`, was this family's
//! PREVIOUS state and is now deleted). Because `async fn` in a trait is not `dyn`-compatible, the
//! old `Arc<dyn DbStorage>`/`&dyn WalStorage` seams are gone too: [`DbBackend`] is a concrete enum
//! naming every backend (this crate has exactly one layer, so no layering problem — see
//! `//#region 🔖️DbBackend`), and its accessors return concrete facet-ref enums
//! ([`WalRef`]/[`SnapshotRef`]/[`PayloadRef`]/[`CatalogRef`]/[`IndexRef`]/[`LeaseRef`]) instead of
//! trait objects. `Send` on a spawned future is therefore obtained STRUCTURALLY, from the concrete
//! enum variant the compiler already knows at every call site — never from a `+ Send` bound on a
//! trait method (ruling **R3**). `#![allow(async_fn_in_trait)]` at the crate root suppresses the
//! resulting "auto trait bounds" lint (ruling **R7**): that lint's suggested fix
//! (`-> impl Future<..> + Send`) is exactly the bound R3 forbids.
//!
//! Backends split two ways: genuinely-async drivers (`db_storage_postgres`'s `sqlx`,
//! `db_storage_neo4j`'s `neo4rs`) simply `.await` their already-async bodies; genuinely-blocking
//! backends (this crate's own `FsStorage`, plus the sibling `db_storage_sqlite`) cross the
//! sync/async boundary through schema-first [`DbIoTask`] owners submitted to the ONE
//! process-wide `semio_framework_async::WorkerPool` on
//! `Lane::Io` — never a private `tokio::runtime` (this crate names no `tokio` at all; see the
//! repo's "`tokio` only in `🛎️services`" rule) and never `HostAsyncRuntime::run_blocking` (removed
//! from that trait — Phase 1 of `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`; callers now submit
//! directly to a `WorkerPool`). `MemoryStorage` (no real I/O) simply resolves immediately.
//! `FsStorage`/`SqliteStorage` construction requires that process pool; no pool-less or inline
//! blocking fallback exists, including for CLI and batch entry points.
//!
//! 🧊️ `FsStorage` (this crate's zero-touch default, behind the default `fs` feature) is native-only
//! (`std::fs`) and `#[cfg(not(target_arch = "wasm32"))]`-gated, mirroring `pack`'s own `pack_io`
//! convention — it compiles to an effectively-empty module on a `wasm32-unknown-unknown` target
//! check. `MemoryStorage` has no such gate and is always available.

use crate::db_durability::{DurabilityClass, EpochFence};
use crate::db_ids::{check_len, ArtifactId, DbError};
use crate::*;
use pack::{ByteRange, ContentHash};
use semio_framework_async::{Job, Lane, WorkerPool, WorkerSubmitErrorKind};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// @emoji 📊️ One process-wide typed DB I/O task depth signal.
static BLOCKING_QUEUE: semio_framework_trace::QueueCounter = semio_framework_trace::QueueCounter::new();

//#region 🔖️Limits
/// @emoji 🛡️ Ceiling on any single blob this crate reads into memory in one call (one WAL read
/// range, one snapshot generation, one payload, one index run, one lease record) — validated via
/// `check_len` BEFORE the read buffer is allocated, mirroring `pack_core`'s stated
/// invariant. This crate's own choice (the contract doesn't fix a number): generous enough for a
/// snapshot generation or a large payload, small enough to refuse an obviously-corrupt on-disk
/// length before trying to allocate it.
const MAX_READ_BYTES: u64 = 496 * 1024;

pub const DB_IO_PAGE_BYTES: usize = 16 * 1024;
const DB_IO_OPERATION_PAGES: usize = 64;
const DB_IO_TOTAL_PAGES: usize = 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DbIoPagePhase {
    Free,
    CheckedOutInput,
    CheckedOutWriter,
    Queued,
    Executing,
    TerminalResult,
    Rejected,
    Closing,
}

struct DbIoPageBacking(std::cell::UnsafeCell<[u8; DB_IO_PAGE_BYTES]>);

impl DbIoPageBacking {
    const fn new() -> Self {
        Self(std::cell::UnsafeCell::new([0; DB_IO_PAGE_BYTES]))
    }
}

unsafe impl Sync for DbIoPageBacking {}

static DB_IO_PAGE_BACKINGS: [DbIoPageBacking; DB_IO_TOTAL_PAGES] = [const { DbIoPageBacking::new() }; DB_IO_TOTAL_PAGES];

#[derive(Clone, Copy)]
struct DbIoPageSlot {
    generation: u64,
    operation: u64,
    phase: DbIoPagePhase,
}

const EMPTY_DB_IO_PAGE_SLOT: DbIoPageSlot = DbIoPageSlot { generation: 0, operation: 0, phase: DbIoPagePhase::Free };

struct DbIoPageArenaState {
    slots: [DbIoPageSlot; DB_IO_TOTAL_PAGES],
    free: [u16; DB_IO_TOTAL_PAGES],
    free_read: usize,
    free_len: usize,
    retired: [Option<(u16, u64)>; DB_IO_TOTAL_PAGES],
    retired_read: usize,
    retired_len: usize,
    next_generation: u64,
}

impl DbIoPageArenaState {
    fn new() -> Self {
        Self {
            slots: [EMPTY_DB_IO_PAGE_SLOT; DB_IO_TOTAL_PAGES],
            free: std::array::from_fn(|index| index as u16),
            free_read: 0,
            free_len: DB_IO_TOTAL_PAGES,
            retired: [None; DB_IO_TOTAL_PAGES],
            retired_read: 0,
            retired_len: 0,
            next_generation: 1,
        }
    }
}

static DB_IO_PAGE_ARENA: std::sync::OnceLock<std::sync::Mutex<DbIoPageArenaState>> = std::sync::OnceLock::new();
static DB_IO_NEXT_OPERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

fn db_io_page_arena() -> &'static std::sync::Mutex<DbIoPageArenaState> {
    DB_IO_PAGE_ARENA.get_or_init(|| std::sync::Mutex::new(DbIoPageArenaState::new()))
}

fn db_io_next_operation() -> Result<u64, DbError> {
    DB_IO_NEXT_OPERATION
        .fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |current| current.checked_add(1).filter(|next| *next != 0))
        .map_err(|_| DbError::LimitExceeded("db_io operation generation"))
}

#[derive(Debug)]
struct DbIoPageLease {
    slot: u16,
    generation: u64,
    operation: u64,
    used: u16,
    returned: bool,
}

impl DbIoPageLease {
    fn transition(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut state.slots[self.slot as usize];
        if slot.generation != self.generation || slot.operation != self.operation || slot.phase != expected {
            return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.generation), actual: crate::db_ids::GenerationId(slot.generation) });
        }
        slot.phase = next;
        Ok(())
    }

    fn bytes(&self) -> &[u8] {
        let used = usize::from(self.used);
        unsafe { &(*DB_IO_PAGE_BACKINGS[self.slot as usize].0.get())[..used] }
    }

    fn write(&mut self, offset: usize, source: &[u8]) -> Result<(), DbError> {
        let end = offset.checked_add(source.len()).ok_or(DbError::LimitExceeded("db_io page write"))?;
        if end > DB_IO_PAGE_BYTES || offset > usize::from(self.used) {
            return Err(DbError::LimitExceeded("db_io page writer reservation"));
        }
        let state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = state.slots[self.slot as usize];
        if slot.generation != self.generation || slot.operation != self.operation || !matches!(slot.phase, DbIoPagePhase::CheckedOutWriter | DbIoPagePhase::Executing) {
            return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.generation), actual: crate::db_ids::GenerationId(slot.generation) });
        }
        unsafe { (*DB_IO_PAGE_BACKINGS[self.slot as usize].0.get())[offset..end].copy_from_slice(source) };
        self.used = end as u16;
        Ok(())
    }

    fn return_to_arena(mut self) -> Result<usize, DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut state.slots[self.slot as usize];
        if slot.generation != self.generation || slot.operation != self.operation || slot.phase != DbIoPagePhase::Closing {
            return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.generation), actual: crate::db_ids::GenerationId(slot.generation) });
        }
        *slot = EMPTY_DB_IO_PAGE_SLOT;
        let write = (state.free_read + state.free_len) % DB_IO_TOTAL_PAGES;
        state.free[write] = self.slot;
        state.free_len += 1;
        self.returned = true;
        Ok(DB_IO_PAGE_BYTES)
    }

    fn install_lost_handle(&mut self) {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut state.slots[self.slot as usize];
        if slot.generation != self.generation || slot.operation != self.operation || slot.phase == DbIoPagePhase::Free {
            return;
        }
        slot.phase = DbIoPagePhase::Closing;
        assert!(state.retired_len < DB_IO_TOTAL_PAGES, "DB I/O fixed page retirement arena saturated");
        let write = (state.retired_read + state.retired_len) % DB_IO_TOTAL_PAGES;
        state.retired[write] = Some((self.slot, self.generation));
        state.retired_len += 1;
        self.returned = true;
    }
}

impl Drop for DbIoPageLease {
    fn drop(&mut self) {
        if !self.returned {
            self.install_lost_handle();
        }
    }
}

fn db_io_checkout_pages(operation: u64, count: usize, phase: DbIoPagePhase) -> Result<[Option<DbIoPageLease>; DB_IO_OPERATION_PAGES], DbError> {
    if operation == 0 || count > DB_IO_OPERATION_PAGES {
        return Err(DbError::LimitExceeded("db_io operation page credit"));
    }
    let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    db_io_preflight_page_checkout(&state, count)?;
    let mut leases: [Option<DbIoPageLease>; DB_IO_OPERATION_PAGES] = std::array::from_fn(|_| None);
    for lease in leases.iter_mut().take(count) {
        let slot = state.free[state.free_read];
        state.free_read = (state.free_read + 1) % DB_IO_TOTAL_PAGES;
        state.free_len -= 1;
        let generation = state.next_generation;
        state.next_generation += 1;
        state.slots[slot as usize] = DbIoPageSlot { generation, operation, phase };
        *lease = Some(DbIoPageLease { slot, generation, operation, used: 0, returned: false });
    }
    Ok(leases)
}

fn db_io_preflight_page_checkout(state: &DbIoPageArenaState, count: usize) -> Result<(), DbError> {
    if count > state.free_len || state.next_generation.checked_add(count as u64).is_none() {
        return Err(DbError::Unavailable("db I/O process page capacity exhausted".to_string()));
    }
    Ok(())
}

/// @emoji ✍️ Retained writer backed only by exact fixed arena page leases.
pub struct DbIoPageWriter {
    operation: u64,
    pages: [Option<DbIoPageLease>; DB_IO_OPERATION_PAGES],
    reserved: u8,
    cursor: u8,
    total_len: usize,
}

#[derive(Debug)]
pub struct DbIoPageWriterRejected {
    error: DbError,
    writer: Option<DbIoPageWriter>,
}

impl DbIoPageWriter {
    pub fn try_reserve(reserved_pages: usize) -> Result<Self, DbIoPageWriterRejected> {
        let operation = db_io_next_operation().map_err(|error| DbIoPageWriterRejected { error, writer: None })?;
        let pages = db_io_checkout_pages(operation, reserved_pages, DbIoPagePhase::CheckedOutWriter).map_err(|error| DbIoPageWriterRejected { error, writer: None })?;
        Ok(Self { operation, pages, reserved: reserved_pages as u8, cursor: 0, total_len: 0 })
    }

    pub fn operation(&self) -> u64 {
        self.operation
    }

    pub fn write_fragment(&mut self, source: &[u8]) -> Result<usize, DbError> {
        let Some(page) = self.pages.get_mut(self.cursor as usize).and_then(Option::as_mut) else {
            return Err(DbError::LimitExceeded("db_io writer page reservation"));
        };
        let available = DB_IO_PAGE_BYTES - usize::from(page.used);
        let written = available.min(source.len());
        page.write(usize::from(page.used), &source[..written])?;
        self.total_len = self.total_len.checked_add(written).ok_or(DbError::LimitExceeded("db_io writer total"))?;
        if usize::from(page.used) == DB_IO_PAGE_BYTES {
            self.cursor += 1;
        }
        Ok(written)
    }

    pub fn seal(mut self) -> Result<DbIoPages, DbIoPageWriterRejected> {
        match self.finish() {
            Ok(pages) => Ok(pages),
            Err(error) => Err(DbIoPageWriterRejected { error, writer: Some(self) }),
        }
    }

    pub fn finish(&mut self) -> Result<DbIoPages, DbError> {
        let visible = if self.total_len == 0 { 0 } else { self.total_len.div_ceil(DB_IO_PAGE_BYTES) };
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let current = self.pages.iter().take(self.reserved as usize).flatten().next().map_or(DbIoPagePhase::CheckedOutWriter, |page| state.slots[page.slot as usize].phase);
        if !matches!(current, DbIoPagePhase::CheckedOutWriter | DbIoPagePhase::Executing) {
            return Err(DbError::Internal("DB I/O writer finished outside an owned phase".to_string()));
        }
        for page in self.pages.iter().take(self.reserved as usize).flatten() {
            let slot = state.slots[page.slot as usize];
            if slot.generation != page.generation || slot.operation != page.operation || slot.phase != current {
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(page.generation), actual: crate::db_ids::GenerationId(slot.generation) });
            }
        }
        let next = if current == DbIoPagePhase::Executing { DbIoPagePhase::TerminalResult } else { DbIoPagePhase::CheckedOutInput };
        for page in self.pages.iter().take(self.reserved as usize).flatten() {
            state.slots[page.slot as usize].phase = next;
        }
        drop(state);
        let pages = std::mem::replace(&mut self.pages, std::array::from_fn(|_| None));
        let owner = DbIoPages { operation: self.operation, pages, retained: self.reserved, visible: visible as u8, first_offset: 0, total_len: self.total_len };
        self.reserved = 0;
        self.cursor = 0;
        self.total_len = 0;
        Ok(owner)
    }

    pub fn len(&self) -> usize {
        self.total_len
    }

    pub fn is_empty(&self) -> bool {
        self.total_len == 0
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        let Some(page) = self.pages.iter_mut().rev().find_map(Option::take) else { return Ok(None) };
        let state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let phase = state.slots[page.slot as usize].phase;
        drop(state);
        page.transition(phase, DbIoPagePhase::Closing)?;
        page.return_to_arena().map(Some)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.iter().all(Option::is_none)
    }

    fn transition(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for page in self.pages.iter().take(self.reserved as usize).flatten() {
            let slot = state.slots[page.slot as usize];
            if slot.generation != page.generation || slot.operation != page.operation || slot.phase != expected {
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(page.generation), actual: crate::db_ids::GenerationId(slot.generation) });
            }
        }
        for page in self.pages.iter().take(self.reserved as usize).flatten() {
            state.slots[page.slot as usize].phase = next;
        }
        Ok(())
    }
}

impl std::fmt::Debug for DbIoPageWriter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DbIoPageWriter").field("operation", &self.operation).field("reserved", &self.reserved).field("cursor", &self.cursor).field("total_len", &self.total_len).finish()
    }
}

impl DbIoPageWriterRejected {
    pub fn error(&self) -> &DbError {
        &self.error
    }

    pub fn into_writer(self) -> Option<DbIoPageWriter> {
        self.writer
    }

    pub fn into_error(self) -> DbError {
        self.error
    }
}

/// @emoji 📥 One-fragment-per-poll copy into already checked-out fixed pages.
pub struct DbIoPageCopy<'a> {
    source: &'a [u8],
    cursor: usize,
    writer: Option<DbIoPageWriter>,
}

pub struct DbIoPageOwnerCopy<'a> {
    source: &'a DbIoPages,
    cursor: u8,
    writer: Option<DbIoPageWriter>,
}

pub fn db_io_copy_pages(source: &[u8]) -> Result<DbIoPageCopy<'_>, DbError> {
    let pages = source.len().div_ceil(DB_IO_PAGE_BYTES);
    let writer = DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)?;
    Ok(DbIoPageCopy { source, cursor: 0, writer: Some(writer) })
}

pub fn db_io_copy_page_owner(source: &DbIoPages) -> Result<DbIoPageOwnerCopy<'_>, DbError> {
    let writer = DbIoPageWriter::try_reserve(source.page_count() as usize).map_err(DbIoPageWriterRejected::into_error)?;
    Ok(DbIoPageOwnerCopy { source, cursor: 0, writer: Some(writer) })
}

impl Future for DbIoPageCopy<'_> {
    type Output = Result<DbIoPages, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        if owner.cursor == owner.source.len() {
            let writer = owner.writer.take().expect("DB I/O page copy writer consumed once");
            return std::task::Poll::Ready(writer.seal().map_err(DbIoPageWriterRejected::into_error));
        }
        let start = owner.cursor;
        let end = start.saturating_add(DB_IO_PAGE_BYTES).min(owner.source.len());
        let source = &owner.source[start..end];
        let written = owner.writer.as_mut().expect("DB I/O page copy writer retained").write_fragment(source)?;
        owner.cursor += written;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Future for DbIoPageOwnerCopy<'_> {
    type Output = Result<DbIoPages, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(source) = owner.source.page(owner.cursor) else {
            let writer = owner.writer.take().expect("DB I/O page-owner copy writer consumed once");
            return std::task::Poll::Ready(writer.seal().map_err(DbIoPageWriterRejected::into_error));
        };
        let written = owner.writer.as_mut().expect("DB I/O page-owner copy writer retained").write_fragment(source)?;
        if written != source.len() {
            return std::task::Poll::Ready(Err(DbError::Internal("DB I/O source fragment exceeded one fixed page".to_string())));
        }
        owner.cursor += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

const DB_IO_PLATFORM_BUFFERS: usize = 16;
const DB_IO_PLATFORM_BUFFER_BYTES: usize = MAX_READ_BYTES as usize;

struct DbIoPlatformBacking(std::cell::UnsafeCell<[u8; DB_IO_PLATFORM_BUFFER_BYTES]>);

impl DbIoPlatformBacking {
    const fn new() -> Self {
        Self(std::cell::UnsafeCell::new([0; DB_IO_PLATFORM_BUFFER_BYTES]))
    }
}

unsafe impl Sync for DbIoPlatformBacking {}

static DB_IO_PLATFORM_BACKINGS: [DbIoPlatformBacking; DB_IO_PLATFORM_BUFFERS] = [const { DbIoPlatformBacking::new() }; DB_IO_PLATFORM_BUFFERS];

#[derive(Clone, Copy)]
struct DbIoPlatformSlot {
    generation: u64,
    occupied: bool,
}

struct DbIoPlatformArena {
    slots: [DbIoPlatformSlot; DB_IO_PLATFORM_BUFFERS],
    retired: [Option<(u8, u64)>; DB_IO_PLATFORM_BUFFERS],
    retired_read: usize,
    retired_len: usize,
    next_generation: u64,
}

static DB_IO_PLATFORM_ARENA: std::sync::Mutex<DbIoPlatformArena> = std::sync::Mutex::new(DbIoPlatformArena {
    slots: [DbIoPlatformSlot { generation: 0, occupied: false }; DB_IO_PLATFORM_BUFFERS],
    retired: [None; DB_IO_PLATFORM_BUFFERS],
    retired_read: 0,
    retired_len: 0,
    next_generation: 1,
});

/// @emoji 🧩 Explicit contiguous platform-call buffer backed by a fixed process slot.
pub struct DbIoPlatformBuffer {
    slot: u8,
    generation: u64,
    len: usize,
    copied: usize,
    credit: DbIoPageWriter,
    returned: bool,
}

pub struct DbIoPlatformCopy<'a> {
    source: &'a DbIoPages,
    cursor: u8,
    owner: Option<DbIoPlatformBuffer>,
}

pub fn db_io_prepare_platform(source: &DbIoPages) -> Result<DbIoPlatformCopy<'_>, DbError> {
    let _ = db_io_platform_maintenance_step()?;
    if source.len() > DB_IO_PLATFORM_BUFFER_BYTES {
        return Err(DbError::LimitExceeded("db_io prepared platform buffer"));
    }
    let (slot, generation) = {
        let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = arena.slots.iter().position(|slot| !slot.occupied).ok_or_else(|| DbError::Unavailable("DB I/O prepared platform capacity exhausted".to_string()))?;
        let generation = arena.next_generation;
        arena.next_generation = arena.next_generation.checked_add(1).filter(|next| *next != 0).ok_or(DbError::LimitExceeded("db_io platform generation"))?;
        arena.slots[slot] = DbIoPlatformSlot { generation, occupied: true };
        (slot as u8, generation)
    };
    let credit = match DbIoPageWriter::try_reserve(source.page_count() as usize) {
        Ok(credit) => credit,
        Err(error) => {
            let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            arena.slots[slot as usize] = DbIoPlatformSlot { generation: 0, occupied: false };
            return Err(error.into_error());
        }
    };
    let owner = DbIoPlatformBuffer { slot, generation, len: source.len(), copied: 0, credit, returned: false };
    Ok(DbIoPlatformCopy { source, cursor: 0, owner: Some(owner) })
}

impl DbIoPlatformBuffer {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { &(*DB_IO_PLATFORM_BACKINGS[self.slot as usize].0.get())[..self.len] }
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.credit.close_step()?.is_some() {
            return Ok(true);
        }
        if self.returned {
            return Ok(false);
        }
        let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut arena.slots[self.slot as usize];
        if !slot.occupied || slot.generation != self.generation {
            return Err(DbError::Internal("DB I/O prepared platform slot lost ABA authority".to_string()));
        }
        *slot = DbIoPlatformSlot { generation: 0, occupied: false };
        self.returned = true;
        Ok(true)
    }
}

impl Drop for DbIoPlatformBuffer {
    fn drop(&mut self) {
        if !self.returned {
            let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let slot = arena.slots[self.slot as usize];
            if slot.occupied && slot.generation == self.generation {
                assert!(arena.retired_len < DB_IO_PLATFORM_BUFFERS, "DB I/O fixed platform retirement arena saturated");
                let write = (arena.retired_read + arena.retired_len) % DB_IO_PLATFORM_BUFFERS;
                arena.retired[write] = Some((self.slot, self.generation));
                arena.retired_len += 1;
                self.returned = true;
            }
        }
    }
}

pub fn db_io_platform_maintenance_step() -> Result<bool, DbError> {
    let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if arena.retired_len == 0 {
        return Ok(false);
    }
    let read = arena.retired_read;
    let (slot, generation) = arena.retired[read].take().expect("DB I/O platform retired length names an exact owner");
    let owner = arena.slots[slot as usize];
    if !owner.occupied || owner.generation != generation {
        return Err(DbError::Internal("DB I/O platform retirement lost ABA authority".to_string()));
    }
    arena.slots[slot as usize] = DbIoPlatformSlot { generation: 0, occupied: false };
    arena.retired_read = (read + 1) % DB_IO_PLATFORM_BUFFERS;
    arena.retired_len -= 1;
    Ok(true)
}

impl Future for DbIoPlatformCopy<'_> {
    type Output = Result<DbIoPlatformBuffer, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(fragment) = owner.source.page(owner.cursor) else {
            return std::task::Poll::Ready(Ok(owner.owner.take().expect("DB I/O prepared platform buffer consumed once")));
        };
        let platform = owner.owner.as_mut().expect("DB I/O prepared platform buffer retained");
        let end = platform.copied.checked_add(fragment.len()).ok_or(DbError::LimitExceeded("db_io prepared platform cursor"))?;
        unsafe { (*DB_IO_PLATFORM_BACKINGS[platform.slot as usize].0.get())[platform.copied..end].copy_from_slice(fragment) };
        platform.copied = end;
        owner.cursor += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

/// @emoji 📄 Exact ordered fixed-page leases with a zero-copy movable range cursor.
#[derive(Debug)]
pub struct DbIoPages {
    operation: u64,
    pages: [Option<DbIoPageLease>; DB_IO_OPERATION_PAGES],
    retained: u8,
    visible: u8,
    first_offset: usize,
    total_len: usize,
}

impl DbIoPages {
    pub fn operation(&self) -> u64 {
        self.operation
    }

    pub fn len(&self) -> usize {
        self.total_len
    }

    pub fn is_empty(&self) -> bool {
        self.total_len == 0
    }

    pub fn page_count(&self) -> u8 {
        self.visible
    }

    pub fn page(&self, index: u8) -> Option<&[u8]> {
        if index >= self.visible {
            return None;
        }
        let physical = usize::from(index) + self.first_offset / DB_IO_PAGE_BYTES;
        let page = self.pages.get(physical)?.as_ref()?.bytes();
        let start = if index == 0 { self.first_offset % DB_IO_PAGE_BYTES } else { 0 };
        let preceding = usize::from(index).checked_mul(DB_IO_PAGE_BYTES)?.saturating_sub(if index == 0 { 0 } else { self.first_offset % DB_IO_PAGE_BYTES });
        let remaining = self.total_len.saturating_sub(preceding);
        page.get(start..start.saturating_add(remaining.min(page.len().saturating_sub(start))))
    }

    pub fn fragments(&self) -> DbIoPageReader<'_> {
        DbIoPageReader { pages: self, cursor: 0 }
    }

    pub fn try_range(mut self, start: usize) -> Result<Self, Self> {
        if start > self.total_len {
            return Err(self);
        }
        self.first_offset += start;
        self.total_len -= start;
        self.visible = if self.total_len == 0 { 0 } else { (self.first_offset % DB_IO_PAGE_BYTES + self.total_len).div_ceil(DB_IO_PAGE_BYTES) as u8 };
        Ok(self)
    }

    pub fn try_prefix(mut self, len: usize) -> Result<Self, Self> {
        if len > self.total_len {
            return Err(self);
        }
        self.total_len = len;
        self.visible = if len == 0 { 0 } else { (self.first_offset % DB_IO_PAGE_BYTES + len).div_ceil(DB_IO_PAGE_BYTES) as u8 };
        Ok(self)
    }

    pub fn advance(&mut self, len: usize) -> Result<(), DbError> {
        if len > self.total_len {
            return Err(DbError::InvalidArgument("DB I/O page cursor advanced past its retained range".to_string()));
        }
        self.first_offset += len;
        self.total_len -= len;
        self.visible = if self.total_len == 0 { 0 } else { (self.first_offset % DB_IO_PAGE_BYTES + self.total_len).div_ceil(DB_IO_PAGE_BYTES) as u8 };
        Ok(())
    }

    fn transition(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for page in self.pages.iter().take(self.retained as usize).flatten() {
            let slot = state.slots[page.slot as usize];
            if slot.generation != page.generation || slot.operation != page.operation || slot.phase != expected {
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(page.generation), actual: crate::db_ids::GenerationId(slot.generation) });
            }
        }
        for page in self.pages.iter().take(self.retained as usize).flatten() {
            state.slots[page.slot as usize].phase = next;
        }
        Ok(())
    }

    fn admit(&self) -> Result<(), DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for page in self.pages.iter().take(self.retained as usize).flatten() {
            let slot = state.slots[page.slot as usize];
            if slot.generation != page.generation || slot.operation != page.operation || !matches!(slot.phase, DbIoPagePhase::CheckedOutInput | DbIoPagePhase::TerminalResult) {
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(page.generation), actual: crate::db_ids::GenerationId(slot.generation) });
            }
        }
        for page in self.pages.iter().take(self.retained as usize).flatten() {
            state.slots[page.slot as usize].phase = DbIoPagePhase::Queued;
        }
        Ok(())
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        let Some(page) = self.pages.iter_mut().rev().find_map(Option::take) else { return Ok(None) };
        let state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let phase = state.slots[page.slot as usize].phase;
        drop(state);
        page.transition(phase, DbIoPagePhase::Closing)?;
        page.return_to_arena().map(Some)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.iter().all(Option::is_none)
    }
}

impl PartialEq<[u8]> for DbIoPages {
    fn eq(&self, expected: &[u8]) -> bool {
        if self.len() != expected.len() {
            return false;
        }
        let mut offset = 0;
        for fragment in self.fragments() {
            let end = offset + fragment.len();
            if fragment != &expected[offset..end] {
                return false;
            }
            offset = end;
        }
        true
    }
}

impl PartialEq for DbIoPages {
    fn eq(&self, expected: &Self) -> bool {
        if self.len() != expected.len() {
            return false;
        }
        let mut left = self.fragments();
        let mut right = expected.fragments();
        let (mut left_fragment, mut right_fragment) = (left.next().unwrap_or_default(), right.next().unwrap_or_default());
        let (mut left_offset, mut right_offset) = (0, 0);
        while left_offset < left_fragment.len() || right_offset < right_fragment.len() || self.len() == 0 {
            if self.len() == 0 {
                return true;
            }
            let compared = (left_fragment.len() - left_offset).min(right_fragment.len() - right_offset);
            if left_fragment[left_offset..left_offset + compared] != right_fragment[right_offset..right_offset + compared] {
                return false;
            }
            left_offset += compared;
            right_offset += compared;
            if left_offset == left_fragment.len() {
                let Some(next) = left.next() else { return right_offset == right_fragment.len() && right.next().is_none() };
                left_fragment = next;
                left_offset = 0;
            }
            if right_offset == right_fragment.len() {
                let Some(next) = right.next() else { return left_offset == left_fragment.len() && left.next().is_none() };
                right_fragment = next;
                right_offset = 0;
            }
        }
        true
    }
}

impl Eq for DbIoPages {}

impl PartialEq<&[u8]> for DbIoPages {
    fn eq(&self, expected: &&[u8]) -> bool {
        self == *expected
    }
}

impl<const N: usize> PartialEq<[u8; N]> for DbIoPages {
    fn eq(&self, expected: &[u8; N]) -> bool {
        self == expected.as_slice()
    }
}

impl PartialEq<Vec<u8>> for DbIoPages {
    fn eq(&self, expected: &Vec<u8>) -> bool {
        self == expected.as_slice()
    }
}

pub struct DbIoPageReader<'a> {
    pages: &'a DbIoPages,
    cursor: u8,
}

impl<'a> Iterator for DbIoPageReader<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let fragment = self.pages.page(self.cursor)?;
        self.cursor += 1;
        Some(fragment)
    }
}

pub fn db_io_page_maintenance_step() -> Result<Option<usize>, DbError> {
    let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some((slot, generation)) = state.retired[state.retired_read].take() else { return Ok(None) };
    state.retired_read = (state.retired_read + 1) % DB_IO_TOTAL_PAGES;
    state.retired_len -= 1;
    let owner = state.slots[slot as usize];
    if owner.generation != generation || owner.phase != DbIoPagePhase::Closing {
        return Err(DbError::Internal("db I/O page retirement lost ABA authority".to_string()));
    }
    state.slots[slot as usize] = EMPTY_DB_IO_PAGE_SLOT;
    let write = (state.free_read + state.free_len) % DB_IO_TOTAL_PAGES;
    state.free[write] = slot;
    state.free_len += 1;
    Ok(Some(DB_IO_PAGE_BYTES))
}

const DB_IO_TEXT_BYTES: usize = 1024;
const DB_IO_LIST_ITEMS: usize = 4096;

/// @emoji 🔤 Fixed repository-owned path, document, key or fault text.
#[derive(Clone, PartialEq, Eq)]
pub struct DbIoText {
    bytes: [u8; DB_IO_TEXT_BYTES],
    len: u16,
}

impl DbIoText {
    fn new() -> Self {
        Self { bytes: [0; DB_IO_TEXT_BYTES], len: 0 }
    }

    pub fn try_from_str(value: &str) -> Result<Self, DbError> {
        if value.len() > DB_IO_TEXT_BYTES {
            return Err(DbError::LimitExceeded("db_io text authority"));
        }
        let mut bytes = [0; DB_IO_TEXT_BYTES];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Ok(Self { bytes, len: value.len() as u16 })
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("DB I/O text is admitted from UTF-8")
    }

    fn terminal_is_empty(&self) -> bool {
        self.len == 0
    }

    fn close_step(&mut self) -> bool {
        if self.len == 0 {
            return false;
        }
        self.len = 0;
        true
    }
}

impl std::fmt::Write for DbIoText {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        let start = usize::from(self.len);
        let end = start.checked_add(value.len()).ok_or(std::fmt::Error)?;
        let target = self.bytes.get_mut(start..end).ok_or(std::fmt::Error)?;
        target.copy_from_slice(value.as_bytes());
        self.len = end as u16;
        Ok(())
    }
}

impl std::fmt::Debug for DbIoText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_tuple("DbIoText").field(&self.as_str()).finish()
    }
}

/// @emoji 🔢 Fixed typed list result without a dynamic vector owner.
pub struct DbIoU64List {
    values: [u64; DB_IO_LIST_ITEMS],
    len: u16,
}

impl DbIoU64List {
    pub fn new() -> Self {
        Self { values: [0; DB_IO_LIST_ITEMS], len: 0 }
    }

    pub fn push(&mut self, value: u64) -> Result<(), DbError> {
        let index = usize::from(self.len);
        let Some(slot) = self.values.get_mut(index) else { return Err(DbError::LimitExceeded("db_io list authority")) };
        *slot = value;
        self.len += 1;
        Ok(())
    }

    pub fn as_slice(&self) -> &[u64] {
        &self.values[..usize::from(self.len)]
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn sort_unstable(&mut self) {
        let len = usize::from(self.len);
        self.values[..len].sort_unstable();
    }

    pub fn close_step(&mut self) -> bool {
        if self.len == 0 {
            return false;
        }
        self.len -= 1;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Default for DbIoU64List {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for DbIoU64List {
    type Target = [u64];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl std::ops::DerefMut for DbIoU64List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let len = usize::from(self.len);
        &mut self.values[..len]
    }
}

impl std::fmt::Debug for DbIoU64List {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list().entries(self.as_slice()).finish()
    }
}

impl PartialEq<Vec<u64>> for DbIoU64List {
    fn eq(&self, expected: &Vec<u64>) -> bool {
        self.as_slice() == expected.as_slice()
    }
}

impl<const N: usize> PartialEq<[u64; N]> for DbIoU64List {
    fn eq(&self, expected: &[u64; N]) -> bool {
        self.as_slice() == expected.as_slice()
    }
}

impl IntoIterator for DbIoU64List {
    type Item = u64;
    type IntoIter = std::iter::Take<std::array::IntoIter<u64, DB_IO_LIST_ITEMS>>;

    fn into_iter(self) -> Self::IntoIter {
        let len = usize::from(self.len);
        self.values.into_iter().take(len)
    }
}

/// @emoji 🧭 Repository-owned backend identity; external driver values never cross this boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoBackendControl {
    Memory { slot: u16, generation: u64 },
    Filesystem { slot: u16, generation: u64 },
    Sqlite { slot: u16, generation: u64 },
    Postgres { slot: u16, generation: u64 },
    Neo4j { slot: u16, generation: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoBackendKind {
    Memory,
    Filesystem,
    Sqlite,
    Postgres,
    Neo4j,
}

/// @emoji 🗂️ Schema-first database I/O task owner.
pub enum DbIoTask {
    BackendOpen { backend: DbIoBackendControl, path: DbIoText },
    WalCreate { backend: DbIoBackendControl, document: DbIoText, index: u64 },
    WalAppend { backend: DbIoBackendControl, document: DbIoText, index: u64, input: DbIoPages },
    WalSync { backend: DbIoBackendControl, document: DbIoText, index: u64, class: DurabilityClass },
    WalSeal { backend: DbIoBackendControl, document: DbIoText, index: u64 },
    WalRead { backend: DbIoBackendControl, document: DbIoText, index: u64, range: ByteRange, output: DbIoPageWriter },
    WalLength { backend: DbIoBackendControl, document: DbIoText, index: u64 },
    WalList { backend: DbIoBackendControl, document: DbIoText, output: DbIoU64List },
    WalTruncate { backend: DbIoBackendControl, document: DbIoText, index: u64, new_len: u64 },
    WalDelete { backend: DbIoBackendControl, document: DbIoText, index: u64 },
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

/// @emoji 📬 Exact typed database I/O terminal result.
pub enum DbIoResult {
    Unit,
    Length(u64),
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

impl DbIoTask {
    pub fn backend(&self) -> DbIoBackendControl {
        match self {
            Self::BackendOpen { backend, .. }
            | Self::WalCreate { backend, .. }
            | Self::WalAppend { backend, .. }
            | Self::WalSync { backend, .. }
            | Self::WalSeal { backend, .. }
            | Self::WalRead { backend, .. }
            | Self::WalLength { backend, .. }
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

    fn admit_pages(&self) -> Result<(), DbError> {
        match self {
            Self::WalAppend { input, .. }
            | Self::SnapshotWrite { input, .. }
            | Self::PayloadPut { input, .. }
            | Self::CatalogCas { input, .. }
            | Self::IndexWrite { input, .. } => input.admit(),
            Self::WalRead { output, .. }
            | Self::SnapshotRead { output, .. }
            | Self::PayloadGet { output, .. }
            | Self::CatalogRead { output, .. }
            | Self::IndexRead { output, .. } => output.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued),
            _ => Ok(()),
        }
    }

    fn transition_pages(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        match self {
            Self::WalAppend { input, .. }
            | Self::SnapshotWrite { input, .. }
            | Self::PayloadPut { input, .. }
            | Self::CatalogCas { input, .. }
            | Self::IndexWrite { input, .. } => input.transition(expected, next),
            Self::WalRead { output, .. }
            | Self::SnapshotRead { output, .. }
            | Self::PayloadGet { output, .. }
            | Self::CatalogRead { output, .. }
            | Self::IndexRead { output, .. } => output.transition(expected, next),
            _ => Ok(()),
        }
    }

    fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        match self {
            Self::WalAppend { document, input, .. }
            | Self::SnapshotWrite { document, input, .. }
            | Self::IndexWrite { document, input, .. } => {
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
            Self::WalRead { document, output, .. }
            | Self::SnapshotRead { document, output, .. }
            | Self::IndexRead { document, output, .. } => {
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
            Self::WalList { document, output, .. }
            | Self::SnapshotLatest { document, output, .. }
            | Self::SnapshotList { document, output, .. }
            | Self::IndexList { document, output, .. } => {
                if output.close_step() || document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::WalCreate { document, .. }
            | Self::WalSync { document, .. }
            | Self::WalSeal { document, .. }
            | Self::WalLength { document, .. }
            | Self::WalTruncate { document, .. }
            | Self::WalDelete { document, .. }
            | Self::SnapshotDelete { document, .. }
            | Self::IndexDelete { document, .. } => {
                if document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::LeaseAcquire { document, holder, .. }
            | Self::LeaseRenew { document, holder, .. }
            | Self::LeaseRelease { document, holder, .. } => {
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

    fn terminal_is_empty(&self) -> bool {
        match self {
            Self::WalAppend { document, input, .. }
            | Self::SnapshotWrite { document, input, .. }
            | Self::IndexWrite { document, input, .. } => document.terminal_is_empty() && input.terminal_is_empty(),
            Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } => input.terminal_is_empty(),
            Self::WalRead { document, output, .. }
            | Self::SnapshotRead { document, output, .. }
            | Self::IndexRead { document, output, .. } => document.terminal_is_empty() && output.terminal_is_empty(),
            Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } => output.terminal_is_empty(),
            Self::BackendOpen { path, .. } => path.terminal_is_empty(),
            Self::WalList { document, output, .. }
            | Self::SnapshotLatest { document, output, .. }
            | Self::SnapshotList { document, output, .. }
            | Self::IndexList { document, output, .. } => document.terminal_is_empty() && output.terminal_is_empty(),
            Self::WalCreate { document, .. }
            | Self::WalSync { document, .. }
            | Self::WalSeal { document, .. }
            | Self::WalLength { document, .. }
            | Self::WalTruncate { document, .. }
            | Self::WalDelete { document, .. }
            | Self::SnapshotDelete { document, .. }
            | Self::IndexDelete { document, .. }
            | Self::LeaseGet { document, .. } => document.terminal_is_empty(),
            Self::LeaseAcquire { document, holder, .. }
            | Self::LeaseRenew { document, holder, .. }
            | Self::LeaseRelease { document, holder, .. } => document.terminal_is_empty() && holder.terminal_is_empty(),
            Self::PayloadExists { .. } | Self::PayloadLength { .. } | Self::PayloadDelete { .. } | Self::BackendClose { .. } => true,
        }
    }
}

impl DbIoResult {
    fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        match self {
            Self::Pages(pages) => pages.close_step(),
            Self::OptionalCatalog(Some((pages, _))) => pages.close_step(),
            Self::List(list) => Ok(list.close_step().then_some(0)),
            Self::Lease(lease) | Self::OptionalLease(Some(lease)) => {
                if lease.holder.close_step() || lease.resource.close_step() {
                    return Ok(Some(0));
                }
                Ok(None)
            }
            Self::Unit
            | Self::Length(_)
            | Self::OptionalLength(_)
            | Self::Exists(_)
            | Self::Hash(_)
            | Self::Fence(_)
            | Self::OptionalCatalog(None)
            | Self::OptionalLease(None) => Ok(None),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Pages(pages) => pages.terminal_is_empty(),
            Self::OptionalCatalog(Some((pages, _))) => pages.terminal_is_empty(),
            Self::List(list) => list.terminal_is_empty(),
            Self::Lease(lease) | Self::OptionalLease(Some(lease)) => lease.resource.terminal_is_empty() && lease.holder.terminal_is_empty(),
            Self::Unit
            | Self::Length(_)
            | Self::OptionalLength(_)
            | Self::Exists(_)
            | Self::Hash(_)
            | Self::Fence(_)
            | Self::OptionalCatalog(None)
            | Self::OptionalLease(None) => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoExecutionStep {
    Yield,
    Complete,
}

/// @emoji 🔌 Platform drivers implement one typed, resumable task step behind repository owners.
pub trait DbIoTaskExecutor: Send + Sync {
    fn execute_step(&self, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError>;

    fn close_operation_step(&self, operation: u64, task: &DbIoTask) -> Result<bool, DbError>;
}

const DB_IO_BACKEND_CONTROLS: usize = 64;

struct DbIoBackendRegistrySlot {
    generation: u64,
    executor: Option<Arc<dyn DbIoTaskExecutor>>,
}

struct DbIoBackendRegistry {
    slots: [DbIoBackendRegistrySlot; DB_IO_BACKEND_CONTROLS],
    free: [u16; DB_IO_BACKEND_CONTROLS],
    free_read: usize,
    free_len: usize,
    next_generation: u64,
}

impl DbIoBackendRegistry {
    fn new() -> Self {
        Self {
            slots: std::array::from_fn(|_| DbIoBackendRegistrySlot { generation: 0, executor: None }),
            free: std::array::from_fn(|index| index as u16),
            free_read: 0,
            free_len: DB_IO_BACKEND_CONTROLS,
            next_generation: 1,
        }
    }
}

static DB_IO_BACKEND_REGISTRY: std::sync::OnceLock<std::sync::Mutex<DbIoBackendRegistry>> = std::sync::OnceLock::new();

fn db_io_backend_registry() -> &'static std::sync::Mutex<DbIoBackendRegistry> {
    DB_IO_BACKEND_REGISTRY.get_or_init(|| std::sync::Mutex::new(DbIoBackendRegistry::new()))
}

pub fn register_db_io_backend(kind: DbIoBackendKind, executor: Arc<dyn DbIoTaskExecutor>) -> Result<DbIoBackendControl, DbError> {
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if registry.free_len == 0 || registry.next_generation == u64::MAX {
        return Err(DbError::Unavailable("db I/O backend control capacity exhausted".to_string()));
    }
    let slot = registry.free[registry.free_read];
    registry.free_read = (registry.free_read + 1) % DB_IO_BACKEND_CONTROLS;
    registry.free_len -= 1;
    let generation = registry.next_generation;
    registry.next_generation += 1;
    registry.slots[slot as usize] = DbIoBackendRegistrySlot { generation, executor: Some(executor) };
    let control = match kind {
        DbIoBackendKind::Memory => DbIoBackendControl::Memory { slot, generation },
        DbIoBackendKind::Filesystem => DbIoBackendControl::Filesystem { slot, generation },
        DbIoBackendKind::Sqlite => DbIoBackendControl::Sqlite { slot, generation },
        DbIoBackendKind::Postgres => DbIoBackendControl::Postgres { slot, generation },
        DbIoBackendKind::Neo4j => DbIoBackendControl::Neo4j { slot, generation },
    };
    Ok(control)
}

fn db_io_backend_parts(control: DbIoBackendControl) -> (u16, u64) {
    match control {
        DbIoBackendControl::Memory { slot, generation }
        | DbIoBackendControl::Filesystem { slot, generation }
        | DbIoBackendControl::Sqlite { slot, generation }
        | DbIoBackendControl::Postgres { slot, generation }
        | DbIoBackendControl::Neo4j { slot, generation } => (slot, generation),
    }
}

fn db_io_executor(control: DbIoBackendControl) -> Result<Arc<dyn DbIoTaskExecutor>, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &registry.slots[slot as usize];
    if owner.generation != generation {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    owner.executor.clone().ok_or(DbError::Closed)
}

pub fn unregister_db_io_backend(control: DbIoBackendControl) -> Result<(), DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &registry.slots[slot as usize];
    if owner.generation != generation || owner.executor.is_none() {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    registry.slots[slot as usize] = DbIoBackendRegistrySlot { generation: 0, executor: None };
    let write = (registry.free_read + registry.free_len) % DB_IO_BACKEND_CONTROLS;
    registry.free[write] = slot;
    registry.free_len += 1;
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoFaultKind {
    Backend,
    Cancelled,
    Panic,
    Saturated,
    Stale,
}

pub struct DbIoFault {
    pub kind: DbIoFaultKind,
    pub detail: DbIoText,
}

impl DbIoFault {
    pub fn into_db_error(self) -> DbError {
        match self.kind {
            DbIoFaultKind::Cancelled => DbError::Closed,
            DbIoFaultKind::Saturated => DbError::Unavailable(self.detail.as_str().to_string()),
            DbIoFaultKind::Stale => DbError::Internal(self.detail.as_str().to_string()),
            DbIoFaultKind::Backend | DbIoFaultKind::Panic => DbError::Internal(self.detail.as_str().to_string()),
        }
    }
}

enum DbIoTerminal {
    Result(DbIoResult),
    Fault(DbIoFault),
    Cancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DbIoTaskHandle {
    slot: u16,
    generation: u64,
    operation: u64,
}

struct DbIoTaskSlot {
    generation: u64,
    operation: u64,
    phase: DbIoTaskPhase,
    task: Option<DbIoTask>,
    terminal: Option<DbIoTerminal>,
    pool: Option<WorkerPool>,
    retry_job: Option<(Job, u8)>,
    terminal_job: Option<(WorkerSubmitErrorKind, Job)>,
    waker: Option<std::task::Waker>,
    retry_generation: u64,
    cancelled: bool,
    abandoned: bool,
    close_enqueued: bool,
    backend_cleanup_done: bool,
    backend_to_close: Option<DbIoBackendControl>,
    counted: bool,
}

impl DbIoTaskSlot {
    const fn empty() -> Self {
        Self {
            generation: 0,
            operation: 0,
            phase: DbIoTaskPhase::Closing,
            task: None,
            terminal: None,
            pool: None,
            retry_job: None,
            terminal_job: None,
            waker: None,
            retry_generation: 0,
            cancelled: false,
            abandoned: false,
            close_enqueued: false,
            backend_cleanup_done: false,
            backend_to_close: None,
            counted: false,
        }
    }
}

static DB_IO_TASK_SLOTS: [std::sync::Mutex<DbIoTaskSlot>; DB_IO_OPERATION_ITEMS] = [const { std::sync::Mutex::new(DbIoTaskSlot::empty()) }; DB_IO_OPERATION_ITEMS];

struct DbIoTaskArena {
    free: [u16; DB_IO_OPERATION_ITEMS],
    free_read: usize,
    free_len: usize,
    closing: [Option<DbIoTaskHandle>; DB_IO_OPERATION_ITEMS],
    closing_read: usize,
    closing_len: usize,
    next_generation: u64,
}

impl DbIoTaskArena {
    fn new() -> Self {
        Self {
            free: std::array::from_fn(|index| index as u16),
            free_read: 0,
            free_len: DB_IO_OPERATION_ITEMS,
            closing: [None; DB_IO_OPERATION_ITEMS],
            closing_read: 0,
            closing_len: 0,
            next_generation: 1,
        }
    }
}

static DB_IO_TASK_ARENA: std::sync::OnceLock<std::sync::Mutex<DbIoTaskArena>> = std::sync::OnceLock::new();

fn db_io_task_arena() -> &'static std::sync::Mutex<DbIoTaskArena> {
    DB_IO_TASK_ARENA.get_or_init(|| std::sync::Mutex::new(DbIoTaskArena::new()))
}

fn db_io_task_fault(kind: DbIoFaultKind, error: &DbError) -> DbIoFault {
    use std::fmt::Write as _;
    let mut detail = DbIoText::new();
    if write!(&mut detail, "{error}").is_err() {
        detail = DbIoText::try_from_str("DB I/O fault detail exceeded fixed authority").expect("fixed fault literal");
    }
    DbIoFault { kind, detail }
}

fn db_io_allocate_task(pool: &WorkerPool, task: DbIoTask) -> Result<DbIoTaskHandle, (DbError, DbIoTask)> {
    let operation = match db_io_next_operation() {
        Ok(operation) => operation,
        Err(error) => return Err((error, task)),
    };
    let mut arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if arena.free_len == 0 || arena.next_generation == u64::MAX {
        return Err((DbError::Unavailable("db I/O task capacity exhausted".to_string()), task));
    }
    if let Err(error) = task.admit_pages() {
        return Err((error, task));
    }
    let slot = arena.free[arena.free_read];
    arena.free_read = (arena.free_read + 1) % DB_IO_OPERATION_ITEMS;
    arena.free_len -= 1;
    let generation = arena.next_generation;
    arena.next_generation += 1;
    drop(arena);
    let handle = DbIoTaskHandle { slot, generation, operation };
    let mut owner = DB_IO_TASK_SLOTS[slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let backend_to_close = match &task {
        DbIoTask::BackendClose { backend } => Some(*backend),
        _ => None,
    };
    *owner = DbIoTaskSlot {
        generation,
        operation,
        phase: DbIoTaskPhase::Admitted,
        task: Some(task),
        terminal: None,
        pool: Some(pool.clone()),
        retry_job: None,
        terminal_job: None,
        waker: None,
        retry_generation: 1,
        cancelled: false,
        abandoned: false,
        close_enqueued: false,
        backend_cleanup_done: false,
        backend_to_close,
        counted: true,
    };
    BLOCKING_QUEUE.enqueued(0);
    Ok(handle)
}

fn db_io_slot_matches(slot: &DbIoTaskSlot, handle: DbIoTaskHandle) -> bool {
    slot.generation == handle.generation && slot.operation == handle.operation && handle.generation != 0 && handle.operation != 0
}

fn db_io_enqueue_close(handle: DbIoTaskHandle) -> Result<(), DbError> {
    let mut arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if arena.closing_len == DB_IO_OPERATION_ITEMS {
        return Err(DbError::Unavailable("DB I/O task close arena saturated".to_string()));
    }
    let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if !db_io_slot_matches(&owner, handle) {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    if owner.close_enqueued {
        return Ok(());
    }
    owner.close_enqueued = true;
    owner.phase = DbIoTaskPhase::Closing;
    drop(owner);
    let write = (arena.closing_read + arena.closing_len) % DB_IO_OPERATION_ITEMS;
    arena.closing[write] = Some(handle);
    arena.closing_len += 1;
    Ok(())
}

fn db_io_submit_close_job(pool: WorkerPool, job: Job) {
    match pool.try_submit(Lane::Io, job) {
        Ok(()) => {}
        Err(error) => {
            let job = error.into_job();
            let retry_pool = pool.clone();
            pool.callback_at(pool.now_ms().saturating_add(DB_IO_RETRY_MS), move || db_io_submit_close_job(retry_pool, job));
        }
    }
}

fn db_io_schedule_close(handle: DbIoTaskHandle) {
    let pool = {
        let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle) {
            return;
        }
        owner.pool.clone()
    };
    if let Some(pool) = pool {
        let resubmit_pool = pool.clone();
        db_io_submit_close_job(
            pool,
            Box::new(move || {
                let _ = db_io_task_close_step();
                let active = {
                    let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    db_io_slot_matches(&owner, handle)
                };
                if active {
                    db_io_schedule_close(handle);
                }
                drop(resubmit_pool);
            }),
        );
    }
}

fn db_io_wake(owner: &mut DbIoTaskSlot) -> Option<std::task::Waker> {
    owner.waker.take()
}

fn db_io_submit_job(handle: DbIoTaskHandle, job: Job, attempt: u8) {
    let pool = {
        let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle) {
            return;
        }
        owner.pool.clone()
    };
    let Some(pool) = pool else { return };
    match pool.try_submit(Lane::Io, job) {
        Ok(()) => {
            let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if db_io_slot_matches(&owner, handle) {
                owner.phase = DbIoTaskPhase::Queued;
            }
        }
        Err(error) => match error.kind() {
            WorkerSubmitErrorKind::Contended | WorkerSubmitErrorKind::Saturated if attempt < DB_IO_RETRY_LIMIT => {
                let retry = error.into_job();
                let generation = {
                    let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    if !db_io_slot_matches(&owner, handle) {
                        return;
                    }
                    owner.retry_job = Some((retry, attempt + 1));
                    owner.retry_generation = owner.retry_generation.checked_add(1).expect("DB I/O retry generation exhausted");
                    owner.retry_generation
                };
                pool.callback_at(pool.now_ms().saturating_add(DB_IO_RETRY_MS), move || db_io_retry(handle, generation));
            }
            kind => {
                let job = error.into_job();
                let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if !db_io_slot_matches(&owner, handle) {
                    return;
                }
                owner.phase = DbIoTaskPhase::Faulted;
                owner.terminal_job = Some((kind, job));
                owner.terminal = Some(DbIoTerminal::Fault(DbIoFault { kind: DbIoFaultKind::Saturated, detail: DbIoText::try_from_str("DB I/O WorkerPool submission failed").expect("fixed submission fault") }));
                if let Some(waker) = db_io_wake(&mut owner) {
                    drop(owner);
                    waker.wake();
                }
            }
        },
    }
}

fn db_io_retry(handle: DbIoTaskHandle, retry_generation: u64) {
    let retry = {
        let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle) || owner.retry_generation != retry_generation {
            return;
        }
        owner.retry_job.take()
    };
    if let Some((job, attempt)) = retry {
        db_io_submit_job(handle, job, attempt);
    }
}

fn db_io_drive_one(handle: DbIoTaskHandle) {
    let (mut task, cancelled) = {
        let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle) || owner.close_enqueued {
            return;
        }
        owner.phase = DbIoTaskPhase::Executing;
        (owner.task.take(), owner.cancelled)
    };
    let Some(mut task_owner) = task.take() else { return };
    if cancelled {
        let _ = task_owner.transition_pages(DbIoPagePhase::Queued, DbIoPagePhase::TerminalResult);
        let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        owner.task = Some(task_owner);
        owner.phase = DbIoTaskPhase::Cancelled;
        owner.terminal = Some(DbIoTerminal::Cancelled);
        if let Some(waker) = db_io_wake(&mut owner) {
            drop(owner);
            waker.wake();
        }
        return;
    }
    let mut panicked = false;
    let execution = task_owner.transition_pages(DbIoPagePhase::Queued, DbIoPagePhase::Executing).and_then(|()| {
        db_io_executor(task_owner.backend()).and_then(|executor| {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| executor.execute_step(handle.operation, &mut task_owner))) {
                Ok(result) => result,
                Err(_) => {
                    panicked = true;
                    Err(DbError::Internal("DB I/O backend panicked".to_string()))
                }
            }
        })
    });
    let execution = match execution {
        Ok((DbIoExecutionStep::Yield, None)) => task_owner.transition_pages(DbIoPagePhase::Executing, DbIoPagePhase::Queued).map(|()| (DbIoExecutionStep::Yield, None)),
        terminal => {
            let transition = task_owner.transition_pages(DbIoPagePhase::Executing, DbIoPagePhase::TerminalResult);
            match (terminal, transition) {
                (Ok(step), Ok(())) => Ok(step),
                (Err(error), _) | (_, Err(error)) => Err(error),
            }
        }
    };
    let mut resubmit = false;
    let mut waker = None;
    {
        let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle) {
            return;
        }
        owner.task = Some(task_owner);
        match execution {
            Ok((DbIoExecutionStep::Yield, None)) => {
                owner.phase = DbIoTaskPhase::Queued;
                resubmit = !owner.cancelled && !owner.abandoned;
            }
            Ok((DbIoExecutionStep::Complete, Some(result))) => {
                owner.phase = if owner.cancelled { DbIoTaskPhase::Cancelled } else { DbIoTaskPhase::Completed };
                owner.terminal = Some(if owner.cancelled { DbIoTerminal::Cancelled } else { DbIoTerminal::Result(result) });
                waker = db_io_wake(&mut owner);
            }
            Ok(_) => {
                owner.phase = DbIoTaskPhase::Faulted;
                owner.terminal = Some(DbIoTerminal::Fault(DbIoFault { kind: DbIoFaultKind::Backend, detail: DbIoText::try_from_str("DB I/O executor returned an invalid typed step").expect("fixed typed-step fault") }));
                waker = db_io_wake(&mut owner);
            }
            Err(error) => {
                owner.phase = DbIoTaskPhase::Faulted;
                owner.terminal = Some(DbIoTerminal::Fault(db_io_task_fault(if panicked { DbIoFaultKind::Panic } else { DbIoFaultKind::Backend }, &error)));
                waker = db_io_wake(&mut owner);
            }
        }
    }
    if let Some(waker) = waker {
        waker.wake();
    }
    if resubmit {
        db_io_submit_job(handle, Box::new(move || db_io_drive_one(handle)), 0);
    }
}

pub struct DbIoTaskOperation {
    handle: DbIoTaskHandle,
    resolved: bool,
}

pub fn submit_db_io_task(pool: &WorkerPool, task: DbIoTask) -> Result<DbIoTaskOperation, (DbError, DbIoTask)> {
    let handle = db_io_allocate_task(pool, task)?;
    db_io_submit_job(handle, Box::new(move || db_io_drive_one(handle)), 0);
    Ok(DbIoTaskOperation { handle, resolved: false })
}

impl DbIoTaskOperation {
    pub fn phase(&self) -> DbIoTaskPhase {
        let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if db_io_slot_matches(&owner, self.handle) {
            owner.phase
        } else {
            DbIoTaskPhase::Closing
        }
    }

    pub fn cancel(&self) -> Result<(), DbError> {
        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, self.handle) {
            return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
        }
        owner.cancelled = true;
        Ok(())
    }

    pub fn resume(&self) -> Result<bool, DbError> {
        let retry_generation = {
            let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) {
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
            }
            if owner.retry_job.is_none() {
                return Ok(false);
            }
            owner.retry_generation
        };
        db_io_retry(self.handle, retry_generation);
        Ok(true)
    }

    pub fn take(&mut self) -> Result<Option<Result<DbIoResult, DbIoFault>>, DbError> {
        {
            let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) {
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
            }
            if owner.terminal.is_none() {
                return Ok(None);
            }
        }
        db_io_enqueue_close(self.handle)?;
        let terminal = {
            let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) {
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
            }
            owner.terminal.take().ok_or_else(|| DbError::Internal("DB I/O terminal owner changed during exact take".to_string()))?
        };
        self.resolved = true;
        db_io_schedule_close(self.handle);
        Ok(Some(match terminal {
            DbIoTerminal::Result(result) => Ok(result),
            DbIoTerminal::Fault(fault) => Err(fault),
            DbIoTerminal::Cancelled => Err(DbIoFault { kind: DbIoFaultKind::Cancelled, detail: DbIoText::try_from_str("DB I/O task cancelled").expect("fixed cancellation fault") }),
        }))
    }
}

impl Future for DbIoTaskOperation {
    type Output = Result<DbIoResult, DbIoFault>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match self.take() {
            Ok(Some(terminal)) => return std::task::Poll::Ready(terminal),
            Ok(None) => {}
            Err(error) => return std::task::Poll::Ready(Err(db_io_task_fault(DbIoFaultKind::Stale, &error))),
        }
        let terminal_published = {
            let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) {
                return std::task::Poll::Ready(Err(DbIoFault { kind: DbIoFaultKind::Stale, detail: DbIoText::try_from_str("stale DB I/O terminal handle").expect("fixed stale fault") }));
            }
            if owner.terminal.is_some() {
                true
            } else {
                owner.waker = Some(context.waker().clone());
                false
            }
        };
        if terminal_published {
            return std::task::Poll::Ready(match self.take() {
                Ok(Some(terminal)) => terminal,
                Ok(None) => Err(DbIoFault { kind: DbIoFaultKind::Stale, detail: DbIoText::try_from_str("DB I/O terminal owner changed during poll").expect("fixed stale fault") }),
                Err(error) => Err(db_io_task_fault(DbIoFaultKind::Stale, &error)),
            });
        }
        std::task::Poll::Pending
    }
}

impl Drop for DbIoTaskOperation {
    fn drop(&mut self) {
        if self.resolved {
            return;
        }
        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if db_io_slot_matches(&owner, self.handle) {
            owner.abandoned = true;
            owner.cancelled = true;
        }
        drop(owner);
        if db_io_enqueue_close(self.handle).is_ok() {
            db_io_schedule_close(self.handle);
        }
    }
}

pub fn db_io_task_close_step() -> Result<Option<usize>, DbError> {
    let handle = {
        let arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if arena.closing_len == 0 {
            return Ok(None);
        }
        arena.closing[arena.closing_read].expect("DB I/O closing length names an exact handle")
    };
    let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if !db_io_slot_matches(&owner, handle) {
        return Err(DbError::Internal("DB I/O close lost task ABA authority".to_string()));
    }
    if owner.phase == DbIoTaskPhase::Executing || owner.phase == DbIoTaskPhase::Queued && owner.retry_job.is_none() {
        return Ok(Some(0));
    }
    if !owner.backend_cleanup_done {
        let task = owner.task.as_ref().ok_or_else(|| DbError::Internal("DB I/O cleanup lost typed task owner".to_string()))?;
        if !db_io_executor(task.backend())?.close_operation_step(handle.operation, task)? {
            return Ok(Some(0));
        }
        owner.backend_cleanup_done = true;
        return Ok(Some(0));
    }
    if let Some(terminal) = owner.terminal.as_mut() {
        let step = match terminal {
            DbIoTerminal::Result(result) => result.close_step()?,
            DbIoTerminal::Fault(fault) => fault.detail.close_step().then_some(0),
            DbIoTerminal::Cancelled => None,
        };
        if let Some(bytes) = step {
            return Ok(Some(bytes));
        }
        owner.terminal = None;
        return Ok(Some(0));
    }
    if let Some(task) = owner.task.as_mut() {
        if let Some(bytes) = task.close_step()? {
            return Ok(Some(bytes));
        }
        if !task.terminal_is_empty() {
            return Err(DbError::Internal("DB I/O task reported false terminal".to_string()));
        }
        owner.task = None;
        return Ok(Some(0));
    }
    if owner.retry_job.take().is_some() || owner.terminal_job.take().is_some() || owner.waker.take().is_some() || owner.pool.take().is_some() {
        return Ok(Some(0));
    }
    if owner.counted {
        owner.counted = false;
        BLOCKING_QUEUE.dequeued(0);
        return Ok(Some(0));
    }
    let backend_to_close = owner.backend_to_close.take();
    *owner = DbIoTaskSlot::empty();
    drop(owner);
    let mut arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let retired = arena.closing[arena.closing_read].take();
    if retired != Some(handle) {
        return Err(DbError::Internal("DB I/O close queue changed exact handle".to_string()));
    }
    arena.closing_read = (arena.closing_read + 1) % DB_IO_OPERATION_ITEMS;
    arena.closing_len -= 1;
    let write = (arena.free_read + arena.free_len) % DB_IO_OPERATION_ITEMS;
    arena.free[write] = handle.slot;
    arena.free_len += 1;
    drop(arena);
    if let Some(backend) = backend_to_close {
        unregister_db_io_backend(backend)?;
    }
    Ok(Some(0))
}
//#endregion 🔖️Limits

//#region 🔖️RetainedDbIo
pub const DB_IO_OPERATION_ITEMS: usize = 64;
#[cfg(test)]
const DB_IO_OPERATION_BYTES: u64 = (DB_IO_PAGE_BYTES * DB_IO_OPERATION_PAGES) as u64;
const DB_IO_RETRY_MS: u64 = 1;
const DB_IO_RETRY_LIMIT: u8 = 8;

#[cfg(test)]
pub(crate) fn db_io_test_pool() -> Arc<WorkerPool> {
    static POOL: std::sync::OnceLock<Arc<WorkerPool>> = std::sync::OnceLock::new();
    POOL.get_or_init(|| Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)))).clone()
}

/// @emoji ✅️ Test-only sync/async bridge. 🚫️async: E5 executor bridge — poll-once: every future
/// this crate's own backends (`MemoryStorage`, and `FsStorage`/`DbBackend` driven by
/// `semio_framework_async::testkit::ManualRuntime`, whose `run_blocking` executes synchronously)
/// hand back is already `Ready` the instant it's first polled — there is no real async wait
/// anywhere in a unit test. This drives one to completion without needing a real executor,
/// mirroring `semio_framework_async`'s own `ManualRuntime` test helper. The crate's single such
/// bridge (R2 E5: "at most one per crate"); [`block_on_ready`] is a thin `Result`-shaped wrapper
/// over it, not a second one.
#[cfg(test)]
async fn poll_once<T>(fut: impl Future<Output = T>) -> T {
    fut.await
}

#[cfg(test)]
async fn block_on_ready<T>(fut: impl Future<Output = Result<T, DbError>>) -> Result<T, DbError> {
    poll_once(fut).await
}
//#endregion 🔖️RetainedDbIo

//#region 🔖️Capabilities
/// @emoji 🎚️ What a concrete `DbStorage` backend actually supports — negotiated once at
/// `Database::open` and folded into `DbCapabilities` alongside enabled Cargo features.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StorageCapabilities {
    /// @emoji 💾️ True iff data written through this backend survives a process restart.
    pub durable: bool,
    /// @emoji 🥇️ The strongest `DurabilityClass` this backend can actually deliver on `sync`.
    pub max_durability: DurabilityClass,
    /// @emoji 🔒️ True iff `WalStorage::sync`/equivalent can force data to physical storage.
    pub supports_fsync: bool,
    /// @emoji ✅️ True iff `CatalogStorage::cas_root`/`LeaseStorage` provide real compare-and-swap
    /// fencing (as opposed to a backend that could only ever serve a single writer).
    pub supports_cas: bool,
}
//#endregion 🔖️Capabilities

//#region 🔖️WalStorage
/// @emoji 📜️ Raw, per-document, per-segment append-only byte storage — `db_wal` frames its own
/// `.spr` records on top of what this trait stores; this trait never interprets a byte written
/// through it. A document's WAL is a sequence of segments identified by a dense `u64` index;
/// exactly one segment (the highest-index one not yet `seal`ed) is ever "active" at a time.
pub trait WalStorage: Send + Sync {
    /// @emoji 🆕️ Creates a new, empty, unsealed segment `index` for `document`. Errors
    /// `AlreadyExists` if `index` already exists for `document`.
    async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError>;

    /// @emoji ➕️ Appends `bytes` to the active segment `index`, returning the segment's new total
    /// length. Errors `NotFound` if the segment doesn't exist, `InvalidArgument` if it is sealed.
    async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError>;

    /// @emoji 🔒️ Forces everything appended to segment `index` so far to the durability level
    /// implied by `class` — a no-op for `Memory`/`Os` (per `DurabilityClass`'s own
    /// doc: `Os` only promises "handed to the OS", not `fsync`ed), a real flush-to-disk for
    /// `Fsync`/`Quorum` (replication itself is `db_cluster`'s concern, not this trait's).
    async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError>;

    /// @emoji 🏁️ Marks segment `index` sealed: no further `append`/`truncate_tail` may target it.
    /// Errors `NotFound` if the segment doesn't exist. Idempotent if already sealed.
    async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError>;

    /// @emoji 📖️ Reads `range` of segment `index`'s bytes. Errors `NotFound` if the segment
    /// doesn't exist, `InvalidArgument` if `range` extends past the segment's current length.
    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError>;

    /// @emoji 📏️ The current length in bytes of segment `index`.
    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError>;

    /// @emoji 📋️ Every segment index that exists for `document`, ascending. Empty (not an error)
    /// if `document` has no WAL yet.
    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError>;

    /// @emoji ✂️ Truncates the ACTIVE (unsealed) segment `index` down to `new_len` bytes — the
    /// crash-recovery primitive for discarding a torn/uncommitted tail write. Errors
    /// `InvalidArgument` if the segment is sealed or if `new_len` exceeds its current length
    /// (this trait never extends a segment via truncation).
    async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError>;

    /// @emoji 🗑️ Deletes segment `index` entirely (both its bytes and seal marker), e.g. after
    /// `db_compact` has folded it into a later generation. Idempotent if already absent.
    async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError>;
}
//#endregion 🔖️WalStorage

//#region 🔖️SnapshotStorage
/// @emoji 📸️ Storage for whole snapshot generations — `db_snapshot` hands this trait complete
/// `.spk` pack-file bytes (pages in `KIND_CHUNK`, descriptor in `KIND_SNAPSHOT`) per generation;
/// this trait never parses them, it only persists and retrieves them by `(document, generation)`.
pub trait SnapshotStorage: Send + Sync {
    /// @emoji ✍️ Durably writes `bytes` as generation `generation` of `document`'s snapshot
    /// history. Overwrites if the same `(document, generation)` is written twice (the caller's
    /// responsibility to pick a fresh generation number per the contract's
    /// `Footer.prev_footer_offset` incremental-generation chain).
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError>;

    /// @emoji 📖️ Reads generation `generation`'s complete bytes. Errors `NotFound` if absent.
    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError>;

    /// @emoji 🥇️ The highest generation number stored for `document`, or `None` if it has no
    /// snapshot yet.
    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError>;

    /// @emoji 📋️ Every generation number stored for `document`, ascending.
    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError>;

    /// @emoji 🗑️ Deletes generation `generation`, e.g. once `db_compact`'s retention policy
    /// supersedes it. Idempotent if already absent.
    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError>;
}
//#endregion 🔖️SnapshotStorage

//#region 🔖️PayloadStorage
/// @emoji 🫙️ Content-addressed blob storage (blake3 CAS), shared across every document — large
/// command/payload bytes referenced from a WAL record or a snapshot page are stored once here and
/// referenced by `ContentHash` everywhere else, so identical payloads never duplicate on disk.
pub trait PayloadStorage: Send + Sync {
    /// @emoji ➕️ Stores `bytes` (if not already present — `put` is idempotent under content
    /// equality) and returns its `blake3` `ContentHash`.
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError>;

    /// @emoji 📖️ Reads the payload stored under `hash`. Errors `NotFound` if absent.
    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError>;

    /// @emoji ❓️ True iff a payload is stored under `hash`, without reading it.
    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError>;

    /// @emoji 🗑️ Deletes the payload stored under `hash` — `db_compact`'s ref-traced payload GC.
    /// Idempotent if already absent.
    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError>;

    /// @emoji 📏️ The byte length of the payload stored under `hash`, without reading it. Errors
    /// `NotFound` if absent.
    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError>;
}
//#endregion 🔖️PayloadStorage

//#region 🔖️CatalogStorage
/// @emoji 🗂️ The single catalog root blob (the family's document directory: names, ids,
/// metadata — opaque to this crate) with compare-and-swap-by-epoch writes: the split-brain gate
/// per `EpochFence`'s doc. Exactly one root exists per `DbStorage` instance.
pub trait CatalogStorage: Send + Sync {
    /// @emoji 📖️ The current root bytes and the `EpochFence` they were written under, or `None`
    /// if `cas_root` has never succeeded yet (a fresh, empty `DbStorage`).
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError>;

    /// @emoji ✅️ Compare-and-swap: succeeds only if `expected` matches the epoch of the currently
    /// stored root (or `EpochFence::INITIAL` if no root has ever been written), in which case the
    /// root becomes `new_bytes` under the next epoch (`expected.next()`), which is returned.
    /// Fails `DbError::Fenced` on any mismatch — a writer that lost leadership never silently
    /// overwrites a newer root.
    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError>;
}
//#endregion 🔖️CatalogStorage

//#region 🔖️IndexStorage
/// @emoji 🔍️ Storage for `db_index`'s immutable sorted runs (LSM-lite) — opaque per-document,
/// per-run byte blobs; this trait has no opinion on what's inside a run.
pub trait IndexStorage: Send + Sync {
    /// @emoji ✍️ Durably writes `bytes` as run `run_id` of `document`'s index. Overwrites if the
    /// same `(document, run_id)` is written twice.
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError>;

    /// @emoji 📖️ Reads run `run_id`'s complete bytes. Errors `NotFound` if absent.
    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError>;

    /// @emoji 📋️ Every run id stored for `document`, ascending.
    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError>;

    /// @emoji 🗑️ Deletes run `run_id`, e.g. after `db_index`'s merge policy folds it into a
    /// larger run. Idempotent if already absent.
    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError>;
}
//#endregion 🔖️IndexStorage

//#region 🔖️LeaseStorage
/// @emoji ⏳️ One resource's current lease state — `LeaseStorage::current`'s return shape.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LeaseInfo {
    pub resource: String,
    pub holder: String,
    pub fence: EpochFence,
    pub expires_at_ms: u64,
}

/// @emoji ⏳️ Named, TTL'd, fenced ownership leases — `db_cluster`'s shard-ownership + epoch
/// failover primitive. A lease is keyed by an opaque `resource` string (e.g. a shard id); at most
/// one `holder` may hold a given resource's lease at a time, and every successful hand-off (a
/// fresh `acquire` after the previous holder's lease expired) bumps the resource's `EpochFence` so
/// a stale former holder's writes are fenced out by `CatalogStorage`-style checks downstream.
pub trait LeaseStorage: Send + Sync {
    /// @emoji 🤝️ Acquires (or idempotently re-acquires, if `holder` already holds an unexpired
    /// lease on `resource`) `resource` for `holder`, valid until `now_ms + ttl_ms`. Returns the
    /// resource's current `EpochFence` — unchanged on re-acquire by the same holder, bumped
    /// (`.next()`) on a genuine hand-off from an expired or absent lease. Errors `Conflict` if
    /// another holder's lease on `resource` has not yet expired.
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError>;

    /// @emoji ♻️ Extends `holder`'s existing, unexpired lease on `resource` to `now_ms + ttl_ms`.
    /// `fence` must match the lease's current `EpochFence` (`DbError::Fenced` otherwise) and
    /// `holder` must match the current holder (`DbError::Unauthorized` otherwise). Errors
    /// `NotFound`/`Unavailable` if no lease (or an already-expired one) exists on `resource`.
    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError>;

    /// @emoji 🕊️ Voluntarily releases `holder`'s lease on `resource` early (`fence` and `holder`
    /// must match, same rules as `renew`), immediately freeing `resource` for another `acquire`
    /// (which will still bump the epoch, since a hand-off occurred).
    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError>;

    /// @emoji 👀️ The current unexpired lease on `resource` as of `now_ms`, or `None` if unheld or
    /// expired (an expired lease is reported as absent, never as stale data).
    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError>;
}
//#endregion 🔖️LeaseStorage

//#region 🔖️DbBackend
/// @emoji 🧰️ The umbrella storage substrate handle `db_engine`/the `db` facade hold as
/// `Arc<DbBackend>` (selected at `Database::open`, never compile-time-only per the contract).
/// Replaces the old `Arc<dyn DbStorage>` seam per ruling **O1** (drop dyn dispatch): every
/// former `&dyn WalStorage`/etc. accessor becomes a concrete facet-ref enum
/// ([`WalRef`]/[`SnapshotRef`]/[`PayloadRef`]/[`CatalogRef`]/[`IndexRef`]/[`LeaseRef`]) instead —
/// `Send`-ness is derived STRUCTURALLY at every spawn site (ruling **R3**) — never via a `+ Send`
/// bound on a trait method. No generic `R: HostAsyncRuntime` parameter (Phase 1 of
/// `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR` deleted it): `Fs`/`Sqlite`/`Fault` never used `R`
/// for anything but the removed `run_blocking` bridge, now [`FsStorage`]/`SqliteStorage`'s own
/// strong process `Arc<WorkerPool>` field.
pub enum DbBackend {
    Memory(MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(crate::db_storage_neo4j::Neo4jStorage),
    Fault(Box<crate::db_testkit::FaultStorage>),
}

impl DbBackend {
    /// @emoji 🔀️ This backend's [`WalRef`] facet — replaces the old `&dyn WalStorage`.
    pub async fn wal(&self) -> WalRef<'_> {
        match self {
            Self::Memory(s) => WalRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => WalRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => WalRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => WalRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => WalRef::Neo4j(s),
            Self::Fault(s) => WalRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`SnapshotRef`] facet — replaces the old `&dyn SnapshotStorage`.
    pub async fn snapshot(&self) -> SnapshotRef<'_> {
        match self {
            Self::Memory(s) => SnapshotRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => SnapshotRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => SnapshotRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => SnapshotRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => SnapshotRef::Neo4j(s),
            Self::Fault(s) => SnapshotRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`PayloadRef`] facet — replaces the old `&dyn PayloadStorage`.
    pub async fn payload(&self) -> PayloadRef<'_> {
        match self {
            Self::Memory(s) => PayloadRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => PayloadRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => PayloadRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => PayloadRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => PayloadRef::Neo4j(s),
            Self::Fault(s) => PayloadRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`CatalogRef`] facet — replaces the old `&dyn CatalogStorage`.
    pub async fn catalog(&self) -> CatalogRef<'_> {
        match self {
            Self::Memory(s) => CatalogRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => CatalogRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => CatalogRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => CatalogRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => CatalogRef::Neo4j(s),
            Self::Fault(s) => CatalogRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`IndexRef`] facet — replaces the old `&dyn IndexStorage`.
    pub async fn index(&self) -> IndexRef<'_> {
        match self {
            Self::Memory(s) => IndexRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => IndexRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => IndexRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => IndexRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => IndexRef::Neo4j(s),
            Self::Fault(s) => IndexRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`LeaseRef`] facet — replaces the old `&dyn LeaseStorage`.
    pub async fn lease(&self) -> LeaseRef<'_> {
        match self {
            Self::Memory(s) => LeaseRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => LeaseRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => LeaseRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => LeaseRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => LeaseRef::Neo4j(s),
            Self::Fault(s) => LeaseRef::Fault(&**s),
        }
    }

    /// @emoji 🎚️ What this concrete backend actually supports.
    pub async fn capabilities(&self) -> StorageCapabilities {
        match self {
            Self::Memory(s) => s.capabilities().await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.capabilities().await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.capabilities().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.capabilities().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.capabilities().await,
            // 🔀️ `FaultStorage::capabilities` calls back through `self.inner: DbBackend`
            // (`🧪️testkit`), so this arm is mutually recursive with this very fn — `Box::pin`
            // breaks the otherwise-infinitely-sized future (E0733).
            Self::Fault(s) => Box::pin(s.capabilities()).await,
        }
    }
}
//#endregion 🔖️DbBackend

//#region 🔖️WalRef
/// @emoji 🔀️ [`DbBackend::wal`]'s return shape — the enum that replaces
/// `&dyn WalStorage` per ruling **O1**.
pub enum WalRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> WalStorage for WalRef<'a> {
    async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.create_segment(document, index).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.create_segment(document, index).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.create_segment(document, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.create_segment(document, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.create_segment(document, index).await,
            Self::Fault(s) => Box::pin(s.create_segment(document, index)).await,
        }
    }

    async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        match self {
            Self::Memory(s) => s.append(document, index, bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.append(document, index, bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.append(document, index, bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.append(document, index, bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.append(document, index, bytes).await,
            Self::Fault(s) => Box::pin(s.append(document, index, bytes)).await,
        }
    }

    async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.sync(document, index, class).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.sync(document, index, class).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.sync(document, index, class).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.sync(document, index, class).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.sync(document, index, class).await,
            Self::Fault(s) => Box::pin(s.sync(document, index, class)).await,
        }
    }

    async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.seal(document, index).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.seal(document, index).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.seal(document, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.seal(document, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.seal(document, index).await,
            Self::Fault(s) => Box::pin(s.seal(document, index)).await,
        }
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        match self {
            Self::Memory(s) => s.read(document, index, range).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.read(document, index, range).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.read(document, index, range).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.read(document, index, range).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.read(document, index, range).await,
            Self::Fault(s) => Box::pin(s.read(document, index, range)).await,
        }
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        match self {
            Self::Memory(s) => s.segment_len(document, index).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.segment_len(document, index).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.segment_len(document, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.segment_len(document, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.segment_len(document, index).await,
            Self::Fault(s) => Box::pin(s.segment_len(document, index)).await,
        }
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match self {
            Self::Memory(s) => s.list_segments(document).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.list_segments(document).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.list_segments(document).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.list_segments(document).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.list_segments(document).await,
            Self::Fault(s) => Box::pin(s.list_segments(document)).await,
        }
    }

    async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.truncate_tail(document, index, new_len).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.truncate_tail(document, index, new_len).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.truncate_tail(document, index, new_len).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.truncate_tail(document, index, new_len).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.truncate_tail(document, index, new_len).await,
            Self::Fault(s) => Box::pin(s.truncate_tail(document, index, new_len)).await,
        }
    }

    async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete_segment(document, index).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.delete_segment(document, index).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.delete_segment(document, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.delete_segment(document, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.delete_segment(document, index).await,
            Self::Fault(s) => Box::pin(s.delete_segment(document, index)).await,
        }
    }
}
//#endregion 🔖️WalRef

//#region 🔖️SnapshotRef
/// @emoji 🔀️ [`DbBackend::snapshot`]'s return shape — the enum that replaces
/// `&dyn SnapshotStorage` per ruling **O1**.
pub enum SnapshotRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> SnapshotStorage for SnapshotRef<'a> {
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.write_generation(document, generation, bytes).await,
            Self::Fault(s) => Box::pin(s.write_generation(document, generation, bytes)).await,
        }
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        match self {
            Self::Memory(s) => s.read_generation(document, generation).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.read_generation(document, generation).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.read_generation(document, generation).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.read_generation(document, generation).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.read_generation(document, generation).await,
            Self::Fault(s) => Box::pin(s.read_generation(document, generation)).await,
        }
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        match self {
            Self::Memory(s) => s.latest_generation(document).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.latest_generation(document).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.latest_generation(document).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.latest_generation(document).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.latest_generation(document).await,
            Self::Fault(s) => Box::pin(s.latest_generation(document)).await,
        }
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match self {
            Self::Memory(s) => s.list_generations(document).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.list_generations(document).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.list_generations(document).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.list_generations(document).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.list_generations(document).await,
            Self::Fault(s) => Box::pin(s.list_generations(document)).await,
        }
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete_generation(document, generation).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.delete_generation(document, generation).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.delete_generation(document, generation).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.delete_generation(document, generation).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.delete_generation(document, generation).await,
            Self::Fault(s) => Box::pin(s.delete_generation(document, generation)).await,
        }
    }
}
//#endregion 🔖️SnapshotRef

//#region 🔖️PayloadRef
/// @emoji 🔀️ [`DbBackend::payload`]'s return shape — the enum that replaces
/// `&dyn PayloadStorage` per ruling **O1**.
pub enum PayloadRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> PayloadStorage for PayloadRef<'a> {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        match self {
            Self::Memory(s) => s.put(bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.put(bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.put(bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.put(bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.put(bytes).await,
            Self::Fault(s) => Box::pin(s.put(bytes)).await,
        }
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        match self {
            Self::Memory(s) => s.get(hash).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.get(hash).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.get(hash).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.get(hash).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.get(hash).await,
            Self::Fault(s) => Box::pin(s.get(hash)).await,
        }
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        match self {
            Self::Memory(s) => s.contains(hash).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.contains(hash).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.contains(hash).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.contains(hash).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.contains(hash).await,
            Self::Fault(s) => Box::pin(s.contains(hash)).await,
        }
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete(hash).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.delete(hash).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.delete(hash).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.delete(hash).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.delete(hash).await,
            Self::Fault(s) => Box::pin(s.delete(hash)).await,
        }
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        match self {
            Self::Memory(s) => s.len(hash).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.len(hash).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.len(hash).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.len(hash).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.len(hash).await,
            Self::Fault(s) => Box::pin(s.len(hash)).await,
        }
    }
}
//#endregion 🔖️PayloadRef

//#region 🔖️CatalogRef
/// @emoji 🔀️ [`DbBackend::catalog`]'s return shape — the enum that replaces
/// `&dyn CatalogStorage` per ruling **O1**.
pub enum CatalogRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> CatalogStorage for CatalogRef<'a> {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        match self {
            Self::Memory(s) => s.read_root().await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.read_root().await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.read_root().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.read_root().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.read_root().await,
            // 🔀️ `FaultStorage::read_root` calls back through `self.inner: DbBackend`, so this
            // arm is mutually recursive with this very fn — `Box::pin` breaks the otherwise-
            // infinitely-sized future (E0733), same as `DbBackend::capabilities`'s `Fault` arm.
            Self::Fault(s) => Box::pin(s.read_root()).await,
        }
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        match self {
            Self::Memory(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.cas_root(expected, new_bytes).await,
            Self::Fault(s) => Box::pin(s.cas_root(expected, new_bytes)).await,
        }
    }
}
//#endregion 🔖️CatalogRef

//#region 🔖️IndexRef
/// @emoji 🔀️ [`DbBackend::index`]'s return shape — the enum that replaces
/// `&dyn IndexStorage` per ruling **O1**.
pub enum IndexRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> IndexStorage for IndexRef<'a> {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.write_run(document, run_id, bytes).await,
            Self::Fault(s) => Box::pin(s.write_run(document, run_id, bytes)).await,
        }
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        match self {
            Self::Memory(s) => s.read_run(document, run_id).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.read_run(document, run_id).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.read_run(document, run_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.read_run(document, run_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.read_run(document, run_id).await,
            Self::Fault(s) => Box::pin(s.read_run(document, run_id)).await,
        }
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match self {
            Self::Memory(s) => s.list_runs(document).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.list_runs(document).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.list_runs(document).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.list_runs(document).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.list_runs(document).await,
            Self::Fault(s) => Box::pin(s.list_runs(document)).await,
        }
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete_run(document, run_id).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.delete_run(document, run_id).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.delete_run(document, run_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.delete_run(document, run_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.delete_run(document, run_id).await,
            Self::Fault(s) => Box::pin(s.delete_run(document, run_id)).await,
        }
    }
}
//#endregion 🔖️IndexRef

//#region 🔖️LeaseRef
/// @emoji 🔀️ [`DbBackend::lease`]'s return shape — the enum that replaces
/// `&dyn LeaseStorage` per ruling **O1**.
pub enum LeaseRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> LeaseStorage for LeaseRef<'a> {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        match self {
            Self::Memory(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            Self::Fault(s) => Box::pin(s.acquire(resource, holder, ttl_ms, now_ms)).await,
        }
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            Self::Fault(s) => Box::pin(s.renew(resource, holder, fence, ttl_ms, now_ms)).await,
        }
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.release(resource, holder, fence).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.release(resource, holder, fence).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.release(resource, holder, fence).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.release(resource, holder, fence).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.release(resource, holder, fence).await,
            Self::Fault(s) => Box::pin(s.release(resource, holder, fence)).await,
        }
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        match self {
            Self::Memory(s) => s.current(resource, now_ms).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.current(resource, now_ms).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.current(resource, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.current(resource, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.current(resource, now_ms).await,
            Self::Fault(s) => Box::pin(s.current(resource, now_ms)).await,
        }
    }
}
//#endregion 🔖️LeaseRef

//#region 🔖️Memory
/// @emoji 🧠️ One in-process, non-durable segment of a document's WAL — `bytes` plus whether
/// `seal` has been called on it.
struct MemWalSegment {
    bytes: Vec<u8>,
    sealed: bool,
}

/// @emoji 🧠️ A pure in-memory `DbStorage`: every store is a `Mutex`-guarded map, nothing ever
/// touches a filesystem. Not durable (`capabilities().durable == false`) — the backend for unit
/// tests and `db_testkit`'s deterministic simulation runtime, never for a real deployment. Every
/// trait method body below is synchronous (no real I/O to await), so it is simply wrapped in an
/// already-`Ready` `{ .. }` per the module doc's "Async-first" section.
#[derive(Default)]
pub struct MemoryStorage {
    wal: std::sync::Mutex<std::collections::HashMap<ArtifactId, std::collections::HashMap<u64, MemWalSegment>>>,
    snapshots: std::sync::Mutex<std::collections::HashMap<ArtifactId, std::collections::HashMap<u64, DbIoPages>>>,
    payloads: std::sync::Mutex<std::collections::HashMap<ContentHash, DbIoPages>>,
    catalog: std::sync::Mutex<Option<(DbIoPages, EpochFence)>>,
    index_runs: std::sync::Mutex<std::collections::HashMap<ArtifactId, std::collections::HashMap<u64, DbIoPages>>>,
    leases: std::sync::Mutex<std::collections::HashMap<String, LeaseInfo>>,
}

/// @emoji 🩹️ Recovers a `Mutex` guard from a poisoned lock instead of panicking — a single
/// panicking mailbox/actor elsewhere in the family must not turn every other document's storage
/// access into a cascading panic.
// 🚫️async: E1 pure accessor (no suspension) — see R9
fn lock<'a, T>(mutex: &'a std::sync::Mutex<T>) -> std::sync::MutexGuard<'a, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl MemoryStorage {
    pub async fn new() -> Self {
        Self::default()
    }
}

impl WalStorage for MemoryStorage {
    async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        let mut wal = lock(&self.wal);
        let segments = wal.entry(document.clone()).or_default();
        if segments.contains_key(&index) {
            return Err(DbError::AlreadyExists(format!("wal segment {index} for {document} already exists")));
        }
        segments.insert(index, MemWalSegment { bytes: Vec::new(), sealed: false });
        Ok(())
    }

    async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        let mut wal = lock(&self.wal);
        let segment = wal.get_mut(document).and_then(|segments| segments.get_mut(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        if segment.sealed {
            return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
        }
        for fragment in bytes.fragments() {
            segment.bytes.extend_from_slice(fragment);
        }
        Ok(segment.bytes.len() as u64)
    }

    async fn sync(&self, _document: &ArtifactId, _index: u64, _class: DurabilityClass) -> Result<(), DbError> {
        // 🎯️ Nothing is ever persisted, so every durability class is trivially satisfied in-process.
        {
            Ok(())
        }
    }

    async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        let mut wal = lock(&self.wal);
        let segment = wal.get_mut(document).and_then(|segments| segments.get_mut(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        segment.sealed = true;
        Ok(())
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        check_len(range.len, MAX_READ_BYTES, "wal_storage::read")?;
        let wal = lock(&self.wal);
        let segment = wal.get(document).and_then(|segments| segments.get(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        let start = range.offset as usize;
        let end = start.checked_add(range.len as usize).ok_or_else(|| DbError::InvalidArgument("wal read range overflows usize".to_string()))?;
        if end > segment.bytes.len() {
            return Err(DbError::InvalidArgument(format!("wal read range {start}..{end} out of bounds (len {})", segment.bytes.len())));
        }
        db_io_copy_pages(&segment.bytes[start..end])?.await
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        let wal = lock(&self.wal);
        let segment = wal.get(document).and_then(|segments| segments.get(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        Ok(segment.bytes.len() as u64)
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        let wal = lock(&self.wal);
        let mut indices = DbIoU64List::new();
        if let Some(segments) = wal.get(document) {
            for index in segments.keys().copied() {
                indices.push(index)?;
            }
        }
        indices.sort_unstable();
        Ok(indices)
    }

    async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
        let mut wal = lock(&self.wal);
        let segment = wal.get_mut(document).and_then(|segments| segments.get_mut(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        if segment.sealed {
            return Err(DbError::InvalidArgument(format!("cannot truncate sealed wal segment {index}")));
        }
        let new_len = new_len as usize;
        if new_len > segment.bytes.len() {
            return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
        }
        segment.bytes.truncate(new_len);
        Ok(())
    }

    async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        let mut wal = lock(&self.wal);
        if let Some(segments) = wal.get_mut(document) {
            segments.remove(&index);
        }
        Ok(())
    }
}

impl SnapshotStorage for MemoryStorage {
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        let mut snapshots = lock(&self.snapshots);
        snapshots.entry(document.clone()).or_default().insert(generation, bytes);
        Ok(())
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        let snapshots = lock(&self.snapshots);
        let bytes = snapshots.get(document).and_then(|generations| generations.get(&generation)).ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} for {document} not found")))?;
        db_io_copy_page_owner(bytes)?.await
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        let snapshots = lock(&self.snapshots);
        Ok(snapshots.get(document).and_then(|generations| generations.keys().max().copied()))
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        let snapshots = lock(&self.snapshots);
        let mut generations = DbIoU64List::new();
        if let Some(owners) = snapshots.get(document) {
            for generation in owners.keys().copied() {
                generations.push(generation)?;
            }
        }
        generations.sort_unstable();
        Ok(generations)
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        let mut snapshots = lock(&self.snapshots);
        if let Some(generations) = snapshots.get_mut(document) {
            generations.remove(&generation);
        }
        Ok(())
    }
}

impl PayloadStorage for MemoryStorage {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        check_len(bytes.len() as u64, MAX_READ_BYTES, "payload_storage::put")?;
        let mut hasher = blake3::Hasher::new();
        for fragment in bytes.fragments() {
            hasher.update(fragment);
        }
        let hash = ContentHash(*hasher.finalize().as_bytes());
        let mut payloads = lock(&self.payloads);
        payloads.entry(hash).or_insert(bytes);
        Ok(hash)
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        let payloads = lock(&self.payloads);
        let bytes = payloads.get(hash).ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?;
        db_io_copy_page_owner(bytes)?.await
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        Ok(lock(&self.payloads).contains_key(hash))
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        lock(&self.payloads).remove(hash);
        Ok(())
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        let payloads = lock(&self.payloads);
        payloads.get(hash).map(|bytes| bytes.len() as u64).ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))
    }
}

impl CatalogStorage for MemoryStorage {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        let catalog = lock(&self.catalog);
        match catalog.as_ref() {
            Some((pages, fence)) => Ok(Some((db_io_copy_page_owner(pages)?.await?, *fence))),
            None => Ok(None),
        }
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        check_len(new_bytes.len() as u64, MAX_READ_BYTES, "catalog_storage::cas_root")?;
        let mut catalog = lock(&self.catalog);
        let current_fence = catalog.as_ref().map_or(EpochFence::INITIAL, |(_, fence)| *fence);
        expected.check(current_fence)?;
        let new_fence = expected.next();
        *catalog = Some((new_bytes, new_fence));
        Ok(new_fence)
    }
}

impl IndexStorage for MemoryStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        let mut runs = lock(&self.index_runs);
        runs.entry(document.clone()).or_default().insert(run_id, bytes);
        Ok(())
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        let runs = lock(&self.index_runs);
        let bytes = runs.get(document).and_then(|runs| runs.get(&run_id)).ok_or_else(|| DbError::NotFound(format!("index run {run_id} for {document} not found")))?;
        db_io_copy_page_owner(bytes)?.await
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        let runs = lock(&self.index_runs);
        let mut ids = DbIoU64List::new();
        if let Some(owners) = runs.get(document) {
            for id in owners.keys().copied() {
                ids.push(id)?;
            }
        }
        ids.sort_unstable();
        Ok(ids)
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        let mut runs = lock(&self.index_runs);
        if let Some(runs) = runs.get_mut(document) {
            runs.remove(&run_id);
        }
        Ok(())
    }
}

impl LeaseStorage for MemoryStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        let mut leases = lock(&self.leases);
        let fence = match leases.get(resource) {
            Some(info) if now_ms < info.expires_at_ms => {
                if info.holder != holder {
                    return Err(DbError::Conflict(format!("resource {resource} is leased by another holder")));
                }
                info.fence
            }
            Some(info) => info.fence.next(),
            None => EpochFence::INITIAL,
        };
        leases.insert(resource.to_string(), LeaseInfo { resource: resource.to_string(), holder: holder.to_string(), fence, expires_at_ms: now_ms + ttl_ms });
        Ok(fence)
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        let mut leases = lock(&self.leases);
        let info = leases.get_mut(resource).ok_or_else(|| DbError::NotFound(format!("lease for {resource} not found")))?;
        if now_ms >= info.expires_at_ms {
            return Err(DbError::Unavailable(format!("lease for {resource} already expired")));
        }
        if info.holder != holder {
            return Err(DbError::Unauthorized(format!("lease for {resource} is not held by {holder}")));
        }
        fence.check(info.fence)?;
        info.expires_at_ms = now_ms + ttl_ms;
        Ok(())
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        let mut leases = lock(&self.leases);
        let info = leases.get(resource).ok_or_else(|| DbError::NotFound(format!("lease for {resource} not found")))?;
        if info.holder != holder {
            return Err(DbError::Unauthorized(format!("lease for {resource} is not held by {holder}")));
        }
        fence.check(info.fence)?;
        leases.remove(resource);
        Ok(())
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        let leases = lock(&self.leases);
        Ok(leases.get(resource).filter(|info| now_ms < info.expires_at_ms).cloned())
    }
}

impl MemoryStorage {
    /// @emoji 🎚️ Pure in-memory: never durable, always CAS-capable.
    pub async fn capabilities(&self) -> StorageCapabilities {
        StorageCapabilities { durable: false, max_durability: DurabilityClass::Memory, supports_fsync: false, supports_cas: true }
    }
}
//#endregion 🔖️Memory

//#region 🔖️Fs
/// @emoji 📁️ The zero-touch default `DbStorage`: pure files under a root directory, no new C
/// dependency (see module doc for the wasm32/native-only gating rationale). Layout under `root`:
/// `wal/<doc>/segment-<index>.bin` (+ `.sealed` marker once sealed), `snapshot/<doc>/gen-<n>.pack`,
/// `payload/<hash[0..2]>/<hash>.bin` (blake3 CAS, two-hex-char sharding), `catalog/root.bin`
/// (8-byte LE epoch prefix + opaque bytes), `index/<doc>/run-<id>.bin`, `lease/<resource>.bin`.
/// Every multi-step write (`cas_root`, lease grants, snapshot/index/payload writes) goes through
/// `pack::write_atomic` (temp file + `fsync` + `rename`) so a reader never observes a torn file.
/// Every blocking filesystem step runs as one typed task grant on the process I/O lane.
#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
mod fs_storage {
    use super::check_len;
    use super::{register_db_io_backend, submit_db_io_task, ArtifactId, ByteRange, ContentHash, DbError, DbIoBackendControl, DbIoBackendKind, DbIoExecutionStep, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, DurabilityClass, EpochFence, LeaseInfo, MAX_READ_BYTES};
    use super::{CatalogStorage, IndexStorage, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalStorage};
    use semio_framework_async::WorkerPool;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};

    /// @emoji 🚨️ Wraps a `std::io::Error` into `DbError::Io` — the only place `std::io::Error` is
    /// allowed to appear, per the contract's no-`std::io::Error`-in-public-signatures rule.
    #[allow(clippy::needless_pass_by_value)] // used as a `map_err` callback, which passes the error by value
                                             // 🚫️async: E4 fn-pointer slot
    fn io_err(err: std::io::Error) -> DbError {
        DbError::Io(err.to_string())
    }

    /// @emoji 🧭️ Maps a `std::io::Error` to `DbError::NotFound(missing())` when it's a missing-file
    /// error, or `DbError::Io` otherwise — used everywhere an open/read/stat is expected to find a
    /// caller-addressed blob that might legitimately not exist yet.
    // 🚫️async: E1 pure accessor called from sync `.map_err(|err| open_err(...))` closures — see R9
    fn open_err(err: std::io::Error, missing: impl FnOnce() -> String) -> DbError {
        if err.kind() == std::io::ErrorKind::NotFound {
            DbError::NotFound(missing())
        } else {
            io_err(err)
        }
    }

    /// @emoji 🛡️ Rejects a path component that could escape `root` (empty, `.`, `..`, or
    /// containing a path separator/NUL) — every document id, resource name, etc. that becomes a
    /// filesystem path component is validated through this before use.
    fn safe_component(raw: &str) -> Result<&str, DbError> {
        if raw.is_empty() || raw == "." || raw == ".." || raw.contains('/') || raw.contains('\\') || raw.contains('\0') {
            return Err(DbError::InvalidArgument(format!("unsafe storage path component: {raw:?}")));
        }
        Ok(raw)
    }

    fn wal_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("wal").join(safe_component(&document.0)?))
    }

    fn snapshot_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("snapshot").join(safe_component(&document.0)?))
    }

    fn index_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("index").join(safe_component(&document.0)?))
    }

    fn payload_path(root: &Path, hash: &ContentHash) -> PathBuf {
        let hex = hash.to_string();
        root.join("payload").join(&hex[0..2]).join(format!("{hex}.bin"))
    }

    fn catalog_path(root: &Path) -> PathBuf {
        root.join("catalog").join("root.bin")
    }

    fn lease_path(root: &Path, resource: &str) -> Result<PathBuf, DbError> {
        Ok(root.join("lease").join(format!("{}.bin", safe_component(resource)?)))
    }

    fn segment_path(dir: &Path, index: u64) -> PathBuf {
        dir.join(format!("segment-{index:020}.bin"))
    }

    fn sealed_marker_path(dir: &Path, index: u64) -> PathBuf {
        dir.join(format!("segment-{index:020}.sealed"))
    }

    fn generation_path(dir: &Path, generation: u64) -> PathBuf {
        dir.join(format!("gen-{generation:020}.pack"))
    }

    fn run_path(dir: &Path, run_id: u64) -> PathBuf {
        dir.join(format!("run-{run_id:020}.bin"))
    }


    fn read_lease_file(path: &Path) -> Result<Option<(EpochFence, u64, DbIoText)>, DbError> {
        if !path.exists() {
            return Ok(None);
        }
        let mut file = std::fs::File::open(path).map_err(io_err)?;
        let mut header = [0u8; 18];
        file.read_exact(&mut header).map_err(io_err)?;
        let epoch = u64::from_le_bytes(header[..8].try_into().expect("fixed lease epoch"));
        let expires_at_ms = u64::from_le_bytes(header[8..16].try_into().expect("fixed lease expiry"));
        let holder_len = usize::from(u16::from_le_bytes(header[16..18].try_into().expect("fixed lease holder length")));
        if holder_len > super::DB_IO_TEXT_BYTES || file.metadata().map_err(io_err)?.len() != (18 + holder_len) as u64 {
            return Err(DbError::Corrupt("lease record exceeds its fixed authority".to_string()));
        }
        let mut holder = [0u8; super::DB_IO_TEXT_BYTES];
        file.read_exact(&mut holder[..holder_len]).map_err(io_err)?;
        let holder = std::str::from_utf8(&holder[..holder_len]).map_err(|_| DbError::Corrupt("lease holder is not valid UTF-8".to_string()))?;
        Ok(Some((EpochFence { epoch }, expires_at_ms, DbIoText::try_from_str(holder)?)))
    }

    fn write_lease_file(path: &Path, fence: EpochFence, expires_at_ms: u64, holder: &str) -> Result<(), DbError> {
        let holder = DbIoText::try_from_str(holder)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(io_err)?;
        }
        let mut record = [0u8; 18 + super::DB_IO_TEXT_BYTES];
        record[..8].copy_from_slice(&fence.epoch.to_le_bytes());
        record[8..16].copy_from_slice(&expires_at_ms.to_le_bytes());
        record[16..18].copy_from_slice(&(holder.as_str().len() as u16).to_le_bytes());
        record[18..18 + holder.as_str().len()].copy_from_slice(holder.as_str().as_bytes());
        pack::write_atomic(path, &record[..18 + holder.as_str().len()])?;
        Ok(())
    }

    struct FsDbIoExecutor {
        root: DbIoText,
        catalog_lock: Mutex<()>,
        lease_lock: Mutex<()>,
        payload_hashes: [Mutex<Option<(u64, blake3::Hasher)>>; 64],
        readers: [Mutex<Option<FsReadState>>; 64],
    }

    struct FsReadState {
        operation: u64,
        file: std::fs::File,
        offset: u64,
        total: usize,
        fence: Option<EpochFence>,
    }

    impl FsDbIoExecutor {
        fn new(root: DbIoText) -> Self {
            Self { root, catalog_lock: Mutex::new(()), lease_lock: Mutex::new(()), payload_hashes: [const { Mutex::new(None) }; 64], readers: [const { Mutex::new(None) }; 64] }
        }

        fn root(&self) -> &Path {
            Path::new(self.root.as_str())
        }

        fn document_dir(&self, family: &str, document: &DbIoText) -> Result<PathBuf, DbError> {
            Ok(self.root().join(family).join(safe_component(document.as_str())?))
        }

        fn read_step(&self, operation: u64, path: &Path, offset: u64, exact_len: Option<u64>, catalog: bool, output: &mut DbIoPageWriter) -> Result<(DbIoExecutionStep, Option<(DbIoPages, Option<EpochFence>)>), DbError> {
            let slot = operation as usize % self.readers.len();
            let mut owner = self.readers[slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if owner.as_ref().is_some_and(|reader| reader.operation != operation) {
                return Err(DbError::Unavailable("filesystem read cursor capacity exhausted".to_string()));
            }
            if owner.is_none() {
                let mut file = std::fs::File::open(path).map_err(|error| open_err(error, || format!("database I/O object {} not found", path.display())))?;
                let available = file.metadata().map_err(io_err)?.len().checked_sub(offset).ok_or_else(|| DbError::InvalidArgument("DB I/O read offset exceeds object length".to_string()))?;
                let total = exact_len.unwrap_or(available);
                check_len(total, MAX_READ_BYTES, "db_io typed filesystem read")?;
                if total > available {
                    return Err(DbError::InvalidArgument("DB I/O read range exceeds object length".to_string()));
                }
                let fence = if catalog {
                    let mut header = [0u8; 8];
                    file.read_exact(&mut header).map_err(io_err)?;
                    Some(EpochFence { epoch: u64::from_le_bytes(header) })
                } else {
                    None
                };
                *owner = Some(FsReadState { operation, file, offset, total: total as usize, fence });
            }
            let reader = owner.as_mut().expect("filesystem read cursor retained");
            if output.len() == reader.total {
                let fence = reader.fence;
                owner.take();
                return Ok((DbIoExecutionStep::Complete, Some((output.finish()?, fence))));
            }
            let mut fragment = [0u8; super::DB_IO_PAGE_BYTES];
            let fragment_len = (reader.total - output.len()).min(fragment.len());
            reader.file.seek(SeekFrom::Start(reader.offset + output.len() as u64)).map_err(io_err)?;
            reader.file.read_exact(&mut fragment[..fragment_len]).map_err(io_err)?;
            if output.write_fragment(&fragment[..fragment_len])? != fragment_len {
                return Err(DbError::Internal("DB I/O output writer accepted a partial fixed fragment".to_string()));
            }
            Ok((DbIoExecutionStep::Yield, None))
        }

        fn read_pages_step(&self, operation: u64, path: &Path, offset: u64, exact_len: Option<u64>, output: &mut DbIoPageWriter) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            let (step, result) = self.read_step(operation, path, offset, exact_len, false, output)?;
            Ok((step, result.map(|(pages, _)| DbIoResult::Pages(pages))))
        }

        fn replace_step(&self, path: &Path, operation: u64, input: &mut DbIoPages, header: &[u8]) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            let parent = path.parent().ok_or_else(|| DbError::InvalidArgument("DB I/O replacement path has no parent".to_string()))?;
            std::fs::create_dir_all(parent).map_err(io_err)?;
            let name = path.file_name().and_then(std::ffi::OsStr::to_str).ok_or_else(|| DbError::InvalidArgument("DB I/O replacement path is not UTF-8".to_string()))?;
            let temporary = parent.join(format!(".{name}.{operation:016x}.dbio"));
            if !temporary.exists() {
                let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(&temporary).map_err(io_err)?;
                if !header.is_empty() {
                    file.write_all(header).map_err(io_err)?;
                }
            }
            if let Some(fragment) = input.page(0) {
                let fragment_len = fragment.len();
                let mut file = std::fs::OpenOptions::new().append(true).open(&temporary).map_err(io_err)?;
                file.write_all(fragment).map_err(io_err)?;
                input.advance(fragment_len)?;
                if !input.is_empty() {
                    return Ok((DbIoExecutionStep::Yield, None));
                }
            }
            let file = std::fs::OpenOptions::new().write(true).open(&temporary).map_err(io_err)?;
            file.sync_all().map_err(io_err)?;
            std::fs::rename(temporary, path).map_err(io_err)?;
            Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
        }

        fn list_step(&self, dir: &Path, prefix: &str, suffix: &str, list: &mut DbIoU64List) -> Result<bool, DbError> {
            if !dir.exists() {
                return Ok(true);
            }
            let after = list.as_slice().last().copied();
            let mut next = None;
            for entry in std::fs::read_dir(dir).map_err(io_err)? {
                let name = entry.map_err(io_err)?.file_name();
                let name = name.to_string_lossy();
                let Some(number) = name.strip_prefix(prefix).and_then(|rest| rest.strip_suffix(suffix)) else { continue };
                let Ok(value) = number.parse::<u64>() else { continue };
                if after.is_none_or(|after| value > after) && next.is_none_or(|next| value < next) {
                    next = Some(value);
                }
            }
            if let Some(value) = next {
                list.push(value)?;
                Ok(false)
            } else {
                Ok(true)
            }
        }

        fn payload_hash_step(&self, operation: u64, input: &mut DbIoPages) -> Result<Option<ContentHash>, DbError> {
            let slot = operation as usize % self.payload_hashes.len();
            let mut state = self.payload_hashes[slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.as_ref().is_some_and(|(owner, _)| *owner != operation) {
                return Err(DbError::Unavailable("DB I/O filesystem hash cursor capacity exhausted".to_string()));
            }
            let (_, hasher) = state.get_or_insert_with(|| (operation, blake3::Hasher::new()));
            if let Some(fragment) = input.page(0) {
                let len = fragment.len();
                hasher.update(fragment);
                input.advance(len)?;
            }
            if !input.is_empty() {
                return Ok(None);
            }
            let (_, hasher) = state.take().expect("DB I/O payload hash cursor retained");
            Ok(Some(ContentHash(*hasher.finalize().as_bytes())))
        }
    }

    impl DbIoTaskExecutor for FsDbIoExecutor {
        fn execute_step(&self, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            match task {
                DbIoTask::BackendOpen { path, .. } => {
                    if path.as_str() != self.root.as_str() {
                        return Err(DbError::InvalidArgument("DB I/O filesystem root authority mismatch".to_string()));
                    }
                    std::fs::create_dir_all(self.root()).map_err(io_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalCreate { document, index, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    std::fs::create_dir_all(&dir).map_err(io_err)?;
                    let path = segment_path(&dir, *index);
                    if path.exists() {
                        return Err(DbError::AlreadyExists(format!("wal segment {index} for {} already exists", document.as_str())));
                    }
                    std::fs::File::create(path).map_err(io_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalAppend { document, index, input, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    if sealed_marker_path(&dir, *index).exists() {
                        return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
                    }
                    let path = segment_path(&dir, *index);
                    if let Some(fragment) = input.page(0) {
                        let len = fragment.len();
                        let mut file = std::fs::OpenOptions::new().append(true).open(&path).map_err(|error| open_err(error, || format!("wal segment {index} not found")))?;
                        file.write_all(fragment).map_err(io_err)?;
                        input.advance(len)?;
                        if !input.is_empty() {
                            return Ok((DbIoExecutionStep::Yield, None));
                        }
                    }
                    let len = std::fs::metadata(path).map_err(io_err)?.len();
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(len))))
                }
                DbIoTask::WalSync { document, index, class, .. } => {
                    if matches!(class, DurabilityClass::Fsync | DurabilityClass::Quorum(_)) {
                        let path = segment_path(&self.document_dir("wal", document)?, *index);
                        std::fs::OpenOptions::new().write(true).open(path).map_err(io_err)?.sync_all().map_err(io_err)?;
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalSeal { document, index, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    if !segment_path(&dir, *index).exists() {
                        return Err(DbError::NotFound(format!("wal segment {index} not found")));
                    }
                    std::fs::File::create(sealed_marker_path(&dir, *index)).map_err(io_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalRead { document, index, range, output, .. } => {
                    let path = segment_path(&self.document_dir("wal", document)?, *index);
                    self.read_pages_step(operation, &path, range.offset, Some(range.len), output)
                }
                DbIoTask::WalLength { document, index, .. } => {
                    let path = segment_path(&self.document_dir("wal", document)?, *index);
                    let len = std::fs::metadata(path).map_err(|error| open_err(error, || format!("wal segment {index} not found")))?.len();
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(len))))
                }
                DbIoTask::WalList { document, output, .. } => {
                    if self.list_step(&self.document_dir("wal", document)?, "segment-", ".bin", output)? {
                        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::List(std::mem::take(output)))))
                    } else {
                        Ok((DbIoExecutionStep::Yield, None))
                    }
                }
                DbIoTask::WalTruncate { document, index, new_len, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    if sealed_marker_path(&dir, *index).exists() {
                        return Err(DbError::InvalidArgument("cannot truncate sealed wal segment".to_string()));
                    }
                    let path = segment_path(&dir, *index);
                    let file = std::fs::OpenOptions::new().write(true).open(path).map_err(io_err)?;
                    if *new_len > file.metadata().map_err(io_err)?.len() {
                        return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
                    }
                    file.set_len(*new_len).map_err(io_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalDelete { document, index, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    for path in [segment_path(&dir, *index), sealed_marker_path(&dir, *index)] {
                        if path.exists() {
                            std::fs::remove_file(path).map_err(io_err)?;
                        }
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::SnapshotWrite { document, generation, input, .. } => self.replace_step(&generation_path(&self.document_dir("snapshot", document)?, *generation), operation, input, &[]),
                DbIoTask::SnapshotRead { document, generation, output, .. } => self.read_pages_step(operation, &generation_path(&self.document_dir("snapshot", document)?, *generation), 0, None, output),
                DbIoTask::SnapshotLatest { document, output, .. } => {
                    if self.list_step(&self.document_dir("snapshot", document)?, "gen-", ".pack", output)? {
                        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalLength(output.as_slice().last().copied()))))
                    } else {
                        Ok((DbIoExecutionStep::Yield, None))
                    }
                }
                DbIoTask::SnapshotList { document, output, .. } => {
                    if self.list_step(&self.document_dir("snapshot", document)?, "gen-", ".pack", output)? {
                        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::List(std::mem::take(output)))))
                    } else {
                        Ok((DbIoExecutionStep::Yield, None))
                    }
                }
                DbIoTask::SnapshotDelete { document, generation, .. } => {
                    let path = generation_path(&self.document_dir("snapshot", document)?, *generation);
                    if path.exists() {
                        std::fs::remove_file(path).map_err(io_err)?;
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::PayloadPut { input, .. } => {
                    let temporary = self.root().join("payload").join(format!(".{operation:016x}.dbio"));
                    if !temporary.exists() {
                        if let Some(parent) = temporary.parent() {
                            std::fs::create_dir_all(parent).map_err(io_err)?;
                        }
                        std::fs::File::create(&temporary).map_err(io_err)?;
                    }
                    if let Some(fragment) = input.page(0) {
                        let mut file = std::fs::OpenOptions::new().create(true).append(true).open(&temporary).map_err(io_err)?;
                        file.write_all(fragment).map_err(io_err)?;
                    }
                    let Some(hash) = self.payload_hash_step(operation, input)? else { return Ok((DbIoExecutionStep::Yield, None)) };
                    let path = payload_path(self.root(), &hash);
                    if temporary.exists() {
                        if let Some(parent) = path.parent() {
                            std::fs::create_dir_all(parent).map_err(io_err)?;
                        }
                        if !path.exists() {
                            std::fs::rename(temporary, path).map_err(io_err)?;
                        } else {
                            std::fs::remove_file(temporary).map_err(io_err)?;
                        }
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Hash(hash))))
                }
                DbIoTask::PayloadGet { hash, output, .. } => self.read_pages_step(operation, &payload_path(self.root(), hash), 0, None, output),
                DbIoTask::PayloadExists { hash, .. } => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Exists(payload_path(self.root(), hash).exists())))),
                DbIoTask::PayloadLength { hash, .. } => {
                    let len = std::fs::metadata(payload_path(self.root(), hash)).map_err(|error| open_err(error, || format!("payload {hash} not found")))?.len();
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(len))))
                }
                DbIoTask::PayloadDelete { hash, .. } => {
                    let path = payload_path(self.root(), hash);
                    if path.exists() {
                        std::fs::remove_file(path).map_err(io_err)?;
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::CatalogRead { output, .. } => {
                    let path = catalog_path(self.root());
                    if !path.exists() {
                        return Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalCatalog(None))));
                    }
                    let (step, result) = self.read_step(operation, &path, 8, None, true, output)?;
                    Ok((step, result.map(|(pages, fence)| DbIoResult::OptionalCatalog(Some((pages, fence.expect("catalog read cursor retains fence")))))))
                }
                DbIoTask::CatalogCas { expected, input, .. } => {
                    let _guard = self.catalog_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let current = read_root_fence(self.root())?.unwrap_or(EpochFence::INITIAL);
                    expected.check(current)?;
                    let next = expected.next();
                    let (step, result) = self.replace_step(&catalog_path(self.root()), operation, input, &next.epoch.to_le_bytes())?;
                    Ok((step, result.map(|_| DbIoResult::Fence(next))))
                }
                DbIoTask::IndexWrite { document, run_id, input, .. } => self.replace_step(&run_path(&self.document_dir("index", document)?, *run_id), operation, input, &[]),
                DbIoTask::IndexRead { document, run_id, output, .. } => self.read_pages_step(operation, &run_path(&self.document_dir("index", document)?, *run_id), 0, None, output),
                DbIoTask::IndexList { document, output, .. } => {
                    if self.list_step(&self.document_dir("index", document)?, "run-", ".bin", output)? {
                        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::List(std::mem::take(output)))))
                    } else {
                        Ok((DbIoExecutionStep::Yield, None))
                    }
                }
                DbIoTask::IndexDelete { document, run_id, .. } => {
                    let path = run_path(&self.document_dir("index", document)?, *run_id);
                    if path.exists() {
                        std::fs::remove_file(path).map_err(io_err)?;
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::LeaseAcquire { document, holder, now_ms, ttl_ms, .. } => {
                    let _guard = self.lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let path = lease_path(self.root(), document.as_str())?;
                    let fence = match read_lease_file(&path)? {
                        Some((fence, expires, existing)) if *now_ms < expires && existing.as_str() != holder.as_str() => return Err(DbError::Conflict("resource is leased by another holder".to_string())),
                        Some((fence, expires, _)) if *now_ms < expires => fence,
                        Some((fence, _, _)) => fence.next(),
                        None => EpochFence::INITIAL,
                    };
                    write_lease_file(&path, fence, (*now_ms).saturating_add(*ttl_ms), holder.as_str())?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Fence(fence))))
                }
                DbIoTask::LeaseRenew { document, holder, fence, now_ms, ttl_ms, .. } => {
                    let _guard = self.lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let path = lease_path(self.root(), document.as_str())?;
                    let (current, expires, existing) = read_lease_file(&path)?.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
                    if *now_ms >= expires || existing.as_str() != holder.as_str() {
                        return Err(DbError::Unauthorized("lease owner or expiry mismatch".to_string()));
                    }
                    fence.check(current)?;
                    write_lease_file(&path, *fence, (*now_ms).saturating_add(*ttl_ms), holder.as_str())?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::LeaseRelease { document, holder, fence, .. } => {
                    let _guard = self.lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let path = lease_path(self.root(), document.as_str())?;
                    let (current, _, existing) = read_lease_file(&path)?.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
                    if existing.as_str() != holder.as_str() {
                        return Err(DbError::Unauthorized("lease owner mismatch".to_string()));
                    }
                    fence.check(current)?;
                    std::fs::remove_file(path).map_err(io_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::LeaseGet { document, now_ms, .. } => {
                    let lease = read_lease_file(&lease_path(self.root(), document.as_str())?)?;
                    let result = match lease {
                        Some((fence, expires_at_ms, holder)) if *now_ms < expires_at_ms => Some(super::DbIoLeaseResult { resource: document.clone(), holder, fence, expires_at_ms }),
                        _ => None,
                    };
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalLease(result))))
                }
                DbIoTask::BackendClose { .. } => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit))),
            }
        }

        fn close_operation_step(&self, operation: u64, task: &DbIoTask) -> Result<bool, DbError> {
            let read_slot = operation as usize % self.readers.len();
            let mut reader = self.readers[read_slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if reader.as_ref().is_some_and(|owner| owner.operation == operation) {
                reader.take();
                return Ok(false);
            }
            drop(reader);
            let temporary = match task {
                DbIoTask::SnapshotWrite { document, generation, .. } => {
                    let path = generation_path(&self.document_dir("snapshot", document)?, *generation);
                    path.parent().zip(path.file_name().and_then(std::ffi::OsStr::to_str)).map(|(parent, name)| parent.join(format!(".{name}.{operation:016x}.dbio")))
                }
                DbIoTask::IndexWrite { document, run_id, .. } => {
                    let path = run_path(&self.document_dir("index", document)?, *run_id);
                    path.parent().zip(path.file_name().and_then(std::ffi::OsStr::to_str)).map(|(parent, name)| parent.join(format!(".{name}.{operation:016x}.dbio")))
                }
                DbIoTask::CatalogCas { .. } => {
                    let path = catalog_path(self.root());
                    path.parent().zip(path.file_name().and_then(std::ffi::OsStr::to_str)).map(|(parent, name)| parent.join(format!(".{name}.{operation:016x}.dbio")))
                }
                DbIoTask::PayloadPut { .. } => Some(self.root().join("payload").join(format!(".{operation:016x}.dbio"))),
                _ => None,
            };
            if let Some(temporary) = temporary {
                if temporary.exists() {
                    std::fs::remove_file(temporary).map_err(io_err)?;
                    return Ok(false);
                }
            }
            let slot = operation as usize % self.payload_hashes.len();
            let mut state = self.payload_hashes[slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.as_ref().is_some_and(|(owner, _)| *owner == operation) {
                state.take();
                return Ok(false);
            }
            Ok(true)
        }
    }

    async fn execute(pool: &WorkerPool, task: DbIoTask) -> Result<DbIoResult, DbError> {
        submit_db_io_task(pool, task).map_err(|(error, _)| error)?.await.map_err(super::DbIoFault::into_db_error)
    }

    fn document_text(document: &ArtifactId) -> Result<DbIoText, DbError> {
        DbIoText::try_from_str(&document.0)
    }

    fn output_writer(bytes: u64) -> Result<DbIoPageWriter, DbError> {
        let pages = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("db_io output writer bytes"))?.div_ceil(super::DB_IO_PAGE_BYTES);
        DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)
    }

    fn typed_result_fault(expected: &'static str) -> DbError {
        DbError::Internal(format!("DB I/O filesystem executor did not return {expected}"))
    }

    fn unit(result: DbIoResult) -> Result<(), DbError> {
        match result {
            DbIoResult::Unit => Ok(()),
            _ => Err(typed_result_fault("unit")),
        }
    }

    fn length(result: DbIoResult) -> Result<u64, DbError> {
        match result {
            DbIoResult::Length(value) => Ok(value),
            _ => Err(typed_result_fault("length")),
        }
    }

    fn optional_length(result: DbIoResult) -> Result<Option<u64>, DbError> {
        match result {
            DbIoResult::OptionalLength(value) => Ok(value),
            _ => Err(typed_result_fault("optional length")),
        }
    }

    fn pages(result: DbIoResult) -> Result<DbIoPages, DbError> {
        match result {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(typed_result_fault("pages")),
        }
    }

    fn list(result: DbIoResult) -> Result<DbIoU64List, DbError> {
        match result {
            DbIoResult::List(value) => Ok(value),
            _ => Err(typed_result_fault("list")),
        }
    }

    /// @emoji 📁️ The zero-touch default `DbStorage` backend — see module doc for the on-disk
    /// layout. `catalog_lock`/`lease_lock` serialize this-process's compare-and-swap operations;
    /// see `CatalogStorage`/`LeaseStorage` impls below for why a bare read-verify-write over
    /// `write_atomic` isn't itself enough across OS processes (documented extension seam). `pool`
    /// is what every trait method dispatches its blocking body through (see module doc); its
    /// `Arc` is cloned into each method's `'static` blocking closure.
    pub struct FsStorage {
        control: DbIoBackendControl,
        pool: Arc<WorkerPool>,
    }

    impl FsStorage {
        /// @emoji 🚀️ Opens (creating if absent) a `FsStorage` rooted at `root`, dispatching every
        /// subsequent trait call's blocking body through the typed task owner onto `pool`'s
        /// `Lane::Io`. The constructor's directory creation uses that same retained authority;
        /// callers never prepare the root synchronously or through a pool-less fallback.
        pub async fn open(pool: Arc<WorkerPool>, root: &Path) -> Result<Self, DbError> {
            let root = root.to_str().ok_or_else(|| DbError::InvalidArgument("filesystem storage root is not UTF-8".to_string())).and_then(DbIoText::try_from_str)?;
            let control = register_db_io_backend(DbIoBackendKind::Filesystem, Arc::new(FsDbIoExecutor::new(root.clone())))?;
            let task = DbIoTask::BackendOpen { backend: control, path: root };
            if let Err(error) = execute(pool.as_ref(), task).await {
                let _ = execute(pool.as_ref(), DbIoTask::BackendClose { backend: control }).await;
                return Err(error);
            }
            Ok(Self { control, pool })
        }

        pub async fn close(&self) -> Result<(), DbError> {
            match execute(self.pool.as_ref(), DbIoTask::BackendClose { backend: self.control }).await? {
                DbIoResult::Unit => Ok(()),
                _ => Err(DbError::Internal("filesystem backend close returned the wrong typed result".to_string())),
            }
        }

        /// @emoji 🎚️ Always durable, `fsync`-capable, CAS-capable — the on-disk default.
        pub async fn capabilities(&self) -> StorageCapabilities {
            StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true }
        }
    }

    impl WalStorage for FsStorage {
        async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalCreate { backend: self.control, document: document_text(document)?, index }).await?)
        }

        async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
            check_len(bytes.len() as u64, MAX_READ_BYTES, "wal_storage::append")?;
            length(execute(self.pool.as_ref(), DbIoTask::WalAppend { backend: self.control, document: document_text(document)?, index, input: bytes }).await?)
        }

        async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalSync { backend: self.control, document: document_text(document)?, index, class }).await?)
        }

        async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalSeal { backend: self.control, document: document_text(document)?, index }).await?)
        }

        async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
            if let Err(err) = check_len(range.len, MAX_READ_BYTES, "wal_storage::read") {
                return { Err(err) };
            }
            let output = output_writer(range.len)?;
            pages(execute(self.pool.as_ref(), DbIoTask::WalRead { backend: self.control, document: document_text(document)?, index, range, output }).await?)
        }

        async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
            length(execute(self.pool.as_ref(), DbIoTask::WalLength { backend: self.control, document: document_text(document)?, index }).await?)
        }

        async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
            list(execute(self.pool.as_ref(), DbIoTask::WalList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalTruncate { backend: self.control, document: document_text(document)?, index, new_len }).await?)
        }

        async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalDelete { backend: self.control, document: document_text(document)?, index }).await?)
        }
    }

    impl SnapshotStorage for FsStorage {
        async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
            check_len(bytes.len() as u64, MAX_READ_BYTES, "snapshot_storage::write_generation")?;
            unit(execute(self.pool.as_ref(), DbIoTask::SnapshotWrite { backend: self.control, document: document_text(document)?, generation, input: bytes }).await?)
        }

        async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
            pages(execute(self.pool.as_ref(), DbIoTask::SnapshotRead { backend: self.control, document: document_text(document)?, generation, output: output_writer(MAX_READ_BYTES)? }).await?)
        }

        async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
            optional_length(execute(self.pool.as_ref(), DbIoTask::SnapshotLatest { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
            list(execute(self.pool.as_ref(), DbIoTask::SnapshotList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::SnapshotDelete { backend: self.control, document: document_text(document)?, generation }).await?)
        }
    }

    impl PayloadStorage for FsStorage {
        async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
            if let Err(err) = check_len(bytes.len() as u64, MAX_READ_BYTES, "payload_storage::put") {
                return { Err(err) };
            }
            match execute(self.pool.as_ref(), DbIoTask::PayloadPut { backend: self.control, input: bytes }).await? {
                DbIoResult::Hash(hash) => Ok(hash),
                _ => Err(typed_result_fault("content hash")),
            }
        }

        async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
            pages(execute(self.pool.as_ref(), DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: output_writer(MAX_READ_BYTES)? }).await?)
        }

        async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
            match execute(self.pool.as_ref(), DbIoTask::PayloadExists { backend: self.control, hash: *hash }).await? {
                DbIoResult::Exists(exists) => Ok(exists),
                _ => Err(typed_result_fault("existence")),
            }
        }

        async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::PayloadDelete { backend: self.control, hash: *hash }).await?)
        }

        async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
            length(execute(self.pool.as_ref(), DbIoTask::PayloadLength { backend: self.control, hash: *hash }).await?)
        }
    }

    impl CatalogStorage for FsStorage {
        async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
            match execute(self.pool.as_ref(), DbIoTask::CatalogRead { backend: self.control, output: output_writer(MAX_READ_BYTES)? }).await? {
                DbIoResult::OptionalCatalog(root) => Ok(root),
                _ => Err(typed_result_fault("optional catalog root")),
            }
        }

        async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
            if let Err(err) = check_len(new_bytes.len() as u64, MAX_READ_BYTES, "catalog_storage::cas_root") {
                return { Err(err) };
            }
            match execute(self.pool.as_ref(), DbIoTask::CatalogCas { backend: self.control, expected, input: new_bytes }).await? {
                DbIoResult::Fence(fence) => Ok(fence),
                _ => Err(typed_result_fault("catalog fence")),
            }
        }
    }

    /// @emoji 📖️ The blocking body behind `CatalogStorage::read_root` — factored out so
    /// `cas_root` can reuse it under `catalog_lock` without recursive task submission.
    // 🚫️async: E1 pure-shaped typed-task accessor.
    fn read_root_fence(root: &Path) -> Result<Option<EpochFence>, DbError> {
        let path = catalog_path(root);
        if !path.exists() {
            return Ok(None);
        }
        let metadata = std::fs::metadata(&path).map_err(io_err)?;
        if metadata.len() < 8 {
            return Err(DbError::Corrupt("catalog root file is shorter than its 8-byte epoch header".to_string()));
        }
        let mut epoch_bytes = [0u8; 8];
        std::fs::File::open(&path).map_err(io_err)?.read_exact(&mut epoch_bytes).map_err(io_err)?;
        Ok(Some(EpochFence { epoch: u64::from_le_bytes(epoch_bytes) }))
    }

    impl IndexStorage for FsStorage {
        async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
            check_len(bytes.len() as u64, MAX_READ_BYTES, "index_storage::write_run")?;
            unit(execute(self.pool.as_ref(), DbIoTask::IndexWrite { backend: self.control, document: document_text(document)?, run_id, input: bytes }).await?)
        }

        async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
            pages(execute(self.pool.as_ref(), DbIoTask::IndexRead { backend: self.control, document: document_text(document)?, run_id, output: output_writer(MAX_READ_BYTES)? }).await?)
        }

        async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
            list(execute(self.pool.as_ref(), DbIoTask::IndexList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::IndexDelete { backend: self.control, document: document_text(document)?, run_id }).await?)
        }
    }

    impl LeaseStorage for FsStorage {
        async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
            match execute(self.pool.as_ref(), DbIoTask::LeaseAcquire { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, now_ms, ttl_ms }).await? {
                DbIoResult::Fence(fence) => Ok(fence),
                _ => Err(typed_result_fault("lease fence")),
            }
        }

        async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::LeaseRenew { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence, now_ms, ttl_ms }).await?)
        }

        async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::LeaseRelease { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence }).await?)
        }

        async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
            match execute(self.pool.as_ref(), DbIoTask::LeaseGet { backend: self.control, document: DbIoText::try_from_str(resource)?, now_ms }).await? {
                DbIoResult::OptionalLease(Some(lease)) => Ok(Some(LeaseInfo { resource: lease.resource.as_str().to_string(), holder: lease.holder.as_str().to_string(), fence: lease.fence, expires_at_ms: lease.expires_at_ms })),
                DbIoResult::OptionalLease(None) => Ok(None),
                _ => Err(typed_result_fault("optional lease")),
            }
        }
    }
}

#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
pub use fs_storage::FsStorage;
//#endregion 🔖️Fs


#[cfg(test)]
mod db_io_retained_fixtures {
    use super::*;

    static FIXTURE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn pages(bytes: &[u8]) -> DbIoPages {
        let mut writer = DbIoPageWriter::try_reserve(bytes.len().div_ceil(DB_IO_PAGE_BYTES)).expect("fixture pages admitted");
        for fragment in bytes.chunks(DB_IO_PAGE_BYTES) {
            assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
        }
        writer.finish().unwrap()
    }

    fn drain_pages(mut pages: DbIoPages) {
        while pages.close_step().unwrap().is_some() {}
        while db_io_page_maintenance_step().unwrap().is_some() {}
    }

    #[test]
    fn db_io_fixed_page_max_plus_one_and_zero_are_exact() {
        let _serial = FIXTURE_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let empty = pages(&[]);
        assert!(empty.is_empty());
        drain_pages(empty);
        let max = vec![0x5a; DB_IO_OPERATION_BYTES as usize];
        let retained = pages(&max);
        assert_eq!(retained.len(), max.len());
        assert_eq!(usize::from(retained.page_count()), DB_IO_OPERATION_PAGES);
        assert!(DbIoPageWriter::try_reserve(DB_IO_OPERATION_PAGES + 1).is_err());
        drain_pages(retained);
    }

    #[test]
    fn db_io_range_moves_the_same_page_leases_without_suffix_copy() {
        let _serial = FIXTURE_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let bytes = vec![0x33; DB_IO_PAGE_BYTES + 3];
        let owner = pages(&bytes);
        let operation = owner.operation();
        let suffix = owner.try_range(DB_IO_PAGE_BYTES).unwrap();
        assert_eq!(suffix.operation(), operation);
        assert_eq!(suffix, [0x33; 3]);
        drain_pages(suffix);
    }

    #[test]
    fn db_io_list_capacity_plus_one_does_not_mutate_the_fixed_owner() {
        let mut list = DbIoU64List::new();
        for value in 0..DB_IO_LIST_ITEMS as u64 {
            list.push(value).unwrap();
        }
        assert!(list.push(DB_IO_LIST_ITEMS as u64).is_err());
        assert_eq!(list.len(), DB_IO_LIST_ITEMS);
        assert_eq!(list.as_slice().last(), Some(&((DB_IO_LIST_ITEMS - 1) as u64)));
    }

    #[test]
    fn db_io_process_page_max_plus_one_preflight_is_atomic() {
        let mut state = DbIoPageArenaState::new();
        for _ in 0..DB_IO_TOTAL_PAGES / DB_IO_OPERATION_PAGES {
            db_io_preflight_page_checkout(&state, DB_IO_OPERATION_PAGES).unwrap();
            state.free_len -= DB_IO_OPERATION_PAGES;
        }
        let before = (state.free_read, state.free_len, state.next_generation);
        assert!(db_io_preflight_page_checkout(&state, 1).is_err());
        assert_eq!((state.free_read, state.free_len, state.next_generation), before);
    }

    #[test]
    fn db_io_result_page_reservation_plus_one_returns_the_writer() {
        let _serial = FIXTURE_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut writer = DbIoPageWriter::try_reserve(1).unwrap();
        assert_eq!(writer.write_fragment(&[0x44; DB_IO_PAGE_BYTES]).unwrap(), DB_IO_PAGE_BYTES);
        assert!(matches!(writer.write_fragment(&[0x55]), Err(DbError::LimitExceeded(_))));
        assert_eq!(writer.len(), DB_IO_PAGE_BYTES);
        assert!(writer.close_step().unwrap().is_some());
        assert!(writer.terminal_is_empty());
    }

    #[test]
    fn db_io_interrupted_close_retires_one_page_or_owner_per_grant() {
        let _serial = FIXTURE_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let input = pages(&[0x66; DB_IO_PAGE_BYTES + 1]);
        let mut task = DbIoTask::WalAppend {
            backend: DbIoBackendControl::Memory { slot: 0, generation: 1 },
            document: DbIoText::try_from_str("close-fixture").unwrap(),
            index: 0,
            input,
        };
        assert_eq!(task.close_step().unwrap(), Some(DB_IO_PAGE_BYTES));
        assert!(!task.terminal_is_empty());
        assert_eq!(task.close_step().unwrap(), Some(DB_IO_PAGE_BYTES));
        assert!(!task.terminal_is_empty());
        assert_eq!(task.close_step().unwrap(), Some(0));
        assert!(task.terminal_is_empty());
        assert_eq!(task.close_step().unwrap(), None);
    }

    #[test]
    fn db_io_retry_generation_and_operation_reject_stale_aba_handles() {
        let current = DbIoTaskSlot { generation: 8, operation: 12, ..DbIoTaskSlot::empty() };
        assert!(db_io_slot_matches(&current, DbIoTaskHandle { slot: 0, generation: 8, operation: 12 }));
        assert!(!db_io_slot_matches(&current, DbIoTaskHandle { slot: 0, generation: 7, operation: 12 }));
        assert!(!db_io_slot_matches(&current, DbIoTaskHandle { slot: 0, generation: 8, operation: 11 }));
    }

    #[test]
    fn db_io_lost_page_handle_resumes_the_same_retirement_cursor() {
        let _serial = FIXTURE_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = pages(&[0x77; DB_IO_PAGE_BYTES + 1]);
        let operation = owner.operation();
        drop(owner);
        assert_eq!(db_io_page_maintenance_step().unwrap(), Some(DB_IO_PAGE_BYTES));
        assert_eq!(db_io_page_maintenance_step().unwrap(), Some(DB_IO_PAGE_BYTES));
        assert_eq!(db_io_page_maintenance_step().unwrap(), None);
        assert_ne!(operation, 0);
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn pages(bytes: &[u8]) -> DbIoPages {
        let mut writer = DbIoPageWriter::try_reserve(bytes.len().div_ceil(DB_IO_PAGE_BYTES)).expect("test storage pages admitted");
        for fragment in bytes.chunks(DB_IO_PAGE_BYTES) {
            assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
        }
        writer.finish().unwrap()
    }

    //#region 🔖️WalStorage
    async fn exercise_wal_storage(storage: &impl WalStorage) {
        let document: ArtifactId = "doc-wal".into();

        block_on_ready(storage.create_segment(&document, 0)).await.unwrap();
        assert!(matches!(block_on_ready(storage.create_segment(&document, 0)).await, Err(DbError::AlreadyExists(_))));

        let len_after_first = block_on_ready(storage.append(&document, 0, pages(b"hello "))).await.unwrap();
        assert_eq!(len_after_first, 6);
        let len_after_second = block_on_ready(storage.append(&document, 0, pages(b"world"))).await.unwrap();
        assert_eq!(len_after_second, 11);
        assert_eq!(block_on_ready(storage.segment_len(&document, 0)).await.unwrap(), 11);

        let read_back = block_on_ready(storage.read(&document, 0, ByteRange { offset: 6, len: 5 })).await.unwrap();
        assert_eq!(read_back, b"world");
        assert!(matches!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 6, len: 100 })).await, Err(DbError::InvalidArgument(_))));

        block_on_ready(storage.sync(&document, 0, DurabilityClass::Fsync)).await.unwrap();

        block_on_ready(storage.truncate_tail(&document, 0, 6)).await.unwrap();
        assert_eq!(block_on_ready(storage.segment_len(&document, 0)).await.unwrap(), 6);
        assert_eq!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 0, len: 6 })).await.unwrap(), b"hello ");

        block_on_ready(storage.create_segment(&document, 1)).await.unwrap();
        assert_eq!(block_on_ready(storage.list_segments(&document)).await.unwrap(), vec![0, 1]);

        block_on_ready(storage.seal(&document, 0)).await.unwrap();
        assert!(matches!(block_on_ready(storage.append(&document, 0, pages(b"!"))).await, Err(DbError::InvalidArgument(_))));
        assert!(matches!(block_on_ready(storage.truncate_tail(&document, 0, 0)).await, Err(DbError::InvalidArgument(_))));

        block_on_ready(storage.delete_segment(&document, 1)).await.unwrap();
        assert_eq!(block_on_ready(storage.list_segments(&document)).await.unwrap(), vec![0]);

        assert!(matches!(block_on_ready(storage.append(&document, 99, pages(b"x"))).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_wal_storage_laws() {
        exercise_wal_storage(&MemoryStorage::new().await).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_wal_storage_laws() {
        exercise_wal_storage(&fs_scratch("wal_laws").await).await;
    }
    //#endregion 🔖️WalStorage

    //#region 🔖️SnapshotStorage
    async fn exercise_snapshot_storage(storage: &impl SnapshotStorage) {
        let document: ArtifactId = "doc-snap".into();
        assert_eq!(block_on_ready(storage.latest_generation(&document)).await.unwrap(), None);

        block_on_ready(storage.write_generation(&document, 0, pages(b"gen-zero-bytes"))).await.unwrap();
        block_on_ready(storage.write_generation(&document, 1, pages(b"gen-one-bytes"))).await.unwrap();
        assert_eq!(block_on_ready(storage.list_generations(&document)).await.unwrap(), vec![0, 1]);
        assert_eq!(block_on_ready(storage.latest_generation(&document)).await.unwrap(), Some(1));
        assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"gen-zero-bytes");

        block_on_ready(storage.write_generation(&document, 0, pages(b"gen-zero-overwritten"))).await.unwrap();
        assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"gen-zero-overwritten");

        block_on_ready(storage.delete_generation(&document, 0)).await.unwrap();
        assert!(matches!(block_on_ready(storage.read_generation(&document, 0)).await, Err(DbError::NotFound(_))));
        assert_eq!(block_on_ready(storage.list_generations(&document)).await.unwrap(), vec![1]);
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_snapshot_storage_laws() {
        exercise_snapshot_storage(&MemoryStorage::new().await).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_snapshot_storage_laws() {
        exercise_snapshot_storage(&fs_scratch("snapshot_laws").await).await;
    }
    //#endregion 🔖️SnapshotStorage

    //#region 🔖️PayloadStorage
    async fn exercise_payload_storage(storage: &impl PayloadStorage) {
        let bytes = b"a payload blob that gets content-addressed";
        let hash_a = block_on_ready(storage.put(pages(bytes))).await.unwrap();
        let hash_b = block_on_ready(storage.put(pages(bytes))).await.unwrap();
        assert_eq!(hash_a, hash_b, "put is idempotent under content equality");
        assert_eq!(hash_a, ContentHash(*blake3::hash(bytes).as_bytes()));

        assert!(block_on_ready(storage.contains(&hash_a)).await.unwrap());
        assert_eq!(block_on_ready(storage.get(&hash_a)).await.unwrap(), bytes);
        assert_eq!(block_on_ready(storage.len(&hash_a)).await.unwrap(), bytes.len() as u64);

        let other_hash = ContentHash([0xAB; 32]);
        assert!(!block_on_ready(storage.contains(&other_hash)).await.unwrap());
        assert!(matches!(block_on_ready(storage.get(&other_hash)).await, Err(DbError::NotFound(_))));

        block_on_ready(storage.delete(&hash_a)).await.unwrap();
        assert!(!block_on_ready(storage.contains(&hash_a)).await.unwrap());
        assert!(matches!(block_on_ready(storage.get(&hash_a)).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_payload_storage_laws() {
        exercise_payload_storage(&MemoryStorage::new().await).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_payload_storage_laws() {
        exercise_payload_storage(&fs_scratch("payload_laws").await).await;
    }
    //#endregion 🔖️PayloadStorage

    //#region 🔖️CatalogStorage
    async fn exercise_catalog_storage(storage: &impl CatalogStorage) {
        assert_eq!(block_on_ready(storage.read_root()).await.unwrap(), None);

        let epoch_1 = block_on_ready(storage.cas_root(EpochFence::INITIAL, pages(b"root-v1"))).await.unwrap();
        assert_eq!(epoch_1, EpochFence::INITIAL.next());
        let (bytes, fence) = block_on_ready(storage.read_root()).await.unwrap().unwrap();
        assert_eq!(bytes, b"root-v1");
        assert_eq!(fence, epoch_1);

        // A stale `expected` (still `INITIAL`, but the root already moved to `epoch_1`) is fenced.
        assert!(matches!(block_on_ready(storage.cas_root(EpochFence::INITIAL, pages(b"root-stale"))).await, Err(DbError::Fenced { .. })));

        let epoch_2 = block_on_ready(storage.cas_root(epoch_1, pages(b"root-v2"))).await.unwrap();
        assert_eq!(epoch_2, epoch_1.next());
        assert_eq!(block_on_ready(storage.read_root()).await.unwrap().unwrap().0, b"root-v2");
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_catalog_storage_laws() {
        exercise_catalog_storage(&MemoryStorage::new().await).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_catalog_storage_laws() {
        exercise_catalog_storage(&fs_scratch("catalog_laws").await).await;
    }
    //#endregion 🔖️CatalogStorage

    //#region 🔖️IndexStorage
    async fn exercise_index_storage(storage: &impl IndexStorage) {
        let document: ArtifactId = "doc-index".into();
        block_on_ready(storage.write_run(&document, 0, pages(b"run-zero"))).await.unwrap();
        block_on_ready(storage.write_run(&document, 1, pages(b"run-one"))).await.unwrap();
        assert_eq!(block_on_ready(storage.list_runs(&document)).await.unwrap(), vec![0, 1]);
        assert_eq!(block_on_ready(storage.read_run(&document, 1)).await.unwrap(), b"run-one");

        block_on_ready(storage.delete_run(&document, 0)).await.unwrap();
        assert_eq!(block_on_ready(storage.list_runs(&document)).await.unwrap(), vec![1]);
        assert!(matches!(block_on_ready(storage.read_run(&document, 0)).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_index_storage_laws() {
        exercise_index_storage(&MemoryStorage::new().await).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_index_storage_laws() {
        exercise_index_storage(&fs_scratch("index_laws").await).await;
    }
    //#endregion 🔖️IndexStorage

    //#region 🔖️LeaseStorage
    async fn exercise_lease_storage(storage: &impl LeaseStorage) {
        let fence_1 = block_on_ready(storage.acquire("shard-1", "node-a", 1_000, 0)).await.unwrap();
        assert_eq!(fence_1, EpochFence::INITIAL);

        // Re-acquiring the same, unexpired lease by the same holder is idempotent (same fence).
        let fence_reacquire = block_on_ready(storage.acquire("shard-1", "node-a", 1_000, 100)).await.unwrap();
        assert_eq!(fence_reacquire, fence_1);

        // A different holder cannot acquire an unexpired lease.
        assert!(matches!(block_on_ready(storage.acquire("shard-1", "node-b", 1_000, 100)).await, Err(DbError::Conflict(_))));

        block_on_ready(storage.renew("shard-1", "node-a", fence_1, 1_000, 500)).await.unwrap();
        assert!(matches!(block_on_ready(storage.renew("shard-1", "node-a", fence_1.next(), 1_000, 500)).await, Err(DbError::Fenced { .. })));
        assert!(matches!(block_on_ready(storage.renew("shard-1", "node-b", fence_1, 1_000, 500)).await, Err(DbError::Unauthorized(_))));

        let current = block_on_ready(storage.current("shard-1", 600)).await.unwrap().unwrap();
        assert_eq!(current.holder, "node-a");
        assert_eq!(current.fence, fence_1);

        // After expiry (renewed at 500 for 1_000ms => expires at 1_500), a different holder can
        // take over, bumping the fence — the fencing law a stale former holder is later rejected by.
        assert_eq!(block_on_ready(storage.current("shard-1", 2_000)).await.unwrap(), None);
        let fence_2 = block_on_ready(storage.acquire("shard-1", "node-b", 1_000, 2_000)).await.unwrap();
        assert_eq!(fence_2, fence_1.next());

        // The old holder's stale fence is now rejected.
        assert!(matches!(block_on_ready(storage.renew("shard-1", "node-a", fence_1, 1_000, 2_100)).await, Err(DbError::Unauthorized(_))));

        block_on_ready(storage.release("shard-1", "node-b", fence_2)).await.unwrap();
        assert_eq!(block_on_ready(storage.current("shard-1", 2_100)).await.unwrap(), None);
        assert!(matches!(block_on_ready(storage.release("shard-1", "node-b", fence_2)).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_lease_storage_laws() {
        exercise_lease_storage(&MemoryStorage::new().await).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_lease_storage_laws() {
        exercise_lease_storage(&fs_scratch("lease_laws").await).await;
    }
    //#endregion 🔖️LeaseStorage

    //#region 🔖️DbBackend
    #[semio_framework_async_macros::async_test]
    async fn memory_storage_db_backend_accessors_and_capabilities() {
        let storage: DbBackend = DbBackend::Memory(MemoryStorage::new().await);
        let document: ArtifactId = "doc-umbrella".into();
        block_on_ready(poll_once(storage.wal()).await.create_segment(&document, 0)).await.unwrap();
        block_on_ready(poll_once(storage.catalog()).await.cas_root(EpochFence::INITIAL, pages(b"root"))).await.unwrap();

        let capabilities = poll_once(storage.capabilities()).await;
        assert!(!capabilities.durable);
        assert_eq!(capabilities.max_durability, DurabilityClass::Memory);
        assert!(capabilities.supports_cas);
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_db_backend_accessors_and_capabilities() {
        let storage: DbBackend = DbBackend::Fs(fs_scratch("umbrella").await);
        let document: ArtifactId = "doc-umbrella".into();
        block_on_ready(poll_once(storage.index()).await.write_run(&document, 0, pages(b"run"))).await.unwrap();
        assert_eq!(block_on_ready(poll_once(storage.index()).await.read_run(&document, 0)).await.unwrap(), b"run");

        let capabilities = poll_once(storage.capabilities()).await;
        assert!(capabilities.durable);
        assert_eq!(capabilities.max_durability, DurabilityClass::Fsync);
        assert!(capabilities.supports_fsync);
    }
    //#endregion 🔖️DbBackend

    //#region 🔖️Fs
    #[cfg(feature = "fs")]
    static SCRATCH_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    /// @emoji 🎲️ A fresh `FsStorage` rooted at a unique scratch directory under
    /// `std::env::temp_dir()` — no external `tempfile` crate dependency, mirroring `pack_io`'s own
    /// test helper convention. The test-owned process pool drives the same retained operation path.
    #[cfg(feature = "fs")]
    async fn fs_scratch(name: &str) -> FsStorage {
        let pid = std::process::id();
        let counter = SCRATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("db_storage_test_{name}_{pid}_{counter}"));
        poll_once(FsStorage::open(db_io_test_pool(), &dir)).await.unwrap()
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_rejects_unsafe_path_components() {
        let storage = fs_scratch("path_safety").await;
        let traversal_document: ArtifactId = "../escape".into();
        assert!(matches!(block_on_ready(storage.create_segment(&traversal_document, 0)).await, Err(DbError::InvalidArgument(_))));

        let separator_document: ArtifactId = "sub/dir".into();
        assert!(matches!(block_on_ready(storage.create_segment(&separator_document, 0)).await, Err(DbError::InvalidArgument(_))));

        let empty_document: ArtifactId = "".into();
        assert!(matches!(block_on_ready(storage.create_segment(&empty_document, 0)).await, Err(DbError::InvalidArgument(_))));
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_write_atomic_survives_reopen_across_instances() {
        let pid = std::process::id();
        let counter = SCRATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("db_storage_test_reopen_{pid}_{counter}"));

        {
            let storage = poll_once(FsStorage::open(db_io_test_pool(), &dir)).await.unwrap();
            let document: ArtifactId = "doc-reopen".into();
            block_on_ready(storage.write_generation(&document, 0, pages(b"persisted across reopen"))).await.unwrap();
        }
        {
            let storage = poll_once(FsStorage::open(db_io_test_pool(), &dir)).await.unwrap();
            let document: ArtifactId = "doc-reopen".into();
            assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"persisted across reopen");
        }
    }
    //#endregion 🔖️Fs
}
//#endregion 🧪️Tests
