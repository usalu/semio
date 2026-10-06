//! 🚪️ IO stdio.png (1.2/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via `crate::register`), not per-leaf
//! register(). Relocated from `⚙️engine` verbatim (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 2: codecs live in `🚪️io/`).
//!
//! 🖼️ Exact PNG source bytes are the sole persisted authority. Checked layout, metadata,
//! sample and RGBA8 preview views are derived without rewriting the source. No-op export returns
//! those exact bytes. Addressed metadata and pixel edits replace only the affected canonical chunks,
//! retain all unrelated chunks and reject profiles whose sample semantics cannot be edited losslessly.
/// 🧩 Borrowed PNG chunk type tag and payload.
type PngChunkView<'a> = ([u8; 4], &'a [u8]);

use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind,native_decoding::NativeDecodeProgress,native_encoding::NativeEncodeProgress};
use crate::{
    schema::snapshot::{PngBackground, PngChromaticities, PngChunk, PngChunkMarker, PngColorType, PngPhysicalDims, PngRgb, PngSrgbIntent, PngTextChunk, PngTextKind, PngTimestamp, PngTransparency},
    PngSnapshot,
};

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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PngRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar)]
#[value(rename_all = "kebab-case")]
pub enum PngNativeProfile {
    Indexed,
    Grayscale,
    GrayscaleAlpha,
    Rgb,
    Rgba,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PngNativePaint {
    pub profile: PngNativeProfile,
    pub first: u16,
    pub second: u16,
    pub third: u16,
    pub fourth: u16,
}

impl PngNativePaint {
    pub const fn indexed(index: u16) -> Self { Self { profile: PngNativeProfile::Indexed, first: index, second: 0, third: 0, fourth: 0 } }
    pub const fn grayscale(gray: u16) -> Self { Self { profile: PngNativeProfile::Grayscale, first: gray, second: 0, third: 0, fourth: 0 } }
    pub const fn grayscale_alpha(gray: u16, alpha: u16) -> Self { Self { profile: PngNativeProfile::GrayscaleAlpha, first: gray, second: alpha, third: 0, fourth: 0 } }
    pub const fn rgb(red: u16, green: u16, blue: u16) -> Self { Self { profile: PngNativeProfile::Rgb, first: red, second: green, third: blue, fourth: 0 } }
    pub const fn rgba(red: u16, green: u16, blue: u16, alpha: u16) -> Self { Self { profile: PngNativeProfile::Rgba, first: red, second: green, third: blue, fourth: alpha } }

    fn samples(self) -> [u16; 4] { [self.first, self.second, self.third, self.fourth] }
}

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_2::subsets::any::io::PngAnalyzer;
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
fn png_crc32_parts(kind: &[u8; 4], data: &[u8]) -> u32 {
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
fn write_chunk(out: &mut Vec<u8>, ty: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(ty);
    out.extend_from_slice(data);
    out.extend_from_slice(&png_crc32_parts(ty, data).to_be_bytes());
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
        if png_crc32_parts(&ty, chunk_data) != stored_crc {
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
    if pos != data.len() {
        return Err("png: bytes follow IEND".into());
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
fn parse_ihdr(data: &[u8]) -> Result<Ihdr, ValueError> {
    if data.len() != 13 {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue,"png IHDR: expected 13 bytes"));
    }
    let width = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    let height = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    let bit_depth = data[8];
    let color_type = data[9];
    let compression = data[10];
    let filter_method = data[11];
    let interlace = data[12];
    if width == 0 || height == 0 {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue,"png IHDR: zero dimension"));
    }
    if compression != 0 {
        return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"png IHDR: unsupported compression method"));
    }
    if filter_method != 0 {
        return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"png IHDR: unsupported filter method"));
    }
    if interlace > 1 {
        return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"png IHDR: unsupported interlace method"));
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
        return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,format!("png IHDR: unsupported color type {color_type} / bit depth {bit_depth}")));
    }
    Ok(Ihdr { width, height, bit_depth, color_type, interlace })
}

fn chunk_position(chunks: &[PngChunkView<'_>], kind: [u8; 4]) -> Option<usize> {
    chunks.iter().position(|(candidate, _)| *candidate == kind)
}

fn require_singleton(chunks: &[PngChunkView<'_>], kind: [u8; 4]) -> Result<(), String> {
    if chunks.iter().filter(|(candidate, _)| *candidate == kind).count() > 1 {
        return Err(format!("png: {} must be a singleton", String::from_utf8_lossy(&kind)));
    }
    Ok(())
}

fn validate_png_structure(chunks: &[PngChunkView<'_>]) -> Result<(), String> {
    let Some((first_kind, first_data)) = chunks.first() else { return Err("png: missing IHDR".into()) };
    if *first_kind != *b"IHDR" {
        return Err("png: IHDR must be first".into());
    }
    let ihdr = parse_ihdr(first_data).map_err(ValueError::into_message)?;
    let Some((last_kind, last_data)) = chunks.last() else { return Err("png: missing IEND".into()) };
    if *last_kind != *b"IEND" {
        return Err("png: IEND must be last".into());
    }
    if !last_data.is_empty() {
        return Err("png: IEND must be empty".into());
    }
    for (kind, _) in chunks {
        if !kind.iter().all(u8::is_ascii_alphabetic) {
            return Err(format!("png: invalid chunk type {}", String::from_utf8_lossy(kind)));
        }
        if kind[2].is_ascii_lowercase() {
            return Err(format!("png: chunk {} uses the reserved type bit", String::from_utf8_lossy(kind)));
        }
    }
    for kind in [*b"IHDR", *b"PLTE", *b"tRNS", *b"gAMA", *b"cHRM", *b"sRGB", *b"pHYs", *b"tIME", *b"bKGD", *b"IEND"] {
        require_singleton(chunks, kind)?;
    }
    let first_idat = chunk_position(chunks, *b"IDAT").ok_or("png: missing IDAT")?;
    let last_idat = chunks.iter().rposition(|(kind, _)| *kind == *b"IDAT").expect("first IDAT exists");
    if chunks[first_idat..=last_idat].iter().any(|(kind, _)| *kind != *b"IDAT") {
        return Err("png: IDAT chunks must be consecutive".into());
    }
    let plte = chunk_position(chunks, *b"PLTE");
    let palette_entries = if let Some(index) = plte {
        if index > first_idat {
            return Err("png: PLTE must precede IDAT".into());
        }
        if matches!(ihdr.color_type, 0 | 4) {
            return Err("png: PLTE is forbidden for grayscale profiles".into());
        }
        let data = chunks[index].1;
        if data.is_empty() || data.len() % 3 != 0 || data.len() > 768 {
            return Err("png: PLTE must contain 1 to 256 complete entries".into());
        }
        let entries = data.len() / 3;
        if ihdr.color_type == 3 && entries > (1usize << ihdr.bit_depth) {
            return Err("png: PLTE has more entries than the indexed bit depth permits".into());
        }
        entries
    } else {
        if ihdr.color_type == 3 {
            return Err("png: color type 3 requires PLTE".into());
        }
        0
    };
    for kind in [*b"gAMA", *b"cHRM", *b"sRGB"] {
        if let Some(index) = chunk_position(chunks, kind) {
            if index > first_idat || plte.is_some_and(|palette| index > palette) {
                return Err(format!("png: {} must precede PLTE and IDAT", String::from_utf8_lossy(&kind)));
            }
        }
    }
    if let Some(index) = chunk_position(chunks, *b"gAMA") {
        let data = chunks[index].1;
        if data.len() != 4 || u32::from_be_bytes(data.try_into().expect("four-byte gAMA")) == 0 {
            return Err("png: gAMA must be a nonzero four-byte integer".into());
        }
    }
    if let Some(index) = chunk_position(chunks, *b"pHYs") {
        if index > first_idat {
            return Err("png: pHYs must precede IDAT".into());
        }
        let data = chunks[index].1;
        if data.len() != 9 || data[8] > 1 {
            return Err("png: pHYs has an invalid length or unit".into());
        }
    }
    if let Some(index) = chunk_position(chunks, *b"tRNS") {
        if index > first_idat || plte.is_some_and(|palette| index < palette) {
            return Err("png: tRNS must follow PLTE when present and precede IDAT".into());
        }
        let data = chunks[index].1;
        let maximum = (1u32 << ihdr.bit_depth) - 1;
        match ihdr.color_type {
            0 if data.len() == 2 && u32::from(u16::from_be_bytes(data.try_into().expect("grayscale tRNS"))) <= maximum => {}
            2 if data.len() == 6 && data.chunks_exact(2).all(|sample| u32::from(u16::from_be_bytes(sample.try_into().expect("RGB tRNS sample"))) <= maximum) => {}
            3 if !data.is_empty() && data.len() <= palette_entries => {}
            4 | 6 => return Err("png: tRNS is forbidden for alpha profiles".into()),
            _ => return Err("png: tRNS payload does not match the image profile".into()),
        }
    }
    if let Some(index) = chunk_position(chunks, *b"bKGD") {
        if index > first_idat || plte.is_some_and(|palette| index < palette) {
            return Err("png: bKGD must follow PLTE when present and precede IDAT".into());
        }
        let data = chunks[index].1;
        let maximum = (1u32 << ihdr.bit_depth) - 1;
        match ihdr.color_type {
            0 | 4 if data.len() == 2 && u32::from(u16::from_be_bytes(data.try_into().expect("grayscale bKGD"))) <= maximum => {}
            2 | 6 if data.len() == 6 && data.chunks_exact(2).all(|sample| u32::from(u16::from_be_bytes(sample.try_into().expect("RGB bKGD sample"))) <= maximum) => {}
            3 if data.len() == 1 && usize::from(data[0]) < palette_entries => {}
            _ => return Err("png: bKGD payload does not match the image profile".into()),
        }
    }
    Ok(())
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
fn pixel_to_rgba(samples: &[u32], ihdr: &Ihdr, palette: &[[u8; 3]], palette_alpha: &[u8], gray_trans: Option<u32>, rgb_trans: Option<(u32, u32, u32)>) -> Result<[u8; 4], ValueError> {
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
            let rgb = palette.get(idx).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("png: palette index {idx} out of range")))?;
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
/// 🌈️ Convert source background samples to the authored RGBA8 wire representation.
fn rgba8_background(snap:&PngProjection,background:&PngBackground)->Result<PngBackground,String>{
    if !matches!(snap.bit_depth,1|2|4|8|16){return Err("png background: invalid source bit depth".into());}
    let sample=|n:u16|u16::from(scale_to_8(u32::from(n),snap.bit_depth));
    let (r,g,b)=match background{
        PngBackground::Grayscale{gray}=>{let n=sample(*gray);(n,n,n)},
        PngBackground::Rgb{r,g,b}=>(sample(*r),sample(*g),sample(*b)),
        PngBackground::Indexed{index}=>{let rgb=snap.plte.as_ref().and_then(|entries|entries.get(*index as usize)).ok_or("png background: palette index out of range")?;(u16::from(rgb.r),u16::from(rgb.g),u16::from(rgb.b))},
    };Ok(PngBackground::Rgb{r,g,b})
}
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

//#region ExactAuthority
pub fn empty_png_bytes() -> Vec<u8> {
    let projection = PngProjection {
        width: 1,
        height: 1,
        bit_depth: 8,
        color_type: PngColorType::Rgba,
        interlace: false,
        plte: None,
        trns: None,
        gama: None,
        chrm: None,
        srgb: None,
        phys: None,
        time: None,
        bkgd: None,
        text_chunks: Vec::new(),
        pixels: vec![255, 255, 255, 255],
        chunk_order: vec![PngChunkMarker::Ihdr, PngChunkMarker::Idat, PngChunkMarker::Iend],
        unknown_chunks: Vec::new(),
    };
    author_png_projection(&projection).expect("one-pixel PNG")
}

fn chunk_addresses(bytes: &[u8]) -> Result<Vec<PngChunkAddress>, String> {
    if bytes.len() < PNG_SIGNATURE.len() || bytes[..PNG_SIGNATURE.len()] != PNG_SIGNATURE {
        return Err("png: bad signature".into());
    }
    let mut cursor = PNG_SIGNATURE.len();
    let mut chunks = Vec::new();
    let mut found_iend = false;
    while cursor < bytes.len() {
        let header = bytes.get(cursor..cursor + 8).ok_or("png: truncated chunk header")?;
        let length = u32::from_be_bytes(header[..4].try_into().expect("four-byte length")) as usize;
        let kind: [u8; 4] = header[4..8].try_into().expect("four-byte kind");
        let data_start = cursor.checked_add(8).ok_or("png: chunk offset overflow")?;
        let data_end = data_start.checked_add(length).ok_or("png: chunk length overflow")?;
        let end = data_end.checked_add(4).ok_or("png: chunk CRC range overflow")?;
        let data = bytes.get(data_start..data_end).ok_or("png: truncated chunk data")?;
        let stored = bytes.get(data_end..end).ok_or("png: truncated chunk CRC")?;
        if png_crc32_parts(&kind, data).to_be_bytes() != stored {
            return Err(format!("png: chunk CRC mismatch ({})", String::from_utf8_lossy(&kind)));
        }
        chunks.push(PngChunkAddress { ordinal: chunks.len(), kind, start: cursor, data_start, data_end, end });
        cursor = end;
        if kind == *b"IEND" {
            found_iend = true;
            break;
        }
    }
    if !found_iend {
        return Err("png: missing IEND".into());
    }
    if cursor != bytes.len() {
        return Err("png: bytes follow IEND".into());
    }
    Ok(chunks)
}

pub fn png_layout_bytes(bytes: &[u8]) -> Result<PngLayout, String> {
    let chunks = chunk_addresses(bytes)?;
    let first = chunks.first().ok_or("png: missing IHDR")?;
    if first.kind != *b"IHDR" {
        return Err("png: IHDR must be first".into());
    }
    let ihdr = parse_ihdr(&bytes[first.data_start..first.data_end]).map_err(ValueError::into_message)?;
    if !chunks.iter().any(|chunk| chunk.kind == *b"IDAT") {
        return Err("png: missing IDAT".into());
    }
    if ihdr.color_type == 3 && !chunks.iter().any(|chunk| chunk.kind == *b"PLTE") {
        return Err("png: color type 3 requires PLTE".into());
    }
    project_png(bytes)?;
    Ok(PngLayout {
        width: ihdr.width,
        height: ihdr.height,
        bit_depth: ihdr.bit_depth,
        color_type: PngColorType::from_u8(ihdr.color_type)?,
        interlace: ihdr.interlace == 1,
        chunks,
    })
}

pub fn png_layout(snapshot: &PngSnapshot) -> Result<PngLayout, String> {
    if snapshot.schema != crate::STDIO_PNG_DOCUMENT_SCHEMA {
        return Err(format!("png: schema must be {}", crate::STDIO_PNG_DOCUMENT_SCHEMA));
    }
    png_layout_bytes(&snapshot.bytes)
}

pub fn decode_png(bytes: &[u8]) -> Result<PngSnapshot, String> {
    png_layout_bytes(bytes)?;
    Ok(PngSnapshot { schema: crate::STDIO_PNG_DOCUMENT_SCHEMA.into(), bytes: bytes.to_vec() })
}

pub fn encode_png(snapshot: &PngSnapshot) -> Result<Vec<u8>, String> {
    png_layout(snapshot)?;
    Ok(snapshot.bytes.clone())
}

pub fn png_preview(snapshot: &PngSnapshot) -> Result<PngPreview, String> {
    const MAXIMUM_RGBA_BYTES: usize = 64 * 1024 * 1024;
    let layout = png_layout(snapshot)?;
    let rgba_bytes = usize::try_from(layout.width)
        .map_err(|_| "png: preview width exceeds address space")?
        .checked_mul(usize::try_from(layout.height).map_err(|_| "png: preview height exceeds address space")?)
        .and_then(|pixels| pixels.checked_mul(4))
        .ok_or("png: preview byte count overflow")?;
    if rgba_bytes > MAXIMUM_RGBA_BYTES {
        return Err(format!("png: preview needs {rgba_bytes} RGBA bytes, above the {MAXIMUM_RGBA_BYTES}-byte display limit"));
    }
    let projection = project_png(&snapshot.bytes)?;
    let bytes = author_png_projection(&projection)?;
    Ok(PngPreview { width: layout.width, height: layout.height, bytes })
}

pub fn png_revision(snapshot: &PngSnapshot) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in snapshot.schema.as_bytes().iter().chain(snapshot.bytes.iter()) {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn require_revision(snapshot: &PngSnapshot, revision: &str) -> Result<(), String> {
    let actual = png_revision(snapshot);
    if revision != actual {
        return Err(format!("png: stale source revision {revision}; expected {actual}"));
    }
    Ok(())
}

pub fn replace_ancillary_chunk_controlled(
    snapshot: &PngSnapshot,
    revision: &str,
    ordinal: usize,
    expected_kind: [u8; 4],
    data: &[u8],
    progress: &mut dyn FnMut(usize, usize) -> bool,
) -> Result<PngSnapshot, String> {
    require_revision(snapshot, revision)?;
    let layout = png_layout(snapshot)?;
    let target = layout.chunks.get(ordinal).ok_or_else(|| format!("png: chunk ordinal {ordinal} is absent"))?;
    if target.kind != expected_kind {
        return Err(format!("png: chunk ordinal {ordinal} is {}, not {}", String::from_utf8_lossy(&target.kind), String::from_utf8_lossy(&expected_kind)));
    }
    if target.kind[0].is_ascii_uppercase() {
        return Err("png: critical chunks require an explicit format conversion".into());
    }
    if !progress(0, 1) {
        return Err("png: ancillary chunk edit cancelled".into());
    }
    let mut replacement = Vec::new();
    write_chunk(&mut replacement, &target.kind, data);
    let mut bytes = snapshot.bytes.clone();
    bytes.splice(target.start..target.end, replacement);
    let next = decode_png(&bytes)?;
    if !progress(1, 1) {
        return Err("png: ancillary chunk edit cancelled".into());
    }
    Ok(next)
}

pub fn set_ancillary_chunk_controlled(
    snapshot: &PngSnapshot,
    revision: &str,
    kind: [u8; 4],
    data: Option<&[u8]>,
    progress: &mut dyn FnMut(usize, usize) -> bool,
) -> Result<PngSnapshot, String> {
    require_revision(snapshot, revision)?;
    if kind[0].is_ascii_uppercase() {
        return Err("png: critical chunks require an explicit format conversion".into());
    }
    let layout = png_layout(snapshot)?;
    let matches: Vec<&PngChunkAddress> = layout.chunks.iter().filter(|chunk| chunk.kind == kind).collect();
    if matches.len() > 1 {
        return Err(format!("png: {} is duplicated; edit by exact chunk address", String::from_utf8_lossy(&kind)));
    }
    if !progress(0, 1) {
        return Err("png: ancillary chunk edit cancelled".into());
    }
    let mut bytes = snapshot.bytes.clone();
    match (matches.first().copied(), data) {
        (Some(target), Some(payload)) => {
            let mut replacement = Vec::new();
            write_chunk(&mut replacement, &kind, payload);
            bytes.splice(target.start..target.end, replacement);
        }
        (Some(target), None) => {
            bytes.drain(target.start..target.end);
        }
        (None, Some(payload)) => {
            let before = layout.chunks.iter().find(|chunk| chunk.kind == *b"IDAT").ok_or("png: missing IDAT")?.start;
            let mut insertion = Vec::new();
            write_chunk(&mut insertion, &kind, payload);
            bytes.splice(before..before, insertion);
        }
        (None, None) => return Ok(snapshot.clone()),
    }
    let next = decode_png(&bytes)?;
    if !progress(1, 1) {
        return Err("png: ancillary chunk edit cancelled".into());
    }
    Ok(next)
}

pub fn set_gamma_chunk_controlled(
    snapshot: &PngSnapshot,
    revision: &str,
    gama: Option<u32>,
    progress: &mut dyn FnMut(usize, usize) -> bool,
) -> Result<PngSnapshot, String> {
    require_revision(snapshot, revision)?;
    if gama == Some(0) {
        return Err("png: gAMA must be nonzero".into());
    }
    let layout = png_layout(snapshot)?;
    let target = layout.chunks.iter().find(|chunk| chunk.kind == *b"gAMA");
    if !progress(0, 1) {
        return Err("png: gamma edit cancelled".into());
    }
    let mut bytes = snapshot.bytes.clone();
    match (target, gama) {
        (Some(chunk), Some(value)) => {
            let mut replacement = Vec::new();
            write_chunk(&mut replacement, b"gAMA", &value.to_be_bytes());
            bytes.splice(chunk.start..chunk.end, replacement);
        }
        (Some(chunk), None) => {
            bytes.drain(chunk.start..chunk.end);
        }
        (None, Some(value)) => {
            let before = layout
                .chunks
                .iter()
                .find(|chunk| chunk.kind == *b"PLTE" || chunk.kind == *b"IDAT")
                .ok_or("png: missing PLTE/IDAT gamma insertion boundary")?
                .start;
            let mut insertion = Vec::new();
            write_chunk(&mut insertion, b"gAMA", &value.to_be_bytes());
            bytes.splice(before..before, insertion);
        }
        (None, None) => return Ok(snapshot.clone()),
    }
    let next = decode_png(&bytes)?;
    if !progress(1, 1) {
        return Err("png: gamma edit cancelled".into());
    }
    Ok(next)
}

fn checked_region(layout: &PngLayout, region: PngRegion) -> Result<(), String> {
    if region.width == 0 || region.height == 0 {
        return Err("png: paint region must be nonempty".into());
    }
    let end_x = region.x.checked_add(region.width).ok_or("png: region x overflow")?;
    let end_y = region.y.checked_add(region.height).ok_or("png: region y overflow")?;
    if end_x > layout.width || end_y > layout.height {
        return Err(format!("png: region exceeds {}x{} image", layout.width, layout.height));
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NativePassRow {
    filter: u8,
    samples: Vec<u8>,
    start_x: u32,
    y: u32,
    step_x: u32,
    width: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PngNativePaintPhase {
    Address,
    Decode,
    Paint,
    Filter,
    Encode,
    Assemble,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PngNativePaintProgress {
    pub phase: PngNativePaintPhase,
    pub completed: usize,
    pub total: usize,
    pub owned_bytes: usize,
}

pub const MAXIMUM_NATIVE_PAINT_OWNED_BYTES: usize = 512 * 1024 * 1024;

fn checked_png_extent(bytes: &[u8]) -> Result<usize, String> {
    if bytes.len() < PNG_SIGNATURE.len() || bytes[..PNG_SIGNATURE.len()] != PNG_SIGNATURE {
        return Err("png: bad signature".into());
    }
    let mut cursor = PNG_SIGNATURE.len();
    let mut count = 0usize;
    let mut found_iend = false;
    while cursor < bytes.len() {
        let header = bytes.get(cursor..cursor + 8).ok_or("png: truncated chunk header")?;
        let length = u32::from_be_bytes(header[..4].try_into().expect("four-byte length")) as usize;
        let kind: [u8; 4] = header[4..8].try_into().expect("four-byte kind");
        let data_start = cursor.checked_add(8).ok_or("png: chunk offset overflow")?;
        let data_end = data_start.checked_add(length).ok_or("png: chunk length overflow")?;
        let end = data_end.checked_add(4).ok_or("png: chunk CRC range overflow")?;
        bytes.get(data_start..end).ok_or("png: truncated chunk data")?;
        count = count.checked_add(1).ok_or("png: chunk count overflow")?;
        cursor = end;
        if kind == *b"IEND" {
            found_iend = true;
            break;
        }
    }
    if !found_iend {
        return Err("png: missing IEND".into());
    }
    if cursor != bytes.len() {
        return Err("png: bytes follow IEND".into());
    }
    Ok(count)
}

fn chunk_addresses_controlled(bytes: &[u8], control: &mut NativeDecodeControl<'_>) -> Result<Vec<PngChunkAddress>, ValueError> {
    let count = checked_png_extent(bytes).map_err(|message| ValueError::new(ValueRefusalKind::InvalidValue, message))?;
    let mut chunks = control.allocate_vec(count)?;
    control.begin_stage(bytes.len())?;
    let mut cursor = PNG_SIGNATURE.len();
    while cursor < bytes.len() {
        let header = &bytes[cursor..cursor + 8];
        let length = u32::from_be_bytes(header[..4].try_into().expect("four-byte length")) as usize;
        let kind: [u8; 4] = header[4..8].try_into().expect("four-byte kind");
        let data_start = cursor + 8;
        let data_end = data_start + length;
        let end = data_end + 4;
        let data = &bytes[data_start..data_end];
        let stored = &bytes[data_end..end];
        if png_crc32_parts(&kind, data).to_be_bytes() != stored {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("png: chunk CRC mismatch ({})", String::from_utf8_lossy(&kind))));
        }
        chunks.push(PngChunkAddress { ordinal: chunks.len(), kind, start: cursor, data_start, data_end, end });
        control.advance(end - cursor)?;
        cursor = end;
        if kind == *b"IEND" {
            break;
        }
    }
    Ok(chunks)
}

fn source_layout(snapshot: &PngSnapshot) -> Result<PngLayout, String> {
    if snapshot.schema != crate::STDIO_PNG_DOCUMENT_SCHEMA {
        return Err(format!("png: schema must be {}", crate::STDIO_PNG_DOCUMENT_SCHEMA));
    }
    let chunks = chunk_addresses(&snapshot.bytes)?;
    let views = chunks.iter().map(|chunk| (chunk.kind, &snapshot.bytes[chunk.data_start..chunk.data_end])).collect::<Vec<_>>();
    validate_png_structure(&views)?;
    let first = chunks.first().ok_or("png: missing IHDR")?;
    let ihdr = parse_ihdr(&snapshot.bytes[first.data_start..first.data_end]).map_err(ValueError::into_message)?;
    Ok(PngLayout {
        width: ihdr.width,
        height: ihdr.height,
        bit_depth: ihdr.bit_depth,
        color_type: PngColorType::from_u8(ihdr.color_type)?,
        interlace: ihdr.interlace == 1,
        chunks,
    })
}

fn source_layout_controlled(snapshot: &PngSnapshot, control: &mut NativeDecodeControl<'_>) -> Result<PngLayout, ValueError> {
    if snapshot.schema != crate::STDIO_PNG_DOCUMENT_SCHEMA {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("png: schema must be {}", crate::STDIO_PNG_DOCUMENT_SCHEMA)));
    }
    let chunks = chunk_addresses_controlled(&snapshot.bytes, control)?;
    let mut views = control.allocate_vec::<PngChunkView<'_>>(chunks.len())?;
    for chunk in &chunks {
        views.push((chunk.kind, &snapshot.bytes[chunk.data_start..chunk.data_end]));
    }
    validate_png_structure(&views).map_err(|message| ValueError::new(ValueRefusalKind::InvalidValue, message))?;
    let first = chunks.first().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "png: missing IHDR"))?;
    let ihdr = parse_ihdr(&snapshot.bytes[first.data_start..first.data_end])?;
    Ok(PngLayout {
        width: ihdr.width,
        height: ihdr.height,
        bit_depth: ihdr.bit_depth,
        color_type: PngColorType::from_u8(ihdr.color_type).map_err(|message| ValueError::new(ValueRefusalKind::InvalidValue, message))?,
        interlace: ihdr.interlace == 1,
        chunks,
    })
}

fn expected_native_raw_bytes(layout: &PngLayout) -> Result<usize, ValueError> {
    let mut total = 0usize;
    let mut add = |width: u32, height: u32| -> Result<(), ValueError> {
        let row = packed_row_bytes(width, layout.color_type.to_u8(), layout.bit_depth).checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "png: scanline extent overflow"))?;
        let pass = row.checked_mul(height as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "png: pass extent overflow"))?;
        total = total.checked_add(pass).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "png: raw extent overflow"))?;
        Ok(())
    };
    if layout.interlace {
        for pass in 0..ADAM7.len() {
            let (width, height) = adam7_pass_dims(layout.width, layout.height, pass);
            if width != 0 && height != 0 {
                add(width, height)?;
            }
        }
    } else {
        add(layout.width, layout.height)?;
    }
    Ok(total)
}

fn native_row_count(layout: &PngLayout) -> Result<usize, ValueError> {
    if layout.interlace {
        ADAM7.iter().enumerate().try_fold(0usize, |total, (pass, _)| {
            let (width, height) = adam7_pass_dims(layout.width, layout.height, pass);
            total.checked_add(if width == 0 { 0 } else { height as usize }).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "png: native row count overflow"))
        })
    } else {
        Ok(layout.height as usize)
    }
}

fn defilter_row_controlled(filter: u8, bytes: &[u8], previous: Option<&[u8]>, bpp: usize, control: &mut NativeDecodeControl<'_>) -> Result<Vec<u8>, ValueError> {
    if filter > 4 {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("png: unsupported filter type {filter}")));
    }
    let mut output = control.allocate_vec(bytes.len())?;
    control.begin_stage(bytes.len())?;
    for (index, byte) in bytes.iter().copied().enumerate() {
        let a = if index >= bpp { output[index - bpp] } else { 0 };
        let b = previous.map_or(0, |row| row[index]);
        let c = if index >= bpp { previous.map_or(0, |row| row[index - bpp]) } else { 0 };
        output.push(match filter {
            0 => byte,
            1 => byte.wrapping_add(a),
            2 => byte.wrapping_add(b),
            3 => byte.wrapping_add(((u16::from(a) + u16::from(b)) / 2) as u8),
            4 => byte.wrapping_add(paeth(a, b, c)),
            _ => unreachable!("filter was validated"),
        });
        control.step()?;
    }
    Ok(output)
}

#[allow(clippy::too_many_arguments)]
fn decode_native_pass(
    raw: &[u8],
    mut position: usize,
    width: u32,
    height: u32,
    start_x: u32,
    start_y: u32,
    step_x: u32,
    step_y: u32,
    bpp: usize,
    layout: &PngLayout,
    rows: &mut Vec<NativePassRow>,
    control: &mut NativeDecodeControl<'_>,
) -> Result<usize, ValueError> {
    let row_bytes = packed_row_bytes(width, layout.color_type.to_u8(), layout.bit_depth);
    let mut previous = None;
    for row in 0..height {
        let filter = *raw.get(position).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "png: truncated scanline filter"))?;
        position += 1;
        let end = position.checked_add(row_bytes).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "png: scanline length overflow"))?;
        let filtered = raw.get(position..end).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "png: truncated scanline data"))?;
        let samples = defilter_row_controlled(filter, filtered, previous.map(|index: usize| rows[index].samples.as_slice()), bpp, control)?;
        position = end;
        rows.push(NativePassRow { filter, samples, start_x, y: start_y + row * step_y, step_x, width });
        previous = Some(rows.len() - 1);
    }
    Ok(position)
}

fn native_rows_controlled(
    snapshot: &PngSnapshot,
    layout: &PngLayout,
    phase: Option<&std::cell::Cell<PngNativePaintPhase>>,
    control: &mut NativeDecodeControl<'_>,
) -> Result<Vec<NativePassRow>, ValueError> {
    let first = layout.chunks.iter().position(|chunk| chunk.kind == *b"IDAT").ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "png: missing IDAT"))?;
    let last = layout.chunks.iter().rposition(|chunk| chunk.kind == *b"IDAT").expect("nonempty IDAT list");
    let idat = &layout.chunks[first..=last];
    if idat.iter().any(|chunk| chunk.kind != *b"IDAT") {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue, "png: IDAT chunks must be consecutive"));
    }
    let compressed_bytes = idat.iter().try_fold(0usize, |total, chunk| total.checked_add(chunk.data_end - chunk.data_start).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "png: compressed IDAT extent overflow")))?;
    let mut compressed = control.allocate_vec(compressed_bytes)?;
    control.begin_stage(compressed_bytes)?;
    for chunk in idat {
        let bytes = &snapshot.bytes[chunk.data_start..chunk.data_end];
        compressed.extend_from_slice(bytes);
        control.advance(bytes.len())?;
    }
    if let Some(phase) = phase {
        phase.set(PngNativePaintPhase::Decode);
    }
    let expected = expected_native_raw_bytes(layout)?;
    let raw = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::binary::snapshot::decompress_zlib(&compressed, expected, control)?;
    if raw.len() != expected {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("png: decompressed extent {} differs from expected {expected}", raw.len())));
    }
    native_rows_from_raw_controlled(&raw, layout, control)
}

fn native_rows_from_raw_controlled(raw: &[u8], layout: &PngLayout, control: &mut NativeDecodeControl<'_>) -> Result<Vec<NativePassRow>, ValueError> {
    let mut rows = control.allocate_vec::<NativePassRow>(native_row_count(layout)?)?;
    let bpp = (layout.color_type.samples_per_pixel() * layout.bit_depth as usize).div_ceil(8).max(1);
    let mut position = 0usize;
    if layout.interlace {
        for (pass, &(start_x, start_y, step_x, step_y)) in ADAM7.iter().enumerate() {
            let (width, height) = adam7_pass_dims(layout.width, layout.height, pass);
            if width != 0 && height != 0 {
                position = decode_native_pass(&raw, position, width, height, start_x, start_y, step_x, step_y, bpp, layout, &mut rows, control)?;
            }
        }
    } else {
        position = decode_native_pass(&raw, position, layout.width, layout.height, 0, 0, 1, 1, bpp, layout, &mut rows, control)?;
    }
    if position != raw.len() {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("png: {} trailing decompressed bytes", raw.len() - position)));
    }
    Ok(rows)
}

fn write_native_sample(row: &mut [u8], ordinal: usize, bit_depth: u8, sample: u16) {
    match bit_depth {
        16 => row[ordinal * 2..ordinal * 2 + 2].copy_from_slice(&sample.to_be_bytes()),
        8 => row[ordinal] = sample as u8,
        depth => {
            let bit = ordinal * depth as usize;
            let shift = 8 - depth as usize - bit % 8;
            let mask = (((1u16 << depth) - 1) as u8) << shift;
            row[bit / 8] = (row[bit / 8] & !mask) | ((sample as u8) << shift & mask);
        }
    }
}

fn native_profile(layout: &PngLayout) -> PngNativeProfile {
    match layout.color_type {
        PngColorType::Grayscale => PngNativeProfile::Grayscale,
        PngColorType::Rgb => PngNativeProfile::Rgb,
        PngColorType::Palette => PngNativeProfile::Indexed,
        PngColorType::GrayscaleAlpha => PngNativeProfile::GrayscaleAlpha,
        PngColorType::Rgba => PngNativeProfile::Rgba,
    }
}

fn checked_native_paint(snapshot: &PngSnapshot, layout: &PngLayout, paint: PngNativePaint) -> Result<(), String> {
    let expected = native_profile(layout);
    if paint.profile != expected { return Err(format!("png: native paint profile {:?} does not match {:?}", paint.profile, expected)); }
    let maximum = if layout.bit_depth == 16 { u16::MAX } else { ((1u32 << layout.bit_depth) - 1) as u16 };
    let sample_count = layout.color_type.samples_per_pixel();
    if paint.samples()[..sample_count].iter().any(|sample| *sample > maximum) {
        return Err(format!("png: native sample exceeds {maximum} for {}-bit profile", layout.bit_depth));
    }
    if paint.profile == PngNativeProfile::Indexed {
        let palette_entries = layout.chunks.iter().find(|chunk| chunk.kind == *b"PLTE").map_or(0, |chunk| (chunk.data_end - chunk.data_start) / 3);
        if usize::from(paint.first) >= palette_entries { return Err(format!("png: palette index {} exceeds {} entries", paint.first, palette_entries)); }
    }
    let _ = snapshot;
    Ok(())
}

pub fn validate_native_paint(snapshot: &PngSnapshot, region: PngRegion, paint: PngNativePaint) -> Result<usize, String> {
    let layout = source_layout(snapshot)?;
    checked_region(&layout, region)?;
    checked_native_paint(snapshot, &layout, paint)?;
    native_row_count(&layout).map_err(ValueError::into_message)
}

fn append_filtered_row(output: &mut Vec<u8>, row: &NativePassRow, previous: Option<&[u8]>, bpp: usize, control: &mut NativeEncodeControl<'_>) -> Result<(), ValueError> {
    output.push(row.filter);
    control.step()?;
    for index in 0..row.samples.len() {
        let current = row.samples[index];
        let a = if index >= bpp { row.samples[index - bpp] } else { 0 };
        let b = previous.map_or(0, |value| value[index]);
        let c = if index >= bpp { previous.map_or(0, |value| value[index - bpp]) } else { 0 };
        output.push(match row.filter {
            0 => current,
            1 => current.wrapping_sub(a),
            2 => current.wrapping_sub(b),
            3 => current.wrapping_sub(((u16::from(a) + u16::from(b)) / 2) as u8),
            4 => current.wrapping_sub(paeth(a, b, c)),
            _ => unreachable!("decoded filters are validated"),
        });
        control.step()?;
    }
    Ok(())
}

fn encode_native_rows_controlled(rows: &[NativePassRow], layout: &PngLayout, control: &mut NativeEncodeControl<'_>) -> Result<Vec<u8>, ValueError> {
    let length = rows.iter().try_fold(0usize, |total, row| total.checked_add(row.samples.len().checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "png: filtered row extent overflow"))?).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "png: filtered raster extent overflow")))?;
    let mut raw = control.allocate_vec(length)?;
    control.begin_stage(length)?;
    let mut previous = None;
    let mut previous_pass = None;
    let bpp = (layout.color_type.samples_per_pixel() * layout.bit_depth as usize).div_ceil(8).max(1);
    for (index, row) in rows.iter().enumerate() {
        let pass = (row.start_x, row.step_x);
        if previous_pass != Some(pass) {
            previous = None;
        }
        append_filtered_row(&mut raw, row, previous.map(|position: usize| rows[position].samples.as_slice()), bpp, control)?;
        previous = Some(index);
        previous_pass = Some(pass);
    }
    Ok(raw)
}

fn write_chunk_preallocated(output: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    output.extend_from_slice(&(data.len() as u32).to_be_bytes());
    output.extend_from_slice(kind);
    output.extend_from_slice(data);
    output.extend_from_slice(&png_crc32_parts(kind, data).to_be_bytes());
}

fn retire_png_vec_step<T>(values: &mut Vec<T>, maximum_items: usize, maximum_bytes: usize) -> Option<(usize, usize)> {
    let item_bytes = std::mem::size_of::<T>();
    if !values.is_empty() {
        let byte_items = if item_bytes == 0 { maximum_items } else { maximum_bytes / item_bytes };
        let released_items = values.len().min(maximum_items).min(byte_items);
        if released_items == 0 { return Some((0, 0)); }
        values.truncate(values.len() - released_items);
        return Some((released_items, released_items * item_bytes));
    }
    if values.capacity() == 0 { return None; }
    let backing = values.capacity().checked_mul(item_bytes).unwrap_or(usize::MAX);
    if maximum_items == 0 || maximum_bytes < backing { return Some((0, 0)); }
    drop(std::mem::take(values));
    Some((1, backing))
}

fn retire_png_string_step(value: &mut String, maximum_items: usize, maximum_bytes: usize) -> Option<(usize, usize)> {
    if !value.is_empty() {
        let bytes = value.chars().next_back().map_or(0, char::len_utf8);
        if maximum_items == 0 || maximum_bytes < bytes { return Some((0, 0)); }
        value.pop();
        return Some((1, bytes));
    }
    if value.capacity() == 0 { return None; }
    let backing = value.capacity();
    if maximum_items == 0 || maximum_bytes < backing { return Some((0, 0)); }
    drop(std::mem::take(value));
    Some((1, backing))
}

#[derive(Debug)]
pub struct PngNativePaintOperation {
    revision: String,
    layout: PngLayout,
    rows: Vec<NativePassRow>,
    region: PngRegion,
    paint: PngNativePaint,
    cursor: usize,
    maximum_owned_bytes: usize,
    admitted_owned_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedNativePaintPhase {
    Decode,
    Paint,
    Filter,
    Encode,
    Assemble,
    Complete,
}

/// 🧵️ Result of one bounded exact native-paint step.
#[derive(Debug)]
pub enum PngNativePaintWorkStep {
    Yield(PngNativePaintProgress),
    Complete,
    Cancelled,
}

/// 🌊️ Retained exact-sample paint with resumable RFC 1950 decode and encode phases.
pub struct PngNativePaintWorkOperation {
    revision: String,
    layout: PngLayout,
    idat_first_index: usize,
    idat_last_index: usize,
    region: PngRegion,
    paint: PngNativePaint,
    decoder: Option<semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::RetainedZlibDecoder>,
    encoder: Option<semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::RetainedZlibEncoder>,
    raw: Vec<u8>,
    rows: Vec<NativePassRow>,
    decode_row: Option<NativePassRow>,
    decode_raw_cursor: usize,
    decode_sample_cursor: usize,
    cursor: usize,
    pixel_cursor: usize,
    filter_cursor: usize,
    compressed: Vec<u8>,
    assembled: Vec<u8>,
    assemble_stage: u8,
    assemble_field_cursor: usize,
    assemble_chunk_index: usize,
    assemble_data_offset: usize,
    assemble_crc: u32,
    maximum_owned_bytes: usize,
    phase: RetainedNativePaintPhase,
    result: Option<PngSnapshot>,
    closing: bool,
}

impl PngNativePaintWorkOperation {
    pub fn try_new(snapshot: &PngSnapshot, revision: &str, region: PngRegion, paint: PngNativePaint, maximum_owned_bytes: usize) -> Result<Self, String> {
        require_revision(snapshot, revision)?;
        let layout = source_layout(snapshot)?;
        checked_region(&layout, region)?;
        checked_native_paint(snapshot, &layout, paint)?;
        let expected = expected_native_raw_bytes(&layout).map_err(ValueError::into_message)?;
        let first = layout.chunks.iter().position(|chunk| chunk.kind == *b"IDAT").ok_or("png: missing IDAT")?;
        let last = layout.chunks.iter().rposition(|chunk| chunk.kind == *b"IDAT").expect("nonempty IDAT list");
        if layout.chunks[first..=last].iter().any(|chunk| chunk.kind != *b"IDAT") {
            return Err("png: IDAT chunks must be consecutive".into());
        }
        let compressed_bytes = layout.chunks[first..=last].iter().try_fold(0usize, |total, chunk| total.checked_add(chunk.data_end - chunk.data_start).ok_or("png: compressed IDAT extent overflow"))?;
        let mut zlib_header = [0u8; 2];
        let mut zlib_header_length = 0usize;
        for chunk in &layout.chunks[first..=last] {
            for byte in &snapshot.bytes[chunk.data_start..chunk.data_end] {
                if zlib_header_length == zlib_header.len() {
                    break;
                }
                zlib_header[zlib_header_length] = *byte;
                zlib_header_length += 1;
            }
            if zlib_header_length == zlib_header.len() {
                break;
            }
        }
        if zlib_header_length != zlib_header.len() {
            return Err("png: retained zlib stream has no complete header".into());
        }
        if zlib_header[0] & 15 != 8 || zlib_header[0] >> 4 > 7 {
            return Err("png: unsupported retained zlib method or window".into());
        }
        let window = 1usize << ((zlib_header[0] >> 4) + 8);
        let preflight = layout
            .chunks
            .capacity()
            .checked_mul(std::mem::size_of::<PngChunkAddress>())
            .and_then(|bytes| bytes.checked_add(revision.len()))
            .and_then(|bytes| bytes.checked_add(compressed_bytes))
            .and_then(|bytes| bytes.checked_add(expected))
            .and_then(|bytes| bytes.checked_add(window))
            .ok_or("png: retained paint preflight ownership overflow")?;
        if preflight > maximum_owned_bytes {
            return Err("png: retained paint preflight exceeds caller ownership limit".into());
        }
        let mut compressed = Vec::new();
        compressed.try_reserve_exact(compressed_bytes).map_err(|_| "png: retained compressed input allocation failed")?;
        for chunk in &layout.chunks[first..=last] {
            compressed.extend_from_slice(&snapshot.bytes[chunk.data_start..chunk.data_end]);
        }
        let revision = revision.to_owned();
        let scaffold = layout
            .chunks
            .capacity()
            .checked_mul(std::mem::size_of::<PngChunkAddress>())
            .and_then(|bytes| bytes.checked_add(revision.capacity()))
            .ok_or("png: retained paint scaffold ownership overflow")?;
        let remaining = maximum_owned_bytes.checked_sub(scaffold).ok_or("png: retained paint scaffold exceeds caller ownership limit")?;
        let decoder = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::RetainedZlibDecoder::try_new(compressed, expected, remaining).map_err(ValueError::into_message)?;
        Ok(Self {
            revision,
            layout,
            idat_first_index: first,
            idat_last_index: last,
            region,
            paint,
            decoder: Some(decoder),
            encoder: None,
            raw: Vec::new(),
            rows: Vec::new(),
            decode_row: None,
            decode_raw_cursor: 0,
            decode_sample_cursor: 0,
            cursor: 0,
            pixel_cursor: 0,
            filter_cursor: 0,
            compressed: Vec::new(),
            assembled: Vec::new(),
            assemble_stage: 0,
            assemble_field_cursor: 0,
            assemble_chunk_index: 0,
            assemble_data_offset: 0,
            assemble_crc: 0xffff_ffff,
            maximum_owned_bytes,
            phase: RetainedNativePaintPhase::Decode,
            result: None,
            closing: false,
        })
    }

    fn retained_scaffold_bytes(&self) -> Result<usize, String> {
        let row_headers = self.rows.capacity().checked_mul(std::mem::size_of::<NativePassRow>()).ok_or("png: retained row-header ownership overflow")?;
        let rows = self.rows.iter().try_fold(row_headers, |total, row| total.checked_add(row.samples.capacity()).ok_or("png: retained row ownership overflow"))?;
        let rows = rows.checked_add(self.decode_row.as_ref().map_or(0, |row| row.samples.capacity())).ok_or("png: retained decode-row ownership overflow")?;
        rows.checked_add(self.layout.chunks.capacity().checked_mul(std::mem::size_of::<PngChunkAddress>()).ok_or("png: retained chunk-address ownership overflow")?)
            .and_then(|bytes| bytes.checked_add(self.revision.capacity()))
            .and_then(|bytes| bytes.checked_add(self.raw.capacity()))
            .and_then(|bytes| bytes.checked_add(self.compressed.capacity()))
            .and_then(|bytes| bytes.checked_add(self.assembled.capacity()))
            .ok_or_else(|| "png: retained paint scaffold ownership overflow".into())
    }

    fn retained_owned_bytes(&self) -> Result<usize, String> {
        let mut total = self.retained_scaffold_bytes()?;
        if let Some(decoder) = &self.decoder { total = total.checked_add(decoder.owned_bytes().map_err(ValueError::into_message)?).ok_or("png: retained decoder ownership overflow")?; }
        if let Some(encoder) = &self.encoder { total = total.checked_add(encoder.owned_bytes().map_err(ValueError::into_message)?).ok_or("png: retained encoder ownership overflow")?; }
        if let Some(result) = &self.result { total = total.checked_add(result.schema.capacity()).and_then(|bytes| bytes.checked_add(result.bytes.capacity())).ok_or("png: retained result ownership overflow")?; }
        Ok(total)
    }

    fn progress(&self) -> PngNativePaintProgress {
        let (phase, completed, total) = match self.phase {
            RetainedNativePaintPhase::Decode if self.raw.is_empty() => self.decoder.as_ref().map_or((PngNativePaintPhase::Decode, 0, 1), |decoder| { let (completed, total) = decoder.progress(); (PngNativePaintPhase::Decode, completed, total.max(1)) }),
            RetainedNativePaintPhase::Decode => (PngNativePaintPhase::Decode, self.decode_raw_cursor, self.raw.len().max(1)),
            RetainedNativePaintPhase::Paint => (PngNativePaintPhase::Paint, self.cursor, self.rows.len().max(1)),
            RetainedNativePaintPhase::Filter => (PngNativePaintPhase::Filter, self.cursor, self.rows.len().max(1)),
            RetainedNativePaintPhase::Encode => self.encoder.as_ref().map_or((PngNativePaintPhase::Encode, 0, 1), |encoder| { let (completed, total) = encoder.progress(); (PngNativePaintPhase::Encode, completed, total.max(1)) }),
            RetainedNativePaintPhase::Assemble => (PngNativePaintPhase::Assemble, self.assembled.len(), self.assembled.capacity().max(1)),
            RetainedNativePaintPhase::Complete => (PngNativePaintPhase::Assemble, 1, 1),
        };
        PngNativePaintProgress { phase, completed, total, owned_bytes: self.retained_owned_bytes().unwrap_or(self.maximum_owned_bytes) }
    }

    fn row_spec(&self, ordinal: usize) -> Result<Option<(u32, u32, u32, u32, usize)>, String> {
        let mut seen = 0usize;
        if self.layout.interlace {
            for (pass, &(start_x, start_y, step_x, step_y)) in ADAM7.iter().enumerate() {
                let (width, height) = adam7_pass_dims(self.layout.width, self.layout.height, pass);
                if width == 0 || height == 0 {
                    continue;
                }
                let end = seen.checked_add(height as usize).ok_or("png: retained row ordinal overflow")?;
                if ordinal < end {
                    let row = ordinal - seen;
                    return Ok(Some((start_x, start_y + row as u32 * step_y, step_x, width, packed_row_bytes(width, self.layout.color_type.to_u8(), self.layout.bit_depth))));
                }
                seen = end;
            }
            return Ok(None);
        }
        if ordinal >= self.layout.height as usize {
            return Ok(None);
        }
        Ok(Some((0, ordinal as u32, 1, self.layout.width, packed_row_bytes(self.layout.width, self.layout.color_type.to_u8(), self.layout.bit_depth))))
    }

    fn start_decode_row(&mut self) -> Result<bool, String> {
        let Some((start_x, y, step_x, width, row_bytes)) = self.row_spec(self.rows.len())? else {
            if self.decode_raw_cursor != self.raw.len() {
                return Err(format!("png: {} trailing decompressed bytes", self.raw.len() - self.decode_raw_cursor));
            }
            self.phase = RetainedNativePaintPhase::Paint;
            self.cursor = 0;
            return Ok(false);
        };
        let filter = *self.raw.get(self.decode_raw_cursor).ok_or("png: truncated retained scanline filter")?;
        if filter > 4 {
            return Err(format!("png: unsupported filter type {filter}"));
        }
        self.decode_raw_cursor += 1;
        let end = self.decode_raw_cursor.checked_add(row_bytes).ok_or("png: retained scanline extent overflow")?;
        if end > self.raw.len() {
            return Err("png: truncated retained scanline data".into());
        }
        let mut samples = Vec::new();
        samples.try_reserve_exact(row_bytes).map_err(|_| "png: retained scanline allocation failed")?;
        self.decode_row = Some(NativePassRow { filter, samples, start_x, y, step_x, width });
        self.decode_sample_cursor = 0;
        if self.retained_owned_bytes()? > self.maximum_owned_bytes {
            return Err("png: retained scanline exceeds caller ownership limit".into());
        }
        Ok(true)
    }

    fn decode_sample(&mut self) -> Result<(), String> {
        if self.decode_row.is_none() && !self.start_decode_row()? {
            return Ok(());
        }
        let row = self.decode_row.as_ref().expect("started retained row");
        let row_bytes = packed_row_bytes(row.width, self.layout.color_type.to_u8(), self.layout.bit_depth);
        if self.decode_sample_cursor == row_bytes {
            self.decode_raw_cursor = self.decode_raw_cursor.checked_add(self.decode_sample_cursor).ok_or("png: retained scanline cursor overflow")?;
            self.rows.push(self.decode_row.take().expect("completed retained row"));
            self.decode_sample_cursor = 0;
            return Ok(());
        }
        let index = self.decode_sample_cursor;
        let byte = self.raw[self.decode_raw_cursor + index];
        let bpp = (self.layout.color_type.samples_per_pixel() * self.layout.bit_depth as usize).div_ceil(8).max(1);
        let previous = self.rows.last().filter(|previous| previous.start_x == row.start_x && previous.step_x == row.step_x).map(|previous| previous.samples.as_slice());
        let a = if index >= bpp { row.samples[index - bpp] } else { 0 };
        let b = previous.map_or(0, |previous| previous[index]);
        let c = if index >= bpp { previous.map_or(0, |previous| previous[index - bpp]) } else { 0 };
        let value = match row.filter {
            0 => byte,
            1 => byte.wrapping_add(a),
            2 => byte.wrapping_add(b),
            3 => byte.wrapping_add(((u16::from(a) + u16::from(b)) / 2) as u8),
            4 => byte.wrapping_add(paeth(a, b, c)),
            _ => unreachable!("retained filter was validated"),
        };
        self.decode_row.as_mut().expect("started retained row").samples.push(value);
        self.decode_sample_cursor += 1;
        Ok(())
    }

    fn paint_pixel(&mut self) {
        let Some(row) = self.rows.get_mut(self.cursor) else { return };
        if self.pixel_cursor == row.width as usize {
            self.cursor += 1;
            self.pixel_cursor = 0;
            return;
        }
        let x = row.start_x + self.pixel_cursor as u32 * row.step_x;
        if row.y >= self.region.y && row.y < self.region.y + self.region.height && x >= self.region.x && x < self.region.x + self.region.width {
            let samples = self.paint.samples();
            let sample_count = self.layout.color_type.samples_per_pixel();
            for (channel, sample) in samples[..sample_count].iter().enumerate() {
                write_native_sample(&mut row.samples, self.pixel_cursor * sample_count + channel, self.layout.bit_depth, *sample);
            }
        }
        self.pixel_cursor += 1;
    }

    fn filter_byte(&mut self) -> Result<bool, String> {
        let Some(row) = self.rows.get(self.cursor) else {
            return Ok(false);
        };
        if self.filter_cursor == 0 {
            self.raw.push(row.filter);
            self.filter_cursor = 1;
            return Ok(true);
        }
        let index = self.filter_cursor - 1;
        if index == row.samples.len() {
            self.cursor += 1;
            self.filter_cursor = 0;
            return Ok(false);
        }
        let previous = self.rows[..self.cursor].last().filter(|previous| previous.start_x == row.start_x && previous.step_x == row.step_x).map(|previous| previous.samples.as_slice());
        let bpp = (self.layout.color_type.samples_per_pixel() * self.layout.bit_depth as usize).div_ceil(8).max(1);
        let current = row.samples[index];
        let a = if index >= bpp { row.samples[index - bpp] } else { 0 };
        let b = previous.map_or(0, |previous| previous[index]);
        let c = if index >= bpp { previous.map_or(0, |previous| previous[index - bpp]) } else { 0 };
        self.raw.push(match row.filter {
            0 => current,
            1 => current.wrapping_sub(a),
            2 => current.wrapping_sub(b),
            3 => current.wrapping_sub(((u16::from(a) + u16::from(b)) / 2) as u8),
            4 => current.wrapping_sub(paeth(a, b, c)),
            _ => unreachable!("decoded filters are validated"),
        });
        self.filter_cursor += 1;
        Ok(true)
    }

    fn update_assembly_crc(&mut self, byte: u8) {
        self.assemble_crc ^= u32::from(byte);
        for _ in 0..8 {
            self.assemble_crc = if self.assemble_crc & 1 != 0 { (self.assemble_crc >> 1) ^ 0xedb8_8320 } else { self.assemble_crc >> 1 };
        }
    }

    fn initialize_assembly(&mut self, snapshot: &PngSnapshot) -> Result<(), String> {
        let first_index = self.idat_first_index;
        let last_index = self.idat_last_index;
        let count = last_index - first_index + 1;
        let total = self.layout.chunks[first_index]
            .start
            .checked_add(self.compressed.len())
            .and_then(|bytes| bytes.checked_add(count.checked_mul(12)?))
            .and_then(|bytes| bytes.checked_add(snapshot.bytes.len() - self.layout.chunks[last_index].end))
            .ok_or("png: retained assembled extent overflow")?;
        let admitted = self.retained_owned_bytes()?.checked_add(total).and_then(|bytes| bytes.checked_add(snapshot.schema.len())).ok_or("png: retained assembled ownership overflow")?;
        if admitted > self.maximum_owned_bytes {
            return Err("png: retained assembled result exceeds caller ownership limit".into());
        }
        self.assembled.try_reserve_exact(total).map_err(|_| "png: retained assembled allocation failed")?;
        if self.retained_owned_bytes()? > self.maximum_owned_bytes {
            return Err("png: retained assembled allocation exceeds caller ownership limit".into());
        }
        self.assemble_stage = 0;
        self.assemble_field_cursor = 0;
        self.assemble_chunk_index = first_index;
        self.assemble_data_offset = 0;
        self.assemble_crc = 0xffff_ffff;
        Ok(())
    }

    fn assembly_chunk_take(&self, last_index: usize) -> usize {
        let remaining = self.compressed.len() - self.assemble_data_offset;
        if self.assemble_chunk_index == last_index {
            remaining
        } else {
            let chunk = &self.layout.chunks[self.assemble_chunk_index];
            (chunk.data_end - chunk.data_start).min(remaining)
        }
    }

    fn assemble_byte(&mut self, snapshot: &PngSnapshot) -> Result<bool, String> {
        let first_index = self.idat_first_index;
        let last_index = self.idat_last_index;
        match self.assemble_stage {
            0 => {
                let end = self.layout.chunks[first_index].start;
                if self.assemble_field_cursor == end {
                    self.assemble_stage = 1;
                    self.assemble_field_cursor = 0;
                    return Ok(false);
                }
                self.assembled.push(snapshot.bytes[self.assemble_field_cursor]);
                self.assemble_field_cursor += 1;
            }
            1 => {
                if self.assemble_chunk_index > last_index {
                    self.assemble_stage = 5;
                    self.assemble_field_cursor = self.layout.chunks[last_index].end;
                    return Ok(false);
                }
                let length = u32::try_from(self.assembly_chunk_take(last_index)).map_err(|_| "png: retained IDAT chunk exceeds PNG length field")?.to_be_bytes();
                self.assembled.push(length[self.assemble_field_cursor]);
                self.assemble_field_cursor += 1;
                if self.assemble_field_cursor == length.len() {
                    self.assemble_stage = 2;
                    self.assemble_field_cursor = 0;
                    self.assemble_crc = 0xffff_ffff;
                }
            }
            2 => {
                let byte = b"IDAT"[self.assemble_field_cursor];
                self.assembled.push(byte);
                self.update_assembly_crc(byte);
                self.assemble_field_cursor += 1;
                if self.assemble_field_cursor == 4 {
                    self.assemble_stage = 3;
                    self.assemble_field_cursor = 0;
                }
            }
            3 => {
                let take = self.assembly_chunk_take(last_index);
                if self.assemble_field_cursor == take {
                    self.assemble_data_offset += take;
                    self.assemble_stage = 4;
                    self.assemble_field_cursor = 0;
                    return Ok(false);
                }
                let byte = self.compressed[self.assemble_data_offset + self.assemble_field_cursor];
                self.assembled.push(byte);
                self.update_assembly_crc(byte);
                self.assemble_field_cursor += 1;
            }
            4 => {
                let crc = (!self.assemble_crc).to_be_bytes();
                self.assembled.push(crc[self.assemble_field_cursor]);
                self.assemble_field_cursor += 1;
                if self.assemble_field_cursor == crc.len() {
                    self.assemble_chunk_index += 1;
                    self.assemble_stage = 1;
                    self.assemble_field_cursor = 0;
                }
            }
            5 => {
                if self.assemble_field_cursor == snapshot.bytes.len() {
                    self.assemble_stage = 6;
                    return Ok(false);
                }
                self.assembled.push(snapshot.bytes[self.assemble_field_cursor]);
                self.assemble_field_cursor += 1;
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    /// ➡️ Advances native work and yields at every caller fuel/deadline boundary.
    pub fn advance(&mut self, snapshot: &PngSnapshot, context: &mut semio_framework_job::StepContext<'_>) -> Result<PngNativePaintWorkStep, String> {
        if self.closing {
            return Err("png: native paint work is closing".into());
        }
        require_revision(snapshot, &self.revision)?;
        loop {
            if context.is_cancelled() {
                return Ok(PngNativePaintWorkStep::Cancelled);
            }
            if context.should_yield() {
                return Ok(PngNativePaintWorkStep::Yield(self.progress()));
            }
            match self.phase {
                RetainedNativePaintPhase::Decode => {
                    if self.raw.is_empty() {
                        let decoder = self.decoder.as_mut().ok_or("png: retained decoder is missing")?;
                        match decoder.advance(context).map_err(ValueError::into_message)? {
                            semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::RetainedZlibStep::Yield => return Ok(PngNativePaintWorkStep::Yield(self.progress())),
                            semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::RetainedZlibStep::Cancelled => return Ok(PngNativePaintWorkStep::Cancelled),
                            semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::RetainedZlibStep::Complete => {
                                self.raw = decoder.take_output().map_err(ValueError::into_message)?;
                                let row_count = native_row_count(&self.layout).map_err(ValueError::into_message)?;
                                self.rows.try_reserve_exact(row_count).map_err(|_| "png: retained row table allocation failed")?;
                                if self.retained_owned_bytes()? > self.maximum_owned_bytes {
                                    return Err("png: retained decoded raster exceeds caller ownership limit".into());
                                }
                            }
                        }
                    } else {
                        self.decode_sample()?;
                        context.consume_fuel(1);
                    }
                }
                RetainedNativePaintPhase::Paint => {
                    if self.cursor == self.rows.len() {
                        self.phase = RetainedNativePaintPhase::Filter;
                        self.raw.clear();
                        self.cursor = 0;
                        self.filter_cursor = 0;
                        continue;
                    }
                    self.paint_pixel();
                    context.consume_fuel(1);
                }
                RetainedNativePaintPhase::Filter => {
                    if self.cursor == self.rows.len() {
                        let raw = std::mem::take(&mut self.raw);
                        let remaining = self.maximum_owned_bytes.checked_sub(self.retained_owned_bytes()?).ok_or("png: retained filter ownership exceeds caller limit")?;
                        self.encoder = Some(semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::RetainedZlibEncoder::try_new(raw, self.maximum_owned_bytes, remaining).map_err(ValueError::into_message)?);
                        if self.retained_owned_bytes()? > self.maximum_owned_bytes {
                            return Err("png: retained encoder exceeds caller ownership limit".into());
                        }
                        self.phase = RetainedNativePaintPhase::Encode;
                        continue;
                    }
                    if self.filter_byte()? {
                        context.consume_fuel(1);
                    }
                }
                RetainedNativePaintPhase::Encode => {
                    let encoder = self.encoder.as_mut().ok_or("png: retained encoder is missing")?;
                    match encoder.advance(context).map_err(ValueError::into_message)? {
                        semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::RetainedZlibStep::Yield => return Ok(PngNativePaintWorkStep::Yield(self.progress())),
                        semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::RetainedZlibStep::Cancelled => return Ok(PngNativePaintWorkStep::Cancelled),
                        semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::RetainedZlibStep::Complete => {
                            self.compressed = encoder.take_output().map_err(ValueError::into_message)?;
                            self.phase = RetainedNativePaintPhase::Assemble;
                            self.initialize_assembly(snapshot)?;
                        }
                    }
                }
                RetainedNativePaintPhase::Assemble => {
                    if self.assemble_stage == 6 {
                        let result = PngSnapshot { schema: snapshot.schema.clone(), bytes: std::mem::take(&mut self.assembled) };
                        validate_completed_native_paint_domain(snapshot, &result, self.region, self.paint)?;
                        self.result = Some(result);
                        if self.retained_owned_bytes()? > self.maximum_owned_bytes {
                            return Err("png: retained completed result exceeds caller ownership limit".into());
                        }
                        self.phase = RetainedNativePaintPhase::Complete;
                        return Ok(PngNativePaintWorkStep::Complete);
                    }
                    if self.assemble_byte(snapshot)? {
                        context.consume_fuel(1);
                    }
                }
                RetainedNativePaintPhase::Complete => return Ok(PngNativePaintWorkStep::Complete),
            }
        }
    }

    pub fn take_result(&mut self) -> Result<PngSnapshot, String> {
        if self.phase != RetainedNativePaintPhase::Complete {
            return Err("png: retained native paint result requested before completion".into());
        }
        self.result.take().ok_or_else(|| "png: retained native paint result was already taken".into())
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
        if let Some(decoder) = self.decoder.as_mut() { decoder.begin_close(); }
        if let Some(encoder) = self.encoder.as_mut() { encoder.begin_close(); }
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if let Some(decoder) = self.decoder.as_mut() {
            let step = decoder.close_step(maximum_items, maximum_bytes);
            if step != InteractiveJobCloseStep::Complete { return step; }
            if !decoder.terminal_is_empty() { return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }; }
            self.decoder = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(encoder) = self.encoder.as_mut() {
            let step = encoder.close_step(maximum_items, maximum_bytes);
            if step != InteractiveJobCloseStep::Complete { return step; }
            if !encoder.terminal_is_empty() { return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }; }
            self.encoder = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(row) = self.decode_row.as_mut() {
            if let Some(step) = retire_png_vec_step(&mut row.samples, maximum_items, maximum_bytes) {
                return InteractiveJobCloseStep::Pending { released_items: step.0, released_bytes: step.1 };
            }
            self.decode_row = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(row) = self.rows.last_mut() {
            if let Some(step) = retire_png_vec_step(&mut row.samples, maximum_items, maximum_bytes) {
                return InteractiveJobCloseStep::Pending { released_items: step.0, released_bytes: step.1 };
            }
            if maximum_items == 0 { return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }; }
            self.rows.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(step) = retire_png_vec_step(&mut self.rows, maximum_items, maximum_bytes) {
            return InteractiveJobCloseStep::Pending { released_items: step.0, released_bytes: step.1 };
        }
        if let Some(step) = retire_png_vec_step(&mut self.raw, maximum_items, maximum_bytes) {
            return InteractiveJobCloseStep::Pending { released_items: step.0, released_bytes: step.1 };
        }
        if let Some(step) = retire_png_vec_step(&mut self.compressed, maximum_items, maximum_bytes) {
            return InteractiveJobCloseStep::Pending { released_items: step.0, released_bytes: step.1 };
        }
        if let Some(step) = retire_png_vec_step(&mut self.assembled, maximum_items, maximum_bytes) {
            return InteractiveJobCloseStep::Pending { released_items: step.0, released_bytes: step.1 };
        }
        if let Some(step) = retire_png_vec_step(&mut self.layout.chunks, maximum_items, maximum_bytes) {
            return InteractiveJobCloseStep::Pending { released_items: step.0, released_bytes: step.1 };
        }
        if let Some(result) = self.result.as_mut() {
            if let Some(step) = retire_png_vec_step(&mut result.bytes, maximum_items, maximum_bytes) {
                return InteractiveJobCloseStep::Pending { released_items: step.0, released_bytes: step.1 };
            }
            if let Some(step) = retire_png_string_step(&mut result.schema, maximum_items, maximum_bytes) {
                return InteractiveJobCloseStep::Pending { released_items: step.0, released_bytes: step.1 };
            }
            self.result = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(step) = retire_png_string_step(&mut self.revision, maximum_items, maximum_bytes) {
            return InteractiveJobCloseStep::Pending { released_items: step.0, released_bytes: step.1 };
        }
        InteractiveJobCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.decoder.is_none()
            && self.encoder.is_none()
            && self.decode_row.is_none()
            && self.rows.capacity() == 0
            && self.raw.capacity() == 0
            && self.compressed.capacity() == 0
            && self.assembled.capacity() == 0
            && self.layout.chunks.capacity() == 0
            && self.revision.capacity() == 0
            && self.result.is_none()
    }
}

impl PngNativePaintOperation {
    pub fn total_rows(&self) -> usize { self.rows.len() }
    pub fn painted_rows(&self) -> usize { self.cursor }
    pub fn paint_complete(&self) -> bool { self.cursor == self.rows.len() }
    pub fn admitted_owned_bytes(&self) -> usize { self.admitted_owned_bytes }

    pub fn paint_next(&mut self) -> bool {
        let Some(row) = self.rows.get_mut(self.cursor) else { return false };
        if row.y >= self.region.y && row.y < self.region.y + self.region.height {
            let samples = self.paint.samples();
            let sample_count = self.layout.color_type.samples_per_pixel();
            for pixel in 0..row.width as usize {
                let x = row.start_x + pixel as u32 * row.step_x;
                if x >= self.region.x && x < self.region.x + self.region.width {
                    for (channel, sample) in samples[..sample_count].iter().enumerate() {
                        write_native_sample(&mut row.samples, pixel * sample_count + channel, self.layout.bit_depth, *sample);
                    }
                }
            }
        }
        self.cursor += 1;
        true
    }

    pub fn finish(self, snapshot: &PngSnapshot, progress: &mut dyn FnMut(PngNativePaintProgress) -> bool) -> Result<PngSnapshot, String> {
        require_revision(snapshot, &self.revision)?;
        if !self.paint_complete() {
            return Err("png: native paint rows are incomplete".into());
        }
        let remaining = self.maximum_owned_bytes.checked_sub(self.admitted_owned_bytes).ok_or("png: native paint ownership accounting overflow")?;
        let phase = std::cell::Cell::new(PngNativePaintPhase::Filter);
        let mut callback = |event: NativeEncodeProgress| progress(PngNativePaintProgress {
            phase: phase.get(),
            completed: event.completed,
            total: event.total,
            owned_bytes: self.admitted_owned_bytes.saturating_add(event.owned_bytes),
        });
        let mut control = NativeEncodeControl::new(remaining, &mut callback);
        let raw = encode_native_rows_controlled(&self.rows, &self.layout, &mut control).map_err(ValueError::into_message)?;
        phase.set(PngNativePaintPhase::Encode);
        let compressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::binary::snapshot::compress_zlib(&raw, self.maximum_owned_bytes, &mut control).map_err(ValueError::into_message)?;
        let first_index = self.layout.chunks.iter().position(|chunk| chunk.kind == *b"IDAT").ok_or("png: missing IDAT")?;
        let last_index = self.layout.chunks.iter().rposition(|chunk| chunk.kind == *b"IDAT").expect("nonempty IDAT list");
        let idat = &self.layout.chunks[first_index..=last_index];
        let first = &idat[0];
        let last = &idat[idat.len() - 1];
        phase.set(PngNativePaintPhase::Assemble);
        let replacement_length = compressed.len().checked_add(idat.len().checked_mul(12).ok_or("png: IDAT framing extent overflow")?).ok_or("png: IDAT replacement extent overflow")?;
        let mut replacement = control.allocate_vec(replacement_length).map_err(ValueError::into_message)?;
        let mut offset = 0usize;
        for (index, chunk) in idat.iter().enumerate() {
            let original = chunk.data_end - chunk.data_start;
            let remaining_compressed = compressed.len() - offset;
            let take = if index + 1 == idat.len() { remaining_compressed } else { original.min(remaining_compressed) };
            write_chunk_preallocated(&mut replacement, b"IDAT", &compressed[offset..offset + take]);
            offset += take;
        }
        let total = first.start.checked_add(replacement.len()).and_then(|value| value.checked_add(snapshot.bytes.len() - last.end)).ok_or("png: painted source extent overflow")?;
        let mut bytes = control.allocate_vec(total).map_err(ValueError::into_message)?;
        control.begin_stage(total).map_err(ValueError::into_message)?;
        bytes.extend_from_slice(&snapshot.bytes[..first.start]);
        control.advance(first.start).map_err(ValueError::into_message)?;
        bytes.extend_from_slice(&replacement);
        control.advance(replacement.len()).map_err(ValueError::into_message)?;
        bytes.extend_from_slice(&snapshot.bytes[last.end..]);
        control.advance(snapshot.bytes.len() - last.end).map_err(ValueError::into_message)?;
        let schema = control.copy_text(&snapshot.schema).map_err(ValueError::into_message)?;
        Ok(PngSnapshot { schema, bytes })
    }
}

fn validate_completed_native_paint_domain(
    base: &PngSnapshot,
    result: &PngSnapshot,
    region: PngRegion,
    paint: PngNativePaint,
) -> Result<(), String> {
    let base_layout = source_layout(base)?;
    checked_region(&base_layout, region)?;
    checked_native_paint(base, &base_layout, paint)?;
    let result_layout = source_layout(result)?;
    if (base_layout.width, base_layout.height, base_layout.bit_depth, base_layout.color_type, base_layout.interlace)
        != (result_layout.width, result_layout.height, result_layout.bit_depth, result_layout.color_type, result_layout.interlace)
    {
        return Err("png: completed native paint changed the source profile".into());
    }
    let base_first = base_layout.chunks.iter().find(|chunk| chunk.kind == *b"IDAT").ok_or("png: missing base IDAT")?;
    let base_last = base_layout.chunks.iter().rfind(|chunk| chunk.kind == *b"IDAT").ok_or("png: missing base IDAT")?;
    let result_first = result_layout.chunks.iter().find(|chunk| chunk.kind == *b"IDAT").ok_or("png: missing result IDAT")?;
    let result_last = result_layout.chunks.iter().rfind(|chunk| chunk.kind == *b"IDAT").ok_or("png: missing result IDAT")?;
    let base_count = base_layout.chunks.iter().filter(|chunk| chunk.kind == *b"IDAT").count();
    let result_count = result_layout.chunks.iter().filter(|chunk| chunk.kind == *b"IDAT").count();
    if base_count != result_count {
        return Err("png: completed native paint changed the IDAT partition count".into());
    }
    if base.bytes[..base_first.start] != result.bytes[..result_first.start] || base.bytes[base_last.end..] != result.bytes[result_last.end..] {
        return Err("png: completed native paint changed bytes outside the IDAT run".into());
    }
    let base_idat = base_layout.chunks.iter().filter(|chunk| chunk.kind == *b"IDAT").collect::<Vec<_>>();
    let result_idat = result_layout.chunks.iter().filter(|chunk| chunk.kind == *b"IDAT").collect::<Vec<_>>();
    let result_extent = result_idat.iter().try_fold(0usize, |total, chunk| total.checked_add(chunk.data_end - chunk.data_start).ok_or("png: completed native paint IDAT extent overflow"))?;
    let mut remaining = result_extent;
    for (index, (before, after)) in base_idat.iter().zip(&result_idat).enumerate() {
        let expected = if index + 1 == base_idat.len() { remaining } else { (before.data_end - before.data_start).min(remaining) };
        if after.data_end - after.data_start != expected {
            return Err("png: completed native paint changed the authored IDAT partition convention".into());
        }
        remaining -= expected;
    }
    Ok(())
}

/// 🛡️ Validates a prepared paint result's closed IDAT replacement domain without replaying codecs.
pub fn validate_completed_native_paint(base: &PngSnapshot, result: &PngSnapshot, region: PngRegion, paint: PngNativePaint) -> Result<(), String> {
    validate_completed_native_paint_domain(base, result, region, paint)
}

pub fn begin_native_paint(
    snapshot: &PngSnapshot,
    revision: &str,
    region: PngRegion,
    paint: PngNativePaint,
    maximum_owned_bytes: usize,
    progress: &mut dyn FnMut(PngNativePaintProgress) -> bool,
) -> Result<PngNativePaintOperation, String> {
    require_revision(snapshot, revision)?;
    let phase = std::cell::Cell::new(PngNativePaintPhase::Address);
    let mut callback = |event: NativeDecodeProgress| progress(PngNativePaintProgress { phase: phase.get(), completed: event.completed, total: event.total, owned_bytes: event.owned_bytes });
    let mut control = NativeDecodeControl::new(maximum_owned_bytes, &mut callback);
    let revision = control.copy_text(revision).map_err(ValueError::into_message)?;
    control.charge(std::mem::size_of::<PngNativePaintOperation>()).map_err(ValueError::into_message)?;
    let layout = source_layout_controlled(snapshot, &mut control).map_err(ValueError::into_message)?;
    checked_region(&layout, region)?;
    checked_native_paint(snapshot, &layout, paint)?;
    let rows = native_rows_controlled(snapshot, &layout, Some(&phase), &mut control).map_err(ValueError::into_message)?;
    let admitted_owned_bytes = control.owned_bytes();
    Ok(PngNativePaintOperation { revision, layout, rows, region, paint, cursor: 0, maximum_owned_bytes, admitted_owned_bytes })
}

pub fn paint_native_region_owned_controlled(
    snapshot: &PngSnapshot,
    revision: &str,
    region: PngRegion,
    paint: PngNativePaint,
    maximum_owned_bytes: usize,
    progress: &mut dyn FnMut(PngNativePaintProgress) -> bool,
) -> Result<PngSnapshot, String> {
    let mut operation = begin_native_paint(snapshot, revision, region, paint, maximum_owned_bytes, progress)?;
    let total = operation.total_rows().max(1);
    while operation.paint_next() {
        if !progress(PngNativePaintProgress { phase: PngNativePaintPhase::Paint, completed: operation.painted_rows(), total, owned_bytes: operation.admitted_owned_bytes() }) {
            return Err("png: native paint canceled".into());
        }
    }
    operation.finish(snapshot, progress)
}

pub fn png_native_pixel(snapshot: &PngSnapshot, x: u32, y: u32) -> Result<Vec<u16>, String> {
    let layout = png_layout(snapshot)?;
    checked_region(&layout, PngRegion { x, y, width: 1, height: 1 })?;
    let mut callback = |_| true;
    let mut control = NativeDecodeControl::new(MAXIMUM_NATIVE_PAINT_OWNED_BYTES, &mut callback);
    let rows = native_rows_controlled(snapshot, &layout, None, &mut control).map_err(ValueError::into_message)?;
    let sample_count = layout.color_type.samples_per_pixel();
    for row in rows {
        if row.y != y || x < row.start_x || !(x - row.start_x).is_multiple_of(row.step_x) { continue; }
        let pixel = ((x - row.start_x) / row.step_x) as usize;
        if pixel >= row.width as usize { continue; }
        return Ok(unpack_samples(&row.samples, row.width as usize, sample_count, layout.bit_depth)[pixel * sample_count..pixel * sample_count + sample_count].iter().map(|sample| *sample as u16).collect());
    }
    Err(format!("png: native sample at {x},{y} is absent"))
}

pub fn paint_native_region_controlled(
    snapshot: &PngSnapshot,
    revision: &str,
    region: PngRegion,
    paint: PngNativePaint,
    progress: &mut dyn FnMut(usize, usize) -> bool,
) -> Result<PngSnapshot, String> {
    paint_native_region_owned_controlled(snapshot, revision, region, paint, MAXIMUM_NATIVE_PAINT_OWNED_BYTES, &mut |event| progress(event.completed, event.total))
}

pub fn paint_rgba8_region_controlled(
    snapshot: &PngSnapshot,
    revision: &str,
    region: PngRegion,
    color: [u8; 4],
    progress: &mut dyn FnMut(usize, usize) -> bool,
) -> Result<PngSnapshot, String> {
    let layout = png_layout(snapshot)?;
    if layout.color_type != PngColorType::Rgba || layout.bit_depth != 8 || layout.interlace {
        return Err("png: RGBA8 paint requires an 8-bit non-interlaced RGBA profile; use a profile-specific sample edit".into());
    }
    paint_native_region_controlled(snapshot, revision, region, PngNativePaint::rgba(color[0].into(), color[1].into(), color[2].into(), color[3].into()), progress)
}
//#endregion ExactAuthority

//#region Codec
/// 🚫 EncodeScopeNote: always emits color type 6 (RGBA) / bit depth 8 / interlace method 0 for
/// the PIXEL data. `pixels` is a canonical 8-bit-RGBA model, so re-encoding a decoded
/// palette/grayscale/16-bit/interlaced source will not byte-for-byte round-trip the original
/// file's IDAT — only its pixel content (see `codec_retention_law`). Decode (below) fully
/// supports the input diversity; only the raster half of encode canonicalizes — every typed
/// ancillary/text/unknown chunk IS honestly re-emitted, in the decoded relative chunk order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn author_png_projection(snap: &PngProjection) -> Result<Vec<u8>, String> {
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
                    write_chunk(&mut out, b"bKGD", &encode_bkgd(&rgba8_background(snap,b)?));
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
    png_layout_bytes(&out)?;
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn project_png(data: &[u8]) -> Result<PngProjection, String> {
    let chunks = read_chunks(data)?;
    validate_png_structure(&chunks)?;
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
            ihdr = Some(parse_ihdr(chunk).map_err(ValueError::into_message)?);
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
            if chunk[nul + 1] != 0 {
                return Err("png zTXt: compression method must be zero".into());
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
            if chunk[pos] > 1 {
                return Err("png iTXt: compression flag must be zero or one".into());
            }
            let compressed_flag = chunk[pos] == 1;
            if chunk[pos + 1] != 0 {
                return Err("png iTXt: compression method must be zero".into());
            }
            pos += 2; // compressed flag + compression method
            let nul2 = chunk[pos..].iter().position(|&b| b == 0).ok_or("png iTXt: missing NUL after language tag")?;
            let language_tag_bytes = &chunk[pos..pos + nul2];
            if !language_tag_bytes.is_ascii() {
                return Err("png iTXt: language tag must be ASCII".into());
            }
            let language_tag = std::str::from_utf8(language_tag_bytes).expect("ASCII language tag").to_owned();
            pos += nul2 + 1;
            let nul3 = chunk[pos..].iter().position(|&b| b == 0).ok_or("png iTXt: missing NUL after translated keyword")?;
            let translated_keyword = std::str::from_utf8(&chunk[pos..pos + nul3]).map_err(|_| "png iTXt: translated keyword must be UTF-8")?.to_owned();
            pos += nul3 + 1;
            let rest = &chunk[pos..];
            let value_bytes = if compressed_flag {
                let decompressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_decompress(rest)?;
                decompressed
            } else {
                rest.to_vec()
            };
            let value = String::from_utf8(value_bytes).map_err(|_| "png iTXt: text must be UTF-8")?;
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
            let px = pixel_to_rgba(&samples[i * spp..i * spp + spp], &ihdr, &palette, &palette_alpha, gray_trans, rgb_trans).map_err(ValueError::into_message)?;
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

    Ok(PngProjection {
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
    use crate::standards::v1_2::subsets::any::io::PngComposer as PngRawAnyComposer;
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

#[path="🚦️native/🦀️.rs"]
pub mod native;

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{PngDiff, PngMutation, PngSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.png` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct PngBuilderConstruction {
        snapshot: PngSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for PngBuilderConstruction {
        type Snapshot = PngSnapshot;
        type Mutation = PngMutation;
        type Diff = PngDiff;
        fn empty() -> Self {
            Self { snapshot: PngSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<PngSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PngSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_png_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <PngDiff as protocol::MutationDiff<PngSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::PngSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.png` parts.
    #[derive(Clone, Debug, Default)]
    pub struct PngParts {
        pub snapshot: Option<PngSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.png` (1.2/✳️any) sources.
    pub struct PngAnalyzerAnalysis;

    impl ArtifactAnalysis for PngAnalyzerAnalysis {
        type Parts = PngParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            const SIG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 8 && bytes[0..8] == SIG {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    // 🔍 stdio.png's text envelope is a hex dump of the raw bytes after the
                    // `semio ...` preamble line — decode the first 8 bytes to sniff the real signature.
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(16).collect();
                    if hex.len() < 16 {
                        return IoConfidence::Low;
                    }
                    let mut decoded = [0u8; 8];
                    for (i, byte) in decoded.iter_mut().enumerate() {
                        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                            Ok(b) => *byte = b,
                            Err(_) => return IoConfidence::Low,
                        }
                    }
                    if decoded == SIG {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PngParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <PngSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <PngSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec PngBuilderFacets {
        construction: PngBuilderConstruction,
        analysis: PngAnalyzerAnalysis,
        composition: crate::standards::v1_2::subsets::any::io::derived_composition::PngComposerComposition,
    }
    builder: PngBuilder,
    analyzer: PngAnalyzer,
    composer: PngComposer,
);
