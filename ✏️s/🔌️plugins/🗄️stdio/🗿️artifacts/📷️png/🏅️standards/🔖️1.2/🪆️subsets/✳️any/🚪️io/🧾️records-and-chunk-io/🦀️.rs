//! 🧾️ Records and chunk framing of the PNG codec: the chunk, projection, address, layout and preview records, the signature, the CRC and the chunk writer and reader. See [`super`].

use super::*;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PngChunk {
    pub kind: [u8; 4],
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "chunk", rename_all = "camelCase")]
pub enum PngChunkMarker {
    Ihdr,
    Plte,
    Trns,
    Gama,
    Chrm,
    Srgb,
    Phys,
    Time,
    Bkgd,
    Idat,
    Iend,
    Text { index: usize },
    Unknown { index: usize },
}

#[derive(Clone, Debug, PartialEq)]
pub struct PngProjection {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: PngColorType,
    pub interlace: bool,
    pub plte: Option<Vec<PngRgb>>,
    pub trns: Option<PngTransparency>,
    pub gama: Option<u32>,
    pub chrm: Option<PngChromaticities>,
    pub srgb: Option<PngSrgbIntent>,
    pub phys: Option<PngPhysicalDims>,
    pub time: Option<PngTimestamp>,
    pub bkgd: Option<PngBackground>,
    pub text_chunks: Vec<PngTextChunk>,
    pub pixels: Vec<u8>,
    pub chunk_order: Vec<PngChunkMarker>,
    pub unknown_chunks: Vec<PngChunk>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PngChunkAddress {
    pub ordinal: usize,
    pub kind: [u8; 4],
    pub start: usize,
    pub data_start: usize,
    pub data_end: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PngLayout {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: PngColorType,
    pub interlace: bool,
    pub chunks: Vec<PngChunkAddress>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PngPreview {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}

//#region Signature
pub const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
//#endregion Signature

//#region Crc
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn png_crc32_parts(kind: &[u8; 4], data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for byte in kind.iter().chain(data) {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xedb8_8320 } else { crc >> 1 };
        }
    }
    !crc
}
//#endregion Crc

//#region ChunkIo
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_chunk(out: &mut Vec<u8>, ty: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(ty);
    out.extend_from_slice(data);
    out.extend_from_slice(&png_crc32_parts(ty, data).to_be_bytes());
}

/// 📖 Splits a PNG byte stream into `(type, data)` chunks, rejecting CRC mismatches and
/// truncation up front so downstream decode logic never has to re-check framing.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_chunks<'a>(data: &'a [u8], control:&mut NativeDecodeControl<'_>) -> Result<Vec<PngChunkView<'a>>, PngReadError> {
    if data.len() < 8 || data[0..8] != PNG_SIGNATURE {
        return Err("png: bad signature".into());
    }
    let mut pos = 8usize;
    let mut count=0usize;let mut measured=8usize;
    while measured<data.len() {let header=data.get(measured..measured+8).ok_or("png: truncated chunk header")?;let len=u32::from_be_bytes(header[..4].try_into().unwrap())as usize;measured=measured.checked_add(12).and_then(|n|n.checked_add(len)).ok_or("png: chunk framing overflow")?;if measured>data.len() {return Err("png: truncated chunk".into());}count+=1;}
    let mut chunks=control.allocate_vec(count)?;
    control.begin_stage(data.len())?;control.advance(8)?;
    loop {
        if pos + 8 > data.len() {
            return Err("png: truncated chunk header".into());
        }
        let len = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        let ty: [u8; 4] = [data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]];
        let start = pos + 8;
        let end = start.checked_add(len).ok_or("png: chunk length overflow")?;
        if end.checked_add(4).is_none_or(|end|end>data.len()) {
            return Err("png: truncated chunk data or crc".into());
        }
        let chunk_data = &data[start..end];
        let stored_crc = u32::from_be_bytes([data[end], data[end + 1], data[end + 2], data[end + 3]]);
        let mut crc=0xffff_ffffu32;
        for byte in ty {crc^=u32::from(byte);for _ in 0..8 {crc=if crc&1!=0 {(crc>>1)^0xedb88320}else{crc>>1};}}
        control.advance(12)?;
        for piece in chunk_data.chunks(256) {for byte in piece {crc^=u32::from(*byte);for _ in 0..8 {crc=if crc&1!=0 {(crc>>1)^0xedb88320}else{crc>>1};}}control.advance(piece.len())?;}
        if !crc!=stored_crc {return Err(format!("png: chunk CRC mismatch ({})",String::from_utf8_lossy(&ty)).into());}
        chunks.push((ty, chunk_data));
        pos = end + 4;
        if ty == *b"IEND" {
            break;
        }
        if pos >= data.len() {
            return Err("png: missing IEND".into());
        }
    }
    if pos != data.len() {
        return Err("png: bytes follow IEND".into());
    }
    Ok(chunks)
}
//#endregion ChunkIo
