//! 🔎️ Full text, blob list, conflict, projection and preview indexes: the full text index, the blob list, the conflict index, the projection index and the preview index. See [`super`].

use super::*;

//#region 🔖️FullTextIndex
/// 🔤️ `term -> [doc_ref]` — a minimal inverted index: `index_document` tokenizes text into
/// lowercase alphanumeric-run terms and records `doc_ref` (an opaque caller-chosen id, typically a
/// field/command location) against each; `search` resolves one term to its posting list. No
/// ranking/stemming/stopwords — `db_query`'s full-text query planner is expected to layer that on
/// top of this crate's exact-term postings.
pub struct FullTextIndex<'a, S: IndexStorage> {
    pub(super) handle: IndexHandle<'a, S>,
}

impl<'a, S: IndexStorage> FullTextIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::FullText).await }
    }

    pub async fn postings(&self, term_key: &[u8], control: &mut IndexCursorControl) -> Result<db_io_core::DbIoU64List, DbError> {
        let key = admit_generated_index_bytes(term_key.to_vec(), MAX_KEY_LEN, control).await?;
        let result = self.handle.get(&key, control).await?;
        close_index_bytes(key, control).await?;
        match result {
            Some(bytes) => decode_index_bytes(bytes, control, decode_postings).await,
            None => Ok(db_io_core::DbIoU64List::new()),
        }
    }

    /// ➕️ Tokenizes `text` and records `doc_ref` against every distinct term it contains.
    pub async fn index_document(&self, doc_ref: u64, text: &str) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(65_536)?;
        for term in text.split(|character: char| !character.is_alphanumeric()).filter(|term| !term.is_empty()) {
            control.grant()?;
            let term = term.to_lowercase();
            let mut postings = self.postings(term.as_bytes(), &mut control).await?;
            if postings.as_slice().binary_search(&doc_ref).is_err() {
                let mut updated = db_io_core::DbIoU64List::new();
                let mut inserted = false;
                for posting in postings.as_slice() {
                    if !inserted && doc_ref < *posting {
                        updated.push(doc_ref)?;
                        inserted = true;
                    }
                    updated.push(*posting)?;
                }
                if !inserted {
                    updated.push(doc_ref)?;
                }
                control.grant()?;
                let _ = postings.close_step();
                drop(postings);
                postings = updated;
            }
            let value = admit_generated_index_bytes(encode_postings(&postings).await, MAX_VALUE_LEN, &mut control).await?;
            control.grant()?;
            let _ = postings.close_step();
            drop(postings);
            let key = IndexBytes::copy_for_operation(value.operation(), term.as_bytes(), &mut control).await?;
            self.handle.put(key, value, &mut control).await?;
        }
        Ok(())
    }

    /// 🔎️ The posting list for `term` (case-folded to match `index_document`'s tokenizer),
    /// or an empty list if the term has never been indexed.
    pub async fn search(&self, term: &str) -> Result<db_io_core::DbIoU64List, DbError> {
        let mut control = self.handle.operation_control(16_384)?;
        self.postings(term.to_lowercase().as_bytes(), &mut control).await
    }
}
//#endregion 🔖️FullTextIndex

//#region 🔖️BlobList
/// 📦️ Encodes a list of opaque byte blobs (`ConflictIndex`'s per-command conflict records)
/// as `count(varint) [len(varint) bytes]...` — the same read-modify-write accumulation shape
/// `TouchedRegionIndex`/`FullTextIndex` use for their posting lists, generalized to arbitrary-size
/// values instead of `u64` postings.
pub struct IndexBlobList {
    blobs: [Option<IndexBytes>; MAX_RUN_ENTRIES as usize],
    pub(super) len: u8,
}

impl IndexBlobList {
    pub fn new() -> Self {
        Self { blobs: std::array::from_fn(|_| None), len: 0 }
    }

    pub fn push(&mut self, bytes: IndexBytes) -> Result<(), IndexBytes> {
        let Some(slot) = self.blobs.get_mut(self.len as usize) else { return Err(bytes) };
        *slot = Some(bytes);
        self.len += 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn get(&self, index: usize) -> Option<&IndexBytes> {
        self.blobs.get(index)?.as_ref()
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.len == 0 {
            return Ok(false);
        }
        let index = self.len as usize - 1;
        let blob = self.blobs[index].as_mut().ok_or_else(|| DbError::Internal("index blob close lost retained owner".to_string()))?;
        if blob.close_step()?.is_some() {
            return Ok(true);
        }
        self.blobs[index] = None;
        self.len -= 1;
        Ok(true)
    }
}

async fn encode_blob_list(blobs: &IndexBlobList, control: &mut IndexCursorControl) -> Result<IndexBytes, DbError> {
    let mut total = varint_len(blobs.len() as u64);
    for index in 0..blobs.len() {
        let blob = blobs.get(index).unwrap();
        total = total.checked_add(varint_len(blob.len() as u64) + blob.len()).ok_or(DbError::LimitExceeded("index blob list bytes"))?;
    }
    let mut writer = db_io_core::DbIoPageWriter::try_reserve(total.div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
    let mut varint = [0u8; 10];
    index_write_unchecked(encode_varint(blobs.len() as u64, &mut varint), &mut writer, control).await?;
    for index in 0..blobs.len() {
        let blob = blobs.get(index).unwrap();
        index_write_unchecked(encode_varint(blob.len() as u64, &mut varint), &mut writer, control).await?;
        for fragment in blob.fragments() {
            index_write_unchecked(fragment, &mut writer, control).await?;
        }
    }
    writer.seal_retained().await.map(|pages| IndexBytes { pages }).map_err(db_io_core::DbIoPageWriterRejected::into_error)
}

async fn index_write_unchecked(bytes: &[u8], writer: &mut db_io_core::DbIoPageWriter, control: &mut IndexCursorControl) -> Result<(), DbError> {
    let mut offset = 0;
    while offset < bytes.len() {
        control.grant()?;
        offset += writer.write_fragment(&bytes[offset..])?;
    }
    Ok(())
}

async fn decode_blob_list_inner(bytes: &IndexBytes, control: &mut IndexCursorControl) -> Result<IndexBlobList, DbError> {
    let operation = bytes.operation();
    let mut reader = RunPageReader::new(&bytes.pages, bytes.len());
    let count = reader.varint()?;
    check_len(count, MAX_RUN_ENTRIES, "db_index::blob_list")?;
    let mut blobs = IndexBlobList::new();
    for _ in 0..count {
        control.grant()?;
        let len = reader.varint()?;
        check_len(len, MAX_VALUE_LEN, "db_index::blob_list_entry")?;
        let blob = index_bytes_from_reader(operation, &mut reader, len as usize, control).await?;
        blobs.push(blob).map_err(|_| DbError::LimitExceeded("index blob list owner"))?;
    }
    if reader.position != reader.limit {
        return Err(DbError::Corrupt("index blob list has trailing bytes".to_string()));
    }
    Ok(blobs)
}

async fn decode_blob_list(bytes: IndexBytes, control: &mut IndexCursorControl) -> Result<IndexBlobList, DbError> {
    let result = decode_blob_list_inner(&bytes, control).await;
    close_index_bytes(bytes, control).await?;
    result
}
//#endregion 🔖️BlobList

//#region 🔖️ConflictIndex
/// ⚔️ `command_seq -> [ConflictRecord bytes]` — a command may surface more than one
/// conflict (touched-region collision, constraint violation, …), so this accumulates a list per
/// `command_seq` the same way `TouchedRegionIndex` accumulates a posting list: read the current
/// list, append, write back. Record shapes are `db_conflict`'s concern; this index only stores and
/// returns the opaque bytes it's handed.
pub struct ConflictIndex<'a, S: IndexStorage> {
    pub(super) handle: IndexHandle<'a, S>,
}

impl<'a, S: IndexStorage> ConflictIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Conflict).await }
    }

    /// ➕️ Appends `record` to `command_seq`'s conflict list.
    pub async fn record_conflict(&self, command_seq: u64, record: IndexBytes) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(32_768)?;
        let mut records = self.conflicts_for_with_control(command_seq, &mut control).await?;
        records.push(record).map_err(|_| DbError::LimitExceeded("index conflict list owner"))?;
        let value = encode_blob_list(&records, &mut control).await?;
        control.grant()?;
        let _ = records.close_step()?;
        drop(records);
        let key = IndexBytes::copy_for_operation(value.operation(), &command_seq.to_be_bytes(), &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    /// 📋️ Every conflict record recorded for `command_seq`, in the order they were
    /// recorded, or empty if none.
    async fn conflicts_for_with_control(&self, command_seq: u64, control: &mut IndexCursorControl) -> Result<IndexBlobList, DbError> {
        let key = admit_generated_index_bytes(command_seq.to_be_bytes().to_vec(), MAX_KEY_LEN, control).await?;
        let result = self.handle.get(&key, control).await?;
        close_index_bytes(key, control).await?;
        match result {
            Some(bytes) => decode_blob_list(bytes, control).await,
            None => Ok(IndexBlobList::new()),
        }
    }

    pub async fn conflicts_for(&self, command_seq: u64) -> Result<IndexBlobList, DbError> {
        let mut control = self.handle.operation_control(32_768)?;
        self.conflicts_for_with_control(command_seq, &mut control).await
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.stats(&mut control).await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        let mut control = self.handle.operation_control(65_536)?;
        self.handle.compact(&mut control).await
    }
}
//#endregion 🔖️ConflictIndex

//#region 🔖️ProjectionIndex
/// 📽️ `(projection_id, frontier_seq) -> opaque projection state bytes`, floor-queryable per
/// projection id — `db_projection`'s "this projection's state as of at or before frontier X"
/// lookup. Keys are `projection_id_bytes || 0x00 || frontier_seq(8, BE)`, the same NUL-separated
/// composite shape `ActorSeqIndex` uses (`projection_id` must not itself contain a NUL byte,
/// validated) so a prefix scan by projection id can't spill into a lexicographically-neighboring
/// projection's entries.
pub struct ProjectionIndex<'a, S: IndexStorage> {
    pub(super) handle: IndexHandle<'a, S>,
}

async fn validate_projection_id_key_safe(projection_id: &str) -> Result<(), DbError> {
    if projection_id.as_bytes().contains(&0u8) {
        return Err(DbError::InvalidArgument("projection id must not contain a NUL byte to be index-key safe".to_string()));
    }
    Ok(())
}

async fn projection_key(projection_id: &str, frontier_seq: u64) -> Result<Vec<u8>, DbError> {
    validate_projection_id_key_safe(projection_id).await?;
    let mut key = Vec::with_capacity(projection_id.len() + 1 + 8);
    key.extend_from_slice(projection_id.as_bytes());
    key.push(0u8);
    key.extend_from_slice(&frontier_seq.to_be_bytes());
    Ok(key)
}

impl<'a, S: IndexStorage> ProjectionIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Projection).await }
    }

    pub async fn record(&self, projection_id: &str, frontier_seq: u64, state: IndexBytes) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key_bytes = projection_key(projection_id, frontier_seq).await?;
        let key = IndexBytes::copy_for_operation(state.operation(), &key_bytes, &mut control).await?;
        self.handle.put(key, state, &mut control).await
    }

    /// 🎯️ The exact state recorded for `projection_id` at `frontier_seq`, or `None` if
    /// nothing was recorded at that exact sequence.
    pub async fn at(&self, projection_id: &str, frontier_seq: u64) -> Result<Option<IndexBytes>, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(projection_key(projection_id, frontier_seq).await?, MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.get(&key, &mut control).await?;
        close_index_bytes(key, &mut control).await?;
        Ok(result)
    }

    /// 🏔️ The state recorded at the greatest `frontier_seq' <= frontier_seq` for
    /// `projection_id` specifically — scoped to `projection_id`'s own key range (via the NUL
    /// separator) before scanning, so a projection with no entry at or before `frontier_seq` never
    /// wrongly surfaces a different, lexicographically-earlier projection's entry.
    pub async fn latest_at_or_before(&self, projection_id: &str, frontier_seq: u64) -> Result<Option<(u64, IndexBytes)>, DbError> {
        validate_projection_id_key_safe(projection_id).await?;
        let mut prefix = projection_id.as_bytes().to_vec();
        prefix.push(0u8);
        let mut upper = prefix.clone();
        upper.extend_from_slice(&frontier_seq.to_be_bytes());
        let mut control = self.handle.operation_control(32_768)?;
        let prefix_owner = admit_generated_index_bytes(prefix, MAX_KEY_LEN, &mut control).await?;
        let upper_owner = admit_generated_index_bytes(upper, MAX_KEY_LEN, &mut control).await?;
        let latest = self.handle.last_live_in_range(&prefix_owner, Some(&upper_owner), &mut control).await;
        close_index_bytes(prefix_owner, &mut control).await?;
        close_index_bytes(upper_owner, &mut control).await?;
        let Some((key, value)) = latest? else { return Ok(None) };
        let mut seq_bytes = [0u8; 8];
        let suffix_read = key.read_fragment(key.len().saturating_sub(8), &mut seq_bytes);
        close_index_bytes(key, &mut control).await?;
        if suffix_read != 8 {
            close_index_bytes(value, &mut control).await?;
            return Err(DbError::Corrupt("projection index key has a malformed suffix".to_string()));
        }
        Ok(Some((u64::from_be_bytes(seq_bytes), value)))
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.stats(&mut control).await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        let mut control = self.handle.operation_control(65_536)?;
        self.handle.compact(&mut control).await
    }
}
//#endregion 🔖️ProjectionIndex

//#region 🔖️PreviewIndex
/// 🌫️ `(actor, preview_key) -> opaque latest preview bytes` — `publish`/`withdraw` are
/// plain `put`/`delete`, so `latest` naturally coalesces to the most recently published-or-
/// withdrawn value per `(actor, preview_key)`, matching the contract's "coalescing
/// latest-per-(actor,key)" preview law. Keys are `actor_bytes || 0x00 || preview_key_bytes`
/// (`actor`'s id must not contain a NUL byte, validated; `preview_key` is the final component so
/// it needs no such restriction). Never durable per that same law — `db_preview` is responsible for
/// never routing this index's writes through a durable `DurabilityClass`.
pub struct PreviewIndex<'a, S: IndexStorage> {
    pub(super) handle: IndexHandle<'a, S>,
}

async fn encode_preview_key(actor: &ActorId, preview_key: &str) -> Result<Vec<u8>, DbError> {
    validate_actor_key_safe(actor).await?;
    let mut key = Vec::with_capacity(actor.0.len() + 1 + preview_key.len());
    key.extend_from_slice(actor.0.as_bytes());
    key.push(0u8);
    key.extend_from_slice(preview_key.as_bytes());
    Ok(key)
}

impl<'a, S: IndexStorage> PreviewIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Preview).await }
    }

    pub async fn publish(&self, actor: &ActorId, preview_key: &str, value: IndexBytes) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key_bytes = encode_preview_key(actor, preview_key).await?;
        let key = IndexBytes::copy_for_operation(value.operation(), &key_bytes, &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    pub async fn withdraw(&self, actor: &ActorId, preview_key: &str) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(encode_preview_key(actor, preview_key).await?, MAX_KEY_LEN, &mut control).await?;
        self.handle.delete(key, &mut control).await
    }

    pub async fn latest(&self, actor: &ActorId, preview_key: &str) -> Result<Option<IndexBytes>, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(encode_preview_key(actor, preview_key).await?, MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.get(&key, &mut control).await?;
        close_index_bytes(key, &mut control).await?;
        Ok(result)
    }

    /// 📋️ Every currently-live `(preview_key, value)` published by `actor`.
    pub async fn for_actor(&self, actor: &ActorId) -> Result<RunEntries, DbError> {
        validate_actor_key_safe(actor).await?;
        let mut prefix = actor.0.as_bytes().to_vec();
        prefix.push(0u8);
        let mut control = self.handle.operation_control(16_384)?;
        let prefix = admit_generated_index_bytes(prefix, MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.scan_prefix(&prefix, &mut control).await?;
        close_index_bytes(prefix, &mut control).await?;
        Ok(result)
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.stats(&mut control).await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        let mut control = self.handle.operation_control(65_536)?;
        self.handle.compact(&mut control).await
    }
}
//#endregion 🔖️PreviewIndex
