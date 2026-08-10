//! ⚙️ GifEngine — real GIF87a codec: variable 3–12 bit LZW, GCT/LCT, interlace de-row on decode.
//! The LZW core (`lzw_encode`/`lzw_decode`) and the surrounding byte-level helpers (sub-blocks,
//! color tables, RGBA quantization) are `pub` so the 89a standard's engine reuses them verbatim
//! instead of duplicating the codec (same "engine functions reused across dialects" shape zip
//! uses for deflate) — see `standards::v89a::engine`.

// 🔀️ S-6: `crate::artifacts::gif::schema` now shims to 89a (canonical) -- 87a's own engine uses
// its own standard-local schema path directly rather than the shared root re-export.
use crate::artifacts::gif::STDIO_GIF_DOCUMENT_SCHEMA;
use crate::artifacts::gif::standards::v87a::subsets::any::schema::{diff::GifDiff, mutations::GifMutation, snapshot::{GifSnapshot, RasterImage}, GifArtifact};
use std::collections::HashMap;

//#region BitIO
/// 📦️ LSB-first bit packer — GIF's LZW codes are packed least-significant-bit-first within each
/// byte (GIF89a Appendix F), the same convention TIFF's LZW variant uses.
struct BitWriter { out: Vec<u8>, cur: u32, nbits: u32 }
impl BitWriter {
    fn new() -> Self { Self { out: Vec::new(), cur: 0, nbits: 0 } }
    fn write_bits(&mut self, value: u32, count: u8) {
        self.cur |= value << self.nbits;
        self.nbits += count as u32;
        while self.nbits >= 8 {
            self.out.push((self.cur & 0xFF) as u8);
            self.cur >>= 8;
            self.nbits -= 8;
        }
    }
    fn finish(mut self) -> Vec<u8> {
        if self.nbits > 0 { self.out.push((self.cur & 0xFF) as u8); }
        self.out
    }
}

struct BitReader<'a> { data: &'a [u8], pos: usize, cur: u32, nbits: u32 }
impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self { Self { data, pos: 0, cur: 0, nbits: 0 } }
    fn read_bits(&mut self, count: u8) -> Result<u32, String> {
        while self.nbits < count as u32 {
            if self.pos >= self.data.len() { return Err("unexpected end of lzw stream".into()); }
            self.cur |= (self.data[self.pos] as u32) << self.nbits;
            self.pos += 1;
            self.nbits += 8;
        }
        let mask = (1u32 << count) - 1;
        let v = self.cur & mask;
        self.cur >>= count;
        self.nbits -= count as u32;
        Ok(v)
    }
}
//#endregion BitIO

//#region Lzw
/// 🧬️ GIF-variant LZW encode: variable 3–12 bit codes, clear/end-of-information codes, table
/// reset on overflow. Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION:
/// the growth threshold is deliberately ASYMMETRIC between this and [`lzw_decode`] (`>` here,
/// `>=` there) — that's not a stylistic choice, it's the real GIF/TIFF-LZW convention. The
/// encoder discovers a new dictionary string one input symbol ahead of when the decoder can
/// reconstruct the same entry (the decoder only learns a string once it has decoded the code
/// that follows it), so encoder growth must lag decoder growth by exactly one table slot or the
/// two sides desync mid-stream. Verified against `dancing.gif` decoded by Pillow (ground truth)
/// in a throwaway scratch-crate harness before porting here — a symmetric `>=`/`>=` pairing
/// passed self-consistency tests against its own output but produced invalid codes against a
/// real third-party-encoded file, exactly the kind of bug hand-tracing alone tends to miss.
pub fn lzw_encode(indices: &[u8], min_code_size: u8) -> Vec<u8> {
    let clear_code: u32 = 1 << min_code_size;
    let end_code: u32 = clear_code + 1;
    let mut code_size: u32 = min_code_size as u32 + 1;
    let mut next_code: u32 = end_code + 1;
    let mut dict: HashMap<(i64, u8), u32> = HashMap::new();
    let mut bw = BitWriter::new();
    bw.write_bits(clear_code, code_size as u8);
    if indices.is_empty() {
        bw.write_bits(end_code, code_size as u8);
        return bw.finish();
    }
    let mut current: i64 = indices[0] as i64;
    for &sym in &indices[1..] {
        let key = (current, sym);
        if let Some(&code) = dict.get(&key) {
            current = code as i64;
        } else {
            bw.write_bits(current as u32, code_size as u8);
            dict.insert(key, next_code);
            next_code += 1;
            if next_code > (1u32 << code_size) && code_size < 12 {
                code_size += 1;
            }
            if next_code >= 4096 {
                bw.write_bits(clear_code, code_size as u8);
                dict.clear();
                code_size = min_code_size as u32 + 1;
                next_code = end_code + 1;
            }
            current = sym as i64;
        }
    }
    bw.write_bits(current as u32, code_size as u8);
    bw.write_bits(end_code, code_size as u8);
    bw.finish()
}

/// 🧬️ GIF-variant LZW decode; see [`lzw_encode`] for the growth-threshold asymmetry.
pub fn lzw_decode(data: &[u8], min_code_size: u8) -> Result<Vec<u8>, String> {
    if !(2..=8).contains(&min_code_size) {
        return Err(format!("invalid lzw minimum code size {min_code_size} (must be 2..=8)"));
    }
    let clear_code: u32 = 1 << min_code_size;
    let end_code: u32 = clear_code + 1;
    let mut code_size: u32 = min_code_size as u32 + 1;
    let mut br = BitReader::new(data);
    let base_len = (clear_code + 2) as usize;
    let mut table: Vec<Vec<u8>> = (0..clear_code).map(|i| vec![i as u8]).collect();
    table.push(Vec::new());
    table.push(Vec::new());
    let mut out = Vec::new();
    let mut prev: Option<Vec<u8>> = None;
    loop {
        let code = br.read_bits(code_size as u8)?;
        if code == clear_code {
            table.truncate(base_len);
            code_size = min_code_size as u32 + 1;
            prev = None;
            continue;
        }
        if code == end_code {
            break;
        }
        let entry: Vec<u8> = if (code as usize) < table.len() {
            table[code as usize].clone()
        } else if code as usize == table.len() {
            let mut e = prev.clone().ok_or("invalid lzw stream: KwKwK code with no preceding entry")?;
            let first = e[0];
            e.push(first);
            e
        } else {
            return Err(format!("invalid lzw code {code} (table has {} entries)", table.len()));
        };
        out.extend_from_slice(&entry);
        if let Some(p) = prev {
            if table.len() < 4096 {
                let mut new_entry = p;
                new_entry.push(entry[0]);
                table.push(new_entry);
                if table.len() >= (1usize << code_size) && code_size < 12 {
                    code_size += 1;
                }
            }
        }
        prev = Some(entry);
    }
    Ok(out)
}
//#endregion Lzw

//#region SubBlocks
/// 📦️ GIF data sub-blocks: length-prefixed (max 255 bytes) chunks terminated by a zero-length
/// block — used for LZW image data and every extension body (GCE, application, comment, ...).
pub fn pack_sub_blocks(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() + data.len() / 255 + 2);
    for chunk in data.chunks(255) {
        out.push(chunk.len() as u8);
        out.extend_from_slice(chunk);
    }
    out.push(0);
    out
}

/// 📦️ Inverse of [`pack_sub_blocks`]; advances `pos` past the terminating zero-length block.
pub fn unpack_sub_blocks(data: &[u8], pos: &mut usize) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    loop {
        let len = *data.get(*pos).ok_or("truncated gif sub-block sequence")? as usize;
        *pos += 1;
        if len == 0 {
            break;
        }
        let end = *pos + len;
        if end > data.len() {
            return Err("truncated gif sub-block payload".into());
        }
        out.extend_from_slice(&data[*pos..end]);
        *pos = end;
    }
    Ok(out)
}
//#endregion SubBlocks

//#region ColorTable
pub type Rgb = [u8; 3];

/// 📐️ The 3-bit "size" field GIF stores for a color table of `len` entries: table size on disk
/// is always `2^(size+1)`, so this is the smallest `size` whose power covers `len`.
pub fn color_table_size_field(len: usize) -> u8 {
    let mut size = 0u8;
    while (1usize << (size as usize + 1)) < len.max(1) {
        size += 1;
    }
    size
}

pub fn read_color_table(data: &[u8], pos: &mut usize, size_field: u8) -> Result<Vec<Rgb>, String> {
    let n = 1usize << (size_field as usize + 1);
    let end = pos.checked_add(n * 3).ok_or("gif color table size overflow")?;
    if end > data.len() {
        return Err("truncated gif color table".into());
    }
    let mut table = Vec::with_capacity(n);
    for i in 0..n {
        let o = *pos + i * 3;
        table.push([data[o], data[o + 1], data[o + 2]]);
    }
    *pos = end;
    Ok(table)
}

/// 📐️ Writes `palette` padded to its `2^(size+1)` disk size with black filler entries — the
/// filler RGB values are never referenced by any index we emit, only present to satisfy the
/// fixed-power-of-two on-disk shape.
pub fn write_color_table(out: &mut Vec<u8>, palette: &[Rgb]) {
    let size_field = color_table_size_field(palette.len());
    let target = 1usize << (size_field as usize + 1);
    for i in 0..target {
        out.extend_from_slice(&palette.get(i).copied().unwrap_or([0, 0, 0]));
    }
}
//#endregion ColorTable

//#region Quantize
/// 📐️ Smallest legal LZW minimum code size (2..=8, GIF caps at 8 since a color index is a byte)
/// whose `2^bits` covers `palette_len` entries.
pub fn min_code_size_for(palette_len: usize) -> u8 {
    let mut bits = 2u8;
    while (1usize << bits) < palette_len.max(1) {
        bits += 1;
    }
    bits
}

/// 🎨️ Builds an indexed palette from RGBA pixels. Pixels with `alpha==0` map to a dedicated
/// reserved index that no opaque color ever shares — GIF has exactly one transparent index per
/// frame, so keying transparency off color value alone would corrupt any opaque pixel that
/// happens to share that RGB (e.g. opaque black colliding with a transparent pixel's undefined
/// placeholder color).
pub fn quantize_rgba(rgba: &[u8]) -> Result<(Vec<Rgb>, Vec<u8>, Option<u8>), String> {
    let has_transparent = rgba.chunks_exact(4).any(|px| px[3] == 0);
    let mut palette: Vec<Rgb> = Vec::new();
    let mut lookup: HashMap<Rgb, u8> = HashMap::new();
    let transparent_index = if has_transparent {
        palette.push([0, 0, 0]);
        Some(0u8)
    } else {
        None
    };
    let mut indices = Vec::with_capacity(rgba.len() / 4);
    for px in rgba.chunks_exact(4) {
        if px[3] == 0 {
            indices.push(transparent_index.expect("has_transparent implies Some"));
            continue;
        }
        let rgb: Rgb = [px[0], px[1], px[2]];
        let idx = if let Some(&i) = lookup.get(&rgb) {
            i
        } else {
            if palette.len() >= 256 {
                return Err("gif encode: more than 256 unique opaque colors (quantization unsupported)".into());
            }
            let i = palette.len() as u8;
            palette.push(rgb);
            lookup.insert(rgb, i);
            i
        };
        indices.push(idx);
    }
    Ok((palette, indices, transparent_index))
}

/// 🎨️ Inverse of [`quantize_rgba`]. Transparent-index pixels normalize to `[0,0,0,0]` — the
/// spec leaves that index's RGB entry undefined, so there is no canonical color to preserve.
pub fn indices_to_rgba(indices: &[u8], palette: &[Rgb], transparent_index: Option<u8>) -> Vec<u8> {
    let mut out = Vec::with_capacity(indices.len() * 4);
    for &idx in indices {
        if Some(idx) == transparent_index {
            out.extend_from_slice(&[0, 0, 0, 0]);
            continue;
        }
        let rgb = palette.get(idx as usize).copied().unwrap_or([0, 0, 0]);
        out.extend_from_slice(&[rgb[0], rgb[1], rgb[2], 255]);
    }
    out
}
//#endregion Quantize

//#region Interlace
/// 🪜️ GIF89a Appendix E interlace pass order (also honored by real-world 87a encoders): rows are
/// stored 0,8,16,...; 4,12,20,...; 2,6,10,...; 1,3,5,... in the compressed stream. We only ever
/// need to de-interlace on decode — [`encode_gif`]/89a's frame encoder always emit progressive
/// (non-interlaced) data, which is spec-legal and simpler while still losing no pixel data.
pub fn deinterlace_rows(rows: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0u8; width * height];
    let mut src_row = 0usize;
    for (start, step) in [(0usize, 8usize), (4, 8), (2, 4), (1, 2)] {
        let mut row = start;
        while row < height {
            let src_off = src_row * width;
            let dst_off = row * width;
            if src_off + width <= rows.len() && dst_off + width <= out.len() {
                out[dst_off..dst_off + width].copy_from_slice(&rows[src_off..src_off + width]);
            }
            src_row += 1;
            row += step;
        }
    }
    out
}
//#endregion Interlace

//#region Codec87a
/// 🔖️ GIF87a has no Graphic Control Extension, so it cannot express per-pixel transparency or
/// animation — encoding a snapshot with any `alpha==0` pixel is a structurally-valid-but-
/// unsupported-here input; callers that need transparency belong on the 89a standard instead.
pub fn encode_gif(snap: &GifSnapshot) -> Result<Vec<u8>, String> {
    let img = &snap.image;
    if img.width == 0 || img.height == 0 {
        return Err("empty image".into());
    }
    if img.rgba.len() != (img.width as usize) * (img.height as usize) * 4 {
        return Err("rgba length mismatch".into());
    }
    if img.width > 0xFFFF || img.height > 0xFFFF {
        return Err("gif87a: image dimensions exceed u16".into());
    }
    let (palette, indices, transparent_index) = quantize_rgba(&img.rgba)?;
    if transparent_index.is_some() {
        return Err("gif87a: transparency needs a Graphic Control Extension, which GIF87a does not have — use the 89a standard".into());
    }
    let w = img.width as u16;
    let h = img.height as u16;
    let min_code_size = min_code_size_for(palette.len());
    let mut out = b"GIF87a".to_vec();
    out.extend_from_slice(&w.to_le_bytes());
    out.extend_from_slice(&h.to_le_bytes());
    out.push(0x80 | color_table_size_field(palette.len()));
    out.push(0); // background color index
    out.push(0); // pixel aspect ratio
    write_color_table(&mut out, &palette);
    out.push(0x2C);
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&w.to_le_bytes());
    out.extend_from_slice(&h.to_le_bytes());
    out.push(0); // no local color table, not interlaced
    out.push(min_code_size);
    out.extend_from_slice(&pack_sub_blocks(&lzw_encode(&indices, min_code_size)));
    out.push(0x3B);
    Ok(out)
}

pub fn decode_gif(data: &[u8]) -> Result<GifSnapshot, String> {
    if data.len() < 13 || &data[0..6] != b"GIF87a" {
        return Err("not a GIF87a file (bad magic)".into());
    }
    let packed = data[10];
    let mut pos = 13usize;
    let gct = if (packed & 0x80) != 0 { Some(read_color_table(data, &mut pos, packed & 0x07)?) } else { None };

    loop {
        let b = *data.get(pos).ok_or("truncated gif87a: missing trailer")?;
        match b {
            0x2C => {
                if pos + 10 > data.len() {
                    return Err("truncated gif87a image descriptor".into());
                }
                let iw = u16::from_le_bytes([data[pos + 5], data[pos + 6]]) as u32;
                let ih = u16::from_le_bytes([data[pos + 7], data[pos + 8]]) as u32;
                let ipacked = data[pos + 9];
                let interlaced = (ipacked & 0x40) != 0;
                pos += 10;
                let local = if (ipacked & 0x80) != 0 { Some(read_color_table(data, &mut pos, ipacked & 0x07)?) } else { None };
                let palette = local.as_ref().or(gct.as_ref()).ok_or("gif87a: image has no color table (neither global nor local)")?;
                let min_code_size = *data.get(pos).ok_or("truncated gif87a: missing lzw minimum code size")?;
                pos += 1;
                let sub = unpack_sub_blocks(data, &mut pos)?;
                let mut indices = lzw_decode(&sub, min_code_size)?;
                let expected = (iw as usize) * (ih as usize);
                if indices.len() < expected {
                    return Err("gif87a: lzw stream decoded fewer pixels than the image needs".into());
                }
                indices.truncate(expected);
                if interlaced {
                    indices = deinterlace_rows(&indices, iw as usize, ih as usize);
                }
                let rgba = indices_to_rgba(&indices, palette, None);
                return Ok(GifSnapshot { schema: STDIO_GIF_DOCUMENT_SCHEMA.into(), image: RasterImage { width: iw, height: ih, rgba } });
            }
            0x21 => {
                return Err("gif87a: this file uses an extension block, a GIF89a-only feature — decode it via the 89a standard instead".into());
            }
            0x3B => return Err("gif87a: file has no image".into()),
            other => return Err(format!("gif87a: unexpected block introducer {other:#04x}")),
        }
    }
}
//#endregion Codec87a

pub fn empty_gif_snapshot() -> GifSnapshot { GifSnapshot::default() }

//#region Sniff
/// 🔍️ Real magic-byte sniffing, shared by every `sniff()` this ticket touches across 87a/89a —
/// binary sources check the raw header directly; text (hex DSL) sources strip whatever preamble
/// `parse_dsl` would strip and hex-decode just the 6-byte prefix. `magic` is the standard's own
/// version string (`GIF87a`/`GIF89a`); a mismatch or too-short/malformed source is Low, never a
/// constant — replaces the prior stub that discarded `source` and always answered `Medium`.
pub fn sniff_magic(source: &semio_framework_plugin::AnalyzeSource<'_>, magic: &[u8; 6]) -> semio_framework_plugin::IoConfidence {
    use semio_framework_plugin::{AnalyzeSource, IoConfidence};
    match source {
        AnalyzeSource::Binary(bytes) => {
            if bytes.len() >= 6 && &bytes[0..6] == magic { IoConfidence::High } else { IoConfidence::Low }
        }
        AnalyzeSource::Text(text) => {
            let body = match store::semio_format::split_text_preamble(text) {
                Ok((_, rest)) => rest,
                Err(_) => text,
            };
            let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(12).collect();
            if hex.len() < 12 {
                return IoConfidence::Low;
            }
            let mut bytes = [0u8; 6];
            for (i, byte) in bytes.iter_mut().enumerate() {
                match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                    Ok(b) => *byte = b,
                    Err(_) => return IoConfidence::Low,
                }
            }
            if &bytes == magic { IoConfidence::Medium } else { IoConfidence::Low }
        }
    }
}
//#endregion Sniff

pub fn register() {
    crate::artifacts::gif::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::gif::standards::v87a::subsets::any::schema::gif_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<GifSnapshot, GifMutation>(STDIO_GIF_DOCUMENT_SCHEMA));
}

pub struct GifEngine { artifact_state: GifArtifact, snapshot_state: GifSnapshot }
impl GifEngine {
    pub fn new(snapshot: GifSnapshot) -> Self {
        Self { artifact_state: GifArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn checkerboard(width: u32, height: u32) -> RasterImage {
        let mut rgba = vec![0u8; (width * height * 4) as usize];
        for y in 0..height {
            for x in 0..width {
                let o = ((y * width + x) * 4) as usize;
                let on = (x + y) % 2 == 0;
                rgba[o] = if on { 255 } else { 10 };
                rgba[o + 1] = if on { 0 } else { 200 };
                rgba[o + 2] = if on { 0 } else { 30 };
                rgba[o + 3] = 255;
            }
        }
        RasterImage { width, height, rgba }
    }

    /// 🧪️ LZW core: trivial round trip at the smallest legal minimum code size.
    #[test]
    fn lzw_round_trip_trivial() {
        let indices = vec![1u8, 2, 1, 2, 1, 2, 3, 3, 3, 3];
        let enc = lzw_encode(&indices, 2);
        let dec = lzw_decode(&enc, 2).expect("decode");
        assert_eq!(dec, indices);
    }

    /// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: an
    /// all-one-color-ish long run at min_code_size=8 forces the dictionary well past the 8-bit
    /// boundary, exercising the asymmetric growth-threshold rule documented on `lzw_encode`.
    #[test]
    fn lzw_round_trip_forces_code_size_growth() {
        let indices: Vec<u8> = (0..5000).map(|i| (i % 2) as u8).collect();
        let enc = lzw_encode(&indices, 8);
        assert!(enc.len() < indices.len(), "highly repetitive data must compress");
        let dec = lzw_decode(&enc, 8).expect("decode");
        assert_eq!(dec, indices);
    }

    /// 🧪️ A single solid color run drives the dictionary to grow every entry from one repeated
    /// symbol — the worst case for the KwKwK (code == table length) decode branch.
    #[test]
    fn lzw_round_trip_solid_run_and_kwkwk() {
        let indices = vec![7u8; 20_000];
        let enc = lzw_encode(&indices, 8);
        let dec = lzw_decode(&enc, 8).expect("decode");
        assert_eq!(dec, indices);
        assert!(enc.len() < indices.len() / 10);
    }

    /// 🧪️ Pseudo-random data at every legal minimum code size (2..=8), large enough to cross
    /// multiple code-size growth boundaries and at least one dictionary-full clear-code reset.
    #[test]
    fn lzw_round_trip_pseudo_random_all_min_code_sizes() {
        for mcs in 2u8..=8 {
            let max_sym = (1u32 << mcs) - 1;
            let mut indices = Vec::new();
            let mut state = 12345u32;
            for _ in 0..60_000 {
                state = state.wrapping_mul(1103515245).wrapping_add(12345);
                indices.push(((state >> 16) % (max_sym + 1)) as u8);
            }
            let enc = lzw_encode(&indices, mcs);
            let dec = lzw_decode(&enc, mcs).unwrap_or_else(|e| panic!("min_code_size={mcs}: {e}"));
            assert_eq!(dec, indices, "min_code_size={mcs}");
        }
    }

    #[test]
    fn lzw_round_trip_empty_and_single_symbol() {
        assert_eq!(lzw_decode(&lzw_encode(&[], 2), 2).unwrap(), Vec::<u8>::new());
        assert_eq!(lzw_decode(&lzw_encode(&[3], 4), 4).unwrap(), vec![3u8]);
    }

    /// 🧪️ `decode_gif` must reject truncated/invalid input with a typed `Err`, never fabricate
    /// pixels — regression guard for the prior stub which silently produced an all-black image.
    #[test]
    fn decode_gif_rejects_garbage() {
        assert!(decode_gif(b"not a gif at all").is_err());
        assert!(decode_gif(b"GIF89a").is_err(), "87a decoder must reject 89a magic");
    }

    /// 🧪️ Full byte-level codec round trip through a real (non-solid) checkerboard image,
    /// exercising quantization, GCT sizing, and the sub-block-packed LZW stream together.
    #[test]
    fn encode_decode_round_trip_checkerboard() {
        let image = checkerboard(37, 29);
        let snap = GifSnapshot { schema: STDIO_GIF_DOCUMENT_SCHEMA.into(), image: image.clone() };
        let bytes = encode_gif(&snap).expect("encode");
        assert_eq!(&bytes[0..6], b"GIF87a");
        let decoded = decode_gif(&bytes).expect("decode");
        assert_eq!(decoded.image, image);
    }

    /// 🧪️ decode(encode(decode(x))) snapshot equality — the acceptance bar from the plan's
    /// fixtures section (model equality across a second round trip, not necessarily byte-exact).
    #[test]
    fn encode_decode_encode_decode_is_stable() {
        let image = checkerboard(9, 13);
        let snap = GifSnapshot { schema: STDIO_GIF_DOCUMENT_SCHEMA.into(), image };
        let once = decode_gif(&encode_gif(&snap).unwrap()).unwrap();
        let twice = decode_gif(&encode_gif(&once).unwrap()).unwrap();
        assert_eq!(once.image, twice.image);
    }

    /// 🧪️ GIF87a has no Graphic Control Extension — encoding a snapshot with a transparent
    /// pixel must be a typed error, not silently dropped alpha.
    #[test]
    fn encode_gif_rejects_transparency() {
        let mut image = checkerboard(4, 4);
        image.rgba[3] = 0; // make the first pixel transparent
        let snap = GifSnapshot { schema: STDIO_GIF_DOCUMENT_SCHEMA.into(), image };
        assert!(encode_gif(&snap).is_err());
    }

    #[test]
    fn interlace_round_trip() {
        let width = 5usize;
        let height = 9usize;
        let rows: Vec<u8> = (0..(width * height) as u32).map(|i| (i % 251) as u8).collect();
        // Interlace the rows using the same pass order deinterlace_rows expects to invert.
        let mut interlaced = vec![0u8; width * height];
        let mut dst = 0usize;
        for (start, step) in [(0usize, 8usize), (4, 8), (2, 4), (1, 2)] {
            let mut row = start;
            while row < height {
                interlaced[dst * width..dst * width + width].copy_from_slice(&rows[row * width..row * width + width]);
                dst += 1;
                row += step;
            }
        }
        let restored = deinterlace_rows(&interlaced, width, height);
        assert_eq!(restored, rows);
    }
}
//#endregion Tests
