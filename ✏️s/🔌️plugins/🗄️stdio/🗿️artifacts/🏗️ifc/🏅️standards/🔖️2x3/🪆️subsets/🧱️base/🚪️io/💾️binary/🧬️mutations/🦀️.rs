//! binary rep for stdio.ifc.2x3 mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v2x3::subsets::base::schema::mutations::*;
use crate::standards::v2x3::subsets::base::schema::diff::{enc_part21_instance, Ifc2x3Diff};
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

/// 🧪️ Mutation-specific real binary primitives backing the upgraded `OpBinary` impl below — reuses
/// the diff sibling's `pub(crate)` recursive `enc_part21_instance_bin`/`enc_part21_header_bin`/
/// `write_str_bin` primitives for the SHARED `Part21Instance`/`Part21Header`/`Part21Value` shape
/// (same intra-artifact-reuse split the TEXT codec above already uses); only `Ifc2x3Snapshot`'s own
/// binary shape is genuinely new here.
pub(crate) fn enc_ifc2x3_snapshot_bin(s: &Ifc2x3Snapshot, out: &mut Vec<u8>) {
    write_str_bin(out, &s.schema);
    enc_part21_header_bin(&s.document.header, out);
    store::pack_rt::write_varint_u64(out, s.document.instances.len() as u64);
    for inst in &s.document.instances {
        enc_part21_instance_bin(inst, out);
    }
    match &s.edm_preamble {
        None => out.push(0),
        Some(value) => {
            out.push(1);
            enc_edm_preamble_bin(value, out);
        }
    }
}

pub(crate) fn dec_ifc2x3_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<Ifc2x3Snapshot, String> {
    let schema = read_str_bin(reader)?;
    let header = dec_part21_header_bin(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut instances = Vec::with_capacity(count as usize);
    for _ in 0..count {
        instances.push(dec_part21_instance_bin(reader)?);
    }
    let edm_preamble = match reader.read_u8().map_err(|e| e.to_string())? {
        0 => None,
        1 => Some(dec_edm_preamble_bin(reader)?),
        tag => return Err(format!("ifc2x3 snapshot: invalid EDM preamble presence {tag}")),
    };
    Ok(Ifc2x3Snapshot { schema, document: Part21Document { header, instances }, edm_preamble })
}

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
            Ifc2x3Mutation::SetSnapshot(..) => TAG_SET_SNAPSHOT,
            Ifc2x3Mutation::PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
            Ifc2x3Mutation::UpsertInstance(..) => TAG_UPSERT_INSTANCE,
            Ifc2x3Mutation::RemoveInstance(..) => TAG_REMOVE_INSTANCE,
            Ifc2x3Mutation::SetHeader(..) => TAG_SET_HEADER,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_ifc2x3_snapshot_bin(snapshot, &mut out),
            Ifc2x3Mutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => out.extend(protocol::OpBinary::encode_op(patch)?),
            Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance }) => enc_part21_instance_bin(instance, &mut out),
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
            TAG_PATCH_SNAPSHOT => Ifc2x3Mutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: <semio_s_artifact_stdio_contract::editing::SnapshotPatch as protocol::OpBinary>::decode_op(reader.read_bytes(reader.remaining()).map_err(|e| protocol::ProtocolError::Malformed { what: "patch-snapshot payload", offset: reader.position() as u64, detail: e.to_string() })?)? }),
            TAG_SET_SNAPSHOT => {
                let snapshot = dec_ifc2x3_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::new(snapshot) })
            }
            TAG_UPSERT_INSTANCE => {
                let instance = dec_part21_instance_bin(&mut reader).map_err(|e| malformed("op instance", reader.position(), e))?;
                Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance })
            }
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
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_PATCH_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "patch-snapshot");
const TAG_UPSERT_INSTANCE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "upsert-instance");
const TAG_REMOVE_INSTANCE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-instance");
const TAG_SET_HEADER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-header");
//#endregion 🏷️WireTags
