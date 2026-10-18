//! ✅️ Committed replay and validation: the committed cursor, the turn, committed replay, retained page scans and prefix validation. See [`super`].

use super::*;

//#region 🔖️Recovery
pub async fn replay_document<'storage, S: db_io_core::WalStorage>(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<WalReplayCursor<'storage, S>, DbError> {
    WalReplayCursor::open(storage, document, control).await
}

/// 🛂️ Releases bounded transaction bodies only after their exact logical commit is verified.
pub struct WalCommittedCursor<'storage, S: db_io_core::WalStorage> {
    pub raw: WalReplayCursor<'storage, S>,
    pub gate: WalTransactionGate,
    pub record: Option<WalRecord>,
    pub record_index: usize,
}

pub enum WalCommittedStep<'cursor, 'storage, S: db_io_core::WalStorage> {
    Transaction(WalCommittedTransaction<'cursor, 'storage, S>),
    Yield,
    Done,
}

/// ⏱️ How long one step of a whole-document WAL scan (one verified segment on open, one committed
/// transaction on replay) may make no progress.
pub const WAL_REPLAY_STEP_STALL_BOUND: std::time::Duration = std::time::Duration::from_secs(30);

/// ⛽️ Fuel one step of a whole-document WAL scan may spend.
pub const WAL_REPLAY_STEP_FUEL: usize = 1_000_000;

/// ⏲️ How long one cooperative turn of a WAL walk or staging copy runs before it yields to its executor.
pub const WAL_TURN: std::time::Duration = std::time::Duration::from_millis(1);

/// 🤝️ Batches bounded WAL steps into cooperative turns: a cursor `Yield`, a record close step, a
/// decoded record or a staged fragment continues inline until the turn's time budget is spent, and
/// only then does the caller yield once to its executor. Yielding after every frame, record and
/// fragment handed the caller's whole future chain back to its executor thousands of times per
/// reopened document and once per staged command of every commit.
pub struct WalTurn {
    ends: std::time::Instant,
}

impl WalTurn {
    pub fn start() -> Self {
        Self { ends: std::time::Instant::now() + WAL_TURN }
    }

    pub async fn step(&mut self) {
        if std::time::Instant::now() >= self.ends {
            semio_framework_async::yield_once().await;
            self.ends = std::time::Instant::now() + WAL_TURN;
        }
    }
}

/// 🤝️ Keeps the source segment and current decoded body borrowed until explicit retirement.
pub struct WalCommittedTransaction<'cursor, 'storage, S: db_io_core::WalStorage> {
    pub cursor: &'cursor mut WalCommittedCursor<'storage, S>,
    finished: bool,
}

pub enum WalCommittedRecordStep<'record> {
    Record(&'record WalRecord),
    Yield,
    Done,
}

impl<'storage, S: db_io_core::WalStorage> WalCommittedCursor<'storage, S> {
    /// ⏱️ Renews the stall bound for the next committed transaction of a whole-document replay, so
    /// the replay's total length follows its document's size and only a step that makes no progress
    /// for `WAL_REPLAY_STEP_STALL_BOUND` fails. A single wall-clock budget for the whole replay made
    /// every large enough document unopenable after a crash.
    pub fn renew_step(&mut self) -> Result<(), DbError> {
        self.replenish(std::time::Instant::now() + WAL_REPLAY_STEP_STALL_BOUND, WAL_REPLAY_STEP_FUEL)
    }

    /// 🗂️ Lists retained segments, including header-only segments; admission completes during replay.
    pub fn segment_indices(&self) -> &[u64] {
        self.raw.segments.as_slice()
    }

    pub async fn open(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<Self, DbError> {
        Ok(Self { raw: WalReplayCursor::open(storage, document, control).await?, gate: WalTransactionGate::new(), record: None, record_index: 0 })
    }

    /// 📍 Opens at one exact retained segment boundary. The selected segment is authenticated as
    /// a retained boundary before any committed transaction is released.
    pub async fn open_at_segment(storage: &'storage S, document: &ArtifactId, segment_index: u64, control: WalCursorControl) -> Result<Self, DbError> {
        let mut raw = WalReplayCursor::open(storage, document, control).await?;
        let Some(segment) = raw.segments.as_slice().iter().position(|index| *index == segment_index) else {
            while raw.close_owner_step()? {}
            return Err(DbError::NotFound(format!("WAL segment {segment_index}")));
        };
        raw.segment = segment;
        Ok(Self { raw, gate: WalTransactionGate::new(), record: None, record_index: 0 })
    }

    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        self.raw.replenish(deadline, fuel)
    }

    async fn prepare_transaction_step(&mut self) -> Result<Option<bool>, DbError> {
        self.raw.control.check()?;
        if self.raw.closed {
            return Ok(None);
        }
        if self.gate.ready.is_some() {
            return Err(DbError::Corrupt("wal committed transaction was not finished".to_string()));
        }
        if self.raw.pages.is_none() {
            return Ok(self.raw.open_segment().await?.then_some(false));
        }
        if let Some(validation) = self.raw.validation.as_mut() {
            let pages = self.raw.pages.as_ref().ok_or_else(|| DbError::Internal("wal committed replay lost validation pages".to_string()))?;
            if validation.step(pages, self.raw.trusted_len, &self.raw.document, &mut self.raw.control)? {
                self.raw.previous_tip = validation.tip;
                self.raw.validation = None;
            }
            return Ok(Some(false));
        }
        if self.raw.offset == self.raw.trusted_len {
            self.gate.finish_segment(false)?;
            if self.raw.close_segment_step().await? {
                return Ok(Some(false));
            }
            self.gate.advance_segment()?;
            self.raw.segment += 1;
            return Ok(Some(false));
        }
        let pages = self.raw.pages.as_ref().ok_or_else(|| DbError::Internal("wal committed replay lost source pages".to_string()))?;
        let WalVerifiedFrameStep::Frame(frame) = wal_next_verified_page_frame(pages, &mut self.raw.offset, self.raw.trusted_len, &mut self.raw.control)? else { return Ok(Some(false)) };
        let ready = self.gate.push(pages, frame)?;
        self.raw.offset = frame.frame_end;
        Ok(Some(ready))
    }

    pub async fn next_transaction_step(&mut self) -> Result<WalCommittedStep<'_, 'storage, S>, DbError> {
        if self.raw.failed {
            return Err(DbError::Corrupt("wal committed replay remains failed until closed".to_string()));
        }
        match self.prepare_transaction_step().await {
            Ok(Some(true)) => Ok(WalCommittedStep::Transaction(WalCommittedTransaction { cursor: self, finished: false })),
            Ok(Some(false)) => Ok(WalCommittedStep::Yield),
            Ok(None) => Ok(WalCommittedStep::Done),
            Err(error) => {
                if !wal_cursor_interrupted(&error) {
                    self.raw.failed = true;
                }
                Err(error)
            }
        }
    }

    pub fn close_owner_step(&mut self) -> Result<bool, DbError> {
        if let Some(record) = self.record.as_mut() {
            if record.close_step()? {
                return Ok(true);
            }
        }
        self.record = None;
        if self.raw.close_owner_step()? {
            return Ok(true);
        }
        self.gate.clear_frames();
        self.gate.ready = None;
        self.gate.transaction_id = None;
        self.record_index = 0;
        Ok(false)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.raw.terminal_is_empty() && self.record.is_none() && self.gate.frames.is_empty() && self.gate.ready.is_none() && self.gate.transaction_id.is_none()
    }
}

fn wal_cursor_interrupted(error: &DbError) -> bool {
    matches!(error, DbError::LimitExceeded("wal cursor fuel")) || matches!(error, DbError::Unavailable(message) if message == "wal cursor cancelled" || message == "wal cursor deadline reached")
}

impl<'cursor, 'storage, S: db_io_core::WalStorage> WalCommittedTransaction<'cursor, 'storage, S> {
    pub fn transaction_id(&self) -> u64 {
        self.cursor.gate.ready.expect("committed transaction owns its ready gate")
    }

    pub fn segment_index(&self) -> u64 {
        self.cursor.raw.segments.as_slice()[self.cursor.raw.segment]
    }

    pub fn record_count(&self) -> usize {
        self.cursor.gate.frames.len()
    }

    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        self.cursor.replenish(deadline, fuel)
    }

    fn decode_record_step(&mut self) -> Result<bool, DbError> {
        if self.cursor.record.is_some() {
            return Err(DbError::Corrupt("wal committed body must be closed before advancing".to_string()));
        }
        let frame = self.cursor.gate.frames.get(self.cursor.record_index).copied().ok_or_else(|| DbError::Internal("wal committed replay lost body span".to_string()))?;
        if self.cursor.raw.decoder.is_none() {
            self.cursor.raw.decoder = Some(WalRetainedRecordDecoder::new(frame));
        }
        let pages = self.cursor.raw.pages.as_ref().ok_or_else(|| DbError::Internal("wal committed replay lost body source".to_string()))?;
        let decoder = self.cursor.raw.decoder.as_mut().ok_or_else(|| DbError::Internal("wal committed replay lost body decoder".to_string()))?;
        if let Some(record) = decoder.step(pages, &mut self.cursor.raw.control)? {
            self.cursor.record = Some(record);
            self.cursor.raw.decoder = None;
            return Ok(true);
        }
        Ok(false)
    }

    pub fn next_record_step(&mut self) -> Result<WalCommittedRecordStep<'_>, DbError> {
        if self.cursor.raw.failed {
            return Err(DbError::Corrupt("wal committed replay remains failed until closed".to_string()));
        }
        if self.cursor.record_index == self.record_count() {
            return Ok(WalCommittedRecordStep::Done);
        }
        match self.decode_record_step() {
            Ok(true) => Ok(WalCommittedRecordStep::Record(self.cursor.record.as_ref().expect("decoded body remains cursor-owned"))),
            Ok(false) => Ok(WalCommittedRecordStep::Yield),
            Err(error) => {
                if !wal_cursor_interrupted(&error) {
                    self.cursor.raw.failed = true;
                }
                Err(error)
            }
        }
    }

    pub fn close_record_step(&mut self) -> Result<bool, DbError> {
        let record = self.cursor.record.as_mut().ok_or_else(|| DbError::Corrupt("wal committed body is not borrowed".to_string()))?;
        if record.close_step()? {
            return Ok(true);
        }
        self.cursor.record = None;
        self.cursor.record_index += 1;
        Ok(false)
    }

    pub fn finish(mut self) -> Result<(), DbError> {
        if self.cursor.raw.failed || self.cursor.record.is_some() || self.cursor.raw.decoder.is_some() || self.cursor.record_index != self.record_count() {
            return Err(DbError::Corrupt("wal committed transaction has unretired bodies".to_string()));
        }
        self.cursor.gate.release()?;
        self.cursor.record_index = 0;
        self.finished = true;
        Ok(())
    }
}

impl<S: db_io_core::WalStorage> Drop for WalCommittedTransaction<'_, '_, S> {
    fn drop(&mut self) {
        if !self.finished {
            self.cursor.raw.failed = true;
        }
    }
}

pub async fn replay_committed_document<'storage, S: db_io_core::WalStorage>(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<WalCommittedCursor<'storage, S>, DbError> {
    WalCommittedCursor::open(storage, document, control).await
}

pub async fn replay_committed_segment<'storage, S: db_io_core::WalStorage>(storage: &'storage S, document: &ArtifactId, segment_index: u64, control: WalCursorControl) -> Result<WalCommittedCursor<'storage, S>, DbError> {
    WalCommittedCursor::open_at_segment(storage, document, segment_index, control).await
}

pub async fn scan_retained_pages(pages: &db_io_core::DbIoPages, control: &mut WalCursorControl) -> Result<protocol::format::retained::VerifiedSprSpan, DbError> {
    use protocol::format::retained::{RetainedSprDiagnostic, RetainedSprLimits, RetainedSprVerification};
    let map_error = |error| match error {
        RetainedSprDiagnostic::Capacity => DbError::LimitExceeded("wal retained verification"),
        RetainedSprDiagnostic::Cancelled => DbError::Unavailable("wal retained verification cancelled".to_string()),
        _ => DbError::Corrupt("wal retained framing or commit verification failed".to_string()),
    };
    let capacity = db_io_core::DB_IO_MAX_READ_BYTES;
    let mut scan = RetainedSprVerification::new(pages.len() as u64, RetainedSprLimits { file_bytes: capacity, frame_body_bytes: capacity, records: capacity / 11 }).map_err(map_error)?;
    for fragment in pages.fragments() {
        control.grant()?;
        let mut fuel = fragment.len();
        if scan.push(fragment, &mut fuel).map_err(map_error)? != fragment.len() {
            return Err(DbError::Corrupt("wal retained scan made partial page progress".to_string()));
        }
        semio_framework_async::yield_once().await;
    }
    let header = protocol::format::read_header(&WalPageSource(pages)).await.map_err(protocol_err)?;
    if header.optional_flags != 0 {
        return Err(DbError::Corrupt("wal optional header flags differ".to_string()));
    }
    scan.finish().map_err(map_error)
}

pub struct WalValidatedPrefix {
    pub records: u64,
    pub next_tx_id: u64,
    pub incomplete_active_tx: Option<u64>,
}

pub async fn validate_wal_prefix(
    pages: &db_io_core::DbIoPages,
    span: &protocol::format::retained::VerifiedSprSpan,
    index: u64,
    prior: WalPriorChainTip,
    document: &ArtifactId,
    gate: &mut WalTransactionGate,
    writable_highest: bool,
    control: &mut WalCursorControl,
) -> Result<WalValidatedPrefix, DbError> {
    let end = usize::try_from(span.end()).map_err(|_| DbError::LimitExceeded("wal verified end"))?;
    let mut chain = WalSegmentChain::new(pages, index, prior)?;
    while !chain.step(pages, end, document, control)? {
        semio_framework_async::yield_once().await;
    }
    if chain.tip != Some(*span.chain()) {
        return Err(DbError::Corrupt("wal verifiers disagree on committed chain".to_string()));
    }
    let mut offset = protocol::format::HEADER_SIZE;
    let mut records = 0u64;
    loop {
        let frame = match wal_next_verified_page_frame(pages, &mut offset, end, control)? {
            WalVerifiedFrameStep::Frame(frame) => frame,
            WalVerifiedFrameStep::PhysicalCommit => {
                semio_framework_async::yield_once().await;
                continue;
            }
            WalVerifiedFrameStep::Done => break,
        };
        if gate.push(pages, frame)? {
            gate.release()?;
        }
        offset = frame.frame_end;
        records = records.checked_add(1).ok_or(DbError::LimitExceeded("wal recovered records"))?;
        semio_framework_async::yield_once().await;
    }
    let incomplete_active_tx = gate.finish_segment(writable_highest)?;
    if incomplete_active_tx.is_none() {
        gate.advance_segment()?;
    }
    Ok(WalValidatedPrefix { records, next_tx_id: gate.next_tx_id, incomplete_active_tx })
}

pub fn exact_prior_tip(prior: WalPriorChainTip) -> Result<Option<[u8; 32]>, DbError> {
    match prior {
        WalPriorChainTip::Genesis => Ok(None),
        WalPriorChainTip::Verified(tip) => Ok(Some(tip)),
        WalPriorChainTip::RetainedBoundary => Err(DbError::Corrupt("wal cannot initialize an unanchored compacted boundary".to_string())),
    }
}
//#endregion 🔖️Recovery
