//! ⚙️ PngEngine — real png codec.
//!
//! Decode supports the full §11 chunk set needed for real-world files (all 5 color
//! types, bit depths 1/2/4/8/16, PLTE/tRNS, Adam7 interlacing) and always canonicalizes
//! into `RasterImage`'s 8-bit RGBA. Encode always emits color type 6 / bit depth 8 /
//! interlace method 0 — see 🚫️EncodeScopeNote below.

use crate::artifacts::png::{schema::snapshot::RasterImage, PngArtifact, PngDiff, PngMutation, PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

//#region Signature
const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
//#endregion Signature

//#region Crc
fn png_crc32(data: &[u8]) -> u32 {
    crate::artifacts::zip::engine::crc32(data)
}
//#endregion Crc

//#region ChunkIo
fn write_chunk(out: &mut Vec<u8>, ty: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(ty);
    out.extend_from_slice(data);
    let mut crc_in = Vec::new();
    crc_in.extend_from_slice(ty);
    crc_in.extend_from_slice(data);
    out.extend_from_slice(&png_crc32(&crc_in).to_be_bytes());
}

/// 📖 Splits a PNG byte stream into `(type, data)` chunks, rejecting CRC mismatches and
/// truncation up front so downstream decode logic never has to re-check framing.
fn read_chunks(data: &[u8]) -> Result<Vec<([u8; 4], &[u8])>, String> {
    if data.len() < 8 || data[0..8] != PNG_SIGNATURE {
        return Err("png: bad signature".into());
    }
    let mut pos = 8usize;
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
        let mut crc_in = Vec::with_capacity(4 + len);
        crc_in.extend_from_slice(&ty);
        crc_in.extend_from_slice(chunk_data);
        if png_crc32(&crc_in) != stored_crc {
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
//#endregion ChunkIo

//#region Ihdr
struct Ihdr {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    interlace: u8,
}

fn parse_ihdr(data: &[u8]) -> Result<Ihdr, String> {
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

fn samples_per_pixel(color_type: u8) -> usize {
    match color_type {
        0 => 1,
        2 => 3,
        3 => 1,
        4 => 2,
        6 => 4,
        _ => unreachable!("validated in parse_ihdr"),
    }
}

fn bpp_bytes(ihdr: &Ihdr) -> usize {
    ((samples_per_pixel(ihdr.color_type) * ihdr.bit_depth as usize + 7) / 8).max(1)
}

fn packed_row_bytes(width: u32, color_type: u8, bit_depth: u8) -> usize {
    let bits = width as usize * samples_per_pixel(color_type) * bit_depth as usize;
    (bits + 7) / 8
}
//#endregion Ihdr

//#region Filter
fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let (a, b, c) = (a as i32, b as i32, c as i32);
    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();
    if pa <= pb && pa <= pc { a as u8 } else if pb <= pc { b as u8 } else { c as u8 }
}

fn filter_row(filter_type: u8, cur: &[u8], prev: Option<&[u8]>, bpp: usize) -> Vec<u8> {
    let mut out = vec![0u8; cur.len()];
    for x in 0..cur.len() {
        let a = if x >= bpp { cur[x - bpp] } else { 0 };
        let b = prev.map(|p| p[x]).unwrap_or(0);
        let c = if x >= bpp { prev.map(|p| p[x - bpp]).unwrap_or(0) } else { 0 };
        out[x] = match filter_type {
            0 => cur[x],
            1 => cur[x].wrapping_sub(a),
            2 => cur[x].wrapping_sub(b),
            3 => cur[x].wrapping_sub(((a as u16 + b as u16) / 2) as u8),
            4 => cur[x].wrapping_sub(paeth(a, b, c)),
            _ => unreachable!("caller only passes 0..=4"),
        };
    }
    out
}

fn defilter_row(filter_type: u8, filt: &[u8], prev: Option<&[u8]>, bpp: usize) -> Result<Vec<u8>, String> {
    if filter_type > 4 {
        return Err(format!("png: unsupported filter type {filter_type}"));
    }
    let mut out = vec![0u8; filt.len()];
    for x in 0..filt.len() {
        let a = if x >= bpp { out[x - bpp] } else { 0 };
        let b = prev.map(|p| p[x]).unwrap_or(0);
        let c = if x >= bpp { prev.map(|p| p[x - bpp]).unwrap_or(0) } else { 0 };
        out[x] = match filter_type {
            0 => filt[x],
            1 => filt[x].wrapping_add(a),
            2 => filt[x].wrapping_add(b),
            3 => filt[x].wrapping_add(((a as u16 + b as u16) / 2) as u8),
            4 => filt[x].wrapping_add(paeth(a, b, c)),
            _ => unreachable!("checked above"),
        };
    }
    Ok(out)
}

/// 🧮 Minimum-sum-of-absolute-values heuristic (bytes read as signed), the common
/// real-world choice per PNG spec §9.8 — not optimal, but genuinely per-scanline-adaptive.
fn choose_filter(cur: &[u8], prev: Option<&[u8]>, bpp: usize) -> (u8, Vec<u8>) {
    let mut best_ft = 0u8;
    let mut best_sum = i64::MAX;
    let mut best = Vec::new();
    for ft in 0u8..=4 {
        let f = filter_row(ft, cur, prev, bpp);
        let sum: i64 = f.iter().map(|&b| (b as i8).unsigned_abs() as i64).sum();
        if sum < best_sum {
            best_sum = sum;
            best_ft = ft;
            best = f;
        }
    }
    (best_ft, best)
}

fn defilter_pass(raw: &[u8], mut pos: usize, height: u32, row_bytes: usize, bpp: usize) -> Result<(Vec<Vec<u8>>, usize), String> {
    let mut rows = Vec::with_capacity(height as usize);
    let mut prev: Option<Vec<u8>> = None;
    for _ in 0..height {
        if pos >= raw.len() {
            return Err("png: truncated scanline data".into());
        }
        let ft = raw[pos];
        pos += 1;
        if pos + row_bytes > raw.len() {
            return Err("png: truncated scanline data".into());
        }
        let filt = &raw[pos..pos + row_bytes];
        pos += row_bytes;
        let recon = defilter_row(ft, filt, prev.as_deref(), bpp)?;
        prev = Some(recon.clone());
        rows.push(recon);
    }
    Ok((rows, pos))
}
//#endregion Filter

//#region Adam7
/// 🪜 Pass geometry `(start_x, start_y, step_x, step_y)`, PNG spec §8.2.
const ADAM7: [(u32, u32, u32, u32); 7] = [
    (0, 0, 8, 8),
    (4, 0, 8, 8),
    (0, 4, 4, 8),
    (2, 0, 4, 4),
    (0, 2, 2, 4),
    (1, 0, 2, 2),
    (0, 1, 1, 2),
];

fn adam7_pass_dims(width: u32, height: u32, pass: usize) -> (u32, u32) {
    let (sx, sy, stx, sty) = ADAM7[pass];
    let w = if width > sx { (width - sx + stx - 1) / stx } else { 0 };
    let h = if height > sy { (height - sy + sty - 1) / sty } else { 0 };
    (w, h)
}
//#endregion Adam7

//#region Unpack
fn unpack_samples(row: &[u8], width: usize, spp: usize, bit_depth: u8) -> Vec<u32> {
    let count = width * spp;
    let mut out = Vec::with_capacity(count);
    if bit_depth == 16 {
        for i in 0..count {
            out.push(((row[i * 2] as u32) << 8) | row[i * 2 + 1] as u32);
        }
    } else if bit_depth == 8 {
        for i in 0..count {
            out.push(row[i] as u32);
        }
    } else {
        let mut bitpos = 0usize;
        for _ in 0..count {
            let mut v = 0u32;
            for _ in 0..bit_depth {
                let byte = row[bitpos / 8];
                let bit = (byte >> (7 - (bitpos % 8))) & 1;
                v = (v << 1) | bit as u32;
                bitpos += 1;
            }
            out.push(v);
        }
    }
    out
}

fn scale_to_8(sample: u32, bit_depth: u8) -> u8 {
    match bit_depth {
        8 => sample as u8,
        16 => (sample >> 8) as u8,
        _ => {
            let maxval = (1u32 << bit_depth) - 1;
            ((sample * 255 + maxval / 2) / maxval) as u8
        }
    }
}

/// 🎨 Converts one pixel's raw (unscaled) samples to 8-bit RGBA using PLTE/tRNS as needed.
fn pixel_to_rgba(
    samples: &[u32],
    ihdr: &Ihdr,
    palette: &[[u8; 3]],
    palette_alpha: &[u8],
    gray_trans: Option<u32>,
    rgb_trans: Option<(u32, u32, u32)>,
) -> Result<[u8; 4], String> {
    match ihdr.color_type {
        0 => {
            let g = samples[0];
            let a = if gray_trans == Some(g) { 0 } else { 255 };
            let g8 = scale_to_8(g, ihdr.bit_depth);
            Ok([g8, g8, g8, a])
        }
        2 => {
            let (r, g, b) = (samples[0], samples[1], samples[2]);
            let a = if rgb_trans == Some((r, g, b)) { 0 } else { 255 };
            Ok([scale_to_8(r, ihdr.bit_depth), scale_to_8(g, ihdr.bit_depth), scale_to_8(b, ihdr.bit_depth), a])
        }
        3 => {
            let idx = samples[0] as usize;
            let rgb = palette.get(idx).ok_or_else(|| format!("png: palette index {idx} out of range"))?;
            let a = palette_alpha.get(idx).copied().unwrap_or(255);
            Ok([rgb[0], rgb[1], rgb[2], a])
        }
        4 => {
            let g8 = scale_to_8(samples[0], ihdr.bit_depth);
            let a8 = scale_to_8(samples[1], ihdr.bit_depth);
            Ok([g8, g8, g8, a8])
        }
        6 => Ok([
            scale_to_8(samples[0], ihdr.bit_depth),
            scale_to_8(samples[1], ihdr.bit_depth),
            scale_to_8(samples[2], ihdr.bit_depth),
            scale_to_8(samples[3], ihdr.bit_depth),
        ]),
        _ => unreachable!("validated in parse_ihdr"),
    }
}
//#endregion Unpack

//#region Codec
pub fn rgba_len(img: &RasterImage) -> Result<usize, String> {
    let n = (img.width as usize).checked_mul(img.height as usize).and_then(|p| p.checked_mul(4))
        .ok_or("dimensions overflow")?;
    if img.rgba.len() != n { return Err("rgba length mismatch".into()); }
    Ok(n)
}

/// 🚫 EncodeScopeNote: always emits color type 6 (RGBA) / bit depth 8 / interlace method 0.
/// `RasterImage` is a canonical 8-bit-RGBA model, so re-encoding a decoded palette/grayscale/
/// 16-bit/interlaced source will not byte-for-byte round-trip the original file — only its
/// pixel content. Decode (below) fully supports the input diversity; only encode canonicalizes.
pub fn encode_png(snap: &PngSnapshot) -> Result<Vec<u8>, String> {
    let img = &snap.image;
    rgba_len(img)?;
    let bpp = 4usize;
    let row_bytes = img.width as usize * bpp;
    let mut idat = Vec::with_capacity((row_bytes + 1) * img.height as usize);
    let mut prev: Option<Vec<u8>> = None;
    for y in 0..img.height as usize {
        let row = &img.rgba[y * row_bytes..(y + 1) * row_bytes];
        let (ft, filtered) = choose_filter(row, prev.as_deref(), bpp);
        idat.push(ft);
        idat.extend_from_slice(&filtered);
        prev = Some(row.to_vec());
    }
    let compressed = crate::artifacts::deflate::engine::zlib_compress(&idat)?;
    let mut out = Vec::new();
    out.extend_from_slice(&PNG_SIGNATURE);
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&img.width.to_be_bytes());
    ihdr.extend_from_slice(&img.height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    write_chunk(&mut out, b"IHDR", &ihdr);
    write_chunk(&mut out, b"IDAT", &compressed);
    write_chunk(&mut out, b"IEND", &[]);
    Ok(out)
}

pub fn decode_png(data: &[u8]) -> Result<PngSnapshot, String> {
    let chunks = read_chunks(data)?;
    let mut ihdr: Option<Ihdr> = None;
    let mut palette: Vec<[u8; 3]> = Vec::new();
    let mut palette_alpha: Vec<u8> = Vec::new();
    let mut gray_trans: Option<u32> = None;
    let mut rgb_trans: Option<(u32, u32, u32)> = None;
    let mut idat = Vec::new();
    let mut seen_idat = false;

    for &(ty, chunk) in &chunks {
        if ty == *b"IHDR" {
            ihdr = Some(parse_ihdr(chunk)?);
        } else if ty == *b"PLTE" {
            if chunk.len() % 3 != 0 { return Err("png PLTE: length not a multiple of 3".into()); }
            palette = chunk.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect();
        } else if ty == *b"tRNS" {
            let color_type = ihdr.as_ref().ok_or("png: tRNS before IHDR")?.color_type;
            match color_type {
                0 => {
                    if chunk.len() != 2 { return Err("png tRNS: expected 2 bytes for grayscale".into()); }
                    gray_trans = Some(u16::from_be_bytes([chunk[0], chunk[1]]) as u32);
                }
                2 => {
                    if chunk.len() != 6 { return Err("png tRNS: expected 6 bytes for truecolor".into()); }
                    rgb_trans = Some((
                        u16::from_be_bytes([chunk[0], chunk[1]]) as u32,
                        u16::from_be_bytes([chunk[2], chunk[3]]) as u32,
                        u16::from_be_bytes([chunk[4], chunk[5]]) as u32,
                    ));
                }
                3 => palette_alpha = chunk.to_vec(),
                _ => {} // spec: tRNS shall not appear for 4/6 (already carry alpha) — ignore rather than fail
            }
        } else if ty == *b"IDAT" {
            idat.extend_from_slice(chunk);
            seen_idat = true;
        } else if ty == *b"IEND" {
            // terminal marker, nothing to collect
        } else if ty[0].is_ascii_uppercase() {
            return Err(format!("png: unsupported critical chunk {}", String::from_utf8_lossy(&ty)));
        }
        // ancillary chunks (gAMA/cHRM/sRGB/tEXt/pHYs/...) are safe to skip per spec §5.4;
        // not preserved by RasterImage's canonical-RGBA model.
    }

    let ihdr = ihdr.ok_or("png: missing IHDR")?;
    if !seen_idat { return Err("png: missing IDAT".into()); }
    if ihdr.color_type == 3 && palette.is_empty() { return Err("png: color type 3 requires PLTE".into()); }

    let raw = crate::artifacts::deflate::engine::zlib_decompress(&idat)?;
    let spp = samples_per_pixel(ihdr.color_type);
    let bpp = bpp_bytes(&ihdr);
    let mut rgba = vec![0u8; ihdr.width as usize * ihdr.height as usize * 4];

    let mut put_row = |samples: &[u32], row_width: usize, base_x: u32, base_y: u32, step_x: u32| -> Result<(), String> {
        for i in 0..row_width {
            let px = pixel_to_rgba(&samples[i * spp..i * spp + spp], &ihdr, &palette, &palette_alpha, gray_trans, rgb_trans)?;
            let x = base_x + i as u32 * step_x;
            let idx = (base_y as usize * ihdr.width as usize + x as usize) * 4;
            rgba[idx..idx + 4].copy_from_slice(&px);
        }
        Ok(())
    };

    if ihdr.interlace == 0 {
        let row_bytes = packed_row_bytes(ihdr.width, ihdr.color_type, ihdr.bit_depth);
        let (rows, _) = defilter_pass(&raw, 0, ihdr.height, row_bytes, bpp)?;
        for (y, row) in rows.iter().enumerate() {
            let samples = unpack_samples(row, ihdr.width as usize, spp, ihdr.bit_depth);
            put_row(&samples, ihdr.width as usize, 0, y as u32, 1)?;
        }
    } else {
        let mut pos = 0usize;
        for pass in 0..7 {
            let (pw, ph) = adam7_pass_dims(ihdr.width, ihdr.height, pass);
            if pw == 0 || ph == 0 { continue; }
            let row_bytes = packed_row_bytes(pw, ihdr.color_type, ihdr.bit_depth);
            let (rows, new_pos) = defilter_pass(&raw, pos, ph, row_bytes, bpp)?;
            pos = new_pos;
            let (sx, sy, stx, sty) = ADAM7[pass];
            for (j, row) in rows.iter().enumerate() {
                let samples = unpack_samples(row, pw as usize, spp, ihdr.bit_depth);
                put_row(&samples, pw as usize, sx, sy + j as u32 * sty, stx)?;
            }
        }
    }

    Ok(PngSnapshot { schema: STDIO_PNG_DOCUMENT_SCHEMA.into(), image: RasterImage { width: ihdr.width, height: ihdr.height, rgba } })
}

pub fn empty_png_snapshot() -> PngSnapshot { PngSnapshot::default() }
//#endregion Codec

//#region Registration
pub fn register() {
    crate::artifacts::png::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::png::schema::png_artifact_schema_descriptor());
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.png", extension: Some("png"), role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::png::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::png::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::png::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::png::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.png"),
    });
    store::register_document_codec(store::ArtifactCodec::of::<PngSnapshot, PngMutation>(STDIO_PNG_DOCUMENT_SCHEMA));
}

pub struct PngEngine { artifact_state: PngArtifact, snapshot_state: PngSnapshot }
impl PngEngine {
    pub fn new(snapshot: PngSnapshot) -> Self {
        Self { artifact_state: PngArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
//#endregion Registration

//#region EngineTests
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

    /// 🔬 The load-bearing regression test: a non-solid image round-tripped through real
    /// encode (per-scanline filter selection) and real decode (filter reconstruction).
    /// Under the old always-filter-0 encode + no-reconstruction decode this still happened
    /// to pass trivially only for solid colors — a gradient/checkerboard is what exposes it.
    #[test]
    fn gradient_checkerboard_round_trip() {
        let (w, h) = (17u32, 13u32);
        let rgba = gradient_checkerboard_rgba(w, h);
        let snap = PngSnapshot { schema: STDIO_PNG_DOCUMENT_SCHEMA.into(), image: RasterImage { width: w, height: h, rgba: rgba.clone() } };
        let encoded = encode_png(&snap).expect("encode");
        let decoded = decode_png(&encoded).expect("decode");
        assert_eq!(decoded.image.width, w);
        assert_eq!(decoded.image.height, h);
        assert_eq!(decoded.image.rgba, rgba, "decoded pixels must exactly match the original");
    }

    #[test]
    fn solid_color_round_trip_still_works() {
        let (w, h) = (4u32, 4u32);
        let rgba: Vec<u8> = (0..w * h).flat_map(|_| [10u8, 20, 30, 255]).collect();
        let snap = PngSnapshot { schema: STDIO_PNG_DOCUMENT_SCHEMA.into(), image: RasterImage { width: w, height: h, rgba: rgba.clone() } };
        let encoded = encode_png(&snap).expect("encode");
        let decoded = decode_png(&encoded).expect("decode");
        assert_eq!(decoded.image.rgba, rgba);
    }

    #[test]
    fn crc_mismatch_is_rejected() {
        let (w, h) = (2u32, 2u32);
        let rgba = gradient_checkerboard_rgba(w, h);
        let snap = PngSnapshot { schema: STDIO_PNG_DOCUMENT_SCHEMA.into(), image: RasterImage { width: w, height: h, rgba } };
        let mut encoded = encode_png(&snap).expect("encode");
        let flip_at = 8 + 4 + 4 + 6; // a few bytes into the IHDR chunk's data
        encoded[flip_at] ^= 0xFF;
        let err = decode_png(&encoded).unwrap_err();
        assert!(err.contains("CRC") || err.contains("crc") || err.contains("truncated"), "unexpected error: {err}");
    }

    #[test]
    fn sniff_rejects_non_png_bytes() {
        let err = decode_png(b"not a png at all").unwrap_err();
        assert!(err.contains("signature"));
    }

    //#region ColorTypeFixtures
    fn hand_encode(width: u32, height: u32, bit_depth: u8, color_type: u8, plte: Option<&[u8]>, trns: Option<&[u8]>, raw_rows: &[u8]) -> Vec<u8> {
        let bpp = bpp_bytes(&Ihdr { width, height, bit_depth, color_type, interlace: 0 });
        let row_bytes = packed_row_bytes(width, color_type, bit_depth);
        assert_eq!(raw_rows.len(), row_bytes * height as usize);
        let mut idat = Vec::new();
        let mut prev: Option<Vec<u8>> = None;
        for y in 0..height as usize {
            let row = &raw_rows[y * row_bytes..(y + 1) * row_bytes];
            let (ft, filtered) = choose_filter(row, prev.as_deref(), bpp);
            idat.push(ft);
            idat.extend_from_slice(&filtered);
            prev = Some(row.to_vec());
        }
        let compressed = crate::artifacts::deflate::engine::zlib_compress(&idat).unwrap();
        let mut out = Vec::new();
        out.extend_from_slice(&PNG_SIGNATURE);
        let mut ihdr = Vec::with_capacity(13);
        ihdr.extend_from_slice(&width.to_be_bytes());
        ihdr.extend_from_slice(&height.to_be_bytes());
        ihdr.extend_from_slice(&[bit_depth, color_type, 0, 0, 0]);
        write_chunk(&mut out, b"IHDR", &ihdr);
        if let Some(p) = plte { write_chunk(&mut out, b"PLTE", p); }
        if let Some(t) = trns { write_chunk(&mut out, b"tRNS", t); }
        write_chunk(&mut out, b"IDAT", &compressed);
        write_chunk(&mut out, b"IEND", &[]);
        out
    }

    #[test]
    fn color_type_0_grayscale() {
        // 4x1, bit depth 8: values 0, 85, 170, 255
        let raw = vec![0u8, 85, 170, 255];
        let bytes = hand_encode(4, 1, 8, 0, None, None, &raw);
        let snap = decode_png(&bytes).expect("decode grayscale");
        let expected: Vec<u8> = raw.iter().flat_map(|&g| [g, g, g, 255]).collect();
        assert_eq!(snap.image.rgba, expected);
    }

    #[test]
    fn color_type_2_rgb() {
        let raw = vec![10u8, 20, 30, 40, 50, 60]; // 2x1 RGB
        let bytes = hand_encode(2, 1, 8, 2, None, None, &raw);
        let snap = decode_png(&bytes).expect("decode rgb");
        assert_eq!(snap.image.rgba, vec![10, 20, 30, 255, 40, 50, 60, 255]);
    }

    #[test]
    fn color_type_3_palette_with_trns() {
        // palette of 3 entries; tRNS makes entry 1 half-transparent, entry 2 fully so
        let plte = [255u8, 0, 0, 0, 255, 0, 0, 0, 255]; // red, green, blue
        let trns = [255u8, 128, 0];
        let raw = vec![0u8, 1, 2, 0]; // 4x1 indices, bit depth 8
        let bytes = hand_encode(4, 1, 8, 3, Some(&plte), Some(&trns), &raw);
        let snap = decode_png(&bytes).expect("decode palette+trns");
        assert_eq!(snap.image.rgba, vec![
            255, 0, 0, 255,
            0, 255, 0, 128,
            0, 0, 255, 0,
            255, 0, 0, 255,
        ]);
    }

    #[test]
    fn color_type_3_sub_byte_indices() {
        // bit depth 2, 4 indices packed into a single byte: 0,1,2,3 -> 0b00_01_10_11 = 0x1B
        let plte = [0u8,0,0, 64,64,64, 128,128,128, 255,255,255];
        let raw = vec![0b00_01_10_11u8];
        let bytes = hand_encode(4, 1, 2, 3, Some(&plte), None, &raw);
        let snap = decode_png(&bytes).expect("decode 2-bit palette");
        assert_eq!(snap.image.rgba, vec![
            0,0,0,255,
            64,64,64,255,
            128,128,128,255,
            255,255,255,255,
        ]);
    }

    #[test]
    fn color_type_4_grayscale_alpha() {
        let raw = vec![100u8, 200, 50, 10]; // 2x1: (gray,alpha) pairs
        let bytes = hand_encode(2, 1, 8, 4, None, None, &raw);
        let snap = decode_png(&bytes).expect("decode grayscale+alpha");
        assert_eq!(snap.image.rgba, vec![100, 100, 100, 200, 50, 50, 50, 10]);
    }

    #[test]
    fn color_type_6_rgba_bit_depth_16() {
        // 1x1 pixel, 16-bit RGBA; high byte should be what survives scale_to_8
        let raw = vec![0x12u8, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
        let bytes = hand_encode(1, 1, 16, 6, None, None, &raw);
        let snap = decode_png(&bytes).expect("decode 16-bit rgba");
        assert_eq!(snap.image.rgba, vec![0x12, 0x56, 0x9A, 0xDE]);
    }
    //#endregion ColorTypeFixtures

    //#region Adam7Fixture
    /// 🧪 Test-only Adam7 *encoder*, used solely to build a genuinely interlaced fixture to
    /// prove `decode_png` de-interlaces correctly. Production `encode_png` intentionally
    /// always emits interlace method 0 (see 🚫️EncodeScopeNote on `encode_png`); this helper
    /// is not exposed outside tests.
    fn adam7_encode_fixture(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
        let bpp = 4usize;
        let mut idat = Vec::new();
        for pass in 0..7 {
            let (pw, ph) = adam7_pass_dims(width, height, pass);
            if pw == 0 || ph == 0 { continue; }
            let (sx, sy, stx, sty) = ADAM7[pass];
            let mut prev: Option<Vec<u8>> = None;
            for j in 0..ph {
                let mut row = Vec::with_capacity(pw as usize * bpp);
                for i in 0..pw {
                    let x = sx + i * stx;
                    let y = sy + j * sty;
                    let idx = ((y * width + x) * 4) as usize;
                    row.extend_from_slice(&rgba[idx..idx + 4]);
                }
                let (ft, filtered) = choose_filter(&row, prev.as_deref(), bpp);
                idat.push(ft);
                idat.extend_from_slice(&filtered);
                prev = Some(row);
            }
        }
        let compressed = crate::artifacts::deflate::engine::zlib_compress(&idat).unwrap();
        let mut out = Vec::new();
        out.extend_from_slice(&PNG_SIGNATURE);
        let mut ihdr = Vec::with_capacity(13);
        ihdr.extend_from_slice(&width.to_be_bytes());
        ihdr.extend_from_slice(&height.to_be_bytes());
        ihdr.extend_from_slice(&[8, 6, 0, 0, 1]); // interlace method 1 = Adam7
        write_chunk(&mut out, b"IHDR", &ihdr);
        write_chunk(&mut out, b"IDAT", &compressed);
        write_chunk(&mut out, b"IEND", &[]);
        out
    }

    #[test]
    fn adam7_interlaced_decode_round_trip() {
        let (w, h) = (9u32, 11u32); // deliberately not a multiple of 8, exercises partial passes
        let rgba = gradient_checkerboard_rgba(w, h);
        let bytes = adam7_encode_fixture(w, h, &rgba);
        let snap = decode_png(&bytes).expect("decode adam7");
        assert_eq!(snap.image.width, w);
        assert_eq!(snap.image.height, h);
        assert_eq!(snap.image.rgba, rgba, "adam7 de-interlace must reconstruct the exact original raster");
    }
    //#endregion Adam7Fixture
}
//#endregion EngineTests
