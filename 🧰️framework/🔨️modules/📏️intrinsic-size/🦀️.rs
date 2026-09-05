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
    let (container_w, container_h) = view_box.map(|(_, _, w, h)| (w, h)).unwrap_or((100.0, 100.0));
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
    if s.chars().next_back().map(|c| c.is_ascii_alphabetic()).unwrap_or(false) {
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
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Corpus {
        #[serde(rename = "svgCases")]
        svg_cases: Vec<SvgCase>,
        #[serde(rename = "rasterCases")]
        raster_cases: Vec<RasterCase>,
    }

    #[derive(Deserialize)]
    struct SvgCase {
        name: String,
        svg: String,
        width: Option<f64>,
        height: Option<f64>,
        #[serde(default)]
        error: bool,
    }

    #[derive(Deserialize)]
    struct RasterCase {
        name: String,
        width: u32,
        height: u32,
        bytes: Vec<u8>,
    }

    fn corpus() -> Corpus {
        serde_json::from_str(include_str!("🧪️tests/🔣️.json")).expect("valid intrinsic-size fixture corpus")
    }

    #[test]
    fn svg_fixture_corpus_matches_recorded_usvg_derived_expectations() {
        let c = corpus();
        assert!(!c.svg_cases.is_empty());
        for case in &c.svg_cases {
            let got = svg_intrinsic_size(&case.svg);
            if case.error {
                assert!(got.is_err(), "{}: expected error, got {got:?}", case.name);
            } else {
                let (w, h) = got.unwrap_or_else(|e| panic!("{}: expected Ok, got {e:?}", case.name));
                assert!((w - case.width.unwrap()).abs() < 1e-6, "{}: width {w} != {:?}", case.name, case.width);
                assert!((h - case.height.unwrap()).abs() < 1e-6, "{}: height {h} != {:?}", case.name, case.height);
            }
        }
        println!("svg fixture corpus: {}/{} matched", c.svg_cases.len(), c.svg_cases.len());
    }

    #[test]
    fn raster_fixture_corpus_matches_recorded_image_crate_derived_expectations() {
        let c = corpus();
        assert!(!c.raster_cases.is_empty());
        for case in &c.raster_cases {
            let (w, h) = raster_dimensions(&case.bytes).unwrap_or_else(|e| panic!("{}: {e:?}", case.name));
            assert_eq!((w, h), (case.width, case.height), "{}", case.name);
        }
        println!("raster fixture corpus: {}/{} matched", c.raster_cases.len(), c.raster_cases.len());
    }

    fn fixture_image(w: u32, h: u32) -> image::RgbaImage {
        image::RgbaImage::from_fn(w, h, |x, y| image::Rgba([(x % 256) as u8, (y % 256) as u8, ((x + y) % 256) as u8, 255]))
    }

    const DIMS: &[(u32, u32)] = &[(1, 1), (1, 7), (7, 1), (2, 3), (16, 16), (17, 9), (64, 64), (100, 1), (1, 100), (33, 257), (513, 129), (300, 200)];

    #[test]
    fn png_oracle_matches_image_crate_across_corpus() {
        use image::{GenericImageView, ImageEncoder};
        let mut checked = 0;
        for &(w, h) in DIMS {
            let img = fixture_image(w, h);
            let mut bytes = Vec::new();
            image::codecs::png::PngEncoder::new(&mut bytes).write_image(&img, w, h, image::ExtendedColorType::Rgba8).expect("encode png");
            let oracle = image::load_from_memory(&bytes).expect("oracle decode png");
            let ours = raster_dimensions(&bytes).expect("our png dims");
            assert_eq!(ours, oracle.dimensions(), "png {w}x{h} mismatch");
            assert_eq!(ours, (w, h));
            checked += 1;
        }
        println!("png oracle: {checked}/{} matched image crate", DIMS.len());
        assert_eq!(checked, DIMS.len());
    }

    #[test]
    fn jpeg_oracle_matches_image_crate_across_corpus() {
        use image::GenericImageView;
        let mut checked = 0;
        for &(w, h) in DIMS {
            let img = fixture_image(w, h);
            let mut bytes = Vec::new();
            image::codecs::jpeg::JpegEncoder::new(&mut bytes).encode_image(&img).expect("encode jpeg");
            let oracle = image::load_from_memory(&bytes).expect("oracle decode jpeg");
            let ours = raster_dimensions(&bytes).expect("our jpeg dims");
            assert_eq!(ours, oracle.dimensions(), "jpeg {w}x{h} mismatch");
            assert_eq!(ours, (w, h));
            checked += 1;
        }
        println!("jpeg oracle: {checked}/{} matched image crate", DIMS.len());
        assert_eq!(checked, DIMS.len());
    }

    #[test]
    fn gif_oracle_matches_image_crate_across_corpus() {
        use image::GenericImageView;
        let mut checked = 0;
        for &(w, h) in DIMS {
            let img = fixture_image(w, h);
            let mut bytes = Vec::new();
            {
                let mut enc = image::codecs::gif::GifEncoder::new(&mut bytes);
                enc.encode(&img, w, h, image::ExtendedColorType::Rgba8).expect("encode gif");
            }
            let oracle = image::load_from_memory(&bytes).expect("oracle decode gif");
            let ours = raster_dimensions(&bytes).expect("our gif dims");
            assert_eq!(ours, oracle.dimensions(), "gif {w}x{h} mismatch");
            assert_eq!(ours, (w, h));
            checked += 1;
        }
        println!("gif oracle: {checked}/{} matched image crate", DIMS.len());
        assert_eq!(checked, DIMS.len());
    }

    #[test]
    fn webp_lossless_oracle_matches_image_crate_across_corpus() {
        use image::GenericImageView;
        let mut checked = 0;
        for &(w, h) in DIMS {
            let img = fixture_image(w, h);
            let mut bytes = Vec::new();
            image::codecs::webp::WebPEncoder::new_lossless(&mut bytes).encode(&img, w, h, image::ExtendedColorType::Rgba8).expect("encode webp");
            assert_eq!(&bytes[12..16], b"VP8L", "expected a lossless VP8L chunk from the encoder");
            let oracle = image::load_from_memory(&bytes).expect("oracle decode webp");
            let ours = raster_dimensions(&bytes).expect("our webp dims");
            assert_eq!(ours, oracle.dimensions(), "webp {w}x{h} mismatch");
            assert_eq!(ours, (w, h));
            checked += 1;
        }
        println!("webp (VP8L) oracle: {checked}/{} matched image crate", DIMS.len());
        assert_eq!(checked, DIMS.len());
    }

    /// 🧩 `image`'s webp encoder only emits lossless `VP8L`; no lossy/`VP8X` encoder was available
    /// to generate a full decodable bitstream as a third-party oracle (a synthetic bitstream with
    /// no real coefficient data fails `image`'s own decode, which performs a full pixel decode, not
    /// a header read). Verified instead as hand-built, spec-conformant fixtures against the WebP
    /// RIFF container spec's own documented byte layout — the same "hand-built known-good block"
    /// technique already used for this ticket's DEFLATE module's stored-block (BTYPE=00) test.
    #[test]
    fn webp_lossy_and_extended_headers_match_spec_hand_built_fixtures() {
        let mut vp8 = Vec::new();
        vp8.extend_from_slice(b"RIFF");
        vp8.extend_from_slice(&0u32.to_le_bytes());
        vp8.extend_from_slice(b"WEBP");
        vp8.extend_from_slice(b"VP8 ");
        vp8.extend_from_slice(&0u32.to_le_bytes());
        vp8.extend_from_slice(&[0x30, 0x01, 0x00]);
        vp8.extend_from_slice(&[0x9D, 0x01, 0x2A]);
        let w: u16 = 200;
        let h: u16 = 100;
        vp8.extend_from_slice(&(w & 0x3FFF).to_le_bytes());
        vp8.extend_from_slice(&(h & 0x3FFF).to_le_bytes());
        assert_eq!(raster_dimensions(&vp8), Ok((200, 100)));

        let mut vp8l = Vec::new();
        vp8l.extend_from_slice(b"RIFF");
        vp8l.extend_from_slice(&0u32.to_le_bytes());
        vp8l.extend_from_slice(b"WEBP");
        vp8l.extend_from_slice(b"VP8L");
        vp8l.extend_from_slice(&0u32.to_le_bytes());
        let width_m1: u32 = 319;
        let height_m1: u32 = 149;
        let mut packed: u32 = width_m1 & 0x3FFF;
        packed |= (height_m1 & 0x3FFF) << 14;
        vp8l.push(0x2F);
        vp8l.extend_from_slice(&packed.to_le_bytes());
        assert_eq!(raster_dimensions(&vp8l), Ok((320, 150)));

        let mut vp8x = Vec::new();
        vp8x.extend_from_slice(b"RIFF");
        vp8x.extend_from_slice(&0u32.to_le_bytes());
        vp8x.extend_from_slice(b"WEBP");
        vp8x.extend_from_slice(b"VP8X");
        vp8x.extend_from_slice(&10u32.to_le_bytes());
        vp8x.push(0x00);
        vp8x.extend_from_slice(&[0, 0, 0]);
        let cw_m1: u32 = 639;
        let ch_m1: u32 = 479;
        vp8x.push((cw_m1 & 0xFF) as u8);
        vp8x.push(((cw_m1 >> 8) & 0xFF) as u8);
        vp8x.push(((cw_m1 >> 16) & 0xFF) as u8);
        vp8x.push((ch_m1 & 0xFF) as u8);
        vp8x.push(((ch_m1 >> 8) & 0xFF) as u8);
        vp8x.push(((ch_m1 >> 16) & 0xFF) as u8);
        assert_eq!(raster_dimensions(&vp8x), Ok((640, 480)));

        println!("webp VP8/VP8X hand-built spec fixtures: 3/3 matched documented byte layout");
    }

    #[test]
    fn raster_dimensions_rejects_malformed_and_truncated_input() {
        assert!(raster_dimensions(&[]).is_err());
        assert!(raster_dimensions(&[0x00, 0x01, 0x02]).is_err());
        assert!(raster_dimensions(&PNG_SIGNATURE).is_err());
        assert!(raster_dimensions(b"GIF89a").is_err());
        assert!(raster_dimensions(&[0xFF, 0xD8]).is_err());
    }

    fn usvg_oracle_size(svg: &str) -> Option<(f64, f64)> {
        let opt = usvg::Options::default();
        let tree = usvg::Tree::from_str(svg, &opt).ok()?;
        let s = tree.size();
        Some((f64::from(s.width()), f64::from(s.height())))
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.01
    }

    #[test]
    fn svg_oracle_matches_usvg_live_across_fixture_corpus() {
        let c = corpus();
        let mut checked = 0;
        let mut hard_fail_agrees = 0;
        for case in &c.svg_cases {
            let oracle = usvg_oracle_size(&case.svg);
            let ours = svg_intrinsic_size(&case.svg);
            match (oracle, ours) {
                (Some((ow, oh)), Ok((w, h))) => {
                    assert!(close(ow, w) && close(oh, h), "{}: oracle=({ow},{oh}) ours=({w},{h})", case.name);
                    checked += 1;
                }
                (None, Err(_)) => hard_fail_agrees += 1,
                other => panic!("{}: disagreement with usvg oracle: {other:?}", case.name),
            }
        }
        println!("svg live usvg oracle: {checked} numeric matches + {hard_fail_agrees} agreed hard failures / {} cases", c.svg_cases.len());
        assert_eq!(checked + hard_fail_agrees, c.svg_cases.len());
    }
}
