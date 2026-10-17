//! 🧱️ PNG chunks and header of the pixels codec: the signature, the chunk writer and readers, the IHDR record and parser and the sample and row size helpers. See [`super`].

use super::*;

//#region PngChunkIo
pub(super) const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

pub(super) fn write_chunk(out: &mut Vec<u8>, ty: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(ty);
    out.extend_from_slice(data);
    let mut crc_in = Vec::with_capacity(4 + data.len());
    crc_in.extend_from_slice(ty);
    crc_in.extend_from_slice(data);
    out.extend_from_slice(&crc32(&crc_in).to_be_bytes());
}

type PngChunk<'a> = ([u8; 4], &'a [u8]);

/// 📖️ Splits a PNG byte stream into `(type, data)` chunks, rejecting CRC mismatches and
/// truncation up front so downstream decode logic never has to re-check framing.
pub(super) fn read_chunks(data: &[u8]) -> Result<Vec<PngChunk<'_>>, String> {
    if data.len() < 8 || data[0..8] != PNG_SIGNATURE {
        return Err("png: bad signature".into());
    }
    read_chunks_from(data, 8)
}

/// 📖️ [`read_chunks`] from an arbitrary chunk boundary `pos` through `IEND`.
pub(super) fn read_chunks_from(data: &[u8], mut pos: usize) -> Result<Vec<PngChunk<'_>>, String> {
    let mut chunks = Vec::new();
    loop {
        if pos + 8 > data.len() {
            return Err("png: truncated chunk header".into());
        }
        let len = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        let ty: [u8; 4] = [data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]];
        let start = pos + 8;
        let end = start.checked_add(len).ok_or("png: chunk length overflow")?;
        if end + 4 > data.len() {
            return Err("png: truncated chunk data or crc".into());
        }
        let chunk_data = &data[start..end];
        let stored_crc = u32::from_be_bytes([data[end], data[end + 1], data[end + 2], data[end + 3]]);
        if crc32_update(crc32_update(0xFFFF_FFFF, &ty), chunk_data) ^ 0xFFFF_FFFF != stored_crc {
            return Err(format!("png: chunk CRC mismatch ({})", String::from_utf8_lossy(&ty)));
        }
        chunks.push((ty, chunk_data));
        pos = end + 4;
        if ty == *b"IEND" {
            break;
        }
        if pos >= data.len() {
            return Err("png: missing IEND".into());
        }
    }
    Ok(chunks)
}
//#endregion PngChunkIo

//#region PngIhdr
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Ihdr {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub(super) interlace: u8,
}

pub(super) fn parse_ihdr(data: &[u8]) -> Result<Ihdr, String> {
    if data.len() != 13 {
        return Err("png IHDR: expected 13 bytes".into());
    }
    let width = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    let height = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    let bit_depth = data[8];
    let color_type = data[9];
    let compression = data[10];
    let filter_method = data[11];
    let interlace = data[12];
    if width == 0 || height == 0 {
        return Err("png IHDR: zero dimension".into());
    }
    if compression != 0 {
        return Err("png IHDR: unsupported compression method".into());
    }
    if filter_method != 0 {
        return Err("png IHDR: unsupported filter method".into());
    }
    if interlace > 1 {
        return Err("png IHDR: unsupported interlace method".into());
    }
    let valid = match color_type {
        0 => matches!(bit_depth, 1 | 2 | 4 | 8 | 16),
        2 => matches!(bit_depth, 8 | 16),
        3 => matches!(bit_depth, 1 | 2 | 4 | 8),
        4 => matches!(bit_depth, 8 | 16),
        6 => matches!(bit_depth, 8 | 16),
        _ => false,
    };
    if !valid {
        return Err(format!("png IHDR: unsupported color type {color_type} / bit depth {bit_depth}"));
    }
    Ok(Ihdr { width, height, bit_depth, color_type, interlace })
}

pub(super) fn samples_per_pixel(color_type: u8) -> usize {
    match color_type {
        0 => 1,
        2 => 3,
        3 => 1,
        4 => 2,
        6 => 4,
        _ => unreachable!("validated in parse_ihdr"),
    }
}

pub(super) fn bpp_bytes(ihdr: &Ihdr) -> usize {
    (samples_per_pixel(ihdr.color_type) * ihdr.bit_depth as usize).div_ceil(8).max(1)
}

pub(super) fn packed_row_bytes(width: u32, color_type: u8, bit_depth: u8) -> usize {
    let bits = width as usize * samples_per_pixel(color_type) * bit_depth as usize;
    bits.div_ceil(8)
}
//#endregion PngIhdr
