//! 🧬️ BcfSnapshot — OOXML zip parts.

use crate::artifacts::bcf::STDIO_BCF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfEntry {
    pub name: String,
    #[serde(default)]
    pub data: Vec<u8>,
}

//#region 🔖️TopicModel
/// 💬 One `<Comment>` under a topic's `markup.bcf` (BCF-XML 2.1 §Comment): `Guid` is the
/// comment's own identity (distinct from the topic's), `date`/`author`/`comment` are the
/// required child elements `<Date>`/`<Author>`/`<Comment>` verbatim (ISO-8601 date text and
/// free-form comment body, kept as literal strings -- not reinterpreted).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfComment {
    pub guid: String,
    pub date: String,
    pub author: String,
    pub comment: String,
}

/// 🗂️ One BCF topic (one `<guid>/markup.bcf` entry inside the container): `guid`/`status` mirror
/// the `Topic` element's `Guid`/`TopicStatus` XML *attributes* (not child elements -- BCF-XML
/// 2.1's `markup.xsd` declares both as `xs:attribute`), `title` is the required `<Title>` child
/// element's text, `comments` are the sibling `<Comment>` elements under `<Markup>`, and
/// `viewpoint_ref` is the referenced viewpoint filename read off `<Viewpoints Viewpoint="...">`
/// (the `.bcfv` file's own camera/visibility content is deliberately left unparsed -- only the
/// filename reference is modeled, per this artifact's D2 minimum-depth scope).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfTopic {
    pub guid: String,
    pub title: String,
    pub status: String,
    #[serde(default)]
    pub comments: Vec<BcfComment>,
    #[serde(default)]
    pub viewpoint_ref: Option<String>,
}
//#endregion 🔖️TopicModel

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bcf")]
pub struct BcfSnapshot {
    #[state(persistent)]
    pub schema: String,
    /// 🗄️ Lossless raw zip-entry substrate: every entry (root `bcf.version`, `markup.bcf`,
    /// `.bcfv` viewpoints, snapshot images, ...) verbatim. This is what actually round-trips
    /// through `encode_bcf`/`decode_bcf`; `topics` below is a derived/reconciled typed view.
    #[state(persistent)]
    #[serde(default)]
    pub entries: Vec<BcfEntry>,
    /// 🧬 Typed view derived from `entries` on decode (parsed out of each topic folder's
    /// `markup.bcf`). `encode_bcf` reconciles this back into `entries` -- regenerating/creating
    /// the corresponding `markup.bcf` XML for every topic present here -- so setting `topics`
    /// directly (without touching `entries`) still round-trips through encode.
    #[state(persistent)]
    #[serde(default)]
    pub topics: Vec<BcfTopic>,
}

impl Default for BcfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_BCF_DOCUMENT_SCHEMA.into(), entries: Vec::new(), topics: Vec::new() }
    }
}

impl store::ArtifactDsl for BcfSnapshot {
    const EXTENSION: &'static str = "bcf";
    fn envelope_id() -> &'static str { "stdio.bcf" }
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
        crate::artifacts::bcf::engine::decode_bcf(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::bcf::engine::encode_bcf(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for BcfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::bcf::engine::encode_bcf(self).map_err(|e| store::PackError::Schema(e))?;
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
        crate::artifacts::bcf::engine::decode_bcf(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
