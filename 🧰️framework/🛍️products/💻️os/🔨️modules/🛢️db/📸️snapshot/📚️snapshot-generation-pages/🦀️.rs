//! 📚️ Snapshot generation pages: the publication claims, the generation page source and retained expectation builders, the generation handle and the generation open and read operations. See [`super`].

use super::*;

//#region 🔖️Generation
pub(super) struct SnapshotPageSink {
    pub(super) writer: db_io_core::DbIoPageWriter,
}

impl semio_framework_pack_source::PackSink for SnapshotPageSink {
    type Error = DbError;
    async fn write_all(&mut self, bytes: &[u8]) -> Result<(), DbError> {
        let mut cursor = 0;
        while cursor < bytes.len() {
            let written = self.writer.write_fragment(&bytes[cursor..])?;
            if written == 0 { return Err(DbError::LimitExceeded("snapshot page writer accepted no bytes")); }
            cursor += written;
            std::future::poll_fn(|context| {
                context.waker().wake_by_ref();
                std::task::Poll::Ready(())
            })
            .await;
        }
        Ok(())
    }

    async fn position(&self) -> u64 {
        self.writer.len() as u64
    }
}

impl SnapshotPageSink {
    fn try_new() -> Result<Self, DbError> {
        let writer = db_io_core::DbIoPageWriter::try_reserve(db_io_core::DB_IO_OPERATION_PAGES).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        Ok(Self { writer })
    }

    async fn patch_parent_footer(&mut self, parent_footer_offset: u64) -> Result<(), DbError> {
        if self.writer.len() < pack::FOOTER_SIZE {
            return Err(DbError::Corrupt("pack pages shorter than one footer".to_string()));
        }
        let footer_start = self.writer.len() - pack::FOOTER_SIZE;
        let mut footer_prefix = [0u8; 80];
        let mut copied = 0;
        while copied < footer_prefix.len() {
            copied += self.writer.read_fragment(footer_start + copied, &mut footer_prefix[copied..])?;
            std::future::poll_fn(|context| {
                context.waker().wake_by_ref();
                std::task::Poll::Ready(())
            })
            .await;
        }
        footer_prefix[72..80].copy_from_slice(&parent_footer_offset.to_le_bytes());
        let mut patched = 0;
        while patched < 8 {
            patched += self.writer.patch_fragment(footer_start + 72 + patched, &footer_prefix[72 + patched..80])?;
            std::future::poll_fn(|context| {
                context.waker().wake_by_ref();
                std::task::Poll::Ready(())
            })
            .await;
        }
        let crc = pack::crc32c(&footer_prefix).to_le_bytes();
        let mut patched_crc = 0;
        while patched_crc < crc.len() {
            patched_crc += self.writer.patch_fragment(footer_start + 80 + patched_crc, &crc[patched_crc..])?;
            std::future::poll_fn(|context| {
                context.waker().wake_by_ref();
                std::task::Poll::Ready(())
            })
            .await;
        }
        Ok(())
    }

    async fn into_pages(self) -> Result<db_io_core::DbIoPages, DbError> {
        self.writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)
    }
}

pub(super) trait SnapshotPageSource {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn page(&self, index: usize) -> Option<&Page>;
}

impl SnapshotPageSource for [Page] {
    fn len(&self) -> usize {
        <[Page]>::len(self)
    }

    fn page(&self, index: usize) -> Option<&Page> {
        self.get(index)
    }
}

pub(super) struct OptionalSnapshotPages<'pages> {
    pub(super) slots: &'pages [Option<Page>],
    pub(super) len: usize,
}

const SNAPSHOT_PUBLICATION_CLAIMS: usize = 64;

/// 🔒️ Live publication claims, one per (storage scope, document) identity. A claim probes the
/// whole fixed table, so only 64 concurrent publications can refuse one another — never two
/// unrelated documents whose identities share a hash slot.
static SNAPSHOT_PUBLICATION_CLAIMS_TABLE: std::sync::Mutex<[u64; SNAPSHOT_PUBLICATION_CLAIMS]> = std::sync::Mutex::new([0; SNAPSHOT_PUBLICATION_CLAIMS]);

pub(super) struct SnapshotPublicationClaim {
    slot: usize,
    identity: u64,
}

impl SnapshotPublicationClaim {
    pub(super) fn try_claim(scope: usize, document: &ArtifactId) -> Result<Self, DbError> {
        let mut hasher = semio_framework_hash::Hasher::new();
        hasher.update(&scope.to_le_bytes());
        hasher.update(document.0.as_bytes());
        let hash = hasher.finalize();
        let bytes = hash.as_bytes();
        let identity = u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]) | 1;
        let mut claims = SNAPSHOT_PUBLICATION_CLAIMS_TABLE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if claims.contains(&identity) {
            return Err(DbError::Conflict("snapshot publication already claimed".to_string()));
        }
        let slot = claims.iter().position(|claim| *claim == 0).ok_or(DbError::LimitExceeded("snapshot publication claims"))?;
        claims[slot] = identity;
        Ok(Self { slot, identity })
    }
}

impl Drop for SnapshotPublicationClaim {
    fn drop(&mut self) {
        let mut claims = SNAPSHOT_PUBLICATION_CLAIMS_TABLE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if claims[self.slot] == self.identity {
            claims[self.slot] = 0;
        }
    }
}

impl SnapshotPageSource for OptionalSnapshotPages<'_> {
    fn len(&self) -> usize {
        self.len
    }

    fn page(&self, index: usize) -> Option<&Page> {
        self.slots.get(index).and_then(Option::as_ref)
    }
}

/// 🏗️ Builds one generation's complete, self-contained `.spk` pack bytes: the descriptor
/// as the first (`KIND_SNAPSHOT`) segment, `new_pages` as `KIND_CHUNK` segments in order (so
/// `descriptor.new_pages[i]` is `pack::ChunkId(i)`), then the standard `pack::PackWriter::finish`
/// trailer. If `parent_footer_position` is `Some`, the trailing footer's `prev_footer_offset` is
/// patched in place afterward (see module doc) and `REQUIRED_FOOTER_CHAIN` is set in both the
/// header and the footer's `required_flags`.
pub(super) async fn build_generation_page_source<P: SnapshotPageSource + ?Sized>(descriptor: &SnapshotDescriptor, new_pages: &P, parent_footer_position: Option<u64>, control: &mut SnapshotCursorControl) -> Result<db_io_core::DbIoPages, DbError> {
    if new_pages.len() != descriptor.new_pages.len() {
        return Err(DbError::InvalidArgument("new_pages count does not match descriptor.new_pages".to_string()));
    }
    for (index, hash) in descriptor.new_pages.iter().enumerate() {
        let page = new_pages.page(index).ok_or_else(|| DbError::Internal("snapshot retained page source lost an owner".to_string()))?;
        if page.hash != *hash {
            return Err(DbError::InvalidArgument("new_pages order/hash does not match descriptor.new_pages".to_string()));
        }
    }

    let mut required_flags = 0u32;
    if parent_footer_position.is_some() {
        required_flags |= pack::REQUIRED_FOOTER_CHAIN;
    }
    if !new_pages.is_empty() {
        required_flags |= pack::REQUIRED_CHUNKED;
    }
    let options = pack::os_pack::WriteOptions { required_flags, optional_flags: 0, codec: pack::CodecId(0) };
    let mut writer = pack::PackWriter::begin(SnapshotPageSink::try_new()?, &options).await?;

    let mut descriptor_segment = writer.begin_identity_segment(pack::KIND_SNAPSHOT, descriptor.retained_len()?).await?;
    descriptor.write_retained(&mut descriptor_segment).await?;
    descriptor_segment.finish().await?;
    for index in 0..new_pages.len() {
        let page = new_pages.page(index).ok_or_else(|| DbError::Internal("snapshot retained page source lost an owner".to_string()))?;
        control.grant()?;
        let mut chunk = writer.begin_identity_chunk(page.len()).await?;
        for fragment in page.fragments() {
            if let Err(error) = control.grant() {
                chunk.close();
                return Err(error);
            }
            if let Err(error) = chunk.write_fragment(fragment).await {
                chunk.close();
                return Err(error.into());
            }
        }
        chunk.finish().await?;
    }

    let manifest = pack::Manifest {
        schema_name: String::new(),
        schema_hash: [0u8; 32],
        doc_span: pack::ByteRange { offset: 0, len: 0 },
        doc_frame_count: 0,
        symbols_span: pack::ByteRange { offset: 0, len: 0 },
        chunk_table_span: pack::ByteRange { offset: 0, len: 0 },
        field_index_span: pack::ByteRange { offset: 0, len: 0 },
        uncompressed_body_len: 0,
        field_count: 0,
        chunk_count: 0,
        symbol_count: 0,
    };
    let mut sink = writer.finish(&manifest).await?;

    if let Some(parent_offset) = parent_footer_position {
        sink.patch_parent_footer(parent_offset).await?;
    }
    sink.into_pages().await
}

fn retained_publication_descriptor_len(document: &ArtifactId, generation: u64, body: &SnapshotBody, page_count: usize) -> Result<usize, DbError> {
    let mut len = 1usize;
    for field in [
        snapshot_field_len(document.0.as_bytes()),
        snapshot_varint_len(generation),
        1,
        snapshot_varint_len(body.head_seq),
        snapshot_varint_len(body.commit_seq),
        snapshot_varint_len(body.epoch),
        32,
        snapshot_varint_len(body.protocol_version as u64),
        1 + body.base_pack_hash.map_or(0, |_| 32),
        snapshot_varint_len(body.roots.len() as u64) + body.roots.len().checked_mul(32).ok_or(DbError::LimitExceeded("snapshot roots bytes"))?,
        snapshot_varint_len(page_count as u64) + page_count.checked_mul(32).ok_or(DbError::LimitExceeded("snapshot retained page hashes"))?,
        snapshot_varint_len(body.created_at_ms),
    ] {
        len = len.checked_add(field).ok_or(DbError::LimitExceeded("snapshot retained publication descriptor bytes"))?;
    }
    Ok(len)
}

async fn write_retained_publication_descriptor<P: SnapshotPageSource + ?Sized>(segment: &mut pack::PackIdentitySegment<'_, SnapshotPageSink>, document: &ArtifactId, generation: u64, body: &SnapshotBody, new_pages: &P) -> Result<(), DbError> {
    snapshot_segment_write(segment, &[DESCRIPTOR_FORMAT_VERSION]).await?;
    snapshot_segment_write_field(segment, document.0.as_bytes()).await?;
    snapshot_segment_write_varint(segment, generation).await?;
    snapshot_segment_write(segment, &[0]).await?;
    snapshot_segment_write_varint(segment, body.head_seq).await?;
    snapshot_segment_write_varint(segment, body.commit_seq).await?;
    snapshot_segment_write_varint(segment, body.epoch).await?;
    snapshot_segment_write(segment, &body.chain_hash).await?;
    snapshot_segment_write_varint(segment, body.protocol_version as u64).await?;
    snapshot_segment_write(segment, &[u8::from(body.base_pack_hash.is_some())]).await?;
    if let Some(hash) = body.base_pack_hash {
        snapshot_segment_write(segment, &hash.0).await?;
    }
    snapshot_segment_write_varint(segment, body.roots.len() as u64).await?;
    for hash in &body.roots {
        snapshot_segment_write(segment, &hash.0).await?;
    }
    snapshot_segment_write_varint(segment, new_pages.len() as u64).await?;
    for index in 0..new_pages.len() {
        let page = new_pages.page(index).ok_or_else(|| DbError::Internal("snapshot retained publication lost a page owner".to_string()))?;
        snapshot_segment_write(segment, &page.hash.0).await?;
    }
    snapshot_segment_write_varint(segment, body.created_at_ms).await
}

pub(super) async fn build_generation_retained_expected<P: SnapshotPageSource + ?Sized>(document: &ArtifactId, generation: u64, body: &SnapshotBody, new_pages: &P, control: &mut SnapshotCursorControl) -> Result<db_io_core::DbIoPages, DbError> {
    let required_flags = if new_pages.is_empty() { 0 } else { pack::REQUIRED_CHUNKED };
    let options = pack::os_pack::WriteOptions { required_flags, optional_flags: 0, codec: pack::CodecId(0) };
    let mut writer = pack::PackWriter::begin(SnapshotPageSink::try_new()?, &options).await?;
    let mut descriptor_segment = writer.begin_identity_segment(pack::KIND_SNAPSHOT, retained_publication_descriptor_len(document, generation, body, new_pages.len())?).await?;
    write_retained_publication_descriptor(&mut descriptor_segment, document, generation, body, new_pages).await?;
    descriptor_segment.finish().await?;
    for index in 0..new_pages.len() {
        let page = new_pages.page(index).ok_or_else(|| DbError::Internal("snapshot retained publication lost a page owner".to_string()))?;
        control.grant()?;
        let mut chunk = writer.begin_identity_chunk(page.len()).await?;
        for fragment in page.fragments() {
            if let Err(error) = control.grant() {
                chunk.close();
                return Err(error);
            }
            if let Err(error) = chunk.write_fragment(fragment).await {
                chunk.close();
                return Err(error.into());
            }
        }
        chunk.finish().await?;
    }
    let manifest = pack::Manifest {
        schema_name: String::new(),
        schema_hash: [0; 32],
        doc_span: pack::ByteRange { offset: 0, len: 0 },
        doc_frame_count: 0,
        symbols_span: pack::ByteRange { offset: 0, len: 0 },
        chunk_table_span: pack::ByteRange { offset: 0, len: 0 },
        field_index_span: pack::ByteRange { offset: 0, len: 0 },
        uncompressed_body_len: 0,
        field_count: 0,
        chunk_count: 0,
        symbol_count: 0,
    };
    writer.finish(&manifest).await?.into_pages().await
}

pub async fn build_generation_pages(descriptor: &SnapshotDescriptor, new_pages: &[Page], parent_footer_position: Option<u64>, control: &mut SnapshotCursorControl) -> Result<db_io_core::DbIoPages, DbError> {
    build_generation_page_source(descriptor, new_pages, parent_footer_position, control).await
}

#[cfg(test)]
pub(super) async fn build_generation(descriptor: &SnapshotDescriptor, new_pages: &[Page], parent_footer_position: Option<u64>) -> Result<Vec<u8>, DbError> {
    let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
    let mut pages = build_generation_pages(descriptor, new_pages, parent_footer_position, &mut control).await?;
    let mut prepared = db_io_registry::db_io_prepare_platform(&pages)?.await?;
    let output = prepared.as_slice().to_vec();
    let _ = prepared.close_step()?;
    drop(prepared);
    let _ = pages.close_step()?;
    drop(pages);
    Ok(output)
}

/// 🩹️ `pack::PackWriter::finish` always writes `prev_footer_offset = 0` (there is no public
/// constructor knob for it — see module doc's design-choice note). This patches the trailing
/// 84-byte footer's `prev_footer_offset` field (wire bytes `[72..80]`, pinned by
/// `pack_format::Footer`'s own layout, verified byte-for-byte against `pack`'s own tests) in
/// place, then recomputes the footer's CRC-32C (over the preceding 80 bytes) so the result still
/// parses cleanly through `pack::read_footer_only`/`PackFile::open_superblock`.
#[cfg(test)]
async fn patch_prev_footer_offset(bytes: &mut [u8], parent_footer_offset: u64) -> Result<(), DbError> {
    if bytes.len() < pack::FOOTER_SIZE {
        return Err(DbError::Corrupt("pack bytes shorter than one footer".to_string()));
    }
    let footer_start = bytes.len() - pack::FOOTER_SIZE;
    bytes[footer_start + 72..footer_start + 80].copy_from_slice(&parent_footer_offset.to_le_bytes());
    let crc = pack::crc32c(&bytes[footer_start..footer_start + 80]);
    bytes[footer_start + 80..footer_start + 84].copy_from_slice(&crc.to_le_bytes());
    Ok(())
}

/// 🪪️ One opened generation: its decoded descriptor plus enough footer/position state to
/// walk to its parent (`parent_footer_offset`) or read its own chunks (`base`/`len`).
pub struct GenerationHandle {
    pub descriptor: SnapshotDescriptor,
    pub(super) base: u64,
    pub(super) len: u64,
    footer_required_flags: u32,
    footer_prev_offset: u64,
}

impl GenerationHandle {
    pub async fn generation(&self) -> u64 {
        self.descriptor.generation
    }

    /// ⛓️ The absolute offset (within the enclosing `combined` buffer) of the parent
    /// generation's own footer, or `None` if this generation has no parent — determined from the
    /// footer's `REQUIRED_FOOTER_CHAIN` bit, per this crate's `prev_footer_offset` semantics (see
    /// module doc), not merely from `prev_footer_offset != 0` (which is a legitimate value for a
    /// parent occupying the chain's very first byte).
    pub async fn parent_footer_offset(&self) -> Option<u64> {
        if self.footer_required_flags & pack::REQUIRED_FOOTER_CHAIN != 0 {
            Some(self.footer_prev_offset)
        } else {
            None
        }
    }
}

#[cfg(any(test, feature = "testkit"))]
pub async fn open_generation_at(combined: &[u8], base: u64, len: u64, footer: &pack::Footer) -> Result<GenerationHandle, DbError> {
    let sub = SubSource { inner: combined, base, len };
    let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
    let descriptor = decode_snapshot_descriptor(&sub, &mut control).await?;
    Ok(GenerationHandle { descriptor, base, len, footer_required_flags: footer.required_flags, footer_prev_offset: footer.prev_footer_offset })
}

async fn open_generation_pages_at(combined: &db_io_core::DbIoPages, base: u64, len: u64, footer: &pack::Footer, control: &mut SnapshotCursorControl) -> Result<GenerationHandle, DbError> {
    let sub = PageSubSource { inner: combined, base, len };
    let descriptor = decode_snapshot_descriptor(&sub, control).await?;
    Ok(GenerationHandle { descriptor, base, len, footer_required_flags: footer.required_flags, footer_prev_offset: footer.prev_footer_offset })
}

pub(super) async fn open_latest_pages(combined: &db_io_core::DbIoPages, control: &mut SnapshotCursorControl) -> Result<GenerationHandle, DbError> {
    control.grant()?;
    let whole = PageSubSource { inner: combined, base: 0, len: combined.len() as u64 };
    let footer = pack::read_footer_only(&whole).await?;
    if footer.file_len > combined.len() as u64 {
        return Err(DbError::Corrupt("snapshot generation footer.file_len exceeds retained pages".to_string()));
    }
    let base = combined.len() as u64 - footer.file_len;
    open_generation_pages_at(combined, base, footer.file_len, &footer, control).await
}

/// 🔚️ Opens the LAST generation physically present in `combined` (the one whose footer sits
/// at `combined.len() - FOOTER_SIZE`) — the entry point for reading a freshly-fetched or
/// freshly-materialized retained chain.
#[cfg(any(test, feature = "testkit"))]
pub async fn open_latest(combined: &[u8]) -> Result<GenerationHandle, DbError> {
    let whole = SubSource { inner: combined, base: 0, len: combined.len() as u64 };
    let footer = pack::read_footer_only(&whole).await?;
    if footer.file_len > combined.len() as u64 {
        return Err(DbError::Corrupt("snapshot generation footer.file_len exceeds buffer length".to_string()));
    }
    let base = combined.len() as u64 - footer.file_len;
    open_generation_at(combined, base, footer.file_len, &footer).await
}

/// ⬅️ Opens the generation whose own footer starts at absolute offset `footer_offset`
/// within `combined` — used to walk one hop up a `GenerationHandle::parent_footer_offset()` chain.
/// Uses only `pack::read_footer_only` (via a length-bounded `SubSource`) to find that footer's own
/// `file_len`, from which its base offset is derived (`footer_offset + FOOTER_SIZE - file_len`) —
/// no `pack_format` private footer-parsing internals needed.
#[cfg(test)]
pub async fn open_ancestor(combined: &[u8], footer_offset: u64) -> Result<GenerationHandle, DbError> {
    let footer_end = footer_offset.checked_add(pack::FOOTER_SIZE as u64).ok_or_else(|| DbError::Corrupt("snapshot chain footer offset overflow".to_string()))?;
    if footer_end > combined.len() as u64 {
        return Err(DbError::Corrupt("snapshot chain prev_footer_offset points past end of buffer".to_string()));
    }
    let bounded = SubSource { inner: combined, base: 0, len: footer_end };
    let footer = pack::read_footer_only(&bounded).await?;
    if footer.file_len > footer_end {
        return Err(DbError::Corrupt("ancestor generation footer.file_len exceeds its own footer offset".to_string()));
    }
    let base = footer_end - footer.file_len;
    open_generation_at(combined, base, footer.file_len, &footer).await
}

/// 📄️ Reads one page's raw bytes by content hash, starting at `handle` and walking to
/// ancestors (via `open_ancestor`) until a generation whose `new_pages` lists it is found.
/// Errors `NotFound` once the chain is exhausted without a match.
#[cfg(test)]
pub async fn read_page(combined: &[u8], handle: &GenerationHandle, hash: ContentHash) -> Result<Vec<u8>, DbError> {
    if let Some(index) = handle.descriptor.new_pages.iter().position(|candidate| *candidate == hash) {
        let sub = SubSource { inner: combined, base: handle.base, len: handle.len };
        let file = pack::PackFile::open_manifest(sub, &pack::PackLimits::default(), pack::os_pack::VerificationLevel::Standard).await?;
        return Ok(file.read_chunk(pack::ChunkId(index as u32), pack::os_pack::VerificationLevel::Standard).await?);
    }
    match handle.parent_footer_offset().await {
        Some(offset) => {
            let parent = open_ancestor(combined, offset).await?;
            Box::pin(read_page(combined, &parent, hash)).await
        }
        None => Err(DbError::NotFound(format!("page {hash} not found anywhere in the snapshot chain"))),
    }
}
//#endregion 🔖️Generation
