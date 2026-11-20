//! 🗺️ CMaps (ISO 32000-1 §9.7.5, §9.10.3): parsing and printing of embedded CMap streams
//! (`codespacerange`, `cidchar`, `cidrange`, `usecmap`) and `ToUnicode` CMaps (`bfchar`,
//! `bfrange` in both scalar and array forms), the predefined `Identity-H`/`-V` maps, and the
//! byte-sequence → code splitter every composite font's text showing relies on (§9.7.6.2).

use crate::standards::v1_7::subsets::base::modules::lexer::{Lexer, Token};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfCidMapping, PdfCodespaceRange, PdfEmbeddedCMap, PdfObject, PdfToUnicode, PdfToUnicodeMapping};

//#region 🔖️Parsing
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn code_of(bytes: &[u8]) -> u32 {
    bytes.iter().take(4).fold(0u32, |code, byte| (code << 8) | *byte as u32)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn utf16_text(bytes: &[u8]) -> String {
    if bytes.len() % 2 == 1 {
        return bytes.iter().map(|byte| *byte as char).collect();
    }
    let units: Vec<u16> = bytes.chunks(2).map(|pair| u16::from_be_bytes([pair[0], pair[1]])).collect();
    String::from_utf16_lossy(&units)
}

/// 🎫 Tokenizes a CMap body into (operands, operator) groups.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn statements(body: &[u8]) -> Vec<(Vec<PdfObject>, String)> {
    let mut lexer = Lexer::new(body);
    let mut operands = Vec::new();
    let mut out = Vec::new();
    loop {
        match lexer.next_token() {
            Ok(Token::Object(value)) => operands.push(value),
            Ok(Token::Keyword(word)) => {
                out.push((std::mem::take(&mut operands), word));
            }
            Ok(Token::End) | Err(_) => break,
        }
    }
    out
}

/// 📖 Parses an embedded CMap stream body. `name` falls back to `/CMapName` or `default`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_cmap(body: &[u8], default_name: &str) -> PdfEmbeddedCMap {
    let mut cmap = PdfEmbeddedCMap { name: default_name.to_string(), vertical: false, codespace: Vec::new(), mappings: Vec::new(), use_cmap: None };
    let mut pending: Vec<PdfObject> = Vec::new();
    let mut section: Option<&str> = None;
    let mut last_name: Option<String> = None;
    for (operands, operator) in statements(body) {
        match operator.as_str() {
            "def" => {
                if let [PdfObject::Name(key), value] = operands.as_slice() {
                    match (key.as_str(), value) {
                        ("CMapName", PdfObject::Name(name)) => cmap.name = name.clone(),
                        ("WMode", PdfObject::Int(mode)) => cmap.vertical = *mode == 1,
                        _ => {}
                    }
                }
            }
            "usecmap" => {
                cmap.use_cmap = match operands.last() {
                    Some(PdfObject::Name(name)) => Some(name.clone()),
                    _ => last_name.clone(),
                };
            }
            "begincodespacerange" | "begincidchar" | "begincidrange" | "beginbfchar" | "beginbfrange" | "beginnotdefrange" => {
                section = Some(match operator.as_str() {
                    "begincodespacerange" => "codespace",
                    "begincidchar" => "cidchar",
                    "begincidrange" => "cidrange",
                    "beginbfchar" => "bfchar",
                    "beginbfrange" => "bfrange",
                    _ => "notdef",
                });
                pending.clear();
            }
            "endcodespacerange" | "endcidchar" | "endcidrange" | "endbfchar" | "endbfrange" | "endnotdefrange" => {
                let mut items = std::mem::take(&mut pending);
                items.extend(operands);
                match section {
                    Some("codespace") => {
                        for pair in items.chunks(2) {
                            if let [PdfObject::Str(low), PdfObject::Str(high)] = pair {
                                cmap.codespace.push(PdfCodespaceRange { byte_width: low.len().clamp(1, 4) as u32, low: code_of(low), high: code_of(high) });
                            }
                        }
                    }
                    Some("cidchar") | Some("bfchar") => {
                        for pair in items.chunks(2) {
                            if let [PdfObject::Str(code), dst] = pair {
                                match dst {
                                    PdfObject::Int(cid) => cmap.mappings.push(PdfCidMapping::Char { code: code_of(code), cid: *cid as u32 }),
                                    PdfObject::Str(dst) => cmap.mappings.push(PdfCidMapping::Char { code: code_of(code), cid: code_of(dst) }),
                                    _ => {}
                                }
                            }
                        }
                    }
                    Some("cidrange") | Some("bfrange") => {
                        for triple in items.chunks(3) {
                            if let [PdfObject::Str(low), PdfObject::Str(high), dst] = triple {
                                match dst {
                                    PdfObject::Int(cid) => cmap.mappings.push(PdfCidMapping::Range { low: code_of(low), high: code_of(high), cid: *cid as u32 }),
                                    PdfObject::Str(dst) => cmap.mappings.push(PdfCidMapping::Range { low: code_of(low), high: code_of(high), cid: code_of(dst) }),
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
                section = None;
            }
            _ => {
                if section.is_some() {
                    pending.extend(operands);
                } else if let Some(PdfObject::Name(name)) = operands.last() {
                    last_name = Some(name.clone());
                }
            }
        }
    }
    if cmap.codespace.is_empty() {
        let width = cmap.mappings.iter().map(|_| 2).next().unwrap_or(2);
        cmap.codespace.push(PdfCodespaceRange { byte_width: width, low: 0, high: if width == 1 { 0xFF } else { 0xFFFF } });
    }
    cmap
}

/// 📖 Parses a `ToUnicode` CMap stream body (bfchar + bfrange, scalar and array destinations).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_to_unicode(body: &[u8]) -> PdfToUnicode {
    let mut out = PdfToUnicode { byte_width: 0, mappings: Vec::new() };
    let mut pending: Vec<PdfObject> = Vec::new();
    let mut section: Option<&str> = None;
    for (operands, operator) in statements(body) {
        match operator.as_str() {
            "begincodespacerange" => {
                section = Some("codespace");
                pending.clear();
            }
            "beginbfchar" => {
                section = Some("bfchar");
                pending.clear();
            }
            "beginbfrange" => {
                section = Some("bfrange");
                pending.clear();
            }
            "endcodespacerange" => {
                pending.extend(operands);
                for pair in std::mem::take(&mut pending).chunks(2) {
                    if let [PdfObject::Str(low), _] = pair {
                        if out.byte_width == 0 {
                            out.byte_width = low.len().clamp(1, 4) as u32;
                        }
                    }
                }
                section = None;
            }
            "endbfchar" => {
                pending.extend(operands);
                for pair in std::mem::take(&mut pending).chunks(2) {
                    if let [PdfObject::Str(code), PdfObject::Str(dst)] = pair {
                        if out.byte_width == 0 {
                            out.byte_width = code.len().clamp(1, 4) as u32;
                        }
                        out.mappings.push(PdfToUnicodeMapping::Char { code: code_of(code), text: utf16_text(dst) });
                    } else if let [PdfObject::Str(code), PdfObject::Name(name)] = pair {
                        if let Some(text) = crate::standards::v1_7::subsets::base::modules::fonts::glyph_name_to_unicode(name) {
                            out.mappings.push(PdfToUnicodeMapping::Char { code: code_of(code), text });
                        }
                    }
                }
                section = None;
            }
            "endbfrange" => {
                pending.extend(operands);
                for triple in std::mem::take(&mut pending).chunks(3) {
                    match triple {
                        [PdfObject::Str(low), PdfObject::Str(high), PdfObject::Str(dst)] => {
                            if out.byte_width == 0 {
                                out.byte_width = low.len().clamp(1, 4) as u32;
                            }
                            out.mappings.push(PdfToUnicodeMapping::Range { low: code_of(low), high: code_of(high), text: utf16_text(dst) });
                        }
                        [PdfObject::Str(low), PdfObject::Str(_), PdfObject::Array(items)] => {
                            if out.byte_width == 0 {
                                out.byte_width = low.len().clamp(1, 4) as u32;
                            }
                            for (offset, item) in items.iter().enumerate() {
                                if let PdfObject::Str(dst) = item {
                                    out.mappings.push(PdfToUnicodeMapping::Char { code: code_of(low) + offset as u32, text: utf16_text(dst) });
                                }
                            }
                        }
                        _ => {}
                    }
                }
                section = None;
            }
            _ => {
                if section.is_some() {
                    pending.extend(operands);
                }
            }
        }
    }
    if out.byte_width == 0 {
        out.byte_width = 2;
    }
    out
}
//#endregion 🔖️Parsing

//#region 🔖️Printing
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_code(code: u32, byte_width: u32) -> String {
    format!("<{:0width$X}>", code, width = (byte_width as usize) * 2)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_utf16(text: &str) -> String {
    let mut out = String::from("<");
    for unit in text.encode_utf16() {
        out.push_str(&format!("{unit:04X}"));
    }
    out.push('>');
    out
}

/// 🖨️ Prints a `ToUnicode` CMap stream body in canonical form (§9.10.3 example shape).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_to_unicode(map: &PdfToUnicode) -> Vec<u8> {
    let width = map.byte_width.clamp(1, 4);
    let mut out = String::new();
    out.push_str("/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n/CMapName /Adobe-Identity-UCS def\n/CMapType 2 def\n1 begincodespacerange\n");
    out.push_str(&format!("{} {}\nendcodespacerange\n", hex_code(0, width), hex_code(if width == 1 { 0xFF } else if width == 2 { 0xFFFF } else { u32::MAX >> (32 - width * 8) }, width)));
    let mut cursor = 0;
    while cursor < map.mappings.len() {
        let is_char = matches!(map.mappings[cursor], PdfToUnicodeMapping::Char { .. });
        let mut end = cursor;
        while end < map.mappings.len() && end - cursor < 100 && matches!(map.mappings[end], PdfToUnicodeMapping::Char { .. }) == is_char {
            end += 1;
        }
        out.push_str(&format!("{} {}\n", end - cursor, if is_char { "beginbfchar" } else { "beginbfrange" }));
        for mapping in &map.mappings[cursor..end] {
            match mapping {
                PdfToUnicodeMapping::Char { code, text } => out.push_str(&format!("{} {}\n", hex_code(*code, width), hex_utf16(text))),
                PdfToUnicodeMapping::Range { low, high, text } => out.push_str(&format!("{} {} {}\n", hex_code(*low, width), hex_code(*high, width), hex_utf16(text))),
            }
        }
        out.push_str(if is_char { "endbfchar\n" } else { "endbfrange\n" });
        cursor = end;
    }
    out.push_str("endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n");
    out.into_bytes()
}

/// 🖨️ Prints an embedded CMap stream body in canonical form.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_cmap(cmap: &PdfEmbeddedCMap) -> Vec<u8> {
    let mut out = String::new();
    out.push_str("%!PS-Adobe-3.0 Resource-CMap\n/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n");
    if let Some(parent) = &cmap.use_cmap {
        out.push_str(&format!("/{parent} usecmap\n"));
    }
    out.push_str(&format!("/CMapName /{} def\n/CMapType 1 def\n/WMode {} def\n", cmap.name, if cmap.vertical { 1 } else { 0 }));
    if !cmap.codespace.is_empty() {
        out.push_str(&format!("{} begincodespacerange\n", cmap.codespace.len()));
        for range in &cmap.codespace {
            out.push_str(&format!("{} {}\n", hex_code(range.low, range.byte_width), hex_code(range.high, range.byte_width)));
        }
        out.push_str("endcodespacerange\n");
    }
    let width_of = |code: u32| cmap.codespace.iter().find(|range| code >= range.low && code <= range.high).map(|range| range.byte_width).unwrap_or(2);
    let mut cursor = 0;
    while cursor < cmap.mappings.len() {
        let is_char = matches!(cmap.mappings[cursor], PdfCidMapping::Char { .. });
        let mut end = cursor;
        while end < cmap.mappings.len() && end - cursor < 100 && matches!(cmap.mappings[end], PdfCidMapping::Char { .. }) == is_char {
            end += 1;
        }
        out.push_str(&format!("{} {}\n", end - cursor, if is_char { "begincidchar" } else { "begincidrange" }));
        for mapping in &cmap.mappings[cursor..end] {
            match mapping {
                PdfCidMapping::Char { code, cid } => out.push_str(&format!("{} {cid}\n", hex_code(*code, width_of(*code)))),
                PdfCidMapping::Range { low, high, cid } => out.push_str(&format!("{} {} {cid}\n", hex_code(*low, width_of(*low)), hex_code(*high, width_of(*low)))),
            }
        }
        out.push_str(if is_char { "endcidchar\n" } else { "endcidrange\n" });
        cursor = end;
    }
    out.push_str("endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n");
    out.into_bytes()
}
//#endregion 🔖️Printing

//#region 🔖️Codes
/// 🔢 A code splitter + code → CID map for one composite font's encoding.
#[derive(Clone, Debug)]
pub struct CidCodec {
    codespace: Vec<PdfCodespaceRange>,
    mappings: Vec<PdfCidMapping>,
    identity: bool,
    pub vertical: bool,
}

impl CidCodec {
    pub fn identity(vertical: bool) -> Self {
        Self { codespace: vec![PdfCodespaceRange { byte_width: 2, low: 0, high: 0xFFFF }], mappings: Vec::new(), identity: true, vertical }
    }

    pub fn embedded(cmap: &PdfEmbeddedCMap) -> Self {
        let identity = matches!(cmap.use_cmap.as_deref(), Some("Identity-H") | Some("Identity-V"));
        Self { codespace: cmap.codespace.clone(), mappings: cmap.mappings.clone(), identity, vertical: cmap.vertical }
    }

    /// 🔢 Whether every code is two bytes wide (what lets a writer emit `<XXXX>` strings).
    pub fn two_byte(&self) -> bool {
        self.codespace.iter().all(|range| range.byte_width == 2)
    }

    /// ✂️ Splits a string operand into character codes and their physical widths (§9.7.6.2).
    pub fn split(&self, bytes: &[u8]) -> Vec<(u32, u32)> {
        codes(Some(self), bytes).collect()
    }

    /// 🔢 The CID a code maps to (0 = notdef when unmapped).
    pub fn cid(&self, code: u32) -> u32 {
        for mapping in &self.mappings {
            match mapping {
                PdfCidMapping::Char { code: c, cid } if *c == code => return *cid,
                PdfCidMapping::Range { low, high, cid } if code >= *low && code <= *high => return cid + (code - low),
                _ => {}
            }
        }
        if self.identity {
            code
        } else {
            0
        }
    }

    /// 🔁 A code that maps to `cid`, if any (the writer's direction).
    pub fn code_for_cid(&self, cid: u32) -> Option<(u32, u32)> {
        for mapping in &self.mappings {
            match mapping {
                PdfCidMapping::Char { code, cid: c } if *c == cid => return Some((*code, self.width_of(*code))),
                PdfCidMapping::Range { low, high, cid: c } if cid >= *c && cid <= c + (high - low) => {
                    let code = low + (cid - c);
                    return Some((code, self.width_of(code)));
                }
                _ => {}
            }
        }
        if self.identity {
            Some((cid, 2))
        } else {
            None
        }
    }

    pub fn width_of(&self, code: u32) -> u32 {
        self.codespace.iter().find(|range| code >= range.low && code <= range.high).map(|range| range.byte_width).unwrap_or(2)
    }

    /// 🔢 Appends `code` as `width` big-endian bytes.
    pub fn push_code(out: &mut Vec<u8>, code: u32, width: u32) {
        for index in (0..width).rev() {
            out.push((code >> (8 * index)) as u8);
        }
    }
}

/// 📜 Reads the shortest matching code, or consumes the shortest matching first-byte width
/// (limited to the bytes remaining). Without a matching prefix, consumes one byte.
pub(super) fn codes<'a>(codec: Option<&'a CidCodec>, mut bytes: &'a [u8]) -> impl Iterator<Item = (u32, u32)> + 'a {
    let ranges = codec.map_or(&[][..], |codec| codec.codespace.as_slice());
    std::iter::from_fn(move || {
        let first = u32::from(*bytes.first()?);
        if ranges.is_empty() {
            bytes = &bytes[1..];
            return Some((first, 1));
        }
        let mut code = 0;
        for (index, &byte) in bytes.iter().take(4).enumerate() {
            code = (code << 8) | u32::from(byte);
            let width = index as u32 + 1;
            if ranges.iter().any(|range| range.byte_width == width && (range.low..=range.high).contains(&code)) {
                bytes = &bytes[width as usize..];
                return Some((code, width));
            }
        }
        let width = ranges.iter().filter_map(|range| {
            let width = range.byte_width;
            if !(1..=4).contains(&width) {
                return None;
            }
            let shift = 8 * (width - 1);
            ((range.low >> shift)..=(range.high >> shift)).contains(&first).then_some(width as usize)
        }).min().unwrap_or(1).min(bytes.len());
        let (head, tail) = bytes.split_at(width);
        bytes = tail;
        Some((code_of(head), width as u32))
    })
}

/// 🈴 Unicode lookup over a typed `ToUnicode` map.
pub fn to_unicode_lookup(map: &PdfToUnicode, code: u32) -> Option<String> {
    to_unicode_text(map, code).map(std::borrow::Cow::into_owned)
}

/// 🗣️ Borrows verbatim Unicode mappings, computing text only for offset range entries.
pub(super) fn to_unicode_text(map: &PdfToUnicode, code: u32) -> Option<std::borrow::Cow<'_, str>> {
    use std::borrow::Cow;
    for mapping in &map.mappings {
        match mapping {
            PdfToUnicodeMapping::Char { code: c, text } if *c == code => return Some(Cow::Borrowed(text)),
            PdfToUnicodeMapping::Range { low, high, text } if code >= *low && code <= *high => {
                if code == *low {
                    return Some(Cow::Borrowed(text));
                }
                let Some((index, last)) = text.char_indices().next_back() else {
                    return Some(Cow::Borrowed(text));
                };
                let mut result = text[..index].to_owned();
                let shifted = (last as u32).checked_add(code - low).and_then(char::from_u32).unwrap_or('\u{FFFD}');
                result.push(shifted);
                return Some(Cow::Owned(result));
            }
            _ => {}
        }
    }
    None
}

/// 🔁 The code a Unicode string maps from, if the map is invertible for it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn to_unicode_reverse(map: &PdfToUnicode, text: &str) -> Option<u32> {
    for mapping in &map.mappings {
        match mapping {
            PdfToUnicodeMapping::Char { code, text: candidate } if candidate == text => return Some(*code),
            PdfToUnicodeMapping::Range { low, high, text: base } if high>=low => {
                let mut base_chars: Vec<char> = base.chars().collect();
                let text_chars: Vec<char> = text.chars().collect();
                if base_chars.len() == text_chars.len() && !base_chars.is_empty() {
                    let last = base_chars.pop().expect("non-empty");
                    if base_chars[..] == text_chars[..text_chars.len() - 1] {
                        let target = *text_chars.last().expect("non-empty") as u32;
                        if target >= last as u32 && target - last as u32 <= high - low {
                            return Some(low + (target - last as u32));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    None
}
//#endregion 🔖️Codes

//#region 🧪️Tests

//#endregion 🧪️Tests

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
