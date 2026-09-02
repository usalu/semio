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
        buf[20..24].copy_from_slice(&crc.to_le_bytes());
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
        let computed_crc = crc32c(&bytes[0..20]);
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
        buf.extend_from_slice(&crc.to_le_bytes());
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
        let computed_crc = crc32c(&bytes[0..80]);
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

fn retained_varint(mut value: u64, output: &mut [u8; 10]) -> &[u8] {
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

fn retained_varint_len(value: u64) -> usize {
    let mut output = [0u8; 10];
    retained_varint(value, &mut output).len()
}

/// @emoji 🧵️ Resolves `CodecId` to this crate's codec implementations for compression.
async fn codec_compress(codec: CodecId, raw: &[u8]) -> Result<Vec<u8>, PackError> {
    match codec.0 {
        0 => Ok(raw.to_vec()),
        1 => crate::codec::deflate_compress(raw),
        other => Err(PackError::UnsupportedCodec(other)),
    }
}

/// @emoji 🧵️ Resolves `CodecId` to this crate's codec implementations for decompression.
async fn codec_decompress(codec: CodecId, stored: &[u8], raw_len: u64, limit: u64) -> Result<Vec<u8>, PackError> {
    match codec.0 {
        0 => NoCompression.decompress(stored, raw_len, limit),
        1 => crate::codec::deflate_decompress(stored, raw_len, limit),
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
    write_varint_u64(&mut buf, stored.len() as u64);
    if compressed {
        write_varint_u64(&mut buf, payload.len() as u64);
    }
    let header_len = buf.len();
    buf.extend_from_slice(&stored);
    let crc = crc32c(&buf);
    buf.extend_from_slice(&crc.to_le_bytes());
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
    let value = read_varint_u64(&tmp, &mut pos)?;
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
        let computed_crc = crc32c(&frame);
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
    write_varint_u64(&mut buf, symbols.len() as u64);
    for symbol in symbols {
        let bytes = symbol.as_bytes();
        write_varint_u64(&mut buf, bytes.len() as u64);
        buf.extend_from_slice(bytes);
    }
    buf
}

/// @emoji 📖️ Parses a symbol table, rejecting a count over `limits.max_symbols` before
/// allocating the output `Vec`.
async fn decode_symbols(payload: &[u8], limits: &PackLimits) -> Result<Vec<String>, PackError> {
    let mut pos = 0usize;
    let count = read_varint_u64(payload, &mut pos)?;
    if count > limits.max_symbols as u64 {
        return Err(PackError::LimitExceeded("symbol count exceeds max_symbols"));
    }
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let len = read_varint_u64(payload, &mut pos)? as usize;
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

/// @emoji 📖️ Parses the chunk table, rejecting a count over `limits.max_items` or an entry
/// length over `limits.max_segment_len` before allocating.
async fn decode_chunk_table(payload: &[u8], limits: &PackLimits) -> Result<Vec<ChunkTableEntry>, PackError> {
    let mut pos = 0usize;
    let count = read_varint_u64(payload, &mut pos)?;
    if count > limits.max_items {
        return Err(PackError::LimitExceeded("chunk_table count exceeds max_items"));
    }
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let offset = read_varint_u64(payload, &mut pos)?;
        let stored_len = read_varint_u64(payload, &mut pos)?;
        let raw_len = read_varint_u64(payload, &mut pos)?;
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
    write_varint_u64(buf, span.offset);
    write_varint_u64(buf, span.len);
}

async fn read_span(payload: &[u8], pos: &mut usize) -> Result<ByteRange, PackError> {
    let offset = read_varint_u64(payload, pos)?;
    let len = read_varint_u64(payload, pos)?;
    Ok(ByteRange { offset, len })
}

/// @emoji ✍️ Serializes the manifest segment payload per the contract's field order.
async fn encode_manifest_bytes(schema_symref: u64, manifest: &Manifest) -> Vec<u8> {
    let mut buf = Vec::new();
    write_varint_u64(&mut buf, schema_symref);
    buf.extend_from_slice(&manifest.schema_hash);
    write_span(&mut buf, manifest.doc_span).await;
    write_varint_u64(&mut buf, manifest.doc_frame_count);
    write_span(&mut buf, manifest.symbols_span).await;
    write_span(&mut buf, manifest.chunk_table_span).await;
    write_span(&mut buf, manifest.field_index_span).await;
    write_varint_u64(&mut buf, manifest.uncompressed_body_len);
    write_varint_u64(&mut buf, manifest.field_count);
    write_varint_u64(&mut buf, manifest.chunk_count);
    write_varint_u64(&mut buf, manifest.symbol_count);
    buf
}

/// @emoji 📖️ Parses the manifest segment payload. Trailing bytes beyond the known fields are
/// silently ignored (additive-evolution slot), never an error.
async fn parse_raw_manifest(payload: &[u8]) -> Result<RawManifest, PackError> {
    let mut pos = 0usize;
    let schema_symref = read_varint_u64(payload, &mut pos)?;
    if pos + 32 > payload.len() {
        return Err(PackError::Truncated(pos as u64));
    }
    let mut schema_hash = [0u8; 32];
    schema_hash.copy_from_slice(&payload[pos..pos + 32]);
    pos += 32;
    let doc_span = read_span(payload, &mut pos).await?;
    let doc_frame_count = read_varint_u64(payload, &mut pos)?;
    let symbols_span = read_span(payload, &mut pos).await?;
    let chunk_table_span = read_span(payload, &mut pos).await?;
    let field_index_span = read_span(payload, &mut pos).await?;
    let uncompressed_body_len = read_varint_u64(payload, &mut pos)?;
    let field_count = read_varint_u64(payload, &mut pos)?;
    let chunk_count = read_varint_u64(payload, &mut pos)?;
    let symbol_count = read_varint_u64(payload, &mut pos)?;
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
    document_hasher: semio_framework_hash::Hasher,
}

pub struct PackIdentitySegment<'a, S: PackSink> {
    owner: &'a mut PackWriter<S>,
    kind: u8,
    payload_len: usize,
    written: usize,
    crc: crate::codec::Crc32cCursor,
}

pub struct PackIdentityChunk<'a, S: PackSink> {
    owner: &'a mut PackWriter<S>,
    payload_offset: u64,
    payload_len: usize,
    written: usize,
    segment_crc: crate::codec::Crc32cCursor,
    payload_crc: crate::codec::Crc32cCursor,
    hash: semio_framework_hash::Hasher,
}

impl<S: PackSink> PackIdentityChunk<'_, S> {
    pub async fn write_fragment(&mut self, fragment: &[u8]) -> Result<(), PackError> {
        self.written = self.written.checked_add(fragment.len()).ok_or(PackError::LimitExceeded("chunk payload length overflow"))?;
        if self.written > self.payload_len {
            return Err(PackError::LimitExceeded("chunk exceeded retained payload reservation"));
        }
        self.segment_crc.update_page(fragment);
        self.payload_crc.update_page(fragment);
        self.hash.update(fragment);
        self.owner.sink.write_all(fragment).await
    }

    pub async fn finish(self) -> Result<ChunkId, PackError> {
        if self.written != self.payload_len {
            return Err(PackError::LimitExceeded("chunk ended before retained payload reservation"));
        }
        self.owner.sink.write_all(&self.segment_crc.finish().to_le_bytes()).await?;
        let id = ChunkId(self.owner.chunks.len() as u32);
        self.owner.chunks.push(ChunkTableEntry { offset: self.payload_offset, stored_len: self.payload_len as u64, raw_len: self.payload_len as u64, crc32: self.payload_crc.finish(), blake3: *self.hash.finalize().as_bytes() });
        Ok(id)
    }

    pub fn close(self) {}
}

impl<'a, S: PackSink> PackIdentitySegment<'a, S> {
    pub async fn write_fragment(&mut self, fragment: &[u8]) -> Result<(), PackError> {
        self.written = self.written.checked_add(fragment.len()).ok_or(PackError::LimitExceeded("segment payload length overflow"))?;
        if self.written > self.payload_len {
            return Err(PackError::LimitExceeded("segment exceeded retained payload reservation"));
        }
        self.crc.update_page(fragment);
        if self.kind == crate::KIND_DOCUMENT {
            self.owner.document_hasher.update(fragment);
        }
        self.owner.sink.write_all(fragment).await
    }

    pub async fn finish(self) -> Result<(), PackError> {
        if self.written != self.payload_len {
            return Err(PackError::LimitExceeded("segment ended before retained payload reservation"));
        }
        self.owner.sink.write_all(&self.crc.finish().to_le_bytes()).await
    }
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
        Ok(Self { sink, options: WriteOptions { required_flags, optional_flags: options.optional_flags, codec: options.codec }, chunks: Vec::new(), symbols: Vec::new(), symbols_span: None, document_hasher: semio_framework_hash::Hasher::new() })
    }

    /// @emoji 📍️ Current absolute write position — the offset the next segment/chunk will start
    /// at. Callers building a `Manifest` (e.g. `doc_span`/`field_index_span`) call this before
    /// and after their own `write_segment` calls to record spans this writer doesn't track
    /// automatically.
    pub async fn position(&self) -> u64 {
        self.sink.position().await
    }

    pub async fn begin_identity_segment(&mut self, kind: u8, payload_len: usize) -> Result<PackIdentitySegment<'_, S>, PackError> {
        let mut length = [0u8; 10];
        let mut remaining = payload_len as u64;
        let mut count = 0;
        loop {
            let mut byte = (remaining & 0x7f) as u8;
            remaining >>= 7;
            if remaining != 0 {
                byte |= 0x80;
            }
            length[count] = byte;
            count += 1;
            if remaining == 0 {
                break;
            }
        }
        let fixed = [kind, 0];
        let mut crc = crate::codec::Crc32cCursor::new();
        crc.update_page(&fixed);
        crc.update_page(&length[..count]);
        self.sink.write_all(&fixed).await?;
        self.sink.write_all(&length[..count]).await?;
        Ok(PackIdentitySegment { owner: self, kind, payload_len, written: 0, crc })
    }

    pub async fn begin_identity_chunk(&mut self, payload_len: usize) -> Result<PackIdentityChunk<'_, S>, PackError> {
        if self.options.codec.0 != 0 {
            return Err(PackError::UnsupportedCodec(self.options.codec.0));
        }
        let base = self.sink.position().await;
        let mut length = [0u8; 10];
        let mut remaining = payload_len as u64;
        let mut count = 0usize;
        loop {
            let mut byte = (remaining & 0x7f) as u8;
            remaining >>= 7;
            if remaining != 0 {
                byte |= 0x80;
            }
            length[count] = byte;
            count += 1;
            if remaining == 0 {
                break;
            }
        }
        let fixed = [crate::KIND_CHUNK, 0];
        let mut segment_crc = crate::codec::Crc32cCursor::new();
        segment_crc.update_page(&fixed);
        segment_crc.update_page(&length[..count]);
        self.sink.write_all(&fixed).await?;
        self.sink.write_all(&length[..count]).await?;
        Ok(PackIdentityChunk { owner: self, payload_offset: base + fixed.len() as u64 + count as u64, payload_len, written: 0, segment_crc, payload_crc: crate::codec::Crc32cCursor::new(), hash: semio_framework_hash::Hasher::new() })
    }

    /// @emoji 🖇️ Frames, compresses (per `options.codec`), CRCs, and writes one segment. A
    /// `KIND_SYMBOLS` segment is parsed and remembered for `schema_name` resolution in `finish`;
    /// a `KIND_DOCUMENT` segment's raw bytes are folded into the running content-hash used for
    /// the footer.
    pub async fn write_segment(&mut self, kind: u8, payload: &[u8]) -> Result<(), PackError> {
        if self.options.codec.0 == 0 && kind != crate::KIND_SYMBOLS {
            let mut segment = self.begin_identity_segment(kind, payload.len()).await?;
            segment.write_fragment(payload).await?;
            return segment.finish().await;
        }
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
    #[cfg(test)]
    pub async fn write_chunk(&mut self, payload: &[u8]) -> Result<ChunkId, PackError> {
        if self.options.codec.0 == 0 {
            let mut chunk = self.begin_identity_chunk(payload.len()).await?;
            chunk.write_fragment(payload).await?;
            return chunk.finish().await;
        }
        let base = self.sink.position().await;
        let encoded = encode_segment(crate::KIND_CHUNK, self.options.codec, payload).await?;
        let payload_offset = base + encoded.header_len as u64;
        let stored_bytes = &encoded.bytes[encoded.header_len..encoded.header_len + encoded.stored_len];
        let stored_crc = crc32c(stored_bytes);
        let raw_hash = semio_framework_hash::hash(payload);
        self.sink.write_all(&encoded.bytes).await?;
        let id = ChunkId(self.chunks.len() as u32);
        self.chunks.push(ChunkTableEntry { offset: payload_offset, stored_len: encoded.stored_len as u64, raw_len: payload.len() as u64, crc32: stored_crc, blake3: *raw_hash.as_bytes() });
        Ok(id)
    }

    /// @emoji 🏁️ Writes the chunk table (if any chunks were written), the manifest, an `End`
    /// segment, then the footer — and returns the underlying sink.
    pub async fn finish(mut self, manifest: &Manifest) -> Result<S, PackError> {
        let chunk_count = self.chunks.len() as u64;
        let mut chunk_table_span = ByteRange { offset: 0, len: 0 };
        if !self.chunks.is_empty() {
            let base = self.sink.position().await;
            let chunks = std::mem::take(&mut self.chunks);
            let payload_len = chunks.iter().try_fold(retained_varint_len(chunks.len() as u64), |length, entry| {
                length
                    .checked_add(retained_varint_len(entry.offset))
                    .and_then(|length| length.checked_add(retained_varint_len(entry.stored_len)))
                    .and_then(|length| length.checked_add(retained_varint_len(entry.raw_len)))
                    .and_then(|length| length.checked_add(4 + 32))
                    .ok_or(PackError::LimitExceeded("chunk table retained length overflow"))
            })?;
            let mut segment = self.begin_identity_segment(crate::KIND_CHUNK_TABLE, payload_len).await?;
            let mut varint = [0u8; 10];
            segment.write_fragment(retained_varint(chunks.len() as u64, &mut varint)).await?;
            for entry in chunks {
                segment.write_fragment(retained_varint(entry.offset, &mut varint)).await?;
                segment.write_fragment(retained_varint(entry.stored_len, &mut varint)).await?;
                segment.write_fragment(retained_varint(entry.raw_len, &mut varint)).await?;
                segment.write_fragment(&entry.crc32.to_le_bytes()).await?;
                segment.write_fragment(&entry.blake3).await?;
            }
            segment.finish().await?;
            chunk_table_span = ByteRange { offset: base, len: self.sink.position().await - base };
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
            chunk_count,
            symbol_count: self.symbols.len() as u64,
        };
        let manifest_bytes = encode_manifest_bytes(schema_symref, &final_manifest).await;
        let manifest_base = self.sink.position().await;
        self.write_segment(crate::KIND_MANIFEST, &manifest_bytes).await?;
        let manifest_span = ByteRange { offset: manifest_base, len: self.sink.position().await - manifest_base };

        let mut end = self.begin_identity_segment(crate::KIND_END, 0).await?;
        end.write_fragment(&[]).await?;
        end.finish().await?;

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

/// @emoji 🧩️ Retained identity-chunk reader that advances by one caller-owned fragment.
pub struct PackIdentityChunkCursor<'file, S: PackSource> {
    source: &'file S,
    entry: ChunkTableEntry,
    verification: VerificationLevel,
    offset: u64,
    crc: crate::codec::Crc32cCursor,
    hash: semio_framework_hash::Hasher,
    terminal: bool,
}

impl<'file, S: PackSource> PackIdentityChunkCursor<'file, S> {
    pub fn len(&self) -> u64 {
        self.entry.raw_len
    }

    pub fn remaining(&self) -> u64 {
        self.entry.raw_len.saturating_sub(self.offset)
    }

    pub async fn read_fragment(&mut self, target: &mut [u8]) -> Result<usize, PackError> {
        if self.terminal {
            return Ok(0);
        }
        let count = usize::try_from(self.remaining().min(target.len() as u64)).map_err(|_| PackError::LimitExceeded("identity chunk fragment length"))?;
        if count == 0 {
            if self.verification.checks_crc() && self.crc.finish() != self.entry.crc32 {
                return Err(PackError::ChecksumMismatch { segment: "chunk", offset: self.entry.offset });
            }
            if self.verification.checks_content_hash() && self.hash.finalize().as_bytes() != &self.entry.blake3 {
                return Err(PackError::ContentHashMismatch);
            }
            self.terminal = true;
            return Ok(0);
        }
        self.source.read_exact_at(self.entry.offset + self.offset, &mut target[..count]).await?;
        self.crc.update_page(&target[..count]);
        self.hash.update(&target[..count]);
        self.offset += count as u64;
        Ok(count)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
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

    /// @emoji 🧩️ Opens an identity chunk without allocating or materializing its payload.
    pub fn identity_chunk_cursor(&self, id: ChunkId, verification: VerificationLevel) -> Result<PackIdentityChunkCursor<'_, S>, PackError> {
        let entry = self.chunk_table.get(id.0 as usize).cloned().ok_or(PackError::Malformed { what: "chunk_id", offset: id.0 as u64, detail: "unknown chunk id".to_string() })?;
        if entry.stored_len != entry.raw_len {
            return Err(PackError::Malformed { what: "chunk", offset: entry.offset, detail: "retained fragment cursor requires an identity chunk".to_string() });
        }
        if entry.raw_len > self.limits.max_segment_len {
            return Err(PackError::LimitExceeded("chunk length exceeds max_segment_len"));
        }
        Ok(PackIdentityChunkCursor { source: &self.source, entry, verification, offset: 0, crc: crate::codec::Crc32cCursor::new(), hash: semio_framework_hash::Hasher::new(), terminal: false })
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
            let computed = crc32c(&stored);
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
            let hash = semio_framework_hash::hash(&raw);
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
            let hash = semio_framework_hash::hash(&out);
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

//#region 🔖️RetainedCanonicalSource
pub const RETAINED_PACK_PAGE_BYTES: usize = 4_096;

#[derive(Debug)]
pub struct RetainedPackPage {
    bytes: [u8; RETAINED_PACK_PAGE_BYTES],
    len: usize,
}

impl RetainedPackPage {
    pub fn try_from_array(bytes: [u8; RETAINED_PACK_PAGE_BYTES], len: usize) -> Result<Self, [u8; RETAINED_PACK_PAGE_BYTES]> {
        if len == 0 || len > RETAINED_PACK_PAGE_BYTES {
            return Err(bytes);
        }
        Ok(Self { bytes, len })
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedPackSourceEvent {
    Byte { offset: u64, value: u8 },
    Complete { bytes: u64, pages: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackSourceProgress {
    pub admitted_pages: usize,
    pub admitted_bytes: usize,
    pub consumed_bytes: u64,
    pub sealed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedPackCloseStep {
    Pending { released_items: usize, released_bytes: usize },
    Complete,
}

pub struct RetainedPackSourceCursor {
    pages: std::mem::ManuallyDrop<Vec<RetainedPackPage>>,
    maximum_pages: usize,
    maximum_bytes: usize,
    admitted_bytes: usize,
    page: usize,
    byte: usize,
    consumed: u64,
    sealed: bool,
    cancelled: bool,
    completed: bool,
    closed: bool,
}

impl RetainedPackSourceCursor {
    pub fn try_new(maximum_pages: usize, maximum_bytes: usize) -> Result<Self, &'static str> {
        if maximum_pages == 0 || maximum_bytes == 0 {
            return Err("retained-pack.zero-credits");
        }
        if maximum_bytes > maximum_pages.checked_mul(RETAINED_PACK_PAGE_BYTES).ok_or("retained-pack.credit-overflow")? {
            return Err("retained-pack.byte-credits");
        }
        let mut pages = Vec::new();
        pages.try_reserve_exact(maximum_pages).map_err(|_| "retained-pack.page-reservation")?;
        if pages.capacity() < maximum_pages {
            return Err("retained-pack.page-capacity");
        }
        Ok(Self { pages: std::mem::ManuallyDrop::new(pages), maximum_pages, maximum_bytes, admitted_bytes: 0, page: 0, byte: 0, consumed: 0, sealed: false, cancelled: false, completed: false, closed: false })
    }

    pub fn preflight_page(&self, len: usize) -> Result<(), &'static str> {
        if self.sealed || self.cancelled || self.closed {
            return Err("retained-pack.source-closed");
        }
        let pages = self.pages.len().checked_add(1).ok_or("retained-pack.page-overflow")?;
        let bytes = self.admitted_bytes.checked_add(len).ok_or("retained-pack.byte-overflow")?;
        if len == 0 || len > RETAINED_PACK_PAGE_BYTES || pages > self.maximum_pages || bytes > self.maximum_bytes {
            return Err("retained-pack.producer-handback");
        }
        Ok(())
    }

    pub fn admit_page(&mut self, page: RetainedPackPage) -> Result<(), RetainedPackPage> {
        if self.preflight_page(page.len()).is_err() {
            return Err(page);
        }
        self.admitted_bytes += page.len();
        self.pages.push(page);
        Ok(())
    }

    pub fn seal(&mut self) -> Result<(), &'static str> {
        if self.cancelled || self.closed {
            return Err("retained-pack.source-closed");
        }
        self.sealed = true;
        Ok(())
    }

    pub fn request_cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn progress(&self) -> RetainedPackSourceProgress {
        RetainedPackSourceProgress { admitted_pages: self.pages.len(), admitted_bytes: self.admitted_bytes, consumed_bytes: self.consumed, sealed: self.sealed }
    }

    pub fn grant(&mut self) -> Result<Option<RetainedPackSourceEvent>, &'static str> {
        if self.cancelled {
            return Err("retained-pack.cancelled");
        }
        if !self.sealed {
            return Ok(None);
        }
        if self.completed {
            return Ok(Some(RetainedPackSourceEvent::Complete { bytes: self.consumed, pages: self.pages.len() }));
        }
        while self.page < self.pages.len() && self.byte == self.pages[self.page].len {
            self.page += 1;
            self.byte = 0;
        }
        if self.page == self.pages.len() {
            self.completed = true;
            return Ok(Some(RetainedPackSourceEvent::Complete { bytes: self.consumed, pages: self.pages.len() }));
        }
        let value = self.pages[self.page].bytes[self.byte];
        let offset = self.consumed;
        self.byte += 1;
        self.consumed += 1;
        Ok(Some(RetainedPackSourceEvent::Byte { offset, value }))
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<RetainedPackCloseStep, &'static str> {
        self.cancelled = true;
        if maximum_items == 0 || maximum_bytes < RETAINED_PACK_PAGE_BYTES {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.pages.pop().is_some() {
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: RETAINED_PACK_PAGE_BYTES });
        }
        self.closed = true;
        Ok(RetainedPackCloseStep::Complete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed && self.pages.is_empty()
    }
}

impl Drop for RetainedPackSourceCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained canonical pack source reached Drop before terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackSegmentHeader {
    pub offset: u64,
    pub kind: u8,
    pub flags: u8,
    pub stored_len: u64,
    pub raw_len: u64,
    pub payload_offset: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedPackSegmentEvent {
    Begin(RetainedPackSegmentHeader),
    RawByte { segment: RetainedPackSegmentHeader, index: u64, value: u8 },
    Complete { segment: RetainedPackSegmentHeader, wire_len: u64 },
    PackComplete { bytes: u64, segments: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedVarintStep {
    Pending,
    Complete(u64),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct RetainedVarintCursor {
    value: u64,
    bytes: u8,
}

impl RetainedVarintCursor {
    fn admit(&mut self, byte: u8, offset: u64) -> Result<RetainedVarintStep, PackError> {
        if self.bytes >= 10 || (self.bytes == 9 && ((byte & 0x80) != 0 || byte & 0x7f > 1)) {
            return Err(PackError::Malformed { what: "varint", offset: offset - self.bytes as u64, detail: "overlong retained varint".into() });
        }
        let payload = (byte & 0x7f) as u64;
        self.value |= payload << (self.bytes as u32 * 7);
        self.bytes += 1;
        if byte & 0x80 != 0 {
            return Ok(RetainedVarintStep::Pending);
        }
        if self.bytes > 1 && payload == 0 {
            return Err(PackError::NonCanonical("non-minimal retained varint"));
        }
        Ok(RetainedVarintStep::Complete(self.value))
    }
}

#[derive(Debug)]
enum RetainedPackSegmentPhase {
    Header(usize),
    Kind,
    Flags,
    StoredLen(RetainedVarintCursor),
    RawLen(RetainedVarintCursor),
    Begin,
    Payload,
    Crc(usize),
    Trailer,
    Complete,
    Closed,
}

pub struct RetainedPackSegmentCursor {
    limits: PackLimits,
    pending: Option<RetainedPackSourceEvent>,
    phase: RetainedPackSegmentPhase,
    segment: RetainedPackSegmentHeader,
    payload_seen: u64,
    raw_seen: u64,
    crc: crate::codec::Crc32cCursor,
    stored_crc: [u8; 4],
    segments: u64,
    total: u64,
    trailer_seen: usize,
    #[cfg(feature = "deflate")]
    inflater: Option<crate::codec::DeflateRetainedCursor>,
    closed: bool,
}

impl RetainedPackSegmentCursor {
    pub fn try_new(limits: PackLimits) -> Result<Self, PackError> {
        if limits.max_segment_len == 0 || limits.max_file_len < (HEADER_SIZE + FOOTER_SIZE) as u64 {
            return Err(PackError::LimitExceeded("retained pack limits"));
        }
        Ok(Self {
            limits,
            pending: None,
            phase: RetainedPackSegmentPhase::Header(0),
            segment: RetainedPackSegmentHeader { offset: 0, kind: 0, flags: 0, stored_len: 0, raw_len: 0, payload_offset: 0 },
            payload_seen: 0,
            raw_seen: 0,
            crc: crate::codec::Crc32cCursor::new(),
            stored_crc: [0; 4],
            segments: 0,
            total: 0,
            trailer_seen: 0,
            #[cfg(feature = "deflate")]
            inflater: None,
            closed: false,
        })
    }

    pub fn preflight(&self) -> Result<(), &'static str> {
        if self.closed || self.pending.is_some() || matches!(self.phase, RetainedPackSegmentPhase::Complete | RetainedPackSegmentPhase::Closed) {
            return Err("retained-pack.segment-admission");
        }
        #[cfg(feature = "deflate")]
        if self.inflater.as_ref().is_some_and(|inflater| !inflater.can_admit()) {
            return Err("retained-pack.inflate-backpressure");
        }
        Ok(())
    }

    pub fn admit(&mut self, event: RetainedPackSourceEvent) -> Result<(), RetainedPackSourceEvent> {
        if self.preflight().is_err() {
            return Err(event);
        }
        self.pending = Some(event);
        Ok(())
    }

    fn take_byte(&mut self) -> Result<Option<(u64, u8)>, PackError> {
        match self.pending.take() {
            Some(RetainedPackSourceEvent::Byte { offset, value }) => {
                if offset != self.total || offset >= self.limits.max_file_len {
                    return Err(PackError::Malformed { what: "retained-segment", offset, detail: "non-contiguous or over-limit source".into() });
                }
                self.total += 1;
                Ok(Some((offset, value)))
            }
            Some(event @ RetainedPackSourceEvent::Complete { .. }) => {
                self.pending = Some(event);
                Ok(None)
            }
            None => Ok(None),
        }
    }

    fn begin_segment(&mut self) -> Result<(), PackError> {
        if self.segment.stored_len > self.limits.max_segment_len || self.segment.raw_len > self.limits.max_segment_len {
            return Err(PackError::LimitExceeded("segment length exceeds max_segment_len"));
        }
        if self.segment.kind == crate::KIND_END && (self.segment.stored_len != 0 || self.segment.raw_len != 0) {
            return Err(PackError::Malformed { what: "end-segment", offset: self.segment.offset, detail: "END payload must be empty".into() });
        }
        let codec = (self.segment.flags >> 1) & 0x07;
        if self.segment.flags & 0xf0 != 0 {
            return Err(PackError::Malformed { what: "segment", offset: self.segment.offset + 1, detail: "reserved segment flags are set".into() });
        }
        if self.segment.flags & 1 == 0 {
            if codec != 0 || self.segment.raw_len != self.segment.stored_len {
                return Err(PackError::Malformed { what: "segment", offset: self.segment.offset + 1, detail: "identity segment length or codec mismatch".into() });
            }
        } else if codec != 1 {
            return Err(PackError::UnsupportedCodec(codec));
        } else {
            #[cfg(feature = "deflate")]
            {
                self.inflater = Some(crate::codec::DeflateRetainedCursor::try_new(self.segment.raw_len, self.limits.max_segment_len)?);
            }
            #[cfg(not(feature = "deflate"))]
            return Err(PackError::UnsupportedCodec(codec));
        }
        Ok(())
    }

    pub fn grant(&mut self) -> Result<Option<RetainedPackSegmentEvent>, PackError> {
        match self.phase {
            RetainedPackSegmentPhase::Header(index) => {
                let Some((_, _)) = self.take_byte()? else { return Ok(None) };
                self.phase = if index + 1 == HEADER_SIZE { RetainedPackSegmentPhase::Kind } else { RetainedPackSegmentPhase::Header(index + 1) };
                Ok(None)
            }
            RetainedPackSegmentPhase::Kind => {
                let Some((offset, value)) = self.take_byte()? else { return Ok(None) };
                self.segment = RetainedPackSegmentHeader { offset, kind: value, flags: 0, stored_len: 0, raw_len: 0, payload_offset: 0 };
                self.payload_seen = 0;
                self.raw_seen = 0;
                self.crc = crate::codec::Crc32cCursor::new();
                self.crc.update_page(&[value]);
                self.phase = RetainedPackSegmentPhase::Flags;
                Ok(None)
            }
            RetainedPackSegmentPhase::Flags => {
                let Some((_, value)) = self.take_byte()? else { return Ok(None) };
                self.segment.flags = value;
                self.crc.update_page(&[value]);
                self.phase = RetainedPackSegmentPhase::StoredLen(RetainedVarintCursor::default());
                Ok(None)
            }
            RetainedPackSegmentPhase::StoredLen(mut cursor) => {
                let Some((offset, value)) = self.take_byte()? else { return Ok(None) };
                self.crc.update_page(&[value]);
                match cursor.admit(value, offset)? {
                    RetainedVarintStep::Pending => self.phase = RetainedPackSegmentPhase::StoredLen(cursor),
                    RetainedVarintStep::Complete(value) => {
                        self.segment.stored_len = value;
                        if self.segment.flags & 1 == 0 {
                            self.segment.raw_len = value;
                            self.segment.payload_offset = self.total;
                            self.phase = RetainedPackSegmentPhase::Begin;
                        } else {
                            self.phase = RetainedPackSegmentPhase::RawLen(RetainedVarintCursor::default());
                        }
                    }
                }
                Ok(None)
            }
            RetainedPackSegmentPhase::RawLen(mut cursor) => {
                let Some((offset, value)) = self.take_byte()? else { return Ok(None) };
                self.crc.update_page(&[value]);
                match cursor.admit(value, offset)? {
                    RetainedVarintStep::Pending => self.phase = RetainedPackSegmentPhase::RawLen(cursor),
                    RetainedVarintStep::Complete(value) => {
                        self.segment.raw_len = value;
                        self.segment.payload_offset = self.total;
                        self.phase = RetainedPackSegmentPhase::Begin;
                    }
                }
                Ok(None)
            }
            RetainedPackSegmentPhase::Begin => {
                self.begin_segment()?;
                self.phase = RetainedPackSegmentPhase::Payload;
                Ok(Some(RetainedPackSegmentEvent::Begin(self.segment)))
            }
            RetainedPackSegmentPhase::Payload => {
                if self.segment.flags & 1 == 0 {
                    if self.payload_seen == self.segment.stored_len {
                        self.phase = RetainedPackSegmentPhase::Crc(0);
                        return Ok(None);
                    }
                    let Some((_, value)) = self.take_byte()? else { return Ok(None) };
                    self.crc.update_page(&[value]);
                    let index = self.raw_seen;
                    self.payload_seen += 1;
                    self.raw_seen += 1;
                    return Ok(Some(RetainedPackSegmentEvent::RawByte { segment: self.segment, index, value }));
                }
                #[cfg(feature = "deflate")]
                {
                    if self.payload_seen < self.segment.stored_len && self.inflater.as_ref().expect("compressed segment has inflater").can_admit() {
                        let Some((_, value)) = self.take_byte()? else { return Ok(None) };
                        self.crc.update_page(&[value]);
                        self.inflater.as_mut().expect("compressed segment has inflater").admit_byte(value).expect("preflight established exact handback");
                        self.payload_seen += 1;
                    }
                    let inflater = self.inflater.as_mut().expect("compressed segment has inflater");
                    match inflater.grant(self.payload_seen == self.segment.stored_len)? {
                        crate::codec::DeflateRetainedStep::NeedInput => Ok(None),
                        crate::codec::DeflateRetainedStep::Byte(value) => {
                            let index = self.raw_seen;
                            self.raw_seen += 1;
                            Ok(Some(RetainedPackSegmentEvent::RawByte { segment: self.segment, index, value }))
                        }
                        crate::codec::DeflateRetainedStep::Complete => {
                            if self.raw_seen != self.segment.raw_len {
                                return Err(PackError::Malformed { what: "segment", offset: self.segment.payload_offset, detail: "raw length mismatch".into() });
                            }
                            inflater.close();
                            self.inflater = None;
                            self.phase = RetainedPackSegmentPhase::Crc(0);
                            Ok(None)
                        }
                    }
                }
                #[cfg(not(feature = "deflate"))]
                Err(PackError::UnsupportedCodec((self.segment.flags >> 1) & 0x07))
            }
            RetainedPackSegmentPhase::Crc(index) => {
                let Some((offset, value)) = self.take_byte()? else { return Ok(None) };
                self.stored_crc[index] = value;
                if index + 1 < 4 {
                    self.phase = RetainedPackSegmentPhase::Crc(index + 1);
                    return Ok(None);
                }
                if u32::from_le_bytes(self.stored_crc) != self.crc.finish() {
                    return Err(PackError::ChecksumMismatch { segment: "segment", offset });
                }
                let wire_len = self.total - self.segment.offset;
                self.segments += 1;
                self.phase = if self.segment.kind == crate::KIND_END { RetainedPackSegmentPhase::Trailer } else { RetainedPackSegmentPhase::Kind };
                Ok(Some(RetainedPackSegmentEvent::Complete { segment: self.segment, wire_len }))
            }
            RetainedPackSegmentPhase::Trailer => match self.pending.take() {
                Some(RetainedPackSourceEvent::Byte { offset, .. }) => {
                    if offset != self.total || self.total >= self.limits.max_file_len || self.trailer_seen == FOOTER_SIZE {
                        return Err(PackError::Malformed { what: "retained-footer", offset, detail: "non-contiguous trailer".into() });
                    }
                    self.total += 1;
                    self.trailer_seen += 1;
                    Ok(None)
                }
                Some(RetainedPackSourceEvent::Complete { bytes, .. }) => {
                    if bytes != self.total || self.trailer_seen != FOOTER_SIZE {
                        return Err(PackError::Truncated(self.total));
                    }
                    self.phase = RetainedPackSegmentPhase::Complete;
                    Ok(Some(RetainedPackSegmentEvent::PackComplete { bytes, segments: self.segments }))
                }
                None => Ok(None),
            },
            RetainedPackSegmentPhase::Complete => Ok(Some(RetainedPackSegmentEvent::PackComplete { bytes: self.total, segments: self.segments })),
            RetainedPackSegmentPhase::Closed => Err(PackError::Malformed { what: "retained-segment", offset: self.total, detail: "cursor is closed".into() }),
        }
    }

    pub fn close_step(&mut self) -> RetainedPackCloseStep {
        #[cfg(feature = "deflate")]
        if let Some(mut inflater) = self.inflater.take() {
            inflater.close();
        }
        self.pending = None;
        self.phase = RetainedPackSegmentPhase::Closed;
        self.closed = true;
        RetainedPackCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed && self.pending.is_none() && {
            #[cfg(feature = "deflate")]
            {
                self.inflater.is_none()
            }
            #[cfg(not(feature = "deflate"))]
            {
                true
            }
        }
    }
}

impl Drop for RetainedPackSegmentCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained segment cursor reached Drop before terminal-empty close");
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetainedPackChunkEntry {
    pub offset: u64,
    pub stored_len: u64,
    pub raw_len: u64,
    pub crc32: u32,
    pub blake3: [u8; 32],
}

#[derive(Debug)]
pub struct RetainedPackCatalog {
    pub manifest: Manifest,
    pub symbols: Vec<String>,
    pub chunks: Vec<RetainedPackChunkEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedPackCatalogEvent {
    DocumentByte { frame: u64, index: u64, value: u8 },
    SchemaByte { index: u64, value: u8 },
    FieldIndexByte { index: u64, value: u8 },
    Item { kind: u8, index: u64 },
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedManifestPhase {
    Varint(usize, RetainedVarintCursor),
    Hash(usize),
    Complete,
}

struct RetainedManifestCursor {
    values: [u64; 14],
    hash: [u8; 32],
    phase: RetainedManifestPhase,
}

impl RetainedManifestCursor {
    fn new() -> Self {
        Self { values: [0; 14], hash: [0; 32], phase: RetainedManifestPhase::Varint(0, RetainedVarintCursor::default()) }
    }

    fn admit(&mut self, byte: u8, offset: u64) -> Result<bool, PackError> {
        match self.phase {
            RetainedManifestPhase::Varint(field, mut cursor) => match cursor.admit(byte, offset)? {
                RetainedVarintStep::Pending => self.phase = RetainedManifestPhase::Varint(field, cursor),
                RetainedVarintStep::Complete(value) => {
                    self.values[field] = value;
                    self.phase = if field == 0 {
                        RetainedManifestPhase::Hash(0)
                    } else if field == 13 {
                        RetainedManifestPhase::Complete
                    } else {
                        RetainedManifestPhase::Varint(field + 1, RetainedVarintCursor::default())
                    };
                }
            },
            RetainedManifestPhase::Hash(index) => {
                self.hash[index] = byte;
                self.phase = if index + 1 == self.hash.len() { RetainedManifestPhase::Varint(1, RetainedVarintCursor::default()) } else { RetainedManifestPhase::Hash(index + 1) };
            }
            RetainedManifestPhase::Complete => {}
        }
        Ok(matches!(self.phase, RetainedManifestPhase::Complete))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct RetainedUtf8Cursor {
    value: u32,
    minimum: u32,
    remaining: u8,
}

impl RetainedUtf8Cursor {
    fn admit(&mut self, byte: u8, offset: u64) -> Result<Option<char>, PackError> {
        if self.remaining == 0 {
            match byte {
                0x00..=0x7f => return Ok(Some(byte as char)),
                0xc2..=0xdf => {
                    self.value = (byte & 0x1f) as u32;
                    self.minimum = 0x80;
                    self.remaining = 1;
                }
                0xe0..=0xef => {
                    self.value = (byte & 0x0f) as u32;
                    self.minimum = 0x800;
                    self.remaining = 2;
                }
                0xf0..=0xf4 => {
                    self.value = (byte & 0x07) as u32;
                    self.minimum = 0x10000;
                    self.remaining = 3;
                }
                _ => return Err(PackError::Malformed { what: "utf8", offset, detail: "invalid leading byte".into() }),
            }
            return Ok(None);
        }
        if byte & 0xc0 != 0x80 {
            return Err(PackError::Malformed { what: "utf8", offset, detail: "invalid continuation byte".into() });
        }
        self.value = (self.value << 6) | (byte & 0x3f) as u32;
        self.remaining -= 1;
        if self.remaining != 0 {
            return Ok(None);
        }
        let value = self.value;
        if value < self.minimum || (0xd800..=0xdfff).contains(&value) || value > 0x10ffff {
            return Err(PackError::Malformed { what: "utf8", offset, detail: "non-scalar or overlong codepoint".into() });
        }
        char::from_u32(value).map(Some).ok_or(PackError::Malformed { what: "utf8", offset, detail: "invalid scalar".into() })
    }

    fn complete(&self) -> bool {
        self.remaining == 0
    }
}

#[derive(Debug)]
enum RetainedSymbolsPhase {
    Count(RetainedVarintCursor),
    Length(RetainedVarintCursor),
    Text { remaining: u64 },
    Complete,
}

struct RetainedSymbolsCursor {
    phase: RetainedSymbolsPhase,
    expected: usize,
    maximum: usize,
    maximum_string_bytes: usize,
    current: String,
    utf8: RetainedUtf8Cursor,
}

impl RetainedSymbolsCursor {
    fn new(maximum: usize, maximum_string_bytes: usize) -> Self {
        Self { phase: RetainedSymbolsPhase::Count(RetainedVarintCursor::default()), expected: 0, maximum, maximum_string_bytes, current: String::new(), utf8: RetainedUtf8Cursor::default() }
    }

    fn admit(&mut self, byte: u8, offset: u64, symbols: &mut Vec<String>) -> Result<Option<u64>, PackError> {
        match self.phase {
            RetainedSymbolsPhase::Count(mut cursor) => match cursor.admit(byte, offset)? {
                RetainedVarintStep::Pending => self.phase = RetainedSymbolsPhase::Count(cursor),
                RetainedVarintStep::Complete(count) => {
                    self.expected = usize::try_from(count).map_err(|_| PackError::LimitExceeded("symbol count"))?;
                    if self.expected > self.maximum {
                        return Err(PackError::LimitExceeded("symbol count exceeds retained registry"));
                    }
                    self.phase = if self.expected == 0 { RetainedSymbolsPhase::Complete } else { RetainedSymbolsPhase::Length(RetainedVarintCursor::default()) };
                }
            },
            RetainedSymbolsPhase::Length(mut cursor) => match cursor.admit(byte, offset)? {
                RetainedVarintStep::Pending => self.phase = RetainedSymbolsPhase::Length(cursor),
                RetainedVarintStep::Complete(len) => {
                    let len = usize::try_from(len).map_err(|_| PackError::LimitExceeded("symbol length"))?;
                    if len > self.maximum_string_bytes {
                        return Err(PackError::LimitExceeded("symbol exceeds retained string bytes"));
                    }
                    self.current.try_reserve_exact(len).map_err(|_| PackError::LimitExceeded("symbol reservation"))?;
                    self.phase = if len == 0 { RetainedSymbolsPhase::Text { remaining: 0 } } else { RetainedSymbolsPhase::Text { remaining: len as u64 } };
                }
            },
            RetainedSymbolsPhase::Text { remaining } => {
                if remaining == 0 {
                    return Err(PackError::Malformed { what: "symbols", offset, detail: "zero-length symbol must complete without a byte".into() });
                }
                if let Some(character) = self.utf8.admit(byte, offset)? {
                    self.current.push(character);
                }
                let remaining = remaining - 1;
                if remaining == 0 {
                    if !self.utf8.complete() {
                        return Err(PackError::Malformed { what: "utf8", offset, detail: "truncated codepoint".into() });
                    }
                    symbols.push(std::mem::take(&mut self.current));
                    let index = symbols.len() as u64 - 1;
                    self.phase = if symbols.len() == self.expected { RetainedSymbolsPhase::Complete } else { RetainedSymbolsPhase::Length(RetainedVarintCursor::default()) };
                    return Ok(Some(index));
                }
                self.phase = RetainedSymbolsPhase::Text { remaining };
            }
            RetainedSymbolsPhase::Complete => return Err(PackError::Malformed { what: "symbols", offset, detail: "trailing symbol bytes".into() }),
        }
        if matches!(self.phase, RetainedSymbolsPhase::Text { remaining: 0 }) {
            symbols.push(String::new());
            let index = symbols.len() as u64 - 1;
            self.phase = if symbols.len() == self.expected { RetainedSymbolsPhase::Complete } else { RetainedSymbolsPhase::Length(RetainedVarintCursor::default()) };
            return Ok(Some(index));
        }
        Ok(None)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedChunksPhase {
    Count(RetainedVarintCursor),
    Varint(usize, RetainedVarintCursor),
    Crc(usize),
    Hash(usize),
    Complete,
}

struct RetainedChunksCursor {
    phase: RetainedChunksPhase,
    expected: usize,
    maximum: usize,
    values: [u64; 3],
    crc: [u8; 4],
    hash: [u8; 32],
}

impl RetainedChunksCursor {
    fn new(maximum: usize) -> Self {
        Self { phase: RetainedChunksPhase::Count(RetainedVarintCursor::default()), expected: 0, maximum, values: [0; 3], crc: [0; 4], hash: [0; 32] }
    }

    fn admit(&mut self, byte: u8, offset: u64, limits: &PackLimits, chunks: &mut Vec<RetainedPackChunkEntry>) -> Result<Option<u64>, PackError> {
        match self.phase {
            RetainedChunksPhase::Count(mut cursor) => match cursor.admit(byte, offset)? {
                RetainedVarintStep::Pending => self.phase = RetainedChunksPhase::Count(cursor),
                RetainedVarintStep::Complete(count) => {
                    self.expected = usize::try_from(count).map_err(|_| PackError::LimitExceeded("chunk count"))?;
                    if self.expected > self.maximum {
                        return Err(PackError::LimitExceeded("chunk count exceeds retained registry"));
                    }
                    self.phase = if self.expected == 0 { RetainedChunksPhase::Complete } else { RetainedChunksPhase::Varint(0, RetainedVarintCursor::default()) };
                }
            },
            RetainedChunksPhase::Varint(field, mut cursor) => match cursor.admit(byte, offset)? {
                RetainedVarintStep::Pending => self.phase = RetainedChunksPhase::Varint(field, cursor),
                RetainedVarintStep::Complete(value) => {
                    self.values[field] = value;
                    if field == 2 {
                        if self.values[1] > limits.max_segment_len || self.values[2] > limits.max_segment_len {
                            return Err(PackError::LimitExceeded("chunk entry length"));
                        }
                        self.phase = RetainedChunksPhase::Crc(0);
                    } else {
                        self.phase = RetainedChunksPhase::Varint(field + 1, RetainedVarintCursor::default());
                    }
                }
            },
            RetainedChunksPhase::Crc(index) => {
                self.crc[index] = byte;
                self.phase = if index + 1 == 4 { RetainedChunksPhase::Hash(0) } else { RetainedChunksPhase::Crc(index + 1) };
            }
            RetainedChunksPhase::Hash(index) => {
                self.hash[index] = byte;
                if index + 1 == 32 {
                    chunks.push(RetainedPackChunkEntry { offset: self.values[0], stored_len: self.values[1], raw_len: self.values[2], crc32: u32::from_le_bytes(self.crc), blake3: self.hash });
                    let item = chunks.len() as u64 - 1;
                    self.phase = if chunks.len() == self.expected { RetainedChunksPhase::Complete } else { RetainedChunksPhase::Varint(0, RetainedVarintCursor::default()) };
                    return Ok(Some(item));
                }
                self.phase = RetainedChunksPhase::Hash(index + 1);
            }
            RetainedChunksPhase::Complete => return Err(PackError::Malformed { what: "chunk-table", offset, detail: "trailing chunk bytes".into() }),
        }
        Ok(None)
    }
}

pub struct RetainedPackCatalogCursor {
    limits: PackLimits,
    symbols: Vec<String>,
    chunks: Vec<RetainedPackChunkEntry>,
    observed_chunks: Vec<RetainedPackSegmentHeader>,
    manifest: RetainedManifestCursor,
    symbol_parser: RetainedSymbolsCursor,
    chunk_parser: RetainedChunksCursor,
    pending: Option<RetainedPackSegmentEvent>,
    active: Option<RetainedPackSegmentHeader>,
    manifest_span: Option<ByteRange>,
    symbols_span: Option<ByteRange>,
    chunks_span: Option<ByteRange>,
    document_frames: u64,
    document_bytes: u64,
    document_span: Option<ByteRange>,
    document_hash: semio_framework_hash::Hasher,
    schema_bytes: u64,
    field_index_bytes: u64,
    complete: bool,
    handed_back: bool,
    closed: bool,
}

impl RetainedPackCatalogCursor {
    pub fn try_new(limits: PackLimits, maximum_symbols: usize, maximum_chunks: usize, maximum_string_bytes: usize) -> Result<Self, PackError> {
        if maximum_symbols > limits.max_symbols as usize || maximum_chunks as u64 > limits.max_items || maximum_string_bytes as u64 > limits.max_total_alloc {
            return Err(PackError::LimitExceeded("retained catalog credits"));
        }
        let mut symbols = Vec::new();
        symbols.try_reserve_exact(maximum_symbols).map_err(|_| PackError::LimitExceeded("symbol registry reservation"))?;
        let mut chunks = Vec::new();
        chunks.try_reserve_exact(maximum_chunks).map_err(|_| PackError::LimitExceeded("chunk registry reservation"))?;
        let mut observed_chunks = Vec::new();
        observed_chunks.try_reserve_exact(maximum_chunks).map_err(|_| PackError::LimitExceeded("chunk segment registry reservation"))?;
        Ok(Self {
            limits,
            symbols,
            chunks,
            observed_chunks,
            manifest: RetainedManifestCursor::new(),
            symbol_parser: RetainedSymbolsCursor::new(maximum_symbols, maximum_string_bytes),
            chunk_parser: RetainedChunksCursor::new(maximum_chunks),
            pending: None,
            active: None,
            manifest_span: None,
            symbols_span: None,
            chunks_span: None,
            document_frames: 0,
            document_bytes: 0,
            document_span: None,
            document_hash: semio_framework_hash::Hasher::new(),
            schema_bytes: 0,
            field_index_bytes: 0,
            complete: false,
            handed_back: false,
            closed: false,
        })
    }

    pub fn admit(&mut self, event: RetainedPackSegmentEvent) -> Result<(), RetainedPackSegmentEvent> {
        if self.closed || self.complete || self.pending.is_some() {
            return Err(event);
        }
        self.pending = Some(event);
        Ok(())
    }

    pub fn grant(&mut self) -> Result<Option<RetainedPackCatalogEvent>, PackError> {
        let Some(event) = self.pending.take() else { return Ok(None) };
        match event {
            RetainedPackSegmentEvent::Begin(segment) => {
                if self.active.is_some() {
                    return Err(PackError::Malformed { what: "retained-catalog", offset: segment.offset, detail: "nested segment begin".into() });
                }
                self.active = Some(segment);
                if segment.kind == crate::KIND_DOCUMENT {
                    self.document_frames += 1;
                }
                Ok(None)
            }
            RetainedPackSegmentEvent::RawByte { segment, index, value } => {
                if self.active != Some(segment) {
                    return Err(PackError::Malformed { what: "retained-catalog", offset: segment.offset, detail: "payload without active segment".into() });
                }
                match segment.kind {
                    crate::KIND_MANIFEST => {
                        self.manifest.admit(value, segment.payload_offset + index)?;
                        Ok(None)
                    }
                    crate::KIND_SYMBOLS => Ok(self.symbol_parser.admit(value, segment.payload_offset + index, &mut self.symbols)?.map(|index| RetainedPackCatalogEvent::Item { kind: crate::KIND_SYMBOLS, index })),
                    crate::KIND_CHUNK_TABLE => {
                        let item = self.chunk_parser.admit(value, segment.payload_offset + index, &self.limits, &mut self.chunks)?;
                        if let Some(index) = item {
                            let entry = &self.chunks[index as usize];
                            let observed = self.observed_chunks.get(index as usize).ok_or(PackError::Malformed { what: "chunk-table", offset: segment.payload_offset + index, detail: "entry has no observed chunk segment".into() })?;
                            if entry.offset != observed.payload_offset || entry.stored_len != observed.stored_len || entry.raw_len != observed.raw_len {
                                return Err(PackError::Malformed { what: "chunk-table", offset: segment.payload_offset + index, detail: "entry does not match observed chunk framing".into() });
                            }
                            return Ok(Some(RetainedPackCatalogEvent::Item { kind: crate::KIND_CHUNK_TABLE, index }));
                        }
                        Ok(None)
                    }
                    crate::KIND_DOCUMENT => {
                        let body_index = self.document_bytes;
                        self.document_bytes += 1;
                        self.document_hash.update(&[value]);
                        Ok(Some(RetainedPackCatalogEvent::DocumentByte { frame: self.document_frames - 1, index: body_index, value }))
                    }
                    crate::KIND_CHUNK => {
                        if self.observed_chunks.len() == self.observed_chunks.capacity() {
                            return Err(PackError::LimitExceeded("observed chunk segment registry"));
                        }
                        self.observed_chunks.push(segment);
                        Ok(None)
                    }
                    crate::KIND_SCHEMA => {
                        let schema_index = self.schema_bytes;
                        self.schema_bytes += 1;
                        Ok(Some(RetainedPackCatalogEvent::SchemaByte { index: schema_index, value }))
                    }
                    crate::KIND_FIELD_INDEX => {
                        let field_index = self.field_index_bytes;
                        self.field_index_bytes += 1;
                        Ok(Some(RetainedPackCatalogEvent::FieldIndexByte { index: field_index, value }))
                    }
                    _ => Ok(None),
                }
            }
            RetainedPackSegmentEvent::Complete { segment, wire_len } => {
                if self.active.take() != Some(segment) {
                    return Err(PackError::Malformed { what: "retained-catalog", offset: segment.offset, detail: "segment completion mismatch".into() });
                }
                let span = ByteRange { offset: segment.offset, len: wire_len };
                match segment.kind {
                    crate::KIND_MANIFEST if self.manifest_span.replace(span).is_some() => return Err(PackError::Malformed { what: "manifest", offset: segment.offset, detail: "duplicate manifest".into() }),
                    crate::KIND_SYMBOLS if self.symbols_span.replace(span).is_some() => return Err(PackError::Malformed { what: "symbols", offset: segment.offset, detail: "duplicate symbols".into() }),
                    crate::KIND_CHUNK_TABLE if self.chunks_span.replace(span).is_some() => return Err(PackError::Malformed { what: "chunk-table", offset: segment.offset, detail: "duplicate chunk table".into() }),
                    crate::KIND_DOCUMENT => {
                        self.document_span = Some(match self.document_span {
                            None => span,
                            Some(existing) if existing.offset + existing.len == span.offset => ByteRange { offset: existing.offset, len: existing.len + span.len },
                            Some(_) => return Err(PackError::Malformed { what: "document", offset: segment.offset, detail: "document frames are not contiguous".into() }),
                        });
                    }
                    _ => {}
                }
                Ok(Some(RetainedPackCatalogEvent::Item { kind: segment.kind, index: 0 }))
            }
            RetainedPackSegmentEvent::PackComplete { .. } => {
                if self.active.is_some()
                    || !matches!(self.manifest.phase, RetainedManifestPhase::Complete)
                    || (self.symbols_span.is_some() && !matches!(self.symbol_parser.phase, RetainedSymbolsPhase::Complete))
                    || (self.chunks_span.is_some() && !matches!(self.chunk_parser.phase, RetainedChunksPhase::Complete))
                {
                    return Err(PackError::Truncated(self.document_bytes));
                }
                self.complete = true;
                Ok(Some(RetainedPackCatalogEvent::Complete))
            }
        }
    }

    /// 🔤️ Borrows one already-verified symbol scalar without cloning the retained registry.
    /// Mounted typed owners advance `character` by one only after their own grant is admitted.
    pub fn symbol_char(&self, symbol: u64, character: usize) -> Result<Option<char>, PackError> {
        let value = self.symbols.get(symbol as usize).ok_or(PackError::Malformed { what: "retained-symref", offset: symbol, detail: "symbol is outside the fixed retained registry".into() })?;
        Ok(value.chars().nth(character))
    }

    /// 📏️ Returns the scalar count of one retained symbol without materializing a copy.
    pub fn symbol_chars(&self, symbol: u64) -> Result<usize, PackError> {
        let value = self.symbols.get(symbol as usize).ok_or(PackError::Malformed { what: "retained-symref", offset: symbol, detail: "symbol is outside the fixed retained registry".into() })?;
        Ok(value.chars().count())
    }

    /// 📏️ Number of canonical document-body bytes already admitted to the value VM.
    pub fn document_bytes(&self) -> u64 {
        self.document_bytes
    }

    pub fn take(&mut self, superblock: Superblock) -> Result<Option<RetainedPackCatalog>, PackError> {
        if !self.complete || self.handed_back {
            return Ok(None);
        }
        let values = self.manifest.values;
        let raw = RawManifest {
            schema_symref: values[0],
            schema_hash: self.manifest.hash,
            doc_span: ByteRange { offset: values[1], len: values[2] },
            doc_frame_count: values[3],
            symbols_span: ByteRange { offset: values[4], len: values[5] },
            chunk_table_span: ByteRange { offset: values[6], len: values[7] },
            field_index_span: ByteRange { offset: values[8], len: values[9] },
            uncompressed_body_len: values[10],
            field_count: values[11],
            chunk_count: values[12],
            symbol_count: values[13],
        };
        if self.manifest_span != Some(ByteRange { offset: superblock.footer.manifest_offset, len: superblock.footer.manifest_len })
            || self.symbols_span.unwrap_or(ByteRange { offset: 0, len: 0 }) != raw.symbols_span
            || self.chunks_span.unwrap_or(ByteRange { offset: 0, len: 0 }) != raw.chunk_table_span
        {
            return Err(PackError::Malformed { what: "manifest", offset: superblock.footer.manifest_offset, detail: "retained segment span mismatch".into() });
        }
        if self.document_span.unwrap_or(ByteRange { offset: 0, len: 0 }) != raw.doc_span
            || raw.doc_frame_count != self.document_frames
            || raw.uncompressed_body_len != self.document_bytes
            || raw.symbol_count != self.symbols.len() as u64
            || raw.chunk_count != self.chunks.len() as u64
            || self.observed_chunks.len() != self.chunks.len()
        {
            return Err(PackError::Malformed { what: "manifest", offset: superblock.footer.manifest_offset, detail: "retained catalog count mismatch".into() });
        }
        if self.document_hash.finalize().as_bytes() != &superblock.footer.content_hash.0 {
            return Err(PackError::ContentHashMismatch);
        }
        let manifest = resolve_manifest(&raw, &self.symbols)?;
        let symbols = std::mem::take(&mut self.symbols);
        let chunks = std::mem::take(&mut self.chunks);
        self.observed_chunks.clear();
        self.handed_back = true;
        Ok(Some(RetainedPackCatalog { manifest, symbols, chunks }))
    }

    pub fn close_step(&mut self, maximum_items: usize) -> RetainedPackCloseStep {
        if maximum_items == 0 {
            return RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.symbols.pop().is_some() || self.chunks.pop().is_some() || self.observed_chunks.pop().is_some() {
            return RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        self.pending = None;
        self.closed = true;
        self.handed_back = true;
        RetainedPackCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.handed_back && self.symbols.is_empty() && self.chunks.is_empty() && self.observed_chunks.is_empty() && self.pending.is_none()
    }
}

impl Drop for RetainedPackCatalogCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained pack catalog reached Drop before handback or terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedPackAnchorPhase {
    Collect,
    VerifyHeader(usize),
    VerifyFooter(usize),
    Ready,
    Closed,
}

pub struct RetainedPackAnchorCursor {
    header: [u8; HEADER_SIZE],
    header_len: usize,
    footer_ring: [u8; FOOTER_SIZE],
    footer_len: usize,
    footer_head: usize,
    total: u64,
    ordered_footer: [u8; FOOTER_SIZE],
    header_crc: crate::codec::Crc32cCursor,
    footer_crc: crate::codec::Crc32cCursor,
    phase: RetainedPackAnchorPhase,
    value: Option<Superblock>,
    handed_back: bool,
}

impl RetainedPackAnchorCursor {
    pub fn new() -> Self {
        Self {
            header: [0; HEADER_SIZE],
            header_len: 0,
            footer_ring: [0; FOOTER_SIZE],
            footer_len: 0,
            footer_head: 0,
            total: 0,
            ordered_footer: [0; FOOTER_SIZE],
            header_crc: crate::codec::Crc32cCursor::new(),
            footer_crc: crate::codec::Crc32cCursor::new(),
            phase: RetainedPackAnchorPhase::Collect,
            value: None,
            handed_back: false,
        }
    }

    pub fn grant(&mut self, event: Option<RetainedPackSourceEvent>) -> Result<bool, PackError> {
        match self.phase {
            RetainedPackAnchorPhase::Collect => match event {
                Some(RetainedPackSourceEvent::Byte { offset, value }) => {
                    if offset != self.total {
                        return Err(PackError::Malformed { what: "retained-anchor", offset, detail: "non-contiguous source event".into() });
                    }
                    if self.header_len < HEADER_SIZE {
                        self.header[self.header_len] = value;
                        self.header_len += 1;
                    }
                    self.footer_ring[self.footer_head] = value;
                    self.footer_head = (self.footer_head + 1) % FOOTER_SIZE;
                    self.footer_len = (self.footer_len + 1).min(FOOTER_SIZE);
                    self.total += 1;
                    Ok(false)
                }
                Some(RetainedPackSourceEvent::Complete { bytes, .. }) => {
                    if bytes != self.total || self.header_len != HEADER_SIZE || self.footer_len != FOOTER_SIZE {
                        return Err(PackError::Truncated(self.total));
                    }
                    for index in 0..FOOTER_SIZE {
                        self.ordered_footer[index] = self.footer_ring[(self.footer_head + index) % FOOTER_SIZE];
                    }
                    self.phase = RetainedPackAnchorPhase::VerifyHeader(0);
                    Ok(false)
                }
                None => Ok(false),
            },
            RetainedPackAnchorPhase::VerifyHeader(index) => {
                if event.is_some() {
                    return Err(PackError::Malformed { what: "retained-anchor", offset: self.total, detail: "source replay after completion".into() });
                }
                if index < 20 {
                    self.header_crc.update_page(&self.header[index..index + 1]);
                    self.phase = RetainedPackAnchorPhase::VerifyHeader(index + 1);
                    return Ok(false);
                }
                let stored = u32::from_le_bytes(self.header[20..24].try_into().expect("fixed header crc"));
                if stored != self.header_crc.finish() {
                    return Err(PackError::ChecksumMismatch { segment: "header", offset: 20 });
                }
                if self.header[..8] != MAGIC {
                    return Err(PackError::BadMagic);
                }
                if self.header[24..32] != [0; 8] {
                    return Err(PackError::Malformed { what: "header", offset: 24, detail: "reserved header bytes are nonzero".into() });
                }
                self.phase = RetainedPackAnchorPhase::VerifyFooter(0);
                Ok(false)
            }
            RetainedPackAnchorPhase::VerifyFooter(index) => {
                if event.is_some() {
                    return Err(PackError::Malformed { what: "retained-anchor", offset: self.total, detail: "source replay after completion".into() });
                }
                if index < 80 {
                    self.footer_crc.update_page(&self.ordered_footer[index..index + 1]);
                    self.phase = RetainedPackAnchorPhase::VerifyFooter(index + 1);
                    return Ok(false);
                }
                let stored = u32::from_le_bytes(self.ordered_footer[80..84].try_into().expect("fixed footer crc"));
                if stored != self.footer_crc.finish() {
                    return Err(PackError::ChecksumMismatch { segment: "footer", offset: self.total - 4 });
                }
                if self.ordered_footer[..8] != FOOTER_MAGIC {
                    return Err(PackError::BadMagic);
                }
                let version_major = u16::from_le_bytes(self.header[8..10].try_into().expect("header major"));
                let version_minor = u16::from_le_bytes(self.header[10..12].try_into().expect("header minor"));
                let required_flags = u32::from_le_bytes(self.header[12..16].try_into().expect("header flags"));
                let optional_flags = u32::from_le_bytes(self.header[16..20].try_into().expect("header optional flags"));
                if version_major != FORMAT_VERSION_MAJOR {
                    return Err(PackError::UnsupportedVersion { major: version_major, minor: version_minor });
                }
                let unknown = required_flags & !REQUIRED_KNOWN_MASK;
                if unknown != 0 {
                    return Err(PackError::UnknownRequiredFlags(unknown));
                }
                let footer_major = u16::from_le_bytes(self.ordered_footer[8..10].try_into().expect("footer major"));
                let footer_minor = u16::from_le_bytes(self.ordered_footer[10..12].try_into().expect("footer minor"));
                let footer_flags = u32::from_le_bytes(self.ordered_footer[12..16].try_into().expect("footer flags"));
                let file_len = u64::from_le_bytes(self.ordered_footer[32..40].try_into().expect("footer length"));
                let manifest_offset = u64::from_le_bytes(self.ordered_footer[16..24].try_into().expect("manifest offset"));
                let manifest_len = u64::from_le_bytes(self.ordered_footer[24..32].try_into().expect("manifest length"));
                let manifest_end = manifest_offset.checked_add(manifest_len).ok_or(PackError::LimitExceeded("manifest span overflow"))?;
                if footer_major != version_major || footer_minor != version_minor || footer_flags != required_flags || file_len != self.total {
                    return Err(PackError::Malformed { what: "footer", offset: self.total - FOOTER_SIZE as u64, detail: "anchor identity mismatch".into() });
                }
                if manifest_offset < HEADER_SIZE as u64 || manifest_len == 0 || manifest_end > self.total - FOOTER_SIZE as u64 {
                    return Err(PackError::Malformed { what: "footer", offset: self.total - FOOTER_SIZE as u64, detail: "manifest span is outside the segment area".into() });
                }
                let mut content_hash = [0; 32];
                content_hash.copy_from_slice(&self.ordered_footer[40..72]);
                self.value = Some(Superblock {
                    header: Header { version_major, version_minor, required_flags, optional_flags },
                    footer: Footer {
                        version_major: footer_major,
                        version_minor: footer_minor,
                        required_flags: footer_flags,
                        manifest_offset,
                        manifest_len,
                        file_len,
                        content_hash: ContentHash(content_hash),
                        prev_footer_offset: u64::from_le_bytes(self.ordered_footer[72..80].try_into().expect("previous footer")),
                    },
                });
                self.phase = RetainedPackAnchorPhase::Ready;
                Ok(true)
            }
            RetainedPackAnchorPhase::Ready => Ok(true),
            RetainedPackAnchorPhase::Closed => Err(PackError::Malformed { what: "retained-anchor", offset: self.total, detail: "anchor is closed".into() }),
        }
    }

    pub fn take(&mut self) -> Option<Superblock> {
        if self.phase != RetainedPackAnchorPhase::Ready || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.value.take()
    }

    pub fn close_step(&mut self) -> RetainedPackCloseStep {
        self.value = None;
        self.phase = RetainedPackAnchorPhase::Closed;
        self.handed_back = true;
        RetainedPackCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.handed_back && self.value.is_none() && matches!(self.phase, RetainedPackAnchorPhase::Ready | RetainedPackAnchorPhase::Closed)
    }
}

impl Default for RetainedPackAnchorCursor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RetainedPackAnchorCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained pack anchors reached Drop before handback or close");
    }
}

#[cfg(test)]
mod retained_pack_source_laws {
    use super::*;

    fn page(bytes: &[u8]) -> RetainedPackPage {
        let mut owned = [0; RETAINED_PACK_PAGE_BYTES];
        owned[..bytes.len()].copy_from_slice(bytes);
        RetainedPackPage::try_from_array(owned, bytes.len()).expect("valid page")
    }

    fn drain(cursor: &mut RetainedPackSourceCursor) -> Vec<u8> {
        let mut bytes = Vec::new();
        loop {
            match cursor.grant().expect("grant") {
                Some(RetainedPackSourceEvent::Byte { value, .. }) => bytes.push(value),
                Some(RetainedPackSourceEvent::Complete { .. }) => break,
                None => panic!("sealed cursor must make progress"),
            }
        }
        bytes
    }

    fn close(cursor: &mut RetainedPackSourceCursor) {
        loop {
            if cursor.close_step(1, RETAINED_PACK_PAGE_BYTES).expect("close") == RetainedPackCloseStep::Complete {
                break;
            }
        }
    }

    async fn canonical_pack(codec: CodecId) -> (Vec<u8>, Vec<u8>) {
        let options = WriteOptions { required_flags: 0, optional_flags: OPTIONAL_CANONICAL, codec };
        let mut writer = PackWriter::begin(Vec::<u8>::new(), &options).await.expect("writer");
        writer.write_segment(crate::KIND_SYMBOLS, &encode_symbols(&["p2d2".to_string(), "ä".to_string()]).await).await.expect("symbols");
        let document = b"retained-canonical-document".to_vec();
        let doc_offset = writer.position().await;
        writer.write_segment(crate::KIND_DOCUMENT, &document).await.expect("document");
        let doc_len = writer.position().await - doc_offset;
        let manifest = Manifest {
            schema_name: "p2d2".into(),
            schema_hash: [2; 32],
            doc_span: ByteRange { offset: doc_offset, len: doc_len },
            doc_frame_count: 1,
            symbols_span: ByteRange { offset: 0, len: 0 },
            chunk_table_span: ByteRange { offset: 0, len: 0 },
            field_index_span: ByteRange { offset: 0, len: 0 },
            uncompressed_body_len: document.len() as u64,
            field_count: 1,
            chunk_count: 0,
            symbol_count: 0,
        };
        (writer.finish(&manifest).await.expect("finish"), document)
    }

    fn source(bytes: &[u8]) -> RetainedPackSourceCursor {
        let pages = bytes.len().div_ceil(RETAINED_PACK_PAGE_BYTES);
        let mut source = RetainedPackSourceCursor::try_new(pages, bytes.len()).expect("source");
        for part in bytes.chunks(RETAINED_PACK_PAGE_BYTES) {
            source.admit_page(page(part)).expect("admit");
        }
        source.seal().expect("seal");
        source
    }

    fn forward_catalog(event: RetainedPackSegmentEvent, catalog: &mut RetainedPackCatalogCursor, document: &mut Vec<u8>) {
        catalog.admit(event).expect("catalog admission");
        if let Some(RetainedPackCatalogEvent::DocumentByte { value, .. }) = catalog.grant().expect("catalog grant") {
            document.push(value);
        }
    }

    fn forward_source(event: RetainedPackSourceEvent, segment: &mut RetainedPackSegmentCursor, catalog: &mut RetainedPackCatalogCursor, document: &mut Vec<u8>) {
        segment.admit(event).expect("segment admission");
        loop {
            if let Some(event) = segment.grant().expect("segment grant") {
                forward_catalog(event, catalog, document);
            }
            if segment.preflight().is_ok() || matches!(event, RetainedPackSourceEvent::Complete { .. }) {
                break;
            }
        }
    }

    #[test]
    fn zero_maximum_plus_one_and_producer_handback_are_exact() {
        assert!(RetainedPackSourceCursor::try_new(0, 1).is_err());
        assert!(RetainedPackSourceCursor::try_new(1, 0).is_err());
        assert!(RetainedPackSourceCursor::try_new(1, RETAINED_PACK_PAGE_BYTES + 1).is_err());
        let mut cursor = RetainedPackSourceCursor::try_new(1, RETAINED_PACK_PAGE_BYTES).expect("maximum");
        cursor.admit_page(page(&vec![7; RETAINED_PACK_PAGE_BYTES])).expect("maximum page");
        let rejected = page(&[8]);
        let rejected = cursor.admit_page(rejected).expect_err("maximum plus one");
        assert_eq!(rejected.len(), 1);
        close(&mut cursor);
    }

    #[test]
    fn interruption_resume_is_byte_exact_and_close_is_incremental() {
        let mut cursor = RetainedPackSourceCursor::try_new(2, 8).expect("credits");
        cursor.admit_page(page(b"SPK")).expect("first");
        cursor.admit_page(page(b"123")).expect("second");
        assert_eq!(cursor.grant().expect("unsealed"), None);
        cursor.seal().expect("seal");
        assert_eq!(cursor.grant().expect("S"), Some(RetainedPackSourceEvent::Byte { offset: 0, value: b'S' }));
        assert_eq!(cursor.progress().consumed_bytes, 1);
        assert_eq!(drain(&mut cursor), b"PK123");
        assert_ne!(cursor.close_step(1, RETAINED_PACK_PAGE_BYTES).expect("first close"), RetainedPackCloseStep::Complete);
        close(&mut cursor);
        assert!(cursor.terminal_is_empty());
    }

    #[test]
    fn cancellation_is_observable_and_still_requires_terminal_empty_close() {
        let mut cursor = RetainedPackSourceCursor::try_new(1, 4).expect("credits");
        cursor.admit_page(page(b"SPK1")).expect("page");
        cursor.seal().expect("seal");
        cursor.request_cancel();
        assert_eq!(cursor.grant(), Err("retained-pack.cancelled"));
        close(&mut cursor);
    }

    #[test]
    fn language_neutral_retained_law_ledger_is_complete() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️tests/🔣️.json")).expect("fixture JSON");
        assert_eq!(fixture["admission"]["pageBytes"], RETAINED_PACK_PAGE_BYTES);
        assert_eq!(fixture["valueTags"].as_array().expect("tags").len(), 24);
        assert!(fixture["hostile"].as_array().expect("hostile laws").iter().any(|law| law == "terminal-empty"));
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_anchors_segments_catalog_and_deflate_are_wire_identical_and_resumable() {
        let (bytes, expected_document) = canonical_pack(CodecId(1)).await;
        let mut source = source(&bytes);
        let limits = PackLimits { max_file_len: bytes.len() as u64, max_segment_len: 4096, max_symbols: 2, max_depth: 8, max_items: 4, max_total_alloc: 4096 };
        let mut anchor = RetainedPackAnchorCursor::new();
        let mut segment = RetainedPackSegmentCursor::try_new(limits.clone()).expect("segment");
        let mut catalog_cursor = RetainedPackCatalogCursor::try_new(limits, 2, 0, 16).expect("catalog");
        let mut document = Vec::new();
        loop {
            let event = source.grant().expect("source grant").expect("sealed source event");
            anchor.grant(Some(event)).expect("anchor collect");
            forward_source(event, &mut segment, &mut catalog_cursor, &mut document);
            if matches!(event, RetainedPackSourceEvent::Complete { .. }) {
                break;
            }
        }
        while !anchor.grant(None).expect("anchor verify") {}
        let superblock = anchor.take().expect("anchor handback");
        let catalog = catalog_cursor.take(superblock).expect("catalog result").expect("catalog handback");
        assert_eq!(document, expected_document);
        assert_eq!(catalog.manifest.schema_name, "p2d2");
        assert_eq!(catalog.symbols, ["p2d2", "ä"]);
        anchor.close_step();
        segment.close_step();
        assert_eq!(catalog_cursor.close_step(1), RetainedPackCloseStep::Complete);
        close(&mut source);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_anchor_rejects_hostile_crc_and_requires_explicit_close() {
        let (mut bytes, _) = canonical_pack(CodecId(0)).await;
        bytes[20] ^= 1;
        let mut source = source(&bytes);
        let mut anchor = RetainedPackAnchorCursor::new();
        loop {
            let event = source.grant().expect("source").expect("event");
            anchor.grant(Some(event)).expect("collection cannot fail before verification");
            if matches!(event, RetainedPackSourceEvent::Complete { .. }) {
                break;
            }
        }
        let mut failure = None;
        while failure.is_none() {
            failure = anchor.grant(None).err();
        }
        assert!(matches!(failure, Some(PackError::ChecksumMismatch { segment: "header", .. })));
        anchor.close_step();
        close(&mut source);
    }
}
//#endregion 🔖️RetainedCanonicalSource
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
        let crc = crc32c(&bytes[0..20]);
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
        let crc = crc32c(&bytes);
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
    async fn retained_identity_chunk_fragment_parity_exact_boundary_and_interrupted_finish() {
        let options = WriteOptions { required_flags: REQUIRED_CHUNKED, optional_flags: 0, codec: CodecId(0) };
        let payload = vec![0xA5; 16_385];
        let manifest = Manifest {
            schema_name: String::new(),
            schema_hash: [0; 32],
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
        let mut oracle = PackWriter::begin(Vec::new(), &options).await.unwrap();
        oracle.write_chunk(&payload).await.unwrap();
        let oracle = oracle.finish(&manifest).await.unwrap();

        let mut retained = PackWriter::begin(Vec::new(), &options).await.unwrap();
        let mut chunk = retained.begin_identity_chunk(payload.len()).await.unwrap();
        chunk.write_fragment(&payload[..1]).await.unwrap();
        chunk.write_fragment(&payload[1..16_384]).await.unwrap();
        chunk.write_fragment(&payload[16_384..]).await.unwrap();
        chunk.finish().await.unwrap();
        let retained = retained.finish(&manifest).await.unwrap();
        assert_eq!(retained, oracle);

        let mut interrupted = PackWriter::begin(Vec::new(), &options).await.unwrap();
        let mut chunk = interrupted.begin_identity_chunk(2).await.unwrap();
        chunk.write_fragment(&payload[..1]).await.unwrap();
        assert!(matches!(chunk.finish().await, Err(PackError::LimitExceeded(_))));

        let mut maximum = PackWriter::begin(Vec::new(), &options).await.unwrap();
        let mut chunk = maximum.begin_identity_chunk(1).await.unwrap();
        assert!(matches!(chunk.write_fragment(&payload[..2]).await, Err(PackError::LimitExceeded(_))));
        chunk.close();
        assert_eq!(payload[0], 0xA5);

        let mut closed = PackWriter::begin(Vec::new(), &options).await.unwrap();
        let chunk = closed.begin_identity_chunk(1).await.unwrap();
        chunk.close();
        assert!(closed.chunks.is_empty());
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
    async fn identity_chunk_cursor_retains_fragment_progress_and_terminal_verification() {
        let (bytes, chunk_id, expected) = build_sample_pack(CodecId(0)).await;
        let file = PackFile::open_manifest(bytes.as_slice(), &PackLimits::default(), VerificationLevel::Full).await.unwrap();
        let mut cursor = file.identity_chunk_cursor(chunk_id, VerificationLevel::Full).unwrap();
        let mut fragment = [0u8; 3];
        let mut actual = Vec::new();
        loop {
            let read = cursor.read_fragment(&mut fragment).await.unwrap();
            if read == 0 {
                break;
            }
            actual.extend_from_slice(&fragment[..read]);
        }
        assert_eq!(actual, expected);
        assert!(cursor.terminal_is_empty());
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
        let expected_empty_hash = ContentHash(*semio_framework_hash::hash(b"").as_bytes());
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
