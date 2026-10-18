//! 🧾️ WAL record: `WalRecord`, the record batch and the record encoding. See [`super`].

use super::*;

//#region 🔖️Records
/// 📜️ One decoded WAL record — the typed shape every `WAL_*` kind decodes to/encodes from.
/// `Command`/`Diff`/`Inverse`/`Event`/`Outbox`/`Migration` carry opaque bytes verbatim (per the
/// contract, no db crate below `db_artifact` interprets operation semantics); the rest are
/// structured since this crate itself owns their meaning (transaction boundaries, segment
/// chaining, frontiers, leases).
pub enum WalRecord {
    SegmentHeader { document: ArtifactId, segment_index: u64, prev_chain_hash: Option<[u8; 32]> },
    TxBegin { tx_id: u64 },
    TxCommit { tx_id: u64, record_count: u32 },
    TxAbort { tx_id: u64 },
    Command(WalBytes),
    Payload(WalPayloadRef),
    Diff(WalBytes),
    Inverse(WalBytes),
    Event(WalBytes),
    Outbox(WalBytes),
    Frontier(Frontier),
    SnapshotPub { generation: u64, frontier: Frontier },
    IndexCkpt { run_ids: db_io_core::DbIoU64List },
    Lease { resource: db_io_core::DbIoText, holder: db_io_core::DbIoText, fence: u64, expires_at_ms: u64 },
    Migration(WalBytes),
}

const WAL_RECORD_BATCH_CAPACITY: usize = 64;

pub struct WalRecordBatch {
    pub records: Box<[Option<WalRecord>]>,
    pub len: u8,
}

impl WalRecordBatch {
    pub fn new() -> Self {
        Self { records: (0..WAL_RECORD_BATCH_CAPACITY).map(|_| None).collect(), len: 0 }
    }

    pub fn push(&mut self, record: WalRecord) -> Result<(), WalRecord> {
        let Some(slot) = self.records.get_mut(self.len as usize) else { return Err(record) };
        *slot = Some(record);
        self.len += 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &WalRecord> {
        self.records[..self.len as usize].iter().flatten()
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.len == 0 {
            return Ok(false);
        }
        let index = self.len as usize - 1;
        let record = self.records[index].as_mut().ok_or_else(|| DbError::Internal("WAL batch close lost retained record".to_string()))?;
        if record.close_step()? {
            return Ok(true);
        }
        self.records[index] = None;
        self.len -= 1;
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.records.iter().all(Option::is_none)
    }
}

impl Default for WalRecordBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl WalRecord {
    /// 🔢️ The transaction id carried by a `TxBegin`/`TxCommit`/`TxAbort` record, or `None`
    /// for every other kind — used by `ArtifactWal::open` to resume `next_tx_id` past whatever was
    /// already durable.
    pub fn tx_id(&self) -> Option<u64> {
        match self {
            WalRecord::TxBegin { tx_id } | WalRecord::TxCommit { tx_id, .. } | WalRecord::TxAbort { tx_id } => Some(*tx_id),
            _ => None,
        }
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        match self {
            Self::Command(bytes) | Self::Diff(bytes) | Self::Inverse(bytes) | Self::Event(bytes) | Self::Outbox(bytes) | Self::Migration(bytes) | Self::Payload(WalPayloadRef::Inline(bytes)) => Ok(bytes.close_step()?.is_some()),
            Self::IndexCkpt { run_ids } => Ok(run_ids.close_step()),
            Self::Lease { resource, holder, .. } => {
                if resource.close_step() {
                    return Ok(true);
                }
                Ok(holder.close_step())
            }
            _ => Ok(false),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Command(bytes) | Self::Diff(bytes) | Self::Inverse(bytes) | Self::Event(bytes) | Self::Outbox(bytes) | Self::Migration(bytes) | Self::Payload(WalPayloadRef::Inline(bytes)) => bytes.terminal_is_empty(),
            Self::IndexCkpt { run_ids } => run_ids.terminal_is_empty(),
            Self::Lease { resource, holder, .. } => resource.terminal_is_empty() && holder.terminal_is_empty(),
            _ => true,
        }
    }

    /// ✍️ Encodes `self` to its on-disk `(kind, critical, payload)` triple, ready for
    /// `protocol::SprWriter::write_record`. Every kind is critical: unlike protocol's own
    /// history-log records (where e.g. a dictionary delta can plausibly be "skippable" to some
    /// future reader), every `WAL_*` record is load-bearing for correct replay — there is no
    /// optional WAL record in this crate's design.
    #[cfg(any(test, feature = "testkit"))]
    pub async fn encode(&self) -> (u8, bool, Vec<u8>) {
        let mut writer = pack::ByteWriter::new();
        let kind = match self {
            WalRecord::SegmentHeader { document, segment_index, prev_chain_hash } => {
                write_field(&mut writer, document.0.as_bytes()).await;
                writer.write_u64_le(*segment_index);
                match prev_chain_hash {
                    Some(hash) => {
                        writer.write_u8(1);
                        writer.write_bytes(hash);
                    }
                    None => writer.write_u8(0),
                }
                WAL_SEGMENT_HEADER
            }
            WalRecord::TxBegin { tx_id } => {
                writer.write_u64_le(*tx_id);
                WAL_TX_BEGIN
            }
            WalRecord::TxCommit { tx_id, record_count } => {
                writer.write_u64_le(*tx_id);
                writer.write_u32_le(*record_count);
                WAL_TX_COMMIT
            }
            WalRecord::TxAbort { tx_id } => {
                writer.write_u64_le(*tx_id);
                WAL_TX_ABORT
            }
            WalRecord::Command(bytes) => {
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_COMMAND
            }
            WalRecord::Payload(WalPayloadRef::Inline(bytes)) => {
                writer.write_u8(0);
                writer.write_varint_u64(bytes.len() as u64);
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_PAYLOAD
            }
            WalRecord::Payload(WalPayloadRef::CasRef(hash)) => {
                writer.write_u8(1);
                writer.write_bytes(&hash.0);
                WAL_PAYLOAD
            }
            WalRecord::Diff(bytes) => {
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_DIFF
            }
            WalRecord::Inverse(bytes) => {
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_INVERSE
            }
            WalRecord::Event(bytes) => {
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_EVENT
            }
            WalRecord::Outbox(bytes) => {
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_OUTBOX
            }
            WalRecord::Frontier(frontier) => {
                encode_frontier(&mut writer, frontier).await;
                WAL_FRONTIER
            }
            WalRecord::SnapshotPub { generation, frontier } => {
                writer.write_u64_le(*generation);
                encode_frontier(&mut writer, frontier).await;
                WAL_SNAPSHOT_PUB
            }
            WalRecord::IndexCkpt { run_ids } => {
                writer.write_varint_u64(run_ids.len() as u64);
                for run_id in run_ids.as_slice() {
                    writer.write_u64_le(*run_id);
                }
                WAL_INDEX_CKPT
            }
            WalRecord::Lease { resource, holder, fence, expires_at_ms } => {
                write_field(&mut writer, resource.as_str().as_bytes()).await;
                write_field(&mut writer, holder.as_str().as_bytes()).await;
                writer.write_u64_le(*fence);
                writer.write_u64_le(*expires_at_ms);
                WAL_LEASE
            }
            WalRecord::Migration(bytes) => {
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_MIGRATION
            }
        };
        (kind, true, writer.into_bytes())
    }

    pub fn retained_shape(&self) -> (u8, usize) {
        match self {
            Self::SegmentHeader { document, prev_chain_hash, .. } => (WAL_SEGMENT_HEADER, wal_field_len(document.0.as_bytes()) + 8 + 1 + prev_chain_hash.map_or(0, |_| 32)),
            Self::TxBegin { .. } => (WAL_TX_BEGIN, 8),
            Self::TxCommit { .. } => (WAL_TX_COMMIT, 12),
            Self::TxAbort { .. } => (WAL_TX_ABORT, 8),
            Self::Command(bytes) => (WAL_COMMAND, bytes.len()),
            Self::Payload(WalPayloadRef::Inline(bytes)) => (WAL_PAYLOAD, 1 + wal_varint_len(bytes.len() as u64) + bytes.len()),
            Self::Payload(WalPayloadRef::CasRef(_)) => (WAL_PAYLOAD, 33),
            Self::Diff(bytes) => (WAL_DIFF, bytes.len()),
            Self::Inverse(bytes) => (WAL_INVERSE, bytes.len()),
            Self::Event(bytes) => (WAL_EVENT, bytes.len()),
            Self::Outbox(bytes) => (WAL_OUTBOX, bytes.len()),
            Self::Frontier(frontier) => (WAL_FRONTIER, wal_frontier_len(frontier)),
            Self::SnapshotPub { frontier, .. } => (WAL_SNAPSHOT_PUB, 8 + wal_frontier_len(frontier)),
            Self::IndexCkpt { run_ids } => (WAL_INDEX_CKPT, wal_varint_len(run_ids.len() as u64) + run_ids.len() * 8),
            Self::Lease { resource, holder, .. } => (WAL_LEASE, wal_field_len(resource.as_str().as_bytes()) + wal_field_len(holder.as_str().as_bytes()) + 16),
            Self::Migration(bytes) => (WAL_MIGRATION, bytes.len()),
        }
    }

    pub async fn write_retained(&self, writer: &mut protocol::SprWriter<SharedBuf>) -> Result<u64, DbError> {
        let (kind, payload_len) = self.retained_shape();
        let mut record = writer.begin_identity_record(kind, true, payload_len).await.map_err(protocol_err)?;
        match self {
            Self::SegmentHeader { document, segment_index, prev_chain_hash } => {
                wal_record_write_field(&mut record, document.0.as_bytes()).await?;
                wal_record_write(&mut record, &segment_index.to_le_bytes()).await?;
                wal_record_write(&mut record, &[u8::from(prev_chain_hash.is_some())]).await?;
                if let Some(hash) = prev_chain_hash {
                    wal_record_write(&mut record, hash).await?;
                }
            }
            Self::TxBegin { tx_id } | Self::TxAbort { tx_id } => wal_record_write(&mut record, &tx_id.to_le_bytes()).await?,
            Self::TxCommit { tx_id, record_count } => {
                wal_record_write(&mut record, &tx_id.to_le_bytes()).await?;
                wal_record_write(&mut record, &record_count.to_le_bytes()).await?;
            }
            Self::Command(bytes) | Self::Diff(bytes) | Self::Inverse(bytes) | Self::Event(bytes) | Self::Outbox(bytes) | Self::Migration(bytes) => {
                for fragment in bytes.fragments() {
                    wal_record_write(&mut record, fragment).await?;
                }
            }
            Self::Payload(WalPayloadRef::Inline(bytes)) => {
                wal_record_write(&mut record, &[0]).await?;
                let mut length = [0u8; 10];
                wal_record_write(&mut record, wal_varint(bytes.len() as u64, &mut length)).await?;
                for fragment in bytes.fragments() {
                    wal_record_write(&mut record, fragment).await?;
                }
            }
            Self::Payload(WalPayloadRef::CasRef(hash)) => {
                wal_record_write(&mut record, &[1]).await?;
                wal_record_write(&mut record, &hash.0).await?;
            }
            Self::Frontier(frontier) => wal_record_write_frontier(&mut record, frontier).await?,
            Self::SnapshotPub { generation, frontier } => {
                wal_record_write(&mut record, &generation.to_le_bytes()).await?;
                wal_record_write_frontier(&mut record, frontier).await?;
            }
            Self::IndexCkpt { run_ids } => {
                let mut count = [0u8; 10];
                wal_record_write(&mut record, wal_varint(run_ids.len() as u64, &mut count)).await?;
                for run_id in run_ids.as_slice() {
                    wal_record_write(&mut record, &run_id.to_le_bytes()).await?;
                }
            }
            Self::Lease { resource, holder, fence, expires_at_ms } => {
                wal_record_write_field(&mut record, resource.as_str().as_bytes()).await?;
                wal_record_write_field(&mut record, holder.as_str().as_bytes()).await?;
                wal_record_write(&mut record, &fence.to_le_bytes()).await?;
                wal_record_write(&mut record, &expires_at_ms.to_le_bytes()).await?;
            }
        }
        record.finish().await.map_err(protocol_err)
    }

    /// 📖️ Inverse of `encode`. Errors `DbError::Corrupt` on an unrecognized `kind` (a
    /// genuinely corrupt or future-version record) rather than silently dropping it — every
    /// `WAL_*` kind is critical (see `encode`'s doc).
    #[cfg(test)]
    pub(super) async fn decode_retained(operation: u64, kind: u8, payload: &[u8], control: &mut WalCursorControl) -> Result<WalRecord, DbError> {
        control.grant()?;
        let mut reader = pack::ByteReader::new(payload);
        let record = match kind {
            WAL_SEGMENT_HEADER => {
                let document = ArtifactId(read_field_string(&mut reader).await?);
                let segment_index = reader.read_u64_le()?;
                let prev_chain_hash = if reader.read_u8()? == 1 { Some(reader.read_array32()?) } else { None };
                WalRecord::SegmentHeader { document, segment_index, prev_chain_hash }
            }
            WAL_TX_BEGIN => WalRecord::TxBegin { tx_id: reader.read_u64_le()? },
            WAL_TX_COMMIT => WalRecord::TxCommit { tx_id: reader.read_u64_le()?, record_count: reader.read_u32_le()? },
            WAL_TX_ABORT => WalRecord::TxAbort { tx_id: reader.read_u64_le()? },
            WAL_COMMAND => WalRecord::Command(WalBytes::copy_for_operation(operation, payload, control).await?),
            WAL_PAYLOAD => match reader.read_u8()? {
                0 => {
                    let len = reader.read_varint_u64()?;
                    check_len(len, MAX_FIELD_BYTES, "wal_record::payload")?;
                    WalRecord::Payload(WalPayloadRef::Inline(WalBytes::copy_for_operation(operation, reader.read_bytes(len as usize)?, control).await?))
                }
                1 => WalRecord::Payload(WalPayloadRef::CasRef(ContentHash(reader.read_array32()?))),
                other => return Err(DbError::Corrupt(format!("unknown wal payload tag {other}"))),
            },
            WAL_DIFF => WalRecord::Diff(WalBytes::copy_for_operation(operation, payload, control).await?),
            WAL_INVERSE => WalRecord::Inverse(WalBytes::copy_for_operation(operation, payload, control).await?),
            WAL_EVENT => WalRecord::Event(WalBytes::copy_for_operation(operation, payload, control).await?),
            WAL_OUTBOX => WalRecord::Outbox(WalBytes::copy_for_operation(operation, payload, control).await?),
            WAL_FRONTIER => WalRecord::Frontier(decode_frontier(&mut reader).await?),
            WAL_SNAPSHOT_PUB => {
                let generation = reader.read_u64_le()?;
                WalRecord::SnapshotPub { generation, frontier: decode_frontier(&mut reader).await? }
            }
            WAL_INDEX_CKPT => {
                let count = reader.read_varint_u64()?;
                check_len(count, MAX_RUN_IDS, "wal_record::index_ckpt run_ids")?;
                let mut run_ids = db_io_core::DbIoU64List::new();
                for _ in 0..count {
                    control.grant()?;
                    run_ids.push(reader.read_u64_le()?)?;
                }
                WalRecord::IndexCkpt { run_ids }
            }
            WAL_LEASE => {
                let resource = db_io_core::DbIoText::try_from_str(&read_field_string(&mut reader).await?)?;
                let holder = db_io_core::DbIoText::try_from_str(&read_field_string(&mut reader).await?)?;
                let fence = reader.read_u64_le()?;
                let expires_at_ms = reader.read_u64_le()?;
                WalRecord::Lease { resource, holder, fence, expires_at_ms }
            }
            WAL_MIGRATION => WalRecord::Migration(WalBytes::copy_for_operation(operation, payload, control).await?),
            other => return Err(DbError::Corrupt(format!("unknown wal record kind {other:#x}"))),
        };
        Ok(record)
    }
}
//#endregion 🔖️Records
