//! 🔗️ Object model of the PDF document model: the indirect reference, dictionary entries, stream filters and predictors, decimals, objects, indirect objects and the geometry aliases. See [`super`].

use super::document_metadata::PdfDate;
use std::fmt;

//#region 🔖️ObjectModel
/// 🔗️ An indirect-object reference `N G R` — also the `objects` collection's diff KEY (the
/// `(id,gen)` pair per the recipe's "numeric id" key kind; `Hash` is needed by the diff module's
/// key-transport absorb maps).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct ObjRef {
    pub num: u32,
    pub gen: u16,
}

/// 🧩 One `key`/`value` pair of a PDF dictionary. A `Vec` (not a map) so parse order survives
/// losslessly -- PDF dictionaries have no canonical key order and real files vary widely.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfDictEntry {
    pub key: String,
    pub value: PdfObject,
}

impl PdfDictEntry {
    pub fn new(key: impl Into<String>, value: PdfObject) -> Self {
        Self { key: key.into(), value }
    }
}

/// 🎛️ Logical predictor parameters attached to a Flate/LZW stream filter (§7.4.4.4).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfPredictor {
    pub predictor: u32,
    pub colors: u32,
    pub bits_per_component: u32,
    pub columns: u32,
}

/// 📠️ CCITTFaxDecode parameters (§7.4.6, Table 11).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfCcittParameters {
    #[value(default)]
    pub k: i32,
    #[value(default = "PdfCcittParameters::default_columns")]
    pub columns: u32,
    #[value(default)]
    pub rows: u32,
    #[value(default)]
    pub black_is_1: bool,
    #[value(default)]
    pub encoded_byte_align: bool,
    #[value(default)]
    pub end_of_line: bool,
    #[value(default = "PdfCcittParameters::default_end_of_block")]
    pub end_of_block: bool,
    #[value(default)]
    pub damaged_rows_before_error: u32,
}

impl PdfCcittParameters {
    fn default_columns() -> u32 {
        1728
    }
    fn default_end_of_block() -> bool {
        true
    }
}

/// 🗜️ Stream filter concepts (§7.4). The first five have logical codecs: their `data` is the
/// DECODED byte sequence. The image codecs (`Dct`, `Jpx`, `Ccitt`, `Jbig2`) are retained as they
/// stand — `data` is still the encoded image; a conforming reader hands such a stream to its image
/// decoder, and this codec hands it back byte for byte. `Crypt` names an identity/explicit crypt
/// filter (§7.6.5) and is likewise transparent.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfStreamFilter {
    Flate { predictor: Option<PdfPredictor> },
    Lzw { predictor: Option<PdfPredictor>, early_change: bool },
    AsciiHex,
    Ascii85,
    RunLength,
    Dct { color_transform: Option<u32> },
    Jpx,
    Ccitt { parameters: PdfCcittParameters },
    Jbig2 { #[value(with="pack::value::bytes::optional", serialize_controlled_with="pack::value::bytes::optional::to_value_controlled", deserialize_controlled_with="pack::value::bytes::optional::from_value_controlled", retire_with="std::mem::drop")] globals: Option<Vec<u8>> },
    Crypt { name: Option<String> },
}

impl PdfStreamFilter {
    /// 🖼️ Whether this filter is an image codec whose payload stays encoded in the typed model.
    pub fn is_image_codec(&self) -> bool {
        matches!(self, PdfStreamFilter::Dct { .. } | PdfStreamFilter::Jpx | PdfStreamFilter::Ccitt { .. } | PdfStreamFilter::Jbig2 { .. })
    }
    /// 🏷️ The `/Filter` name this concept is written as.
    pub fn name(&self) -> &'static str {
        match self {
            PdfStreamFilter::Flate { .. } => "FlateDecode",
            PdfStreamFilter::Lzw { .. } => "LZWDecode",
            PdfStreamFilter::AsciiHex => "ASCIIHexDecode",
            PdfStreamFilter::Ascii85 => "ASCII85Decode",
            PdfStreamFilter::RunLength => "RunLengthDecode",
            PdfStreamFilter::Dct { .. } => "DCTDecode",
            PdfStreamFilter::Jpx => "JPXDecode",
            PdfStreamFilter::Ccitt { .. } => "CCITTFaxDecode",
            PdfStreamFilter::Jbig2 { .. } => "JBIG2Decode",
            PdfStreamFilter::Crypt { .. } => "Crypt",
        }
    }
}

/// 🔢️ Exact logical PDF real number represented as decimal coefficient and scale.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfDecimal {
    pub negative: bool,
    pub coefficient: String,
    pub scale: u32,
}

impl PdfDecimal {
    pub fn parse(text: &str) -> Result<Self, String> {
        let (negative, unsigned) = match text.as_bytes().first() {
            Some(b'-') => (true, &text[1..]),
            Some(b'+') => (false, &text[1..]),
            _ => (false, text),
        };
        let (integer, fraction) = unsigned.split_once('.').ok_or_else(|| format!("PDF real requires decimal point: {text:?}"))?;
        if (integer.is_empty() && fraction.is_empty()) || !integer.bytes().all(|byte| byte.is_ascii_digit()) || !fraction.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(format!("invalid PDF real: {text:?}"));
        }
        let integer = if integer.is_empty() { "0" } else { integer };
        Ok(Self { negative, coefficient: format!("{integer}{fraction}"), scale: fraction.len() as u32 })
    }

    pub fn from_f64(value: f64) -> Self {
        let text = format!("{value}");
        let (mantissa, exponent) = match text.find(['e', 'E']) {
            Some(index) => (&text[..index], text[index + 1..].parse::<i32>().expect("finite f64 exponent")),
            None => (text.as_str(), 0),
        };
        let normalized = if mantissa.contains('.') { mantissa.to_string() } else { format!("{mantissa}.") };
        let mut decimal = Self::parse(&normalized).expect("finite f64 has valid decimal mantissa");
        let scale = decimal.scale as i64 - exponent as i64;
        if scale < 0 {
            decimal.coefficient.extend(std::iter::repeat_n('0', -scale as usize));
            decimal.scale = 0;
        } else {
            decimal.scale = scale as u32;
        }
        decimal
    }

    pub fn to_f64(&self) -> Option<f64> {
        self.to_string().parse().ok()
    }
}

impl From<f64> for PdfDecimal {
    fn from(value: f64) -> Self {
        Self::from_f64(value)
    }
}

impl fmt::Display for PdfDecimal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negative {
            formatter.write_str("-")?;
        }
        let scale = self.scale as usize;
        if scale >= self.coefficient.len() {
            formatter.write_str("0.")?;
            for _ in 0..scale.saturating_sub(self.coefficient.len()) {
                formatter.write_str("0")?;
            }
            formatter.write_str(&self.coefficient)
        } else {
            let split = self.coefficient.len() - scale;
            write!(formatter, "{}.{}", &self.coefficient[..split], &self.coefficient[split..])
        }
    }
}

/// 🎯 A parsed PDF object -- the full COS object grammar (ISO 32000-1 §7.3), including
/// streams. Stream `data` is the logical byte sequence after applying every filter that has a
/// logical decoder (@see [`PdfStreamFilter`]); `/Filter`, `/F`, `/DecodeParms`, and `/DP` are
/// removed during native deserialization and regenerated from `filters` on write.
#[derive(semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, Clone, Debug, PartialEq)]
#[derive(Default)]
pub enum PdfObject {
    #[default]
    Null,
    Bool(bool),
    Int(i64),
    Real(PdfDecimal),
    Str(Vec<u8>),
    Text(String),
    Date(PdfDate),
    Name(String),
    Array(Vec<PdfObject>),
    Dict(Vec<PdfDictEntry>),
    Ref(ObjRef),
    Stream {
        dict: Vec<PdfDictEntry>,
        data: Vec<u8>,
        filters: Vec<PdfStreamFilter>,
    },
}

impl PdfObject {
    pub fn as_dict(&self) -> Option<&[PdfDictEntry]> {
        match self {
            PdfObject::Dict(d) => Some(d),
            PdfObject::Stream { dict, .. } => Some(dict),
            _ => None,
        }
    }
    pub fn dict_get<'a>(&'a self, key: &str) -> Option<&'a PdfObject> {
        self.as_dict()?.iter().find(|e| e.key == key).map(|e| &e.value)
    }
    pub fn as_name(&self) -> Option<&str> {
        match self {
            PdfObject::Name(n) => Some(n.as_str()),
            _ => None,
        }
    }
    pub fn as_ref(&self) -> Option<ObjRef> {
        match self {
            PdfObject::Ref(r) => Some(*r),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<&[PdfObject]> {
        match self {
            PdfObject::Array(a) => Some(a),
            _ => None,
        }
    }
    pub fn as_str_bytes(&self) -> Option<&[u8]> {
        match self {
            PdfObject::Str(bytes) => Some(bytes),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            PdfObject::Bool(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            PdfObject::Int(i) => Some(*i as f64),
            PdfObject::Real(r) => r.to_f64(),
            _ => None,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            PdfObject::Int(i) => Some(*i),
            PdfObject::Real(r) => r.to_f64().map(|value| value as i64),
            _ => None,
        }
    }
    /// 🔢️ A number operand written in canonical form: integers stay integers, everything else is
    /// an exact decimal (never exponent notation, which PDF has no syntax for).
    pub fn number(value: f64) -> PdfObject {
        if value.is_finite() && value.fract() == 0.0 && value.abs() < 1e15 {
            PdfObject::Int(value as i64)
        } else if value.is_finite() {
            PdfObject::Real(PdfDecimal::from_f64(value))
        } else {
            PdfObject::Int(0)
        }
    }
    pub fn name(name: impl Into<String>) -> PdfObject {
        PdfObject::Name(name.into())
    }
    pub fn numbers(values: &[f64]) -> PdfObject {
        PdfObject::Array(values.iter().map(|value| PdfObject::number(*value)).collect())
    }
}

/// 🗄️ One `N G obj ... endobj` indirect object, keyed by its `id`.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
pub struct PdfIndirectObject {
    pub id: ObjRef,
    pub value: PdfObject,
}
//#endregion 🔖️ObjectModel

//#region 🔖️Geometry
/// 📐️ `[llx lly urx ury]` rectangle in default user space (§7.9.5).
pub type PdfRect = [f64; 4];

/// 🔀️ `[a b c d e f]` transformation matrix (§8.3.3).
pub type PdfMatrix = [f64; 6];

/// 🆔 Identity matrix.
pub const PDF_IDENTITY_MATRIX: PdfMatrix = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
//#endregion 🔖️Geometry
