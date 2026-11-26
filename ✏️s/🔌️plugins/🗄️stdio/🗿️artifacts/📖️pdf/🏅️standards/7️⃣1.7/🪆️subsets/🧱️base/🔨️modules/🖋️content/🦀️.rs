//! 🖋️ Content streams (ISO 32000-1 §7.8.2, §8, §9): every operator of Table 51 parsed into
//! typed [`PdfOp`]s and printed back canonically. Text-showing operands are decoded through the
//! font the stream selected (`Tf`) and re-encoded through it on print, so the typed lane carries
//! Unicode wherever the font can vouch for it and raw codes everywhere else.

use crate::standards::v1_7::subsets::base::modules::colour::{lift_colour_space_inline, lower_colour_space_inline};
use crate::standards::v1_7::subsets::base::modules::filters::{decode_stream, encode_stream};
use crate::standards::v1_7::subsets::base::modules::fonts::FontCodec;
use crate::standards::v1_7::subsets::base::modules::images::{admit_packed_samples, packed_image_geometry};
use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, number_text, write_object, ContentToken, ContentTokenCursor, InlineImageBoundary, InlineImageSpanCursor, InlineImageSpanStep};
use crate::standards::v1_7::subsets::base::io::error::{PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfBaseEncoding, PdfDictEntry, PdfInlineImage, PdfImageBody, PdfLineCap, PdfLineJoin, PdfObject, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString};
use std::collections::HashMap;

#[path = "🧵️program/🦀️.rs"]
mod program;
pub use program::{ContentProgramCursor, ContentProgramEvent, ContentProgramReceipt};

//#region 🔖️FontTable
/// 🔢️ Physical width of a qualified, fixed-width PDF font codespace.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PdfFixedCodeWidth { One = 1, Two = 2 }

/// 🔤 The fonts a content stream may select, by resource name.
pub trait FontTable {
    fn font(&self, name: &str) -> Option<&FontCodec>;
    /// 🔡 Supplies the qualified Type1 encoding for measured content turns.
    fn controlled_type1(&self, _name: &str) -> Option<PdfBaseEncoding> { None }
    /// 🔤️ Decodes one reversible code of a qualified Type1 font, honouring its `/Differences`.
    fn controlled_type1_text(&self, name: &str, byte: u8) -> Option<&'static str> {
        crate::standards::v1_7::subsets::base::modules::fonts::controlled_type1_text(self.controlled_type1(name)?, byte)
    }
    /// 🔒️ Supplies a separately admitted codespace whose physical codes must be retained.
    fn controlled_code_width(&self, _name: &str) -> Option<PdfFixedCodeWidth> { None }
    fn content_artifact(&self,_reference:&semio_framework_artifact_reference::ArtifactRef)->PResult<PdfObject>{Err(PdfEngineError::Unsupported("content artifact resolution is unavailable".into()))}
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
const MAX_CONTROLLED_REAL_DIGITS: usize = 128;

/// 🔢️ Converts a bounded PDF decimal exactly as the reference parser without heap scratch.
pub fn bounded_number(value: &PdfObject) -> Result<Option<f64>, &'static str> {
    use std::io::Write;
    match value {
        PdfObject::Int(value) => Ok(Some(*value as f64)),
        PdfObject::Real(decimal) if bounded_spelling(value) => {
            let mut text = [0u8; MAX_CONTROLLED_REAL_DIGITS + 4];
            let mut remaining = text.as_mut_slice();
            write!(&mut remaining, "{decimal}").map_err(|_| "PDF decimal exceeds bounded scratch")?;
            let length = MAX_CONTROLLED_REAL_DIGITS + 4 - remaining.len();
            Ok(std::str::from_utf8(&text[..length]).ok().and_then(|text| text.parse().ok()))
        }
        PdfObject::Real(_) => Err("PDF decimal exceeds bounded scratch"),
        _ => Ok(None),
    }
}

fn bounded_spelling(value: &PdfObject) -> bool {
    match value {
        PdfObject::Real(decimal) => decimal.coefficient.len() <= MAX_CONTROLLED_REAL_DIGITS && decimal.scale as usize <= MAX_CONTROLLED_REAL_DIGITS,
        _ => true,
    }
}

fn number(value: &PdfObject) -> Option<f64> {
    bounded_number(value).unwrap_or_else(|_| value.as_f64())
}

fn num(operands: &[PdfObject], index: usize) -> f64 {
    operands.get(index).and_then(number).unwrap_or(0.0)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn nums(operands: &[PdfObject]) -> Vec<f64> {
    operands.iter().filter_map(number).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn name(operands: &[PdfObject], index: usize) -> Option<String> {
    operands.get(index).and_then(PdfObject::as_name).map(str::to_string)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn matrix(operands: &[PdfObject]) -> Option<[f64; 6]> {
    let mut numbers = operands.iter().filter_map(number);
    Some([numbers.next()?, numbers.next()?, numbers.next()?, numbers.next()?, numbers.next()?, numbers.next()?])
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
        None if bytes.iter().all(|byte| plain_text_byte(*byte)) => {
            return PdfTextString::Text { text: bytes.iter().map(|byte| *byte as char).collect() };
        }
        None => {}
    }
    PdfTextString::Codes { codes:font.map_or_else(||bytes.iter().map(|byte|u32::from(*byte)).collect(),|font|font.character_codes(bytes).collect()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn text_array(items: &[PdfObject], font: Option<&FontCodec>) -> Vec<PdfTextArrayItem> {
    items
        .iter()
        .filter_map(|item| match item {
            PdfObject::Str(bytes) => Some(array_text_item(text_string(bytes, font))),
            other => number(other).map(|amount| PdfTextArrayItem::Adjust { amount }),
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

/// 🔢️ Reads an inline-image dictionary integer within the typed image domain.
fn inline_image_integer(dict: &[PdfDictEntry], key: &str, default: Option<u32>) -> PResult<u32> {
    let value = match dict_get(dict, key) {
        Some(PdfObject::Int(value)) => u32::try_from(*value).ok().filter(|value| *value > 0),
        None => default,
        _ => None,
    };
    value.ok_or_else(|| PdfEngineError::Malformed(format!("invalid inline image {key}")))
}

/// 🖼️ Reads the authored source span through the same framing cursor used by retained parsers.
fn inline_image_body(data: &[u8], id_end: usize, boundary: InlineImageBoundary) -> PResult<(&[u8], usize)> {
    let mut cursor = InlineImageSpanCursor::new(id_end, boundary);
    loop {
        match cursor.advance(data, 4096, false).1 {
            Ok(InlineImageSpanStep::More) => {}
            Ok(InlineImageSpanStep::Complete(span)) => return Ok((&data[span.start..span.end], span.resume)),
            Ok(InlineImageSpanStep::Cancelled) => return Err(PdfEngineError::Malformed("inline image framing was cancelled".into())),
            Err(reason) => return Err(PdfEngineError::Malformed(reason.into())),
        }
    }
}

#[derive(Default)]
struct FontState {
    current: Option<String>,
    saved: Option<Box<SavedFont>>,
}

struct SavedFont {
    name: Option<String>,
    next: Option<Box<SavedFont>>,
}

impl FontState {
    fn save(&mut self) -> PdfOp {
        self.saved = Some(Box::new(SavedFont { name: self.current.clone(), next: self.saved.take() }));
        PdfOp::Save
    }

    fn restore(&mut self) -> PdfOp {
        if let Some(mut saved) = self.saved.take() {
            self.current = saved.name.take();
            self.saved = saved.next.take();
        }
        PdfOp::Restore
    }

    fn select(&mut self, operands: &[PdfObject]) -> PdfOp {
        let name = name(operands, 0).unwrap_or_default();
        self.current = Some(name.clone());
        PdfOp::SetFont { name, size: num(operands, 1) }
    }

    fn codec<'a>(&self, fonts: &'a dyn FontTable) -> Option<&'a FontCodec> {
        self.current.as_deref().and_then(|name| fonts.font(name))
    }
}

impl Drop for FontState {
    fn drop(&mut self) {
        while let Some(mut frame) = self.saved.take() {
            self.saved = frame.next.take();
        }
    }
}

fn plain_text_byte(byte: u8) -> bool {
    (0x20..=0x7E).contains(&byte)
}

#[derive(Clone, Copy)]
enum TextShow {
    Here,
    Next,
    Spaced { word_spacing: f64, char_spacing: f64 },
}

impl TextShow {
    fn new(word: &str, operands: &[PdfObject]) -> Option<Self> {
        Some(match word {
            "Tj" => Self::Here,
            "'" => Self::Next,
            "\"" => Self::Spaced { word_spacing: num(operands, 0), char_spacing: num(operands, 1) },
            _ => return None,
        })
    }

    fn bytes(self, operands: &[PdfObject]) -> &[u8] {
        let index = if matches!(self, Self::Spaced { .. }) { 2 } else { 0 };
        operands.get(index).and_then(PdfObject::as_str_bytes).unwrap_or(&[])
    }

    fn finish(self, text: PdfTextString) -> PdfOp {
        match self {
            Self::Here => PdfOp::ShowText { text },
            Self::Next => PdfOp::NextLineShowText { text },
            Self::Spaced { word_spacing, char_spacing } => PdfOp::NextLineShowTextSpaced { word_spacing, char_spacing, text },
        }
    }
}

fn fixed_operator(word: &str, operands: &[PdfObject]) -> Option<PdfOp> {
    Some(match word {
        "w" => PdfOp::SetLineWidth { width: num(operands, 0) },
        "J" => PdfOp::SetLineCap { cap: match num(operands, 0) as i64 { 1 => PdfLineCap::Round, 2 => PdfLineCap::Square, _ => PdfLineCap::Butt } },
        "j" => PdfOp::SetLineJoin { join: match num(operands, 0) as i64 { 1 => PdfLineJoin::Round, 2 => PdfLineJoin::Bevel, _ => PdfLineJoin::Miter } },
        "M" => PdfOp::SetMiterLimit { limit: num(operands, 0) },
        "i" => PdfOp::SetFlatness { flatness: num(operands, 0) },
        "cm" => PdfOp::Transform { matrix: matrix(operands).unwrap_or([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]) },
        "m" => PdfOp::MoveTo { x: num(operands, 0), y: num(operands, 1) },
        "l" => PdfOp::LineTo { x: num(operands, 0), y: num(operands, 1) },
        "c" => PdfOp::CurveTo { x1: num(operands, 0), y1: num(operands, 1), x2: num(operands, 2), y2: num(operands, 3), x3: num(operands, 4), y3: num(operands, 5) },
        "v" => PdfOp::CurveToInitial { x2: num(operands, 0), y2: num(operands, 1), x3: num(operands, 2), y3: num(operands, 3) },
        "y" => PdfOp::CurveToFinal { x1: num(operands, 0), y1: num(operands, 1), x3: num(operands, 2), y3: num(operands, 3) },
        "h" => PdfOp::ClosePath,
        "re" => PdfOp::Rectangle { x: num(operands, 0), y: num(operands, 1), width: num(operands, 2), height: num(operands, 3) },
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
        "Tc" => PdfOp::SetCharSpacing { spacing: num(operands, 0) },
        "Tw" => PdfOp::SetWordSpacing { spacing: num(operands, 0) },
        "Tz" => PdfOp::SetHorizontalScale { scale: num(operands, 0) },
        "TL" => PdfOp::SetLeading { leading: num(operands, 0) },
        "Tr" => PdfOp::SetTextRenderingMode { mode: num(operands, 0).max(0.0) as u32 },
        "Ts" => PdfOp::SetTextRise { rise: num(operands, 0) },
        "Td" => PdfOp::MoveText { tx: num(operands, 0), ty: num(operands, 1) },
        "TD" => PdfOp::MoveTextSetLeading { tx: num(operands, 0), ty: num(operands, 1) },
        "Tm" => PdfOp::SetTextMatrix { matrix: matrix(operands).unwrap_or([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]) },
        "T*" => PdfOp::NextLine,
        "d0" => PdfOp::SetGlyphWidth { wx: num(operands, 0), wy: num(operands, 1) },
        "d1" => PdfOp::SetGlyphWidthAndBox { wx: num(operands, 0), wy: num(operands, 1), llx: num(operands, 2), lly: num(operands, 3), urx: num(operands, 4), ury: num(operands, 5) },
        "G" => PdfOp::SetStrokeGray { gray: num(operands, 0) },
        "g" => PdfOp::SetFillGray { gray: num(operands, 0) },
        "RG" => PdfOp::SetStrokeRgb { r: num(operands, 0), g: num(operands, 1), b: num(operands, 2) },
        "rg" => PdfOp::SetFillRgb { r: num(operands, 0), g: num(operands, 1), b: num(operands, 2) },
        "K" => PdfOp::SetStrokeCmyk { c: num(operands, 0), m: num(operands, 1), y: num(operands, 2), k: num(operands, 3) },
        "k" => PdfOp::SetFillCmyk { c: num(operands, 0), m: num(operands, 1), y: num(operands, 2), k: num(operands, 3) },
        "EMC" => PdfOp::EndMarkedContent,
        "BX" => PdfOp::BeginCompatibility,
        "EX" => PdfOp::EndCompatibility,
        _ => return None,
    })
}

fn named_operator(word: &str, value: impl FnOnce() -> String) -> Option<PdfOp> {
    Some(match word {
        "ri" => PdfOp::SetRenderingIntent { intent: value() },
        "gs" => PdfOp::SetExtGState { name: value() },
        "CS" => PdfOp::SetStrokeColorSpace { name: value() },
        "cs" => PdfOp::SetFillColorSpace { name: value() },
        "sh" => PdfOp::PaintShading { name: value() },
        "Do" => PdfOp::PaintXObject { name: value() },
        "MP" => PdfOp::MarkedContentPoint { tag: value() },
        "BMC" => PdfOp::BeginMarkedContent { tag: value() },
        _ => return None,
    })
}

fn first_array(operands: &[PdfObject]) -> &[PdfObject] {
    operands.first().and_then(PdfObject::as_array).unwrap_or(&[])
}

#[derive(Clone, Copy)]
enum NumericArray {
    Dash,
    Stroke,
    Fill,
    StrokePattern,
    FillPattern,
}

impl NumericArray {
    fn new(word: &str) -> Option<Self> {
        Some(match word {
            "d" => Self::Dash,
            "SC" => Self::Stroke,
            "sc" => Self::Fill,
            "SCN" => Self::StrokePattern,
            "scn" => Self::FillPattern,
            _ => return None,
        })
    }

    fn values(self, operands: &[PdfObject]) -> &[PdfObject] {
        if matches!(self, Self::Dash) { first_array(operands) } else { operands }
    }

    fn pattern(self, operands: &[PdfObject]) -> Option<&str> {
        if matches!(self, Self::StrokePattern | Self::FillPattern) {
            operands.last().and_then(PdfObject::as_name)
        } else {
            None
        }
    }

    fn finish(self, components: Vec<f64>, operands: &[PdfObject]) -> PdfOp {
        match self {
            Self::Dash => PdfOp::SetDash { array: components, phase: num(operands, 1) },
            Self::Stroke => PdfOp::SetStrokeColor { components },
            Self::Fill => PdfOp::SetFillColor { components },
            Self::StrokePattern => PdfOp::SetStrokeColorN { components, pattern: self.pattern(operands).map(str::to_owned) },
            Self::FillPattern => PdfOp::SetFillColorN { components, pattern: self.pattern(operands).map(str::to_owned) },
        }
    }
}

fn array_text_item(text: PdfTextString) -> PdfTextArrayItem {
    match text {
        PdfTextString::Text { text } => PdfTextArrayItem::Text { text },
        PdfTextString::Codes { codes } => PdfTextArrayItem::Codes { codes },
    }
}

fn reference_operator(word: &str, operands: &mut Vec<PdfObject>, fonts: &mut FontState, table: &dyn FontTable) -> PdfOp {
    if let Some(op) = fixed_operator(word, operands) {
        return op;
    }
    if let Some(op) = named_operator(word, || name(operands, 0).unwrap_or_default()) {
        return op;
    }
    if let Some(numbers) = NumericArray::new(word) {
        return numbers.finish(nums(numbers.values(operands)), operands);
    }
    let font = fonts.codec(table);
    if let Some(show) = TextShow::new(word, operands) {
        return show.finish(text_string(show.bytes(operands), font));
    }
    match word {
        "q" => fonts.save(),
        "Q" => fonts.restore(),
        "Tf" => fonts.select(operands),
        "TJ" => PdfOp::ShowTextArray { items: text_array(first_array(operands), font) },
        "DP" => PdfOp::MarkedContentPointWithProperties { tag: name(operands, 0).unwrap_or_default(), properties: property_list(operands.get(1)) },
        "BDC" => PdfOp::BeginMarkedContentWithProperties { tag: name(operands, 0).unwrap_or_default(), properties: property_list(operands.get(1)) },
        other => PdfOp::Unknown { operator: other.to_string(), operands: std::mem::take(operands) },
    }
}

fn parse_inline_image(data: &[u8], tokens: &mut ContentTokenCursor, resources: &mut dyn crate::standards::v1_7::subsets::base::io::foreign_artifacts::PdfArtifactResourcePort) -> PResult<PdfOp> {
    let mut entries: Vec<PdfDictEntry> = Vec::new();
    let mut key: Option<String> = None;
    loop {
        match tokens.next_token(data) {
            Ok(ContentToken::Value(PdfObject::Name(name))) if key.is_none() => key = Some(name),
            Ok(ContentToken::Value(value)) => {
                if let Some(k) = key.take() {
                    let expanded = expand_inline_key(&k).to_string();
                    let value = expand_inline_value(&expanded, value);
                    entries.push(PdfDictEntry { key: expanded, value });
                }
            }
            Ok(ContentToken::Operator { start, end }) if &data[start..end] == b"ID" => break,
            Ok(ContentToken::Operator { .. }) => {}
            Ok(ContentToken::End) | Err(_) => return Err(PdfEngineError::Malformed("missing inline image ID operator".into())),
        }
    }
    let width = inline_image_integer(&entries, "Width", None)?;
    let height = inline_image_integer(&entries, "Height", None)?;
    let bits_per_component = inline_image_integer(&entries, "BitsPerComponent", Some(0))?;
    let image_mask = dict_get(&entries, "ImageMask").and_then(PdfObject::as_bool) == Some(true);
    let color_space = dict_get(&entries, "ColorSpace").map(lift_colour_space_inline);
    let components = if image_mask { 1 } else { color_space.as_ref().and_then(|color| color.components()).unwrap_or(1) };
    let sample_bits = if image_mask { 1 } else { bits_per_component };
    let filter = match dict_get(&entries, "Filter") {
        None | Some(PdfObject::Null) => None,
        Some(PdfObject::Array(items)) => items.first(),
        value => value,
    };
    let boundary = match filter {
        None => InlineImageBoundary::Packed(packed_image_geometry(width, height, components, sample_bits)?.1),
        Some(value) if value.as_name() == Some("ASCII85Decode") => InlineImageBoundary::Ascii85,
        _ => InlineImageBoundary::Delimited,
    };
    let (raw, resume) = inline_image_body(data, tokens.position(), boundary)?;
    if !tokens.resume_after_image(resume) { return Err(PdfEngineError::Malformed("invalid inline image span".into())); }
    let (decoded, filters) = decode_stream(&entries, raw)?;
    let mut image = PdfInlineImage {
        width, height, bits_per_component, color_space, image_mask,
        decode: entries.iter().find(|e| e.key == "Decode").and_then(|e| e.value.as_array()).map(nums).unwrap_or_default(),
        interpolate: entries.iter().any(|e| e.key == "Interpolate" && e.value.as_bool() == Some(true)),
        body:PdfImageBody::Samples {packed:Vec::new()},
        extra: entries.into_iter().filter(|e| !matches!(e.key.as_str(), "Width" | "Height" | "BitsPerComponent" | "ColorSpace" | "ImageMask" | "Decode" | "Interpolate" | "Filter" | "DecodeParms" | "Length")).collect(),
    };
    image.body=if let Some(filter)=filters.iter().find(|filter|filter.is_image_codec()) {let kind=match filter {crate::standards::v1_7::subsets::base::schema::snapshot::PdfStreamFilter::Dct {..}=>"s.stdio.jpeg",crate::standards::v1_7::subsets::base::schema::snapshot::PdfStreamFilter::Jpx=>"s.stdio.jpeg2000",crate::standards::v1_7::subsets::base::schema::snapshot::PdfStreamFilter::Ccitt {..}=>"s.stdio.ccitt",crate::standards::v1_7::subsets::base::schema::snapshot::PdfStreamFilter::Jbig2 {..}=>"s.stdio.jbig2",_=>unreachable!()};PdfImageBody::Artifact {reference:resources.admit(kind,PdfObject::Stream {dict:Vec::new(),data:crate::standards::v1_7::subsets::base::schema::snapshot::stream_bytes(&decoded),filters})?}}else{PdfImageBody::Samples {packed:admit_packed_samples(width, height, components, sample_bits, decoded)?}};
    Ok(PdfOp::InlineImage { image: Box::new(image) })
}

/// 📖 Parses a content stream into typed operators.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_content(data: &[u8], fonts: &dyn FontTable,resources:&mut dyn crate::standards::v1_7::subsets::base::io::foreign_artifacts::PdfArtifactResourcePort) -> PResult<Vec<PdfOp>> {
    let mut ops = Vec::new();
    let mut tokens = ContentTokenCursor::new(0);
    let mut operands: Vec<PdfObject> = Vec::new();
    let mut state = FontState::default();
    loop {
        let token = match tokens.next_token(data) {
            Ok(token) => token,
            Err(_) if tokens.recover() => continue,
            Err(reason) => return Err(PdfEngineError::Malformed(reason.into())),
        };
        let word = match token {
            ContentToken::End => break,
            ContentToken::Value(value) => {
                if operands.len() < 64 {
                    operands.push(value);
                }
                continue;
            }
            ContentToken::Operator { start, end } => String::from_utf8_lossy(&data[start..end]),
        };
        let op = if word == "BI" {
            parse_inline_image(data, &mut tokens, resources)?
        } else {
            reference_operator(&word, &mut operands, &mut state, fonts)
        };
        operands.clear();
        ops.push(op);
    }
    Ok(ops)
}

/// 📏️ What [`parse_content`] types from one content stream, measured without typing it: the operator count, an upper bound of the heap bytes its kept operands own once typed (text decoded through any codec included, every operand of an operator the parser keeps untyped held as a whole value), and of the bytes its inline images own (decoded packed bytes, one `u32` per sample, the boxed image and its dictionary).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PdfContentExtent {
    pub operators: usize,
    pub operand_bytes: usize,
    pub inline_bytes: usize,
}

/// 📐️ Measures [`PdfContentExtent`] in one token pass; `None` wherever the parser would recover from or refuse a token, so a caller keeps its worst-case bound.
pub fn content_extent(data: &[u8]) -> Option<PdfContentExtent> {
    let mut extent = PdfContentExtent::default();
    let mut tokens = ContentTokenCursor::new(0);
    let mut operands = 0;
    loop {
        match tokens.next_token(data).ok()? {
            ContentToken::End => return Some(extent),
            ContentToken::Value(value) => {
                if operands < 64 { extent.operand_bytes += typed_operand_bytes(&value); }
                operands += 1;
            }
            ContentToken::Operator { start, end } => {
                let word = std::str::from_utf8(&data[start..end]).unwrap_or("");
                if !typed_operator(word) { extent.operand_bytes += operands.min(64) * std::mem::size_of::<PdfObject>() + word.len(); }
                operands = 0;
                extent.operators += 1;
                if word == "BI" { extent.inline_bytes = extent.inline_bytes.saturating_add(inline_image_extent(data, &mut tokens)?); }
            }
        }
    }
}

/// 🔖️ Whether [`reference_operator`] types `word` (every other operator keeps its operands as untyped values).
fn typed_operator(word: &str) -> bool {
    fixed_operator(word, &[]).is_some() || named_operator(word, String::new).is_some() || NumericArray::new(word).is_some()
        || TextShow::new(word, &[]).is_some() || matches!(word, "q" | "Q" | "Tf" | "TJ" | "DP" | "BDC" | "BI")
}

/// 📏️ Bounds the heap one operand owns once typed: none for a number kept inline, one `f64` for a number collected into a component list, four bytes per string byte (a decoded code is at most one `u32` or four UTF-8 bytes), a value node per array item and dictionary entry.
fn typed_operand_bytes(value: &PdfObject) -> usize {
    match value {
        PdfObject::Int(_) | PdfObject::Real(_) => std::mem::size_of::<f64>(),
        PdfObject::Name(text) | PdfObject::Text(text) => text.len(),
        PdfObject::Str(bytes) => 4 * bytes.len(),
        PdfObject::Array(items) => items.iter().map(|item| std::mem::size_of::<PdfObject>() + typed_operand_bytes(item)).sum(),
        PdfObject::Dict(entries) => entries.iter().map(|entry| std::mem::size_of::<PdfDictEntry>() + entry.key.len() + typed_operand_bytes(&entry.value)).sum(),
        _ => 0,
    }
}

/// 🖼️ Frames one inline image exactly as [`parse_inline_image`] does and bounds what typing it owns.
fn inline_image_extent(data: &[u8], tokens: &mut ContentTokenCursor) -> Option<usize> {
    let mut entries: Vec<PdfDictEntry> = Vec::new();
    let mut key: Option<String> = None;
    loop {
        match tokens.next_token(data).ok()? {
            ContentToken::Value(PdfObject::Name(name)) if key.is_none() => key = Some(name),
            ContentToken::Value(value) => if let Some(k) = key.take() {
                let expanded = expand_inline_key(&k).to_string();
                let value = expand_inline_value(&expanded, value);
                entries.push(PdfDictEntry { key: expanded, value });
            },
            ContentToken::Operator { start, end } if &data[start..end] == b"ID" => break,
            ContentToken::Operator { .. } => {}
            ContentToken::End => return None,
        }
    }
    let width = inline_image_integer(&entries, "Width", None).ok()?;
    let height = inline_image_integer(&entries, "Height", None).ok()?;
    let bits_per_component = inline_image_integer(&entries, "BitsPerComponent", Some(0)).ok()?;
    let image_mask = dict_get(&entries, "ImageMask").and_then(PdfObject::as_bool) == Some(true);
    let components = if image_mask { 1 } else { dict_get(&entries, "ColorSpace").map(lift_colour_space_inline).and_then(|color| color.components()).unwrap_or(1) };
    let (_, packed) = packed_image_geometry(width, height, components, if image_mask { 1 } else { bits_per_component }).ok()?;
    let filter = match dict_get(&entries, "Filter") {
        None | Some(PdfObject::Null) => None,
        Some(PdfObject::Array(items)) => items.first(),
        value => value,
    };
    let boundary = match filter {
        None => InlineImageBoundary::Packed(packed),
        Some(value) if value.as_name() == Some("ASCII85Decode") => InlineImageBoundary::Ascii85,
        _ => InlineImageBoundary::Delimited,
    };
    let (raw, resume) = inline_image_body(data, tokens.position(), boundary).ok()?;
    if !tokens.resume_after_image(resume) { return None; }
    let dictionary: usize = entries.iter().map(|entry| std::mem::size_of::<PdfDictEntry>() + entry.key.len() + typed_operand_bytes(&entry.value)).sum();
    packed.max(raw.len()).checked_add(raw.len())?.checked_add(std::mem::size_of::<PdfInlineImage>() + dictionary)
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
    crate::standards::v1_7::subsets::base::modules::lexer::write_name(out, name);
    out.push(b' ');
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_text(text: &PdfTextString, font: Option<&FontCodec>, font_name: Option<&str>) -> PResult<Vec<u8>> {
    match text {
        PdfTextString::Codes { codes } => match font {Some(font)=>font.encode_codes(codes).ok_or_else(||PdfEngineError::Malformed("logical character code exceeds current font codespace".into())),None=>codes.iter().map(|code|u8::try_from(*code).map_err(|_|PdfEngineError::Malformed("logical character code needs a declared composite font".into()))).collect()},
        PdfTextString::Text { text } => match font {
            Some(font) => font.encode(text).ok_or_else(|| PdfEngineError::Unsupported(format!("font /{} cannot show {text:?}", font_name.unwrap_or("?")))),
            None if text.bytes().all(|byte| (0x20..=0x7E).contains(&byte)) => Ok(text.as_bytes().to_vec()),
            None => Err(PdfEngineError::Malformed(format!("text {text:?} shown before any font was selected (or font /{} is unknown)", font_name.unwrap_or("?")))),
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_string(out: &mut Vec<u8>, bytes: &[u8]) {
    crate::standards::v1_7::subsets::base::modules::lexer::write_string(out, bytes);
    out.push(b' ');
}

/// 🖨️ Prints typed operators as a content stream (one operator per line).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_content<'a>(ops: impl IntoIterator<Item = &'a PdfOp>, fonts: &dyn FontTable) -> PResult<Vec<u8>> {
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
                        PdfTextArrayItem::Text { text } => crate::standards::v1_7::subsets::base::modules::lexer::write_string(&mut out, &encode_text(&PdfTextString::Text { text: text.clone() }, font, font_name)?),
                        PdfTextArrayItem::Codes { codes } => crate::standards::v1_7::subsets::base::modules::lexer::write_string(&mut out, &encode_text(&PdfTextString::Codes {codes:codes.clone()},font,font_name)?),
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
                        write_object(&mut out, &lower_colour_space_inline(color_space)?);
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
                let (data,filters)=match &image.body {PdfImageBody::Samples {packed}=>{image.packed_samples().transpose().map_err(|reason|PdfEngineError::Unsupported(reason.into()))?;(packed.clone(),Vec::new())},PdfImageBody::Artifact {reference}=>{let PdfObject::Stream {data,filters,..}=fonts.content_artifact(reference)? else {return Err(PdfEngineError::Unsupported("inline image artifact is not a native stream".into()));};if !filters.iter().any(|filter|filter.is_image_codec()){return Err(PdfEngineError::Unsupported("referenced image artifact has no native image codec".into()));}(data.to_vec_owner(),filters)}};
                let (encoded, filter_entries) = encode_stream(&data, &filters);
                for entry in filter_entries.iter().chain(image.extra.iter()) {
                    out.push(b' ');
                    crate::standards::v1_7::subsets::base::modules::lexer::write_name(&mut out, &entry.key);
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
pub fn extract_text<'a>(ops: impl IntoIterator<Item = &'a PdfOp>, fonts: &dyn FontTable) -> String {
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
        PdfTextString::Codes { codes } => match font {
            Some(font)=>{for glyph in font.decode_codes(codes) {out.push_str(glyph.text.as_deref().unwrap_or("\u{FFFD}"));}},
            None=>out.extend(codes.iter().map(|code|if(0x20..=0x7E).contains(code){char::from_u32(*code).unwrap()}else{'\u{FFFD}'})),
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
                        PdfTextArrayItem::Codes { codes } => show(&mut out, &PdfTextString::Codes { codes: codes.clone() }, font),
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
pub fn content_references<'a>(ops: impl IntoIterator<Item = &'a PdfOp>) -> ContentReferences {
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

//#endregion 🧪️Tests
