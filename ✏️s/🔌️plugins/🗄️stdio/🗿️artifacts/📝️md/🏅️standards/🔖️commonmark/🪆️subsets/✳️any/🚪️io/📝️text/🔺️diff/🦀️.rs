//! 📝️ Text representation codec surface for `stdio.md` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type MdDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_commonmark::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::snapshot::{MdBlock, MdInline};
use crate::MdSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bool(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bool: expected 0/1, got {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}

/// 🌳 `MdInline`, tag range A-I (see file doc comment) — order matches its declaration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_inline(n: &MdInline) -> String {
    match n {
        MdInline::Text { text } => format!("A[{}]", enc_str(text)),
        MdInline::Emphasis { inlines } => format!("B[{}]", enc_inline_list(inlines)),
        MdInline::Strong { inlines } => format!("C[{}]", enc_inline_list(inlines)),
        MdInline::Code { literal } => format!("D[{}]", enc_str(literal)),
        MdInline::Link { text, url, title } => {
            format!("E[{},{},{}]", enc_inline_list(text), enc_str(url), encode_option(title, |v| enc_str(v)))
        }
        MdInline::Image { alt, url, title } => {
            format!("F[{},{},{}]", enc_str(alt), enc_str(url), encode_option(title, |v| enc_str(v)))
        }
        MdInline::SoftBreak => "G[]".to_string(),
        MdInline::HardBreak => "H[]".to_string(),
        MdInline::HtmlInline { raw } => format!("I[{}]", enc_str(raw)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_inline(s: &str) -> Result<MdInline, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "A" => Ok(MdInline::Text { text: dec_str(inner)? }),
        "B" => Ok(MdInline::Emphasis { inlines: dec_inline_list(inner)? }),
        "C" => Ok(MdInline::Strong { inlines: dec_inline_list(inner)? }),
        "D" => Ok(MdInline::Code { literal: dec_str(inner)? }),
        "E" => {
            let parts = split_top_level(inner, ',');
            let [text, url, title] = parts.as_slice() else { return Err(format!("inline link: expected 3 fields, got {}", parts.len())) };
            Ok(MdInline::Link { text: dec_inline_list(text)?, url: dec_str(url)?, title: decode_option(title, dec_str)? })
        }
        "F" => {
            let parts = split_top_level(inner, ',');
            let [alt, url, title] = parts.as_slice() else { return Err(format!("inline image: expected 3 fields, got {}", parts.len())) };
            Ok(MdInline::Image { alt: dec_str(alt)?, url: dec_str(url)?, title: decode_option(title, dec_str)? })
        }
        "G" => Ok(MdInline::SoftBreak),
        "H" => Ok(MdInline::HardBreak),
        "I" => Ok(MdInline::HtmlInline { raw: dec_str(inner)? }),
        other => Err(format!("inline: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_inline_list(list: &[MdInline]) -> String {
    format!("[{}]", list.iter().map(enc_inline).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_inline_list(s: &str) -> Result<Vec<MdInline>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_inline).collect()
}

/// 🧱 `MdBlock`, tag range J-P (see file doc comment) — order matches its declaration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block(b: &MdBlock) -> String {
    match b {
        MdBlock::Heading { level, inlines } => format!("J[{},{}]", level, enc_inline_list(inlines)),
        MdBlock::Paragraph { inlines } => format!("K[{}]", enc_inline_list(inlines)),
        MdBlock::List { ordered, start, tight, items } => format!("L[{},{},{},{}]", enc_bool(*ordered), encode_option(start, |v| v.to_string()), enc_bool(*tight), enc_item_list(items),),
        MdBlock::CodeBlock { info, literal } => format!("M[{},{}]", encode_option(info, |v| enc_str(v)), enc_str(literal)),
        MdBlock::BlockQuote { blocks } => format!("N[{}]", enc_block_list(blocks)),
        MdBlock::ThematicBreak => "O[]".to_string(),
        MdBlock::HtmlBlock { raw } => format!("P[{}]", enc_str(raw)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block(s: &str) -> Result<MdBlock, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "J" => {
            let parts = split_top_level(inner, ',');
            let [level, inlines] = parts.as_slice() else { return Err(format!("heading: expected 2 fields, got {}", parts.len())) };
            Ok(MdBlock::Heading { level: level.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, inlines: dec_inline_list(inlines)? })
        }
        "K" => Ok(MdBlock::Paragraph { inlines: dec_inline_list(inner)? }),
        "L" => {
            let parts = split_top_level(inner, ',');
            let [ordered, start, tight, items] = parts.as_slice() else { return Err(format!("list: expected 4 fields, got {}", parts.len())) };
            Ok(MdBlock::List { ordered: dec_bool(ordered)?, start: decode_option(start, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, tight: dec_bool(tight)?, items: dec_item_list(items)? })
        }
        "M" => {
            let parts = split_top_level(inner, ',');
            let [info, literal] = parts.as_slice() else { return Err(format!("code block: expected 2 fields, got {}", parts.len())) };
            Ok(MdBlock::CodeBlock { info: decode_option(info, dec_str)?, literal: dec_str(literal)? })
        }
        "N" => Ok(MdBlock::BlockQuote { blocks: dec_block_list(inner)? }),
        "O" => Ok(MdBlock::ThematicBreak),
        "P" => Ok(MdBlock::HtmlBlock { raw: dec_str(inner)? }),
        other => Err(format!("block: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_list(list: &[MdBlock]) -> String {
    format!("[{}]", list.iter().map(enc_block).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_list(s: &str) -> Result<Vec<MdBlock>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_block).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_item_list(items: &[Vec<MdBlock>]) -> String {
    format!("[{}]", items.iter().map(|item| enc_block_list(item)).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_item_list(s: &str) -> Result<Vec<Vec<MdBlock>>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_block_list).collect()
}

/// 🌳 `MdBlockDiff`, tag range Q-X (see file doc comment) — order matches its declaration, `X` =
/// `Replace` (the kind-change fallback, mirrors `SvgNodeDiff::Replace`'s `R`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_diff(d: &MdBlockDiff) -> String {
    match d {
        MdBlockDiff::Heading { level, inlines } => {
            format!("Q[{},{}]", encode_option(level, |v| v.to_string()), encode_option(inlines, |v| enc_inline_list(v)))
        }
        MdBlockDiff::Paragraph { inlines } => format!("R[{}]", encode_option(inlines, |v| enc_inline_list(v))),
        MdBlockDiff::List { ordered, start, tight, items } => {
            format!("S[{},{},{},{}]", encode_option(ordered, |v| enc_bool(*v).to_string()), encode_option(start, |v| encode_option(v, |x| x.to_string())), encode_option(tight, |v| enc_bool(*v).to_string()), encode_option(items, enc_list_items_diff),)
        }
        MdBlockDiff::CodeBlock { info, literal } => format!("T[{},{}]", encode_option(info, |v| encode_option(v, |x| enc_str(x))), encode_option(literal, |v| enc_str(v)),),
        MdBlockDiff::BlockQuote { blocks } => format!("U[{}]", encode_option(blocks, enc_blocks_diff)),
        MdBlockDiff::ThematicBreak => "V[]".to_string(),
        MdBlockDiff::HtmlBlock { raw } => format!("W[{}]", encode_option(raw, |v| enc_str(v))),
        MdBlockDiff::Replace { block } => format!("X[{}]", enc_block(block)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_diff(s: &str) -> Result<MdBlockDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "Q" => {
            let parts = split_top_level(inner, ',');
            let [level, inlines] = parts.as_slice() else { return Err(format!("heading diff: expected 2 fields, got {}", parts.len())) };
            Ok(MdBlockDiff::Heading { level: decode_option(level, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, inlines: decode_option(inlines, dec_inline_list)? })
        }
        "R" => Ok(MdBlockDiff::Paragraph { inlines: decode_option(inner, dec_inline_list)? }),
        "S" => {
            let parts = split_top_level(inner, ',');
            let [ordered, start, tight, items] = parts.as_slice() else { return Err(format!("list diff: expected 4 fields, got {}", parts.len())) };
            Ok(MdBlockDiff::List {
                ordered: decode_option(ordered, dec_bool)?,
                start: decode_option(start, |v| decode_option(v, |x| x.parse().map_err(|e: std::num::ParseIntError| e.to_string())))?,
                tight: decode_option(tight, dec_bool)?,
                items: decode_option(items, dec_list_items_diff)?,
            })
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [info, literal] = parts.as_slice() else { return Err(format!("code block diff: expected 2 fields, got {}", parts.len())) };
            Ok(MdBlockDiff::CodeBlock { info: decode_option(info, |v| decode_option(v, dec_str))?, literal: decode_option(literal, dec_str)? })
        }
        "U" => Ok(MdBlockDiff::BlockQuote { blocks: decode_option(inner, dec_blocks_diff)? }),
        "V" => Ok(MdBlockDiff::ThematicBreak),
        "W" => Ok(MdBlockDiff::HtmlBlock { raw: decode_option(inner, dec_str)? }),
        "X" => Ok(MdBlockDiff::Replace { block: dec_block(inner)? }),
        other => Err(format!("block diff: unknown tag {other:?}")),
    }
}

/// 🌳 `MdBlocksDiff` (BARE triple, no tag) — reused verbatim by `MdDiff.blocks`, `BlockQuote.blocks`,
/// and (via `MdListItemsDiff`) a `List` item's own content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_blocks_diff(d: &MdBlocksDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_block_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_block(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_blocks_diff(body: &str) -> Result<MdBlocksDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("blocks diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("block modified: bad entry {entry:?}"))?;
            Ok(MdBlockModified { index: parse_usize(idx)?, diff: dec_block_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("block added: bad entry {entry:?}"))?;
            Ok(MdBlockAdded { index: parse_usize(idx)?, item: dec_block(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(MdBlocksDiff { removed, modified, added })
}

/// 🌳 `MdListItemsDiff` (BARE triple, no tag) over a `List`'s `items: Vec<Vec<MdBlock>>`.
/// `modified` entries wrap their nested `MdBlocksDiff` in an EXTRA bracket pair (`{}:[{}]`, not
/// `{}:{}`) — see the region doc comment's "one structural device worth flagging" note for why a
/// bare triple embedded directly (not via `encode_option` or a tag-prefixed enum) needs it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list_items_diff(d: &MdListItemsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:[{}]", m.index, enc_blocks_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_block_list(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list_items_diff(body: &str) -> Result<MdListItemsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("list items diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("list item modified: bad entry {entry:?}"))?;
            Ok(MdListItemModified { index: parse_usize(idx)?, diff: dec_blocks_diff(strip_brackets(rest)?)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("list item added: bad entry {entry:?}"))?;
            Ok(MdListItemAdded { index: parse_usize(idx)?, item: dec_block_list(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(MdListItemsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_md_diff(d: &MdDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.blocks {
        tokens.push(format!("blocks={}", enc_blocks_diff(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_md_diff(line: &str) -> Result<MdDiff, String> {
    let mut d = MdDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("blocks=") {
            d.blocks = Some(dec_blocks_diff(rest)?);
        } else {
            return Err(format!("md diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for MdDiff {
fn print_diff(&self) -> String {
    print_md_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_md_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}

}
pub use diff_codec::*;
