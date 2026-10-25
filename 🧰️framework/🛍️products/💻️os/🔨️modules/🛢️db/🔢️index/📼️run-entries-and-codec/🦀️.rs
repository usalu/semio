//! 📼️ Run entries and codec: the run value, entry and entries, the run header and page reader, the varints and the run page encoders and decoders. See [`super`].

use super::*;

//#region 🔖️SortedRun
#[derive(Debug)]
pub enum RunValue {
    Put(IndexBytes),
    Tombstone,
}

/// 📌️ One `(key, value)` pair inside a sorted run. A well-formed run's entries are strictly
/// ascending and unique by `key` — both `encode_run` (on the way in) and `decode_run` (on the way
/// back out, defending against on-disk corruption) enforce this.
#[derive(Debug)]
pub struct RunEntry {
    pub key: IndexBytes,
    pub value: RunValue,
}

pub struct RunEntries {
    pub entries: Box<[Option<RunEntry>]>,
    pub len: u8,
}

impl RunEntries {
    pub fn new() -> Self {
        Self { entries: (0..MAX_RUN_ENTRIES).map(|_| None).collect(), len: 0 }
    }

    pub fn push(&mut self, entry: RunEntry) -> Result<(), RunEntry> {
        let Some(slot) = self.entries.get_mut(self.len as usize) else { return Err(entry) };
        *slot = Some(entry);
        self.len += 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&RunEntry> {
        self.entries.get(index)?.as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut RunEntry> {
        self.entries.get_mut(index)?.as_mut()
    }

    pub fn take(&mut self, index: usize) -> Option<RunEntry> {
        self.entries.get_mut(index)?.take()
    }

    pub fn pop(&mut self) -> Option<RunEntry> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.entries[self.len as usize].take()
    }

    pub fn sort_step(&mut self, left: usize, right: usize, control: &mut IndexCursorControl) -> Result<bool, DbError> {
        control.grant()?;
        let swap = match (self.get(left), self.get(right)) {
            (Some(left), Some(right)) => index_bytes_cmp(&left.key, &right.key) == std::cmp::Ordering::Greater,
            _ => false,
        };
        if swap {
            self.entries.swap(left, right);
        }
        Ok(swap)
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.len == 0 {
            return Ok(false);
        }
        let index = self.len as usize - 1;
        let Some(entry) = self.entries[index].as_mut() else {
            self.len -= 1;
            return Ok(true);
        };
        if let RunValue::Put(value) = &mut entry.value {
            if value.close_step()?.is_some() {
                return Ok(true);
            }
        }
        if entry.key.close_step()?.is_some() {
            return Ok(true);
        }
        self.entries[index] = None;
        self.len -= 1;
        Ok(true)
    }
}

/// 🪧️ A run's 6-byte header: 4-byte magic, 1-byte format version, 1-byte `IndexKind` tag —
/// see `read_run_header`.
pub(super) const RUN_MAGIC: [u8; 4] = *b"DBIR";

pub(super) const RUN_VERSION: u8 = 1;

/// 📐️ A run header's parsed fields plus how many bytes of `body` it occupied, so the caller
/// knows where the entry stream starts.
pub(super) struct RunHeader {
    pub(super) entry_count: u64,
}

pub(super) struct RunPageReader<'pages> {
    pub(super) pages: &'pages db_io_core::DbIoPages,
    pub(super) position: usize,
    pub(super) limit: usize,
}

impl<'pages> RunPageReader<'pages> {
    pub(super) fn new(pages: &'pages db_io_core::DbIoPages, limit: usize) -> Self {
        Self { pages, position: 0, limit }
    }

    pub(super) fn fragment(&self) -> Result<&'pages [u8], DbError> {
        if self.position >= self.limit {
            return Err(DbError::Corrupt("index run ended mid-field".to_string()));
        }
        let mut base = 0usize;
        for fragment in self.pages.fragments() {
            let end = base + fragment.len();
            if self.position < end {
                return Ok(&fragment[self.position - base..fragment.len().min(self.limit - base)]);
            }
            base = end;
        }
        Err(DbError::Corrupt("index run retained page cursor lost its fragment".to_string()))
    }

    pub(super) fn byte(&mut self) -> Result<u8, DbError> {
        let byte = self.fragment()?[0];
        self.position += 1;
        Ok(byte)
    }

    pub(super) fn varint(&mut self) -> Result<u64, DbError> {
        let mut value = 0u64;
        for shift in (0..70).step_by(7) {
            let byte = self.byte()?;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(DbError::Corrupt("index run varint exceeds u64".to_string()))
    }

    pub(super) fn skip(&mut self, len: usize) -> Result<RunRange, DbError> {
        let end = self.position.checked_add(len).ok_or(DbError::LimitExceeded("index run field cursor"))?;
        if end > self.limit {
            return Err(DbError::Corrupt("index run field exceeds retained body".to_string()));
        }
        let range = RunRange { start: self.position, len };
        self.position = end;
        Ok(range)
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
}

pub(super) async fn read_run_header(reader: &mut RunPageReader<'_>, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunHeader, DbError> {
    control.grant()?;
    if reader.array::<4>()? != RUN_MAGIC {
        return Err(DbError::Corrupt("index run has a bad magic".to_string()));
    }
    let version = reader.byte()?;
    if version != RUN_VERSION {
        return Err(DbError::Corrupt(format!("unsupported index run version {version}")));
    }
    let kind_tag = reader.byte()?;
    if kind_tag != expected_kind.tag() {
        return Err(DbError::Corrupt(format!("index run kind mismatch: expected {expected_kind:?} (tag {}), found tag {kind_tag}", expected_kind.tag())));
    }
    let entry_count = reader.varint()?;
    check_len(entry_count, LEVEL_RUN_ENTRIES_MAX, "db_index::entries")?;
    Ok(RunHeader { entry_count })
}

/// ✍️ Encodes a well-formed (strictly ascending, unique-by-key) entry list into one run's
/// bytes: `MAGIC(4) VERSION(1) KIND(1) entry_count(varint) entries... crc32c(4, LE)`. Each entry is
/// `key_len(varint) key value_tag(1: 0=tombstone,1=put) [value_len(varint) value]`. Errors
/// `InvalidArgument` if `entries` isn't strictly ascending — this fn never silently re-sorts, since
/// a caller with unsorted/duplicate entries must use the bounded incremental sorter first.
pub(super) fn varint_len(mut value: u64) -> usize {
    let mut len = 1;
    while value >= 0x80 {
        value >>= 7;
        len += 1;
    }
    len
}

pub(super) fn encode_varint(mut value: u64, buffer: &mut [u8; 10]) -> &[u8] {
    let mut cursor = 0;
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buffer[cursor] = byte;
        cursor += 1;
        if value == 0 {
            return &buffer[..cursor];
        }
    }
}

pub(super) async fn run_write(writer: &mut db_io_core::DbIoPageWriter, checksum: &mut pack::codec::Crc32cCursor, bytes: &[u8]) -> Result<(), DbError> {
    let mut cursor = 0;
    while cursor < bytes.len() {
        let written = writer.write_fragment(&bytes[cursor..])?;
        checksum.update_page(&bytes[cursor..cursor + written]);
        cursor += written;
        semio_framework_async::yield_once().await;
    }
    Ok(())
}

pub(super) async fn run_write_trailer(writer: &mut db_io_core::DbIoPageWriter, bytes: &[u8]) -> Result<(), DbError> {
    let mut cursor = 0;
    while cursor < bytes.len() {
        cursor += writer.write_fragment(&bytes[cursor..])?;
        semio_framework_async::yield_once().await;
    }
    Ok(())
}

async fn run_write_pages(writer: &mut db_io_core::DbIoPageWriter, checksum: &mut pack::codec::Crc32cCursor, bytes: &IndexBytes, control: &mut IndexCursorControl) -> Result<(), DbError> {
    for fragment in bytes.fragments() {
        control.grant()?;
        run_write(writer, checksum, fragment).await?;
    }
    Ok(())
}

pub(super) async fn encode_run_pages(kind: IndexKind, entries: &RunEntries, control: &mut IndexCursorControl) -> Result<db_io_core::DbIoPages, DbError> {
    check_len(entries.len() as u64, MAX_RUN_ENTRIES, "db_index::entries")?;
    let mut encoded_len = RUN_MAGIC.len() + 2 + varint_len(entries.len() as u64) + 4;
    let mut previous_key: Option<&IndexBytes> = None;
    for index in 0..entries.len() {
        control.grant()?;
        let entry = entries.get(index).ok_or_else(|| DbError::Internal("index run entry slot lost".to_string()))?;
        if let Some(previous) = previous_key {
            if index_bytes_cmp(&entry.key, previous) != std::cmp::Ordering::Greater {
                return Err(DbError::InvalidArgument("db_index run entries must be strictly ascending and unique by key".to_string()));
            }
        }
        previous_key = Some(&entry.key);
        check_len(entry.key.len() as u64, MAX_KEY_LEN, "db_index::key")?;
        encoded_len = encoded_len.checked_add(varint_len(entry.key.len() as u64)).and_then(|len| len.checked_add(entry.key.len() + 1)).ok_or(DbError::LimitExceeded("db_index encoded run bytes"))?;
        match &entry.value {
            RunValue::Tombstone => {}
            RunValue::Put(value) => {
                check_len(value.len() as u64, MAX_VALUE_LEN, "db_index::value")?;
                encoded_len = encoded_len.checked_add(varint_len(value.len() as u64)).and_then(|len| len.checked_add(value.len())).ok_or(DbError::LimitExceeded("db_index encoded run bytes"))?;
            }
        }
        semio_framework_async::yield_once().await;
    }
    let mut writer = db_io_core::DbIoPageWriter::try_reserve(encoded_len.div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
    let mut checksum = pack::codec::Crc32cCursor::new();
    run_write(&mut writer, &mut checksum, &RUN_MAGIC).await?;
    run_write(&mut writer, &mut checksum, &[RUN_VERSION, kind.tag()]).await?;
    let mut varint = [0u8; 10];
    run_write(&mut writer, &mut checksum, encode_varint(entries.len() as u64, &mut varint)).await?;
    for index in 0..entries.len() {
        control.grant()?;
        let entry = entries.get(index).ok_or_else(|| DbError::Internal("index run entry slot lost".to_string()))?;
        run_write(&mut writer, &mut checksum, encode_varint(entry.key.len() as u64, &mut varint)).await?;
        run_write_pages(&mut writer, &mut checksum, &entry.key, control).await?;
        match &entry.value {
            RunValue::Tombstone => run_write(&mut writer, &mut checksum, &[0]).await?,
            RunValue::Put(value) => {
                run_write(&mut writer, &mut checksum, &[1]).await?;
                run_write(&mut writer, &mut checksum, encode_varint(value.len() as u64, &mut varint)).await?;
                run_write_pages(&mut writer, &mut checksum, value, control).await?;
            }
        }
    }
    run_write_trailer(&mut writer, &checksum.finish().to_le_bytes()).await?;
    writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)
}

/// 📥️ Encodes strictly ascending, unique `(key, value)` puts into one run's bytes (the same
/// layout as `encode_run_pages`) straight from caller-owned slices, under one page writer.
pub(super) async fn encode_sorted_run_pages(kind: IndexKind, entries: &[(&[u8], &[u8])], control: &mut IndexCursorControl) -> Result<db_io_core::DbIoPages, DbError> {
    check_len(entries.len() as u64, MAX_RUN_ENTRIES, "db_index::entries")?;
    let mut encoded_len = RUN_MAGIC.len() + 2 + varint_len(entries.len() as u64) + 4;
    for (index, (key, value)) in entries.iter().enumerate() {
        control.grant()?;
        if index > 0 && entries[index - 1].0 >= *key {
            return Err(DbError::InvalidArgument("db_index sorted run entries must be strictly ascending and unique by key".to_string()));
        }
        check_len(key.len() as u64, MAX_KEY_LEN, "db_index::key")?;
        check_len(value.len() as u64, MAX_VALUE_LEN, "db_index::value")?;
        encoded_len = encoded_len
            .checked_add(varint_len(key.len() as u64) + key.len() + 1 + varint_len(value.len() as u64) + value.len())
            .ok_or(DbError::LimitExceeded("db_index encoded run bytes"))?;
    }
    let mut writer = db_io_core::DbIoPageWriter::try_reserve(encoded_len.div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
    let mut checksum = pack::codec::Crc32cCursor::new();
    let mut varint = [0u8; 10];
    let written = async {
        run_write(&mut writer, &mut checksum, &RUN_MAGIC).await?;
        run_write(&mut writer, &mut checksum, &[RUN_VERSION, kind.tag()]).await?;
        run_write(&mut writer, &mut checksum, encode_varint(entries.len() as u64, &mut varint)).await?;
        for (key, value) in entries {
            control.grant()?;
            run_write(&mut writer, &mut checksum, encode_varint(key.len() as u64, &mut varint)).await?;
            run_write(&mut writer, &mut checksum, key).await?;
            run_write(&mut writer, &mut checksum, &[1]).await?;
            run_write(&mut writer, &mut checksum, encode_varint(value.len() as u64, &mut varint)).await?;
            run_write(&mut writer, &mut checksum, value).await?;
        }
        Ok::<(), DbError>(())
    }
    .await;
    if let Err(error) = written {
        let _ = writer.seal_retained().await.map(close_run_pages);
        return Err(error);
    }
    run_write_trailer(&mut writer, &checksum.finish().to_le_bytes()).await?;
    writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)
}

pub(super) async fn index_bytes_from_reader(operation: u64, reader: &mut RunPageReader<'_>, len: usize, control: &mut IndexCursorControl) -> Result<IndexBytes, DbError> {
    let mut writer = db_io_core::DbIoPageWriter::try_reserve_for_operation(operation, len.div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
    let end = reader.position.checked_add(len).ok_or(DbError::LimitExceeded("index run field cursor"))?;
    if end > reader.limit {
        return Err(DbError::Corrupt("index run field exceeds retained body".to_string()));
    }
    while reader.position < end {
        control.grant()?;
        let fragment = reader.fragment()?;
        let count = (end - reader.position).min(fragment.len());
        let written = writer.write_fragment(&fragment[..count])?;
        reader.position += written;
    }
    writer.seal_retained().await.map(|pages| IndexBytes { pages }).map_err(db_io_core::DbIoPageWriterRejected::into_error)
}

pub(super) async fn index_bytes_from_slice_for_operation(operation: u64, bytes: &[u8], control: &mut IndexCursorControl) -> Result<IndexBytes, DbError> {
    let mut writer = db_io_core::DbIoPageWriter::try_reserve_for_operation(operation, bytes.len().div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
    let mut offset = 0usize;
    while offset < bytes.len() {
        control.grant()?;
        offset += writer.write_fragment(&bytes[offset..])?;
    }
    writer.seal_retained().await.map(|pages| IndexBytes { pages }).map_err(db_io_core::DbIoPageWriterRejected::into_error)
}

#[cfg(test)]
async fn decode_run_pages_inner(pages: &db_io_core::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {
    control.grant()?;
    let operation = pages.operation();
    if pages.len() < 4 {
        return Err(DbError::Corrupt("index run is shorter than its checksum trailer".to_string()));
    }
    let body_len = pages.len() - 4;
    let mut checksum = pack::codec::Crc32cCursor::new();
    let mut remaining = body_len;
    for fragment in pages.fragments() {
        control.grant()?;
        let count = remaining.min(fragment.len());
        checksum.update_page(&fragment[..count]);
        remaining -= count;
        if remaining == 0 {
            break;
        }
    }
    let mut trailer = RunPageReader::new(&pages, pages.len());
    trailer.position = body_len;
    if checksum.finish() != u32::from_le_bytes(trailer.array::<4>()?) {
        return Err(DbError::Corrupt("index run checksum mismatch".to_string()));
    }
    let mut reader = RunPageReader::new(&pages, body_len);
    let header = read_run_header(&mut reader, expected_kind, control).await?;
    let mut entries = RunEntries::new();
    for _ in 0..header.entry_count {
        control.grant()?;
        let key_len = reader.varint()?;
        check_len(key_len, MAX_KEY_LEN, "db_index::key")?;
        let key = index_bytes_from_reader(operation, &mut reader, key_len as usize, control).await?;
        if entries.get(entries.len().saturating_sub(1)).is_some_and(|previous| index_bytes_cmp(&key, &previous.key) != std::cmp::Ordering::Greater) {
            return Err(DbError::Corrupt("index run entries are not strictly ascending by key".to_string()));
        }
        let value = match reader.byte()? {
            0 => RunValue::Tombstone,
            1 => {
                let value_len = reader.varint()?;
                check_len(value_len, MAX_VALUE_LEN, "db_index::value")?;
                RunValue::Put(index_bytes_from_reader(operation, &mut reader, value_len as usize, control).await?)
            }
            other => return Err(DbError::Corrupt(format!("index run entry has unknown value tag {other}"))),
        };
        entries.push(RunEntry { key, value }).map_err(|_| DbError::LimitExceeded("index run fixed entry owner"))?;
    }
    if reader.position != body_len {
        return Err(DbError::Corrupt("index run has trailing bytes before checksum".to_string()));
    }
    Ok(entries)
}

#[cfg(test)]
pub(super) async fn decode_run_pages(mut pages: db_io_core::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {
    let result = decode_run_pages_inner(&pages, expected_kind, control).await;
    let _ = pages.close_step()?;
    drop(pages);
    result
}
//#endregion 🔖️SortedRun
