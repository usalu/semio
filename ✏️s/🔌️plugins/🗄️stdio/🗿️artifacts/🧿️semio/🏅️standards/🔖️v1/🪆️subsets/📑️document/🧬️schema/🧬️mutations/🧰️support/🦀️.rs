//! 🧰️ `SemioDocumentMutation` support: the diff builders and base accessors its mutation leaves call, hosted below the leaves so that no leaf has to import the aggregate module.

use crate::schema::diff::BlocksDiff;
use crate::schema::snapshot::DocBlock;
use crate::schema::diff::DocBlockDiff;
use crate::schema::diff::DocHeadingDiff;
use crate::schema::snapshot::DocImage;
use crate::schema::diff::DocParagraphDiff;
use crate::schema::diff::DocQuoteDiff;
use crate::schema::snapshot::DocRun;
use crate::schema::snapshot::DocStyle;
use crate::schema::diff::DocTableCellDiff;
use crate::schema::diff::DocTableRowDiff;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexAdded;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexModified;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use crate::schema::diff::ListItemsDiff;
use crate::schema::diff::RunsDiff;
use crate::schema::diff::SemioDocumentDiff;
use crate::schema::snapshot::SemioDocumentSnapshot;
use crate::schema::diff::TableCellsDiff;
use crate::schema::diff::TableRowsDiff;

/// 🧭️ One step down into a nested block container: `Quote` (own `blocks`), a `List` item's own
/// `blocks`, or a `Table` cell's own `blocks`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum DocPathSegment {
    Quote { block_index: usize },
    ListItem { block_index: usize, item: usize },
    TableCell { block_index: usize, row: usize, cell: usize },
}

/// 🧭️ Addresses one block-list slot: `segments` navigate through nested containers, `index` is
/// the slot within the innermost `Vec<DocBlock>`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DocBlockPath {
    #[value(default)]
    pub segments: Vec<DocPathSegment>,
    pub index: usize,
}

impl DocBlockPath {
    pub fn top(index: usize) -> Self {
        Self { segments: Vec::new(), index }
    }
}

/// 🧭️ Resolves the block list a path's segments navigate to (the parent list `path.index` slots
/// into), immutable form. `pub(crate)` so builder/composer callers can reuse it without
/// duplicating the traversal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn resolve_blocks<'a>(body: &'a [DocBlock], segments: &[DocPathSegment]) -> Option<&'a [DocBlock]> {
    match segments.split_first() {
        None => Some(body),
        Some((seg, rest)) => match seg {
            DocPathSegment::Quote { block_index } => {
                let DocBlock::Quote { blocks } = body.get(*block_index)? else { return None };
                resolve_blocks(blocks, rest)
            }
            DocPathSegment::ListItem { block_index, item } => {
                let DocBlock::List { items, .. } = body.get(*block_index)? else { return None };
                resolve_blocks(&items.get(*item)?.blocks, rest)
            }
            DocPathSegment::TableCell { block_index, row, cell } => {
                let DocBlock::Table { rows } = body.get(*block_index)? else { return None };
                resolve_blocks(&rows.get(*row)?.cells.get(*cell)?.blocks, rest)
            }
        },
    }
}

pub enum DocBlockLeaf {
    Modified(DocBlockDiff),
    Inserted(DocBlock),
    Removed,
}

impl DocBlockLeaf {
    fn into_blocks_diff(self, index: usize) -> BlocksDiff {
        match self {
            Self::Modified(diff) => BlocksDiff { modified: vec![IndexModified { index, diff }], ..Default::default() },
            Self::Inserted(block) => BlocksDiff { added: vec![IndexAdded { index, item: block }], ..Default::default() },
            Self::Removed => BlocksDiff { removed: vec![index], ..Default::default() },
        }
    }
}

/// 🧭️ Lowers a `leaf` diff targeting the block addressed by `path` into a full
/// `SemioDocumentDiff` by nesting it through `Quote`/`List`/`Table` from the document root down
/// to that depth (mirrors docx's `wrap_body_diff`, generalized to 3 container kinds).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wrap_body_diff(path: &DocBlockPath, leaf: DocBlockLeaf) -> SemioDocumentDiff {
    fn go(segments: &[DocPathSegment], index: usize, leaf: DocBlockLeaf) -> BlocksDiff {
        match segments.split_first() {
            None => leaf.into_blocks_diff(index),
            Some((seg, rest)) => {
                let inner = go(rest, index, leaf);
                match seg {
                    DocPathSegment::Quote { block_index } => {
                        let qd = DocBlockDiff::Quote(DocQuoteDiff { blocks: Some(inner) });
                        BlocksDiff { modified: vec![IndexModified { index: *block_index, diff: qd }], ..Default::default() }
                    }
                    DocPathSegment::ListItem { block_index, item } => {
                        let item_diff = crate::schema::diff::DocListItemDiff { blocks: Some(inner) };
                        let items_diff: ListItemsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *item, diff: item_diff }], ..Default::default() };
                        let ld = DocBlockDiff::List(crate::schema::diff::DocListDiff { ordered: None, items: Some(items_diff) });
                        BlocksDiff { modified: vec![IndexModified { index: *block_index, diff: ld }], ..Default::default() }
                    }
                    DocPathSegment::TableCell { block_index, row, cell } => {
                        let cell_diff = DocTableCellDiff { blocks: Some(inner) };
                        let cells_diff: TableCellsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *cell, diff: cell_diff }], ..Default::default() };
                        let row_diff = DocTableRowDiff { cells: Some(cells_diff) };
                        let rows_diff: TableRowsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *row, diff: row_diff }], ..Default::default() };
                        let td = DocBlockDiff::Table(crate::schema::diff::DocTableDiff { rows: Some(rows_diff) });
                        BlocksDiff { modified: vec![IndexModified { index: *block_index, diff: td }], ..Default::default() }
                    }
                }
            }
        }
    }
    let blocks = go(&path.segments, path.index, leaf);
    SemioDocumentDiff { styles: None, images: None, blocks: Some(blocks) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn block_at<'a>(base: &'a SemioDocumentSnapshot, path: &DocBlockPath) -> Option<&'a DocBlock> {
    resolve_blocks(&base.blocks, &path.segments)?.get(path.index)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn style_at<'a>(base: &'a SemioDocumentSnapshot, id: &str) -> Option<&'a DocStyle> {
    base.styles.iter().find(|s| s.id == id)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn image_at<'a>(base: &'a SemioDocumentSnapshot, id: &str) -> Option<&'a DocImage> {
    base.images.iter().find(|i| i.id == id)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn runs_of(block: &DocBlock) -> Option<&Vec<DocRun>> {
    match block {
        DocBlock::Paragraph { runs, .. } | DocBlock::Heading { runs, .. } => Some(runs),
        _ => None,
    }
}

/// 🎯️ Wraps a `RunsDiff` into the right `DocBlockDiff` variant depending on whether `block` is a
/// `Paragraph` or a `Heading` (the only two run-carrying kinds).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wrap_runs_diff(block: &DocBlock, runs: RunsDiff) -> Option<DocBlockDiff> {
    match block {
        DocBlock::Paragraph { .. } => Some(DocBlockDiff::Paragraph(DocParagraphDiff { style_id: None, runs: Some(runs) })),
        DocBlock::Heading { .. } => Some(DocBlockDiff::Heading(DocHeadingDiff { level: None, style_id: None, runs: Some(runs) })),
        _ => None,
    }
}
