//! las rep for stdio.las 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1_0::subsets::any::schema::mutations::*;
use crate::schema::diff::{self, LasDiff};
use crate::schema::snapshot::{LasHeader, LasPoint, LasVlr};
use crate::LasSnapshot;
use protocol::Mutation;

/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: REAL binary
/// twins backing the upgraded `OpBinary::encode_op`/`decode_op` below — replaces the old F6
/// `print_las_mutation(self).into_bytes()` text-as-binary shortcut. Reuses the diff facet's own
/// `write_bytes_lp`/`write_str_lp`/`enc_vlr_bin`/`enc_point_bin` primitives
/// (`../🔺️diff/🦀️.rs`'s `#region 🔖️BinaryDiffCodec`, `pub(crate)`) — `LasHeader`/
/// `LasVlr`/`LasPoint` are the SAME real records whether embedded in a sparse diff-patch or (here)
/// an `InsertVlr`/`InsertPoint`/`SetPoint` payload, so one binary encoder per
/// record type, shared across both facets, is the correct de-duplication (not a second,
/// independently-drifting copy).
pub(crate) fn enc_f64x3_bin(t: (f64, f64, f64), out: &mut Vec<u8>) {
    out.extend_from_slice(&t.0.to_le_bytes());
    out.extend_from_slice(&t.1.to_le_bytes());
    out.extend_from_slice(&t.2.to_le_bytes());
}

pub(crate) fn dec_f64x3_bin(reader: &mut store::ByteReader<'_>) -> Result<(f64, f64, f64), String> {
    Ok((reader.read_f64_le().map_err(|e| e.to_string())?, reader.read_f64_le().map_err(|e| e.to_string())?, reader.read_f64_le().map_err(|e| e.to_string())?))
}

impl protocol::OpBinary for LasMutation {
    /// ⚡️ REAL binary frame (`format u8 | tag u8 | <variant-specific fields>`), matching
    /// `../💾️binary/📡️.protocol.semio`'s `format`/`tag` leading fields exactly —
    /// upgraded from F6's `print_las_mutation(self).into_bytes()` text-as-binary shortcut. Every
    /// variant's payload is genuinely, individually field-by-field encoded below (see
    /// `#region 🔖️BinaryOpCodec` for the shared record encoders).
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
        match self {
            LasMutation::SetVersion(set_version::SetVersion { major, minor }) => {
                out.push(TAG_SET_VERSION);
                out.push(*major);
                out.push(*minor);
            }
            LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier }) => {
                out.push(TAG_SET_SYSTEM_IDENTIFIER);
                crate::standards::v1_0::subsets::any::io::binary::diff::write_str_lp(&mut out, system_identifier);
            }
            LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software }) => {
                out.push(TAG_SET_SOFTWARE_INFO);
                crate::standards::v1_0::subsets::any::io::binary::diff::write_str_lp(&mut out, generating_software);
            }
            LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year, year }) => {
                out.push(TAG_SET_CREATION_DATE);
                store::pack_rt::write_varint_u64(&mut out, *day_of_year as u64);
                store::pack_rt::write_varint_u64(&mut out, *year as u64);
            }
            LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale, offset }) => {
                out.push(TAG_SET_SCALE_AND_OFFSET);
                enc_f64x3_bin(*scale, &mut out);
                enc_f64x3_bin(*offset, &mut out);
            }
            LasMutation::SetBounds(set_bounds::SetBounds { max, min }) => {
                out.push(TAG_SET_BOUNDS);
                enc_f64x3_bin(*max, &mut out);
                enc_f64x3_bin(*min, &mut out);
            }
            LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts }) => {
                out.push(TAG_SET_POINTS_BY_RETURN);
                for c in counts {
                    store::pack_rt::write_varint_u64(&mut out, *c as u64);
                }
            }
            LasMutation::InsertVlr(insert_vlr::InsertVlr { index, vlr }) => {
                out.push(TAG_INSERT_VLR);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                crate::standards::v1_0::subsets::any::io::binary::diff::enc_vlr_bin(vlr, &mut out);
            }
            LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index }) => {
                out.push(TAG_REMOVE_VLR);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            LasMutation::SetVlrData(set_vlr_data::SetVlrData { index, data }) => {
                out.push(TAG_SET_VLR_DATA);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                crate::standards::v1_0::subsets::any::io::binary::diff::write_bytes_lp(&mut out, data);
            }
            LasMutation::InsertPoint(insert_point::InsertPoint { index, point }) => {
                out.push(TAG_INSERT_POINT);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                crate::standards::v1_0::subsets::any::io::binary::diff::enc_point_bin(point, &mut out);
            }
            LasMutation::RemovePoint(remove_point::RemovePoint { index }) => {
                out.push(TAG_REMOVE_POINT);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            LasMutation::SetPoint(set_point::SetPoint { index, point }) => {
                out.push(TAG_SET_POINT);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                crate::standards::v1_0::subsets::any::io::binary::diff::enc_point_bin(point, &mut out);
            }
        }
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        fn go(bytes: &[u8]) -> Result<LasMutation, String> {
            let mut reader = store::ByteReader::new(bytes);
            let format = reader.read_u8().map_err(|e| e.to_string())?;
            if format != store::pack_rt::OP_BINARY_FORMAT {
                return Err(format!("bad op format byte {format}"));
            }
            let tag = reader.read_u8().map_err(|e| e.to_string())?;
            Ok(match tag {
                TAG_SET_VERSION => LasMutation::SetVersion(set_version::SetVersion { major: reader.read_u8().map_err(|e| e.to_string())?, minor: reader.read_u8().map_err(|e| e.to_string())? }),
                TAG_SET_SYSTEM_IDENTIFIER => LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: crate::standards::v1_0::subsets::any::io::binary::diff::read_str_lp(&mut reader)? }),
                TAG_SET_SOFTWARE_INFO => LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software: crate::standards::v1_0::subsets::any::io::binary::diff::read_str_lp(&mut reader)? }),
                TAG_SET_CREATION_DATE => LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year: reader.read_varint_u64().map_err(|e| e.to_string())? as u16, year: reader.read_varint_u64().map_err(|e| e.to_string())? as u16 }),
                TAG_SET_SCALE_AND_OFFSET => LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale: dec_f64x3_bin(&mut reader)?, offset: dec_f64x3_bin(&mut reader)? }),
                TAG_SET_BOUNDS => LasMutation::SetBounds(set_bounds::SetBounds { max: dec_f64x3_bin(&mut reader)?, min: dec_f64x3_bin(&mut reader)? }),
                TAG_SET_POINTS_BY_RETURN => {
                    let mut counts = [0u32; 5];
                    for slot in counts.iter_mut() {
                        *slot = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
                    }
                    LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts })
                }
                TAG_INSERT_VLR => LasMutation::InsertVlr(insert_vlr::InsertVlr { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize, vlr: crate::standards::v1_0::subsets::any::io::binary::diff::dec_vlr_bin(&mut reader)? }),
                TAG_REMOVE_VLR => LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize }),
                TAG_SET_VLR_DATA => LasMutation::SetVlrData(set_vlr_data::SetVlrData { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize, data: crate::standards::v1_0::subsets::any::io::binary::diff::read_bytes_lp(&mut reader)? }),
                TAG_INSERT_POINT => LasMutation::InsertPoint(insert_point::InsertPoint { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize, point: crate::standards::v1_0::subsets::any::io::binary::diff::dec_point_bin(&mut reader)? }),
                TAG_REMOVE_POINT => LasMutation::RemovePoint(remove_point::RemovePoint { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize }),
                TAG_SET_POINT => LasMutation::SetPoint(set_point::SetPoint { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize, point: crate::standards::v1_0::subsets::any::io::binary::diff::dec_point_bin(&mut reader)? }),
                other => return Err(format!("las mutation: unknown binary tag {other}")),
            })
        }
        go(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "las op binary", offset: 0, detail: e })
    }
}




}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `LasMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_VERSION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-version");
const TAG_SET_SYSTEM_IDENTIFIER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-system-identifier");
const TAG_SET_SOFTWARE_INFO: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-software-info");
const TAG_SET_CREATION_DATE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-creation-date");
const TAG_SET_SCALE_AND_OFFSET: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-scale-and-offset");
const TAG_SET_BOUNDS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-bounds");
const TAG_SET_POINTS_BY_RETURN: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-points-by-return");
const TAG_INSERT_VLR: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-vlr");
const TAG_REMOVE_VLR: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-vlr");
const TAG_SET_VLR_DATA: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-vlr-data");
const TAG_INSERT_POINT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-point");
const TAG_REMOVE_POINT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-point");
const TAG_SET_POINT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-point");
//#endregion 🏷️WireTags
