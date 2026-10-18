//! 🧷️ Location, command and inverse indexes: the record location, the sequence location index, the command index and the inverse index. See [`super`].

use super::*;

//#region 🔖️RecordLocation
/// 📍️ A pointer into a document's WAL: which segment, what byte offset, how many bytes.
/// `CommandIndex`/`InverseIndex`'s value shape — deliberately NOT the WAL record itself (this crate
/// never depends on `db_wal`/`protocol`; a location is exactly enough for a caller who DOES depend
/// on those to seek and re-read the actual record).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RecordLocation {
    pub segment: u64,
    pub offset: u64,
    pub len: u64,
}

async fn encode_location(location: RecordLocation) -> Vec<u8> {
    let mut writer = ByteWriter::new();
    writer.write_varint_u64(location.segment);
    writer.write_varint_u64(location.offset);
    writer.write_varint_u64(location.len);
    writer.into_bytes()
}

pub(super) fn decode_location(reader: &mut RunPageReader<'_>) -> Result<RecordLocation, DbError> {
    let segment = reader.varint()?;
    let offset = reader.varint()?;
    let len = reader.varint()?;
    Ok(RecordLocation { segment, offset, len })
}

pub(super) async fn decode_index_bytes<T>(mut bytes: IndexBytes, control: &mut IndexCursorControl, decode: impl FnOnce(&mut RunPageReader<'_>) -> Result<T, DbError>) -> Result<T, DbError> {
    control.grant()?;
    let result = (|| {
        let mut reader = RunPageReader::new(&bytes.pages, bytes.len());
        let decoded = decode(&mut reader)?;
        if reader.position != reader.limit {
            return Err(DbError::Corrupt("typed index value has trailing bytes".to_string()));
        }
        Ok(decoded)
    })();
    control.grant()?;
    let _ = bytes.close_step()?;
    drop(bytes);
    result
}

/// 🔢️ `u64 -> RecordLocation`, keyed big-endian so byte order matches numeric order — the
/// shared shape behind both `CommandIndex` (keyed by command seq) and `InverseIndex` (keyed by the
/// same command seq, pointing at its inverse's location instead).
struct SeqLocationIndex<'a, S: IndexStorage> {
    pub(super) handle: IndexHandle<'a, S>,
}

impl<'a, S: IndexStorage> SeqLocationIndex<'a, S> {
    pub(super) async fn new(storage: &'a S, document: ArtifactId, kind: IndexKind) -> Self {
        Self { handle: IndexHandle::new(storage, document, kind).await }
    }

    pub(super) async fn record(&self, seq: u64, location: RecordLocation) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let value = admit_generated_index_bytes(encode_location(location).await, MAX_VALUE_LEN, &mut control).await?;
        let key = IndexBytes::copy_for_operation(value.operation(), &seq.to_be_bytes(), &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    pub(super) async fn lookup(&self, seq: u64) -> Result<Option<RecordLocation>, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(seq.to_be_bytes().to_vec(), MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.get(&key, &mut control).await?;
        close_index_bytes(key, &mut control).await?;
        match result {
            Some(bytes) => Ok(Some(decode_index_bytes(bytes, &mut control, decode_location).await?)),
            None => Ok(None),
        }
    }

    pub(super) async fn remove(&self, seq: u64) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(seq.to_be_bytes().to_vec(), MAX_KEY_LEN, &mut control).await?;
        self.handle.delete(key, &mut control).await
    }

    pub(super) async fn record_owned_run(&self, runs: &mut OwnedRuns, entries: &[(u64, RecordLocation)]) -> Result<(), DbError> {
        let mut encoded = Vec::with_capacity(entries.len());
        for (seq, location) in entries {
            encoded.push((seq.to_be_bytes(), encode_location(*location).await));
        }
        let slices: Vec<(&[u8], &[u8])> = encoded.iter().map(|(key, value)| (&key[..], &value[..])).collect();
        let mut control = self.handle.operation_control(65_536)?;
        self.handle.append_owned_sorted_run(runs, &slices, &mut control).await
    }

    pub(super) async fn owned_runs(&self) -> Result<OwnedRuns, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.owned_runs(&mut control).await
    }

    pub(super) async fn indexed_through(&self) -> Result<u64, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let Some(view) = self.handle.newest_run(&mut control).await? else { return Ok(0) };
        let highest = view.entries.len().checked_sub(1).map_or(Ok(0), |last| view.key_u64_be(last));
        view.close()?;
        highest
    }
}
//#endregion 🔖️RecordLocation

//#region 🔖️CommandIndex
/// 🗃️ `command_seq -> RecordLocation` — `db_artifact`'s primary lookup for "where in the
/// WAL is command N", the backbone of replay-from-a-point and `Consistency::Exact`/`AtLeast` query
/// resolution.
pub struct CommandIndex<'a, S: IndexStorage>(pub(super) SeqLocationIndex<'a, S>);

impl<'a, S: IndexStorage> CommandIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self(SeqLocationIndex::new(storage, document, IndexKind::Command).await)
    }

    pub async fn record(&self, command_seq: u64, location: RecordLocation) -> Result<(), DbError> {
        self.0.record(command_seq, location).await
    }

    /// 📥️ Records ascending `(command_seq, location)` pairs as ONE run (at most `MAX_RUN_ENTRIES`) of `runs`, without
    /// listing ([`IndexHandle::append_owned_sorted_run`]).
    pub async fn record_owned_run(&self, runs: &mut OwnedRuns, entries: &[(u64, RecordLocation)]) -> Result<(), DbError> {
        self.0.record_owned_run(runs, entries).await
    }

    /// 🗂️ This kind's runs, listed once for its append-only owner.
    pub async fn owned_runs(&self) -> Result<OwnedRuns, DbError> {
        self.0.owned_runs().await
    }

    /// 🔝️ The highest command seq recorded, `0` when none — read from the newest run alone,
    /// which holds it for an owner that records in ascending order.
    pub async fn indexed_through(&self) -> Result<u64, DbError> {
        self.0.indexed_through().await
    }

    pub async fn lookup(&self, command_seq: u64) -> Result<Option<RecordLocation>, DbError> {
        self.0.lookup(command_seq).await
    }

    pub async fn remove(&self, command_seq: u64) -> Result<(), DbError> {
        self.0.remove(command_seq).await
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        let mut control = self.0.handle.operation_control(8_192)?;
        self.0.handle.stats(&mut control).await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        let mut control = self.0.handle.operation_control(65_536)?;
        self.0.handle.compact(&mut control).await
    }
}
//#endregion 🔖️CommandIndex

//#region 🔖️InverseIndex
/// ↩️ `command_seq -> RecordLocation` of that command's inverse operation payload —
/// `db_artifact`'s undo machinery's lookup.
pub struct InverseIndex<'a, S: IndexStorage>(pub(super) SeqLocationIndex<'a, S>);

impl<'a, S: IndexStorage> InverseIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self(SeqLocationIndex::new(storage, document, IndexKind::Inverse).await)
    }

    pub async fn record(&self, command_seq: u64, location: RecordLocation) -> Result<(), DbError> {
        self.0.record(command_seq, location).await
    }

    /// 📥️ Records ascending `(command_seq, location)` pairs as ONE run (at most `MAX_RUN_ENTRIES`) of `runs`, without
    /// listing ([`IndexHandle::append_owned_sorted_run`]).
    pub async fn record_owned_run(&self, runs: &mut OwnedRuns, entries: &[(u64, RecordLocation)]) -> Result<(), DbError> {
        self.0.record_owned_run(runs, entries).await
    }

    /// 🗂️ This kind's runs, listed once for its append-only owner.
    pub async fn owned_runs(&self) -> Result<OwnedRuns, DbError> {
        self.0.owned_runs().await
    }

    /// 🔝️ The highest command seq recorded, `0` when none — read from the newest run alone,
    /// which holds it for an owner that records in ascending order.
    pub async fn indexed_through(&self) -> Result<u64, DbError> {
        self.0.indexed_through().await
    }

    pub async fn lookup(&self, command_seq: u64) -> Result<Option<RecordLocation>, DbError> {
        self.0.lookup(command_seq).await
    }

    pub async fn remove(&self, command_seq: u64) -> Result<(), DbError> {
        self.0.remove(command_seq).await
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        let mut control = self.0.handle.operation_control(8_192)?;
        self.0.handle.stats(&mut control).await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        let mut control = self.0.handle.operation_control(65_536)?;
        self.0.handle.compact(&mut control).await
    }
}
//#endregion 🔖️InverseIndex
