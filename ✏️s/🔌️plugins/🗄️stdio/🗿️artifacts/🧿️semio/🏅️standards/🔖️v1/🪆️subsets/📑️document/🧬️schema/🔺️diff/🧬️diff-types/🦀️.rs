//! 🧬️ Document diff types: the collection diff aliases, the sparse style, image, run, list, table and block diff records, the block diff enum and the document diff. See [`super`].

use super::*;

//#region 🔖️DocumentDiffTypes
pub type StylesDiff = NamedTripleDiff<String, DocStyleDiff, NamedAdded<DocStyle>>;

pub type ImagesDiff = NamedTripleDiff<String, DocImageDiff, NamedAdded<DocImage>>;

pub type BlocksDiff = IndexedTripleDiff<DocBlockDiff, DocBlock>;

pub type RunsDiff = IndexedTripleDiff<DocRunDiff, DocRun>;

pub type ListItemsDiff = IndexedTripleDiff<DocListItemDiff, DocListItem>;

pub type TableRowsDiff = IndexedTripleDiff<DocTableRowDiff, DocTableRow>;

pub type TableCellsDiff = IndexedTripleDiff<DocTableCellDiff, DocTableCell>;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocStyleDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = cleared, `Some(Some(id))` = set.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub based_on: Option<Option<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocImageDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RunStyleDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<Option<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocRunDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<RunStyleDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocListItemDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<BlocksDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocTableRowDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cells: Option<TableCellsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocTableCellDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<BlocksDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocParagraphDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style_id: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<RunsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocHeadingDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style_id: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<RunsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocListDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ordered: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<ListItemsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocTableDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<TableRowsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocCodeDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocQuoteDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<BlocksDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocImageBlockDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<Option<f64>>,
}

/// 🌳 Per-block diff, shaped like `DocBlock`. `Replace` covers a block-KIND change (e.g.
/// Paragraph -> Table).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum DocBlockDiff {
    Paragraph(DocParagraphDiff),
    Heading(DocHeadingDiff),
    List(DocListDiff),
    Table(DocTableDiff),
    Code(DocCodeDiff),
    Quote(DocQuoteDiff),
    Image(DocImageBlockDiff),
    Replace { block: DocBlock },
}

/// 🩹 Manual `Default` (no fieldless variant exists to `#[derive(Default)]` from) for the same
/// shared `engine::triples::IndexedTripleDiff<D,T>` bound reason `DocBlock`/`DocStyle` document —
/// `DocBlockDiff` is used as `D` in `BlocksDiff`. Never meaningfully constructed via this path
/// (only satisfies the trait bound); `Replace{block: DocBlock::default()}` is the least-surprising
/// technically-valid value (a no-op replace onto the default `PageBreak`).
impl Default for DocBlockDiff {
    fn default() -> Self {
        DocBlockDiff::Replace { block: DocBlock::default() }
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.document.diff")]
pub struct SemioDocumentDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub styles: Option<StylesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<ImagesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<BlocksDiff>,
}
//#endregion 🔖️GenericNamedEngine
