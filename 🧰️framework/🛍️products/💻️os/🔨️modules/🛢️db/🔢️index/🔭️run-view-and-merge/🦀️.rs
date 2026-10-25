//! 🔭️ Run view and merge: the run range, view and picks, the run page views, the merge of run views and entries, the merge policy and the index stats. See [`super`].

use super::*;

//#region 🔖️RunView
/// 📏️ One byte range inside a run's retained pages.
#[derive(Clone, Copy, Debug)]
pub(super) struct RunRange {
    pub(super) start: usize,
    pub(super) len: usize,
}

/// 👁️ One entry of a run read in place: its key and, for a put, its value, as ranges of the
/// run's own pages.
#[derive(Clone, Copy, Debug)]
pub(super) struct RunViewEntry {
    pub(super) key: RunRange,
    pub(super) value: Option<RunRange>,
}

/// 👁️ One run read in place. Loading, searching and merging a run costs exactly its retained
/// pages — never a page per entry — so a run of `MAX_RUN_ENTRIES` small entries stays inside one
/// operation's I/O credit, and an index never outgrows that credit as its document grows.
pub(super) struct RunView {
    pub(super) pages: db_io_core::DbIoPages,
    pub(super) entries: Box<[RunViewEntry]>,
}

/// 🧹️ Returns every page of a run read to its arena before the owner is dropped, so a read
/// never parks its pages as a lost owner for maintenance to reclaim later.
pub(super) fn close_run_pages(mut pages: db_io_core::DbIoPages) -> Result<(), DbError> {
    while pages.close_step()?.is_some() {}
    drop(pages);
    Ok(())
}

impl RunView {
    pub(super) fn close(self) -> Result<(), DbError> {
        close_run_pages(self.pages)
    }

    pub(super) fn key_cmp(&self, index: usize, key: &IndexBytes) -> Result<std::cmp::Ordering, DbError> {
        run_range_cmp(&self.pages, self.entries[index].key, &key.pages, RunRange { start: 0, len: key.len() })
    }

    pub(super) fn key_starts_with(&self, index: usize, prefix: &IndexBytes) -> Result<bool, DbError> {
        let key = self.entries[index].key;
        if prefix.len() > key.len {
            return Ok(false);
        }
        Ok(run_range_cmp(&self.pages, RunRange { start: key.start, len: prefix.len() }, &prefix.pages, RunRange { start: 0, len: prefix.len() })? == std::cmp::Ordering::Equal)
    }

    /// 🔢️ Entry `index`'s key as a big-endian `u64` (the seq-keyed kinds' key shape).
    pub(super) fn key_u64_be(&self, index: usize) -> Result<u64, DbError> {
        let key = self.entries[index].key;
        if key.len != 8 {
            return Err(DbError::Corrupt("index run key is not an 8-byte sequence".to_string()));
        }
        let mut reader = RunPageReader { pages: &self.pages, position: key.start, limit: key.start + key.len };
        Ok(u64::from_be_bytes(reader.array()?))
    }

    /// 🔢️ Entry `index`'s put value as a little-endian `u64` (the actor-seq kind's value shape).
    pub(super) fn value_u64_le(&self, index: usize) -> Result<u64, DbError> {
        let value = self.entries[index].value.ok_or_else(|| DbError::Corrupt("index run entry has no value".to_string()))?;
        if value.len != 8 {
            return Err(DbError::Corrupt("index run value is not an 8-byte sequence".to_string()));
        }
        let mut reader = RunPageReader { pages: &self.pages, position: value.start, limit: value.start + value.len };
        Ok(u64::from_le_bytes(reader.array()?))
    }

    /// 🔎️ The entry holding exactly `key`, by binary search over the run's ascending keys.
    pub(super) fn find(&self, key: &IndexBytes, control: &mut IndexCursorControl) -> Result<Option<usize>, DbError> {
        let (mut low, mut high) = (0usize, self.entries.len());
        while low < high {
            control.grant()?;
            let middle = low + (high - low) / 2;
            match self.key_cmp(middle, key)? {
                std::cmp::Ordering::Less => low = middle + 1,
                std::cmp::Ordering::Greater => high = middle,
                std::cmp::Ordering::Equal => return Ok(Some(middle)),
            }
        }
        Ok(None)
    }

    /// 📤️ Copies one of this run's ranges out as caller-owned bytes, charged to the run's own
    /// read operation.
    pub(super) async fn materialize(&self, range: RunRange, control: &mut IndexCursorControl) -> Result<IndexBytes, DbError> {
        let mut writer = db_io_core::DbIoPageWriter::try_reserve_for_operation(self.pages.operation(), range.len.div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        let mut reader = RunPageReader { pages: &self.pages, position: range.start, limit: range.start + range.len };
        while reader.position < reader.limit {
            control.grant()?;
            let fragment = reader.fragment()?;
            let written = writer.write_fragment(fragment)?;
            reader.position += written;
        }
        writer.seal_retained().await.map(|pages| IndexBytes { pages }).map_err(db_io_core::DbIoPageWriterRejected::into_error)
    }
}

/// ⚖️ Lexicographic order of two ranges, each over its own pages.
fn run_range_cmp(left: &db_io_core::DbIoPages, left_range: RunRange, right: &db_io_core::DbIoPages, right_range: RunRange) -> Result<std::cmp::Ordering, DbError> {
    let mut left_reader = RunPageReader { pages: left, position: left_range.start, limit: left_range.start + left_range.len };
    let mut right_reader = RunPageReader { pages: right, position: right_range.start, limit: right_range.start + right_range.len };
    loop {
        match (left_reader.position < left_reader.limit, right_reader.position < right_reader.limit) {
            (false, false) => return Ok(std::cmp::Ordering::Equal),
            (false, true) => return Ok(std::cmp::Ordering::Less),
            (true, false) => return Ok(std::cmp::Ordering::Greater),
            (true, true) => {}
        }
        let left_fragment = left_reader.fragment()?;
        let right_fragment = right_reader.fragment()?;
        let count = left_fragment.len().min(right_fragment.len());
        match left_fragment[..count].cmp(&right_fragment[..count]) {
            std::cmp::Ordering::Equal => {
                left_reader.position += count;
                right_reader.position += count;
            }
            order => return Ok(order),
        }
    }
}

/// 👁️ Verifies one run's checksum and structure and reads its entries in place.
pub(super) async fn view_run_pages(pages: db_io_core::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunView, DbError> {
    match view_run_entries(&pages, expected_kind, control).await {
        Ok(entries) => Ok(RunView { pages, entries }),
        Err(error) => {
            close_run_pages(pages)?;
            Err(error)
        }
    }
}

async fn view_run_entries(pages: &db_io_core::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<Box<[RunViewEntry]>, DbError> {
    control.grant()?;
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
    let mut trailer = RunPageReader::new(pages, pages.len());
    trailer.position = body_len;
    if checksum.finish() != u32::from_le_bytes(trailer.array::<4>()?) {
        return Err(DbError::Corrupt("index run checksum mismatch".to_string()));
    }
    let mut reader = RunPageReader::new(pages, body_len);
    let header = read_run_header(&mut reader, expected_kind, control).await?;
    let mut entries: Vec<RunViewEntry> = Vec::with_capacity(header.entry_count as usize);
    for _ in 0..header.entry_count {
        control.grant()?;
        let key_len = reader.varint()?;
        check_len(key_len, MAX_KEY_LEN, "db_index::key")?;
        let key = reader.skip(key_len as usize)?;
        if let Some(previous) = entries.last() {
            if run_range_cmp(pages, previous.key, pages, key)? != std::cmp::Ordering::Less {
                return Err(DbError::Corrupt("index run entries are not strictly ascending by key".to_string()));
            }
        }
        let value = match reader.byte()? {
            0 => None,
            1 => {
                let value_len = reader.varint()?;
                check_len(value_len, MAX_VALUE_LEN, "db_index::value")?;
                Some(reader.skip(value_len as usize)?)
            }
            other => return Err(DbError::Corrupt(format!("index run entry has unknown value tag {other}"))),
        };
        entries.push(RunViewEntry { key, value });
    }
    if reader.position != body_len {
        return Err(DbError::Corrupt("index run has trailing bytes before checksum".to_string()));
    }
    Ok(entries.into_boxed_slice())
}

/// 📌️ One output entry of a merge: which input view, which of its entries.
#[derive(Clone, Copy)]
pub(super) struct RunPick {
    pub(super) view: usize,
    pub(super) entry: usize,
}

/// 🔀️ The ascending merge of adjacent runs (`views` oldest first), the newest winning on an equal
/// key and tombstones dropped only when nothing older remains beneath.
pub(super) fn merge_run_views(views: &[&RunView], drop_tombstones: bool, control: &mut IndexCursorControl) -> Result<Vec<RunPick>, DbError> {
    let mut heads = vec![0usize; views.len()];
    let mut picks = Vec::with_capacity(views.iter().map(|view| view.entries.len()).sum());
    loop {
        control.grant()?;
        let mut winner: Option<usize> = None;
        for (view, head) in heads.iter().enumerate() {
            if *head == views[view].entries.len() {
                continue;
            }
            winner = match winner {
                None => Some(view),
                Some(best) => match run_range_cmp(&views[view].pages, views[view].entries[*head].key, &views[best].pages, views[best].entries[heads[best]].key)? {
                    std::cmp::Ordering::Greater => Some(best),
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Some(view),
                },
            };
        }
        let Some(winner) = winner else { break };
        let pick = RunPick { view: winner, entry: heads[winner] };
        for view in 0..views.len() {
            if view != winner && heads[view] < views[view].entries.len() && run_range_cmp(&views[view].pages, views[view].entries[heads[view]].key, &views[winner].pages, views[winner].entries[pick.entry].key)? == std::cmp::Ordering::Equal {
                heads[view] += 1;
            }
        }
        heads[winner] += 1;
        if !(drop_tombstones && views[winner].entries[pick.entry].value.is_none()) {
            picks.push(pick);
        }
    }
    Ok(picks)
}

async fn run_write_range(writer: &mut db_io_core::DbIoPageWriter, checksum: &mut pack::codec::Crc32cCursor, pages: &db_io_core::DbIoPages, range: RunRange, control: &mut IndexCursorControl) -> Result<(), DbError> {
    let mut reader = RunPageReader { pages, position: range.start, limit: range.start + range.len };
    while reader.position < reader.limit {
        control.grant()?;
        let fragment = reader.fragment()?;
        run_write(writer, checksum, fragment).await?;
        reader.position += fragment.len();
    }
    Ok(())
}

/// ✍️ Encodes picked entries of in-place runs as one run, copying key and value bytes straight
/// from their source pages — the same wire as `encode_run_pages`.
pub(super) async fn encode_run_from_views(kind: IndexKind, views: &[&RunView], picks: &[RunPick], control: &mut IndexCursorControl) -> Result<db_io_core::DbIoPages, DbError> {
    check_len(picks.len() as u64, LEVEL_RUN_ENTRIES_MAX, "db_index::entries")?;
    let mut encoded_len = RUN_MAGIC.len() + 2 + varint_len(picks.len() as u64) + 4;
    for pick in picks {
        control.grant()?;
        let entry = views[pick.view].entries[pick.entry];
        encoded_len = encoded_len.checked_add(varint_len(entry.key.len as u64) + entry.key.len + 1).ok_or(DbError::LimitExceeded("db_index encoded run bytes"))?;
        if let Some(value) = entry.value {
            encoded_len = encoded_len.checked_add(varint_len(value.len as u64) + value.len).ok_or(DbError::LimitExceeded("db_index encoded run bytes"))?;
        }
    }
    let mut writer = db_io_core::DbIoPageWriter::try_reserve(encoded_len.div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
    let mut checksum = pack::codec::Crc32cCursor::new();
    run_write(&mut writer, &mut checksum, &RUN_MAGIC).await?;
    run_write(&mut writer, &mut checksum, &[RUN_VERSION, kind.tag()]).await?;
    let mut varint = [0u8; 10];
    run_write(&mut writer, &mut checksum, encode_varint(picks.len() as u64, &mut varint)).await?;
    for pick in picks {
        let view = views[pick.view];
        let entry = view.entries[pick.entry];
        run_write(&mut writer, &mut checksum, encode_varint(entry.key.len as u64, &mut varint)).await?;
        run_write_range(&mut writer, &mut checksum, &view.pages, entry.key, control).await?;
        match entry.value {
            None => run_write(&mut writer, &mut checksum, &[0]).await?,
            Some(value) => {
                run_write(&mut writer, &mut checksum, &[1]).await?;
                run_write(&mut writer, &mut checksum, encode_varint(value.len as u64, &mut varint)).await?;
                run_write_range(&mut writer, &mut checksum, &view.pages, value, control).await?;
            }
        }
    }
    run_write_trailer(&mut writer, &checksum.finish().to_le_bytes()).await?;
    writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)
}
//#endregion 🔖️RunView

//#region 🔖️Merge
pub(super) async fn close_run_entry(mut entry: RunEntry, control: &mut IndexCursorControl) -> Result<(), DbError> {
    control.grant()?;
    if let RunValue::Put(value) = &mut entry.value {
        let _ = value.close_step()?;
    } else {
        let _ = entry.key.close_step()?;
    }
    drop(entry);
    Ok(())
}

#[cfg(test)]
pub(super) async fn merge_run_entries(mut older: RunEntries, mut newer: RunEntries, drop_tombstones: bool, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {
    let (mut old_index, mut new_index) = (0, 0);
    let mut output = RunEntries::new();
    while old_index < older.len() || new_index < newer.len() {
        control.grant()?;
        let order = match (older.get(old_index), newer.get(new_index)) {
            (Some(old), Some(new)) => index_bytes_cmp(&old.key, &new.key),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => break,
        };
        let entry = match order {
            std::cmp::Ordering::Less => {
                let entry = older.take(old_index).ok_or_else(|| DbError::Internal("index older merge owner lost".to_string()))?;
                old_index += 1;
                entry
            }
            std::cmp::Ordering::Greater => {
                let entry = newer.take(new_index).ok_or_else(|| DbError::Internal("index newer merge owner lost".to_string()))?;
                new_index += 1;
                entry
            }
            std::cmp::Ordering::Equal => {
                let shadowed = older.take(old_index).ok_or_else(|| DbError::Internal("index shadowed merge owner lost".to_string()))?;
                close_run_entry(shadowed, control).await?;
                old_index += 1;
                let entry = newer.take(new_index).ok_or_else(|| DbError::Internal("index winning merge owner lost".to_string()))?;
                new_index += 1;
                entry
            }
        };
        if drop_tombstones && matches!(entry.value, RunValue::Tombstone) {
            close_run_entry(entry, control).await?;
        } else if let Err(entry) = output.push(entry) {
            close_run_entry(entry, control).await?;
            return Err(DbError::LimitExceeded("index merged fixed entry owner"));
        }
        semio_framework_async::yield_once().await;
    }
    Ok(output)
}
//#endregion 🔖️Merge

//#region 🔖️MergePolicy
/// ⚖️ When an append (`IndexHandle::put_batch`) folds runs together. This
/// crate's own choice (the contract fixes the LSM-lite shape, not the trigger threshold): after every
/// append, if a kind has more than `max_runs_before_merge` runs, the oldest adjacent pair among its
/// newest `max_runs_before_merge + 1` runs whose entries fit one run (`MAX_RUN_ENTRIES`) is merged
/// into one — at most ONE merge per append, sized from the run ids alone (see
/// `IndexHandle::merge_one_within_policy`). A run never grows past `MAX_RUN_ENTRIES`, so every run —
/// and every merge of two — stays inside one operation's I/O credit however large its document grows;
/// full runs simply accumulate behind the newest window and are never read again by an append.
#[derive(Clone, Copy, Debug)]
pub struct MergePolicy {
    pub max_runs_before_merge: usize,
}

impl Default for MergePolicy {
    fn default() -> Self {
        Self { max_runs_before_merge: 4 }
    }
}
//#endregion 🔖️MergePolicy

//#region 🔖️Stats
/// 📊️ A kind's current shape: how many runs it's spread across, how many live entries (each
/// counted once even if shadowed copies exist in older runs — `entry_count` sums each run's raw
/// header count, so a key overwritten `N` times across `N` runs is NOT deduplicated here; `compact`
/// first is the way to get an exact live-key count) and how many bytes on `IndexStorage`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IndexStats {
    pub run_count: usize,
    pub entry_count: u64,
    pub total_bytes: u64,
}
//#endregion 🔖️Stats
