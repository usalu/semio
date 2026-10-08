//! 💾️ Binary representation grammar surface for `s.stdio.semio.cad.mutations`.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::cad::schema::mutations::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{NamedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::cad::schema::diff::{wrap_block_diff, wrap_block_entity_diff, wrap_entity_diff, wrap_layer_diff, CadBlockDiff, CadEntityRecordDiff, CadLayerDiff, SemioCadDiff};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_entity_record};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_entity_record};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_layer};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_layer};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_entity};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_entity};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_point2};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_point2};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_block};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_block};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{decode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{encode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntity, CadEntityRecord, CadLayer, SemioCadSnapshot};
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use crate::standards::v1::subsets::cad::io::text::mutations::{print_cad_mutation};
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn wire_tag(m: &SemioCadMutation) -> u8 {
    match m {
        SemioCadMutation::AddLayer(_) => TAG_ADD_LAYER,
        SemioCadMutation::RemoveLayer(_) => TAG_REMOVE_LAYER,
        SemioCadMutation::SetLayer(_) => TAG_SET_LAYER,
        SemioCadMutation::AddBlock(_) => TAG_ADD_BLOCK,
        SemioCadMutation::RemoveBlock(_) => TAG_REMOVE_BLOCK,
        SemioCadMutation::SetBlockBasePoint(_) => TAG_SET_BLOCK_BASE_POINT,
        SemioCadMutation::AddEntity(_) => TAG_ADD_ENTITY,
        SemioCadMutation::RemoveEntity(_) => TAG_REMOVE_ENTITY,
        SemioCadMutation::SetEntityLayer(_) => TAG_SET_ENTITY_LAYER,
        SemioCadMutation::SetEntityGeometry(_) => TAG_SET_ENTITY_GEOMETRY,
        SemioCadMutation::AddBlockEntity(_) => TAG_ADD_BLOCK_ENTITY,
        SemioCadMutation::RemoveBlockEntity(_) => TAG_REMOVE_BLOCK_ENTITY,
        SemioCadMutation::SetBlockEntityLayer(_) => TAG_SET_BLOCK_ENTITY_LAYER,
        SemioCadMutation::SetBlockEntityGeometry(_) => TAG_SET_BLOCK_ENTITY_GEOMETRY,
    }
}

/// ✂️ Just the `key=value ...` argument tail of `print_cad_mutation` — the binary frame's `tag`
/// byte already carries the keyword, so the text keyword itself is redundant in the binary payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_cad_mutation_args(m: &SemioCadMutation) -> String {
    match print_cad_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ Real binary op frame, replacing the old `print_op().into_bytes()` text-as-binary shortcut.
/// `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, see
/// [`KINDS`]) are two REAL fixed fields; the variant's own `key=value ...` argument payload
/// follows as one opaque trailing `bytes` chain — reusing the already-real, already-tested
/// `print_cad_mutation`/`parse_cad_mutation` text codec rather than re-deriving a second
/// independent encoding.
impl OpBinary for SemioCadMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, wire_tag(self)];
        out.extend_from_slice(print_cad_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = dsl::protocol_record::kind(WIRE_PROTOCOL, u64::from(tag)).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} names no record of 📡️.protocol.semio") })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioCadMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_ADD_LAYER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-layer");
const TAG_REMOVE_LAYER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-layer");
const TAG_SET_LAYER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-layer");
const TAG_ADD_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-block");
const TAG_REMOVE_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-block");
const TAG_SET_BLOCK_BASE_POINT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-block-base-point");
const TAG_ADD_ENTITY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-entity");
const TAG_REMOVE_ENTITY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-entity");
const TAG_SET_ENTITY_LAYER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-entity-layer");
const TAG_SET_ENTITY_GEOMETRY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-entity-geometry");
const TAG_ADD_BLOCK_ENTITY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-block-entity");
const TAG_REMOVE_BLOCK_ENTITY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-block-entity");
const TAG_SET_BLOCK_ENTITY_LAYER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-block-entity-layer");
const TAG_SET_BLOCK_ENTITY_GEOMETRY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-block-entity-geometry");
//#endregion 🏷️WireTags
