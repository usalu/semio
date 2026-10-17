//! 🧩️ Segments and symbols of the pack format: the retained varints, the segment codec, the segment decoder and the symbol table codec. See [`super`].

use super::*;

//#region 🔖️Segment
/// 📦️ The fully-encoded wire bytes of one framed segment, plus the byte offsets within
/// them a writer needs to record chunk-table/manifest-span metadata without re-parsing.
struct EncodedSegment {
    pub(super) bytes: Vec<u8>,
    #[cfg(test)]
    pub(super) header_len: usize,
    #[cfg(test)]
    pub(super) stored_len: usize,
}

pub(super) fn retained_varint(mut value: u64, output: &mut [u8; 10]) -> &[u8] {
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

pub(super) fn retained_varint_len(value: u64) -> usize {
    let mut output = [0u8; 10];
    retained_varint(value, &mut output).len()
}

/// 🧵️ Resolves `CodecId` to this crate's codec implementations for compression.
async fn codec_compress(codec: CodecId, raw: &[u8]) -> Result<Vec<u8>, PackRefusal> {
    match codec.0 {
        0 => Ok(raw.to_vec()),
        1 => semio_framework_replication_codec::deflate_compress(raw),
        other => Err(PackRefusal::UnsupportedCodec(other)),
    }
}

/// 🧵️ Resolves `CodecId` to this crate's codec implementations for decompression.
pub(super) async fn codec_decompress(codec: CodecId, stored: &[u8], raw_len: u64, limit: u64) -> Result<Vec<u8>, PackRefusal> {
    match codec.0 {
        0 => NoCompression.decompress(stored, raw_len, limit),
        1 => semio_framework_replication_codec::deflate_decompress(stored, raw_len, limit),
        other => Err(PackRefusal::UnsupportedCodec(other)),
    }
}

/// 🖇️ Frames `payload` as a segment per the contract's byte layout: `kind, flags,
/// seg_len, [raw_len], payload, crc32`. Compresses first when `codec` is non-identity.
pub(super) async fn encode_segment(kind: u8, codec: CodecId, payload: &[u8]) -> Result<EncodedSegment, PackRefusal> {
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
    #[cfg(test)]
    let header_len = buf.len();
    buf.extend_from_slice(&stored);
    let crc = crc32c(&buf);
    buf.extend_from_slice(&crc.to_le_bytes());
    Ok(EncodedSegment { bytes: buf, #[cfg(test)] header_len, #[cfg(test)] stored_len: stored.len() })
}

/// 👓️ A decoded, CRC-checked, decompressed segment plus enough position bookkeeping for
/// callers walking a sequence of segments.
struct DecodedSegment {
    pub(super) kind: u8,
    pub(super) payload: Vec<u8>,
    /// ➡️ Total wire bytes consumed by this segment (frame header + stored payload + crc).
    pub(super) consumed: u64,
}

/// 1⃣ Bounds-checked single-byte read at an absolute file offset.
pub(super) async fn read_u8_at<S: PackSource>(source: &S, offset: u64) -> Result<u8, S::Error> {
    let mut buf = [0u8; 1];
    source.read_exact_at(offset, &mut buf).await?;
    Ok(buf[0])
}

/// 🔢️ Reads one LEB128 varint starting at an absolute file offset, one byte at a time so
/// it never over-reads past a legitimately short remaining file. Returns `(value, bytes_consumed)`.
pub(super) async fn read_varint_u64_at<S: PackSource>(source: &S, offset: u64) -> Result<(u64, u64), S::Error> {
    let mut tmp = [0u8; 10];
    let mut i = 0u64;
    loop {
        let byte = read_u8_at(source, offset + i).await?;
        tmp[i as usize] = byte;
        i += 1;
        if byte & 0x80 == 0 {
            break;
        }
        if i >= 10 {
            return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"varint", offset, detail: "overlong varint".to_string() }).into());
        }
    }
    let mut pos = 0usize;
    let value = read_varint_u64(&tmp[..i as usize], &mut pos)?;
    Ok((value, i))
}

/// 🚪️ Decodes one framed segment starting at an absolute file offset: reads and
/// bounds-checks `kind, flags, seg_len, [raw_len]`, validates lengths against
/// `limits.max_segment_len` **before** allocating the payload buffer, then optionally verifies
/// the frame's CRC-32C and decompresses. Unknown `kind` values are decoded and returned as-is —
/// callers that only want known kinds are responsible for skipping/rejecting them, this function
/// never errors on an unrecognized kind.
pub(super) async fn decode_segment_at<S: PackSource>(source: &S, offset: u64, limits: &PackLimits, verify_crc: bool) -> Result<DecodedSegment, S::Error> {
    let total_len = source.len().await;
    if offset >= total_len {
        return Err((PackRefusal::Truncated(offset)).into());
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
        return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"segment length exceeds max_segment_len"}).into());
    }
    let payload_offset = cursor;
    let payload_end = payload_offset.checked_add(stored_len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment payload offset overflow"})?;
    if payload_end > total_len {
        return Err((PackRefusal::Truncated(payload_offset)).into());
    }
    let frame_len=(payload_offset-offset).checked_add(stored_len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"frame allocation overflow"})?;
    let allocation=frame_len.checked_add(raw_len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"decoded frame allocation overflow"})?;
    if allocation>limits.max_total_alloc{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"decoded frame exceeds max_total_alloc"}).into());}
    let header_len = usize::try_from(payload_offset-offset).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"frame header exceeds address space"})?;
    let frame_len=usize::try_from(frame_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"frame exceeds address space"})?;
    let mut frame=Vec::new();check_allocation_layout::<u8>(frame_len,"frame allocation failed")?;frame.try_reserve_exact(frame_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::AllocationFailed,limit:"frame allocation failed"})?;frame.resize(frame_len,0);
    source.read_exact_at(offset, &mut frame).await?;
    let crc_offset = payload_end;
    if crc_offset + 4 > total_len {
        return Err((PackRefusal::Truncated(crc_offset)).into());
    }
    let mut crc_bytes = [0u8; 4];
    source.read_exact_at(crc_offset, &mut crc_bytes).await?;
    if verify_crc {
        let stored_crc = u32::from_le_bytes(crc_bytes);
        let computed_crc = crc32c(&frame);
        if stored_crc != computed_crc {
            return Err((PackRefusal::ChecksumMismatch { segment: "segment", offset: crc_offset }).into());
        }
    }
    let stored_payload = &frame[header_len..];
    let payload = if compressed { codec_decompress(codec, stored_payload, raw_len, limits.max_segment_len).await? } else { stored_payload.to_vec() };
    let consumed = (crc_offset + 4) - offset;
    Ok(DecodedSegment { kind, payload, consumed })
}
//#endregion 🔖️Segment

//#region 🔖️Symbols
/// ✍️ Serializes a symbol table: `count varint, then count × (len varint, utf8 bytes)`.
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

/// 📖️ Parses a symbol table, rejecting a count over `limits.max_symbols` before
/// allocating the output `Vec`.
pub(super) async fn decode_symbols(payload: &[u8], limits: &PackLimits) -> Result<Vec<String>, PackRefusal> {
    let mut pos = 0usize;
    let count = read_varint_u64(payload, &mut pos)?;
    if count > limits.max_symbols as u64 {
        return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"symbol count exceeds max_symbols"});
    }
    let slots = count.checked_mul(size_of::<String>() as u64).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol allocation overflow"})?;
    if slots > limits.max_total_alloc || count > payload.len() as u64 {
        return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol storage exceeds max_total_alloc"});
    }
    let mut owned_bytes = slots;
    let mut out = Vec::new();
    let count=usize::try_from(count).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol count exceeds address space"})?;
    check_allocation_layout::<String>(count,"symbol allocation")?;
    out.try_reserve_exact(count).map_err(|_| PackRefusal::LimitExceeded{kind:ValueRefusalKind::AllocationFailed,limit:"symbol allocation"})?;
    for _ in 0..count {
        let len = read_varint_u64(payload, &mut pos)?;
        owned_bytes = owned_bytes.checked_add(len).filter(|bytes| *bytes <= limits.max_total_alloc).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol storage exceeds max_total_alloc"})?;
        let len = usize::try_from(len).map_err(|_| PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol length exceeds address space"})?;
        let end = pos.checked_add(len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol length overflow"})?;
        if end > payload.len() {
            return Err(PackRefusal::Truncated(pos as u64));
        }
        let borrowed = std::str::from_utf8(&payload[pos..end]).map_err(|_| PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"symbol", offset: pos as u64, detail: "invalid utf8".to_string() })?;
        let mut text = String::new();
        check_allocation_layout::<u8>(len,"symbol text allocation")?;
        text.try_reserve_exact(len).map_err(|_| PackRefusal::LimitExceeded{kind:ValueRefusalKind::AllocationFailed,limit:"symbol text allocation"})?;
        text.push_str(borrowed);
        pos = end;
        out.push(text);
    }
    Ok(out)
}
//#endregion 🔖️Symbols
