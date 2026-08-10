//! ⚙️ BmpEngine — real bmp codec (BITMAPFILEHEADER + BITMAPINFOHEADER).
//!
//! Decode supports 1/4/8-bit indexed (BGR[A] palette), 16/32-bit `BI_BITFIELDS`, 24-bit
//! `BI_RGB`, and 32-bit `BI_RGB` (default full-byte channel masks) — always canonicalized into
//! an 8-bit RGBA `pixels` buffer (`width * height * 4` bytes, row 0 = image top). Encode always
//! emits 24-bit `BI_RGB`, bottom-up, uncompressed — see 🚫️EncodeScopeNote below.

use crate::artifacts::bmp::{BmpArtifact, BmpDiff, BmpMutation, BmpSnapshot, STDIO_BMP_DOCUMENT_SCHEMA};

//#region ByteIo
const BMP_MAGIC: [u8; 2] = *b"BM";

fn read_u16(b: &[u8], pos: usize) -> Result<u16, String> {
    b.get(pos..pos + 2).map(|s| u16::from_le_bytes([s[0], s[1]])).ok_or_else(|| "bmp: truncated (u16)".into())
}
fn read_u32(b: &[u8], pos: usize) -> Result<u32, String> {
    b.get(pos..pos + 4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]])).ok_or_else(|| "bmp: truncated (u32)".into())
}
fn read_i32(b: &[u8], pos: usize) -> Result<i32, String> {
    b.get(pos..pos + 4).map(|s| i32::from_le_bytes([s[0], s[1], s[2], s[3]])).ok_or_else(|| "bmp: truncated (i32)".into())
}
//#endregion ByteIo

//#region RowGeometry
/// 📏 BMP scanlines are padded to a 4-byte boundary: `((width*bpp + 31) / 32) * 4`.
fn row_bytes(width: u32, bpp: u16) -> usize {
    (((width as usize * bpp as usize) + 31) / 32) * 4
}
//#endregion RowGeometry

//#region Bitfields
/// 🧮 `(shift, bit-width)` of a contiguous bitfield mask, used to extract+normalize a channel.
fn mask_shift_width(mask: u32) -> (u32, u32) {
    if mask == 0 { return (0, 0); }
    let shift = mask.trailing_zeros();
    let width = (mask >> shift).trailing_ones();
    (shift, width)
}

fn extract_channel(raw: u32, mask: u32) -> u8 {
    let (shift, width) = mask_shift_width(mask);
    if width == 0 { return 0; }
    let v = (raw & mask) >> shift;
    if width >= 8 {
        (v >> (width - 8)) as u8
    } else {
        let maxval = (1u32 << width) - 1;
        ((v * 255 + maxval / 2) / maxval) as u8
    }
}
//#endregion Bitfields

//#region IndexUnpack
fn unpack_index(row: &[u8], x: usize, bpp: u16) -> usize {
    match bpp {
        8 => row[x] as usize,
        4 => {
            let byte = row[x / 2];
            if x % 2 == 0 { (byte >> 4) as usize } else { (byte & 0x0F) as usize }
        }
        1 => {
            let byte = row[x / 8];
            let bit = 7 - (x % 8);
            ((byte >> bit) & 1) as usize
        }
        _ => unreachable!("caller only passes 1|4|8"),
    }
}
//#endregion IndexUnpack

//#region Codec
pub fn decode_bmp(bytes: &[u8]) -> Result<BmpSnapshot, String> {
    if bytes.len() < 14 || bytes[0..2] != BMP_MAGIC {
        return Err("bmp: bad signature".into());
    }
    let data_offset = read_u32(bytes, 10)? as usize;
    let header_size = read_u32(bytes, 14)? as usize;
    if header_size < 40 {
        return Err(format!("bmp: unsupported info header size {header_size}"));
    }
    let width_i = read_i32(bytes, 18)?;
    let height_i = read_i32(bytes, 22)?;
    if width_i == 0 && height_i == 0 {
        // 🌱 The zero-dimension "empty document" case round-tripped by encode_bmp — no pixel
        // data to read, nothing else to validate.
        return Ok(BmpSnapshot { schema: STDIO_BMP_DOCUMENT_SCHEMA.into(), width: 0, height: 0, pixels: Vec::new() });
    }
    if width_i <= 0 {
        return Err("bmp: non-positive width".into());
    }
    if height_i == 0 {
        return Err("bmp: zero height".into());
    }
    let width = width_i as u32;
    let top_down = height_i < 0;
    let height = height_i.unsigned_abs();
    let bpp = read_u16(bytes, 28)?;
    let compression = read_u32(bytes, 30)?;
    let colors_used_field = read_u32(bytes, 46)?;

    if compression != 0 && compression != 3 {
        return Err(format!("bmp: unsupported compression {compression} (only BI_RGB/BI_BITFIELDS are implemented)"));
    }

    let mut cursor = 14 + header_size;
    let mut masks = [0u32; 4]; // r, g, b, a
    if compression == 3 {
        if bpp != 16 && bpp != 32 {
            return Err("bmp: BI_BITFIELDS only valid for 16/32bpp".into());
        }
        if header_size == 40 {
            // 📌 Classic Win9x extension: 3 (sometimes 4) DWORD masks immediately follow the
            // core 40-byte BITMAPINFOHEADER, before the pixel data.
            masks[0] = read_u32(bytes, cursor)?;
            masks[1] = read_u32(bytes, cursor + 4)?;
            masks[2] = read_u32(bytes, cursor + 8)?;
            cursor += 12;
            if cursor + 4 <= data_offset {
                masks[3] = read_u32(bytes, cursor)?;
                cursor += 4;
            }
        } else {
            // 📌 BITMAPV2/V3/V4/V5INFOHEADER embed the masks at fixed offsets inside the header.
            masks[0] = read_u32(bytes, 14 + 40)?;
            masks[1] = read_u32(bytes, 14 + 44)?;
            masks[2] = read_u32(bytes, 14 + 48)?;
            if header_size >= 56 {
                masks[3] = read_u32(bytes, 14 + 52)?;
            }
        }
    } else if bpp == 16 {
        masks = [0x7C00, 0x03E0, 0x001F, 0]; // BI_RGB default: X1R5G5B5
    } else if bpp == 32 {
        masks = [0x00FF0000, 0x0000FF00, 0x000000FF, 0]; // BI_RGB default: 8-8-8, no alpha
    }

    let palette_count = if bpp <= 8 {
        let raw = if colors_used_field != 0 { colors_used_field as usize } else { 1usize << bpp };
        if raw > 1usize << bpp {
            return Err("bmp: colorsUsed exceeds bit-depth capacity".into());
        }
        raw
    } else {
        0
    };
    let mut palette: Vec<[u8; 4]> = Vec::with_capacity(palette_count);
    for i in 0..palette_count {
        let o = cursor + i * 4;
        if o + 4 > bytes.len() || o + 4 > data_offset {
            return Err("bmp: palette truncated".into());
        }
        palette.push([bytes[o], bytes[o + 1], bytes[o + 2], bytes[o + 3]]);
    }

    let rb = row_bytes(width, bpp);
    let mut pixels = vec![0u8; width as usize * height as usize * 4];
    for file_row in 0..height as usize {
        let row_off = data_offset + file_row * rb;
        if row_off + rb > bytes.len() {
            return Err("bmp: pixel data truncated".into());
        }
        let row = &bytes[row_off..row_off + rb];
        let out_y = if top_down { file_row } else { height as usize - 1 - file_row };
        match bpp {
            1 | 4 | 8 => {
                for x in 0..width as usize {
                    let idx = unpack_index(row, x, bpp);
                    let entry = palette.get(idx).ok_or_else(|| format!("bmp: palette index {idx} out of range"))?;
                    let o = (out_y * width as usize + x) * 4;
                    pixels[o] = entry[2];
                    pixels[o + 1] = entry[1];
                    pixels[o + 2] = entry[0];
                    pixels[o + 3] = 255;
                }
            }
            24 => {
                for x in 0..width as usize {
                    let so = x * 3;
                    let o = (out_y * width as usize + x) * 4;
                    pixels[o] = row[so + 2];
                    pixels[o + 1] = row[so + 1];
                    pixels[o + 2] = row[so];
                    pixels[o + 3] = 255;
                }
            }
            16 => {
                for x in 0..width as usize {
                    let so = x * 2;
                    let raw = u16::from_le_bytes([row[so], row[so + 1]]) as u32;
                    let o = (out_y * width as usize + x) * 4;
                    pixels[o] = extract_channel(raw, masks[0]);
                    pixels[o + 1] = extract_channel(raw, masks[1]);
                    pixels[o + 2] = extract_channel(raw, masks[2]);
                    pixels[o + 3] = if masks[3] != 0 { extract_channel(raw, masks[3]) } else { 255 };
                }
            }
            32 => {
                for x in 0..width as usize {
                    let so = x * 4;
                    let raw = u32::from_le_bytes([row[so], row[so + 1], row[so + 2], row[so + 3]]);
                    let o = (out_y * width as usize + x) * 4;
                    pixels[o] = extract_channel(raw, masks[0]);
                    pixels[o + 1] = extract_channel(raw, masks[1]);
                    pixels[o + 2] = extract_channel(raw, masks[2]);
                    pixels[o + 3] = if masks[3] != 0 { extract_channel(raw, masks[3]) } else { 255 };
                }
            }
            _ => return Err(format!("bmp: unsupported bit depth {bpp}")),
        }
    }
    Ok(BmpSnapshot { schema: STDIO_BMP_DOCUMENT_SCHEMA.into(), width, height, pixels })
}

/// 🚫 EncodeScopeNote: always emits 24-bit `BI_RGB`, uncompressed, bottom-up row order.
/// `pixels` is treated as canonical 8-bit RGBA; encode drops the alpha channel (`BI_RGB` has
/// none) — only decode supports the wider palette/bitfields input diversity documented above.
/// A real implementation could reasonably restrict *encode* to 24/32-bit only while *decode*
/// covers every depth, which is exactly the scope cut made here.
pub fn encode_bmp(snap: &BmpSnapshot) -> Result<Vec<u8>, String> {
    let (w, h) = (snap.width, snap.height);
    let expected = w as usize * h as usize * 4;
    if snap.pixels.len() != expected {
        return Err("bmp: pixels length mismatch (expected width*height*4 RGBA)".into());
    }
    let rb = row_bytes(w, 24);
    let pixel_bytes = rb * h as usize;
    let file_size = 14 + 40 + pixel_bytes;
    let mut out = Vec::with_capacity(file_size);
    out.extend_from_slice(&BMP_MAGIC);
    out.extend_from_slice(&(file_size as u32).to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&54u32.to_le_bytes());
    out.extend_from_slice(&40u32.to_le_bytes());
    out.extend_from_slice(&(w as i32).to_le_bytes());
    out.extend_from_slice(&(h as i32).to_le_bytes()); // positive height => bottom-up
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&24u16.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // BI_RGB
    out.extend_from_slice(&(pixel_bytes as u32).to_le_bytes());
    out.extend_from_slice(&[0u8; 16]); // resolution x2, colorsUsed, colorsImportant
    for file_row in 0..h as usize {
        let src_y = h as usize - 1 - file_row;
        let mut row_buf = vec![0u8; rb];
        for x in 0..w as usize {
            let i = (src_y * w as usize + x) * 4;
            let o = x * 3;
            row_buf[o] = snap.pixels[i + 2];
            row_buf[o + 1] = snap.pixels[i + 1];
            row_buf[o + 2] = snap.pixels[i];
        }
        out.extend_from_slice(&row_buf);
    }
    Ok(out)
}

/// 🌱 Empty persisted snapshot.
pub fn empty_bmp_snapshot() -> BmpSnapshot {
    BmpSnapshot::default()
}
//#endregion Codec

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::bmp::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<BmpSnapshot, BmpMutation>(STDIO_BMP_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (bmp).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.bmp",
        extension: Some("bin"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::bmp::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::bmp::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::bmp::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::bmp::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.bmp"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.bmp`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::bmp::schema::bmp_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.bmp` artifact engine.
pub struct BmpEngine {
    artifact_state: BmpArtifact,
    snapshot_state: BmpSnapshot,
}

impl BmpEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: BmpSnapshot) -> Self {
        let artifact_state = BmpArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}

impl protocol::ArtifactEngine for BmpEngine {
    type Artifact = BmpArtifact;
    type Snapshot = BmpSnapshot;
    type Mutation = BmpMutation;
    type Diff = BmpDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact_state
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot_state
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn gradient_checkerboard_rgba(w: u32, h: u32) -> Vec<u8> {
        let mut out = Vec::with_capacity((w * h * 4) as usize);
        for y in 0..h {
            for x in 0..w {
                let checker = if (x + y) % 2 == 0 { 255u8 } else { 0u8 };
                out.extend_from_slice(&[checker, ((x * 37) % 256) as u8, ((y * 53) % 256) as u8, 255]);
            }
        }
        out
    }

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_bmp_snapshot();
        assert_eq!(snapshot.schema, STDIO_BMP_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_bmp_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <BmpSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <BmpSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region RowPadding
    /// 🔬 width=5 at 24bpp is 15 raw bytes/row, padded to 16 — a width that divides evenly would
    /// not catch a broken padding formula.
    #[test]
    fn row_bytes_padding_is_exact() {
        assert_eq!(row_bytes(5, 24), 16);
        assert_eq!(row_bytes(4, 24), 12);
        assert_eq!(row_bytes(1, 1), 4);
        assert_eq!(row_bytes(9, 1), 4);
        assert_eq!(row_bytes(5, 4), 4);
        assert_eq!(row_bytes(6, 8), 8);
        assert_eq!(row_bytes(3, 32), 12);
    }
    //#endregion RowPadding

    //#region TwentyFourBitRoundTrip
    /// 🔬 Load-bearing regression: non-solid 6x4 checkerboard/gradient through real 24-bit
    /// BI_RGB encode+decode, width chosen so raw row bytes (18) is NOT a multiple of 4 —
    /// exercises row padding on both the encode and decode sides.
    #[test]
    fn gradient_checkerboard_24bit_round_trip() {
        let (w, h) = (6u32, 4u32);
        let pixels = gradient_checkerboard_rgba(w, h);
        let snap = BmpSnapshot { schema: STDIO_BMP_DOCUMENT_SCHEMA.into(), width: w, height: h, pixels: pixels.clone() };
        let encoded = encode_bmp(&snap).expect("encode");
        // sanity: row padded to 4-byte boundary (6*3=18 raw -> 20 padded)
        assert_eq!(row_bytes(w, 24), 20);
        let decoded = decode_bmp(&encoded).expect("decode");
        assert_eq!(decoded.width, w);
        assert_eq!(decoded.height, h);
        assert_eq!(decoded.pixels, pixels, "decoded pixels must exactly match the original (alpha forced to 255)");
    }
    //#endregion TwentyFourBitRoundTrip

    //#region IndexedFixture
    /// 🧪 Hand-encodes a 4-bit indexed (16-color-capable, 4 used) BMP with a non-trivial
    /// checkerboard-ish index pattern and asserts `decode_bmp` reconstructs the exact palette
    /// colors — proves palette lookup + sub-byte (nibble) unpacking + bottom-up row order.
    fn hand_encode_indexed(width: u32, height: u32, bpp: u16, palette: &[[u8; 4]], indices: &[Vec<usize>]) -> Vec<u8> {
        let rb = row_bytes(width, bpp);
        let palette_bytes = palette.len() * 4;
        let data_offset = 14 + 40 + palette_bytes;
        let pixel_bytes = rb * height as usize;
        let file_size = data_offset + pixel_bytes;
        let mut out = Vec::with_capacity(file_size);
        out.extend_from_slice(&BMP_MAGIC);
        out.extend_from_slice(&(file_size as u32).to_le_bytes());
        out.extend_from_slice(&[0u8; 4]);
        out.extend_from_slice(&(data_offset as u32).to_le_bytes());
        out.extend_from_slice(&40u32.to_le_bytes());
        out.extend_from_slice(&(width as i32).to_le_bytes());
        out.extend_from_slice(&(height as i32).to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&bpp.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&(pixel_bytes as u32).to_le_bytes());
        out.extend_from_slice(&[0u8; 8]);
        out.extend_from_slice(&(palette.len() as u32).to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        for entry in palette {
            out.extend_from_slice(entry);
        }
        // indices[0] is the file's bottom-most (first-written) row per BMP's default bottom-up order
        for row_indices in indices {
            let mut row_buf = vec![0u8; rb];
            for (x, &idx) in row_indices.iter().enumerate() {
                match bpp {
                    8 => row_buf[x] = idx as u8,
                    4 => {
                        let byte = &mut row_buf[x / 2];
                        if x % 2 == 0 { *byte = (*byte & 0x0F) | ((idx as u8) << 4); } else { *byte = (*byte & 0xF0) | (idx as u8); }
                    }
                    1 => {
                        let byte = &mut row_buf[x / 8];
                        let bit = 7 - (x % 8);
                        if idx != 0 { *byte |= 1 << bit; }
                    }
                    _ => unreachable!(),
                }
            }
            out.extend_from_slice(&row_buf);
        }
        out
    }

    #[test]
    fn indexed_4bit_palette_round_trip() {
        // 5x3 image (row_bytes(5,4) = 4, NOT equal to raw 5*4bits/8=2.5->3 bytes — exercises padding),
        // palette of 4 colors, non-trivial (non-solid) index pattern.
        let palette = [
            [255u8, 0, 0, 0],   // B,G,R,pad -> red
            [0u8, 255, 0, 0],   // green
            [0u8, 0, 255, 0],   // blue
            [10u8, 20, 30, 0],  // arbitrary
        ];
        assert_eq!(row_bytes(5, 4), 4);
        // file rows are bottom-up; row 0 here = bottom of the image
        let file_rows = vec![
            vec![0usize, 1, 2, 3, 0], // bottom row (displayed last)
            vec![3usize, 2, 1, 0, 1],
            vec![1usize, 0, 3, 2, 3], // top row (displayed first)
        ];
        let bytes = hand_encode_indexed(5, 3, 4, &palette, &file_rows);
        let decoded = decode_bmp(&bytes).expect("decode 4-bit indexed");
        assert_eq!(decoded.width, 5);
        assert_eq!(decoded.height, 3);
        // top displayed row (out_y=0) must equal file_rows[2] (last file row, bottom-up)
        let expect_row = |row_indices: &[usize]| -> Vec<u8> {
            row_indices.iter().flat_map(|&i| {
                let e = palette[i];
                [e[2], e[1], e[0], 255]
            }).collect()
        };
        let top = &decoded.pixels[0..5 * 4];
        let mid = &decoded.pixels[5 * 4..10 * 4];
        let bottom = &decoded.pixels[10 * 4..15 * 4];
        assert_eq!(top, expect_row(&file_rows[2]).as_slice());
        assert_eq!(mid, expect_row(&file_rows[1]).as_slice());
        assert_eq!(bottom, expect_row(&file_rows[0]).as_slice());
    }
    //#endregion IndexedFixture

    //#region BitfieldsFixture
    /// 🧪 Hand-encodes a 16-bit `BI_BITFIELDS` (5-5-5) BMP and checks the classic "count
    /// trailing zeros then scale by mask bit-width" extraction is exact for known values.
    #[test]
    fn bitfields_16bit_555_round_trip() {
        let (w, h) = (4u32, 2u32);
        // masks: R=0x7C00 G=0x03E0 B=0x001F, no alpha
        let r_mask = 0x7C00u32;
        let g_mask = 0x03E0u32;
        let b_mask = 0x001Fu32;
        let pack = |r5: u16, g5: u16, b5: u16| -> u16 { (r5 << 10) | (g5 << 5) | b5 };
        // 4 distinct pixels per row, 2 rows — deliberately not a solid color.
        let raw_pixels: [[u16; 4]; 2] = [
            [pack(31, 0, 0), pack(0, 31, 0), pack(0, 0, 31), pack(31, 31, 31)],
            [pack(16, 8, 4), pack(4, 16, 8), pack(8, 4, 16), pack(0, 0, 0)],
        ];
        let rb = row_bytes(w, 16);
        assert_eq!(rb, 8); // 4px * 2bytes = 8, already 4-byte aligned
        let header_size = 40u32;
        let masks_size = 12usize;
        let data_offset = 14 + header_size as usize + masks_size;
        let pixel_bytes = rb * h as usize;
        let file_size = data_offset + pixel_bytes;
        let mut out = Vec::with_capacity(file_size);
        out.extend_from_slice(&BMP_MAGIC);
        out.extend_from_slice(&(file_size as u32).to_le_bytes());
        out.extend_from_slice(&[0u8; 4]);
        out.extend_from_slice(&(data_offset as u32).to_le_bytes());
        out.extend_from_slice(&header_size.to_le_bytes());
        out.extend_from_slice(&(w as i32).to_le_bytes());
        out.extend_from_slice(&(h as i32).to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&16u16.to_le_bytes());
        out.extend_from_slice(&3u32.to_le_bytes()); // BI_BITFIELDS
        out.extend_from_slice(&(pixel_bytes as u32).to_le_bytes());
        out.extend_from_slice(&[0u8; 16]);
        out.extend_from_slice(&r_mask.to_le_bytes());
        out.extend_from_slice(&g_mask.to_le_bytes());
        out.extend_from_slice(&b_mask.to_le_bytes());
        // file rows bottom-up: write row 1 (h-1) first, then row 0
        for file_row in 0..h as usize {
            let src_row = h as usize - 1 - file_row;
            for &px in &raw_pixels[src_row] {
                out.extend_from_slice(&px.to_le_bytes());
            }
        }
        let decoded = decode_bmp(&out).expect("decode 16-bit bitfields");
        assert_eq!(decoded.width, w);
        assert_eq!(decoded.height, h);
        // top displayed row (out_y=0) corresponds to raw_pixels[0]
        assert_eq!(&decoded.pixels[0..4 * 4], &[255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255][..]);
        // second row: verify scaled mid-intensity channel values (16/31*255 rounded = 132, 8/31*255=66, 4/31*255=33)
        let row2 = &decoded.pixels[4 * 4..8 * 4];
        assert_eq!(row2[0], 132); // r of pack(16,8,4)
        assert_eq!(row2[1], 66);  // g of pack(16,8,4)
        assert_eq!(row2[2], 33);  // b of pack(16,8,4)
        assert_eq!(row2[3], 255);
    }
    //#endregion BitfieldsFixture

    #[test]
    fn sniff_rejects_non_bmp_bytes() {
        let err = decode_bmp(b"not a bmp at all").unwrap_err();
        assert!(err.contains("signature"));
    }
}
//#endregion 🧪️Tests
