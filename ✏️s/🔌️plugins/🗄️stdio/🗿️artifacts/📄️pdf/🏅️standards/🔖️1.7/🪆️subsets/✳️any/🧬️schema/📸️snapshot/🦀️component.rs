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

/// 🏷️ Document schema id for `stdio.pdf` (1.7) -- deliberately distinct from 1.4's flat
/// `stdio.pdf` (avoids colliding with 1.4's own `store::register_document_codec` registration,
/// which the flat pre-D4 registry can't disambiguate by standard; same shape as gif 89a's
/// `STDIO_GIF89A_DOCUMENT_SCHEMA`, see that module's doc comment).
pub const STDIO_PDF17_DOCUMENT_SCHEMA: &str = "stdio.pdf.1.7";

//#region 🔖️ObjectModel
/// 🔗️ An indirect-object reference `N G R`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
    Real(f64),
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
    fn default() -> Self { PdfObject::Null }
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
        match self { PdfObject::Name(n) => Some(n.as_str()), _ => None }
    }
    pub fn as_ref(&self) -> Option<ObjRef> {
        match self { PdfObject::Ref(r) => Some(*r), _ => None }
    }
    pub fn as_array(&self) -> Option<&[PdfObject]> {
        match self { PdfObject::Array(a) => Some(a), _ => None }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self { PdfObject::Int(i) => Some(*i as f64), PdfObject::Real(r) => Some(*r), _ => None }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match self { PdfObject::Int(i) => Some(*i), PdfObject::Real(r) => Some(*r as i64), _ => None }
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
    fn default() -> Self { Self { media_box: [0.0, 0.0, 612.0, 792.0], crop_box: None, rotate: 0, text: String::new() } }
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
/// is the resolved, editable view the analyzer/builder/mutations actually work with. The writer
/// regenerates a fresh minimal file from `pages`+`info` alone (requirement #7: "valid classic
/// xref + trailer is sufficient") -- `objects` is NOT re-emitted, which is why the
/// decode→encode→decode round trip is asserted structurally (page-level), not on `objects`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.1.7")]
pub struct PdfSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub declared_version: String,
    #[state(persistent)]
    #[serde(default)]
    pub pages: Vec<PdfPage>,
    #[state(persistent)]
    #[serde(default)]
    pub info: PdfInfo,
    #[state(persistent)]
    #[serde(default)]
    pub objects: Vec<PdfIndirectObject>,
}

impl Default for PdfSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
            declared_version: "1.7".into(),
            pages: Vec::new(),
            info: PdfInfo::default(),
            objects: Vec::new(),
        }
    }
}

impl store::ArtifactDsl for PdfSnapshot {
    const EXTENSION: &'static str = "pdf";
    fn envelope_id() -> &'static str { STDIO_PDF17_DOCUMENT_SCHEMA }
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
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
                store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1))
            })?);
        }
        crate::artifacts::pdf::standards::v1_7::engine::decode_pdf(&bytes)
            .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::pdf::standards::v1_7::engine::encode_pdf(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PdfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::pdf::standards::v1_7::engine::encode_pdf(self)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        crate::artifacts::pdf::standards::v1_7::engine::decode_pdf(&inner)
            .map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️Snapshot
