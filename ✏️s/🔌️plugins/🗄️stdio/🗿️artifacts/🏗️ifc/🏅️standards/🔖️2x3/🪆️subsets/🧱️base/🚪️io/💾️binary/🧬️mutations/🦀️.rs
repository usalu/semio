//! binary rep for stdio.ifc.2x3 mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v2x3::subsets::base::schema::mutations::*;
use crate::standards::v2x3::subsets::base::schema::diff::Ifc2x3Diff;
use crate::standards::v2x3::subsets::base::io::text::diff::{dec_part21_instance};
use crate::standards::v2x3::subsets::base::io::binary::diff::{dec_part21_instance_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{enc_part21_instance_bin};
use crate::standards::v2x3::subsets::base::io::text::diff::{dec_instance_list};
use crate::standards::v2x3::subsets::base::io::text::diff::{enc_instance_list_into};
use crate::standards::v2x3::subsets::base::io::text::diff::{dec_part21_header};
use crate::standards::v2x3::subsets::base::io::text::diff::{enc_part21_header};
use crate::standards::v2x3::subsets::base::io::text::diff::{strip_brackets};
use crate::standards::v2x3::subsets::base::io::text::diff::{split_top_level};
use crate::standards::v2x3::subsets::base::io::text::diff::{dec_optional_edm_preamble};
use crate::standards::v2x3::subsets::base::io::text::diff::{enc_optional_edm_preamble};
use crate::standards::v2x3::subsets::base::io::text::diff::{dec_str};
use crate::standards::v2x3::subsets::base::io::text::diff::{enc_str};
use crate::standards::v2x3::subsets::base::io::binary::diff::{dec_part21_header_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{enc_part21_header_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{dec_edm_preamble_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{enc_edm_preamble_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{read_str_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{write_str_bin};
use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use protocol::os_spr::command::DiffAlgebra;
use protocol::Mutation;
#[cfg(test)]
use semio_s_artifact_stdio_contract::part21::Part21Value;
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Instance};

/// 🧪️ REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from the literal-JSON shortcut above. `tag` is the `Ifc2x3Mutation` variant ordinal,
/// same 0-4 order `parse_ifc2x3_mutation`'s own keyword match uses. Every field is real
/// (`id` varints, `Part21Instance`/`Part21Header` field-by-field via the reused diff-sibling
/// primitives) — the only place the recursion bottoms out through a fully spec-expressible
/// per-variant tag (`enc_part21_value_bin`), never an opaque byte-chain fallback.
impl protocol::OpBinary for Ifc2x3Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            Ifc2x3Mutation::UpsertInstance(..) => TAG_UPSERT_INSTANCE,
            Ifc2x3Mutation::RemoveInstance(..) => TAG_REMOVE_INSTANCE,
            Ifc2x3Mutation::SetHeader(..) => TAG_SET_HEADER,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance, index }) => {
                enc_part21_instance_bin(instance, &mut out);
                store::pack_rt::write_varint_u64(&mut out, index.map_or(0, |index| index as u64 + 1));
            }
            Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id }) => store::pack_rt::write_varint_u64(&mut out, *id),
            Ifc2x3Mutation::SetHeader(set_header::SetHeader { header }) => enc_part21_header_bin(header, &mut out),
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed("op format", 0, format!("unsupported format {format}")));
        }
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        let mutation = match tag {
            TAG_REMOVE_INSTANCE => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id })
            }
            TAG_SET_HEADER => {
                let header = dec_part21_header_bin(&mut reader).map_err(|e| malformed("op header", reader.position(), e))?;
                Ifc2x3Mutation::SetHeader(set_header::SetHeader { header })
            }
            other => return Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        };
        if reader.remaining() != 0 {
            return Err(malformed("op trailing bytes", reader.position(), format!("{} trailing bytes", reader.remaining())));
        }
        Ok(mutation)
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `Ifc2x3Mutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_UPSERT_INSTANCE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "upsert-instance");
const TAG_REMOVE_INSTANCE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-instance");
const TAG_SET_HEADER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-header");
//#endregion 🏷️WireTags
