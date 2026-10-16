//! 🪵️ Db write-ahead log segments and the per-document ArtifactWal.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;

use db_durability::DurabilityClass;
use db_ids::{ArtifactId, DbError};
use db_wal_log::{GroupCommitPolicy, SharedBuf, WAL_COMMAND, WAL_REPLAY_STEP_FUEL, WAL_REPLAY_STEP_STALL_BOUND, WAL_TRANSACTION_RECORDS_MAX, WalCursorControl, WalPageReader, WalPriorChainTip, WalRecord, WalRecordBatch, WalRecoveryReport, WalTransactionGate, WalValidatedPrefix, exact_prior_tip, protocol_err, scan_retained_pages, segment_write_options, validate_wal_prefix, wal_field_len, wal_record_write, wal_varint_len};
#[cfg(any(test, feature = "testkit"))]
use db_durability::Frontier;
#[cfg(any(test, feature = "testkit"))]
use db_wal_log::{MAX_FIELD_BYTES, WalBytes, WalPayloadRef};
#[cfg(any(test, feature = "testkit"))]
use pack::ContentHash;
#[cfg(any(test, feature = "testkit"))]
use db_wal_log::{WAL_FRONTIER, WAL_PAYLOAD, WAL_SNAPSHOT_PUB, WalAuthenticatedSource, WalAuthenticatedStep, WalCommittedRecordStep, WalCommittedStep, WalImmutableByteSource, WalReplayStep, WalSegmentChain, WalVerifiedFrameStep, replay_committed_document, replay_document, wal_crc_range, wal_next_verified_page_frame};

//#region 🔖️Segment
/// 📦️ The live write path for exactly one WAL segment: an in-memory `protocol::SprWriter`
/// over a `SharedBuf` tail window, plus how much of the segment has been flushed (durably appended)
/// to `db_storage::WalStorage` so far. The writer lives as long as the segment — sealed and
/// replaced by a fresh one on rotation — while the window holds pages only between an admitted
/// append and its flush.
struct SegmentWriter {
    pub document: ArtifactId,
    pub index: u64,
    buf: Option<SharedBuf>,
    pub writer: Option<protocol::SprWriter<SharedBuf>>,
    flushed_len: u64,
    pub pending_records: u32,
    pub oldest_pending_at_ms: Option<u64>,
}

impl SegmentWriter {
    fn writer_mut(&mut self) -> Result<&mut protocol::SprWriter<SharedBuf>, DbError> {
        self.writer.as_mut().ok_or(DbError::Closed)
    }

    fn buf(&self) -> Result<&SharedBuf, DbError> {
        self.buf.as_ref().ok_or(DbError::Closed)
    }

    fn ensure_open(&self) -> Result<(), DbError> {
        if self.writer.is_some() && self.buf.is_some() {
            Ok(())
        } else {
            Err(DbError::Closed)
        }
    }

    fn poison(&mut self) {
        self.writer = None;
    }

    async fn retire_poisoned(&mut self) -> Result<(), DbError> {
        self.poison();
        while self.close_step()? {
            semio_framework_async::yield_once().await;
        }
        Ok(())
    }

    async fn resume_existing_verified(document: ArtifactId, index: u64, buf: SharedBuf, span: protocol::format::retained::VerifiedSprSpan) -> Result<Self, DbError> {
        let flushed_len = span.end();
        let writer = match protocol::SprWriter::resume_verified(buf.clone(), span).await.map_err(protocol_err) {
            Ok(writer) => writer,
            Err(error) => {
                let mut buf = buf;
                while buf.close_step()? {
                    semio_framework_async::yield_once().await;
                }
                return Err(error);
            }
        };
        Ok(Self { document, index, buf: Some(buf), writer: Some(writer), flushed_len, pending_records: 0, oldest_pending_at_ms: None })
    }

    async fn initialize_existing_empty(
        storage: &impl db_io_core::WalStorage,
        permit: &db_io_core::WalWriterPermit,
        document: ArtifactId,
        index: u64,
        prev_chain_hash: Option<[u8; 32]>,
        partial: &db_io_core::DbIoPages,
        now_ms: u64,
        control: &mut WalCursorControl,
    ) -> Result<Self, DbError> {
        let mut buf = SharedBuf::at(0);
        buf.admit(protocol::format::HEADER_SIZE as u64)?;
        let writer = match protocol::SprWriter::begin(buf.clone(), &segment_write_options()).await.map_err(protocol_err) {
            Ok(writer) => writer,
            Err(error) => {
                while buf.close_step()? {
                    semio_framework_async::yield_once().await;
                }
                return Err(error);
            }
        };
        let mut segment = Self { document: document.clone(), index, buf: Some(buf), writer: Some(writer), flushed_len: 0, pending_records: 0, oldest_pending_at_ms: None };
        let result = async {
            let mut expected = [0u8; protocol::format::HEADER_SIZE];
            segment.buf()?.read_exact(0, &mut expected).await?;
            let mut actual = WalPageReader::new(partial, 0, partial.len())?;
            for byte in &expected[..partial.len()] {
                if actual.byte()? != *byte {
                    return Err(DbError::Corrupt("wal partial header differs from its exact profile".to_string()));
                }
            }
            control.grant()?;
            if !partial.is_empty() {
                storage.truncate_tail(permit, index, 0).await?;
                storage.sync(permit, index, DurabilityClass::Fsync).await?;
            }
            segment.append_record(&WalRecord::SegmentHeader { document: document.clone(), segment_index: index, prev_chain_hash }, now_ms).await?;
            segment.commit_and_flush(storage, permit, DurabilityClass::Fsync).await?;
            Ok(())
        }
        .await;
        if let Err(error) = result {
            segment.retire_poisoned().await?;
            return Err(error);
        }
        Ok(segment)
    }

    /// 🆕️ Creates segment `index` in `storage`, writes its `WAL_SEGMENT_HEADER` record, and
    /// commits+flushes immediately (a segment's own identity/chain-link should never be lost to a
    /// crash before the segment records anything else).
    async fn begin(storage: &impl db_io_core::WalStorage, permit: &db_io_core::WalWriterPermit, document: ArtifactId, index: u64, prev_chain_hash: Option<[u8; 32]>, now_ms: u64) -> Result<Self, DbError> {
        storage.create_segment(permit, index).await?;
        let mut buf = SharedBuf::at(0);
        buf.admit(protocol::format::HEADER_SIZE as u64)?;
        let writer = match protocol::SprWriter::begin(buf.clone(), &segment_write_options()).await.map_err(protocol_err) {
            Ok(writer) => writer,
            Err(error) => {
                while buf.close_step()? {
                    semio_framework_async::yield_once().await;
                }
                return Err(error);
            }
        };
        let mut segment = Self { document: document.clone(), index, buf: Some(buf), writer: Some(writer), flushed_len: 0, pending_records: 0, oldest_pending_at_ms: None };
        let result = async {
            segment.append_record(&WalRecord::SegmentHeader { document, segment_index: index, prev_chain_hash }, now_ms).await?;
            segment.commit_and_flush(storage, permit, DurabilityClass::Fsync).await?;
            Ok(())
        }
        .await;
        if let Err(error) = result {
            segment.poison();
            while segment.close_step()? {
                semio_framework_async::yield_once().await;
            }
            return Err(error);
        }
        Ok(segment)
    }

    async fn append_record(&mut self, record: &WalRecord, now_ms: u64) -> Result<u64, DbError> {
        self.buf()?.admit(wal_frame_bytes(record.retained_shape().1)?)?;
        let offset = record.write_retained(self.writer_mut()?).await?;
        self.appended(now_ms);
        Ok(offset)
    }

    /// 📨️ Appends one `WAL_COMMAND` record straight from the command's own encoded bytes — the same frame
    /// `WalRecord::Command` writes, without first staging the bytes in DB I/O pages.
    async fn append_command(&mut self, command: &[u8], now_ms: u64) -> Result<u64, DbError> {
        self.buf()?.admit(wal_frame_bytes(command.len())?)?;
        let mut record = self.writer_mut()?.begin_identity_record(WAL_COMMAND, true, command.len()).await.map_err(protocol_err)?;
        wal_record_write(&mut record, command).await?;
        let offset = record.finish().await.map_err(protocol_err)?;
        self.appended(now_ms);
        Ok(offset)
    }

    fn appended(&mut self, now_ms: u64) {
        if self.pending_records == 0 {
            self.oldest_pending_at_ms = Some(now_ms);
        }
        self.pending_records += 1;
    }

    /// 📏️ Bytes written since the last flush — not yet visible to `WalStorage`.
    pub fn pending_bytes(&self) -> Result<u64, DbError> {
        self.buf()?.len().checked_sub(self.flushed_len).ok_or_else(|| DbError::Corrupt("WAL flushed length exceeds its retained writer".to_string()))
    }

    /// 📏️ The segment's total logical length so far, flushed or not — what
    /// `ArtifactWal::submit` compares against its segment-rotation threshold.
    fn total_len(&self) -> Result<u64, DbError> {
        Ok(self.buf()?.len())
    }

    /// ⛓️ Physically commits (`SprWriter::commit`, hash-chaining everything pending),
    /// flushes the newly-committed suffix to `WalStorage::append` + `sync(class)` and hands the
    /// flushed tail window's pages back — the group-commit primitive `ArtifactWal::submit`/
    /// `force_flush`/`rotate` all funnel through. A no-op (`Ok(None)`) if nothing is pending.
    async fn commit_and_flush(&mut self, storage: &impl db_io_core::WalStorage, permit: &db_io_core::WalWriterPermit, class: DurabilityClass) -> Result<Option<u64>, DbError> {
        self.ensure_open()?;
        if self.pending_records == 0 {
            return Ok(None);
        }
        self.buf()?.admit(protocol::format::COMMIT_FRAME_LEN)?;
        let commit = self.writer_mut()?.commit().await.map_err(protocol_err);
        let commit_offset = match commit {
            Ok(offset) => offset,
            Err(error) => {
                self.poison();
                return Err(error);
            }
        };
        let flush = async {
            let expected_len = self.buf()?.len();
            let suffix_len =
                usize::try_from(expected_len.checked_sub(self.flushed_len).ok_or_else(|| DbError::Corrupt("WAL flushed length exceeds its retained writer".to_string()))?).map_err(|_| DbError::LimitExceeded("WAL retained suffix length"))?;
            let pages = self.buf()?.copy_range(self.flushed_len as usize, suffix_len).await?;
            let new_len = storage.append(permit, self.index, pages).await?;
            if new_len != expected_len {
                return Err(DbError::Corrupt(format!("WAL append returned segment length {new_len}, expected {expected_len}")));
            }
            self.flushed_len = new_len;
            storage.sync(permit, self.index, class).await?;
            while self.buf()?.release_step()? {
                semio_framework_async::yield_once().await;
            }
            Ok(())
        }
        .await;
        match flush {
            Ok(()) => {
                self.pending_records = 0;
                self.oldest_pending_at_ms = None;
                Ok(Some(commit_offset))
            }
            Err(error) => {
                self.poison();
                Err(error)
            }
        }
    }

    /// ⛓️ The chain_hash of this segment's last commit (`blake3(header)` if nothing has
    /// committed beyond the segment's own header write, which `begin` always performs). Used by
    /// `ArtifactWal::rotate` to seed the next segment's `WAL_SEGMENT_HEADER.prev_chain_hash`; the
    /// writer carries it, because the committed bytes themselves have already left the tail window.
    fn tip_chain_hash(&self) -> Result<[u8; 32], DbError> {
        self.ensure_open()?;
        Ok(self.writer.as_ref().ok_or(DbError::Closed)?.committed_chain_hash())
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.pending_records != 0 && self.writer.is_some() {
            return Err(DbError::InvalidArgument("WAL has pending records; force_flush is required before close".to_string()));
        }
        if self.writer.take().is_some() {
            return Ok(true);
        }
        let Some(buf) = self.buf.as_mut() else { return Ok(false) };
        if buf.close_step()? {
            return Ok(true);
        }
        if !buf.terminal_is_empty() {
            return Err(DbError::Internal("WAL retained buffer did not reach terminal ownership".to_string()));
        }
        self.buf = None;
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.writer.is_none() && self.buf.is_none()
    }
}
//#endregion 🔖️Segment
//#region 🔖️ArtifactWal
/// 📏️ Default segment-rotation threshold (this crate's own choice — the contract fixes
/// "per-document segment files", not an exact size): large enough that rotation stays rare under
/// ordinary load, small enough that a single segment's crash-recovery replay stays bounded.
const DEFAULT_MAX_SEGMENT_BYTES: u64 = db_io_core::DB_IO_MAX_READ_BYTES;

fn wal_frame_bytes(payload: usize) -> Result<u64, DbError> {
    let body = (payload as u64).checked_add(2).ok_or(DbError::LimitExceeded("wal frame bytes"))?;
    body.checked_add(wal_varint_len(body) as u64 + 8).ok_or(DbError::LimitExceeded("wal frame bytes"))
}

/// 🎟️ Bytes one submitted transaction adds to its segment: its framed commands and records plus the
/// commit that may seal them.
fn wal_submit_reservation(commands: &[Vec<u8>], records: &WalRecordBatch) -> Result<u64, DbError> {
    wal_transaction_frame_bytes(commands, records)?.checked_add(protocol::format::COMMIT_FRAME_LEN).ok_or(DbError::LimitExceeded("wal transaction reservation"))
}

fn wal_transaction_frame_bytes(commands: &[Vec<u8>], records: &WalRecordBatch) -> Result<u64, DbError> {
    let mut bytes = wal_frame_bytes(8)?.checked_add(wal_frame_bytes(12)?).ok_or(DbError::LimitExceeded("wal transaction bytes"))?;
    for payload in commands.iter().map(Vec::len).chain(records.iter().map(|record| record.retained_shape().1)) {
        bytes = bytes.checked_add(wal_frame_bytes(payload)?).ok_or(DbError::LimitExceeded("wal transaction bytes"))?;
    }
    Ok(bytes)
}

/// 🧾️ What `ArtifactWal::submit` did with one transaction's records.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalAppendReceipt {
    pub segment_index: u64,
    pub tx_id: u64,
    /// ✅️ True iff this call caused (or piggybacked on) a physical `commit()`+`sync` —
    /// false means the transaction is durable only up to `DurabilityClass::Memory` so far, still
    /// batched behind `GroupCommitPolicy`.
    pub committed: bool,
}

/// 📼️ One document's write-ahead log: an ordered chain of per-segment `.spr` files over
/// `db_storage::WalStorage`, with group-commit batching and crash recovery. This is the type
/// `db_artifact`'s authority actor owns one of per open document. Its backend's writer is scarce, so
/// the WAL parks it whenever nothing waits in its group commit ([`Self::park_writer`]) and resumes it
/// before every write ([`Self::begin_write`]); a writer the backend reclaimed meanwhile is replaced by a
/// fresh acquisition, verified against the active segment exactly as this WAL left it.
pub struct ArtifactWal {
    pub document: ArtifactId,
    pub writer: Option<db_io_core::WalWriterPermit>,
    pub release: Option<db_io_core::WalWriterRelease>,
    release_retry: bool,
    pub policy: GroupCommitPolicy,
    max_segment_bytes: u64,
    next_segment_index: u64,
    pub active: SegmentWriter,
    pub next_tx_id: u64,
}

/// 🧲️ A failed recovery returns the caller's exact acquired writer for same-owner retry.
#[must_use = "the acquired WAL writer must be retried or converted into a retained close"]
pub struct ArtifactWalAcquiredRejected {
    pub cause: DbError,
    pub writer: db_io_core::WalWriterPermit,
}

impl ArtifactWalAcquiredRejected {
    pub fn new(cause: DbError, writer: db_io_core::WalWriterPermit) -> Self {
        Self { cause, writer }
    }

    pub fn error(&self) -> &DbError {
        &self.cause
    }

    pub fn into_parts(self) -> (DbError, db_io_core::WalWriterPermit) {
        (self.cause, self.writer)
    }

    pub async fn retry_open(self, storage: &impl db_io_core::WalStorage, policy: GroupCommitPolicy, now_ms: u64, control: &mut WalCursorControl) -> Result<(ArtifactWal, WalRecoveryReport), ArtifactWalAcquiredRejected> {
        ArtifactWal::open_acquired(storage, self.writer, policy, now_ms, control).await
    }

    pub fn into_open_rejected(self) -> ArtifactWalOpenRejected {
        ArtifactWalOpenRejected::retained(self.cause, self.writer.release())
    }
}

impl std::fmt::Debug for ArtifactWalAcquiredRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("ArtifactWalAcquiredRejected").field("cause", &self.cause).finish_non_exhaustive()
    }
}

/// 🚑️ Self-acquiring WAL rejection retaining the exact writer-release witness until terminal ACK.
#[must_use = "the WAL rejection must be closed to its terminal writer-release witness"]
pub enum ArtifactWalOpenRejected {
    BeforeAcquire { cause: DbError },
    Retained { cause: DbError, release_error: Option<DbError>, release: db_io_core::WalWriterRelease },
}

impl ArtifactWalOpenRejected {
    fn before_acquire(cause: DbError) -> Self {
        Self::BeforeAcquire { cause }
    }

    pub fn retained(cause: DbError, release: db_io_core::WalWriterRelease) -> Self {
        Self::Retained { cause, release_error: None, release }
    }

    pub fn error(&self) -> &DbError {
        match self {
            Self::BeforeAcquire { cause } | Self::Retained { cause, .. } => cause,
        }
    }

    pub fn release_error(&self) -> Option<&DbError> {
        match self {
            Self::BeforeAcquire { .. } => None,
            Self::Retained { release_error, .. } => release_error.as_ref(),
        }
    }

    pub fn has_release_owner(&self) -> bool {
        matches!(self, Self::Retained { .. })
    }

    pub fn into_parts(self) -> (DbError, Option<db_io_core::WalWriterRelease>) {
        match self {
            Self::BeforeAcquire { cause } => (cause, None),
            Self::Retained { cause, release, .. } => (cause, Some(release)),
        }
    }

    pub async fn retry_close(self) -> Result<DbError, ArtifactWalOpenRejected> {
        match self {
            Self::BeforeAcquire { cause } => Ok(cause),
            Self::Retained { cause, release_error, release } => {
                let release = if release_error.is_some() { release.retry() } else { release };
                match release.await {
                    Ok(()) => Ok(cause),
                    Err(failure) => {
                        let (release_error, release) = failure.into_parts();
                        Err(Self::Retained { cause, release_error: Some(release_error), release })
                    }
                }
            }
        }
    }
}

impl std::fmt::Debug for ArtifactWalOpenRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("ArtifactWalOpenRejected").field("cause", self.error()).field("release_error", &self.release_error()).field("has_release_owner", &self.has_release_owner()).finish()
    }
}

impl ArtifactWal {
    /// 🌱️ Creates a brand new WAL for `document` (segment 0, genesis — no prior segment to
    /// chain from). Errors `AlreadyExists` if `document` already has WAL segments in `storage`.
    pub async fn create(storage: &impl db_io_core::WalStorage, document: ArtifactId, policy: GroupCommitPolicy, now_ms: u64) -> Result<Self, ArtifactWalOpenRejected> {
        let writer = storage.acquire_writer(&document).await.map_err(ArtifactWalOpenRejected::before_acquire)?;
        let active = match SegmentWriter::begin(storage, &writer, document.clone(), 0, None, now_ms).await {
            Ok(active) => active,
            Err(error) => return Err(ArtifactWalAcquiredRejected::new(error, writer).into_open_rejected()),
        };
        Ok(Self { document, writer: Some(writer), release: None, release_retry: false, policy, max_segment_bytes: DEFAULT_MAX_SEGMENT_BYTES, next_segment_index: 1, active, next_tx_id: 1 })
    }

    /// 🚑️ Verifies the retained chain, durably aborts incomplete active transactions, repairs the
    /// uncommitted tail, and resumes the physical sequence under the caller's exclusive write authority.
    /// The bound holds per verified segment (`WAL_REPLAY_STEP_STALL_BOUND`, `WAL_REPLAY_STEP_FUEL`):
    /// one wall-clock budget for the whole chain made every document whose history outgrew it
    /// unopenable, and a loaded host reached that size first.
    pub async fn open(storage: &impl db_io_core::WalStorage, document: ArtifactId, policy: GroupCommitPolicy, now_ms: u64) -> Result<(Self, WalRecoveryReport), ArtifactWalOpenRejected> {
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut control = WalCursorControl::stall_bounded(cancelled, WAL_REPLAY_STEP_STALL_BOUND, WAL_REPLAY_STEP_FUEL).map_err(ArtifactWalOpenRejected::before_acquire)?;
        Self::open_with_control(storage, document, policy, now_ms, &mut control).await
    }

    /// 🚦️ Caller-controlled bounded recovery; cleanup is never gated by cancellation.
    pub async fn open_with_control(storage: &impl db_io_core::WalStorage, document: ArtifactId, policy: GroupCommitPolicy, now_ms: u64, control: &mut WalCursorControl) -> Result<(Self, WalRecoveryReport), ArtifactWalOpenRejected> {
        control.grant().map_err(ArtifactWalOpenRejected::before_acquire)?;
        let writer = storage.acquire_writer(&document).await.map_err(ArtifactWalOpenRejected::before_acquire)?;
        Self::open_acquired(storage, writer, policy, now_ms, control).await.map_err(ArtifactWalAcquiredRejected::into_open_rejected)
    }

    pub async fn open_acquired(storage: &impl db_io_core::WalStorage, writer: db_io_core::WalWriterPermit, policy: GroupCommitPolicy, now_ms: u64, control: &mut WalCursorControl) -> Result<(Self, WalRecoveryReport), ArtifactWalAcquiredRejected> {
        let document = ArtifactId(writer.document().as_str().to_string());
        let mut writer = Some(writer);
        let mut indices = match storage.list_segments(&document).await {
            Ok(indices) => indices,
            Err(error) => return Err(ArtifactWalAcquiredRejected::new(error, writer.take().expect("acquired writer"))),
        };
        let result = async {
            let permit = writer.as_ref().expect("recovery retains its acquired writer");
            if indices.is_empty() {
                control.grant()?;
                let active = SegmentWriter::begin(storage, permit, document.clone(), 0, None, now_ms).await?;
                return Ok((
                    Self { document: document.clone(), writer: writer.take(), release: None, release_retry: false, policy, max_segment_bytes: DEFAULT_MAX_SEGMENT_BYTES, next_segment_index: 1, active, next_tx_id: 1 },
                    WalRecoveryReport::default(),
                ));
            }
            let first = indices.as_slice()[0];
            let last = *indices.last().ok_or_else(|| DbError::Internal("wal segment list lost its highest index".to_string()))?;
            let mut next_segment_index = last.checked_add(1).ok_or(DbError::LimitExceeded("wal segment sequence"))?;
            for (ordinal, index) in indices.as_slice().iter().enumerate() {
                if first.checked_add(ordinal as u64) != Some(*index) {
                    return Err(DbError::Corrupt("wal retained segment sequence is not dense".to_string()));
                }
            }
            let mut prior = if first == 0 { WalPriorChainTip::Genesis } else { WalPriorChainTip::RetainedBoundary };
            let mut report = WalRecoveryReport { segments_seen: indices.len() as u64, ..WalRecoveryReport::default() };
            let mut next_tx_id = 1;
            let mut logical = WalTransactionGate::new();
            let mut active = None;
            for &index in indices.as_slice() {
                control.renew_step()?;
                control.grant()?;
                let state = storage.segment_state(&document, index).await?;
                let writable = index == last && state == db_io_core::WalSegmentState::Active;
                if index != last && state != db_io_core::WalSegmentState::Sealed {
                    return Err(DbError::Corrupt("wal has an active segment before its highest index".to_string()));
                }
                if index == last && !writable {
                    next_segment_index = next_segment_index.checked_add(1).ok_or(DbError::LimitExceeded("wal successor sequence"))?;
                }
                control.grant()?;
                let len = storage.segment_len(&document, index).await?;
                control.grant()?;
                let mut pages = storage.read(&document, index, pack::ByteRange { offset: 0, len }).await?;
                let outcome = async {
                    if pages.len() as u64 != len {
                        return Err(DbError::Corrupt("wal retained read length differs".to_string()));
                    }
                    if len < protocol::format::HEADER_SIZE as u64 {
                        if !writable {
                            return Err(DbError::Corrupt("sealed wal segment has no committed header".to_string()));
                        }
                        let predecessor = exact_prior_tip(prior)?;
                        let segment = SegmentWriter::initialize_existing_empty(storage, permit, document.clone(), index, predecessor, &pages, now_ms, control).await?;
                        return Ok((Some(segment), None, 0, 1, len, None));
                    }
                    let span = scan_retained_pages(&pages, control).await?;
                    if !writable && (span.tail() != 0 || span.sequence() == 0) {
                        return Err(DbError::Corrupt("sealed wal segment has uncommitted bytes".to_string()));
                    }
                    let genesis = span.sequence() == 0;
                    let predecessor = if genesis { exact_prior_tip(prior)? } else { None };
                    let validated =
                        if genesis { WalValidatedPrefix { records: 0, next_tx_id: logical.next_tx_id, incomplete_active_tx: None } } else { validate_wal_prefix(&pages, &span, index, prior, &document, &mut logical, writable, control).await? };
                    let tip = *span.chain();
                    let tail = span.tail();
                    let end = span.end();
                    if !writable {
                        return Ok((None, Some(tip), validated.records, validated.next_tx_id, 0, None));
                    }
                    if validated.incomplete_active_tx.is_some() {
                        let repaired_end = end.checked_add(wal_frame_bytes(8)?).and_then(|bytes| bytes.checked_add(protocol::format::COMMIT_FRAME_LEN));
                        if repaired_end.is_none_or(|bytes| bytes > db_io_core::DB_IO_MAX_READ_BYTES) {
                            return Err(DbError::LimitExceeded("wal recovery abort exceeds retained segment budget"));
                        }
                    }
                    let mut segment = SegmentWriter::resume_existing_verified(document.clone(), index, SharedBuf::at(end), span).await?;
                    let repair = async {
                        control.grant()?;
                        if tail != 0 {
                            storage.truncate_tail(permit, index, end).await?;
                            storage.sync(permit, index, DurabilityClass::Fsync).await?;
                        }
                        if genesis {
                            segment.append_record(&WalRecord::SegmentHeader { document: document.clone(), segment_index: index, prev_chain_hash: predecessor }, now_ms).await?;
                            segment.commit_and_flush(storage, permit, DurabilityClass::Fsync).await?;
                        }
                        if let Some(tx_id) = validated.incomplete_active_tx {
                            segment.append_record(&WalRecord::TxAbort { tx_id }, now_ms).await?;
                            segment.commit_and_flush(storage, permit, DurabilityClass::Fsync).await?;
                        }
                        segment.tip_chain_hash()
                    }
                    .await;
                    let tip = match repair {
                        Ok(tip) => tip,
                        Err(error) => {
                            segment.retire_poisoned().await?;
                            return Err(error);
                        }
                    };
                    Ok((Some(segment), Some(tip), validated.records, validated.next_tx_id, tail, validated.incomplete_active_tx))
                }
                .await;
                while pages.close_step()?.is_some() {
                    semio_framework_async::yield_once().await;
                }
                let (segment, tip, records, next_tx, tail, recovered_abort) = outcome?;
                report.records_replayed = report.records_replayed.checked_add(records).ok_or(DbError::LimitExceeded("wal recovered records"))?;
                report.torn_tail_bytes = tail;
                report.recovered_abort_tx_id = recovered_abort;
                next_tx_id = next_tx_id.max(next_tx);
                if let Some(tip) = tip {
                    prior = WalPriorChainTip::Verified(tip);
                }
                if let Some(segment) = segment {
                    active = Some(segment);
                }
            }
            let active = match active {
                Some(active) => active,
                None => {
                    let predecessor = exact_prior_tip(prior)?;
                    control.grant()?;
                    SegmentWriter::begin(storage, permit, document.clone(), last + 1, predecessor, now_ms).await?
                }
            };
            Ok((Self { document: document.clone(), writer: writer.take(), release: None, release_retry: false, policy, max_segment_bytes: DEFAULT_MAX_SEGMENT_BYTES, next_segment_index, active, next_tx_id }, report))
        }
        .await;
        while indices.close_step() {
            semio_framework_async::yield_once().await;
        }
        match (result, writer) {
            (Err(error), Some(writer)) => Err(ArtifactWalAcquiredRejected::new(error, writer)),
            (Ok(value), _) => Ok(value),
            (Err(_), None) => unreachable!("failed WAL open transferred its writer only on success"),
        }
    }

    pub async fn document(&self) -> &ArtifactId {
        &self.document
    }

    pub async fn active_segment_index(&self) -> u64 {
        self.active.index
    }

    /// 🖊️ Holds this document's writer for the writes that follow: the parked one resumed, else — its backend reclaimed it
    /// for another document — a fresh acquisition in the backend's admission order, once the reclaimed one's release
    /// completed. A fresh writer is verified against the active segment as this WAL left it (still active, exactly its
    /// flushed length): a WAL another process appended to meanwhile is refused (`Conflict`), never appended to. Call it
    /// before the caller changes anything a failed write would have to undo.
    pub async fn begin_write(&mut self, storage: &impl db_io_core::WalStorage) -> Result<(), DbError> {
        if self.writer.as_ref().is_some_and(db_io_core::WalWriterPermit::resume) {
            return Ok(());
        }
        if let Some(reclaimed) = self.writer.take() {
            self.release = Some(reclaimed.release());
            self.release_retry = false;
        }
        if let Some(release) = self.release.take() {
            let release = if std::mem::take(&mut self.release_retry) { release.retry() } else { release };
            if let Err(failure) = release.await {
                let (error, release) = failure.into_parts();
                self.release = Some(release);
                self.release_retry = true;
                return Err(error);
            }
        }
        let writer = storage.acquire_writer(&self.document).await?;
        let verified = async {
            let state = storage.segment_state(&self.document, self.active.index).await?;
            let len = storage.segment_len(&self.document, self.active.index).await?;
            if state != db_io_core::WalSegmentState::Active || len != self.active.flushed_len {
                return Err(DbError::Conflict("the document's WAL changed while its writer was reclaimed".to_string()));
            }
            Ok(())
        }
        .await;
        match verified {
            Ok(()) => {
                self.writer = Some(writer);
                Ok(())
            }
            Err(error) => {
                self.release = Some(writer.release());
                Err(error)
            }
        }
    }

    /// 🅿️ Parks this WAL's writer while nothing waits in its group commit, so its backend may reclaim it for another
    /// document until the next [`Self::begin_write`]; a pending group commit keeps it held.
    pub fn park_writer(&self) {
        if !self.has_pending() {
            if let Some(writer) = self.writer.as_ref() {
                writer.park();
            }
        }
    }

    /// 🧹 Deletes one snapshot-covered sealed segment under this WAL's writer.
    pub async fn delete_compacted_sealed_segment(&mut self, storage: &impl db_io_core::WalStorage, index: u64) -> Result<(), DbError> {
        if index == self.active.index {
            return Err(DbError::InvalidArgument("active WAL segment cannot be compacted".to_string()));
        }
        if storage.segment_state(&self.document, index).await? != db_io_core::WalSegmentState::Sealed {
            return Err(DbError::InvalidArgument("only sealed WAL segments can be compacted".to_string()));
        }
        self.begin_write(storage).await?;
        storage.delete_segment(self.writer.as_ref().ok_or(DbError::Closed)?, index).await
    }

    /// 📐️ Validates the complete framed transaction against the current or successor
    /// readable-segment bound without consuming a transaction id or beginning storage I/O.
    pub fn preflight_submit(&self, commands: &[Vec<u8>], records: &WalRecordBatch) -> Result<bool, DbError> {
        self.active.ensure_open()?;
        if commands.len().checked_add(records.len()).is_none_or(|count| count > WAL_TRANSACTION_RECORDS_MAX) {
            return Err(DbError::LimitExceeded("wal transaction records"));
        }
        let reservation = wal_submit_reservation(commands, records)?;
        self.next_tx_id.checked_add(1).ok_or(DbError::LimitExceeded("wal transaction sequence"))?;
        if self.active.total_len()?.checked_add(reservation).ok_or(DbError::LimitExceeded("wal segment reservation"))? <= db_io_core::DB_IO_MAX_READ_BYTES {
            return Ok(false);
        }
        let header_payload = wal_field_len(self.document.0.as_bytes()).checked_add(41).ok_or(DbError::LimitExceeded("wal successor header"))?;
        let header_bytes = wal_frame_bytes(header_payload)?.checked_add(protocol::format::HEADER_SIZE as u64 + protocol::format::COMMIT_FRAME_LEN).ok_or(DbError::LimitExceeded("wal successor header"))?;
        if header_bytes.checked_add(reservation).ok_or(DbError::LimitExceeded("wal transaction reservation"))? > db_io_core::DB_IO_MAX_READ_BYTES {
            return Err(DbError::LimitExceeded("wal transaction exceeds readable segment"));
        }
        Ok(true)
    }

    /// ✍️ Appends `commands` (each a `WAL_COMMAND` record written from its own encoded bytes) and then
    /// `records` as one transaction (`WAL_TX_BEGIN` .. `WAL_TX_COMMIT`), then group-commits per `GroupCommitPolicy` —
    /// except `durability >= Fsync` always forces an immediate commit, since deferring one can never satisfy a
    /// durability request stronger than what's already flushed. Rotates to a new segment (sealing this one first, which
    /// forces a commit if anything is still pending) once the active segment crosses `max_segment_bytes`. Commands take
    /// no DB I/O page credit of their own: a whole declared-legal command batch (`protocol`'s document backbone batch
    /// maximum, 256 KiB of encoded envelopes) is one transaction bounded only by the readable segment.
    pub async fn submit(&mut self, storage: &impl db_io_core::WalStorage, commands: &[Vec<u8>], records: &WalRecordBatch, durability: DurabilityClass, now_ms: u64) -> Result<WalAppendReceipt, DbError> {
        let rotate = self.preflight_submit(commands, records)?;
        self.begin_write(storage).await?;
        let next_tx_id = self.next_tx_id.checked_add(1).ok_or(DbError::LimitExceeded("wal transaction sequence"))?;
        if rotate {
            self.rotate(storage, now_ms).await?;
        }
        self.active.buf()?.admit(wal_submit_reservation(commands, records)?)?;
        let record_count = u32::try_from(commands.len() + records.len()).map_err(|_| DbError::LimitExceeded("wal transaction record count"))?;
        let tx_id = self.next_tx_id;
        self.next_tx_id = next_tx_id;
        let segment_index = self.active.index;

        self.active.append_record(&WalRecord::TxBegin { tx_id }, now_ms).await?;
        for command in commands {
            self.active.append_command(command, now_ms).await?;
        }
        for record in records.iter() {
            self.active.append_record(record, now_ms).await?;
        }
        self.active.append_record(&WalRecord::TxCommit { tx_id, record_count }, now_ms).await?;

        let forced = matches!(durability, DurabilityClass::Fsync | DurabilityClass::Quorum(_));
        let due = forced || self.policy.is_due(self.active.pending_bytes()?, self.active.pending_records, self.active.oldest_pending_at_ms, now_ms);
        let mut committed = false;
        if due {
            committed = self.active.commit_and_flush(storage, self.writer.as_ref().ok_or(DbError::Closed)?, durability).await?.is_some();
        }
        if self.active.total_len()? >= self.max_segment_bytes {
            self.rotate(storage, now_ms).await?;
            committed = true;
        }
        Ok(WalAppendReceipt { segment_index, tx_id, committed })
    }

    /// 🚿️ Forces a commit+flush of whatever is currently pending, regardless of policy —
    /// the primitive a timer-driven group-commit loop or a clean-shutdown drain calls. Returns
    /// `true` iff there was anything to flush.
    pub async fn force_flush(&mut self, storage: &impl db_io_core::WalStorage) -> Result<bool, DbError> {
        self.active.ensure_open()?;
        if self.active.pending_records == 0 {
            return Ok(false);
        }
        self.begin_write(storage).await?;
        Ok(self.active.commit_and_flush(storage, self.writer.as_ref().ok_or(DbError::Closed)?, DurabilityClass::Fsync).await?.is_some())
    }

    /// ⏸️ True while group-committed records still wait in the open active segment; the
    /// retained close must drain them through `close_flush` before `close_step` can retire it.
    pub fn has_pending(&self) -> bool {
        self.active.pending_records != 0 && self.active.writer.is_some()
    }

    /// 🚰️ Clean-shutdown drain of the pending group commit. A failed drain poisons the active
    /// segment, so `close_step` still reaches terminal and only the never-fsynced suffix is lost.
    pub async fn close_flush(&mut self, storage: &impl db_io_core::WalStorage) -> Result<(), DbError> {
        let flushed = self.force_flush(storage).await;
        if flushed.is_err() {
            self.active.poison();
        }
        flushed.map(|_| ())
    }

    /// 🔄️ Seals the active segment (after a final commit+flush) and begins a fresh one,
    /// carrying the sealed segment's tip `chain_hash` forward as the new segment's
    /// `WAL_SEGMENT_HEADER.prev_chain_hash` — the cross-segment hash-chain link.
    async fn rotate(&mut self, storage: &impl db_io_core::WalStorage, now_ms: u64) -> Result<(), DbError> {
        self.active.ensure_open()?;
        let following_index = self.next_segment_index.checked_add(1).ok_or(DbError::LimitExceeded("wal segment sequence"))?;
        self.active.commit_and_flush(storage, self.writer.as_ref().ok_or(DbError::Closed)?, DurabilityClass::Fsync).await?;
        let chain_hash = self.active.tip_chain_hash()?;
        let sealed_index = self.active.index;
        let sealed = storage.seal(self.writer.as_ref().ok_or(DbError::Closed)?, sealed_index).await;
        self.active.poison();
        sealed?;
        let new_index = self.next_segment_index;
        while self.active.close_step()? {
            semio_framework_async::yield_once().await;
        }
        let active = SegmentWriter::begin(storage, self.writer.as_ref().ok_or(DbError::Closed)?, self.document.clone(), new_index, Some(chain_hash), now_ms).await?;
        self.active = active;
        self.next_segment_index = following_index;
        Ok(())
    }

    /// 🔕️ Advances one close step. A step that made progress wakes its own waker; a pending
    /// writer release parks the waker in the release signal, so the backend's terminal
    /// notification is the only thing that resumes the caller — never a re-poll.
    pub fn poll_close(&mut self, context: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), DbError>> {
        if self.active.close_step()? {
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        if let Some(writer) = self.writer.take() {
            self.release = Some(writer.release());
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        if self.release_retry {
            self.release = self.release.take().map(db_io_core::WalWriterRelease::retry);
            self.release_retry = false;
        }
        let Some(release) = self.release.as_mut() else { return std::task::Poll::Ready(Ok(())) };
        match std::future::Future::poll(std::pin::Pin::new(release), context) {
            std::task::Poll::Pending => std::task::Poll::Pending,
            std::task::Poll::Ready(Ok(())) => {
                self.release = None;
                std::task::Poll::Ready(Ok(()))
            }
            std::task::Poll::Ready(Err(failure)) => {
                let (error, release) = failure.into_parts();
                self.release = Some(release);
                self.release_retry = true;
                std::task::Poll::Ready(Err(error))
            }
        }
    }

    pub async fn close(&mut self) -> Result<(), DbError> {
        std::future::poll_fn(|context| self.poll_close(context)).await
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.active.terminal_is_empty() && self.writer.is_none() && self.release.is_none()
    }
}
//#endregion 🔖️ArtifactWal

#[cfg(any(test, feature = "testkit"))]
#[path = "🧰️testkit/🦀️.rs"]
pub mod testkit;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️retained/🦀️.rs"]
mod retained_tests;
//#endregion 🧪️Tests
