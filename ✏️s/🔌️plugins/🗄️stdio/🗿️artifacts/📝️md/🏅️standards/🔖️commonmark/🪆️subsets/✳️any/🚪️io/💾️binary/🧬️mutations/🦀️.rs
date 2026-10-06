//! binary rep for stdio.md 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

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
use crate::schema::diff::{diff_at_path, diff_set_snapshot, MdBlockDiff, MdBlocksLeafDiff, MdDiff};
use crate::schema::snapshot::{MdBlock, MdInline};
use crate::MdSnapshot;
use protocol::{Mutation, OpText};

/// 🧪️ P2-FG1: mutation-specific real binary primitives backing the upgraded `OpBinary` impl below
/// — reuses `MdDiff`'s `pub(crate)` recursive `enc_block_bin`/`enc_inline_list_bin`/`write_str_bin`/
/// `write_option_bin` primitives (`../../🔺️diff/🦀️.rs`, imported above) for the SHARED
/// `MdBlock`/`MdInline` shape (same intra-artifact-reuse split the TEXT codec above already uses),
/// only `MdSnapshot`/`MdPathStep`'s own binary shape is genuinely new here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_snapshot_bin(s: &MdSnapshot, out: &mut Vec<u8>) {
    write_str_bin(out, &s.schema);
    enc_block_list_bin(&s.blocks, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<MdSnapshot, String> {
    let schema = read_str_bin(reader)?;
    let blocks = dec_block_list_bin(reader)?;
    Ok(MdSnapshot { schema, blocks })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_path_step_bin(step: &MdPathStep, out: &mut Vec<u8>) {
    match step {
        MdPathStep::BlockQuote { index } => {
            out.push(0);
            store::pack_rt::write_varint_u64(out, *index as u64);
        }
        MdPathStep::ListItem { index, item } => {
            out.push(1);
            store::pack_rt::write_varint_u64(out, *index as u64);
            store::pack_rt::write_varint_u64(out, *item as u64);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_path_step_bin(reader: &mut store::ByteReader<'_>) -> Result<MdPathStep, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(MdPathStep::BlockQuote { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize }),
        1 => {
            let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
            let item = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
            Ok(MdPathStep::ListItem { index, item })
        }
        other => Err(format!("path step binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_path_bin(path: &[MdPathStep], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, path.len() as u64);
    for step in path {
        enc_path_step_bin(step, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_path_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<MdPathStep>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_path_step_bin(reader)).collect()
}

/// 🧪️ P2-FG1: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the `MdMutation`
/// variant ordinal, same 1-5 order `print_md_mutation`'s own keyword match uses (0 was
/// `NoMutation`'s, dropped by the `26/08/29/S-END-TO-END` mutation-leaf migration; the remaining
/// tags are left as they were rather than renumbered down, since nothing needs them contiguous).
impl protocol::OpBinary for MdMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            MdMutation::SetSnapshot(_) => TAG_SET_SNAPSHOT,
            MdMutation::InsertBlock(_) => TAG_INSERT_BLOCK,
            MdMutation::RemoveBlock(_) => TAG_REMOVE_BLOCK,
            MdMutation::ReplaceBlock(_) => TAG_REPLACE_BLOCK,
            MdMutation::SetInlines(_) => TAG_SET_INLINES,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            MdMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_snapshot_bin(snapshot, &mut out),
            MdMutation::InsertBlock(insert_block::InsertBlock { path, index, block }) => {
                enc_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_block_bin(block, &mut out);
            }
            MdMutation::RemoveBlock(remove_block::RemoveBlock { path, index }) => {
                enc_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path, index, block }) => {
                enc_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_block_bin(block, &mut out);
            }
            MdMutation::SetInlines(set_inlines::SetInlines { path, index, inlines }) => {
                enc_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_inline_list_bin(inlines, &mut out);
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            TAG_SET_SNAPSHOT => {
                let snapshot = dec_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(MdMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            TAG_INSERT_BLOCK => {
                let path = dec_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let block = dec_block_bin(&mut reader).map_err(|e| malformed("op block", reader.position(), e))?;
                Ok(MdMutation::InsertBlock(insert_block::InsertBlock { path, index, block }))
            }
            TAG_REMOVE_BLOCK => {
                let path = dec_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(MdMutation::RemoveBlock(remove_block::RemoveBlock { path, index }))
            }
            TAG_REPLACE_BLOCK => {
                let path = dec_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let block = dec_block_bin(&mut reader).map_err(|e| malformed("op block", reader.position(), e))?;
                Ok(MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path, index, block }))
            }
            TAG_SET_INLINES => {
                let path = dec_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let inlines = dec_inline_list_bin(&mut reader).map_err(|e| malformed("op inlines", reader.position(), e))?;
                Ok(MdMutation::SetInlines(set_inlines::SetInlines { path, index, inlines }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `MdMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_INSERT_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-block");
const TAG_REMOVE_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-block");
const TAG_REPLACE_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "replace-block");
const TAG_SET_INLINES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-inlines");
//#endregion 🏷️WireTags
