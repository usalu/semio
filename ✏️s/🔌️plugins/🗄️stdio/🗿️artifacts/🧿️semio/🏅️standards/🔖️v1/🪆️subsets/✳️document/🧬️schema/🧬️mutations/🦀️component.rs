//! 🧬️ SemioDocumentMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (never apply-and-capture) and every variant's `inverse()` is handcrafted, key/index-aware.
//! Addresses the recursive `blocks` tree via `DocBlockPath` (segments navigate through nested
//! `Quote`/`List`/`Table` containers — svg's `NodePath` precedent, extended for 3 nesting kinds
//! instead of docx's single `Table`-only nesting), named styles by `DocStyle::id`, and named
//! images by `DocImage::id`.
//!
//! 🧪️ Per f6-final-summary.md §4.3/§4.4: `#[derive(dsl::DslOps)]` would fail here for the same
//! reasons `DocxMutation` hit — `SetSnapshot{snapshot}` reaches the data-carrying `DocBlockDiff`
//! enum transitively; `InsertBlock`/`SetBlockContent`'s bare `block: DocBlock` fails directly
//! (`DocBlock: DslField` not satisfied — it's a data-carrying enum); `style: RunStyle`/
//! `path: DocBlockPath` also fail (`DslField` not satisfied, neither is `#[derive(DslRecord)]`).
//! `OpText`/`OpBinary` hand-rolled below, reusing `SemioDocumentDiff`'s `pub(crate)` grammar
//! primitives.

use crate::artifacts::semio::standards::v1::engine::triples::{split_top_level, strip_brackets, IndexAdded, IndexModified, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::document::schema::diff::{
    dec_bool, dec_f64, dec_image, dec_run_style, dec_str, dec_style, dec_u8, decode_option, diff_block, diff_set_snapshot, enc_bool, enc_f64, enc_image,
    enc_run_style, enc_str, enc_style, enc_u8, encode_option, hex_decode, hex_encode, BlocksDiff, DocBlockDiff, DocHeadingDiff,
    DocParagraphDiff, DocQuoteDiff, DocRunDiff, DocTableCellDiff, DocTableRowDiff, ListItemsDiff, RunsDiff, SemioDocumentDiff, TableCellsDiff, TableRowsDiff,
};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocRun, DocStyle, RunStyle, SemioDocumentSnapshot};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioDocumentMutation` block
/// below calls `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in
/// scope in production code too, not merely under `#[cfg(test)]` (W2b closer fix).
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️PathAddressing
/// 🧭️ One step down into a nested block container: `Quote` (own `blocks`), a `List` item's own
/// `blocks`, or a `Table` cell's own `blocks`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocPathSegment {
    Quote { block_index: usize },
    ListItem { block_index: usize, item: usize },
    TableCell { block_index: usize, row: usize, cell: usize },
}

/// 🧭️ Addresses one block-list slot: `segments` navigate through nested containers, `index` is
/// the slot within the innermost `Vec<DocBlock>`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocBlockPath {
    #[serde(default)]
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
pub(crate) fn resolve_blocks<'a>(body: &'a [DocBlock], segments: &[DocPathSegment]) -> Option<&'a [DocBlock]> {
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

enum DocBlockLeaf {
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
fn wrap_body_diff(path: &DocBlockPath, leaf: DocBlockLeaf) -> SemioDocumentDiff {
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
                        let item_diff = crate::artifacts::semio::standards::v1::subsets::document::schema::diff::DocListItemDiff { blocks: Some(inner) };
                        let items_diff: ListItemsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *item, diff: item_diff }], ..Default::default() };
                        let ld = DocBlockDiff::List(crate::artifacts::semio::standards::v1::subsets::document::schema::diff::DocListDiff { ordered: None, items: Some(items_diff) });
                        BlocksDiff { modified: vec![IndexModified { index: *block_index, diff: ld }], ..Default::default() }
                    }
                    DocPathSegment::TableCell { block_index, row, cell } => {
                        let cell_diff = DocTableCellDiff { blocks: Some(inner) };
                        let cells_diff: TableCellsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *cell, diff: cell_diff }], ..Default::default() };
                        let row_diff = DocTableRowDiff { cells: Some(cells_diff) };
                        let rows_diff: TableRowsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *row, diff: row_diff }], ..Default::default() };
                        let td = DocBlockDiff::Table(crate::artifacts::semio::standards::v1::subsets::document::schema::diff::DocTableDiff { rows: Some(rows_diff) });
                        BlocksDiff { modified: vec![IndexModified { index: *block_index, diff: td }], ..Default::default() }
                    }
                }
            }
        }
    }
    let blocks = go(&path.segments, path.index, leaf);
    SemioDocumentDiff { styles: None, images: None, blocks: Some(blocks) }
}
//#endregion 🔖️PathAddressing

//#region 🔖️Mutations
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioDocumentMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: SemioDocumentSnapshot,
    },
    /// ➕️ Inserts `block` at `path` (`path.index` = insertion index, FINAL state).
    InsertBlock {
        path: DocBlockPath,
        block: DocBlock,
    },
    /// ➖️ Removes the block at `path` (`path.index` = BASE-state index).
    RemoveBlock {
        path: DocBlockPath,
    },
    /// ✍️ Replaces the full content of the block at `path` with `block` (may change kind).
    SetBlockContent {
        path: DocBlockPath,
        block: DocBlock,
    },
    /// 🎨️ Sets (or clears) a `Paragraph` block's `style_id`.
    SetParagraphStyle {
        path: DocBlockPath,
        style_id: Option<String>,
    },
    /// 🔢️ Sets a `Heading` block's `level`.
    SetHeadingLevel {
        path: DocBlockPath,
        level: u8,
    },
    /// 🔀️ Sets a `List` block's `ordered` flag.
    SetListOrdered {
        path: DocBlockPath,
        ordered: bool,
    },
    /// ✍️ Replaces the literal text of run `run_index` in the Paragraph/Heading at `path`.
    SetRunText {
        path: DocBlockPath,
        run_index: usize,
        text: String,
    },
    /// 🎨️ Replaces run `run_index`'s full `RunStyle` in the Paragraph/Heading at `path`.
    SetRunStyle {
        path: DocBlockPath,
        run_index: usize,
        style: RunStyle,
    },
    /// 🖼️ Replaces an `Image` block's `image_id`/`alt`/`width`/`height` at `path`.
    SetImageBlock {
        path: DocBlockPath,
        image_id: String,
        alt: String,
        width: Option<f64>,
        height: Option<f64>,
    },
    /// ➕️ Inserts a named style.
    InsertStyle {
        style: DocStyle,
    },
    /// ➖️ Removes the style with id `id`.
    RemoveStyle {
        id: String,
    },
    /// 🏷️ Renames the style with id `id`.
    SetStyleName {
        id: String,
        name: String,
    },
    /// 🔗 Sets (or, if `None`, clears) the style with id `id`'s `based_on`.
    SetStyleBasedOn {
        id: String,
        based_on: Option<String>,
    },
    /// ➕️ Inserts a named image.
    InsertImage {
        image: DocImage,
    },
    /// ➖️ Removes the image with id `id`.
    RemoveImage {
        id: String,
    },
    /// ✍️ Replaces the mime/bytes of the image with id `id`.
    SetImageBytes {
        id: String,
        mime: String,
        bytes: Vec<u8>,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` -- the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
pub fn apply_semio_document_mutation(snapshot: &mut SemioDocumentSnapshot, mutation: &SemioDocumentMutation) -> SemioDocumentDiff {
    let diff = Mutation::diff(mutation, snapshot);
    *snapshot = protocol::MutationDiff::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
fn block_at<'a>(base: &'a SemioDocumentSnapshot, path: &DocBlockPath) -> Option<&'a DocBlock> {
    resolve_blocks(&base.blocks, &path.segments)?.get(path.index)
}
fn style_at<'a>(base: &'a SemioDocumentSnapshot, id: &str) -> Option<&'a DocStyle> {
    base.styles.iter().find(|s| s.id == id)
}
fn image_at<'a>(base: &'a SemioDocumentSnapshot, id: &str) -> Option<&'a DocImage> {
    base.images.iter().find(|i| i.id == id)
}
fn runs_of(block: &DocBlock) -> Option<&Vec<DocRun>> {
    match block {
        DocBlock::Paragraph { runs, .. } | DocBlock::Heading { runs, .. } => Some(runs),
        _ => None,
    }
}
/// 🎯️ Wraps a `RunsDiff` into the right `DocBlockDiff` variant depending on whether `block` is a
/// `Paragraph` or a `Heading` (the only two run-carrying kinds).
fn wrap_runs_diff(block: &DocBlock, runs: RunsDiff) -> Option<DocBlockDiff> {
    match block {
        DocBlock::Paragraph { .. } => Some(DocBlockDiff::Paragraph(DocParagraphDiff { style_id: None, runs: Some(runs) })),
        DocBlock::Heading { .. } => Some(DocBlockDiff::Heading(DocHeadingDiff { level: None, style_id: None, runs: Some(runs) })),
        _ => None,
    }
}
//#endregion 🔖️Helpers

//#region 🔖️MutationTrait
impl Mutation<SemioDocumentSnapshot> for SemioDocumentMutation {
    type Diff = SemioDocumentDiff;

    fn diff(&self, base: &SemioDocumentSnapshot) -> Self::Diff {
        match self {
            SemioDocumentMutation::NoMutation => SemioDocumentDiff::default(),
            SemioDocumentMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            SemioDocumentMutation::InsertBlock { path, block } => wrap_body_diff(path, DocBlockLeaf::Inserted(block.clone())),
            SemioDocumentMutation::RemoveBlock { path } => wrap_body_diff(path, DocBlockLeaf::Removed),
            SemioDocumentMutation::SetBlockContent { path, block } => match block_at(base, path) {
                Some(old) => match diff_block(old, block) {
                    Some(d) => wrap_body_diff(path, DocBlockLeaf::Modified(d)),
                    None => SemioDocumentDiff::default(),
                },
                None => SemioDocumentDiff::default(),
            },
            SemioDocumentMutation::SetParagraphStyle { path, style_id } => match block_at(base, path) {
                Some(DocBlock::Paragraph { style_id: old, .. }) if old != style_id => {
                    wrap_body_diff(path, DocBlockLeaf::Modified(DocBlockDiff::Paragraph(DocParagraphDiff { style_id: Some(style_id.clone()), runs: None })))
                }
                _ => SemioDocumentDiff::default(),
            },
            SemioDocumentMutation::SetHeadingLevel { path, level } => match block_at(base, path) {
                Some(DocBlock::Heading { level: old, .. }) if old != level => {
                    wrap_body_diff(path, DocBlockLeaf::Modified(DocBlockDiff::Heading(DocHeadingDiff { level: Some(*level), style_id: None, runs: None })))
                }
                _ => SemioDocumentDiff::default(),
            },
            SemioDocumentMutation::SetListOrdered { path, ordered } => match block_at(base, path) {
                Some(DocBlock::List { ordered: old, .. }) if old != ordered => {
                    wrap_body_diff(path, DocBlockLeaf::Modified(DocBlockDiff::List(crate::artifacts::semio::standards::v1::subsets::document::schema::diff::DocListDiff { ordered: Some(*ordered), items: None })))
                }
                _ => SemioDocumentDiff::default(),
            },
            SemioDocumentMutation::SetRunText { path, run_index, text } => {
                let Some(block) = block_at(base, path) else { return SemioDocumentDiff::default() };
                let Some(runs) = runs_of(block) else { return SemioDocumentDiff::default() };
                let Some(run) = runs.get(*run_index) else { return SemioDocumentDiff::default() };
                if &run.text == text {
                    return SemioDocumentDiff::default();
                }
                let rd: RunsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *run_index, diff: DocRunDiff { text: Some(text.clone()), style: None } }], ..Default::default() };
                match wrap_runs_diff(block, rd) {
                    Some(bd) => wrap_body_diff(path, DocBlockLeaf::Modified(bd)),
                    None => SemioDocumentDiff::default(),
                }
            }
            SemioDocumentMutation::SetRunStyle { path, run_index, style } => {
                let Some(block) = block_at(base, path) else { return SemioDocumentDiff::default() };
                let Some(runs) = runs_of(block) else { return SemioDocumentDiff::default() };
                let Some(run) = runs.get(*run_index) else { return SemioDocumentDiff::default() };
                if &run.style == style {
                    return SemioDocumentDiff::default();
                }
                let style_diff = crate::artifacts::semio::standards::v1::subsets::document::schema::diff::RunStyleDiff {
                    bold: Some(style.bold),
                    italic: Some(style.italic),
                    underline: Some(style.underline),
                    size: Some(style.size),
                    font: Some(style.font.clone()),
                    color: Some(style.color.clone()),
                    link: Some(style.link.clone()),
                };
                let rd: RunsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *run_index, diff: DocRunDiff { text: None, style: Some(style_diff) } }], ..Default::default() };
                match wrap_runs_diff(block, rd) {
                    Some(bd) => wrap_body_diff(path, DocBlockLeaf::Modified(bd)),
                    None => SemioDocumentDiff::default(),
                }
            }
            SemioDocumentMutation::SetImageBlock { path, image_id, alt, width, height } => match block_at(base, path) {
                Some(old @ DocBlock::Image { .. }) => {
                    let new = DocBlock::Image { image_id: image_id.clone(), alt: alt.clone(), width: *width, height: *height };
                    match diff_block(old, &new) {
                        Some(d) => wrap_body_diff(path, DocBlockLeaf::Modified(d)),
                        None => SemioDocumentDiff::default(),
                    }
                }
                _ => SemioDocumentDiff::default(),
            },
            SemioDocumentMutation::InsertStyle { style } => SemioDocumentDiff {
                styles: Some(crate::artifacts::semio::standards::v1::subsets::document::schema::diff::StylesDiff { added: vec![style.clone()], ..Default::default() }),
                images: None,
                blocks: None,
            },
            SemioDocumentMutation::RemoveStyle { id } => SemioDocumentDiff {
                styles: Some(crate::artifacts::semio::standards::v1::subsets::document::schema::diff::StylesDiff { removed: vec![id.clone()], ..Default::default() }),
                images: None,
                blocks: None,
            },
            SemioDocumentMutation::SetStyleName { id, name } => match style_at(base, id) {
                Some(old) if &old.name != name => SemioDocumentDiff {
                    styles: Some(crate::artifacts::semio::standards::v1::subsets::document::schema::diff::StylesDiff {
                        modified: vec![crate::artifacts::semio::standards::v1::engine::triples::NamedModified {
                            key: id.clone(),
                            diff: crate::artifacts::semio::standards::v1::subsets::document::schema::diff::DocStyleDiff { name: Some(name.clone()), based_on: None },
                        }],
                        ..Default::default()
                    }),
                    images: None,
                    blocks: None,
                },
                _ => SemioDocumentDiff::default(),
            },
            SemioDocumentMutation::SetStyleBasedOn { id, based_on } => match style_at(base, id) {
                Some(old) if &old.based_on != based_on => SemioDocumentDiff {
                    styles: Some(crate::artifacts::semio::standards::v1::subsets::document::schema::diff::StylesDiff {
                        modified: vec![crate::artifacts::semio::standards::v1::engine::triples::NamedModified {
                            key: id.clone(),
                            diff: crate::artifacts::semio::standards::v1::subsets::document::schema::diff::DocStyleDiff { name: None, based_on: Some(based_on.clone()) },
                        }],
                        ..Default::default()
                    }),
                    images: None,
                    blocks: None,
                },
                _ => SemioDocumentDiff::default(),
            },
            SemioDocumentMutation::InsertImage { image } => SemioDocumentDiff {
                styles: None,
                images: Some(crate::artifacts::semio::standards::v1::subsets::document::schema::diff::ImagesDiff { added: vec![image.clone()], ..Default::default() }),
                blocks: None,
            },
            SemioDocumentMutation::RemoveImage { id } => SemioDocumentDiff {
                styles: None,
                images: Some(crate::artifacts::semio::standards::v1::subsets::document::schema::diff::ImagesDiff { removed: vec![id.clone()], ..Default::default() }),
                blocks: None,
            },
            SemioDocumentMutation::SetImageBytes { id, mime, bytes } => match image_at(base, id) {
                Some(old) if &old.mime != mime || &old.bytes != bytes => SemioDocumentDiff {
                    styles: None,
                    images: Some(crate::artifacts::semio::standards::v1::subsets::document::schema::diff::ImagesDiff {
                        modified: vec![crate::artifacts::semio::standards::v1::engine::triples::NamedModified {
                            key: id.clone(),
                            diff: crate::artifacts::semio::standards::v1::subsets::document::schema::diff::DocImageDiff { mime: Some(mime.clone()), bytes: Some(bytes.clone()) },
                        }],
                        ..Default::default()
                    }),
                    blocks: None,
                },
                _ => SemioDocumentDiff::default(),
            },
        }
    }

    fn inverse(&self, base: &SemioDocumentSnapshot) -> Vec<Self> {
        match self {
            SemioDocumentMutation::NoMutation => vec![SemioDocumentMutation::NoMutation],
            SemioDocumentMutation::SetSnapshot { .. } => vec![SemioDocumentMutation::SetSnapshot { snapshot: base.clone() }],
            SemioDocumentMutation::InsertBlock { path, .. } => vec![SemioDocumentMutation::RemoveBlock { path: path.clone() }],
            SemioDocumentMutation::RemoveBlock { path } => match block_at(base, path) {
                Some(block) => vec![SemioDocumentMutation::InsertBlock { path: path.clone(), block: block.clone() }],
                None => vec![SemioDocumentMutation::NoMutation],
            },
            SemioDocumentMutation::SetBlockContent { path, .. } => match block_at(base, path) {
                Some(block) => vec![SemioDocumentMutation::SetBlockContent { path: path.clone(), block: block.clone() }],
                None => vec![SemioDocumentMutation::NoMutation],
            },
            SemioDocumentMutation::SetParagraphStyle { path, .. } => match block_at(base, path) {
                Some(DocBlock::Paragraph { style_id, .. }) => vec![SemioDocumentMutation::SetParagraphStyle { path: path.clone(), style_id: style_id.clone() }],
                _ => vec![SemioDocumentMutation::NoMutation],
            },
            SemioDocumentMutation::SetHeadingLevel { path, .. } => match block_at(base, path) {
                Some(DocBlock::Heading { level, .. }) => vec![SemioDocumentMutation::SetHeadingLevel { path: path.clone(), level: *level }],
                _ => vec![SemioDocumentMutation::NoMutation],
            },
            SemioDocumentMutation::SetListOrdered { path, .. } => match block_at(base, path) {
                Some(DocBlock::List { ordered, .. }) => vec![SemioDocumentMutation::SetListOrdered { path: path.clone(), ordered: *ordered }],
                _ => vec![SemioDocumentMutation::NoMutation],
            },
            SemioDocumentMutation::SetRunText { path, run_index, .. } => match block_at(base, path).and_then(runs_of).and_then(|r| r.get(*run_index)) {
                Some(run) => vec![SemioDocumentMutation::SetRunText { path: path.clone(), run_index: *run_index, text: run.text.clone() }],
                None => vec![SemioDocumentMutation::NoMutation],
            },
            SemioDocumentMutation::SetRunStyle { path, run_index, .. } => match block_at(base, path).and_then(runs_of).and_then(|r| r.get(*run_index)) {
                Some(run) => vec![SemioDocumentMutation::SetRunStyle { path: path.clone(), run_index: *run_index, style: run.style.clone() }],
                None => vec![SemioDocumentMutation::NoMutation],
            },
            SemioDocumentMutation::SetImageBlock { path, .. } => match block_at(base, path) {
                Some(DocBlock::Image { image_id, alt, width, height }) => {
                    vec![SemioDocumentMutation::SetImageBlock { path: path.clone(), image_id: image_id.clone(), alt: alt.clone(), width: *width, height: *height }]
                }
                _ => vec![SemioDocumentMutation::NoMutation],
            },
            SemioDocumentMutation::InsertStyle { style } => vec![SemioDocumentMutation::RemoveStyle { id: style.id.clone() }],
            SemioDocumentMutation::RemoveStyle { id } => match style_at(base, id) {
                Some(style) => vec![SemioDocumentMutation::InsertStyle { style: style.clone() }],
                None => vec![SemioDocumentMutation::NoMutation],
            },
            SemioDocumentMutation::SetStyleName { id, .. } => match style_at(base, id) {
                Some(style) => vec![SemioDocumentMutation::SetStyleName { id: id.clone(), name: style.name.clone() }],
                None => vec![SemioDocumentMutation::NoMutation],
            },
            SemioDocumentMutation::SetStyleBasedOn { id, .. } => match style_at(base, id) {
                Some(style) => vec![SemioDocumentMutation::SetStyleBasedOn { id: id.clone(), based_on: style.based_on.clone() }],
                None => vec![SemioDocumentMutation::NoMutation],
            },
            SemioDocumentMutation::InsertImage { image } => vec![SemioDocumentMutation::RemoveImage { id: image.id.clone() }],
            SemioDocumentMutation::RemoveImage { id } => match image_at(base, id) {
                Some(image) => vec![SemioDocumentMutation::InsertImage { image: image.clone() }],
                None => vec![SemioDocumentMutation::NoMutation],
            },
            SemioDocumentMutation::SetImageBytes { id, .. } => match image_at(base, id) {
                Some(image) => vec![SemioDocumentMutation::SetImageBytes { id: id.clone(), mime: image.mime.clone(), bytes: image.bytes.clone() }],
                None => vec![SemioDocumentMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Hand-rolled `OpText`/`OpBinary`: `keyword arg=value ...` grammar (space-separated), same
/// shape docx/svg/gif's hand-rolled ops use.
fn enc_path_segment(seg: &DocPathSegment) -> String {
    match seg {
        DocPathSegment::Quote { block_index } => format!("Q[{block_index}]"),
        DocPathSegment::ListItem { block_index, item } => format!("L[{block_index},{item}]"),
        DocPathSegment::TableCell { block_index, row, cell } => format!("T[{block_index},{row},{cell}]"),
    }
}
fn dec_path_segment(s: &str) -> Result<DocPathSegment, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    match tag {
        "Q" => {
            let [block_index] = parts.as_slice() else { return Err(format!("quote segment: expected 1 field, got {}", parts.len())) };
            Ok(DocPathSegment::Quote { block_index: parse_usize(block_index)? })
        }
        "L" => {
            let [block_index, item] = parts.as_slice() else { return Err(format!("list-item segment: expected 2 fields, got {}", parts.len())) };
            Ok(DocPathSegment::ListItem { block_index: parse_usize(block_index)?, item: parse_usize(item)? })
        }
        "T" => {
            let [block_index, row, cell] = parts.as_slice() else { return Err(format!("table-cell segment: expected 3 fields, got {}", parts.len())) };
            Ok(DocPathSegment::TableCell { block_index: parse_usize(block_index)?, row: parse_usize(row)?, cell: parse_usize(cell)? })
        }
        other => Err(format!("path segment: unknown tag {other:?}")),
    }
}
fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
fn enc_block_path(p: &DocBlockPath) -> String {
    format!("[{},{}]", enc_list(&p.segments, enc_path_segment), p.index)
}
fn dec_block_path(s: &str) -> Result<DocBlockPath, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [segments, index] = parts.as_slice() else { return Err(format!("block path: expected 2 fields, got {}", parts.len())) };
    Ok(DocBlockPath { segments: dec_list(segments, dec_path_segment)?, index: parse_usize(index)? })
}
fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|i| enc(i)).collect::<Vec<_>>().join(","))
}
fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

/// 🌱 Full (non-diff) `DocBlock`/`SemioDocumentSnapshot` codecs -- only `SetSnapshot`/
/// `InsertBlock`/`SetBlockContent`'s whole-payload encoding needs these; reuses `SemioDocumentDiff`'s
/// `pub(crate)` `enc_block`/`enc_style`/`enc_image` for the shared per-item shape.
fn enc_block(b: &DocBlock) -> String {
    crate::artifacts::semio::standards::v1::subsets::document::schema::diff::enc_block(b)
}
fn dec_block(s: &str) -> Result<DocBlock, String> {
    crate::artifacts::semio::standards::v1::subsets::document::schema::diff::dec_block(s)
}
fn enc_run_style_full(s: &RunStyle) -> String {
    enc_run_style(s)
}
fn dec_run_style_full(s: &str) -> Result<RunStyle, String> {
    dec_run_style(s)
}
fn enc_snapshot(s: &SemioDocumentSnapshot) -> String {
    format!("[{},{},{}]", enc_list(&s.styles, enc_style), enc_list(&s.images, enc_image), enc_list(&s.blocks, enc_block))
}
fn dec_snapshot(s: &str) -> Result<SemioDocumentSnapshot, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [styles, images, blocks] = parts.as_slice() else { return Err(format!("snapshot: expected 3 fields, got {}", parts.len())) };
    Ok(SemioDocumentSnapshot {
        schema: crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
        styles: dec_list(styles, dec_style)?,
        images: dec_list(images, dec_image)?,
        blocks: dec_list(blocks, dec_block)?,
    })
}

fn print_document_mutation(m: &SemioDocumentMutation) -> String {
    match m {
        SemioDocumentMutation::NoMutation => "no-mutation".to_string(),
        SemioDocumentMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_snapshot(snapshot)),
        SemioDocumentMutation::InsertBlock { path, block } => format!("insert-block path={} block={}", enc_block_path(path), enc_block(block)),
        SemioDocumentMutation::RemoveBlock { path } => format!("remove-block path={}", enc_block_path(path)),
        SemioDocumentMutation::SetBlockContent { path, block } => format!("set-block-content path={} block={}", enc_block_path(path), enc_block(block)),
        SemioDocumentMutation::SetParagraphStyle { path, style_id } => format!("set-paragraph-style path={} style-id={}", enc_block_path(path), encode_option(style_id, |v| enc_str(v))),
        SemioDocumentMutation::SetHeadingLevel { path, level } => format!("set-heading-level path={} level={}", enc_block_path(path), enc_u8(level)),
        SemioDocumentMutation::SetListOrdered { path, ordered } => format!("set-list-ordered path={} ordered={}", enc_block_path(path), enc_bool(ordered)),
        SemioDocumentMutation::SetRunText { path, run_index, text } => format!("set-run-text path={} run-index={} text={}", enc_block_path(path), run_index, enc_str(text)),
        SemioDocumentMutation::SetRunStyle { path, run_index, style } => format!("set-run-style path={} run-index={} style={}", enc_block_path(path), run_index, enc_run_style_full(style)),
        SemioDocumentMutation::SetImageBlock { path, image_id, alt, width, height } => format!(
            "set-image-block path={} image-id={} alt={} width={} height={}",
            enc_block_path(path), enc_str(image_id), enc_str(alt), encode_option(width, enc_f64), encode_option(height, enc_f64)
        ),
        SemioDocumentMutation::InsertStyle { style } => format!("insert-style style={}", enc_style(style)),
        SemioDocumentMutation::RemoveStyle { id } => format!("remove-style id={}", enc_str(id)),
        SemioDocumentMutation::SetStyleName { id, name } => format!("set-style-name id={} name={}", enc_str(id), enc_str(name)),
        SemioDocumentMutation::SetStyleBasedOn { id, based_on } => format!("set-style-based-on id={} based-on={}", enc_str(id), encode_option(based_on, |v| enc_str(v))),
        SemioDocumentMutation::InsertImage { image } => format!("insert-image image={}", enc_image(image)),
        SemioDocumentMutation::RemoveImage { id } => format!("remove-image id={}", enc_str(id)),
        SemioDocumentMutation::SetImageBytes { id, mime, bytes } => format!("set-image-bytes id={} mime={} bytes={}", enc_str(id), enc_str(mime), hex_encode(bytes)),
    }
}
fn parse_document_mutation(line: &str) -> Result<SemioDocumentMutation, String> {
    if line == "no-mutation" {
        return Ok(SemioDocumentMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("document mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("document mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(SemioDocumentMutation::SetSnapshot { snapshot: dec_snapshot(arg("snapshot")?)? }),
        "insert-block" => Ok(SemioDocumentMutation::InsertBlock { path: dec_block_path(arg("path")?)?, block: dec_block(arg("block")?)? }),
        "remove-block" => Ok(SemioDocumentMutation::RemoveBlock { path: dec_block_path(arg("path")?)? }),
        "set-block-content" => Ok(SemioDocumentMutation::SetBlockContent { path: dec_block_path(arg("path")?)?, block: dec_block(arg("block")?)? }),
        "set-paragraph-style" => Ok(SemioDocumentMutation::SetParagraphStyle { path: dec_block_path(arg("path")?)?, style_id: decode_option(arg("style-id")?, dec_str)? }),
        "set-heading-level" => Ok(SemioDocumentMutation::SetHeadingLevel { path: dec_block_path(arg("path")?)?, level: dec_u8(arg("level")?)? }),
        "set-list-ordered" => Ok(SemioDocumentMutation::SetListOrdered { path: dec_block_path(arg("path")?)?, ordered: dec_bool(arg("ordered")?)? }),
        "set-run-text" => Ok(SemioDocumentMutation::SetRunText { path: dec_block_path(arg("path")?)?, run_index: usize_arg("run-index")?, text: dec_str(arg("text")?)? }),
        "set-run-style" => Ok(SemioDocumentMutation::SetRunStyle { path: dec_block_path(arg("path")?)?, run_index: usize_arg("run-index")?, style: dec_run_style_full(arg("style")?)? }),
        "set-image-block" => Ok(SemioDocumentMutation::SetImageBlock {
            path: dec_block_path(arg("path")?)?,
            image_id: dec_str(arg("image-id")?)?,
            alt: dec_str(arg("alt")?)?,
            width: decode_option(arg("width")?, dec_f64)?,
            height: decode_option(arg("height")?, dec_f64)?,
        }),
        "insert-style" => Ok(SemioDocumentMutation::InsertStyle { style: dec_style(arg("style")?)? }),
        "remove-style" => Ok(SemioDocumentMutation::RemoveStyle { id: dec_str(arg("id")?)? }),
        "set-style-name" => Ok(SemioDocumentMutation::SetStyleName { id: dec_str(arg("id")?)?, name: dec_str(arg("name")?)? }),
        "set-style-based-on" => Ok(SemioDocumentMutation::SetStyleBasedOn { id: dec_str(arg("id")?)?, based_on: decode_option(arg("based-on")?, dec_str)? }),
        "insert-image" => Ok(SemioDocumentMutation::InsertImage { image: dec_image(arg("image")?)? }),
        "remove-image" => Ok(SemioDocumentMutation::RemoveImage { id: dec_str(arg("id")?)? }),
        "set-image-bytes" => Ok(SemioDocumentMutation::SetImageBytes { id: dec_str(arg("id")?)?, mime: dec_str(arg("mime")?)?, bytes: hex_decode(arg("bytes")?)? }),
        other => Err(format!("document mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for SemioDocumentMutation {
    fn print_op(&self) -> String {
        print_document_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_document_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// ⚡️ Binary = the text bytes verbatim, same simplification `SemioDocumentDiff`'s hand-rolled
/// codec uses.
impl protocol::OpBinary for SemioDocumentMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::document::schema::diff::DocBlockDiff as TestDocBlockDiff;
    use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocListItem, DocTableCell, DocTableRow};
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    fn fixture() -> SemioDocumentSnapshot {
        SemioDocumentSnapshot {
            schema: "s.stdio.semio.document".into(),
            styles: vec![DocStyle { id: "Normal".into(), name: "Normal".into(), based_on: None }],
            images: Vec::new(),
            blocks: vec![DocBlock::paragraph("first"), DocBlock::paragraph("second")],
        }
    }

    fn table_path(block_index: usize, row: usize, cell: usize, index: usize) -> DocBlockPath {
        DocBlockPath { segments: vec![DocPathSegment::TableCell { block_index, row, cell }], index }
    }

    #[test]
    fn insert_then_remove_block_apply_and_inverse() {
        let base = fixture();
        let insert = SemioDocumentMutation::InsertBlock { path: DocBlockPath::top(1), block: DocBlock::paragraph("inserted") };
        let mut after = base.clone();
        apply_semio_document_mutation(&mut after, &insert);
        assert_eq!(after.blocks.len(), 3);
        assert_eq!(after.blocks[1], DocBlock::paragraph("inserted"));

        let inverses = Mutation::inverse(&insert, &base);
        let mut restored = after.clone();
        for inv in &inverses {
            apply_semio_document_mutation(&mut restored, inv);
        }
        assert_eq!(restored, base);
    }

    #[test]
    fn nested_quote_and_list_path_addressing_apply_and_inverse() {
        let mut base = fixture();
        base.blocks.push(DocBlock::Quote { blocks: vec![DocBlock::paragraph("quoted")] });
        base.blocks.push(DocBlock::List { ordered: false, items: vec![DocListItem { blocks: vec![DocBlock::paragraph("item")] }] });

        let quote_path = DocBlockPath { segments: vec![DocPathSegment::Quote { block_index: 2 }], index: 0 };
        let mutation = SemioDocumentMutation::SetRunText { path: quote_path.clone(), run_index: 0, text: "changed quote".into() };
        let mut after = base.clone();
        apply_semio_document_mutation(&mut after, &mutation);
        let DocBlock::Quote { blocks } = &after.blocks[2] else { panic!("quote") };
        let DocBlock::Paragraph { runs, .. } = &blocks[0] else { panic!("paragraph") };
        assert_eq!(runs[0].text, "changed quote");
        for inv in Mutation::inverse(&mutation, &base) {
            apply_semio_document_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let list_path = DocBlockPath { segments: vec![DocPathSegment::ListItem { block_index: 3, item: 0 }], index: 0 };
        let list_mutation = SemioDocumentMutation::SetRunText { path: list_path, run_index: 0, text: "changed item".into() };
        let mut after2 = base.clone();
        apply_semio_document_mutation(&mut after2, &list_mutation);
        let DocBlock::List { items, .. } = &after2.blocks[3] else { panic!("list") };
        let DocBlock::Paragraph { runs, .. } = &items[0].blocks[0] else { panic!("paragraph") };
        assert_eq!(runs[0].text, "changed item");
        for inv in Mutation::inverse(&list_mutation, &base) {
            apply_semio_document_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, base);
    }

    #[test]
    fn table_path_addressing_sets_nested_cell_content() {
        let mut base = fixture();
        base.blocks.push(DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] });
        let path = table_path(2, 0, 0, 0);
        let mutation = SemioDocumentMutation::SetBlockContent { path: path.clone(), block: DocBlock::paragraph("changed cell") };
        let mut after = base.clone();
        apply_semio_document_mutation(&mut after, &mutation);
        let DocBlock::Table { rows } = &after.blocks[2] else { panic!("table") };
        assert_eq!(rows[0].cells[0].blocks[0], DocBlock::paragraph("changed cell"));
        for inv in Mutation::inverse(&mutation, &base) {
            apply_semio_document_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }

    #[test]
    fn style_and_image_mutations_apply_and_inverse() {
        let base = fixture();
        let insert = SemioDocumentMutation::InsertStyle { style: DocStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) } };
        let mut after = base.clone();
        apply_semio_document_mutation(&mut after, &insert);
        assert_eq!(after.styles.len(), 2);
        for inv in Mutation::inverse(&insert, &base) {
            apply_semio_document_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let insert_img = SemioDocumentMutation::InsertImage { image: DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1, 2] } };
        let mut with_img = base.clone();
        apply_semio_document_mutation(&mut with_img, &insert_img);
        assert_eq!(with_img.images.len(), 1);
        let set_bytes = SemioDocumentMutation::SetImageBytes { id: "img1".into(), mime: "image/jpeg".into(), bytes: vec![9] };
        let mut after2 = with_img.clone();
        apply_semio_document_mutation(&mut after2, &set_bytes);
        assert_eq!(after2.images[0].mime, "image/jpeg");
        for inv in Mutation::inverse(&set_bytes, &with_img) {
            apply_semio_document_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, with_img);
    }

    //#region 🔖️Fixtures
    /// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field. `blocks` uses different-length lists
    /// so a single same-direction `between()` shows removed+modified+added simultaneously; the
    /// modified paragraph exercises `style_id`'s tri-state AND nested `runs` modified+added; the
    /// reverse direction (asserted in `field_sweep`) exercises `blocks.added` carrying a whole
    /// recursively-structured `Table`. `styles`/`images` each get one removed, one
    /// modified-in-every-field, one added.
    fn sweep_a() -> SemioDocumentSnapshot {
        SemioDocumentSnapshot {
            schema: "s.stdio.semio.document".into(),
            styles: vec![
                DocStyle { id: "keep".into(), name: "Keep".into(), based_on: None },
                DocStyle { id: "toModify".into(), name: "old".into(), based_on: None },
                DocStyle { id: "toRemove".into(), name: "Gone".into(), based_on: None },
            ],
            images: vec![
                DocImage { id: "toModify".into(), mime: "image/png".into(), bytes: vec![1] },
                DocImage { id: "toRemove".into(), mime: "image/gif".into(), bytes: vec![2] },
            ],
            blocks: vec![
                DocBlock::Paragraph { style_id: None, runs: vec![DocRun { text: "old".into(), style: RunStyle { bold: false, ..Default::default() } }] },
                DocBlock::paragraph("stay"),
                DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("toDrop cell")] }] }] },
            ],
        }
    }

    fn sweep_b() -> SemioDocumentSnapshot {
        SemioDocumentSnapshot {
            schema: "s.stdio.semio.document".into(),
            styles: vec![
                DocStyle { id: "keep".into(), name: "Keep".into(), based_on: None },
                DocStyle { id: "toModify".into(), name: "new".into(), based_on: Some("keep".into()) },
                DocStyle { id: "added".into(), name: "Added".into(), based_on: None },
            ],
            images: vec![
                DocImage { id: "toModify".into(), mime: "image/jpeg".into(), bytes: vec![9, 9] },
                DocImage { id: "added".into(), mime: "image/webp".into(), bytes: vec![3] },
            ],
            blocks: vec![
                DocBlock::Paragraph {
                    style_id: Some("keep".into()),
                    runs: vec![DocRun { text: "new".into(), style: RunStyle { bold: true, ..Default::default() } }, DocRun::plain("second run")],
                },
                DocBlock::paragraph("stay"),
            ],
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MutationDiffLaw
    fn sample_mutations() -> Vec<SemioDocumentMutation> {
        vec![
            SemioDocumentMutation::NoMutation,
            SemioDocumentMutation::SetSnapshot { snapshot: sweep_b() },
            SemioDocumentMutation::InsertBlock { path: DocBlockPath::top(1), block: DocBlock::paragraph("x") },
            SemioDocumentMutation::RemoveBlock { path: DocBlockPath::top(0) },
            SemioDocumentMutation::SetBlockContent { path: DocBlockPath::top(0), block: DocBlock::paragraph("y") },
            SemioDocumentMutation::SetParagraphStyle { path: DocBlockPath::top(0), style_id: Some("Normal".into()) },
            SemioDocumentMutation::SetRunText { path: DocBlockPath::top(0), run_index: 0, text: "z".into() },
            SemioDocumentMutation::SetRunStyle { path: DocBlockPath::top(0), run_index: 0, style: RunStyle { bold: true, italic: false, underline: true, ..Default::default() } },
            SemioDocumentMutation::InsertStyle { style: DocStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: None } },
            SemioDocumentMutation::RemoveStyle { id: "Normal".into() },
            SemioDocumentMutation::SetStyleName { id: "Normal".into(), name: "Body".into() },
            SemioDocumentMutation::SetStyleBasedOn { id: "Normal".into(), based_on: Some("Heading1".into()) },
            SemioDocumentMutation::InsertImage { image: DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1] } },
            SemioDocumentMutation::RemoveImage { id: "img1".into() },
        ]
    }

    #[test]
    fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = fixture();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = MutationDiff::apply(&diff_direct, &base);

            let mut via_apply = base.clone();
            let diff_from_apply = apply_semio_document_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[test]
    fn inverse_law() {
        for mutation in sample_mutations() {
            let base = fixture();

            let mut round_tripped = base.clone();
            apply_semio_document_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <SemioDocumentMutation as Mutation<SemioDocumentSnapshot>>::inverse(&mutation, &base) {
                apply_semio_document_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level) failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = MutationDiff::apply(&diff, &base);
            let inverse_diff = DiffAlgebra::inverse(&diff, &base);
            let restored = MutationDiff::apply(&inverse_diff, &next);
            assert_eq!(restored, base, "inverse_law (diff-level) failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    fn assert_absorb_matches_sequential(base: &SemioDocumentSnapshot, d1: &SemioDocumentDiff, d2: &SemioDocumentDiff) -> SemioDocumentDiff {
        let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base));
        let mut absorbed = d1.clone();
        MutationDiff::absorb(&mut absorbed, d2.clone());
        assert_eq!(MutationDiff::apply(&absorbed, base), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
        absorbed
    }

    fn blocks_diff(diff: &SemioDocumentDiff) -> &BlocksDiff {
        diff.blocks.as_ref().expect("blocks diff present")
    }

    #[test]
    fn absorb_law() {
        // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
        {
            let base = fixture();
            let d1 = Mutation::diff(&SemioDocumentMutation::InsertBlock { path: DocBlockPath::top(2), block: DocBlock::paragraph("f") }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&SemioDocumentMutation::RemoveBlock { path: DocBlockPath::top(0) }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = blocks_diff(&absorbed);
            assert_eq!(triple.removed, vec![0]);
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].index, 1);
            assert_eq!(triple.added[0].item, DocBlock::paragraph("f"));
        }

        // Canonical: Insert(2,f)+Insert(2,g) -> both survive.
        {
            let base = fixture();
            let d1 = Mutation::diff(&SemioDocumentMutation::InsertBlock { path: DocBlockPath::top(2), block: DocBlock::paragraph("f") }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&SemioDocumentMutation::InsertBlock { path: DocBlockPath::top(2), block: DocBlock::paragraph("g") }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = blocks_diff(&absorbed);
            assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
            assert!(triple.added.iter().any(|a| a.item == DocBlock::paragraph("f")));
            assert!(triple.added.iter().any(|a| a.item == DocBlock::paragraph("g")));
        }

        // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
        {
            let base = fixture();
            let d1 = Mutation::diff(&SemioDocumentMutation::InsertBlock { path: DocBlockPath::top(1), block: DocBlock::paragraph("f") }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&SemioDocumentMutation::SetRunText { path: DocBlockPath::top(1), run_index: 0, text: "patched".into() }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = blocks_diff(&absorbed);
            assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].item, DocBlock::paragraph("patched"));
        }

        // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
        {
            let base = fixture();
            let d1 = Mutation::diff(&SemioDocumentMutation::SetRunText { path: DocBlockPath::top(1), run_index: 0, text: "patched".into() }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&SemioDocumentMutation::RemoveBlock { path: DocBlockPath::top(1) }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = blocks_diff(&absorbed);
            assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
            assert_eq!(triple.removed, vec![1]);
        }

        // Associativity over a triple.
        {
            let base = fixture();
            let d1 = Mutation::diff(&SemioDocumentMutation::InsertBlock { path: DocBlockPath::top(2), block: DocBlock::paragraph("f") }, &base);
            let mid1 = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&SemioDocumentMutation::InsertBlock { path: DocBlockPath::top(2), block: DocBlock::paragraph("g") }, &mid1);
            let mid2 = MutationDiff::apply(&d2, &mid1);
            let d3 = Mutation::diff(&SemioDocumentMutation::RemoveBlock { path: DocBlockPath::top(0) }, &mid2);
            let sequential = MutationDiff::apply(&d3, &mid2);

            let mut left = d1.clone();
            MutationDiff::absorb(&mut left, d2.clone());
            MutationDiff::absorb(&mut left, d3.clone());

            let mut d2_then_d3 = d2.clone();
            MutationDiff::absorb(&mut d2_then_d3, d3.clone());
            let mut right = d1.clone();
            MutationDiff::absorb(&mut right, d2_then_d3);

            assert_eq!(MutationDiff::apply(&left, &base), sequential, "absorb associativity (left) failed");
            assert_eq!(MutationDiff::apply(&right, &base), sequential, "absorb associativity (right) failed");
        }
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[test]
    fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(MutationDiff::apply(&<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&a, &b), &a), b);
        assert_eq!(MutationDiff::apply(&<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&b, &a), &b), a);

        let sample = fixture();
        assert_eq!(MutationDiff::apply(&<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&sample, &sample), &sample), sample);

        let mut mutated = sample.clone();
        apply_semio_document_mutation(&mut mutated, &SemioDocumentMutation::SetRunText { path: DocBlockPath::top(0), run_index: 0, text: "Chapter Two".into() });
        assert_ne!(sample, mutated);
        assert_eq!(MutationDiff::apply(&<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&sample, &mutated), &sample), mutated);
        assert_eq!(MutationDiff::apply(&<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&mutated, &sample), &mutated), sample);
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️CodecRetentionLaw
    #[test]
    fn codec_retention_law() {
        let snap = sweep_b();
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️FieldSweep
    /// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field (see the
    /// fixtures' doc comment for exactly how each collection flavor -- removed/modified/added --
    /// is exercised).
    #[test]
    fn field_sweep() {
        let a = sweep_a();
        let b = sweep_b();

        let diff_ab = <SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&a, &b);
        assert_eq!(MutationDiff::apply(&diff_ab, &a), b);
        let diff_ba = <SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&b, &a);
        assert_eq!(MutationDiff::apply(&diff_ba, &b), a);
        assert!(<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&a, &a).is_empty());

        let styles_diff = diff_ab.styles.as_ref().expect("styles diff present");
        assert!(!styles_diff.removed.is_empty(), "styles: removed not exercised");
        assert!(!styles_diff.added.is_empty(), "styles: added not exercised");
        let style_mod = styles_diff.modified.iter().find(|m| m.key == "toModify").expect("toModify style modified");
        assert!(style_mod.diff.name.is_some());
        assert_eq!(style_mod.diff.based_on, Some(Some("keep".to_string())), "style based_on tri-state Some(Some(_)) not exercised");

        let images_diff = diff_ab.images.as_ref().expect("images diff present");
        assert!(!images_diff.removed.is_empty(), "images: removed not exercised");
        assert!(!images_diff.added.is_empty(), "images: added not exercised");
        let image_mod = images_diff.modified.iter().find(|m| m.key == "toModify").expect("toModify image modified");
        assert!(image_mod.diff.mime.is_some() && image_mod.diff.bytes.is_some());

        let body_diff = diff_ab.blocks.as_ref().expect("blocks diff present");
        assert!(!body_diff.removed.is_empty(), "blocks: removed not exercised");
        assert_eq!(body_diff.modified.len(), 1);
        let TestDocBlockDiff::Paragraph(p_diff) = &body_diff.modified[0].diff else { panic!("expected paragraph diff") };
        let runs_diff = p_diff.runs.as_ref().expect("modified paragraph: runs not exercised");
        assert_eq!(p_diff.style_id, Some(Some("keep".to_string())), "modified paragraph: style_id tri-state Some(Some(_)) not exercised");
        assert!(!runs_diff.modified.is_empty(), "modified paragraph: runs.modified not exercised");
        let run_diff = &runs_diff.modified[0].diff;
        assert!(run_diff.text.is_some(), "modified run: text not exercised");
        let style_diff = run_diff.style.as_ref().expect("modified run: style not exercised");
        assert!(style_diff.bold.is_some(), "modified run style: bold not exercised");
        assert!(!runs_diff.added.is_empty(), "modified paragraph: runs.added (nested) not exercised");

        let body_diff_ba = diff_ba.blocks.as_ref().expect("blocks diff (b->a) present");
        assert!(!body_diff_ba.added.is_empty(), "blocks (b->a): added not exercised");
        let DocBlock::Table { rows } = &body_diff_ba.added[0].item else { panic!("expected added table") };
        assert!(!rows.is_empty());

        // Some(None) tri-state coverage: style based_on cleared going the OTHER direction.
        let style_mod_ba = diff_ba.styles.as_ref().unwrap().modified.iter().find(|m| m.key == "toModify").expect("toModify present in b->a");
        assert_eq!(style_mod_ba.diff.based_on, Some(None), "style based_on tri-state Some(None) not exercised");
    }
    //#endregion 🔖️FieldSweep

    //#region 🔖️OpTextBinaryRoundtripLaw
    /// 🧪️ `OpText`/`OpBinary` round-trip laws for the hand-rolled `SemioDocumentMutation`
    /// grammar -- exercises every variant, incl. `InsertBlock`'s bare `DocBlock` payload (a
    /// `Table` carrying nested rows/cells/blocks), `SetSnapshot`'s whole snapshot, and every
    /// `Option`/tri-state field.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let table_block = DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] };
        let mutations = vec![
            SemioDocumentMutation::NoMutation,
            SemioDocumentMutation::SetSnapshot { snapshot: sweep_b() },
            SemioDocumentMutation::InsertBlock { path: DocBlockPath::top(1), block: table_block.clone() },
            SemioDocumentMutation::InsertBlock { path: table_path(0, 0, 0, 0), block: DocBlock::paragraph("nested") },
            SemioDocumentMutation::RemoveBlock { path: DocBlockPath::top(0) },
            SemioDocumentMutation::SetBlockContent { path: DocBlockPath::top(0), block: table_block },
            SemioDocumentMutation::SetParagraphStyle { path: DocBlockPath::top(0), style_id: None },
            SemioDocumentMutation::SetHeadingLevel { path: DocBlockPath::top(0), level: 2 },
            SemioDocumentMutation::SetListOrdered { path: DocBlockPath::top(0), ordered: true },
            SemioDocumentMutation::SetRunText { path: DocBlockPath::top(0), run_index: 0, text: "hello world".into() },
            SemioDocumentMutation::SetRunStyle { path: DocBlockPath::top(0), run_index: 0, style: RunStyle { bold: true, size: Some(12.0), font: Some("Arial".into()), ..Default::default() } },
            SemioDocumentMutation::SetImageBlock { path: DocBlockPath::top(0), image_id: "img1".into(), alt: "alt".into(), width: Some(10.0), height: None },
            SemioDocumentMutation::InsertStyle { style: DocStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) } },
            SemioDocumentMutation::RemoveStyle { id: "Normal".into() },
            SemioDocumentMutation::SetStyleName { id: "Normal".into(), name: "Body Text".into() },
            SemioDocumentMutation::SetStyleBasedOn { id: "Normal".into(), based_on: Some("Other".into()) },
            SemioDocumentMutation::SetStyleBasedOn { id: "Normal".into(), based_on: None },
            SemioDocumentMutation::InsertImage { image: DocImage { id: "img2".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] } },
            SemioDocumentMutation::RemoveImage { id: "img2".into() },
            SemioDocumentMutation::SetImageBytes { id: "img1".into(), mime: "image/gif".into(), bytes: vec![7] },
        ];
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioDocumentMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = SemioDocumentMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw
}
//#endregion 🔖️Tests
