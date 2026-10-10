//! 🧬️ SemioDocumentSnapshot — complete-per-spec block tree (Paragraph/Heading/List/Table/Code/
//! Quote/Image/PageBreak) + named styles + id-keyed images, informed by docx's body block tree
//! and md's `MdBlock`/`MdInline`; replaces `PageDoc`/`TextDoc`. Reused by `presentation`'s
//! `SlideShape::TextBox`, which embeds `DocBlock` directly (spec-mandated cross-reuse, see
//! `w1b-type-ownership.md`) — `DocBlock`/`DocRun`/`DocStyle` are this subset's owned types.
use framework_schema::ArtifactSchema;
//#region 🔖️DocumentModel
/// 🎨️ Character-level formatting for one `DocRun`. Named struct (never a bare tuple) per the f6
/// §4.3 `DslField`-for-tuples gap this schema style avoids everywhere.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RunStyle {
    #[value(default)]
    pub bold: bool,
    #[value(default)]
    pub italic: bool,
    #[value(default)]
    pub underline: bool,
    #[value(default)]
    pub size: Option<f64>,
    #[value(default)]
    pub font: Option<String>,
    #[value(default)]
    pub color: Option<String>,
    #[value(default)]
    pub link: Option<String>,
}
/// ✍️ One inline run of literal text plus its formatting.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocRun {
    pub text: String,
    #[value(default)]
    pub style: RunStyle,
}
impl DocRun {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn plain(text: impl Into<String>) -> Self {
        Self { text: text.into(), style: RunStyle::default() }
    }
}
/// 🎨️ One named paragraph/character style (docx `w:style`-shaped: id, display name, optional
/// parent for inheritance chains).
/// 🩹 Derives `Default` (empty id/name, no parent) so `DocStyle` satisfies the shared
/// `engine::triples::NamedTripleDiff<K,D,T>`'s conservative `T: Default` bound (a serde-derive
/// limitation identical to the one docx's OWN local `NamedTripleDiff` copy works around via an
/// explicit `#[value(bound(...))]` override — the shared `engine::triples` copy lacks that
/// override; per this ticket's "shared infra gaps → report only" rule, fixed here locally rather
/// than editing that shared file).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocStyle {
    #[value(default)]
    pub id: String,
    #[value(default)]
    pub name: String,
    #[value(default)]
    pub based_on: Option<String>,
}
/// 🖼️ One embedded raster/vector image, addressed by id from `DocBlock::Image`. Derives
/// `Default` for the same shared-`engine::triples`-bound reason as `DocStyle` above.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocImage {
    #[value(default)]
    pub id: String,
    #[value(default)]
    pub mime: String,
    #[value(default)]
    pub bytes: Vec<u8>,
}
/// 🔲 One list item — recursively holds its own block content (a list item may itself contain
/// paragraphs, nested lists, tables, …), matching CommonMark/WordprocessingML's own model.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocListItem {
    #[value(default)]
    pub blocks: Vec<DocBlock>,
}
/// 🔲️ One table cell — recursively holds block content.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocTableCell {
    #[value(default)]
    pub blocks: Vec<DocBlock>,
}
/// ➖️ One table row.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocTableRow {
    #[value(default)]
    pub cells: Vec<DocTableCell>,
}
/// 🧱️ One block-level content item — the recursive tree shape the master plan's snapshot spec
/// names: Paragraph/Heading/List/Table/Code/Quote/Image/PageBreak. `List`/`Table`/`Quote` nest
/// `DocBlock` recursively (list items, table cells, blockquote body), the same recursive-diff
/// shape svg's `SvgNodeDiff` and docx's `DocxBlock::Table` establish.
/// 🩹 Derives `Default` (`#[default]` on the fieldless `PageBreak` variant) for the same shared
/// `engine::triples::IndexedTripleDiff<D,T>` bound reason `DocStyle` documents above — `DocBlock`
/// is used as `T` in `BlocksDiff = IndexedTripleDiff<DocBlockDiff, DocBlock>`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum DocBlock {
    Paragraph {
        #[value(default)]
        style_id: Option<String>,
        #[value(default)]
        runs: Vec<DocRun>,
    },
    Heading {
        level: u8,
        #[value(default)]
        style_id: Option<String>,
        #[value(default)]
        runs: Vec<DocRun>,
    },
    List {
        #[value(default)]
        ordered: bool,
        #[value(default)]
        items: Vec<DocListItem>,
    },
    Table {
        #[value(default)]
        rows: Vec<DocTableRow>,
    },
    Code {
        #[value(default)]
        language: Option<String>,
        #[value(default)]
        text: String,
    },
    Quote {
        #[value(default)]
        blocks: Vec<DocBlock>,
    },
    Image {
        image_id: String,
        #[value(default)]
        alt: String,
        #[value(default)]
        width: Option<f64>,
        #[value(default)]
        height: Option<f64>,
    },
    #[default]
    PageBreak,
}
impl DocBlock {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn paragraph(text: impl Into<String>) -> Self {
        Self::Paragraph { style_id: None, runs: vec![DocRun::plain(text)] }
    }
}
//#endregion 🔖️DocumentModel
//#region 🔖️Ids
pub const STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA: &str = "s.stdio.semio.document";
//#endregion 🔖️Ids
//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.document")]
pub struct SemioDocumentSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🎨️ Named styles, keyed by `DocStyle::id`.
    #[state(artifact)]
    #[value(default)]
    pub styles: Vec<DocStyle>,
    /// 🖼️ Embedded images, keyed by `DocImage::id`, referenced from `DocBlock::Image::image_id`.
    #[state(artifact)]
    #[value(default)]
    pub images: Vec<DocImage>,
    /// 🧱️ The top-level block tree.
    #[state(artifact)]
    #[value(default)]
    pub blocks: Vec<DocBlock>,
}
impl Default for SemioDocumentSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(), styles: Default::default(), images: Default::default(), blocks: Default::default() }
    }
}
//#endregion 🔖️Snapshot
//#region 🔖️TextCodec
//#endregion 🔖️TextCodec
//#region 🔖️BinaryCodec
//#endregion 🔖️BinaryCodec
//#region 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️HandcraftedArtifactCodecs
//#region 🌉️ExternalCodecBridge
//#endregion 🌉️ExternalCodecBridge
//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.document` snapshot — one style, one image, and one block of every
/// kind (Heading/Paragraph/List/Table/Code/Quote/Image/PageBreak), exercising every leaf shape at
/// least once. Single source of truth for `📚️examples/🗒️memo/🖼️assets/🗣️.dsl.semio`/
/// `🎒️.pack.semio` and for the conformance-law tests in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_semio_document_snapshot() -> SemioDocumentSnapshot {
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
//#endregion 🔖️Demo
//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
                                                   