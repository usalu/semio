//! 🔐️ Authenticated source and replay cursor: the authenticated source and the replay cursor. See [`super`].

use super::*;

//#region 🔖️Recovery
pub enum WalAuthenticatedStep {
    Yield,
    Committed,
    Done,
}

/// 🔐️ Owns the one immutable source shared by physical authentication and logical admission.
pub struct WalAuthenticatedSource<S> {
    pub source: S,
    pub gate: WalTransactionGate,
    pub chain: Option<WalSegmentChain>,
    pub segment: u64,
    pub previous: WalPriorChainTip,
    pub offset: usize,
    pub verified: bool,
    done: bool,
}

impl<S: WalImmutableByteSource> WalAuthenticatedSource<S> {
    pub fn new(source: S, gate: WalTransactionGate, segment: u64, previous: WalPriorChainTip) -> Self {
        Self { source, gate, chain: None, segment, previous, offset: protocol::format::HEADER_SIZE, verified: false, done: false }
    }

    pub fn source(&self) -> &S {
        &self.source
    }

    pub fn verify_step(&mut self, document: &ArtifactId, control: &mut WalCursorControl) -> Result<bool, DbError> {
        control.check()?;
        if self.verified {
            return Ok(true);
        }
        if self.chain.is_none() {
            control.grant()?;
            self.chain = Some(WalSegmentChain::new(&self.source, self.segment, self.previous)?);
            return Ok(false);
        }
        self.verified = self.chain.as_mut().ok_or(DbError::Closed)?.step(&self.source, self.source.byte_len(), document, control)?;
        Ok(self.verified)
    }

    pub fn next_step(&mut self, control: &mut WalCursorControl) -> Result<WalAuthenticatedStep, DbError> {
        control.check()?;
        if !self.verified {
            return Err(DbError::Corrupt("wal source has not completed authentication".to_string()));
        }
        if self.done {
            return Ok(WalAuthenticatedStep::Done);
        }
        if self.gate.ready.is_some() {
            return Err(DbError::Corrupt("wal authenticated transaction remains unfinished".to_string()));
        }
        match wal_next_verified_page_frame(&self.source, &mut self.offset, self.source.byte_len(), control)? {
            WalVerifiedFrameStep::Frame(frame) => {
                let committed = self.gate.push(&self.source, frame)?;
                self.offset = frame.frame_end;
                Ok(if committed { WalAuthenticatedStep::Committed } else { WalAuthenticatedStep::Yield })
            }
            WalVerifiedFrameStep::PhysicalCommit => Ok(WalAuthenticatedStep::Yield),
            WalVerifiedFrameStep::Done => {
                self.gate.advance_segment()?;
                self.done = true;
                Ok(WalAuthenticatedStep::Done)
            }
        }
    }

    pub fn committed_frame(&self, index: usize) -> Option<WalRecordFrame> {
        self.gate.committed_frame(index)
    }

    pub fn finish_transaction(&mut self) -> Result<(), DbError> {
        self.gate.release()
    }

    pub fn finish(self) -> Result<(S, WalTransactionGate, [u8; 32]), Self> {
        match (self.done, self.chain.as_ref().and_then(|chain| chain.tip)) {
            (true, Some(tip)) => Ok((self.source, self.gate, tip)),
            _ => Err(self),
        }
    }

    /// 🛑️ Destroys verification and transaction continuation, returning only raw ownership for retirement.
    pub fn abort_into_source(self) -> S {
        self.source
    }
}

/// 🔁️ Decodes every `WAL_*` record across a document's ENTIRE WAL (every sealed segment in
/// full, plus the active segment), in segment then on-disk order. Each segment's complete retained
/// bytes must end in a verified commit; torn tails are rejected. Page-bounded verification checks
/// every frame and commit before releasing that segment's records, carrying the validated chain
/// tip into the next exact document/index header. Recovery/truncation belongs to the WAL opener.
/// A compacted suffix has no authenticated preceding tip; use `open_genesis` for a genesis proof.
pub struct WalReplayCursor<'storage, S: db_io_core::WalStorage> {
    pub storage: &'storage S,
    pub document: ArtifactId,
    pub segments: db_io_core::DbIoU64List,
    pub segment: usize,
    pub pages: Option<db_io_core::DbIoPages>,
    pub offset: usize,
    pub(super) trusted_len: usize,
    pub(super) validation: Option<WalSegmentChain>,
    pub(super) decoder: Option<WalRetainedRecordDecoder>,
    pub(super) previous_tip: Option<[u8; 32]>,
    genesis_required: bool,
    pub failed: bool,
    pub control: WalCursorControl,
    pub closed: bool,
}

pub enum WalReplayStep {
    Record(WalRecord),
    Yield,
    Done,
}

impl<'storage, S: db_io_core::WalStorage> WalReplayCursor<'storage, S> {
    /// 🔋️ Replenishes one bounded caller-owned replay opportunity without replacing the cursor.
    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        self.control.replenish(deadline, fuel)
    }

    pub async fn open(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<Self, DbError> {
        let segments = storage.list_segments(document).await?;
        Ok(Self {
            storage,
            document: document.clone(),
            segments,
            segment: 0,
            pages: None,
            offset: protocol::format::HEADER_SIZE,
            trusted_len: 0,
            validation: None,
            decoder: None,
            previous_tip: None,
            genesis_required: false,
            failed: false,
            control,
            closed: false,
        })
    }

    /// 📍 Starts raw replay at one exact retained segment boundary.
    pub async fn open_at_segment(storage: &'storage S, document: &ArtifactId, segment_index: u64, control: WalCursorControl) -> Result<Self, DbError> {
        let mut cursor = Self::open(storage, document, control).await?;
        let Some(segment) = cursor.segments.as_slice().iter().position(|index| *index == segment_index) else {
            while cursor.close_owner_step()? {}
            return Err(DbError::NotFound(format!("WAL segment {segment_index}")));
        };
        cursor.segment = segment;
        Ok(cursor)
    }

    /// 🌱️ Requires the retained chain to start at genesis rather than trusting a compacted boundary.
    pub async fn open_genesis(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<Self, DbError> {
        let mut cursor = Self::open(storage, document, control).await?;
        cursor.genesis_required = true;
        Ok(cursor)
    }

    pub(super) async fn open_segment(&mut self) -> Result<bool, DbError> {
        let Some(index) = self.segments.as_slice().get(self.segment).copied() else { return Ok(false) };
        self.control.grant()?;
        let first = self.segments.as_slice()[0];
        if (self.genesis_required && first != 0) || first.checked_add(self.segment as u64) != Some(index) {
            return Err(DbError::Corrupt("wal segment sequence differs from its required boundary".to_string()));
        }
        let len = self.storage.segment_len(&self.document, index).await?;
        self.pages = Some(self.storage.read(&self.document, index, pack::ByteRange { offset: 0, len }).await?);
        let pages = self.pages.as_ref().ok_or_else(|| DbError::Internal("wal replay lost admitted pages".to_string()))?;
        self.offset = protocol::format::HEADER_SIZE;
        self.trusted_len = pages.len();
        let previous_tip = match self.previous_tip {
            Some(tip) => WalPriorChainTip::Verified(tip),
            None if index == 0 => WalPriorChainTip::Genesis,
            None => WalPriorChainTip::RetainedBoundary,
        };
        self.validation = Some(WalSegmentChain::new(pages, index, previous_tip)?);
        Ok(true)
    }

    pub(super) async fn close_segment_step(&mut self) -> Result<bool, DbError> {
        self.control.grant()?;
        if let Some(pages) = self.pages.as_mut() {
            if pages.close_step()?.is_some() {
                return Ok(true);
            }
        }
        self.pages = None;
        self.trusted_len = 0;
        Ok(false)
    }

    pub async fn next_step(&mut self) -> Result<WalReplayStep, DbError> {
        if self.failed {
            return Err(DbError::Corrupt("wal replay remains failed until closed".to_string()));
        }
        let result = self.next_validated_step().await;
        let interrupted = matches!(&result, Err(DbError::LimitExceeded("wal cursor fuel"))) || matches!(&result, Err(DbError::Unavailable(message)) if message == "wal cursor cancelled" || message == "wal cursor deadline reached");
        if result.is_err() && !interrupted {
            self.failed = true;
        }
        result
    }

    async fn next_validated_step(&mut self) -> Result<WalReplayStep, DbError> {
        self.control.check()?;
        if self.closed {
            return Ok(WalReplayStep::Done);
        }
        if self.pages.is_none() {
            return Ok(if self.open_segment().await? { WalReplayStep::Yield } else { WalReplayStep::Done });
        }
        if let Some(validation) = self.validation.as_mut() {
            let pages = self.pages.as_ref().ok_or_else(|| DbError::Internal("wal chain lost segment pages".to_string()))?;
            if validation.step(pages, self.trusted_len, &self.document, &mut self.control)? {
                self.previous_tip = validation.tip;
                self.validation = None;
            }
            return Ok(WalReplayStep::Yield);
        }
        if self.offset == self.trusted_len {
            if self.close_segment_step().await? {
                return Ok(WalReplayStep::Yield);
            }
            self.segment += 1;
            return Ok(WalReplayStep::Yield);
        }
        let pages = self.pages.as_ref().ok_or_else(|| DbError::Internal("wal replay lost segment pages".to_string()))?;
        if self.decoder.is_none() {
            if let WalVerifiedFrameStep::Frame(frame) = wal_next_verified_page_frame(pages, &mut self.offset, self.trusted_len, &mut self.control)? {
                self.decoder = Some(WalRetainedRecordDecoder::new(frame));
            }
            return Ok(WalReplayStep::Yield);
        }
        let decoder = self.decoder.as_mut().ok_or_else(|| DbError::Internal("wal replay lost retained decoder".to_string()))?;
        if let Some(record) = decoder.step(pages, &mut self.control)? {
            self.offset = decoder.frame.frame_end;
            self.decoder = None;
            return Ok(WalReplayStep::Record(record));
        }
        Ok(WalReplayStep::Yield)
    }

    #[cfg(test)]
    pub async fn next(&mut self) -> Result<Option<WalRecord>, DbError> {
        loop {
            match self.next_step().await? {
                WalReplayStep::Record(record) => return Ok(Some(record)),
                WalReplayStep::Yield => {}
                WalReplayStep::Done => return Ok(None),
            }
        }
    }

    pub fn close_owner_step(&mut self) -> Result<bool, DbError> {
        if let Some(decoder) = self.decoder.as_mut() {
            if decoder.close_owner_step()? {
                return Ok(true);
            }
        }
        self.decoder = None;
        if let Some(pages) = self.pages.as_mut() {
            if pages.close_step()?.is_some() {
                return Ok(true);
            }
        }
        self.pages = None;
        self.validation = None;
        self.previous_tip = None;
        self.trusted_len = 0;
        if self.segments.close_step() {
            return Ok(true);
        }
        if self.closed {
            return Ok(false);
        }
        self.closed = true;
        Ok(true)
    }

    pub async fn close_step(&mut self) -> Result<bool, DbError> {
        if self.close_owner_step()? {
            return Ok(true);
        }
        Ok(false)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed && self.pages.is_none() && self.validation.is_none() && self.decoder.is_none() && self.segments.terminal_is_empty()
    }
}
//#endregion 🔖️Recovery
