//! las rep for stdio.las 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_0::subsets::any::schema::diff::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use crate::schema::snapshot::{LasHeader, LasPoint, LasVlr};
use crate::LasSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use framework_schema::ArtifactSchema;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vlr_bin(v: &LasVlr, out: &mut Vec<u8>) {
    write_str_lp(out, &v.user_id);
    store::pack_rt::write_varint_u64(out, v.record_id as u64);
    write_str_lp(out, &v.description);
    write_bytes_lp(out, &v.data);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vlr_bin(reader: &mut store::ByteReader<'_>) -> Result<LasVlr, String> {
    Ok(LasVlr { user_id: read_str_lp(reader)?, record_id: reader.read_varint_u64().map_err(|e| e.to_string())? as u16, description: read_str_lp(reader)?, data: read_bytes_lp(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point_bin(p: &LasPoint, out: &mut Vec<u8>) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
    out.extend_from_slice(&p.z.to_le_bytes());
    store::pack_rt::write_varint_u64(out, p.intensity as u64);
    out.push(p.return_number);
    out.push(p.number_of_returns);
    out.push(p.scan_direction_flag as u8);
    out.push(p.edge_of_flight_line as u8);
    out.push(p.classification);
    out.push(p.scan_angle_rank as u8);
    out.push(p.user_data);
    store::pack_rt::write_varint_u64(out, p.point_source_id as u64);
    match p.gps_time {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    match p.rgb {
        None => out.push(0),
        Some((r, g, b)) => {
            out.push(1);
            store::pack_rt::write_varint_u64(out, r as u64);
            store::pack_rt::write_varint_u64(out, g as u64);
            store::pack_rt::write_varint_u64(out, b as u64);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point_bin(reader: &mut store::ByteReader<'_>) -> Result<LasPoint, String> {
    let x = reader.read_f64_le().map_err(|e| e.to_string())?;
    let y = reader.read_f64_le().map_err(|e| e.to_string())?;
    let z = reader.read_f64_le().map_err(|e| e.to_string())?;
    let intensity = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let return_number = reader.read_u8().map_err(|e| e.to_string())?;
    let number_of_returns = reader.read_u8().map_err(|e| e.to_string())?;
    let scan_direction_flag = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let edge_of_flight_line = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let classification = reader.read_u8().map_err(|e| e.to_string())?;
    let scan_angle_rank = reader.read_u8().map_err(|e| e.to_string())? as i8;
    let user_data = reader.read_u8().map_err(|e| e.to_string())?;
    let point_source_id = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let gps_time = match reader.read_u8().map_err(|e| e.to_string())? {
        0 => None,
        1 => Some(reader.read_f64_le().map_err(|e| e.to_string())?),
        other => return Err(format!("bad option tag {other}")),
    };
    let rgb = match reader.read_u8().map_err(|e| e.to_string())? {
        0 => None,
        1 => Some((reader.read_varint_u64().map_err(|e| e.to_string())? as u16, reader.read_varint_u64().map_err(|e| e.to_string())? as u16, reader.read_varint_u64().map_err(|e| e.to_string())? as u16)),
        other => return Err(format!("bad option tag {other}")),
    };
    Ok(LasPoint { x, y, z, intensity, return_number, number_of_returns, scan_direction_flag, edge_of_flight_line, classification, scan_angle_rank, user_data, point_source_id, gps_time, rgb })
}

const VDF_USER_ID: u8 = 1 << 0;

const VDF_RECORD_ID: u8 = 1 << 1;

const VDF_DESCRIPTION: u8 = 1 << 2;

const VDF_DATA: u8 = 1 << 3;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vlr_diff_bin(d: &LasVlrDiff, out: &mut Vec<u8>) {
    let mut mask = 0u8;
    if d.user_id.is_some() {
        mask |= VDF_USER_ID;
    }
    if d.record_id.is_some() {
        mask |= VDF_RECORD_ID;
    }
    if d.description.is_some() {
        mask |= VDF_DESCRIPTION;
    }
    if d.data.is_some() {
        mask |= VDF_DATA;
    }
    out.push(mask);
    if let Some(v) = &d.user_id {
        write_str_lp(out, v);
    }
    if let Some(v) = d.record_id {
        store::pack_rt::write_varint_u64(out, v as u64);
    }
    if let Some(v) = &d.description {
        write_str_lp(out, v);
    }
    if let Some(v) = &d.data {
        write_bytes_lp(out, v);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vlr_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<LasVlrDiff, String> {
    let mask = reader.read_u8().map_err(|e| e.to_string())?;
    let mut d = LasVlrDiff::default();
    if mask & VDF_USER_ID != 0 {
        d.user_id = Some(read_str_lp(reader)?);
    }
    if mask & VDF_RECORD_ID != 0 {
        d.record_id = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u16);
    }
    if mask & VDF_DESCRIPTION != 0 {
        d.description = Some(read_str_lp(reader)?);
    }
    if mask & VDF_DATA != 0 {
        d.data = Some(read_bytes_lp(reader)?);
    }
    Ok(d)
}

const PDF_X: u16 = 1 << 0;

const PDF_Y: u16 = 1 << 1;

const PDF_Z: u16 = 1 << 2;

const PDF_INTENSITY: u16 = 1 << 3;

const PDF_RETURN_NUMBER: u16 = 1 << 4;

const PDF_NUMBER_OF_RETURNS: u16 = 1 << 5;

const PDF_SCAN_DIRECTION_FLAG: u16 = 1 << 6;

const PDF_EDGE_OF_FLIGHT_LINE: u16 = 1 << 7;

const PDF_CLASSIFICATION: u16 = 1 << 8;

const PDF_SCAN_ANGLE_RANK: u16 = 1 << 9;

const PDF_USER_DATA: u16 = 1 << 10;

const PDF_POINT_SOURCE_ID: u16 = 1 << 11;

const PDF_GPS_TIME: u16 = 1 << 12;

const PDF_RGB: u16 = 1 << 13;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point_diff_bin(d: &LasPointDiff, out: &mut Vec<u8>) {
    let mut mask = 0u16;
    if d.x.is_some() {
        mask |= PDF_X;
    }
    if d.y.is_some() {
        mask |= PDF_Y;
    }
    if d.z.is_some() {
        mask |= PDF_Z;
    }
    if d.intensity.is_some() {
        mask |= PDF_INTENSITY;
    }
    if d.return_number.is_some() {
        mask |= PDF_RETURN_NUMBER;
    }
    if d.number_of_returns.is_some() {
        mask |= PDF_NUMBER_OF_RETURNS;
    }
    if d.scan_direction_flag.is_some() {
        mask |= PDF_SCAN_DIRECTION_FLAG;
    }
    if d.edge_of_flight_line.is_some() {
        mask |= PDF_EDGE_OF_FLIGHT_LINE;
    }
    if d.classification.is_some() {
        mask |= PDF_CLASSIFICATION;
    }
    if d.scan_angle_rank.is_some() {
        mask |= PDF_SCAN_ANGLE_RANK;
    }
    if d.user_data.is_some() {
        mask |= PDF_USER_DATA;
    }
    if d.point_source_id.is_some() {
        mask |= PDF_POINT_SOURCE_ID;
    }
    if d.gps_time.is_some() {
        mask |= PDF_GPS_TIME;
    }
    if d.rgb.is_some() {
        mask |= PDF_RGB;
    }
    out.extend_from_slice(&mask.to_le_bytes());
    if let Some(v) = d.x {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = d.y {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = d.z {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = d.intensity {
        store::pack_rt::write_varint_u64(out, v as u64);
    }
    if let Some(v) = d.return_number {
        out.push(v);
    }
    if let Some(v) = d.number_of_returns {
        out.push(v);
    }
    if let Some(v) = d.scan_direction_flag {
        out.push(v as u8);
    }
    if let Some(v) = d.edge_of_flight_line {
        out.push(v as u8);
    }
    if let Some(v) = d.classification {
        out.push(v);
    }
    if let Some(v) = d.scan_angle_rank {
        out.push(v as u8);
    }
    if let Some(v) = d.user_data {
        out.push(v);
    }
    if let Some(v) = d.point_source_id {
        store::pack_rt::write_varint_u64(out, v as u64);
    }
    if let Some(v) = d.gps_time {
        match v {
            None => out.push(0),
            Some(x) => {
                out.push(1);
                out.extend_from_slice(&x.to_le_bytes());
            }
        }
    }
    if let Some(v) = d.rgb {
        match v {
            None => out.push(0),
            Some((r, g, b)) => {
                out.push(1);
                store::pack_rt::write_varint_u64(out, r as u64);
                store::pack_rt::write_varint_u64(out, g as u64);
                store::pack_rt::write_varint_u64(out, b as u64);
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<LasPointDiff, String> {
    let mask = reader.read_u16_le().map_err(|e| e.to_string())?;
    let mut d = LasPointDiff::default();
    if mask & PDF_X != 0 {
        d.x = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
    }
    if mask & PDF_Y != 0 {
        d.y = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
    }
    if mask & PDF_Z != 0 {
        d.z = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
    }
    if mask & PDF_INTENSITY != 0 {
        d.intensity = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u16);
    }
    if mask & PDF_RETURN_NUMBER != 0 {
        d.return_number = Some(reader.read_u8().map_err(|e| e.to_string())?);
    }
    if mask & PDF_NUMBER_OF_RETURNS != 0 {
        d.number_of_returns = Some(reader.read_u8().map_err(|e| e.to_string())?);
    }
    if mask & PDF_SCAN_DIRECTION_FLAG != 0 {
        d.scan_direction_flag = Some(reader.read_u8().map_err(|e| e.to_string())? != 0);
    }
    if mask & PDF_EDGE_OF_FLIGHT_LINE != 0 {
        d.edge_of_flight_line = Some(reader.read_u8().map_err(|e| e.to_string())? != 0);
    }
    if mask & PDF_CLASSIFICATION != 0 {
        d.classification = Some(reader.read_u8().map_err(|e| e.to_string())?);
    }
    if mask & PDF_SCAN_ANGLE_RANK != 0 {
        d.scan_angle_rank = Some(reader.read_u8().map_err(|e| e.to_string())? as i8);
    }
    if mask & PDF_USER_DATA != 0 {
        d.user_data = Some(reader.read_u8().map_err(|e| e.to_string())?);
    }
    if mask & PDF_POINT_SOURCE_ID != 0 {
        d.point_source_id = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u16);
    }
    if mask & PDF_GPS_TIME != 0 {
        d.gps_time = Some(match reader.read_u8().map_err(|e| e.to_string())? {
            0 => None,
            1 => Some(reader.read_f64_le().map_err(|e| e.to_string())?),
            other => return Err(format!("bad option tag {other}")),
        });
    }
    if mask & PDF_RGB != 0 {
        d.rgb = Some(match reader.read_u8().map_err(|e| e.to_string())? {
            0 => None,
            1 => Some((reader.read_varint_u64().map_err(|e| e.to_string())? as u16, reader.read_varint_u64().map_err(|e| e.to_string())? as u16, reader.read_varint_u64().map_err(|e| e.to_string())? as u16)),
            other => return Err(format!("bad option tag {other}")),
        });
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vlrs_diff_bin(d: &LasVlrsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for i in &d.removed {
        store::pack_rt::write_varint_u64(out, *i as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_vlr_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_vlr_bin(&a.vlr, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vlrs_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<LasVlrsDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let removed = (0..removed_count).map(|_| Ok(reader.read_varint_u64().map_err(|e| e.to_string())? as usize)).collect::<Result<Vec<_>, String>>()?;
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let modified = (0..modified_count)
        .map(|_| -> Result<LasVlrModified, String> {
            let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
            let diff = dec_vlr_diff_bin(reader)?;
            Ok(LasVlrModified { index, diff })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let added = (0..added_count)
        .map(|_| -> Result<LasVlrAdded, String> {
            let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
            let vlr = dec_vlr_bin(reader)?;
            Ok(LasVlrAdded { index, vlr })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(LasVlrsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_points_diff_bin(d: &LasPointsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for i in &d.removed {
        store::pack_rt::write_varint_u64(out, *i as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_point_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_point_bin(&a.point, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_points_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<LasPointsDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let removed = (0..removed_count).map(|_| Ok(reader.read_varint_u64().map_err(|e| e.to_string())? as usize)).collect::<Result<Vec<_>, String>>()?;
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let modified = (0..modified_count)
        .map(|_| -> Result<LasPointModified, String> {
            let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
            let diff = dec_point_diff_bin(reader)?;
            Ok(LasPointModified { index, diff })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let added = (0..added_count)
        .map(|_| -> Result<LasPointAdded, String> {
            let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
            let point = dec_point_bin(reader)?;
            Ok(LasPointAdded { index, point })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(LasPointsDiff { removed, modified, added })
}

const HDR_VERSION_MAJOR: u32 = 1 << 0;

const HDR_VERSION_MINOR: u32 = 1 << 1;

const HDR_SYSTEM_IDENTIFIER: u32 = 1 << 2;

const HDR_GENERATING_SOFTWARE: u32 = 1 << 3;

const HDR_CREATION_DAY_OF_YEAR: u32 = 1 << 4;

const HDR_CREATION_YEAR: u32 = 1 << 5;

const HDR_HEADER_SIZE: u32 = 1 << 6;

const HDR_OFFSET_TO_POINT_DATA: u32 = 1 << 7;

const HDR_NUMBER_OF_VLRS: u32 = 1 << 8;

const HDR_POINT_DATA_FORMAT_ID: u32 = 1 << 9;

const HDR_POINT_DATA_RECORD_LENGTH: u32 = 1 << 10;

const HDR_NUMBER_OF_POINT_RECORDS: u32 = 1 << 11;

const HDR_POINTS_BY_RETURN: u32 = 1 << 12;

const HDR_X_SCALE: u32 = 1 << 13;

const HDR_Y_SCALE: u32 = 1 << 14;

const HDR_Z_SCALE: u32 = 1 << 15;

const HDR_X_OFFSET: u32 = 1 << 16;

const HDR_Y_OFFSET: u32 = 1 << 17;

const HDR_Z_OFFSET: u32 = 1 << 18;

const HDR_MAX_X: u32 = 1 << 19;

const HDR_MIN_X: u32 = 1 << 20;

const HDR_MAX_Y: u32 = 1 << 21;

const HDR_MIN_Y: u32 = 1 << 22;

const HDR_MAX_Z: u32 = 1 << 23;

const HDR_MIN_Z: u32 = 1 << 24;

const HDR_VLRS: u32 = 1 << 25;

const HDR_POINTS: u32 = 1 << 26;

impl protocol::DiffBinary for LasDiff {
/// ⚡️ REAL binary frame (`format u8 | header_mask u32 | <present header fields> | <vlrs diff
/// if present> | <points diff if present>`), matching `../💾️binary/📡️component.protocol.
/// semio`'s `format`/`header_mask` leading fields exactly — upgraded from F6's
/// `print_diff().into_bytes()` text-as-binary shortcut (see `#region 🔖️BinaryDiffCodec`'s doc
/// comment for the full rationale).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut mask = 0u32;
    if self.version_major.is_some() {
        mask |= HDR_VERSION_MAJOR;
    }
    if self.version_minor.is_some() {
        mask |= HDR_VERSION_MINOR;
    }
    if self.system_identifier.is_some() {
        mask |= HDR_SYSTEM_IDENTIFIER;
    }
    if self.generating_software.is_some() {
        mask |= HDR_GENERATING_SOFTWARE;
    }
    if self.creation_day_of_year.is_some() {
        mask |= HDR_CREATION_DAY_OF_YEAR;
    }
    if self.creation_year.is_some() {
        mask |= HDR_CREATION_YEAR;
    }
    if self.header_size.is_some() {
        mask |= HDR_HEADER_SIZE;
    }
    if self.offset_to_point_data.is_some() {
        mask |= HDR_OFFSET_TO_POINT_DATA;
    }
    if self.number_of_vlrs.is_some() {
        mask |= HDR_NUMBER_OF_VLRS;
    }
    if self.point_data_format_id.is_some() {
        mask |= HDR_POINT_DATA_FORMAT_ID;
    }
    if self.point_data_record_length.is_some() {
        mask |= HDR_POINT_DATA_RECORD_LENGTH;
    }
    if self.number_of_point_records.is_some() {
        mask |= HDR_NUMBER_OF_POINT_RECORDS;
    }
    if self.points_by_return.is_some() {
        mask |= HDR_POINTS_BY_RETURN;
    }
    if self.x_scale.is_some() {
        mask |= HDR_X_SCALE;
    }
    if self.y_scale.is_some() {
        mask |= HDR_Y_SCALE;
    }
    if self.z_scale.is_some() {
        mask |= HDR_Z_SCALE;
    }
    if self.x_offset.is_some() {
        mask |= HDR_X_OFFSET;
    }
    if self.y_offset.is_some() {
        mask |= HDR_Y_OFFSET;
    }
    if self.z_offset.is_some() {
        mask |= HDR_Z_OFFSET;
    }
    if self.max_x.is_some() {
        mask |= HDR_MAX_X;
    }
    if self.min_x.is_some() {
        mask |= HDR_MIN_X;
    }
    if self.max_y.is_some() {
        mask |= HDR_MAX_Y;
    }
    if self.min_y.is_some() {
        mask |= HDR_MIN_Y;
    }
    if self.max_z.is_some() {
        mask |= HDR_MAX_Z;
    }
    if self.min_z.is_some() {
        mask |= HDR_MIN_Z;
    }
    if self.vlrs.is_some() {
        mask |= HDR_VLRS;
    }
    if self.points.is_some() {
        mask |= HDR_POINTS;
    }

    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
    out.extend_from_slice(&mask.to_le_bytes());
    if let Some(v) = self.version_major {
        out.push(v);
    }
    if let Some(v) = self.version_minor {
        out.push(v);
    }
    if let Some(v) = &self.system_identifier {
        write_str_lp(&mut out, v);
    }
    if let Some(v) = &self.generating_software {
        write_str_lp(&mut out, v);
    }
    if let Some(v) = self.creation_day_of_year {
        store::pack_rt::write_varint_u64(&mut out, v as u64);
    }
    if let Some(v) = self.creation_year {
        store::pack_rt::write_varint_u64(&mut out, v as u64);
    }
    if let Some(v) = self.header_size {
        store::pack_rt::write_varint_u64(&mut out, v as u64);
    }
    if let Some(v) = self.offset_to_point_data {
        store::pack_rt::write_varint_u64(&mut out, v as u64);
    }
    if let Some(v) = self.number_of_vlrs {
        store::pack_rt::write_varint_u64(&mut out, v as u64);
    }
    if let Some(v) = self.point_data_format_id {
        out.push(v);
    }
    if let Some(v) = self.point_data_record_length {
        store::pack_rt::write_varint_u64(&mut out, v as u64);
    }
    if let Some(v) = self.number_of_point_records {
        store::pack_rt::write_varint_u64(&mut out, v as u64);
    }
    if let Some(v) = self.points_by_return {
        for x in v {
            store::pack_rt::write_varint_u64(&mut out, x as u64);
        }
    }
    if let Some(v) = self.x_scale {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = self.y_scale {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = self.z_scale {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = self.x_offset {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = self.y_offset {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = self.z_offset {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = self.max_x {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = self.min_x {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = self.max_y {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = self.min_y {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = self.max_z {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = self.min_z {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = &self.vlrs {
        enc_vlrs_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.points {
        enc_points_diff_bin(v, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    fn go(bytes: &[u8]) -> Result<LasDiff, String> {
        let mut reader = store::ByteReader::new(bytes);
        let format = reader.read_u8().map_err(|e| e.to_string())?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(format!("bad diff format byte {format}"));
        }
        let mask = reader.read_u32_le().map_err(|e| e.to_string())?;
        let mut d = LasDiff::default();
        if mask & HDR_VERSION_MAJOR != 0 {
            d.version_major = Some(reader.read_u8().map_err(|e| e.to_string())?);
        }
        if mask & HDR_VERSION_MINOR != 0 {
            d.version_minor = Some(reader.read_u8().map_err(|e| e.to_string())?);
        }
        if mask & HDR_SYSTEM_IDENTIFIER != 0 {
            d.system_identifier = Some(read_str_lp(&mut reader)?);
        }
        if mask & HDR_GENERATING_SOFTWARE != 0 {
            d.generating_software = Some(read_str_lp(&mut reader)?);
        }
        if mask & HDR_CREATION_DAY_OF_YEAR != 0 {
            d.creation_day_of_year = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u16);
        }
        if mask & HDR_CREATION_YEAR != 0 {
            d.creation_year = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u16);
        }
        if mask & HDR_HEADER_SIZE != 0 {
            d.header_size = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u16);
        }
        if mask & HDR_OFFSET_TO_POINT_DATA != 0 {
            d.offset_to_point_data = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u32);
        }
        if mask & HDR_NUMBER_OF_VLRS != 0 {
            d.number_of_vlrs = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u32);
        }
        if mask & HDR_POINT_DATA_FORMAT_ID != 0 {
            d.point_data_format_id = Some(reader.read_u8().map_err(|e| e.to_string())?);
        }
        if mask & HDR_POINT_DATA_RECORD_LENGTH != 0 {
            d.point_data_record_length = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u16);
        }
        if mask & HDR_NUMBER_OF_POINT_RECORDS != 0 {
            d.number_of_point_records = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u32);
        }
        if mask & HDR_POINTS_BY_RETURN != 0 {
            let mut a = [0u32; 5];
            for slot in a.iter_mut() {
                *slot = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
            }
            d.points_by_return = Some(a);
        }
        if mask & HDR_X_SCALE != 0 {
            d.x_scale = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
        }
        if mask & HDR_Y_SCALE != 0 {
            d.y_scale = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
        }
        if mask & HDR_Z_SCALE != 0 {
            d.z_scale = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
        }
        if mask & HDR_X_OFFSET != 0 {
            d.x_offset = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
        }
        if mask & HDR_Y_OFFSET != 0 {
            d.y_offset = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
        }
        if mask & HDR_Z_OFFSET != 0 {
            d.z_offset = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
        }
        if mask & HDR_MAX_X != 0 {
            d.max_x = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
        }
        if mask & HDR_MIN_X != 0 {
            d.min_x = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
        }
        if mask & HDR_MAX_Y != 0 {
            d.max_y = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
        }
        if mask & HDR_MIN_Y != 0 {
            d.min_y = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
        }
        if mask & HDR_MAX_Z != 0 {
            d.max_z = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
        }
        if mask & HDR_MIN_Z != 0 {
            d.min_z = Some(reader.read_f64_le().map_err(|e| e.to_string())?);
        }
        if mask & HDR_VLRS != 0 {
            d.vlrs = Some(dec_vlrs_diff_bin(&mut reader)?);
        }
        if mask & HDR_POINTS != 0 {
            d.points = Some(dec_points_diff_bin(&mut reader)?);
        }
        Ok(d)
    }
    go(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "las diff binary", offset: 0, detail: e })
}
}
}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_wire_codec {
use super::*;
use crate::standards::v1_0::subsets::any::schema::diff::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use crate::schema::snapshot::{LasHeader, LasPoint, LasVlr};
use crate::LasSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use framework_schema::ArtifactSchema;

/// 🧭️ Full (non-sparse) `LasHeader` binary record — every field always present, declaration
/// order — reused by `🧬️mutations/🦀️.rs`'s `SetSnapshot` binary payload (a whole
/// `LasSnapshot` embeds a whole `LasHeader`, never a sparse patch).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_bin(h: &LasHeader, out: &mut Vec<u8>) {
    out.push(h.version_major);
    out.push(h.version_minor);
    write_str_lp(out, &h.system_identifier);
    write_str_lp(out, &h.generating_software);
    store::pack_rt::write_varint_u64(out, h.creation_day_of_year as u64);
    store::pack_rt::write_varint_u64(out, h.creation_year as u64);
    store::pack_rt::write_varint_u64(out, h.header_size as u64);
    store::pack_rt::write_varint_u64(out, h.offset_to_point_data as u64);
    store::pack_rt::write_varint_u64(out, h.number_of_vlrs as u64);
    out.push(h.point_data_format_id);
    store::pack_rt::write_varint_u64(out, h.point_data_record_length as u64);
    store::pack_rt::write_varint_u64(out, h.number_of_point_records as u64);
    for v in h.points_by_return {
        store::pack_rt::write_varint_u64(out, v as u64);
    }
    for v in [h.x_scale, h.y_scale, h.z_scale, h.x_offset, h.y_offset, h.z_offset, h.max_x, h.min_x, h.max_y, h.min_y, h.max_z, h.min_z] {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_bin(reader: &mut store::ByteReader<'_>) -> Result<LasHeader, String> {
    let version_major = reader.read_u8().map_err(|e| e.to_string())?;
    let version_minor = reader.read_u8().map_err(|e| e.to_string())?;
    let system_identifier = read_str_lp(reader)?;
    let generating_software = read_str_lp(reader)?;
    let creation_day_of_year = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let creation_year = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let header_size = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let offset_to_point_data = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    let number_of_vlrs = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    let point_data_format_id = reader.read_u8().map_err(|e| e.to_string())?;
    let point_data_record_length = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let number_of_point_records = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    let mut points_by_return = [0u32; 5];
    for slot in points_by_return.iter_mut() {
        *slot = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    }
    let mut floats = [0.0f64; 12];
    for slot in floats.iter_mut() {
        *slot = reader.read_f64_le().map_err(|e| e.to_string())?;
    }
    let [x_scale, y_scale, z_scale, x_offset, y_offset, z_offset, max_x, min_x, max_y, min_y, max_z, min_z] = floats;
    Ok(LasHeader {
        version_major,
        version_minor,
        system_identifier,
        generating_software,
        creation_day_of_year,
        creation_year,
        header_size,
        offset_to_point_data,
        number_of_vlrs,
        point_data_format_id,
        point_data_record_length,
        number_of_point_records,
        points_by_return,
        x_scale,
        y_scale,
        z_scale,
        x_offset,
        y_offset,
        z_offset,
        max_x,
        min_x,
        max_y,
        min_y,
        max_z,
        min_z,
    })
}
}
pub use diff_wire_codec::*;
