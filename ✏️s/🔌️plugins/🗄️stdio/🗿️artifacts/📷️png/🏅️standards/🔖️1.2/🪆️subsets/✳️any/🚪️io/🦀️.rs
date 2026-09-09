//! 🚪️ IO stdio.png (1.2/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via `crate::register`), not per-leaf
//! register(). Relocated from `⚙️engine` verbatim (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 2: codecs live in `🚪️io/`).
//!
//! 🖼️ Decode supports the full §11 chunk set needed for real-world files (all 5 color
//! types, bit depths 1/2/4/8/16, PLTE/tRNS, Adam7 interlacing, the typed ancillary set
//! gAMA/cHRM/sRGB/pHYs/tIME/bKGD, tEXt/zTXt/iTXt text, and verbatim retention of anything else)
//! and always canonicalizes the raster into `pixels`: 8-bit RGBA. Encode always emits color
//! type 6 / bit depth 8 / interlace method 0 for the pixel data — see 🚫️EncodeScopeNote below
//! — but DOES honestly re-emit every typed ancillary/text/unknown chunk it decoded, in the
//! original relative chunk order.
/// 🧩 Borrowed PNG chunk type tag and payload.
type PngChunkView<'a> = ([u8; 4], &'a [u8]);

use crate::{
    schema::snapshot::{PngBackground, PngChromaticities, PngChunk, PngChunkMarker, PngColorType, PngPhysicalDims, PngRgb, PngSrgbIntent, PngTextChunk, PngTextKind, PngTimestamp, PngTransparency},
    PngSnapshot,
};

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_2::subsets::any::schema::PngAnalyzer;
    use crate::PngSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    pub struct PngComposerComposition;

    impl ArtifactComposition for PngComposerComposition {
        type Snapshot = PngSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY, DEP_DEFLATE]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY || s.dialect == DEP_DEFLATE)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "PngComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = PngAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "PngComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region Signature
const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
//#endregion Signature

//#region Crc
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn png_crc32(data: &[u8]) -> u32 {
    semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::crc32(data)
}
//#endregion Crc

//#region ChunkIo
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_chunks(data: &[u8]) -> Result<Vec<PngChunkView<'_>>, String> {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bpp_bytes(ihdr: &Ihdr) -> usize {
    (samples_per_pixel(ihdr.color_type) * ihdr.bit_depth as usize).div_ceil(8).max(1)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn packed_row_bytes(width: u32, color_type: u8, bit_depth: u8) -> usize {
    let bits = width as usize * samples_per_pixel(color_type) * bit_depth as usize;
    bits.div_ceil(8)
}
//#endregion Ihdr

//#region Filter
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let (a, b, c) = (a as i32, b as i32, c as i32);
    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();
    if pa <= pb && pa <= pc {
        a as u8
    } else if pb <= pc {
        b as u8
    } else {
        c as u8
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn filter_row(filter_type: u8, cur: &[u8], prev: Option<&[u8]>, bpp: usize) -> Vec<u8> {
    let mut out = vec![0u8; cur.len()];
    for x in 0..cur.len() {
        let a = if x >= bpp { cur[x - bpp] } else { 0 };
        let b = prev.map_or(0, |p| p[x]);
        let c = if x >= bpp { prev.map_or(0, |p| p[x - bpp]) } else { 0 };
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn defilter_row(filter_type: u8, filt: &[u8], prev: Option<&[u8]>, bpp: usize) -> Result<Vec<u8>, String> {
    if filter_type > 4 {
        return Err(format!("png: unsupported filter type {filter_type}"));
    }
    let mut out = vec![0u8; filt.len()];
    for x in 0..filt.len() {
        let a = if x >= bpp { out[x - bpp] } else { 0 };
        let b = prev.map_or(0, |p| p[x]);
        let c = if x >= bpp { prev.map_or(0, |p| p[x - bpp]) } else { 0 };
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
const ADAM7: [(u32, u32, u32, u32); 7] = [(0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2)];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn adam7_pass_dims(width: u32, height: u32, pass: usize) -> (u32, u32) {
    let (sx, sy, stx, sty) = ADAM7[pass];
    let w = if width > sx { (width - sx).div_ceil(stx) } else { 0 };
    let h = if height > sy { (height - sy).div_ceil(sty) } else { 0 };
    (w, h)
}
//#endregion Adam7

//#region Unpack
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unpack_samples(row: &[u8], width: usize, spp: usize, bit_depth: u8) -> Vec<u32> {
    let count = width * spp;
    let mut out = Vec::with_capacity(count);
    if bit_depth == 16 {
        for i in 0..count {
            out.push(((row[i * 2] as u32) << 8) | row[i * 2 + 1] as u32);
        }
    } else if bit_depth == 8 {
        out.extend(row[..count].iter().map(|&sample| sample as u32));
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pixel_to_rgba(samples: &[u32], ihdr: &Ihdr, palette: &[[u8; 3]], palette_alpha: &[u8], gray_trans: Option<u32>, rgb_trans: Option<(u32, u32, u32)>) -> Result<[u8; 4], String> {
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
        6 => Ok([scale_to_8(samples[0], ihdr.bit_depth), scale_to_8(samples[1], ihdr.bit_depth), scale_to_8(samples[2], ihdr.bit_depth), scale_to_8(samples[3], ihdr.bit_depth)]),
        _ => unreachable!("validated in parse_ihdr"),
    }
}
//#endregion Unpack

//#region AncillaryCodec
// 🧩 Typed encode/decode for the ancillary chunk set — kept next to the chunk-order-aware
// `encode_png`/`decode_png` bodies since every one of these is a single (type, wire-shape) pair.

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_bkgd(b: &PngBackground) -> Vec<u8> {
    match b {
        PngBackground::Grayscale { gray } => gray.to_be_bytes().to_vec(),
        PngBackground::Rgb { r, g, b } => {
            let mut v = Vec::with_capacity(6);
            v.extend_from_slice(&r.to_be_bytes());
            v.extend_from_slice(&g.to_be_bytes());
            v.extend_from_slice(&b.to_be_bytes());
            v
        }
        PngBackground::Indexed { index } => vec![*index],
    }
}

/// 📝 Serializes one `PngTextChunk` back to its real `tEXt`/`zTXt`/`iTXt` wire shape (§11.3.4).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_text_chunk(out: &mut Vec<u8>, tc: &PngTextChunk) {
    match tc.kind {
        PngTextKind::Text => {
            let mut data = Vec::with_capacity(tc.keyword.len() + 1 + tc.value.len());
            data.extend_from_slice(tc.keyword.as_bytes());
            data.push(0);
            data.extend_from_slice(tc.value.as_bytes());
            write_chunk(out, b"tEXt", &data);
        }
        PngTextKind::ZText => {
            let mut data = Vec::with_capacity(tc.keyword.len() + 2);
            data.extend_from_slice(tc.keyword.as_bytes());
            data.push(0);
            data.push(0); // compression method 0 = zlib/deflate
            let compressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(tc.value.as_bytes()).unwrap_or_default();
            data.extend_from_slice(&compressed);
            write_chunk(out, b"zTXt", &data);
        }
        PngTextKind::IText => {
            let mut data = Vec::new();
            data.extend_from_slice(tc.keyword.as_bytes());
            data.push(0);
            data.push(if tc.compressed { 1 } else { 0 });
            data.push(0); // compression method 0 = zlib/deflate
            data.extend_from_slice(tc.language_tag.as_bytes());
            data.push(0);
            data.extend_from_slice(tc.translated_keyword.as_bytes());
            data.push(0);
            if tc.compressed {
                let compressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(tc.value.as_bytes()).unwrap_or_default();
                data.extend_from_slice(&compressed);
            } else {
                data.extend_from_slice(tc.value.as_bytes());
            }
            write_chunk(out, b"iTXt", &data);
        }
    }
}
//#endregion AncillaryCodec

//#region Codec
/// 🚫 EncodeScopeNote: always emits color type 6 (RGBA) / bit depth 8 / interlace method 0 for
/// the PIXEL data. `pixels` is a canonical 8-bit-RGBA model, so re-encoding a decoded
/// palette/grayscale/16-bit/interlaced source will not byte-for-byte round-trip the original
/// file's IDAT — only its pixel content (see `codec_retention_law`). Decode (below) fully
/// supports the input diversity; only the raster half of encode canonicalizes — every typed
/// ancillary/text/unknown chunk IS honestly re-emitted, in the decoded relative chunk order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_png(snap: &PngSnapshot) -> Result<Vec<u8>, String> {
    let expected_len = (snap.width as usize).checked_mul(snap.height as usize).and_then(|p| p.checked_mul(4)).ok_or("dimensions overflow")?;
    if snap.pixels.len() != expected_len {
        return Err("pixels length mismatch".into());
    }
    let bpp = 4usize;
    let row_bytes = snap.width as usize * bpp;
    let mut idat = Vec::with_capacity((row_bytes + 1) * snap.height as usize);
    let mut prev: Option<Vec<u8>> = None;
    for y in 0..snap.height as usize {
        let row = &snap.pixels[y * row_bytes..(y + 1) * row_bytes];
        let (ft, filtered) = choose_filter(row, prev.as_deref(), bpp);
        idat.push(ft);
        idat.extend_from_slice(&filtered);
        prev = Some(row.to_vec());
    }
    let compressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(&idat)?;

    let mut out = Vec::new();
    out.extend_from_slice(&PNG_SIGNATURE);
    let mut idat_written = false;
    let mut iend_written = false;
    for marker in &snap.chunk_order {
        match marker {
            PngChunkMarker::Ihdr => {
                let mut ihdr = Vec::with_capacity(13);
                ihdr.extend_from_slice(&snap.width.to_be_bytes());
                ihdr.extend_from_slice(&snap.height.to_be_bytes());
                ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
                write_chunk(&mut out, b"IHDR", &ihdr);
            }
            PngChunkMarker::Plte => {
                if let Some(entries) = &snap.plte {
                    let mut data = Vec::with_capacity(entries.len() * 3);
                    for e in entries {
                        data.extend_from_slice(&[e.r, e.g, e.b]);
                    }
                    write_chunk(&mut out, b"PLTE", &data);
                }
            }
            // 🚫 §11.3.3: "a tRNS chunk shall not appear for colour types 4 and 6". This encoder
            // always writes colour type 6 (the EncodeScopeNote above), so a tRNS chunk here would
            // make the output non-conforming and unreadable — the reference `png` decoder rejects
            // it outright with `ColorWithBadTrns`. Alpha decoded from the source's own tRNS is
            // already folded into `pixels`, so nothing is lost by omitting the chunk; what would be
            // lost by writing it is the whole file.
            PngChunkMarker::Trns => {}
            PngChunkMarker::Gama => {
                if let Some(g) = snap.gama {
                    write_chunk(&mut out, b"gAMA", &g.to_be_bytes());
                }
            }
            PngChunkMarker::Chrm => {
                if let Some(c) = &snap.chrm {
                    let mut data = Vec::with_capacity(32);
                    for v in [c.white_x, c.white_y, c.red_x, c.red_y, c.green_x, c.green_y, c.blue_x, c.blue_y] {
                        data.extend_from_slice(&v.to_be_bytes());
                    }
                    write_chunk(&mut out, b"cHRM", &data);
                }
            }
            PngChunkMarker::Srgb => {
                if let Some(s) = snap.srgb {
                    write_chunk(&mut out, b"sRGB", &[s.to_u8()]);
                }
            }
            PngChunkMarker::Phys => {
                if let Some(p) = &snap.phys {
                    let mut data = Vec::with_capacity(9);
                    data.extend_from_slice(&p.ppu_x.to_be_bytes());
                    data.extend_from_slice(&p.ppu_y.to_be_bytes());
                    data.push(if p.unit_is_meter { 1 } else { 0 });
                    write_chunk(&mut out, b"pHYs", &data);
                }
            }
            PngChunkMarker::Time => {
                if let Some(t) = &snap.time {
                    let mut data = Vec::with_capacity(7);
                    data.extend_from_slice(&t.year.to_be_bytes());
                    data.extend_from_slice(&[t.month, t.day, t.hour, t.minute, t.second]);
                    write_chunk(&mut out, b"tIME", &data);
                }
            }
            PngChunkMarker::Bkgd => {
                if let Some(b) = &snap.bkgd {
                    write_chunk(&mut out, b"bKGD", &encode_bkgd(b));
                }
            }
            PngChunkMarker::Idat => {
                if !idat_written {
                    write_chunk(&mut out, b"IDAT", &compressed);
                    idat_written = true;
                }
            }
            PngChunkMarker::Iend => {
                if !iend_written {
                    write_chunk(&mut out, b"IEND", &[]);
                    iend_written = true;
                }
            }
            PngChunkMarker::Text { index } => {
                if let Some(tc) = snap.text_chunks.get(*index) {
                    write_text_chunk(&mut out, tc);
                }
            }
            PngChunkMarker::Unknown { index } => {
                if let Some(c) = snap.unknown_chunks.get(*index) {
                    write_chunk(&mut out, &c.kind, &c.data);
                }
            }
        }
    }
    // 🛟 Structural fallback: a snapshot whose `chunk_order` doesn't (yet) carry IDAT/IEND
    // markers — e.g. hand-built in a test without going through `PngSnapshot::default()` —
    // still produces a valid, decodable file.
    if !idat_written {
        write_chunk(&mut out, b"IDAT", &compressed);
    }
    if !iend_written {
        write_chunk(&mut out, b"IEND", &[]);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_png(data: &[u8]) -> Result<PngSnapshot, String> {
    let chunks = read_chunks(data)?;
    let mut ihdr: Option<Ihdr> = None;
    let mut palette: Vec<[u8; 3]> = Vec::new();
    let mut palette_alpha: Vec<u8> = Vec::new();
    let mut gray_trans: Option<u32> = None;
    let mut rgb_trans: Option<(u32, u32, u32)> = None;
    let mut idat = Vec::new();
    let mut seen_idat = false;

    let mut plte_out: Option<Vec<PngRgb>> = None;
    let mut trns_out: Option<PngTransparency> = None;
    let mut gama_out: Option<u32> = None;
    let mut chrm_out: Option<PngChromaticities> = None;
    let mut srgb_out: Option<PngSrgbIntent> = None;
    let mut phys_out: Option<PngPhysicalDims> = None;
    let mut time_out: Option<PngTimestamp> = None;
    let mut bkgd_out: Option<PngBackground> = None;
    let mut text_chunks: Vec<PngTextChunk> = Vec::new();
    let mut unknown_chunks: Vec<PngChunk> = Vec::new();
    let mut chunk_order: Vec<PngChunkMarker> = Vec::new();
    let mut idat_marker_emitted = false;

    for &(ty, chunk) in &chunks {
        if ty == *b"IHDR" {
            ihdr = Some(parse_ihdr(chunk)?);
            chunk_order.push(PngChunkMarker::Ihdr);
        } else if ty == *b"PLTE" {
            if chunk.len() % 3 != 0 {
                return Err("png PLTE: length not a multiple of 3".into());
            }
            palette = chunk.as_chunks::<3>().0.iter().map(|c| [c[0], c[1], c[2]]).collect();
            plte_out = Some(palette.iter().map(|c| PngRgb { r: c[0], g: c[1], b: c[2] }).collect());
            chunk_order.push(PngChunkMarker::Plte);
        } else if ty == *b"tRNS" {
            let color_type = ihdr.as_ref().ok_or("png: tRNS before IHDR")?.color_type;
            match color_type {
                0 => {
                    if chunk.len() != 2 {
                        return Err("png tRNS: expected 2 bytes for grayscale".into());
                    }
                    let g = u16::from_be_bytes([chunk[0], chunk[1]]);
                    gray_trans = Some(g as u32);
                    trns_out = Some(PngTransparency::Grayscale { gray: g });
                }
                2 => {
                    if chunk.len() != 6 {
                        return Err("png tRNS: expected 6 bytes for truecolor".into());
                    }
                    let r = u16::from_be_bytes([chunk[0], chunk[1]]);
                    let g = u16::from_be_bytes([chunk[2], chunk[3]]);
                    let b = u16::from_be_bytes([chunk[4], chunk[5]]);
                    rgb_trans = Some((r as u32, g as u32, b as u32));
                    trns_out = Some(PngTransparency::Rgb { r, g, b });
                }
                3 => {
                    palette_alpha = chunk.to_vec();
                    trns_out = Some(PngTransparency::Indexed { alpha: palette_alpha.clone() });
                }
                _ => {} // spec: tRNS shall not appear for 4/6 (already carry alpha) — ignore rather than fail
            }
            chunk_order.push(PngChunkMarker::Trns);
        } else if ty == *b"gAMA" {
            if chunk.len() != 4 {
                return Err("png gAMA: expected 4 bytes".into());
            }
            gama_out = Some(u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
            chunk_order.push(PngChunkMarker::Gama);
        } else if ty == *b"cHRM" {
            if chunk.len() != 32 {
                return Err("png cHRM: expected 32 bytes".into());
            }
            let v = |i: usize| u32::from_be_bytes([chunk[i], chunk[i + 1], chunk[i + 2], chunk[i + 3]]);
            chrm_out = Some(PngChromaticities { white_x: v(0), white_y: v(4), red_x: v(8), red_y: v(12), green_x: v(16), green_y: v(20), blue_x: v(24), blue_y: v(28) });
            chunk_order.push(PngChunkMarker::Chrm);
        } else if ty == *b"sRGB" {
            if chunk.len() != 1 {
                return Err("png sRGB: expected 1 byte".into());
            }
            srgb_out = Some(PngSrgbIntent::from_u8(chunk[0])?);
            chunk_order.push(PngChunkMarker::Srgb);
        } else if ty == *b"pHYs" {
            if chunk.len() != 9 {
                return Err("png pHYs: expected 9 bytes".into());
            }
            phys_out = Some(PngPhysicalDims { ppu_x: u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]), ppu_y: u32::from_be_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]), unit_is_meter: chunk[8] == 1 });
            chunk_order.push(PngChunkMarker::Phys);
        } else if ty == *b"tIME" {
            if chunk.len() != 7 {
                return Err("png tIME: expected 7 bytes".into());
            }
            time_out = Some(PngTimestamp { year: u16::from_be_bytes([chunk[0], chunk[1]]), month: chunk[2], day: chunk[3], hour: chunk[4], minute: chunk[5], second: chunk[6] });
            chunk_order.push(PngChunkMarker::Time);
        } else if ty == *b"bKGD" {
            let color_type = ihdr.as_ref().ok_or("png: bKGD before IHDR")?.color_type;
            bkgd_out = Some(match color_type {
                0 | 4 => {
                    if chunk.len() != 2 {
                        return Err("png bKGD: expected 2 bytes for grayscale".into());
                    }
                    PngBackground::Grayscale { gray: u16::from_be_bytes([chunk[0], chunk[1]]) }
                }
                2 | 6 => {
                    if chunk.len() != 6 {
                        return Err("png bKGD: expected 6 bytes for truecolor".into());
                    }
                    PngBackground::Rgb { r: u16::from_be_bytes([chunk[0], chunk[1]]), g: u16::from_be_bytes([chunk[2], chunk[3]]), b: u16::from_be_bytes([chunk[4], chunk[5]]) }
                }
                3 => {
                    if chunk.len() != 1 {
                        return Err("png bKGD: expected 1 byte for palette".into());
                    }
                    PngBackground::Indexed { index: chunk[0] }
                }
                _ => return Err("png bKGD: unsupported color type".into()),
            });
            chunk_order.push(PngChunkMarker::Bkgd);
        } else if ty == *b"tEXt" {
            let nul = chunk.iter().position(|&b| b == 0).ok_or("png tEXt: missing NUL after keyword")?;
            let keyword = String::from_utf8_lossy(&chunk[..nul]).to_string();
            let value = String::from_utf8_lossy(&chunk[nul + 1..]).to_string();
            let index = text_chunks.len();
            text_chunks.push(PngTextChunk { keyword, value, compressed: false, kind: PngTextKind::Text, language_tag: String::new(), translated_keyword: String::new() });
            chunk_order.push(PngChunkMarker::Text { index });
        } else if ty == *b"zTXt" {
            let nul = chunk.iter().position(|&b| b == 0).ok_or("png zTXt: missing NUL after keyword")?;
            let keyword = String::from_utf8_lossy(&chunk[..nul]).to_string();
            if chunk.len() < nul + 2 {
                return Err("png zTXt: missing compression method".into());
            }
            let value_bytes = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_decompress(&chunk[nul + 2..])?;
            let value = String::from_utf8_lossy(&value_bytes).to_string();
            let index = text_chunks.len();
            text_chunks.push(PngTextChunk { keyword, value, compressed: true, kind: PngTextKind::ZText, language_tag: String::new(), translated_keyword: String::new() });
            chunk_order.push(PngChunkMarker::Text { index });
        } else if ty == *b"iTXt" {
            let mut pos = 0usize;
            let nul1 = chunk[pos..].iter().position(|&b| b == 0).ok_or("png iTXt: missing NUL after keyword")?;
            let keyword = String::from_utf8_lossy(&chunk[pos..pos + nul1]).to_string();
            pos += nul1 + 1;
            if pos + 2 > chunk.len() {
                return Err("png iTXt: truncated flags".into());
            }
            let compressed_flag = chunk[pos] != 0;
            pos += 2; // compressed flag + compression method
            let nul2 = chunk[pos..].iter().position(|&b| b == 0).ok_or("png iTXt: missing NUL after language tag")?;
            let language_tag = String::from_utf8_lossy(&chunk[pos..pos + nul2]).to_string();
            pos += nul2 + 1;
            let nul3 = chunk[pos..].iter().position(|&b| b == 0).ok_or("png iTXt: missing NUL after translated keyword")?;
            let translated_keyword = String::from_utf8_lossy(&chunk[pos..pos + nul3]).to_string();
            pos += nul3 + 1;
            let rest = &chunk[pos..];
            let value = if compressed_flag {
                let decompressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_decompress(rest)?;
                String::from_utf8_lossy(&decompressed).to_string()
            } else {
                String::from_utf8_lossy(rest).to_string()
            };
            let index = text_chunks.len();
            text_chunks.push(PngTextChunk { keyword, value, compressed: compressed_flag, kind: PngTextKind::IText, language_tag, translated_keyword });
            chunk_order.push(PngChunkMarker::Text { index });
        } else if ty == *b"IDAT" {
            idat.extend_from_slice(chunk);
            seen_idat = true;
            if !idat_marker_emitted {
                chunk_order.push(PngChunkMarker::Idat);
                idat_marker_emitted = true;
            }
        } else if ty == *b"IEND" {
            chunk_order.push(PngChunkMarker::Iend);
        } else if ty[0].is_ascii_uppercase() {
            return Err(format!("png: unsupported critical chunk {}", String::from_utf8_lossy(&ty)));
        } else {
            // 🗃️ Ancillary chunk the codec doesn't specifically model — typed raw-retention,
            // verbatim, in position (the recipe's "nothing real on disk silently dropped" rule).
            let index = unknown_chunks.len();
            unknown_chunks.push(PngChunk { kind: ty, data: chunk.to_vec() });
            chunk_order.push(PngChunkMarker::Unknown { index });
        }
    }

    let ihdr = ihdr.ok_or("png: missing IHDR")?;
    if !seen_idat {
        return Err("png: missing IDAT".into());
    }
    if ihdr.color_type == 3 && palette.is_empty() {
        return Err("png: color type 3 requires PLTE".into());
    }

    let raw = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_decompress(&idat)?;
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
        for (pass, &(sx, sy, stx, sty)) in ADAM7.iter().enumerate() {
            let (pw, ph) = adam7_pass_dims(ihdr.width, ihdr.height, pass);
            if pw == 0 || ph == 0 {
                continue;
            }
            let row_bytes = packed_row_bytes(pw, ihdr.color_type, ihdr.bit_depth);
            let (rows, new_pos) = defilter_pass(&raw, pos, ph, row_bytes, bpp)?;
            pos = new_pos;
            for (j, row) in rows.iter().enumerate() {
                let samples = unpack_samples(row, pw as usize, spp, ihdr.bit_depth);
                put_row(&samples, pw as usize, sx, sy + j as u32 * sty, stx)?;
            }
        }
    }

    Ok(PngSnapshot {
        schema: crate::STDIO_PNG_DOCUMENT_SCHEMA.into(),
        width: ihdr.width,
        height: ihdr.height,
        bit_depth: ihdr.bit_depth,
        color_type: PngColorType::from_u8(ihdr.color_type)?,
        interlace: ihdr.interlace == 1,
        plte: plte_out,
        trns: trns_out,
        gama: gama_out,
        chrm: chrm_out,
        srgb: srgb_out,
        phys: phys_out,
        time: time_out,
        bkgd: bkgd_out,
        text_chunks,
        pixels: rgba,
        chunk_order,
        unknown_chunks,
    })
}
//#endregion Codec

//#region 🚪️DerivedIoRegistry
/// 🚪️ Relocated verbatim from `⚙️engine` (rule 3: `io_registry`/`ComposerEntry` live in `🚪️io/`).
pub mod io_registry {
    use crate::standards::v1_2::subsets::any::schema::PngComposer as PngRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<PngRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🧪️CodecTests
#[cfg(test)]
#[path = "🧪️tests/🔬️codec/🦀️.rs"]
mod codec_tests;
//#endregion 🧪️CodecTests
