//! 📦️ `pack_format` — the `SPK` binary document container: exact byte-level header/footer/
//! segment/manifest/symbols/chunk-table layout, a `PackWriter`/`PackFile` pair for building and
//! random-access reading packs of any size, three verification levels trading speed for
//! integrity guarantees, forward-scan recovery for footer-less/truncated files, and an optional
//! deflate codec. Every length is validated against `crate::PackLimits` before allocation.
//!
//! Layout notes: see the `## pack_format` section of the wave-0 contract at
//! `.🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/contract.md`. One deliberate
//! deviation is documented in `🔖️Footer` below: the contract's prose arithmetic for footer size
//! (`"= 80 bytes exactly"`) undercounts its own `footer_crc32` field by 4 bytes; this crate
//! implements the mathematically self-consistent 84-byte footer and exports `FOOTER_SIZE` so
//! downstream crates never have to hardcode the number themselves.

use crate::{crc32c, read_varint_u64, write_varint_u64, ByteRange, ChunkId, CodecId, CompressionCodec, ContentHash, NoCompression, PackError, PackLimits, PackSink, PackSource};

//#region 🔖️Header
/// @emoji 🧲️ The 8-byte magic every `.spk` pack file begins with.
pub const MAGIC: [u8; 8] = [0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A];
/// @emoji 📏️ Fixed wire size of the header, in bytes.
pub const HEADER_SIZE: usize = 32;
/// @emoji 🔢️ The container format version this crate writes and reads.
pub const FORMAT_VERSION_MAJOR: u16 = 1;
/// @emoji 🔢️ The container format minor version this crate writes.
pub const FORMAT_VERSION_MINOR: u16 = 0;

/// @emoji 🗜️ Required flag bit: at least one segment/chunk in this file uses compression.
pub const REQUIRED_COMPRESSED: u32 = 1 << 0;
/// @emoji 🧱️ Required flag bit: this file contains a chunk table.
pub const REQUIRED_CHUNKED: u32 = 1 << 1;
/// @emoji 🔒️ Required flag bit: reserved for encryption, never set by this crate.
pub const REQUIRED_ENCRYPTED: u32 = 1 << 2;
/// @emoji ⛓️ Required flag bit: reserved for footer chaining.
pub const REQUIRED_FOOTER_CHAIN: u32 = 1 << 3;
const REQUIRED_KNOWN_MASK: u32 = REQUIRED_COMPRESSED | REQUIRED_CHUNKED | REQUIRED_ENCRYPTED | REQUIRED_FOOTER_CHAIN;

/// @emoji 🧮️ Optional flag bit: the document body was encoded in canonical form.
pub const OPTIONAL_CANONICAL: u32 = 1 << 0;
/// @emoji 🌊️ Optional flag bit: the file was produced by a streaming writer.
pub const OPTIONAL_STREAMED: u32 = 1 << 1;
/// @emoji 🧬️ Optional flag bit: a schema segment is present.
pub const OPTIONAL_HAS_SCHEMA: u32 = 1 << 2;

/// @emoji 🪪️ The fixed 32-byte superblock header: magic, version, feature flags, self-CRC.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    pub version_major: u16,
    pub version_minor: u16,
    pub required_flags: u32,
    pub optional_flags: u32,
}

impl Header {
    /// @emoji ✍️ Serializes to the exact 32-byte wire form, computing `header_crc32` over
    /// bytes `0..20` and zeroing the 8 reserved bytes.
    // 🧮️ Every helper this fn calls (`crc32c`, `PackSink::write_all`) is a first-party `async fn`
    // one hop away in `semio-framework-replication`, which this packet does not own; R9 rule 3
    // ("if every consumer can become async, make it async instead") applies, not R9 rule 2 — see
    // 📓️terra-pack-finish-report.md §"pure-computation-made-async: the recipe".
    async fn write_bytes(&self) -> [u8; HEADER_SIZE] {
        let mut buf = [0u8; HEADER_SIZE];
        buf[0..8].copy_from_slice(&MAGIC);
        buf[8..10].copy_from_slice(&self.version_major.to_le_bytes());
        buf[10..12].copy_from_slice(&self.version_minor.to_le_bytes());
        buf[12..16].copy_from_slice(&self.required_flags.to_le_bytes());
        buf[16..20].copy_from_slice(&self.optional_flags.to_le_bytes());
        let crc = crc32c(&buf[0..20]);
        buf[20..24].copy_from_slice(&crc.await.to_le_bytes());
        buf
    }

    /// @emoji 📖️ Parses and validates a 32-byte header: magic, self-CRC, and that
    /// `required_flags` sets no bit outside the known `0..=3` range.
    async fn parse(bytes: &[u8]) -> Result<Self, PackError> {
        if bytes.len() < HEADER_SIZE {
            return Err(PackError::Truncated(bytes.len() as u64));
        }
        if bytes[0..8] != MAGIC {
            return Err(PackError::BadMagic);
        }
        let version_major = u16::from_le_bytes([bytes[8], bytes[9]]);
        let version_minor = u16::from_le_bytes([bytes[10], bytes[11]]);
        let required_flags = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let optional_flags = u32::from_le_bytes(bytes[16..20].try_into().unwrap());
        let stored_crc = u32::from_le_bytes(bytes[20..24].try_into().unwrap());
        let computed_crc = crc32c(&bytes[0..20]).await;
        if stored_crc != computed_crc {
            return Err(PackError::ChecksumMismatch { segment: "header", offset: 20 });
        }
        let unknown = required_flags & !REQUIRED_KNOWN_MASK;
        if unknown != 0 {
            return Err(PackError::UnknownRequiredFlags(unknown));
        }
        if version_major != FORMAT_VERSION_MAJOR {
            return Err(PackError::UnsupportedVersion { major: version_major, minor: version_minor });
        }
        Ok(Self { version_major, version_minor, required_flags, optional_flags })
    }
}
//#endregion 🔖️Header

//#region 🔖️Footer
/// @emoji 🧲️ The 8-byte magic the footer begins with.
pub const FOOTER_MAGIC: [u8; 8] = *b"SPKFOOT1";
/// @emoji 📏️ Fixed wire size of the footer, in bytes, at the end of the file.
/// See the module doc for why this is 84, not the contract prose's arithmetically-inconsistent
/// "80" (the prose sum omits the trailing `footer_crc32` field's own 4 bytes).
pub const FOOTER_SIZE: usize = 84;

/// @emoji 🪶️ The fixed-size trailer every pack file ends with — the single root of trust a
/// reader locates by seeking to `file_len - FOOTER_SIZE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Footer {
    pub version_major: u16,
    pub version_minor: u16,
    pub required_flags: u32,
    pub manifest_offset: u64,
    pub manifest_len: u64,
    pub file_len: u64,
    pub content_hash: ContentHash,
    pub prev_footer_offset: u64,
}

impl Footer {
    /// @emoji ✍️ Serializes to the exact 84-byte wire form, computing `footer_crc32` over the
    /// preceding 80 bytes.
    // 🧮️ Same R9-rule-3 reasoning as `Header::write_bytes` above — `crc32c` is a pure but
    // externally-owned `async fn`, so this fn propagates `async` rather than fighting it.
    async fn write_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(FOOTER_SIZE);
        buf.extend_from_slice(&FOOTER_MAGIC);
        buf.extend_from_slice(&self.version_major.to_le_bytes());
        buf.extend_from_slice(&self.version_minor.to_le_bytes());
        buf.extend_from_slice(&self.required_flags.to_le_bytes());
        buf.extend_from_slice(&self.manifest_offset.to_le_bytes());
        buf.extend_from_slice(&self.manifest_len.to_le_bytes());
        buf.extend_from_slice(&self.file_len.to_le_bytes());
        buf.extend_from_slice(&self.content_hash.0);
        buf.extend_from_slice(&self.prev_footer_offset.to_le_bytes());
        let crc = crc32c(&buf);
        buf.extend_from_slice(&crc.await.to_le_bytes());
        buf
    }

    /// @emoji 📖️ Parses and validates an 84-byte footer: magic and self-CRC over the first 80
    /// bytes. Does not cross-check `file_len` against an actual source — callers that have one
    /// should do so themselves (see `PackFile::open_superblock`).
    async fn parse(bytes: &[u8]) -> Result<Self, PackError> {
        if bytes.len() < FOOTER_SIZE {
            return Err(PackError::Truncated(bytes.len() as u64));
        }
        if bytes[0..8] != FOOTER_MAGIC {
            return Err(PackError::BadMagic);
        }
        let version_major = u16::from_le_bytes(bytes[8..10].try_into().unwrap());
        let version_minor = u16::from_le_bytes(bytes[10..12].try_into().unwrap());
        let required_flags = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let manifest_offset = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
        let manifest_len = u64::from_le_bytes(bytes[24..32].try_into().unwrap());
        let file_len = u64::from_le_bytes(bytes[32..40].try_into().unwrap());
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&bytes[40..72]);
        let prev_footer_offset = u64::from_le_bytes(bytes[72..80].try_into().unwrap());
        let stored_crc = u32::from_le_bytes(bytes[80..84].try_into().unwrap());
        let computed_crc = crc32c(&bytes[0..80]).await;
        if stored_crc != computed_crc {
            return Err(PackError::ChecksumMismatch { segment: "footer", offset: 80 });
        }
        Ok(Self { version_major, version_minor, required_flags, manifest_offset, manifest_len, file_len, content_hash: ContentHash(hash), prev_footer_offset })
    }
}
//#endregion 🔖️Footer

//#region 🔖️Segment
/// @emoji 📦️ The fully-encoded wire bytes of one framed segment, plus the byte offsets within
/// them a writer needs to record chunk-table/manifest-span metadata without re-parsing.
struct EncodedSegment {
    bytes: Vec<u8>,
    header_len: usize,
    stored_len: usize,
}

/// @emoji 🧵️ Resolves `CodecId` to this crate's codec implementations for compression.
async fn codec_compress(codec: CodecId, raw: &[u8]) -> Result<Vec<u8>, PackError> {
    match codec.0 {
        0 => Ok(raw.to_vec()),
        1 => crate::codec::deflate_compress(raw).await,
        other => Err(PackError::UnsupportedCodec(other)),
    }
}

/// @emoji 🧵️ Resolves `CodecId` to this crate's codec implementations for decompression.
async fn codec_decompress(codec: CodecId, stored: &[u8], raw_len: u64, limit: u64) -> Result<Vec<u8>, PackError> {
    match codec.0 {
        0 => NoCompression.decompress(stored, raw_len, limit).await,
        1 => crate::codec::deflate_decompress(stored, raw_len, limit).await,
        other => Err(PackError::UnsupportedCodec(other)),
    }
}

/// @emoji 🖇️ Frames `payload` as a segment per the contract's byte layout: `kind, flags,
/// seg_len, [raw_len], payload, crc32`. Compresses first when `codec` is non-identity.
async fn encode_segment(kind: u8, codec: CodecId, payload: &[u8]) -> Result<EncodedSegment, PackError> {
    let compressed = codec.0 != 0;
    let stored = if compressed { codec_compress(codec, payload).await? } else { payload.to_vec() };
    let flags: u8 = if compressed { 1 | (codec.0 << 1) } else { 0 };
    let mut buf = Vec::with_capacity(stored.len() + 24);
    buf.push(kind);
    buf.push(flags);
    write_varint_u64(&mut buf, stored.len() as u64).await;
    if compressed {
        write_varint_u64(&mut buf, payload.len() as u64).await;
    }
    let header_len = buf.len();
    buf.extend_from_slice(&stored);
    let crc = crc32c(&buf);
    buf.extend_from_slice(&crc.await.to_le_bytes());
    Ok(EncodedSegment { bytes: buf, header_len, stored_len: stored.len() })
}

/// @emoji 👓️ A decoded, CRC-checked, decompressed segment plus enough position bookkeeping for
/// callers walking a sequence of segments.
struct DecodedSegment {
    kind: u8,
    payload: Vec<u8>,
    /// @emoji ➡️ Total wire bytes consumed by this segment (frame header + stored payload + crc).
    consumed: u64,
}

/// @emoji 1⃣ Bounds-checked single-byte read at an absolute file offset.
async fn read_u8_at<S: PackSource>(source: &S, offset: u64) -> Result<u8, PackError> {
    let mut buf = [0u8; 1];
    source.read_exact_at(offset, &mut buf).await?;
    Ok(buf[0])
}

/// @emoji 🔢️ Reads one LEB128 varint starting at an absolute file offset, one byte at a time so
/// it never over-reads past a legitimately short remaining file. Returns `(value, bytes_consumed)`.
async fn read_varint_u64_at<S: PackSource>(source: &S, offset: u64) -> Result<(u64, u64), PackError> {
    let mut tmp: Vec<u8> = Vec::with_capacity(10);
    let mut i = 0u64;
    loop {
        let byte = read_u8_at(source, offset + i).await?;
        tmp.push(byte);
        i += 1;
        if byte & 0x80 == 0 {
            break;
        }
        if i >= 10 {
            return Err(PackError::Malformed { what: "varint", offset, detail: "overlong varint".to_string() });
        }
    }
    let mut pos = 0usize;
    let value = read_varint_u64(&tmp, &mut pos).await?;
    Ok((value, i))
}

/// @emoji 🚪️ Decodes one framed segment starting at an absolute file offset: reads and
/// bounds-checks `kind, flags, seg_len, [raw_len]`, validates lengths against
/// `limits.max_segment_len` **before** allocating the payload buffer, then optionally verifies
/// the frame's CRC-32C and decompresses. Unknown `kind` values are decoded and returned as-is —
/// callers that only want known kinds are responsible for skipping/rejecting them, this function
/// never errors on an unrecognized kind.
async fn decode_segment_at<S: PackSource>(source: &S, offset: u64, limits: &PackLimits, verify_crc: bool) -> Result<DecodedSegment, PackError> {
    let total_len = source.len().await;
    if offset >= total_len {
        return Err(PackError::Truncated(offset));
    }
    let kind = read_u8_at(source, offset).await?;
    let flags = read_u8_at(source, offset + 1).await?;
    let compressed = flags & 0x01 != 0;
    let codec = CodecId((flags >> 1) & 0x07);
    let (stored_len, n1) = read_varint_u64_at(source, offset + 2).await?;
    let mut cursor = offset + 2 + n1;
    let raw_len = if compressed {
        let (v, n2) = read_varint_u64_at(source, cursor).await?;
        cursor += n2;
        v
    } else {
        stored_len
    };
    if stored_len > limits.max_segment_len || raw_len > limits.max_segment_len {
        return Err(PackError::LimitExceeded("segment length exceeds max_segment_len"));
    }
    let payload_offset = cursor;
    let payload_end = payload_offset.checked_add(stored_len).ok_or(PackError::LimitExceeded("segment payload offset overflow"))?;
    if payload_end > total_len {
        return Err(PackError::Truncated(payload_offset));
    }
    let header_len = (payload_offset - offset) as usize;
    let mut frame = vec![0u8; header_len + stored_len as usize];
    source.read_exact_at(offset, &mut frame).await?;
    let crc_offset = payload_end;
    if crc_offset + 4 > total_len {
        return Err(PackError::Truncated(crc_offset));
    }
    let mut crc_bytes = [0u8; 4];
    source.read_exact_at(crc_offset, &mut crc_bytes).await?;
    if verify_crc {
        let stored_crc = u32::from_le_bytes(crc_bytes);
        let computed_crc = crc32c(&frame).await;
        if stored_crc != computed_crc {
            return Err(PackError::ChecksumMismatch { segment: "segment", offset: crc_offset });
        }
    }
    let stored_payload = &frame[header_len..];
    let payload = if compressed { codec_decompress(codec, stored_payload, raw_len, limits.max_segment_len).await? } else { stored_payload.to_vec() };
    let consumed = (crc_offset + 4) - offset;
    Ok(DecodedSegment { kind, payload, consumed })
}
//#endregion 🔖️Segment

//#region 🔖️Symbols
/// @emoji ✍️ Serializes a symbol table: `count varint, then count × (len varint, utf8 bytes)`.
/// Exposed so callers (e.g. `pack_value`) can build a `KIND_SYMBOLS` segment payload for
/// `PackWriter::write_segment` without re-implementing this crate's wire format.
pub async fn encode_symbols(symbols: &[String]) -> Vec<u8> {
    let mut buf = Vec::new();
    write_varint_u64(&mut buf, symbols.len() as u64).await;
    for symbol in symbols {
        let bytes = symbol.as_bytes();
        write_varint_u64(&mut buf, bytes.len() as u64).await;
        buf.extend_from_slice(bytes);
    }
    buf
}

/// @emoji 📖️ Parses a symbol table, rejecting a count over `limits.max_symbols` before
/// allocating the output `Vec`.
async fn decode_symbols(payload: &[u8], limits: &PackLimits) -> Result<Vec<String>, PackError> {
    let mut pos = 0usize;
    let count = read_varint_u64(payload, &mut pos).await?;
    if count > limits.max_symbols as u64 {
        return Err(PackError::LimitExceeded("symbol count exceeds max_symbols"));
    }
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let len = read_varint_u64(payload, &mut pos).await? as usize;
        if pos + len > payload.len() {
            return Err(PackError::Truncated(pos as u64));
        }
        let text = std::str::from_utf8(&payload[pos..pos + len]).map_err(|_| PackError::Malformed { what: "symbol", offset: pos as u64, detail: "invalid utf8".to_string() })?.to_string();
        pos += len;
        out.push(text);
    }
    Ok(out)
}
//#endregion 🔖️Symbols

//#region 🔖️ChunkTable
/// @emoji 🧱️ One row of the chunk table: where a chunk's (possibly compressed) payload lives
/// and how to verify it. `crc32` covers the stored (on-disk) bytes; `blake3` covers the raw
/// (decompressed) content — the former is cheap and always checked at `Standard`+, the latter is
/// the content-identity hash only checked at `Full`.
#[derive(Clone, Debug)]
struct ChunkTableEntry {
    offset: u64,
    stored_len: u64,
    raw_len: u64,
    crc32: u32,
    blake3: [u8; 32],
}

/// @emoji ✍️ Serializes the chunk table: `count varint, then count × (offset, stored_len,
/// raw_len varints, crc32 u32 LE, blake3 [u8;32])`.
async fn encode_chunk_table(entries: &[ChunkTableEntry]) -> Vec<u8> {
    let mut buf = Vec::new();
    write_varint_u64(&mut buf, entries.len() as u64).await;
    for entry in entries {
        write_varint_u64(&mut buf, entry.offset).await;
        write_varint_u64(&mut buf, entry.stored_len).await;
        write_varint_u64(&mut buf, entry.raw_len).await;
        buf.extend_from_slice(&entry.crc32.to_le_bytes());
        buf.extend_from_slice(&entry.blake3);
    }
    buf
}

/// @emoji 📖️ Parses the chunk table, rejecting a count over `limits.max_items` or an entry
/// length over `limits.max_segment_len` before allocating.
async fn decode_chunk_table(payload: &[u8], limits: &PackLimits) -> Result<Vec<ChunkTableEntry>, PackError> {
    let mut pos = 0usize;
    let count = read_varint_u64(payload, &mut pos).await?;
    if count > limits.max_items {
        return Err(PackError::LimitExceeded("chunk_table count exceeds max_items"));
    }
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let offset = read_varint_u64(payload, &mut pos).await?;
        let stored_len = read_varint_u64(payload, &mut pos).await?;
        let raw_len = read_varint_u64(payload, &mut pos).await?;
        if stored_len > limits.max_segment_len || raw_len > limits.max_segment_len {
            return Err(PackError::LimitExceeded("chunk table entry length exceeds max_segment_len"));
        }
        if pos + 4 > payload.len() {
            return Err(PackError::Truncated(pos as u64));
        }
        let crc32 = u32::from_le_bytes(payload[pos..pos + 4].try_into().unwrap());
        pos += 4;
        if pos + 32 > payload.len() {
            return Err(PackError::Truncated(pos as u64));
        }
        let mut blake3 = [0u8; 32];
        blake3.copy_from_slice(&payload[pos..pos + 32]);
        pos += 32;
        out.push(ChunkTableEntry { offset, stored_len, raw_len, crc32, blake3 });
    }
    Ok(out)
}
//#endregion 🔖️ChunkTable

//#region 🔖️Manifest
/// @emoji 🗺️ The manifest: spans and counts describing every other segment in the file.
/// `schema_name` round-trips through the symbol table as a symref on the wire (see
/// `PackWriter::finish`/`PackFile::open_manifest`) but is resolved to a plain `String` here for
/// callers' convenience.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Manifest {
    pub schema_name: String,
    pub schema_hash: [u8; 32],
    pub doc_span: ByteRange,
    pub doc_frame_count: u64,
    pub symbols_span: ByteRange,
    pub chunk_table_span: ByteRange,
    pub field_index_span: ByteRange,
    pub uncompressed_body_len: u64,
    pub field_count: u64,
    pub chunk_count: u64,
    pub symbol_count: u64,
}

/// @emoji 🗺️ The wire-level manifest fields before `schema_symref` has been resolved against a
/// symbol table (on decode) or after it has been resolved to a symref (on encode).
struct RawManifest {
    schema_symref: u64,
    schema_hash: [u8; 32],
    doc_span: ByteRange,
    doc_frame_count: u64,
    symbols_span: ByteRange,
    chunk_table_span: ByteRange,
    field_index_span: ByteRange,
    uncompressed_body_len: u64,
    field_count: u64,
    chunk_count: u64,
    symbol_count: u64,
}

async fn write_span(buf: &mut Vec<u8>, span: ByteRange) {
    write_varint_u64(buf, span.offset).await;
    write_varint_u64(buf, span.len).await;
}

async fn read_span(payload: &[u8], pos: &mut usize) -> Result<ByteRange, PackError> {
    let offset = read_varint_u64(payload, pos).await?;
    let len = read_varint_u64(payload, pos).await?;
    Ok(ByteRange { offset, len })
}

/// @emoji ✍️ Serializes the manifest segment payload per the contract's field order.
async fn encode_manifest_bytes(schema_symref: u64, manifest: &Manifest) -> Vec<u8> {
    let mut buf = Vec::new();
    write_varint_u64(&mut buf, schema_symref).await;
    buf.extend_from_slice(&manifest.schema_hash);
    write_span(&mut buf, manifest.doc_span).await;
    write_varint_u64(&mut buf, manifest.doc_frame_count).await;
    write_span(&mut buf, manifest.symbols_span).await;
    write_span(&mut buf, manifest.chunk_table_span).await;
    write_span(&mut buf, manifest.field_index_span).await;
    write_varint_u64(&mut buf, manifest.uncompressed_body_len).await;
    write_varint_u64(&mut buf, manifest.field_count).await;
    write_varint_u64(&mut buf, manifest.chunk_count).await;
    write_varint_u64(&mut buf, manifest.symbol_count).await;
    buf
}

/// @emoji 📖️ Parses the manifest segment payload. Trailing bytes beyond the known fields are
/// silently ignored (additive-evolution slot), never an error.
async fn parse_raw_manifest(payload: &[u8]) -> Result<RawManifest, PackError> {
    let mut pos = 0usize;
    let schema_symref = read_varint_u64(payload, &mut pos).await?;
    if pos + 32 > payload.len() {
        return Err(PackError::Truncated(pos as u64));
    }
    let mut schema_hash = [0u8; 32];
    schema_hash.copy_from_slice(&payload[pos..pos + 32]);
    pos += 32;
    let doc_span = read_span(payload, &mut pos).await?;
    let doc_frame_count = read_varint_u64(payload, &mut pos).await?;
    let symbols_span = read_span(payload, &mut pos).await?;
    let chunk_table_span = read_span(payload, &mut pos).await?;
    let field_index_span = read_span(payload, &mut pos).await?;
    let uncompressed_body_len = read_varint_u64(payload, &mut pos).await?;
    let field_count = read_varint_u64(payload, &mut pos).await?;
    let chunk_count = read_varint_u64(payload, &mut pos).await?;
    let symbol_count = read_varint_u64(payload, &mut pos).await?;
    Ok(RawManifest { schema_symref, schema_hash, doc_span, doc_frame_count, symbols_span, chunk_table_span, field_index_span, uncompressed_body_len, field_count, chunk_count, symbol_count })
}

// 🚫️async: E1 pure struct-field resolution — no I/O, no call into any async fn (unlike its
// siblings in this region it never touches `crc32c`/`read_varint_u64`), so nothing forces async.
fn resolve_manifest(raw: &RawManifest, symbols: &[String]) -> Result<Manifest, PackError> {
    let schema_name =
        if symbols.is_empty() && raw.schema_symref == 0 { String::new() } else { symbols.get(raw.schema_symref as usize).cloned().ok_or(PackError::Malformed { what: "manifest", offset: 0, detail: "schema symref out of range".to_string() })? };
    Ok(Manifest {
        schema_name,
        schema_hash: raw.schema_hash,
        doc_span: raw.doc_span,
        doc_frame_count: raw.doc_frame_count,
        symbols_span: raw.symbols_span,
        chunk_table_span: raw.chunk_table_span,
        field_index_span: raw.field_index_span,
        uncompressed_body_len: raw.uncompressed_body_len,
        field_count: raw.field_count,
        chunk_count: raw.chunk_count,
        symbol_count: raw.symbol_count,
    })
}
//#endregion 🔖️Manifest

//#region 🔖️Verify
/// @emoji 🛡️ How much a read verifies as it goes: `Trusted` skips all checksums (fastest,
/// for already-verified local data), `Standard` (default) verifies every segment's CRC-32C as
/// it's read, `Full` additionally re-hashes chunk/document content against the blake3 hashes in
/// the chunk table and footer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum VerificationLevel {
    Trusted,
    #[default]
    Standard,
    Full,
}

impl VerificationLevel {
    // 🚫️async: E1 pure enum-variant predicate — no I/O, no async call anywhere in the body.
    fn checks_crc(self) -> bool {
        !matches!(self, Self::Trusted)
    }

    fn checks_content_hash(self) -> bool {
        matches!(self, Self::Full)
    }
}
//#endregion 🔖️Verify

//#region 🔖️Writer
/// @emoji ⚙️ Header flags and the codec to compress every segment/chunk with when building a
/// pack file. `REQUIRED_COMPRESSED` is set automatically in the written header whenever `codec`
/// is non-identity, so callers only need to set it themselves for other reasons (e.g. signaling
/// intent before any segment is written).
#[derive(Clone, Copy, Debug)]
pub struct WriteOptions {
    pub required_flags: u32,
    pub optional_flags: u32,
    pub codec: CodecId,
}

/// @emoji ✒️ Sequential pack file builder. Write segments/chunks in any order, then `finish`
/// with a `Manifest` — `finish` fills in `symbols_span`/`chunk_table_span`/`chunk_count`/
/// `symbol_count` authoritatively from what was actually written (the caller-supplied values in
/// those fields are ignored), and resolves `manifest.schema_name` to a symref against the last
/// `write_segment(KIND_SYMBOLS, ...)` call, so a symbols segment must be written (containing
/// `schema_name`, unless it's empty) before `finish` is called.
pub struct PackWriter<S: PackSink> {
    sink: S,
    options: WriteOptions,
    chunks: Vec<ChunkTableEntry>,
    symbols: Vec<String>,
    symbols_span: Option<ByteRange>,
    document_hasher: blake3::Hasher,
}

impl<S: PackSink> PackWriter<S> {
    /// @emoji 🚀️ Writes the 32-byte header and returns a writer positioned right after it.
    pub async fn begin(mut sink: S, options: &WriteOptions) -> Result<Self, PackError> {
        let mut required_flags = options.required_flags;
        if options.codec.0 != 0 {
            required_flags |= REQUIRED_COMPRESSED;
        }
        let unknown = required_flags & !REQUIRED_KNOWN_MASK;
        if unknown != 0 {
            return Err(PackError::UnknownRequiredFlags(unknown));
        }
        let header = Header { version_major: FORMAT_VERSION_MAJOR, version_minor: FORMAT_VERSION_MINOR, required_flags, optional_flags: options.optional_flags };
        sink.write_all(&header.write_bytes().await).await?;
        Ok(Self { sink, options: WriteOptions { required_flags, optional_flags: options.optional_flags, codec: options.codec }, chunks: Vec::new(), symbols: Vec::new(), symbols_span: None, document_hasher: blake3::Hasher::new() })
    }

    /// @emoji 📍️ Current absolute write position — the offset the next segment/chunk will start
    /// at. Callers building a `Manifest` (e.g. `doc_span`/`field_index_span`) call this before
    /// and after their own `write_segment` calls to record spans this writer doesn't track
    /// automatically.
    pub async fn position(&self) -> u64 {
        self.sink.position().await
    }

    /// @emoji 🖇️ Frames, compresses (per `options.codec`), CRCs, and writes one segment. A
    /// `KIND_SYMBOLS` segment is parsed and remembered for `schema_name` resolution in `finish`;
    /// a `KIND_DOCUMENT` segment's raw bytes are folded into the running content-hash used for
    /// the footer.
    pub async fn write_segment(&mut self, kind: u8, payload: &[u8]) -> Result<(), PackError> {
        let base = self.sink.position().await;
        let encoded = encode_segment(kind, self.options.codec, payload).await?;
        self.sink.write_all(&encoded.bytes).await?;
        if kind == crate::KIND_SYMBOLS {
            self.symbols = decode_symbols(payload, &PackLimits::default()).await?;
            self.symbols_span = Some(ByteRange { offset: base, len: encoded.bytes.len() as u64 });
        }
        if kind == crate::KIND_DOCUMENT {
            self.document_hasher.update(payload);
        }
        Ok(())
    }

    /// @emoji 🧱️ Writes a `KIND_CHUNK` segment and records its offset/lengths/hashes for the
    /// chunk table `finish` will emit.
    pub async fn write_chunk(&mut self, payload: &[u8]) -> Result<ChunkId, PackError> {
        let base = self.sink.position().await;
        let encoded = encode_segment(crate::KIND_CHUNK, self.options.codec, payload).await?;
        let payload_offset = base + encoded.header_len as u64;
        let stored_bytes = &encoded.bytes[encoded.header_len..encoded.header_len + encoded.stored_len];
        let stored_crc = crc32c(stored_bytes).await;
        let raw_hash = blake3::hash(payload);
        self.sink.write_all(&encoded.bytes).await?;
        let id = ChunkId(self.chunks.len() as u32);
        self.chunks.push(ChunkTableEntry { offset: payload_offset, stored_len: encoded.stored_len as u64, raw_len: payload.len() as u64, crc32: stored_crc, blake3: *raw_hash.as_bytes() });
        Ok(id)
    }

    /// @emoji 🏁️ Writes the chunk table (if any chunks were written), the manifest, an `End`
    /// segment, then the footer — and returns the underlying sink.
    pub async fn finish(mut self, manifest: &Manifest) -> Result<S, PackError> {
        let mut chunk_table_span = ByteRange { offset: 0, len: 0 };
        if !self.chunks.is_empty() {
            let base = self.sink.position().await;
            let table_bytes = encode_chunk_table(&self.chunks).await;
            let encoded = encode_segment(crate::KIND_CHUNK_TABLE, self.options.codec, &table_bytes).await?;
            self.sink.write_all(&encoded.bytes).await?;
            chunk_table_span = ByteRange { offset: base, len: encoded.bytes.len() as u64 };
        }
        let schema_symref = if manifest.schema_name.is_empty() {
            0u64
        } else {
            self.symbols.iter().position(|symbol| symbol == &manifest.schema_name).ok_or_else(|| PackError::Schema(format!("schema_name {:?} not found in written symbols table", manifest.schema_name)))? as u64
        };
        let final_manifest = Manifest {
            schema_name: manifest.schema_name.clone(),
            schema_hash: manifest.schema_hash,
            doc_span: manifest.doc_span,
            doc_frame_count: manifest.doc_frame_count,
            symbols_span: self.symbols_span.unwrap_or(ByteRange { offset: 0, len: 0 }),
            chunk_table_span,
            field_index_span: manifest.field_index_span,
            uncompressed_body_len: manifest.uncompressed_body_len,
            field_count: manifest.field_count,
            chunk_count: self.chunks.len() as u64,
            symbol_count: self.symbols.len() as u64,
        };
        let manifest_bytes = encode_manifest_bytes(schema_symref, &final_manifest).await;
        let manifest_base = self.sink.position().await;
        let manifest_encoded = encode_segment(crate::KIND_MANIFEST, self.options.codec, &manifest_bytes).await?;
        self.sink.write_all(&manifest_encoded.bytes).await?;
        let manifest_span = ByteRange { offset: manifest_base, len: manifest_encoded.bytes.len() as u64 };

        let end_encoded = encode_segment(crate::KIND_END, CodecId(0), &[]).await?;
        self.sink.write_all(&end_encoded.bytes).await?;

        let content_hash = ContentHash(*self.document_hasher.finalize().as_bytes());
        let file_len = self.sink.position().await + FOOTER_SIZE as u64;
        let footer = Footer {
            version_major: FORMAT_VERSION_MAJOR,
            version_minor: FORMAT_VERSION_MINOR,
            required_flags: self.options.required_flags,
            manifest_offset: manifest_span.offset,
            manifest_len: manifest_span.len,
            file_len,
            content_hash,
            prev_footer_offset: 0,
        };
        self.sink.write_all(&footer.write_bytes().await).await?;
        self.sink.flush().await?;
        Ok(self.sink)
    }
}
//#endregion 🔖️Writer

//#region 🔖️Reader
/// @emoji 🪪️ The two fixed-size, always-present anchors of a pack file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Superblock {
    pub header: Header,
    pub footer: Footer,
}

/// @emoji 📂️ Random-access pack file reader with three progressively-deeper open levels:
/// `open_superblock` (header+footer only), `open_manifest` (+manifest+symbols+chunk table),
/// and `body_bytes`/`read_chunk` (full content, decompressed and optionally content-hash
/// verified).
pub struct PackFile<S: PackSource> {
    source: S,
    limits: PackLimits,
    superblock: Superblock,
    manifest: Option<Manifest>,
    symbols: Vec<String>,
    chunk_table: Vec<ChunkTableEntry>,
}

impl<S: PackSource> PackFile<S> {
    /// @emoji 1⃣ Level 1: parses and CRC-validates the header and footer only, and cross-checks
    /// the footer's `file_len` against the actual source length.
    pub async fn open_superblock(source: S, limits: &PackLimits) -> Result<Self, PackError> {
        let len = source.len().await;
        if len < FOOTER_SIZE as u64 {
            return Err(PackError::Truncated(len));
        }
        let mut header_bytes = [0u8; HEADER_SIZE];
        source.read_exact_at(0, &mut header_bytes).await?;
        let header = Header::parse(&header_bytes).await?;
        let mut footer_bytes = vec![0u8; FOOTER_SIZE];
        source.read_exact_at(len - FOOTER_SIZE as u64, &mut footer_bytes).await?;
        let footer = Footer::parse(&footer_bytes).await?;
        if footer.file_len != len {
            return Err(PackError::Malformed { what: "footer", offset: len - FOOTER_SIZE as u64, detail: "file_len does not match actual source length".to_string() });
        }
        Ok(Self { source, limits: limits.clone(), superblock: Superblock { header, footer }, manifest: None, symbols: Vec::new(), chunk_table: Vec::new() })
    }

    /// @emoji 2⃣ Level 2: `open_superblock` plus decoding the manifest, its symbol table (used
    /// to resolve `manifest().schema_name`), and the chunk table (if present).
    pub async fn open_manifest(source: S, limits: &PackLimits, verification: VerificationLevel) -> Result<Self, PackError> {
        let mut this = Self::open_superblock(source, limits).await?;
        let verify_crc = verification.checks_crc();
        let manifest_seg = decode_segment_at(&this.source, this.superblock.footer.manifest_offset, &this.limits, verify_crc).await?;
        if manifest_seg.kind != crate::KIND_MANIFEST {
            return Err(PackError::Malformed { what: "manifest", offset: this.superblock.footer.manifest_offset, detail: "expected KIND_MANIFEST segment".to_string() });
        }
        if manifest_seg.consumed != this.superblock.footer.manifest_len {
            return Err(PackError::Malformed { what: "manifest", offset: this.superblock.footer.manifest_offset, detail: "manifest_len mismatch".to_string() });
        }
        let raw = parse_raw_manifest(&manifest_seg.payload).await?;
        let symbols = if raw.symbols_span.len > 0 {
            let seg = decode_segment_at(&this.source, raw.symbols_span.offset, &this.limits, verify_crc).await?;
            if seg.kind != crate::KIND_SYMBOLS {
                return Err(PackError::Malformed { what: "symbols", offset: raw.symbols_span.offset, detail: "expected KIND_SYMBOLS segment".to_string() });
            }
            decode_symbols(&seg.payload, &this.limits).await?
        } else {
            Vec::new()
        };
        let chunk_table = if raw.chunk_table_span.len > 0 {
            let seg = decode_segment_at(&this.source, raw.chunk_table_span.offset, &this.limits, verify_crc).await?;
            if seg.kind != crate::KIND_CHUNK_TABLE {
                return Err(PackError::Malformed { what: "chunk_table", offset: raw.chunk_table_span.offset, detail: "expected KIND_CHUNK_TABLE segment".to_string() });
            }
            decode_chunk_table(&seg.payload, &this.limits).await?
        } else {
            Vec::new()
        };
        let manifest = resolve_manifest(&raw, &symbols)?;
        this.manifest = Some(manifest);
        this.symbols = symbols;
        this.chunk_table = chunk_table;
        Ok(this)
    }

    pub fn superblock(&self) -> &Superblock {
        &self.superblock
    }

    pub fn manifest(&self) -> Option<&Manifest> {
        self.manifest.as_ref()
    }

    /// @emoji 🔤️ Resolves a symref (index into the symbol table loaded by `open_manifest`).
    pub fn symbol(&self, symref: u64) -> Result<&str, PackError> {
        self.symbols.get(symref as usize).map(String::as_str).ok_or(PackError::Malformed { what: "symref", offset: symref, detail: "symref out of range".to_string() })
    }

    pub fn chunk_count(&self) -> u64 {
        self.chunk_table.len() as u64
    }

    /// @emoji 📏️ The `(offset, stored_len)` range of a chunk's on-disk (possibly compressed)
    /// payload bytes — suitable for a range-fetch (see `pack_http`) without decoding.
    pub fn chunk_range(&self, id: ChunkId) -> Result<ByteRange, PackError> {
        self.chunk_table.get(id.0 as usize).map(|entry| ByteRange { offset: entry.offset, len: entry.stored_len }).ok_or(PackError::Malformed { what: "chunk_id", offset: id.0 as u64, detail: "unknown chunk id".to_string() })
    }

    /// @emoji 3⃣ Level 3: reads, optionally CRC-verifies (`Standard`+) and decompresses one
    /// chunk; at `Full` also verifies its blake3 content hash.
    pub async fn read_chunk(&self, id: ChunkId, verification: VerificationLevel) -> Result<Vec<u8>, PackError> {
        let entry = self.chunk_table.get(id.0 as usize).ok_or(PackError::Malformed { what: "chunk_id", offset: id.0 as u64, detail: "unknown chunk id".to_string() })?;
        if entry.stored_len > self.limits.max_segment_len || entry.raw_len > self.limits.max_segment_len {
            return Err(PackError::LimitExceeded("chunk length exceeds max_segment_len"));
        }
        let total_len = self.source.len().await;
        let end = entry.offset.checked_add(entry.stored_len).ok_or(PackError::LimitExceeded("chunk range overflow"))?;
        if end > total_len {
            return Err(PackError::Truncated(entry.offset));
        }
        let mut stored = vec![0u8; entry.stored_len as usize];
        self.source.read_exact_at(entry.offset, &mut stored).await?;
        if verification.checks_crc() {
            let computed = crc32c(&stored).await;
            if computed != entry.crc32 {
                return Err(PackError::ChecksumMismatch { segment: "chunk", offset: entry.offset });
            }
        }
        let raw = if entry.stored_len == entry.raw_len {
            stored
        } else {
            let codec = if self.superblock.header.required_flags & REQUIRED_COMPRESSED != 0 { CodecId(1) } else { CodecId(0) };
            codec_decompress(codec, &stored, entry.raw_len, self.limits.max_segment_len).await?
        };
        if verification.checks_content_hash() {
            let hash = blake3::hash(&raw);
            if hash.as_bytes() != &entry.blake3 {
                return Err(PackError::ContentHashMismatch);
            }
        }
        Ok(raw)
    }

    /// @emoji 📄️ Level 3: reads and concatenates the `doc_frame_count` `KIND_DOCUMENT` segments
    /// starting at `manifest().doc_span.offset`; at `Full` also verifies the result's blake3
    /// hash against the footer's `content_hash`.
    pub async fn body_bytes(&self, verification: VerificationLevel) -> Result<Vec<u8>, PackError> {
        let manifest = self.manifest.as_ref().ok_or_else(|| PackError::Schema("manifest not loaded; call open_manifest first".to_string()))?;
        let mut out = Vec::new();
        if manifest.doc_span.len > 0 {
            let mut offset = manifest.doc_span.offset;
            let frames = manifest.doc_frame_count.max(1);
            for _ in 0..frames {
                let seg = decode_segment_at(&self.source, offset, &self.limits, verification.checks_crc()).await?;
                if seg.kind != crate::KIND_DOCUMENT {
                    return Err(PackError::Malformed { what: "document", offset, detail: "expected KIND_DOCUMENT segment".to_string() });
                }
                out.extend_from_slice(&seg.payload);
                offset += seg.consumed;
            }
        }
        if verification.checks_content_hash() {
            let hash = blake3::hash(&out);
            if hash.as_bytes() != &self.superblock.footer.content_hash.0 {
                return Err(PackError::ContentHashMismatch);
            }
        }
        Ok(out)
    }

    /// @emoji #⃣ The footer's content hash — no decode needed.
    pub fn content_hash(&self) -> ContentHash {
        self.superblock.footer.content_hash
    }
}

/// @emoji 🔎️ Standalone helper (used by `crate::content_hash`) that reads and parses only the
/// last `FOOTER_SIZE` bytes of `source`, without touching the header or any segment.
pub async fn read_footer_only<S: PackSource>(source: &S) -> Result<Footer, PackError> {
    let len = source.len().await;
    if len < FOOTER_SIZE as u64 {
        return Err(PackError::Truncated(len));
    }
    let mut buf = vec![0u8; FOOTER_SIZE];
    source.read_exact_at(len - FOOTER_SIZE as u64, &mut buf).await?;
    Footer::parse(&buf).await
}
//#endregion 🔖️Reader

//#region 🔖️Recover
/// @emoji 🩹️ What a forward-scan recovery pass managed to salvage.
#[derive(Clone, Debug)]
pub struct RecoveryReport {
    pub segments_recovered: u64,
    pub bytes_recovered: u64,
    pub manifest: Option<Manifest>,
}

/// @emoji 🩺️ Forward-scans from byte `HEADER_SIZE` (right after the header), CRC-validating and
/// accumulating one segment at a time until the first invalid/truncated segment, a `KIND_END`
/// segment, or end of file — whichever comes first. Unrecognized segment kinds are accumulated,
/// not rejected. If a `KIND_MANIFEST` and (when its `schema_name` is non-empty) a matching
/// `KIND_SYMBOLS` segment were both recovered, the manifest is resolved and returned too. Used
/// when the footer itself fails to parse/validate.
pub async fn recover<S: PackSource>(source: &S, limits: &PackLimits) -> Result<RecoveryReport, PackError> {
    let len = source.len().await;
    if len < HEADER_SIZE as u64 {
        return Err(PackError::Truncated(len));
    }
    let mut header_bytes = [0u8; HEADER_SIZE];
    source.read_exact_at(0, &mut header_bytes).await?;
    Header::parse(&header_bytes).await?;

    let mut offset = HEADER_SIZE as u64;
    let mut segments_recovered = 0u64;
    let mut bytes_recovered = 0u64;
    let mut found: Vec<(u8, Vec<u8>)> = Vec::new();
    while offset < len {
        match decode_segment_at(source, offset, limits, true).await {
            Ok(seg) => {
                segments_recovered += 1;
                bytes_recovered += seg.consumed;
                let is_end = seg.kind == crate::KIND_END;
                offset += seg.consumed;
                found.push((seg.kind, seg.payload));
                if is_end {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    let mut symbols = Vec::new();
    if let Some((_, payload)) = found.iter().find(|(kind, _)| *kind == crate::KIND_SYMBOLS) {
        if let Ok(decoded) = decode_symbols(payload, limits).await {
            symbols = decoded;
        }
    }
    let mut manifest = None;
    if let Some((_, payload)) = found.iter().find(|(kind, _)| *kind == crate::KIND_MANIFEST) {
        if let Ok(raw) = parse_raw_manifest(payload).await {
            manifest = resolve_manifest(&raw, &symbols).ok();
        }
    }

    Ok(RecoveryReport { segments_recovered, bytes_recovered, manifest })
}
//#endregion 🔖️Recover


//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️Header
    #[semio_framework_async_macros::async_test]
    async fn header_hand_built_bytes_parse_round_trip() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&MAGIC);
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes.extend_from_slice(&REQUIRED_COMPRESSED.to_le_bytes());
        bytes.extend_from_slice(&OPTIONAL_CANONICAL.to_le_bytes());
        let crc = crc32c(&bytes[0..20]).await;
        bytes.extend_from_slice(&crc.to_le_bytes());
        bytes.extend_from_slice(&[0u8; 8]);
        assert_eq!(bytes.len(), HEADER_SIZE);

        let header = Header::parse(&bytes).await.unwrap();
        assert_eq!(header.version_major, 1);
        assert_eq!(header.version_minor, 0);
        assert_eq!(header.required_flags, REQUIRED_COMPRESSED);
        assert_eq!(header.optional_flags, OPTIONAL_CANONICAL);
        assert_eq!(header.write_bytes().await[0..24], bytes[0..24]);
    }

    #[semio_framework_async_macros::async_test]
    async fn header_parse_rejects_bad_magic() {
        let bytes = [0u8; HEADER_SIZE];
        assert_eq!(Header::parse(&bytes).await, Err(PackError::BadMagic));
    }

    #[semio_framework_async_macros::async_test]
    async fn header_parse_rejects_bad_crc() {
        let header = Header { version_major: 1, version_minor: 0, required_flags: 0, optional_flags: 0 };
        let mut bytes = header.write_bytes().await;
        bytes[20] ^= 0xFF;
        assert!(matches!(Header::parse(&bytes).await, Err(PackError::ChecksumMismatch { segment: "header", .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn header_parse_rejects_unknown_required_flags() {
        let header = Header { version_major: 1, version_minor: 0, required_flags: 1 << 4, optional_flags: 0 };
        let bytes = header.write_bytes().await;
        assert_eq!(Header::parse(&bytes).await, Err(PackError::UnknownRequiredFlags(1 << 4)));
    }

    #[semio_framework_async_macros::async_test]
    async fn header_parse_rejects_unsupported_version() {
        let header = Header { version_major: 2, version_minor: 0, required_flags: 0, optional_flags: 0 };
        let bytes = header.write_bytes().await;
        assert_eq!(Header::parse(&bytes).await, Err(PackError::UnsupportedVersion { major: 2, minor: 0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn header_truncated_at_every_byte_boundary_errors_never_panics() {
        let header = Header { version_major: 1, version_minor: 0, required_flags: REQUIRED_COMPRESSED, optional_flags: 0 };
        let full = header.write_bytes().await;
        for len in 0..HEADER_SIZE {
            let slice = &full[..len];
            assert!(Header::parse(slice).await.is_err(), "expected error at header truncation length {len}");
            let limits = PackLimits::default();
            assert!(PackFile::open_superblock(slice, &limits).await.is_err(), "expected error opening superblock at length {len}");
        }
    }
    //#endregion 🔖️Header

    //#region 🔖️Footer
    #[semio_framework_async_macros::async_test]
    async fn footer_hand_built_bytes_parse_round_trip() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&FOOTER_MAGIC);
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&32u64.to_le_bytes());
        bytes.extend_from_slice(&100u64.to_le_bytes());
        bytes.extend_from_slice(&500u64.to_le_bytes());
        let hash = [7u8; 32];
        bytes.extend_from_slice(&hash);
        bytes.extend_from_slice(&0u64.to_le_bytes());
        assert_eq!(bytes.len(), FOOTER_SIZE - 4);
        let crc = crc32c(&bytes).await;
        bytes.extend_from_slice(&crc.to_le_bytes());
        assert_eq!(bytes.len(), FOOTER_SIZE);

        let footer = Footer::parse(&bytes).await.unwrap();
        assert_eq!(footer.version_major, 1);
        assert_eq!(footer.manifest_offset, 32);
        assert_eq!(footer.manifest_len, 100);
        assert_eq!(footer.file_len, 500);
        assert_eq!(footer.content_hash.0, hash);
        assert_eq!(footer.prev_footer_offset, 0);
        assert_eq!(footer.write_bytes().await, bytes);
    }

    #[semio_framework_async_macros::async_test]
    async fn footer_parse_rejects_bad_magic() {
        let bytes = [0u8; FOOTER_SIZE];
        assert_eq!(Footer::parse(&bytes).await, Err(PackError::BadMagic));
    }

    #[semio_framework_async_macros::async_test]
    async fn footer_parse_rejects_bad_crc() {
        let footer = Footer { version_major: 1, version_minor: 0, required_flags: 0, manifest_offset: 32, manifest_len: 10, file_len: 200, content_hash: ContentHash([1u8; 32]), prev_footer_offset: 0 };
        let mut bytes = footer.write_bytes().await;
        let last = bytes.len() - 1;
        bytes[last] ^= 0xFF;
        assert!(matches!(Footer::parse(&bytes).await, Err(PackError::ChecksumMismatch { segment: "footer", .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn footer_truncated_at_every_byte_boundary_errors_never_panics() {
        let footer = Footer { version_major: 1, version_minor: 0, required_flags: 0, manifest_offset: 32, manifest_len: 10, file_len: 200, content_hash: ContentHash([2u8; 32]), prev_footer_offset: 0 };
        let full = footer.write_bytes().await;
        for len in 0..FOOTER_SIZE {
            let slice = &full[..len];
            assert!(Footer::parse(slice).await.is_err(), "expected error at footer truncation length {len}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn open_superblock_truncated_at_every_footer_byte_boundary_errors_never_panics() {
        let options = WriteOptions { required_flags: 0, optional_flags: 0, codec: CodecId(0) };
        let mut writer = PackWriter::begin(Vec::<u8>::new(), &options).await.unwrap();
        writer.write_segment(crate::KIND_DOCUMENT, b"hello world").await.unwrap();
        let manifest = Manifest {
            schema_name: String::new(),
            schema_hash: [0u8; 32],
            doc_span: ByteRange { offset: HEADER_SIZE as u64, len: 0 },
            doc_frame_count: 0,
            symbols_span: ByteRange { offset: 0, len: 0 },
            chunk_table_span: ByteRange { offset: 0, len: 0 },
            field_index_span: ByteRange { offset: 0, len: 0 },
            uncompressed_body_len: 11,
            field_count: 0,
            chunk_count: 0,
            symbol_count: 0,
        };
        let full = writer.finish(&manifest).await.unwrap();
        let limits = PackLimits::default();
        let start = full.len() - FOOTER_SIZE;
        for len in start..full.len() {
            let slice = &full[..len];
            assert!(PackFile::open_superblock(slice, &limits).await.is_err(), "expected error at file truncation length {len}");
        }
        assert!(PackFile::open_superblock(full.as_slice(), &limits).await.is_ok());
    }
    //#endregion 🔖️Footer

    //#region 🔖️Segment
    #[semio_framework_async_macros::async_test]
    async fn segment_skip_unknown_kind_decodes_without_error() {
        let unknown_kind = 0x50u8;
        let encoded = encode_segment(unknown_kind, CodecId(0), b"extension payload").await.unwrap();
        let mut file = vec![0u8; HEADER_SIZE];
        file.extend_from_slice(&encoded.bytes);
        let limits = PackLimits::default();
        let decoded = decode_segment_at(&file, HEADER_SIZE as u64, &limits, true).await.unwrap();
        assert_eq!(decoded.kind, unknown_kind);
        assert_eq!(decoded.payload, b"extension payload");
    }

    #[semio_framework_async_macros::async_test]
    async fn recover_accumulates_unknown_kind_segments_without_erroring() {
        let header = Header { version_major: 1, version_minor: 0, required_flags: 0, optional_flags: 0 };
        let mut file = header.write_bytes().await.to_vec();
        file.extend_from_slice(&encode_segment(0x60, CodecId(0), b"future extension").await.unwrap().bytes);
        file.extend_from_slice(&encode_segment(crate::KIND_END, CodecId(0), &[]).await.unwrap().bytes);
        let limits = PackLimits::default();
        let report = recover(&file, &limits).await.unwrap();
        assert_eq!(report.segments_recovered, 2);
        assert!(report.manifest.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn segment_crc_mismatch_is_detected() {
        let encoded = encode_segment(crate::KIND_DOCUMENT, CodecId(0), b"payload").await.unwrap();
        let mut file = vec![0u8; HEADER_SIZE];
        file.extend_from_slice(&encoded.bytes);
        let last = file.len() - 1;
        file[last] ^= 0xFF;
        let limits = PackLimits::default();
        let result = decode_segment_at(&file, HEADER_SIZE as u64, &limits, true).await;
        assert!(matches!(result, Err(PackError::ChecksumMismatch { segment: "segment", .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn segment_crc_mismatch_ignored_at_trusted_level() {
        let encoded = encode_segment(crate::KIND_DOCUMENT, CodecId(0), b"payload").await.unwrap();
        let mut file = vec![0u8; HEADER_SIZE];
        file.extend_from_slice(&encoded.bytes);
        let last = file.len() - 1;
        file[last] ^= 0xFF;
        let limits = PackLimits::default();
        let result = decode_segment_at(&file, HEADER_SIZE as u64, &limits, false).await;
        assert!(result.is_ok());
    }
    //#endregion 🔖️Segment

    //#region 🔖️Writer
    async fn build_sample_pack(codec: CodecId) -> (Vec<u8>, ChunkId, Vec<u8>) {
        let options = WriteOptions { required_flags: 0, optional_flags: OPTIONAL_CANONICAL, codec };
        let mut writer = PackWriter::begin(Vec::<u8>::new(), &options).await.unwrap();

        writer.write_segment(crate::KIND_SYMBOLS, &encode_symbols(&["widget.v1".to_string(), "name".to_string()]).await).await.unwrap();

        let doc_offset = writer.position().await;
        let document_payload = b"the quick brown fox jumps over the lazy dog, repeatedly, for compressibility".to_vec();
        writer.write_segment(crate::KIND_DOCUMENT, &document_payload).await.unwrap();
        let doc_len = writer.position().await - doc_offset;

        let chunk_payload = vec![42u8; 4096];
        let chunk_id = writer.write_chunk(&chunk_payload).await.unwrap();

        let manifest = Manifest {
            schema_name: "widget.v1".to_string(),
            schema_hash: [9u8; 32],
            doc_span: ByteRange { offset: doc_offset, len: doc_len },
            doc_frame_count: 1,
            symbols_span: ByteRange { offset: 0, len: 0 },
            chunk_table_span: ByteRange { offset: 0, len: 0 },
            field_index_span: ByteRange { offset: 0, len: 0 },
            uncompressed_body_len: document_payload.len() as u64,
            field_count: 1,
            chunk_count: 0,
            symbol_count: 0,
        };
        let bytes = writer.finish(&manifest).await.unwrap();
        (bytes, chunk_id, chunk_payload)
    }

    #[semio_framework_async_macros::async_test]
    async fn write_then_read_round_trip_uncompressed() {
        let (bytes, chunk_id, chunk_payload) = build_sample_pack(CodecId(0)).await;
        let limits = PackLimits::default();
        let file = PackFile::open_manifest(bytes.as_slice(), &limits, VerificationLevel::Full).await.unwrap();

        let manifest = file.manifest().unwrap();
        assert_eq!(manifest.schema_name, "widget.v1");
        assert_eq!(manifest.chunk_count, 1);
        assert_eq!(manifest.symbol_count, 2);

        assert_eq!(file.chunk_count(), 1);
        let read_back = file.read_chunk(chunk_id, VerificationLevel::Full).await.unwrap();
        assert_eq!(read_back, chunk_payload);

        let body = file.body_bytes(VerificationLevel::Full).await.unwrap();
        assert_eq!(body, b"the quick brown fox jumps over the lazy dog, repeatedly, for compressibility");

        assert_eq!(file.content_hash(), file.superblock().footer.content_hash);
    }

    #[semio_framework_async_macros::async_test]
    async fn write_then_read_round_trip_with_compressed_segment_and_chunk() {
        let (bytes, chunk_id, chunk_payload) = build_sample_pack(CodecId(1)).await;
        let limits = PackLimits::default();
        assert_eq!(bytes[0..8], MAGIC);

        let superblock_only = PackFile::open_superblock(bytes.as_slice(), &limits).await.unwrap();
        assert_eq!(superblock_only.superblock().header.required_flags & REQUIRED_COMPRESSED, REQUIRED_COMPRESSED);

        let file = PackFile::open_manifest(bytes.as_slice(), &limits, VerificationLevel::Full).await.unwrap();
        let manifest = file.manifest().unwrap();
        assert_eq!(manifest.schema_name, "widget.v1");

        let read_back = file.read_chunk(chunk_id, VerificationLevel::Full).await.unwrap();
        assert_eq!(read_back, chunk_payload);

        let body = file.body_bytes(VerificationLevel::Full).await.unwrap();
        assert_eq!(body, b"the quick brown fox jumps over the lazy dog, repeatedly, for compressibility");
    }

    #[semio_framework_async_macros::async_test]
    async fn read_footer_only_matches_full_open() {
        let (bytes, _chunk_id, _chunk_payload) = build_sample_pack(CodecId(0)).await;
        let footer = read_footer_only(&bytes).await.unwrap();
        let limits = PackLimits::default();
        let file = PackFile::open_superblock(bytes.as_slice(), &limits).await.unwrap();
        assert_eq!(footer, file.superblock().footer);
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_document_and_no_chunks_round_trips() {
        let options = WriteOptions { required_flags: 0, optional_flags: 0, codec: CodecId(0) };
        let writer = PackWriter::begin(Vec::<u8>::new(), &options).await.unwrap();
        let manifest = Manifest {
            schema_name: String::new(),
            schema_hash: [0u8; 32],
            doc_span: ByteRange { offset: 0, len: 0 },
            doc_frame_count: 0,
            symbols_span: ByteRange { offset: 0, len: 0 },
            chunk_table_span: ByteRange { offset: 0, len: 0 },
            field_index_span: ByteRange { offset: 0, len: 0 },
            uncompressed_body_len: 0,
            field_count: 0,
            chunk_count: 0,
            symbol_count: 0,
        };
        let bytes = writer.finish(&manifest).await.unwrap();
        let limits = PackLimits::default();
        let file = PackFile::open_manifest(bytes.as_slice(), &limits, VerificationLevel::Full).await.unwrap();
        assert_eq!(file.chunk_count(), 0);
        assert_eq!(file.body_bytes(VerificationLevel::Full).await.unwrap(), Vec::<u8>::new());
        let expected_empty_hash = ContentHash(*blake3::hash(b"").as_bytes());
        assert_eq!(file.content_hash(), expected_empty_hash);
    }

    #[semio_framework_async_macros::async_test]
    async fn finish_errors_when_schema_name_not_in_symbols() {
        let options = WriteOptions { required_flags: 0, optional_flags: 0, codec: CodecId(0) };
        let writer = PackWriter::begin(Vec::<u8>::new(), &options).await.unwrap();
        let manifest = Manifest {
            schema_name: "missing".to_string(),
            schema_hash: [0u8; 32],
            doc_span: ByteRange { offset: 0, len: 0 },
            doc_frame_count: 0,
            symbols_span: ByteRange { offset: 0, len: 0 },
            chunk_table_span: ByteRange { offset: 0, len: 0 },
            field_index_span: ByteRange { offset: 0, len: 0 },
            uncompressed_body_len: 0,
            field_count: 0,
            chunk_count: 0,
            symbol_count: 0,
        };
        let result = writer.finish(&manifest).await;
        assert!(matches!(result, Err(PackError::Schema(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn begin_rejects_unknown_required_flags() {
        let options = WriteOptions { required_flags: 1 << 5, optional_flags: 0, codec: CodecId(0) };
        let result = PackWriter::begin(Vec::<u8>::new(), &options).await;
        assert_eq!(result.err(), Some(PackError::UnknownRequiredFlags(1 << 5)));
    }
    //#endregion 🔖️Writer

    //#region 🔖️Corruption
    #[semio_framework_async_macros::async_test]
    async fn open_manifest_rejects_flipped_chunk_payload_crc_at_standard_level() {
        let (bytes, chunk_id, _chunk_payload) = build_sample_pack(CodecId(0)).await;
        let limits = PackLimits::default();
        let mut corrupted = bytes.clone();
        let file = PackFile::open_manifest(bytes.as_slice(), &limits, VerificationLevel::Standard).await.unwrap();
        let range = file.chunk_range(chunk_id).unwrap();
        corrupted[range.offset as usize] ^= 0xFF;
        let corrupted_file = PackFile::open_manifest(corrupted.as_slice(), &limits, VerificationLevel::Standard).await.unwrap();
        let result = corrupted_file.read_chunk(chunk_id, VerificationLevel::Standard).await;
        assert!(matches!(result, Err(PackError::ChecksumMismatch { segment: "chunk", .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn open_manifest_rejects_wrong_kind_at_manifest_offset() {
        let (bytes, _chunk_id, _chunk_payload) = build_sample_pack(CodecId(0)).await;
        let limits = PackLimits::default();
        let mut footer_bytes = bytes[bytes.len() - FOOTER_SIZE..].to_vec();
        let mut footer = Footer::parse(&footer_bytes).await.unwrap();
        footer.manifest_offset = HEADER_SIZE as u64; // points at the symbols segment instead
        footer_bytes = footer.write_bytes().await;
        let mut corrupted = bytes[..bytes.len() - FOOTER_SIZE].to_vec();
        corrupted.extend_from_slice(&footer_bytes);
        let result = PackFile::open_manifest(corrupted.as_slice(), &limits, VerificationLevel::Standard).await;
        assert!(matches!(result, Err(PackError::Malformed { what: "manifest", .. })));
    }
    //#endregion 🔖️Corruption

}
//#endregion 🧪️Tests
