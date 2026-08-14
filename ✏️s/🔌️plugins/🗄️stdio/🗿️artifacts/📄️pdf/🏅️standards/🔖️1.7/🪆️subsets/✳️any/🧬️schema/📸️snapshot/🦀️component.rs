//! 🧬️ PdfSnapshot schema (1.7) — real typed model. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION D2 `📄️pdf` row: 1.7
//! folds 1.4 in (reads 1.0-1.7 leniently, Decision #5) with a real object lexer/parser, xref
//! (classic+stream+hybrid+brute-force), filters, page-tree inheritance, ToUnicode-aware content
//! extraction, and a minimal multi-page writer -- see `standards::v1_7::engine` for the codec.
//!
//! Ground rule: byte/token parsing lives in `⚙️engine`; this module is the *typed* model (raw
//! indirect object graph for lossless retention of everything the writer doesn't regenerate,
//! plus the resolved page tree the analyzer/builder actually operate on).

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjRef {
    pub num: u32,
    pub gen: u16,
}

/// 🧩 One `key`/`value` pair of a PDF dictionary. A `Vec` (not a map) so parse order survives
/// losslessly -- PDF dictionaries have no canonical key order and real files vary widely.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfDictEntry {
    pub key: String,
    pub value: PdfObject,
}

/// 🔢️ Exact logical PDF real number represented as decimal coefficient and scale.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// 🎯 A parsed PDF object -- the full COS object grammar (ISO 32000-1 §7.3), including streams.
/// `Stream.raw_filter` is `Some(name)` when `data` is still filter-encoded verbatim (an
/// unsupported filter like `/DCTDecode`/`/CCITTFaxDecode` we deliberately don't decode --
/// ground rule: retain unmodeled bytes losslessly rather than corrupt/drop them) and `None` when
/// `data` has already been fully filter-decoded by the engine.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
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
    Stream {
        dict: Vec<PdfDictEntry>,
        data: Vec<u8>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        raw_filter: Option<String>,
    },
}

impl Default for PdfObject {
    fn default() -> Self {
        PdfObject::Null
    }
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
}

/// 🗄️ One `N G obj ... endobj` indirect object, keyed by its `id`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfPage {
    pub media_box: [f64; 4],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crop_box: Option<[f64; 4]>,
    #[serde(default)]
    pub rotate: i32,
    #[serde(default)]
    pub text: String,
}

impl Default for PdfPage {
    fn default() -> Self {
        Self { media_box: [0.0, 0.0, 612.0, 792.0], crop_box: None, rotate: 0, text: String::new() }
    }
}

impl PdfPage {
    pub fn new(width: f64, height: f64) -> Self {
        Self { media_box: [0.0, 0.0, width, height], crop_box: None, rotate: 0, text: String::new() }
    }
}

/// 📇️ Document `/Info` dictionary -- the fields `SetInfo` actually exposes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer: Option<String>,
}
//#endregion 🔖️PageModel

//#region 🔖️Snapshot
/// 🧬️ `stdio.pdf` (1.7) persistent snapshot. `objects` is the FULL raw indirect-object graph as
/// read (fonts, images, outlines, everything -- lossless retention per D2 ground rules); `pages`
/// is the resolved, editable view the analyzer/builder/mutations actually work with; `trailer` is
/// the trailer dictionary (`/Root`/`/Info`/`/Size`/… key-value pairs, same shape as a `Dict` --
/// the diff module's `PdfDictDiff` triple is reused verbatim for it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.1.7")]
pub struct PdfSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub declared_version: String,
    #[state(artifact)]
    #[serde(default)]
    pub pages: Vec<PdfPage>,
    #[state(artifact)]
    #[serde(default)]
    pub info: PdfInfo,
    #[state(artifact)]
    #[serde(default)]
    pub objects: Vec<PdfIndirectObject>,
    #[state(artifact)]
    #[serde(default)]
    pub trailer: Vec<PdfDictEntry>,
}

impl Default for PdfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(), declared_version: "1.7".into(), pages: Vec::new(), info: PdfInfo::default(), objects: Vec::new(), trailer: Vec::new() }
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
        let mut reader = store::ByteReader::new(&bytes);
        let snapshot = crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::dec_pdf_snapshot_bin(&mut reader).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
        if reader.remaining() != 0 {
            return Err(store::TextError::new(format!("{} trailing snapshot bytes", reader.remaining()), dsl::TextSpan::at(1, 1)));
        }
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let mut bytes = Vec::new();
        crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::enc_pdf_snapshot_bin(self, &mut bytes);
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PdfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let mut raw = Vec::new();
        crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::enc_pdf_snapshot_bin(self, &mut raw);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        let mut reader = store::ByteReader::new(&inner);
        let snapshot = crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::dec_pdf_snapshot_bin(&mut reader).map_err(store::PackError::Schema)?;
        if reader.remaining() != 0 {
            return Err(store::PackError::Schema(format!("{} trailing snapshot bytes", reader.remaining())));
        }
        Ok(snapshot)
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️SnapshotFixtures
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
/// STATE-MACHINES) — pure snapshot constructors, no codec/IO concern.
pub fn empty_pdf_snapshot() -> PdfSnapshot {
    PdfSnapshot::default()
}

/// 📄️ The demo `stdio.pdf.1.7` document -- the single source of truth for `🏅️standards/🔖️1.7/
/// 📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are literally this
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
pub fn demo_pdf17_snapshot() -> PdfSnapshot {
    let seed = PdfSnapshot {
        schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
        declared_version: "1.7".into(),
        pages: vec![PdfPage { media_box: [0.0, 0.0, 200.0, 300.0], crop_box: None, rotate: 0, text: "Semio".into() }],
        info: PdfInfo::default(),
        objects: Vec::new(),
        trailer: Vec::new(),
    };
    let bytes = crate::artifacts::pdf::standards::v1_7::subsets::any::io::encode_pdf(&seed).expect("encode_pdf(seed) must succeed");
    crate::artifacts::pdf::standards::v1_7::subsets::any::io::decode_pdf(&bytes).expect("decode_pdf(encode_pdf(seed)) must succeed")
}
//#endregion 🔖️SnapshotFixtures
