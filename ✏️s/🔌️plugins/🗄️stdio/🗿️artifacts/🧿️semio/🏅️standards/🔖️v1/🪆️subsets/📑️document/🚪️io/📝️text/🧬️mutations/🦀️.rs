//! 📝️ Text representation codec surface for `s.stdio.semio.document.mutations` — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::document::schema::mutations::*;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::document::schema::diff::{diff_block, BlocksDiff, DocBlockDiff, DocHeadingDiff, DocParagraphDiff, DocQuoteDiff, DocRunDiff, DocTableCellDiff, DocTableRowDiff, ListItemsDiff, RunsDiff, SemioDocumentDiff, TableCellsDiff, TableRowsDiff};
use crate::standards::v1::subsets::document::io::text::diff::{dec_run_style};
use crate::standards::v1::subsets::document::io::text::diff::{enc_run_style};
use crate::standards::v1::subsets::document::io::text::diff::{dec_u8};
use crate::standards::v1::subsets::document::io::text::diff::{enc_u8};
use crate::standards::v1::subsets::document::io::text::diff::{dec_image};
use crate::standards::v1::subsets::document::io::text::diff::{enc_image};
use crate::standards::v1::subsets::video::io::text::snapshot::{dec_bool};
use crate::standards::v1::subsets::document::io::text::diff::{dec_style};
use crate::standards::v1::subsets::document::io::text::diff::{enc_style};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_bool};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{decode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{encode_option};
use crate::standards::v1::subsets::flow::io::text::snapshot::{dec_f64};
use crate::standards::v1::subsets::flow::io::text::snapshot::{enc_f64};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{hex_decode};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{hex_encode};
use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocRun, DocStyle, RunStyle, SemioDocumentSnapshot};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioDocumentMutation` block
/// below calls `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in
/// scope in production code too, not merely under `#[cfg(test)]` (W2b closer fix).
use protocol::{OpBinary, OpText};

/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", ...}`) JSON
/// projection — the shape `📃️mutate-semio-document`'s committed specification vectors carry in their
/// `mutation` member — into a real [`SemioDocumentMutation`]. A thin `pack::from_json_str` wrapper (over
/// `ToValue`/`FromValue`, first-party, per this ticket's serde→value conversion), so the test adapter reads
/// the committed vector instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_document_mutation_json(text: &str) -> Result<SemioDocumentMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 🎙️ Hand-rolled `OpText`/`OpBinary`: `keyword arg=value ...` grammar (space-separated), same
/// shape docx/svg/gif's hand-rolled ops use.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_path_segment(seg: &DocPathSegment) -> String {
    match seg {
        DocPathSegment::Quote { block_index } => format!("Q[{block_index}]"),
        DocPathSegment::ListItem { block_index, item } => format!("L[{block_index},{item}]"),
        DocPathSegment::TableCell { block_index, row, cell } => format!("T[{block_index},{row},{cell}]"),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_path_segment(s: &str) -> Result<DocPathSegment, String> {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_path(p: &DocBlockPath) -> String {
    format!("[{},{}]", enc_list(&p.segments, enc_path_segment), p.index)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_path(s: &str) -> Result<DocBlockPath, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [segments, index] = parts.as_slice() else { return Err(format!("block path: expected 2 fields, got {}", parts.len())) };
    Ok(DocBlockPath { segments: dec_list(segments, dec_path_segment)?, index: parse_usize(index)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

/// 🌱 Full (non-diff) `DocBlock`/`SemioDocumentSnapshot` codecs -- only `SetSnapshot`/
/// `InsertBlock`/`SetBlockContent`'s whole-payload encoding needs these; reuses `SemioDocumentDiff`'s
/// `pub(crate)` `enc_block`/`enc_style`/`enc_image` for the shared per-item shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block(b: &DocBlock) -> String {
    crate::standards::v1::subsets::document::io::text::diff::enc_block(b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block(s: &str) -> Result<DocBlock, String> {
    crate::standards::v1::subsets::document::io::text::diff::dec_block(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_run_style_full(s: &RunStyle) -> String {
    enc_run_style(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_run_style_full(s: &str) -> Result<RunStyle, String> {
    dec_run_style(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_snapshot(s: &SemioDocumentSnapshot) -> String {
    format!("[{},{},{}]", enc_list(&s.styles, enc_style), enc_list(&s.images, enc_image), enc_list(&s.blocks, enc_block))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_snapshot(s: &str) -> Result<SemioDocumentSnapshot, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [styles, images, blocks] = parts.as_slice() else { return Err(format!("snapshot: expected 3 fields, got {}", parts.len())) };
    Ok(SemioDocumentSnapshot {
        schema: crate::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
        styles: dec_list(styles, dec_style)?,
        images: dec_list(images, dec_image)?,
        blocks: dec_list(blocks, dec_block)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_at(at: Option<usize>) -> String {
    at.map(|at| format!(" at={at}")).unwrap_or_default()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_at(args: &std::collections::BTreeMap<&str, &str>) -> Result<Option<usize>, String> {
    args.get("at").map(|at| at.parse::<usize>().map_err(|error| error.to_string())).transpose()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_document_mutation(m: &SemioDocumentMutation) -> String {
    match m {
        SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path, block }) => format!("insert-block path={} block={}", enc_block_path(path), enc_block(block)),
        SemioDocumentMutation::RemoveBlock(remove_block::RemoveBlock { path }) => format!("remove-block path={}", enc_block_path(path)),
        SemioDocumentMutation::SetBlockContent(set_block_content::SetBlockContent { path, block }) => format!("set-block-content path={} block={}", enc_block_path(path), enc_block(block)),
        SemioDocumentMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { path, style_id }) => format!("set-paragraph-style path={} style-id={}", enc_block_path(path), encode_option(style_id, |v| enc_str(v))),
        SemioDocumentMutation::SetHeadingLevel(set_heading_level::SetHeadingLevel { path, level }) => format!("set-heading-level path={} level={}", enc_block_path(path), enc_u8(level)),
        SemioDocumentMutation::SetListOrdered(set_list_ordered::SetListOrdered { path, ordered }) => format!("set-list-ordered path={} ordered={}", enc_block_path(path), enc_bool(*ordered)),
        SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path, run_index, text }) => format!("set-run-text path={} run-index={} text={}", enc_block_path(path), run_index, enc_str(text)),
        SemioDocumentMutation::SetRunStyle(set_run_style::SetRunStyle { path, run_index, style }) => format!("set-run-style path={} run-index={} style={}", enc_block_path(path), run_index, enc_run_style_full(style)),
        SemioDocumentMutation::SetImageBlock(set_image_block::SetImageBlock { path, image_id, alt, width, height }) => {
            format!("set-image-block path={} image-id={} alt={} width={} height={}", enc_block_path(path), enc_str(image_id), enc_str(alt), encode_option(width, |value|enc_f64(*value)), encode_option(height, |value|enc_f64(*value)))
        }
        SemioDocumentMutation::InsertStyle(insert_style::InsertStyle { style, at }) => format!("insert-style style={}{}", enc_style(style), enc_at(*at)),
        SemioDocumentMutation::RemoveStyle(remove_style::RemoveStyle { id }) => format!("remove-style id={}", enc_str(id)),
        SemioDocumentMutation::SetStyleName(set_style_name::SetStyleName { id, name }) => format!("set-style-name id={} name={}", enc_str(id), enc_str(name)),
        SemioDocumentMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id, based_on }) => format!("set-style-based-on id={} based-on={}", enc_str(id), encode_option(based_on, |v| enc_str(v))),
        SemioDocumentMutation::InsertImage(insert_image::InsertImage { image, at }) => format!("insert-image image={}{}", enc_image(image), enc_at(*at)),
        SemioDocumentMutation::RemoveImage(remove_image::RemoveImage { id }) => format!("remove-image id={}", enc_str(id)),
        SemioDocumentMutation::SetImageBytes(set_image_bytes::SetImageBytes { id, mime, bytes }) => format!("set-image-bytes id={} mime={} bytes={}", enc_str(id), enc_str(mime), hex_encode(bytes)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_document_mutation(line: &str) -> Result<SemioDocumentMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> =
        rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("document mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("document mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "insert-block" => Ok(SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: dec_block_path(arg("path")?)?, block: dec_block(arg("block")?)? })),
        "remove-block" => Ok(SemioDocumentMutation::RemoveBlock(remove_block::RemoveBlock { path: dec_block_path(arg("path")?)? })),
        "set-block-content" => Ok(SemioDocumentMutation::SetBlockContent(set_block_content::SetBlockContent { path: dec_block_path(arg("path")?)?, block: dec_block(arg("block")?)? })),
        "set-paragraph-style" => Ok(SemioDocumentMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { path: dec_block_path(arg("path")?)?, style_id: decode_option(arg("style-id")?, dec_str)? })),
        "set-heading-level" => Ok(SemioDocumentMutation::SetHeadingLevel(set_heading_level::SetHeadingLevel { path: dec_block_path(arg("path")?)?, level: dec_u8(arg("level")?)? })),
        "set-list-ordered" => Ok(SemioDocumentMutation::SetListOrdered(set_list_ordered::SetListOrdered { path: dec_block_path(arg("path")?)?, ordered: dec_bool(arg("ordered")?)? })),
        "set-run-text" => Ok(SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path: dec_block_path(arg("path")?)?, run_index: usize_arg("run-index")?, text: dec_str(arg("text")?)? })),
        "set-run-style" => Ok(SemioDocumentMutation::SetRunStyle(set_run_style::SetRunStyle { path: dec_block_path(arg("path")?)?, run_index: usize_arg("run-index")?, style: dec_run_style_full(arg("style")?)? })),
        "set-image-block" => Ok(SemioDocumentMutation::SetImageBlock(set_image_block::SetImageBlock {
            path: dec_block_path(arg("path")?)?,
            image_id: dec_str(arg("image-id")?)?,
            alt: dec_str(arg("alt")?)?,
            width: decode_option(arg("width")?, dec_f64)?,
            height: decode_option(arg("height")?, dec_f64)?,
        })),
        "insert-style" => Ok(SemioDocumentMutation::InsertStyle(insert_style::InsertStyle { style: dec_style(arg("style")?)?, at: dec_at(&args)? })),
        "remove-style" => Ok(SemioDocumentMutation::RemoveStyle(remove_style::RemoveStyle { id: dec_str(arg("id")?)? })),
        "set-style-name" => Ok(SemioDocumentMutation::SetStyleName(set_style_name::SetStyleName { id: dec_str(arg("id")?)?, name: dec_str(arg("name")?)? })),
        "set-style-based-on" => Ok(SemioDocumentMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: dec_str(arg("id")?)?, based_on: decode_option(arg("based-on")?, dec_str)? })),
        "insert-image" => Ok(SemioDocumentMutation::InsertImage(insert_image::InsertImage { image: dec_image(arg("image")?)?, at: dec_at(&args)? })),
        "remove-image" => Ok(SemioDocumentMutation::RemoveImage(remove_image::RemoveImage { id: dec_str(arg("id")?)? })),
        "set-image-bytes" => Ok(SemioDocumentMutation::SetImageBytes(set_image_bytes::SetImageBytes { id: dec_str(arg("id")?)?, mime: dec_str(arg("mime")?)?, bytes: hex_decode(arg("bytes")?)? })),
        other => Err(format!("document mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioDocumentMutation {
    fn print_op(&self) -> String {
        print_document_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_document_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
