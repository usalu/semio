//! 🧬️ PdfSnapshot schema (1.7) — real typed model. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION D2 `📄️pdf` row: 1.7
//! folds 1.4 in (reads 1.0-1.7 leniently, Decision #5) with a real object lexer/parser, xref
//! (classic+stream+hybrid+brute-force), filters, page-tree inheritance, ToUnicode-aware content
//! extraction, and a minimal multi-page writer -- see `standards::v1_7::engine` for the codec.
//!
//! Ground rule: byte/token parsing lives in `⚙️engine`; this module is the *typed* logical
//! model. Native lexical choices and encoded stream representations never enter the snapshot.

use schema::ArtifactSchema;
use std::fmt;

/// 🏷️ Document schema id for `stdio.pdf` (1.7) -- deliberately distinct from 1.4's flat
/// `stdio.pdf` (avoids colliding with 1.4's own `store::register_document_codec` registration,
/// which the flat pre-D4 registry can't disambiguate by standard; same shape as gif 89a's
/// `STDIO_GIF89A_DOCUMENT_SCHEMA`, see that module's doc comment).
pub const STDIO_PDF17_DOCUMENT_SCHEMA: &str = "stdio.pdf.1.7";

//#region 🔖️ObjectModel
/// 🔗️ An indirect-object reference `N G R` — also the `objects` collection's diff KEY (the
/// `(id,gen)` pair per the recipe's "numeric id" key kind; `Hash` is needed by the diff module's
/// key-transport absorb maps).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjRef {
    pub num: u32,
    pub gen: u16,
}

/// 🧩 One `key`/`value` pair of a PDF dictionary. A `Vec` (not a map) so parse order survives
/// losslessly -- PDF dictionaries have no canonical key order and real files vary widely.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfDictEntry {
    pub key: String,
    pub value: PdfObject,
}

/// 🎛️ Logical predictor parameters attached to a PDF stream filter.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPredictor {
    pub predictor: u32,
    pub colors: u32,
    pub bits_per_component: u32,
    pub columns: u32,
}

/// 🗜️ Supported logical stream-filter pipeline concepts.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum PdfStreamFilter {
    Flate { predictor: Option<PdfPredictor> },
    AsciiHex,
    Ascii85,
    RunLength,
}

/// 🔢️ Exact logical PDF real number represented as decimal coefficient and scale.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfDecimal {
    pub negative: bool,
    pub coefficient: String,
    pub scale: u32,
}

impl PdfDecimal {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
/// streams. Stream `data` is the logical byte sequence after applying the stream filter chain;
/// `/Filter`, `/F`, `/DecodeParms`, and `/DP` are removed during native deserialization.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum PdfObject {
    Null,
    Bool(bool),
    Int(i64),
    Real(PdfDecimal),
    Str(Vec<u8>),
    Name(String),
    Array(Vec<PdfObject>),
    Dict(Vec<PdfDictEntry>),
    Ref(ObjRef),
    Stream { dict: Vec<PdfDictEntry>, data: Vec<u8>, filters: Vec<PdfStreamFilter> },
}

impl Default for PdfObject {
    fn default() -> Self {
        PdfObject::Null
    }
}

impl PdfObject {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_dict(&self) -> Option<&[PdfDictEntry]> {
        match self {
            PdfObject::Dict(d) => Some(d),
            PdfObject::Stream { dict, .. } => Some(dict),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn dict_get<'a>(&'a self, key: &str) -> Option<&'a PdfObject> {
        self.as_dict()?.iter().find(|e| e.key == key).map(|e| &e.value)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_name(&self) -> Option<&str> {
        match self {
            PdfObject::Name(n) => Some(n.as_str()),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_ref(&self) -> Option<ObjRef> {
        match self {
            PdfObject::Ref(r) => Some(*r),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_array(&self) -> Option<&[PdfObject]> {
        match self {
            PdfObject::Array(a) => Some(a),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            PdfObject::Int(i) => Some(*i as f64),
            PdfObject::Real(r) => r.to_f64(),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            PdfObject::Int(i) => Some(*i),
            PdfObject::Real(r) => r.to_f64().map(|value| value as i64),
            _ => None,
        }
    }
}

/// 🗄️ One `N G obj ... endobj` indirect object, keyed by its `id`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfIndirectObject {
    pub id: ObjRef,
    pub value: PdfObject,
}
//#endregion 🔖️ObjectModel

//#region 🔖️PageModel
/// 📄️ One resolved page -- inherited `/Resources`/`/MediaBox`/`/CropBox`/`/Rotate` already
/// applied (ISO 32000-1 §7.7.3.4 inheritance), text already extracted from its content stream(s).
/// `text` doubles as the builder's authoring surface: the writer regenerates a fresh content
/// stream from it on encode (see `AppendPageContent`'s doc comment for why one field suffices).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPage {
    pub media_box: [f64; 4],
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub crop_box: Option<[f64; 4]>,
    #[value(default)]
    pub rotate: i32,
    #[value(default)]
    pub text: String,
}

impl Default for PdfPage {
    fn default() -> Self {
        Self { media_box: [0.0, 0.0, 612.0, 792.0], crop_box: None, rotate: 0, text: String::new() }
    }
}

impl PdfPage {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(width: f64, height: f64) -> Self {
        Self { media_box: [0.0, 0.0, width, height], crop_box: None, rotate: 0, text: String::new() }
    }
}

/// 📇️ Document `/Info` dictionary -- the fields `SetInfo` actually exposes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfInfo {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub producer: Option<String>,
}
//#endregion 🔖️PageModel

//#region 🔖️Snapshot
/// 🧬️ `stdio.pdf` (1.7) persistent snapshot. `objects` is the full logical indirect-object graph as
/// read (fonts, images, outlines, everything -- lossless retention per D2 ground rules); `pages`
/// is the resolved, editable view the analyzer/builder/mutations actually work with; `trailer` is
/// the trailer dictionary (`/Root`/`/Info`/`/Size`/… key-value pairs, same shape as a `Dict` --
/// the diff module's `PdfDictDiff` triple is reused verbatim for it.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.stdio.pdf.1.7")]
pub struct PdfSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub declared_version: String,
    #[state(artifact)]
    pub pages: Vec<PdfPage>,
    #[state(artifact)]
    pub info: PdfInfo,
    #[state(artifact)]
    pub objects: Vec<PdfIndirectObject>,
    #[state(artifact)]
    pub trailer: Vec<PdfDictEntry>,
}

impl Default for PdfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(), declared_version: "1.7".into(), pages: Vec::new(), info: PdfInfo::default(), objects: Vec::new(), trailer: Vec::new() }
    }
}

/// 🌱️ First-party value encoding for the public PDF snapshot, preserving the exact camelCase
/// object shape previously emitted by `value_derive::ToValue`.
impl pack::value::ToValue for PdfSnapshot {
    fn to_value(&self) -> pack::value::DslValue {
        pack::value::DslValue::object([
            ("schema".to_string(), self.schema.to_value()),
            ("declaredVersion".to_string(), self.declared_version.to_value()),
            ("pages".to_string(), self.pages.to_value()),
            ("info".to_string(), self.info.to_value()),
            ("objects".to_string(), self.objects.to_value()),
            ("trailer".to_string(), self.trailer.to_value()),
        ])
    }
}

/// 🔀️ First-party value decoding for the public PDF snapshot. `schema` remains required while
/// every other field retains the derive codec's field-level default behavior.
impl pack::value::FromValue for PdfSnapshot {
    fn from_value(value: pack::value::DslValue) -> Result<Self, pack::value::ValueError> {
        use pack::value::FromValue;
        let entries = value.into_object()?;
        let field = |key: &str| entries.iter().find(|(candidate, _)| candidate == key).map(|(_, value)| value.clone());
        fn decode_or_default<T: FromValue + Default>(value: Option<pack::value::DslValue>, key: &str) -> Result<T, pack::value::ValueError> {
            match value {
                Some(value) => T::from_value(value).map_err(|error| error.under(key)),
                None => Ok(T::default()),
            }
        }
        let schema = field("schema").ok_or_else(|| pack::value::ValueError::new("missing field `schema`"))?;
        Ok(Self {
            schema: String::from_value(schema).map_err(|error| error.under("schema"))?,
            declared_version: decode_or_default(field("declaredVersion"), "declaredVersion")?,
            pages: decode_or_default(field("pages"), "pages")?,
            info: decode_or_default(field("info"), "info")?,
            objects: decode_or_default(field("objects"), "objects")?,
            trailer: decode_or_default(field("trailer"), "trailer")?,
        })
    }
}

#[cfg(test)]
mod pdf_snapshot_value_tests {
    use super::*;
    use pack::value::{DslValue, FromValue, ToValue};

    fn populated_snapshot() -> PdfSnapshot {
        let root = ObjRef { num: 1, gen: 0 };
        PdfSnapshot {
            schema: STDIO_PDF17_DOCUMENT_SCHEMA.to_string(),
            declared_version: "1.7".to_string(),
            pages: vec![PdfPage { media_box: [0.0, 0.0, 612.0, 792.0], crop_box: Some([12.0, 12.0, 600.0, 780.0]), rotate: 90, text: "Semio".to_string() }],
            info: PdfInfo { title: Some("Value path".to_string()), producer: Some("semio".to_string()), ..PdfInfo::default() },
            objects: vec![PdfIndirectObject { id: root, value: PdfObject::Name("Catalog".to_string()) }],
            trailer: vec![PdfDictEntry { key: "Root".to_string(), value: PdfObject::Ref(root) }],
        }
    }

    #[test]
    fn populated_snapshot_round_trips_through_value() {
        let snapshot = populated_snapshot();
        assert_eq!(PdfSnapshot::from_value(snapshot.to_value()), Ok(snapshot));
    }

    #[test]
    fn missing_optional_fields_keep_the_derived_defaults() {
        let value = DslValue::object([("schema".to_string(), DslValue::String("stdio.pdf.1.7".to_string()))]);
        assert_eq!(
            PdfSnapshot::from_value(value),
            Ok(PdfSnapshot { schema: "stdio.pdf.1.7".to_string(), declared_version: String::new(), pages: Vec::new(), info: PdfInfo::default(), objects: Vec::new(), trailer: Vec::new() })
        );
    }

    #[test]
    fn missing_schema_reports_the_exact_field() {
        let error = PdfSnapshot::from_value(DslValue::object([])).unwrap_err();
        assert_eq!(error.to_string(), "missing field `schema`");
    }

    #[test]
    fn camel_case_json_shape_agrees_with_serde_json_oracle() {
        let snapshot = PdfSnapshot::default();
        let actual: serde_json::Value = snapshot.to_value().into();
        let oracle = serde_json::json!({
            "schema": "stdio.pdf.1.7",
            "declaredVersion": "1.7",
            "pages": [],
            "info": {},
            "objects": [],
            "trailer": []
        });
        assert_eq!(actual, oracle);
        let json = serde_json::to_string(&oracle).expect("serde_json oracle encodes");
        assert_eq!(pack::json::from_json_str::<PdfSnapshot>(&json), Ok(snapshot));
    }
}

impl store::ArtifactDsl for PdfSnapshot {
    const EXTENSION: &'static str = "pdf";
    fn envelope_id() -> &'static str {
        STDIO_PDF17_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        for i in (0..hex.len()).step_by(2) {
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?);
        }
        crate::artifacts::pdf::standards::v1_7::subsets::base::io::decode_pdf(&bytes).map_err(|e| store::TextError::new(format!("{e:?}"), dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::pdf::standards::v1_7::subsets::base::io::encode_pdf(self).expect("PDF snapshot must encode before DSL transport");
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PdfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::pdf::standards::v1_7::subsets::base::io::encode_pdf(self).map_err(|e| store::PackError::Schema(format!("{e:?}")))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        crate::artifacts::pdf::standards::v1_7::subsets::base::io::decode_pdf(&inner).map_err(|e| store::PackError::Schema(format!("{e:?}")))
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️SnapshotFixtures
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
/// STATE-MACHINES) — pure snapshot constructors, no codec/IO concern.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_pdf_snapshot() -> PdfSnapshot {
    PdfSnapshot::default()
}

/// 📄️ The demo `stdio.pdf.1.7` document -- the single source of truth for `🏅️standards/🔖️1.7/
/// 📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law`).
///
/// Deliberately the real `decode_pdf(encode_pdf(seed))` FIXED POINT, not a hand-built struct with
/// empty `objects`/`trailer`: `encode_pdf` only ever reads `pages`/`info` from its input (the
/// writer's own doc comment: "the original `objects` graph is deliberately NOT re-emitted") and
/// regenerates a FRESH Catalog/Pages/Font/Content-stream object graph every time; `decode_pdf`
/// then reads that fresh graph back into `objects`/`trailer`. A hand-built snapshot with
/// `objects: vec![]` would make `parse_dsl(print_dsl(demo)) != demo` (confirmed empirically: the
/// real `decode_pdf` output has 6 populated `objects` + a real `trailer`, never empty) -- `parse_
/// dsl` genuinely calls `decode_pdf` on the hex-decoded bytes, not an identity round-trip, same
/// "1.7 stays a frozen stub" pattern 1.4's own `demo_pdf_snapshot` doc comment documents for its
/// hardcoded `width`/`height`. `pages`/`info` DO survive this round trip losslessly (the
/// bachelor-thesis example's own `decode_encode_decode_is_structurally_equal_at_page_level` test
/// already proves this at scale) -- only `objects`/`trailer` need the fixed-point construction.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_pdf17_snapshot() -> PdfSnapshot {
    let seed = PdfSnapshot {
        schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
        declared_version: "1.7".into(),
        pages: vec![PdfPage { media_box: [0.0, 0.0, 200.0, 300.0], crop_box: None, rotate: 0, text: "Semio".into() }],
        info: PdfInfo::default(),
        objects: Vec::new(),
        trailer: Vec::new(),
    };
    let bytes = crate::artifacts::pdf::standards::v1_7::subsets::base::io::encode_pdf(&seed).expect("encode_pdf(seed) must succeed");
    crate::artifacts::pdf::standards::v1_7::subsets::base::io::decode_pdf(&bytes).expect("decode_pdf(encode_pdf(seed)) must succeed")
}
//#endregion 🔖️SnapshotFixtures
