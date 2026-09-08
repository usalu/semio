//! 📐 First-party intrinsic-dimension reader for image/SVG bytes — no runtime dependency.
//!
//! Answers exactly one narrow question: given PNG/JPEG/GIF/WebP or SVG bytes, what are the
//! intrinsic pixel dimensions? Header/attribute reading only — never a full pixel decode, never a
//! renderer. Built so a `wasm32-wasip2` guest component can answer that question for a widget's
//! natural size (`preview_media_natural_size` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`)
//! without linking `image`/`usvg` and their ~50-crate dependency tail (ticket
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
//! `🔍️research/📓️intrinsic-size-parser.md`).

use std::fmt;

//#region 🔖️Error

/// ⚠️ Everything that can go wrong reading intrinsic dimensions — a malformed/unsupported input,
/// never a panic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntrinsicSizeError {
    UnrecognizedFormat,
    Truncated,
    NoSvgElement,
    InvalidDimensions,
}

impl fmt::Display for IntrinsicSizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnrecognizedFormat => write!(f, "unrecognized image format"),
            Self::Truncated => write!(f, "input truncated before dimensions could be read"),
            Self::NoSvgElement => write!(f, "no <svg> root element found"),
            Self::InvalidDimensions => write!(f, "explicit width/height is zero, negative, or otherwise invalid"),
        }
    }
}

impl std::error::Error for IntrinsicSizeError {}

//#endregion 🔖️Error

//#region 🔖️Raster

/// 🖼️ Reads the intrinsic pixel dimensions of a PNG, JPEG, GIF, or WebP byte buffer from its
/// header alone (magic-byte dispatch), never a full pixel decode.
pub fn raster_dimensions(bytes: &[u8]) -> Result<(u32, u32), IntrinsicSizeError> {
    if bytes.starts_with(&PNG_SIGNATURE) {
        return png_dimensions(bytes);
    }
    if bytes.starts_with(&[0xFF, 0xD8]) {
        return jpeg_dimensions(bytes);
    }
    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return gif_dimensions(bytes);
    }
    if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        return webp_dimensions(bytes);
    }
    Err(IntrinsicSizeError::UnrecognizedFormat)
}

const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

/// 🅿️ PNG `IHDR` is always the first chunk, at a fixed offset right after the 8-byte signature:
/// `[len:4][type:4=b"IHDR"][width:4 BE][height:4 BE]...`.
fn png_dimensions(bytes: &[u8]) -> Result<(u32, u32), IntrinsicSizeError> {
    if bytes.len() < 8 + 8 + 8 {
        return Err(IntrinsicSizeError::Truncated);
    }
    if &bytes[12..16] != b"IHDR" {
        return Err(IntrinsicSizeError::UnrecognizedFormat);
    }
    let w = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    let h = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
    if w == 0 || h == 0 {
        return Err(IntrinsicSizeError::InvalidDimensions);
    }
    Ok((w, h))
}

/// 📷 JPEG dimensions live in the first `SOFn` segment (`n` in `0..=15`, excluding `SOF4`/`SOF8`/
/// `SOF12` — those marker bytes are reused for DHT/JPG/DAC, not a start-of-frame). Segment layout
/// after the marker byte: `[len:2 BE][precision:1][height:2 BE][width:2 BE]...`.
fn jpeg_dimensions(bytes: &[u8]) -> Result<(u32, u32), IntrinsicSizeError> {
    let mut pos = 2;
    while pos + 4 <= bytes.len() {
        if bytes[pos] != 0xFF {
            pos += 1;
            continue;
        }
        let mut marker_pos = pos;
        while marker_pos < bytes.len() && bytes[marker_pos] == 0xFF {
            marker_pos += 1;
        }
        if marker_pos >= bytes.len() {
            return Err(IntrinsicSizeError::Truncated);
        }
        let marker = bytes[marker_pos];
        pos = marker_pos + 1;
        if marker == 0xD8 || marker == 0x01 || (0xD0..=0xD7).contains(&marker) {
            continue;
        }
        if marker == 0xD9 {
            return Err(IntrinsicSizeError::UnrecognizedFormat);
        }
        if pos + 2 > bytes.len() {
            return Err(IntrinsicSizeError::Truncated);
        }
        let seg_len = u16::from_be_bytes([bytes[pos], bytes[pos + 1]]) as usize;
        if seg_len < 2 {
            return Err(IntrinsicSizeError::UnrecognizedFormat);
        }
        let is_sof = (0xC0..=0xCF).contains(&marker) && marker != 0xC4 && marker != 0xC8 && marker != 0xCC;
        if is_sof {
            if pos + 1 + 5 > bytes.len() {
                return Err(IntrinsicSizeError::Truncated);
            }
            let h = u16::from_be_bytes([bytes[pos + 3], bytes[pos + 4]]);
            let w = u16::from_be_bytes([bytes[pos + 5], bytes[pos + 6]]);
            if w == 0 || h == 0 {
                return Err(IntrinsicSizeError::InvalidDimensions);
            }
            return Ok((u32::from(w), u32::from(h)));
        }
        if marker == 0xDA {
            return Err(IntrinsicSizeError::UnrecognizedFormat);
        }
        pos += seg_len;
    }
    Err(IntrinsicSizeError::Truncated)
}

/// 🎞️ GIF logical screen descriptor: 6-byte signature, then `[width:2 LE][height:2 LE]` at
/// bytes 6..10.
fn gif_dimensions(bytes: &[u8]) -> Result<(u32, u32), IntrinsicSizeError> {
    if bytes.len() < 10 {
        return Err(IntrinsicSizeError::Truncated);
    }
    let w = u16::from_le_bytes([bytes[6], bytes[7]]);
    let h = u16::from_le_bytes([bytes[8], bytes[9]]);
    if w == 0 || h == 0 {
        return Err(IntrinsicSizeError::InvalidDimensions);
    }
    Ok((u32::from(w), u32::from(h)))
}

/// 🕸️ WebP is a RIFF container; the first chunk's FourCC selects the payload layout:
/// `VP8 ` (lossy, 14-bit dims after a 3-byte frame tag + 3-byte start code),
/// `VP8L` (lossless, 14-bit dims bit-packed after a 1-byte signature),
/// `VP8X` (extended, 24-bit canvas dims after a 1-byte flags + 3-byte reserved).
fn webp_dimensions(bytes: &[u8]) -> Result<(u32, u32), IntrinsicSizeError> {
    if bytes.len() < 20 {
        return Err(IntrinsicSizeError::Truncated);
    }
    let fourcc = &bytes[12..16];
    let payload = &bytes[20..];
    match fourcc {
        b"VP8 " => {
            if payload.len() < 10 {
                return Err(IntrinsicSizeError::Truncated);
            }
            if payload[3..6] != [0x9D, 0x01, 0x2A] {
                return Err(IntrinsicSizeError::UnrecognizedFormat);
            }
            let w = u16::from_le_bytes([payload[6], payload[7]]) & 0x3FFF;
            let h = u16::from_le_bytes([payload[8], payload[9]]) & 0x3FFF;
            if w == 0 || h == 0 {
                return Err(IntrinsicSizeError::InvalidDimensions);
            }
            Ok((u32::from(w), u32::from(h)))
        }
        b"VP8L" => {
            if payload.len() < 5 {
                return Err(IntrinsicSizeError::Truncated);
            }
            if payload[0] != 0x2F {
                return Err(IntrinsicSizeError::UnrecognizedFormat);
            }
            let bits = u32::from_le_bytes([payload[1], payload[2], payload[3], payload[4]]);
            let w = (bits & 0x3FFF) + 1;
            let h = ((bits >> 14) & 0x3FFF) + 1;
            Ok((w, h))
        }
        b"VP8X" => {
            if payload.len() < 10 {
                return Err(IntrinsicSizeError::Truncated);
            }
            let w = 1 + u32::from(payload[4]) + (u32::from(payload[5]) << 8) + (u32::from(payload[6]) << 16);
            let h = 1 + u32::from(payload[7]) + (u32::from(payload[8]) << 8) + (u32::from(payload[9]) << 16);
            Ok((w, h))
        }
        _ => Err(IntrinsicSizeError::UnrecognizedFormat),
    }
}

//#endregion 🔖️Raster

//#region 🔖️Svg

/// 🔤 Reads the intrinsic (`width`/`height`, falling back to `viewBox`) size of an SVG document
/// from its root `<svg>` element's attributes only — XML attribute reading, not a renderer. Rules
/// were reverse-derived against `usvg::Tree::size()`'s observed resolution (see
/// `🔍️research/📓️intrinsic-size-parser.md`): an explicit `width`/`height` that parses as a
/// non-positive or non-finite number is a hard error; a value that fails to parse at all (garbage,
/// unrecognized unit, overflow-to-infinity) is treated the same as an absent attribute; a
/// present-and-absolute value is used directly; a present percentage resolves against the
/// container; an absent value falls back to the container; the container is the `viewBox` (when
/// it parses to at least 4 numbers with a positive width and height, extra trailing numbers
/// ignored) or a flat 100×100 default otherwise.
pub fn svg_intrinsic_size(svg: &str) -> Result<(f64, f64), IntrinsicSizeError> {
    let tag = find_svg_tag(svg).ok_or(IntrinsicSizeError::NoSvgElement)?;
    let attrs = parse_attributes(tag);
    let view_box = attrs.get("viewBox").and_then(|v| parse_view_box(v));
    let (container_w, container_h) = view_box.map_or((100.0, 100.0), |(_, _, w, h)| (w, h));
    let width = resolve_length(attrs.get("width").map(String::as_str), container_w)?;
    let height = resolve_length(attrs.get("height").map(String::as_str), container_h)?;
    Ok((width, height))
}

fn find_svg_tag(svg: &str) -> Option<&str> {
    let bytes = svg.as_bytes();
    let mut search_from = 0usize;
    loop {
        let rel = svg[search_from..].find("<svg")?;
        let start = search_from + rel;
        let after = start + 4;
        let boundary_ok = match bytes.get(after) {
            None => true,
            Some(b) => b.is_ascii_whitespace() || *b == b'>' || *b == b'/',
        };
        if boundary_ok {
            return extract_tag(svg, after);
        }
        search_from = after;
    }
}

fn extract_tag(svg: &str, from: usize) -> Option<&str> {
    let bytes = svg.as_bytes();
    let mut i = from;
    let mut quote: Option<u8> = None;
    while i < bytes.len() {
        let b = bytes[i];
        match quote {
            Some(q) => {
                if b == q {
                    quote = None;
                }
            }
            None => match b {
                b'"' | b'\'' => quote = Some(b),
                b'>' => return Some(&svg[from..i]),
                _ => {}
            },
        }
        i += 1;
    }
    None
}

fn parse_attributes(tag: &str) -> std::collections::HashMap<String, String> {
    let mut out = std::collections::HashMap::new();
    let bytes = tag.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        while i < bytes.len() && (bytes[i].is_ascii_whitespace() || bytes[i] == b'/') {
            i += 1;
        }
        let name_start = i;
        while i < bytes.len() && bytes[i] != b'=' && !bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if name_start == i {
            break;
        }
        let name = &tag[name_start..i];
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] != b'=' {
            continue;
        }
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        let quote = bytes[i];
        if quote != b'"' && quote != b'\'' {
            continue;
        }
        i += 1;
        let value_start = i;
        while i < bytes.len() && bytes[i] != quote {
            i += 1;
        }
        if i > bytes.len() {
            break;
        }
        let value = tag.get(value_start..i).unwrap_or_default();
        out.insert(name.to_string(), value.to_string());
        i += 1;
    }
    out
}

/// 📦 `viewBox="minx miny w h"` (whitespace and/or comma separated, extra trailing numbers
/// ignored — matches observed `usvg` tolerance). `None` when unparseable or `w`/`h` non-positive.
fn parse_view_box(raw: &str) -> Option<(f64, f64, f64, f64)> {
    let nums: Vec<f64> = raw
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f64>())
        .collect::<Result<_, _>>()
        .ok()?;
    if nums.len() < 4 {
        return None;
    }
    let (minx, miny, w, h) = (nums[0], nums[1], nums[2], nums[3]);
    if w > 0.0 && h > 0.0 && w.is_finite() && h.is_finite() {
        Some((minx, miny, w, h))
    } else {
        None
    }
}

enum RawLength {
    Absent,
    Value(f64),
    Percent(f64),
}

fn parse_raw_length(raw: &str) -> RawLength {
    let s = raw.trim();
    if s.is_empty() {
        return RawLength::Absent;
    }
    let (number_part, unit) = split_unit(s);
    let Ok(n) = number_part.parse::<f64>() else {
        return RawLength::Absent;
    };
    if !n.is_finite() {
        return RawLength::Absent;
    }
    match unit {
        Unit::Percent => RawLength::Percent(n),
        Unit::Px => RawLength::Value(n),
        Unit::Pt => RawLength::Value(n * 96.0 / 72.0),
        Unit::Pc => RawLength::Value(n * 16.0),
        Unit::In => RawLength::Value(n * 96.0),
        Unit::Cm => RawLength::Value(n * 96.0 / 2.54),
        Unit::Mm => RawLength::Value(n * 96.0 / 25.4),
        Unit::Unrecognized => RawLength::Absent,
    }
}

enum Unit {
    Percent,
    Px,
    Pt,
    Pc,
    In,
    Cm,
    Mm,
    Unrecognized,
}

/// 📏 Absolute CSS units at the standard 96dpi reference (matches `usvg`'s own conversion).
fn split_unit(s: &str) -> (&str, Unit) {
    if let Some(n) = s.strip_suffix('%') {
        return (n, Unit::Percent);
    }
    for (suffix, unit) in [("px", Unit::Px), ("pt", Unit::Pt), ("pc", Unit::Pc), ("in", Unit::In), ("cm", Unit::Cm), ("mm", Unit::Mm)] {
        if let Some(n) = s.strip_suffix(suffix) {
            return (n, unit);
        }
    }
    if s.chars().next_back().is_some_and(|c| c.is_ascii_alphabetic()) {
        return (s, Unit::Unrecognized);
    }
    (s, Unit::Px)
}

fn resolve_length(raw: Option<&str>, container: f64) -> Result<f64, IntrinsicSizeError> {
    let parsed = match raw {
        None => RawLength::Absent,
        Some(r) => parse_raw_length(r),
    };
    match parsed {
        RawLength::Absent => Ok(container),
        RawLength::Value(v) => {
            if v > 0.0 && v.is_finite() {
                Ok(v)
            } else {
                Err(IntrinsicSizeError::InvalidDimensions)
            }
        }
        RawLength::Percent(p) => {
            if p > 0.0 && p.is_finite() {
                Ok(p / 100.0 * container)
            } else {
                Err(IntrinsicSizeError::InvalidDimensions)
            }
        }
    }
}

//#endregion 🔖️Svg

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
