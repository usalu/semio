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
//! generation order, into one byte buffer (`materialize_chain` does exactly this), then
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

    /// @emoji 📖️ Inverse of `encode`. Never panics on malformed input — every field read is
    /// bounds-checked by `pack::ByteReader` and every count is checked against
    /// `MAX_HASH_LIST_LEN`/`MAX_STRING_BYTES` before the corresponding `Vec`/`String` is allocated.
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

async fn write_string(w: &mut pack::ByteWriter, s: &str) {
    w.write_varint_u64(s.len() as u64);
    w.write_bytes(s.as_bytes());
}

async fn read_string(r: &mut pack::ByteReader<'_>) -> Result<String, DbError> {
    let len = r.read_varint_u64()?;
    check_len(len, MAX_STRING_BYTES, "snapshot descriptor string")?;
    let bytes = r.read_bytes(len as usize)?;
    String::from_utf8(bytes.to_vec()).map_err(|_| DbError::Corrupt("invalid utf8 in snapshot descriptor".to_string()))
}

async fn write_option_u64(w: &mut pack::ByteWriter, value: Option<u64>) {
    match value {
        Some(v) => {
            w.write_u8(1);
            w.write_varint_u64(v);
        }
        None => w.write_u8(0),
    }
}

async fn read_option_u64(r: &mut pack::ByteReader<'_>) -> Result<Option<u64>, DbError> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(r.read_varint_u64()?)),
        other => Err(DbError::Corrupt(format!("bad option tag {other}"))),
    }
}

async fn write_hash_list(w: &mut pack::ByteWriter, hashes: &[ContentHash]) {
    w.write_varint_u64(hashes.len() as u64);
    for hash in hashes {
        w.write_bytes(&hash.0);
    }
}

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
struct SubSource<'a> {
    inner: &'a [u8],
    base: u64,
    len: u64,
}

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

/// @emoji 🔎️ Reads the `KIND_SNAPSHOT` segment this crate always writes as the very first segment
/// (fixed local offset `pack::HEADER_SIZE`, right after the header) of one generation's own
/// coordinate space. Only understands identity-codec framing (`flags == 0`) since this crate never
/// compresses that segment; CRC-validates before returning the payload.
async fn decode_snapshot_segment(source: &SubSource<'_>) -> Result<Vec<u8>, DbError> {
    use pack::PackSource as _;
    let offset = pack::HEADER_SIZE as u64;
    const PREFIX_CAP: usize = 16;
    let mut prefix = [0u8; PREFIX_CAP];
    let read = source.read_at(offset, &mut prefix).await?;
    if read < 2 {
        return Err(DbError::Corrupt("truncated snapshot descriptor segment header".to_string()));
    }
    let kind = prefix[0];
    let flags = prefix[1];
    if flags != 0 {
        return Err(DbError::Corrupt("snapshot descriptor segment must use the identity codec".to_string()));
    }
    let mut pos = 2usize;
    let seg_len = pack::os_pack::read_varint_u64(&prefix[..read], &mut pos)?;
    check_len(seg_len, 64 * 1024 * 1024, "snapshot descriptor segment length")?;
    let header_len = pos as u64;
    let mut frame = vec![0u8; (header_len + seg_len) as usize];
    source.read_exact_at(offset, &mut frame).await?;
    let mut crc_bytes = [0u8; 4];
    source.read_exact_at(offset + header_len + seg_len, &mut crc_bytes).await?;
    let stored_crc = u32::from_le_bytes(crc_bytes);
    let computed_crc = pack::crc32c(&frame);
    if stored_crc != computed_crc {
        return Err(DbError::Corrupt("snapshot descriptor segment checksum mismatch".to_string()));
    }
    if kind != pack::KIND_SNAPSHOT {
        return Err(DbError::Corrupt(format!("expected KIND_SNAPSHOT segment (0x{:02x}), found 0x{kind:02x}", pack::KIND_SNAPSHOT)));
    }
    Ok(frame[header_len as usize..].to_vec())
}
//#endregion 🔖️SegmentIo

//#region 🔖️Generation
/// @emoji 🏗️ Builds one generation's complete, self-contained `.spk` pack bytes: the descriptor
/// as the first (`KIND_SNAPSHOT`) segment, `new_pages` as `KIND_CHUNK` segments in order (so
/// `descriptor.new_pages[i]` is `pack::ChunkId(i)`), then the standard `pack::PackWriter::finish`
/// trailer. If `parent_footer_position` is `Some`, the trailing footer's `prev_footer_offset` is
/// patched in place afterward (see module doc) and `REQUIRED_FOOTER_CHAIN` is set in both the
/// header and the footer's `required_flags`.
pub async fn build_generation(descriptor: &SnapshotDescriptor, new_pages: &[Page], parent_footer_position: Option<u64>) -> Result<Vec<u8>, DbError> {
    if new_pages.len() != descriptor.new_pages.len() {
        return Err(DbError::InvalidArgument("new_pages count does not match descriptor.new_pages".to_string()));
    }
    for (page, hash) in new_pages.iter().zip(descriptor.new_pages.iter()) {
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
    let mut writer = pack::PackWriter::begin(Vec::<u8>::new(), &options).await?;

    let descriptor_bytes = descriptor.encode().await;
    writer.write_segment(pack::KIND_SNAPSHOT, &descriptor_bytes).await?;
    for page in new_pages {
        writer.write_chunk(&page.bytes).await?;
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
    let mut bytes = writer.finish(&manifest).await?;

    if let Some(parent_offset) = parent_footer_position {
        patch_prev_footer_offset(&mut bytes, parent_offset).await?;
    }
    Ok(bytes)
}

/// @emoji 🩹️ `pack::PackWriter::finish` always writes `prev_footer_offset = 0` (there is no public
/// constructor knob for it — see module doc's design-choice note). This patches the trailing
/// 84-byte footer's `prev_footer_offset` field (wire bytes `[72..80]`, pinned by
/// `pack_format::Footer`'s own layout, verified byte-for-byte against `pack`'s own tests) in
/// place, then recomputes the footer's CRC-32C (over the preceding 80 bytes) so the result still
/// parses cleanly through `pack::read_footer_only`/`PackFile::open_superblock`.
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

async fn open_generation_at(combined: &[u8], base: u64, len: u64, footer: &pack::Footer) -> Result<GenerationHandle, DbError> {
    let sub = SubSource { inner: combined, base, len };
    let descriptor_bytes = decode_snapshot_segment(&sub).await?;
    let descriptor = SnapshotDescriptor::decode(&descriptor_bytes).await?;
    Ok(GenerationHandle { descriptor, base, len, footer_required_flags: footer.required_flags, footer_prev_offset: footer.prev_footer_offset })
}

/// @emoji 🔚️ Opens the LAST generation physically present in `combined` (the one whose footer sits
/// at `combined.len() - FOOTER_SIZE`) — the entry point for reading a freshly-fetched or
/// freshly-`materialize_chain`d buffer.
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

impl<'storage, S: SnapshotStorage> SnapshotManager<'storage, S> {
    pub async fn new(storage: &'storage S) -> SnapshotManager<'storage, S> {
        SnapshotManager { storage }
    }

    /// @emoji ✍️ Builds and durably writes the next generation. `origin == Incremental` chains to
    /// the document's current `latest_generation` (`DbError::InvalidArgument` if there is none
    /// yet); `origin == FullBaseline` starts (or restarts) a self-sufficient lineage.
    pub async fn publish(&self, document: &ArtifactId, origin: SnapshotOrigin, new_pages: &[Page], body: SnapshotBody) -> Result<u64, DbError> {
        let latest = self.storage.latest_generation(document).await?;
        let (generation, parent_generation, parent_footer_position) = match origin {
            SnapshotOrigin::FullBaseline => (latest.map_or(0, |g| g + 1), None, None),
            SnapshotOrigin::Incremental => {
                let parent_generation = latest.ok_or_else(|| DbError::InvalidArgument("cannot publish an incremental snapshot with no prior generation".to_string()))?;
                let combined = self.materialize_chain(document, parent_generation).await?;
                let parent_footer_position = combined.len() as u64 - pack::FOOTER_SIZE as u64;
                (parent_generation + 1, Some(parent_generation), Some(parent_footer_position))
            }
        };

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
            new_pages: new_pages.iter().map(|page| page.hash).collect(),
            created_at_ms: body.created_at_ms,
        };
        let bytes = build_generation(&descriptor, new_pages, parent_footer_position).await?;
        let pages = db_storage::db_io_copy_pages(&bytes)?.await?;
        self.storage.write_generation(document, generation, pages).await?;
        Ok(generation)
    }

    /// @emoji 🔗️ Fetches every generation from the chain's root through `through_generation`
    /// (walking `SnapshotDescriptor::parent_generation` backward, then concatenating forward) into
    /// one buffer — the "virtual combined space" `Footer.prev_footer_offset` addresses (see module
    /// doc). Used internally by `publish` and exposed for callers (`db_cli`, `db_compact`) that
    /// need the full lineage as one archive.
    pub async fn materialize_chain(&self, document: &ArtifactId, through_generation: u64) -> Result<Vec<u8>, DbError> {
        let mut chain: Vec<db_storage::DbIoPages> = Vec::new();
        let mut current = Some(through_generation);
        while let Some(generation) = current {
            let bytes = self.storage.read_generation(document, generation).await?;
            let prepared = db_storage::db_io_prepare_platform(&bytes)?.await?;
            let handle = open_latest(prepared.as_slice()).await?;
            current = handle.descriptor.parent_generation;
            chain.push(bytes);
        }
        chain.reverse();
        let total: usize = chain.iter().map(db_storage::DbIoPages::len).sum();
        let mut combined = Vec::with_capacity(total);
        for bytes in chain {
            for fragment in bytes.fragments() {
                combined.extend_from_slice(fragment);
            }
        }
        Ok(combined)
    }

    /// @emoji 🥇️ The document's latest generation number and descriptor, or `None` if it has no
    /// snapshot yet.
    pub async fn load_latest(&self, document: &ArtifactId) -> Result<Option<(u64, SnapshotDescriptor)>, DbError> {
        match self.storage.latest_generation(document).await? {
            None => Ok(None),
            Some(generation) => {
                let bytes = self.storage.read_generation(document, generation).await?;
                let prepared = db_storage::db_io_prepare_platform(&bytes)?.await?;
                let handle = open_latest(prepared.as_slice()).await?;
                Ok(Some((generation, handle.descriptor)))
            }
        }
    }

    /// @emoji 🎯️ Selection: the highest-numbered generation whose `head_seq` does not exceed
    /// `at_most_head_seq`, or `None` if no generation qualifies — the snapshot a materializer
    /// should start replaying the WAL suffix from for a point-in-time read.
    pub async fn select_generation(&self, document: &ArtifactId, at_most_head_seq: u64) -> Result<Option<u64>, DbError> {
        let mut best: Option<(u64, u64)> = None;
        for generation in self.storage.list_generations(document).await? {
            let bytes = self.storage.read_generation(document, generation).await?;
            let prepared = db_storage::db_io_prepare_platform(&bytes)?.await?;
            let handle = open_latest(prepared.as_slice()).await?;
            if handle.descriptor.head_seq <= at_most_head_seq {
                let better = best.is_none_or(|(_, best_head_seq)| handle.descriptor.head_seq > best_head_seq);
                if better {
                    best = Some((generation, handle.descriptor.head_seq));
                }
            }
        }
        Ok(best.map(|(generation, _)| generation))
    }

    /// @emoji 🗑️ Deletes every generation strictly below `floor_generation`. `floor_generation`
    /// must itself be a full baseline (`parent_generation.is_none()`) — see module doc's scope
    /// boundary on why an incremental floor is rejected rather than silently breaking its chain.
    pub async fn retain_from(&self, document: &ArtifactId, floor_generation: u64) -> Result<(), DbError> {
        let floor_bytes = self.storage.read_generation(document, floor_generation).await?;
        let floor_prepared = db_storage::db_io_prepare_platform(&floor_bytes)?.await?;
        let floor_handle = open_latest(floor_prepared.as_slice()).await?;
        if floor_handle.descriptor.parent_generation.is_some() {
            return Err(DbError::InvalidArgument("retention floor must be a full-baseline generation (no parent)".to_string()));
        }
        for generation in self.storage.list_generations(document).await? {
            if generation < floor_generation {
                self.storage.delete_generation(document, generation).await?;
            }
        }
        Ok(())
    }

    /// @emoji 🔬️ Verifies generation `generation` decodes cleanly at `level`: the `KIND_SNAPSHOT`
    /// descriptor round-trips (`open_latest` itself decodes it) and every declared local chunk
    /// (`descriptor.new_pages`) decodes — at `VerificationLevel::Full` this transitively checks
    /// each chunk's content hash too, since `pack::PackFile::read_chunk` already validates it
    /// against the chunk table `pack::PackWriter::write_chunk` built from these same page bytes;
    /// no separate hash recomputation needed here.
    pub async fn verify(&self, document: &ArtifactId, generation: u64, level: pack::os_pack::VerificationLevel) -> Result<(), DbError> {
        let bytes = self.storage.read_generation(document, generation).await?;
        let prepared = db_storage::db_io_prepare_platform(&bytes)?.await?;
        let handle = open_latest(prepared.as_slice()).await?;
        let sub = SubSource { inner: prepared.as_slice(), base: 0, len: bytes.len() as u64 };
        let file = pack::PackFile::open_manifest(sub, &pack::PackLimits::default(), level).await?;
        for index in 0..handle.descriptor.new_pages.len() {
            file.read_chunk(pack::ChunkId(index as u32), level).await?;
        }
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
    /// `materialize_chain` + `publish` sequence for a deep incremental chain.
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
        Page::new(bytes.to_vec())
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
        let storage = MemoryStorage::new().await;
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
        let storage = MemoryStorage::new().await;
        let manager = SnapshotManager::new(&storage).await;
        let document: ArtifactId = "doc-b".into();
        let result = db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &[], body(0).await));
        assert!(matches!(result, Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn manager_incremental_chain_materializes_and_resolves_inherited_pages() {
        let storage = MemoryStorage::new().await;
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

        let combined = db_actor::block_on(manager.materialize_chain(&document, 1)).unwrap();
        let handle = open_latest(&combined).await.unwrap();
        let inherited = read_page(&combined, &handle, gen0_pages[1].hash).await.unwrap();
        assert_eq!(inherited, b"base-b");
        let local = read_page(&combined, &handle, gen1_pages[0].hash).await.unwrap();
        assert_eq!(local, b"delta-a");
    }

    #[semio_framework_async_macros::async_test]
    async fn manager_retain_from_requires_full_baseline_floor() {
        let storage = MemoryStorage::new().await;
        let manager = SnapshotManager::new(&storage).await;
        let document: ArtifactId = "doc-d".into();
        db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &[], body(0).await)).unwrap();
        db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &[], body(1).await)).unwrap();

        assert!(matches!(db_actor::block_on(manager.retain_from(&document, 1)), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn manager_retain_from_deletes_generations_below_a_valid_baseline_floor() {
        let storage = MemoryStorage::new().await;
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
        let storage = MemoryStorage::new().await;
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
        let storage = MemoryStorage::new().await;
        let manager = SnapshotManager::new(&storage).await;
        let document: ArtifactId = "doc-g".into();
        let pages = vec![page(b"verify-me").await];
        db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &pages, body(0).await)).unwrap();

        db_actor::block_on(manager.verify(&document, 0, pack::os_pack::VerificationLevel::Full)).unwrap();

        let mut corrupted = db_actor::block_on(storage.read_generation(&document, 0)).unwrap();
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
        let storage = MemoryStorage::new().await;
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
