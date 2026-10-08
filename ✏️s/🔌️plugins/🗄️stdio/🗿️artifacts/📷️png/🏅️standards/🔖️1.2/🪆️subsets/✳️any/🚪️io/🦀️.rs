//! 🚪️ IO stdio.png (1.2/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via `crate::register`), not per-leaf
//! register(). Relocated from `⚙️engine` verbatim (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 2: codecs live in `🚪️io/`).
//!
//! 🖼️ Native PNG syntax admits precise owned image records; canonical publication owns filtering, compression, framing and CRCs.
/// 🧩 Borrowed PNG chunk type tag and payload.
type PngChunkView<'a> = ([u8; 4], &'a [u8]);

use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};
use crate::{
    schema::snapshot::{PngAncillaryChunk, PngImage, PngBackground, PngChromaticities, PngColorType, PngPhysicalDims, PngRgb, PngSrgbIntent, PngTextChunk, PngTextKind, PngTimestamp, PngTransparency},
    PngSnapshot,
};

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

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_2::subsets::any::io::PngAnalyzer;
    use crate::PngSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

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
fn read_chunks<'a>(data: &'a [u8], control:&mut NativeDecodeControl<'_>) -> Result<Vec<PngChunkView<'a>>, PngReadError> {
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
    let samples = width as usize * samples_per_pixel(color_type);
    if bit_depth >= 8 { samples * (bit_depth as usize / 8) } else { samples.div_ceil(8 / bit_depth as usize) }
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
fn choose_filter_controlled(cur:&[u8],prev:Option<&[u8]>,bpp:usize,control:&mut NativeEncodeControl<'_>)->Result<(u8,Vec<u8>),ValueError> {
    let mut best_filter=0;let mut best_sum=u64::MAX;let mut best=Vec::new();
    for filter in 0..=4 {
        let mut output=control.allocate_vec(cur.len())?;let mut sum=0;control.begin_stage(cur.len())?;
        for x in 0..cur.len() {let a=if x>=bpp {cur[x-bpp]}else{0};let b=prev.map_or(0,|p|p[x]);let c=if x>=bpp {prev.map_or(0,|p|p[x-bpp])}else{0};let byte=match filter {0=>cur[x],1=>cur[x].wrapping_sub(a),2=>cur[x].wrapping_sub(b),3=>cur[x].wrapping_sub(((u16::from(a)+u16::from(b))/2)as u8),_=>cur[x].wrapping_sub(paeth(a,b,c))};output.push(byte);sum+=u64::from((byte as i8).unsigned_abs());control.step()?;}
        if sum<best_sum {best=output;best_sum=sum;best_filter=filter;}
    }
    Ok((best_filter,best))
}
fn defilter_row_controlled(filter:u8,filt:&[u8],prev:Option<&[u8]>,bpp:usize,control:&mut NativeDecodeControl<'_>)->Result<Vec<u8>,PngReadError> {
    if filter>4 {return Err("png: unsupported filter type".into());}let mut output=control.allocate_vec(filt.len())?;control.begin_stage(filt.len())?;
    for x in 0..filt.len() {let a=if x>=bpp {output[x-bpp]}else{0};let b=prev.map_or(0,|p|p[x]);let c=if x>=bpp {prev.map_or(0,|p|p[x-bpp])}else{0};let byte=match filter {0=>filt[x],1=>filt[x].wrapping_add(a),2=>filt[x].wrapping_add(b),3=>filt[x].wrapping_add(((u16::from(a)+u16::from(b))/2)as u8),_=>filt[x].wrapping_add(paeth(a,b,c))};output.push(byte);control.step()?;}Ok(output)
}

fn defilter_pass(raw: &[u8], mut pos: usize, height: u32, row_bytes: usize, bpp: usize, control:&mut NativeDecodeControl<'_>) -> Result<(Vec<Vec<u8>>, usize), PngReadError> {
    let mut rows:Vec<Vec<u8>>=control.allocate_vec(height as usize)?;
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
        let recon=defilter_row_controlled(ft,filt,rows.last().map(Vec::as_slice),bpp,control)?;
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
fn write_text_chunk(out: &mut Vec<u8>, tc: &PngTextChunk) -> Result<(), String> {
    match tc.kind {
        PngTextKind::Text => {
            let mut data = Vec::with_capacity(tc.keyword.len() + 1 + tc.value.len());
            data.extend_from_slice(&latin1_bytes(&tc.keyword));
            data.push(0);
            data.extend_from_slice(&latin1_bytes(&tc.value));
            write_chunk(out, b"tEXt", &data);
        }
        PngTextKind::ZText => {
            let mut data = Vec::with_capacity(tc.keyword.len() + 2);
            data.extend_from_slice(&latin1_bytes(&tc.keyword));
            data.push(0);
            data.push(0); // compression method 0 = zlib/deflate
            let compressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(&latin1_bytes(&tc.value))?;
            data.extend_from_slice(&compressed);
            write_chunk(out, b"zTXt", &data);
        }
        PngTextKind::IText => {
            let mut data = Vec::new();
            data.extend_from_slice(&latin1_bytes(&tc.keyword));
            data.push(0);
            data.push(if tc.compressed { 1 } else { 0 });
            data.push(0); // compression method 0 = zlib/deflate
            data.extend_from_slice(tc.language_tag.as_bytes());
            data.push(0);
            data.extend_from_slice(tc.translated_keyword.as_bytes());
            data.push(0);
            if tc.compressed {
                let compressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(tc.value.as_bytes())?;
                data.extend_from_slice(&compressed);
            } else {
                data.extend_from_slice(tc.value.as_bytes());
            }
            write_chunk(out, b"iTXt", &data);
        }
    }
    Ok(())
}

//#endregion AncillaryCodec

//#region ExactAuthority
pub fn empty_png_bytes()->Vec<u8> {encode_png(&PngSnapshot::default()).expect("authored empty PNG model")}

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
    decode_png_image(bytes)?;
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
    png_layout_bytes(&encode_png(snapshot)?)
}

pub fn decode_png(bytes: &[u8]) -> Result<PngSnapshot, String> {
    Ok(PngSnapshot { schema: crate::STDIO_PNG_DOCUMENT_SCHEMA.into(), image: decode_png_image(bytes)? })
}

pub fn encode_png(snapshot: &PngSnapshot) -> Result<Vec<u8>, String> {
    if snapshot.schema != crate::STDIO_PNG_DOCUMENT_SCHEMA {return Err("png: undeclared semantic schema".into());}
    encode_png_image(&snapshot.image)
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
    let bytes = encode_png(snapshot)?;
    Ok(PngPreview { width: layout.width, height: layout.height, bytes })
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
                    write_text_chunk(&mut out, tc)?;
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

fn latin1_text(bytes: &[u8]) -> String { bytes.iter().map(|byte| char::from(*byte)).collect() }
fn latin1_bytes(value: &str) -> Vec<u8> { value.chars().map(|character| character as u8).collect() }


struct PngEncodedChunks {chunks:Vec<([u8;4],Vec<u8>)>,file_size:usize,maximum_file:usize}
impl PngEncodedChunks {
    fn push(&mut self,kind:[u8;4],data:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),ValueError> {
        if data.len()>u32::MAX as usize {return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"PNG chunk exceeds native length"));}
        self.file_size=self.file_size.checked_add(12).and_then(|n|n.checked_add(data.len())).filter(|n|*n<=self.maximum_file).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PNG output exceeds file limit"))?;
        self.chunks.push((kind,control.copy_bytes(data)?));Ok(())
    }
}

fn png_latin1_encode(text:&str,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,ValueError> {let mut output=control.allocate_vec(text.len())?;control.begin_stage(text.len())?;for character in text.chars() {output.push(u8::try_from(character as u32).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"PNG text exceeds Latin-1"))?);control.advance(character.len_utf8())?;}Ok(output)}

fn png_text_native(text:&PngTextChunk,maximum_file:usize,control:&mut NativeEncodeControl<'_>)->Result<([u8;4],Vec<u8>),ValueError> {
    let keyword=png_latin1_encode(&text.keyword,control)?;
    let value=if text.kind==PngTextKind::IText {control.copy_bytes(text.value.as_bytes())?} else {png_latin1_encode(&text.value,control)?};
    let compressed=if text.compressed {semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::binary::snapshot::compress_zlib(&value,maximum_file,control)?} else {value};
    let extra=match text.kind {PngTextKind::Text=>1,PngTextKind::ZText=>2,PngTextKind::IText=>5+text.language_tag.len()+text.translated_keyword.len()};
    let size=keyword.len().checked_add(extra).and_then(|n|n.checked_add(compressed.len())).filter(|n|*n<=maximum_file).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PNG text exceeds file limit"))?;
    let mut data=control.allocate_vec(size)?;data.extend_from_slice(&keyword);data.push(0);
    let kind=match text.kind {PngTextKind::Text=>*b"tEXt",PngTextKind::ZText=>{data.push(0);*b"zTXt"},PngTextKind::IText=>{data.extend_from_slice(&[u8::from(text.compressed),0]);data.extend_from_slice(text.language_tag.as_bytes());data.push(0);data.extend_from_slice(text.translated_keyword.as_bytes());data.push(0);*b"iTXt"}};
    control.begin_stage(compressed.len())?;for piece in compressed.chunks(256) {data.extend_from_slice(piece);control.advance(piece.len())?;}Ok((kind,data))
}

fn encode_png_image(image:&PngImage)->Result<Vec<u8>,String> {let mut callback=|_|true;let mut control=NativeEncodeControl::new(1024*1024*1024,&mut callback);encode_png_image_controlled(image,512*1024*1024,&mut control).map_err(ValueError::into_message)}

fn validate_png_image_controlled(image:&PngImage,checkpoint:&mut dyn FnMut(bool,usize)->Result<(),ValueError>)->Result<(),ValueError> {
    let invalid=|message:&str|ValueError::new(ValueRefusalKind::InvalidValue,message);
    image.validate_header().map_err(|message|invalid(&message))?;
    let maximum=if image.bit_depth==16 {u16::MAX}else{(1u16<<image.bit_depth)-1};
    checkpoint(true,image.samples.len())?;
    for sample in &image.samples {if *sample>maximum||image.color_type==PngColorType::Palette&&usize::from(*sample)>=image.palette.as_ref().map_or(0,Vec::len) {return Err(invalid("png: native sample precision or palette identity differs from the owned profile"));}checkpoint(false,1)?;}
    for text in &image.text_chunks {
        checkpoint(true,1)?;
        if text.keyword.is_empty()||text.kind==PngTextKind::Text&&text.compressed||text.kind==PngTextKind::ZText&&!text.compressed {return Err(invalid("png: text metadata differs from its native text profile"));}
        checkpoint(false,1)?;
        let mut length=0;checkpoint(true,text.keyword.len())?;
        for character in text.keyword.chars() {length+=1;if length>79||character=='\0'||character as u32>255 {return Err(invalid("png: text metadata differs from its native text profile"));}checkpoint(false,character.len_utf8())?;}
        if text.kind!=PngTextKind::IText&&(!text.language_tag.is_empty()||!text.translated_keyword.is_empty()) {return Err(invalid("png: Latin-1 text metadata contains an international-only field"));}
        checkpoint(true,text.value.len())?;for character in text.value.chars() {if text.kind!=PngTextKind::IText&&character as u32>255 {return Err(invalid("png: Latin-1 text metadata contains an international-only field"));}checkpoint(false,character.len_utf8())?;}
        checkpoint(true,text.language_tag.len())?;for byte in text.language_tag.bytes() {if !byte.is_ascii() {return Err(invalid("png: international text language tag must be ASCII"));}checkpoint(false,1)?;}
    }
    let mut after=false;checkpoint(true,image.ancillary_chunks.len())?;
    for chunk in &image.ancillary_chunks {if !chunk.kind.iter().all(u8::is_ascii_alphabetic)||!chunk.kind[0].is_ascii_lowercase()||!chunk.kind[2].is_ascii_uppercase()||[*b"tRNS",*b"gAMA",*b"cHRM",*b"sRGB",*b"pHYs",*b"tIME",*b"bKGD",*b"tEXt",*b"zTXt",*b"iTXt"].contains(&chunk.kind)||after&&!chunk.after_raster {return Err(invalid("png: opaque ancillary metadata overlaps a typed field or has invalid placement"));}after|=chunk.after_raster;checkpoint(false,1)?;}
    Ok(())
}

fn encode_png_image_controlled(image:&PngImage,maximum_file:usize,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,ValueError> {
    validate_png_image_controlled(image,&mut |start,amount|if start {control.begin_stage(amount)}else{control.advance(amount)})?;
    let color=image.color_type.to_u8();let spp=image.color_type.samples_per_pixel();
    let ihdr=Ihdr {width:image.width,height:image.height,bit_depth:image.bit_depth,color_type:color,interlace:u8::from(image.interlace)};
    let bpp=bpp_bytes(&ihdr);let mut passes=Vec::with_capacity(7);
    if image.interlace {for(pass,&(sx,sy,dx,dy))in ADAM7.iter().enumerate() {let(width,height)=adam7_pass_dims(image.width,image.height,pass);passes.push((sx,sy,dx,dy,width,height));}}else{passes.push((0,0,1,1,image.width,image.height));}
    let raw_size=passes.iter().try_fold(0usize,|total,&(_,_,_,_,width,height)| {if width==0||height==0 {return Ok(total);}total.checked_add((packed_row_bytes(width,color,image.bit_depth)+1).checked_mul(height as usize).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PNG raw extent overflow"))?).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PNG raw extent overflow"))})?;
    let mut raw=control.allocate_vec(raw_size)?;
    for(sx,sy,dx,dy,width,height)in passes {
        if width==0||height==0 {continue;}let row_bytes=packed_row_bytes(width,color,image.bit_depth);let mut previous=None;
        for y in 0..height {
            let mut row=control.allocate_vec(row_bytes)?;row.resize(row_bytes,0);control.begin_stage(width as usize*spp)?;
            for x in 0..width {let source=(((sy+y*dy)as usize*image.width as usize)+(sx+x*dx)as usize)*spp;for channel in 0..spp {write_native_sample(&mut row,x as usize*spp+channel,image.bit_depth,image.samples[source+channel]);control.step()?;}}
            let(filter,filtered)=choose_filter_controlled(&row,previous.as_deref(),bpp,control)?;raw.push(filter);control.begin_stage(filtered.len())?;for piece in filtered.chunks(256) {raw.extend_from_slice(piece);control.advance(piece.len())?;}previous=Some(row);
        }
    }
    let compressed=semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::binary::snapshot::compress_zlib(&raw,maximum_file,control)?;
    let count=14usize.checked_add(image.text_chunks.len()).and_then(|n|n.checked_add(image.ancillary_chunks.len())).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PNG chunk count overflow"))?;
    let mut output=PngEncodedChunks {chunks:control.allocate_vec(count)?,file_size:8,maximum_file};
    let mut header=[0;13];header[..4].copy_from_slice(&image.width.to_be_bytes());header[4..8].copy_from_slice(&image.height.to_be_bytes());header[8..].copy_from_slice(&[image.bit_depth,color,0,0,u8::from(image.interlace)]);output.push(*b"IHDR",&header,control)?;
    if let Some(gamma)=image.gamma {output.push(*b"gAMA",&gamma.to_be_bytes(),control)?;}
    if let Some(c)=image.chromaticities {let mut data=[0;32];for(index,field)in[c.white_x,c.white_y,c.red_x,c.red_y,c.green_x,c.green_y,c.blue_x,c.blue_y].into_iter().enumerate() {data[index*4..index*4+4].copy_from_slice(&field.to_be_bytes());}output.push(*b"cHRM",&data,control)?;}
    if let Some(intent)=image.srgb {output.push(*b"sRGB",&[intent.to_u8()],control)?;}
    if let Some(palette)=&image.palette {let mut data=control.allocate_vec(palette.len()*3)?;control.begin_stage(palette.len())?;for entry in palette {data.extend_from_slice(&[entry.r,entry.g,entry.b]);control.step()?;}output.push(*b"PLTE",&data,control)?;}
    if let Some(transparency)=&image.transparency {match transparency {PngTransparency::Indexed {alpha}=>output.push(*b"tRNS",alpha,control)?,PngTransparency::Grayscale {gray}=>output.push(*b"tRNS",&gray.to_be_bytes(),control)?,PngTransparency::Rgb {r,g,b}=>{let mut data=[0;6];data[..2].copy_from_slice(&r.to_be_bytes());data[2..4].copy_from_slice(&g.to_be_bytes());data[4..].copy_from_slice(&b.to_be_bytes());output.push(*b"tRNS",&data,control)?;}}}
    if let Some(background)=&image.background {let data=encode_bkgd(background);control.charge(data.capacity())?;output.push(*b"bKGD",&data,control)?;}
    if let Some(p)=image.physical_dims {let mut data=[0;9];data[..4].copy_from_slice(&p.ppu_x.to_be_bytes());data[4..8].copy_from_slice(&p.ppu_y.to_be_bytes());data[8]=u8::from(p.unit_is_meter);output.push(*b"pHYs",&data,control)?;}
    if let Some(t)=image.timestamp {let mut data=[0;7];data[..2].copy_from_slice(&t.year.to_be_bytes());data[2..].copy_from_slice(&[t.month,t.day,t.hour,t.minute,t.second]);output.push(*b"tIME",&data,control)?;}
    for text in &image.text_chunks {let(kind,data)=png_text_native(text,maximum_file,control)?;output.push(kind,&data,control)?;}
    for chunk in image.ancillary_chunks.iter().filter(|chunk|!chunk.after_raster) {output.push(chunk.kind,&chunk.data,control)?;}
    output.push(*b"IDAT",&compressed,control)?;
    for chunk in image.ancillary_chunks.iter().filter(|chunk|chunk.after_raster) {output.push(chunk.kind,&chunk.data,control)?;}
    output.push(*b"IEND",&[],control)?;
    let mut bytes=control.allocate_vec(output.file_size)?;bytes.extend_from_slice(&PNG_SIGNATURE);
    for(kind,data)in output.chunks {bytes.extend_from_slice(&(data.len()as u32).to_be_bytes());bytes.extend_from_slice(&kind);let mut crc=0xffff_ffffu32;for byte in kind {crc^=u32::from(byte);for _ in 0..8 {crc=if crc&1!=0 {(crc>>1)^0xedb88320}else{crc>>1};}}control.begin_stage(data.len())?;for piece in data.chunks(256) {bytes.extend_from_slice(piece);for byte in piece {crc^=u32::from(*byte);for _ in 0..8 {crc=if crc&1!=0 {(crc>>1)^0xedb88320}else{crc>>1};}}control.advance(piece.len())?;}bytes.extend_from_slice(&(!crc).to_be_bytes());}
    Ok(bytes)
}

pub fn project_png(bytes: &[u8]) -> Result<PngProjection, String> {
    let image = decode_png_image(bytes)?;
    let pixels = crate::schema::operations::png_rgba8_preview(&image)?;
    let mut chunk_order = vec![PngChunkMarker::Ihdr];
    if image.gamma.is_some() { chunk_order.push(PngChunkMarker::Gama); }
    if image.chromaticities.is_some() { chunk_order.push(PngChunkMarker::Chrm); }
    if image.srgb.is_some() { chunk_order.push(PngChunkMarker::Srgb); }
    if image.palette.is_some() { chunk_order.push(PngChunkMarker::Plte); }
    if image.transparency.is_some() { chunk_order.push(PngChunkMarker::Trns); }
    if image.background.is_some() { chunk_order.push(PngChunkMarker::Bkgd); }
    if image.physical_dims.is_some() { chunk_order.push(PngChunkMarker::Phys); }
    if image.timestamp.is_some() { chunk_order.push(PngChunkMarker::Time); }
    chunk_order.extend((0..image.text_chunks.len()).map(|index|PngChunkMarker::Text { index }));
    let mut unknown_chunks=Vec::new();
    for chunk in image.ancillary_chunks.iter().filter(|chunk|!chunk.after_raster) { chunk_order.push(PngChunkMarker::Unknown { index:unknown_chunks.len() }); unknown_chunks.push(PngChunk { kind:chunk.kind,data:chunk.data.clone() }); }
    chunk_order.push(PngChunkMarker::Idat);
    for chunk in image.ancillary_chunks.iter().filter(|chunk|chunk.after_raster) { chunk_order.push(PngChunkMarker::Unknown { index:unknown_chunks.len() }); unknown_chunks.push(PngChunk { kind:chunk.kind,data:chunk.data.clone() }); }
    chunk_order.push(PngChunkMarker::Iend);
    Ok(PngProjection { width:image.width,height:image.height,bit_depth:image.bit_depth,color_type:image.color_type,interlace:image.interlace,plte:image.palette,trns:image.transparency,gama:image.gamma,chrm:image.chromaticities,srgb:image.srgb,phys:image.physical_dims,time:image.timestamp,bkgd:image.background,text_chunks:image.text_chunks,pixels,chunk_order,unknown_chunks })
}

fn decode_png_image(data:&[u8])->Result<PngImage,String> {let mut progress=|_|true;let mut control=NativeDecodeControl::new(512*1024*1024,&mut progress);decode_png_image_controlled(data,&mut control).map_err(|error|error.into_value().into_message())}

enum PngReadError {Format(String),Refusal(ValueError)}
impl From<String> for PngReadError {fn from(value:String)->Self {Self::Format(value)}}
impl From<&str> for PngReadError {fn from(value:&str)->Self {Self::Format(value.into())}}
impl From<ValueError> for PngReadError {fn from(value:ValueError)->Self {Self::Refusal(value)}}
impl PngReadError {fn into_value(self)->ValueError {match self {Self::Format(message)=>ValueError::new(ValueRefusalKind::InvalidValue,message),Self::Refusal(error)=>error}}}

fn decode_png_image_controlled(data: &[u8],control:&mut NativeDecodeControl<'_>) -> Result<PngImage, PngReadError> {
    let chunks = read_chunks(data,control)?;
    validate_png_structure(&chunks)?;
    let mut ihdr: Option<Ihdr> = None;
    let compressed_size=chunks.iter().filter(|(kind,_)|*kind==*b"IDAT").try_fold(0usize,|n,(_,data)|n.checked_add(data.len()).ok_or("png: IDAT size overflow"))?;
    let mut idat=control.allocate_vec(compressed_size)?;
    let mut seen_idat = false;

    let mut plte_out: Option<Vec<PngRgb>> = None;
    let mut trns_out: Option<PngTransparency> = None;
    let mut gama_out: Option<u32> = None;
    let mut chrm_out: Option<PngChromaticities> = None;
    let mut srgb_out: Option<PngSrgbIntent> = None;
    let mut phys_out: Option<PngPhysicalDims> = None;
    let mut time_out: Option<PngTimestamp> = None;
    let mut bkgd_out: Option<PngBackground> = None;
    let mut text_chunks: Vec<PngTextChunk> = control.allocate_vec(chunks.len())?;
    let mut ancillary_chunks: Vec<PngAncillaryChunk> = control.allocate_vec(chunks.len())?;

    for &(ty, chunk) in &chunks {
        if ty == *b"IHDR" {
            ihdr = Some(parse_ihdr(chunk).map_err(ValueError::into_message)?);

        } else if ty == *b"PLTE" {
            if chunk.len() % 3 != 0 {
                return Err("png PLTE: length not a multiple of 3".into());
            }
            let mut entries=control.allocate_vec(chunk.len()/3)?;control.begin_stage(chunk.len()/3)?;for c in chunk.as_chunks::<3>().0 {entries.push(PngRgb {r:c[0],g:c[1],b:c[2]});control.step()?;}
            plte_out=Some(entries);

        } else if ty == *b"tRNS" {
            let color_type = ihdr.as_ref().ok_or("png: tRNS before IHDR")?.color_type;
            match color_type {
                0 => {
                    if chunk.len() != 2 {
                        return Err("png tRNS: expected 2 bytes for grayscale".into());
                    }
                    let g = u16::from_be_bytes([chunk[0], chunk[1]]);
                    trns_out = Some(PngTransparency::Grayscale { gray: g });
                }
                2 => {
                    if chunk.len() != 6 {
                        return Err("png tRNS: expected 6 bytes for truecolor".into());
                    }
                    let r = u16::from_be_bytes([chunk[0], chunk[1]]);
                    let g = u16::from_be_bytes([chunk[2], chunk[3]]);
                    let b = u16::from_be_bytes([chunk[4], chunk[5]]);
                    trns_out = Some(PngTransparency::Rgb { r, g, b });
                }
                3 => {
                    trns_out = Some(PngTransparency::Indexed { alpha: control.copy_bytes(chunk)? });
                }
                _ => {} // spec: tRNS shall not appear for 4/6 (already carry alpha) — ignore rather than fail
            }

        } else if ty == *b"gAMA" {
            if chunk.len() != 4 {
                return Err("png gAMA: expected 4 bytes".into());
            }
            gama_out = Some(u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));

        } else if ty == *b"cHRM" {
            if chunk.len() != 32 {
                return Err("png cHRM: expected 32 bytes".into());
            }
            let v = |i: usize| u32::from_be_bytes([chunk[i], chunk[i + 1], chunk[i + 2], chunk[i + 3]]);
            chrm_out = Some(PngChromaticities { white_x: v(0), white_y: v(4), red_x: v(8), red_y: v(12), green_x: v(16), green_y: v(20), blue_x: v(24), blue_y: v(28) });

        } else if ty == *b"sRGB" {
            if chunk.len() != 1 {
                return Err("png sRGB: expected 1 byte".into());
            }
            srgb_out = Some(PngSrgbIntent::from_u8(chunk[0])?);

        } else if ty == *b"pHYs" {
            if chunk.len() != 9 {
                return Err("png pHYs: expected 9 bytes".into());
            }
            phys_out = Some(PngPhysicalDims { ppu_x: u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]), ppu_y: u32::from_be_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]), unit_is_meter: chunk[8] == 1 });

        } else if ty == *b"tIME" {
            if chunk.len() != 7 {
                return Err("png tIME: expected 7 bytes".into());
            }
            time_out = Some(PngTimestamp { year: u16::from_be_bytes([chunk[0], chunk[1]]), month: chunk[2], day: chunk[3], hour: chunk[4], minute: chunk[5], second: chunk[6] });

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

        } else if ty == *b"tEXt" {
            let nul = chunk.iter().position(|&b| b == 0).ok_or("png tEXt: missing NUL after keyword")?;
            control.charge(chunk.len().saturating_mul(2))?;
            let keyword = latin1_text(&chunk[..nul]);
            let value = latin1_text(&chunk[nul + 1..]);
            let index = text_chunks.len();
            text_chunks.push(PngTextChunk { keyword, value, compressed: false, kind: PngTextKind::Text, language_tag: String::new(), translated_keyword: String::new() });

        } else if ty == *b"zTXt" {
            let nul = chunk.iter().position(|&b| b == 0).ok_or("png zTXt: missing NUL after keyword")?;
            control.charge(chunk.len().saturating_mul(2))?;
            let keyword = latin1_text(&chunk[..nul]);
            if chunk.len() < nul + 2 {
                return Err("png zTXt: missing compression method".into());
            }
            if chunk[nul + 1] != 0 {
                return Err("png zTXt: compression method must be zero".into());
            }
            let value_bytes=semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::binary::snapshot::decompress_zlib(&chunk[nul+2..],control.maximum_bytes().saturating_sub(control.owned_bytes()),control)?;
            let value = latin1_text(&value_bytes);
            let index = text_chunks.len();
            text_chunks.push(PngTextChunk { keyword, value, compressed: true, kind: PngTextKind::ZText, language_tag: String::new(), translated_keyword: String::new() });

        } else if ty == *b"iTXt" {
            let mut pos = 0usize;
            let nul1 = chunk[pos..].iter().position(|&b| b == 0).ok_or("png iTXt: missing NUL after keyword")?;
            control.charge(chunk.len().saturating_mul(2))?;
            let keyword = latin1_text(&chunk[pos..pos + nul1]);
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
            let translated_keyword=control.copy_text(std::str::from_utf8(&chunk[pos..pos+nul3]).map_err(|_|"png iTXt: translated keyword must be UTF-8")?)?;
            pos += nul3 + 1;
            let rest = &chunk[pos..];
            let value_bytes = if compressed_flag {
                let decompressed=semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::binary::snapshot::decompress_zlib(rest,control.maximum_bytes().saturating_sub(control.owned_bytes()),control)?;
                decompressed
            } else {
                control.copy_bytes(rest)?
            };
            let value = String::from_utf8(value_bytes).map_err(|_| "png iTXt: text must be UTF-8")?;
            let index = text_chunks.len();
            text_chunks.push(PngTextChunk { keyword, value, compressed: compressed_flag, kind: PngTextKind::IText, language_tag, translated_keyword });

        } else if ty == *b"IDAT" {
            idat.extend_from_slice(chunk);
            seen_idat = true;
        } else if ty == *b"IEND" {

        } else if ty[0].is_ascii_uppercase() {
            return Err(format!("png: unsupported critical chunk {}", String::from_utf8_lossy(&ty)).into());
        } else {
            // 🗃️ Ancillary chunk the codec doesn't specifically model — typed raw-retention,
            // verbatim, in position (the recipe's "nothing real on disk silently dropped" rule).
            ancillary_chunks.push(PngAncillaryChunk { kind: ty, data: control.copy_bytes(chunk)?, after_raster: seen_idat });
        }
    }

    let ihdr = ihdr.ok_or("png: missing IHDR")?;
    if !seen_idat {
        return Err("png: missing IDAT".into());
    }
    if ihdr.color_type == 3 && plte_out.as_ref().is_none_or(Vec::is_empty) {
        return Err("png: color type 3 requires PLTE".into());
    }

    let spp = samples_per_pixel(ihdr.color_type);
    let count = (ihdr.width as usize).checked_mul(ihdr.height as usize).and_then(|n|n.checked_mul(spp)).ok_or("png: native sample count overflow")?;
    if count > 256 * 1024 * 1024 { return Err("png: native sample admission exceeds 512 MiB".into()); }
    let expected=if ihdr.interlace==0 {(packed_row_bytes(ihdr.width,ihdr.color_type,ihdr.bit_depth)+1).checked_mul(ihdr.height as usize).ok_or("png: raw raster extent overflow")?} else {let mut count=0usize;for pass in 0..7 {let(w,h)=adam7_pass_dims(ihdr.width,ihdr.height,pass);if w!=0&&h!=0 {count=count.checked_add((packed_row_bytes(w,ihdr.color_type,ihdr.bit_depth)+1).checked_mul(h as usize).ok_or("png: Adam7 raster extent overflow")?).ok_or("png: Adam7 raster extent overflow")?;}}count};
    let raw=semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::binary::snapshot::decompress_zlib(&idat,expected,control)?;
    let bpp = bpp_bytes(&ihdr);
    let mut native_samples=control.allocate_vec(count)?;control.begin_stage(count)?;while native_samples.len()<count {let added=(count-native_samples.len()).min(256);native_samples.resize(native_samples.len()+added,0);control.advance(added)?;}

    let mut put_row = |row: &[u8], row_width: usize, base_x: u32, base_y: u32, step_x: u32,control:&mut NativeDecodeControl<'_>| -> Result<(), PngReadError> {
        control.begin_stage(row_width*spp)?;
        for i in 0..row_width {
            let x = base_x + i as u32 * step_x;
            let idx = (base_y as usize * ihdr.width as usize + x as usize) * spp;
            for channel in 0..spp {let ordinal=i*spp+channel;native_samples[idx+channel]=match ihdr.bit_depth {16=>u16::from_be_bytes([row[ordinal*2],row[ordinal*2+1]]),8=>u16::from(row[ordinal]),depth=>{let bit=ordinal*depth as usize;u16::from((row[bit/8]>>(8-depth as usize-bit%8))&((1u8<<depth)-1))}};control.step()?;}
        }
        Ok(())
    };

    if ihdr.interlace == 0 {
        let row_bytes = packed_row_bytes(ihdr.width, ihdr.color_type, ihdr.bit_depth);
        let (rows, consumed) = defilter_pass(&raw, 0, ihdr.height, row_bytes, bpp,control)?;
        if consumed != raw.len() { return Err("png: bytes follow the final native raster row".into()); }
        for (y, row) in rows.iter().enumerate() {
            put_row(row, ihdr.width as usize, 0, y as u32, 1,control)?;
        }
    } else {
        let mut pos = 0usize;
        for (pass, &(sx, sy, stx, sty)) in ADAM7.iter().enumerate() {
            let (pw, ph) = adam7_pass_dims(ihdr.width, ihdr.height, pass);
            if pw == 0 || ph == 0 {
                continue;
            }
            let row_bytes = packed_row_bytes(pw, ihdr.color_type, ihdr.bit_depth);
            let (rows, new_pos) = defilter_pass(&raw, pos, ph, row_bytes, bpp,control)?;
            pos = new_pos;
            for (j, row) in rows.iter().enumerate() {
                put_row(row, pw as usize, sx, sy + j as u32 * sty, stx,control)?;
            }
        }
        if pos != raw.len() { return Err("png: bytes follow the final Adam7 raster row".into()); }
    }

    let image = PngImage {
        width: ihdr.width,
        height: ihdr.height,
        bit_depth: ihdr.bit_depth,
        color_type: PngColorType::from_u8(ihdr.color_type)?,
        interlace: ihdr.interlace == 1,
        palette: plte_out,
        transparency: trns_out,
        gamma: gama_out,
        chromaticities: chrm_out,
        srgb: srgb_out,
        physical_dims: phys_out,
        timestamp: time_out,
        background: bkgd_out,
        text_chunks,
        samples: native_samples,
        ancillary_chunks,
    };
    validate_png_image_controlled(&image,&mut |start,amount|if start {control.begin_stage(amount)}else{control.advance(amount)})?;
    Ok(image)
}
//#endregion Codec

//#region 🚪️DerivedIoRegistry
/// 🚪️ Relocated verbatim from `⚙️engine` (rule 3: `io_registry`/`ComposerEntry` live in `🚪️io/`).
pub mod io_registry {
    use crate::standards::v1_2::subsets::any::io::PngComposer as PngRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
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
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
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
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

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

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            const SIG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 8 && bytes[0..8] == SIG {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
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
                        return semio_framework_plugin::io::Confidence::Low;
                    }
                    let mut decoded = [0u8; 8];
                    for (i, byte) in decoded.iter_mut().enumerate() {
                        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                            Ok(b) => *byte = b,
                            Err(_) => return semio_framework_plugin::io::Confidence::Low,
                        }
                    }
                    if decoded == SIG {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PngParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <PngSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <PngSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
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

#[cfg(test)]
#[path="🧪️tests/🧬️owned-native-oracle/🦀️.rs"]
mod owned_native_oracle_tests;
