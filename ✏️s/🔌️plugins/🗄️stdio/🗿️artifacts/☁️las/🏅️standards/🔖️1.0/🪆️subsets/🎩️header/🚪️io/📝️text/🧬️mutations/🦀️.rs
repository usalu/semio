//! 📝️ Text representation codec surface for `stdio.las` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1_0::subsets::any::schema::mutations::*;
use crate::schema::diff::{self, LasDiff};
use crate::schema::snapshot::{LasHeader, LasPoint, LasVlr};
use crate::LasSnapshot;
use protocol::Mutation;
#[cfg(test)]
use crate::standards::v1_0::subsets::any::schema::mutations::{point};
#[cfg(test)]
use crate::standards::v1_0::subsets::any::schema::mutations::{vlr};
/// 📋 Whole-`LasHeader` positional codec — only needed by `SetSnapshot`'s `snapshot` argument (no
/// other variant carries a full header).
pub(crate) fn enc_header(h: &LasHeader) -> String {
    let fields: Vec<String> = vec![
        h.version_major.to_string(),
        h.version_minor.to_string(),
        crate::standards::v1_0::subsets::any::io::text::diff::hex_encode(h.system_identifier.as_bytes()),
        crate::standards::v1_0::subsets::any::io::text::diff::hex_encode(h.generating_software.as_bytes()),
        h.creation_day_of_year.to_string(),
        h.creation_year.to_string(),
        h.header_size.to_string(),
        h.offset_to_point_data.to_string(),
        h.number_of_vlrs.to_string(),
        h.point_data_format_id.to_string(),
        h.point_data_record_length.to_string(),
        h.number_of_point_records.to_string(),
        crate::standards::v1_0::subsets::any::io::text::diff::enc_u32x5(&h.points_by_return),
        h.x_scale.to_string(),
        h.y_scale.to_string(),
        h.z_scale.to_string(),
        h.x_offset.to_string(),
        h.y_offset.to_string(),
        h.z_offset.to_string(),
        h.max_x.to_string(),
        h.min_x.to_string(),
        h.max_y.to_string(),
        h.min_y.to_string(),
        h.max_z.to_string(),
        h.min_z.to_string(),
    ];
    let las_field_separator = ",";
    format!("[{}]", fields.join(las_field_separator))
}

pub(crate) fn dec_header(s: &str) -> Result<LasHeader, String> {
    let parts = crate::standards::v1_0::subsets::any::io::text::diff::split_top_level(crate::standards::v1_0::subsets::any::io::text::diff::strip_brackets(s)?, ',');
    let [version_major, version_minor, system_identifier, generating_software, creation_day_of_year, creation_year, header_size, offset_to_point_data, number_of_vlrs, point_data_format_id, point_data_record_length, number_of_point_records, points_by_return, x_scale, y_scale, z_scale, x_offset, y_offset, z_offset, max_x, min_x, max_y, min_y, max_z, min_z] =
        parts.as_slice()
    else {
        return Err(format!("header: expected 25 fields, got {}", parts.len()));
    };
    Ok(LasHeader {
        version_major: crate::standards::v1_0::subsets::any::io::text::diff::parse_u8(version_major)?,
        version_minor: crate::standards::v1_0::subsets::any::io::text::diff::parse_u8(version_minor)?,
        system_identifier: String::from_utf8(crate::standards::v1_0::subsets::any::io::text::diff::hex_decode(system_identifier)?).map_err(|e| e.to_string())?,
        generating_software: String::from_utf8(crate::standards::v1_0::subsets::any::io::text::diff::hex_decode(generating_software)?).map_err(|e| e.to_string())?,
        creation_day_of_year: crate::standards::v1_0::subsets::any::io::text::diff::parse_u16(creation_day_of_year)?,
        creation_year: crate::standards::v1_0::subsets::any::io::text::diff::parse_u16(creation_year)?,
        header_size: crate::standards::v1_0::subsets::any::io::text::diff::parse_u16(header_size)?,
        offset_to_point_data: crate::standards::v1_0::subsets::any::io::text::diff::parse_u32(offset_to_point_data)?,
        number_of_vlrs: crate::standards::v1_0::subsets::any::io::text::diff::parse_u32(number_of_vlrs)?,
        point_data_format_id: crate::standards::v1_0::subsets::any::io::text::diff::parse_u8(point_data_format_id)?,
        point_data_record_length: crate::standards::v1_0::subsets::any::io::text::diff::parse_u16(point_data_record_length)?,
        number_of_point_records: crate::standards::v1_0::subsets::any::io::text::diff::parse_u32(number_of_point_records)?,
        points_by_return: crate::standards::v1_0::subsets::any::io::text::diff::dec_u32x5(points_by_return)?,
        x_scale: crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(x_scale)?,
        y_scale: crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(y_scale)?,
        z_scale: crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(z_scale)?,
        x_offset: crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(x_offset)?,
        y_offset: crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(y_offset)?,
        z_offset: crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(z_offset)?,
        max_x: crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(max_x)?,
        min_x: crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(min_x)?,
        max_y: crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(max_y)?,
        min_y: crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(min_y)?,
        max_z: crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(max_z)?,
        min_z: crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(min_z)?,
    })
}

pub(crate) fn enc_snapshot(s: &LasSnapshot) -> String {
    let las_list_separator = ",";
    let vlrs = s.vlrs.iter().map(crate::standards::v1_0::subsets::any::io::text::diff::enc_vlr).collect::<Vec<_>>().join(las_list_separator);
    let points = s.points.iter().map(crate::standards::v1_0::subsets::any::io::text::diff::enc_point).collect::<Vec<_>>().join(las_list_separator);
    format!("[{},[{}],[{}]]", enc_header(&s.header), vlrs, points)
}

pub(crate) fn dec_snapshot(s: &str) -> Result<LasSnapshot, String> {
    let inner = crate::standards::v1_0::subsets::any::io::text::diff::strip_brackets(s)?;
    let parts = crate::standards::v1_0::subsets::any::io::text::diff::split_top_level(inner, ',');
    let [header_s, vlrs_s, points_s] = parts.as_slice() else {
        return Err(format!("snapshot: expected 3 top-level fields, got {}", parts.len()));
    };
    let header = dec_header(header_s)?;
    let vlrs = crate::standards::v1_0::subsets::any::io::text::diff::split_top_level(crate::standards::v1_0::subsets::any::io::text::diff::strip_brackets(vlrs_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(crate::standards::v1_0::subsets::any::io::text::diff::dec_vlr).collect::<Result<Vec<_>, String>>()?;
    let points = crate::standards::v1_0::subsets::any::io::text::diff::split_top_level(crate::standards::v1_0::subsets::any::io::text::diff::strip_brackets(points_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(crate::standards::v1_0::subsets::any::io::text::diff::dec_point).collect::<Result<Vec<_>, String>>()?;
    Ok(LasSnapshot { schema: crate::STDIO_LAS_DOCUMENT_SCHEMA.into(), header, vlrs, points })
}

pub(crate) fn enc_f64x3(t: &(f64, f64, f64)) -> String {
    format!("[{},{},{}]", t.0, t.1, t.2)
}

pub(crate) fn dec_f64x3(s: &str) -> Result<(f64, f64, f64), String> {
    let parts = crate::standards::v1_0::subsets::any::io::text::diff::split_top_level(crate::standards::v1_0::subsets::any::io::text::diff::strip_brackets(s)?, ',');
    let [a, b, c] = parts.as_slice() else { return Err(format!("f64x3: expected 3 fields, got {}", parts.len())) };
    Ok((crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(a)?, crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(b)?, crate::standards::v1_0::subsets::any::io::text::diff::parse_f64(c)?))
}

pub(crate) fn print_las_mutation(m: &LasMutation) -> String {
    match m {
        LasMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_snapshot(snapshot)),
        LasMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),
        LasMutation::SetVersion(set_version::SetVersion { major, minor }) => format!("set-version major={major} minor={minor}"),
        LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier }) => format!("set-system-identifier system-identifier={}", crate::standards::v1_0::subsets::any::io::text::diff::hex_encode(system_identifier.as_bytes())),
        LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software }) => format!("set-software-info generating-software={}", crate::standards::v1_0::subsets::any::io::text::diff::hex_encode(generating_software.as_bytes())),
        LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year, year }) => format!("set-creation-date day-of-year={day_of_year} year={year}"),
        LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale, offset }) => format!("set-scale-and-offset scale={} offset={}", enc_f64x3(scale), enc_f64x3(offset)),
        LasMutation::SetBounds(set_bounds::SetBounds { max, min }) => format!("set-bounds max={} min={}", enc_f64x3(max), enc_f64x3(min)),
        LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts }) => format!("set-points-by-return counts={}", crate::standards::v1_0::subsets::any::io::text::diff::enc_u32x5(counts)),
        LasMutation::InsertVlr(insert_vlr::InsertVlr { index, vlr }) => format!("insert-vlr index={index} vlr={}", crate::standards::v1_0::subsets::any::io::text::diff::enc_vlr(vlr)),
        LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index }) => format!("remove-vlr index={index}"),
        LasMutation::SetVlrData(set_vlr_data::SetVlrData { index, data }) => format!("set-vlr-data index={index} data={}", crate::standards::v1_0::subsets::any::io::text::diff::hex_encode(data)),
        LasMutation::InsertPoint(insert_point::InsertPoint { index, point }) => format!("insert-point index={index} point={}", crate::standards::v1_0::subsets::any::io::text::diff::enc_point(point)),
        LasMutation::RemovePoint(remove_point::RemovePoint { index }) => format!("remove-point index={index}"),
        LasMutation::SetPoint(set_point::SetPoint { index, point }) => format!("set-point index={index} point={}", crate::standards::v1_0::subsets::any::io::text::diff::enc_point(point)),
    }
}

pub(crate) fn parse_las_mutation(line: &str) -> Result<LasMutation, String> {
    let mut tokens = line.split(' ');
    let keyword = tokens.next().filter(|k| !k.is_empty()).ok_or_else(|| "empty mutation line".to_string())?;
    let rest: Vec<&str> = tokens.collect();
    let arg = |key: &str| -> Result<&str, String> { rest.iter().find_map(|t| t.strip_prefix(key)).ok_or_else(|| format!("{keyword}: missing arg {key:?}")) };
    match keyword {
        "patch-snapshot" => semio_s_artifact_stdio_contract::editing::snapshot_patch_from_text(line).map(|patch| LasMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch })),
        "set-snapshot" => Ok(LasMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_snapshot(arg("snapshot=")?)? })),
        "set-version" => Ok(LasMutation::SetVersion(set_version::SetVersion { major: crate::standards::v1_0::subsets::any::io::text::diff::parse_u8(arg("major=")?)?, minor: crate::standards::v1_0::subsets::any::io::text::diff::parse_u8(arg("minor=")?)? })),
        "set-system-identifier" => Ok(LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: String::from_utf8(crate::standards::v1_0::subsets::any::io::text::diff::hex_decode(arg("system-identifier=")?)?).map_err(|e| e.to_string())? })),
        "set-software-info" => Ok(LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software: String::from_utf8(crate::standards::v1_0::subsets::any::io::text::diff::hex_decode(arg("generating-software=")?)?).map_err(|e| e.to_string())? })),
        "set-creation-date" => Ok(LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year: crate::standards::v1_0::subsets::any::io::text::diff::parse_u16(arg("day-of-year=")?)?, year: crate::standards::v1_0::subsets::any::io::text::diff::parse_u16(arg("year=")?)? })),
        "set-scale-and-offset" => Ok(LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale: dec_f64x3(arg("scale=")?)?, offset: dec_f64x3(arg("offset=")?)? })),
        "set-bounds" => Ok(LasMutation::SetBounds(set_bounds::SetBounds { max: dec_f64x3(arg("max=")?)?, min: dec_f64x3(arg("min=")?)? })),
        "set-points-by-return" => Ok(LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts: crate::standards::v1_0::subsets::any::io::text::diff::dec_u32x5(arg("counts=")?)? })),
        "insert-vlr" => Ok(LasMutation::InsertVlr(insert_vlr::InsertVlr { index: crate::standards::v1_0::subsets::any::io::text::diff::parse_usize(arg("index=")?)?, vlr: crate::standards::v1_0::subsets::any::io::text::diff::dec_vlr(arg("vlr=")?)? })),
        "remove-vlr" => Ok(LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index: crate::standards::v1_0::subsets::any::io::text::diff::parse_usize(arg("index=")?)? })),
        "set-vlr-data" => Ok(LasMutation::SetVlrData(set_vlr_data::SetVlrData { index: crate::standards::v1_0::subsets::any::io::text::diff::parse_usize(arg("index=")?)?, data: crate::standards::v1_0::subsets::any::io::text::diff::hex_decode(arg("data=")?)? })),
        "insert-point" => Ok(LasMutation::InsertPoint(insert_point::InsertPoint { index: crate::standards::v1_0::subsets::any::io::text::diff::parse_usize(arg("index=")?)?, point: crate::standards::v1_0::subsets::any::io::text::diff::dec_point(arg("point=")?)? })),
        "remove-point" => Ok(LasMutation::RemovePoint(remove_point::RemovePoint { index: crate::standards::v1_0::subsets::any::io::text::diff::parse_usize(arg("index=")?)? })),
        "set-point" => Ok(LasMutation::SetPoint(set_point::SetPoint { index: crate::standards::v1_0::subsets::any::io::text::diff::parse_usize(arg("index=")?)?, point: crate::standards::v1_0::subsets::any::io::text::diff::dec_point(arg("point=")?)? })),
        other => Err(format!("las mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for LasMutation {
    fn print_op(&self) -> String {
        print_las_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_las_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
