//! ⛓️ Transaction gate and segment chain: the transaction gate, the recovery report and the segment chain. See [`super`].

use super::*;

//#region 🔖️Recovery
/// 🧮️ Most body records one WAL transaction holds: a declared-legal document batch's commands
/// (`protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES`) plus its frontier. `ArtifactWal::preflight_submit`
/// refuses a larger transaction before any I/O and `WalTransactionGate` holds up to exactly this many frames, so
/// every transaction the writer admits is one recovery and replay read back (it held 64 while the writer bounded a
/// transaction only by the readable segment: a committed batch of 64 or more envelopes left its document unopenable).
pub const WAL_TRANSACTION_RECORDS_MAX: usize = protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES + 1;

pub struct WalTransactionGate {
    pub transaction_id: Option<u64>,
    pub ready: Option<u64>,
    pub frames: Vec<WalRecordFrame>,
    pub next_tx_id: u64,
    header_seen: bool,
    transaction_seen: bool,
}

impl WalTransactionGate {
    pub fn new() -> Self {
        Self { transaction_id: None, ready: None, frames: Vec::new(), next_tx_id: 1, header_seen: false, transaction_seen: false }
    }

    pub(super) fn clear_frames(&mut self) {
        self.frames.clear();
    }

    pub fn committed_frame(&self, index: usize) -> Option<WalRecordFrame> {
        self.ready?;
        self.frames.get(index).copied()
    }

    pub fn push(&mut self, pages: &dyn WalImmutableByteSource, frame: WalRecordFrame) -> Result<bool, DbError> {
        if self.ready.is_some() {
            return Err(DbError::Corrupt("wal committed transaction remains borrowed".to_string()));
        }
        if frame.kind == WAL_SEGMENT_HEADER {
            if self.header_seen || self.transaction_id.is_some() {
                return Err(DbError::Corrupt("wal logical segment header repeated".to_string()));
            }
            self.header_seen = true;
            return Ok(false);
        }
        if !self.header_seen {
            return Err(DbError::Corrupt("wal logical segment header missing".to_string()));
        }
        let mut reader = WalPageReader::new(pages, frame.payload_start, frame.payload_end)?;
        match frame.kind {
            WAL_TX_BEGIN => {
                let tx_id = u64::from_le_bytes(reader.array()?);
                if reader.position != reader.limit || self.transaction_id.is_some() || tx_id < self.next_tx_id || (self.transaction_seen && tx_id != self.next_tx_id) {
                    return Err(DbError::Corrupt("wal transaction begin is nested or out of sequence".to_string()));
                }
                self.next_tx_id = tx_id.checked_add(1).ok_or(DbError::LimitExceeded("wal transaction sequence"))?;
                self.transaction_id = Some(tx_id);
                self.transaction_seen = true;
                Ok(false)
            }
            WAL_TX_COMMIT | WAL_TX_ABORT => {
                let tx_id = u64::from_le_bytes(reader.array()?);
                let count = if frame.kind == WAL_TX_COMMIT { Some(u32::from_le_bytes(reader.array()?)) } else { None };
                if reader.position != reader.limit || self.transaction_id != Some(tx_id) || count.is_some_and(|count| usize::try_from(count).ok() != Some(self.frames.len())) {
                    return Err(DbError::Corrupt("wal logical terminal id or count differs".to_string()));
                }
                self.transaction_id = None;
                if count.is_some() {
                    self.ready = Some(tx_id);
                    Ok(true)
                } else {
                    self.clear_frames();
                    Ok(false)
                }
            }
            WAL_COMMAND..=WAL_MIGRATION => {
                if self.transaction_id.is_none() {
                    return Err(DbError::Corrupt("wal body outside a transaction".to_string()));
                }
                if self.frames.len() >= WAL_TRANSACTION_RECORDS_MAX {
                    return Err(DbError::LimitExceeded("wal transaction records"));
                }
                self.frames.try_reserve(1).map_err(|_| DbError::LimitExceeded("wal transaction records"))?;
                self.frames.push(frame);
                Ok(false)
            }
            _ => Err(DbError::Corrupt("unknown wal logical record kind".to_string())),
        }
    }

    pub fn release(&mut self) -> Result<(), DbError> {
        self.ready.take().ok_or_else(|| DbError::Corrupt("wal has no committed transaction to finish".to_string()))?;
        self.clear_frames();
        Ok(())
    }

    pub fn finish_segment(&self, writable_highest: bool) -> Result<Option<u64>, DbError> {
        if !self.header_seen || self.ready.is_some() {
            return Err(DbError::Corrupt("wal logical segment is not drained".to_string()));
        }
        if self.transaction_id.is_some() && !writable_highest {
            return Err(DbError::Corrupt("wal logical transaction crosses a sealed boundary".to_string()));
        }
        Ok(self.transaction_id)
    }

    pub fn advance_segment(&mut self) -> Result<(), DbError> {
        self.finish_segment(false)?;
        self.header_seen = false;
        Ok(())
    }
}

/// 📋️ What `ArtifactWal::open` found while recovering a document's WAL.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WalRecoveryReport {
    pub segments_seen: u64,
    pub records_replayed: u64,
    /// ✂️ Bytes discarded off the active segment's tail because they came after the last
    /// trusted commit (a torn write from a crash mid-append) — `0` on a clean recovery.
    pub torn_tail_bytes: u64,
    /// 🧯️ Incomplete active transaction durably aborted before this opener returned.
    pub recovered_abort_tx_id: Option<u64>,
}

struct WalChainFrame {
    pub(super) start: usize,
    pub body: usize,
    pub end: usize,
    pub next: usize,
    pub position: usize,
    pub kind: u8,
    digest: semio_framework_hash::Hasher,
    pub(super) crc: protocol::codec::Crc32cCursor,
}

pub struct WalSegmentChain {
    pub offset: usize,
    pub frame: Option<WalChainFrame>,
    pending: semio_framework_hash::Hasher,
    records_len: u64,
    pub record_count: u32,
    next_commit: u64,
    pub last_commit_offset: u64,
    pub tip: Option<[u8; 32]>,
    pub(super) previous_tip: WalPriorChainTip,
    pub segment: u64,
    header_seen: bool,
}

#[derive(Clone, Copy)]
pub enum WalPriorChainTip {
    Genesis,
    RetainedBoundary,
    Verified([u8; 32]),
}

impl WalSegmentChain {
    pub fn new(pages: &dyn WalImmutableByteSource, segment: u64, previous_tip: WalPriorChainTip) -> Result<Self, DbError> {
        let bytes = WalPageReader::new(pages, 0, protocol::format::HEADER_SIZE)?.array::<{ protocol::format::HEADER_SIZE }>()?;
        let header = protocol::format::parse_header_bytes(&bytes).map_err(protocol_err)?;
        if header.required_flags != protocol::wire::REQUIRED_HASH_CHAIN || header.optional_flags != 0 || header.version_minor != protocol::format::FORMAT_VERSION_MINOR {
            return Err(DbError::Corrupt("wal requires its exact hash-chain format".to_string()));
        }
        let mut pending = semio_framework_hash::Hasher::new();
        pending.update(semio_framework_hash::hash(&bytes).as_bytes());
        Ok(Self { offset: protocol::format::HEADER_SIZE, frame: None, pending, records_len: 0, record_count: 0, next_commit: 1, last_commit_offset: 0, tip: None, previous_tip, segment, header_seen: false })
    }

    fn check_segment_header(&mut self, pages: &dyn WalImmutableByteSource, frame: &WalChainFrame, document: &ArtifactId) -> Result<(), DbError> {
        if frame.kind != WAL_SEGMENT_HEADER || self.header_seen || frame.start != protocol::format::HEADER_SIZE {
            return Err(DbError::Corrupt("wal segment header is missing or repeated".to_string()));
        }
        let mut reader = WalPageReader::new(pages, frame.body + 2, frame.end)?;
        if reader.varint()? != document.0.len() as u64 {
            return Err(DbError::Corrupt("wal segment document differs".to_string()));
        }
        for byte in document.0.as_bytes() {
            if reader.byte()? != *byte {
                return Err(DbError::Corrupt("wal segment document differs".to_string()));
            }
        }
        if u64::from_le_bytes(reader.array()?) != self.segment {
            return Err(DbError::Corrupt("wal segment index differs".to_string()));
        }
        let previous = match reader.byte()? {
            0 => None,
            1 => Some(reader.array::<32>()?),
            _ => return Err(DbError::Corrupt("wal previous tip marker differs".to_string())),
        };
        let matches = match self.previous_tip {
            WalPriorChainTip::Genesis => previous.is_none(),
            WalPriorChainTip::RetainedBoundary => previous.is_some(),
            WalPriorChainTip::Verified(tip) => previous == Some(tip),
        };
        if !matches || reader.position != frame.end {
            return Err(DbError::Corrupt("wal previous committed chain tip differs".to_string()));
        }
        self.header_seen = true;
        Ok(())
    }

    pub fn step(&mut self, pages: &dyn WalImmutableByteSource, trusted_len: usize, document: &ArtifactId, control: &mut WalCursorControl) -> Result<bool, DbError> {
        control.grant()?;
        if self.frame.is_none() {
            if self.offset == trusted_len {
                if !self.header_seen || self.tip.is_none() || self.record_count != 0 || self.records_len != 0 || self.last_commit_offset.checked_add(protocol::format::COMMIT_FRAME_LEN) != Some(trusted_len as u64) {
                    return Err(DbError::Corrupt("wal segment ends outside a verified commit".to_string()));
                }
                return Ok(true);
            }
            let mut reader = WalPageReader::new(pages, self.offset, trusted_len)?;
            let length = reader.varint()?;
            check_len(length, protocol::ProtocolLimits::default().max_frame_len, "wal chain frame")?;
            let body = reader.position;
            let end = body.checked_add(usize::try_from(length).map_err(|_| DbError::LimitExceeded("wal chain frame length"))?).ok_or(DbError::LimitExceeded("wal chain frame end"))?;
            let next = end.checked_add(8).ok_or(DbError::LimitExceeded("wal chain trailer"))?;
            if length < 2 || next > trusted_len {
                return Err(DbError::Corrupt("wal chain frame exceeds committed bytes".to_string()));
            }
            let kind = reader.byte()?;
            let flags = reader.byte()?;
            if flags != protocol::wire::FRAME_FLAG_CRITICAL {
                return Err(DbError::Corrupt("wal chain frame flags differ".to_string()));
            }
            self.frame = Some(WalChainFrame { start: self.offset, body, end, next, position: self.offset, kind, digest: semio_framework_hash::Hasher::new(), crc: protocol::codec::Crc32cCursor::new() });
        }
        let frame = self.frame.as_mut().ok_or_else(|| DbError::Internal("wal chain frame disappeared".to_string()))?;
        let fragment = pages.fragment_at(frame.position, frame.next)?;
        let count = fragment.len().min(db_io_core::DB_IO_PAGE_BYTES);
        if count == 0 {
            return Err(DbError::Corrupt("wal chain made no byte progress".to_string()));
        }
        let bytes = &fragment[..count];
        frame.digest.update(bytes);
        let crc_start = frame.position.max(frame.body);
        let crc_end = (frame.position + count).min(frame.end);
        if crc_start < crc_end {
            frame.crc.update_page(&bytes[crc_start - frame.position..crc_end - frame.position]);
        }
        frame.position += count;
        if frame.position != frame.next {
            return Ok(false);
        }
        let frame = self.frame.take().ok_or_else(|| DbError::Internal("wal chain frame disappeared".to_string()))?;
        let mut trailer = WalPageReader::new(pages, frame.end, frame.next)?;
        if u32::from_le_bytes(trailer.array()?) != frame.crc.finish() || u32::from_le_bytes(trailer.array()?) as usize != frame.next - frame.start {
            return Err(DbError::Corrupt("wal chain crc or frame length differs".to_string()));
        }
        if frame.kind == protocol::wire::REC_COMMIT {
            if frame.end - frame.body - 2 != protocol::format::COMMIT_PAYLOAD_LEN || frame.next - frame.start != protocol::format::COMMIT_FRAME_LEN as usize {
                return Err(DbError::Corrupt("wal chain commit framing differs".to_string()));
            }
            let payload = WalPageReader::new(pages, frame.body + 2, frame.end)?.array::<{ protocol::format::COMMIT_PAYLOAD_LEN }>()?;
            let commit = protocol::format::parse_commit_payload(&payload).map_err(protocol_err)?;
            if commit.commit_seq != self.next_commit
                || commit.prev_commit_offset != self.last_commit_offset
                || commit.record_count != self.record_count
                || commit.records_len != self.records_len
                || payload[28..32] != [0; 4]
                || commit.chain_hash != *self.pending.finalize().as_bytes()
            {
                return Err(DbError::Corrupt("wal committed hash chain differs".to_string()));
            }
            self.tip = Some(commit.chain_hash);
            self.pending = semio_framework_hash::Hasher::new();
            self.pending.update(&commit.chain_hash);
            self.record_count = 0;
            self.records_len = 0;
            self.last_commit_offset = frame.start as u64;
            self.next_commit = self.next_commit.checked_add(1).ok_or(DbError::LimitExceeded("wal commit sequence"))?;
        } else {
            if !is_wal_record_kind(frame.kind) {
                return Err(DbError::Corrupt("wal chain contains an unknown record".to_string()));
            }
            if !self.header_seen || frame.kind == WAL_SEGMENT_HEADER {
                self.check_segment_header(pages, &frame, document)?;
            }
            self.pending.update(frame.digest.finalize().as_bytes());
            self.record_count = self.record_count.checked_add(1).ok_or(DbError::LimitExceeded("wal commit record count"))?;
            self.records_len = self.records_len.checked_add((frame.next - frame.start) as u64).ok_or(DbError::LimitExceeded("wal commit record bytes"))?;
        }
        self.offset = frame.next;
        Ok(false)
    }
}
//#endregion 🔖️Recovery
