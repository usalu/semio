//! 🖋️ Content streams (ISO 32000-1 §7.8.2, §8, §9): every operator of Table 51 parsed into
//! typed [`PdfOp`]s and printed back canonically. Text-showing operands are decoded through the
//! font the stream selected (`Tf`) and re-encoded through it on print, so the typed lane carries
//! Unicode wherever the font can vouch for it and raw codes everywhere else.

use super::colour::{lift_colour_space_inline, lower_colour_space_inline};
use super::filters::{decode_stream, encode_stream};
use super::fonts::FontCodec;
use super::lexer::{is_ws, number_text, write_object, Lexer, PResult, PdfEngineError, Token};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfDictEntry, PdfInlineImage, PdfLineCap, PdfLineJoin, PdfObject, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString};
use std::collections::HashMap;

//#region 🔖️FontTable
/// 🔤 The fonts a content stream may select, by resource name.
pub trait FontTable {
    fn font(&self, name: &str) -> Option<&FontCodec>;
}

impl FontTable for HashMap<String, FontCodec> {
    fn font(&self, name: &str) -> Option<&FontCodec> {
        self.get(name)
    }
}

/// 🔤 A table with no fonts: every text operand stays raw codes.
pub struct NoFonts;

impl FontTable for NoFonts {
    fn font(&self, _name: &str) -> Option<&FontCodec> {
        None
    }
}
//#endregion 🔖️FontTable

//#region 🔖️Parse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn num(operands: &[PdfObject], index: usize) -> f64 {
    operands.get(index).and_then(PdfObject::as_f64).unwrap_or(0.0)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn nums(operands: &[PdfObject]) -> Vec<f64> {
    operands.iter().filter_map(PdfObject::as_f64).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn name(operands: &[PdfObject], index: usize) -> Option<String> {
    operands.get(index).and_then(PdfObject::as_name).map(str::to_string)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn matrix(operands: &[PdfObject]) -> Option<[f64; 6]> {
    let values = nums(operands);
    (values.len() >= 6).then(|| [values[0], values[1], values[2], values[3], values[4], values[5]])
}

/// 🔤 Decodes a string operand through the current font: `Text` when the font maps every code to
/// Unicode AND re-encoding that text yields the same bytes (so the typed lane is a fixed point),
/// `Codes` otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn text_string(bytes: &[u8], font: Option<&FontCodec>) -> PdfTextString {
    match font {
        Some(font) => {
            if let Some(text) = font.decode_text(bytes) {
                if font.encode(&text).as_deref() == Some(bytes) {
                    return PdfTextString::Text { text };
                }
            }
        }
        None if bytes.iter().all(|byte| (0x20..=0x7E).contains(byte)) => {
            // 🔤 No font to consult: printable ASCII is its own encoding in every simple font.
            return PdfTextString::Text { text: bytes.iter().map(|byte| *byte as char).collect() };
        }
        None => {}
    }
    PdfTextString::Codes { bytes: bytes.to_vec() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn text_array(items: &[PdfObject], font: Option<&FontCodec>) -> Vec<PdfTextArrayItem> {
    items
        .iter()
        .filter_map(|item| match item {
            PdfObject::Str(bytes) => Some(match text_string(bytes, font) {
                PdfTextString::Text { text } => PdfTextArrayItem::Text { text },
                PdfTextString::Codes { bytes } => PdfTextArrayItem::Codes { bytes },
            }),
            other => other.as_f64().map(|amount| PdfTextArrayItem::Adjust { amount }),
        })
        .collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn property_list(operand: Option<&PdfObject>) -> PdfPropertyList {
    match operand {
        Some(PdfObject::Name(name)) => PdfPropertyList::Named { name: name.clone() },
        Some(PdfObject::Dict(entries)) => PdfPropertyList::Inline { entries: entries.clone() },
        _ => PdfPropertyList::Inline { entries: Vec::new() },
    }
}

/// 🖼️ Expands the abbreviated inline-image keys (§8.9.7, Table 93).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand_inline_key(key: &str) -> &str {
    match key {
        "BPC" => "BitsPerComponent",
        "CS" => "ColorSpace",
        "D" => "Decode",
        "DP" => "DecodeParms",
        "F" => "Filter",
        "H" => "Height",
        "W" => "Width",
        "IM" => "ImageMask",
        "I" => "Interpolate",
        "L" => "Length",
        other => other,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand_inline_value(key: &str, value: PdfObject) -> PdfObject {
    let expand_name = |name: &str| -> String {
        match name {
            "G" => "DeviceGray",
            "RGB" => "DeviceRGB",
            "CMYK" => "DeviceCMYK",
            "I" => "Indexed",
            "AHx" => "ASCIIHexDecode",
            "A85" => "ASCII85Decode",
            "LZW" => "LZWDecode",
            "Fl" => "FlateDecode",
            "RL" => "RunLengthDecode",
            "CCF" => "CCITTFaxDecode",
            "DCT" => "DCTDecode",
            other => other,
        }
        .to_string()
    };
    match (key, value) {
        ("ColorSpace" | "Filter", PdfObject::Name(name)) => PdfObject::Name(expand_name(&name)),
        ("ColorSpace" | "Filter", PdfObject::Array(items)) => PdfObject::Array(items.into_iter().map(|item| match item {
            PdfObject::Name(name) => PdfObject::Name(expand_name(&name)),
            other => other,
        }).collect()),
        (_, value) => value,
    }
}

/// 🖼️ Reads the binary body of an inline image after `ID`: an unfiltered image has a known
/// length; a filtered one ends at the first `EI` delimited by whitespace.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inline_image_body<'a>(data: &'a [u8], start: usize, dict: &[PdfDictEntry]) -> (&'a [u8], usize) {
    let int = |key: &str| dict.iter().find(|e| e.key == key).and_then(|e| e.value.as_i64()).unwrap_or(0).max(0) as usize;
    let filtered = dict.iter().any(|e| e.key == "Filter" && !matches!(&e.value, PdfObject::Null) && !matches!(&e.value, PdfObject::Array(items) if items.is_empty()));
    if !filtered {
        let width = int("Width");
        let height = int("Height");
        let mask = dict.iter().any(|e| e.key == "ImageMask" && e.value.as_bool() == Some(true));
        let bpc = if mask { 1 } else { int("BitsPerComponent").max(1) };
        let components = if mask {
            1
        } else {
            match dict.iter().find(|e| e.key == "ColorSpace").map(|e| &e.value) {
                Some(PdfObject::Name(name)) => match name.as_str() {
                    "DeviceRGB" | "CalRGB" | "Lab" => 3,
                    "DeviceCMYK" => 4,
                    _ => 1,
                },
                Some(PdfObject::Array(items)) => match items.first().and_then(PdfObject::as_name) {
                    Some("DeviceRGB") | Some("CalRGB") | Some("Lab") => 3,
                    Some("DeviceCMYK") => 4,
                    Some("ICCBased") => items.get(1).and_then(|v| v.dict_get("N")).and_then(PdfObject::as_i64).unwrap_or(1) as usize,
                    _ => 1,
                },
                _ => 1,
            }
        };
        let length = (width * components * bpc).div_ceil(8) * height;
        if start + length <= data.len() {
            let mut end = start + length;
            let mut lexer = Lexer::new(data).at(end);
            lexer.skip_ws();
            if lexer.consume_keyword(b"EI") {
                return (&data[start..end], lexer.pos);
            }
            end = start;
            let _ = end;
        }
    }
    let mut cursor = start;
    while cursor + 2 <= data.len() {
        if data[cursor] == b'E' && data[cursor + 1] == b'I' && (cursor + 2 == data.len() || is_ws(data[cursor + 2])) && (cursor == 0 || is_ws(data[cursor - 1])) {
            let mut end = cursor;
            if end > start && is_ws(data[end - 1]) {
                end -= 1;
            }
            return (&data[start..end], cursor + 2);
        }
        cursor += 1;
    }
    (&data[start..], data.len())
}

/// 📖 Parses a content stream into typed operators.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_content(data: &[u8], fonts: &dyn FontTable) -> Vec<PdfOp> {
    let mut ops = Vec::new();
    let mut lexer = Lexer::new(data);
    let mut operands: Vec<PdfObject> = Vec::new();
    let mut current_font: Option<String> = None;
    let mut font_stack: Vec<Option<String>> = Vec::new();
    loop {
        let token = match lexer.next_token() {
            Ok(token) => token,
            Err(_) => {
                lexer.pos += 1;
                continue;
            }
        };
        let word = match token {
            Token::End => break,
            Token::Object(value) => {
                if operands.len() < 64 {
                    operands.push(value);
                }
                continue;
            }
            Token::Keyword(word) => word,
        };
        let font = current_font.as_deref().and_then(|name| fonts.font(name));
        let op = match word.as_str() {
            "w" => PdfOp::SetLineWidth { width: num(&operands, 0) },
            "J" => PdfOp::SetLineCap { cap: match num(&operands, 0) as i64 { 1 => PdfLineCap::Round, 2 => PdfLineCap::Square, _ => PdfLineCap::Butt } },
            "j" => PdfOp::SetLineJoin { join: match num(&operands, 0) as i64 { 1 => PdfLineJoin::Round, 2 => PdfLineJoin::Bevel, _ => PdfLineJoin::Miter } },
            "M" => PdfOp::SetMiterLimit { limit: num(&operands, 0) },
            "d" => PdfOp::SetDash { array: operands.first().and_then(PdfObject::as_array).map(nums).unwrap_or_default(), phase: num(&operands, 1) },
            "ri" => PdfOp::SetRenderingIntent { intent: name(&operands, 0).unwrap_or_default() },
            "i" => PdfOp::SetFlatness { flatness: num(&operands, 0) },
            "gs" => PdfOp::SetExtGState { name: name(&operands, 0).unwrap_or_default() },
            "q" => {
                font_stack.push(current_font.clone());
                PdfOp::Save
            }
            "Q" => {
                if let Some(saved) = font_stack.pop() {
                    current_font = saved;
                }
                PdfOp::Restore
            }
            "cm" => PdfOp::Transform { matrix: matrix(&operands).unwrap_or([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]) },
            "m" => PdfOp::MoveTo { x: num(&operands, 0), y: num(&operands, 1) },
            "l" => PdfOp::LineTo { x: num(&operands, 0), y: num(&operands, 1) },
            "c" => PdfOp::CurveTo { x1: num(&operands, 0), y1: num(&operands, 1), x2: num(&operands, 2), y2: num(&operands, 3), x3: num(&operands, 4), y3: num(&operands, 5) },
            "v" => PdfOp::CurveToInitial { x2: num(&operands, 0), y2: num(&operands, 1), x3: num(&operands, 2), y3: num(&operands, 3) },
            "y" => PdfOp::CurveToFinal { x1: num(&operands, 0), y1: num(&operands, 1), x3: num(&operands, 2), y3: num(&operands, 3) },
            "h" => PdfOp::ClosePath,
            "re" => PdfOp::Rectangle { x: num(&operands, 0), y: num(&operands, 1), width: num(&operands, 2), height: num(&operands, 3) },
            "S" => PdfOp::Stroke,
            "s" => PdfOp::CloseStroke,
            "f" | "F" => PdfOp::Fill,
            "f*" => PdfOp::FillEvenOdd,
            "B" => PdfOp::FillStroke,
            "B*" => PdfOp::FillStrokeEvenOdd,
            "b" => PdfOp::CloseFillStroke,
            "b*" => PdfOp::CloseFillStrokeEvenOdd,
            "n" => PdfOp::EndPath,
            "W" => PdfOp::Clip,
            "W*" => PdfOp::ClipEvenOdd,
            "BT" => PdfOp::BeginText,
            "ET" => PdfOp::EndText,
            "Tc" => PdfOp::SetCharSpacing { spacing: num(&operands, 0) },
            "Tw" => PdfOp::SetWordSpacing { spacing: num(&operands, 0) },
            "Tz" => PdfOp::SetHorizontalScale { scale: num(&operands, 0) },
            "TL" => PdfOp::SetLeading { leading: num(&operands, 0) },
            "Tf" => {
                let font_name = name(&operands, 0).unwrap_or_default();
                current_font = Some(font_name.clone());
                PdfOp::SetFont { name: font_name, size: num(&operands, 1) }
            }
            "Tr" => PdfOp::SetTextRenderingMode { mode: num(&operands, 0).max(0.0) as u32 },
            "Ts" => PdfOp::SetTextRise { rise: num(&operands, 0) },
            "Td" => PdfOp::MoveText { tx: num(&operands, 0), ty: num(&operands, 1) },
            "TD" => PdfOp::MoveTextSetLeading { tx: num(&operands, 0), ty: num(&operands, 1) },
            "Tm" => PdfOp::SetTextMatrix { matrix: matrix(&operands).unwrap_or([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]) },
            "T*" => PdfOp::NextLine,
            "Tj" => PdfOp::ShowText { text: text_string(operands.first().and_then(PdfObject::as_str_bytes).unwrap_or(&[]), font) },
            "'" => PdfOp::NextLineShowText { text: text_string(operands.first().and_then(PdfObject::as_str_bytes).unwrap_or(&[]), font) },
            "\"" => PdfOp::NextLineShowTextSpaced { word_spacing: num(&operands, 0), char_spacing: num(&operands, 1), text: text_string(operands.get(2).and_then(PdfObject::as_str_bytes).unwrap_or(&[]), font) },
            "TJ" => PdfOp::ShowTextArray { items: text_array(operands.first().and_then(PdfObject::as_array).unwrap_or(&[]), font) },
            "d0" => PdfOp::SetGlyphWidth { wx: num(&operands, 0), wy: num(&operands, 1) },
            "d1" => PdfOp::SetGlyphWidthAndBox { wx: num(&operands, 0), wy: num(&operands, 1), llx: num(&operands, 2), lly: num(&operands, 3), urx: num(&operands, 4), ury: num(&operands, 5) },
            "CS" => PdfOp::SetStrokeColorSpace { name: name(&operands, 0).unwrap_or_default() },
            "cs" => PdfOp::SetFillColorSpace { name: name(&operands, 0).unwrap_or_default() },
            "SC" => PdfOp::SetStrokeColor { components: nums(&operands) },
            "sc" => PdfOp::SetFillColor { components: nums(&operands) },
            "SCN" => PdfOp::SetStrokeColorN { components: nums(&operands), pattern: operands.last().and_then(PdfObject::as_name).map(str::to_string) },
            "scn" => PdfOp::SetFillColorN { components: nums(&operands), pattern: operands.last().and_then(PdfObject::as_name).map(str::to_string) },
            "G" => PdfOp::SetStrokeGray { gray: num(&operands, 0) },
            "g" => PdfOp::SetFillGray { gray: num(&operands, 0) },
            "RG" => PdfOp::SetStrokeRgb { r: num(&operands, 0), g: num(&operands, 1), b: num(&operands, 2) },
            "rg" => PdfOp::SetFillRgb { r: num(&operands, 0), g: num(&operands, 1), b: num(&operands, 2) },
            "K" => PdfOp::SetStrokeCmyk { c: num(&operands, 0), m: num(&operands, 1), y: num(&operands, 2), k: num(&operands, 3) },
            "k" => PdfOp::SetFillCmyk { c: num(&operands, 0), m: num(&operands, 1), y: num(&operands, 2), k: num(&operands, 3) },
            "sh" => PdfOp::PaintShading { name: name(&operands, 0).unwrap_or_default() },
            "Do" => PdfOp::PaintXObject { name: name(&operands, 0).unwrap_or_default() },
            "BI" => {
                // 🖼️ Key/value pairs up to `ID`, then the binary body up to `EI`.
                let mut entries: Vec<PdfDictEntry> = Vec::new();
                let mut key: Option<String> = None;
                loop {
                    match lexer.next_token() {
                        Ok(Token::Object(PdfObject::Name(n))) if key.is_none() => key = Some(n),
                        Ok(Token::Object(value)) => {
                            if let Some(k) = key.take() {
                                let expanded = expand_inline_key(&k).to_string();
                                let value = expand_inline_value(&expanded, value);
                                entries.push(PdfDictEntry { key: expanded, value });
                            }
                        }
                        Ok(Token::Keyword(k)) if k == "ID" => break,
                        Ok(Token::Keyword(_)) => {}
                        Ok(Token::End) | Err(_) => break,
                    }
                }
                let body_start = (lexer.pos + 1).min(data.len());
                let (raw, resume) = inline_image_body(data, body_start, &entries);
                lexer.pos = resume;
                let (decoded, filters) = decode_stream(&entries, raw).unwrap_or_else(|_| (raw.to_vec(), Vec::new()));
                let int = |k: &str| entries.iter().find(|e| e.key == k).and_then(|e| e.value.as_i64()).unwrap_or(0).max(0) as u32;
                let image = PdfInlineImage {
                    width: int("Width"),
                    height: int("Height"),
                    bits_per_component: int("BitsPerComponent"),
                    color_space: entries.iter().find(|e| e.key == "ColorSpace").map(|e| lift_colour_space_inline(&e.value)),
                    image_mask: entries.iter().any(|e| e.key == "ImageMask" && e.value.as_bool() == Some(true)),
                    decode: entries.iter().find(|e| e.key == "Decode").and_then(|e| e.value.as_array()).map(nums).unwrap_or_default(),
                    interpolate: entries.iter().any(|e| e.key == "Interpolate" && e.value.as_bool() == Some(true)),
                    filters,
                    data: decoded,
                    extra: entries.into_iter().filter(|e| !matches!(e.key.as_str(), "Width" | "Height" | "BitsPerComponent" | "ColorSpace" | "ImageMask" | "Decode" | "Interpolate" | "Filter" | "DecodeParms" | "Length")).collect(),
                };
                operands.clear();
                ops.push(PdfOp::InlineImage { image });
                continue;
            }
            "MP" => PdfOp::MarkedContentPoint { tag: name(&operands, 0).unwrap_or_default() },
            "DP" => PdfOp::MarkedContentPointWithProperties { tag: name(&operands, 0).unwrap_or_default(), properties: property_list(operands.get(1)) },
            "BMC" => PdfOp::BeginMarkedContent { tag: name(&operands, 0).unwrap_or_default() },
            "BDC" => PdfOp::BeginMarkedContentWithProperties { tag: name(&operands, 0).unwrap_or_default(), properties: property_list(operands.get(1)) },
            "EMC" => PdfOp::EndMarkedContent,
            "BX" => PdfOp::BeginCompatibility,
            "EX" => PdfOp::EndCompatibility,
            other => PdfOp::Unknown { operator: other.to_string(), operands: std::mem::take(&mut operands) },
        };
        operands.clear();
        ops.push(op);
    }
    ops
}
//#endregion 🔖️Parse

//#region 🔖️Print
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_nums(out: &mut Vec<u8>, values: &[f64]) {
    for value in values {
        out.extend_from_slice(number_text(*value).as_bytes());
        out.push(b' ');
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_name(out: &mut Vec<u8>, name: &str) {
    super::lexer::write_name(out, name);
    out.push(b' ');
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_text(text: &PdfTextString, font: Option<&FontCodec>, font_name: Option<&str>) -> PResult<Vec<u8>> {
    match text {
        PdfTextString::Codes { bytes } => Ok(bytes.clone()),
        PdfTextString::Text { text } => match font {
            Some(font) => font.encode(text).ok_or_else(|| PdfEngineError::Unsupported(format!("font /{} cannot show {text:?}", font_name.unwrap_or("?")))),
            None if text.bytes().all(|byte| (0x20..=0x7E).contains(&byte)) => Ok(text.as_bytes().to_vec()),
            None => Err(PdfEngineError::Malformed(format!("text {text:?} shown before any font was selected (or font /{} is unknown)", font_name.unwrap_or("?")))),
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_string(out: &mut Vec<u8>, bytes: &[u8]) {
    super::lexer::write_string(out, bytes);
    out.push(b' ');
}

/// 🖨️ Prints typed operators as a content stream (one operator per line).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_content(ops: &[PdfOp], fonts: &dyn FontTable) -> PResult<Vec<u8>> {
    let mut out = Vec::new();
    let mut current_font: Option<String> = None;
    let mut font_stack: Vec<Option<String>> = Vec::new();
    for op in ops {
        let font = current_font.as_deref().and_then(|name| fonts.font(name));
        let font_name = current_font.as_deref();
        let word: &str = match op {
            PdfOp::SetLineWidth { width } => {
                push_nums(&mut out, &[*width]);
                "w"
            }
            PdfOp::SetLineCap { cap } => {
                push_nums(&mut out, &[*cap as u8 as f64]);
                "J"
            }
            PdfOp::SetLineJoin { join } => {
                push_nums(&mut out, &[*join as u8 as f64]);
                "j"
            }
            PdfOp::SetMiterLimit { limit } => {
                push_nums(&mut out, &[*limit]);
                "M"
            }
            PdfOp::SetDash { array, phase } => {
                out.push(b'[');
                for (index, value) in array.iter().enumerate() {
                    if index > 0 {
                        out.push(b' ');
                    }
                    out.extend_from_slice(number_text(*value).as_bytes());
                }
                out.extend_from_slice(b"] ");
                push_nums(&mut out, &[*phase]);
                "d"
            }
            PdfOp::SetRenderingIntent { intent } => {
                push_name(&mut out, intent);
                "ri"
            }
            PdfOp::SetFlatness { flatness } => {
                push_nums(&mut out, &[*flatness]);
                "i"
            }
            PdfOp::SetExtGState { name } => {
                push_name(&mut out, name);
                "gs"
            }
            PdfOp::Save => {
                font_stack.push(current_font.clone());
                "q"
            }
            PdfOp::Restore => {
                if let Some(saved) = font_stack.pop() {
                    current_font = saved;
                }
                "Q"
            }
            PdfOp::Transform { matrix } => {
                push_nums(&mut out, matrix);
                "cm"
            }
            PdfOp::MoveTo { x, y } => {
                push_nums(&mut out, &[*x, *y]);
                "m"
            }
            PdfOp::LineTo { x, y } => {
                push_nums(&mut out, &[*x, *y]);
                "l"
            }
            PdfOp::CurveTo { x1, y1, x2, y2, x3, y3 } => {
                push_nums(&mut out, &[*x1, *y1, *x2, *y2, *x3, *y3]);
                "c"
            }
            PdfOp::CurveToInitial { x2, y2, x3, y3 } => {
                push_nums(&mut out, &[*x2, *y2, *x3, *y3]);
                "v"
            }
            PdfOp::CurveToFinal { x1, y1, x3, y3 } => {
                push_nums(&mut out, &[*x1, *y1, *x3, *y3]);
                "y"
            }
            PdfOp::ClosePath => "h",
            PdfOp::Rectangle { x, y, width, height } => {
                push_nums(&mut out, &[*x, *y, *width, *height]);
                "re"
            }
            PdfOp::Stroke => "S",
            PdfOp::CloseStroke => "s",
            PdfOp::Fill => "f",
            PdfOp::FillEvenOdd => "f*",
            PdfOp::FillStroke => "B",
            PdfOp::FillStrokeEvenOdd => "B*",
            PdfOp::CloseFillStroke => "b",
            PdfOp::CloseFillStrokeEvenOdd => "b*",
            PdfOp::EndPath => "n",
            PdfOp::Clip => "W",
            PdfOp::ClipEvenOdd => "W*",
            PdfOp::BeginText => "BT",
            PdfOp::EndText => "ET",
            PdfOp::SetCharSpacing { spacing } => {
                push_nums(&mut out, &[*spacing]);
                "Tc"
            }
            PdfOp::SetWordSpacing { spacing } => {
                push_nums(&mut out, &[*spacing]);
                "Tw"
            }
            PdfOp::SetHorizontalScale { scale } => {
                push_nums(&mut out, &[*scale]);
                "Tz"
            }
            PdfOp::SetLeading { leading } => {
                push_nums(&mut out, &[*leading]);
                "TL"
            }
            PdfOp::SetFont { name, size } => {
                current_font = Some(name.clone());
                push_name(&mut out, name);
                push_nums(&mut out, &[*size]);
                "Tf"
            }
            PdfOp::SetTextRenderingMode { mode } => {
                push_nums(&mut out, &[*mode as f64]);
                "Tr"
            }
            PdfOp::SetTextRise { rise } => {
                push_nums(&mut out, &[*rise]);
                "Ts"
            }
            PdfOp::MoveText { tx, ty } => {
                push_nums(&mut out, &[*tx, *ty]);
                "Td"
            }
            PdfOp::MoveTextSetLeading { tx, ty } => {
                push_nums(&mut out, &[*tx, *ty]);
                "TD"
            }
            PdfOp::SetTextMatrix { matrix } => {
                push_nums(&mut out, matrix);
                "Tm"
            }
            PdfOp::NextLine => "T*",
            PdfOp::ShowText { text } => {
                push_string(&mut out, &encode_text(text, font, font_name)?);
                "Tj"
            }
            PdfOp::NextLineShowText { text } => {
                push_string(&mut out, &encode_text(text, font, font_name)?);
                "'"
            }
            PdfOp::NextLineShowTextSpaced { word_spacing, char_spacing, text } => {
                push_nums(&mut out, &[*word_spacing, *char_spacing]);
                push_string(&mut out, &encode_text(text, font, font_name)?);
                "\""
            }
            PdfOp::ShowTextArray { items } => {
                out.push(b'[');
                for item in items {
                    match item {
                        PdfTextArrayItem::Text { text } => super::lexer::write_string(&mut out, &encode_text(&PdfTextString::Text { text: text.clone() }, font, font_name)?),
                        PdfTextArrayItem::Codes { bytes } => super::lexer::write_string(&mut out, bytes),
                        PdfTextArrayItem::Adjust { amount } => {
                            out.push(b' ');
                            out.extend_from_slice(number_text(*amount).as_bytes());
                            out.push(b' ');
                        }
                    }
                }
                out.extend_from_slice(b"] ");
                "TJ"
            }
            PdfOp::SetGlyphWidth { wx, wy } => {
                push_nums(&mut out, &[*wx, *wy]);
                "d0"
            }
            PdfOp::SetGlyphWidthAndBox { wx, wy, llx, lly, urx, ury } => {
                push_nums(&mut out, &[*wx, *wy, *llx, *lly, *urx, *ury]);
                "d1"
            }
            PdfOp::SetStrokeColorSpace { name } => {
                push_name(&mut out, name);
                "CS"
            }
            PdfOp::SetFillColorSpace { name } => {
                push_name(&mut out, name);
                "cs"
            }
            PdfOp::SetStrokeColor { components } => {
                push_nums(&mut out, components);
                "SC"
            }
            PdfOp::SetFillColor { components } => {
                push_nums(&mut out, components);
                "sc"
            }
            PdfOp::SetStrokeColorN { components, pattern } => {
                push_nums(&mut out, components);
                if let Some(pattern) = pattern {
                    push_name(&mut out, pattern);
                }
                "SCN"
            }
            PdfOp::SetFillColorN { components, pattern } => {
                push_nums(&mut out, components);
                if let Some(pattern) = pattern {
                    push_name(&mut out, pattern);
                }
                "scn"
            }
            PdfOp::SetStrokeGray { gray } => {
                push_nums(&mut out, &[*gray]);
                "G"
            }
            PdfOp::SetFillGray { gray } => {
                push_nums(&mut out, &[*gray]);
                "g"
            }
            PdfOp::SetStrokeRgb { r, g, b } => {
                push_nums(&mut out, &[*r, *g, *b]);
                "RG"
            }
            PdfOp::SetFillRgb { r, g, b } => {
                push_nums(&mut out, &[*r, *g, *b]);
                "rg"
            }
            PdfOp::SetStrokeCmyk { c, m, y, k } => {
                push_nums(&mut out, &[*c, *m, *y, *k]);
                "K"
            }
            PdfOp::SetFillCmyk { c, m, y, k } => {
                push_nums(&mut out, &[*c, *m, *y, *k]);
                "k"
            }
            PdfOp::PaintShading { name } => {
                push_name(&mut out, name);
                "sh"
            }
            PdfOp::PaintXObject { name } => {
                push_name(&mut out, name);
                "Do"
            }
            PdfOp::InlineImage { image } => {
                out.extend_from_slice(b"BI /W ");
                out.extend_from_slice(image.width.to_string().as_bytes());
                out.extend_from_slice(b" /H ");
                out.extend_from_slice(image.height.to_string().as_bytes());
                if image.image_mask {
                    out.extend_from_slice(b" /IM true");
                } else {
                    out.extend_from_slice(b" /BPC ");
                    out.extend_from_slice(image.bits_per_component.max(1).to_string().as_bytes());
                    if let Some(color_space) = &image.color_space {
                        out.extend_from_slice(b" /CS ");
                        write_object(&mut out, &lower_colour_space_inline(color_space));
                    }
                }
                if !image.decode.is_empty() {
                    out.extend_from_slice(b" /D [");
                    for (index, value) in image.decode.iter().enumerate() {
                        if index > 0 {
                            out.push(b' ');
                        }
                        out.extend_from_slice(number_text(*value).as_bytes());
                    }
                    out.push(b']');
                }
                if image.interpolate {
                    out.extend_from_slice(b" /I true");
                }
                let (encoded, filter_entries) = encode_stream(&image.data, &image.filters);
                for entry in filter_entries.iter().chain(image.extra.iter()) {
                    out.push(b' ');
                    super::lexer::write_name(&mut out, &entry.key);
                    out.push(b' ');
                    write_object(&mut out, &entry.value);
                }
                out.extend_from_slice(b" ID ");
                out.extend_from_slice(&encoded);
                out.extend_from_slice(b"\nEI\n");
                continue;
            }
            PdfOp::MarkedContentPoint { tag } => {
                push_name(&mut out, tag);
                "MP"
            }
            PdfOp::MarkedContentPointWithProperties { tag, properties } => {
                push_name(&mut out, tag);
                push_properties(&mut out, properties);
                "DP"
            }
            PdfOp::BeginMarkedContent { tag } => {
                push_name(&mut out, tag);
                "BMC"
            }
            PdfOp::BeginMarkedContentWithProperties { tag, properties } => {
                push_name(&mut out, tag);
                push_properties(&mut out, properties);
                "BDC"
            }
            PdfOp::EndMarkedContent => "EMC",
            PdfOp::BeginCompatibility => "BX",
            PdfOp::EndCompatibility => "EX",
            PdfOp::Unknown { operator, operands } => {
                for operand in operands {
                    write_object(&mut out, operand);
                    out.push(b' ');
                }
                operator.as_str()
            }
        };
        out.extend_from_slice(word.as_bytes());
        out.push(b'\n');
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_properties(out: &mut Vec<u8>, properties: &PdfPropertyList) {
    match properties {
        PdfPropertyList::Named { name } => push_name(out, name),
        PdfPropertyList::Inline { entries } => {
            write_object(out, &PdfObject::Dict(entries.clone()));
            out.push(b' ');
        }
    }
}
//#endregion 🔖️Print

//#region 🔖️Extract
/// 🔤 The Unicode text `ops` show, decoding raw codes through `fonts` where possible; runs are
/// separated by newlines where the stream starts a new line.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn extract_text(ops: &[PdfOp], fonts: &dyn FontTable) -> String {
    let mut out = String::new();
    let mut current: Option<String> = None;
    let mut stack: Vec<Option<String>> = Vec::new();
    let newline = |out: &mut String| {
        if !out.is_empty() && !out.ends_with('\n') {
            out.push('\n');
        }
    };
    let show = |out: &mut String, text: &PdfTextString, font: Option<&FontCodec>| match text {
        PdfTextString::Text { text } => out.push_str(text),
        PdfTextString::Codes { bytes } => match font {
            Some(font) => match font.decode_text(bytes) {
                Some(text) => out.push_str(&text),
                None => {
                    for glyph in font.decode(bytes) {
                        out.push_str(glyph.text.as_deref().unwrap_or("\u{FFFD}"));
                    }
                }
            },
            None => out.extend(bytes.iter().map(|byte| if (0x20..=0x7E).contains(byte) { *byte as char } else { '\u{FFFD}' })),
        },
    };
    for op in ops {
        let font = current.as_deref().and_then(|name| fonts.font(name));
        match op {
            PdfOp::SetFont { name, .. } => current = Some(name.clone()),
            PdfOp::Save => stack.push(current.clone()),
            PdfOp::Restore => {
                if let Some(saved) = stack.pop() {
                    current = saved;
                }
            }
            PdfOp::ShowText { text } => show(&mut out, text, font),
            PdfOp::NextLineShowText { text } | PdfOp::NextLineShowTextSpaced { text, .. } => {
                newline(&mut out);
                show(&mut out, text, font);
            }
            PdfOp::ShowTextArray { items } => {
                for item in items {
                    match item {
                        PdfTextArrayItem::Text { text } => out.push_str(text),
                        PdfTextArrayItem::Codes { bytes } => show(&mut out, &PdfTextString::Codes { bytes: bytes.clone() }, font),
                        PdfTextArrayItem::Adjust { amount } => {
                            if *amount < -200.0 && !out.ends_with(' ') {
                                out.push(' ');
                            }
                        }
                    }
                }
            }
            PdfOp::NextLine | PdfOp::MoveText { .. } | PdfOp::MoveTextSetLeading { .. } | PdfOp::SetTextMatrix { .. } => newline(&mut out),
            _ => {}
        }
    }
    while out.ends_with('\n') {
        out.pop();
    }
    out
}
//#endregion 🔖️Extract

//#region 🔖️Walk
/// 🚶 Every resource name a content stream references, by category (fonts, XObjects, graphics
/// states, shadings, patterns, colour spaces, property lists) — what a lowering pass needs to
/// decide which document resources to bind.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ContentReferences {
    pub fonts: Vec<String>,
    pub x_objects: Vec<String>,
    pub ext_g_states: Vec<String>,
    pub shadings: Vec<String>,
    pub patterns: Vec<String>,
    pub color_spaces: Vec<String>,
    pub properties: Vec<String>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_unique(list: &mut Vec<String>, name: &str) {
    if !list.iter().any(|existing| existing == name) {
        list.push(name.to_string());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn content_references(ops: &[PdfOp]) -> ContentReferences {
    let mut refs = ContentReferences::default();
    for op in ops {
        match op {
            PdfOp::SetFont { name, .. } => push_unique(&mut refs.fonts, name),
            PdfOp::PaintXObject { name } => push_unique(&mut refs.x_objects, name),
            PdfOp::SetExtGState { name } => push_unique(&mut refs.ext_g_states, name),
            PdfOp::PaintShading { name } => push_unique(&mut refs.shadings, name),
            PdfOp::SetStrokeColorN { pattern: Some(name), .. } | PdfOp::SetFillColorN { pattern: Some(name), .. } => push_unique(&mut refs.patterns, name),
            PdfOp::SetStrokeColorSpace { name } | PdfOp::SetFillColorSpace { name } => {
                if !matches!(name.as_str(), "DeviceGray" | "DeviceRGB" | "DeviceCMYK" | "Pattern") {
                    push_unique(&mut refs.color_spaces, name);
                }
            }
            PdfOp::MarkedContentPointWithProperties { properties: PdfPropertyList::Named { name }, .. } | PdfOp::BeginMarkedContentWithProperties { properties: PdfPropertyList::Named { name }, .. } => push_unique(&mut refs.properties, name),
            _ => {}
        }
    }
    refs
}
//#endregion 🔖️Walk

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
