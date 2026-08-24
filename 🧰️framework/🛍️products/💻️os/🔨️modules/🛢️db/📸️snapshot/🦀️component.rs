//! 🗄️ 📸️ `db_snapshot` — pack-file-based document snapshots for the `db` crate family:
//! immutable state pages are written into `KIND_CHUNK` segments, a `SnapshotDescriptor`
//! (frontier, protocol version, VCS head, base pack hash, root list) is written into the
//! reserved `KIND_SNAPSHOT` (`0x07`) pack segment kind — this crate is the first real consumer
//! of that segment kind — and incremental generations chain via `pack::Footer.prev_footer_offset`
//! plus the `REQUIRED_FOOTER_CHAIN` flag, also previously reserved-unused. Built strictly against
//! the frozen contract at
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_snapshot` row).
//!
//! 🎯️ Design choice — how `Footer.prev_footer_offset` is actually used: `db_storage::SnapshotStorage`
//! stores each generation as an *independent*, self-contained `.spk` blob (own header at local
//! offset 0, own footer, individually `delete_generation`-able without corrupting siblings — see
//! that trait's own doc). So `prev_footer_offset`/`REQUIRED_FOOTER_CHAIN` cannot literally address
//! "the other stored blob" — instead this crate defines their meaning precisely: if every
//! generation from the chain's root up to and including this one were concatenated, in
//! generation order, into one retained page authority (`SnapshotChainCursor::materialize_pages`), then
//! `prev_footer_offset` is the absolute offset *within that concatenation* of the parent
//! generation's own footer (`parent_base + parent_len - FOOTER_SIZE`), and `REQUIRED_FOOTER_CHAIN`
//! marks that the field is meaningful at all (distinguishing a legitimate `prev_footer_offset == 0`
//! — a parent that starts at the very first byte, i.e. the chain's root — from "no parent").
//! Reading only ever needs `pack`'s public API (`read_footer_only`, `open_manifest`, `PackSource`)
//! plus a small local length-bounded `PackSource` view (`SubSource`, below) — no `pack_format`
//! private internals are reimplemented.
//!
//! 🎯️ Design choice — locating the `KIND_SNAPSHOT` descriptor segment: `pack::Manifest` has no
//! dedicated span field for it (unsurprising — it's a newly-real segment kind as of this crate).
//! This crate always writes it as the very first segment right after the 32-byte header (before
//! any `write_chunk` call), so it lives at a fixed, predictable local offset
//! (`pack::HEADER_SIZE`) in every generation's own coordinate space, and `decode_snapshot_segment`
//! reads it directly from there using only `pack_core`-level primitives re-exported by the `pack`
//! facade (`crc32c`, `read_varint_u64`, `PackSource`) — the segment is always written with the
//! identity codec, so this reader only needs to understand the uncompressed framing.
//!
//! 🎯️ Scope boundary: retention (`SnapshotManager::retain_from`) only allows pruning generations
//! *below* a full-baseline floor (a generation with no parent) — rolling an incremental chain up
//! into a fresh full baseline first is `db_compact`'s "snapshot consolidation" responsibility
//! (see the contract's `db_compact` row), not this crate's. Similarly, coordinating retention with
//! `db_storage::LeaseStorage` so an in-flight replica read isn't pruned out from under it belongs
//! to `db_compact`'s "online compaction with manifest CAS + fencing" — `retain_from` here is the
//! safe, mechanical pruning primitive that caller is expected to call after fencing itself.

use crate::db_durability::EpochFence;
use crate::db_durability::Frontier;
use crate::db_ids::{check_len, ArtifactId, DbError};
use crate::*;
use db_state::Page;
use db_storage::{LeaseInfo, LeaseStorage, SnapshotStorage};

//#region 🔖️Descriptor
/// @emoji 🔢️ Wire format tag for `SnapshotDescriptor::encode`/`decode` — bumped on any
/// incompatible field layout change so a stale reader fails loudly (`DbError::Corrupt`) instead
/// of misparsing.
const DESCRIPTOR_FORMAT_VERSION: u8 = 1;

/// @emoji 🛡️ Ceiling on `roots`/`new_pages` entry counts and on any embedded string's byte
/// length, validated before allocating the destination `Vec`/`String` — mirrors `pack_core`'s
/// stated "validate before allocating" invariant.
const MAX_HASH_LIST_LEN: u64 = 4_000_000;
const MAX_STRING_BYTES: u64 = 1024 * 1024;

/// @emoji 📇️ One generation's manifest of a document's snapshot state: the frontier it was taken
/// at, VCS/protocol provenance, the root page hashes needed to reconstruct the document tree, and
/// the hashes of the pages whose `KIND_CHUNK` bytes live *in this generation's own blob*
/// (everything else reachable from `roots` is expected to resolve via `parent_generation`'s chain
/// — see `resolve_page`). This is exactly the payload written into the `KIND_SNAPSHOT` segment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotDescriptor {
    pub document: ArtifactId,
    pub generation: u64,
    /// @emoji 🌳️ `None` for a full baseline (self-sufficient, chains to nothing); `Some(g)` for an
    /// incremental generation whose unlisted pages must be resolved from generation `g` onward.
    pub parent_generation: Option<u64>,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub epoch: u64,
    pub chain_hash: [u8; 32],
    pub protocol_version: u32,
    pub vcs_head: Option<String>,
    pub base_pack_hash: Option<ContentHash>,
    pub roots: Vec<ContentHash>,
    /// @emoji 🧱️ Page hashes with a `KIND_CHUNK` in *this* blob, in the exact order they were
    /// written — index `i` here is `pack::ChunkId(i)` in this generation's own chunk table.
    pub new_pages: Vec<ContentHash>,
    pub created_at_ms: u64,
}

impl SnapshotDescriptor {
    /// @emoji 🧭️ Reconstructs the `Frontier` this generation was taken at.
    pub async fn frontier(&self) -> Frontier {
        Frontier { document: self.document.clone(), head_seq: self.head_seq, commit_seq: self.commit_seq, chain_hash: self.chain_hash, epoch: self.epoch }
    }

    /// @emoji ✍️ Serializes this descriptor to the exact bytes written into the `KIND_SNAPSHOT`
    /// segment — a flat, versioned, varint-framed encoding (this crate's own choice; the contract
    /// fixes only the segment kind, not the payload layout).
    #[cfg(test)]
    pub async fn encode(&self) -> Vec<u8> {
        let mut w = pack::ByteWriter::new();
        w.write_u8(DESCRIPTOR_FORMAT_VERSION);
        write_string(&mut w, &self.document.0).await;
        w.write_varint_u64(self.generation);
        write_option_u64(&mut w, self.parent_generation).await;
        w.write_varint_u64(self.head_seq);
        w.write_varint_u64(self.commit_seq);
        w.write_varint_u64(self.epoch);
        w.write_bytes(&self.chain_hash);
        w.write_varint_u64(self.protocol_version as u64);
        match &self.vcs_head {
            Some(head) => {
                w.write_u8(1);
                write_string(&mut w, head).await;
            }
            None => w.write_u8(0),
        }
        match &self.base_pack_hash {
            Some(hash) => {
                w.write_u8(1);
                w.write_bytes(&hash.0);
            }
            None => w.write_u8(0),
        }
        write_hash_list(&mut w, &self.roots).await;
        write_hash_list(&mut w, &self.new_pages).await;
        w.write_varint_u64(self.created_at_ms);
        w.into_bytes()
    }

    fn retained_len(&self) -> Result<usize, DbError> {
        let mut len = 1usize;
        for field in [
            snapshot_field_len(self.document.0.as_bytes()),
            snapshot_varint_len(self.generation),
            1 + self.parent_generation.map_or(0, snapshot_varint_len),
            snapshot_varint_len(self.head_seq),
            snapshot_varint_len(self.commit_seq),
            snapshot_varint_len(self.epoch),
            32,
            snapshot_varint_len(self.protocol_version as u64),
            1 + self.vcs_head.as_ref().map_or(0, |value| snapshot_field_len(value.as_bytes())),
            1 + self.base_pack_hash.map_or(0, |_| 32),
            snapshot_varint_len(self.roots.len() as u64) + self.roots.len().checked_mul(32).ok_or(DbError::LimitExceeded("snapshot roots bytes"))?,
            snapshot_varint_len(self.new_pages.len() as u64) + self.new_pages.len().checked_mul(32).ok_or(DbError::LimitExceeded("snapshot new pages bytes"))?,
            snapshot_varint_len(self.created_at_ms),
        ] {
            len = len.checked_add(field).ok_or(DbError::LimitExceeded("snapshot descriptor bytes"))?;
        }
        Ok(len)
    }

    async fn write_retained(&self, segment: &mut pack::PackIdentitySegment<'_, SnapshotPageSink>) -> Result<(), DbError> {
        snapshot_segment_write(segment, &[DESCRIPTOR_FORMAT_VERSION]).await?;
        snapshot_segment_write_field(segment, self.document.0.as_bytes()).await?;
        snapshot_segment_write_varint(segment, self.generation).await?;
        snapshot_segment_write(segment, &[u8::from(self.parent_generation.is_some())]).await?;
        if let Some(parent) = self.parent_generation {
            snapshot_segment_write_varint(segment, parent).await?;
        }
        snapshot_segment_write_varint(segment, self.head_seq).await?;
        snapshot_segment_write_varint(segment, self.commit_seq).await?;
        snapshot_segment_write_varint(segment, self.epoch).await?;
        snapshot_segment_write(segment, &self.chain_hash).await?;
        snapshot_segment_write_varint(segment, self.protocol_version as u64).await?;
        snapshot_segment_write(segment, &[u8::from(self.vcs_head.is_some())]).await?;
        if let Some(head) = &self.vcs_head {
            snapshot_segment_write_field(segment, head.as_bytes()).await?;
        }
        snapshot_segment_write(segment, &[u8::from(self.base_pack_hash.is_some())]).await?;
        if let Some(hash) = self.base_pack_hash {
            snapshot_segment_write(segment, &hash.0).await?;
        }
        snapshot_segment_write_varint(segment, self.roots.len() as u64).await?;
        for hash in &self.roots {
            snapshot_segment_write(segment, &hash.0).await?;
        }
        snapshot_segment_write_varint(segment, self.new_pages.len() as u64).await?;
        for hash in &self.new_pages {
            snapshot_segment_write(segment, &hash.0).await?;
        }
        snapshot_segment_write_varint(segment, self.created_at_ms).await
    }

    /// @emoji 📖️ Inverse of `encode`. Never panics on malformed input — every field read is
    /// bounds-checked by `pack::ByteReader` and every count is checked against
    /// `MAX_HASH_LIST_LEN`/`MAX_STRING_BYTES` before the corresponding `Vec`/`String` is allocated.
    #[cfg(test)]
    pub async fn decode(bytes: &[u8]) -> Result<SnapshotDescriptor, DbError> {
        let mut r = pack::ByteReader::new(bytes);
        let version = r.read_u8()?;
        if version != DESCRIPTOR_FORMAT_VERSION {
            return Err(DbError::Corrupt(format!("unsupported snapshot descriptor format version {version}")));
        }
        let document = ArtifactId(read_string(&mut r).await?);
        let generation = r.read_varint_u64()?;
        let parent_generation = read_option_u64(&mut r).await?;
        let head_seq = r.read_varint_u64()?;
        let commit_seq = r.read_varint_u64()?;
        let epoch = r.read_varint_u64()?;
        let chain_hash = r.read_array32()?;
        let protocol_version = r.read_varint_u64()? as u32;
        let vcs_head = match r.read_u8()? {
            0 => None,
            1 => Some(read_string(&mut r).await?),
            other => return Err(DbError::Corrupt(format!("bad option tag {other}"))),
        };
        let base_pack_hash = match r.read_u8()? {
            0 => None,
            1 => Some(ContentHash(r.read_array32()?)),
            other => return Err(DbError::Corrupt(format!("bad option tag {other}"))),
        };
        let roots = read_hash_list(&mut r).await?;
        let new_pages = read_hash_list(&mut r).await?;
        let created_at_ms = r.read_varint_u64()?;
        Ok(SnapshotDescriptor { document, generation, parent_generation, head_seq, commit_seq, epoch, chain_hash, protocol_version, vcs_head, base_pack_hash, roots, new_pages, created_at_ms })
    }
}

fn snapshot_varint(mut value: u64, output: &mut [u8; 10]) -> &[u8] {
    let mut len = 0;
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        output[len] = byte;
        len += 1;
        if value == 0 {
            return &output[..len];
        }
    }
}

fn snapshot_varint_len(value: u64) -> usize {
    let mut output = [0u8; 10];
    snapshot_varint(value, &mut output).len()
}

fn snapshot_field_len(bytes: &[u8]) -> usize {
    snapshot_varint_len(bytes.len() as u64) + bytes.len()
}

async fn snapshot_segment_write(segment: &mut pack::PackIdentitySegment<'_, SnapshotPageSink>, bytes: &[u8]) -> Result<(), DbError> {
    segment.write_fragment(bytes).await.map_err(DbError::from)
}

async fn snapshot_segment_write_varint(segment: &mut pack::PackIdentitySegment<'_, SnapshotPageSink>, value: u64) -> Result<(), DbError> {
    let mut output = [0u8; 10];
    snapshot_segment_write(segment, snapshot_varint(value, &mut output)).await
}

async fn snapshot_segment_write_field(segment: &mut pack::PackIdentitySegment<'_, SnapshotPageSink>, bytes: &[u8]) -> Result<(), DbError> {
    snapshot_segment_write_varint(segment, bytes.len() as u64).await?;
    snapshot_segment_write(segment, bytes).await
}

#[cfg(test)]
async fn write_string(w: &mut pack::ByteWriter, s: &str) {
    w.write_varint_u64(s.len() as u64);
    w.write_bytes(s.as_bytes());
}

#[cfg(test)]
async fn read_string(r: &mut pack::ByteReader<'_>) -> Result<String, DbError> {
    let len = r.read_varint_u64()?;
    check_len(len, MAX_STRING_BYTES, "snapshot descriptor string")?;
    let bytes = r.read_bytes(len as usize)?;
    String::from_utf8(bytes.to_vec()).map_err(|_| DbError::Corrupt("invalid utf8 in snapshot descriptor".to_string()))
}

#[cfg(test)]
async fn write_option_u64(w: &mut pack::ByteWriter, value: Option<u64>) {
    match value {
        Some(v) => {
            w.write_u8(1);
            w.write_varint_u64(v);
        }
        None => w.write_u8(0),
    }
}

#[cfg(test)]
async fn read_option_u64(r: &mut pack::ByteReader<'_>) -> Result<Option<u64>, DbError> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(r.read_varint_u64()?)),
        other => Err(DbError::Corrupt(format!("bad option tag {other}"))),
    }
}

#[cfg(test)]
async fn write_hash_list(w: &mut pack::ByteWriter, hashes: &[ContentHash]) {
    w.write_varint_u64(hashes.len() as u64);
    for hash in hashes {
        w.write_bytes(&hash.0);
    }
}

#[cfg(test)]
async fn read_hash_list(r: &mut pack::ByteReader<'_>) -> Result<Vec<ContentHash>, DbError> {
    let count = r.read_varint_u64()?;
    check_len(count, MAX_HASH_LIST_LEN, "snapshot descriptor hash list")?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(ContentHash(r.read_array32()?));
    }
    Ok(out)
}
//#endregion 🔖️Descriptor

//#region 🔖️SegmentIo
/// @emoji 🪟️ A length-bounded, base-shifted view over a borrowed byte buffer that implements
/// `pack::PackSource` — the mechanism this crate uses to open one generation's own pack structure
/// (`PackFile::open_manifest`, `read_footer_only`) at an arbitrary offset inside a larger
/// multi-generation concatenation, without touching any `pack_format` private internals.
#[cfg(test)]
struct SubSource<'a> {
    inner: &'a [u8],
    base: u64,
    len: u64,
}

#[cfg(test)]
impl<'a> pack::PackSource for SubSource<'a> {
    async fn len(&self) -> u64 {
        self.len
    }

    async fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, pack::PackError> {
        if offset > self.len {
            return Err(pack::PackError::Truncated(offset));
        }
        let available = ((self.len - offset) as usize).min(buf.len());
        self.inner.read_at(self.base + offset, &mut buf[..available]).await
    }
}

#[derive(Clone, Copy)]
struct PageSubSource<'pages> {
    inner: &'pages db_storage::DbIoPages,
    base: u64,
    len: u64,
}

impl pack::PackSource for PageSubSource<'_> {
    async fn len(&self) -> u64 {
        self.len
    }

    async fn read_at(&self, offset: u64, output: &mut [u8]) -> Result<usize, pack::PackError> {
        if offset > self.len {
            return Err(pack::PackError::Truncated(offset));
        }
        let absolute = self.base.checked_add(offset).ok_or(pack::PackError::Truncated(offset))? as usize;
        let available = ((self.len - offset) as usize).min(output.len());
        let mut base = 0usize;
        let mut written = 0usize;
        for fragment in self.inner.fragments() {
            let end = base + fragment.len();
            if end <= absolute {
                base = end;
                continue;
            }
            let start = absolute.saturating_sub(base);
            let count = (available - written).min(fragment.len() - start);
            output[written..written + count].copy_from_slice(&fragment[start..start + count]);
            written += count;
            base = end;
            if written == available {
                break;
            }
        }
        Ok(written)
    }
}

struct SnapshotDescriptorReader<'source, 'control, S: pack::PackSource> {
    source: &'source S,
    cursor: u64,
    end: u64,
    crc: pack::codec::Crc32cCursor,
    control: &'control mut SnapshotCursorControl,
}

impl<S: pack::PackSource> SnapshotDescriptorReader<'_, '_, S> {
    async fn fixed<const N: usize>(&mut self) -> Result<[u8; N], DbError> {
        if self.cursor.checked_add(N as u64).is_none_or(|end| end > self.end) {
            return Err(DbError::Corrupt("snapshot descriptor ended early".to_string()));
        }
        self.control.grant()?;
        let mut output = [0u8; N];
        self.source.read_exact_at(self.cursor, &mut output).await?;
        self.cursor += N as u64;
        self.crc.update_page(&output);
        Ok(output)
    }

    async fn byte(&mut self) -> Result<u8, DbError> {
        Ok(self.fixed::<1>().await?[0])
    }

    async fn varint(&mut self) -> Result<u64, DbError> {
        let mut value = 0u64;
        for shift in (0..70).step_by(7) {
            let byte = self.byte().await?;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(DbError::Corrupt("snapshot descriptor varint overflow".to_string()))
    }

    async fn option_u64(&mut self) -> Result<Option<u64>, DbError> {
        match self.byte().await? {
            0 => Ok(None),
            1 => Ok(Some(self.varint().await?)),
            tag => Err(DbError::Corrupt(format!("bad snapshot option tag {tag}"))),
        }
    }

    async fn text(&mut self) -> Result<String, DbError> {
        let len = self.varint().await?;
        check_len(len, MAX_STRING_BYTES, "snapshot descriptor string")?;
        if self.cursor.checked_add(len).is_none_or(|end| end > self.end) {
            return Err(DbError::Corrupt("snapshot descriptor string ended early".to_string()));
        }
        let mut remaining = len as usize;
        let mut output = Vec::with_capacity(remaining);
        let mut fragment = [0u8; 4096];
        while remaining != 0 {
            self.control.grant()?;
            let count = remaining.min(fragment.len());
            self.source.read_exact_at(self.cursor, &mut fragment[..count]).await?;
            self.cursor += count as u64;
            remaining -= count;
            self.crc.update_page(&fragment[..count]);
            output.extend_from_slice(&fragment[..count]);
        }
        String::from_utf8(output).map_err(|_| DbError::Corrupt("invalid utf8 in snapshot descriptor".to_string()))
    }

    async fn hashes(&mut self) -> Result<Vec<ContentHash>, DbError> {
        let count = self.varint().await?;
        check_len(count, MAX_HASH_LIST_LEN, "snapshot descriptor hash list")?;
        if count.checked_mul(32).and_then(|bytes| self.cursor.checked_add(bytes)).is_none_or(|end| end > self.end) {
            return Err(DbError::Corrupt("snapshot descriptor hash list ended early".to_string()));
        }
        let mut output = Vec::with_capacity(count as usize);
        for _ in 0..count {
            output.push(ContentHash(self.fixed::<32>().await?));
        }
        Ok(output)
    }
}

async fn decode_snapshot_descriptor(source: &impl pack::PackSource, control: &mut SnapshotCursorControl) -> Result<SnapshotDescriptor, DbError> {
    use pack::PackSource as _;
    let offset = pack::HEADER_SIZE as u64;
    let mut prefix = [0u8; 12];
    control.grant()?;
    source.read_exact_at(offset, &mut prefix).await?;
    let kind = prefix[0];
    let flags = prefix[1];
    if flags != 0 {
        return Err(DbError::Corrupt("snapshot descriptor segment must use the identity codec".to_string()));
    }
    let mut position = 2usize;
    let segment_len = pack::os_pack::read_varint_u64(&prefix, &mut position)?;
    check_len(segment_len, 64 * 1024 * 1024, "snapshot descriptor segment length")?;
    let header_len = position as u64;
    let end = offset.checked_add(header_len).and_then(|cursor| cursor.checked_add(segment_len)).ok_or(DbError::LimitExceeded("snapshot descriptor segment range"))?;
    let mut crc = pack::codec::Crc32cCursor::new();
    crc.update_page(&prefix[..position]);
    let mut reader = SnapshotDescriptorReader { source, cursor: offset + header_len, end, crc, control };
    let version = reader.byte().await?;
    if version != DESCRIPTOR_FORMAT_VERSION {
        return Err(DbError::Corrupt(format!("unsupported snapshot descriptor format version {version}")));
    }
    let document = ArtifactId(reader.text().await?);
    let generation = reader.varint().await?;
    let parent_generation = reader.option_u64().await?;
    let head_seq = reader.varint().await?;
    let commit_seq = reader.varint().await?;
    let epoch = reader.varint().await?;
    let chain_hash = reader.fixed::<32>().await?;
    let protocol_version = u32::try_from(reader.varint().await?).map_err(|_| DbError::Corrupt("snapshot protocol version exceeds u32".to_string()))?;
    let vcs_head = match reader.byte().await? {
        0 => None,
        1 => Some(reader.text().await?),
        tag => return Err(DbError::Corrupt(format!("bad snapshot option tag {tag}"))),
    };
    let base_pack_hash = match reader.byte().await? {
        0 => None,
        1 => Some(ContentHash(reader.fixed::<32>().await?)),
        tag => return Err(DbError::Corrupt(format!("bad snapshot option tag {tag}"))),
    };
    let roots = reader.hashes().await?;
    let new_pages = reader.hashes().await?;
    let created_at_ms = reader.varint().await?;
    if reader.cursor != reader.end {
        return Err(DbError::Corrupt("snapshot descriptor segment has trailing bytes".to_string()));
    }
    let mut crc_bytes = [0u8; 4];
    reader.control.grant()?;
    source.read_exact_at(end, &mut crc_bytes).await?;
    if u32::from_le_bytes(crc_bytes) != reader.crc.finish() {
        return Err(DbError::Corrupt("snapshot descriptor segment checksum mismatch".to_string()));
    }
    if kind != pack::KIND_SNAPSHOT {
        return Err(DbError::Corrupt(format!("expected KIND_SNAPSHOT segment (0x{:02x}), found 0x{kind:02x}", pack::KIND_SNAPSHOT)));
    }
    Ok(SnapshotDescriptor { document, generation, parent_generation, head_seq, commit_seq, epoch, chain_hash, protocol_version, vcs_head, base_pack_hash, roots, new_pages, created_at_ms })
}
//#endregion 🔖️SegmentIo

//#region 🔖️Generation
struct SnapshotPageSink {
    writer: db_storage::DbIoPageWriter,
}

impl pack::PackSink for SnapshotPageSink {
    async fn write_all(&mut self, bytes: &[u8]) -> Result<(), pack::PackError> {
        let mut cursor = 0;
        while cursor < bytes.len() {
            cursor += self.writer.write_fragment(&bytes[cursor..]).map_err(|error| pack::PackError::Io(error.to_string()))?;
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
        let writer = db_storage::DbIoPageWriter::try_reserve(db_storage::DB_IO_OPERATION_PAGES).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
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

    async fn into_pages(self) -> Result<db_storage::DbIoPages, DbError> {
        self.writer.seal_retained().await.map_err(db_storage::DbIoPageWriterRejected::into_error)
    }
}

trait SnapshotPageSource {
    fn len(&self) -> usize;
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

struct OptionalSnapshotPages<'pages> {
    slots: &'pages [Option<Page>],
    len: usize,
}

const SNAPSHOT_PUBLICATION_CLAIMS: usize = 64;

static SNAPSHOT_PUBLICATION_CLAIM_STATE: [std::sync::atomic::AtomicU64; SNAPSHOT_PUBLICATION_CLAIMS] = [const { std::sync::atomic::AtomicU64::new(0) }; SNAPSHOT_PUBLICATION_CLAIMS];

struct SnapshotPublicationClaim {
    slot: usize,
    identity: u64,
}

impl SnapshotPublicationClaim {
    fn try_claim(document: &ArtifactId) -> Result<Self, DbError> {
        let hash = blake3::hash(document.0.as_bytes());
        let bytes = hash.as_bytes();
        let identity = u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]) | 1;
        let slot = usize::try_from(identity % SNAPSHOT_PUBLICATION_CLAIMS as u64).map_err(|_| DbError::LimitExceeded("snapshot publication claim slot"))?;
        match SNAPSHOT_PUBLICATION_CLAIM_STATE[slot].compare_exchange(0, identity, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire) {
            Ok(_) => Ok(Self { slot, identity }),
            Err(observed) if observed == identity => Err(DbError::Conflict("snapshot publication already claimed".to_string())),
            Err(_) => Err(DbError::LimitExceeded("snapshot publication claim collision")),
        }
    }
}

impl Drop for SnapshotPublicationClaim {
    fn drop(&mut self) {
        let _ = SNAPSHOT_PUBLICATION_CLAIM_STATE[self.slot].compare_exchange(self.identity, 0, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire);
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

/// @emoji 🏗️ Builds one generation's complete, self-contained `.spk` pack bytes: the descriptor
/// as the first (`KIND_SNAPSHOT`) segment, `new_pages` as `KIND_CHUNK` segments in order (so
/// `descriptor.new_pages[i]` is `pack::ChunkId(i)`), then the standard `pack::PackWriter::finish`
/// trailer. If `parent_footer_position` is `Some`, the trailing footer's `prev_footer_offset` is
/// patched in place afterward (see module doc) and `REQUIRED_FOOTER_CHAIN` is set in both the
/// header and the footer's `required_flags`.
async fn build_generation_page_source<P: SnapshotPageSource + ?Sized>(descriptor: &SnapshotDescriptor, new_pages: &P, parent_footer_position: Option<u64>, control: &mut SnapshotCursorControl) -> Result<db_storage::DbIoPages, DbError> {
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
        1 + body.vcs_head.as_ref().map_or(0, |value| snapshot_field_len(value.as_bytes())),
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
    snapshot_segment_write(segment, &[u8::from(body.vcs_head.is_some())]).await?;
    if let Some(head) = &body.vcs_head {
        snapshot_segment_write_field(segment, head.as_bytes()).await?;
    }
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

async fn build_generation_retained_expected<P: SnapshotPageSource + ?Sized>(document: &ArtifactId, generation: u64, body: &SnapshotBody, new_pages: &P, control: &mut SnapshotCursorControl) -> Result<db_storage::DbIoPages, DbError> {
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

pub async fn build_generation_pages(descriptor: &SnapshotDescriptor, new_pages: &[Page], parent_footer_position: Option<u64>, control: &mut SnapshotCursorControl) -> Result<db_storage::DbIoPages, DbError> {
    build_generation_page_source(descriptor, new_pages, parent_footer_position, control).await
}

#[cfg(test)]
async fn build_generation(descriptor: &SnapshotDescriptor, new_pages: &[Page], parent_footer_position: Option<u64>) -> Result<Vec<u8>, DbError> {
    let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
    let mut pages = build_generation_pages(descriptor, new_pages, parent_footer_position, &mut control).await?;
    let mut prepared = db_storage::db_io_prepare_platform(&pages)?.await?;
    let output = prepared.as_slice().to_vec();
    let _ = prepared.close_step()?;
    drop(prepared);
    let _ = pages.close_step()?;
    drop(pages);
    Ok(output)
}

/// @emoji 🩹️ `pack::PackWriter::finish` always writes `prev_footer_offset = 0` (there is no public
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

/// @emoji 🪪️ One opened generation: its decoded descriptor plus enough footer/position state to
/// walk to its parent (`parent_footer_offset`) or read its own chunks (`base`/`len`).
pub struct GenerationHandle {
    pub descriptor: SnapshotDescriptor,
    base: u64,
    len: u64,
    footer_required_flags: u32,
    footer_prev_offset: u64,
}

impl GenerationHandle {
    pub async fn generation(&self) -> u64 {
        self.descriptor.generation
    }

    /// @emoji ⛓️ The absolute offset (within the enclosing `combined` buffer) of the parent
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

#[cfg(test)]
async fn open_generation_at(combined: &[u8], base: u64, len: u64, footer: &pack::Footer) -> Result<GenerationHandle, DbError> {
    let sub = SubSource { inner: combined, base, len };
    let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
    let descriptor = decode_snapshot_descriptor(&sub, &mut control).await?;
    Ok(GenerationHandle { descriptor, base, len, footer_required_flags: footer.required_flags, footer_prev_offset: footer.prev_footer_offset })
}

async fn open_generation_pages_at(combined: &db_storage::DbIoPages, base: u64, len: u64, footer: &pack::Footer, control: &mut SnapshotCursorControl) -> Result<GenerationHandle, DbError> {
    let sub = PageSubSource { inner: combined, base, len };
    let descriptor = decode_snapshot_descriptor(&sub, control).await?;
    Ok(GenerationHandle { descriptor, base, len, footer_required_flags: footer.required_flags, footer_prev_offset: footer.prev_footer_offset })
}

async fn open_latest_pages(combined: &db_storage::DbIoPages, control: &mut SnapshotCursorControl) -> Result<GenerationHandle, DbError> {
    control.grant()?;
    let whole = PageSubSource { inner: combined, base: 0, len: combined.len() as u64 };
    let footer = pack::read_footer_only(&whole).await?;
    if footer.file_len > combined.len() as u64 {
        return Err(DbError::Corrupt("snapshot generation footer.file_len exceeds retained pages".to_string()));
    }
    let base = combined.len() as u64 - footer.file_len;
    open_generation_pages_at(combined, base, footer.file_len, &footer, control).await
}

/// @emoji 🔚️ Opens the LAST generation physically present in `combined` (the one whose footer sits
/// at `combined.len() - FOOTER_SIZE`) — the entry point for reading a freshly-fetched or
/// freshly-materialized retained chain.
#[cfg(test)]
pub async fn open_latest(combined: &[u8]) -> Result<GenerationHandle, DbError> {
    let whole = SubSource { inner: combined, base: 0, len: combined.len() as u64 };
    let footer = pack::read_footer_only(&whole).await?;
    if footer.file_len > combined.len() as u64 {
        return Err(DbError::Corrupt("snapshot generation footer.file_len exceeds buffer length".to_string()));
    }
    let base = combined.len() as u64 - footer.file_len;
    open_generation_at(combined, base, footer.file_len, &footer).await
}

/// @emoji ⬅️ Opens the generation whose own footer starts at absolute offset `footer_offset`
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

/// @emoji 📄️ Reads one page's raw bytes by content hash, starting at `handle` and walking to
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

//#region 🔖️Manager
/// @emoji 🌱️ Which lineage a `SnapshotManager::publish` call starts: `FullBaseline` is
/// self-sufficient (safe future `retain_from` floor); `Incremental` chains to the document's
/// current latest generation (errors if there isn't one yet).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SnapshotOrigin {
    FullBaseline,
    Incremental,
}

/// @emoji 📦️ Everything about a generation that isn't derived from `new_pages`/plumbing — the
/// `SnapshotManager::publish` caller-supplied half of a `SnapshotDescriptor`.
#[derive(Clone, Debug)]
pub struct SnapshotBody {
    pub head_seq: u64,
    pub commit_seq: u64,
    pub epoch: u64,
    pub chain_hash: [u8; 32],
    pub protocol_version: u32,
    pub vcs_head: Option<String>,
    pub base_pack_hash: Option<ContentHash>,
    pub roots: Vec<ContentHash>,
    pub created_at_ms: u64,
}

/// 🧵️ Generation-qualified publication witness returning the exact dynamic body owner.
#[derive(Debug)]
pub struct SnapshotRetainedPublication {
    generation: u64,
    body: SnapshotBody,
}

impl SnapshotRetainedPublication {
    pub fn into_parts(self) -> (u64, SnapshotBody) {
        (self.generation, self.body)
    }
}

/// 🛡️ Atomic publication refusal retaining the exact body owner for incremental close.
#[derive(Debug)]
pub struct SnapshotRetainedPublicationRejected {
    error: DbError,
    body: SnapshotBody,
}

impl SnapshotRetainedPublicationRejected {
    pub fn into_parts(self) -> (DbError, SnapshotBody) {
        (self.error, self.body)
    }
}

/// @emoji 🎛️ Publish-time trigger thresholds — `should_snapshot` fires if any is met. This
/// crate's own choice of shape (the contract fixes only that triggers exist, not their inputs).
#[derive(Clone, Copy, Debug)]
pub struct SnapshotPolicy {
    pub max_ops_since_last: u64,
    pub max_bytes_since_last: u64,
    pub max_ms_since_last: u64,
}

impl SnapshotPolicy {
    pub async fn should_snapshot(&self, ops_since_last: u64, bytes_since_last: u64, ms_since_last: u64) -> bool {
        ops_since_last >= self.max_ops_since_last || bytes_since_last >= self.max_bytes_since_last || ms_since_last >= self.max_ms_since_last
    }
}

/// @emoji 🧑️‍💼️ Orchestrates `db_snapshot`'s pack-encoding logic on top of a
/// `db_storage::SnapshotStorage` backend: publish (`build_generation` + `write_generation`), load,
/// chain materialization, retention, and verification.
pub struct SnapshotManager<'storage, S: SnapshotStorage> {
    storage: &'storage S,
}

//#region 🔖️ChainCursor
/// @emoji ⏳️ One resumable snapshot-chain grant authority.
pub struct SnapshotCursorControl {
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    deadline: std::time::Instant,
    fuel: usize,
}

impl SnapshotCursorControl {
    pub fn new(cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("snapshot cursor fuel"));
        }
        Ok(Self { cancelled, deadline, fuel })
    }

    pub fn replenish(&mut self, fuel: usize, deadline: std::time::Instant) -> Result<(), DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("snapshot cursor fuel"));
        }
        self.fuel = fuel;
        self.deadline = deadline;
        Ok(())
    }

    fn grant(&mut self) -> Result<(), DbError> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Unavailable("snapshot cursor cancelled".to_string()));
        }
        if std::time::Instant::now() >= self.deadline {
            return Err(DbError::Unavailable("snapshot cursor deadline reached".to_string()));
        }
        self.fuel = self.fuel.checked_sub(1).ok_or(DbError::LimitExceeded("snapshot cursor fuel"))?;
        Ok(())
    }
}

/// @emoji ⛓️ Incremental snapshot-chain reader retaining at most one generation owner and one
/// prepared platform slot per grant.
#[must_use]
pub struct SnapshotChainCursor<'manager, 'storage, S: SnapshotStorage> {
    manager: &'manager SnapshotManager<'storage, S>,
    document: &'manager ArtifactId,
    through_generation: u64,
    control: SnapshotCursorControl,
    operation: Option<u64>,
    closed: bool,
}

impl<'manager, 'storage, S: SnapshotStorage> SnapshotChainCursor<'manager, 'storage, S> {
    pub fn operation(&self) -> Option<u64> {
        self.operation
    }

    async fn descriptor_at(&mut self, generation: u64) -> Result<(SnapshotDescriptor, usize), DbError> {
        self.control.grant()?;
        let mut pages = self.manager.storage.read_generation(self.document, generation).await?;
        self.operation = Some(pages.operation());
        let len = pages.len();
        self.control.grant()?;
        let descriptor = open_latest_pages(&pages, &mut self.control).await?.descriptor;
        drop(pages);
        Ok((descriptor, len))
    }

    pub async fn latest_descriptor(&mut self) -> Result<SnapshotDescriptor, DbError> {
        self.descriptor_at(self.through_generation).await.map(|(descriptor, _)| descriptor)
    }

    pub async fn descriptor(&mut self, generation: u64) -> Result<SnapshotDescriptor, DbError> {
        self.descriptor_at(generation).await.map(|(descriptor, _)| descriptor)
    }

    pub async fn read_page(&mut self, hash: ContentHash) -> Result<db_storage::DbIoPages, DbError> {
        let mut generation = Some(self.through_generation);
        while let Some(current) = generation {
            self.control.grant()?;
            let mut source = self.manager.storage.read_generation(self.document, current).await?;
            self.operation = Some(source.operation());
            self.control.grant()?;
            let handle = open_latest_pages(&source, &mut self.control).await?;
            if let Some(index) = handle.descriptor.new_pages.iter().position(|candidate| *candidate == hash) {
                let sub = PageSubSource { inner: &source, base: handle.base, len: handle.len };
                let file = pack::PackFile::open_manifest(sub, &pack::PackLimits::default(), pack::os_pack::VerificationLevel::Standard).await?;
                let mut cursor = file.identity_chunk_cursor(pack::ChunkId(index as u32), pack::os_pack::VerificationLevel::Standard)?;
                let mut writer = db_storage::DbIoPageWriter::try_reserve_for_operation(source.operation(), usize::try_from(cursor.len()).map_err(|_| DbError::LimitExceeded("snapshot identity chunk length"))?.div_ceil(db_storage::DB_IO_PAGE_BYTES))
                    .map_err(db_storage::DbIoPageWriterRejected::into_error)?;
                let mut fragment = [0u8; db_storage::DB_IO_PAGE_BYTES];
                loop {
                    self.control.grant()?;
                    let read = cursor.read_fragment(&mut fragment).await?;
                    if read == 0 {
                        break;
                    }
                    if writer.write_fragment(&fragment[..read])? != read {
                        return Err(DbError::Internal("snapshot identity cursor made partial writer progress".to_string()));
                    }
                }
                drop(cursor);
                drop(file);
                drop(source);
                return writer.seal_retained().await.map_err(db_storage::DbIoPageWriterRejected::into_error);
            }
            generation = handle.descriptor.parent_generation;
            drop(source);
        }
        Err(DbError::NotFound(format!("page {hash} not found anywhere in the snapshot chain")))
    }

    pub async fn materialize_pages(&mut self) -> Result<db_storage::DbIoPages, DbError> {
        let mut root = self.through_generation;
        let mut current = Some(self.through_generation);
        let mut total = 0usize;
        while let Some(generation) = current {
            let (descriptor, len) = self.descriptor_at(generation).await?;
            root = generation;
            total = total.checked_add(len).ok_or(DbError::LimitExceeded("snapshot chain bytes"))?;
            current = descriptor.parent_generation;
        }
        let page_count = total.div_ceil(db_storage::DB_IO_PAGE_BYTES);
        if page_count > db_storage::DB_IO_OPERATION_PAGES {
            return Err(DbError::LimitExceeded("snapshot chain page reservation"));
        }
        let mut writer = db_storage::DbIoPageWriter::try_reserve(page_count).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
        self.operation = Some(writer.operation());
        let mut expected_parent = None;
        for generation in root..=self.through_generation {
            self.control.grant()?;
            let mut source = self.manager.storage.read_generation(self.document, generation).await?;
            self.control.grant()?;
            let descriptor = open_latest_pages(&source, &mut self.control).await?.descriptor;
            if descriptor.parent_generation != expected_parent {
                return Err(DbError::Corrupt("snapshot generation lineage is not contiguous".to_string()));
            }
            expected_parent = Some(generation);
            for fragment in source.fragments() {
                let mut offset = 0;
                while offset < fragment.len() {
                    self.control.grant()?;
                    offset += writer.write_fragment(&fragment[offset..])?;
                }
            }
            drop(source);
        }
        writer.seal_retained().await.map_err(db_storage::DbIoPageWriterRejected::into_error)
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        self.control.grant()?;
        if self.closed {
            return Ok(false);
        }
        self.closed = true;
        self.operation = None;
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed && self.operation.is_none()
    }
}

impl<S: SnapshotStorage> Drop for SnapshotChainCursor<'_, '_, S> {
    fn drop(&mut self) {
        self.control.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.operation = None;
        self.closed = true;
    }
}
//#endregion 🔖️ChainCursor

impl<'storage, S: SnapshotStorage> SnapshotManager<'storage, S> {
    pub async fn new(storage: &'storage S) -> SnapshotManager<'storage, S> {
        SnapshotManager { storage }
    }

    pub fn chain_cursor<'manager>(&'manager self, document: &'manager ArtifactId, through_generation: u64, control: SnapshotCursorControl) -> SnapshotChainCursor<'manager, 'storage, S> {
        SnapshotChainCursor { manager: self, document, through_generation, control, operation: None, closed: false }
    }

    /// @emoji ✍️ Builds and durably writes the next generation. `origin == Incremental` chains to
    /// the document's current `latest_generation` (`DbError::InvalidArgument` if there is none
    /// yet); `origin == FullBaseline` starts (or restarts) a self-sufficient lineage.
    pub async fn publish(&self, document: &ArtifactId, origin: SnapshotOrigin, new_pages: &[Page], body: SnapshotBody) -> Result<u64, DbError> {
        self.publish_page_source(document, origin, new_pages, body).await
    }

    /// @emoji 🧵️ Publishes from a fixed optional page owner array without constructing a
    /// dynamic retained page graph.
    pub async fn publish_retained(&self, document: &ArtifactId, origin: SnapshotOrigin, new_pages: &[Option<Page>], retained_len: usize, body: SnapshotBody) -> Result<u64, DbError> {
        if retained_len > new_pages.len() || new_pages[..retained_len].iter().any(Option::is_none) {
            return Err(DbError::InvalidArgument("snapshot retained page source is not contiguous".to_string()));
        }
        self.publish_page_source(document, origin, &OptionalSnapshotPages { slots: new_pages, len: retained_len }, body).await
    }

    /// 🛡️ Publishes one full baseline only while `expected_generation` owns the atomic document claim.
    pub async fn publish_retained_expected(
        &self,
        document: &ArtifactId,
        expected_generation: u64,
        new_pages: &[Option<Page>],
        retained_len: usize,
        body: SnapshotBody,
        control: &mut SnapshotCursorControl,
    ) -> Result<SnapshotRetainedPublication, SnapshotRetainedPublicationRejected> {
        let publication = async {
            if retained_len > new_pages.len() || new_pages[..retained_len].iter().any(Option::is_none) {
                return Err(DbError::InvalidArgument("snapshot retained page source is not contiguous".to_string()));
            }
            let _claim = SnapshotPublicationClaim::try_claim(document)?;
            let observed = self.storage.latest_generation(document).await?;
            if observed != Some(expected_generation) {
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(expected_generation), actual: crate::db_ids::GenerationId(observed.unwrap_or(0)) });
            }
            let generation = expected_generation.checked_add(1).ok_or(DbError::LimitExceeded("snapshot publication generation"))?;
            let pages = build_generation_retained_expected(document, generation, &body, &OptionalSnapshotPages { slots: new_pages, len: retained_len }, control).await?;
            self.storage.write_generation(document, generation, pages).await?;
            Ok(generation)
        }
        .await;
        match publication {
            Ok(generation) => Ok(SnapshotRetainedPublication { generation, body }),
            Err(error) => Err(SnapshotRetainedPublicationRejected { error, body }),
        }
    }

    async fn publish_page_source<P: SnapshotPageSource + ?Sized>(&self, document: &ArtifactId, origin: SnapshotOrigin, new_pages: &P, body: SnapshotBody) -> Result<u64, DbError> {
        let _claim = SnapshotPublicationClaim::try_claim(document)?;
        let latest = self.storage.latest_generation(document).await?;
        let (generation, parent_generation, parent_footer_position) = match origin {
            SnapshotOrigin::FullBaseline => (latest.map_or(0, |g| g + 1), None, None),
            SnapshotOrigin::Incremental => {
                let parent_generation = latest.ok_or_else(|| DbError::InvalidArgument("cannot publish an incremental snapshot with no prior generation".to_string()))?;
                let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
                let control = SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), db_storage::DB_IO_OPERATION_PAGES * 8)?;
                let mut cursor = self.chain_cursor(document, parent_generation, control);
                let mut combined = cursor.materialize_pages().await?;
                let parent_footer_position = combined.len() as u64 - pack::FOOTER_SIZE as u64;
                let _ = combined.close_step()?;
                drop(combined);
                let _ = cursor.close_step()?;
                (parent_generation + 1, Some(parent_generation), Some(parent_footer_position))
            }
        };

        let mut hashes = Vec::with_capacity(new_pages.len());
        for index in 0..new_pages.len() {
            hashes.push(new_pages.page(index).ok_or_else(|| DbError::Internal("snapshot retained page source lost an owner".to_string()))?.hash);
        }
        let descriptor = SnapshotDescriptor {
            document: document.clone(),
            generation,
            parent_generation,
            head_seq: body.head_seq,
            commit_seq: body.commit_seq,
            epoch: body.epoch,
            chain_hash: body.chain_hash,
            protocol_version: body.protocol_version,
            vcs_head: body.vcs_head,
            base_pack_hash: body.base_pack_hash,
            roots: body.roots,
            new_pages: hashes,
            created_at_ms: body.created_at_ms,
        };
        let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        let pages = build_generation_page_source(&descriptor, new_pages, parent_footer_position, &mut control).await?;
        self.storage.write_generation(document, generation, pages).await?;
        Ok(generation)
    }

    /// @emoji 🥇️ The document's latest generation number and descriptor, or `None` if it has no
    /// snapshot yet.
    pub async fn load_latest(&self, document: &ArtifactId) -> Result<Option<(u64, SnapshotDescriptor)>, DbError> {
        match self.storage.latest_generation(document).await? {
            None => Ok(None),
            Some(generation) => {
                let mut bytes = self.storage.read_generation(document, generation).await?;
                let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
                let handle = open_latest_pages(&bytes, &mut control).await?;
                let descriptor = handle.descriptor;
                let _ = bytes.close_step()?;
                drop(bytes);
                Ok(Some((generation, descriptor)))
            }
        }
    }

    /// @emoji 🎯️ Selection: the highest-numbered generation whose `head_seq` does not exceed
    /// `at_most_head_seq`, or `None` if no generation qualifies — the snapshot a materializer
    /// should start replaying the WAL suffix from for a point-in-time read.
    pub async fn select_generation(&self, document: &ArtifactId, at_most_head_seq: u64) -> Result<Option<u64>, DbError> {
        let mut best: Option<(u64, u64)> = None;
        let mut generations = self.storage.list_generations(document).await?;
        let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        for generation in generations.as_slice() {
            let mut bytes = self.storage.read_generation(document, *generation).await?;
            let handle = open_latest_pages(&bytes, &mut control).await?;
            if handle.descriptor.head_seq <= at_most_head_seq {
                let better = best.is_none_or(|(_, best_head_seq)| handle.descriptor.head_seq > best_head_seq);
                if better {
                    best = Some((*generation, handle.descriptor.head_seq));
                }
            }
            let _ = bytes.close_step()?;
            drop(bytes);
        }
        let _ = generations.close_step();
        drop(generations);
        Ok(best.map(|(generation, _)| generation))
    }

    /// @emoji 🗑️ Deletes every generation strictly below `floor_generation`. `floor_generation`
    /// must itself be a full baseline (`parent_generation.is_none()`) — see module doc's scope
    /// boundary on why an incremental floor is rejected rather than silently breaking its chain.
    pub async fn retain_from(&self, document: &ArtifactId, floor_generation: u64) -> Result<(), DbError> {
        let mut floor_bytes = self.storage.read_generation(document, floor_generation).await?;
        let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        let floor_handle = open_latest_pages(&floor_bytes, &mut control).await?;
        let incremental = floor_handle.descriptor.parent_generation.is_some();
        let _ = floor_bytes.close_step()?;
        drop(floor_bytes);
        if incremental {
            return Err(DbError::InvalidArgument("retention floor must be a full-baseline generation (no parent)".to_string()));
        }
        let mut generations = self.storage.list_generations(document).await?;
        for generation in generations.as_slice() {
            if *generation < floor_generation {
                self.storage.delete_generation(document, *generation).await?;
            }
        }
        let _ = generations.close_step();
        drop(generations);
        Ok(())
    }

    /// @emoji 🔬️ Verifies generation `generation` decodes cleanly at `level`: the `KIND_SNAPSHOT`
    /// descriptor round-trips (`open_latest` itself decodes it) and every declared local chunk
    /// (`descriptor.new_pages`) decodes — at `VerificationLevel::Full` this transitively checks
    /// each chunk's content hash too, since `pack::PackFile::read_chunk` already validates it
    /// against the chunk table `pack::PackWriter::write_chunk` built from these same page bytes;
    /// no separate hash recomputation needed here.
    pub async fn verify(&self, document: &ArtifactId, generation: u64, level: pack::os_pack::VerificationLevel) -> Result<(), DbError> {
        let mut bytes = self.storage.read_generation(document, generation).await?;
        let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        let handle = open_latest_pages(&bytes, &mut control).await?;
        let sub = PageSubSource { inner: &bytes, base: 0, len: bytes.len() as u64 };
        let file = pack::PackFile::open_manifest(sub, &pack::PackLimits::default(), level).await?;
        for index in 0..handle.descriptor.new_pages.len() {
            let mut cursor = file.identity_chunk_cursor(pack::ChunkId(index as u32), level)?;
            let mut fragment = [0u8; db_storage::DB_IO_PAGE_BYTES];
            while cursor.read_fragment(&mut fragment).await? != 0 {
                control.grant()?;
            }
        }
        drop(file);
        drop(bytes);
        Ok(())
    }
}
//#endregion 🔖️Manager

//#region 🔖️Lease
/// @emoji ⏳️ The fencing primitive the module doc's "Scope boundary" note references: this crate
/// deliberately keeps `SnapshotManager::publish`/`retain_from` as mechanical, lease-agnostic
/// operations (concurrency coordination is `db_compact`'s "online compaction with manifest CAS +
/// fencing" responsibility) — `SnapshotLease` is the thin `db_storage::LeaseStorage` wrapper a
/// caller (a document actor, or `db_compact`) uses to serialize concurrent `publish`/`retain_from`
/// calls for one document before invoking them, keyed by a resource name derived from the document
/// id alone (so two different documents' snapshot builders never contend on the same lease).
pub struct SnapshotLease;

impl SnapshotLease {
    /// @emoji 🏷️ The `LeaseStorage` resource name guarding `document`'s snapshot builder.
    // 🚫️async: E1 pure accessor consumed synchronously by `acquire`/`renew`/`release`/`current` — see R9
    pub fn resource(document: &ArtifactId) -> String {
        format!("snapshot:{document}")
    }

    /// @emoji 🤝️ Acquires (or idempotently re-acquires) the snapshot-builder lease for `document`.
    pub async fn acquire(storage: &impl LeaseStorage, document: &ArtifactId, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        storage.acquire(&Self::resource(document), holder, ttl_ms, now_ms).await
    }

    /// @emoji ♻️ Extends `holder`'s existing lease for `document` — e.g. around a long
    /// retained chain cursor + `publish` sequence for a deep incremental chain.
    pub async fn renew(storage: &impl LeaseStorage, document: &ArtifactId, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        storage.renew(&Self::resource(document), holder, fence, ttl_ms, now_ms).await
    }

    /// @emoji 🕊️ Releases `holder`'s lease for `document` once its `publish`/`retain_from` call
    /// has completed.
    pub async fn release(storage: &impl LeaseStorage, document: &ArtifactId, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        storage.release(&Self::resource(document), holder, fence).await
    }

    /// @emoji 👀️ The lease's current holder/fence for `document`, or `None` if unheld — lets a
    /// caller check whether it's safe to `retain_from` without blindly racing another builder.
    pub async fn current(storage: &impl LeaseStorage, document: &ArtifactId, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        storage.current(&Self::resource(document), now_ms).await
    }
}
//#endregion 🔖️Lease

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use db_storage::MemoryStorage;

    fn pages(bytes: &[u8]) -> db_storage::DbIoPages {
        let mut writer = db_storage::DbIoPageWriter::try_reserve(bytes.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).expect("test snapshot writer admitted");
        for fragment in bytes.chunks(db_storage::DB_IO_PAGE_BYTES) {
            assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
        }
        writer.finish().unwrap()
    }

    //#region 🔖️Descriptor
    async fn sample_descriptor(generation: u64, parent: Option<u64>) -> SnapshotDescriptor {
        SnapshotDescriptor {
            document: "doc-1".into(),
            generation,
            parent_generation: parent,
            head_seq: generation * 10,
            commit_seq: generation * 5,
            epoch: 1,
            chain_hash: [generation as u8; 32],
            protocol_version: 1,
            vcs_head: Some("ck-abcdef".to_string()),
            base_pack_hash: Some(pack::ContentHash([9u8; 32])),
            roots: vec![pack::ContentHash([1u8; 32]), pack::ContentHash([2u8; 32])],
            new_pages: vec![],
            created_at_ms: 1_700_000_000_000 + generation,
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn descriptor_encode_decode_round_trips_all_fields() {
        let descriptor = sample_descriptor(3, Some(2)).await;
        let bytes = descriptor.encode().await;
        let decoded = SnapshotDescriptor::decode(&bytes).await.unwrap();
        assert_eq!(decoded, descriptor);
    }

    #[semio_framework_async_macros::async_test]
    async fn descriptor_encode_decode_round_trips_none_optionals() {
        let mut descriptor = sample_descriptor(0, None).await;
        descriptor.vcs_head = None;
        descriptor.base_pack_hash = None;
        descriptor.roots.clear();
        let bytes = descriptor.encode().await;
        let decoded = SnapshotDescriptor::decode(&bytes).await.unwrap();
        assert_eq!(decoded, descriptor);
        assert_eq!(decoded.frontier().await.head_seq, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn descriptor_decode_rejects_bad_version_tag() {
        let mut bytes = sample_descriptor(0, None).await.encode().await;
        bytes[0] = 0xFF;
        assert!(matches!(SnapshotDescriptor::decode(&bytes).await, Err(DbError::Corrupt(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn descriptor_decode_never_panics_on_truncated_input() {
        let bytes = sample_descriptor(1, Some(0)).await.encode().await;
        for len in 0..bytes.len() {
            assert!(SnapshotDescriptor::decode(&bytes[..len]).await.is_err(), "expected error at truncation length {len}");
        }
    }
    //#endregion 🔖️Descriptor

    //#region 🔖️Generation
    async fn page(bytes: &[u8]) -> Page {
        Page::try_from_pages(pages(bytes)).await.unwrap()
    }

    #[semio_framework_async_macros::async_test]
    async fn single_generation_round_trips_through_pack_public_api() {
        let pages = vec![page(b"page-zero").await, page(b"page-one").await];
        let mut descriptor = sample_descriptor(0, None).await;
        descriptor.new_pages = pages.iter().map(|p| p.hash).collect();
        descriptor.roots = vec![pages[0].hash];

        let bytes = build_generation(&descriptor, &pages, None).await.unwrap();

        // 🔬️ A real `pack::PackFile`, unrelated to this crate's own reader, must accept the bytes.
        let file = pack::PackFile::open_manifest(bytes.as_slice(), &pack::PackLimits::default(), pack::os_pack::VerificationLevel::Full).await.unwrap();
        assert_eq!(file.chunk_count(), 2);
        assert_eq!(file.manifest().unwrap().schema_name, "");

        let handle = open_latest(&bytes).await.unwrap();
        assert_eq!(handle.generation().await, 0);
        assert_eq!(handle.descriptor, descriptor);
        assert!(handle.parent_footer_offset().await.is_none());

        let read_back = read_page(&bytes, &handle, pages[1].hash).await.unwrap();
        assert_eq!(read_back, b"page-one");
    }

    #[semio_framework_async_macros::async_test]
    async fn build_generation_rejects_new_pages_mismatched_with_descriptor() {
        let pages = vec![page(b"a").await];
        let descriptor = sample_descriptor(0, None).await; // descriptor.new_pages left empty
        assert!(matches!(build_generation(&descriptor, &pages, None).await, Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn footer_chain_flag_and_prev_offset_are_genuine_pack_wire_data() {
        let parent_pages = vec![page(b"gen0-page").await];
        let mut gen0_descriptor = sample_descriptor(0, None).await;
        gen0_descriptor.new_pages = parent_pages.iter().map(|p| p.hash).collect();
        let gen0_bytes = build_generation(&gen0_descriptor, &parent_pages, None).await.unwrap();

        let parent_footer_position = gen0_bytes.len() as u64 - pack::FOOTER_SIZE as u64;
        let child_pages = vec![page(b"gen1-page").await];
        let mut gen1_descriptor = sample_descriptor(1, Some(0)).await;
        gen1_descriptor.new_pages = child_pages.iter().map(|p| p.hash).collect();
        let gen1_bytes = build_generation(&gen1_descriptor, &child_pages, Some(parent_footer_position)).await.unwrap();

        // 🔬️ Read gen1's footer via `pack`'s own public superblock API, not this crate's helpers.
        let superblock = pack::PackFile::open_superblock(gen1_bytes.as_slice(), &pack::PackLimits::default()).await.unwrap();
        let footer = superblock.superblock().footer;
        assert_eq!(footer.required_flags & pack::REQUIRED_FOOTER_CHAIN, pack::REQUIRED_FOOTER_CHAIN);
        assert_eq!(footer.prev_footer_offset, parent_footer_position);
        assert_eq!(footer.file_len, gen1_bytes.len() as u64);

        let direct_footer = pack::read_footer_only(&gen1_bytes.as_slice()).await.unwrap();
        assert_eq!(direct_footer, footer);
    }

    #[semio_framework_async_macros::async_test]
    async fn two_generation_incremental_chain_resolves_inherited_pages() {
        let gen0_pages = vec![page(b"root-page").await, page(b"stable-page").await];
        let mut gen0_descriptor = sample_descriptor(0, None).await;
        gen0_descriptor.new_pages = gen0_pages.iter().map(|p| p.hash).collect();
        gen0_descriptor.roots = vec![gen0_pages[0].hash, gen0_pages[1].hash];
        let gen0_bytes = build_generation(&gen0_descriptor, &gen0_pages, None).await.unwrap();

        let parent_footer_position = gen0_bytes.len() as u64 - pack::FOOTER_SIZE as u64;
        let gen1_pages = vec![page(b"changed-page").await];
        let mut gen1_descriptor = sample_descriptor(1, Some(0)).await;
        gen1_descriptor.new_pages = gen1_pages.iter().map(|p| p.hash).collect();
        gen1_descriptor.roots = vec![gen1_pages[0].hash, gen0_pages[1].hash];
        let gen1_bytes = build_generation(&gen1_descriptor, &gen1_pages, Some(parent_footer_position)).await.unwrap();

        let mut combined = Vec::new();
        combined.extend_from_slice(&gen0_bytes);
        combined.extend_from_slice(&gen1_bytes);

        let latest = open_latest(&combined).await.unwrap();
        assert_eq!(latest.generation().await, 1);
        let parent_offset = latest.parent_footer_offset().await.unwrap();
        assert_eq!(parent_offset, parent_footer_position);

        let ancestor = open_ancestor(&combined, parent_offset).await.unwrap();
        assert_eq!(ancestor.generation().await, 0);
        assert_eq!(ancestor.descriptor, gen0_descriptor);
        assert!(ancestor.parent_footer_offset().await.is_none());

        // ✅️ New-in-gen1 page resolves directly from gen1's own chunk table.
        let changed = read_page(&combined, &latest, gen1_pages[0].hash).await.unwrap();
        assert_eq!(changed, b"changed-page");

        // ✅️ Unchanged page (not listed in gen1) resolves by walking to gen0.
        let inherited = read_page(&combined, &latest, gen0_pages[1].hash).await.unwrap();
        assert_eq!(inherited, b"stable-page");

        // ❌️ A hash present in neither generation reports NotFound, not a panic.
        assert!(matches!(read_page(&combined, &latest, pack::ContentHash([0xEE; 32])).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn open_latest_rejects_truncated_buffer() {
        let pages = vec![page(b"only-page").await];
        let mut descriptor = sample_descriptor(0, None).await;
        descriptor.new_pages = pages.iter().map(|p| p.hash).collect();
        let bytes = build_generation(&descriptor, &pages, None).await.unwrap();
        for len in 0..pack::FOOTER_SIZE {
            assert!(open_latest(&bytes[..len]).await.is_err(), "expected error at truncation length {len}");
        }
        assert!(open_latest(&bytes).await.is_ok());
    }
    //#endregion 🔖️Generation

    //#region 🔖️Manager
    async fn body(head_seq: u64) -> SnapshotBody {
        SnapshotBody { head_seq, commit_seq: head_seq, epoch: 0, chain_hash: [0u8; 32], protocol_version: 1, vcs_head: None, base_pack_hash: None, roots: vec![], created_at_ms: head_seq * 1000 }
    }

    #[semio_framework_async_macros::async_test]
    async fn manager_publishes_full_baseline_then_loads_it_back() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let manager = SnapshotManager::new(&storage).await;
        let document: ArtifactId = "doc-a".into();
        let pages = vec![page(b"p0").await];

        let generation = db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &pages, body(10).await)).unwrap();
        assert_eq!(generation, 0);

        let (loaded_generation, descriptor) = db_actor::block_on(manager.load_latest(&document)).unwrap().unwrap();
        assert_eq!(loaded_generation, 0);
        assert_eq!(descriptor.parent_generation, None);
        assert_eq!(descriptor.head_seq, 10);
    }

    #[semio_framework_async_macros::async_test]
    async fn manager_incremental_publish_without_prior_generation_errors() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let manager = SnapshotManager::new(&storage).await;
        let document: ArtifactId = "doc-b".into();
        let result = db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &[], body(0).await));
        assert!(matches!(result, Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn manager_incremental_chain_materializes_and_resolves_inherited_pages() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let manager = SnapshotManager::new(&storage).await;
        let document: ArtifactId = "doc-c".into();

        let gen0_pages = vec![page(b"base-a").await, page(b"base-b").await];
        db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &gen0_pages, body(0).await)).unwrap();

        let gen1_pages = vec![page(b"delta-a").await];
        let gen1 = db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &gen1_pages, body(5).await)).unwrap();
        assert_eq!(gen1, 1);

        let (latest_generation, descriptor) = db_actor::block_on(manager.load_latest(&document)).unwrap().unwrap();
        assert_eq!(latest_generation, 1);
        assert_eq!(descriptor.parent_generation, Some(0));

        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let control = SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 8_192).unwrap();
        let mut cursor = manager.chain_cursor(&document, 1, control);
        let mut inherited = cursor.read_page(gen0_pages[1].hash).await.unwrap();
        assert_eq!(inherited, b"base-b");
        while inherited.close_step().unwrap().is_some() {}
        let mut local = cursor.read_page(gen1_pages[0].hash).await.unwrap();
        assert_eq!(local, b"delta-a");
        while local.close_step().unwrap().is_some() {}
        while cursor.close_step().unwrap() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn manager_retain_from_requires_full_baseline_floor() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let manager = SnapshotManager::new(&storage).await;
        let document: ArtifactId = "doc-d".into();
        db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &[], body(0).await)).unwrap();
        db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &[], body(1).await)).unwrap();

        assert!(matches!(db_actor::block_on(manager.retain_from(&document, 1)), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn manager_retain_from_deletes_generations_below_a_valid_baseline_floor() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let manager = SnapshotManager::new(&storage).await;
        let document: ArtifactId = "doc-e".into();
        db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &[], body(0).await)).unwrap();
        db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &[], body(1).await)).unwrap();
        db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &[], body(2).await)).unwrap();

        db_actor::block_on(manager.retain_from(&document, 2)).unwrap();
        assert_eq!(db_actor::block_on(storage.list_generations(&document)).unwrap(), vec![2]);
    }

    #[semio_framework_async_macros::async_test]
    async fn manager_select_generation_picks_highest_head_seq_at_most_target() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let manager = SnapshotManager::new(&storage).await;
        let document: ArtifactId = "doc-f".into();
        db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &[], body(0).await)).unwrap();
        db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &[], body(10).await)).unwrap();
        db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &[], body(20).await)).unwrap();

        assert_eq!(db_actor::block_on(manager.select_generation(&document, 15)).unwrap(), Some(1));
        assert_eq!(db_actor::block_on(manager.select_generation(&document, 25)).unwrap(), Some(2));
        assert_eq!(db_actor::block_on(manager.select_generation(&document, 5)).unwrap(), Some(0));

        let empty_document: ArtifactId = "doc-empty".into();
        assert_eq!(db_actor::block_on(manager.select_generation(&empty_document, 100)).unwrap(), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn manager_verify_accepts_intact_and_rejects_corrupted_generation() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let manager = SnapshotManager::new(&storage).await;
        let document: ArtifactId = "doc-g".into();
        let pages = vec![page(b"verify-me").await];
        db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &pages, body(0).await)).unwrap();

        db_actor::block_on(manager.verify(&document, 0, pack::os_pack::VerificationLevel::Full)).unwrap();

        let mut retained = db_actor::block_on(storage.read_generation(&document, 0)).unwrap();
        let mut prepared = db_storage::db_io_prepare_platform(&retained).unwrap().await.unwrap();
        let mut corrupted = prepared.as_slice().to_vec();
        while prepared.close_step().unwrap() {}
        while retained.close_step().unwrap().is_some() {}
        let last = corrupted.len() - 1;
        corrupted[last] ^= 0xFF;
        db_actor::block_on(storage.write_generation(&document, 0, pages(&corrupted))).unwrap();
        assert!(db_actor::block_on(manager.verify(&document, 0, pack::os_pack::VerificationLevel::Standard)).is_err());
    }
    //#endregion 🔖️Manager

    //#region 🔖️Policy
    #[semio_framework_async_macros::async_test]
    async fn snapshot_policy_fires_on_any_threshold_alone() {
        let policy = SnapshotPolicy { max_ops_since_last: 100, max_bytes_since_last: 1_000_000, max_ms_since_last: 60_000 };
        assert!(!policy.should_snapshot(10, 10, 10).await);
        assert!(policy.should_snapshot(100, 0, 0).await);
        assert!(policy.should_snapshot(0, 1_000_000, 0).await);
        assert!(policy.should_snapshot(0, 0, 60_000).await);
    }
    //#endregion 🔖️Policy

    //#region 🔖️Lease
    #[semio_framework_async_macros::async_test]
    async fn snapshot_lease_round_trips_acquire_renew_release_via_memory_storage() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();

        let fence = db_actor::block_on(SnapshotLease::acquire(&storage, &document, "actor-a", 1_000, 0)).unwrap();
        assert_eq!(fence, EpochFence::INITIAL);
        assert!(db_actor::block_on(SnapshotLease::current(&storage, &document, 0)).unwrap().is_some());

        db_actor::block_on(SnapshotLease::renew(&storage, &document, "actor-a", fence, 1_000, 500)).unwrap();

        // ❌️ A second holder can't acquire while actor-a's lease is still unexpired.
        assert!(matches!(db_actor::block_on(SnapshotLease::acquire(&storage, &document, "actor-b", 1_000, 500)), Err(DbError::Conflict(_))));

        db_actor::block_on(SnapshotLease::release(&storage, &document, "actor-a", fence)).unwrap();
        assert!(db_actor::block_on(SnapshotLease::current(&storage, &document, 500)).unwrap().is_none());

        // ✅️ Now free, actor-b can acquire fresh (an explicit release clears the record entirely,
        // per `LeaseStorage::acquire`'s own doc — only a hand-off from an EXPIRED-but-still-present
        // lease bumps the epoch fence, exercised next).
        let after_release = db_actor::block_on(SnapshotLease::acquire(&storage, &document, "actor-b", 1_000, 500)).unwrap();
        assert_eq!(after_release, EpochFence::INITIAL);

        // ✅️ A hand-off from an unreleased but time-expired lease DOES bump the fence.
        let expired_handoff = db_actor::block_on(SnapshotLease::acquire(&storage, &document, "actor-c", 1_000, 1_600)).unwrap();
        assert_eq!(expired_handoff, after_release.next());
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_lease_resource_name_is_scoped_per_document() {
        let a: ArtifactId = "doc-a".into();
        let b: ArtifactId = "doc-b".into();
        assert_ne!(SnapshotLease::resource(&a), SnapshotLease::resource(&b));
    }
    //#endregion 🔖️Lease
}
//#endregion 🧪️Tests
//#region 🧪️RetainedTests
#[cfg(test)]
mod retained_tests {
    use super::*;
    use db_storage::MemoryStorage;

    #[semio_framework_async_macros::async_test]
    async fn snapshot_cursor_cancel_fuel_interrupted_close_and_terminal_empty_are_exact() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let manager = SnapshotManager::new(&storage).await;
        let document = ArtifactId::from("retained-snapshot");
        let body = SnapshotBody { head_seq: 0, commit_seq: 0, epoch: 0, chain_hash: [0; 32], protocol_version: 1, vcs_head: None, base_pack_hash: None, roots: Vec::new(), created_at_ms: 0 };
        manager.publish(&document, SnapshotOrigin::FullBaseline, &[], body).await.unwrap();

        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let control = SnapshotCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
        let mut cursor = manager.chain_cursor(&document, 0, control);
        assert!(cursor.latest_descriptor().await.is_err());
        cursor.control.replenish(64, std::time::Instant::now() + std::time::Duration::from_secs(30)).unwrap();
        cancelled.store(true, std::sync::atomic::Ordering::Release);
        assert!(matches!(cursor.latest_descriptor().await, Err(DbError::Unavailable(_))));
        cancelled.store(false, std::sync::atomic::Ordering::Release);
        assert!(cursor.close_step().unwrap());
        assert!(cursor.terminal_is_empty());
        assert!(!cursor.close_step().unwrap());
    }
}
//#endregion 🧪️RetainedTests
