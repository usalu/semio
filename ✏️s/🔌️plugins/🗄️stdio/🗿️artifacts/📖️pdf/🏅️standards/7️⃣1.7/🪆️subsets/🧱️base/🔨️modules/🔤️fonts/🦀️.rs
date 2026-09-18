//! 🔤️ Fonts (ISO 32000-1 §9): the standard 14 metrics, Annex D encodings and the Adobe Glyph
//! List (`🅰️tables`), TrueType programs (`🔠️truetype`), CMaps (`🗺️cmap`), text-string codecs
//! (§7.9.2), and [`FontCodec`] — the one place that turns a typed [`PdfFont`] into "these bytes
//! show this text at these widths" and back.

use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfBaseEncoding, PdfCMap, PdfFont, PdfFontKind, PdfFontProgram, PdfSimpleEncoding, PdfToUnicode, PdfToUnicodeMapping};
use std::collections::{BTreeMap, HashMap};
use std::sync::OnceLock;

#[path = "🅰️tables/🦀️.rs"]
pub mod tables;
#[path = "🔠️truetype/🦀️.rs"]
pub mod truetype;
#[path = "🗺️cmap/🦀️.rs"]
pub mod cmap;

pub use tables::{StandardFontMetrics, ADOBE_GLYPH_LIST, MAC_ROMAN_ENCODING, PDF_DOC_ENCODING, STANDARD_ENCODING, STANDARD_FONTS, WIN_ANSI_ENCODING};
pub use truetype::TrueTypeFont;

//#region 🔖️GlyphList
/// 🔤 Unicode text of a glyph name: the Adobe Glyph List, `uniXXXX[XXXX…]`, `uXXXX[XX]`, ligature
/// components joined by `_`, with `.suffix` variants stripped (AGL specification §3).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn glyph_name_to_unicode(name: &str) -> Option<String> {
    let base = name.split('.').next().unwrap_or(name);
    if base.is_empty() {
        return None;
    }
    if base.contains('_') {
        let mut out = String::new();
        for part in base.split('_') {
            out.push_str(&glyph_name_to_unicode(part)?);
        }
        return Some(out);
    }
    if let Ok(index) = ADOBE_GLYPH_LIST.binary_search_by(|(candidate, _)| candidate.cmp(&base)) {
        return Some(ADOBE_GLYPH_LIST[index].1.to_string());
    }
    if let Some(hex) = base.strip_prefix("uni") {
        if hex.len() >= 4 && hex.len() % 4 == 0 && hex.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_lowercase()) {
            let mut out = String::new();
            for chunk in hex.as_bytes().chunks(4) {
                let value = u32::from_str_radix(std::str::from_utf8(chunk).ok()?, 16).ok()?;
                out.push(char::from_u32(value)?);
            }
            return Some(out);
        }
    }
    if let Some(hex) = base.strip_prefix('u') {
        if (4..=6).contains(&hex.len()) && hex.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_lowercase()) {
            return char::from_u32(u32::from_str_radix(hex, 16).ok()?).map(|c| c.to_string());
        }
    }
    None
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reverse_glyph_list() -> &'static HashMap<u32, &'static str> {
    static REVERSE: OnceLock<HashMap<u32, &'static str>> = OnceLock::new();
    REVERSE.get_or_init(|| {
        let scalar = |text: &'static str| -> Option<u32> {
            let mut chars = text.chars();
            match (chars.next(), chars.next()) {
                (Some(only), None) => Some(only as u32),
                _ => None,
            }
        };
        let mut map: HashMap<u32, &'static str> = HashMap::new();
        for table in [STANDARD_ENCODING, WIN_ANSI_ENCODING, MAC_ROMAN_ENCODING] {
            for (_, name) in table {
                if let Ok(index) = ADOBE_GLYPH_LIST.binary_search_by(|(candidate, _)| candidate.cmp(name)) {
                    if let Some(value) = scalar(ADOBE_GLYPH_LIST[index].1) {
                        map.entry(value).or_insert(name);
                    }
                }
            }
        }
        for (name, text) in ADOBE_GLYPH_LIST {
            let Some(value) = scalar(text) else { continue };
            match map.get(&value) {
                Some(existing) if existing.len() <= name.len() => {}
                _ => {
                    map.insert(value, name);
                }
            }
        }
        map
    })
}

/// 🔤 The AGL glyph name of a character (`uniXXXX` when the list has none).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unicode_to_glyph_name(character: char) -> String {
    reverse_glyph_list().get(&(character as u32)).map(|name| name.to_string()).unwrap_or_else(|| format!("uni{:04X}", character as u32))
}
//#endregion 🔖️GlyphList

//#region 🔖️Encodings
/// 🔡 The Annex D table of a base encoding.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn base_encoding_table(base: PdfBaseEncoding) -> &'static [(u8, &'static str)] {
    match base {
        PdfBaseEncoding::Standard | PdfBaseEncoding::MacExpert => STANDARD_ENCODING,
        PdfBaseEncoding::WinAnsi => WIN_ANSI_ENCODING,
        PdfBaseEncoding::MacRoman => MAC_ROMAN_ENCODING,
    }
}

/// 🔡 Parses a base-encoding name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn base_encoding_from_name(name: &str) -> Option<PdfBaseEncoding> {
    match name {
        "StandardEncoding" => Some(PdfBaseEncoding::Standard),
        "WinAnsiEncoding" => Some(PdfBaseEncoding::WinAnsi),
        "MacRomanEncoding" => Some(PdfBaseEncoding::MacRoman),
        "MacExpertEncoding" => Some(PdfBaseEncoding::MacExpert),
        _ => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn base_encoding_name(base: PdfBaseEncoding) -> &'static str {
    match base {
        PdfBaseEncoding::Standard => "StandardEncoding",
        PdfBaseEncoding::WinAnsi => "WinAnsiEncoding",
        PdfBaseEncoding::MacRoman => "MacRomanEncoding",
        PdfBaseEncoding::MacExpert => "MacExpertEncoding",
    }
}

/// 🔤 Decodes a PDF text string (§7.9.2.2): UTF-16BE with BOM, UTF-8 with BOM, else PDFDoc.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_text_string(bytes: &[u8]) -> String {
    if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
        let units: Vec<u16> = bytes[2..].chunks(2).filter(|c| c.len() == 2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();
        return String::from_utf16_lossy(&units);
    }
    if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
        return String::from_utf8_lossy(&bytes[3..]).into_owned();
    }
    bytes.iter().map(|byte| PDF_DOC_ENCODING.binary_search_by(|(code, _)| code.cmp(byte)).ok().and_then(|index| char::from_u32(PDF_DOC_ENCODING[index].1)).unwrap_or('\u{FFFD}')).collect()
}

/// 🔤 Encodes a text string: PDFDoc when every character is representable, else UTF-16BE + BOM.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_text_string(text: &str) -> Vec<u8> {
    static REVERSE: OnceLock<HashMap<u32, u8>> = OnceLock::new();
    let reverse = REVERSE.get_or_init(|| PDF_DOC_ENCODING.iter().map(|(code, value)| (*value, *code)).collect());
    let mut out = Vec::with_capacity(text.len());
    for character in text.chars() {
        match reverse.get(&(character as u32)) {
            Some(code) if character != '\u{FFFD}' => out.push(*code),
            _ => {
                let mut utf16 = vec![0xFE, 0xFF];
                for unit in text.encode_utf16() {
                    utf16.extend_from_slice(&unit.to_be_bytes());
                }
                return utf16;
            }
        }
    }
    out
}
//#endregion 🔖️Encodings

//#region 🔖️Standard14
/// 🅰️ Normalizes a base font name onto one of the standard 14 (subset prefixes stripped, the
/// usual aliases — Arial, TimesNewRoman, CourierNew and their style suffixes — folded).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn standard_font_name(base_font: &str) -> Option<&'static str> {
    let name = base_font.split_once('+').filter(|(prefix, _)| prefix.len() == 6 && prefix.bytes().all(|b| b.is_ascii_uppercase())).map_or(base_font, |(_, rest)| rest);
    if let Some(font) = STANDARD_FONTS.iter().find(|font| font.name == name) {
        return Some(font.name);
    }
    let lower = name.to_ascii_lowercase().replace([' ', '-', '_', ','], "");
    let bold = lower.contains("bold") || lower.contains("black") || lower.contains("heavy") || lower.contains("semibold");
    let italic = lower.contains("italic") || lower.contains("oblique");
    let family = if lower.starts_with("courier") || lower.contains("mono") {
        "Courier"
    } else if lower.starts_with("times") || lower.contains("serif") && !lower.contains("sans") || lower.starts_with("georgia") || lower.starts_with("book") {
        "Times"
    } else if lower.starts_with("symbol") {
        return Some("Symbol");
    } else if lower.starts_with("zapf") || lower.starts_with("dingbat") {
        return Some("ZapfDingbats");
    } else if lower.starts_with("helvetica") || lower.starts_with("arial") || lower.contains("sans") || lower.starts_with("verdana") || lower.starts_with("calibri") || lower.starts_with("segoe") {
        "Helvetica"
    } else {
        return None;
    };
    Some(match (family, bold, italic) {
        ("Courier", false, false) => "Courier",
        ("Courier", true, false) => "Courier-Bold",
        ("Courier", false, true) => "Courier-Oblique",
        ("Courier", true, true) => "Courier-BoldOblique",
        ("Times", false, false) => "Times-Roman",
        ("Times", true, false) => "Times-Bold",
        ("Times", false, true) => "Times-Italic",
        ("Times", true, true) => "Times-BoldItalic",
        (_, false, false) => "Helvetica",
        (_, true, false) => "Helvetica-Bold",
        (_, false, true) => "Helvetica-Oblique",
        _ => "Helvetica-BoldOblique",
    })
}

/// 🅰️ Metrics of a standard-14 font (aliases folded).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn standard_font(base_font: &str) -> Option<&'static StandardFontMetrics> {
    let name = standard_font_name(base_font)?;
    STANDARD_FONTS.iter().find(|font| font.name == name)
}

impl StandardFontMetrics {
    /// 📏 Advance width of a glyph name (AFM units).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn width(&self, glyph: &str) -> Option<f64> {
        self.widths.binary_search_by(|(name, _)| (*name).cmp(glyph)).ok().map(|index| self.widths[index].1 as f64)
    }
    /// 🏳️ `/Flags` of a font descriptor for this face (§9.8.2).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn flags(&self) -> u32 {
        let mut flags = 0u32;
        if self.fixed_pitch {
            flags |= 1;
        }
        if self.family.starts_with("Times") {
            flags |= 2;
        }
        flags |= if self.symbolic { 4 } else { 32 };
        if self.italic_angle != 0.0 {
            flags |= 64;
        }
        if self.bold {
            flags |= 1 << 18;
        }
        flags
    }
}
//#endregion 🔖️Standard14

//#region 🔖️Type1
/// 🔡 The built-in encoding of a Type 1 program's clear-text portion (`dup <code> /<name> put`,
/// or `StandardEncoding`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn type1_builtin_encoding(program: &[u8]) -> Option<Vec<(u8, String)>> {
    let clear_end = program.windows(9).position(|w| w == b"eexec\r\n" || w == b"eexec\r\r\n" || w.starts_with(b"eexec")).unwrap_or(program.len());
    let text = String::from_utf8_lossy(&program[..clear_end]);
    let encoding_at = text.find("/Encoding")?;
    let rest = &text[encoding_at + 9..];
    if rest.trim_start().starts_with("StandardEncoding") {
        return Some(STANDARD_ENCODING.iter().map(|(code, name)| (*code, name.to_string())).collect());
    }
    let end = rest.find(" readonly def").or_else(|| rest.find(" def")).unwrap_or(rest.len());
    let mut out = Vec::new();
    for line in rest[..end].split("dup ").skip(1) {
        let mut parts = line.split_whitespace();
        let (Some(code), Some(name)) = (parts.next(), parts.next()) else { continue };
        if let (Ok(code), Some(name)) = (code.parse::<u32>(), name.strip_prefix('/')) {
            if code < 256 {
                out.push((code as u8, name.to_string()));
            }
        }
    }
    Some(out)
}
//#endregion 🔖️Type1

//#region 🔖️FontCodec
/// 🔠 One shown glyph: its character code, advance in 1/1000 text space (before size, spacing
/// and horizontal scaling), the glyph id it selects in an embedded TrueType program (when known),
/// and the Unicode text it stands for (when known).
#[derive(Clone, Debug, PartialEq)]
pub struct DecodedGlyph {
    pub code: u32,
    pub code_width: u32,
    pub width: f64,
    pub glyph_id: Option<u16>,
    pub cid: u32,
    pub text: Option<String>,
    pub is_space_char: bool,
}

/// 🔤 Everything needed to show text with one font in both directions.
#[derive(Clone, Debug)]
pub struct FontCodec {
    composite: bool,
    cid_codec: Option<cmap::CidCodec>,
    code_to_text: BTreeMap<u32, String>,
    text_to_code: HashMap<String, (u32, u32)>,
    simple_widths: BTreeMap<u32, f64>,
    default_width: f64,
    cid_widths: BTreeMap<u32, f64>,
    cid_to_gid: BTreeMap<u32, u16>,
    cid_to_gid_identity: bool,
    program: Option<TrueTypeFont>,
    code_to_gid: BTreeMap<u32, u16>,
    to_unicode: Option<PdfToUnicode>,
    pub type3_matrix: Option<[f64; 6]>,
}

impl FontCodec {
    /// 🏗️ Builds the codec for `font`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(font: &PdfFont) -> Self {
        let mut codec = Self { composite: false, cid_codec: None, code_to_text: BTreeMap::new(), text_to_code: HashMap::new(), simple_widths: BTreeMap::new(), default_width: 0.0, cid_widths: BTreeMap::new(), cid_to_gid: BTreeMap::new(), cid_to_gid_identity: true, program: None, code_to_gid: BTreeMap::new(), to_unicode: font.to_unicode.clone(), type3_matrix: None };
        match &font.kind {
            PdfFontKind::Type1 { base_font, encoding, first_char, widths, descriptor, program } => {
                let symbolic = descriptor.as_ref().is_some_and(|d| d.flags & 4 != 0 && d.flags & 32 == 0);
                let builtin: Vec<(u8, String)> = match program {
                    Some(PdfFontProgram::Type1 { data, .. }) => type1_builtin_encoding(data).unwrap_or_default(),
                    _ => standard_font(base_font).filter(|f| f.symbolic).map(|f| f.builtin_encoding.iter().map(|(c, n)| (*c, n.to_string())).collect()).unwrap_or_default(),
                };
                let names = simple_names(encoding, &builtin, symbolic && !builtin.is_empty());
                codec.install_simple(&names, *first_char, widths, standard_font(base_font), descriptor.as_ref().and_then(|d| d.missing_width), symbolic || standard_font(base_font).is_some_and(|f| f.symbolic));
            }
            PdfFontKind::TrueType { base_font, encoding, first_char, widths, descriptor, program } => {
                let symbolic = descriptor.as_ref().is_some_and(|d| d.flags & 4 != 0 && d.flags & 32 == 0);
                let parsed = match program {
                    Some(PdfFontProgram::TrueType { data }) | Some(PdfFontProgram::OpenType { data }) => TrueTypeFont::parse(data).ok(),
                    _ => None,
                };
                let names = simple_names(encoding, &[], false);
                if let Some(ttf) = &parsed {
                    for code in 0u32..256 {
                        let gid = truetype_simple_gid(ttf, code, names.get(&code).map(String::as_str), symbolic || encoding.base.is_none() && encoding.differences.is_empty());
                        if let Some(gid) = gid {
                            codec.code_to_gid.insert(code, gid);
                        }
                    }
                }
                let names = if symbolic && encoding.base.is_none() && encoding.differences.is_empty() {
                    // 🔣 A symbolic TrueType font with no /Encoding: text is whatever the ToUnicode says.
                    BTreeMap::new()
                } else {
                    names
                };
                codec.install_simple(&names, *first_char, widths, standard_font(base_font), descriptor.as_ref().and_then(|d| d.missing_width), symbolic);
                if let Some(ttf) = &parsed {
                    if widths.is_empty() {
                        for (code, gid) in &codec.code_to_gid {
                            codec.simple_widths.insert(*code, ttf.advance_1000(*gid));
                        }
                    }
                }
                codec.program = parsed;
            }
            PdfFontKind::Type3 { font_matrix, encoding, first_char, widths, .. } => {
                let names = simple_names(encoding, &[], true);
                codec.install_simple(&names, *first_char, widths, None, None, false);
                codec.type3_matrix = Some(*font_matrix);
            }
            PdfFontKind::Type0 { cmap: encoding, descendant, .. } => {
                codec.composite = true;
                codec.cid_codec = Some(match encoding {
                    PdfCMap::Predefined { name } => cmap::CidCodec::identity(name.ends_with("-V")),
                    PdfCMap::Embedded { cmap } => cmap::CidCodec::embedded(cmap),
                });
                codec.default_width = descendant.default_width;
                for run in &descendant.widths {
                    for (offset, width) in run.widths.iter().enumerate() {
                        codec.cid_widths.insert(run.start_cid + offset as u32, *width);
                    }
                }
                match &descendant.cid_to_gid {
                    Some(crate::standards::v1_7::subsets::base::schema::snapshot::PdfCidToGid::Map { data }) => {
                        codec.cid_to_gid_identity = false;
                        for (cid, pair) in data.chunks(2).enumerate() {
                            if pair.len() == 2 {
                                codec.cid_to_gid.insert(cid as u32, u16::from_be_bytes([pair[0], pair[1]]));
                            }
                        }
                    }
                    _ => codec.cid_to_gid_identity = true,
                }
                codec.program = match &descendant.program {
                    Some(PdfFontProgram::TrueType { data }) | Some(PdfFontProgram::OpenType { data }) => TrueTypeFont::parse(data).ok(),
                    _ => None,
                };
            }
        }
        if let Some(map) = &font.to_unicode {
            for mapping in &map.mappings {
                match mapping {
                    PdfToUnicodeMapping::Char { code, text } => {
                        codec.code_to_text.insert(*code, text.clone());
                    }
                    PdfToUnicodeMapping::Range { low, high, text } => {
                        for code in *low..=(*high).min(low + 65535) {
                            if let Some(text) = cmap::to_unicode_lookup(map, code) {
                                codec.code_to_text.insert(code, text);
                            }
                            let _ = text;
                        }
                    }
                }
            }
            let byte_width = map.byte_width;
            for (code, text) in &codec.code_to_text {
                codec.text_to_code.entry(text.clone()).or_insert((*code, byte_width));
            }
        }
        if codec.composite && codec.text_to_code.is_empty() {
            if let (Some(program), Some(cid_codec)) = (&codec.program, &codec.cid_codec) {
                if codec.cid_to_gid_identity {
                    for (unicode, gid) in &program.unicode_map {
                        if let (Some(character), Some((code, width))) = (char::from_u32(*unicode), cid_codec.code_for_cid(*gid as u32)) {
                            let text = character.to_string();
                            codec.text_to_code.entry(text.clone()).or_insert((code, width));
                            codec.code_to_text.entry(code).or_insert(text);
                        }
                    }
                }
            }
        }
        codec
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn install_simple(&mut self, names: &BTreeMap<u32, String>, first_char: u32, widths: &[f64], standard: Option<&StandardFontMetrics>, missing_width: Option<f64>, symbolic: bool) {
        for (code, name) in names {
            if let Some(text) = glyph_name_to_unicode(name) {
                self.text_to_code.entry(text.clone()).or_insert((*code, 1));
                self.code_to_text.insert(*code, text);
            } else if (0x20..=0x7E).contains(code) && name != ".notdef" {
                // 🔤 A glyph name outside every list (bitmap `aNN` names of TeX Type 3 fonts, …):
                // the code is its own Unicode, the fallback poppler and MuPDF apply as well.
                let text = (*code as u8 as char).to_string();
                self.text_to_code.entry(text.clone()).or_insert((*code, 1));
                self.code_to_text.insert(*code, text);
            }
        }
        if !symbolic {
            for code in 0x20u32..=0x7E {
                if !self.code_to_text.contains_key(&code) && !names.contains_key(&code) {
                    let text = (code as u8 as char).to_string();
                    self.text_to_code.entry(text.clone()).or_insert((code, 1));
                    self.code_to_text.insert(code, text);
                }
            }
        }
        self.default_width = missing_width.unwrap_or(0.0);
        if widths.is_empty() {
            if let Some(standard) = standard {
                for (code, name) in names {
                    if let Some(width) = standard.width(name) {
                        self.simple_widths.insert(*code, width);
                    }
                }
            }
        } else {
            for (offset, width) in widths.iter().enumerate() {
                self.simple_widths.insert(first_char + offset as u32, *width);
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_composite(&self) -> bool {
        self.composite
    }

    /// 📖 Splits `bytes` into the glyphs they show.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn decode(&self, bytes: &[u8]) -> Vec<DecodedGlyph> {
        let codes: Vec<(u32, u32)> = match &self.cid_codec {
            Some(cid_codec) => cid_codec.split(bytes),
            None => bytes.iter().map(|byte| (*byte as u32, 1)).collect(),
        };
        codes
            .into_iter()
            .map(|(code, code_width)| {
                let cid = self.cid_codec.as_ref().map_or(code, |c| c.cid(code));
                let width = if self.composite { self.cid_widths.get(&cid).copied().unwrap_or(self.default_width) } else { self.simple_widths.get(&code).copied().unwrap_or(self.default_width) };
                let glyph_id = if self.composite {
                    if self.cid_to_gid_identity {
                        u16::try_from(cid).ok()
                    } else {
                        self.cid_to_gid.get(&cid).copied()
                    }
                } else {
                    self.code_to_gid.get(&code).copied()
                };
                let text = self.code_to_text.get(&code).cloned();
                DecodedGlyph { code, code_width, width, glyph_id, cid, is_space_char: code == 32 && code_width == 1, text }
            })
            .collect()
    }

    /// 🔤 The Unicode text `bytes` show, when every code is mapped.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn decode_text(&self, bytes: &[u8]) -> Option<String> {
        let glyphs = self.decode(bytes);
        let mut out = String::new();
        for glyph in glyphs {
            out.push_str(glyph.text.as_deref()?);
        }
        Some(out)
    }

    /// ✍️ Encodes `text` into the string operand that shows it, when the font can show every
    /// character.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn encode(&self, text: &str) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        let mut cursor = text;
        while !cursor.is_empty() {
            let mut matched = false;
            // 🧵 Longest multi-character mapping first (ligatures), then single characters.
            for length in (1..=cursor.chars().count().min(4)).rev() {
                let candidate: String = cursor.chars().take(length).collect();
                if let Some((code, width)) = self.text_to_code.get(&candidate) {
                    if self.composite {
                        cmap::CidCodec::push_code(&mut out, *code, *width);
                    } else {
                        out.push(*code as u8);
                    }
                    cursor = &cursor[candidate.len()..];
                    matched = true;
                    break;
                }
            }
            if !matched {
                return None;
            }
        }
        Some(out)
    }

    /// 📏 Width of a code in 1/1000 text space.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn width_of_code(&self, code: u32) -> f64 {
        if self.composite {
            let cid = self.cid_codec.as_ref().map_or(code, |c| c.cid(code));
            self.cid_widths.get(&cid).copied().unwrap_or(self.default_width)
        } else {
            self.simple_widths.get(&code).copied().unwrap_or(self.default_width)
        }
    }

    /// 📏 Width of a text run in 1/1000 text space, when encodable.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn text_width(&self, text: &str) -> Option<f64> {
        let bytes = self.encode(text)?;
        Some(self.decode(&bytes).iter().map(|glyph| glyph.width).sum())
    }

    /// 🔠 The embedded TrueType program, when the font carries one.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn program(&self) -> Option<&TrueTypeFont> {
        self.program.as_ref()
    }

    /// 🈴 The typed `ToUnicode` of the font, if any.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_unicode(&self) -> Option<&PdfToUnicode> {
        self.to_unicode.as_ref()
    }
}

/// 🔡 The code → glyph-name table of a simple font (§9.6.6): base encoding (or the built-in one),
/// then `/Differences`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn simple_names(encoding: &PdfSimpleEncoding, builtin: &[(u8, String)], prefer_builtin: bool) -> BTreeMap<u32, String> {
    let mut names: BTreeMap<u32, String> = BTreeMap::new();
    match encoding.base {
        Some(base) if !prefer_builtin || builtin.is_empty() => {
            for (code, name) in base_encoding_table(base) {
                names.insert(*code as u32, name.to_string());
            }
        }
        _ if !builtin.is_empty() => {
            for (code, name) in builtin {
                names.insert(*code as u32, name.clone());
            }
        }
        _ => {
            for (code, name) in STANDARD_ENCODING {
                names.insert(*code as u32, name.to_string());
            }
        }
    }
    for difference in &encoding.differences {
        names.insert(difference.code, difference.glyph.clone());
    }
    names
}

/// 🔢 Glyph id a simple TrueType font shows for `code` (§9.6.6.4).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn truetype_simple_gid(font: &TrueTypeFont, code: u32, name: Option<&str>, symbolic: bool) -> Option<u16> {
    if symbolic {
        if let Some(gid) = font.symbol_map.get(&(0xF000 + code)).or_else(|| font.symbol_map.get(&code)).or_else(|| font.symbol_map.get(&(0xF100 + code))).or_else(|| font.symbol_map.get(&(0xF200 + code))) {
            return Some(*gid);
        }
        if let Some(gid) = font.mac_roman_map.get(&(code as u8)) {
            return Some(*gid);
        }
    }
    if let Some(name) = name {
        if let Some(text) = glyph_name_to_unicode(name) {
            if let Some(character) = text.chars().next() {
                if let Some(gid) = font.unicode_map.get(&(character as u32)) {
                    return Some(*gid);
                }
                if let Some(gid) = font.symbol_map.get(&(0xF000 + (character as u32 & 0xFF))) {
                    return Some(*gid);
                }
            }
        }
        if let Some(gid) = font.glyph_for_name(name) {
            return Some(gid);
        }
    }
    if !symbolic {
        if let Some(gid) = font.symbol_map.get(&(0xF000 + code)).or_else(|| font.symbol_map.get(&code)) {
            return Some(*gid);
        }
        if let Some(gid) = font.unicode_map.get(&code) {
            return Some(*gid);
        }
    }
    if font.unicode_map.is_empty() && font.symbol_map.is_empty() && font.mac_roman_map.is_empty() {
        return u16::try_from(code).ok().filter(|gid| *gid < font.num_glyphs);
    }
    None
}
//#endregion 🔖️FontCodec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
