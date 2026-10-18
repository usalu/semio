//! 🔖️ Page set and cursors: the history page set and the envelope and frontier cursors. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️HistoryReplay
#[derive(Default)]
pub(super) struct HistoryPageSet {
    pub pages: Vec<Option<Vec<u8>>>,
    pub len: u64,
}

impl db_wal_log::WalImmutableByteSource for HistoryPageSet {
    fn byte_len(&self) -> usize {
        self.len as usize
    }

    fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError> {
        if offset >= limit || limit as u64 > self.len {
            return Err(DbError::Corrupt("history immutable source range".to_string()));
        }
        let bytes = self.page_slice(offset as u64, (limit - offset) as u64)?;
        if bytes.is_empty() {
            return Err(DbError::Corrupt("history immutable source lost bytes".to_string()));
        }
        Ok(bytes)
    }
}

impl HistoryPageSet {
    pub fn byte(&self, offset: u64) -> Result<u8, DbError> {
        if offset >= self.len {
            return Err(DbError::Corrupt("history page cursor exceeded the segment".to_string()));
        }
        let page = usize::try_from(offset / HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history page index"))?;
        let within = usize::try_from(offset % HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history page offset"))?;
        self.pages.get(page).and_then(Option::as_ref).and_then(|bytes| bytes.get(within)).copied().ok_or_else(|| DbError::Corrupt("history page owner is missing admitted bytes".to_string()))
    }

    pub(super) fn page_slice(&self, offset: u64, maximum: u64) -> Result<&[u8], DbError> {
        if offset >= self.len {
            return Ok(&[]);
        }
        let page = usize::try_from(offset / HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history page index"))?;
        let within = usize::try_from(offset % HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history page offset"))?;
        let bytes = self.pages.get(page).and_then(Option::as_ref).ok_or_else(|| DbError::Corrupt("history page owner is missing".to_string()))?;
        let available = bytes.len().checked_sub(within).ok_or_else(|| DbError::Corrupt("history page offset exceeds owner".to_string()))?;
        let take = available.min(usize::try_from(maximum).unwrap_or(usize::MAX));
        Ok(&bytes[within..within + take])
    }

    fn read_varint(&self, pos: &mut u64, end: u64) -> Result<u64, DbError> {
        let mut next = *pos;
        let value = db_wal_log::wal_read_canonical_varint(|| {
            if next >= end {
                return Err(DbError::Corrupt("history varint exceeds its admitted range".to_string()));
            }
            let byte = self.byte(next)?;
            next = next.checked_add(1).ok_or(DbError::LimitExceeded("history varint offset"))?;
            Ok(byte)
        })?;
        *pos = next;
        Ok(value)
    }

    fn read_range(&self, pos: &mut u64, end: u64, maximum: u64) -> Result<std::ops::Range<u64>, DbError> {
        let len = self.read_varint(pos, end)?;
        if len > maximum {
            return Err(DbError::LimitExceeded("history envelope field bytes"));
        }
        let start = *pos;
        let next = start.checked_add(len).ok_or(DbError::LimitExceeded("history envelope field range"))?;
        if next > end {
            return Err(DbError::Corrupt("history envelope field exceeds its frame payload".to_string()));
        }
        *pos = next;
        Ok(start..next)
    }

    fn copy_small<'a>(&self, range: std::ops::Range<u64>, scratch: &'a mut [u8]) -> Result<&'a [u8], DbError> {
        let len = range.end.checked_sub(range.start).ok_or(DbError::LimitExceeded("history copied range"))?;
        if len > HISTORY_REPLAY_PAGE_BYTES {
            return Err(DbError::LimitExceeded("history scratch page bytes"));
        }
        let first = self.page_slice(range.start, len)?;
        scratch[..first.len()].copy_from_slice(first);
        let remaining = usize::try_from(len).map_err(|_| DbError::LimitExceeded("history scratch page bytes"))?.checked_sub(first.len()).ok_or(DbError::LimitExceeded("history scratch page bytes"))?;
        if remaining != 0 {
            let offset = range.start.checked_add(first.len() as u64).ok_or(DbError::LimitExceeded("history scratch page offset"))?;
            let second = self.page_slice(offset, remaining as u64)?;
            if second.len() != remaining {
                return Err(DbError::Corrupt("history scratch page is truncated".to_string()));
            }
            scratch[first.len()..first.len() + remaining].copy_from_slice(second);
        }
        Ok(&scratch[..len as usize])
    }

    fn read_array<const N: usize>(&self, pos: &mut u64, end: u64, scratch: &mut [u8]) -> Result<[u8; N], DbError> {
        let next = pos.checked_add(N as u64).ok_or(DbError::LimitExceeded("history scalar range"))?;
        if next > end {
            return Err(DbError::Corrupt("history scalar exceeds its frame payload".to_string()));
        }
        let owner = self.copy_small(*pos..next, scratch)?;
        *pos = next;
        owner.try_into().map_err(|_| DbError::Corrupt("history scalar length mismatch".to_string()))
    }
}

pub enum HistoryEnvelopeField {
    MutationId,
    Document,
    Actor,
    DependencyCount,
    Dependency,
    ObservedFlag,
    Observed,
    TargetCount,
    TargetSegment,
    DiffSchema,
    DiffPayload,
    InverseSchema,
    InversePayload,
    ClockActor,
    ClockPhysical,
    ClockLogical,
    TrailingFlags,
    TransactionId,
    TransactionTool,
    Verb,
    Line,
    Done,
}

pub struct HistoryEnvelopeCursor {
    pub pos: u64,
    pub end: u64,
    pub field: HistoryEnvelopeField,
    pub dependencies: u64,
    pub target_segments: u64,
    pub verb: bool,
    pub line: bool,
    pub mutation_id: Option<std::ops::Range<u64>>,
}

impl HistoryEnvelopeCursor {
    pub fn new(pages: &HistoryPageSet, offset: u64, len: u64) -> Result<Self, DbError> {
        let end = offset.checked_add(len).ok_or(DbError::LimitExceeded("history command range"))?;
        if end > pages.len {
            return Err(DbError::Corrupt("history command range exceeds retained pages".to_string()));
        }
        Ok(Self { pos: offset, end, field: HistoryEnvelopeField::MutationId, dependencies: 0, target_segments: 0, verb: false, line: false, mutation_id: None })
    }

    pub fn finish(&self) -> Result<HistoryEnvelopeField, DbError> {
        if self.pos != self.end {
            return Err(DbError::Corrupt("history envelope has trailing bytes".to_string()));
        }
        Ok(HistoryEnvelopeField::Done)
    }

    fn skip_text(&mut self, pages: &HistoryPageSet, scratch: &mut [u8]) -> Result<(), DbError> {
        let range = pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FIELD_BYTES as u64)?;
        let scratch = pages.copy_small(range, scratch)?;
        std::str::from_utf8(&scratch).map_err(|_| DbError::Corrupt("history envelope text is not valid utf-8".to_string()))?;
        Ok(())
    }

    pub fn step(&mut self, pages: &HistoryPageSet, scratch: &mut [u8], document: &ArtifactId) -> Result<Option<std::ops::Range<u64>>, DbError> {
        match self.field {
            HistoryEnvelopeField::MutationId => {
                let range = pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FIELD_BYTES as u64)?;
                let value = pages.copy_small(range.clone(), scratch)?;
                std::str::from_utf8(value).map_err(|_| DbError::Corrupt("history operation id is not valid utf-8".to_string()))?;
                self.mutation_id = Some(range);
                self.field = HistoryEnvelopeField::Document;
            }
            HistoryEnvelopeField::Document => {
                let range = pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FIELD_BYTES as u64)?;
                if pages.copy_small(range, scratch)? != document.0.as_bytes() {
                    return Err(DbError::Corrupt("history envelope document differs".to_string()));
                }
                self.field = HistoryEnvelopeField::Actor;
            }
            HistoryEnvelopeField::Actor => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::DependencyCount;
            }
            HistoryEnvelopeField::DependencyCount => {
                self.dependencies = pages.read_varint(&mut self.pos, self.end)?;
                if self.dependencies > HISTORY_REPLAY_MAX_OPERATION_IDS as u64 {
                    return Err(DbError::LimitExceeded("history dependency item credit"));
                }
                self.field = if self.dependencies == 0 { HistoryEnvelopeField::ObservedFlag } else { HistoryEnvelopeField::Dependency };
            }
            HistoryEnvelopeField::Dependency => {
                self.skip_text(pages, scratch)?;
                self.dependencies -= 1;
                if self.dependencies == 0 {
                    self.field = HistoryEnvelopeField::ObservedFlag;
                }
            }
            HistoryEnvelopeField::ObservedFlag => {
                self.field = match pages.read_varint(&mut self.pos, self.end)? {
                    0 => HistoryEnvelopeField::TargetCount,
                    1 => HistoryEnvelopeField::Observed,
                    _ => return Err(DbError::Corrupt("history envelope observed flag is invalid".to_string())),
                };
            }
            HistoryEnvelopeField::Observed => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::TargetCount;
            }
            HistoryEnvelopeField::TargetCount => {
                self.target_segments = pages.read_varint(&mut self.pos, self.end)?;
                if self.target_segments > HISTORY_REPLAY_MAX_OPERATION_IDS as u64 {
                    return Err(DbError::LimitExceeded("history target segment credit"));
                }
                self.field = if self.target_segments == 0 { HistoryEnvelopeField::DiffSchema } else { HistoryEnvelopeField::TargetSegment };
            }
            HistoryEnvelopeField::TargetSegment => {
                self.skip_text(pages, scratch)?;
                self.target_segments -= 1;
                if self.target_segments == 0 {
                    self.field = HistoryEnvelopeField::DiffSchema;
                }
            }
            HistoryEnvelopeField::DiffSchema => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::DiffPayload;
            }
            HistoryEnvelopeField::DiffPayload => {
                pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FRAME_BYTES)?;
                self.field = HistoryEnvelopeField::InverseSchema;
            }
            HistoryEnvelopeField::InverseSchema => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::InversePayload;
            }
            HistoryEnvelopeField::InversePayload => {
                pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FRAME_BYTES)?;
                self.field = HistoryEnvelopeField::ClockActor;
            }
            HistoryEnvelopeField::ClockActor => {
                pages.read_varint(&mut self.pos, self.end)?;
                self.field = HistoryEnvelopeField::ClockPhysical;
            }
            HistoryEnvelopeField::ClockPhysical => {
                pages.read_varint(&mut self.pos, self.end)?;
                self.field = HistoryEnvelopeField::ClockLogical;
            }
            HistoryEnvelopeField::ClockLogical => {
                pages.read_varint(&mut self.pos, self.end)?;
                self.field = HistoryEnvelopeField::TrailingFlags;
            }
            HistoryEnvelopeField::TrailingFlags => {
                let flags = pages.read_varint(&mut self.pos, self.end)?;
                if flags > 0b111 {
                    return Err(DbError::Corrupt("history envelope trailing flags are invalid".to_string()));
                }
                self.verb = flags & 0b10 != 0;
                self.line = flags & 0b100 != 0;
                self.field = match (flags & 0b01 != 0, self.verb, self.line) {
                    (true, _, _) => HistoryEnvelopeField::TransactionId,
                    (false, true, _) => HistoryEnvelopeField::Verb,
                    (false, false, true) => HistoryEnvelopeField::Line,
                    (false, false, false) => self.finish()?,
                };
            }
            HistoryEnvelopeField::TransactionId => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::TransactionTool;
            }
            HistoryEnvelopeField::TransactionTool => {
                self.skip_text(pages, scratch)?;
                self.field = if self.verb { HistoryEnvelopeField::Verb } else if self.line { HistoryEnvelopeField::Line } else { self.finish()? };
            }
            HistoryEnvelopeField::Verb => {
                self.skip_text(pages, scratch)?;
                self.field = if self.line { HistoryEnvelopeField::Line } else { self.finish()? };
            }
            HistoryEnvelopeField::Line => {
                self.skip_text(pages, scratch)?;
                self.field = self.finish()?;
            }
            HistoryEnvelopeField::Done => return Ok(self.mutation_id.take()),
        }
        Ok(None)
    }
}

pub enum HistoryFrontierField {
    Document,
    Head,
    Commit,
    Chain,
    Epoch,
}

pub struct HistoryFrontierCursor {
    pub pos: u64,
    pub end: u64,
    pub field: HistoryFrontierField,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub chain_hash: [u8; 32],
}

impl HistoryFrontierCursor {
    pub fn new(pages: &HistoryPageSet, offset: u64, len: u64) -> Result<Self, DbError> {
        let end = offset.checked_add(len).ok_or(DbError::LimitExceeded("history frontier range"))?;
        if end > pages.len {
            return Err(DbError::Corrupt("history frontier range exceeds retained pages".to_string()));
        }
        Ok(Self { pos: offset, end, field: HistoryFrontierField::Document, head_seq: 0, commit_seq: 0, chain_hash: [0; 32] })
    }

    pub fn step(&mut self, pages: &HistoryPageSet, scratch: &mut [u8], document: &ArtifactId) -> Result<Option<(u64, u64, [u8; 32], u64)>, DbError> {
        match self.field {
            HistoryFrontierField::Document => {
                let range = pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FIELD_BYTES as u64)?;
                if pages.copy_small(range, scratch)? != document.0.as_bytes() {
                    return Err(DbError::Corrupt("history frontier document differs".to_string()));
                }
                self.field = HistoryFrontierField::Head;
            }
            HistoryFrontierField::Head => {
                self.head_seq = u64::from_le_bytes(pages.read_array(&mut self.pos, self.end, scratch)?);
                self.field = HistoryFrontierField::Commit;
            }
            HistoryFrontierField::Commit => {
                self.commit_seq = u64::from_le_bytes(pages.read_array(&mut self.pos, self.end, scratch)?);
                self.field = HistoryFrontierField::Chain;
            }
            HistoryFrontierField::Chain => {
                self.chain_hash = pages.read_array(&mut self.pos, self.end, scratch)?;
                self.field = HistoryFrontierField::Epoch;
            }
            HistoryFrontierField::Epoch => {
                let epoch = u64::from_le_bytes(pages.read_array(&mut self.pos, self.end, scratch)?);
                if self.pos != self.end {
                    return Err(DbError::Corrupt("history frontier has trailing bytes".to_string()));
                }
                return Ok(Some((self.head_seq, self.commit_seq, self.chain_hash, epoch)));
            }
        }
        Ok(None)
    }
}
//#endregion 🔖️HistoryReplay
//#endregion 🔖️Ids
