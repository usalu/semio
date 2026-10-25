//! 📜️ Chunk table and manifest of the pack format: the chunk table entry and decoder, the manifest records and their span, encode, parse and resolve helpers. See [`super`].

use super::*;

//#region 🔖️ChunkTable
/// 🧱️ One row of the chunk table: where a chunk's (possibly compressed) payload lives
/// and how to verify it. `crc32` covers the stored (on-disk) bytes; `blake3` covers the raw
/// (decompressed) content — the former is cheap and always checked at `Standard`+, the latter is
/// the content-identity hash only checked at `Full`.
#[derive(Clone, Debug)]
pub(super) struct ChunkTableEntry {
    pub(super) offset: u64,
    pub(super) stored_len: u64,
    pub(super) raw_len: u64,
    pub(super) crc32: u32,
    pub(super) blake3: [u8; 32],
}

/// 🏓️ Parses the chunk table, rejecting a count over `limits.max_items` or an entry
/// length over `limits.max_segment_len` before allocating.
pub(super) async fn decode_chunk_table(payload: &[u8], limits: &PackLimits) -> Result<Vec<ChunkTableEntry>, PackRefusal> {
    let mut pos = 0usize;
    let count = read_varint_u64(payload, &mut pos)?;
    if count > limits.max_items {
        return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"chunk_table count exceeds max_items"});
    }
    let slots = count.checked_mul(size_of::<ChunkTableEntry>() as u64).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk table allocation overflow"})?;
    if slots > limits.max_total_alloc || count > payload.len() as u64 / 39 {
        return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk table storage exceeds max_total_alloc"});
    }
    let mut out = Vec::new();
    let count=usize::try_from(count).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk count exceeds address space"})?;
    check_allocation_layout::<ChunkTableEntry>(count,"chunk table allocation")?;
    out.try_reserve_exact(count).map_err(|_| PackRefusal::LimitExceeded{kind:ValueRefusalKind::AllocationFailed,limit:"chunk table allocation"})?;
    for _ in 0..count {
        let offset = read_varint_u64(payload, &mut pos)?;
        let stored_len = read_varint_u64(payload, &mut pos)?;
        let raw_len = read_varint_u64(payload, &mut pos)?;
        if stored_len > limits.max_segment_len || raw_len > limits.max_segment_len {
            return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"chunk table entry length exceeds max_segment_len"});
        }
        if pos + 4 > payload.len() {
            return Err(PackRefusal::Truncated(pos as u64));
        }
        let crc32 = u32::from_le_bytes(payload[pos..pos + 4].try_into().unwrap());
        pos += 4;
        if pos + 32 > payload.len() {
            return Err(PackRefusal::Truncated(pos as u64));
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
/// 🗺️ The manifest: spans and counts describing every other segment in the file.
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

/// 🧬️ The wire-level manifest fields before `schema_symref` has been resolved against a
/// symbol table (on decode) or after it has been resolved to a symref (on encode).
pub struct RawManifest {
    pub schema_symref: u64,
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

async fn write_span(buf: &mut Vec<u8>, span: ByteRange) {
    write_varint_u64(buf, span.offset);
    write_varint_u64(buf, span.len);
}

async fn read_span(payload: &[u8], pos: &mut usize) -> Result<ByteRange, PackRefusal> {
    let offset = read_varint_u64(payload, pos)?;
    let len = read_varint_u64(payload, pos)?;
    Ok(ByteRange { offset, len })
}

/// ✍️ Serializes the manifest segment payload per the contract's field order.
pub(super) async fn encode_manifest_bytes(schema_symref: u64, manifest: &Manifest) -> Vec<u8> {
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

/// 📖️ Parses the manifest segment payload. Trailing bytes beyond the known fields are
/// silently ignored (additive-evolution slot), never an error.
pub(super) async fn parse_raw_manifest(payload: &[u8]) -> Result<RawManifest, PackRefusal> {
    let mut pos = 0usize;
    let schema_symref = read_varint_u64(payload, &mut pos)?;
    if pos + 32 > payload.len() {
        return Err(PackRefusal::Truncated(pos as u64));
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
pub(super) fn resolve_manifest(raw: &RawManifest, symbols: &[String]) -> Result<Manifest, PackRefusal> {
    let schema_name =
        if symbols.is_empty() && raw.schema_symref == 0 { String::new() } else { symbols.get(raw.schema_symref as usize).cloned().ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest", offset: 0, detail: "schema symref out of range".to_string() })? };
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
