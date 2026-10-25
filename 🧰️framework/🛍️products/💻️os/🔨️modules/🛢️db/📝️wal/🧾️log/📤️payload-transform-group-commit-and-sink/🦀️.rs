//! 📤️ Payload transform, group commit and sink: the payload transform, the group commit policy, the tail window and the shared buffer sink. See [`super`].

use super::*;

//#region 🔖️PayloadTransform
/// 🔐️ The encryption hook for `WAL_PAYLOAD` bytes: a caller building a `WalPayloadRef`
/// applies `encrypt` to the plaintext BEFORE wrapping it as `WalPayloadRef::Inline` or handing it to
/// `db_storage::PayloadStorage::put` for a `WalPayloadRef::CasRef`, and applies `decrypt` after
/// reading either form back — this crate itself never calls `PayloadStorage` (see `WalPayloadRef`'s
/// doc: the inline-vs-CAS choice is deliberately the caller's, keeping `db_wal` decoupled from that
/// trait), so it cannot invoke the hook itself either; it only defines the seam. Per the contract:
/// a trait only, no real implementation here — external crypto libraries stay behind it (the
/// family's "external libs behind an interface" rule).
pub trait PayloadTransform: Send + Sync {
    /// 🔒️ Transforms `plaintext` before it is embedded inline or stored via `PayloadStorage`.
    async fn encrypt(&self, plaintext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError>;
    /// 🔓️ Inverts `encrypt` — must exactly reconstruct the original bytes.
    async fn decrypt(&self, ciphertext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError>;
}

/// 🪟️ A `PayloadTransform` that passes bytes through unchanged — the default for a
/// deployment with no encryption configured.
#[derive(Clone, Copy, Default, Debug)]
pub struct IdentityPayloadTransform;

impl PayloadTransform for IdentityPayloadTransform {
    async fn encrypt(&self, plaintext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError> {
        control.grant()?;
        Ok(plaintext)
    }

    async fn decrypt(&self, ciphertext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError> {
        control.grant()?;
        Ok(ciphertext)
    }
}
//#endregion 🔖️PayloadTransform

//#region 🔖️GroupCommit
/// ⏱️ Bounds a segment's group-commit batching: whichever of delay/bytes/records is hit
/// first triggers the next physical `SprWriter::commit()` + `WalStorage::sync`. This crate's own
/// choice of defaults (the contract fixes the mechanism, not the numbers) — 20ms/256KiB/256
/// records amortizes fsync cost under load while keeping worst-case latency low for a single
/// isolated command.
#[derive(Clone, Copy, Debug)]
pub struct GroupCommitPolicy {
    pub max_delay_ms: u64,
    pub max_bytes: u64,
    pub max_records: u32,
}

impl Default for GroupCommitPolicy {
    fn default() -> Self {
        Self { max_delay_ms: 20, max_bytes: 256 * 1024, max_records: 256 }
    }
}

impl GroupCommitPolicy {
    /// ⏰️ True iff any bound is currently exceeded — `ArtifactWal::submit` also always
    /// commits immediately regardless of this policy when the caller requests `Fsync`/`Quorum`
    /// durability (a durability request stronger than what's already durable can never be
    /// satisfied by deferring the commit).
    pub fn is_due(&self, pending_bytes: u64, pending_records: u32, oldest_pending_at_ms: Option<u64>, now_ms: u64) -> bool {
        pending_bytes >= self.max_bytes || pending_records >= self.max_records || oldest_pending_at_ms.is_some_and(|started| now_ms.saturating_sub(started) >= self.max_delay_ms)
    }
}
//#endregion 🔖️GroupCommit

//#region 🔖️Sink
/// 📐️ Pages one active segment's tail window may ever hold: the window never outgrows the
/// readable segment it belongs to.
const WAL_TAIL_WINDOW_PAGES: usize = (db_io_core::DB_IO_MAX_READ_BYTES as usize).div_ceil(db_io_core::DB_IO_PAGE_BYTES);

/// 🔢️ Writers one tail window may chain under its one I/O operation. Each new writer is at
/// least as large as the whole window before it, so this many always reach `WAL_TAIL_WINDOW_PAGES`.
const WAL_TAIL_WINDOW_WRITERS: usize = 6;

const _: () = assert!(1 << (WAL_TAIL_WINDOW_WRITERS - 1) >= WAL_TAIL_WINDOW_PAGES);

/// 🪟️ The unflushed tail of one active WAL segment, held in fixed DB I/O pages only while
/// records wait for their commit and flush. `base` is the absolute segment offset of the window's
/// first byte: everything before it is durable in `WalStorage` and no longer in memory, so an idle
/// document holds no I/O pages at all. Holding the whole segment image instead charged every mounted
/// document a full operation's pages for as long as it stayed mounted, and a process could not mount
/// its sixteenth document.
struct WalTailWindow {
    pub base: u64,
    writers: [Option<db_io_core::DbIoPageWriter>; WAL_TAIL_WINDOW_WRITERS],
    capacities: [usize; WAL_TAIL_WINDOW_WRITERS],
}

impl WalTailWindow {
    pub fn at(base: u64) -> Self {
        Self { base, writers: std::array::from_fn(|_| None), capacities: [0; WAL_TAIL_WINDOW_WRITERS] }
    }

    pub(super) fn written(&self) -> usize {
        self.writers.iter().flatten().map(db_io_core::DbIoPageWriter::len).sum()
    }

    pub fn len(&self) -> u64 {
        self.base + self.written() as u64
    }

    pub fn admit(&mut self, bytes: u64) -> Result<(), DbError> {
        let bytes = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("wal tail window admission"))?;
        let reserved = self.capacities.iter().sum::<usize>();
        let free = (reserved * db_io_core::DB_IO_PAGE_BYTES).checked_sub(self.written()).ok_or_else(|| DbError::Internal("WAL tail window wrote past its pages".to_string()))?;
        if bytes <= free {
            return Ok(());
        }
        let needed = (bytes - free).div_ceil(db_io_core::DB_IO_PAGE_BYTES);
        let pages = needed.max(reserved).min(WAL_TAIL_WINDOW_PAGES.saturating_sub(reserved));
        let slot = self.writers.iter().position(Option::is_none);
        let Some(slot) = slot.filter(|_| pages >= needed) else { return Err(DbError::LimitExceeded("wal tail window")) };
        let writer = match self.writers.iter().flatten().next() {
            Some(first) => db_io_core::DbIoPageWriter::try_reserve_for_operation(first.operation(), pages),
            None => db_io_core::DbIoPageWriter::try_reserve(pages),
        }
        .map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        self.writers[slot] = Some(writer);
        self.capacities[slot] = pages;
        Ok(())
    }

    fn write_step(&mut self, bytes: &[u8]) -> Result<usize, DbError> {
        for (writer, capacity) in self.writers.iter_mut().zip(self.capacities) {
            let Some(writer) = writer.as_mut() else { break };
            if writer.len() < capacity * db_io_core::DB_IO_PAGE_BYTES {
                return writer.write_fragment(bytes);
            }
        }
        Err(DbError::LimitExceeded("wal tail window admission"))
    }

    fn read_step(&self, offset: u64, output: &mut [u8]) -> Result<usize, DbError> {
        let relative = offset.checked_sub(self.base).ok_or_else(|| DbError::Internal("WAL tail window no longer holds a flushed offset".to_string()))?;
        let mut relative = usize::try_from(relative).map_err(|_| DbError::LimitExceeded("wal tail window offset"))?;
        for writer in self.writers.iter().flatten() {
            if relative < writer.len() {
                return writer.read_fragment(relative, output);
            }
            relative -= writer.len();
        }
        Ok(0)
    }

    pub fn release_step(&mut self) -> Result<bool, DbError> {
        for writer in self.writers.iter_mut().rev().flatten() {
            if writer.close_step()?.is_some() {
                return Ok(true);
            }
        }
        self.base = self.len();
        self.writers = std::array::from_fn(|_| None);
        self.capacities = [0; WAL_TAIL_WINDOW_WRITERS];
        Ok(false)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.writers.iter().flatten().all(db_io_core::DbIoPageWriter::terminal_is_empty)
    }
}

/// 🪞️ A `semio_framework_pack_source::PackSink` over one segment's `WalTailWindow`. `protocol::SprWriter` owns one
/// clone and `SegmentWriter` another to flush the unflushed suffix to `db_storage::WalStorage` —
/// `SprWriter` has no public accessor for its private `sink` field. Every write is admitted first
/// (`admit`), so a full process budget refuses a transaction before any of its bytes exist.
#[derive(Clone)]
pub struct SharedBuf(pub std::sync::Arc<std::sync::Mutex<WalTailWindow>>);

/// 🩹️ Recovers a poisoned lock instead of panicking — one panicking document actor must
/// never turn every other document's WAL access into a cascading panic (mirrors `db_storage`'s
/// own `MemoryStorage` convention).
// 🚫️async: E1 pure accessor (no suspension: `Mutex::lock` on a never-genuinely-contended
// in-process lock), consumed synchronously alongside `.await`ed uses elsewhere — see R9
fn lock<T>(mutex: &std::sync::Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl SharedBuf {
    /// 📍️ An empty window whose next byte lands at absolute segment offset `base`.
    pub fn at(base: u64) -> Self {
        Self(std::sync::Arc::new(std::sync::Mutex::new(WalTailWindow::at(base))))
    }

    pub fn len(&self) -> u64 {
        lock(&self.0).len()
    }

    /// 🎟️ Ensures the next `bytes` written fit the window's pages, growing it by one writer
    /// under the window's own I/O operation when they do not.
    pub fn admit(&self, bytes: u64) -> Result<(), DbError> {
        lock(&self.0).admit(bytes)
    }

    /// 🧺️ Returns one flushed page (or writer shell) to the process budget; `false` once the
    /// window is empty again and rebased past everything it held.
    pub fn release_step(&self) -> Result<bool, DbError> {
        lock(&self.0).release_step()
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if std::sync::Arc::strong_count(&self.0) != 1 {
            return Err(DbError::Internal("WAL retained buffer still has a writer clone".to_string()));
        }
        self.release_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        std::sync::Arc::strong_count(&self.0) == 1 && lock(&self.0).terminal_is_empty()
    }

    pub async fn copy_range(&self, offset: usize, len: usize) -> Result<db_io_core::DbIoPages, DbError> {
        let mut output = db_io_core::DbIoPageWriter::try_reserve(len.div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        let mut copied = 0;
        let mut fragment = [0u8; db_io_core::DB_IO_PAGE_BYTES];
        while copied < len {
            let requested = (len - copied).min(fragment.len());
            let read = lock(&self.0).read_step((offset + copied) as u64, &mut fragment[..requested])?;
            if read == 0 {
                return Err(DbError::Corrupt("WAL retained page range ended early".to_string()));
            }
            let mut written = 0;
            while written < read {
                let count = output.write_fragment(&fragment[written..read])?;
                if count == 0 {
                    return Err(DbError::LimitExceeded("WAL retained suffix writer"));
                }
                written += count;
                semio_framework_async::yield_once().await;
            }
            copied += read;
            semio_framework_async::yield_once().await;
        }
        output.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)
    }

    pub async fn read_exact(&self, offset: usize, output: &mut [u8]) -> Result<(), DbError> {
        let mut copied = 0;
        while copied < output.len() {
            let read = lock(&self.0).read_step((offset + copied) as u64, &mut output[copied..])?;
            if read == 0 {
                return Err(DbError::Corrupt("WAL retained page read ended early".to_string()));
            }
            copied += read;
            semio_framework_async::yield_once().await;
        }
        Ok(())
    }
}

impl semio_framework_pack_source::PackSink for SharedBuf {
    type Error = DbError;
    async fn write_all(&mut self, bytes: &[u8]) -> Result<(), DbError> {
        let mut cursor = 0;
        while cursor < bytes.len() {
            let written = lock(&self.0).write_step(&bytes[cursor..])?;
            if written == 0 {
                return Err(DbError::LimitExceeded("WAL tail window accepted no bytes"));
            }
            cursor += written;
            semio_framework_async::yield_once().await;
        }
        Ok(())
    }

    async fn position(&self) -> u64 {
        self.len()
    }
}

/// ⛓️ Every `db_wal` segment is opened requiring `protocol::wire::REQUIRED_HASH_CHAIN` —
/// `SprWriter` always computes the commit chain regardless of this flag, but stamping it into the
/// header makes the requirement an explicit, reader-enforced part of the file's contract rather
/// than an implicit convention only this crate happens to uphold.
// 🚫️async: E1 pure constructor, always used by reference inline (`&segment_write_options()`) — see R9
pub fn segment_write_options() -> protocol::format::WriteOptions {
    protocol::format::WriteOptions { required_flags: protocol::wire::REQUIRED_HASH_CHAIN, optional_flags: 0 }
}
//#endregion 🔖️Sink
