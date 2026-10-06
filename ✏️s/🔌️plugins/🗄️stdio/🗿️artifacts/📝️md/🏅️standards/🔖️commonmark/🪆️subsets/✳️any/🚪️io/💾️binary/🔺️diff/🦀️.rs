//! binary rep for stdio.md 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_commonmark::subsets::any::schema::diff::*;
use crate::schema::snapshot::{MdBlock, MdInline};
use crate::MdSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

/// 🧪️ P2-FG1: real LEB128-varint-framed binary primitives (length-prefixed strings, a tag-byte
/// `Option<T>` wrapper) backing the upgraded `OpBinary` (`../../🧬️mutations/🦀️.rs`) and
/// `DiffCodec` (below) frames — mirrors json's own `write_str_lp`/`read_str_lp` shape, reusing
/// `store::pack_rt::write_varint_u64`/`store::ByteReader` rather than reinventing varint encode/
/// decode. `pub(crate)` so the mutations sibling can reuse these rather than duplicating them a
/// second time in that file (same intra-artifact-reuse split the TEXT codec primitives above use).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bool_bin(out: &mut Vec<u8>, b: bool) {
    out.push(if b { 1 } else { 0 });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bool_bin(reader: &mut store::ByteReader<'_>) -> Result<bool, String> {
    Ok(reader.read_u8().map_err(|e| e.to_string())? != 0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    String::from_utf8(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec()).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_option_bin<T>(out: &mut Vec<u8>, opt: &Option<T>, enc: impl FnOnce(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            enc(v, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_option_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(dec(reader)?)),
        other => Err(format!("option binary: unknown tag {other}")),
    }
}

/// 🏳️ Tri-state `Option<Option<T>>` binary wrapper (`MdBlockDiff::List.start`/`CodeBlock.info`) —
/// `0`=unchanged (`None`), `1`=cleared (`Some(None)`), `2`=set (`Some(Some(v))`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_tristate_bin<T>(out: &mut Vec<u8>, opt: &Option<Option<T>>, enc: impl FnOnce(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(None) => out.push(1),
        Some(Some(v)) => {
            out.push(2);
            enc(v, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_tristate_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<Option<T>>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(None)),
        2 => Ok(Some(Some(dec(reader)?))),
        other => Err(format!("tristate binary: unknown tag {other}")),
    }
}

/// 🧪️ P2-FG1: real recursive binary twin of [`enc_inline`]/[`dec_inline`] above — same 0-8
/// ordinal order as the text codec's `A`-`I` tag range, backing the upgraded `OpBinary`/`DiffCodec`
/// frames (`../../🧬️mutations/🦀️.rs`, `#region 🔖️TopLevel` below).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_inline_bin(n: &MdInline, out: &mut Vec<u8>) {
    match n {
        MdInline::Text { text } => {
            out.push(0);
            write_str_bin(out, text);
        }
        MdInline::Emphasis { inlines } => {
            out.push(1);
            enc_inline_list_bin(inlines, out);
        }
        MdInline::Strong { inlines } => {
            out.push(2);
            enc_inline_list_bin(inlines, out);
        }
        MdInline::Code { literal } => {
            out.push(3);
            write_str_bin(out, literal);
        }
        MdInline::Link { text, url, title } => {
            out.push(4);
            enc_inline_list_bin(text, out);
            write_str_bin(out, url);
            write_option_bin(out, title, |v, o| write_str_bin(o, v));
        }
        MdInline::Image { alt, url, title } => {
            out.push(5);
            write_str_bin(out, alt);
            write_str_bin(out, url);
            write_option_bin(out, title, |v, o| write_str_bin(o, v));
        }
        MdInline::SoftBreak => out.push(6),
        MdInline::HardBreak => out.push(7),
        MdInline::HtmlInline { raw } => {
            out.push(8);
            write_str_bin(out, raw);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_inline_bin(reader: &mut store::ByteReader<'_>) -> Result<MdInline, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(MdInline::Text { text: read_str_bin(reader)? }),
        1 => Ok(MdInline::Emphasis { inlines: dec_inline_list_bin(reader)? }),
        2 => Ok(MdInline::Strong { inlines: dec_inline_list_bin(reader)? }),
        3 => Ok(MdInline::Code { literal: read_str_bin(reader)? }),
        4 => {
            let text = dec_inline_list_bin(reader)?;
            let url = read_str_bin(reader)?;
            let title = read_option_bin(reader, read_str_bin)?;
            Ok(MdInline::Link { text, url, title })
        }
        5 => {
            let alt = read_str_bin(reader)?;
            let url = read_str_bin(reader)?;
            let title = read_option_bin(reader, read_str_bin)?;
            Ok(MdInline::Image { alt, url, title })
        }
        6 => Ok(MdInline::SoftBreak),
        7 => Ok(MdInline::HardBreak),
        8 => Ok(MdInline::HtmlInline { raw: read_str_bin(reader)? }),
        other => Err(format!("inline binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_inline_list_bin(list: &[MdInline], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for n in list {
        enc_inline_bin(n, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_inline_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<MdInline>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_inline_bin(reader)).collect()
}

/// 🧪️ P2-FG1: real recursive binary twin of [`enc_block`]/[`dec_block`] above — same 0-6 ordinal
/// order as the text codec's `J`-`P` tag range.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_bin(b: &MdBlock, out: &mut Vec<u8>) {
    match b {
        MdBlock::Heading { level, inlines } => {
            out.push(0);
            out.push(*level);
            enc_inline_list_bin(inlines, out);
        }
        MdBlock::Paragraph { inlines } => {
            out.push(1);
            enc_inline_list_bin(inlines, out);
        }
        MdBlock::List { ordered, start, tight, items } => {
            out.push(2);
            write_bool_bin(out, *ordered);
            write_option_bin(out, start, |v, o| store::pack_rt::write_varint_u64(o, *v as u64));
            write_bool_bin(out, *tight);
            enc_item_list_bin(items, out);
        }
        MdBlock::CodeBlock { info, literal } => {
            out.push(3);
            write_option_bin(out, info, |v, o| write_str_bin(o, v));
            write_str_bin(out, literal);
        }
        MdBlock::BlockQuote { blocks } => {
            out.push(4);
            enc_block_list_bin(blocks, out);
        }
        MdBlock::ThematicBreak => out.push(5),
        MdBlock::HtmlBlock { raw } => {
            out.push(6);
            write_str_bin(out, raw);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_bin(reader: &mut store::ByteReader<'_>) -> Result<MdBlock, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let level = reader.read_u8().map_err(|e| e.to_string())?;
            let inlines = dec_inline_list_bin(reader)?;
            Ok(MdBlock::Heading { level, inlines })
        }
        1 => Ok(MdBlock::Paragraph { inlines: dec_inline_list_bin(reader)? }),
        2 => {
            let ordered = read_bool_bin(reader)?;
            let start = read_option_bin(reader, |r| Ok(r.read_varint_u64().map_err(|e| e.to_string())? as u32))?;
            let tight = read_bool_bin(reader)?;
            let items = dec_item_list_bin(reader)?;
            Ok(MdBlock::List { ordered, start, tight, items })
        }
        3 => {
            let info = read_option_bin(reader, read_str_bin)?;
            let literal = read_str_bin(reader)?;
            Ok(MdBlock::CodeBlock { info, literal })
        }
        4 => Ok(MdBlock::BlockQuote { blocks: dec_block_list_bin(reader)? }),
        5 => Ok(MdBlock::ThematicBreak),
        6 => Ok(MdBlock::HtmlBlock { raw: read_str_bin(reader)? }),
        other => Err(format!("block binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_list_bin(list: &[MdBlock], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for b in list {
        enc_block_bin(b, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<MdBlock>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_block_bin(reader)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_item_list_bin(items: &[Vec<MdBlock>], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for item in items {
        enc_block_list_bin(item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_item_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<Vec<MdBlock>>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_block_list_bin(reader)).collect()
}

/// 🧪️ P2-FG1: real recursive binary twin of [`enc_block_diff`]/[`dec_block_diff`] — same 0-7
/// ordinal order as the text codec's `Q`-`X` tag range (`7`=`Replace`), backing the upgraded
/// `DiffBinary::encode_diff`/`decode_diff` below. `List`/`CodeBlock`'s tri-state fields use
/// [`write_tristate_bin`]/[`read_tristate_bin`]; every other `Option<T>` field uses the plain
/// [`write_option_bin`]/[`read_option_bin`] pair.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_diff_bin(d: &MdBlockDiff, out: &mut Vec<u8>) {
    match d {
        MdBlockDiff::Heading { level, inlines } => {
            out.push(0);
            write_option_bin(out, level, |v, o| o.push(*v));
            write_option_bin(out, inlines, |v, o| enc_inline_list_bin(v, o));
        }
        MdBlockDiff::Paragraph { inlines } => {
            out.push(1);
            write_option_bin(out, inlines, |v, o| enc_inline_list_bin(v, o));
        }
        MdBlockDiff::List { ordered, start, tight, items } => {
            out.push(2);
            write_option_bin(out, ordered, |v, o| write_bool_bin(o, *v));
            write_tristate_bin(out, start, |v, o| store::pack_rt::write_varint_u64(o, *v as u64));
            write_option_bin(out, tight, |v, o| write_bool_bin(o, *v));
            write_option_bin(out, items, enc_list_items_diff_bin);
        }
        MdBlockDiff::CodeBlock { info, literal } => {
            out.push(3);
            write_tristate_bin(out, info, |v, o| write_str_bin(o, v));
            write_option_bin(out, literal, |v, o| write_str_bin(o, v));
        }
        MdBlockDiff::BlockQuote { blocks } => {
            out.push(4);
            write_option_bin(out, blocks, enc_blocks_diff_bin);
        }
        MdBlockDiff::ThematicBreak => out.push(5),
        MdBlockDiff::HtmlBlock { raw } => {
            out.push(6);
            write_option_bin(out, raw, |v, o| write_str_bin(o, v));
        }
        MdBlockDiff::Replace { block } => {
            out.push(7);
            enc_block_bin(block, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<MdBlockDiff, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let level = read_option_bin(reader, |r| r.read_u8().map_err(|e| e.to_string()))?;
            let inlines = read_option_bin(reader, dec_inline_list_bin)?;
            Ok(MdBlockDiff::Heading { level, inlines })
        }
        1 => Ok(MdBlockDiff::Paragraph { inlines: read_option_bin(reader, dec_inline_list_bin)? }),
        2 => {
            let ordered = read_option_bin(reader, read_bool_bin)?;
            let start = read_tristate_bin(reader, |r| Ok(r.read_varint_u64().map_err(|e| e.to_string())? as u32))?;
            let tight = read_option_bin(reader, read_bool_bin)?;
            let items = read_option_bin(reader, dec_list_items_diff_bin)?;
            Ok(MdBlockDiff::List { ordered, start, tight, items })
        }
        3 => {
            let info = read_tristate_bin(reader, read_str_bin)?;
            let literal = read_option_bin(reader, read_str_bin)?;
            Ok(MdBlockDiff::CodeBlock { info, literal })
        }
        4 => Ok(MdBlockDiff::BlockQuote { blocks: read_option_bin(reader, dec_blocks_diff_bin)? }),
        5 => Ok(MdBlockDiff::ThematicBreak),
        6 => Ok(MdBlockDiff::HtmlBlock { raw: read_option_bin(reader, read_str_bin)? }),
        7 => Ok(MdBlockDiff::Replace { block: dec_block_bin(reader)? }),
        other => Err(format!("block diff binary: unknown tag {other}")),
    }
}

/// 🌳 `MdBlocksDiff` binary twin of [`enc_blocks_diff`]/[`dec_blocks_diff`] — three varint-counted,
/// recursively-encoded lists (removed/modified/added), genuinely structured binary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_blocks_diff_bin(d: &MdBlocksDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for idx in &d.removed {
        store::pack_rt::write_varint_u64(out, *idx as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for entry in &d.modified {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_block_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for entry in &d.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_block_bin(&entry.item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_blocks_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<MdBlocksDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let diff = dec_block_diff_bin(reader)?;
        modified.push(MdBlockModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_block_bin(reader)?;
        added.push(MdBlockAdded { index, item });
    }
    Ok(MdBlocksDiff { removed, modified, added })
}

/// 🌳 `MdListItemsDiff` binary twin of [`enc_list_items_diff`]/[`dec_list_items_diff`] — same
/// 3-part shape, `modified.diff` a recursive `MdBlocksDiff`, `added.item` a `Vec<MdBlock>`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list_items_diff_bin(d: &MdListItemsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for idx in &d.removed {
        store::pack_rt::write_varint_u64(out, *idx as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for entry in &d.modified {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_blocks_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for entry in &d.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_block_list_bin(&entry.item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list_items_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<MdListItemsDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let diff = dec_blocks_diff_bin(reader)?;
        modified.push(MdListItemModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_block_list_bin(reader)?;
        added.push(MdListItemAdded { index, item });
    }
    Ok(MdListItemsDiff { removed, modified, added })
}

impl protocol::DiffBinary for MdDiff {
/// 🧪️ P2-FG1: REAL binary frame (`format u8 | has_value u8 | blocks-diff payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100% of stdio's
/// `DiffCodec` impls were still on that shortcut per the P2-W0 census).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, if self.blocks.is_some() { 1 } else { 0 }];
    if let Some(blocks) = &self.blocks {
        enc_blocks_diff_bin(blocks, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let _format = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: e.to_string() })?;
    let has_value = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "diff has_value", offset: 1, detail: e.to_string() })?;
    let blocks = if has_value != 0 { Some(dec_blocks_diff_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff blocks", offset: reader.position() as u64, detail: e })?) } else { None };
    Ok(MdDiff { blocks })
}
}

}
pub use diff_codec::*;
