//! 📖️ Page source, reader and frames: the page sources, the page reader, the retained record decoder and the verified frame steps. See [`super`].

use super::*;

//#region 🔖️Recovery
/// 🚨️ Maps `protocol::ProtocolError` (this crate never leaks it in a public signature, per
/// the family's `DbError`-only rule) onto the closest `DbError` variant.
// 🚫️async: E4 fn-pointer slot
pub fn protocol_err(err: protocol::ProtocolError) -> DbError {
    match err {
        protocol::ProtocolError::Pack(pack_error) => DbError::from(pack_error),
        protocol::ProtocolError::Io(message) => DbError::Io(message),
        protocol::ProtocolError::LimitExceeded(what) => DbError::LimitExceeded(what),
        other => DbError::Corrupt(other.to_string()),
    }
}

pub(super) struct WalPageSource<'pages>(pub(super) &'pages db_io_core::DbIoPages);

impl semio_framework_pack_source::PackSource for WalPageSource<'_> {
    async fn len(&self) -> u64 {
        self.0.len() as u64
    }

    async fn read_at(&self, offset: u64, output: &mut [u8]) -> Result<usize, protocol::codec::PackError> {
        let offset = usize::try_from(offset).map_err(|_| protocol::codec::PackError::Refusal(protocol::codec::PackRefusal::Truncated(offset)))?;
        if offset > self.0.len() {
            return Err(protocol::codec::PackError::Refusal(protocol::codec::PackRefusal::Truncated(offset as u64)));
        }
        let mut base = 0usize;
        let mut written = 0usize;
        for fragment in self.0.fragments() {
            let end = base + fragment.len();
            if end <= offset {
                base = end;
                continue;
            }
            let start = offset.saturating_sub(base);
            let count = (output.len() - written).min(fragment.len() - start);
            output[written..written + count].copy_from_slice(&fragment[start..start + count]);
            written += count;
            base = end;
            if written == output.len() {
                break;
            }
        }
        Ok(written)
    }
}

pub trait WalImmutableByteSource: Sync {
    fn byte_len(&self) -> usize;
    fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError>;
}

impl<S: WalImmutableByteSource + ?Sized> WalImmutableByteSource for &S {
    fn byte_len(&self) -> usize {
        (**self).byte_len()
    }
    fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError> {
        (**self).fragment_at(offset, limit)
    }
}

impl WalImmutableByteSource for db_io_core::DbIoPages {
    fn byte_len(&self) -> usize {
        self.len()
    }

    fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError> {
        if offset >= limit || limit > self.len() {
            return Err(DbError::Corrupt("wal immutable source range".to_string()));
        }
        let first = self.page(0).ok_or_else(|| DbError::Corrupt("wal immutable source lost first page".to_string()))?.len();
        let (page, local) = if offset < first {
            (0, offset)
        } else {
            let rest = offset - first;
            (1 + rest / db_io_core::DB_IO_PAGE_BYTES, rest % db_io_core::DB_IO_PAGE_BYTES)
        };
        let bytes = self.page(u8::try_from(page).map_err(|_| DbError::LimitExceeded("wal immutable source page"))?).ok_or_else(|| DbError::Corrupt("wal immutable source lost page".to_string()))?;
        let remaining = bytes.len().checked_sub(local).ok_or_else(|| DbError::Corrupt("wal immutable source offset".to_string()))?;
        let count = remaining.min(limit - offset);
        if count == 0 {
            return Err(DbError::Corrupt("wal immutable source empty fragment".to_string()));
        }
        Ok(&bytes[local..local + count])
    }
}

pub struct WalPageReader<'pages> {
    pub pages: &'pages dyn WalImmutableByteSource,
    pub position: usize,
    pub limit: usize,
}

impl<'pages> WalPageReader<'pages> {
    pub fn new(pages: &'pages dyn WalImmutableByteSource, position: usize, limit: usize) -> Result<Self, DbError> {
        if position > limit || limit > pages.byte_len() {
            return Err(DbError::Corrupt("wal retained reader authority".to_string()));
        }
        Ok(Self { pages, position, limit })
    }

    pub fn fragment(&self) -> Result<&'pages [u8], DbError> {
        self.pages.fragment_at(self.position, self.limit)
    }

    pub fn byte(&mut self) -> Result<u8, DbError> {
        let byte = self.fragment()?[0];
        self.position += 1;
        Ok(byte)
    }

    pub(super) fn varint(&mut self) -> Result<u64, DbError> {
        let start = self.position;
        let result = wal_read_canonical_varint(|| self.byte());
        if result.is_err() {
            self.position = start;
        }
        result
    }

    pub(super) fn array<const N: usize>(&mut self) -> Result<[u8; N], DbError> {
        let mut output = [0u8; N];
        let mut written = 0usize;
        while written < N {
            let fragment = self.fragment()?;
            let count = (N - written).min(fragment.len());
            output[written..written + count].copy_from_slice(&fragment[..count]);
            self.position += count;
            written += count;
        }
        Ok(output)
    }

    pub(super) fn text(&mut self) -> Result<db_io_core::DbIoText, DbError> {
        let len = self.varint()?;
        check_len(len, db_io_core::DbIoText::maximum_capacity() as u64, "wal retained text")?;
        let mut bytes = [0u8; 1024];
        let mut written = 0usize;
        while written < len as usize {
            let fragment = self.fragment()?;
            let count = (len as usize - written).min(fragment.len());
            bytes[written..written + count].copy_from_slice(&fragment[..count]);
            self.position += count;
            written += count;
        }
        let value = std::str::from_utf8(&bytes[..written]).map_err(|_| DbError::Corrupt("wal retained text is not utf-8".to_string()))?;
        db_io_core::DbIoText::try_from_str(value)
    }

    fn string(&mut self) -> Result<String, DbError> {
        let mut text = self.text()?;
        let output = text.as_str().to_string();
        text.close_step();
        Ok(output)
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn wal_crc_range(pages: &db_io_core::DbIoPages, start: usize, len: usize, control: &mut WalCursorControl) -> Result<u32, DbError> {
    let end = start.checked_add(len).ok_or(DbError::LimitExceeded("wal frame crc range"))?;
    if end > pages.len() {
        return Err(DbError::Corrupt("wal frame crc exceeds segment".to_string()));
    }
    let mut crc = protocol::codec::Crc32cCursor::new();
    let mut base = 0usize;
    for fragment in pages.fragments() {
        let fragment_end = base + fragment.len();
        if fragment_end <= start {
            base = fragment_end;
            continue;
        }
        if base >= end {
            break;
        }
        control.grant()?;
        let local_start = start.saturating_sub(base);
        let local_end = fragment.len().min(end - base);
        crc.update_page(&fragment[local_start..local_end]);
        base = fragment_end;
    }
    Ok(crc.finish())
}

fn wal_decode_frontier(reader: &mut WalPageReader<'_>) -> Result<Frontier, DbError> {
    let document = ArtifactId(reader.string()?);
    let head_seq = u64::from_le_bytes(reader.array()?);
    let commit_seq = u64::from_le_bytes(reader.array()?);
    let chain_hash = reader.array()?;
    let epoch = u64::from_le_bytes(reader.array()?);
    Ok(Frontier { document, head_seq, commit_seq, chain_hash, epoch })
}

fn wal_decode_scalar_record(kind: u8, reader: &mut WalPageReader<'_>) -> Result<WalRecord, DbError> {
    let record = match kind {
        WAL_SEGMENT_HEADER => {
            let document = ArtifactId(reader.string()?);
            let segment_index = u64::from_le_bytes(reader.array()?);
            let prev_chain_hash = match reader.byte()? {
                0 => None,
                1 => Some(reader.array()?),
                _ => return Err(DbError::Corrupt("wal segment hash tag".to_string())),
            };
            WalRecord::SegmentHeader { document, segment_index, prev_chain_hash }
        }
        WAL_TX_BEGIN => WalRecord::TxBegin { tx_id: u64::from_le_bytes(reader.array()?) },
        WAL_TX_COMMIT => WalRecord::TxCommit { tx_id: u64::from_le_bytes(reader.array()?), record_count: u32::from_le_bytes(reader.array()?) },
        WAL_TX_ABORT => WalRecord::TxAbort { tx_id: u64::from_le_bytes(reader.array()?) },
        WAL_PAYLOAD => match reader.byte()? {
            1 => WalRecord::Payload(WalPayloadRef::CasRef(ContentHash(reader.array()?))),
            _ => return Err(DbError::Corrupt("wal payload tag".to_string())),
        },
        WAL_FRONTIER => WalRecord::Frontier(wal_decode_frontier(reader)?),
        WAL_SNAPSHOT_PUB => WalRecord::SnapshotPub { generation: u64::from_le_bytes(reader.array()?), frontier: wal_decode_frontier(reader)? },
        WAL_LEASE => WalRecord::Lease { resource: reader.text()?, holder: reader.text()?, fence: u64::from_le_bytes(reader.array()?), expires_at_ms: u64::from_le_bytes(reader.array()?) },
        _ => return Err(DbError::Corrupt(format!("unknown wal record kind {kind:#x}"))),
    };
    if reader.position != reader.limit {
        return Err(DbError::Corrupt("wal record retained decoder left trailing payload".to_string()));
    }
    Ok(record)
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct WalRecordFrame {
    pub kind: u8,
    pub payload_start: usize,
    pub payload_end: usize,
    pub frame_end: usize,
}

enum WalRetainedDecodeState {
    Start,
    Bytes { position: usize, end: usize },
    Index { position: usize, remaining: usize },
    Done,
}

pub(super) struct WalRetainedRecordDecoder {
    pub frame: WalRecordFrame,
    pub state: WalRetainedDecodeState,
    pub writer: Option<db_io_core::DbIoPageWriter>,
    pub(super) run_ids: Option<db_io_core::DbIoU64List>,
}

impl WalRetainedRecordDecoder {
    pub fn new(frame: WalRecordFrame) -> Self {
        Self { frame, state: WalRetainedDecodeState::Start, writer: None, run_ids: None }
    }

    pub fn step(&mut self, pages: &db_io_core::DbIoPages, control: &mut WalCursorControl) -> Result<Option<WalRecord>, DbError> {
        control.grant()?;
        match &mut self.state {
            WalRetainedDecodeState::Start => {
                let mut reader = WalPageReader::new(pages, self.frame.payload_start, self.frame.payload_end)?;
                if self.frame.kind == WAL_INDEX_CKPT {
                    let count = reader.varint()?;
                    check_len(count, 4_096, "wal retained checkpoint")?;
                    if reader.position.checked_add(count as usize * 8) != Some(reader.limit) {
                        return Err(DbError::Corrupt("wal checkpoint payload length differs".to_string()));
                    }
                    self.run_ids = Some(db_io_core::DbIoU64List::new());
                    self.state = WalRetainedDecodeState::Index { position: reader.position, remaining: count as usize };
                    return Ok(None);
                }
                let opaque = matches!(self.frame.kind, WAL_COMMAND | WAL_DIFF | WAL_INVERSE | WAL_EVENT | WAL_OUTBOX | WAL_MIGRATION);
                let inline = if self.frame.kind == WAL_PAYLOAD {
                    match reader.byte()? {
                        0 => {
                            let len = reader.varint()?;
                            check_len(len, MAX_FIELD_BYTES, "wal retained payload")?;
                            if reader.position.checked_add(len as usize) != Some(reader.limit) {
                                return Err(DbError::Corrupt("wal inline payload length differs".to_string()));
                            }
                            true
                        }
                        1 => false,
                        _ => return Err(DbError::Corrupt("wal payload tag".to_string())),
                    }
                } else {
                    false
                };
                if opaque || inline {
                    self.state = WalRetainedDecodeState::Bytes { position: reader.position, end: reader.limit };
                    match db_io_core::DbIoPageWriter::try_reserve_for_operation(pages.operation(), (reader.limit - reader.position).div_ceil(db_io_core::DB_IO_PAGE_BYTES)) {
                        Ok(writer) => self.writer = Some(writer),
                        Err(rejected) => {
                            let (error, writer) = rejected.into_parts();
                            self.writer = writer;
                            return Err(error);
                        }
                    }
                    return Ok(None);
                }
                let mut reader = WalPageReader::new(pages, self.frame.payload_start, self.frame.payload_end)?;
                let record = wal_decode_scalar_record(self.frame.kind, &mut reader)?;
                self.state = WalRetainedDecodeState::Done;
                Ok(Some(record))
            }
            WalRetainedDecodeState::Bytes { position, end } => {
                let writer = self.writer.as_mut().ok_or_else(|| DbError::Internal("wal decoder lost byte writer".to_string()))?;
                if *position < *end {
                    let reader = WalPageReader::new(pages, *position, *end)?;
                    let fragment = reader.fragment()?;
                    let written = writer.write_fragment(fragment)?;
                    if written == 0 {
                        return Err(DbError::LimitExceeded("wal retained decoder made no progress"));
                    }
                    *position += written;
                    return Ok(None);
                }
                let Some(pages) = writer.seal_retained_step()? else { return Ok(None) };
                self.writer = None;
                self.state = WalRetainedDecodeState::Done;
                let bytes = WalBytes { pages };
                Ok(Some(match self.frame.kind {
                    WAL_COMMAND => WalRecord::Command(bytes),
                    WAL_PAYLOAD => WalRecord::Payload(WalPayloadRef::Inline(bytes)),
                    WAL_DIFF => WalRecord::Diff(bytes),
                    WAL_INVERSE => WalRecord::Inverse(bytes),
                    WAL_EVENT => WalRecord::Event(bytes),
                    WAL_OUTBOX => WalRecord::Outbox(bytes),
                    WAL_MIGRATION => WalRecord::Migration(bytes),
                    _ => unreachable!(),
                }))
            }
            WalRetainedDecodeState::Index { position, remaining } => {
                if *remaining == 0 {
                    let run_ids = self.run_ids.take().ok_or_else(|| DbError::Internal("wal decoder lost checkpoint list".to_string()))?;
                    self.state = WalRetainedDecodeState::Done;
                    return Ok(Some(WalRecord::IndexCkpt { run_ids }));
                }
                let mut reader = WalPageReader::new(pages, *position, self.frame.payload_end)?;
                let value = u64::from_le_bytes(reader.array()?);
                self.run_ids.as_mut().ok_or_else(|| DbError::Internal("wal decoder lost checkpoint list".to_string()))?.push(value)?;
                *position = reader.position;
                *remaining -= 1;
                Ok(None)
            }
            WalRetainedDecodeState::Done => Err(DbError::Closed),
        }
    }

    pub fn close_owner_step(&mut self) -> Result<bool, DbError> {
        if let Some(writer) = self.writer.as_mut() {
            if writer.close_step()?.is_some() {
                return Ok(true);
            }
        }
        self.writer = None;
        if let Some(run_ids) = self.run_ids.as_mut() {
            if run_ids.close_step() {
                return Ok(true);
            }
        }
        self.run_ids = None;
        self.state = WalRetainedDecodeState::Done;
        Ok(false)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.writer.is_none() && self.run_ids.is_none() && matches!(self.state, WalRetainedDecodeState::Done)
    }
}

pub enum WalVerifiedFrameStep {
    Frame(WalRecordFrame),
    PhysicalCommit,
    Done,
}

/// 🧷️ Reads one span only after `WalSegmentChain` verified these exact immutable pages in full.
pub fn wal_next_verified_page_frame(pages: &dyn WalImmutableByteSource, offset: &mut usize, trusted_len: usize, control: &mut WalCursorControl) -> Result<WalVerifiedFrameStep, DbError> {
    control.grant()?;
    if *offset == trusted_len {
        return Ok(WalVerifiedFrameStep::Done);
    }
    let frame_start = *offset;
    let mut reader = WalPageReader::new(pages, frame_start, trusted_len)?;
    let body_len = reader.varint()?;
    check_len(body_len, protocol::ProtocolLimits::default().max_frame_len, "wal retained frame")?;
    let body_start = reader.position;
    let body_end = body_start.checked_add(body_len as usize).ok_or(DbError::LimitExceeded("wal frame body"))?;
    let frame_end = body_end.checked_add(8).ok_or(DbError::LimitExceeded("wal frame trailer"))?;
    if body_len < 2 || frame_end > trusted_len {
        return Err(DbError::Corrupt("wal frame exceeds trusted segment".to_string()));
    }
    let kind = reader.byte()?;
    let flags = reader.byte()?;
    if flags != protocol::wire::FRAME_FLAG_CRITICAL {
        return Err(DbError::Corrupt("db_wal requires exact critical frame flags".to_string()));
    }
    let payload_start = reader.position;
    let mut trailer = WalPageReader::new(pages, body_end, frame_end)?;
    let _verified_crc = trailer.array::<4>()?;
    let back_len = u32::from_le_bytes(trailer.array()?) as usize;
    if back_len != frame_end - frame_start {
        return Err(DbError::Corrupt("wal frame retained back length mismatch".to_string()));
    }
    if is_wal_record_kind(kind) {
        return Ok(WalVerifiedFrameStep::Frame(WalRecordFrame { kind, payload_start, payload_end: body_end, frame_end }));
    }
    if kind != protocol::wire::REC_COMMIT {
        return Err(DbError::Corrupt(format!("unexpected non-wal, non-commit frame kind {kind:#x} in a db_wal segment")));
    }
    *offset = frame_end;
    Ok(WalVerifiedFrameStep::PhysicalCommit)
}
//#endregion 🔖️Recovery
