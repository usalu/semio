//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by an
//! independent hand-rolled TIFF 6.0 baseline IFD-chain codec (this module's own [`read_tiff`]/
//! [`write_tiff`]), so the subject's own mutation has a genuinely independent result to be compared
//! against instead of being checked against its own reading.
//!
//! `image` 0.25 (this subset's registered reference package, tiff feature on) only exposes a
//! SINGLE-IFD raster encoder/decoder publicly (`image::codecs::tiff::{TiffEncoder, TiffDecoder}` —
//! confirmed by reading the vendored crate source: `TiffEncoder::write_image` always emits exactly
//! one IFD with auto-computed baseline tags, and `TiffDecoder::new` reads only the first IFD). It
//! has no public surface for multi-IFD chains or arbitrary tag get/set — the exact structural
//! vocabulary `InsertIfd`/`RemoveIfd`/`SetTag`/`RemoveTag`/`SetByteOrder` need. Same shape as the
//! `obj`/`tobj` precedent (`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🧊️mesh/🦀️component.rs`: "OBJ has no
//! reference WRITER... so the oracle writes the grammar directly"): this module hand-writes the IFD
//! chain directly, independent of (and never importing) this subset's own `🚪️io::{decode_tiff,
//! encode_tiff}` — the code under test.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `raster` module rather than by copying it.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️JsonHelpers
/// 🔎️ Small local accessors `protocol::Json` doesn't itself provide (it only exposes top-level
/// object `str`/`array`) — needed for the nested `params` payloads mutation specs carry.
fn j_get<'a>(v: &'a Json, key: &str) -> Option<&'a Json> {
    v.get(key)
}
fn j_num(v: &Json) -> Option<f64> {
    match v {
        Json::Number(n) => Some(*n),
        _ => None,
    }
}
fn j_str(v: &Json) -> Option<&str> {
    match v {
        Json::String(s) => Some(s.as_str()),
        _ => None,
    }
}
fn j_arr(v: &Json) -> Option<&Vec<Json>> {
    match v {
        Json::Array(items) => Some(items),
        _ => None,
    }
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    let s = s.trim();
    if s.len() % 2 != 0 {
        return Err("hex: odd length".to_string());
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| format!("hex: {e}"))).collect()
}
//#endregion 🔖️JsonHelpers

//#region 🔖️IndependentCodec
/// 🧭️ Endianness — read/written locally, never imported from the subject's own `Endian`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Endian {
    Little,
    Big,
}
impl Endian {
    fn u16(self, b: &[u8]) -> u16 {
        match self {
            Endian::Little => u16::from_le_bytes([b[0], b[1]]),
            Endian::Big => u16::from_be_bytes([b[0], b[1]]),
        }
    }
    fn u32(self, b: &[u8]) -> u32 {
        match self {
            Endian::Little => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            Endian::Big => u32::from_be_bytes([b[0], b[1], b[2], b[3]]),
        }
    }
}
fn read_u16(data: &[u8], pos: usize, e: Endian) -> Result<u16, String> {
    data.get(pos..pos + 2).map(|s| e.u16(s)).ok_or_else(|| "tiff oracle: truncated (u16)".to_string())
}
fn read_u32(data: &[u8], pos: usize, e: Endian) -> Result<u32, String> {
    data.get(pos..pos + 4).map(|s| e.u32(s)).ok_or_else(|| "tiff oracle: truncated (u32)".to_string())
}
fn write_u16(out: &mut Vec<u8>, v: u16, little: bool) {
    out.extend_from_slice(&if little { v.to_le_bytes() } else { v.to_be_bytes() });
}
fn write_u32(out: &mut Vec<u8>, v: u32, little: bool) {
    out.extend_from_slice(&if little { v.to_le_bytes() } else { v.to_be_bytes() });
}

/// 📦️ TIFF6 §2 Table 2 field types, by their OWN numeric code (1-12) — the standard's own
/// numbering, not a codec-specific spelling, so mutation `params` use the same numbers directly.
#[derive(Clone, Debug, PartialEq)]
enum OracleValue {
    Byte(Vec<u8>),
    Ascii(String),
    Short(Vec<u16>),
    Long(Vec<u32>),
    Rational(Vec<(u32, u32)>),
    SByte(Vec<i8>),
    Undefined(Vec<u8>),
    SShort(Vec<i16>),
    SLong(Vec<i32>),
    SRational(Vec<(i32, i32)>),
    Float(Vec<f32>),
    Double(Vec<f64>),
}
impl OracleValue {
    fn type_code(&self) -> u16 {
        match self {
            Self::Byte(_) => 1,
            Self::Ascii(_) => 2,
            Self::Short(_) => 3,
            Self::Long(_) => 4,
            Self::Rational(_) => 5,
            Self::SByte(_) => 6,
            Self::Undefined(_) => 7,
            Self::SShort(_) => 8,
            Self::SLong(_) => 9,
            Self::SRational(_) => 10,
            Self::Float(_) => 11,
            Self::Double(_) => 12,
        }
    }
    fn element_size(type_code: u16) -> usize {
        match type_code {
            1 | 2 | 6 | 7 => 1,
            3 | 8 => 2,
            4 | 9 | 11 => 4,
            5 | 10 | 12 => 8,
            _ => 1,
        }
    }
    fn count(&self) -> u32 {
        match self {
            Self::Byte(v) | Self::Undefined(v) => v.len() as u32,
            Self::Ascii(s) => s.len() as u32 + 1,
            Self::Short(v) => v.len() as u32,
            Self::Long(v) => v.len() as u32,
            Self::Rational(v) => v.len() as u32,
            Self::SByte(v) => v.len() as u32,
            Self::SShort(v) => v.len() as u32,
            Self::SLong(v) => v.len() as u32,
            Self::SRational(v) => v.len() as u32,
            Self::Float(v) => v.len() as u32,
            Self::Double(v) => v.len() as u32,
        }
    }
    fn first_u32(&self) -> Option<u32> {
        match self {
            Self::Byte(v) => v.first().map(|&x| x as u32),
            Self::Short(v) => v.first().map(|&x| x as u32),
            Self::Long(v) => v.first().copied(),
            _ => None,
        }
    }
    fn bytes(&self, little: bool) -> Vec<u8> {
        let mut out = Vec::new();
        match self {
            Self::Byte(v) | Self::Undefined(v) => out.extend_from_slice(v),
            Self::Ascii(s) => {
                out.extend_from_slice(s.as_bytes());
                out.push(0);
            }
            Self::Short(v) => v.iter().for_each(|&x| write_u16(&mut out, x, little)),
            Self::Long(v) => v.iter().for_each(|&x| write_u32(&mut out, x, little)),
            Self::Rational(v) => v.iter().for_each(|&(n, d)| {
                write_u32(&mut out, n, little);
                write_u32(&mut out, d, little);
            }),
            Self::SByte(v) => out.extend(v.iter().map(|&x| x as u8)),
            Self::SShort(v) => v.iter().for_each(|&x| write_u16(&mut out, x as u16, little)),
            Self::SLong(v) => v.iter().for_each(|&x| write_u32(&mut out, x as u32, little)),
            Self::SRational(v) => v.iter().for_each(|&(n, d)| {
                write_u32(&mut out, n as u32, little);
                write_u32(&mut out, d as u32, little);
            }),
            Self::Float(v) => v.iter().for_each(|&x| write_u32(&mut out, x.to_bits(), little)),
            Self::Double(v) => v.iter().for_each(|&x| out.extend_from_slice(&if little { x.to_bits().to_le_bytes() } else { x.to_bits().to_be_bytes() })),
        }
        out
    }
    /// 🔁️ Projection shape — SAME shape [`from_json`] parses, so a mutation's `params` and this
    /// module's own projection output are visually symmetric.
    fn to_json(&self) -> Json {
        match self {
            Self::Byte(v) => Json::Array(v.iter().map(|&b| Json::Number(b as f64)).collect()),
            Self::Ascii(s) => Json::Array(vec![Json::String(s.clone())]),
            Self::Short(v) => Json::Array(v.iter().map(|&x| Json::Number(x as f64)).collect()),
            Self::Long(v) => Json::Array(v.iter().map(|&x| Json::Number(x as f64)).collect()),
            Self::Rational(v) => Json::Array(v.iter().map(|&(n, d)| Json::Array(vec![Json::Number(n as f64), Json::Number(d as f64)])).collect()),
            Self::SByte(v) => Json::Array(v.iter().map(|&x| Json::Number(x as f64)).collect()),
            Self::Undefined(v) => Json::Array(v.iter().map(|&b| Json::Number(b as f64)).collect()),
            Self::SShort(v) => Json::Array(v.iter().map(|&x| Json::Number(x as f64)).collect()),
            Self::SLong(v) => Json::Array(v.iter().map(|&x| Json::Number(x as f64)).collect()),
            Self::SRational(v) => Json::Array(v.iter().map(|&(n, d)| Json::Array(vec![Json::Number(n as f64), Json::Number(d as f64)])).collect()),
            Self::Float(v) => Json::Array(v.iter().map(|&x| Json::Number(x as f64)).collect()),
            Self::Double(v) => Json::Array(v.iter().map(|&x| Json::Number(x as f64)).collect()),
        }
    }
    /// 🔁️ Inverse of [`to_json`] given the field's type code — parses a mutation spec's
    /// `values` array for `set-tag`/`insert-ifd`/`set-snapshot` params.
    fn from_json(type_code: u16, values: &Json) -> Result<OracleValue, String> {
        let items = j_arr(values).ok_or("tiff oracle: tag values must be a JSON array")?;
        let nums = || -> Result<Vec<f64>, String> { items.iter().map(|v| j_num(v).ok_or_else(|| "tiff oracle: expected a number in tag values".to_string())).collect() };
        Ok(match type_code {
            1 => OracleValue::Byte(nums()?.into_iter().map(|n| n as u8).collect()),
            2 => OracleValue::Ascii(items.first().and_then(j_str).ok_or("tiff oracle: ascii tag values must be [\"text\"]")?.to_string()),
            3 => OracleValue::Short(nums()?.into_iter().map(|n| n as u16).collect()),
            4 => OracleValue::Long(nums()?.into_iter().map(|n| n as u32).collect()),
            5 => OracleValue::Rational(
                items
                    .iter()
                    .map(|pair| {
                        let p = j_arr(pair).ok_or("tiff oracle: rational value must be [num,den]")?;
                        Ok((j_num(&p[0]).unwrap_or(0.0) as u32, j_num(&p[1]).unwrap_or(1.0) as u32))
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            ),
            6 => OracleValue::SByte(nums()?.into_iter().map(|n| n as i8).collect()),
            7 => OracleValue::Undefined(nums()?.into_iter().map(|n| n as u8).collect()),
            8 => OracleValue::SShort(nums()?.into_iter().map(|n| n as i16).collect()),
            9 => OracleValue::SLong(nums()?.into_iter().map(|n| n as i32).collect()),
            10 => OracleValue::SRational(
                items
                    .iter()
                    .map(|pair| {
                        let p = j_arr(pair).ok_or("tiff oracle: srational value must be [num,den]")?;
                        Ok((j_num(&p[0]).unwrap_or(0.0) as i32, j_num(&p[1]).unwrap_or(1.0) as i32))
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            ),
            11 => OracleValue::Float(nums()?.into_iter().map(|n| n as f32).collect()),
            12 => OracleValue::Double(nums()?),
            other => return Err(format!("tiff oracle: unrecognized field type code {other} (TIFF 6.0 core table is 1-12)")),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct OracleTag {
    tag: u16,
    value: OracleValue,
}

const TAG_IMAGE_WIDTH: u16 = 256;
const TAG_IMAGE_LENGTH: u16 = 257;
const TAG_BITS_PER_SAMPLE: u16 = 258;
const TAG_COMPRESSION: u16 = 259;
const TAG_PHOTOMETRIC: u16 = 262;
const TAG_STRIP_OFFSETS: u16 = 273;
const TAG_SAMPLES_PER_PIXEL: u16 = 277;
const TAG_ROWS_PER_STRIP: u16 = 278;
const TAG_STRIP_BYTE_COUNTS: u16 = 279;

/// 🗂️ One IFD: its non-layout-dependent tags plus (if it carries a baseline raster) the raw strip
/// payload — `StripOffsets`/`StripByteCounts` are recomputed fresh at [`write_tiff`] time, exactly
/// like the subject's own encoder recomputes them (a conformant writer must: strip placement is
/// layout-dependent, never semantic content).
#[derive(Clone, Debug, Default)]
struct OracleIfd {
    entries: Vec<OracleTag>,
    strip: Option<Vec<u8>>,
}
impl OracleIfd {
    fn get(&self, tag: u16) -> Option<&OracleTag> {
        self.entries.iter().find(|t| t.tag == tag)
    }
    fn set(&mut self, tag: u16, value: OracleValue) {
        match self.entries.iter_mut().find(|t| t.tag == tag) {
            Some(existing) => existing.value = value,
            None => self.entries.push(OracleTag { tag, value }),
        }
        self.entries.sort_by_key(|t| t.tag);
    }
}

struct OracleDoc {
    little_endian: bool,
    ifds: Vec<OracleIfd>,
}

/// 📖️ Reads one entry's typed value, resolving TIFF6 §2's inline-vs-offset rule generically.
fn read_tag_value(data: &[u8], type_code: u16, count: u32, value_field: &[u8; 4], e: Endian) -> Result<OracleValue, String> {
    let elem = OracleValue::element_size(type_code);
    let n = count as usize;
    let total = elem * n;
    let owned;
    let src: &[u8] = if total <= 4 {
        &value_field[..total]
    } else {
        let off = e.u32(value_field) as usize;
        owned = data.get(off..off + total).ok_or("tiff oracle: tag value offset out of range")?;
        owned
    };
    Ok(match type_code {
        1 => OracleValue::Byte(src.to_vec()),
        2 => OracleValue::Ascii(String::from_utf8_lossy(src).trim_end_matches('\u{0}').to_string()),
        3 => OracleValue::Short((0..n).map(|i| e.u16(&src[i * 2..i * 2 + 2])).collect()),
        4 => OracleValue::Long((0..n).map(|i| e.u32(&src[i * 4..i * 4 + 4])).collect()),
        5 => OracleValue::Rational((0..n).map(|i| (e.u32(&src[i * 8..i * 8 + 4]), e.u32(&src[i * 8 + 4..i * 8 + 8]))).collect()),
        6 => OracleValue::SByte(src.iter().map(|&b| b as i8).collect()),
        7 => OracleValue::Undefined(src.to_vec()),
        8 => OracleValue::SShort((0..n).map(|i| e.u16(&src[i * 2..i * 2 + 2]) as i16).collect()),
        9 => OracleValue::SLong((0..n).map(|i| e.u32(&src[i * 4..i * 4 + 4]) as i32).collect()),
        10 => OracleValue::SRational((0..n).map(|i| (e.u32(&src[i * 8..i * 8 + 4]) as i32, e.u32(&src[i * 8 + 4..i * 8 + 8]) as i32)).collect()),
        11 => OracleValue::Float((0..n).map(|i| f32::from_bits(e.u32(&src[i * 4..i * 4 + 4]))).collect()),
        12 => OracleValue::Double((0..n).map(|i| f64::from_bits(u64::from_le_bytes(src[i * 8..i * 8 + 8].try_into().unwrap()))).collect()),
        other => return Err(format!("tiff oracle: unrecognized field type code {other}")),
    })
}

/// 📖️ Walks the header + the WHOLE `next IFD offset` chain, reading every IFD's tags and (for any
/// IFD carrying `StripOffsets`/`StripByteCounts`) its concatenated raw strip payload.
fn read_tiff(data: &[u8]) -> Result<OracleDoc, String> {
    if data.len() < 8 {
        return Err("tiff oracle: truncated header".to_string());
    }
    let (e, little_endian) = match &data[0..2] {
        b"II" => (Endian::Little, true),
        b"MM" => (Endian::Big, false),
        _ => return Err("tiff oracle: bad byte-order mark".to_string()),
    };
    if read_u16(data, 2, e)? != 42 {
        return Err("tiff oracle: bad magic number".to_string());
    }
    let mut ifds = Vec::new();
    let mut off = read_u32(data, 4, e)? as usize;
    let mut seen = std::collections::HashSet::new();
    while off != 0 {
        if !seen.insert(off) {
            return Err("tiff oracle: IFD offset cycle detected".to_string());
        }
        let count = read_u16(data, off, e)? as usize;
        let mut entries = Vec::with_capacity(count);
        let mut pos = off + 2;
        for _ in 0..count {
            let tag = read_u16(data, pos, e)?;
            let type_code = read_u16(data, pos + 2, e)?;
            let cnt = read_u32(data, pos + 4, e)?;
            let mut vf = [0u8; 4];
            vf.copy_from_slice(data.get(pos + 8..pos + 12).ok_or("tiff oracle: truncated IFD entry")?);
            entries.push((tag, read_tag_value(data, type_code, cnt, &vf, e)?));
            pos += 12;
        }
        let next = read_u32(data, pos, e)? as usize;

        let strip = if let (Some((_, OracleValue::Long(offs))), Some((_, OracleValue::Long(counts)))) = (entries.iter().find(|(t, _)| *t == TAG_STRIP_OFFSETS), entries.iter().find(|(t, _)| *t == TAG_STRIP_BYTE_COUNTS)) {
            let mut bytes = Vec::new();
            for (i, &start) in offs.iter().enumerate() {
                let len = *counts.get(i).ok_or("tiff oracle: StripByteCounts shorter than StripOffsets")? as usize;
                bytes.extend_from_slice(data.get(start as usize..start as usize + len).ok_or("tiff oracle: strip data truncated")?);
            }
            Some(bytes)
        } else {
            None
        };
        let entries: Vec<OracleTag> = entries.into_iter().filter(|(t, _)| *t != TAG_STRIP_OFFSETS && *t != TAG_STRIP_BYTE_COUNTS).map(|(tag, value)| OracleTag { tag, value }).collect();
        ifds.push(OracleIfd { entries, strip });
        off = next;
    }
    if ifds.is_empty() {
        return Err("tiff oracle: no IFD present".to_string());
    }
    Ok(OracleDoc { little_endian, ifds })
}

fn dir_size(n: usize) -> usize {
    2 + 12 * n + 4
}

/// ✍️ Re-serializes the WHOLE IFD chain — every IFD gets a real `next IFD offset` link (the
/// subject's own encoder now also writes a real multi-IFD chain; this independent writer stays a
/// genuinely separate implementation of the same real vocabulary, never importing the subject's
/// own `🚪️io::encode_tiff`). Any IFD carrying a `strip` gets fresh `StripOffsets`/`StripByteCounts`
/// computed from the actual final layout — unlike the subject, this oracle CAN back a non-primary
/// IFD's raster with real bytes when a caller's `pixels` param supplies them (`OracleIfd.strip`),
/// since it isn't constrained by `TiffSnapshot`'s single `pixels` field (see subject's own
/// `MultiIfdEncodeScopeNote`, `../🚪️io/🦀️component.rs`).
fn write_tiff(doc: &OracleDoc) -> Vec<u8> {
    let little = doc.little_endian;
    let mut out = Vec::new();
    out.extend_from_slice(if little { b"II" } else { b"MM" });
    write_u16(&mut out, 42, little);
    write_u32(&mut out, 8, little);

    // First pass: every IFD's full entry list (incl. fresh strip tags), so directory sizes are
    // known before any offset is computed.
    let mut full: Vec<Vec<OracleTag>> = doc
        .ifds
        .iter()
        .map(|ifd| {
            let mut entries = ifd.entries.clone();
            if let Some(strip) = &ifd.strip {
                // Always re-laid-out as ONE combined strip covering every row (never the
                // possibly-multi-strip layout the source may have used) — so `RowsPerStrip` MUST
                // be forced to the full `ImageLength`, or a conformant reader (rightly) expects as
                // many `StripOffsets`/`StripByteCounts` entries as `ceil(height/rowsPerStrip)`
                // and finds only the one this writer ever emits.
                if let Some(height) = entries.iter().find(|t| t.tag == TAG_IMAGE_LENGTH).and_then(|t| t.value.first_u32()) {
                    entries.retain(|t| t.tag != TAG_ROWS_PER_STRIP);
                    entries.push(OracleTag { tag: TAG_ROWS_PER_STRIP, value: OracleValue::Long(vec![height]) });
                }
                entries.push(OracleTag { tag: TAG_STRIP_OFFSETS, value: OracleValue::Long(vec![0]) });
                entries.push(OracleTag { tag: TAG_STRIP_BYTE_COUNTS, value: OracleValue::Long(vec![strip.len() as u32]) });
            }
            entries.sort_by_key(|t| t.tag);
            entries
        })
        .collect();

    let mut cursor = 8usize;
    let mut dir_offsets = Vec::with_capacity(full.len());
    for entries in &full {
        dir_offsets.push(cursor);
        let out_of_line: usize = entries
            .iter()
            .map(|t| {
                let l = t.value.bytes(little).len();
                if l <= 4 {
                    0
                } else {
                    l + (l % 2)
                }
            })
            .sum();
        cursor += dir_size(entries.len()) + out_of_line;
    }
    // Strip payloads are appended after ALL directories + out-of-line data, in IFD order.
    let mut strip_offsets = Vec::with_capacity(full.len());
    for ifd in &doc.ifds {
        strip_offsets.push(cursor);
        if let Some(strip) = &ifd.strip {
            cursor += strip.len();
        }
    }
    for (i, ifd) in doc.ifds.iter().enumerate() {
        if ifd.strip.is_some() {
            if let Some(t) = full[i].iter_mut().find(|t| t.tag == TAG_STRIP_OFFSETS) {
                t.value = OracleValue::Long(vec![strip_offsets[i] as u32]);
            }
        }
    }

    for (i, entries) in full.iter().enumerate() {
        debug_assert_eq!(out.len(), dir_offsets[i]);
        write_u16(&mut out, entries.len() as u16, little);
        let out_of_line_start = dir_offsets[i] + dir_size(entries.len());
        let mut oo_cursor = out_of_line_start;
        for t in entries {
            write_u16(&mut out, t.tag, little);
            write_u16(&mut out, t.value.type_code(), little);
            write_u32(&mut out, t.value.count(), little);
            let vb = t.value.bytes(little);
            if vb.len() <= 4 {
                let mut field = [0u8; 4];
                field[..vb.len()].copy_from_slice(&vb);
                out.extend_from_slice(&field);
            } else {
                write_u32(&mut out, oo_cursor as u32, little);
                oo_cursor += vb.len() + (vb.len() % 2);
            }
        }
        let next_ifd_offset = if i + 1 < dir_offsets.len() { dir_offsets[i + 1] as u32 } else { 0 };
        write_u32(&mut out, next_ifd_offset, little);
        for t in entries {
            let vb = t.value.bytes(little);
            if vb.len() > 4 {
                out.extend_from_slice(&vb);
                if vb.len() % 2 == 1 {
                    out.push(0);
                }
            }
        }
    }
    for ifd in &doc.ifds {
        if let Some(strip) = &ifd.strip {
            out.extend_from_slice(strip);
        }
    }
    out
}
//#endregion 🔖️IndependentCodec

//#region 🔖️RasterProjection
/// 👁️ Decodes IFD 0's baseline uncompressed raster into 8-bit RGBA — scoped to exactly what this
/// module's own [`write_tiff`] and the derived real-world fixtures ever produce (uncompressed,
/// 8-bit, 1/3/4 samples per pixel, single combined strip), honestly erroring otherwise rather than
/// fabricating pixels (same `CompressionScopeNote` honesty the subject's own decoder documents).
fn decode_raster(ifd: &OracleIfd) -> Result<(u32, u32, Vec<u8>), String> {
    let width = ifd.get(TAG_IMAGE_WIDTH).and_then(|t| t.value.first_u32()).ok_or("tiff oracle: missing ImageWidth")?;
    let height = ifd.get(TAG_IMAGE_LENGTH).and_then(|t| t.value.first_u32()).ok_or("tiff oracle: missing ImageLength")?;
    let bits = ifd.get(TAG_BITS_PER_SAMPLE).and_then(|t| t.value.first_u32()).unwrap_or(8);
    if bits != 8 {
        return Err(format!("tiff oracle: unsupported BitsPerSample {bits} (only 8 is implemented)"));
    }
    let spp = ifd.get(TAG_SAMPLES_PER_PIXEL).and_then(|t| t.value.first_u32()).unwrap_or(1);
    let compression = ifd.get(TAG_COMPRESSION).and_then(|t| t.value.first_u32()).unwrap_or(1);
    if compression != 1 {
        return Err(format!("tiff oracle: unsupported compression {compression} (only uncompressed is implemented)"));
    }
    let photometric = ifd.get(TAG_PHOTOMETRIC).and_then(|t| t.value.first_u32()).unwrap_or(1);
    let strip = ifd.strip.as_ref().ok_or("tiff oracle: IFD carries no strip payload")?;
    let row_bytes = width as usize * spp as usize;
    if strip.len() < row_bytes * height as usize {
        return Err("tiff oracle: strip shorter than width*height*samplesPerPixel".to_string());
    }
    let mut rgba = vec![0u8; width as usize * height as usize * 4];
    for p in 0..(width as usize * height as usize) {
        let so = p * spp as usize;
        let o = p * 4;
        match spp {
            1 => {
                let g = if photometric == 0 { 255 - strip[so] } else { strip[so] };
                rgba[o..o + 4].copy_from_slice(&[g, g, g, 255]);
            }
            3 => {
                rgba[o..o + 3].copy_from_slice(&strip[so..so + 3]);
                rgba[o + 3] = 255;
            }
            4 => rgba[o..o + 4].copy_from_slice(&strip[so..so + 4]),
            other => return Err(format!("tiff oracle: unsupported SamplesPerPixel {other}")),
        }
    }
    Ok((width, height, rgba))
}

/// 🔁️ The projection every mutate/inverse/round-trip scenario is compared through. Reports the
/// byte-order mark, every IFD's full typed tag list (proving `InsertIfd`/`RemoveIfd`/`SetTag`/
/// `RemoveTag` structurally), and IFD 0's independently decoded raster (proving `SetPixels` and the
/// pixel-affecting geometry tags). Key names deliberately avoid `semantic-raster-v1`'s own
/// `ignoreKeys` (`filter`, `interlace`, `compression`, `chunkOrder`, `ancillaryChunks`, `gamma`,
/// `software`, `encoderVersion`, `byteLength`, `fileSize`, `rowStride`, `paletteOrder`) so nothing
/// this module wants compared is silently stripped.
/// #️⃣️ Deterministic, dependency-free FNV-1a 64-bit digest, hex-formatted — TIFF (unlike JPEG) is
/// lossless, so an exact digest is the right compact stand-in for "every sample survived": a real
/// mutation-round-trip bug shows up as a mismatch just as reliably as dumping the full sample
/// array would, without materializing millions of `Json::Number` nodes for a real-world-sized
/// raster (this fixture alone decodes to 23M+ RGBA bytes).
fn fnv1a_hex(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn project_doc(doc: &OracleDoc) -> Json {
    let ifds: Vec<Json> = doc
        .ifds
        .iter()
        .map(|ifd| {
            let entries: Vec<Json> = ifd.entries.iter().map(|t| Json::Object(vec![("tag".to_string(), Json::Number(t.tag as f64)), ("type".to_string(), Json::Number(t.value.type_code() as f64)), ("values".to_string(), t.value.to_json())])).collect();
            Json::Object(vec![("entries".to_string(), Json::Array(entries))])
        })
        .collect();
    let mut fields = vec![
        ("format".to_string(), Json::String("tiff".to_string())),
        ("byteOrder".to_string(), Json::String(if doc.little_endian { "little-endian" } else { "big-endian" }.to_string())),
        ("ifdCount".to_string(), Json::Number(doc.ifds.len() as f64)),
        ("ifds".to_string(), Json::Array(ifds)),
    ];
    if let Some(ifd0) = doc.ifds.first() {
        if let Ok((width, height, rgba)) = decode_raster(ifd0) {
            let mut luma_buckets = [0u32; 8];
            for px in rgba.chunks_exact(4) {
                let luma = (u32::from(px[0]) * 299 + u32::from(px[1]) * 587 + u32::from(px[2]) * 114) / 1000;
                luma_buckets[(luma / 32).min(7) as usize] += 1;
            }
            fields.push(("width".to_string(), Json::Number(width as f64)));
            fields.push(("height".to_string(), Json::Number(height as f64)));
            fields.push(("channels".to_string(), Json::Number(4.0)));
            fields.push(("bitDepth".to_string(), Json::Number(8.0)));
            fields.push(("samplesDigest".to_string(), Json::String(fnv1a_hex(&rgba))));
            fields.push(("lumaHistogram".to_string(), Json::Array(luma_buckets.iter().map(|&c| Json::Number(c as f64)).collect())));
        }
    }
    Json::Object(fields)
}

/// 👁️ Projects real TIFF bytes with the INDEPENDENT reader above — used identically by both the
/// oracle and the subject's own handlers (same pattern `project_pdf`/`project_obj`/`project_image`
/// already use), so it is the projection, not the writer, that carries independence here.
#[cfg(feature = "oracles")]
pub fn project_tiff(input: &[u8]) -> Result<Json, String> {
    read_tiff(input).map(|doc| project_doc(&doc))
}
//#endregion 🔖️RasterProjection

//#region 🔖️MutationParams
/// 🧩️ Parses one `{"entries":[{"tag":n,"type":n,"values":[...]}],"pixels":"<hex>"}` JSON object
/// into an [`OracleIfd`] — the shared shape `insert-ifd`'s `ifd` param and `set-snapshot`'s `ifds[]`
/// entries both use. `StripOffsets`/`StripByteCounts` are never accepted from a caller (they are
/// always layout-computed at [`write_tiff`] time) — a caller wanting a raster IFD supplies `pixels`
/// (raw strip bytes, hex, already in the sample layout its own `SamplesPerPixel` tag declares).
fn parse_ifd_json(v: &Json) -> Result<OracleIfd, String> {
    let entries = j_get(v, "entries").and_then(j_arr).ok_or("tiff oracle: ifd needs an `entries` array")?;
    let entries: Vec<OracleTag> = entries
        .iter()
        .filter_map(|e| {
            let tag = j_get(e, "tag").and_then(j_num)? as u16;
            if tag == TAG_STRIP_OFFSETS || tag == TAG_STRIP_BYTE_COUNTS {
                return None;
            }
            let type_code = j_get(e, "type").and_then(j_num)? as u16;
            let values = j_get(e, "values")?;
            Some(OracleValue::from_json(type_code, values).map(|value| OracleTag { tag, value }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let mut entries = entries;
    entries.sort_by_key(|t| t.tag);
    let strip = match j_get(v, "pixels").and_then(j_str) {
        Some(hex) => Some(hex_decode(hex)?),
        None => None,
    };
    Ok(OracleIfd { entries, strip })
}

fn parse_doc_json(v: &Json) -> Result<OracleDoc, String> {
    let little_endian = match j_get(v, "byteOrder").and_then(j_str) {
        Some("big-endian") => false,
        _ => true,
    };
    let ifds = j_get(v, "ifds").and_then(j_arr).ok_or("tiff oracle: set-snapshot needs an `ifds` array")?;
    let ifds = ifds.iter().map(parse_ifd_json).collect::<Result<Vec<_>, String>>()?;
    Ok(OracleDoc { little_endian, ifds })
}
//#endregion 🔖️MutationParams

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test. Index/tag targets out of range are documented no-ops, mirroring
/// `TiffMutation`'s own semantics exactly (`../🧬️schema/🧬️mutations/🦀️component.rs`).
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    let params = spec.get("params");
    let p_num = |key: &str| -> Option<f64> { params.and_then(|p| j_get(p, key)).and_then(j_num) };
    let p_str = |key: &str| -> Option<&str> { params.and_then(|p| j_get(p, key)).and_then(j_str) };

    if kind == "set-snapshot" {
        let params = params.ok_or("tiff oracle: set-snapshot needs `params`")?;
        return Ok(write_tiff(&parse_doc_json(params)?));
    }

    let mut doc = read_tiff(input)?;
    match kind.as_str() {
        "no-mutation" => {}
        "set-byte-order" => {
            doc.little_endian = p_str("byteOrder") != Some("big-endian");
        }
        "insert-ifd" => {
            let index = (p_num("index").ok_or("tiff oracle: insert-ifd needs `index`")? as usize).min(doc.ifds.len());
            let ifd_json = params.and_then(|p| j_get(p, "ifd")).ok_or("tiff oracle: insert-ifd needs `ifd`")?;
            doc.ifds.insert(index, parse_ifd_json(ifd_json)?);
        }
        "remove-ifd" => {
            let index = p_num("index").ok_or("tiff oracle: remove-ifd needs `index`")? as usize;
            if index < doc.ifds.len() {
                doc.ifds.remove(index);
            }
        }
        "set-tag" => {
            let ifd_index = p_num("ifdIndex").ok_or("tiff oracle: set-tag needs `ifdIndex`")? as usize;
            let tag = p_num("tag").ok_or("tiff oracle: set-tag needs `tag`")? as u16;
            let type_code = p_num("type").ok_or("tiff oracle: set-tag needs `type`")? as u16;
            let values = params.and_then(|p| j_get(p, "values")).ok_or("tiff oracle: set-tag needs `values`")?;
            if let Some(ifd) = doc.ifds.get_mut(ifd_index) {
                ifd.set(tag, OracleValue::from_json(type_code, values)?);
            }
        }
        "remove-tag" => {
            let ifd_index = p_num("ifdIndex").ok_or("tiff oracle: remove-tag needs `ifdIndex`")? as usize;
            let tag = p_num("tag").ok_or("tiff oracle: remove-tag needs `tag`")? as u16;
            if let Some(ifd) = doc.ifds.get_mut(ifd_index) {
                ifd.entries.retain(|t| t.tag != tag);
            }
        }
        "set-pixels" => {
            let hex = p_str("pixels").ok_or("tiff oracle: set-pixels needs `pixels` (hex RGBA8)")?;
            let rgba = hex_decode(hex)?;
            let ifd0 = doc.ifds.first_mut().ok_or("tiff oracle: set-pixels needs an existing IFD 0")?;
            let width = ifd0.get(TAG_IMAGE_WIDTH).and_then(|t| t.value.first_u32()).ok_or("tiff oracle: IFD 0 has no ImageWidth")?;
            let height = ifd0.get(TAG_IMAGE_LENGTH).and_then(|t| t.value.first_u32()).ok_or("tiff oracle: IFD 0 has no ImageLength")?;
            let expected = width as usize * height as usize * 4;
            if rgba.len() != expected {
                return Err(format!("tiff oracle: set-pixels payload is {} byte(s), expected {} ({width}x{height} RGBA8)", rgba.len(), expected));
            }
            let rgb: Vec<u8> = rgba.chunks_exact(4).flat_map(|px| [px[0], px[1], px[2]]).collect();
            ifd0.set(TAG_BITS_PER_SAMPLE, OracleValue::Short(vec![8]));
            ifd0.set(TAG_COMPRESSION, OracleValue::Short(vec![1]));
            ifd0.set(TAG_PHOTOMETRIC, OracleValue::Short(vec![2]));
            ifd0.set(TAG_SAMPLES_PER_PIXEL, OracleValue::Short(vec![3]));
            ifd0.set(TAG_ROWS_PER_STRIP, OracleValue::Long(vec![height]));
            ifd0.strip = Some(rgb);
        }
        other => return Err(format!("mutation kind {other:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
    Ok(write_tiff(&doc))
}

/// ↩️ Applies the INDEPENDENTLY computed inverse of `spec` on top of `mutated`, so that
/// `inverse(m) . m` must be the identity on the semantic projection. The inverse is reasoned from
/// the PRE-mutation document (`original_input`) exactly the way `TiffMutation::inverse`
/// (`../🧬️schema/🧬️mutations/🦀️component.rs`) reasons over `TiffSnapshot` — "restore `base`'s own
/// value for the facet this kind touched" — reimplemented here over [`OracleDoc`] rather than
/// called through that trait. `set-pixels` restores IFD 0 wholesale because this oracle's own
/// forward `set-pixels` rewrites IFD 0's strip AND the five layout tags that describe it, so
/// restoring only the raster would not be its inverse.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let params = spec.get("params");
    let p_num = |key: &str| -> Option<f64> { params.and_then(|p| j_get(p, key)).and_then(j_num) };
    let original = read_tiff(original_input)?;
    if kind == "set-snapshot" {
        return Ok(write_tiff(&original));
    }
    let mut doc = read_tiff(mutated)?;
    match kind.as_str() {
        "no-mutation" => {}
        "set-byte-order" => doc.little_endian = original.little_endian,
        "insert-ifd" => {
            let index = (p_num("index").ok_or("tiff oracle: insert-ifd needs `index`")? as usize).min(original.ifds.len());
            if index < doc.ifds.len() {
                doc.ifds.remove(index);
            }
        }
        "remove-ifd" => {
            let index = p_num("index").ok_or("tiff oracle: remove-ifd needs `index`")? as usize;
            if let Some(ifd) = original.ifds.get(index) {
                let at = index.min(doc.ifds.len());
                doc.ifds.insert(at, ifd.clone());
            }
        }
        "set-tag" | "remove-tag" => {
            let ifd_index = p_num("ifdIndex").ok_or("tiff oracle: set-tag/remove-tag inverse needs `ifdIndex`")? as usize;
            let tag = p_num("tag").ok_or("tiff oracle: set-tag/remove-tag inverse needs `tag`")? as u16;
            let restored = original.ifds.get(ifd_index).and_then(|ifd| ifd.get(tag)).cloned();
            if let Some(ifd) = doc.ifds.get_mut(ifd_index) {
                match restored {
                    Some(existing) => ifd.set(tag, existing.value),
                    None => ifd.entries.retain(|entry| entry.tag != tag),
                }
            }
        }
        "set-pixels" => {
            let source = original.ifds.first().ok_or("tiff oracle: set-pixels inverse needs an original IFD 0")?.clone();
            let target = doc.ifds.first_mut().ok_or("tiff oracle: set-pixels inverse needs a mutated IFD 0")?;
            *target = source;
        }
        other => return Err(format!("mutation kind {other:?} has no oracle inverse ({} mutated byte(s))", mutated.len())),
    }
    Ok(write_tiff(&doc))
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation_inverse(_original_input: &[u8], _spec: &Json, _mutated: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
#[cfg(not(feature = "oracles"))]
pub fn project_tiff(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️FixtureDerivation
/// 🧫️ One-off real-world fixture derivation (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 7).
/// NOT a test step — `#[ignore]`d, run once by hand, same convention as this artifact's own
/// `zzz_write_native_tiff_fixture` (`../🚪️io/🦀️component.rs`). Builds the committed
/// `shared://🖼️abbau-aufbau-masterarbeit-grundriss.tiff`: IFD 0 is the REAL 500 DPI architectural
/// scan (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/…jpg`, 2275x2560), encoded to
/// TIFF with the registered `image` reference encoder (`image::codecs::tiff::TiffEncoder` — the
/// REAL third-party writer, not this module's own). IFD 1 is a REAL second page — the actual
/// decoded, downsampled pixels of `…rathaus-ahlen-grundriss.png` (no synthetic content) — appended
/// with this module's own [`write_tiff`], since `image` cannot itself emit a second IFD. The two
/// pages make `RemoveIfd`'s mutate/inverse scenarios substantive on a genuinely multi-IFD document
/// without needing a second `Given` fixture per the Scenario Outline's single shared input.
#[cfg(all(test, feature = "oracles"))]
mod fixture_derivation {
    use super::*;

    fn ifd0_from_image_encoder(rgb: image::RgbImage) -> OracleIfd {
        let mut cursor = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgb8(rgb).write_to(&mut cursor, image::ImageFormat::Tiff).expect("image crate: encode reference TIFF");
        let doc = read_tiff(&cursor.into_inner()).expect("re-parse the reference encoder's own bytes");
        doc.ifds.into_iter().next().expect("reference encoder wrote at least one IFD")
    }

    /// 🧭️ Walks up from `start` looking for the repo root's own `CLAUDE.md` — robust regardless of
    /// how deep the compiling crate's manifest happens to sit (this file is also `#[path]`-included
    /// from a throwaway type-checking crate outside the real oracle crate's tree during review).
    fn find_repo_root(start: &std::path::Path) -> std::path::PathBuf {
        let mut dir = start.to_path_buf();
        for _ in 0..32 {
            if dir.join("CLAUDE.md").is_file() {
                return dir;
            }
            if !dir.pop() {
                break;
            }
        }
        panic!("could not find repo root (CLAUDE.md) above {}", start.display());
    }

    #[test]
    #[ignore]
    fn derive_real_world_fixture() {
        let repo_root = find_repo_root(std::path::Path::new(env!("CARGO_MANIFEST_DIR")));
        let jpeg_path = repo_root.join("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/🖼️abbau-aufbau-masterarbeit-grundriss.jpg");
        let png_path = repo_root.join("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/🖼️rathaus-ahlen-grundriss.png");
        let out_path = repo_root.join("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🧫️fixtures/🖼️abbau-aufbau-masterarbeit-grundriss.tiff");

        // IFD 0: the real 500 DPI scan, RGB8, via the registered `image` reference encoder.
        let photo = image::open(&jpeg_path).expect("open real JPEG scan").to_rgb8();
        assert_eq!((photo.width(), photo.height()), (2275, 2560), "source scan dimensions moved — re-check the ticket's own numbers");
        let ifd0 = ifd0_from_image_encoder(photo.clone());

        // IFD 1: the real second page — genuine decoded+downsampled pixels of the rathaus PNG,
        // decoded with the registered `png` reference decoder (independent of `image`, which has
        // no PNG feature linked in this crate) then downsampled with `image`'s own generic resize.
        let png_bytes = std::fs::read(&png_path).expect("read real PNG floor plan");
        let mut png_reader = png::Decoder::new(std::io::Cursor::new(&png_bytes)).read_info().expect("png: read_info");
        let mut buf = vec![0u8; png_reader.output_buffer_size().unwrap_or(0)];
        let frame = png_reader.next_frame(&mut buf).expect("png: next_frame");
        let info = png_reader.info();
        let palette = info.palette.clone();
        let trns = info.trns.clone();
        let rgba: Vec<u8> = match frame.color_type {
            png::ColorType::Rgba => buf[..frame.buffer_size()].to_vec(),
            png::ColorType::Rgb => buf[..frame.buffer_size()].chunks_exact(3).flat_map(|p| [p[0], p[1], p[2], 255]).collect(),
            png::ColorType::Grayscale => buf[..frame.buffer_size()].iter().flat_map(|&g| [g, g, g, 255]).collect(),
            png::ColorType::GrayscaleAlpha => buf[..frame.buffer_size()].chunks_exact(2).flat_map(|p| [p[0], p[0], p[0], p[1]]).collect(),
            png::ColorType::Indexed => {
                let table = palette.as_deref().expect("indexed PNG without a palette");
                buf[..frame.buffer_size()]
                    .iter()
                    .flat_map(|&index| {
                        let base = index as usize * 3;
                        let alpha = trns.as_deref().and_then(|t| t.get(index as usize).copied()).unwrap_or(255);
                        [table[base], table[base + 1], table[base + 2], alpha]
                    })
                    .collect()
            }
        };
        let full = image::RgbaImage::from_raw(frame.width, frame.height, rgba).expect("rathaus PNG raw buffer matches its own dimensions");
        let small = image::imageops::thumbnail(&full, 16, 16);
        let ifd1 = ifd0_from_image_encoder(image::DynamicImage::ImageRgba8(small).to_rgb8());

        let doc = OracleDoc { little_endian: true, ifds: vec![ifd0, ifd1] };
        let bytes = write_tiff(&doc);

        // Prove the spliced file is genuinely readable back — both by this module's own
        // independent reader AND by the registered `image` decoder (IFD 0 only, its own scope).
        let reparsed = read_tiff(&bytes).expect("re-parse the derived multi-IFD fixture");
        assert_eq!(reparsed.ifds.len(), 2, "derived fixture must carry exactly two real IFDs");
        let (w, h, _) = decode_raster(&reparsed.ifds[0]).expect("decode IFD 0 raster");
        assert_eq!((w, h), (2275, 2560));
        let via_image = image::codecs::tiff::TiffDecoder::new(std::io::Cursor::new(&bytes)).expect("image crate: independently parse derived fixture");
        assert_eq!(image::ImageDecoder::dimensions(&via_image), (2275, 2560));

        std::fs::write(&out_path, &bytes).expect("write committed shared:// fixture");
        eprintln!("wrote {} ({} bytes)", out_path.display(), bytes.len());

        // A SMALL real thumbnail (8x8, genuine decoded+downsampled rathaus pixels — not
        // synthetic) reused inline as `insert-ifd`/`set-snapshot`'s real embedded-IFD content in
        // the feature file's Examples table: printed as hex here rather than committed, since it's
        // small enough to live directly in the feature text (192 bytes = 384 hex chars).
        let tiny = image::imageops::thumbnail(&full, 8, 8);
        let tiny_rgb = image::DynamicImage::ImageRgba8(tiny).to_rgb8();
        let tiny_hex: String = tiny_rgb.as_raw().iter().map(|b| format!("{b:02x}")).collect();
        eprintln!("inline 8x8 real thumbnail hex (insert-ifd/set-snapshot pixels): {tiny_hex}");

        // A committed `local://` binary fixture for `set-pixels`: the SAME real photo's own
        // pixels, horizontally flipped (still 100% real content, but a genuinely different,
        // provable raster) — full IFD 0 resolution, so it can only reasonably live as a binary
        // fixture, not inline JSON hex.
        let (w, h) = (photo.width(), photo.height());
        let mut flipped_rgba = Vec::with_capacity(w as usize * h as usize * 4);
        for y in 0..h {
            for x in 0..w {
                let px = photo.get_pixel(w - 1 - x, y);
                flipped_rgba.extend_from_slice(&[px[0], px[1], px[2], 255]);
            }
        }
        let case_fixture_dir = repo_root.join("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🧪️tests/mutate-tiff-6-0/🧫️fixtures");
        std::fs::create_dir_all(&case_fixture_dir).expect("create case fixtures dir");
        let flipped_path = case_fixture_dir.join("🔄️flipped-scan.rgba");
        std::fs::write(&flipped_path, &flipped_rgba).expect("write local:// set-pixels fixture");
        eprintln!("wrote {} ({} bytes, {w}x{h} RGBA8)", flipped_path.display(), flipped_rgba.len());
    }
}
//#endregion 🔖️FixtureDerivation
