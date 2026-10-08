//! 📝️ Text representation codec surface for `stdio.md` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_commonmark::subsets::any::schema::mutations::*;
use crate::schema::diff::navigate_container;
use crate::schema::diff::MdPathStep;
use crate::standards::v_commonmark::subsets::any::io::text::diff::{dec_block_list};
use crate::standards::v_commonmark::subsets::any::io::text::diff::{enc_block_list};
use crate::standards::v_commonmark::subsets::any::io::text::diff::{dec_inline_list};
use crate::standards::v_commonmark::subsets::any::io::text::diff::{enc_inline_list};
use crate::standards::v_commonmark::subsets::any::io::text::diff::{dec_str};
use crate::standards::v_commonmark::subsets::any::io::text::diff::{enc_str};
use crate::standards::v_commonmark::subsets::any::io::text::diff::{dec_block};
use crate::standards::v_commonmark::subsets::any::io::text::diff::{enc_block};
use crate::standards::v_commonmark::subsets::any::io::text::diff::{strip_brackets};
use crate::standards::v_commonmark::subsets::any::io::text::diff::{split_top_level};
use crate::standards::v_commonmark::subsets::any::io::text::diff::{parse_usize};
use crate::standards::v_commonmark::subsets::any::io::binary::diff::{dec_block_list_bin};
use crate::standards::v_commonmark::subsets::any::io::binary::diff::{enc_block_list_bin};
use crate::standards::v_commonmark::subsets::any::io::binary::diff::{dec_inline_list_bin};
use crate::standards::v_commonmark::subsets::any::io::binary::diff::{enc_inline_list_bin};
use crate::standards::v_commonmark::subsets::any::io::binary::diff::{read_str_bin};
use crate::standards::v_commonmark::subsets::any::io::binary::diff::{write_str_bin};
use crate::standards::v_commonmark::subsets::any::io::binary::diff::{dec_block_bin};
use crate::standards::v_commonmark::subsets::any::io::binary::diff::{enc_block_bin};
use crate::schema::diff::{diff_at_path, MdBlockDiff, MdBlocksLeafDiff, MdDiff};
use crate::schema::snapshot::{MdBlock, MdInline};
use crate::MdSnapshot;
use protocol::{Mutation, OpText};

/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `MdMutation` (`#[derive(dsl::DslOps)]` confirmed
/// rejected above) — reuses `MdDiff`'s `pub(crate)` grammar primitives (`enc_block`/
/// `enc_inline_list`/`split_top_level`/...) rather than duplicating them a second time in this
/// file, same intra-artifact-reuse pattern `SvgMutation` uses for `SvgDiff`'s primitives. Grammar:
/// `keyword arg=value ...` (space-separated, same shape the derive's own handcrafted-wrapper
/// convention uses), one match arm per variant (no `DslVariants` scaffolding available since
/// nothing here derives it). `MdPathStep` gets tag range Y-Z (see `MdDiff`'s region doc comment for
/// the full tag-vocabulary table).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_path_step(step: &MdPathStep) -> String {
    match step {
        MdPathStep::BlockQuote { index } => format!("Y[{index}]"),
        MdPathStep::ListItem { index, item } => format!("Z[{index},{item}]"),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_path_step(s: &str) -> Result<MdPathStep, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "Y" => Ok(MdPathStep::BlockQuote { index: parse_usize(inner)? }),
        "Z" => {
            let parts = split_top_level(inner, ',');
            let [index, item] = parts.as_slice() else { return Err(format!("list item path step: expected 2 fields, got {}", parts.len())) };
            Ok(MdPathStep::ListItem { index: parse_usize(index)?, item: parse_usize(item)? })
        }
        other => Err(format!("path step: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_path(path: &[MdPathStep]) -> String {
    format!("[{}]", path.iter().map(enc_path_step).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_path(s: &str) -> Result<Vec<MdPathStep>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_path_step).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_md_snapshot(s: &MdSnapshot) -> String {
    format!("[{},{}]", enc_str(&s.schema), enc_block_list(&s.blocks))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_md_snapshot(s: &str) -> Result<MdSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, blocks] = parts.as_slice() else { return Err(format!("md snapshot: expected 2 fields, got {}", parts.len())) };
    Ok(MdSnapshot { schema: dec_str(schema)?, blocks: dec_block_list(blocks)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_source_splices(s: &str) -> Result<Vec<splice_source::SourceSplice>, String> {
    split_top_level(strip_brackets(s)?, ',')
        .into_iter()
        .map(|item| {
            let parts = split_top_level(strip_brackets(item)?, ',');
            let [offset, delete, insert] = parts.as_slice() else { return Err(format!("source splice: expected 3 fields, got {}", parts.len())) };
            let number = |field: &str| field.parse::<u32>().map_err(|error| error.to_string());
            Ok(splice_source::SourceSplice { offset: number(offset)?, delete: number(delete)?, insert: dec_str(insert)? })
        })
        .collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_md_mutation(m: &MdMutation) -> String {
    match m {
        MdMutation::InsertBlock(insert_block::InsertBlock { path, index, block }) => format!("insert-block path={} index={index} block={}", enc_path(path), enc_block(block)),
        MdMutation::RemoveBlock(remove_block::RemoveBlock { path, index }) => format!("remove-block path={} index={index}", enc_path(path)),
        MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path, index, block }) => format!("replace-block path={} index={index} block={}", enc_path(path), enc_block(block)),
        MdMutation::SetInlines(set_inlines::SetInlines { path, index, inlines }) => format!("set-inlines path={} index={index} inlines={}", enc_path(path), enc_inline_list(inlines)),
        MdMutation::SpliceSource(splice_source::SpliceSource { splices }) => format!("splice-source splices=[{}]", splices.iter().map(|splice| format!("[{},{},{}]", splice.offset, splice.delete, enc_str(&splice.insert))).collect::<Vec<_>>().join(",")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_md_mutation(line: &str) -> Result<MdMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("md mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("md mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "insert-block" => Ok(MdMutation::InsertBlock(insert_block::InsertBlock { path: dec_path(arg("path")?)?, index: usize_arg("index")?, block: dec_block(arg("block")?)? })),
        "remove-block" => Ok(MdMutation::RemoveBlock(remove_block::RemoveBlock { path: dec_path(arg("path")?)?, index: usize_arg("index")? })),
        "replace-block" => Ok(MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path: dec_path(arg("path")?)?, index: usize_arg("index")?, block: dec_block(arg("block")?)? })),
        "set-inlines" => Ok(MdMutation::SetInlines(set_inlines::SetInlines { path: dec_path(arg("path")?)?, index: usize_arg("index")?, inlines: dec_inline_list(arg("inlines")?)? })),
        "splice-source" => Ok(MdMutation::SpliceSource(splice_source::SpliceSource { splices: dec_source_splices(arg("splices")?)? })),
        other => Err(format!("md mutation: unknown keyword {other:?}")),
    }
}

impl OpText for MdMutation {
    fn print_op(&self) -> String {
        print_md_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_md_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
