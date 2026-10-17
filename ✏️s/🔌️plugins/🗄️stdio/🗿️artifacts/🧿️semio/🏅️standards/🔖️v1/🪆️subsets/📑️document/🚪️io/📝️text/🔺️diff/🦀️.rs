//! 📝️ Text representation codec surface for `s.stdio.semio.document.diff` — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::io::binary::diff::{encode_option, decode_option};
use crate::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use semio_s_artifact_stdio_semio_base::schema::triples::IndexAdded;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexModified;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedModified;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_indexed_triple;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_named_added;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_named_triple;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_indexed_triple;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_named_added;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_named_triple;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use crate::schema::snapshot::{DocBlock, DocImage, DocListItem, DocRun, DocStyle, DocTableCell, DocTableRow, RunStyle, SemioDocumentSnapshot};
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio_base::io::text::codec::{hex_encode, hex_decode, enc_str, dec_str};
use crate::io::text::snapshot::{dec_block, dec_bool, dec_cell, dec_f64, dec_image, dec_list, dec_list_item, dec_row, dec_run, dec_run_style, dec_style, dec_u8, enc_block, enc_bool, enc_cell, enc_f64, enc_image, enc_list, enc_list_item, enc_row, enc_run, enc_run_style, enc_style, enc_u8};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_document_diff(d: &SemioDocumentDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.styles {
        tokens.push(format!("styles={}", enc_styles_diff(v)));
    }
    if let Some(v) = &d.images {
        tokens.push(format!("images={}", enc_images_diff(v)));
    }
    if let Some(v) = &d.blocks {
        tokens.push(format!("blocks={}", enc_blocks_diff(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_document_diff(line: &str) -> Result<SemioDocumentDiff, String> {
    let mut d = SemioDocumentDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("styles=") {
            d.styles = Some(dec_styles_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("images=") {
            d.images = Some(dec_images_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("blocks=") {
            d.blocks = Some(dec_blocks_diff(rest)?);
        } else {
            return Err(format!("document diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for SemioDocumentDiff {
fn print_diff(&self) -> String {
    print_document_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_document_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_runs_diff(d: &RunsDiff) -> String {
    enc_indexed_triple(d, enc_run_diff, enc_run)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_runs_diff(s: &str) -> Result<RunsDiff, String> {
    dec_indexed_triple(s, dec_run_diff, dec_run)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_blocks_diff(d: &BlocksDiff) -> String {
    enc_indexed_triple(d, enc_block_diff, enc_block)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_blocks_diff(s: &str) -> Result<BlocksDiff, String> {
    dec_indexed_triple(s, dec_block_diff, dec_block)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list_items_diff(d: &ListItemsDiff) -> String {
    enc_indexed_triple(d, enc_list_item_diff, enc_list_item)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list_items_diff(s: &str) -> Result<ListItemsDiff, String> {
    dec_indexed_triple(s, dec_list_item_diff, dec_list_item)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_table_rows_diff(d: &TableRowsDiff) -> String {
    enc_indexed_triple(d, enc_row_diff, enc_row)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_table_rows_diff(s: &str) -> Result<TableRowsDiff, String> {
    dec_indexed_triple(s, dec_row_diff, dec_row)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_table_cells_diff(d: &TableCellsDiff) -> String {
    enc_indexed_triple(d, enc_cell_diff, enc_cell)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_table_cells_diff(s: &str) -> Result<TableCellsDiff, String> {
    dec_indexed_triple(s, dec_cell_diff, dec_cell)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_styles_diff(d: &StylesDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), enc_style_diff, |a| enc_named_added(a, enc_style))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_styles_diff(s: &str) -> Result<StylesDiff, String> {
    dec_named_triple(s, dec_str, dec_style_diff, |t| dec_named_added(t, dec_style))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_images_diff(d: &ImagesDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), enc_image_diff, |a| enc_named_added(a, enc_image))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_images_diff(s: &str) -> Result<ImagesDiff, String> {
    dec_named_triple(s, dec_str, dec_image_diff, |t| dec_named_added(t, dec_image))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_run_style_diff(d: &RunStyleDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{}]",
        encode_option(&d.bold, enc_bool),
        encode_option(&d.italic, enc_bool),
        encode_option(&d.underline, enc_bool),
        encode_option(&d.size, |v: &Option<f64>| encode_option(v, enc_f64)),
        encode_option(&d.font, |v: &Option<String>| encode_option(v, |s| enc_str(s))),
        encode_option(&d.color, |v: &Option<String>| encode_option(v, |s| enc_str(s))),
        encode_option(&d.link, |v: &Option<String>| encode_option(v, |s| enc_str(s))),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_run_style_diff(s: &str) -> Result<RunStyleDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [bold, italic, underline, size, font, color, link] = parts.as_slice() else { return Err(format!("run style diff: expected 7 fields, got {}", parts.len())) };
    Ok(RunStyleDiff {
        bold: decode_option(bold, dec_bool)?,
        italic: decode_option(italic, dec_bool)?,
        underline: decode_option(underline, dec_bool)?,
        size: decode_option(size, |s| decode_option(s, dec_f64))?,
        font: decode_option(font, |s| decode_option(s, dec_str))?,
        color: decode_option(color, |s| decode_option(s, dec_str))?,
        link: decode_option(link, |s| decode_option(s, dec_str))?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_run_diff(d: &DocRunDiff) -> String {
    format!("[{},{}]", encode_option(&d.text, |v| enc_str(v)), encode_option(&d.style, enc_run_style_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_run_diff(s: &str) -> Result<DocRunDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [text, style] = parts.as_slice() else { return Err(format!("run diff: expected 2 fields, got {}", parts.len())) };
    Ok(DocRunDiff { text: decode_option(text, dec_str)?, style: decode_option(style, dec_run_style_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list_item_diff(d: &DocListItemDiff) -> String {
    format!("[{}]", encode_option(&d.blocks, enc_blocks_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list_item_diff(s: &str) -> Result<DocListItemDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(DocListItemDiff { blocks: decode_option(inner, dec_blocks_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_cell_diff(d: &DocTableCellDiff) -> String {
    format!("[{}]", encode_option(&d.blocks, enc_blocks_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_cell_diff(s: &str) -> Result<DocTableCellDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(DocTableCellDiff { blocks: decode_option(inner, dec_blocks_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_row_diff(d: &DocTableRowDiff) -> String {
    format!("[{}]", encode_option(&d.cells, enc_table_cells_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_row_diff(s: &str) -> Result<DocTableRowDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(DocTableRowDiff { cells: decode_option(inner, dec_table_cells_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style_diff(d: &DocStyleDiff) -> String {
    format!("[{},{}]", encode_option(&d.name, |v| enc_str(v)), encode_option(&d.based_on, |v: &Option<String>| encode_option(v, |s| enc_str(s))))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style_diff(s: &str) -> Result<DocStyleDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [name, based_on] = parts.as_slice() else { return Err(format!("style diff: expected 2 fields, got {}", parts.len())) };
    Ok(DocStyleDiff { name: decode_option(name, dec_str)?, based_on: decode_option(based_on, |s| decode_option(s, dec_str))? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_image_diff(d: &DocImageDiff) -> String {
    format!("[{},{}]", encode_option(&d.mime, |v| enc_str(v)), encode_option(&d.bytes, |v: &Vec<u8>| hex_encode(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_image_diff(s: &str) -> Result<DocImageDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [mime, bytes] = parts.as_slice() else { return Err(format!("image diff: expected 2 fields, got {}", parts.len())) };
    Ok(DocImageDiff { mime: decode_option(mime, dec_str)?, bytes: decode_option(bytes, hex_decode)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_paragraph_diff(d: &DocParagraphDiff) -> String {
    format!("[{},{}]", encode_option(&d.style_id, |v: &Option<String>| encode_option(v, |s| enc_str(s))), encode_option(&d.runs, enc_runs_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_paragraph_diff(s: &str) -> Result<DocParagraphDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [style_id, runs] = parts.as_slice() else { return Err(format!("paragraph diff: expected 2 fields, got {}", parts.len())) };
    Ok(DocParagraphDiff { style_id: decode_option(style_id, |s| decode_option(s, dec_str))?, runs: decode_option(runs, dec_runs_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_heading_diff(d: &DocHeadingDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.level, enc_u8), encode_option(&d.style_id, |v: &Option<String>| encode_option(v, |s| enc_str(s))), encode_option(&d.runs, enc_runs_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_heading_diff(s: &str) -> Result<DocHeadingDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [level, style_id, runs] = parts.as_slice() else { return Err(format!("heading diff: expected 3 fields, got {}", parts.len())) };
    Ok(DocHeadingDiff { level: decode_option(level, dec_u8)?, style_id: decode_option(style_id, |s| decode_option(s, dec_str))?, runs: decode_option(runs, dec_runs_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list_diff(d: &DocListDiff) -> String {
    format!("[{},{}]", encode_option(&d.ordered, enc_bool), encode_option(&d.items, enc_list_items_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list_diff(s: &str) -> Result<DocListDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [ordered, items] = parts.as_slice() else { return Err(format!("list diff: expected 2 fields, got {}", parts.len())) };
    Ok(DocListDiff { ordered: decode_option(ordered, dec_bool)?, items: decode_option(items, dec_list_items_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_table_diff(d: &DocTableDiff) -> String {
    format!("[{}]", encode_option(&d.rows, enc_table_rows_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_table_diff(s: &str) -> Result<DocTableDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(DocTableDiff { rows: decode_option(inner, dec_table_rows_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_code_diff(d: &DocCodeDiff) -> String {
    format!("[{},{}]", encode_option(&d.language, |v: &Option<String>| encode_option(v, |s| enc_str(s))), encode_option(&d.text, |v| enc_str(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_code_diff(s: &str) -> Result<DocCodeDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [language, text] = parts.as_slice() else { return Err(format!("code diff: expected 2 fields, got {}", parts.len())) };
    Ok(DocCodeDiff { language: decode_option(language, |s| decode_option(s, dec_str))?, text: decode_option(text, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quote_diff(d: &DocQuoteDiff) -> String {
    format!("[{}]", encode_option(&d.blocks, enc_blocks_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quote_diff(s: &str) -> Result<DocQuoteDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(DocQuoteDiff { blocks: decode_option(inner, dec_blocks_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_image_block_diff(d: &DocImageBlockDiff) -> String {
    format!(
        "[{},{},{},{}]",
        encode_option(&d.image_id, |v| enc_str(v)),
        encode_option(&d.alt, |v| enc_str(v)),
        encode_option(&d.width, |v: &Option<f64>| encode_option(v, enc_f64)),
        encode_option(&d.height, |v: &Option<f64>| encode_option(v, enc_f64)),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_image_block_diff(s: &str) -> Result<DocImageBlockDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [image_id, alt, width, height] = parts.as_slice() else { return Err(format!("image block diff: expected 4 fields, got {}", parts.len())) };
    Ok(DocImageBlockDiff { image_id: decode_option(image_id, dec_str)?, alt: decode_option(alt, dec_str)?, width: decode_option(width, |s| decode_option(s, dec_f64))?, height: decode_option(height, |s| decode_option(s, dec_f64))? })
}

/// 🌳️ `P[...]`/`H[...]`/`L[...]`/`T[...]`/`C[...]`/`Q[...]`/`I[...]` -- per-kind diff, `R[block]`
/// wholesale replace (block-KIND changed).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_diff(d: &DocBlockDiff) -> String {
    match d {
        DocBlockDiff::Paragraph(pd) => format!("P{}", enc_paragraph_diff(pd)),
        DocBlockDiff::Heading(hd) => format!("H{}", enc_heading_diff(hd)),
        DocBlockDiff::List(ld) => format!("L{}", enc_list_diff(ld)),
        DocBlockDiff::Table(td) => format!("T{}", enc_table_diff(td)),
        DocBlockDiff::Code(cd) => format!("C{}", enc_code_diff(cd)),
        DocBlockDiff::Quote(qd) => format!("Q{}", enc_quote_diff(qd)),
        DocBlockDiff::Image(id) => format!("I{}", enc_image_block_diff(id)),
        DocBlockDiff::Replace { block } => format!("R[{}]", enc_block(block)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_diff(s: &str) -> Result<DocBlockDiff, String> {
    let (tag, rest) = s.split_at(1);
    match tag {
        "P" => Ok(DocBlockDiff::Paragraph(dec_paragraph_diff(rest)?)),
        "H" => Ok(DocBlockDiff::Heading(dec_heading_diff(rest)?)),
        "L" => Ok(DocBlockDiff::List(dec_list_diff(rest)?)),
        "T" => Ok(DocBlockDiff::Table(dec_table_diff(rest)?)),
        "C" => Ok(DocBlockDiff::Code(dec_code_diff(rest)?)),
        "Q" => Ok(DocBlockDiff::Quote(dec_quote_diff(rest)?)),
        "I" => Ok(DocBlockDiff::Image(dec_image_block_diff(rest)?)),
        "R" => Ok(DocBlockDiff::Replace { block: dec_block(strip_brackets(rest)?)? }),
        other => Err(format!("block diff: unknown tag {other:?}")),
    }
}
}
pub use diff_codec::*;
