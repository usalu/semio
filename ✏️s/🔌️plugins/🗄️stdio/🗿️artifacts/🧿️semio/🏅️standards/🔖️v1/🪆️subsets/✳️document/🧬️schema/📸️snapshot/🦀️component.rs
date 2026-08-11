//! 🧬️ SemioDocumentSnapshot — complete-per-spec block tree (Paragraph/Heading/List/Table/Code/
//! Quote/Image/PageBreak) + named styles + id-keyed images, informed by docx's body block tree
//! and md's `MdBlock`/`MdInline`; replaces `PageDoc`/`TextDoc`. Reused by `presentation`'s
//! `SlideShape::TextBox`, which embeds `DocBlock` directly (spec-mandated cross-reuse, see
//! `w1b-type-ownership.md`) — `DocBlock`/`DocRun`/`DocStyle` are this subset's owned types.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️DocumentModel
/// 🎨️ Character-level formatting for one `DocRun`. Named struct (never a bare tuple) per the f6
/// §4.3 `DslField`-for-tuples gap this schema style avoids everywhere.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunStyle {
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub size: Option<f64>,
    #[serde(default)]
    pub font: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
}

/// ✍️ One inline run of literal text plus its formatting.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocRun {
    pub text: String,
    #[serde(default)]
    pub style: RunStyle,
}

impl DocRun {
    pub fn plain(text: impl Into<String>) -> Self {
        Self { text: text.into(), style: RunStyle::default() }
    }
}

/// 🎨️ One named paragraph/character style (docx `w:style`-shaped: id, display name, optional
/// parent for inheritance chains).
/// 🩹 Derives `Default` (empty id/name, no parent) so `DocStyle` satisfies the shared
/// `engine::triples::NamedTripleDiff<K,D,T>`'s conservative `T: Default` bound (a serde-derive
/// limitation identical to the one docx's OWN local `NamedTripleDiff` copy works around via an
/// explicit `#[serde(bound(...))]` override — the shared `engine::triples` copy lacks that
/// override; per this ticket's "shared infra gaps → report only" rule, fixed here locally rather
/// than editing that shared file).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocStyle {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub based_on: Option<String>,
}

/// 🖼️ One embedded raster/vector image, addressed by id from `DocBlock::Image`. Derives
/// `Default` for the same shared-`engine::triples`-bound reason as `DocStyle` above.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocImage {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub mime: String,
    #[serde(default)]
    pub bytes: Vec<u8>,
}

/// 🔲 One list item — recursively holds its own block content (a list item may itself contain
/// paragraphs, nested lists, tables, …), matching CommonMark/WordprocessingML's own model.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocListItem {
    #[serde(default)]
    pub blocks: Vec<DocBlock>,
}

/// 🔲️ One table cell — recursively holds block content.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocTableCell {
    #[serde(default)]
    pub blocks: Vec<DocBlock>,
}

/// ➖️ One table row.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocTableRow {
    #[serde(default)]
    pub cells: Vec<DocTableCell>,
}

/// 🧱️ One block-level content item — the recursive tree shape the master plan's snapshot spec
/// names: Paragraph/Heading/List/Table/Code/Quote/Image/PageBreak. `List`/`Table`/`Quote` nest
/// `DocBlock` recursively (list items, table cells, blockquote body), the same recursive-diff
/// shape svg's `SvgNodeDiff` and docx's `DocxBlock::Table` establish.
/// 🩹 Derives `Default` (`#[default]` on the fieldless `PageBreak` variant) for the same shared
/// `engine::triples::IndexedTripleDiff<D,T>` bound reason `DocStyle` documents above — `DocBlock`
/// is used as `T` in `BlocksDiff = IndexedTripleDiff<DocBlockDiff, DocBlock>`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocBlock {
    Paragraph {
        #[serde(default)]
        style_id: Option<String>,
        #[serde(default)]
        runs: Vec<DocRun>,
    },
    Heading {
        level: u8,
        #[serde(default)]
        style_id: Option<String>,
        #[serde(default)]
        runs: Vec<DocRun>,
    },
    List {
        #[serde(default)]
        ordered: bool,
        #[serde(default)]
        items: Vec<DocListItem>,
    },
    Table {
        #[serde(default)]
        rows: Vec<DocTableRow>,
    },
    Code {
        #[serde(default)]
        language: Option<String>,
        #[serde(default)]
        text: String,
    },
    Quote {
        #[serde(default)]
        blocks: Vec<DocBlock>,
    },
    Image {
        image_id: String,
        #[serde(default)]
        alt: String,
        #[serde(default)]
        width: Option<f64>,
        #[serde(default)]
        height: Option<f64>,
    },
    #[default]
    PageBreak,
}

impl DocBlock {
    pub fn paragraph(text: impl Into<String>) -> Self {
        Self::Paragraph { style_id: None, runs: vec![DocRun::plain(text)] }
    }
}
//#endregion 🔖️DocumentModel

//#region 🔖️Ids
pub const STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA: &str = "s.stdio.semio.document";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.document")]
pub struct SemioDocumentSnapshot {
    #[state(persistent)]
    pub schema: String,
    /// 🎨️ Named styles, keyed by `DocStyle::id`.
    #[state(persistent)]
    #[serde(default)]
    pub styles: Vec<DocStyle>,
    /// 🖼️ Embedded images, keyed by `DocImage::id`, referenced from `DocBlock::Image::image_id`.
    #[state(persistent)]
    #[serde(default)]
    pub images: Vec<DocImage>,
    /// 🧱️ The top-level block tree.
    #[state(persistent)]
    #[serde(default)]
    pub blocks: Vec<DocBlock>,
}

impl Default for SemioDocumentSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: Default::default(),
            images: Default::default(),
            blocks: Default::default(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🧩️ JSON-pack round trip — honest for THIS subset: the semio document snapshot is a neutral
/// semio type (not an on-disk file format with its own byte layout), so the pack/dsl envelope
/// carries the structural JSON encoding, wrapped in the same `store::semio_format` envelope every
/// stdio artifact uses.
impl store::ArtifactDsl for SemioDocumentSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA }

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
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        serde_json::from_slice(&bytes).map_err(|e| store::TextError::new(format!("json decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioDocumentSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn rich_snapshot() -> SemioDocumentSnapshot {
        SemioDocumentSnapshot {
            schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: vec![DocStyle { id: "heading1".into(), name: "Heading 1".into(), based_on: Some("normal".into()) }],
            images: vec![DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] }],
            blocks: vec![
                DocBlock::Heading { level: 1, style_id: Some("heading1".into()), runs: vec![DocRun { text: "Title".into(), style: RunStyle { bold: true, ..Default::default() } }] },
                DocBlock::Paragraph { style_id: None, runs: vec![DocRun::plain("Body")] },
                DocBlock::List { ordered: true, items: vec![DocListItem { blocks: vec![DocBlock::paragraph("item one")] }] },
                DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] },
                DocBlock::Code { language: Some("rust".into()), text: "fn main() {}".into() },
                DocBlock::Quote { blocks: vec![DocBlock::paragraph("quoted")] },
                DocBlock::Image { image_id: "img1".into(), alt: "alt text".into(), width: Some(100.0), height: Some(50.0) },
                DocBlock::PageBreak,
            ],
        }
    }

    #[test]
    fn json_pack_round_trips() {
        let snap = rich_snapshot();
        let bytes = <SemioDocumentSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = rich_snapshot();
        let text = <SemioDocumentSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }
}
//#endregion 🔖️Tests
